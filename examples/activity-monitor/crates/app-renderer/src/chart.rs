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
impl ChartConfiguration {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChartConfiguration {
        Default::default()
    }

    #[wasm_bindgen(getter, js_name = "type")]
    pub fn kind(&self) -> Option<JsString> {
        self.kind.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_kind(&mut self, value: Option<JsString>) {
        self.kind = value;
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Option<ChartData> {
        self.data.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_data(&mut self, value: Option<ChartData>) {
        self.data = value;
    }

    #[wasm_bindgen(getter)]
    pub fn options(&self) -> Option<ChartOptions> {
        self.options.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_options(&mut self, value: Option<ChartOptions>) {
        self.options = value;
    }
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct ChartData {
    labels: Option<Box<[JsValue]>>,
    datasets: Option<Box<[JsValue]>>,
}

#[wasm_bindgen]
impl ChartData {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChartData {
        Default::default()
    }

    #[wasm_bindgen(getter)]
    pub fn labels(&self) -> Option<Box<[JsValue]>> {
        self.labels.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_labels(&mut self, value: Option<Box<[JsValue]>>) {
        self.labels = value;
    }

    #[wasm_bindgen(getter)]
    pub fn datasets(&self) -> Option<Box<[JsValue]>> {
        self.datasets.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_options(&mut self, value: Option<Box<[JsValue]>>) {
        self.datasets = value;
    }   
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct ChartLegendOptions {
    pub display: Option<bool>,
    labels: Option<ChartLegendLabelOptions>,
}

#[wasm_bindgen]
impl ChartLegendOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChartLegendOptions {
        Default::default()
    }

    #[wasm_bindgen(getter)]
    pub fn labels(&self) -> Option<ChartLegendLabelOptions> {
        self.labels.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_labels(&mut self, value: Option<ChartLegendLabelOptions>) {
        self.labels = value;
    }
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct ChartLegendLabelOptions {
    font_color: Option<JsString>,
    font_size: Option<u8>,
}

#[wasm_bindgen]
impl ChartLegendLabelOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChartLegendLabelOptions {
        Default::default()
    }

    #[wasm_bindgen(getter, js_name = "fontColor")]
    pub fn font_color(&self) -> Option<JsString> {
        self.font_color.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_font_color(&mut self, value: Option<JsString>) {
        self.font_color = value;
    }

    #[wasm_bindgen(getter, js_name = "fontSize")]
    pub fn font_size(&self) -> Option<u8> {
        self.font_size
    }

    #[wasm_bindgen(setter)]
    pub fn set_font_size(&mut self, value: Option<u8>) {
        self.font_size = value;
    }
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct ChartTitleOptions {
    pub display: Option<bool>,
    font_color: Option<JsString>,
    font_size: Option<u8>,
    text: Option<JsString>,
}

#[wasm_bindgen]
impl ChartTitleOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChartTitleOptions {
        Default::default()
    }

    #[wasm_bindgen(getter, js_name = "fontColor")]
    pub fn font_color(&self) -> Option<JsString> {
        self.font_color.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_font_color(&mut self, value: Option<JsString>) {
        self.font_color = value;
    }

    #[wasm_bindgen(getter, js_name = "fontSize")]
    pub fn font_size(&self) -> Option<u8> {
        self.font_size
    }

    #[wasm_bindgen(setter)]
    pub fn set_font_size(&mut self, value: Option<u8>) {
        self.font_size = value;
    }

    #[wasm_bindgen(getter)]
    pub fn text(&self) -> Option<JsString> {
        self.text.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_text(&mut self, value: Option<JsString>) {
        self.text = value;
    }
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct ChartOptions {
    legend: Option<ChartLegendOptions>,
    maintain_aspect_ratio: Option<bool>,
    title: Option<ChartTitleOptions>,
}

#[wasm_bindgen]
impl ChartOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChartOptions {
        Default::default()
    }

    #[wasm_bindgen(getter)]
    pub fn legend(&self) -> Option<ChartLegendOptions> {
        self.legend.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_legend(&mut self, value: Option<ChartLegendOptions>) {
        self.legend = value;
    }

    #[wasm_bindgen(getter, js_name = "maintainAspectRatio")]
    pub fn maintain_aspect_ratio(&self) -> Option<bool> {
        self.maintain_aspect_ratio
    }

    #[wasm_bindgen(setter)]
    pub fn set_maintain_aspect_ratio(&mut self, value: Option<bool>) {
        self.maintain_aspect_ratio = value;
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> Option<ChartTitleOptions> {
        self.title.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_title(&mut self, value: Option<ChartTitleOptions>) {
        self.title = value;
    }
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
