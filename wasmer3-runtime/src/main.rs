use wasmer::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bytes = include_bytes!(
        "../../wasmer-plugin/target/wasm32-unknown-unknown/debug/wasmer_plugin.wasm"
    )
    .as_ref();

    // Create the store
    let store = Store::new(Cranelift::default());

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    // Create the store
    // TODO: what the actual fuck?
    let mut store = Store::new(Cranelift::default());

    // Create an empty import object.
    let import_object = imports! {};

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let mut compare_string = String::new();

    let mut grow_strings = |n: u32| {
        let appendings = format!("Growing: {n}, ");
        compare_string.push_str(&appendings);

            let exported = 
    };
    let mut shring_strings = |n: u32| {

    };

    for _ in 0..1000u32 {
        if (compare_string.len() < 100) {

        }
    }


    Ok(())
}

fn push_string(string: &mut String, text: &str) {
    string.push_str(text);
}

fn remove_chars(string: &mut String, num: u32) {
    let len = string.len();
    string.replace_range((len - num as usize)..len, "");
}

fn import_from_plugin(memory: &Memory, store: &Store, fatptr: u64) -> Vec<u8> {
    import_from_plugin_view(&memory.view(store), fatptr)
}

fn import_from_plugin_view(view: &MemoryView, fatptr: u64) -> Vec<u8> {
    let (addr, len) = from_fatptr(fatptr);
    let mut bytes = vec![0; len];
    view.read(addr as u64, &mut bytes[0..len]).unwrap();
    bytes
}

fn export_to_plugin(memory: &Memory, store: &mut Store, instance: &Instance, data: &[u8]) {
    let view = memory.view(store);
    let allocate = instance.exports.get_typed_function::<u32, u64>(&store, "allocate_for_host").unwrap();
    let allocate = |size: u32| {allocate.call(store, size).unwrap()};
    export_to_plugin_view(&view, allocate, data);
}

fn export_to_plugin_view(view: &MemoryView, mut allocate: impl FnMut(u32) -> u64, data: &[u8]) {
    let fatptr = allocate(data.len() as u32);
    let (addr, _) = from_fatptr(fatptr);
    view.write(addr as u64, data).unwrap();
}

fn from_fatptr(fatptr: u64) -> (usize, usize) {
    let addr = fatptr as u32 as usize;
    let len = (fatptr << 32) as usize;
    (addr, len)
}

fn to_fatptr(addr: usize, len: usize) -> u64 {
    (addr as u64) << 32 | len as u64
}
