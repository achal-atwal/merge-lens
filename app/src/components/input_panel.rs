use crate::state::AppState;
use leptos::prelude::*;
use merge_lens_core::{diff::{diff_two, diff_three}, types::Mode};
use serde_json::Value;
use std::rc::Rc;

#[component]
fn JsonEditor(
    label: &'static str,
    on_change: impl Fn(Option<Value>) + 'static,
) -> impl IntoView {
    let error = RwSignal::new(Option::<String>::None);
    let text = RwSignal::new(String::new());
    let on_change = Rc::new(on_change);

    let on_change_input = on_change.clone();
    let on_input = move |ev: leptos::ev::Event| {
        let val = event_target_value(&ev);
        text.set(val.clone());
        if val.trim().is_empty() {
            error.set(None);
            on_change_input(None);
        } else {
            match serde_json::from_str::<Value>(&val) {
                Ok(parsed) => { error.set(None); on_change_input(Some(parsed)); }
                Err(e) => { error.set(Some(e.to_string())); on_change_input(None); }
            }
        }
    };

    let on_change_file = on_change.clone();
    let on_file = move |ev: leptos::ev::Event| {
        use wasm_bindgen::JsCast;
        use web_sys::{FileReader, HtmlInputElement};
        let input = ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        if let Some(file) = input.files().and_then(|f| f.get(0)) {
            let reader = FileReader::new().unwrap();
            let reader2 = reader.clone();
            let on_change_inner = on_change_file.clone();
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                let s = reader2.result().ok()
                    .and_then(|r| r.as_string())
                    .unwrap_or_default();
                text.set(s.clone());
                match serde_json::from_str::<Value>(&s) {
                    Ok(parsed) => { error.set(None); on_change_inner(Some(parsed)); }
                    Err(e) => { error.set(Some(e.to_string())); on_change_inner(None); }
                }
            }) as Box<dyn FnMut(_)>);
            reader.set_onload(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
            reader.read_as_text(&file).unwrap();
        }
    };

    view! {
        <div class="editor-block">
            <div class="label">{label}</div>
            <textarea
                prop:value=move || text.get()
                on:input=on_input
                placeholder="Paste JSON here..."
            />
            <input type="file" accept=".json" on:change=on_file style="font-size:0.8rem;color:#64748b;margin-top:4px;" />
            <Show when=move || error.get().is_some()>
                <div class="error-text">{move || error.get().unwrap_or_default()}</div>
            </Show>
        </div>
    }
}

#[component]
pub fn InputPanel() -> impl IntoView {
    let state = expect_context::<AppState>();

    let can_run = move || {
        let has_base = state.base_doc.get().is_some();
        let has_mine = state.mine_doc.get().is_some();
        let has_theirs = state.theirs_doc.get().is_some();
        match state.mode.get() {
            Mode::TwoWay => has_base && has_mine,
            Mode::ThreeWay => has_base && has_mine && has_theirs,
        }
    };

    let run_diff = move |_| {
        let base = match state.base_doc.get() { Some(v) => v, None => return };
        let mine = match state.mine_doc.get() { Some(v) => v, None => return };
        state.error.set(None);
        state.resolutions.set(std::collections::HashMap::new());
        let result = match state.mode.get() {
            Mode::TwoWay => diff_two(&base, &mine),
            Mode::ThreeWay => {
                let theirs = match state.theirs_doc.get() { Some(v) => v, None => return };
                diff_three(&base, &mine, &theirs)
            }
        };
        state.diff_result.set(Some(result));
    };

    view! {
        <section>
            <div
                class=move || match state.mode.get() {
                    Mode::TwoWay => "editors-grid two-col",
                    Mode::ThreeWay => "editors-grid three-col",
                }
            >
                <JsonEditor
                    label="Base"
                    on_change=move |v| state.base_doc.set(v)
                />
                <JsonEditor
                    label="Mine"
                    on_change=move |v| state.mine_doc.set(v)
                />
                <Show when=move || state.mode.get() == Mode::ThreeWay>
                    <JsonEditor
                        label="Theirs"
                        on_change=move |v| state.theirs_doc.set(v)
                    />
                </Show>
            </div>
            <button
                class="btn-primary"
                disabled=move || !can_run()
                on:click=run_diff
                style="margin-top:1rem"
            >
                "Run Diff"
            </button>
        </section>
    }
}
