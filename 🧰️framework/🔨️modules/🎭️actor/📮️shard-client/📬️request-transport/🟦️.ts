import type { ShardClientState } from "../🗃️state/🟦️.ts";
import type { ShardClientHub } from "../🧩️wiring/🟦️.ts";
import { hopTraceEpochNowMs } from "@semio-tech/framework/trace";
import { OwnedActorTurnOutput } from "../../🪪️activation/🚪️instance/📥️output/🟦️.ts";
import { type ShardSlot, graftWorkerStack } from "../🔐️activation/🟦️.ts";
import { describeShardMessageError, describeShardWorkerError, formatShardWorkerFault, freshHeartbeatState } from "../💓️heartbeat/🟦️.ts";
import { type InboundMessage, type OutboundMessage, publishShardWorkerTurnSpans } from "../📨️messages/🟦️.ts";

/** 📬️ Spawns shard workers, posts correlated requests and settles them from the per-worker inbound messages. */
export class ShardRequestTransport {
  private readonly state: ShardClientState;
  constructor(private readonly hub: ShardClientHub) { this.state = hub.state; }

  spawnShard(index: number): ShardSlot {
    const worker = this.state.createWorker(index);
    const slot: ShardSlot = { index, worker, available: true, heartbeat: freshHeartbeatState(this.state.now()), pendingRequestIds: new Set(), actorIds: new Set() };
    worker.onmessage = (event) => this.handleMessage(slot, event.data as InboundMessage);
    worker.onerror = (error) => {
      if (this.state.shards[index] !== slot) return;
      const detail = describeShardWorkerError(error);
      this.failShard(slot, new Error(`shard ${index} worker crashed: ${detail}`));
    };
    worker.onmessageerror = (event) => {
      if (this.state.shards[index] !== slot) return;
      const detail = describeShardMessageError(index, event);
      this.failShard(slot, new Error(`shard ${index} worker crashed: ${detail}`));
    };
    if (this.state.heartbeatSabView) worker.postMessage({ kind: "attachHeartbeatSab", shardIndex: index, sab: this.state.heartbeatSabView.buffer });
    return slot;
  }

  /** 📬️ Per-worker inbound dispatch. `"frame"` is checked BEFORE the generic `pending`-lookup path
   * below (`"result"`'s implicit fallthrough) — mirroring `🟨️shard-worker.js`'s own `deliverEffectResult`
   * ordering note (🧪️ terra-web-bridges): an inbound `"frame"` carries no `requestId` of its own and is
   * never an answer this class is waiting on, so falling through to the generic path would look up a
   * `requestId` nothing ever registered and silently no-op, masking a real effect-request. */
  private handleMessage(slot: ShardSlot, message: InboundMessage): void {
    const answeredAtEpochMs = hopTraceEpochNowMs();
    if (!slot.available || this.state.shards[slot.index] !== slot) return;
    this.hub.watchdog.noteLiveness(slot, this.state.now());
    if (message.kind === "heartbeat") {
      this.hub.watchdog.recordHeartbeat(slot, message.turnSeq, this.state.now(), message.phase ?? null);
      return;
    }
    // 🫀️ A beat that rode this crossing instead of costing its own message — folded into exactly the
    // state a dedicated `heartbeat` would have reached, before anything else this message means.
    if (message.kind === "result" && message.beat && typeof message.beat.turnSeq === "number") this.hub.watchdog.recordHeartbeat(slot, message.beat.turnSeq, this.state.now(), message.beat.phase ?? null);
    if (message.kind === "worker-fault") {
      const detail = formatShardWorkerFault(slot.index, message);
      this.state.onActorTrap?.(message.actorId ?? "*", detail);
      return;
    }
    if (message.kind === "trap") {
      if ((message.actorId === "*" && message.activationGeneration === null) || this.hub.activations.inboundActivation(slot, message.actorId, message.activationGeneration)) this.state.onActorTrap?.(message.actorId, message.message);
      return;
    }
    if (message.kind === "frame") {
      const activation = this.hub.activations.inboundActivation(slot, message.actorId, message.activationGeneration);
      if (activation) this.hub.effects.handleInboundFrame(activation, message.frame);
      return;
    }
    if (message.kind === "original-turn-receipt" || message.kind === "original-producer-held") {
      const operation = message.kind === "original-turn-receipt" ? message.receipt.input.operation : message.operation;
      const generation = message.kind === "original-turn-receipt" ? message.receipt.input.generation : message.generation;
      const original = this.state.originalClosures.get(operation) ?? this.state.actorActivations.get(message.actorId);
      if (!original || original.slot !== slot || original.actorId !== message.actorId || original.generation !== message.activationGeneration || original.originalAdmission.operation !== operation || original.originalAdmission.generation !== generation) return;
      const before = this.state.originalReturns.get(operation) ?? Promise.resolve();
      const returned = before.then(() => message.kind === "original-turn-receipt" ? this.state.originalWorkStock.receive(original.originalAdmission, message.receipt) : this.state.originalWorkStock.hold(original.originalAdmission, message.refusal));
      this.state.originalReturns.set(operation, returned);
      void returned.catch(refusal => slot.worker.postMessage({ kind: "original-root-refusal", actorId: original.actorId, activationGeneration: original.generation, refusal }));
      return;
    }
    if (message.kind === "original-source-close") {
      const original = this.state.originalClosures.get(message.operation);
      if (!original || original.slot !== slot || original.actorId !== message.actorId || original.generation !== message.activationGeneration || original.originalAdmission.generation !== message.generation || message.sourceClosed !== true) return;
      void (this.state.originalReturns.get(message.operation) ?? Promise.resolve()).then(() => this.state.originalWorkStock.returnClosed(original.originalAdmission, true)).then(() => { this.state.originalClosures.delete(message.operation); this.state.originalReturns.delete(message.operation); }).catch(() => {});
      return;
    }
    const entry = this.state.pending.get(message.requestId);
    if (!entry || entry.slot !== slot || this.state.shards[slot.index] !== slot) return;
    if (message.kind === "result" && message.ok && message.timings) publishShardWorkerTurnSpans(message.timings, answeredAtEpochMs, entry.actorId);
    if (entry.output && !entry.output.captureResponse(message)) { entry.reject(new Error("actor-output.response-refused")); return; }
    try {
      this.state.pending.delete(message.requestId);
      slot.pendingRequestIds.delete(message.requestId);
      if (entry.kind === "turn" && entry.actorId !== "") this.state.actorsPastFirstTurn.add(entry.actorId);
      this.recomputeOldestPending(slot);
      if (message.ok) entry.resolve(message.value);
      else entry.reject(graftWorkerStack(entry.actorId, message.error, message.stack, message.type, message.framesBytes, message.retryableLifecycle));
    } catch (error) { entry.reject(error); }
  }

  private recomputeOldestPending(slot: ShardSlot): void {
    let oldest: number | null = null;
    for (const requestId of slot.pendingRequestIds) {
      const entry = this.state.pending.get(requestId);
      if (!entry) continue;
      if (oldest === null || entry.startedAtMs < oldest) oldest = entry.startedAtMs;
    }
    slot.heartbeat.oldestPendingStartedAtMs = oldest;
  }

  /** 🩹️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P4 fix): before this, `failShard` rejected every
   * in-flight request but left `this.actorShard`/`slot.actorIds` untouched — verified by reading the
   * pre-fix source: only `pendingRequestIds` and `heartbeat.oldestPendingStartedAtMs` were cleared.
   * That meant a shard whose worker died via `onerror` (which calls `failShard` directly, WITHOUT
   * `terminate()`+`rebuild()` — see `spawnShard`'s `worker.onerror` below) kept every one of its
   * actors routed to it: a later `activate()`/`turn()` for the same `actorId` would `postMessage` into
   * the same dead worker and hang forever, undetectable until the heartbeat watchdog's own 3-strike
   * ladder eventually caught it. Clearing the routing here means a dead shard stops receiving newly
   * routed work immediately, not only after `rebuild()` runs. */
  failShard(slot: ShardSlot, error: Error): void {
    slot.available = false;
    for (const requestId of slot.pendingRequestIds) {
      const entry = this.state.pending.get(requestId);
      if (!entry) continue;
      this.state.pending.delete(requestId);
      entry.reject(error);
    }
    slot.pendingRequestIds.clear();
    slot.heartbeat.oldestPendingStartedAtMs = null;
    for (const actorId of slot.actorIds) {
      this.hub.effects.abortOutstandingEffects(actorId);
      const activation = this.state.actorActivations.get(actorId);
      if (activation?.slot === slot) {
        activation.available = false;
        if (activation.instance) activation.instance.failure = "worker-lost";
      }
      this.state.actorShard.delete(actorId);
    }
    slot.actorIds.clear();
  }

  rejectActorPending(slot: ShardSlot, actorId: string, error: Error): void {
    for (const requestId of [...slot.pendingRequestIds]) {
      const entry = this.state.pending.get(requestId);
      if (entry?.actorId !== actorId) continue;
      this.state.pending.delete(requestId);
      slot.pendingRequestIds.delete(requestId);
      entry.reject(error);
    }
    this.recomputeOldestPending(slot);
  }

  nextRequestId(): string {
    if (this.state.requestSeq >= Number.MAX_SAFE_INTEGER) throw new Error("shard-request.sequence-exhausted");
    this.state.requestSeq += 1;
    return `r${this.state.requestSeq}`;
  }

  send<T>(slot: ShardSlot, message: OutboundMessage, requestId: string | null, posted?: () => void, output: OwnedActorTurnOutput | null = null): Promise<T> {
    if (requestId === null) {
      slot.worker.postMessage(message);
      posted?.();
      return Promise.resolve(undefined as T);
    }
    return new Promise<T>((resolve, reject) => {
      const startedAtMs = this.state.now();
      const actorId = "actorId" in message ? message.actorId : "";
      const firstTurn = message.kind === "turn" && actorId !== "" && !this.state.actorsPastFirstTurn.has(actorId);
      this.state.pending.set(requestId, { resolve: resolve as (value: unknown) => void, reject, slot, startedAtMs, actorId, kind: message.kind, firstTurn, output });
      slot.pendingRequestIds.add(requestId);
      if (slot.heartbeat.oldestPendingStartedAtMs === null) slot.heartbeat.oldestPendingStartedAtMs = startedAtMs;
      try { slot.worker.postMessage(message.kind === "turn" ? { ...message, postedAtEpochMs: hopTraceEpochNowMs() } : message); posted?.(); }
      catch (error) {
        this.state.pending.delete(requestId);
        slot.pendingRequestIds.delete(requestId);
        this.recomputeOldestPending(slot);
        reject(error);
      }
    });
  }
}
