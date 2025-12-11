use tsify::Tsify;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

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

#[derive(Tsify)]
#[tsify(namespace)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum Foobar {
    English,
    Latin,
}

#[wasm_bindgen]
pub async fn query_foo(repo: String) -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let url = format!("https://api.github.com/repos/{}/branches/master", repo);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Send the JSON response back to JS.
    Ok(json)
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