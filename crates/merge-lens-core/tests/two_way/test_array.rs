use merge_lens_core::diff::diff_two;
use merge_lens_core::types::DiffNode;
use serde_json::json;

#[test]
fn array_changed_is_opaque() {
    let base = json!({ "items": [1, 2, 3] });
    let mine = json!({ "items": [1, 2, 4] });
    let result = diff_two(&base, &mine);
    match &result.root {
        DiffNode::Object(map) => match map.get("items") {
            Some(DiffNode::ArrayChange { base: b, mine: m, theirs: None }) => {
                assert_eq!(b, &json!([1, 2, 3]));
                assert_eq!(m, &json!([1, 2, 4]));
            }
            _ => panic!("expected ArrayChange"),
        },
        _ => panic!("expected Object"),
    }
}

#[test]
fn identical_arrays_are_unchanged() {
    let base = json!({ "items": [1, 2, 3] });
    let mine = json!({ "items": [1, 2, 3] });
    let result = diff_two(&base, &mine);
    match &result.root {
        DiffNode::Object(map) => {
            assert!(matches!(map.get("items"), Some(DiffNode::Unchanged(_))));
        }
        _ => panic!("expected Object"),
    }
}
