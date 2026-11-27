use std::collections::HashMap;

use tracing::info;
use wasmtime::{StoreLimits, component::*};

wasmtime::component::bindgen!({
    world: "backend",
    path: "wit",
});

struct HostState {
    table: ResourceTable,
    config: HashMap<String, comp::config::host::Value>,
    limits: StoreLimits,
}

impl comp::config::host::Host for HostState {
    fn set(&mut self, key: String, value: comp::config::host::Value) {
        self.config.insert(key, value);
    }

    fn log(&mut self, msg: String) {
        info!("backend: {msg}");
    }
}
