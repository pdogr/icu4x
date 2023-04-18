// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::CodePointTrieBuilder;
use crate::CodePointTrieBuilderData;
use icu_collections::codepointtrie::TrieType;
use icu_collections::codepointtrie::TrieValue;
use std::io::Read;
use wasmer::TypedFunction;
use wasmer::WasmPtr;
use wasmer::{Instance, Module, Store};
use wasmer_wasi::{Pipe, WasiState};

const WASM_BYTES: &[u8] = include_bytes!("../list_to_ucptrie.wasm");

pub(crate) fn run_wasm<T>(builder: &CodePointTrieBuilder<T>) -> String
where
    T: TrieValue + Into<u32>,
{
    // Set up the execution environment with a WasiState
    let args = &[
        format!("{}", builder.default_value.into()),
        format!("{}", builder.error_value.into()),
        match builder.trie_type {
            TrieType::Fast => "fast",
            TrieType::Small => "small",
        }
        .to_owned(),
        format!("{}", std::mem::size_of::<T::ULE>() * 8),
    ];

    let trie_type_str = match builder.trie_type {
        TrieType::Fast => "fast",
        TrieType::Small => "small",
    };

    let mut store = Store::default();
    let module = Module::new(&store, WASM_BYTES).expect("valid WASM");

    let mut stdout = Pipe::new();

    let wasi_env = WasiState::new("list_to_ucptrie")
        .stdout(Box::new(stdout.clone()))
        .args(args)
        .finalize(&mut store)
        .expect("Unable to create wasi_env");

    // Create the WebAssembly instance with the module and the WasiState
    let import_object = wasi_env
        .import_object(&mut store, &module)
        .expect("walid wasm file");
    let instance = Instance::new(&mut store, &module, &import_object).expect("valid instance");
    let memory = instance.exports.get_memory("memory").expect("memory");

    let memory_view = memory.view(&store);
    let malloc = instance
        .exports
        .get_typed_function::<i32, WasmPtr<u8>>(&mut store, "malloc")
        .expect("malloc is exported");
    let trie_type_ptr: WasmPtr<u8> = malloc.call(&mut store, trie_type_str.len() as i32).unwrap();
    memory_view
        .write(trie_type_ptr.offset().into(), trie_type_str.as_bytes())
        .expect("unable to write 'trie_type_str'");
    wasi_env.data_mut(&mut store).set_memory(memory.clone());

    let CodePointTrieBuilderData::ValuesByCodePoint(values) = builder.data;
    let malloc = instance
        .exports
        .get_typed_function::<i32, WasmPtr<u32>>(&mut store, "malloc")
        .expect("malloc is exported");
    let base_ptr: WasmPtr<u32> = malloc
        .call(&mut store, (values.len() * 4) as i32)
        .expect("Unable to allocate memory for values");
    for (i, value) in values.iter().enumerate() {
        let value_ptr = base_ptr
            .add_offset(i as u32)
            .expect(&format!("{} element offset", i));
        let value_deref_ptr = value_ptr.deref(&memory_view);
        value_deref_ptr.write((*value).into()).expect("Unable to write value");
    }

    let construct_ucptrie: TypedFunction<(i32, i32, i32, i32, i32, i32), i32> = instance
        .exports
        .get_typed_function(&mut store, "construct_ucptrie")
        .expect("'construct_ucptrie' is exported");

    let exit_result = construct_ucptrie.call(
        &mut store,
        builder.default_value.into() as i32,
        builder.error_value.into() as i32,
        trie_type_ptr.offset().try_into().expect("trie_type_ptr is valid"),
        // size_of::<T::ULE>() * 8 fits in i32
        (std::mem::size_of::<T::ULE>() * 8).try_into().unwrap(),
        base_ptr.offset().try_into().expect("base ptr is valid"),
        values.len() as i32,
    );

    match exit_result {
        // 0 denotes success,
        Ok(0) => {}
        e => panic!("list_to_ucptrie failed in C++: args were: {args:?}: {e:?}"),
    }

    // The output is a TOML blob, which we can save in a string
    let mut buf = String::new();
    stdout
        .read_to_string(&mut buf)
        .expect("pipe contains valid utf-8");
    buf
}
