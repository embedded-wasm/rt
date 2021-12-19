
use structopt::StructOpt;
use strum_macros::{Display, EnumString, EnumVariantNames};
use log::{LevelFilter, debug};

#[cfg(feature="hal-mock")]
use wasm_embedded_rt::mock::MockCtx;

#[cfg(feature="hal-linux")]
use wasm_embedded_rt::linux::LinuxCtx;

#[derive(Clone, PartialEq, Debug, StructOpt)]
struct Options {
    /// Operating mode
    #[structopt(long, default_value="dynamic")]
    mode: Mode,

    /// Runtime
    #[structopt(long, default_value="wasmtime")]
    runtime: Runtime,

    /// Configuration file (toml)
    #[structopt(long)]
    config: Option<String>,

    /// WASM binary to execute
    #[structopt()]
    bin: String,

    #[structopt(long = "log-level", default_value = "info")]
    /// Configure app logging levels (warn, info, debug, trace)
    pub log_level: LevelFilter,
}

/// Runtime mode
#[derive(Clone, PartialEq, Debug, StructOpt)]
#[derive(Display, EnumVariantNames, EnumString)]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum Mode {
    Mock,
    Linux,
}

impl Default for Mode {
    fn default() -> Self {
        #[cfg(feature="hal-linux")]
        return Mode::Linux;

        #[cfg(not(feature="hal-linux"))]
        return Mode::Mock;
    }
}

#[derive(Clone, PartialEq, Debug, StructOpt)]
#[derive(Display, EnumVariantNames, EnumString)]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum Runtime {
    Wasmtime,
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


fn main() -> Result<(), anyhow::Error> {
    // Load options
    let opts = Options::from_args();

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


    match (&opts.runtime, &opts.mode) {
        #[cfg(all(feature="rt-wasmtime", feature="hal-mock"))]
        (Runtime::Wasmtime, Mode::Mock) => {
            // Load mock configuration
            let cfg = match &opts.config {
                Some(c) => c,
                None => return Err(anyhow::anyhow!("mock mode requires --config file")),
            };
            let ctx = MockCtx::load(&cfg)?;

            wasm_embedded_rt::wasmtime::run(ctx, &bin)?;
        },
        #[cfg(all(feature="rt-wasmtime", feature="hal-linux"))]
        (Runtime::Wasmtime, Mode::Linux) => {
            // Load linux configuration
            // TODO: config files?
            let ctx = LinuxCtx::new();

            wasm_embedded_rt::wasmtime::run(ctx, &bin)?;
        },
        #[cfg(all(feature="rt-wasm3", feature="hal-mock"))]
        (Runtime::Wasm3, Mode::Mock) => {
            // Load mock configuration
            let cfg = match &opts.config {
                Some(c) => c,
                None => return Err(anyhow::anyhow!("mock mode requires --config file")),
            };
            let mut ctx = MockCtx::load(&cfg)?;
            
            let mut rt = wasm_embedded_rt::wasm3::Wasm3Runtime::new(&bin)?;
            rt.bind_all(&mut ctx)?;
            rt.run()?;
        },
        #[cfg(all(feature="rt-wasm3", feature="hal-linux"))]
        (Runtime::Wasm3, Mode::Linux) => {
            // Load linux configuration
            // TODO: config files?
            let mut ctx = LinuxCtx::new();

            let mut rt = wasm_embedded_rt::wasm3::Wasm3Runtime::new(&bin)?;
            rt.bind_all(&mut ctx)?;
            rt.run()?;
        },
        _ => {
            return Err(anyhow::anyhow!("Runtime was not built with {}:{} features", opts.runtime, opts.mode))
        },
    }

    Ok(())
}
