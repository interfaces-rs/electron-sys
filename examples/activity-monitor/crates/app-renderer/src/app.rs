use node_sys::{os, CpuInfo};
use wasm_bindgen::{prelude::*, JsCast};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[allow(dead_code)]
fn get_cpus() -> impl Iterator<Item = CpuInfo> {
    os::cpus().into_vec().into_iter().map(JsCast::unchecked_into)
}

#[allow(dead_code)]
fn set_last_measure_times<T>(cpus: T)
where
    T: Iterator<Item = CpuInfo>,
{
    for (_i, _cpu) in cpus.enumerate() {
        unimplemented!("FIXME")
    }
}

#[allow(dead_code)]
fn get_datasets() {
    unimplemented!("getDatasets")
}

#[allow(dead_code)]
fn update_datasets() {
    unimplemented!("updateDatasets")
}

#[allow(dead_code)]
fn get_cpu_times(cpu_info: &CpuInfo) -> [f64; 3] {
    let times = cpu_info.times();
    [times.user(), times.sys(), times.idle()]
}

#[allow(dead_code)]
fn draw_chart() {
    unimplemented!("draw_chart")
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}
