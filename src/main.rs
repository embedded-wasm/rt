
use clap::Parser;
use strum::{Display, EnumString, EnumVariantNames};
use log::{LevelFilter, debug};

use wasm_embedded_rt::opts::*;

#[cfg(feature="hal-mock")]
use wasm_embedded_rt::mock::MockCtx;

#[cfg(feature="hal-linux")]
use wasm_embedded_rt::linux::LinuxCtx;

#[cfg(feature="rt-wasm3")]
use wasm_embedded_rt_wasm3::{Wasm3Runtime};

#[cfg(feature="rt-wasmtime")]
use wasm_embedded_rt_wasmtime::{WasmtimeRuntime};


#[derive(Clone, PartialEq, Debug, Parser)]
struct Args {
    /// Backing engine
    #[clap(long, value_enum)]
    engine: Engine,

    /// WASM Runtime
    #[clap(long, value_enum)]
    runtime: Runtime,

    /// Optional configuration file
    #[clap(long)]
    config: Option<String>,

    /// WASM binary to execute
    #[clap()]
    bin: String,

    #[clap(long = "log-level", default_value = "info")]
    /// Configure app logging levels (warn, info, debug, trace)
    pub log_level: LevelFilter,
}

/// Mode for runtime execution
#[derive(Clone, PartialEq, Debug, Parser)]
pub enum Mode {
    /// Execute a provided WASM binary
    Exec{
        /// WASM binary to execute
        #[clap()]
        bin: String,
    },
    /// Start remote WASM server
    Serve{
        /// Port for remote server binding
        #[clap(long, default_value = "14321")]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Load options
    let opts = Args::parse();

    // Setup logging
    let log_config = simplelog::ConfigBuilder::new()
        .add_filter_ignore_str("cranelift_codegen")
        .add_filter_ignore_str("regalloc")
        .add_filter_ignore_str("cranelift_wasm")
        .build();
    let _ = simplelog::SimpleLogger::init(opts.log_level, log_config);

    // Load WASM binary
    debug!("Loading WASM binary: {}", opts.bin);
    let bin = std::fs::read(opts.bin)?;

    #[allow(unreachable_patterns)]
    match (&opts.runtime, &opts.engine) {
        #[cfg(all(feature="rt-wasmtime", feature="hal-mock"))]
        (Runtime::Wasmtime, Engine::Mock) => {
            // Load mock configuration
            let cfg = match &opts.config {
                Some(c) => c,
                None => return Err(anyhow::anyhow!("mock mode requires --config file")),
            };
            let ctx = MockCtx::load(&cfg)?;
            let mut rt = WasmtimeRuntime::new(ctx, &bin)?;

            rt.run()?;
        },
        #[cfg(all(feature="rt-wasmtime", feature="hal-linux"))]
        (Runtime::Wasmtime, Engine::Linux) => {
            // Load linux configuration
            // TODO: config files?
            let ctx = LinuxCtx::new();
            let mut rt = WasmtimeRuntime::new(ctx, &bin)?;

            rt.run()?;
        },
        #[cfg(all(feature="rt-wasm3", feature="hal-mock"))]
        (Runtime::Wasm3, Engine::Mock) => {
            // Load mock configuration
            let cfg = match &opts.config {
                Some(c) => c,
                None => return Err(anyhow::anyhow!("mock mode requires --config file")),
            };
            let mut ctx = MockCtx::load(&cfg)?;
            let mut rt = Wasm3Runtime::new(&mut ctx, &bin)?;
            
            // TODO: bind drivers

            rt.run()?;
        },
        #[cfg(all(feature="rt-wasm3", feature="hal-linux"))]
        (Runtime::Wasm3, Engine::Linux) => {
            // Load linux configuration
            // TODO: config files?
            let mut ctx = LinuxCtx::new();
            let mut rt = Wasm3Runtime::new(&mut ctx, &bin)?;
            rt.run()?;
        },
        _ => {
            return Err(anyhow::anyhow!("Runtime was not built with {}:{} enabled", opts.runtime, opts.engine))
        },
    }

    Ok(())
}
