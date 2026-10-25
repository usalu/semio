import type { ShardClientState } from "../🗃️state/🟦️.ts";
import type { ShardClientHubPort } from "../🧩️hub/🟦️.ts";
import type { PendingEntry, ShardSlot } from "../🔐️activation/🟦️.ts";
import { describeShardSilence, evaluateShardLiveness } from "../💓️heartbeat/🟦️.ts";
import type { ShardMetricsSample } from "../📜️wire/🟦️.ts";

/** ⏱️ Folds every sign of life into the per-shard heartbeat state and terminates and rebuilds a silent shard. */
export class ShardWatchdog {
  private readonly state: ShardClientState;
  private readonly hub: ShardClientHubPort;
  constructor(hub: ShardClientHubPort) { this.hub = hub; this.state = hub.state; }

  /** 🫀️ Every inbound message is proof this worker's event loop is running — a start-of-request
   * heartbeat, one of the generated worker's `module-fetch`/`module-ready`/`actor-ready`/`progress`
   * heartbeats, a `result`, a `trap`, an effect `frame`. {@link evaluateShardLiveness} reads exactly
   * this clock, so a long turn that keeps ticking is never mistaken for a wedged one. */
  noteLiveness(slot: ShardSlot, atMs: number): void {
    slot.heartbeat.lastLivenessAtMs = atMs;
    slot.heartbeat.missedCount = 0;
    slot.heartbeat.lastMissCountedAtMs = atMs;
    if (!this.state.onLiveness) return;
    try {
      this.state.onLiveness(slot.index, slot.heartbeat.lastHeartbeatPhase);
    } catch (error) {
      console.error("shard liveness subscriber failed", error);
    }
  }

  recordHeartbeat(slot: ShardSlot, turnSeq: number, atMs: number, phase: string | null = slot.heartbeat.lastHeartbeatPhase): void {
    slot.heartbeat.lastHeartbeatAtMs = atMs;
    slot.heartbeat.lastHeartbeatTurnSeq = turnSeq;
    slot.heartbeat.lastHeartbeatPhase = phase;
    this.noteLiveness(slot, atMs);
  }

  /** 📬️ Every request still outstanding on `slot`, oldest first — the watchdog's whole view of what
   * the worker is busy with, and the payload {@link describeShardSilence} turns into a cause. */
  private outstandingRequests(slot: ShardSlot): readonly PendingEntry[] {
    const entries: PendingEntry[] = [];
    for (const requestId of slot.pendingRequestIds) {
      const entry = this.state.pending.get(requestId);
      if (entry) entries.push(entry);
    }
    return entries.sort((left, right) => left.startedAtMs - right.startedAtMs);
  }

  /**
   * 🔭️ SAB path: polls every shard's `Atomics.load` slot and folds any advance into the SAME state
   * machine `postMessage` heartbeats update — call alongside `checkHeartbeats` on a scheduler tick.
   * A no-op when this client was built without a `heartbeatSab` (postMessage is the only source then;
   * see this class's header doc — correctness never depends on this method being called at all).
   *
   * 🫀️ Only a MONOTONIC ADVANCE is proof of life. `!==` used to accept a REGRESSION too, which a
   * worker that never processed `attachHeartbeatSab` produces on every tick (its slot stays `0`
   * while `postMessage` beats advance `lastHeartbeatTurnSeq`) — under the liveness rule that would
   * refresh a dead shard's clock forever and disarm the watchdog entirely.
   */
  pollHeartbeatSab(nowMs: number = this.state.now()): void {
    if (!this.state.heartbeatSabView) return;
    for (const slot of this.state.shards) {
      const seq = Atomics.load(this.state.heartbeatSabView, slot.index);
      if (seq > slot.heartbeat.lastHeartbeatTurnSeq || slot.heartbeat.oldestPendingStartedAtMs === null) {
        this.recordHeartbeat(slot, seq, nowMs);
      }
    }
  }

  /** 🚑️ Applies {@link evaluateShardLiveness} — the whole decision, including what "silence" means —
   * to every shard and executes the one side effect it can ask for. `SHARD_LIVENESS_POLICY.missedLimit`
   * consecutive timeout windows of real silence (not that many calls to this method) trigger
   * `terminate()` + `rebuild()` and `onShardLost`. */
  checkHeartbeats(nowMs: number = this.state.now()): void {
    for (const slot of this.state.shards) {
      const outstanding = this.outstandingRequests(slot);
      const decision = evaluateShardLiveness({
        nowMs,
        oldestPendingStartedAtMs: slot.heartbeat.oldestPendingStartedAtMs,
        lastLivenessAtMs: slot.heartbeat.lastLivenessAtMs,
        missedCount: slot.heartbeat.missedCount,
        lastMissCountedAtMs: slot.heartbeat.lastMissCountedAtMs,
        heartbeatTimeoutMs: this.state.heartbeatTimeoutMs,
        firstTurnTimeoutMs: this.state.firstTurnTimeoutMs,
        oldestPendingIsFirstTurn: outstanding[0]?.firstTurn ?? false,
      });
      slot.heartbeat.missedCount = decision.missedCount;
      slot.heartbeat.lastMissCountedAtMs = decision.lastMissCountedAtMs;
      if (!decision.terminate) continue;
      const actorIds = [...slot.actorIds];
      const detail = describeShardSilence({
        shardIndex: slot.index,
        nowMs,
        lastLivenessAtMs: slot.heartbeat.lastLivenessAtMs,
        lastHeartbeatPhase: slot.heartbeat.lastHeartbeatPhase,
        inFlight: outstanding.map((entry) => ({ kind: entry.kind, actorId: entry.actorId, startedAtMs: entry.startedAtMs, firstTurn: entry.firstTurn })),
      });
      this.hub.routing.terminate(slot.index, detail);
      this.hub.routing.rebuild(slot.index);
      this.state.onShardLost?.(slot.index, actorIds);
    }
  }

  /** ▶️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P4): self-ticks {@link pollHeartbeatSab} +
   * {@link checkHeartbeats} on a real interval. Before this method existed, NEITHER was ever called
   * outside this file's own tests (verified with `grep -rn "checkHeartbeats(\|pollHeartbeatSab("` across
   * the repo) — the watchdog's whole failure ladder was wired but nothing in production ever turned
   * the crank, so a wedged shard went undetected forever in the real app. Idempotent: calling this
   * again while already running is a no-op (use {@link stopWatchdog} first to change the interval).
   * Uses the real `setInterval`/`clearInterval` — same convention as this repo's own
   * `ActivationRegistry.startRuntimeMetricsPublisher` (kernel `🟦️.ts`), which tests with
   * `vi.useFakeTimers()`/`vi.advanceTimersByTime` rather than an injected interval function; this
   * class's `now` option already covers the OTHER half of the clock (what "too long ago" means), so
   * no separate injectable timer is needed for correctness, only fake timers for tests. */
  startWatchdog(intervalMs: number = this.state.watchdogIntervalMs): void {
    if (this.state.watchdogHandle !== null) return;
    this.state.watchdogHandle = setInterval(() => {
      this.pollHeartbeatSab();
      this.checkHeartbeats();
    }, intervalMs);
  }

  /** ⏹️ Cancels a running {@link startWatchdog} loop. Idempotent; also called from {@link disposeAll}. */
  stopWatchdog(): void {
    if (this.state.watchdogHandle === null) return;
    clearInterval(this.state.watchdogHandle);
    this.state.watchdogHandle = null;
  }

  /** 📈️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1): one `ShardMetricsSample` per shard, built
   * purely from state this class already tracks — `actorIds.size` (`ShardMetrics.actors`),
   * `pendingRequestIds.size / actorIds.size` (`busyRatio`: the fraction of this shard's resident
   * actors with an in-flight `turn()` right now — the same "busy" proxy `checkHeartbeats` already
   * uses via `oldestPendingStartedAtMs`), and `nowMs - lastHeartbeatAtMs` (`heartbeatAgeMs`; a shard
   * that has never heartbeated reports `Number.POSITIVE_INFINITY`, matching `freshHeartbeatState`'s
   * own "never `nowMs`" convention). Field-compatible with the Rust `ShardMetricsSample` the native
   * host publishes — see that type's own doc comment for why `heartbeatAgeMs` needs a host overlay
   * there but not here (this class IS the thing holding the heartbeat clock on web). */
  shardMetricsSamples(nowMs: number = this.state.now()): readonly ShardMetricsSample[] {
    return this.state.shards.map((slot) => {
      const actors = slot.actorIds.size;
      const busyRatio = actors > 0 ? slot.pendingRequestIds.size / actors : 0;
      const heartbeatAgeMs = Number.isFinite(slot.heartbeat.lastHeartbeatAtMs) ? Math.max(0, nowMs - slot.heartbeat.lastHeartbeatAtMs) : Number.POSITIVE_INFINITY;
      return { shard: slot.index, metrics: { actors, busyRatio, heartbeatAgeMs } };
    });
  }
}
