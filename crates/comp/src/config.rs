use std::{collections::HashMap, path::PathBuf};

use tracing::info;
use wasmtime::{Engine, Instance, Module, Store, StoreLimits};

wasmtime::component::bindgen!({
    world: "backend",
    path: "../../wit",
});

#[derive(Debug)]
pub(crate) struct HostState {
    instance: Instance,
    store: Store<()>,
}

impl comp::config::host::Host for HostState {
    fn set(&mut self, key: String, value: comp::config::host::Value) {
        // self.config.insert(key, value);
    }

    fn log(&mut self, msg: String) {
        info!("backend: {msg}");
    }
}

impl HostState {
    pub(crate) fn new(plugin: PathBuf) -> anyhow::Result<Self> {
        let engine = Engine::default();
        let module = Module::from_file(&engine, plugin)?;

        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[])?;

        Ok(Self { instance, store })
    }
}
