use std::io;
use std::process::ExitCode;

use wasmtime::StoreLimits;
use wasmtime::StoreLimitsBuilder;

use wasmtime::Instance;
use wasmtime::Module;
use wasmtime::TypedFunc;

use rs_cpuinfo4wasm::State;
use rs_cpuinfo4wasm::WasmStore;

use rs_cpuinfo4wasm::WasmEngine;
use rs_cpuinfo4wasm::WasmLinker;

fn sub() -> Result<(), io::Error> {
    let bldr = StoreLimitsBuilder::new().memory_size(16777216).instances(1);
    let lmt: StoreLimits = bldr.build();

    let mut state: State = State::from_limits(lmt);
    state.refresh_cpu_all();

    let wat_path: String = std::env::var("ENV_WAT_PATH").unwrap_or_default();
    println!("wat path: {wat_path}");

    let wat_bytes: Vec<u8> = std::fs::read(wat_path)?;

    let we: WasmEngine = WasmEngine::default();
    let wl: WasmLinker = we
        .create_linker()
        .wrap_cpu_count()?
        .wrap_cpu_usage_pcnt_by_index()?
        .wrap_cpu_freq_mhz_by_index()?
        .wrap_print_cpu_count()?
        .wrap_print_cpu_ix()?
        .wrap_print_cpu_usage_pcnt_by_index()?
        .wrap_print_cpu_freq_mhz_by_index()?
        .wrap_print_cpu_brand_by_index()?
        .wrap_print_cpu_name_by_index()?
        .wrap_print_cpu_vendor_id_by_index()?
        .wrap_print_f32()?
        .wrap_print_u64()?;

    let wm: Module = we.wat2module(&wat_bytes)?;

    // add sleep when the duration between the 1st call is too short
    state.refresh_cpu_all();

    let mut wstore: WasmStore = we.state2store(state);
    wstore.apply_limit();

    let wi: Instance = wl.to_instance(&mut wstore, &wm)?;

    let io_main: TypedFunc<(), ()> = wi
        .get_typed_func(wstore.as_store_mut(), "io_main")
        .map_err(io::Error::other)?;

    io_main
        .call(wstore.as_store_mut(), ())
        .map_err(io::Error::other)?;

    Ok(())
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
