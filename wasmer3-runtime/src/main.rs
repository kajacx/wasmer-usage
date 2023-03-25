use std::borrow::Borrow;

use wasmer::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bytes = include_bytes!(
        "../../wasmer-plugin/target/wasm32-unknown-unknown/debug/wasmer_plugin.wasm"
    )
    .as_ref();

    let fp = to_fatptr(420, 20);
    let (addr, len) = from_fatptr(fp);
    println!("ADDR: {addr}, LEN: {len}");

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

    let memory = instance.exports.get_memory("memory").unwrap();

    let mut compare_string = String::new();

    // let mut shring_strings = |n: u32| {};

    for i in 0..10_000_000u32 {
        if i % 1000 == 0 {
            println!("{}: {}", i, compare_string.len());
        }

        if compare_string.len() < 1000 {
            grow_strings(memory, &mut store, &instance, &mut compare_string, i);
            continue;
        }
        if compare_string.len() > 1000000 {
            shrink_strings(memory, &mut store, &instance, &mut compare_string, i % 100);
            continue;
        }
        if i % 13 < 5 {
            grow_strings(memory, &mut store, &instance, &mut compare_string, i);
        } else {
            shrink_strings(memory, &mut store, &instance, &mut compare_string, i % 100);
        }
    }

    Ok(())
}

fn grow_strings(
    memory: &Memory,
    store: &mut Store,
    instance: &Instance,
    compare_string: &mut String,
    n: u32,
) {
    let appendings = format!("Growing: {n}, ");
    compare_string.push_str(&appendings);
    let exported = export_to_plugin(memory, store, instance, appendings.as_bytes());

    // println!("Passing into plugin: {:?}", from_fatptr(exported));

    let push_str = instance
        .exports
        .get_typed_function::<u64, u64>(&store, "push_string")
        .unwrap();
    let ret = push_str.call(store, exported).unwrap();

    // println!("Getting from  plugin: {:?}", from_fatptr(ret));

    let imported = import_from_plugin(instance, memory, store, ret);
    let check = String::from_utf8(imported).unwrap();

    let a: &str = compare_string.as_str();
    let b: &str = check.as_str();
    assert_eq!(a, b);
}

fn shrink_strings(
    memory: &Memory,
    store: &mut Store,
    instance: &Instance,
    compare_string: &mut String,
    n: u32,
) {
    remove_chars(compare_string, n);
    let rm_chars = instance
        .exports
        .get_typed_function::<u32, ()>(&store, "remove_chars")
        .unwrap();
    rm_chars.call(store, n).unwrap();
}

fn push_string(string: &mut String, text: &str) {
    string.push_str(text);
}

fn remove_chars(string: &mut String, num: u32) {
    let len = string.len();
    string.replace_range((len - num as usize)..len, "");
}

fn import_from_plugin(
    instace: &Instance,
    memory: &Memory,
    store: &mut Store,
    fatptr: u64,
) -> Vec<u8> {
    let (addr, len) = from_fatptr(fatptr);
    // println!("addr: {addr}, len: {len}");
    let mut bytes = vec![0; len];
    let view = memory.view(store);
    view.read(addr as u64, &mut bytes[0..len]).unwrap();

    let free = instace
        .exports
        .get_typed_function::<u64, ()>(store, "free_from_host")
        .unwrap();
    free.call(store, fatptr);

    bytes
}

fn export_to_plugin(memory: &Memory, store: &mut Store, instance: &Instance, data: &[u8]) -> u64 {
    let allocate = instance
        .exports
        .get_typed_function::<u32, u64>(&store, "allocate_for_host")
        .unwrap();
    let mut allocate = |size: u32| allocate.call(store, size).unwrap();

    let fatptr = allocate(data.len() as u32);
    // println!("Allocated in host: {:?}", from_fatptr(fatptr));
    let (addr, _) = from_fatptr(fatptr);
    let view = memory.view(store);
    view.write(addr as u64, data).unwrap();
    fatptr
}

fn from_fatptr(fatptr: u64) -> (usize, usize) {
    let addr = fatptr as u32 as usize;
    let len = (fatptr >> 32) as usize;
    (addr, len)
}

fn to_fatptr(addr: usize, len: usize) -> u64 {
    (addr as u32) as u64 | (len as u64) << 32
}
