
use std::collections::HashMap;

use linux_embedded_hal::{I2cdev, Spidev, SysfsPin};

pub mod i2c;
pub mod spi;
pub mod gpio;

/// Linux wasm-embedded context
pub struct LinuxCtx {
    pub(super) count: i32,

    pub(super) spi: HashMap<i32, Spidev>,
    pub(super) i2c: HashMap<i32, I2cdev>,
    pub(super) gpio: HashMap<i32, SysfsPin>,
}


impl LinuxCtx {
    pub fn new() -> Self {
        Self{
            count: 0,
            spi: HashMap::new(),
            i2c: HashMap::new(),
            gpio: HashMap::new(),
        }
    }
}
