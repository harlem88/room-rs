use thiserror::Error;

pub mod wifi;
pub mod config;

#[derive(Error, Debug)]
pub enum RoomError {
    #[error(transparent)]
    EspError(#[from] esp_idf_hal::sys::EspError),
    #[error("unknown room error")]
    Unknown,
}
