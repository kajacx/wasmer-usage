#[no_mangle]
pub fn add(left: i32, right: i32) -> i32 {
    unsafe { add_one(left) + add_one(right) }
}

#[link(wasm_import_module = "my_imports")]
extern "C" {
    fn add_one(arg: i32) -> i32;
}
