use merge_lens_core::diff::diff_three;
use merge_lens_core::merge::apply_resolutions;
use merge_lens_core::types::{PathSegment, Resolution, Resolutions};
use serde_json::json;

#[test]
fn resolve_conflict_with_base() {
    let base = json!({ "status": "active" });
    let mine = json!({ "status": "pending" });
    let theirs = json!({ "status": "archived" });
    let diff = diff_three(&base, &mine, &theirs);

    let mut resolutions: Resolutions = std::collections::HashMap::new();
    resolutions.insert(vec![PathSegment::Key("status".into())], Resolution::Base);

    let result = apply_resolutions(&diff, &resolutions);
    assert!(result.unresolved.is_empty());
    assert_eq!(result.merged, json!({ "status": "active" }));
}

#[test]
fn removed_key_absent_from_merged_output() {
    let base = json!({ "a": 1, "b": 2 });
    let mine = json!({ "a": 1 });
    let theirs = json!({ "a": 1 });
    // both sides removed "b"
    let diff = diff_three(&base, &mine, &theirs);
    let result = apply_resolutions(&diff, &std::collections::HashMap::new());
    assert_eq!(result.merged, json!({ "a": 1 }));
}
