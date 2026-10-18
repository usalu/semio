import { OriginalBootstrapOwner } from "../../🎟️retained-turn/🏛️bootstrap/🟦️.ts";
import { SHARD_LIVENESS_POLICY } from "../🫀️liveness/🟦️.ts";
import { OwnedResidentLedger } from "@semio-tech/value-ts/resident";
import { type HostEffectLedger, type PendingEntry, type ShardActivation, type ShardInstanceOwner, type ShardSlot, residentCapacity } from "../🔐️activation/🟦️.ts";
import { OwnedNativeUiPatchAuthority } from "../🩹️ui-patch/🟦️.ts";
import type { CreateShardWorker, HostEffectHandler } from "../🚨️quota/🟦️.ts";
import type { ShardClientOptions } from "../⚙️options/🟦️.ts";
import { DEFAULT_MAX_OUTSTANDING_EFFECTS_PER_ACTOR } from "../💓️heartbeat/🟦️.ts";

/** 🗃️ The single private state object of one {@link ShardClient}: every mutable field and every injected policy of the transport lives here, and the facade and its collaborators share it instead of holding many private fields. */
export class ShardClientState {
  readonly residentLedger: OwnedResidentLedger;
  readonly shards: ShardSlot[] = [];
  readonly actorShard = new Map<string, number>();
  readonly actorActivations = new Map<string, ShardActivation>();
  readonly instanceLifecycles = new Map<number, ShardInstanceOwner>();
  readonly instanceTurns = new WeakMap<object, { readonly owner: ShardInstanceOwner; readonly patches: WeakMap<object, OwnedNativeUiPatchAuthority> }>();
  readonly pending = new Map<string, PendingEntry>();
  /** 🫀️ Actors whose first turn on their CURRENT activation has already settled — the set
   * {@link evaluateShardLiveness}'s `oldestPendingIsFirstTurn` reads. Cleared for an actor whenever
   * it activates again, because a fresh activation is a fresh guest with the same one-off
   * initialization cost in front of it. */
  readonly actorsPastFirstTurn = new Set<string>();
  readonly exclusiveIndices: ReadonlySet<number>;
  readonly heartbeatSabView: Int32Array | null;
  readonly heartbeatTimeoutMs: number;
  readonly firstTurnTimeoutMs: number;
  readonly watchdogIntervalMs: number;
  readonly now: () => number;
  readonly createWorker: CreateShardWorker;
  readonly onShardLost?: ShardClientOptions["onShardLost"];
  readonly onActorTrap?: ShardClientOptions["onActorTrap"];
  readonly onLiveness?: ShardClientOptions["onLiveness"];
  readonly onHostEffect?: HostEffectHandler;
  readonly maxOutstandingEffectsPerActor: number;
  /** 🌉️ terra-shard-effect-bridge: `actorId` → (`requestId` → its `AbortController`) — the ledger
   * {@link handleEffectRequest}/{@link settleEffect}/{@link abortOutstandingEffects} share; its size
   * per actor IS the outstanding-effect count {@link handleEffectRequest} caps. */
  readonly outstandingEffectsByActor = new Map<string, HostEffectLedger>();
  effectReplySeq = 0;
  nextRoundRobin = 0;
  requestSeq = 0;
  activationGeneration = 0n;
  watchdogHandle: ReturnType<typeof setInterval> | null = null;
  readonly originalWorkStock: OriginalBootstrapOwner;
  readonly originalClosures = new Map<bigint, ShardActivation>();
  readonly originalReturns = new Map<bigint, Promise<void>>();
  constructor(options: ShardClientOptions) {
    if (!(options.originalWorkStock instanceof OriginalBootstrapOwner)) throw new Error("original-bootstrap.invalid-owner");
    this.originalWorkStock = options.originalWorkStock;
    try { Reflect.apply(residentCapacity(), options.residentLedger, []); } catch { throw new Error("actor-resident.invalid-ledger"); }
    this.residentLedger = options.residentLedger;
    if (options.shardCount < 1) throw new Error("ShardClient requires shardCount >= 1");
    this.createWorker = options.createWorker;
    this.now = options.now ?? (() => Date.now());
    this.heartbeatTimeoutMs = options.heartbeatTimeoutMs ?? SHARD_LIVENESS_POLICY.heartbeatTimeoutMs;
    this.firstTurnTimeoutMs = options.firstTurnTimeoutMs ?? SHARD_LIVENESS_POLICY.firstTurnTimeoutMs;
    this.watchdogIntervalMs = options.watchdogIntervalMs ?? this.heartbeatTimeoutMs;
    this.heartbeatSabView = options.heartbeatSab ? new Int32Array(options.heartbeatSab) : null;
    this.onShardLost = options.onShardLost;
    this.onActorTrap = options.onActorTrap;
    this.onLiveness = options.onLiveness;
    this.onHostEffect = options.onHostEffect;
    this.maxOutstandingEffectsPerActor = options.maxOutstandingEffectsPerActor ?? DEFAULT_MAX_OUTSTANDING_EFFECTS_PER_ACTOR;
    const exclusiveCount = Math.max(0, Math.min(options.exclusiveShardCount ?? Math.min(2, options.shardCount - 1), options.shardCount - 1));
    const exclusive = new Set<number>();
    for (let index = options.shardCount - exclusiveCount; index < options.shardCount; index += 1) exclusive.add(index);
    this.exclusiveIndices = exclusive;
  }
}
