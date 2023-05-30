use wasmtime::component::Linker;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiCtxBuilder, WasiView};
use wit_component::ComponentEncoder;

mod host_impl;

fn encode_wasm_component(path: &str, output: Option<String>) {
    let file_bytes = std::fs::read(path).expect("Wat parse wasm file error");
    let wasi_adapter = std::fs::read("./wasi_snapshot_preview1.reactor.wasm").unwrap();

    let component = ComponentEncoder::default()
        .module(&file_bytes)
        .expect("Pull custom sections from module")
        .validate(true)
        .adapter("wasi_snapshot_preview1", &wasi_adapter)
        .expect("Add adapter to component")
        .encode()
        .expect("Encode component");

    let output = output.unwrap_or_else(|| path.to_string());
    std::fs::write(&output, component).expect("Write component file error");
    println!("Convert wasm module to component success, {}", &output)
}

fn create_wasmtime_config() -> Config {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    config
}

#[tokio::main]
pub async fn main() {
    println!("call wasm_lib");
    call_wasm_lib().await;
}

pub struct Context {
    wasi_ctx: WasiCtx,
    table: Table,
}

impl WasiView for Context {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi_ctx
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

impl Context {
    pub fn new() -> Self {
        let mut table = Table::new();
        Context {
            wasi_ctx: WasiCtxBuilder::new()
                .inherit_stdio()
                .build(&mut table)
                .unwrap(),
            table,
        }
    }
}

async fn call_wasm_lib() {
    let target = "target/wasm32-wasi/release/wasm_lib.wasm";
    let output = "target/wasm32-wasi/release/wasm_lib.component.wasm";

    encode_wasm_component(target, Some(output.to_string()));
    println!("Run component: {}", output);

    let engine = Engine::new(&create_wasmtime_config()).unwrap();
    let component = wasmtime::component::Component::from_file(&engine, output).unwrap();

    let mut linker: Linker<Context> = Linker::new(&engine);
    wasmtime_wasi::preview2::wasi::command::add_to_linker(&mut linker)
        .expect("add wasi::command linker failed");

    let context = Context::new();
    let mut store = Store::new(&engine, context);

    let (exports, _) = host_impl::TestWorld::instantiate_async(&mut store, &component, &linker)
        .await
        .unwrap();
    exports
        .foo_bar_export_iface()
        .call_print(&mut store)
        .await
        .unwrap();
}
