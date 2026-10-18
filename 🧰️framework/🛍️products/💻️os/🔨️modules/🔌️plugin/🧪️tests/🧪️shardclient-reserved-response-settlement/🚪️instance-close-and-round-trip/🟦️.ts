/** 🚪️ The exact instance close transport and activation turn round trip tests. */
import { afterSettle, flushAdmission } from "../🧰️settlement-test-kit/🟦️.ts";
import type { ActorInstanceLifecycleReceipt, ShardClientSettlementTestKit, ShardEventEnvelope } from "../🧰️settlement-test-kit/🟦️.ts";

/** 🔚 Registers the exact instance close transport and the activation plus turn round trip tests. */
export function registerInstanceCloseAndRoundTripTests(kit: ShardClientSettlementTestKit): void {
  const { BUDGET, answerLifecycle, captureFixtureInstance, describe, encodeActorInstanceLifecycle, expect, fixtureRetirement, harness, it, retireFixtureInstance, testSource } = kit;
  describe("ShardClient exact instance close transport", () => {
    it("retains and retries the same close authority after transport refusal", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../🚪️lifetime/🧪️tests/🧫️fixtures/🔣️.json", testSource.url), "utf8"));
      for (const failure of fixture.leaseFailures) {
        const { client, workers } = harness(1);
        const worker = workers[0]!;
        const activation = client.activate("retry", "https://x/retry.js", [], BUDGET);
        await flushAdmission();
        worker.deliver({ kind: "result", requestId: (worker.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
        await activation;
        const lease = await captureFixtureInstance(client, worker, "retry");
        const request = lease.beginClose();
        const send = worker.postMessage.bind(worker);
        let original: unknown;
        worker.postMessage = (message) => {
          original = message;
          if (failure === "postMessage-throw") throw new Error("fixture transport refusal");
          send(message);
        };
        const pending = lease.close(BUDGET);
        const observed = afterSettle(pending, () => expect(pending).rejects.toThrow("fixture"));
        if (failure === "worker-error") worker.deliver({ kind: "result", requestId: (original as { requestId: string }).requestId, ok: false, error: "fixture worker refusal" });
        await observed;
        expect(lease.progress()).toEqual({ kind: "blocked", failure: failure === "postMessage-throw" ? "transport-refused" : "worker-refused" });
        expect(() => client.dispose("retry")).toThrow("actor-close.native-retirement-pending");
        worker.postMessage = send;
        const retried = lease.close(BUDGET);
        const message = worker.sent.at(-1) as { events: ShardEventEnvelope[] };
        expect(message.events[0]!.payload).toBe(request);
        expect(message.events).toEqual((original as { events: ShardEventEnvelope[] }).events);
        await answerLifecycle(worker, retried);
        await retireFixtureInstance(worker, lease);
        expect(lease.progress()).toEqual({ kind: "complete", failure: null });
        client.dispose("retry");
      }
    });

    it("waits for the captured worker's exact accepted and retired receipts", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../🚪️lifetime/🧪️tests/🧫️fixtures/🔣️.json", testSource.url), "utf8"));
      for (const row of fixture.leaseReceipts) {
        const { client, workers } = harness(2);
        const worker = workers[0]!;
        const activation = client.activate("close-owner", "https://x/close.js", [], BUDGET);
        await flushAdmission();
        worker.deliver({ kind: "result", requestId: (worker.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
        await activation;
        const lease = await captureFixtureInstance(client, worker, "close-owner");
        const request = lease.beginClose();
        await answerLifecycle(worker, lease.close(BUDGET));
        const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: lease.lifetime!, requestSequence: request.requestSequence, closeGeneration: 17n };
        for (const event of row.events) {
          const receipt: ActorInstanceLifecycleReceipt = { ...accepted, kind: event.endsWith("retired") ? "retired" : "accepted", ...(event === "old-activation-accepted" ? { lifetime: { ...lease.lifetime!, activationGeneration: lease.lifetime!.activationGeneration + 1n } } : {}), ...(event === "wrong-request-accepted" ? { requestSequence: request.requestSequence + 1 } : {}), ...(event === "wrong-generation-retired" ? { closeGeneration: 16n } : {}) };
          const pending = lease.poll(BUDGET);
          if (event === "foreign-worker-accepted") {
            const requestId = (worker.sent.at(-1) as { requestId: string }).requestId;
            workers[1]!.deliver({ kind: "result", requestId, ok: true, value: { status: { tag: "idle" }, lifecycleReceipt: encodeActorInstanceLifecycle(receipt) } });
            await answerLifecycle(worker, pending);
          } else {
            const delivered = answerLifecycle(worker, pending, receipt);
            if (event.startsWith("wrong-") || event === "old-activation-accepted" || receipt.kind === "retired" && lease.progress().kind !== "accepted") await expect(delivered).rejects.toThrow("actor-lifecycle.receipt-mismatch");
            else await delivered;
          }
          if (lease.pendingReceipt?.kind === "accepted") await answerLifecycle(worker, lease.acknowledge(lease.pendingReceipt, BUDGET));
        }
        expect(lease.pendingReceipt?.kind === "retired", row.name).toBe(row.settled);
        expect(client.shardIndexFor("close-owner")).toBe(0);
        if (lease.pendingReceipt?.kind !== "retired") {
          await answerLifecycle(worker, lease.poll(BUDGET), accepted);
          await answerLifecycle(worker, lease.acknowledge(accepted, BUDGET), { ...accepted, kind: "retired" });
        }
        await answerLifecycle(worker, lease.acknowledge(lease.pendingReceipt!, BUDGET, fixtureRetirement(lease)));
        expect(lease.progress().kind).toBe("complete");
        client.dispose("close-owner");
      }
    });

    it("does not revive a completed owner when the numeric ID or actor activation is reused", async () => {
      const { client, workers } = harness(1);
      const worker = workers[0]!;
      const activate = async () => {
        const pending = client.activate("reused", "https://x/close.js", [], BUDGET);
        await flushAdmission();
        worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: undefined });
        await pending;
      };
      await activate();
      const old = await captureFixtureInstance(client, worker, "reused");
      await retireFixtureInstance(worker, old);
      const sameActivation = await captureFixtureInstance(client, worker, "reused", 7, 14n);
      expect(sameActivation.lifetime!.activationGeneration).toBe(old.lifetime!.activationGeneration);
      expect(sameActivation.lifetime!.guestLifetime).not.toBe(old.lifetime!.guestLifetime);
      expect(() => old.activation.assertActive()).toThrow("actor-activation.revoked");
      const before = worker.sent.length;
      await expect(old.poll(BUDGET)).rejects.toThrow("actor-lifecycle.already-complete");
      expect(worker.sent).toHaveLength(before);
      await retireFixtureInstance(worker, sameActivation);
      client.dispose("reused");
      await activate();
      const fresh = await captureFixtureInstance(client, worker, "reused");
      expect(fresh.lifetime!.activationGeneration).toBe(old.lifetime!.activationGeneration + 1n);
      await retireFixtureInstance(worker, fresh);
      client.dispose("reused");
    });
  });
  describe("ShardClient activation + turn round-trip", () => {
    it("routes activate then turn to the same shard, resolving the reply by requestId", async () => {
      const { client, workers } = harness(2);
      const activatePromise = client.activate("actor-1", "https://x/plugin.js", [], BUDGET);
      await flushAdmission();
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
}
