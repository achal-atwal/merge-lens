use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Mode {
    TwoWay,
    ThreeWay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiffNode {
    Unchanged(Value),
    Added(Value),
    Removed(Value),
    Modified {
        base: Value,
        mine: Value,
        theirs: Option<Value>,
        conflict: bool,
    },
    Object(IndexMap<String, DiffNode>),
    ArrayChange {
        base: Value,
        mine: Value,
        theirs: Option<Value>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffResult {
    pub root: DiffNode,
    pub conflict_count: usize,
    pub auto_merged_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PathSegment {
    Key(String),
    Index(usize),
}

pub type JsonPath = Vec<PathSegment>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Resolution {
    Mine,
    Theirs,
    Base,
    /// User-typed JSON value — not limited to any of the three document versions.
    Custom(Value),
}

pub type Resolutions = HashMap<JsonPath, Resolution>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeResult {
    pub merged: Value,
    pub unresolved: Vec<JsonPath>,
}
