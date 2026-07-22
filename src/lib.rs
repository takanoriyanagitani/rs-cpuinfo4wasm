use std::io;

use sysinfo::Cpu;
use sysinfo::System;

use wasmtime::Caller;
use wasmtime::Engine;
use wasmtime::Instance;
use wasmtime::Linker;
use wasmtime::Module;
use wasmtime::Store;

use wasmtime::StoreLimits;

#[derive(Default)]
pub struct CpuInfo {
    pub brand: String,
    pub usage_pcnt: f32,
    pub freq_mhz: u64,
    pub name: String,
    pub vendor_id: String,
}

impl From<&sysinfo::Cpu> for CpuInfo {
    fn from(c: &sysinfo::Cpu) -> Self {
        Self {
            brand: c.brand().into(),
            usage_pcnt: c.cpu_usage(),
            freq_mhz: c.frequency(),
            name: c.name().into(),
            vendor_id: c.vendor_id().into(),
        }
    }
}

pub struct State {
    sys: System,
    lmt: StoreLimits,
    cpus: Vec<CpuInfo>,
}

impl State {
    pub fn from_limits(lmt: StoreLimits) -> Self {
        Self {
            sys: System::new(),
            lmt,
            cpus: vec![],
        }
    }
}

impl State {
    pub fn refresh_cpu_all(&mut self) {
        self.sys.refresh_cpu_all();

        let cpus: &[Cpu] = self.sys.cpus();
        self.cpus = cpus.iter().map(CpuInfo::from).collect();
    }
}

impl State {
    pub fn cpu_count(&self) -> usize {
        self.cpus.len()
    }
}

impl State {
    pub fn cpu_by_index(&self, ix: usize) -> Option<&CpuInfo> {
        self.cpus.get(ix)
    }

    pub fn cpu_brand_by_index(&self, ix: usize) -> &str {
        self.cpu_by_index(ix)
            .map(|c| c.brand.as_str())
            .unwrap_or_default()
    }

    pub fn cpu_name_by_index(&self, ix: usize) -> &str {
        self.cpu_by_index(ix)
            .map(|c| c.name.as_str())
            .unwrap_or_default()
    }

    pub fn cpu_vendor_id_by_index(&self, ix: usize) -> &str {
        self.cpu_by_index(ix)
            .map(|c| c.vendor_id.as_str())
            .unwrap_or_default()
    }

    pub fn cpu_usage_pcnt_by_index(&self, ix: usize) -> f32 {
        self.cpu_by_index(ix)
            .map(|c| c.usage_pcnt)
            .unwrap_or_default()
    }

    pub fn cpu_freq_mhz_by_index(&self, ix: usize) -> u64 {
        self.cpu_by_index(ix)
            .map(|c| c.freq_mhz)
            .unwrap_or_default()
    }
}

pub struct WasmStore(Store<State>);

impl WasmStore {
    pub fn as_store_mut(&mut self) -> &mut Store<State> {
        &mut self.0
    }
}

impl WasmStore {
    pub fn apply_limit(&mut self) {
        self.0.limiter(|state: &mut State| &mut state.lmt)
    }
}

pub struct WasmLinker(Linker<State>);

impl WasmLinker {
    pub fn wrap_print_f32(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "print_f32", |_: Caller<'_, State>, val: f32| {
            println!("float 32: {val}");
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn wrap_print_u64(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "print_u64", |_: Caller<'_, State>, val: i64| {
            let u: u64 = val as u64;
            println!("unsigned 64: {u}");
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }
}

impl WasmLinker {
    pub fn wrap_cpu_count(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "cpu_count", |caller: Caller<'_, State>| {
            let s: &State = caller.data();
            let cnt: usize = s.cpu_count();
            cnt as u32
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }
}

impl WasmLinker {
    pub fn wrap_cpu_usage_pcnt_by_index(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "cpu_usage_pcnt_by_index",
            |caller: Caller<'_, State>, index: i32| {
                let s: &State = caller.data();
                let ix: usize = index as usize;
                s.cpu_usage_pcnt_by_index(ix)
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn wrap_cpu_freq_mhz_by_index(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "cpu_freq_mhz_by_index",
            |caller: Caller<'_, State>, index: i32| {
                let s: &State = caller.data();
                let ix: usize = index as usize;
                s.cpu_freq_mhz_by_index(ix)
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }
}

impl WasmLinker {
    pub fn wrap_print_cpu_count(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap("host", "print_cpu_count", |caller: Caller<'_, State>| {
            let s: &State = caller.data();
            let cnt: usize = s.cpu_count();
            println!("cpu count: {cnt}");
        })
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }
}

impl WasmLinker {
    pub fn wrap_print_cpu_ix(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_cpu_ix",
            |_: Caller<'_, State>, index: i32| {
                println!("cpu index: {index}");
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }
}

impl WasmLinker {
    pub fn wrap_print_cpu_usage_pcnt_by_index(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_cpu_usage_pcnt_by_index",
            |caller: Caller<'_, State>, index: i32| {
                let s: &State = caller.data();
                let usage: f32 = s.cpu_usage_pcnt_by_index(index as usize);
                println!("cpu usage: {usage:.2}%");
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn wrap_print_cpu_freq_mhz_by_index(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_cpu_freq_mhz_by_index",
            |caller: Caller<'_, State>, index: i32| {
                let s: &State = caller.data();
                let freq = s.cpu_freq_mhz_by_index(index as usize);
                println!("cpu frequency: {freq} MHz");
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }
}

impl WasmLinker {
    pub fn wrap_print_cpu_brand_by_index(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_cpu_brand_by_index",
            |caller: Caller<'_, State>, index: i32| {
                let s: &State = caller.data();
                let ix: usize = index as usize;
                let brand: &str = s.cpu_brand_by_index(ix);
                println!("cpu brand: {brand}");
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn wrap_print_cpu_name_by_index(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_cpu_name_by_index",
            |caller: Caller<'_, State>, index: i32| {
                let s: &State = caller.data();
                let ix: usize = index as usize;
                let name: &str = s.cpu_name_by_index(ix);
                println!("cpu name: {name}");
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }

    pub fn wrap_print_cpu_vendor_id_by_index(self) -> Result<Self, io::Error> {
        let mut lnkr: Linker<_> = self.0;
        lnkr.func_wrap(
            "host",
            "print_cpu_vendor_id_by_index",
            |caller: Caller<'_, State>, index: i32| {
                let s: &State = caller.data();
                let ix: usize = index as usize;
                let vendor: &str = s.cpu_vendor_id_by_index(ix);
                println!("cpu vendor id: {vendor}");
            },
        )
        .map_err(io::Error::other)?;
        Ok(Self(lnkr))
    }
}

impl WasmLinker {
    pub fn to_instance(&self, store: &mut WasmStore, mdl: &Module) -> Result<Instance, io::Error> {
        self.0
            .instantiate(&mut store.0, mdl)
            .map_err(io::Error::other)
    }
}

#[derive(Default)]
pub struct WasmEngine(Engine);

impl WasmEngine {
    pub fn create_linker(&self) -> WasmLinker {
        WasmLinker(Linker::<State>::new(&self.0))
    }
}

impl WasmEngine {
    pub fn wat2module(&self, wat: &[u8]) -> Result<Module, io::Error> {
        Module::new(&self.0, wat).map_err(io::Error::other)
    }
}

impl WasmEngine {
    pub fn state2store(&self, state: State) -> WasmStore {
        WasmStore(Store::new(&self.0, state))
    }
}
