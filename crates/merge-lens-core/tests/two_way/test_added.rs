use merge_lens_core::diff::diff_two;
use merge_lens_core::types::DiffNode;
use serde_json::json;

#[test]
fn key_added_in_mine() {
    let base = json!({});
    let mine = json!({ "name": "Alice" });
    let result = diff_two(&base, &mine);
    assert_eq!(result.conflict_count, 0);
    match &result.root {
        DiffNode::Object(map) => {
            assert!(matches!(map.get("name"), Some(DiffNode::Added(v)) if v == &json!("Alice")));
        }
        _ => panic!("expected Object"),
    }
}

#[test]
fn multiple_keys_added() {
    let base = json!({ "a": 1 });
    let mine = json!({ "a": 1, "b": 2, "c": 3 });
    let result = diff_two(&base, &mine);
    match &result.root {
        DiffNode::Object(map) => {
            assert!(matches!(map.get("a"), Some(DiffNode::Unchanged(_))));
            assert!(matches!(map.get("b"), Some(DiffNode::Added(_))));
            assert!(matches!(map.get("c"), Some(DiffNode::Added(_))));
        }
        _ => panic!("expected Object"),
    }
}
