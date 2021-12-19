
use log::debug;

use crate::api::{Gpio, Error};
use super::{MockCtx, Op, Kind};

impl Gpio for MockCtx {
    fn init(&mut self, port: u32, pin: u32, output: bool) -> Result<i32, Error> {
        debug!("Configuring GPIO port: {} pin: {} (mode: {})", port, pin, output);

        let op = Kind::GpioInit{port, pin, output};
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

    fn set(&mut self, handle: i32, state: embedded_hal::digital::PinState) -> Result<(), Error> {
        let Op{kind, ..} = &self.expected[self.index];

        debug!("GPIO set handle: {} value: {:?}", handle, state);

        let op = Kind::GpioSet{handle, state: state.into()};
        assert_eq!(&op, kind);

        self.actual.push(op);
        self.index += 1;

        Ok(())
    }

    fn get(&mut self, handle: i32) -> Result<embedded_hal::digital::PinState, Error> {
        let Op{kind, ..} = &self.expected[self.index];

        let state = if let Kind::GpioGet{state, ..} = kind {
            state.clone().into()
        } else {
            embedded_hal::digital::PinState::Low
        };

        debug!("GPIO get handle: {} value: {:?}", handle, state);

        let op = Kind::GpioGet{handle, state: state.into()};
        assert_eq!(&op, kind);

        self.actual.push(op);
        self.index += 1;

        Ok(state)
    }
}
