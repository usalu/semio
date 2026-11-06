# Design Notes

## Preserved Source Rationale

<a id="source-notes-13"></a>

### 🦀️.rs:13

🧪️ Opens (creating on first use) a real `ProbeStore` bound through `ArtifactHost` — real VCS
history, real backbone attachment (`ArtifactHost::send`/`subscribe` — §6 of the brief: "guest
`SendMessage{Backbone}` effects route to `ArtifactHost::send`; inbound
`ArtifactEvent::RemoteMutations` come back as guest events"). Applies `initial` as the FIRST
commit only if the document has no history yet, so re-running this against an already-seeded
workspace is idempotent and never double-commits. Returns the resulting `RevisionStamp`.

<a id="source-notes-60"></a>

### 🦀️.rs:60

🧪️ A real SECOND (or later) commit against an already-`ensure_probe_artifact`-opened
document — proves genuine mutate-and-observe end to end for the ONE document kind this crate
owns outright, independent of `ensure_probe_artifact`'s own idempotent-seed-only contract.
This is deliberately scoped to `os.agent.probe/v1` — see this file's own module doc and
`PluginArtifactChannel::exchange`'s `PureCommand`/`TransactionPrepare` arms for exactly why a
generic, arbitrary-plugin equivalent is not real yet (a guest-dispatch gap in `🔌️plugin`'s
own shared code, out of this packet's owned file).
