use merge_lens_core::{
    diff::{diff_three, diff_two},
    merge::apply_resolutions,
    types::{DiffResult, Resolutions},
};
use serde_json::Value;
use wasm_bindgen::prelude::*;

/// Compute a 2-way diff between base and mine.
/// Returns serialized DiffResult as JSON string.
#[wasm_bindgen]
pub fn wasm_diff_two(base: &str, mine: &str) -> Result<String, JsValue> {
    let base: Value = serde_json::from_str(base).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let mine: Value = serde_json::from_str(mine).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let result = diff_two(&base, &mine);
    serde_json::to_string(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Compute a 3-way diff between base, mine, and theirs.
/// Returns serialized DiffResult as JSON string.
#[wasm_bindgen]
pub fn wasm_diff_three(base: &str, mine: &str, theirs: &str) -> Result<String, JsValue> {
    let base: Value = serde_json::from_str(base).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let mine: Value = serde_json::from_str(mine).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let theirs: Value =
        serde_json::from_str(theirs).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let result = diff_three(&base, &mine, &theirs);
    serde_json::to_string(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Apply resolutions to a DiffResult and produce a merged JSON value.
/// diff_json: serialized DiffResult. resolutions_json: serialized Resolutions.
/// Returns serialized MergeResult as JSON string.
#[wasm_bindgen]
pub fn wasm_apply_merge(diff_json: &str, resolutions_json: &str) -> Result<String, JsValue> {
    let diff: DiffResult =
        serde_json::from_str(diff_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let resolutions: Resolutions =
        serde_json::from_str(resolutions_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let result = apply_resolutions(&diff, &resolutions);
    serde_json::to_string(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}
