# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🏗️ Ergonomic semantic builders (`ui::stack()`, `ui::button()`) — wasip2-safe, no engine.

🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
below is plain sync by owner ruling U1.

⚠️ Decision (flagged for the coordinator, detailed in `📓️terra-contract-builder-report.md`):
[`BuiltNode`] duplicates the shape the sibling `semio-ui-runtime` crate's own
`ComponentTree` needs, because this crate must not depend on that crate (the dependency runs the
other way — see `🦀️.rs`). The runtime converts a `BuiltNode` tree into its own reconciler
input; if the two shapes drift, that conversion is the one place to fix, never a second builder.
