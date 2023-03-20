use wasmer::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bytes = include_bytes!(
        "../../wasmer-plugin/target/wasm32-unknown-unknown/debug/wasmer2_plugin.wasm"
    )
    .as_ref();

    // Create the store
    let mut store = Store::new(Cranelift::default());

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    // Create an empty import object.
    let import_object = imports! {};

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&mut store, &module, &import_object)?;
    let give_string: TypedFunction<u32, u64> = instance
        .exports
        .get_typed_function(&mut store, "give_string")?;

    let fatptr = give_string.call(&mut store, 5)?;

    let address = fatptr & 0xffffffff;
    let size = (fatptr >> 32) as usize;

    let memory = instance.exports.get_memory("memory").expect("get memory");
    let view = memory.view(&store);

    let mut bytes = vec![0u8; size];

    view.read(address, &mut bytes).expect("view read");

    let text = String::from_utf8(bytes);

    println!("{:?}, {:?}, {:?}", address, size, text);

    Ok(())
}
