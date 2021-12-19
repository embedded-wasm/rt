
use log::debug;

use crate::api::{I2c, Error};
use super::{MockCtx, Op, Kind};

impl I2c for MockCtx {
    fn init(&mut self, port: u32, baud: u32, sda: i32, scl: i32) -> Result<i32, Error> {
        debug!("Opening I2C port: {} (baud: {} sda: {} scl: {})", port, baud, sda, scl);

        let op = Kind::I2cInit{port, baud, sda, scl};
        let Op{kind, res} = &self.expected[self.index];

        assert_eq!(&op, kind);

        self.actual.push(op);
        self.index += 1;

        Ok(*res)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        debug!("Closing I2C handle: {}", handle);
        let op = Kind::I2cDeinit{handle};
        let Op{kind, ..} = &self.expected[self.index];

        assert_eq!(&op, kind);

        self.actual.push(op);
        self.index += 1;

        Ok(())
    }

    fn write(&mut self, handle: i32, addr: u16, data: &[u8]) -> Result<(), Error> {
        let op = Kind::I2cWrite{handle, addr, data_out: data.to_vec()};
        let Op{kind, ..} = &self.expected[self.index];

        debug!("I2C write handle: {} addr: {} data: {:02x?}", handle, addr, data);

        assert_eq!(&op, kind);

        self.actual.push(op);
        self.index += 1;

        Ok(())
    }

    fn read(&mut self, handle: i32, addr: u16, buff: &mut [u8]) -> Result<(), Error> {
        let Op{kind, ..} = &self.expected[self.index];

        debug!("I2C read handle: {} addr: {}", handle, addr);

        if let Kind::I2cRead{data_in, ..} = kind {
            buff.copy_from_slice(&data_in);
        }

        let op = Kind::I2cRead{handle, addr, data_in: buff.to_vec()};
        assert_eq!(&op, kind);

        self.actual.push(op);
        self.index += 1;

        Ok(())
    }

    fn write_read(&mut self, handle: i32, addr: u16, data: &[u8], buff: &mut [u8]) -> Result<(), Error> {
        let Op{kind, ..} = &self.expected[self.index];

        if let Kind::I2cWriteRead{data_in, ..} = kind {
            buff.copy_from_slice(&data_in);
        }

        debug!("I2C write handle: {} addr: {} data: {:02x?} buff: {:02x?}", handle, addr, data, buff);

        let op = Kind::I2cWriteRead{handle, addr, data_out: data.to_vec(), data_in: buff.to_vec()};
        assert_eq!(&op, kind);

        self.actual.push(op);
        self.index += 1;
        
        Ok(())
    }
}
