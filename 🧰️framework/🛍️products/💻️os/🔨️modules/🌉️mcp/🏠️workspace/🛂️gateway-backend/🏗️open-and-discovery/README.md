
## 🗿️ A hub document of another schema is not an ERROR, it is simply not a probe document.

This used to answer a retryable `PLUGIN_UNAVAILABLE` naming the probe schema, which made
`artifact_open` of EVERY real hub document fail before it ever reached
[`Self::read_artifact_bytes`] — measured live on hub 7631 against a `gis.map` document
(ticket 26/09/18 slice M8, `📓️m8-mcp-agent-third-participant.md` §2.3 row 4). The
predicate's own name is the contract: it answers whether the probe lane owns this id.

## 🗂️ Every document id currently known to this workspace: the union of (a) every id this

workspace has itself opened a live `ProbeStore` for (`self.open_probes` — populated
SYNCHRONOUSLY by `ensure_probe_artifact`, before this call can ever see it) and (b) whatever
is already durably persisted on disk (`FolderEventLogStorage`'s document events).

🪲️ Post-unblock fix (see `📓️terra-P7-report.md`'s "## post-unblock fixes"): this used to
consult ONLY (b). `ArtifactHost::open`'s actor persists asynchronously on its own thread —
`ensure_probe_artifact`'s `dispatch`+`send` returns as soon as the LOCAL `ArtifactStore` has
applied the mutation, before the actor thread has necessarily flushed it to
`<folder>/.semio/events.semio`. A cold-disk-only reader therefore raced the actor and lost:
exactly `resolve_context_reports_the_open_probe_artifact_as_active`'s failure. (a) closes that
gap because it is the SAME map `ensure_probe_artifact` writes to, read under the same lock,
with no cross-thread hop in between.

## 📖️ One document's real pack+spr bytes, read from whoever OWNS that document right now:

the LIVE in-memory `ProbeStore` snapshot (`ArtifactStore::snapshot_pack`, real bytes, no
cross-thread wait) when this workspace has one open for `artifact_id`; the session's own
channel when this session resolved `shell` and `artifact_id` is one this gateway bound to a
plugin ([`read_session_artifact_bytes`] — the store the human is editing); else a cold
`FolderEventLogStorage` read (host-opaque either way — no codec, no interpretation). `None`
for a hub binding with no live store open, or a folder id with neither an open store nor a
persisted row. Same race fix as `workspace_artifact_ids` — see that method's own doc.

🧊️ The middle lane is the law `artifact_snapshot` was missing: `create_plugin_artifact`
seeds the folder event log from ONE `ReadArtifact` at create time, so every later read of
that row answers the bytes as they were when the agent opened the document. Measured
2026-09-20 (AP1, `🗑️generated/ap1-shell-route-probe.txt`): an `addBlock` invocation moved the
live head from `cursor 0` to `cursor 1, headEditId "apply"` and `artifact_snapshot` answered
a byte-identical 688-char pack on both sides of it. A snapshot that cannot show the change
the agent just made is worse than no snapshot, so a session-owned document is re-read from
its owner and the frozen row is never consulted for it.

🧊️ Headless lane: the live guest instance the mutation went to, never the row frozen
at create time — see [`Self::read_live_session_artifact_bytes`].
