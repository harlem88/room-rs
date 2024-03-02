use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::Pins;
use esp_idf_hal::i2c::I2C0;
use esp_idf_hal::{
    i2c::{I2cConfig, I2cDriver},
    prelude::*,
};
use si70xx::Si70xx;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, oneshot};

use crate::RoomSensorError;

pub struct Si70xxSensor;

type Responder<T> = oneshot::Sender<Option<T>>;

#[derive(Debug)]
pub enum SensorMeasure {
    Humidity { resp: Responder<f32> },
    Temperature { resp: Responder<f32> },
}

impl Si70xxSensor {
    pub fn init(pins: Pins, i2c0: I2C0) -> Result<Sender<SensorMeasure>, RoomSensorError> {
        log::info!("init sensor!");

        let sda = pins.gpio21;
        let scl = pins.gpio22;

        let config = I2cConfig::new().baudrate(100.kHz().into());
        let i2c = I2cDriver::new(i2c0, sda, scl, &config)?;
        let sensor = Si70xx::new(i2c);

        let tx = Si70xxSensor::run_sensor_handler(sensor);
        Ok(tx)
    }

    fn run_sensor_handler(mut sensor: Si70xx<I2cDriver<'static>>) -> Sender<SensorMeasure> {
        let (tx, mut rx) = mpsc::channel(2);
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                log::info!("measure sensor!");

                if let Err(err) = sensor.measure() {
                    log::error!("Unable to measure sensor {:?}", err);
                    match message {
                        SensorMeasure::Humidity { resp } | SensorMeasure::Temperature { resp } => {
                            let _ = resp.send(None);
                        }
                    };

                    continue;
                };

                FreeRtos::delay_ms(20);
                match message {
                    SensorMeasure::Humidity { resp } => {
                        let value = match sensor.read_humidity() {
                            Ok(hum) => Some(hum as f32 / 100.0),
                            Err(_) => None,
                        };
                        let _ = resp.send(value);
                    }
                    SensorMeasure::Temperature { resp } => {
                        let value = match sensor.read_temperature() {
                            Ok(temperature) => Some(temperature as f32 / 100.0),
                            Err(_) => None,
                        };
                        let _ = resp.send(value);
                    }
                };
            }
        });
        tx
    }
}
