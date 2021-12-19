
use std::format;

use log::{debug, warn, error};
use embedded_hal::i2c::blocking::*;
use linux_embedded_hal::I2cdev;

use crate::api::{self, Error};
use super::LinuxCtx;

impl api::I2c for LinuxCtx {
    fn init(&mut self, dev: u32, _baud: u32, _sda: i32, _sck: i32) -> Result<i32, Error> {
        
        let p = format!("/dev/i2c-{}", dev);
        debug!("Opening I2C device: {}", p);

        let idx = self.count;
        self.count += 1;

        // Build i2cdev string and open device
        let i2c_dev = match I2cdev::new(p) {
            Ok(d) => d,
            Err(e) => {
                error!("Failed to open i2c device: {:?}", e);
                return Err(Error::Failed);
            }
        };

        // Store for later use
        self.i2c.insert(idx, i2c_dev);

        // Return index
        Ok(idx)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        debug!("Dropping I2C handle: {}", handle);

        let mut _i2c_dev = self.i2c.remove(&handle);

        Ok(())
    }

    fn write(&mut self, handle: i32, addr: u16, data: &[u8]) -> Result<(), Error> {
        debug!("I2C write for handle: {} addr: {}", handle, addr);

        // Fetch i2c device instance
        let i2c_dev = match self.i2c.get_mut(&handle) {
            Some(d) => d,
            None => {
                error!("No i2c device for handle: {}", handle);
                return Err(Error::NoDevice)
            }
        };

        // Perform operation
        if let Err(e) = i2c_dev.write(addr as u8, data) {
            warn!("I2C write failed: {:?}", e);
            return Err(Error::Failed)
        }

        Ok(())
    }

    fn read(&mut self, handle: i32, addr: u16, buff: &mut [u8]) -> Result<(), Error> {
        debug!("I2C read for handle: {} addr: {}", handle, addr);

        // Fetch i2c device instance
        let i2c_dev = match self.i2c.get_mut(&handle) {
            Some(d) => d,
            None => {
                error!("No i2c device for handle: {}", handle);
                return Err(Error::NoDevice)
            }
        };

        // Perform operation
        if let Err(e) = i2c_dev.read(addr as u8, buff) {
            warn!("I2C write read failed: {:?}", e);
            return Err(Error::Failed)
        }

        Ok(())
    }

    fn write_read(&mut self, handle: i32, addr: u16, data: &[u8], buff: &mut [u8]) -> Result<(), Error> {
        debug!("I2C write_read for handle: {} addr: {}", handle, addr);

        // Fetch i2c device instance
        let i2c_dev = match self.i2c.get_mut(&handle) {
            Some(d) => d,
            None => {
                error!("No i2c device for handle: {}", handle);
                return Err(Error::NoDevice)
            }
        };

        // Perform operation
        if let Err(e) = i2c_dev.write_read(addr as u8, data, buff) {
            warn!("I2C write_read failed: {:?}", e);
            return Err(Error::Failed)
        }

        Ok(())
    }
}
