use merge_lens_core::diff::diff_two;
use merge_lens_core::types::DiffNode;
use serde_json::json;

#[test]
fn nested_object_change() {
    let base = json!({ "user": { "name": "Alice", "age": 30 } });
    let mine = json!({ "user": { "name": "Alice", "age": 31 } });
    let result = diff_two(&base, &mine);
    match &result.root {
        DiffNode::Object(root_map) => match root_map.get("user") {
            Some(DiffNode::Object(user_map)) => {
                assert!(matches!(user_map.get("name"), Some(DiffNode::Unchanged(_))));
                assert!(matches!(user_map.get("age"), Some(DiffNode::Modified { conflict: false, .. })));
            }
            _ => panic!("expected nested Object"),
        },
        _ => panic!("expected Object"),
    }
}
