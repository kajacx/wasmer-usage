#[no_mangle]
pub fn add_three_i32(val: i32) -> i32 {
    (unsafe { add_one_i32(val + 1) }) + 1
}

#[link(wasm_import_module = "my_imports")]
extern "C" {
    fn add_one_i32(arg: i32) -> i32;
}

#[no_mangle]
pub fn add_three_f32(left: f32) -> f32 {
    (unsafe { add_one_f32(left + 1.0) }) + 1.0
}

#[link(wasm_import_module = "my_imports")]
extern "C" {
    fn add_one_f32(arg: f32) -> f32;
}
