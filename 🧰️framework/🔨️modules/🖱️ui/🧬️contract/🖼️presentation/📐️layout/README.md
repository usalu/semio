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
