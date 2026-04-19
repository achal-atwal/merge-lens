use crate::state::AppState;
use leptos::prelude::*;
use merge_lens_core::types::{JsonPath, Resolution};
use serde_json::Value;
use std::sync::Arc;

/// Inline conflict resolution panel shown under a conflicted DiffNode.
/// Not a #[component] — called as a plain function from diff_tree.rs.
pub fn conflict_panel_inline(
    base: Value,
    mine: Value,
    theirs: Option<Value>,
    path: JsonPath,
    state: AppState,
) -> impl IntoView {
    let path = Arc::new(path);

    // Single Memo<Option<Resolution>> — Memo is Copy in Leptos 0.7, so it can be
    // captured by multiple independent move || closures without ownership conflicts.
    let current = Memo::new({
        let p = path.clone();
        let s = state.clone();
        move |_| s.resolutions.get().get(p.as_ref()).cloned()
    });

    // Accept closures — inserting overwrites any previous resolution, so re-clicking works.
    let accept_base = { let p = path.clone(); let s = state.clone(); move |_| s.resolutions.update(|r| { r.insert((*p).clone(), Resolution::Base); }) };
    let accept_mine = { let p = path.clone(); let s = state.clone(); move |_| s.resolutions.update(|r| { r.insert((*p).clone(), Resolution::Mine); }) };

    // Custom input state
    let custom_text = RwSignal::new(String::new());
    let custom_error = RwSignal::new(Option::<String>::None);
    let apply_custom = {
        let p = path.clone();
        let s = state.clone();
        move |_| {
            let text = custom_text.get();
            match serde_json::from_str::<Value>(&text) {
                Ok(v) => {
                    custom_error.set(None);
                    s.resolutions.update(|r| { r.insert((*p).clone(), Resolution::Custom(v)); });
                }
                Err(e) => {
                    custom_error.set(Some(format!("Invalid JSON: {e}")));
                }
            }
        }
    };

    let base_str = serde_json::to_string(&base).unwrap_or_default();
    let mine_str = serde_json::to_string(&mine).unwrap_or_default();

    let theirs_view = theirs.map(|t| {
        let theirs_str = serde_json::to_string(&t).unwrap_or_default();
        let accept_theirs = { let p = path.clone(); let s = state.clone(); move |_| s.resolutions.update(|r| { r.insert((*p).clone(), Resolution::Theirs); }) };
        view! {
            <div class="conflict-row">
                <span class="conflict-label">"Theirs"</span>
                <span class="conflict-value">{theirs_str}</span>
                <button
                    class=move || if current.get() == Some(Resolution::Theirs) { "btn-ghost active" } else { "btn-ghost" }
                    style="font-size:0.8rem"
                    on:click=accept_theirs
                >"Accept"</button>
            </div>
        }
    });

    view! {
        <div class="conflict-panel">
            <Show when=move || current.get().is_some()>
                <div style="color:#4ade80;font-size:0.8rem">{move || match current.get() {
                    Some(Resolution::Base)      => "✓ Base accepted",
                    Some(Resolution::Mine)      => "✓ Mine accepted",
                    Some(Resolution::Theirs)    => "✓ Theirs accepted",
                    Some(Resolution::Custom(_)) => "✓ Custom value accepted",
                    None                        => "",
                }}</div>
            </Show>
            <div class="conflict-row">
                <span class="conflict-label">"Base"</span>
                <span class="conflict-value">{base_str}</span>
                <button
                    class=move || if current.get() == Some(Resolution::Base) { "btn-ghost active" } else { "btn-ghost" }
                    style="font-size:0.8rem"
                    on:click=accept_base
                >"Accept"</button>
            </div>
            <div class="conflict-row">
                <span class="conflict-label">"Mine"</span>
                <span class="conflict-value">{mine_str}</span>
                <button
                    class=move || if current.get() == Some(Resolution::Mine) { "btn-ghost active" } else { "btn-ghost" }
                    style="font-size:0.8rem"
                    on:click=accept_mine
                >"Accept"</button>
            </div>
            {theirs_view}
            <div class="conflict-row">
                <span class="conflict-label">"Custom"</span>
                <input
                    type="text"
                    placeholder="Enter any valid JSON value..."
                    prop:value=move || custom_text.get()
                    on:input=move |e| custom_text.set(event_target_value(&e))
                    style="flex:1;background:#0f1117;border:1px solid #334155;border-radius:4px;color:#e2e8f0;font-family:monospace;font-size:0.85rem;padding:0.35rem 0.5rem"
                />
                <button
                    class=move || if matches!(current.get(), Some(Resolution::Custom(_))) { "btn-ghost active" } else { "btn-ghost" }
                    style="font-size:0.8rem"
                    on:click=apply_custom
                >"Apply"</button>
            </div>
            <Show when=move || custom_error.get().is_some()>
                <div class="error-text">{move || custom_error.get().unwrap_or_default()}</div>
            </Show>
        </div>
    }
}
