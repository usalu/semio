# Design Notes

## Preserved Source Rationale

<a id="source-notes-53"></a>

### 🦀️.rs:53

⏳️ Blocking receive with a timeout — `ShardExecutor` (terra-shard-grants,
`🖥️host/🧵️shard/🧵️executor/🦀️.rs`) parks here between pumps instead of a caller-driven
busy-poll loop. `mpsc::Receiver::recv_timeout` blocks only the CALLING thread; it spawns
no thread of its own, so the purity grep this crate's core is verified against (no
`std::thread` in `🦀️.rs`) still holds. Not part of the [`ShardTransport`]
trait — `WorkerTransport`/`ProcessTransport` have their own, different ways of parking
(a JS event loop, a blocking `read`), so this stays a `ThreadTransport`-only inherent
method, the same shape `Self::beat` already uses for its own extra, non-trait surface.

<a id="source-notes-168"></a>

### 🦀️.rs:168

🔗️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): explicit parent→children
edge table for extension cascade — see `//#region 🔗️ExtensionActivation` below. Kept explicit
rather than re-derived from `ActorKind::Extension.plugin` on every cascade so that (a) cascade is
O(children) instead of an O(actors) scan, and (b) it survives multiple *instances* of one plugin
(several `PluginApp` actors sharing a `PackageId`, each with its own extension subtree) without
the `plugin: PackageId` field alone being able to disambiguate which instance a child belongs to.
