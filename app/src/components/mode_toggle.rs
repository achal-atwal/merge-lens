use crate::state::AppState;
use leptos::prelude::*;
use merge_lens_core::types::Mode;

#[component]
pub fn ModeToggle() -> impl IntoView {
    let state = expect_context::<AppState>();

    view! {
        <div class="mode-toggle">
            <button
                class=move || if state.mode.get() == Mode::TwoWay { "btn-ghost active" } else { "btn-ghost" }
                on:click=move |_| {
                    state.mode.set(Mode::TwoWay);
                    state.diff_result.set(None);
                    state.resolutions.set(std::collections::HashMap::new());
                }
            >
                "2-Way Diff"
            </button>
            <button
                class=move || if state.mode.get() == Mode::ThreeWay { "btn-ghost active" } else { "btn-ghost" }
                on:click=move |_| {
                    state.mode.set(Mode::ThreeWay);
                    state.diff_result.set(None);
                    state.resolutions.set(std::collections::HashMap::new());
                }
            >
                "3-Way Merge"
            </button>
        </div>
    }
}
