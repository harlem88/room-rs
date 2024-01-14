use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use crate::room_sensor::SensorRead;
use crate::si70xx_sensor::SensorMeasure;
use crate::RoomSensorError;

pub enum Sensors {
    Humidity(String, HumiditySensor),
    Temperature(String, TemperatureSensor),
}

impl Sensors {
    pub fn humidity(name: String, sensor: Sender<SensorMeasure>) -> Sensors {
        Sensors::Humidity(
            name,
            HumiditySensor {
                sensor_handler: sensor,
            },
        )
    }

    pub fn temperature(name: String, sensor: Sender<SensorMeasure>) -> Sensors {
        Sensors::Temperature(
            name,
            TemperatureSensor {
                sensor_handler: sensor,
            },
        )
    }
}
pub struct HumiditySensor {
    sensor_handler: Sender<SensorMeasure>,
}
pub struct TemperatureSensor {
    sensor_handler: Sender<SensorMeasure>,
}

impl SensorRead<f32> for HumiditySensor {
    async fn read(&self) -> Result<f32, RoomSensorError> {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = SensorMeasure::Humidity { resp: resp_tx };
        let _ = self.sensor_handler.send(cmd).await?;
        resp_rx.await?.ok_or(RoomSensorError::Unknown)
    }
}

impl SensorRead<f32> for TemperatureSensor {
    async fn read(&self) -> Result<f32, RoomSensorError> {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = SensorMeasure::Temperature { resp: resp_tx };
        self.sensor_handler.send(cmd).await?;
        resp_rx.await?.ok_or(RoomSensorError::Unknown)
    }
}
