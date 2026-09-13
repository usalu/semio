// #region 🧲️Header
/** @emoji 🩺️ The runtime-diagnostics door a shard worker — and through it every wasm GUEST it hosts —
 * is armed by. A Worker realm owns no `localStorage`, so the preference the PAGE resolved has to
 * cross the worker boundary on the worker's own URL; the worker reads the stamp back and hands it to
 * each component through `wasi:cli/environment`, where the guest's `std::env::var` (Rust
 * `semio_framework_trace::RUNTIME_DIAGNOSTICS_ENV`) finally reads it.
 *
 * A LEAF on purpose. `🧵️shard-runtime/🟦️.ts` imports `ShardClient` and, with it, the whole pooled-actor
 * graph; the wgpu UI isolate that actually constructs the shard workers
 * (`🚚️browser-frame-transport/🟦️.ts`) must not pull that graph into its boot bundle just to read one
 * preference. Both import this file instead, so the switch stays ONE implementation rather than a
 * hand-synced second copy (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 *
 * @see 🧰️framework/🔨️modules/⏱️trace/🦀️.rs — the guest-side `RUNTIME_DIAGNOSTICS_ENV` this mirrors. */
// #endregion 🧲️Header

//#region 🔖️Switch
/** 🩺️ Mirrors the guest-side `RUNTIME_DIAGNOSTICS_ENV` (`🧰️framework/🔨️modules/⏱️trace/🦀️.rs`) and the
 * renderer's own `RUNTIME_DIAGNOSTICS_KEY`/`TURN_DIAGNOSTICS_KEY` — one name, now three languages. */
export const SHARD_RUNTIME_DIAGNOSTICS_KEY = "SEMIO_RUNTIME_DIAGNOSTICS";

/** 🩺️ The query parameter {@link stampShardWorkerDiagnostics} puts on a shard-worker URL when
 * diagnostics are armed — the only channel that exists before the worker's first message. */
export const SHARD_WORKER_DIAGNOSTICS_PARAM = "diagnostics";

/** 🩺️ Whether this realm has armed runtime diagnostics: the stored
 * {@link SHARD_RUNTIME_DIAGNOSTICS_KEY} preference, read defensively because a Worker (and a
 * storage-blocked browser) throws on the accessor itself rather than returning null. */
export function shardRuntimeDiagnosticsArmed(): boolean {
  try {
    const stored = globalThis.localStorage?.getItem(SHARD_RUNTIME_DIAGNOSTICS_KEY)?.trim().toLowerCase();
    return stored === "1" || stored === "true" || stored === "on" || stored === "yes";
  } catch {
    return false;
  }
}

/** 🩺️ `url`, plus the diagnostics stamp when THIS realm armed them. The stamp belongs to the realm
 * that CONSTRUCTS the worker, because that is the realm holding the preference: wgpu spawns its
 * shards from the UI isolate on a url its FRAME WORKER named, and a frame worker owns no storage, so
 * a url stamped at the naming site was never stamped at all and every guest `[DEBUG]` trace site was
 * unreachable on that target. Idempotent, so an already-stamped url crosses unchanged. */
export function stampShardWorkerDiagnostics(url: string): string {
  if (!shardRuntimeDiagnosticsArmed() || url.includes(`${SHARD_WORKER_DIAGNOSTICS_PARAM}=1`)) return url;
  return `${url}${url.includes("?") ? "&" : "?"}${SHARD_WORKER_DIAGNOSTICS_PARAM}=1`;
}
//#endregion 🔖️Switch
