//! Mock I2C driver implementation

use std::sync::{Arc, Mutex};

use log::debug;

use wasm_embedded_spec::{Error, I2c};
use super::{Inner, Op, Kind};

pub struct MockI2c {
    inner: Arc<Mutex<Inner>>,
}

impl MockI2c {
    pub(crate)  const fn new(inner: Arc<Mutex<Inner>>) -> Self {
        Self { inner }
    }
}

impl I2c for MockI2c {
    fn init(&mut self, port: u32, baud: u32, sda: i32, scl: i32) -> Result<i32, Error> {
        let mut inner = self.inner.lock().unwrap();

        debug!("Opening I2C port: {} (baud: {} sda: {} scl: {})", port, baud, sda, scl);

        let op = Kind::I2cInit{port, baud, sda, scl};
        let Op{kind, res} = inner.expected[inner.index].clone();

        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;

        Ok(res)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        let mut inner = self.inner.lock().unwrap();

        debug!("Closing I2C handle: {}", handle);
        let op = Kind::I2cDeinit{handle};
        let kind = inner.expected[inner.index].kind.clone();

        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;

        Ok(())
    }

    fn write(&mut self, handle: i32, addr: u16, data: &[u8]) -> Result<(), Error> {
        let mut inner = self.inner.lock().unwrap();

        let op = Kind::I2cWrite{handle, addr, data_out: data.to_vec()};
        let kind = inner.expected[inner.index].kind.clone();

        debug!("I2C write handle: {} addr: {} data: {:02x?}", handle, addr, data);

        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;

        Ok(())
    }

    fn read(&mut self, handle: i32, addr: u16, buff: &mut [u8]) -> Result<(), Error> {
        let mut inner = self.inner.lock().unwrap();

        let kind = inner.expected[inner.index].kind.clone();

        debug!("I2C read handle: {} addr: {}", handle, addr);

        if let Kind::I2cRead{data_in, ..} = &kind {
            buff.copy_from_slice(&data_in);
        }

        let op = Kind::I2cRead{handle, addr, data_in: buff.to_vec()};
        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;

        Ok(())
    }

    fn write_read(&mut self, handle: i32, addr: u16, data: &[u8], buff: &mut [u8]) -> Result<(), Error> {
        let mut inner = self.inner.lock().unwrap();

        let kind = inner.expected[inner.index].kind.clone();

        if let Kind::I2cWriteRead{data_in, ..} = &kind {
            buff.copy_from_slice(&data_in);
        }

        debug!("I2C write handle: {} addr: {} data: {:02x?} buff: {:02x?}", handle, addr, data, buff);

        let op = Kind::I2cWriteRead{handle, addr, data_out: data.to_vec(), data_in: buff.to_vec()};
        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;
        
        Ok(())
    }
}
