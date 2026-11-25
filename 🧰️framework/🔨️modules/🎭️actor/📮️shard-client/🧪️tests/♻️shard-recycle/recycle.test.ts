/** ♻️ LAW: a shard whose last actor is gone gives its wasm memory back by being replaced with a fresh worker; a shard with a live actor, a pending request or a pending original closure is never touched, and the replaced slot is the only route to a worker afterwards. */
import { describe, expect, it } from "bun:test";
import { ShardRouting } from "../../🧭️routing/🟦️.ts";

type FakeWorker = { terminated: boolean; terminate(): void };
const worker = (): FakeWorker => ({ terminated: false, terminate() { this.terminated = true; } });
const slot = (index: number, w: FakeWorker) => ({ index, worker: w, available: true, pendingRequestIds: new Set<string>(), actorIds: new Set<string>() });

function rig() {
  const state: any = { shards: [] as any[], originalClosures: new Map(), actorActivations: new Map(), actorShard: new Map(), exclusiveIndices: new Set(), nextRoundRobin: 0 };
  const failed: string[] = [];
  const hub: any = { state, transport: { failShard: (s: any, e: Error) => { failed.push(e.message); s.available = false; }, spawnShard: (i: number) => slot(i, worker()) } };
  state.shards.push(slot(0, worker()), slot(1, worker()));
  return { routing: new ShardRouting(hub), state, failed };
}

describe("shard recycling", () => {
  it("replaces an idle shard with a fresh worker and terminates the old one", () => {
    const { routing, state, failed } = rig();
    const old = state.shards[0];
    expect(routing.recycleIdle(old)).toBe(true);
    expect(old.worker.terminated).toBe(true);
    expect(failed).toEqual(["shard 0 recycled"]);
    expect(state.shards[0]).not.toBe(old);
    expect(state.shards[0].available).toBe(true);
    expect(state.shards[0].worker).not.toBe(old.worker);
  });
  it("never recycles a shard that is still in use", () => {
    const { routing, state } = rig();
    const busy = state.shards[0];
    busy.actorIds.add("a#1");
    expect(routing.recycleIdle(busy)).toBe(false);
    busy.actorIds.clear();
    busy.pendingRequestIds.add("r1");
    expect(routing.recycleIdle(busy)).toBe(false);
    busy.pendingRequestIds.clear();
    state.originalClosures.set(1n, { slot: busy });
    expect(routing.recycleIdle(busy)).toBe(false);
    state.originalClosures.clear();
    state.actorActivations.set("a#2", { slot: busy });
    expect(routing.recycleIdle(busy)).toBe(false);
    expect(busy.worker.terminated).toBe(false);
  });
  it("ignores the closure that just reported but not another pending one", () => {
    const { routing, state } = rig();
    const idle = state.shards[0];
    state.originalClosures.set(7n, { slot: idle });
    state.originalClosures.set(8n, { slot: idle });
    expect(routing.recycleIdle(idle, 7n)).toBe(false);
    state.originalClosures.delete(8n);
    expect(routing.recycleIdle(idle, 7n)).toBe(true);
  });
  it("ignores a stale slot", () => {
    const { routing, state } = rig();
    const stale = state.shards[1];
    expect(routing.recycleIdle(stale)).toBe(true);
    expect(routing.recycleIdle(stale)).toBe(false);
  });
});
