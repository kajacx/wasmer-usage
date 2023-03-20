use std::fmt::format;

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

// #[no_mangle]
// pub fn exported_returns_bool() -> bool {
//     unsafe { imported_returns_bool() }
// }

// #[no_mangle]
// pub fn exported_returns_u8() -> u8 {
//     unsafe { imported_returns_u8() }
// }

// #[no_mangle]
// pub fn exported_takes_bool(arg: bool) {
//     unsafe { imported_takes_bool(arg) }
// }

// #[no_mangle]
// pub fn exported_takes_u8(arg: u8) {
//     unsafe { imported_takes_u8(arg) }
// }

// #[link(wasm_import_module = "my_imports")]
// extern "C" {
//     fn imported_returns_bool() -> bool;
//     fn imported_returns_u8() -> u8;
//     fn imported_takes_bool(arg: bool);
//     fn imported_takes_u8(arg: u8);
// }

#[no_mangle]
pub fn give_string(number: u32) -> u64 {
    let text = format!(
        "Hello, I have {} {}",
        number,
        if number == 1 { "apple" } else { "apples" }
    );

    let leaked = Box::leak(text.into_boxed_str());

    let len = leaked.len() as u64;
    let address = leaked as *const str as *const () as usize as u64;

    // Address is in lower bytes, len in higher ones
    (len << 32) | address
}
