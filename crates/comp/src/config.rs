use std::path::PathBuf;

use tracing::{info, instrument};
use wasmtime::{
    Engine, Store,
    component::{Component, Instance, Linker, TypedFunc},
};
use wasmtime_wasi::{FsPerms, ResourceTable, WasiCtx, WasiCtxView, WasiView};

use crate::cli::ConfigPlugin;

wasmtime::component::bindgen!({
    world: "backend",
    path: "../../wit",
});

pub(crate) struct ConfigEngine {
    instance: Instance,
    store: Store<MyState>,

    tick_func: TypedFunc<(), (Result<(), String>,)>,
}

struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.ctx,
            table: &mut self.table,
        }
    }
}

impl comp::config::host::Host for ConfigEngine {
    fn set(&mut self, key: String, value: comp::config::host::Value) {
        // self.config.insert(key, value);
    }

    fn log(&mut self, msg: String) {
        info!("backend: {msg}");
    }
}

impl ConfigEngine {
    #[instrument(ret)]
    pub(crate) fn resolve_config_plugin(plugin: ConfigPlugin) -> anyhow::Result<PathBuf> {
        match plugin {
            ConfigPlugin::Custom(str) => Ok(PathBuf::from(str)),
            _ => unimplemented!("builtin paths are not implemented yet :("),
        }
    }

    pub(crate) fn new(plugin: ConfigPlugin) -> anyhow::Result<Self> {
        let engine = Engine::default();

        let component = Component::from_file(&engine, Self::resolve_config_plugin(plugin)?)?;
        let mut linker = Linker::<MyState>::new(&engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

        let mut builder = WasiCtx::builder();

        builder.preopened_dir("/uiahsduiashd/", "/config", FsPerms::ReadWrite)?;

        let mut store = Store::new(
            &engine,
            MyState {
                ctx: builder.build(),
                table: ResourceTable::new(),
            },
        );

        let instance = linker.instantiate(&mut store, &component)?;

        let tick_func: TypedFunc<(), (Result<(), String>,)> =
            instance.get_typed_func(&mut store, "tick")?;

        Ok(Self {
            instance,
            store,
            tick_func,
        })
    }

    pub(crate) fn tick(
        &mut self,
    ) -> Result<(std::result::Result<(), std::string::String>,), wasmtime::Error> {
        self.tick_func.call(&mut self.store, ())
    }
}
