use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SetLoginItemSettings {
    open_at_login: Option<bool>,
    open_as_hidden: Option<bool>, // FIXME: macos
    path: Option<JsString>,       // FIXME: windows
    args: JsString,               // FIXME: windows
}

#[wasm_bindgen]
impl SetLoginItemSettings {
    #[wasm_bindgen(constructor)]
    pub fn new(
        open_at_login: Option<bool>,
        open_as_hidden: Option<bool>, // FIXME: macos
        path: Option<JsString>,       // FIXME: windows
        args: JsString,               // FIXME: windows
    ) -> SetLoginItemSettings {
        SetLoginItemSettings {
            open_at_login,
            open_as_hidden,
            path,
            args,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn open_at_login(&self) -> Option<bool> {
        self.open_at_login
    }

    #[wasm_bindgen(setter)]
    pub fn set_open_at_login(&mut self, open_at_login: Option<bool>) {
        self.open_at_login = open_at_login;
    }

    #[wasm_bindgen(getter)]
    pub fn open_as_hidden(&self) -> Option<bool> {
        self.open_as_hidden
    }

    #[wasm_bindgen(getter)]
    pub fn path(&self) -> Option<JsString> {
        self.path.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_path(&mut self, path: Option<JsString>) {
        self.path = path;
    }

    #[wasm_bindgen(getter)]
    pub fn args(&self) -> JsString {
        self.args.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_args(&mut self, args: JsString) {
        self.args = args;
    }
}