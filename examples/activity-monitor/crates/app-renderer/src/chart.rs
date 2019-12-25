use js_sys::JsString;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct ChartConfiguration {
    kind: Option<JsString>,
    data: Option<ChartData>,
    options: Option<ChartOptions>,
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct ChartData {
    labels: Option<Box<[JsValue]>>,
    datasets: Option<Box<[JsValue]>>,
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct ChartLegendOptions {
    display: Option<bool>,
    labels: Option<ChartLegendLabelOptions>,
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct ChartLegendLabelOptions {
    font_color: Option<JsString>,
    font_size: Option<u8>,
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct ChartTitleOptions {
    display: Option<bool>,
    font_color: Option<JsString>,
    font_size: Option<u8>,
    text: Option<JsString>,
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct ChartOptions {
    legend: Option<ChartLegendOptions>,
    maintain_aspect_ratio: Option<bool>,
    title: Option<ChartTitleOptions>,
}

#[wasm_bindgen]
extern {
    #[derive(Debug, Clone)]
    pub type ChartDataSets;

    #[wasm_bindgen(method, getter)]
    pub fn data(this: &ChartDataSets) -> Box<[JsValue]>;

    #[wasm_bindgen(method, setter)]
    pub fn set_data(this: &ChartDataSets, value: Box<[JsValue]>);
}

#[wasm_bindgen]
extern {
    #[derive(Debug, Clone)]
    pub type Chart;

    #[wasm_bindgen(constructor)]
    pub fn new(context: &HtmlCanvasElement, options: ChartConfiguration) -> Chart;

    #[wasm_bindgen(method, getter)]
    pub fn data(this: &Chart) -> ChartData;

    #[wasm_bindgen(method)]
    pub fn update(this: &Chart);
}
