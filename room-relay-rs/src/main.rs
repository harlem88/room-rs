use core::pin::pin;
use std::time::Duration;

use embassy_futures::select::select;

use esp_idf_hal::{gpio::Gpio23, peripherals::Peripherals};
use esp_idf_svc::{
    mqtt::client::{
        EspAsyncMqttClient, EspAsyncMqttConnection, EventPayload, MqttClientConfiguration, QoS,
    },
    timer::{EspTaskTimerService, EspTimerService},
};
use eyre::Result;
use room_relay_rs::Relay;
use room_rs::wifi;

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Relay starting ... {}", room_rs::config::CONFIG.room_name);

    let res = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move { start().await });

    log::info!("Main thread finished");

    res
}

async fn start() -> Result<()> {
    let peripherals = Peripherals::take().expect("Unable to take peripherals");

    let Peripherals { modem, pins, .. } = peripherals;
    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config = room_rs::config::CONFIG;

    let timer_service = EspTimerService::new()?;
    let mut wifi = wifi::init_wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        modem,
        &timer_service,
    )
    .await
    .expect("Unable to init wifi");

    while let Err(err) = wifi::connect_wifi(&mut wifi).await {
        log::error!("Unable to connect wifi, {:?}", err);
    }

    let mqtt_config = MqttClientConfiguration::default();
    let (mut mqtt_client, mut mqtt_connection) =
        EspAsyncMqttClient::new(app_config.mqtt_host, &mqtt_config).expect("Unable to init mqtt");

    log::info!("MQTT Listening for messages");

    let mut relay = Relay::new("on_air", pins.gpio23)?;
    let topic_sub = format!(
        "/{0}/{1}/{2}",
        app_config.topic_root, app_config.room_name, relay.name
    );

    let topic_sub_cl = topic_sub.clone();
    let _ = select(
        pin!(async move { mqtt_poll(topic_sub_cl, relay, &mut mqtt_connection).await }),
        pin!(async move { mqtt_subscribe(topic_sub, &mut mqtt_client, &timer_service).await }),
    )
    .await;

    log::info!("Connection closed");

    Ok(())
}

pub async fn mqtt_subscribe(
    topic_sub: String,
    mqtt_client: &mut EspAsyncMqttClient,
    timer_service: &EspTaskTimerService,
) -> Result<()> {
    let mut timer = timer_service.timer_async()?;
    timer.after(Duration::from_millis(500)).await?;

    let _ = mqtt_client.subscribe(&topic_sub, QoS::AtMostOnce).await;

    loop {
        timer.after(Duration::from_secs(1)).await?;
    }
}

pub async fn mqtt_poll(
    topic_sub: String,
    mut relay: Relay<'_, Gpio23>,
    mqtt_connection: &mut EspAsyncMqttConnection,
) -> Result<()> {
    log::info!("MQTT Listening for messages");

    while let Ok(event) = mqtt_connection.next().await {
        match event.payload() {
            EventPayload::Connected(_) => log::info!("Connected"),
            EventPayload::Subscribed(id) => log::info!("Subscribed to {} id", id),
            EventPayload::Received {
                id: _,
                topic,
                data,
                details: _,
            } => {
                log::info!("[Queue] Event: {}", event.payload());
                topic
                    .is_some_and(|topic| topic.eq(&topic_sub))
                    .then(|| relay.handle_message(std::str::from_utf8(&data)));
            }
            _ => log::info!("[Queue] Event: {}", event.payload()),
        }
    }
    log::info!("Connection closed");

    Ok(())
}
