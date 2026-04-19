use merge_lens_core::diff::diff_two;
use merge_lens_core::types::DiffNode;
use serde_json::json;

#[test]
fn scalar_value_changed() {
    let base = json!({ "age": 30 });
    let mine = json!({ "age": 31 });
    let result = diff_two(&base, &mine);
    match &result.root {
        DiffNode::Object(map) => match map.get("age") {
            Some(DiffNode::Modified { base: b, mine: m, theirs: None, conflict: false }) => {
                assert_eq!(b, &json!(30));
                assert_eq!(m, &json!(31));
            }
            _ => panic!("expected Modified"),
        },
        _ => panic!("expected Object"),
    }
}

#[test]
fn unchanged_value() {
    let base = json!({ "name": "Alice" });
    let mine = json!({ "name": "Alice" });
    let result = diff_two(&base, &mine);
    match &result.root {
        DiffNode::Object(map) => {
            assert!(matches!(map.get("name"), Some(DiffNode::Unchanged(_))));
        }
        _ => panic!("expected Object"),
    }
}
