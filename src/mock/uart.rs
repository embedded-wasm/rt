//! Mock UART driver implementation

use std::sync::{Arc, Mutex};

use log::debug;

use wasm_embedded_spec::{Error, Uart};
use super::{Inner, Op, Kind};

pub struct MockUart {
    inner: Arc<Mutex<Inner>>,
}

impl MockUart {
    pub(crate)  const fn new(inner: Arc<Mutex<Inner>>) -> Self {
        Self { inner }
    }
}

impl Uart for MockUart {
    fn init(&mut self, port: u32, baud: u32, tx: i32, rx: i32) -> Result<i32, Error> {
        let mut inner = self.inner.lock().unwrap();

        debug!("Opening UART port: {} (baud: {} tx: {} rx: {})", port, baud, tx, rx);

        let op = Kind::UartInit{port, baud, tx, rx};
        let Op{kind, res} = inner.expected[inner.index].clone();

        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;

        Ok(res)
    }

    fn deinit(&mut self, handle: i32) -> Result<(), Error> {
        let mut inner = self.inner.lock().unwrap();

        debug!("Closing UART handle: {}", handle);
        let op = Kind::UartDeinit{handle};
        let Op{kind, ..} = inner.expected[inner.index].clone();

        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;

        Ok(())
    }

    fn write(&mut self, handle: i32, flags: u32, data: &[u8]) -> Result<(), Error> {
        let mut inner = self.inner.lock().unwrap();

        let op = Kind::UartWrite{handle, flags, data_out: data.to_vec()};
        let Op{kind, ..} = inner.expected[inner.index].clone();

        debug!("UART write handle: {} flags: {} data: {:02x?}", handle, flags, data);

        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;

        Ok(())
    }

    fn read(&mut self, handle: i32, flags: u32, buff: &mut [u8]) -> Result<(), Error> {
        let mut inner = self.inner.lock().unwrap();

        let Op{kind, ..} = &inner.expected[inner.index];

        debug!("UART read handle: {} flags: {}", handle, flags);

        if let Kind::UartRead{data_in, ..} = kind {
            buff.copy_from_slice(&data_in);
        }

        let op = Kind::UartRead{handle, flags, data_in: buff.to_vec()};
        assert_eq!(&op, kind);

        inner.actual.push(op);
        inner.index += 1;

        Ok(())
    }
}
