use crate::types::{DiffNode, DiffResult, JsonPath, MergeResult, PathSegment, Resolution, Resolutions};
use serde_json::Value;

pub fn apply_resolutions(diff: &DiffResult, resolutions: &Resolutions) -> MergeResult {
    let mut unresolved = Vec::new();
    let merged = apply_node(&diff.root, resolutions, &mut Vec::new(), &mut unresolved)
        .unwrap_or(Value::Null);
    MergeResult { merged, unresolved }
}

/// Returns None if the node represents a deletion (should be excluded from output).
fn apply_node(
    node: &DiffNode,
    resolutions: &Resolutions,
    path: &mut JsonPath,
    unresolved: &mut Vec<JsonPath>,
) -> Option<Value> {
    match node {
        DiffNode::Unchanged(v) => Some(v.clone()),
        DiffNode::Added(v) => Some(v.clone()),
        DiffNode::Removed(_) => None,

        DiffNode::Modified { base, mine, theirs, conflict } => {
            if !conflict {
                Some(auto_resolve(base, mine, theirs.as_ref()))
            } else {
                match resolutions.get(&*path) {
                    Some(Resolution::Mine)       => Some(mine.clone()),
                    Some(Resolution::Theirs)     => Some(theirs.as_ref().unwrap_or(base).clone()),
                    Some(Resolution::Base)       => Some(base.clone()),
                    Some(Resolution::Custom(v))  => Some(v.clone()),
                    None => {
                        unresolved.push(path.clone());
                        Some(Value::Null) // placeholder
                    }
                }
            }
        }

        DiffNode::ArrayChange { base, mine, theirs } => match theirs {
            None => match resolutions.get(&*path) {
                Some(Resolution::Base)      => Some(base.clone()),
                Some(Resolution::Custom(v)) => Some(v.clone()),
                _                           => Some(mine.clone()),
            },
            Some(t) => match resolutions.get(&*path) {
                Some(Resolution::Mine)      => Some(mine.clone()),
                Some(Resolution::Theirs)    => Some(t.clone()),
                Some(Resolution::Base)      => Some(base.clone()),
                Some(Resolution::Custom(v)) => Some(v.clone()),
                None => {
                    if mine == t {
                        Some(mine.clone()) // both changed to same array, auto-accept
                    } else {
                        unresolved.push(path.clone());
                        Some(Value::Null)
                    }
                }
            },
        },

        DiffNode::Object(map) => {
            let mut result = serde_json::Map::new();
            for (key, child) in map {
                path.push(PathSegment::Key(key.clone()));
                if let Some(value) = apply_node(child, resolutions, path, unresolved) {
                    result.insert(key.clone(), value);
                }
                path.pop();
            }
            Some(Value::Object(result))
        }
    }
}

/// Determines the auto-resolved value for a non-conflict Modified node.
/// At least one of: mine == base, theirs == base, or mine == theirs.
fn auto_resolve(base: &Value, mine: &Value, theirs: Option<&Value>) -> Value {
    match theirs {
        None => mine.clone(), // 2-way: take mine
        Some(t) => {
            if mine == base {
                t.clone() // only theirs changed
            } else {
                mine.clone() // mine changed (or both changed to same value)
            }
        }
    }
}
