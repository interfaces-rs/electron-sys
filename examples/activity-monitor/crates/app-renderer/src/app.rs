use crate::chart::{
    Chart,
    ChartConfiguration,
    ChartData,
    ChartLegendLabelOptions,
    ChartLegendOptions,
    ChartOptions,
    ChartTitleOptions,
};
use node_sys::{os, CpuInfo};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlCanvasElement;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

thread_local! {
    static CHART: Chart = {
        let document = web_sys::window().unwrap_throw().document().unwrap_throw();
        let context = document
            .get_element_by_id("text-input")
            .unwrap_throw()
            .unchecked_into::<HtmlCanvasElement>();
        let options = {
            let mut options = ChartConfiguration::new();
            options.set_kind(Some("doughnut".into()));
            options.set_data(Some({
                let mut data = ChartData::new();
                data.set_labels(Some(vec![
                    "User Time (ms)".into(),
                    "System Time (ms)".into(),
                    "Idle Time (ms)".into(),
                ].into_boxed_slice()));
                data.set_datasets(Some(vec![
                    // FIXME
                ].into_boxed_slice()));
                data
            }));
            options.set_options(Some({
                let mut options = ChartOptions::new();
                options.set_maintain_aspect_ratio(Some(false));
                options.set_title(Some({
                    let mut title = ChartTitleOptions::new();
                    title.set_display(Some(true));
                    title.set_text(Some("CPU Activity".into()));
                    title.set_font_color(Some("rgb(250, 250, 250)".into()));
                    title.set_font_size(Some(16u8));
                    title
                }));
                options.set_legend(Some({
                    let mut legend = ChartLegendOptions::new();
                    legend.set_display(Some(true));
                    legend.set_labels(Some({
                        let mut labels = ChartLegendLabelOptions::new();
                        labels.set_font_color(Some("rgb(250, 250, 250)".into()));
                        labels.set_font_size(Some(12u8));
                        labels
                    }));
                    legend
                }));
                options
            }));
            options
        };
        Chart::new(&context, options)
    };
}

thread_local! {
    static LAST_MEASURE_TIMES: Box<[[f64; 3]]> = Box::new([]);
}

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
