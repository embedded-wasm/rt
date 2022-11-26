
use log::debug;

use crate::api::{Uart, Error};
use super::{MockCtx, Op, Kind};

impl Uart for MockCtx {
    fn init(&mut self, port: u32, baud: u32, tx: i32, rx: i32) -> Result<i32, Error> {
        debug!("Opening UART port: {} (baud: {} tx: {} rx: {})", port, baud, tx, rx);

        let op = Kind::UartInit{port, baud, tx, rx};
        let Op{kind, res} = &self.expected[self.index];

        assert_eq!(&op, kind);

        self.actual.push(op);
        self.index += 1;

        Ok(*res)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        debug!("Closing UART handle: {}", handle);
        let op = Kind::UartDeinit{handle};
        let Op{kind, ..} = &self.expected[self.index];

        assert_eq!(&op, kind);

        self.actual.push(op);
        self.index += 1;

        Ok(())
    }

    fn write(&mut self, handle: i32, flags: u32, data: &[u8]) -> Result<(), Error> {
        let op = Kind::UartWrite{handle, flags, data_out: data.to_vec()};
        let Op{kind, ..} = &self.expected[self.index];

        debug!("UART write handle: {} flags: {} data: {:02x?}", handle, flags, data);

        assert_eq!(&op, kind);

        self.actual.push(op);
        self.index += 1;

        Ok(())
    }

    fn read(&mut self, handle: i32, flags: u32, buff: &mut [u8]) -> Result<(), Error> {
        let Op{kind, ..} = &self.expected[self.index];

        debug!("UART read handle: {} flags: {}", handle, flags);

        if let Kind::UartRead{data_in, ..} = kind {
            buff.copy_from_slice(&data_in);
        }

        let op = Kind::UartRead{handle, flags, data_in: buff.to_vec()};
        assert_eq!(&op, kind);

        self.actual.push(op);
        self.index += 1;

        Ok(())
    }
}
