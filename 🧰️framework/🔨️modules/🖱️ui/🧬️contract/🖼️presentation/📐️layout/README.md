# Design Notes

## Preserved Source Rationale

<a id="source-notes-411"></a>

### 🦀️.rs:411

🌲️ The window-shell tree: a single recursive, internally-tagged enum replacing the old
`WindowLayoutWindowNode`/`WindowLayoutStackNode`/`WindowLayoutAxisNode` trio and their
`kind: String` + `#[serde(untagged)]` scheme. `size` stays an `Option<f64>` fraction of the parent
split (a ratio, not a pixel measurement, so it is exempt from the [`SpaceToken`] rule). The
`alias = "activeId"` serde alias on the old stack node is dropped — greenfield, fixtures
re-handcrafted, no compatibility requirement.

## Window layout contract

`🧬️schema/🔣️.json` is the single schema of the window layout (`window` / `stack` / `split`), `🟦️.ts` and `🦀️.rs` its types, and `🧪️tests/🧫️fixtures/🔣️.json` the shared corpus of layouts, operations, queries and builders that the Rust test and the TS test both run. A window's id is `instanceId ?? windowKindId`, a stack's `activeWindowId` names such an id, sizes are fractions of the parent split and paths are child-index lists. Every drop path addresses the layout after the dragged window or stack was removed.
