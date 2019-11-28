use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = Object)]
    /// Docs: http://electronjs.org/docs/api/native-image
    pub type NativeImage;
}
