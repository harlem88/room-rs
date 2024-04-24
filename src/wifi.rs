use esp_idf_hal::modem::Modem;
use esp_idf_svc::wifi::{AsyncWifi, AuthMethod, ClientConfiguration, Configuration, EspWifi};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, timer::EspTaskTimerService,
};

use crate::RoomError;

pub async fn init_wifi(
    ssid: &str,
    password: &str,
    modem: Modem,
    timer_service: &EspTaskTimerService,
) -> Result<AsyncWifi<EspWifi<'static>>, RoomError> {
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi_driver = AsyncWifi::wrap(
        EspWifi::new(modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
        timer_service.clone(),
    )?;

    wifi_driver.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: password.try_into().unwrap(),
        channel: None,
        ..Default::default()
    }))?;

    Ok(wifi_driver)
}

pub async fn connect_wifi(wifi: &mut AsyncWifi<EspWifi<'static>>) -> Result<(), RoomError> {
    wifi.start().await?;

    log::info!("Waiting for connect");
    while let Err(err) = wifi.connect().await {
        log::error!("Unable to connect wifi, {:?}", err);
    }

    log::info!("Waiting network interface is up");
    // Wait until the network interface is up
    wifi.wait_netif_up().await?;

    while !wifi.is_connected()? {
        let config = wifi.get_configuration()?;
        log::info!("Waiting for station {:?}", config);
    }

    log::info!("Should be connected now");
    Ok(())
}
