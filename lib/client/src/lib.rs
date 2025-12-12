mod query;

use tsify::Tsify;
use wasm_bindgen::prelude::*;
use dto::part_of_speech::PartOfSpeechDto;

#[derive(Tsify)]
#[tsify(namespace)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum Color {
    Red,
    Green,
    Blue
}

#[derive(Tsify)]
#[tsify(namespace)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum Shape {
    #[serde(skip_deserializing)]
    Color { primary_color: Color },
    Circle { r: f64 },
    Rectangle { x: f64, y: f64 },
    Triangle { a: f64, b: f64, c: f64 },
}

#[wasm_bindgen]
pub struct Counter {
    value: i32,
}

#[wasm_bindgen]
impl Counter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Counter {
        Counter { value: 0 }
    }

    #[wasm_bindgen]
    pub fn increment(&mut self) {
        self.value += 1;
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> i32 {
        self.value
    }
}

#[wasm_bindgen]
pub fn print_a_part_of_speech(part: PartOfSpeechDto) {
    println!("{:?}", part);
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
