/// This can be called with `module.instance.exports.hello_world_1()`, without even using wasm-bindgen.
/// However, no dead code stripping has occured, so it's comparatively huge - a 0.8MB raw `.wasm` file out of cargo.
/// No wonder every WASM tutorial insists on using `wasm-pack`!
/// To experiment, delete everything bellow `hello_world_3` and run:
#[no_mangle]
pub fn hello_world_1() -> u32 {
    42
}

/// This also works... presumably the stdlib contains an allocator that just relies on e.g. a linear memory section,
/// which requires no WASM imports.
#[no_mangle]
pub fn hello_world_2() -> u32 {
    let mut v = vec!{1,2,3};
    v.push(4);
    let mut accum = 0;
    for i in v {
        accum += i;
    }
    accum
}

/// This doesn't work.
/// 
/// The `#[no_mangle]` version returns undefined, possibly invokes UB?  Certainly doesn't do the WASM ABI right to just
/// invoke `hello_world_3()` directly - you're supposed to feed it a pointer...
/// 
/// The `#[wasm_bindgen]` version complains:  `error: it is currently not sound to use lifetimes in function signatures`
#[no_mangle]
// #[wasm_bindgen]
pub fn hello_world_3() -> &'static str {
    "Hello, world!"
}
