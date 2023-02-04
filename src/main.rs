use wit_component::ComponentEncoder;

mod host_impl;

fn encode_wasm_component(path: &str, output: Option<String>) {
    let file_bytes = std::fs::read(path).expect("Wat parse wasm file error");
    let wasi_adapter = std::fs::read("./wasi_snapshot_preview1.wasm").unwrap();

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

fn create_wasmtime_config() -> wasmtime::Config {
    let mut config = wasmtime::Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    config
}

#[tokio::main]
pub async fn main() {
    let target = "target/wasm32-wasi/release/wasm_lib.wasm";
    let output = "target/wasm32-wasi/release/wasm_lib.component.wasm";
    encode_wasm_component(target, Some(output.to_string()));
    println!("Run component: {}", output);

    let engine = wasmtime::Engine::new(&create_wasmtime_config()).unwrap();
    let component = wasmtime::component::Component::from_file(&engine, output).unwrap();
    let mut linker = wasmtime::component::Linker::new(&engine);
    wasi_host::add_to_linker(&mut linker, |x| x).unwrap();

    let mut store = wasmtime::Store::new(
        &engine,
        wasi_cap_std_sync::WasiCtxBuilder::new()
            .inherit_stdio()
            .build(),
    );

    let (exports, _) = host_impl::TestWorld::instantiate_async(&mut store, &component, &linker)
        .await
        .unwrap();
    exports.exports().call_print(&mut store).await.unwrap();
}
