use merge_lens_core::diff::diff_three;
use merge_lens_core::merge::apply_resolutions;
use merge_lens_core::types::{PathSegment, Resolution, Resolutions};
use serde_json::json;

#[test]
fn resolve_conflict_with_mine() {
    let base = json!({ "age": 30 });
    let mine = json!({ "age": 31 });
    let theirs = json!({ "age": 32 });
    let diff = diff_three(&base, &mine, &theirs);

    let mut resolutions: Resolutions = std::collections::HashMap::new();
    resolutions.insert(vec![PathSegment::Key("age".into())], Resolution::Mine);

    let result = apply_resolutions(&diff, &resolutions);
    assert!(result.unresolved.is_empty());
    assert_eq!(result.merged, json!({ "age": 31 }));
}

#[test]
fn auto_merged_field_does_not_need_resolution() {
    let base = json!({ "x": 1, "y": 10 });
    let mine = json!({ "x": 2, "y": 10 });
    let theirs = json!({ "x": 1, "y": 10 });
    let diff = diff_three(&base, &mine, &theirs);

    // no resolution needed — x was auto-merged (mine changed, theirs didn't)
    let result = apply_resolutions(&diff, &std::collections::HashMap::new());
    assert!(result.unresolved.is_empty());
    assert_eq!(result.merged, json!({ "x": 2, "y": 10 }));
}
