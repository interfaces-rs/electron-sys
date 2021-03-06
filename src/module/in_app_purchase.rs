use js_sys::{JsString, Promise};
use node_sys::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type InAppPurchase;

    #[wasm_bindgen(js_name = "inAppPurchase")]
    pub static in_app_purchase: InAppPurchase;

    // FIXME: event overloads

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "canMakePayments")]
    pub fn can_make_payments(this: &InAppPurchase) -> bool;

    #[wasm_bindgen(method, js_name = "finishAllTransactions")]
    pub fn finish_all_transactions(this: &InAppPurchase);

    #[wasm_bindgen(method, js_name = "finishTransactionsByDate")]
    pub fn finish_transactions_by_date(this: &InAppPurchase, date: &str);

    #[must_use]
    #[wasm_bindgen(method, js_name = "getProducts")]
    pub fn get_products(this: &InAppPurchase, product_ids: Box<[JsValue]>) -> Promise;

    #[wasm_bindgen(method, js_name = "getReceiptsURL")]
    pub fn get_receipts_url(this: &InAppPurchase) -> JsString;

    #[must_use]
    #[wasm_bindgen(method, js_name = "purchaseProduct")]
    pub fn purchaseProduct(this: &InAppPurchase, product_id: &str, quantity: Option<u32>) -> Promise;
}
