use merge_lens_core::diff::diff_three;
use merge_lens_core::merge::apply_resolutions;
use merge_lens_core::types::{PathSegment, Resolution, Resolutions};
use serde_json::json;

#[test]
fn resolve_conflict_with_custom_value() {
    let base = json!({ "age": 30 });
    let mine = json!({ "age": 31 });
    let theirs = json!({ "age": 32 });
    let diff = diff_three(&base, &mine, &theirs);

    let mut resolutions: Resolutions = std::collections::HashMap::new();
    resolutions.insert(
        vec![PathSegment::Key("age".into())],
        Resolution::Custom(json!(99)),
    );

    let result = apply_resolutions(&diff, &resolutions);
    assert!(result.unresolved.is_empty());
    assert_eq!(result.merged, json!({ "age": 99 }));
}

#[test]
fn resolve_conflict_with_custom_object() {
    let base = json!({ "addr": { "city": "London" } });
    let mine = json!({ "addr": { "city": "Paris" } });
    let theirs = json!({ "addr": { "city": "Berlin" } });
    let diff = diff_three(&base, &mine, &theirs);

    let mut resolutions: Resolutions = std::collections::HashMap::new();
    resolutions.insert(
        vec![PathSegment::Key("addr".into()), PathSegment::Key("city".into())],
        Resolution::Custom(json!("Amsterdam")),
    );

    let result = apply_resolutions(&diff, &resolutions);
    assert!(result.unresolved.is_empty());
    assert_eq!(result.merged, json!({ "addr": { "city": "Amsterdam" } }));
}
