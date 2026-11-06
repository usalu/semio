# Design Notes

## Preserved Source Rationale

<a id="source-notes-36"></a>

### 🦀️.rs:36

🗿️ The artifact id this channel's ONE live guest session document is, when the owning
workspace has bound exactly one artifact to this plugin. Every `RevisionStamp` this channel
answers names it.

🐛️ Before ticket 26/09/18 slice M8 the stamp carried `entry.plugin_id`, so the headless
lane's `expectedRevision` named `"note"` while `artifact_create`/`artifact_snapshot` named
`"…-typed"` — two ids for one document, and `artifact_snapshot(revisionAfter.artifactId)`
answered `no such artifact: note` (`📓️ce1-client-e2e-pinning-and-puzzle-bound.md` §8 gap 4).

<a id="source-notes-50"></a>

### 🦀️.rs:50

🧵️ Document-backbone messages this guest published and this channel has not handed on yet —
the `Effect::SendMessage { target: Backbone { uri } }` payloads addressed at this channel's
own actor uri. They used to be dropped on the floor: `exchange_one_turn` named every effect
for diagnostics and admitted only the `Shell{instance}` reply lane, so a committed
transaction's envelopes — the ONE thing that has to leave the process for anyone else to see
the edit — went nowhere at all.
