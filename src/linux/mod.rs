

use wasm_embedded_spec::Engine;

mod i2c;
pub use i2c::I2cDriver;

mod spi;
pub use spi::SpiDriver;

mod gpio;
pub use gpio::GpioDriver;

mod uart;
pub use uart::UartDriver;

/// Linux embedded wasm driver context
pub struct LinuxCtx {
    pub(super) spi: SpiDriver,
    pub(super) i2c: I2cDriver,
    pub(super) uart: UartDriver,
    pub(super) gpio: GpioDriver,
}

impl LinuxCtx {
    /// Create a new linux driver context
    pub fn new() -> Self {
        Self{
            spi: SpiDriver::new(),
            i2c: I2cDriver::new(),
            uart: UartDriver::new(),
            gpio: GpioDriver::new(),
        }
    }
}

impl Engine for LinuxCtx {
    type Gpio = GpioDriver;

    type I2c = I2cDriver;

    type Spi = SpiDriver;

    type Uart = UartDriver;

    fn gpio(&mut self) -> Option<&mut Self::Gpio> {
        Some(&mut self.gpio)
    }

    fn i2c(&mut self) -> Option<&mut Self::I2c> {
        Some(&mut self.i2c)
    }

    fn spi(&mut self) -> Option<&mut Self::Spi> {
        Some(&mut self.spi)
    }

    fn uart(&mut self) -> Option<&mut Self::Uart> {
        Some(&mut self.uart)
    }
}
