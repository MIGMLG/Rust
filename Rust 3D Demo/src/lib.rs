extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn say_hello_from_rust(){
    log("Hello World from Rust!")
}

#[wasm_bindgen]
pub struct MigsClient {

}

#[wasm_bindgen]
impl MigsClient{

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        log("Graphics Engine is Working!");
        Self{}
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue>{
        log("Update Calculations Delivered!");
        Ok(())
    }

    pub fn render(&self) {
        log("Update Frame Delivered!");
    }
}