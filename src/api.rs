
//mod i2c_api;
//mod spi_api;
//mod gpio_api;

pub use wasm_embedded_spec::i2c::I2c;
pub use wasm_embedded_spec::spi::Spi;
pub use wasm_embedded_spec::gpio::Gpio;
pub use wasm_embedded_spec::Error;

/// Engine trait combines API traits for convenience
pub trait Engine: I2c + Spi + Gpio + 'static {}

impl <T> Engine for T where
    T: I2c + Spi + Gpio + 'static {}
