# Design Notes

## Preserved Source Rationale

<a id="source-notes-33"></a>

### 🦀️.rs:33

🗂️ One plugin-typed artifact's real ownership: the artifact kind its owning plugin declares
(`ArtifactKindSpec.kind_id`/`AppIo.artifact_schema`), that plugin's id, and the app whose media
ports can export it.

🌎️ For a HUB document the binding additionally carries the surface the hub's own execution-target
lease names (`DocumentOpenSurfaceV1.surface_id`) and the document's
canonical pair, so the plugin's guest can be seeded with THE HUB'S document rather than with the
plugin's genesis — see [`RoutingArtifactChannel::exchange`]. A folder-created artifact has neither:
it has no hub surface, and its bytes already live in this workspace's own event log.
🚦️ What this session last heard from a hub document's own actor: its sync status (`version`
counts every status it reported) and the last coded fault it raised, e.g. a link that turned
terminal. [`HubRelay::await_acknowledged`] is how a commit learns whether the hub HAS the edit.

<a id="source-notes-88"></a>

### 🦀️.rs:88

🧭️ Folds what the actor delivered against the pair the guest was seeded from. The NEWEST archive
supersedes everything delivered before it and reseeds the guest unless it is byte-identical to that
seed (the usual first bootstrap: the actor installs the same checkpoint `artifact_open` read); the
actor never re-delivers an operation this replica authored or already applied, so the batches after
it are exactly what the guest lacks, delivered in order.

🐛️ Without this a hub-bound agent acted on the hub's last CHECKPOINT forever: the guest was seeded
once from the canonical pair and nothing the actor received ever reached it, so a fresh agent session
re-derived the same state — and the same ids (`note`'s `text-<scope>-<block count>`) — as every
session before it, and the hub accepted each as a distinct op (WG9 15:4x, G12 ids probe 18:4x).

<a id="source-notes-134"></a>

### 🦀️.rs:134

🚦️ Why a hub session's edit may not run, decided before any guest executes it — `None` when it may. In order: the
bound document's link is terminal (the hub withdrew access or the link expired); the hub holds the principal as a
spectator (a `read` delegation) or its authority is not ready; no single hub document of the verb's plugin is bound
(nothing the guest authored could reach the hub — a hub session never edits a plugin's own genesis document); the
bound document has no live document actor. Every edit that passes is relayed and waits for the hub's acknowledgement.
