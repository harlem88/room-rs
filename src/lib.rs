use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

use crate::si70xx_sensor::SensorMeasure;

pub mod room_sensor;
pub mod sensor;
pub mod si70xx_sensor;

#[derive(Error, Debug)]
pub enum RoomSensorError {
    #[error("Error during measure operation")]
    Measure(#[from] mpsc::error::SendError<SensorMeasure>),
    #[error("Oneshot Receiver Error")]
    Receiver(#[from] oneshot::error::RecvError),
    #[error("ESP Error")]
    EspError(#[from] esp_idf_hal::sys::EspError),
    #[error("unknown room sensor error")]
    Unknown,
}
