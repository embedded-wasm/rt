
use std::collections::HashMap;

use linux_embedded_hal::{I2cdev, Spidev, SysfsPin};

#[cfg(feature = "rt-wasmtime")]
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

use crate::api::{Engine};

pub mod i2c;
pub mod spi;
pub mod gpio;

/// Linux wasm-embedded context
pub struct LinuxCtx {
    #[cfg(feature = "rt-wasmtime")]
    pub(crate) wasi: WasiCtx,

    pub(super) count: i32,

    pub(super) spi: HashMap<i32, Spidev>,
    pub(super) i2c: HashMap<i32, I2cdev>,
    pub(super) gpio: HashMap<i32, SysfsPin>,
}


impl LinuxCtx {
    pub fn new() -> Self {
        #[cfg(feature = "rt-wasmtime")]
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args().unwrap()
            .build();

        Self{
            #[cfg(feature = "rt-wasmtime")]
            wasi,
            count: 0,
            spi: HashMap::new(),
            i2c: HashMap::new(),
            gpio: HashMap::new(),
        }
    }

    #[cfg(feature = "rt-wasmtime")]
    pub fn bind(linker: &mut wasmtime::Linker<Self>) -> anyhow::Result<()> {
        wasmtime_wasi::add_to_linker(linker, |ctx: &mut Self| &mut ctx.wasi )?;

        // Bind embedded APIs
        wasm_embedded_spec::api::spi::add_to_linker(linker, |m: &mut Self| m )?;
        wasm_embedded_spec::api::i2c::add_to_linker(linker, |m: &mut Self| m )?;
        wasm_embedded_spec::api::gpio::add_to_linker(linker, |m: &mut Self| m )?;
        
        Ok(())
    }
}

impl Engine for LinuxCtx {
    #[cfg(feature = "rt-wasmtime")]
    fn wasi(&mut self) -> &mut WasiCtx { 
        &mut self.wasi    
    }
}

#[cfg(feature = "wiggle")]
impl wasm_embedded_spec::api::spi::UserErrorConversion for LinuxCtx {
    fn errno_from_error(&mut self, e: crate::api::Error) -> Result<wasm_embedded_spec::api::types::Errno, wiggle::Trap> {
        use wasm_embedded_spec::Error::*;
        use wasm_embedded_spec::api::Errno;

        let r = match e {
            InvalidArg => Errno::InvalidArg,
            Unexpected => Errno::Unexpected,
            Failed => Errno::Failed,
            NoDevice => Errno::NoDevice,
        };

        Ok(r)
    }
}
