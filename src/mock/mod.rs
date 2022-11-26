
use std::{vec, vec::Vec};

use serde::{Serialize, Deserialize};
use log::debug;

mod spi;
mod i2c;
mod uart;
mod gpio;

#[derive(Clone, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(tag="kind", rename_all="snake_case")]
pub enum Kind {
    I2cInit{
        port: u32,
        baud: u32,
        sda: i32,
        scl: i32,
    },
    I2cDeinit{
        handle: i32,
    },
    I2cWrite{
        handle: i32,
        addr: u16,
        data_out: Vec<u8>,
    },
    I2cRead{
        handle: i32,
        addr: u16,
        data_in: Vec<u8>,
    },
    I2cWriteRead{
        handle: i32,
        addr: u16,
        data_out: Vec<u8>,
        data_in: Vec<u8>,
    },
    SpiInit{
        port: u32,
        baud: u32,
        mosi: i32,
        miso: i32,
        sck: i32,
        cs: i32,
    },
    SpiDeinit{
        handle: i32,
    },
    SpiRead{
        handle: i32,
        data_in: Vec<u8>,
    },
    SpiWrite{
        handle: i32,
        data_out: Vec<u8>,
    },
    SpiTransfer{
        handle: i32,
        data_out: Vec<u8>,
        data_in: Vec<u8>,
    },
    UartInit{
        port: u32,
        baud: u32,
        tx: i32,
        rx: i32,
    },
    UartDeinit{
        handle: i32,
    },
    UartWrite{
        handle: i32,
        flags: u32,
        data_out: Vec<u8>,
    },
    UartRead{
        handle: i32,
        flags: u32,
        data_in: Vec<u8>,
    },
    GpioInit{
        port: i32,
        pin: i32,
        output: bool,
    },
    GpioDeinit{
        handle: i32,
    },
    GpioSet{
        handle: i32,
        state: PinState,
    },
    GpioGet{
        handle: i32,
        state: PinState,
    },
}

/// Mock configuration
#[derive(Clone, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub struct MockConfig {
    pub ops: Vec<Op>,
}

/// Mock operation
#[derive(Clone, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub struct Op {
    #[serde(flatten)]
    pub kind: Kind,
    pub res: i32,
}

pub struct MockCtx {
    expected: Vec<Op>,
    actual: Vec<Kind>,
    index: usize,
}

impl MockCtx {
    /// Load a new mock context
    pub fn load(config: &str) -> anyhow::Result<Self> {
        debug!("Loading mock config: {}", config);

        // Load expectations from config file
        let d = std::fs::read(config)?;
        let f: MockConfig = toml::from_slice(&d)?;

        debug!("Using expectations: {:?}", f);

        Ok(Self{
            expected: f.ops,
            actual: vec![],
            index: 0,
        })
    }
}

impl Drop for MockCtx {
    fn drop(&mut self) {
        let ex: Vec<Kind> = self.expected.iter().map(|v| v.kind.clone()).collect();
        assert_eq!(&ex, &self.actual, "Mock result mismatch");
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum PinState {
    High,
    Low,
}

impl From<PinState> for embedded_hal::digital::PinState {
    fn from(s: PinState) -> Self {
        use embedded_hal::digital::PinState::*;

        match s {
            PinState::High => High,
            PinState::Low => Low,
        }
    }
}


impl From<embedded_hal::digital::PinState> for PinState {
    fn from(s: embedded_hal::digital::PinState) -> Self {
        use embedded_hal::digital::PinState::*;

        match s {
            High => PinState::High,
            Low => PinState::Low,
        }
    }
}
