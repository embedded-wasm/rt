//! Mock SPI driver implementation

use std::sync::{Arc, Mutex};

use log::debug;

use wasm_embedded_spec::{Error, Spi};
use super::{Inner, Op, Kind};

pub struct MockSpi {
    inner: Arc<Mutex<Inner>>,
}


impl MockSpi {
    pub(crate)  const fn new(inner: Arc<Mutex<Inner>>) -> Self {
        Self { inner }
    }
}

impl Spi for MockSpi {
    fn init(&mut self, port: u32, baud: u32, mosi: i32, miso: i32, sck: i32, cs: i32) -> Result<i32, Error> {
        let mut inner = self.inner.lock().unwrap();

        debug!("Opening SPI port: {} (baud: {} mosi: {} miso: {} sck: {} cs: {})", port, baud, mosi, miso, sck, cs);

        let op = Kind::SpiInit{port, baud, mosi, miso, sck, cs};
        let Op{kind, res} = inner.expected[inner.index].clone();

        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;

        Ok(res)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        let mut inner = self.inner.lock().unwrap();

        debug!("Closing SPI handle: {}", handle);
        let op = Kind::SpiDeinit{handle};
        let Op{kind, ..} = inner.expected[inner.index].clone();

        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;

        Ok(())
    }

    fn read<'a>(&mut self, handle: i32, data: &mut [u8]) -> Result<(), Error> {
        let mut inner = self.inner.lock().unwrap();

        let kind = inner.expected[inner.index].kind.clone();

        if let Kind::SpiRead{data_in, ..} = &kind {
            data.copy_from_slice(&data_in);
        }

        debug!("SPI write read: {} data: {:02x?}", handle, data);

        let op = Kind::SpiRead{handle, data_in: data.to_vec()};
        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;

        Ok(())
    }

    fn write<'a>(&mut self, handle: i32, data: &[u8]) -> Result<(), Error> {
        let mut inner = self.inner.lock().unwrap();

        let op = Kind::SpiWrite{handle, data_out: data.to_vec()};
        let kind = inner.expected[inner.index].kind.clone();

        debug!("SPI write handle: {} data: {:02x?}", handle, data);

        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;

        Ok(())
    }

    fn transfer_inplace<'a>(&mut self, handle: i32, data: &mut [u8]) -> Result<(), Error> {
        let mut inner = self.inner.lock().unwrap();

        let kind = inner.expected[inner.index].kind.clone();

        let d = data.to_vec();

        if let Kind::SpiTransfer{data_in, ..} = &kind {
            data.copy_from_slice(&data_in);
        }

        debug!("SPI transfer handle: {} write: {:02x?} read: {:02x?}", handle, d, data);

        let op = Kind::SpiTransfer{handle, data_out: d, data_in: data.to_vec()};
        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;
        
        Ok(())
    }

    fn transfer<'a>(&mut self, handle: i32, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        let mut inner = self.inner.lock().unwrap();

        let kind = inner.expected[inner.index].kind.clone();

        if let Kind::SpiTransfer{data_in, ..} = &kind {
            read.copy_from_slice(&data_in);
        }

        debug!("SPI transfer handle: {} write: {:02x?} read: {:02x?}", handle, write, read);

        let op = Kind::SpiTransfer{handle, data_out: write.to_vec(), data_in: read.to_vec()};
        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;
        
        Ok(())
    }

}
