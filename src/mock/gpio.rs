//! Mock GPIO driver implementation

use std::sync::{Arc, Mutex};

use log::debug;

use wasm_embedded_spec::{Error, Gpio};

use super::{Inner, Op, Kind};

pub struct MockGpio {
    inner: Arc<Mutex<Inner>>,
}

impl MockGpio {
    pub(crate) const fn new(inner: Arc<Mutex<Inner>>) -> Self {
        Self { inner }
    }
}

impl Gpio for MockGpio {
    fn init(&mut self, port: i32, pin: i32, output: bool) -> Result<i32, Error> {
        let mut inner = self.inner.lock().unwrap();

        debug!("Configuring GPIO port: {} pin: {} (mode: {})", port, pin, output);

        let op = Kind::GpioInit{port, pin, output};
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
        let Op{kind, ..} = inner.expected[inner.index].clone();

        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;

        Ok(())
    }

    fn set(&mut self, handle: i32, state: embedded_hal::digital::PinState) -> Result<(), Error> {
        let mut inner = self.inner.lock().unwrap();

        let Op{kind, ..} = inner.expected[inner.index].clone();

        debug!("GPIO set handle: {} value: {:?}", handle, state);

        let op = Kind::GpioSet{handle, state: state.into()};
        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;

        Ok(())
    }

    fn get(&mut self, handle: i32) -> Result<embedded_hal::digital::PinState, Error> {
        let mut inner = self.inner.lock().unwrap();

        let Op{kind, ..} = inner.expected[inner.index].clone();

        let state = if let Kind::GpioGet{state, ..} = kind {
            state.clone().into()
        } else {
            embedded_hal::digital::PinState::Low
        };

        debug!("GPIO get handle: {} value: {:?}", handle, state);

        let op = Kind::GpioGet{handle, state: state.into()};
        assert_eq!(op, kind);

        inner.actual.push(op);
        inner.index += 1;

        Ok(state)
    }
}
