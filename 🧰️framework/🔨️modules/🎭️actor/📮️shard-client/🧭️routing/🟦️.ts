import type { ShardClientState } from "../🗃️state/🟦️.ts";
import type { ShardClientHub } from "../🧩️wiring/🟦️.ts";
import type { ShardSlot } from "../🔐️activation/🟦️.ts";

/** 🧭️ Pins actors onto shards, leases the exclusive shards and terminates, rebuilds or disposes whole shards. */
export class ShardRouting {
  private readonly state: ShardClientState;
  constructor(private readonly hub: ShardClientHub) { this.state = hub.state; }

  assignShard(actorId: string): ShardSlot {
    const existing = this.state.actorShard.get(actorId);
    if (existing !== undefined) return this.state.shards[existing]!;
    const roundRobinCount = this.state.shards.length - this.state.exclusiveIndices.size;
    let index = this.state.nextRoundRobin % Math.max(roundRobinCount, 1);
    while (this.state.exclusiveIndices.has(index)) index = (index + 1) % this.state.shards.length;
    this.state.nextRoundRobin = (this.state.nextRoundRobin + 1) % Math.max(roundRobinCount, 1);
    this.state.actorShard.set(actorId, index);
    this.state.shards[index]!.actorIds.add(actorId);
    return this.state.shards[index]!;
  }

  /** ▶️ design-runtime.md §1 `ShardTable::request_exclusive` — moves `actorId` onto one of the ≤2
   * exclusive shards for the duration of foreground work. Purely a routing decision: this class does
   * NOT migrate any in-worker instance state (design: migration only happens at a quiescent point via
   * application-level checkpoint) — if the actor was already activated on a different shard, the
   * caller must `checkpoint()` there and `activate()`+`restore()` on the returned exclusive shard. */
  leaseExclusive(actorId: string, options?: { readonly force?: boolean }): number {
    const already = this.state.actorShard.get(actorId);
    if (already !== undefined && this.state.exclusiveIndices.has(already)) return already;
    for (const index of this.state.exclusiveIndices) {
      const slot = this.state.shards[index]!;
      if (slot.actorIds.size === 0 || options?.force) {
        const activation = this.state.actorActivations.get(actorId);
        if (activation) activation.operationsAllowed = false;
        this.hub.effects.abortOutstandingEffects(actorId);
        if (already !== undefined) this.state.shards[already]!.actorIds.delete(actorId);
        slot.actorIds.add(actorId);
        this.state.actorShard.set(actorId, index);
        return index;
      }
    }
    throw new Error(`ShardClient.leaseExclusive(${actorId}): no free exclusive shard (${this.state.exclusiveIndices.size} reserved, all leased)`);
  }

  /** ◀️ Returns `actorId` to the round-robin pool — its NEXT `activate()`/`turn()` targets whichever
   * shard round-robin picks, not necessarily the one it just left. */
  releaseExclusive(actorId: string): void {
    const index = this.state.actorShard.get(actorId);
    if (index === undefined || !this.state.exclusiveIndices.has(index)) return;
    const activation = this.state.actorActivations.get(actorId);
    if (activation) activation.operationsAllowed = false;
    this.hub.effects.abortOutstandingEffects(actorId);
    this.state.shards[index]!.actorIds.delete(actorId);
    this.state.actorShard.delete(actorId);
  }

  shardIndexFor(actorId: string): number | undefined {
    return this.state.actorShard.get(actorId);
  }

  requireShard(actorId: string): ShardSlot {
    const index = this.state.actorShard.get(actorId);
    if (index === undefined) throw new Error(`ShardClient: actor ${actorId} is not activated on any shard`);
    return this.state.shards[index]!;
  }

  /** 🔪️ Kills shard `index`'s worker and rejects every in-flight request on it. Actor routing entries
   * (`actorShard`) are left pointing at this now-dead index deliberately — `rebuild()` respawns a
   * fresh worker at the SAME index, so a caller's already-resolved `shardIndexFor(actorId)` stays
   * valid once the caller re-`activate()`s (from checkpoint) on the rebuilt shard. */
  terminate(index: number, detail?: string): readonly string[] {
    const slot = this.state.shards[index];
    if (!slot) throw new Error(`ShardClient.terminate: no shard ${index}`);
    const actorIds = [...slot.actorIds];
    this.hub.transport.failShard(slot, new Error(detail ?? `shard ${index} terminated`));
    slot.worker.terminate();
    return actorIds;
  }

  /** 🌱️ Respawns shard `index` with a fresh worker. Every actor formerly on it needs a fresh
   * `activate()` + `restore()` from its last checkpoint — this class does not do that itself (it has
   * no checkpoint bytes to restore from; that's the kernel-side `ActivationRegistry`'s job), so it
   * clears the routing entries for whoever was there rather than silently leaving them dangling. */
  rebuild(index: number): void {
    const old = this.state.shards[index];
    if (!old) throw new Error(`ShardClient.rebuild: no shard ${index}`);
    for (const actorId of old.actorIds) this.state.actorShard.delete(actorId);
    this.state.shards[index] = this.hub.transport.spawnShard(index);
  }

  /** ⏏️ Tears down every shard's worker — call on full app shutdown, never per-actor (use {@link dispose}). */
  disposeAll(): void {
    this.hub.watchdog.stopWatchdog();
    for (const slot of this.state.shards) {
      this.hub.transport.failShard(slot, new Error("ShardClient disposed"));
      slot.worker.terminate();
    }
  }
}
