use serde_json::Value;

/// Returns true if both sides changed from base to different values (true conflict).
pub fn is_conflict(base: &Value, mine: &Value, theirs: &Value) -> bool {
    mine != base && theirs != base && mine != theirs
}

/// Returns true if both sides converged to the same value (safe to auto-merge).
pub fn both_same(mine: &Value, theirs: &Value) -> bool {
    mine == theirs
}
