
//mod i2c_api;
//mod spi_api;
//mod gpio_api;

pub use wasm_embedded_spec::{
    i2c::I2c,
    spi::Spi,
    uart::Uart,
    gpio::Gpio,
    Error,
};

/// Engine trait combines API traits for convenience
pub trait Engine: I2c + Spi + Uart + Gpio + 'static {}

impl <T> Engine for T where
    T: I2c + Spi + Uart + Gpio + 'static {}
