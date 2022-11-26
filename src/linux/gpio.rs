
use log::{debug, error};
use embedded_hal::digital::{PinState, blocking::*};
use linux_embedded_hal::{SysfsPin, sysfs_gpio::Direction};

use crate::api::{Gpio, Error};
use super::*;

impl Gpio for LinuxCtx {
    /// Initialise the provided GPIO pin in input or output mode
    fn init(&mut self, _port: i32, pin: i32, output: bool) -> Result<i32, Error> {
        
        debug!("GPIO init port: {} pin: {} output: {:?}", _port, pin, output);

        let idx = self.count;
        self.count += 1;
        
        let pin = SysfsPin::new(pin as u64);

        if let Err(e) = pin.export() {
            error!("Failed to export pin: {:?}", e);
            return Err(Error::Failed)
        }

        let dir = match output {
            true => Direction::Out,
            false => Direction::In,
        };

        if let Err(e) = pin.set_direction(dir) {
            error!("Failed to set direction: {:?}", e);
            return Err(Error::Failed)
        }

        // Store for later use
        self.gpio.insert(idx, pin);

        // Return index
        Ok(idx)
    }

    /// Deinitialise the specified GPIO pin
    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        debug!("Dropping GPIO handle: {}", handle);

        let mut _pin = self.gpio.remove(&handle);

        Ok(())
    }

    /// Write to a GPIO pin
    fn set(&mut self, handle: i32, state: PinState) -> Result<(), Error> {
        debug!("GPIO set handle: {} val: {:?}", handle, state);
        
        // Fetch gpio instance
        let pin = match self.gpio.get_mut(&handle) {
            Some(d) => d,
            None => {
                error!("No gpio device for handle: {}", handle);
                return Err(Error::NoDevice)
            }
        };

        // Attempt to set state
        if let Err(e) = pin.set_state(state) {
            error!("Failed to set pin state: {:?}", e);
            return Err(Error::Failed)
        }

        Ok(())
    }

    // Read from a GPIO pin
    fn get(&mut self, handle: i32) -> Result<PinState, Error> {
        debug!("GPIO get handle: {}", handle);
        // Fetch gpio instance
        let pin = match self.gpio.get_mut(&handle) {
            Some(d) => d,
            None => {
                error!("No gpio device for handle: {}", handle);
                return Err(Error::NoDevice)
            }
        };

        // Attempt to fetch pin state
        match pin.is_high() {
            Ok(true) => Ok(PinState::High),
            Ok(false) => Ok(PinState::Low),
            Err(e) => {
                error!("Failed to fetch pin state: {:?}", e);
                Err(Error::Failed)
            }
        }
    }
}
