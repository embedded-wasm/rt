

use embedded_hal::spi::blocking::Operation;
use log::debug;

use crate::api::{Spi, Error};
use super::{MockCtx, Op, Kind};


impl Spi for MockCtx {
    fn init(&mut self, port: u32, baud: u32, mosi: i32, miso: i32, sck: i32, cs: i32) -> Result<i32, Error> {
        debug!("Opening SPI port: {} (baud: {} mosi: {} miso: {} sck: {} cs: {})", port, baud, mosi, miso, sck, cs);

        let op = Kind::SpiInit{port, baud, mosi, miso, sck, cs};
        let Op{kind, res} = &self.expected[self.index];

        assert_eq!(&op, kind);

        self.actual.push(op);
        self.index += 1;

        Ok(*res)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        debug!("Closing SPI handle: {}", handle);
        let op = Kind::SpiDeinit{handle};
        let Op{kind, ..} = &self.expected[self.index];

        assert_eq!(&op, kind);

        self.actual.push(op);
        self.index += 1;

        Ok(())
    }

    fn write<'a>(&mut self, handle: i32, data: &[u8]) -> Result<(), Error> {
        let op = Kind::SpiWrite{handle, data_out: data.to_vec()};
        let Op{kind, ..} = &self.expected[self.index];

        debug!("SPI write handle: {} data: {:02x?}", handle, data);

        assert_eq!(&op, kind);

        self.actual.push(op);
        self.index += 1;

        Ok(())
    }

    fn transfer<'a>(&mut self, handle: i32, data: &mut [u8]) -> Result<(), Error> {
        let Op{kind, ..} = &self.expected[self.index];

        let d = data.to_vec();

        if let Kind::SpiTransfer{data_in, ..} = kind {
            data.copy_from_slice(&data_in);
        }

        debug!("SPI transfer handle: {} write: {:02x?} read: {:02x?}", handle, d, data);

        let op = Kind::SpiTransfer{handle, data_out: d, data_in: data.to_vec()};
        assert_eq!(&op, kind);

        self.actual.push(op);
        self.index += 1;
        
        Ok(())
    }

    fn exec<'a>(&mut self, _handle: i32, _ops: &[Operation<u8>]) -> Result<(), Error> {
        todo!()
    }
}
