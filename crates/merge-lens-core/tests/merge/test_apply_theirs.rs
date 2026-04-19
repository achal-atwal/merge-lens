use merge_lens_core::diff::diff_three;
use merge_lens_core::merge::apply_resolutions;
use merge_lens_core::types::{PathSegment, Resolution, Resolutions};
use serde_json::json;

#[test]
fn resolve_conflict_with_theirs() {
    let base = json!({ "name": "Alice" });
    let mine = json!({ "name": "Alicia" });
    let theirs = json!({ "name": "Alex" });
    let diff = diff_three(&base, &mine, &theirs);

    let mut resolutions: Resolutions = std::collections::HashMap::new();
    resolutions.insert(vec![PathSegment::Key("name".into())], Resolution::Theirs);

    let result = apply_resolutions(&diff, &resolutions);
    assert!(result.unresolved.is_empty());
    assert_eq!(result.merged, json!({ "name": "Alex" }));
}

#[test]
fn unresolved_conflict_is_reported() {
    let base = json!({ "x": 1 });
    let mine = json!({ "x": 2 });
    let theirs = json!({ "x": 3 });
    let diff = diff_three(&base, &mine, &theirs);

    let result = apply_resolutions(&diff, &std::collections::HashMap::new());
    assert_eq!(result.unresolved.len(), 1);
    assert_eq!(result.unresolved[0], vec![PathSegment::Key("x".into())]);
}
