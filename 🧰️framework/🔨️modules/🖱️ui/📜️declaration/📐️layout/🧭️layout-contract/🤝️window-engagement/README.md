# Design Notes

## Preserved Source Rationale

<a id="source-notes-150"></a>

### 🦀️.rs:150

🔌️ Closed replacement for `Option<WindowEngagement>` — makes "this window kind never engages" a
named variant instead of `None`, so absence is an explicit, typed state rather than an implicit gap.
⚠️ `WindowEngagement` is a wide variant (nested `Vec`/`Option` fields), making `Some` far
larger than `None` — boxing it would be a breaking public-API change (every construction/match
site across ~30 plugins would need `Box::new`/deref updates), out of scope for a mechanical pass.
