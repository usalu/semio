# Design Notes

## Preserved Source Rationale

<a id="source-notes-8"></a>

### 🦀️.rs:8

🧪️ A generic, single-JSON-value document kind this crate owns outright — used ONLY to exercise
the real `store::sync::ArtifactHost` / `ArtifactStore` / backbone-propagation pipeline end to end
(§6 of the packet brief: "a commit made headlessly propagates to any live shell on the same
folder/hub — prove it in a test if you can") without pretending to understand any actual plugin's
document schema, which is host-opaque by design (this file's own module doc). Schema id
`os.agent.probe/v1` is namespaced under `os.agent.*` precisely so it can never collide with a real
plugin schema id.

<a id="source-notes-257"></a>

### 🦀️.rs:257

🔐️ The one real owner catalog every `ProbeStore` installs right after construction
(`ensure_probe_artifact`) — required before `ArtifactStore::drop`'s terminal-empty witness can
ever be reached (`install_document_store_owners_exact`'s own doc: "There is no default catalog").
`store::ArtifactStoreCursorDisposer` is the store crate's own canonical full-store close driver
(used the same way by its `#[cfg(test)] demo_closable_store_owners`): it walks every detached
authority — displaced retirements, returned reads, history, metadata, message ledgers,
conflicts, the pending report, every runtime string (applied/redo edit ids, the revision
accumulator, the checkpoint/local-actor ids), the tail/current snapshot roots, the backbone, the
causal DAG, and finally the envelope shell — in the exact order `ArtifactStore::drop`'s witness
checks them.

<a id="source-notes-271"></a>

### 🦀️.rs:271

🚪️ Drains one `ProbeStore` to `ArtifactStore::drop`'s exact terminal-empty witness before
letting it actually drop — the production teardown a `--folder`/`--hub`-bound `semio-os-mcp`
process needs at shutdown (`HeadlessWorkspace`'s own `Drop`, below) and what every `workspace`/
`artifact` test that opens a real probe now relies on too. `close_owned_step` is bounded per call
(`SpaceMember`'s own contract), so this is a loop to completion, not a single call; a `Blocked`
step means an external owner (the backbone's shared queue — its remote `ChannelBackboneRemote`
end lives on the `ArtifactHost` actor this same caller already asked to close) hasn't released
its side yet. That release happens on the real `WorkerPool` background thread the actor runs on,
concurrently with this thread — never something a plain busy-spin can force — so a `Blocked` step
yields the thread briefly instead of re-polling immediately, giving that background thread real
wall-clock time to finish tearing the actor down.
