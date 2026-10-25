
//#region 🌉️HostEffect
/** ⚖️ Stand-in for the generated mirror of Rust `semio_framework_kernel::QuotaBreach` (same "hand-mirrored,
 * not-yet-emitted generated type" pattern as {@link ShardBudget} above) — describes ONE outstanding-
 * effects cap breach, mirroring `QuotaBreach { quota, limit, actual }` field-for-field rather than
 * inventing a parallel vocabulary. `design-abi.md`'s `QuotaSchema.outstanding_requests` is the host-side
 * analog this client-side cap protects independently of — a guest's granted budget is enforced
 * host-side too, but this cap exists so `ShardClient` itself never queues unbounded concurrent
 * host-effect handler invocations regardless of what the host later decides. */
export interface ShardQuotaBreach {
  readonly quota: string;
  readonly limit: number;
  readonly actual: number;
}

export function formatQuotaBreachMessage(breach: ShardQuotaBreach): string {
  return `outstanding effect quota exceeded: ${breach.quota} limit=${breach.limit} actual=${breach.actual}`;
}

/** 🌉️ What `ShardClient` calls for every `"effect-request"` frame a worker posts up
 * (`🟨️.js`'s `effectRequest` — 🧪️ terra-web-bridges) — the ONE seam `http-fetch`/`blob-read`/
 * `storage-read`/… etc actually resolve through. `ShardClient` implements NONE of these itself (`🎭️actor`
 * stays free of `web_sys`/host assumptions per this ticket's naming-hazards rule) — the React host, the
 * wgpu host, and tests each supply their own. `signal` aborts when the owning shard is lost
 * (`terminate`/watchdog rebuild) or the actor is `dispose`d, so a real fetch-backed handler can hand it
 * straight to `fetch(url, { signal })` and genuinely cancel a dead actor's in-flight network request
 * rather than merely forgetting it. Resolve with the effect's success value; reject (throw) to signal
 * failure — the rejection's `message` becomes the guest's `effect-error` `.message`. */
export type HostEffectHandler = (actorId: string, effect: string, params: unknown, signal: AbortSignal) => Promise<unknown>;
//#endregion 🌉️HostEffect

/** 👷️ The slice of `Worker` `ShardClient` depends on — lets tests (and any non-browser host) inject a
 * fake without a real `Worker`/`MessagePort`. A real browser `Worker` satisfies this structurally. */
export interface ShardWorkerLike {
  postMessage(message: unknown, transfer?: readonly Transferable[]): void;
  terminate(): void;
  onmessage: ((event: { readonly data: unknown }) => void) | null;
  onerror: ((event: unknown) => void) | null;
  /** 🩺️ Optional only so a test double may omit it; a real `Worker` always has it, and a shard that
   * loses a message to a failed structured-clone deserialization otherwise just goes quiet. */
  onmessageerror?: ((event: unknown) => void) | null;
}

export type CreateShardWorker = (shardIndex: number) => ShardWorkerLike;
