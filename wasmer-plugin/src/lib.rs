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
