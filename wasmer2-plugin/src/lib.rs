#[no_mangle]
pub fn add(left: i32, right: i32) -> i32 {
    unsafe { add_one(left) + add_one(right) }
}

#[link(wasm_import_module = "my_imports")]
extern "C" {
    fn add_one(arg: i32) -> i32;
}

#[no_mangle]
pub fn add_three(left: f32) -> f32 {
    (unsafe { add_one_f(left + 1.0) }) + 1.0
}

#[link(wasm_import_module = "my_imports")]
extern "C" {
    fn add_one_f(arg: f32) -> f32;
}
