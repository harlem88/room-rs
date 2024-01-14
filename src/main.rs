use embedded_hal::delay::DelayUs;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::task::block_on;

use room_sensor_rs::room_sensor::Room;
use room_sensor_rs::sensor::Sensors;
use room_sensor_rs::si70xx_sensor::Si70xxSensor;
use room_sensor_rs::RoomSensorError;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Sensor Room starting ... ");
    block_on(start());
    log::info!("Main thread finished");
}

async fn start() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let _ = sensor_reads().await;
        });
}

pub async fn sensor_reads() -> Result<(), RoomSensorError> {
    let mut room = Room::new("room1".to_string());

    let si70xx = Si70xxSensor::init()?;

    room.add_sensor(Sensors::humidity("humidity-1".to_string(), si70xx.clone()));
    room.add_sensor(Sensors::temperature("temperature-1".to_string(), si70xx));

    loop {
        room.collect().await;
        FreeRtos.delay_ms(2000u32);
    }
}
