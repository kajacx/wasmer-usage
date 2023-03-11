use wasmer::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bytes = include_bytes!(
        "../../wasmer2-plugin/target/wasm32-unknown-unknown/debug/wasmer2_plugin.wasm"
    )
    .as_ref();

    // Use Singlepass compiler with the default settings
    let compiler = Singlepass::default();
    let engine = Universal::new(compiler).engine();

    // Create the store
    let store = Store::new(&engine);

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    fn add_one(a: i32) -> i32 {
        println!("Calling `multiply_native`...");
        let result = a + 1;

        println!("Result of `multiply_native`: {:?}", result);

        result
    }
    let multiply_native = Function::new_native(&store, add_one);

    fn add_one_f(a: f32) -> f32 {
        a + 1.0
    }
    let add_one_f_native = Function::new_native(&store, add_one_f);

    // Create an empty import object.
    let import_object = imports! {
        "my_imports" => { "add_one" => multiply_native, "add_one_f" => add_one_f_native }
    };

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&module, &import_object)?;

    let sum = instance.exports.get_function("add")?;

    println!("Calling `sum` function...");
    // Let's call the `sum` exported function. The parameters are a
    // slice of `Value`s. The results are a boxed slice of `Value`s.
    //let results = sum.call(&store, &[Value::I32(1), Value::I32(2)])?;
    let results = sum.call(&[Value::I32(1), Value::I32(2)])?;

    println!("Results: {:?}", results);
    assert_eq!(results.to_vec(), vec![Value::I32(5)]);

    let sum = instance.exports.get_function("add_three")?;

    println!("Calling `sum` function...");
    // Let's call the `sum` exported function. The parameters are a
    // slice of `Value`s. The results are a boxed slice of `Value`s.
    //let results = sum.call(&store, &[Value::I32(1), Value::I32(2)])?;
    let results = sum.call(&[Value::F32(5.5)])?;

    println!("Results: {:?}", results);
    assert_eq!(results.to_vec(), vec![Value::F32(5.0)]);

    Ok(())
}
