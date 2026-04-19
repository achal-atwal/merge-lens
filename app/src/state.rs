use leptos::prelude::*;
use merge_lens_core::types::{DiffResult, Mode, Resolutions};
use serde_json::Value;

#[derive(Clone)]
pub struct AppState {
    pub mode: RwSignal<Mode>,
    pub base_doc: RwSignal<Option<Value>>,
    pub mine_doc: RwSignal<Option<Value>>,
    pub theirs_doc: RwSignal<Option<Value>>,
    pub diff_result: RwSignal<Option<DiffResult>>,
    pub resolutions: RwSignal<Resolutions>,
    pub error: RwSignal<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            mode: RwSignal::new(Mode::TwoWay),
            base_doc: RwSignal::new(None),
            mine_doc: RwSignal::new(None),
            theirs_doc: RwSignal::new(None),
            diff_result: RwSignal::new(None),
            resolutions: RwSignal::new(std::collections::HashMap::new()),
            error: RwSignal::new(None),
        }
    }
}
