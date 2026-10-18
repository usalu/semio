/** 🛂️ The inbound activation authority tests. */
import { findEffectReply, flushMicrotasks, makeEffectRequestFrame } from "../🧰️settlement-test-kit/🟦️.ts";
import type { ShardClientSettlementTestKit, ShardInstanceLifecycleLease } from "../🧰️settlement-test-kit/🟦️.ts";

/** 🪧 Registers the inbound activation authority tests of the shard client. */
export function registerInboundActivationAuthorityTests(kit: ShardClientSettlementTestKit): void {
  const { BUDGET, activateActor, answerLifecycle, captureFixtureInstance, describe, expect, harness, it, retireFixtureInstance, testSource } = kit;
  describe("ShardClient inbound activation authority", () => {
    it("matches the neutral stale-source matrix without consuming exact close receipts", async () => {
      const { default: Ajv } = await import("ajv");
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../🪪️activation/📨️inbound/🧪️tests/🧫️fixtures/🔣️.json", testSource.url), "utf8")) as { actorId: string; requestId: string; cases: Array<{ name: string; effects: number; traps: number }> };
      
      const oracle = new Ajv();
      
      for (const row of fixture.cases) {
        const effects: string[] = [];
        const traps: string[] = [];
        const { client, workers } = harness(1, { onHostEffect: async (_actor, effect) => { effects.push(effect); return "ok"; }, onActorTrap: (_actor, message) => traps.push(message) });
        await activateActor(client, workers, fixture.actorId);
        const source = workers[0]!;
        const generation = client.captureActorActivation(fixture.actorId).activationGeneration;
        let close: ShardInstanceLifecycleLease | undefined;
        if (row.name === "replaced-worker") {
          client.terminate(0);
          client.rebuild(0);
          await activateActor(client, workers, fixture.actorId, 1);
        } else if (row.name === "reactivated") {
          client.dispose(fixture.actorId);
          await activateActor(client, workers, fixture.actorId);
        } else if (row.name === "closing") {
          close = await captureFixtureInstance(client, source, fixture.actorId);
          close.beginClose();
        }
        const frame = makeEffectRequestFrame(fixture.actorId, "storage-read", fixture.requestId, {});
        const message: Record<string, unknown> = { ...frame, activationGeneration: row.name === "foreign-generation" ? generation + 1n : generation };
        const trap: Record<string, unknown> = { kind: "trap", actorId: fixture.actorId, activationGeneration: message.activationGeneration, message: "fixture-trap" };
        if (row.name === "missing-generation") { delete message.activationGeneration; delete trap.activationGeneration; }
        if (row.name === "foreign-envelope" && frame.kind === "frame" && frame.frame.kind === "Envelope") {
          message.frame = { ...frame.frame, envelope: { ...frame.frame.envelope, from: { kind: "actor", id: "another-actor" } } };
        }
        source.onmessage?.({ data: message });
        source.onmessage?.({ data: trap });
        await flushMicrotasks();
        const actual = { name: row.name, effects: effects.length, traps: traps.length };
        expect(actual, row.name).toEqual(row);
        expect(oracle.validate({ const: row }, actual), row.name).toBe(true);
        if (close) {
          await retireFixtureInstance(source, close);
          expect(close.progress().kind).toBe("complete");
        }
        client.dispose(fixture.actorId);
      }
    });

    it("keeps a replacement effect with a reused request id when the old handler settles", async () => {
      const settlements: Array<(value: unknown) => void> = [];
      const signals: AbortSignal[] = [];
      const { client, workers } = harness(1, { onHostEffect: (_actor, _effect, _params, signal) => new Promise((resolve) => { signals.push(signal); settlements.push(resolve); }) });
      await activateActor(client, workers, "a");
      const oldGeneration = client.captureActorActivation("a").activationGeneration;
      workers[0]!.onmessage?.({ data: { ...makeEffectRequestFrame("a", "storage-read", "shared", {}), activationGeneration: oldGeneration } });
      client.dispose("a");
      expect(signals[0]!.aborted).toBe(true);
      await activateActor(client, workers, "a");
      const generation = client.captureActorActivation("a").activationGeneration;
      workers[0]!.onmessage?.({ data: { ...makeEffectRequestFrame("a", "storage-read", "shared", {}), activationGeneration: generation } });
      settlements[0]!("old");
      await flushMicrotasks();
      expect(findEffectReply(workers[0]!.sent, "shared", "effect-complete")).toBeUndefined();
      expect(signals[1]!.aborted).toBe(false);
      settlements[1]!("new");
      await flushMicrotasks();
      const reply = findEffectReply(workers[0]!.sent, "shared", "effect-complete");
      expect(reply?.frame.envelope.payload.payload.value).toBe("new");
      expect((reply as unknown as { activationGeneration: bigint }).activationGeneration).toBe(generation);
      client.dispose("a");
    });

    it("aborts the admitted host effect on close while retaining the native close owner", async () => {
      let signal: AbortSignal | undefined;
      let settle!: (value: unknown) => void;
      const { client, workers } = harness(1, { onHostEffect: (_actor, _effect, _params, captured) => new Promise((resolve) => { signal = captured; settle = resolve; }) });
      await activateActor(client, workers, "a");
      workers[0]!.onmessage?.({ data: { ...makeEffectRequestFrame("a", "storage-read", "closing", {}), activationGeneration: client.captureActorActivation("a").activationGeneration } });
      const lease = await captureFixtureInstance(client, workers[0]!, "a");
      void lease.beginClose();
      expect(signal?.aborted).toBe(false);
      await answerLifecycle(workers[0]!, lease.poll(BUDGET));
      expect(signal?.aborted).toBe(true);
      settle("late");
      await flushMicrotasks();
      expect(findEffectReply(workers[0]!.sent, "closing", "effect-complete")).toBeUndefined();
      expect(lease.progress().kind).toBe("closing");
      expect(() => client.dispose("a")).toThrow("actor-close.native-retirement-pending");
    });

    it("ignores late old-worker errors and worker traps after slot replacement", async () => {
      const traps: string[] = [];
      const { client, workers } = harness(1, { onActorTrap: (_actor, message) => traps.push(message) });
      await activateActor(client, workers, "a");
      const staleError = workers[0]!.onerror!;
      client.terminate(0);
      client.rebuild(0);
      await activateActor(client, workers, "a", 1);
      const lease = client.captureActorActivation("a");
      staleError(new Error("old-worker"));
      workers[0]!.onmessage?.({ data: { kind: "trap", actorId: "*", activationGeneration: null, message: "old-trap" } });
      expect(() => lease.assertActive()).not.toThrow();
      expect(traps).toEqual([]);
      workers[1]!.onmessage?.({ data: { kind: "trap", actorId: "*", activationGeneration: null, message: "current-worker-trap" } });
      expect(traps).toEqual(["current-worker-trap"]);
      client.dispose("a");
    });
  });
}
