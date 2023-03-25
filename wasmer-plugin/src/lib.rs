use std::{alloc::*, sync::Mutex};

static TEXT: Mutex<String> = Mutex::new(String::new());

#[no_mangle]
pub fn push_string(text: u64) -> u64 {
    // text
    let bytes = import_from_host(text);
    let text = String::from_utf8(bytes).unwrap();

    TEXT.lock().unwrap().push_str(&text);

    // FIXME: there is a bug. `export_to_host` takes vec, but that vec might have bigger capacity than size
    // Let's see if the test uncovers it
    let cloned = TEXT
        .lock()
        .unwrap()
        .as_bytes()
        .to_owned()
        .into_boxed_slice();
    export_to_host(cloned)
}

#[no_mangle]
pub fn remove_chars(num: u32) {
    let len = TEXT.lock().unwrap().len();
    TEXT.lock()
        .unwrap()
        .replace_range((len - num as usize)..len, "");
}

fn export_to_host(mut data: Box<[u8]>) -> u64 {
    let len = data.len();
    let addr = data.as_mut_ptr() as usize;
    std::mem::forget(data);
    to_fatptr(addr, len)
}

fn import_from_host(fatptr: u64) -> Vec<u8> {
    let (addr, len) = from_fatptr(fatptr);
    // SAFETY: Host is giving us full ownership of these bytes
    unsafe { Vec::from_raw_parts(addr as *mut u8, len, len) }
}

#[no_mangle]
pub fn allocate_for_host(size: usize) -> u64 {
    // let values = Box::new([0u8; 20]);
    // let x = Box::leak(values);
    // return x as *mut _ as usize as u64;
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
    let len = (fatptr >> 32) as usize;
    (addr, len)
}

fn to_fatptr(addr: usize, len: usize) -> u64 {
    (addr as u32) as u64 | (len as u64) << 32
}
