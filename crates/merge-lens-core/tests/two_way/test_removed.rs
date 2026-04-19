use merge_lens_core::diff::diff_two;
use merge_lens_core::types::DiffNode;
use serde_json::json;

#[test]
fn key_removed_in_mine() {
    let base = json!({ "name": "Alice", "age": 30 });
    let mine = json!({ "name": "Alice" });
    let result = diff_two(&base, &mine);
    match &result.root {
        DiffNode::Object(map) => {
            assert!(matches!(map.get("name"), Some(DiffNode::Unchanged(_))));
            assert!(matches!(map.get("age"), Some(DiffNode::Removed(v)) if v == &json!(30)));
        }
        _ => panic!("expected Object"),
    }
}
