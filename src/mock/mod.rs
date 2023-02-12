//! Mock driver implementation for application and API testing

use std::{vec, vec::Vec, sync::{Arc, Mutex}};

use serde::{Serialize, Deserialize};
use log::debug;

use wasm_embedded_spec::Engine;

mod spi;
pub use spi::MockSpi;
mod i2c;
pub use i2c::MockI2c;
mod uart;
pub use uart::MockUart;
mod gpio;
pub use gpio::MockGpio;

mod ops;
pub use ops::{Op, Kind};

/// Mock configuration
#[derive(Clone, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub struct MockConfig {
    pub ops: Vec<Op>,
}

/// Mock driver context
pub struct MockCtx {
    inner: Arc<Mutex<Inner>>,

    gpio: MockGpio,
    i2c: MockI2c,
    spi: MockSpi,
    uart: MockUart,
}

/// Inner storage for mock drivers
pub(crate) struct Inner {
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

        let inner = Arc::new(Mutex::new(Inner{
            expected: f.ops,
            actual: Vec::new(),
            index: 0,
        }));

        Ok(Self{
            inner: inner.clone(),
            gpio: MockGpio::new(inner.clone()),
            i2c: MockI2c::new(inner.clone()),
            spi: MockSpi::new(inner.clone()),
            uart: MockUart::new(inner.clone()),
        })
    }
}

impl Engine for MockCtx {
    type Gpio = MockGpio;

    type I2c = MockI2c;

    type Spi = MockSpi;

    type Uart = MockUart;

    fn gpio(&mut self) -> Option<&mut Self::Gpio> { return Some(&mut self.gpio) }

    fn i2c(&mut self) -> Option<&mut Self::I2c> { return Some(&mut self.i2c) }

    fn spi(&mut self) -> Option<&mut Self::Spi> { return Some(&mut self.spi) }

    fn uart(&mut self) -> Option<&mut Self::Uart> { return Some(&mut self.uart) }
}

impl Drop for MockCtx {
    fn drop(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        let ex: Vec<Kind> = inner.expected.iter().map(|v| v.kind.clone()).collect();
        assert_eq!(&ex, &inner.actual, "Mock result mismatch");
    }
}
