use crate::chart;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn set_last_measure_times() {
    unimplemented!("setLastMeasuretimes")
}

fn get_datasets() {
    unimplemented!("getDatasets")
}

fn update_datasets() {
    unimplemented!("updateDatasets")
}

fn get_cpu_times() {
    unimplemented!("getCpuTimes")
}

fn draw_chart() {
    unimplemented!("draw_chart")
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}
