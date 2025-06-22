use wasmtime::{Caller, Func, Module};

pub struct State {
    pub name: String,
    pub count: usize,
}

pub fn run_wasm() -> wasmtime::Result<()> {
    println!("Compiling module...");
    let engine = wasmtime::Engine::default();
    let module = Module::from_file(&engine, "hello.wat")?;

    println!("Initializing...");
    let data = State {
        name: "hello, world!".to_string(),
        count: 0,
    };
    let mut store = wasmtime::Store::new(&engine, data);

    println!("Creating callback...");
    let hello_func = Func::wrap(&mut store, |mut caller: Caller<'_, State>| {
        println!("Calling back...");
        println!("> {}", caller.data().name);
        caller.data_mut().count += 1;
    });

    println!("Instantiating module...");
    let imports = [hello_func.into()];
    let instance = wasmtime::Instance::new(&mut store, &module, &imports)?;

    println!("Extracting export...");
    let run = instance.get_typed_func::<(), ()>(&mut store, "run")?;

    println!("Calling export...");
    run.call(&mut store, ())?;

    println!("Done.");
    Ok(())
}
