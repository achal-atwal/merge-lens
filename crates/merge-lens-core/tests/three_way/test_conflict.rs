use merge_lens_core::diff::diff_three;
use merge_lens_core::types::DiffNode;
use serde_json::json;

#[test]
fn both_changed_differently() {
    let base = json!({ "age": 30 });
    let mine = json!({ "age": 31 });
    let theirs = json!({ "age": 32 });
    let result = diff_three(&base, &mine, &theirs);
    assert_eq!(result.conflict_count, 1);
    match &result.root {
        DiffNode::Object(map) => {
            assert!(matches!(map.get("age"), Some(DiffNode::Modified { conflict: true, .. })));
        }
        _ => panic!("expected Object"),
    }
}

#[test]
fn both_added_same_key_different_values() {
    let base = json!({});
    let mine = json!({ "x": 1 });
    let theirs = json!({ "x": 2 });
    let result = diff_three(&base, &mine, &theirs);
    assert_eq!(result.conflict_count, 1);
    match &result.root {
        DiffNode::Object(map) => {
            assert!(matches!(map.get("x"), Some(DiffNode::Modified { conflict: true, .. })));
        }
        _ => panic!("expected Object"),
    }
}

#[test]
fn array_conflict() {
    let base = json!({ "tags": ["a", "b"] });
    let mine = json!({ "tags": ["a", "c"] });
    let theirs = json!({ "tags": ["a", "d"] });
    let result = diff_three(&base, &mine, &theirs);
    assert_eq!(result.conflict_count, 1);
    match &result.root {
        DiffNode::Object(map) => {
            assert!(matches!(map.get("tags"), Some(DiffNode::ArrayChange { theirs: Some(_), .. })));
        }
        _ => panic!("expected Object"),
    }
}
