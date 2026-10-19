/** 🎟️ The activation lease tests. */
import { loadActorActivationFixture } from "@semio-tech/framework-actor/testing/fixtures";
import { FakeShardWorker, afterSettle, flushAdmission } from "../🧰️settlement-test-kit/🟦️.ts";
import type { OutboundMessage, ShardClient, ShardClientSettlementTestKit } from "../🧰️settlement-test-kit/🟦️.ts";

/** 🎫 Registers the activation lease tests of the shard client. */
export function registerActivationLeaseTests(kit: ShardClientSettlementTestKit): void {
  const { BUDGET, captureFixtureInstance, describe, expect, harness, it, retireFixtureInstance, testSource } = kit;
  describe("ShardClient activation lease", () => {
    async function fixture() {
      const { readFileSync } = await import("node:fs");
      return JSON.parse(readFileSync(new URL("../🪪️activation/🧪️tests/🧫️fixtures/🔣️.json", testSource.url), "utf8")) as { actorId: string; instanceId: number; revocations: Array<{ action: string; expected: { activeBefore: boolean; activeAfter: boolean; newTurns: number } }> };
    }

    async function activate(client: ShardClient, worker: FakeShardWorker, actorId: string) {
      const pending = client.activate(actorId, "https://fixture.invalid/actor.js", [], BUDGET);
      await flushAdmission();
      const message = worker.sent.at(-1) as { requestId: string };
      worker.deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
      await pending;
      return client.captureActorActivation(actorId);
    }

    it("validates the neutral cases and captures only a ready immutable activation", async () => {
      const { default: Ajv } = await import("ajv");
      const { readFileSync } = await import("node:fs");
      const row = await fixture();
      
      
      const { client, workers } = harness(1);
      expect(() => client.captureActorActivation(row.actorId)).toThrow("actor-activation.not-ready");
      const pending = client.activate(row.actorId, "https://fixture.invalid/actor.js", [], BUDGET);
      await flushAdmission();
      expect(() => client.captureActorActivation(row.actorId)).toThrow("actor-activation.not-ready");
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await pending;
      const lease = client.captureActorActivation(row.actorId);
      expect(Object.isFrozen(lease)).toBe(true);
      expect(lease.actorId).toBe(row.actorId);
      expect(lease.activationGeneration).toBe(1n);
      expect(() => lease.assertActive()).not.toThrow();
      expect(workers[0]!.sent).toHaveLength(1);
      client.dispose(row.actorId);
    });

    it("revokes each close loss or disposal without dispatching another operation", async () => {
      const { default: Ajv } = await import("ajv");
      const row = await fixture();
      const oracle = new Ajv();
      for (const test of row.revocations) {
        const { client, workers } = harness(1);
        const worker = workers[0]!;
        const lease = await activate(client, worker, row.actorId);
        lease.assertActive();
        const close = test.action.startsWith("close") ? await captureFixtureInstance(client, worker, row.actorId, row.instanceId) : null;
        const post = worker.postMessage.bind(worker);
        if (test.action === "close-refused") worker.postMessage = () => { throw new Error("fixture transport refused"); };
        if (close) {
          close.beginClose();
          if (test.action === "close-refused") await expect(close.close(BUDGET)).rejects.toThrow("fixture transport refused");
        }
        else if (test.action === "dispose") client.dispose(row.actorId);
        else if (test.action === "worker-error") worker.onerror?.(new Error("fixture worker lost"));
        else if (test.action === "terminate") client.terminate(0);
        else client.disposeAll();
        const before = worker.sent.length;
        let activeAfter = true;
        try { lease.assertActive(); } catch { activeAfter = false; }
        await expect(lease.turn([], BUDGET)).rejects.toThrow("actor-activation.revoked");
        const actual = { activeBefore: true, activeAfter, newTurns: worker.sent.length - before };
        expect(actual, test.action).toEqual(test.expected);
        
        if (close) {
          expect(close.progress().kind).toBe(test.action === "close" ? "closing" : "blocked");
          worker.postMessage = post;
          await retireFixtureInstance(worker, close);
          client.dispose(row.actorId);
        }
      }
    });

    it("never refreshes an old lease when the same actor name is activated again", async () => {
      const { actorId } = await fixture();
      const { client, workers } = harness(1);
      const old = await activate(client, workers[0]!, actorId);
      client.dispose(actorId);
      const fresh = await activate(client, workers[0]!, actorId);
      expect(fresh.activationGeneration).toBe(old.activationGeneration + 1n);
      expect(() => old.assertActive()).toThrow("actor-activation.revoked");
      expect(() => fresh.assertActive()).not.toThrow();
      client.dispose(actorId);
    });

    it("never refreshes an old lease after replacement of its worker at the same slot", async () => {
      const { actorId } = await fixture();
      const { client, workers } = harness(1);
      const old = await activate(client, workers[0]!, actorId);
      client.terminate(0);
      client.rebuild(0);
      const fresh = await activate(client, workers[1]!, actorId);
      expect(workers[1]!.index).toBe(workers[0]!.index);
      expect(() => old.assertActive()).toThrow("actor-activation.revoked");
      expect(() => fresh.assertActive()).not.toThrow();
      client.dispose(actorId);
    });

    it("permanently revokes a released route even when the same worker route is reacquired", async () => {
      const { actorId, instanceId } = await fixture();
      const { client, workers } = harness(2, { exclusiveShardCount: 1 });
      client.leaseExclusive(actorId);
      const operation = await activate(client, workers[1]!, actorId);
      const close = await captureFixtureInstance(client, workers[1]!, actorId, instanceId);
      client.releaseExclusive(actorId);
      expect(client.leaseExclusive(actorId)).toBe(1);
      expect(() => operation.assertActive()).toThrow("actor-activation.revoked");
      expect(() => client.captureActorActivation(actorId)).toThrow("actor-activation.revoked");
      close.beginClose();
      expect(close.progress().kind).toBe("closing");
      await retireFixtureInstance(workers[1]!, close);
      client.dispose(actorId);
    });

    it("keeps the old close root after moving operations onto an exclusive route", async () => {
      const { actorId, instanceId } = await fixture();
      const { client, workers } = harness(2, { exclusiveShardCount: 1 });
      const operation = await activate(client, workers[0]!, actorId);
      const close = await captureFixtureInstance(client, workers[0]!, actorId, instanceId);
      client.leaseExclusive(actorId);
      expect(() => operation.assertActive()).toThrow("actor-activation.revoked");
      close.beginClose();
      expect(workers[1]!.sent).toHaveLength(0);
      await retireFixtureInstance(workers[0]!, close);
      expect(close.progress().kind).toBe("complete");
      client.disposeAll();
    });

    it("disposes the captured worker after a moved or released route and preserves refusal for retry", async () => {
      const rows = await loadActorActivationFixture();
      
      const { default: Ajv } = await import("ajv");
      const oracle = new Ajv({ strict: true });
      
      for (const row of rows.disposals) {
        const { client, workers } = harness(2, { exclusiveShardCount: 1 });
        const original = workers[row.route === "released" ? 1 : 0]!;
        const other = workers[row.route === "released" ? 0 : 1]!;
        if (row.route === "released") client.leaseExclusive(rows.actorId);
        const operation = await activate(client, original, rows.actorId);
        const owner = await captureFixtureInstance(client, original, rows.actorId, rows.instanceId);
        if (row.route === "released") client.releaseExclusive(rows.actorId); else client.leaseExclusive(rows.actorId);
        expect(() => operation.assertActive()).toThrow("actor-activation.revoked");
        await retireFixtureInstance(original, owner);
        const route = client.shardIndexFor(rows.actorId);
        const post = original.postMessage.bind(original);
        if (row.refuseFirst) {
          original.postMessage = () => { throw new Error("fixture dispose refused"); };
          expect(() => client.dispose(rows.actorId)).toThrow("fixture dispose refused");
          expect(client.shardIndexFor(rows.actorId)).toBe(route);
          expect(owner.progress().kind).toBe("complete");
          await expect(client.activate(rows.actorId, "https://fixture.invalid/replacement.js", [], BUDGET)).rejects.toThrow("actor-close.activation-already-owned");
          original.postMessage = post;
        }
        client.dispose(rows.actorId);
        const isDisposal = (value: unknown): value is Extract<OutboundMessage, { kind: "dispose" }> => value !== null && typeof value === "object" && Reflect.get(value, "kind") === "dispose";
        const messages = original.sent.filter(isDisposal);
        expect(messages.length, row.route).toBe(row.expected.originalPosts);
        expect(other.sent.filter(isDisposal).length, row.route).toBe(row.expected.otherPosts);
        const routeReleased = client.shardIndexFor(rows.actorId) === undefined;
        const fresh = await activate(client, workers[0]!, rows.actorId);
        const before = workers.reduce((count, worker) => count + worker.sent.length, 0);
        owner.dispose();
        let replacementActive = true;
        try { fresh.assertActive(); } catch { replacementActive = false; }
        const actual = { originalPosts: messages.length, otherPosts: other.sent.filter(isDisposal).length, sameGeneration: messages[0]?.activationGeneration === operation.activationGeneration, routeReleased, oldLeasePosts: workers.reduce((count, worker) => count + worker.sent.length, 0) - before, replacementActive };
        expect(actual, JSON.stringify(row)).toEqual(row.expected);
        
        client.dispose(rows.actorId);
      }
    });

    it("refuses captured disposal before exact close and never addresses a replacement worker", async () => {
      const { client, workers } = harness(1);
      await activate(client, workers[0]!, "captured-dispose");
      const owner = await captureFixtureInstance(client, workers[0]!, "captured-dispose");
      expect(() => owner.dispose()).toThrow("actor-close.native-retirement-pending");
      await retireFixtureInstance(workers[0]!, owner);
      client.terminate(0);
      client.rebuild(0);
      const fresh = await activate(client, workers[1]!, "captured-dispose");
      const before = workers[1]!.sent.length;
      expect(() => owner.dispose()).toThrow("actor-close.worker-lost");
      expect(workers[1]!.sent.length).toBe(before);
      expect(() => fresh.assertActive()).not.toThrow();
      client.dispose("captured-dispose");
    });

    it("refuses a reply from a foreign or replaced worker incarnation", async () => {
      const { actorId } = await fixture();
      const { client, workers } = harness(2, { exclusiveShardCount: 1 });
      await activate(client, workers[0]!, actorId);
      client.terminate(0);
      client.rebuild(0);
      const lease = await activate(client, workers[2]!, actorId);
      let settled = false;
      const turning = lease.turn([], BUDGET).then((result) => { settled = true; return result; });
      const message = workers[2]!.sent.at(-1) as { requestId: string };
      for (const foreign of [workers[0]!, workers[1]!]) foreign.deliver({ kind: "result", requestId: message.requestId, ok: true, value: "foreign" });
      await Promise.resolve();
      await Promise.resolve();
      expect(settled).toBe(false);
      workers[2]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: "owned" });
      await expect(turning).resolves.toBe("owned");
      client.dispose(actorId);
    });

    it("rejects a settled result after close begins without acknowledging or releasing the close root", async () => {
      const { actorId, instanceId } = await fixture();
      const { client, workers } = harness(1);
      const worker = workers[0]!;
      const lease = await activate(client, worker, actorId);
      const close = await captureFixtureInstance(client, worker, actorId, instanceId);
      const turning = lease.turn([{ kind: "completed", payload: { req: 41n } }], BUDGET);
      const observed = afterSettle(turning, () => expect(turning).rejects.toThrow("actor-activation.revoked"));
      const turn = worker.sent.at(-1) as { requestId: string };
      const before = worker.sent.length;
      close.beginClose();
      worker.deliver({ kind: "result", requestId: turn.requestId, ok: true, value: { uiPatches: ["retained"] } });
      await observed;
      expect(worker.sent).toHaveLength(before);
      expect(close.progress().kind).toBe("closing");
      expect(() => client.dispose(actorId)).toThrow("actor-close.native-retirement-pending");
      await retireFixtureInstance(worker, close);
      client.dispose(actorId);
    });
  });
}
