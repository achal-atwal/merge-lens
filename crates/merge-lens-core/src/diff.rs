use crate::types::{DiffNode, DiffResult};
use indexmap::IndexMap;
use serde_json::Value;

pub fn diff_three(base: &Value, mine: &Value, theirs: &Value) -> DiffResult {
    let mut conflict_count = 0;
    let mut auto_merged_count = 0;
    let root = diff_three_values(base, mine, theirs, &mut conflict_count, &mut auto_merged_count);
    DiffResult { root, conflict_count, auto_merged_count }
}

fn diff_three_values(
    base: &Value,
    mine: &Value,
    theirs: &Value,
    conflicts: &mut usize,
    auto: &mut usize,
) -> DiffNode {
    match (base, mine, theirs) {
        (Value::Object(b), Value::Object(m), Value::Object(t)) => {
            use crate::conflict::both_same;
            let mut map = IndexMap::new();
            let mut seen = std::collections::HashSet::new();
            let mut keys: Vec<String> = Vec::new();
            for k in b.keys().chain(m.keys()).chain(t.keys()) {
                if seen.insert(k.clone()) { keys.push(k.clone()); }
            }
            for key in keys {
                let in_base = b.get(&key);
                let in_mine = m.get(&key);
                let in_theirs = t.get(&key);
                let node = match (in_base, in_mine, in_theirs) {
                    (Some(bv), None, None) => DiffNode::Removed(bv.clone()),
                    (None, Some(mv), None) => DiffNode::Added(mv.clone()),
                    (None, None, Some(tv)) => DiffNode::Added(tv.clone()),
                    (None, Some(mv), Some(tv)) => {
                        if both_same(mv, tv) {
                            DiffNode::Added(mv.clone())
                        } else {
                            *conflicts += 1;
                            DiffNode::Modified {
                                base: serde_json::Value::Null,
                                mine: mv.clone(),
                                theirs: Some(tv.clone()),
                                conflict: true,
                            }
                        }
                    }
                    (Some(bv), Some(mv), Some(tv)) => {
                        diff_three_values(bv, mv, tv, conflicts, auto)
                    }
                    (Some(bv), None, Some(tv)) => {
                        // mine deleted; check if theirs is unchanged from base
                        if tv == bv {
                            DiffNode::Removed(bv.clone()) // auto-accept deletion
                        } else {
                            *conflicts += 1;
                            DiffNode::Modified {
                                base: bv.clone(),
                                mine: serde_json::Value::Null,
                                theirs: Some(tv.clone()),
                                conflict: true,
                            }
                        }
                    }
                    (Some(bv), Some(mv), None) => {
                        // theirs deleted; check if mine is unchanged from base
                        if mv == bv {
                            DiffNode::Removed(bv.clone()) // auto-accept deletion
                        } else {
                            *conflicts += 1;
                            DiffNode::Modified {
                                base: bv.clone(),
                                mine: mv.clone(),
                                theirs: Some(serde_json::Value::Null),
                                conflict: true,
                            }
                        }
                    }
                    _ => unreachable!(),
                };
                map.insert(key, node);
            }
            DiffNode::Object(map)
        }
        _ => classify_scalar(base, mine, theirs, conflicts, auto),
    }
}

fn classify_scalar(
    base: &Value,
    mine: &Value,
    theirs: &Value,
    conflicts: &mut usize,
    auto: &mut usize,
) -> DiffNode {
    use crate::conflict::{both_same, is_conflict};

    // Array: treat as opaque — if any version differs, flag as ArrayChange
    if (base.is_array() || mine.is_array() || theirs.is_array()) && !(base == mine && mine == theirs) {
        if mine != theirs {
            *conflicts += 1;
        } else {
            *auto += 1;
        }
        return DiffNode::ArrayChange {
            base: base.clone(),
            mine: mine.clone(),
            theirs: Some(theirs.clone()),
        };
    }

    if mine == theirs && mine == base {
        return DiffNode::Unchanged(base.clone());
    }
    if both_same(mine, theirs) {
        *auto += 1;
        return DiffNode::Modified { base: base.clone(), mine: mine.clone(), theirs: Some(theirs.clone()), conflict: false };
    }
    if is_conflict(base, mine, theirs) {
        *conflicts += 1;
        return DiffNode::Modified { base: base.clone(), mine: mine.clone(), theirs: Some(theirs.clone()), conflict: true };
    }
    // one side changed, other unchanged
    *auto += 1;
    DiffNode::Modified { base: base.clone(), mine: mine.clone(), theirs: Some(theirs.clone()), conflict: false }
}

pub fn diff_two(base: &Value, mine: &Value) -> DiffResult {
    let root = diff_two_values(base, mine);
    DiffResult {
        root,
        conflict_count: 0,
        auto_merged_count: 0,
    }
}

fn diff_two_values(base: &Value, mine: &Value) -> DiffNode {
    match (base, mine) {
        (Value::Object(b), Value::Object(m)) => {
            let mut map = IndexMap::new();
            for (k, bv) in b {
                match m.get(k) {
                    Some(mv) => { map.insert(k.clone(), diff_two_values(bv, mv)); }
                    None => { map.insert(k.clone(), DiffNode::Removed(bv.clone())); }
                }
            }
            for (k, mv) in m {
                if !b.contains_key(k) {
                    map.insert(k.clone(), DiffNode::Added(mv.clone()));
                }
            }
            DiffNode::Object(map)
        }
        _ => {
            if base == mine {
                DiffNode::Unchanged(base.clone())
            } else if base.is_array() || mine.is_array() {
                DiffNode::ArrayChange {
                    base: base.clone(),
                    mine: mine.clone(),
                    theirs: None,
                }
            } else {
                DiffNode::Modified {
                    base: base.clone(),
                    mine: mine.clone(),
                    theirs: None,
                    conflict: false,
                }
            }
        }
    }
}
