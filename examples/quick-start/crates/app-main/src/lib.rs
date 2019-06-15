use electron_sys::{BrowserWindow, app, browser_window};
use wasm_bindgen::{JsCast, prelude::*};

fn create_window() -> Result<(), JsValue> {
    let options = browser_window::Options {
        height: 800,
        width: 600,
    };
    let window = BrowserWindow::new(Some(options));
    window.load_file("..\\..\\..\\..\\..\\index.html".into()); // FIXME
    Ok(())
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let clo = Closure::wrap(Box::new(|| {
        create_window().unwrap();
    }) as Box<dyn FnMut()>);
    app.on("ready".into(), clo.as_ref().unchecked_ref());
    clo.forget();
    Ok(())
}