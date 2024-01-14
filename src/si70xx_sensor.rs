use embedded_hal::delay::DelayUs;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::{
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
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
    pub fn init() -> Result<Sender<SensorMeasure>, RoomSensorError> {
        log::info!("init sensor!");

        let peripherals = Peripherals::take()?;

        let sda = peripherals.pins.gpio21;
        let scl = peripherals.pins.gpio22;

        let config = I2cConfig::new().baudrate(100.kHz().into());
        let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config)?;
        let sensor = Si70xx::new(i2c);

        let tx = Si70xxSensor::run_sensor_handler(sensor);
        Ok(tx)
    }

    fn run_sensor_handler(mut sensor: Si70xx<I2cDriver<'static>>) -> Sender<SensorMeasure> {
        let (tx, mut rx) = mpsc::channel(2);
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                log::info!("measure sensor!");
                let _ = sensor.measure();
                FreeRtos.delay_ms(20u32);
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
                }
            }
        });
        tx
    }
}
