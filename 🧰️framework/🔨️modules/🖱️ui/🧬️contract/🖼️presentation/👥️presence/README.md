# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

👥️ The presence channel: `PresenceUpdate`/`PeerMark`, TTL-scoped and coalesced.

🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
below is plain sync by owner ruling U1.

[`Activity`] is the one presence-shaped field that DOES live on [`crate::UiNodeRecord`] — a node
genuinely *is* loading, waiting, or finished, and that is document state every subscriber must
receive with the node. Everything else here (hover, selection, peer cursors, own color) travels on
this separate, coalesced, TTL-scoped channel and never touches the document: those signals change
at input frequency, and forcing a document revision for every mouse-move would defeat the whole
point of a revisioned patch protocol. A peer whose session drops does not need an explicit
"goodbye" message either — its marks simply age out once `ttl_ms` elapses without a refresh.

<a id="source-notes-92"></a>

### 🦀️.rs:92

📡️ One coalesced, TTL-scoped update on the presence channel, keyed `(surface, node_key)` — never
carried by [`crate::UiSnapshot`]/[`crate::UiPatch`] themselves. A receiver clears a peer's mark once
`ttl_ms` has elapsed without a fresh `PresenceUpdate` for that key, so a disconnected peer fades out
on a timer instead of leaving a stuck mark. Replaces the old `ui_tree_stamp_presence`, which
mutated hover/selection/color/peers directly onto tree nodes.

The normative shape is `framework.ui.contract`'s `PresenceUpdate` export
(`🧬️schema/🔣️.json#/$defs/PresenceUpdate`), which this struct is re-exported into by
`🧬️schema/🦀️.rs`; `deny_unknown_fields` here is that schema's `additionalProperties: false`, so
both halves of the export accept exactly the same documents.
