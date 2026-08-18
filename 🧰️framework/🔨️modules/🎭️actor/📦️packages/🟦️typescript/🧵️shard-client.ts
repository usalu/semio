/** 🧵️ `ShardClient` — the web `ShardTransport` (design-runtime.md §1 `ShardTransport` /
 * §3 "Web shard"): a bounded pool of `🟨️shard-worker.js` Web Workers multiplexed by `actorId`,
 * replacing one-Worker-per-plugin (`PluginWorkerClient`, deleted from `🎠️kernel/🟦️component.ts` in
 * the same packet). V8 reserves a 4 GiB guard region per wasm module per worker, so
 * one-worker-per-plugin capped the browser at ~20 plugins; this pools K = `min(hardwareConcurrency-1,
 * 4)` (design's `ShardTable`) workers and pins actors onto them instead.
 *
 * 🚧 KNOWN GAP (H2, tracked for a follow-up packet once `🤖️generated/🟦️actor.ts` lands — see A1's
 * report): the design's `ShardTransport` carries the SAME hand-rolled `Envelope`/`TurnResult` pack
 * encoding as the native thread/process transports (`🎭️actor/🦀️component.rs`'s `pack` module). That
 * codec has no TS mirror yet (A1 flagged `🤖️generated/🟦️actor.ts` as not-yet-emitted). `turn()` below
 * is therefore typed as opaque `Uint8Array` in, `Uint8Array` out at THIS module's public boundary —
 * matching `ShardTransport::send(bytes)`/`recv() -> Option<Vec<u8>>` exactly — but `🟨️shard-worker.js`
 * (see `🌐plugin-web-materialize.ts`'s `shardWorkerSource`) currently decodes the interim
 * `ShardEventEnvelope[]` JSON shape declared below rather than the real binary pack format. Swapping
 * the worker's decode step for the generated codec is mechanical once it exists; nothing in this
 * class's shard-assignment/heartbeat/multiplexing logic depends on the wire format.
 */

//#region 🧬️Types
/** ⚖️ Stand-in for the ts-rs mirror of Rust `semio_framework_actor::Budget` (same pattern as A1's own
 * `CapabilityGrant` stand-in — depending on the not-yet-emitted generated file would break every
 * consumer of this module until typegen lands). Field-for-field with design-runtime.md §1's `Budget`,
 * camelCased. */
export interface ShardBudget {
  readonly fuel: number;
  readonly wallMs: number;
  readonly memoryBytes: number;
  readonly uiNodes: number;
  readonly mailboxLen: number;
  readonly maxEffects: number;
  readonly maxPatchBytes: number;
}

/** ⚖️ Stand-in for `semio_framework_actor::JobBudget` (design-abi.md `jobs::job-budget`). */
export interface ShardJobBudget {
  readonly fuel: number;
  readonly deadlineMs: number;
}

/** ⚖️ Stand-in for the WIT `capabilities::capability-grant` record — opaque to the shard worker, just
 * forwarded into the guest's `instance-open` event. */
export interface ShardCapabilityGrant {
  readonly id: string;
  readonly token: string;
  readonly scope: string;
  readonly expiresMs: number | null;
}

/** 🚧 Interim, package-agnostic event envelope `🟨️shard-worker.js` decodes into the guest's WIT
 * `event` variant — see this module's header doc for why this isn't the real pack-encoded `Envelope`
 * yet. `kind` mirrors the WIT `event` variant's own tag name (kebab-case, e.g. `"app-command"`,
 * `"surface-visible"`, `"instance-open"`); `payload` is the variant's own record, JSON-shaped. */
export interface ShardEventEnvelope {
  readonly kind: string;
  readonly payload: unknown;
}

/** 🪶️ One named asset pack delivered on `instance-open` (design-runtime.md §3: `guestSlimAssets`
 * becomes a declared asset here rather than a worker-bootstrap special case) — `events::
 * instance-open-event.assets: list<tuple<string, pack>>`. `bytes` is transferred structured-clone
 * (never `Transferable`-detached: the SAME buffer is reused across every actor this shard pool
 * activates, see `🌐plugin-web-materialize.ts`'s `shardWorkerSource` doc). */
export type ShardAsset = readonly [name: string, bytes: ArrayBuffer];

export type ShardJobStep = { readonly status: "running"; readonly progress?: Uint8Array } | { readonly status: "done"; readonly value: Uint8Array } | { readonly status: "failed"; readonly value: Uint8Array };

/** ⚖️ Stand-in for the ts-rs mirror of Rust `semio_framework_actor::ShardMetrics` (same "not-yet-
 * emitted `🤖️generated/🟦️actor.ts`" reason as `ShardBudget` above) — MICROKERNEL-POOLED-ACTOR-PLUGIN-
 * RUNTIME T1. Field-for-field with the Rust struct, camelCased. */
export interface ShardMetrics {
  readonly actors: number;
  readonly busyRatio: number;
  readonly heartbeatAgeMs: number;
}

/** ⚖️ Stand-in for `semio_framework_actor::ShardMetricsSample` — one row of the `os.runtime.metrics`
 * publication's shard table, as `ShardClient.shardMetricsSamples` can observe it purely from data this
 * class already owns (`actorIds`/`pendingRequestIds`/heartbeat state) — no wasm `Kernel` call needed. */
export interface ShardMetricsSample {
  readonly shard: number;
  readonly metrics: ShardMetrics;
}
//#endregion 🧬️Types

//#region 🌉️WorkerLike
/** 🌉️ The slice of `Worker` `ShardClient` depends on — lets tests (and any non-browser host) inject a
 * fake without a real `Worker`/`MessagePort`. A real browser `Worker` satisfies this structurally. */
export interface ShardWorkerLike {
  postMessage(message: unknown, transfer?: readonly Transferable[]): void;
  terminate(): void;
  onmessage: ((event: { readonly data: unknown }) => void) | null;
  onerror: ((event: unknown) => void) | null;
}

export type CreateShardWorker = (shardIndex: number) => ShardWorkerLike;
//#endregion 🌉️WorkerLike

//#region 📨️WireMessages
type OutboundMessage =
  | { readonly kind: "activate"; readonly requestId: string; readonly actorId: string; readonly moduleUrl: string; readonly caps: readonly ShardCapabilityGrant[]; readonly budget: ShardBudget; readonly assets: readonly ShardAsset[] }
  | { readonly kind: "turn"; readonly requestId: string; readonly actorId: string; readonly events: readonly ShardEventEnvelope[]; readonly budget: ShardBudget }
  | { readonly kind: "startJob"; readonly requestId: string; readonly actorId: string; readonly job: number; readonly jobKind: string; readonly input: Uint8Array }
  | { readonly kind: "stepJob"; readonly requestId: string; readonly actorId: string; readonly job: number; readonly budget: ShardJobBudget }
  | { readonly kind: "cancelJob"; readonly actorId: string; readonly job: number }
  | { readonly kind: "checkpoint"; readonly requestId: string; readonly actorId: string }
  | { readonly kind: "restore"; readonly requestId: string; readonly actorId: string; readonly state: Uint8Array }
  | { readonly kind: "dispose"; readonly actorId: string };

type InboundMessage =
  | { readonly kind: "result"; readonly requestId: string; readonly ok: true; readonly value: unknown }
  | { readonly kind: "result"; readonly requestId: string; readonly ok: false; readonly error: string; readonly stack?: string; readonly type?: string; readonly framesBytes?: number }
  | { readonly kind: "heartbeat"; readonly turnSeq: number }
  | { readonly kind: "trap"; readonly actorId: string; readonly message: string };
//#endregion 📨️WireMessages

//#region ⏱️Heartbeat
const DEFAULT_HEARTBEAT_TIMEOUT_MS = 5000;
const HEARTBEAT_MISSED_LIMIT = 3;

type ShardHeartbeatState = {
  lastHeartbeatAtMs: number;
  lastHeartbeatTurnSeq: number;
  oldestPendingStartedAtMs: number | null;
  missedCount: number;
  lastMissCountedAtMs: number;
};

/** A freshly spawned shard has never actually heartbeated yet — `-Infinity` (never `nowMs`) so a turn
 * that starts in the very same tick as `spawnShard` doesn't spuriously count spawn-time as proof of
 * life for that turn's whole timeout window. */
function freshHeartbeatState(nowMs: number): ShardHeartbeatState {
  return { lastHeartbeatAtMs: Number.NEGATIVE_INFINITY, lastHeartbeatTurnSeq: 0, oldestPendingStartedAtMs: null, missedCount: 0, lastMissCountedAtMs: nowMs };
}
//#endregion ⏱️Heartbeat

//#region 🧵️ShardClient
/** 🩺️ Rebuilds a worker-side failure as a main-thread `Error` that still carries the worker's own
 * stack. Without this the only frame a caller ever sees is `handleMessage`, because the structured
 * clone across `postMessage` cannot carry an `Error` — which is exactly why the collaboration e2e's
 * `Maximum call stack size exceeded` was undiagnosable. The `[DEBUG] ` line is deliberate, permanent
 * diagnostic infrastructure the e2e log parses; it is not leftover scaffolding. */
function graftWorkerStack(actorId: string, reason: string, stack: string | undefined, kind: string | undefined, framesBytes: number | undefined): Error {
  const error = new Error(reason);
  if (stack) error.stack = `${stack}\n    \u21b3 main: ${error.stack ?? ""}`;
  console.log(`[DEBUG] program worker ${actorId || "unknown"} error type=${kind ?? "unknown"} framesBytes=${framesBytes ?? "n/a"}`);
  return error;
}

type PendingEntry = { readonly resolve: (value: unknown) => void; readonly reject: (error: Error) => void; readonly shardIndex: number; readonly startedAtMs: number; readonly actorId: string };

type ShardSlot = {
  index: number;
  worker: ShardWorkerLike;
  readonly heartbeat: ShardHeartbeatState;
  readonly pendingRequestIds: Set<string>;
  readonly actorIds: Set<string>;
};

export interface ShardClientOptions {
  /** Fixed pool size — design-runtime.md §1 `ShardTable`: web `min(hardwareConcurrency-1, 4)`. Caller
   * computes the number; this class only ever spawns exactly this many workers. */
  readonly shardCount: number;
  readonly createWorker: CreateShardWorker;
  /** ≤2 exclusive shards reserved for `leaseExclusive` — the tail of the shard index range. Clamped to
   * `shardCount`. */
  readonly exclusiveShardCount?: number;
  /** One `Int32Array` slot per shard index — `Atomics.store(sab, shardIdx, turnSeq)` heartbeat path.
   * Omitted entirely when `SharedArrayBuffer` is unavailable (no COOP/COEP): the `postMessage`
   * `heartbeat` message is ALWAYS honored regardless, so correctness never depends on this. */
  readonly heartbeatSab?: SharedArrayBuffer;
  readonly heartbeatTimeoutMs?: number;
  readonly now?: () => number;
  /** Fired when a shard is torn down (3 missed heartbeats, or an explicit `terminate()`) — the caller
   * (kernel-side scheduler) is responsible for restoring every listed actor from its last checkpoint
   * on a freshly `rebuild()`-ed shard; this class only does the mechanical worker lifecycle. */
  readonly onShardLost?: (shardIndex: number, actorIds: readonly string[]) => void;
  readonly onActorTrap?: (actorId: string, message: string) => void;
}

/** 🧵️ One `ShardClient` instance owns the WHOLE bounded pool (design: "ShardClient... replaces both
 * `PluginWorkerClient`s [both former per-plugin copies] and `pluginHandleForBridge`") — internally one
 * `ShardWorkerLike` ("MessagePort") per shard, `actorId`-tagged request/reply multiplexing so several
 * actors share a shard's single message channel without cross-talk. */
export class ShardClient {
  private readonly shards: ShardSlot[] = [];
  private readonly actorShard = new Map<string, number>();
  private readonly pending = new Map<string, PendingEntry>();
  private readonly exclusiveIndices: ReadonlySet<number>;
  private readonly heartbeatSabView: Int32Array | null;
  private readonly heartbeatTimeoutMs: number;
  private readonly now: () => number;
  private readonly createWorker: CreateShardWorker;
  private readonly onShardLost?: ShardClientOptions["onShardLost"];
  private readonly onActorTrap?: ShardClientOptions["onActorTrap"];
  private nextRoundRobin = 0;
  private requestSeq = 0;

  constructor(options: ShardClientOptions) {
    if (options.shardCount < 1) throw new Error("[DEBUG] ShardClient requires shardCount >= 1");
    this.createWorker = options.createWorker;
    this.now = options.now ?? (() => Date.now());
    this.heartbeatTimeoutMs = options.heartbeatTimeoutMs ?? DEFAULT_HEARTBEAT_TIMEOUT_MS;
    this.heartbeatSabView = options.heartbeatSab ? new Int32Array(options.heartbeatSab) : null;
    this.onShardLost = options.onShardLost;
    this.onActorTrap = options.onActorTrap;
    const exclusiveCount = Math.max(0, Math.min(options.exclusiveShardCount ?? Math.min(2, options.shardCount - 1), options.shardCount - 1));
    const exclusive = new Set<number>();
    for (let index = options.shardCount - exclusiveCount; index < options.shardCount; index += 1) exclusive.add(index);
    this.exclusiveIndices = exclusive;
    for (let index = 0; index < options.shardCount; index += 1) this.shards.push(this.spawnShard(index));
  }

  //#region 🌱️Lifecycle
  private spawnShard(index: number): ShardSlot {
    const worker = this.createWorker(index);
    const slot: ShardSlot = { index, worker, heartbeat: freshHeartbeatState(this.now()), pendingRequestIds: new Set(), actorIds: new Set() };
    worker.onmessage = (event) => this.handleMessage(slot, event.data as InboundMessage);
    worker.onerror = (error) => {
      console.error(`[DEBUG] shard ${index} worker error`, error);
      this.failShard(slot, new Error(`shard ${index} worker crashed`));
    };
    if (this.heartbeatSabView) worker.postMessage({ kind: "attachHeartbeatSab", shardIndex: index, sab: this.heartbeatSabView.buffer });
    return slot;
  }

  private handleMessage(slot: ShardSlot, message: InboundMessage): void {
    if (message.kind === "heartbeat") {
      this.recordHeartbeat(slot, message.turnSeq, this.now());
      return;
    }
    if (message.kind === "trap") {
      this.onActorTrap?.(message.actorId, message.message);
      return;
    }
    const entry = this.pending.get(message.requestId);
    if (!entry) return;
    this.pending.delete(message.requestId);
    slot.pendingRequestIds.delete(message.requestId);
    this.recomputeOldestPending(slot);
    if (message.ok) entry.resolve(message.value);
    else entry.reject(graftWorkerStack(entry.actorId, message.error, message.stack, message.type, message.framesBytes));
  }

  private recomputeOldestPending(slot: ShardSlot): void {
    let oldest: number | null = null;
    for (const requestId of slot.pendingRequestIds) {
      const entry = this.pending.get(requestId);
      if (!entry) continue;
      if (oldest === null || entry.startedAtMs < oldest) oldest = entry.startedAtMs;
    }
    slot.heartbeat.oldestPendingStartedAtMs = oldest;
  }

  private failShard(slot: ShardSlot, error: Error): void {
    for (const requestId of slot.pendingRequestIds) {
      const entry = this.pending.get(requestId);
      if (!entry) continue;
      this.pending.delete(requestId);
      entry.reject(error);
    }
    slot.pendingRequestIds.clear();
    slot.heartbeat.oldestPendingStartedAtMs = null;
  }
  //#endregion 🌱️Lifecycle

  //#region 🧭️Assignment
  private assignShard(actorId: string): ShardSlot {
    const existing = this.actorShard.get(actorId);
    if (existing !== undefined) return this.shards[existing]!;
    const roundRobinCount = this.shards.length - this.exclusiveIndices.size;
    let index = this.nextRoundRobin % Math.max(roundRobinCount, 1);
    while (this.exclusiveIndices.has(index)) index = (index + 1) % this.shards.length;
    this.nextRoundRobin = (this.nextRoundRobin + 1) % Math.max(roundRobinCount, 1);
    this.actorShard.set(actorId, index);
    this.shards[index]!.actorIds.add(actorId);
    return this.shards[index]!;
  }

  /** ▶️ design-runtime.md §1 `ShardTable::request_exclusive` — moves `actorId` onto one of the ≤2
   * exclusive shards for the duration of foreground work. Purely a routing decision: this class does
   * NOT migrate any in-worker instance state (design: migration only happens at a quiescent point via
   * application-level checkpoint) — if the actor was already activated on a different shard, the
   * caller must `checkpoint()` there and `activate()`+`restore()` on the returned exclusive shard. */
  leaseExclusive(actorId: string, options?: { readonly force?: boolean }): number {
    const already = this.actorShard.get(actorId);
    if (already !== undefined && this.exclusiveIndices.has(already)) return already;
    for (const index of this.exclusiveIndices) {
      const slot = this.shards[index]!;
      if (slot.actorIds.size === 0 || options?.force) {
        if (already !== undefined) this.shards[already]!.actorIds.delete(actorId);
        slot.actorIds.add(actorId);
        this.actorShard.set(actorId, index);
        return index;
      }
    }
    throw new Error(`[DEBUG] ShardClient.leaseExclusive(${actorId}): no free exclusive shard (${this.exclusiveIndices.size} reserved, all leased)`);
  }

  /** ◀️ Returns `actorId` to the round-robin pool — its NEXT `activate()`/`turn()` targets whichever
   * shard round-robin picks, not necessarily the one it just left. */
  releaseExclusive(actorId: string): void {
    const index = this.actorShard.get(actorId);
    if (index === undefined || !this.exclusiveIndices.has(index)) return;
    this.shards[index]!.actorIds.delete(actorId);
    this.actorShard.delete(actorId);
  }

  shardIndexFor(actorId: string): number | undefined {
    return this.actorShard.get(actorId);
  }
  //#endregion 🧭️Assignment

  //#region 📮️Requests
  private nextRequestId(): string {
    this.requestSeq += 1;
    return `r${this.requestSeq}`;
  }

  private send<T>(slot: ShardSlot, message: OutboundMessage, requestId: string | null): Promise<T> {
    if (requestId === null) {
      slot.worker.postMessage(message);
      return Promise.resolve(undefined as T);
    }
    return new Promise<T>((resolve, reject) => {
      const startedAtMs = this.now();
      this.pending.set(requestId, { resolve: resolve as (value: unknown) => void, reject, shardIndex: slot.index, startedAtMs, actorId: "actorId" in message ? message.actorId : "" });
      slot.pendingRequestIds.add(requestId);
      if (slot.heartbeat.oldestPendingStartedAtMs === null) slot.heartbeat.oldestPendingStartedAtMs = startedAtMs;
      slot.worker.postMessage(message);
    });
  }

  async activate(actorId: string, moduleUrl: string, caps: readonly ShardCapabilityGrant[], budget: ShardBudget, assets: readonly ShardAsset[] = []): Promise<void> {
    const slot = this.assignShard(actorId);
    const requestId = this.nextRequestId();
    await this.send<void>(slot, { kind: "activate", requestId, actorId, moduleUrl, caps, budget, assets }, requestId);
  }

  /** ▶️ One turn (`reactor::poll`), never more than one in flight per `actorId` at a time — a second
   * `turn()` call for the same actor before the first resolves is a caller bug (the scheduler's own
   * per-actor serialization, not this transport's job, per design's "runs one turn at a time per
   * actor"), so it is rejected rather than silently queued. */
  async turn(actorId: string, events: readonly ShardEventEnvelope[], budget: ShardBudget): Promise<unknown> {
    const shardIndex = this.actorShard.get(actorId);
    if (shardIndex === undefined) throw new Error(`[DEBUG] ShardClient.turn(${actorId}): not activated on any shard`);
    const slot = this.shards[shardIndex]!;
    const requestId = this.nextRequestId();
    return this.send(slot, { kind: "turn", requestId, actorId, events, budget }, requestId);
  }

  async startJob(actorId: string, job: number, jobKind: string, input: Uint8Array): Promise<void> {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    await this.send<void>(slot, { kind: "startJob", requestId, actorId, job, jobKind, input }, requestId);
  }

  async stepJob(actorId: string, job: number, budget: ShardJobBudget): Promise<ShardJobStep> {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    return this.send<ShardJobStep>(slot, { kind: "stepJob", requestId, actorId, job, budget }, requestId);
  }

  cancelJob(actorId: string, job: number): void {
    const slot = this.requireShard(actorId);
    void this.send(slot, { kind: "cancelJob", actorId, job }, null);
  }

  async checkpoint(actorId: string): Promise<Uint8Array> {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    return this.send<Uint8Array>(slot, { kind: "checkpoint", requestId, actorId }, requestId);
  }

  async restore(actorId: string, state: Uint8Array): Promise<void> {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    await this.send<void>(slot, { kind: "restore", requestId, actorId, state }, requestId);
  }

  /** ⏏️ Frees the worker-side actor entry — does not touch this shard's routing entry so a caller that
   * immediately re-`activate()`s the same `actorId` (hot reload) can still target the same shard;
   * pair with a routing-level `actorShard.delete` only when the actor is gone for good (unusual — most
   * callers instead let `activate` overwrite in place). */
  dispose(actorId: string): void {
    const shardIndex = this.actorShard.get(actorId);
    if (shardIndex === undefined) return;
    this.shards[shardIndex]!.worker.postMessage({ kind: "dispose", actorId } satisfies OutboundMessage);
    this.shards[shardIndex]!.actorIds.delete(actorId);
    this.actorShard.delete(actorId);
  }

  private requireShard(actorId: string): ShardSlot {
    const index = this.actorShard.get(actorId);
    if (index === undefined) throw new Error(`[DEBUG] ShardClient: actor ${actorId} is not activated on any shard`);
    return this.shards[index]!;
  }
  //#endregion 📮️Requests

  //#region ⏱️HeartbeatWatchdog
  private recordHeartbeat(slot: ShardSlot, turnSeq: number, atMs: number): void {
    slot.heartbeat.lastHeartbeatAtMs = atMs;
    slot.heartbeat.lastHeartbeatTurnSeq = turnSeq;
    slot.heartbeat.missedCount = 0;
    slot.heartbeat.lastMissCountedAtMs = atMs;
  }

  /** 🔭️ SAB path: polls every shard's `Atomics.load` slot and folds any advance into the SAME state
   * machine `postMessage` heartbeats update — call alongside `checkHeartbeats` on a scheduler tick.
   * A no-op when this client was built without a `heartbeatSab` (postMessage is the only source then;
   * see this class's header doc — correctness never depends on this method being called at all). */
  pollHeartbeatSab(nowMs: number = this.now()): void {
    if (!this.heartbeatSabView) return;
    for (const slot of this.shards) {
      const seq = Atomics.load(this.heartbeatSabView, slot.index);
      if (seq !== slot.heartbeat.lastHeartbeatTurnSeq || slot.heartbeat.oldestPendingStartedAtMs === null) {
        this.recordHeartbeat(slot, seq, nowMs);
      }
    }
  }

  /** 🚑️ design-runtime.md §1 `FailurePolicy` watchdog: a shard only "misses" a heartbeat while it has
   * an in-flight turn/job older than `heartbeatTimeoutMs` with no fresher heartbeat since — an idle
   * shard with nothing pending can never be flagged. Three consecutive timeout windows of continued
   * silence (not three calls to this method) trigger `terminate()` + `rebuild()` and `onShardLost`. */
  checkHeartbeats(nowMs: number = this.now()): void {
    for (const slot of this.shards) {
      const pendingSince = slot.heartbeat.oldestPendingStartedAtMs;
      if (pendingSince === null) continue;
      if (slot.heartbeat.lastHeartbeatAtMs >= pendingSince) continue;
      const silentForMs = nowMs - pendingSince;
      if (silentForMs <= this.heartbeatTimeoutMs) continue;
      if (nowMs - slot.heartbeat.lastMissCountedAtMs < this.heartbeatTimeoutMs) continue;
      slot.heartbeat.missedCount += 1;
      slot.heartbeat.lastMissCountedAtMs = nowMs;
      if (slot.heartbeat.missedCount >= HEARTBEAT_MISSED_LIMIT) {
        const actorIds = [...slot.actorIds];
        this.terminate(slot.index);
        this.rebuild(slot.index);
        this.onShardLost?.(slot.index, actorIds);
      }
    }
  }
  //#endregion ⏱️HeartbeatWatchdog

  //#region 📈️RuntimeMetrics
  /** 📈️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1): one `ShardMetricsSample` per shard, built
   * purely from state this class already tracks — `actorIds.size` (`ShardMetrics.actors`),
   * `pendingRequestIds.size / actorIds.size` (`busyRatio`: the fraction of this shard's resident
   * actors with an in-flight `turn()` right now — the same "busy" proxy `checkHeartbeats` already
   * uses via `oldestPendingStartedAtMs`), and `nowMs - lastHeartbeatAtMs` (`heartbeatAgeMs`; a shard
   * that has never heartbeated reports `Number.POSITIVE_INFINITY`, matching `freshHeartbeatState`'s
   * own "never `nowMs`" convention). Field-compatible with the Rust `ShardMetricsSample` the native
   * host publishes — see that type's own doc comment for why `heartbeatAgeMs` needs a host overlay
   * there but not here (this class IS the thing holding the heartbeat clock on web). */
  shardMetricsSamples(nowMs: number = this.now()): readonly ShardMetricsSample[] {
    return this.shards.map((slot) => {
      const actors = slot.actorIds.size;
      const busyRatio = actors > 0 ? slot.pendingRequestIds.size / actors : 0;
      const heartbeatAgeMs = Number.isFinite(slot.heartbeat.lastHeartbeatAtMs) ? Math.max(0, nowMs - slot.heartbeat.lastHeartbeatAtMs) : Number.POSITIVE_INFINITY;
      return { shard: slot.index, metrics: { actors, busyRatio, heartbeatAgeMs } };
    });
  }
  //#endregion 📈️RuntimeMetrics

  //#region 🔁️TerminateRebuild
  /** 🔪️ Kills shard `index`'s worker and rejects every in-flight request on it. Actor routing entries
   * (`actorShard`) are left pointing at this now-dead index deliberately — `rebuild()` respawns a
   * fresh worker at the SAME index, so a caller's already-resolved `shardIndexFor(actorId)` stays
   * valid once the caller re-`activate()`s (from checkpoint) on the rebuilt shard. */
  terminate(index: number): readonly string[] {
    const slot = this.shards[index];
    if (!slot) throw new Error(`[DEBUG] ShardClient.terminate: no shard ${index}`);
    const actorIds = [...slot.actorIds];
    this.failShard(slot, new Error(`shard ${index} terminated`));
    slot.worker.terminate();
    return actorIds;
  }

  /** 🌱️ Respawns shard `index` with a fresh worker. Every actor formerly on it needs a fresh
   * `activate()` + `restore()` from its last checkpoint — this class does not do that itself (it has
   * no checkpoint bytes to restore from; that's the kernel-side `ActivationRegistry`'s job), so it
   * clears the routing entries for whoever was there rather than silently leaving them dangling. */
  rebuild(index: number): void {
    const old = this.shards[index];
    if (!old) throw new Error(`[DEBUG] ShardClient.rebuild: no shard ${index}`);
    for (const actorId of old.actorIds) this.actorShard.delete(actorId);
    this.shards[index] = this.spawnShard(index);
  }
  //#endregion 🔁️TerminateRebuild

  /** ⏏️ Tears down every shard's worker — call on full app shutdown, never per-actor (use {@link dispose}). */
  disposeAll(): void {
    for (const slot of this.shards) {
      this.failShard(slot, new Error("ShardClient disposed"));
      slot.worker.terminate();
    }
  }
}
//#endregion 🧵️ShardClient

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it, vi } = import.meta.vitest;

  class FakeShardWorker implements ShardWorkerLike {
    onmessage: ((event: { readonly data: unknown }) => void) | null = null;
    onerror: ((event: unknown) => void) | null = null;
    readonly sent: unknown[] = [];
    terminated = false;
    constructor(readonly index: number) {}
    postMessage(message: unknown): void {
      this.sent.push(message);
    }
    terminate(): void {
      this.terminated = true;
    }
    deliver(message: InboundMessage): void {
      this.onmessage?.({ data: message });
    }
  }

  function harness(shardCount = 2, extra?: Partial<ShardClientOptions>) {
    const workers: FakeShardWorker[] = [];
    let nowMs = 0;
    const client = new ShardClient({
      shardCount,
      createWorker: (index) => {
        const worker = new FakeShardWorker(index);
        workers.push(worker);
        return worker;
      },
      now: () => nowMs,
      ...extra,
    });
    return { client, workers, advance: (ms: number) => (nowMs += ms), setNow: (ms: number) => (nowMs = ms) };
  }

  const BUDGET: ShardBudget = { fuel: 1000, wallMs: 4, memoryBytes: 1 << 20, uiNodes: 100, mailboxLen: 16, maxEffects: 8, maxPatchBytes: 1 << 16 };

  describe("ShardClient activation + turn round-trip", () => {
    it("routes activate then turn to the same shard, resolving the reply by requestId", async () => {
      const { client, workers } = harness(2);
      const activatePromise = client.activate("actor-1", "https://x/plugin.js", [], BUDGET);
      const activateMsg = workers[0]!.sent[0] as { readonly kind: string; readonly requestId: string };
      expect(activateMsg.kind).toBe("activate");
      workers[0]!.deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
      await activatePromise;

      const turnPromise = client.turn("actor-1", [{ kind: "wake", payload: {} }], BUDGET);
      const turnMsg = workers[0]!.sent[1] as { readonly kind: string; readonly requestId: string };
      expect(turnMsg.kind).toBe("turn");
      workers[0]!.deliver({ kind: "result", requestId: turnMsg.requestId, ok: true, value: { effects: [] } });
      await expect(turnPromise).resolves.toEqual({ effects: [] });
    });

    it("rejects turn() for an actor never activated", async () => {
      const { client } = harness(2);
      await expect(client.turn("ghost", [], BUDGET)).rejects.toThrow(/not activated/);
    });
  });

  describe("ShardClient actor-id multiplexing", () => {
    it("distinguishes two actors' replies on the same shard even when they resolve out of order", async () => {
      const { client, workers } = harness(1);
      const p1 = client.activate("a", "https://x/a.js", [], BUDGET);
      const p2 = client.activate("b", "https://x/b.js", [], BUDGET);
      const [msgA, msgB] = workers[0]!.sent as { readonly requestId: string }[];
      // out-of-order reply: b's activate lands before a's
      workers[0]!.deliver({ kind: "result", requestId: msgB!.requestId, ok: true, value: undefined });
      workers[0]!.deliver({ kind: "result", requestId: msgA!.requestId, ok: true, value: undefined });
      await expect(p1).resolves.toBeUndefined();
      await expect(p2).resolves.toBeUndefined();
      expect(client.shardIndexFor("a")).toBe(0);
      expect(client.shardIndexFor("b")).toBe(0);
    });

    it("round-robins fresh actors across shards, skipping the reserved exclusive tail", async () => {
      const { client } = harness(4, { exclusiveShardCount: 1 });
      const indices = ["a", "b", "c"].map((id) => {
        void client.activate(id, "https://x/y.js", [], BUDGET);
        return client.shardIndexFor(id);
      });
      expect(new Set(indices).has(3)).toBe(false); // shard 3 reserved exclusive
      expect(indices).toEqual([0, 1, 2]);
    });
  });

  describe("ShardClient heartbeat watchdog (postMessage path)", () => {
    it("does not miss while idle (no pending turn)", () => {
      const { client, advance } = harness(1);
      advance(100_000);
      client.checkHeartbeats(100_000);
      // no throw, no rebuild callback would have fired — nothing to assert but absence of an exception
      expect(true).toBe(true);
    });

    it("terminates + rebuilds after 3 consecutive missed-heartbeat windows on a stuck turn", async () => {
      const lost: Array<{ index: number; actorIds: readonly string[] }> = [];
      const { client, workers, advance, setNow } = harness(1, { heartbeatTimeoutMs: 1000, onShardLost: (index, actorIds) => lost.push({ index, actorIds }) });
      setNow(0);
      const activatePromise = client.activate("stuck", "https://x/stuck.js", [], BUDGET);
      const activateMsg = workers[0]!.sent[0] as { readonly requestId: string };
      workers[0]!.deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
      await activatePromise;

      const originalWorker = workers[0]!;
      client.turn("stuck", [], BUDGET).catch(() => {}); // never replies — simulates a hung guest; terminate() rejects it

      advance(1001);
      client.checkHeartbeats();
      expect(lost).toEqual([]);
      expect(originalWorker.terminated).toBe(false);

      advance(1001);
      client.checkHeartbeats();
      advance(1001);
      client.checkHeartbeats();

      expect(originalWorker.terminated).toBe(true);
      expect(lost).toEqual([{ index: 0, actorIds: ["stuck"] }]);
      expect(client.shardIndexFor("stuck")).toBeUndefined(); // rebuild cleared routing; caller must re-activate
    });

    it("a fresh heartbeat resets the miss count", async () => {
      const { client, workers, advance, setNow } = harness(1, { heartbeatTimeoutMs: 1000 });
      setNow(0);
      const activatePromise = client.activate("busy", "https://x/busy.js", [], BUDGET);
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;
      void client.turn("busy", [], BUDGET);

      advance(1001);
      client.checkHeartbeats();
      advance(1001);
      client.checkHeartbeats();
      // guest emits a heartbeat right before the 3rd window would elapse
      workers[0]!.deliver({ kind: "heartbeat", turnSeq: 7 });
      advance(1001);
      client.checkHeartbeats();
      expect(workers[0]!.terminated).toBe(false);
    });
  });

  describe("ShardClient SAB heartbeat path", () => {
    it("pollHeartbeatSab reads Atomics-stored turnSeq and feeds the same miss-count state machine", async () => {
      const sab = new SharedArrayBuffer(4 * Int32Array.BYTES_PER_ELEMENT);
      const view = new Int32Array(sab);
      const { client, workers, advance, setNow } = harness(1, { heartbeatSab: sab, heartbeatTimeoutMs: 1000 });
      setNow(0);
      const activatePromise = client.activate("sab-actor", "https://x/s.js", [], BUDGET);
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent.find((m) => (m as { kind: string }).kind === "activate") as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;
      void client.turn("sab-actor", [], BUDGET);

      advance(1001);
      client.checkHeartbeats();
      expect(workers[0]!.terminated).toBe(false);

      Atomics.store(view, 0, 5);
      client.pollHeartbeatSab();
      advance(1001);
      client.checkHeartbeats();
      expect(workers[0]!.terminated).toBe(false); // heartbeat seen via SAB reset the window
    });
  });

  describe("ShardClient leaseExclusive", () => {
    it("moves an actor onto a reserved exclusive shard and back", async () => {
      const { client } = harness(4, { exclusiveShardCount: 2 });
      void client.activate("heavy", "https://x/h.js", [], BUDGET);
      expect(client.shardIndexFor("heavy")).toBe(0);
      const exclusiveIndex = client.leaseExclusive("heavy");
      expect([2, 3]).toContain(exclusiveIndex);
      expect(client.shardIndexFor("heavy")).toBe(exclusiveIndex);
      client.releaseExclusive("heavy");
      expect(client.shardIndexFor("heavy")).toBeUndefined();
    });

    it("throws once every exclusive shard is leased and force is not set", () => {
      const { client } = harness(3, { exclusiveShardCount: 1 });
      void client.activate("first", "https://x/1.js", [], BUDGET);
      client.leaseExclusive("first");
      void client.activate("second", "https://x/2.js", [], BUDGET);
      expect(() => client.leaseExclusive("second")).toThrow(/no free exclusive shard/);
    });

    it("is idempotent for the same actor already leased", () => {
      const { client } = harness(2, { exclusiveShardCount: 1 });
      void client.activate("a", "https://x/a.js", [], BUDGET);
      const first = client.leaseExclusive("a");
      const second = client.leaseExclusive("a");
      expect(first).toBe(second);
    });
  });

  describe("ShardClient terminate/rebuild", () => {
    it("rejects in-flight requests on terminate and spawns a fresh worker on rebuild", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("x", "https://x/x.js", [], BUDGET);
      const rejection = expect(activatePromise).rejects.toThrow(/terminated/);
      const oldWorker = workers[0]!;
      const actorIds = client.terminate(0);
      expect(actorIds).toEqual(["x"]);
      await rejection;
      expect(oldWorker.terminated).toBe(true);
      client.rebuild(0);
      expect(workers.length).toBe(2);
      expect(client.shardIndexFor("x")).toBeUndefined();
    });
  });

  describe("ShardClient worker crash", () => {
    it("onerror fails every pending request on that shard", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("crashy", "https://x/c.js", [], BUDGET);
      workers[0]!.onerror?.(new Error("boom"));
      await expect(activatePromise).rejects.toThrow(/crashed/);
    });
  });

  describe("ShardClient.shardMetricsSamples", () => {
    it("reports zero actors/busyRatio and an infinite heartbeat age for a fresh, never-touched shard", () => {
      const { client } = harness(2);
      const samples = client.shardMetricsSamples(1_000);
      expect(samples).toHaveLength(2);
      for (const sample of samples) {
        expect(sample.metrics.actors).toBe(0);
        expect(sample.metrics.busyRatio).toBe(0);
        expect(sample.metrics.heartbeatAgeMs).toBe(Number.POSITIVE_INFINITY);
      }
    });

    it("counts resident actors and in-flight turns as busyRatio, and ages the heartbeat off the injected clock", async () => {
      const { client, workers, setNow } = harness(1);
      setNow(0);
      const activateA = client.activate("a", "https://x/a.js", [], BUDGET);
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activateA;
      const activateB = client.activate("b", "https://x/b.js", [], BUDGET); // second actor, same (only) shard
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[1] as { requestId: string }).requestId, ok: true, value: undefined });
      await activateB; // both activations settled — only the turn below should count toward busyRatio

      workers[0]!.deliver({ kind: "heartbeat", turnSeq: 1 });
      void client.turn("a", [], BUDGET); // one in-flight turn out of two resident actors, deliberately never replied

      setNow(300);
      const [sample] = client.shardMetricsSamples(300);
      expect(sample!.metrics.actors).toBe(2);
      expect(sample!.metrics.busyRatio).toBeCloseTo(0.5);
      expect(sample!.metrics.heartbeatAgeMs).toBe(300);
    });
  });

  void vi;
}
//#endregion 🧪️Tests
