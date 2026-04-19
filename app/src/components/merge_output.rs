use crate::state::AppState;
use leptos::prelude::*;
use merge_lens_core::merge::apply_resolutions;
use merge_lens_core::types::Mode;
use wasm_bindgen::JsCast;

#[component]
pub fn MergeOutput() -> impl IntoView {
    let state = expect_context::<AppState>();

    // In 2-way mode the merged output is gated behind an explicit "View Merged JSON"
    // button (since there are no conflicts to resolve as a natural gate).
    // Reset whenever a new diff is run so the button reappears.
    let show_merged = RwSignal::new(false);
    Effect::new(move |_| {
        state.diff_result.track();
        show_merged.set(false);
    });

    let merge_result = move || {
        let diff = state.diff_result.get()?;
        let resolutions = state.resolutions.get();
        Some(apply_resolutions(&diff, &resolutions))
    };

    let unresolved_count = move || {
        merge_result().map(|r| r.unresolved.len()).unwrap_or(0)
    };

    let merged_json = move || {
        merge_result()
            .and_then(|r| if r.unresolved.is_empty() {
                serde_json::to_string_pretty(&r.merged).ok()
            } else {
                None
            })
    };

    // 2-way: user must click "View Merged JSON"
    // 3-way: auto-reveal once all conflicts are resolved
    let can_view = move || match state.mode.get() {
        Mode::TwoWay  => show_merged.get(),
        Mode::ThreeWay => unresolved_count() == 0,
    };

    let copy_to_clipboard = move |_| {
        if let Some(json) = merged_json() {
            let window = web_sys::window().unwrap();
            let navigator = window.navigator();
            let _ = navigator.clipboard().write_text(&json);
        }
    };

    let download = move |_| {
        if let Some(json) = merged_json() {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let blob_options = web_sys::BlobPropertyBag::new();
            blob_options.set_type("application/json");
            let blob = web_sys::Blob::new_with_str_sequence_and_options(
                &js_sys::Array::of1(&wasm_bindgen::JsValue::from_str(&json)),
                &blob_options,
            ).unwrap();
            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
            let a = document.create_element("a").unwrap()
                .dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
            a.set_href(&url);
            a.set_download("merged.json");
            a.click();
            web_sys::Url::revoke_object_url(&url).unwrap();
        }
    };

    view! {
        <section class="merge-output">
            <div style="display:flex;align-items:center;gap:1rem;margin-bottom:0.75rem">
                <div class="label">"Merged Output"</div>
                <Show when=move || { unresolved_count() > 0 }>
                    <span class="badge badge-conflict">
                        {move || unresolved_count()}" conflicts remaining"
                    </span>
                </Show>
                <Show when=move || can_view()>
                    <div style="display:flex;gap:0.5rem;margin-left:auto">
                        <button class="btn-ghost" style="font-size:0.8rem" on:click=copy_to_clipboard>"Copy"</button>
                        <button class="btn-ghost" style="font-size:0.8rem" on:click=download>"Download .json"</button>
                    </div>
                </Show>
            </div>
            <Show
                when=move || can_view()
                fallback=move || view! {
                    // 2-way: show the "View Merged JSON" button as the gate
                    // 3-way: show "X conflicts remaining" message
                    <Show
                        when=move || state.mode.get() == Mode::TwoWay
                        fallback=move || view! {
                            <div style="color:#475569;font-size:0.875rem">
                                "Resolve all conflicts above to view merged JSON."
                            </div>
                        }
                    >
                        <button
                            class="btn-primary"
                            style="font-size:0.875rem"
                            on:click=move |_| show_merged.set(true)
                        >"View Merged JSON"</button>
                    </Show>
                }
            >
                <pre class="json-output">{move || merged_json().unwrap_or_default()}</pre>
            </Show>
        </section>
    }
}
