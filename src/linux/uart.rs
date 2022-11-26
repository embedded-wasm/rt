
use std::format;

use log::{debug, error};

use embedded_hal::serial::{
    nb::{Read, Write},
};
use linux_embedded_hal::Serial;

use crate::api::{self, Error};
use super::LinuxCtx;

impl api::Uart for LinuxCtx {
    fn init(&mut self, dev: u32, _baud: u32, _tx: i32, _rx: i32) -> Result<i32, Error> {
        
        // TODO: swap to string for naming... easier to reverse than otherwise
        let p = format!("/dev/tty{}", dev);
        debug!("Opening UART device: {}", p);

        let idx = self.count;
        self.count += 1;

        // Build device string and open device
        let uart_dev = match Serial::open(p) {
            Ok(d) => d,
            Err(e) => {
                error!("Failed to open uart device: {:?}", e);
                return Err(Error::Failed);
            }
        };

        // TODO: set baud etc.

        // Store for later use
        self.uart.insert(idx, uart_dev);

        // Return index
        Ok(idx)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        debug!("Dropping UART handle: {}", handle);

        let mut _uart_dev = self.uart.remove(&handle);

        Ok(())
    }

    fn write(&mut self, handle: i32, flags: u32, data: &[u8]) -> Result<(), Error> {
        debug!("UART write for handle: {} flags: {}", handle, flags);

        // Fetch uart device instance
        let uart_dev = match self.uart.get_mut(&handle) {
            Some(d) => d,
            None => {
                error!("No uart device for handle: {}", handle);
                return Err(Error::NoDevice)
            }
        };

        // Perform write
        // TODO: this is extremely inefficient, where's the blocking version..?
        for d in data.iter() {
            uart_dev.write(*d)
                .map_err(|_| Error::Failed)?;
        }

        Ok(())
    }

    fn read(&mut self, handle: i32, flags: u32, buff: &mut [u8]) -> Result<(), Error> {
        debug!("UART read for handle: {} flags: {}", handle, flags);

        // Fetch uart device instance
        let uart_dev = match self.uart.get_mut(&handle) {
            Some(d) => d,
            None => {
                error!("No uart device for handle: {}", handle);
                return Err(Error::NoDevice)
            }
        };

        // Perform read
        // TODO: this is fairly inefficient, where's the blocking version..?
        for d in buff.iter_mut() {
            *d = uart_dev.read()
                    .map_err(|_| Error::Failed)?;
        }

        Ok(())
    }
}
