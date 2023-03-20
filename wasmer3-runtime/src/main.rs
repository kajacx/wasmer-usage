use wasmer::*;

#[derive(Clone, Default)]
struct MyEnv {
    pub memory: Option<Memory>,
}

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

    let env = FunctionEnv::new(&mut store, MyEnv { memory: None });

    fn add_one_i32(_env: FunctionEnvMut<MyEnv>, a: i32) -> i32 {
        a + 1
    }
    let add_one_i32_native = Function::new_typed_with_env(&mut store, &env, add_one_i32);

    fn add_one_f32(a: f32) -> f32 {
        a + 1.0
    }
    let add_one_f32_native = Function::new_typed(&mut store, add_one_f32);

    // Create an empty import object.
    let import_object = imports! {
        "my_imports" => {
            "add_one_i32" => add_one_i32_native,
            "add_one_f32" => add_one_f32_native,
        }
    };

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&mut store, &module, &import_object)?;

    // let mut env_mut = env.as_mut(&mut store);
    // env_mut.memory = Some(instance.exports.get_memory("memory").expect("get memory"));

    let add_three_i32: TypedFunction<i32, i32> = instance
        .exports
        .get_function("add_three_i32")?
        .typed(&mut store)?;

    println!("Calling `add_three_i32` function...");
    let results = add_three_i32.call(&mut store, 5)?;

    println!("Results: {:?}", results);
    assert_eq!(results, 8);

    let add_three_f32 = instance.exports.get_function("add_three_f32")?;

    println!("Calling `add_three_f32` function...");
    let results = add_three_f32.call(&mut store, &[Value::F32(5.5)])?;

    println!("Results: {:?}", results);
    assert_eq!(results.to_vec(), vec![Value::F32(8.5)]);

    let give_string: TypedFunction<u32, u64> = instance
        .exports
        .get_typed_function(&mut store, "give_string")?;

    let fatptr = give_string.call(&mut store, 5)?;

    let address = fatptr as u32 as usize;
    let size = (fatptr >> 32) as usize;

    let memory = instance.exports.get_memory("memory").expect("get memory");
    let view = memory.view(&store);

    let mut bytes = vec![0u8; size];

    view.read(dbg!(address.try_into().expect("usize to u64")), &mut bytes)
        .expect("view read");

    let text = String::from_utf8(bytes);

    println!("{:?}, {:?}, {:?}", address, size, text);

    Ok(())
}
