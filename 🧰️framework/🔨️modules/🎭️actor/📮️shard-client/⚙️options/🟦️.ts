import { OriginalBootstrapOwner } from "../../🎟️retained-turn/🏛️bootstrap/🟦️.ts";
import { OwnedResidentLedger } from "@semio-tech/value-ts/resident";
import type { CreateShardWorker, HostEffectHandler } from "../🚨️quota/🟦️.ts";

export interface ShardClientOptions {
  readonly originalWorkStock: OriginalBootstrapOwner;
  readonly residentLedger: OwnedResidentLedger;
  /** 🧩️ Fixed pool size — design-runtime.md §1 `ShardTable`: web `min(hardwareConcurrency-1, 4)`. Caller
   * computes the number; this class only ever spawns exactly this many workers. */
  readonly shardCount: number;
  readonly createWorker: CreateShardWorker;
  /** ≤2 exclusive shards reserved for `leaseExclusive` — the tail of the shard index range. Clamped to
   * `shardCount`. */
  readonly exclusiveShardCount?: number;
  /** 🎰️ One `Int32Array` slot per shard index — `Atomics.store(sab, shardIdx, turnSeq)` heartbeat path.
   * Omitted entirely when `SharedArrayBuffer` is unavailable (no COOP/COEP): the `postMessage`
   * `heartbeat` message is ALWAYS honored regardless, so correctness never depends on this. */
  readonly heartbeatSab?: SharedArrayBuffer;
  readonly heartbeatTimeoutMs?: number;
  /** 🌳️ The window an actor's FIRST turn on an activation is measured against instead — see
   * {@link evaluateShardLiveness} for why that turn is a different class of work (its one-off guest
   * initialization runs synchronously and starves the worker's own progress ticker). */
  readonly firstTurnTimeoutMs?: number;
  /** ⏱️ Cadence for {@link ShardClient.startWatchdog}'s self-tick when called with no explicit override.
   * Defaults to `heartbeatTimeoutMs` — the same cadence the pre-existing manual `checkHeartbeats()`
   * call pattern already assumed (see that method's own doc: "three consecutive timeout windows"). */
  readonly watchdogIntervalMs?: number;
  readonly now?: () => number;
  /** 🔪️ Fired when a shard is torn down (3 missed heartbeats, or an explicit `terminate()`) — the caller
   * (kernel-side scheduler) is responsible for restoring every listed actor from its last checkpoint
   * on a freshly `rebuild()`-ed shard; this class only does the mechanical worker lifecycle. */
  readonly onShardLost?: (shardIndex: number, actorIds: readonly string[]) => void;
  readonly onActorTrap?: (actorId: string, message: string) => void;
  /** 🫀️ Fired for every beat this client folds into a shard's liveness — a dedicated `heartbeat`
   * message, a beat carried on a `result`, or the `Atomics` slot the watchdog reads. `phase` is the
   * worker's own await boundary (`module-fetch`, `module-ready`, `actor-ready`, `progress`).
   *
   * The plugin-load deadline (`🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx`'s `loadPluginModuleResilient`) subscribes to it:
   * a shard that is beating THROUGH its `await import()` of a multi-hundred-MB component is the only
   * proof a concurrently starving load has that the pipeline is moving, and without it every pane
   * whose small descriptor request queued behind those module fetches died on an idle window it was
   * never idle in. */
  readonly onLiveness?: (shardIndex: number, phase: string | null) => void;
  /** 🌉️ terra-shard-effect-bridge: answers `effect-request` frames — `http-fetch`/`blob-read`/
   * `storage-read`/… . Omitted entirely means every effect-request fails FAST with `"no host effect
   * handler installed"` rather than hanging the guest's `.await` forever; see {@link HostEffectHandler}'s
   * own doc for the full contract (including the cancellation `signal`). */
  readonly onHostEffect?: HostEffectHandler;
  /** 🚦️ Per-actor cap on CONCURRENT unresolved `effect-request`s — mirrors the CONCEPT of
   * `QuotaSchema.outstanding_requests` (the host-side per-instance quota) on the client's own ledger,
   * independent of it: this is `ShardClient`'s own backpressure against queuing unbounded concurrent
   * host-effect handler invocations, regardless of what a later host-side quota decides. A request
   * beyond the cap is rejected immediately with a {@link ShardQuotaBreach}-shaped message. Defaults to
   * {@link DEFAULT_MAX_OUTSTANDING_EFFECTS_PER_ACTOR}. */
  readonly maxOutstandingEffectsPerActor?: number;
}
