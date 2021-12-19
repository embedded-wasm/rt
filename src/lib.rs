// Signal no_std if std feature disabled
// (a hack to keep wiggle working in std contexts)
#![cfg_attr(not(feature="std"), no_std)]

pub mod api;

#[cfg(feature="hal-mock")]
pub mod mock;

#[cfg(feature="hal-linux")]
pub mod linux;

#[cfg(feature="rt-wasm3")]
pub mod wasm3;

#[cfg(feature="rt-wasmtime")]
pub mod wasmtime;

#[derive(Clone, PartialEq, Debug)]
pub struct Config {
    
}
