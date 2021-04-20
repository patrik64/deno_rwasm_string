use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn compute(rounds: u32) -> u32 {
    let mut count: u32 = 0;
    for _ in 0..rounds {
        for _ in 0..rounds {
            count += 1;
        }
    }
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