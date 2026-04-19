mod components;
mod state;

use components::{
    diff_tree::DiffTree, input_panel::InputPanel, merge_output::MergeOutput,
    mode_toggle::ModeToggle,
};
use leptos::prelude::*;
use state::AppState;

#[component]
fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state.clone());

    view! {
        <header class="app-header">
            <h1>"merge-lens"</h1>
            <ModeToggle />
            <Show when=move || {
                state.diff_result.get().map(|d| d.conflict_count > 0).unwrap_or(false)
            }>
                <span class="badge badge-conflict" style="margin-left:auto">
                    {move || state.diff_result.get().map(|d| d.conflict_count).unwrap_or(0)}
                    " conflicts"
                </span>
            </Show>
        </header>
        <main class="app-body">
            <InputPanel />
            <Show when=move || state.diff_result.get().is_some()>
                <DiffTree />
                <MergeOutput />
            </Show>
            <Show when=move || state.error.get().is_some()>
                <div class="error-banner">
                    {move || state.error.get().unwrap_or_default()}
                </div>
            </Show>
        </main>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
