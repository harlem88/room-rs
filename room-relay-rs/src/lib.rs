use std::str::Utf8Error;

use esp_idf_hal::{
    gpio::{self, OutputPin, PinDriver},
    sys::EspError,
};

pub struct Relay<'a, T: OutputPin> {
    pub name: &'a str,
    pin: PinDriver<'a, T, gpio::Output>,
}

impl<'a, T> Relay<'a, T>
where
    T: OutputPin,
{
    pub fn new(name: &'a str, pin: T) -> Result<Self, EspError> {
        log::info!("init relay!");

        let mut pin = PinDriver::output(pin)?;
        let _ = pin.set_low();
        Ok(Self { name, pin })
    }

    pub fn handle_message(&mut self, msg: Result<&str, Utf8Error>) {
        if msg.is_ok_and(|msg| msg == "1") {
            let _ = self.pin.set_high();
        } else {
            log::info!("0");
            let _ = self.pin.set_low();
        }
    }
}
