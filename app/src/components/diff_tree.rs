use crate::state::AppState;
use leptos::prelude::*;
use merge_lens_core::types::{DiffNode, JsonPath, PathSegment};
use serde_json::Value;

#[component]
pub fn DiffTree() -> impl IntoView {
    let state = expect_context::<AppState>();

    view! {
        <section>
            <div class="label" style="margin-bottom:0.5rem">"Diff Result"</div>
            <div class="diff-tree">
                {move || {
                    state.diff_result.get().map(|diff| {
                        render_node(&diff.root, &mut vec![], state.clone()).into_any()
                    })
                }}
            </div>
        </section>
    }
}

fn format_value(v: &Value) -> String {
    match v {
        Value::String(s) => format!("\"{}\"", s),
        other => other.to_string(),
    }
}

fn render_node(node: &DiffNode, path: &mut JsonPath, state: AppState) -> AnyView {
    match node {
        DiffNode::Unchanged(v) => view! {
            <span class="diff-unchanged">{format_value(v)}</span>
        }.into_any(),

        DiffNode::Added(v) => view! {
            <span class="diff-added">"+ "{format_value(v)}</span>
        }.into_any(),

        DiffNode::Removed(v) => view! {
            <span class="diff-removed">"- "{format_value(v)}</span>
        }.into_any(),

        DiffNode::Modified { base, mine, theirs, conflict } => {
            let show_conflict = *conflict;
            let path_clone = path.clone();
            let state_clone = state.clone();
            if show_conflict {
                view! {
                    <span class="diff-conflict">
                        <span class="badge badge-conflict">"conflict"</span>
                        " "
                        {crate::components::conflict_panel::conflict_panel_inline(
                            base.clone(), mine.clone(), theirs.clone(), path_clone, state_clone
                        )}
                    </span>
                }.into_any()
            } else {
                let resolved = auto_resolve_display(base, mine, theirs.as_ref());
                view! {
                    <span class="diff-modified">"→ "{format_value(&resolved)}</span>
                }.into_any()
            }
        }

        DiffNode::ArrayChange { base, mine, theirs } => {
            let path_clone = path.clone();
            let state_clone = state.clone();
            view! {
                <span class="diff-conflict">
                    <span class="badge badge-conflict">"array conflict"</span>
                    " "
                    {crate::components::conflict_panel::conflict_panel_inline(
                        base.clone(), mine.clone(), theirs.clone(), path_clone, state_clone
                    )}
                </span>
            }.into_any()
        }

        DiffNode::Object(map) => {
            let items: Vec<_> = map.iter().map(|(key, child)| {
                path.push(PathSegment::Key(key.clone()));
                let child_view = render_node(child, path, state.clone());
                path.pop();
                let conflict_count = count_conflicts(child);
                let key = key.clone();
                view! {
                    <div class="diff-node" style="padding-left: 1.25rem">
                        <span class="diff-node-key">{format!("\"{}\":", key)}</span>
                        {if conflict_count > 0 {
                            view! { <span class="badge badge-conflict">{conflict_count}" conflicts"</span> }.into_any()
                        } else {
                            view! { <span /> }.into_any()
                        }}
                        {child_view}
                    </div>
                }
            }).collect();
            view! {
                <div>
                    "{"
                    {items}
                    "}"
                </div>
            }.into_any()
        }
    }
}

fn auto_resolve_display(base: &Value, mine: &Value, theirs: Option<&Value>) -> Value {
    match theirs {
        None => mine.clone(),
        Some(t) => if mine == base { t.clone() } else { mine.clone() }
    }
}

fn count_conflicts(node: &DiffNode) -> usize {
    match node {
        DiffNode::Modified { conflict: true, .. } => 1,
        DiffNode::ArrayChange { theirs: Some(t), mine, .. } if t != mine => 1,
        DiffNode::Object(map) => map.values().map(count_conflicts).sum(),
        _ => 0,
    }
}
