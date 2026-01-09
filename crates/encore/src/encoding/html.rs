use irongate_encore::Operation;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HtmlEncode {
    html_encode: irongate_encore::HtmlEncode,
}

#[wasm_bindgen]
impl HtmlEncode {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HtmlEncode {
        HtmlEncode {
            html_encode: irongate_encore::HtmlEncode::new(),
        }
    }

    pub fn apply(&self, input: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.html_encode
            .execute(input)
            .map_err(|err| JsValue::from_str(&format!("{err:?}")))
    }
}

#[wasm_bindgen]
pub struct HtmlDecode {
    html_decode: irongate_encore::HtmlDecode,
}

#[wasm_bindgen]
impl HtmlDecode {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HtmlDecode {
        HtmlDecode {
            html_decode: irongate_encore::HtmlDecode::new(),
        }
    }

    pub fn apply(&self, input: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.html_decode
            .execute(input)
            .map_err(|err| JsValue::from_str(&format!("{err:?}")))
    }
}
