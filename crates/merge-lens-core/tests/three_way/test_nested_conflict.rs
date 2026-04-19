use merge_lens_core::diff::diff_three;
use merge_lens_core::types::DiffNode;
use serde_json::json;

#[test]
fn conflict_inside_nested_object() {
    let base = json!({ "user": { "name": "Alice", "age": 30 } });
    let mine = json!({ "user": { "name": "Bob", "age": 31 } });
    let theirs = json!({ "user": { "name": "Carol", "age": 30 } });
    let result = diff_three(&base, &mine, &theirs);
    assert_eq!(result.conflict_count, 1); // only "name" conflicts; "age" is mine-only change
    match &result.root {
        DiffNode::Object(root) => match root.get("user") {
            Some(DiffNode::Object(user)) => {
                assert!(matches!(user.get("name"), Some(DiffNode::Modified { conflict: true, .. })));
                assert!(matches!(user.get("age"), Some(DiffNode::Modified { conflict: false, .. })));
            }
            _ => panic!("expected nested Object"),
        },
        _ => panic!("expected Object"),
    }
}
