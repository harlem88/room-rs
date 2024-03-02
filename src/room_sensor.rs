use crate::sensor::Sensors;
use crate::{RoomSensorError, SensorValue};

pub(crate) trait SensorRead<T> {
    async fn read(&self) -> Result<T, RoomSensorError>;
}

pub struct Room<'a> {
    name: &'a str,
    sensors: Vec<Sensors<'a>>,
}

impl<'a> Room<'a> {
    pub fn new(name: &'a str) -> Self {
        Room {
            name,
            sensors: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_sensor(&mut self, sensor: Sensors<'a>) {
        self.sensors.push(sensor)
    }

    pub async fn collect(&self) -> Vec<SensorValue> {
        let mut values: Vec<SensorValue> = vec![];
        for sensor in &self.sensors {
            let (name, value) = match sensor {
                Sensors::Humidity(sensor_name, sensor) => {
                    let Ok(value) = sensor.read().await else {
                        log::warn!("Unable to read humidity-sensor");
                        continue;
                    };

                    (sensor_name.to_string(), value.to_be_bytes().to_vec())
                }
                Sensors::Temperature(sensor_name, sensor) => {
                    let Ok(value) = sensor.read().await else {
                        log::warn!("Unable to read temperature-sensor");
                        continue;
                    };

                    (sensor_name.to_string(), value.to_be_bytes().to_vec())
                }
            };
            values.push(SensorValue { name, value })
        }
        values
    }
}
