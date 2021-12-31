/// Wasmtime runtime support

// TODO: move this out to rt_wasmtime

use wasm_embedded_spec::api::UserErrorConversion;
use crate::api::Engine;

/// Run the provided application using the wasmtime runtime
pub fn run<E: Engine + UserErrorConversion>(manager: E, bin: &[u8]) -> anyhow::Result<()> {
    use wasmtime::*;

    // Create new engine and linker
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);

    // Create store for variables / instances / etc.
    let mut store = Store::new(&engine, manager);

    // Bind APIs
    wasmtime_wasi::add_to_linker(&mut linker, |ctx: &mut E| ctx.wasi() )?;

    // Bind embedded APIs
    wasm_embedded_spec::api::spi::add_to_linker(&mut linker, |m: &mut E| m )?;
    wasm_embedded_spec::api::i2c::add_to_linker(&mut linker, |m: &mut E| m )?;
    wasm_embedded_spec::api::gpio::add_to_linker(&mut linker, |m: &mut E| m )?;

    // Load module from file
    let module = Module::from_binary(&engine, bin)?;

    // Bind module and run default fn
    linker.module(&mut store, "", &module)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), (), _>(&store)?
        .call(&mut store, ())?;

    Ok(())
}
