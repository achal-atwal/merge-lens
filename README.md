# merge-lens

A WASM-powered JSON diff and merge conflict visualizer built in Rust. Given two or three JSON documents, merge-lens computes a structural diff, detects conflicts, visualizes them in a browser-based tree UI, and lets you resolve conflicts and export a merged result.

**Two modes:**
- **2-Way Diff** — base + mine → visualize what changed; scalar/object diffs are auto-merged, array changes let you pick Base or Mine; click "View Merged JSON" to export the result
- **3-Way Merge** — base + mine + theirs → detect conflicts, resolve them field by field, export merged JSON

---

## How It Works

merge-lens processes documents in two stages: **diffing** and **merging**.

### Stage 1 — Diff

The diff engine walks both (or all three) JSON documents simultaneously, key by key and value by value, producing a `DiffNode` tree that mirrors the shape of the input.

**2-way diff (`base` + `mine`):**

Each key is classified by comparing where it appears:

| Situation | Result | Merge behaviour |
|-----------|--------|-----------------|
| Key only in `base` | `Removed` | excluded from output |
| Key only in `mine` | `Added` | kept as-is |
| Both are objects | recurse into `Object` | — |
| Both are arrays, values differ | `ArrayChange` | user picks Base or Mine |
| Values are equal | `Unchanged` | kept as-is |
| Values differ | `Modified { conflict: false }` | auto-takes `mine` |

Array changes are the only case in 2-way mode where the UI asks for input. All other differences are auto-resolved.

**3-way diff (`base` + `mine` + `theirs`):**

Extends the 2-way walk with conflict detection per key:

| Situation | Result |
|-----------|--------|
| Key only in `base` | `Removed` |
| Key only in `mine` or only in `theirs` | `Added` (auto-accept) |
| New key in both, same value | `Added` (auto-accept) |
| New key in both, different values | `Modified { conflict: true }` |
| All three are objects | recurse into `Object` |
| Any array differs across versions | `ArrayChange` |
| `mine == theirs == base` | `Unchanged` |
| Only `mine` changed | `Modified { conflict: false }` — auto-take mine |
| Only `theirs` changed | `Modified { conflict: false }` — auto-take theirs |
| Both changed, differently | `Modified { conflict: true }` |

The output is a `DiffResult` containing the root `DiffNode` tree, a total `conflict_count`, and an `auto_merged_count`.

### Stage 2 — Merge

Once the user has recorded a `Resolution` (Mine / Theirs / Base) for each conflicted node, `apply_resolutions` walks the `DiffNode` tree and reconstructs a concrete `serde_json::Value`:

- `Unchanged` / `Added` → kept as-is
- `Removed` → excluded from output
- `Modified { conflict: false }` → auto-resolved value selected (mine if mine changed, theirs if only theirs changed)
- `Modified { conflict: true }` → resolution looked up by `JsonPath`; if missing, path added to `unresolved`
- `ArrayChange` → same resolution lookup; auto-accepted if both sides converged on the same array
- `Object` → recurse, building path as `[Key("field"), ...]`

The result is a `MergeResult` with the final `merged` value and a list of any `unresolved` paths. The UI blocks export until `unresolved` is empty.

### Path addressing

Every node in the diff tree is identified by a `JsonPath` — a `Vec<PathSegment>` where each segment is either `Key("field")` for object keys or `Index(n)` for array positions. Resolutions are stored in a `HashMap<JsonPath, Resolution>`, so clicking "Accept Mine" on `user.address.city` records an entry keyed by `[Key("user"), Key("address"), Key("city")]`.

---

## Features

- Recursive structural diff of JSON objects
- 3-way merge with automatic conflict detection (Git-style: both sides changed differently from base)
- Auto-merge of non-conflicting changes (one side changed, or both changed to the same value)
- Inline conflict resolution with Accept Base / Mine / Theirs per field, plus a free-form custom JSON input
- Array conflict detection (arrays treated as opaque; whole-array resolution)
- JSON paste or file upload for each document
- Real-time validation with inline parse errors
- Merged output: pretty-printed JSON, copy to clipboard, download as `.json`
- Pure Rust core — zero JS logic, zero server, runs entirely in the browser via WASM

---

## Architecture

```
merge-lens/
├── crates/
│   ├── merge-lens-core/        Pure Rust library — no browser deps
│   │   └── src/
│   │       ├── types.rs        DiffNode, DiffResult, Resolution, MergeResult
│   │       ├── diff.rs         diff_two(), diff_three()
│   │       ├── conflict.rs     is_conflict(), both_same() helpers
│   │       └── merge.rs        apply_resolutions()
│   │
│   └── merge-lens-wasm/        Thin wasm-bindgen wrapper for external JS consumers
│       └── src/
│           └── lib.rs          wasm_diff_two, wasm_diff_three, wasm_apply_merge
│
├── app/                        Leptos 0.7 CSR web app (built with trunk)
│   ├── src/
│   │   ├── main.rs
│   │   ├── state.rs            App-wide signals: mode, docs, diff result, resolutions
│   │   └── components/
│   │       ├── mode_toggle.rs
│   │       ├── input_panel.rs
│   │       ├── diff_tree.rs
│   │       ├── conflict_panel.rs
│   │       └── merge_output.rs
│   └── index.html
│
└── Cargo.toml                  Workspace manifest
```

### Key design decisions

**No WASM boundary inside the app.** The Leptos app depends directly on `merge-lens-core` as a Rust crate. The diff and merge functions are plain Rust function calls — no serialization overhead. The `merge-lens-wasm` crate is a separate artifact for external JS/TS consumers who need the browser WASM API.

**Pure Rust core.** `merge-lens-core` has no browser dependencies, no `wasm-bindgen`. It's a normal Rust library that can be tested with `cargo test` on any platform, embedded in other Rust tools, or exposed over any API boundary.

**3-way merge semantics (Git-style).** A conflict occurs when both mine and theirs differ from base, and differ from each other. Non-conflicting changes are auto-merged. This is different from CRDT-based tools (like Figma) where conflicts are invisible; here conflicts are surfaced explicitly for user resolution.

**Arrays as opaque blobs.** Arrays are diffed as whole values. If the array changed on both sides differently, the entire array is flagged as a conflict and resolved by choosing one version. Element-level LCS diffing is on the future roadmap.

---

## Core Types

```rust
pub enum DiffNode {
    Unchanged(Value),
    Added(Value),
    Removed(Value),
    Modified { base: Value, mine: Value, theirs: Option<Value>, conflict: bool },
    Object(IndexMap<String, DiffNode>),
    ArrayChange { base: Value, mine: Value, theirs: Option<Value> },
}

pub struct DiffResult {
    pub root: DiffNode,
    pub conflict_count: usize,
    pub auto_merged_count: usize,
}

pub enum Resolution { Mine, Theirs, Base }
pub type Resolutions = HashMap<Vec<PathSegment>, Resolution>;

pub struct MergeResult {
    pub merged: Value,
    pub unresolved: Vec<Vec<PathSegment>>,
}
```

---

## Building

### Prerequisites

```bash
# Rust (1.88+ required for Leptos 0.7)
rustup update stable
rustup target add wasm32-unknown-unknown

# trunk (Leptos build tool)
cargo install trunk

# wasm-pack (for the standalone WASM package)
cargo install wasm-pack
```

### Run the web app (development)

```bash
cd app
trunk serve
# Open http://localhost:8080
```

### Build the web app (production)

```bash
cd app
trunk build --release
# Output: app/dist/
```

### Run core tests

```bash
cargo test -p merge-lens-core
# 24 tests: 8 two-way, 10 three-way, 6 merge
```

### Build the standalone WASM package

For embedding in non-Rust JS/TS projects:

```bash
cd crates/merge-lens-wasm
wasm-pack build --target web --out-dir pkg
```

This produces a `pkg/` directory with `merge_lens_wasm.js`, `merge_lens_wasm.d.ts`, and the `.wasm` binary.

---

## Using the Web App

1. **Choose a mode** — toggle between "2-Way Diff" and "3-Way Merge" at the top.

2. **Paste or upload JSON** — each editor accepts paste or file upload (`.json`). Invalid JSON shows an inline error.

3. **Run Diff** — the button activates once all required editors have valid JSON. Click it to compute the diff.

4. **Review the diff tree** — each field is color-coded:
   - Grey — unchanged
   - Green `+` — added
   - Red `-` — removed
   - Amber `→` — modified (auto-merged)
   - Red `conflict` badge — needs your decision

5. **Resolve array changes** (2-way mode) or **resolve conflicts** (3-way mode) — for each flagged field, Accept buttons let you choose Base, Mine, or Theirs. You can also type any valid JSON into the **Custom** input and click **Apply** to use a value not present in any of the three documents. In 2-way mode only array-level changes need a decision; all scalar and object diffs are auto-merged.

6. **Export** — click "View Merged JSON" (2-way) or wait for all conflicts to be resolved (3-way). Use Copy or Download to export.

---

## Using the WASM API (JS/TS)

```typescript
import init, { wasm_diff_two, wasm_diff_three, wasm_apply_merge } from './pkg/merge_lens_wasm.js';

await init();

// 2-way diff
const diffResult = wasm_diff_two(
  JSON.stringify({ name: "Alice", age: 30 }),
  JSON.stringify({ name: "Alice", age: 31 })
);
const diff = JSON.parse(diffResult);
// diff.conflict_count, diff.auto_merged_count, diff.root

// 3-way merge
const diffResult3 = wasm_diff_three(
  JSON.stringify({ name: "Alice", age: 30 }),
  JSON.stringify({ name: "Alice", age: 31 }),
  JSON.stringify({ name: "Bob",   age: 30 })
);
const diff3 = JSON.parse(diffResult3);
// diff3.conflict_count === 1 (name conflicts)

// Apply resolutions — three built-in variants plus Custom for a free-form value
const resolutions = {
  '[{"Key":"name"}]': "Mine",          // accept mine ("Alice")
  // '[{"Key":"name"}]': "Theirs"      // accept theirs ("Bob")
  // '[{"Key":"name"}]': "Base"        // accept base ("Alice")
  // '[{"Key":"name"}]': { "Custom": "\"Carol\"" }  // any valid JSON string/number/object
};
const mergeResult = wasm_apply_merge(
  diffResult3,
  JSON.stringify(resolutions)
);
const result = JSON.parse(mergeResult);
// result.merged === { name: "Alice", age: 31 }
// result.unresolved === []
```

All three functions accept JSON strings and return JSON strings. Errors are returned as rejected `JsValue` strings.

**Resolution variants** (value side of the resolutions map):
- `"Mine"` / `"Theirs"` / `"Base"` — pick one of the three document versions
- `{ "Custom": <json-value> }` — any valid JSON value not limited to the three versions

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Core logic | Rust, `serde_json`, `indexmap` |
| WASM bindings | `wasm-bindgen`, `wasm-pack` |
| Web UI | Leptos 0.7 CSR |
| Build tool | `trunk` |
| Browser APIs | `web-sys`, `js-sys` |

---

## Future Roadmap

- **LCS-based array diffing** — element-level diff for arrays instead of whole-array conflict. Show individual added/removed/moved elements.

- **Svelte UI** — a second frontend built with Svelte + the `merge-lens-wasm` package. The Rust core and WASM API stay the same; Svelte becomes an alternative to the Leptos app for users who prefer a JS-native stack.

- **XML and other formats** — parse XML, YAML, and TOML inputs, normalize to an internal tree for diffing, and output the result in the original format.
