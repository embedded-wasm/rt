use std::format;

use log::{debug, error};

use embedded_hal::spi::blocking::*;
use linux_embedded_hal::{Spidev, spidev::{SpiModeFlags, SpidevOptions}};

use crate::api::{self, Error};
use super::LinuxCtx;


impl api::Spi for LinuxCtx {
    fn init(&mut self, dev: u32, baud: u32, _mosi: i32, _miso: i32, _sck: i32, _cs: i32) -> Result<i32, Error> {

        // TODO: how to deal with subdevices here?!
        let p = format!("/dev/spidev{}.{}", dev, 0);

        debug!("Opening SPI device {} at {} baud", p, baud);

        let idx = self.count;
        self.count += 1;

        // Build spidev string and open device
        let mut spi_dev = match Spidev::open(p) {
            Ok(d) => d,
            Err(e) => {
                error!("Failed to open spi device: {:?}", e);
                return Err(Error::Failed);
            }
        };

        // TODO: pass through SPI mode
        let opts = SpidevOptions{
            max_speed_hz: Some(baud),
            spi_mode: Some(SpiModeFlags::SPI_MODE_2 | SpiModeFlags::SPI_NO_CS),
            ..Default::default()
        };

        // Attempt configuration
        if let Err(e) = spi_dev.configure(&opts) {
            error!("Failed to configure SPI device: {:?}", e);
            return Err(Error::Failed)
        }

        // Store for later use
        self.spi.insert(idx, spi_dev);

        // Return index
        Ok(idx)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        debug!("Dropping SPI handle: {}", handle);

        let mut _spi_dev = self.spi.remove(&handle);

        Ok(())
    }

    fn write<'a>(&mut self, handle: i32, data: &[u8]) -> Result<(), Error> {
        debug!("SPI write for handle: {}", handle);

        // Fetch spi device instance
        let mut spi_dev = match self.spi.get_mut(&handle) {
            Some(d) => d,
            None => {
                error!("No spi device for handle: {}", handle);
                return Err(Error::NoDevice)
            }
        };

        // Perform operation
        if let Err(e) = SpiBusWrite::write(&mut spi_dev, data) {
            error!("SPI write failed: {:?}", e);
            return Err(Error::Failed)
        }

        Ok(())
    }

    fn read<'a>(&mut self, handle: i32, data: &mut [u8]) -> Result<(), Error> {
        debug!("SPI read for handle: {}", handle);

        // Fetch spi device instance
        let mut spi_dev = match self.spi.get_mut(&handle) {
            Some(d) => d,
            None => {
                error!("No spi device for handle: {}", handle);
                return Err(Error::NoDevice)
            }
        };

        // Perform operation
        if let Err(e) = SpiBusRead::read(&mut spi_dev, data) {
            error!("SPI read failed: {:?}", e);
            return Err(Error::Failed)
        }

        Ok(())
    }

    fn transfer_inplace<'a>(&mut self, handle: i32, data: &mut [u8]) -> Result<(), Error> {
        debug!("SPI transfer for handle: {}", handle);

        // Fetch spi device instance
        let mut spi_dev = match self.spi.get_mut(&handle) {
            Some(d) => d,
            None => {
                error!("No spi device for handle: {}", handle);
                return Err(Error::NoDevice)
            }
        };

        // Perform operation
        if let Err(e) = SpiBus::transfer_in_place(&mut spi_dev, data) {
            error!("SPI write failed: {:?}", e);
            return Err(Error::Failed)
        }

        Ok(())
    }

    fn transfer<'a>(&mut self, handle: i32, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        debug!("SPI transfer for handle: {}", handle);

        // Fetch spi device instance
        let mut spi_dev = match self.spi.get_mut(&handle) {
            Some(d) => d,
            None => {
                error!("No spi device for handle: {}", handle);
                return Err(Error::NoDevice)
            }
        };

        // Perform operation
        if let Err(e) = SpiBus::transfer(&mut spi_dev, read, write) {
            error!("SPI write failed: {:?}", e);
            return Err(Error::Failed)
        }

        Ok(())
    }


}
