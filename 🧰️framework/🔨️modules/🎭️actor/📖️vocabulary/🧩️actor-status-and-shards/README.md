# Design Notes

## Preserved Source Rationale

<a id="source-notes-54"></a>

### 🦀️.rs:54

#region 🧩️ShardTable
🖥️ Which host EXECUTION SURFACE a shard runs on — `design-runtime.md` §2's "thread, worker, or
process" trio, unchanged by the P1c one-pool-worker-runtime refactor EXCEPT for what `Native`
itself now means. A shard was never "a thread" as an identity here — `ShardTable` (below) only
ever pinned an [`ActorId`] to a [`ShardId`], a bookkeeping integer; this crate's own purity rule
means it never spawned anything. What changed in the HOST crates (`semio-plugin-host`'s
`ShardExecutor`, `semio-os`'s `NativeKernelRuntime`) is that `Native` no longer implies
"and therefore gets one dedicated OS thread" — a `Native` shard is now a logical affinity unit
(its actors' `wasmtime::Store`s are pinned to it so guest instance state stays coherent) whose
turns are submitted as jobs onto one process-wide `semio_framework_async::WorkerPool`, mutually
exclusive PER SHARD (so affinity is enforced by a lock, not by thread identity) rather than
concurrent by construction. `WebWorker`/`Process` were already real boundary crossings (a browser
worker, an OS process) unaffected by this distinction — `Native` is the one variant whose meaning
narrows from "a thread" to "this process, pool-scheduled."
