use electron_sys::{app, BrowserWindow, BrowserWindowOptions, WebPreferences};
use node_sys::{__dirname, path};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let on_ready = Closure::wrap(Box::new(|| {
        let win = BrowserWindow::new(Some({
            let mut opts = <BrowserWindowOptions as Default>::default();
            opts.set_width(Some(640));
            opts.set_height(Some(480));
            opts.set_web_preferences(Some({
                let mut opts = <WebPreferences as Default>::default();
                // Disable node integration in remote page
                opts.set_node_integration(Some(false));
                opts.set_preload(Some(path::resolve(
                    vec![__dirname.clone().into(), "index.js".into()].into_boxed_slice(),
                )));
                opts
            }));
            opts.set_show(Some(false));
            opts
        }));
        // open the dev tools panel (undocked)
        {
            let activate = Some(false);
            let mode = "undocked".into();
            let options = Some(electron_sys::OpenDevToolsOptions::new(activate, mode));
            win.web_contents().open_dev_tools(options);
        }
        // load the html file
        win.load_file(
            &path::join(vec!["..".into(), "..".into(), "..".into(), "index.html".into()].into_boxed_slice()),
            None,
        );
        // show the window when ready
        let ready_to_show = {
            let win = win.clone();
            Closure::wrap(Box::new(move || {
                win.show();
            }) as Box<dyn Fn()>)
        };
        win.once("ready-to-show", ready_to_show.as_ref().unchecked_ref());
        ready_to_show.forget();
    }) as Box<dyn Fn()>);
    app.on("ready".into(), on_ready.as_ref().unchecked_ref());
    on_ready.forget();
    Ok(())
}
