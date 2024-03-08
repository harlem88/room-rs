use esp_idf_hal::modem::Modem;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::timer::EspTaskTimerService;
use esp_idf_svc::wifi::{AsyncWifi, ClientConfiguration, Configuration, EspWifi};

use crate::RoomError;

pub async fn init_wifi(
    ssid: &str,
    password: &str,
    modem: Modem,
    timer_service: &EspTaskTimerService,
) -> Result<AsyncWifi<EspWifi<'static>>, RoomError> {
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi_driver = EspWifi::new(modem, sys_loop.clone(), Some(nvs))?;

    wifi_driver.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.try_into().unwrap(),
        password: password.try_into().unwrap(),
        ..Default::default()
    }))?;

    Ok(AsyncWifi::wrap(
        wifi_driver,
        sys_loop.clone(),
        timer_service.clone(),
    )?)
}

pub async fn connect_wifi(wifi: &mut AsyncWifi<EspWifi<'static>>) -> Result<(), RoomError> {
    wifi.start().await?;
    wifi.connect().await?;

    // Wait until the network interface is up
    wifi.wait_netif_up().await?;

    while !wifi.is_connected()? {
        let config = wifi.get_configuration()?;
        log::info!("Waiting for station {:?}", config);
    }

    log::info!("Should be connected now");
    Ok(())
}
