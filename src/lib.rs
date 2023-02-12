// Signal no_std if std feature disabled
// (a hack to keep wiggle working in std contexts)
#![cfg_attr(not(feature="std"), no_std)]

#![feature(return_position_impl_trait_in_trait)]

pub use wasm_embedded_spec::{
    Engine,
    Error,
};

pub mod opts;

#[cfg(feature="hal-mock")]
pub mod mock;

#[cfg(feature="hal-linux")]
pub mod linux;

#[cfg(feature="rt-wasm3")]
pub use wasm_embedded_rt_wasm3::{self as rt_wasm3};

#[cfg(feature="rt-wasmtime")]
pub use wasm_embedded_rt_wasmtime::{self as rt_wasmtime};

/// WASM server
pub struct Server<E: Engine> {
    config: opts::Config,
    runtime: RuntimeCtx<E>,
}

impl <E: Engine> Server<E> {
    /// Create new server instance
    pub fn new(config: opts::Config) -> Self {
        Self {
            config,
            runtime: RuntimeCtx::None,
        }
    }

    /// Load and execute a provided wasm binary
    pub fn exec(&mut self, bin: &[u8]) -> Result<(), ()> {
        Ok(())
    }

    /// Start the server listening for remote operations
    pub fn serve(&mut self, port: u16) -> Result<(), ()> {
        Ok(())
    }
}

/// Storage for runtime context
enum RuntimeCtx<E: Engine> {
    /// No runtime loaded
    None,

    /// WASM3 runtime loaded
    #[cfg(feature="rt-wasm3")]
    Wasm3(rt_wasm3::Wasm3Runtime),

    /// Wasmtime runtime loaded
    #[cfg(feature="rt-wasmtime")]
    Wasmtime(rt_wasmtime::WasmtimeRuntime<E>),
}
