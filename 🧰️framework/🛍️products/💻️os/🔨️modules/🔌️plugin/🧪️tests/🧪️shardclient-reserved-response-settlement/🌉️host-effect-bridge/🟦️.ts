/** 🌉️ The host effect bridge tests. */
import { findEffectReply, flushMicrotasks, makeEffectRequestFrame } from "../🧰️settlement-test-kit/🟦️.ts";
import type { ShardClientSettlementTestKit } from "../🧰️settlement-test-kit/🟦️.ts";

/** 🌁 Registers the handler success, handler error, missing handler, backpressure cap and shard loss settlement tests of the host effect bridge. */
export function registerHostEffectBridgeTests(kit: ShardClientSettlementTestKit): void {
  const { activateActor, describe, expect, harness, it } = kit;
  describe("ShardClient host-effect bridge — handler success", () => {
    it("resolves an effect-request through onHostEffect and posts an effect-complete frame back to the worker", async () => {
      const { client, workers } = harness(1, { onHostEffect: async (actorId, effect, params) => ({ actorId, effect, params, from: "handler" }) });
      await activateActor(client, workers, "a");

      workers[0]!.deliver(makeEffectRequestFrame("a", "http-fetch", "a:http-fetch:1", { url: "https://example.test" }));
      await flushMicrotasks();

      const reply = findEffectReply(workers[0]!.sent, "a:http-fetch:1", "effect-complete");
      expect(reply).toBeDefined();
      expect(reply?.frame.envelope.payload.payload.value).toEqual({ actorId: "a", effect: "http-fetch", params: { url: "https://example.test" }, from: "handler" });
    });
  });
  describe("ShardClient host-effect bridge — handler error", () => {
    it("a rejected onHostEffect settles as effect-error, never a hang", async () => {
      const { client, workers } = harness(1, { onHostEffect: async () => { throw new Error("boom"); } });
      await activateActor(client, workers, "a");

      workers[0]!.deliver(makeEffectRequestFrame("a", "blob-read", "a:blob-read:1", { hash: "x" }));
      await flushMicrotasks();

      const reply = findEffectReply(workers[0]!.sent, "a:blob-read:1", "effect-error");
      expect(reply?.frame.envelope.payload.payload.message).toBe("boom");
    });
  });
  describe("ShardClient host-effect bridge — no handler installed", () => {
    it("fails FAST with an explicit effect-error, synchronously, never a silent hang", async () => {
      const { client, workers } = harness(1);
      await activateActor(client, workers, "a");

      workers[0]!.deliver(makeEffectRequestFrame("a", "storage-read", "a:storage-read:1", {}));
      const reply = findEffectReply(workers[0]!.sent, "a:storage-read:1", "effect-error");
      expect(reply?.frame.envelope.payload.payload.message).toBe("no host effect handler installed");
    });
  });
  describe("ShardClient host-effect bridge — backpressure cap", () => {
    it("rejects an effect-request beyond maxOutstandingEffectsPerActor with a quota-shaped effect-error, while the earlier one stays pending", async () => {
      const { client, workers } = harness(1, { maxOutstandingEffectsPerActor: 1, onHostEffect: () => new Promise(() => {}) });
      await activateActor(client, workers, "a");

      workers[0]!.deliver(makeEffectRequestFrame("a", "spawn-job", "a:spawn-job:1", {}));
      workers[0]!.deliver(makeEffectRequestFrame("a", "spawn-job", "a:spawn-job:2", {}));

      expect(findEffectReply(workers[0]!.sent, "a:spawn-job:1", "effect-error")).toBeUndefined();
      expect(findEffectReply(workers[0]!.sent, "a:spawn-job:1", "effect-complete")).toBeUndefined();
      const reply = findEffectReply(workers[0]!.sent, "a:spawn-job:2", "effect-error");
      expect(reply?.frame.envelope.payload.payload.message).toMatch(/outstandingRequests.*limit=1.*actual=1/);
    });
  });
  describe("ShardClient host-effect bridge — shard-loss settlement", () => {
    it("terminate() aborts every outstanding effect for its actors, and a late handler resolution posts no reply to the dead worker", async () => {
      let capturedSignal: AbortSignal | undefined;
      const { client, workers } = harness(1, {
        onHostEffect: (_actorId, _effect, _params, signal) =>
          new Promise((resolve) => {
            capturedSignal = signal;
            signal.addEventListener("abort", () => resolve("too-late"));
          }),
      });
      await activateActor(client, workers, "a");

      workers[0]!.deliver(makeEffectRequestFrame("a", "http-fetch", "a:http-fetch:1", {}));
      expect(capturedSignal?.aborted).toBe(false);

      client.terminate(0);
      expect(capturedSignal?.aborted).toBe(true);

      const sentBeforeLateResolve = workers[0]!.sent.length;
      await flushMicrotasks();
      expect(workers[0]!.sent.length).toBe(sentBeforeLateResolve);
      expect(findEffectReply(workers[0]!.sent, "a:http-fetch:1", "effect-complete")).toBeUndefined();
      expect(findEffectReply(workers[0]!.sent, "a:http-fetch:1", "effect-error")).toBeUndefined();
    });

    it("dispose(actorId) aborts that actor's outstanding effects without touching a sibling actor's", async () => {
      const signals: Record<string, AbortSignal> = {};
      const { client, workers } = harness(1, {
        onHostEffect: (actorId, _effect, _params, signal) => {
          signals[actorId] = signal;
          return new Promise(() => {});
        },
      });
      await activateActor(client, workers, "a");
      await activateActor(client, workers, "b");

      workers[0]!.deliver(makeEffectRequestFrame("a", "http-fetch", "a:http-fetch:1", {}));
      workers[0]!.deliver(makeEffectRequestFrame("b", "http-fetch", "b:http-fetch:1", {}, client.captureActorActivation("b").activationGeneration));

      client.dispose("a");
      expect(signals.a?.aborted).toBe(true);
      expect(signals.b?.aborted).toBe(false);
    });
  });
}
