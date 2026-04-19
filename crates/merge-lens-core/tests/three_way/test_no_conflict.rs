use merge_lens_core::diff::diff_three;
use merge_lens_core::types::DiffNode;
use serde_json::json;

#[test]
fn mine_changed_theirs_unchanged() {
    let base = json!({ "age": 30 });
    let mine = json!({ "age": 31 });
    let theirs = json!({ "age": 30 });
    let result = diff_three(&base, &mine, &theirs);
    assert_eq!(result.conflict_count, 0);
    assert_eq!(result.auto_merged_count, 1);
    match &result.root {
        DiffNode::Object(map) => match map.get("age") {
            Some(DiffNode::Modified { conflict: false, mine: m, theirs: Some(t), .. }) => {
                assert_eq!(m, &json!(31));
                assert_eq!(t, &json!(30));
            }
            _ => panic!("expected non-conflict Modified"),
        },
        _ => panic!("expected Object"),
    }
}

#[test]
fn theirs_changed_mine_unchanged() {
    let base = json!({ "age": 30 });
    let mine = json!({ "age": 30 });
    let theirs = json!({ "age": 32 });
    let result = diff_three(&base, &mine, &theirs);
    assert_eq!(result.conflict_count, 0);
    assert_eq!(result.auto_merged_count, 1);
    match &result.root {
        DiffNode::Object(map) => {
            assert!(matches!(map.get("age"), Some(DiffNode::Modified { conflict: false, .. })));
        }
        _ => panic!("expected Object"),
    }
}

#[test]
fn both_changed_to_same_value() {
    let base = json!({ "name": "Alice" });
    let mine = json!({ "name": "Alicia" });
    let theirs = json!({ "name": "Alicia" });
    let result = diff_three(&base, &mine, &theirs);
    assert_eq!(result.conflict_count, 0);
    assert_eq!(result.auto_merged_count, 1);
}

#[test]
fn key_only_in_mine() {
    let base = json!({});
    let mine = json!({ "new_key": 42 });
    let theirs = json!({});
    let result = diff_three(&base, &mine, &theirs);
    assert_eq!(result.conflict_count, 0);
    match &result.root {
        DiffNode::Object(map) => {
            assert!(matches!(map.get("new_key"), Some(DiffNode::Added(_))));
        }
        _ => panic!("expected Object"),
    }
}

#[test]
fn key_only_in_theirs() {
    let base = json!({});
    let mine = json!({});
    let theirs = json!({ "new_key": 99 });
    let result = diff_three(&base, &mine, &theirs);
    assert_eq!(result.conflict_count, 0);
    match &result.root {
        DiffNode::Object(map) => {
            assert!(matches!(map.get("new_key"), Some(DiffNode::Added(_))));
        }
        _ => panic!("expected Object"),
    }
}

#[test]
fn both_added_same_key_same_value() {
    let base = json!({});
    let mine = json!({ "x": 1 });
    let theirs = json!({ "x": 1 });
    let result = diff_three(&base, &mine, &theirs);
    assert_eq!(result.conflict_count, 0);
    match &result.root {
        DiffNode::Object(map) => {
            assert!(matches!(map.get("x"), Some(DiffNode::Added(_))));
        }
        _ => panic!("expected Object"),
    }
}
