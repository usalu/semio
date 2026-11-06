# Design Notes

## Preserved Source Rationale

<a id="source-notes-5"></a>

### 🦀️.rs:5

#region 🔖️Apply
🩹️ Applies `patch` to `state`, totally transactionally: `base_revision` must equal `state`'s
current revision; every op then applies to a **shadow draft clone**, never to `state` directly; the
draft is validated against `limits` via [`validate_snapshot`]'s shared core; only on success does
`state` swap to the draft in one move. On ANY rejection path, `state` is untouched — not partially
applied, not touched at all — which is the property that lets a sender resynchronise by trusting
`state`'s revision never moved.
🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
