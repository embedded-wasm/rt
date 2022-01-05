// Signal no_std if std feature disabled
// (a hack to keep wiggle working in std contexts)
#![cfg_attr(not(feature="std"), no_std)]

pub mod api;

#[cfg(feature="hal-mock")]
pub mod mock;

#[cfg(feature="hal-linux")]
pub mod linux;

#[cfg(feature="rt-wasm3")]
pub use wasm_embedded_rt_wasm3::{self as rt_wasm3};

#[cfg(feature="rt-wasmtime")]
pub use wasm_embedded_rt_wasmtime::{self as rt_wasmtime};

#[derive(Clone, PartialEq, Debug)]
pub struct Config {
    
}
