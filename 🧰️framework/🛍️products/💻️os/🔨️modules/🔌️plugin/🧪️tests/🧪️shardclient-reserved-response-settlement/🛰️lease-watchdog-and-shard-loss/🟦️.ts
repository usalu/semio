/** 🛰️ The exclusive lease, watchdog, shard failure, rebuild, crash and metrics tests. */
import { fixtureOriginalWorkStock } from "@semio-tech/framework-actor/retained-turn/bootstrap/testing/fixtures";
import { FakeShardWorker, afterSettle, flushAdmission } from "../🧰️settlement-test-kit/🟦️.ts";
import type { ShardClientSettlementTestKit } from "../🧰️settlement-test-kit/🟦️.ts";

/** 🐕 Registers the exclusive lease, watchdog start and stop, shard failure, terminate and rebuild, worker crash and metrics tests. */
export function registerLeaseWatchdogAndShardLossTests(kit: ShardClientSettlementTestKit): void {
  const { BUDGET, OwnedResidentLedger, ShardClient, describe, expect, harness, it, vi } = kit;
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
  describe("ShardClient.startWatchdog / stopWatchdog", () => {
    it("self-ticks checkHeartbeats + pollHeartbeatSab with no external caller, detects a missed heartbeat, and rebuilds", async () => {
      vi.useFakeTimers();
      try {
        const lost: Array<{ index: number; actorIds: readonly string[] }> = [];
        const workers: FakeShardWorker[] = [];
        const client = new ShardClient({ originalWorkStock: fixtureOriginalWorkStock(),
          residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }),
          shardCount: 1,
          createWorker: (index) => {
            const worker = new FakeShardWorker(index);
            workers.push(worker);
            return worker;
          },
          heartbeatTimeoutMs: 1000,
          firstTurnTimeoutMs: 1000,
          onShardLost: (index, actorIds) => lost.push({ index, actorIds }),
        });

        const activatePromise = client.activate("stuck", "https://x/stuck.js", [], BUDGET);
        await flushAdmission();
        const activateMsg = workers[0]!.sent[0] as { readonly requestId: string };
        workers[0]!.deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
        await activatePromise;

        client.turn("stuck", [], BUDGET).catch(() => {});
        client.startWatchdog(1001);

        vi.advanceTimersByTime(1001);
        expect(lost).toEqual([]);
        vi.advanceTimersByTime(1001);
        expect(lost).toEqual([]);
        vi.advanceTimersByTime(1001);
        expect(workers[0]!.terminated).toBe(true);
        expect(lost).toEqual([{ index: 0, actorIds: ["stuck"] }]);
        expect(client.shardIndexFor("stuck")).toBeUndefined();

        client.stopWatchdog();
        const lostCountAfterStop = lost.length;
        vi.advanceTimersByTime(10_000);
        expect(lost.length).toBe(lostCountAfterStop);
      } finally {
        vi.useRealTimers();
      }
    });

    it("is idempotent to call twice, and stopWatchdog before ever starting is a no-op", () => {
      vi.useFakeTimers();
      try {
        const { client } = harness(1);
        client.startWatchdog(500);
        client.startWatchdog(500);
        client.stopWatchdog();
        client.stopWatchdog();
        expect(true).toBe(true);
      } finally {
        vi.useRealTimers();
      }
    });
  });
  describe("ShardClient failShard clears routing", () => {
    it("clears actorShard + slot.actorIds immediately on a worker crash (onerror), before any terminate()/rebuild()", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("x", "https://x/x.js", [], BUDGET);
      await flushAdmission();
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;
      expect(client.shardIndexFor("x")).toBe(0);

      workers[0]!.onerror?.(new Error("boom"));

      expect(client.shardIndexFor("x")).toBeUndefined();
      await expect(client.turn("x", [], BUDGET)).rejects.toThrow(/not activated/);
    });
  });
  describe("ShardClient terminate/rebuild", () => {
    it("rejects in-flight requests on terminate and spawns a fresh worker on rebuild", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("x", "https://x/x.js", [], BUDGET);
      await flushAdmission();
      const rejection = afterSettle(activatePromise, () => expect(activatePromise).rejects.toThrow(/terminated/));
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
      await flushAdmission();
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
      await flushAdmission();
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activateA;
      const activateB = client.activate("b", "https://x/b.js", [], BUDGET);
      await flushAdmission();
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[1] as { requestId: string }).requestId, ok: true, value: undefined });
      await activateB;

      workers[0]!.deliver({ kind: "heartbeat", turnSeq: 1 });
      void client.turn("a", [], BUDGET);

      setNow(300);
      const [sample] = client.shardMetricsSamples(300);
      expect(sample!.metrics.actors).toBe(2);
      expect(sample!.metrics.busyRatio).toBeCloseTo(0.5);
      expect(sample!.metrics.heartbeatAgeMs).toBe(300);
    });
  });
}
