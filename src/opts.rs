
use strum::{Display, EnumString, EnumVariantNames};

/// WASM Server Configuration
#[derive(Clone, PartialEq, Default, Debug)]
pub struct Config {
    /// Runtime for WASM execution
    pub runtime: Runtime,
    /// Engine providing embedded-wasm APIs
    pub engine: Engine,
}


/// Server runtime selector
#[derive(Clone, PartialEq, Debug, clap::ValueEnum)]
#[derive(Display, EnumVariantNames, EnumString)]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum Runtime {
    /// Wasmtime based runtime
    Wasmtime,
    /// Wasm3 based runtime
    Wasm3,
}

impl Default for Runtime {
    fn default() -> Self {
        #[cfg(feature="rt-wasmtime")]
        return Runtime::Wasmtime;

        #[cfg(not(feature="rt-wasmtime"))]
        return Runtime::Wasm3;
    }
}

/// Server engine selector
#[derive(Clone, PartialEq, Debug, clap::ValueEnum)]
#[derive(Display, EnumVariantNames, EnumString)]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum Engine {
    /// Mock provides mocked drivers for testing
    Mock,
    /// Linux provides linux-embedded-hal backed drivers
    Linux,
}

impl Default for Engine {
    fn default() -> Self {
        #[cfg(feature="hal-linux")]
        return Engine::Linux;

        #[cfg(not(feature="hal-linux"))]
        return Engine::Mock;
    }
}
