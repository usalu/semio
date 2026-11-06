# Design Notes

## Preserved Source Rationale

<a id="source-notes-5"></a>

### 🦀️.rs:5

#region 🎨️Props
📊️ Props for `Component::Table` — a column-headed data table. The header lives HERE, as props, and
every [`TableRowProps`] child is one row, so a table costs one node record plus one per MATERIALISED
row however many columns and row actions it has. `window` is the same [`TreeWindow`] contract a tree
section carries: the children are the rows `[offset, offset + children.len())` of a logically
`total`-long row list, a renderer pitches the unmaterialised rows as spacers and asks for the rows its
viewport shows through `ViewModel::tree_windows` — one windowing mechanism and one body-wide node
ledger ([`TREE_WINDOW_BODY_NODE_BUDGET`]) for trees and tables alike.

<a id="source-notes-85"></a>

### 🦀️.rs:85

🧩️ Props for `Component::Extension` — the old `ExternalSlot`. `params_json: String` becomes
structured `crate::UiValue`; `plugin_id`/`app_id`/`body_key` collapse into one opaque `extension`
address string (the old three-part addressing is a concern of whatever resolves `extension` to a
slot, not of this contract).
🌱️ No `ToValue`/`FromValue` here: `props: crate::UiValue` directly — the deliberate DslValue-free
exception, see `UiValue`'s own docstring in `🎬️action.rs`.

<a id="source-notes-97"></a>

### 🦀️.rs:97

⚠️ Decision: `crate::UiValue` is referenced, not defined, here. The packet brief's explicit
"leave unresolved" list (`LayoutSpec`/`StyleSpec`/`AccessibilitySpec`/`ActionBinding`/
`MenuRef`/`Activity`/`SurfaceProps`) does not name `UiValue`, but `📋️master.md`'s "1. Contract
crate" section places `UiValue` right beside `ActionId`/`Trigger`/`UiIntent` — the action
model, owned by packet `contract-action`'s `🎬️action.rs`, not this file. Defining it here
risks the exact duplicate-definition collision U2 calls out as worse than an unresolved name.

<a id="source-notes-107"></a>

### 🦀️.rs:107

#region 🧩️Enum
🧱️ The closed set of things a [`crate::UiNodeRecord`] can render.

⚠️ Unlike the old `UiNode`, no `#[allow(clippy::large_enum_variant)]` is needed: the old
`ComponentScene` variant carried up to fifteen `Option<XxxScene>` fields inline, which is exactly
the size disparity that lint was suppressing. `Component::Surface` now carries one
`crate::SurfaceProps` (a single pack-encoded payload keyed by a `doc_schema` id), so the variants
are all comparably small.
🌱️ No `ToValue`/`FromValue` on the dispatch enum itself: the `TreeItem`/`Extension` variants carry
`TreeItemProps`/`ExtensionProps`, both deliberately DslValue-free-exception types — see their own
notes above.
