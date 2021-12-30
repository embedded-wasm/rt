
use std::{vec, vec::Vec};

use serde::{Serialize, Deserialize};
use log::debug;

#[cfg(feature = "rt-wasmtime")]
use wasmtime_wasi::{WasiCtx, sync::WasiCtxBuilder};

use crate::api::{Engine};

mod spi;
mod i2c;
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
    SpiWrite{
        handle: i32,
        data_out: Vec<u8>,
    },
    SpiTransfer{
        handle: i32,
        data_out: Vec<u8>,
        data_in: Vec<u8>,
    },
    GpioInit{
        port: u32,
        pin: u32,
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
    #[cfg(feature = "rt-wasmtime")]
    wasi: WasiCtx,
}

impl MockCtx {
    /// Load a new mock context
    pub fn load(config: &str) -> anyhow::Result<Self> {
        debug!("Loading mock config: {}", config);

        // Load expectations from config file
        let d = std::fs::read(config)?;
        let f: MockConfig = toml::from_slice(&d)?;

        debug!("Using expectations: {:?}", f);

        // Setup WASI ctx
        #[cfg(feature = "rt-wasmtime")]
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args().unwrap()
            .build();

        Ok(Self{
            expected: f.ops,
            actual: vec![],
            index: 0,
            #[cfg(feature = "rt-wasmtime")]
            wasi,
        })
    }
}

impl Engine for MockCtx {
    #[cfg(feature = "rt-wasmtime")]
    fn wasi(&mut self) -> &mut WasiCtx { 
        &mut self.wasi    
    }
}

impl Drop for MockCtx {
    fn drop(&mut self) {
        let ex: Vec<Kind> = self.expected.iter().map(|v| v.kind.clone()).collect();
        assert_eq!(&ex, &self.actual, "Mock result mismatch");
    }
}

#[cfg(feature = "rt-wasmtime")]
impl wasm_embedded_spec::api::spi::UserErrorConversion for MockCtx {
    fn errno_from_error(&mut self, _e: crate::api::Error) -> Result<wasm_embedded_spec::api::types::Errno, wiggle::Trap> {
        // TODO: convert errors here
        todo!()
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
