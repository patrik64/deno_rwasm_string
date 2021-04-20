use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn compute(rounds: u32) -> u32 {
    let mut count: u32 = 0;
    for _ in 0..rounds {
        for _ in 0..rounds {
            count += 1;
        }
    }
    log_u32(count);
    count
}

#[wasm_bindgen]
pub fn say() -> String {
    let r = String::from("hello from Rust!!!");
    return r;
}

#[wasm_bindgen]
pub fn echo(content: &str) -> String {
    return content.to_string();
}


#[wasm_bindgen]
pub fn test_log() {
    log("Testing console log from Rust!");
    log_u32(42);
    log_many("Logging", "many values!");
    using_a_macro();
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn using_a_macro() {
    console_log!("Hello {}!", "rust macros");
    console_log!("Let's print some numbers...");
    console_log!("1 + 3 = {}", 1 + 3);
}
