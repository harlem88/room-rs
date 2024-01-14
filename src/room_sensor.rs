use embedded_hal::delay::DelayUs;
use esp_idf_hal::delay::FreeRtos;

use crate::sensor::Sensors;
use crate::RoomSensorError;

pub(crate) trait SensorRead<T> {
    async fn read(&self) -> Result<T, RoomSensorError>;
}

pub struct Room {
    name: String,
    sensors: Vec<Sensors>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Room {
            name,
            sensors: Vec::new(),
        }
    }

    pub fn add_sensor(&mut self, sensor: Sensors) {
        self.sensors.push(sensor)
    }

    pub async fn collect(&self) {
        for sensor in &self.sensors {
            match sensor {
                Sensors::Humidity(name, sensor) => {
                    let Ok(value) = sensor.read().await else {
                        log::info!("none");
                        continue;
                    };

                    log::info!("hum {}", value)
                }
                Sensors::Temperature(name, sensor) => {
                    let Ok(value) = sensor.read().await else {
                        log::info!("none");
                        continue;
                    };
                    log::info!("temp {}", value)
                }
            }
            FreeRtos.delay_ms(100u32);
        }
    }
}
