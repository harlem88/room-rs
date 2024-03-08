use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

use crate::si70xx_sensor::SensorMeasure;

pub mod room_sensor;
pub mod sensor;
pub mod si70xx_sensor;
pub struct SensorValue {
    pub name: String,
    pub value: Vec<u8>,
}

#[derive(Error, Debug)]
pub enum RoomSensorError {
    #[error(transparent)]
    Measure(#[from] mpsc::error::SendError<SensorMeasure>),
    #[error(transparent)]
    Receiver(#[from] oneshot::error::RecvError),
    #[error(transparent)]
    EspError(#[from] esp_idf_hal::sys::EspError),
    #[error("unknown room sensor error")]
    Unknown,
}
