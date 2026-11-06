# Design Notes

## Preserved Source Rationale

<a id="source-notes-45"></a>

### 🦀️.rs:45

#region 🔖️Value
🧬️ A neutral, JSON-shaped value — the ONE recursive type in this crate. Every node in
`📃️document.rs` avoids inline recursion by addressing children through [`crate::UiNodeId`] instead
of nesting a node inside another; `UiValue` is the deliberate exception because it does not
describe document shape at all, it describes an arbitrary opaque payload (action args, extension
props) that genuinely IS JSON-shaped, and owned fixed list/map pages give the schema an indirection
to resolve (heap-allocated, not an inline field) rather than the infinitely-sized-struct problem
direct node-in-node nesting would create.

⚠️ The os-kernel's `DslValue` (`🧰️framework/🔨️modules/🌱️value/🦀️.rs`) must NEVER appear in
this crate — this crate has no such dependency and stays `wasm32-wasip2`/`wasm32-unknown-unknown`
safe by construction. `From`/`Into` conversions between `UiValue` and `DslValue` belong in the
os-kernel crate, never here.
🌱️ No `ToValue`/`FromValue` on `UiValue` — see the DslValue prohibition above. `#[serde(untagged)]`
also has no `#[value(...)]` equivalent regardless (would need a hand-written impl).
