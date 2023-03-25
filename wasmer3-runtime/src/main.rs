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

#[no_mangle]
pub fn push_string(string: &mut String, text: &str) {
    string.push_str(text);
}

#[no_mangle]
pub fn remove_chars(string: &mut String, num: u32) {
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

fn import_from_host(fatptr: u64) -> Vec<u8> {
    let (addr, len) = from_fatptr(fatptr);
    // SAFETY: Host is giving us full ownership of these bytes
    unsafe { Vec::from_raw_parts(addr as *mut u8, len, len) }
}

#[no_mangle]
pub fn allocate_for_host(size: usize) -> u64 {
    let addr = if size == 0 {
        0
    } else {
        let layout = Layout::from_size_align(size, 1).unwrap();
        // SAFETY: size is not zero
        let bytes = unsafe { alloc(layout) };
        bytes as *mut u8 as usize
    };
    to_fatptr(addr, size)
}

#[no_mangle]
pub fn free_from_host(fatptr: u64) {
    let (addr, len) = from_fatptr(fatptr);
    if len != 0 {
        let layout = Layout::from_size_align(len, 1).unwrap();
        // SAFETY: size is not zero, and host guarantees to not use these bytes anymore
        unsafe { dealloc(addr as *mut u8, layout) };
    };
}

fn from_fatptr(fatptr: u64) -> (usize, usize) {
    let addr = fatptr as u32 as usize;
    let len = (fatptr << 32) as usize;
    (addr, len)
}

fn to_fatptr(addr: usize, len: usize) -> u64 {
    (addr as u64) << 32 | len as u64
}
