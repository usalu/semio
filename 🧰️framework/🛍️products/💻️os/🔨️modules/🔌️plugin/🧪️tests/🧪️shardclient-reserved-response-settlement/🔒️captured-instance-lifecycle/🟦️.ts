/** 🔒️ The captured instance lifecycle tests. */
import { FakeShardWorker, afterSettle, flushAdmission, makeEffectRequestFrame } from "../🧰️settlement-test-kit/🟦️.ts";
import type { ActorInstanceLifecycleReceipt, OwnedUiPatchAcknowledgement, OwnedUiPatchInputRetirement, ShardClient, ShardClientSettlementTestKit, ShardEventEnvelope, ShardInstanceLifecycleLease } from "../🧰️settlement-test-kit/🟦️.ts";

/** 🔐 Registers the captured instance lifecycle tests of the shard client. */
export function registerCapturedInstanceLifecycleTests(kit: ShardClientSettlementTestKit): void {
  const { BUDGET, OwnedNativeUiPatchAuthority, OwnedNativeUiPatchSubmissionReceipt, OwnedUiInstance, OwnedUiInstanceRetirement, describe, encodeActorInstanceLifecycle, encodeActorUiPatchReceipt, expect, harness, it, testSource } = kit;
  describe("ShardClient captured instance lifecycle", () => {
    const openInput = { appId: "fixture", actor: "fixture", config: new Uint8Array(), assets: [], capabilities: [], quotas: new Uint8Array() };
    async function activateCaptured(client: ShardClient, worker: FakeShardWorker, actorId = "captured") {
      const activated = client.activate(actorId, "https://fixture.invalid/actor.js", [], BUDGET);
      await flushAdmission();
      worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: undefined });
      await activated;
      return client.captureInstanceLifecycle(actorId, 7);
    }
    async function answer(worker: FakeShardWorker, pending: Promise<unknown>, receipt?: ActorInstanceLifecycleReceipt) {
      const message = worker.sent.at(-1) as { requestId: string };
      worker.deliver({ kind: "result", requestId: message.requestId, ok: true, value: { status: { tag: "idle" }, lifecycleReceipt: receipt ? encodeActorInstanceLifecycle(receipt) : undefined } });
      return pending;
    }
    async function openCaptured(worker: FakeShardWorker, lease: ShardInstanceLifecycleLease, guestLifetime = 13n) {
      const captured: ActorInstanceLifecycleReceipt = { kind: "captured", lifetime: { activationGeneration: lease.activation.activationGeneration, instanceId: 7, guestLifetime }, requestSequence: lease.openRequest.requestSequence };
      await answer(worker, lease.open(openInput, BUDGET), captured);
      await answer(worker, lease.acknowledge(captured, BUDGET));
      return captured;
    }
    it("joins canonical captured accepted retired and exact ACK with host retirement", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const fixture = JSON.parse(readFileSync(new URL("../🪪️activation/🚪️instance/🧪️tests/🧫️fixtures/🔣️.json", testSource.url), "utf8"));
      
      const oracle = new Ajv({ strict: true });
      
      const { client, workers } = harness(1);
      const worker = workers[0]!;
      const lease = await activateCaptured(client, worker);
      const phases = [lease.progress().kind];
      expect(lease.lifetime).toBeNull();
      expect(() => lease.beginClose()).toThrow("actor-lifecycle.capture-pending");
      expect(() => client.dispose("captured")).toThrow("actor-close.native-retirement-pending");
      const captured: ActorInstanceLifecycleReceipt = { kind: "captured", lifetime: { activationGeneration: 1n, instanceId: fixture.instanceId, guestLifetime: BigInt(fixture.guestLifetime) }, requestSequence: lease.openRequest.requestSequence };
      await answer(worker, lease.open(openInput, BUDGET), captured);
      phases.push(lease.progress().kind);
      expect(lease.lifetime).toEqual(captured.lifetime);
      expect(Object.isFrozen(lease.lifetime)).toBe(true);
      await answer(worker, lease.acknowledge(captured, BUDGET));
      phases.push(lease.progress().kind);
      const host = new OwnedUiInstance(lease.activation, lease.lifetime!, { maxNodes: 128, maxDepth: 16, maxChildren: 32, maxTextBytes: 4096, maxPatchOps: 128, maxPatchBytes: 65536 }, { usizeBits: 32 });
      lease.bindHostRetirement(host);
      const close = lease.beginClose();
      phases.push(lease.progress().kind);
      expect(lease.beginClose()).toBe(close);
      await expect(lease.activation.turn([{ kind: "request", payload: {} }], BUDGET)).rejects.toThrow("actor-activation.revoked");
      const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: captured.lifetime, requestSequence: close.requestSequence, closeGeneration: 41n };
      await answer(worker, lease.close(BUDGET), accepted);
      phases.push(lease.progress().kind);
      const retired: ActorInstanceLifecycleReceipt = { ...accepted, kind: "retired" };
      await answer(worker, lease.acknowledge(accepted, BUDGET), retired);
      phases.push(lease.progress().kind);
      const before = worker.sent.length;
      await expect(lease.acknowledge(retired, BUDGET)).rejects.toThrow("actor-lifecycle.host-retirement-pending");
      expect(worker.sent).toHaveLength(before);
      expect(() => client.dispose("captured")).toThrow("actor-close.native-retirement-pending");
      host.beginClose();
      expect(host.closeStep({ maxItems: 1, maxBytes: 4096 }).kind).toBe("complete");
      const witness = host.takeRetirementWitness()!;
      await expect(lease.acknowledge(retired, BUDGET, Object.create(OwnedUiInstanceRetirement.prototype))).rejects.toThrow("actor-lifecycle.host-retirement-pending");
      await answer(worker, lease.acknowledge(retired, BUDGET, witness));
      phases.push(lease.progress().kind);
      expect(phases).toEqual(fixture.phases);
      
      const events = worker.sent.flatMap(value => (value as { events?: ShardEventEnvelope[] }).events ?? []);
      expect(events.map(event => event.kind)).toEqual(["instance-open", "instance-lifecycle-ack", "instance-close", "instance-lifecycle-ack", "instance-lifecycle-ack"]);
      expect(events[1]!.payload).toEqual({ kind: "ack", receipt: captured });
      client.dispose("captured");
    });
    it("keeps exact lifecycle authority after route revocation and rejects a replacement guest receipt", async () => {
      const { client, workers } = harness(2, { exclusiveShardCount: 1 });
      const worker = workers[0]!;
      const lease = await activateCaptured(client, worker);
      await openCaptured(worker, lease);
      client.leaseExclusive("captured");
      const close = lease.beginClose();
      const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: lease.lifetime!, requestSequence: close.requestSequence, closeGeneration: 41n };
      const pending = lease.close(BUDGET);
      const requestId = (worker.sent.at(-1) as { requestId: string }).requestId;
      workers[1]!.deliver({ kind: "result", requestId, ok: true, value: { status: { tag: "idle" }, lifecycleReceipt: encodeActorInstanceLifecycle(accepted) } });
      expect(lease.pendingReceipt).toBeNull();
      await answer(worker, pending, accepted);
      expect(workers[1]!.sent).toHaveLength(0);
      await expect(answer(worker, lease.acknowledge(accepted, BUDGET), { ...accepted, kind: "retired", lifetime: { ...accepted.lifetime, guestLifetime: 14n } })).rejects.toThrow("actor-lifecycle.receipt-mismatch");
      expect(lease.progress().kind).not.toBe("complete");
      client.disposeAll();
    });
    it("retains close authority after transport failure and worker loss without admitting commands", async () => {
      const { client, workers } = harness(1);
      const worker = workers[0]!;
      const lease = await activateCaptured(client, worker);
      await openCaptured(worker, lease);
      const request = lease.beginClose();
      const post = worker.postMessage.bind(worker);
      worker.postMessage = () => { throw new Error("fixture transport refused"); };
      await expect(lease.close(BUDGET)).rejects.toThrow("fixture transport refused");
      expect(lease.progress()).toEqual({ kind: "blocked", failure: "transport-refused" });
      expect(lease.beginClose()).toBe(request);
      worker.postMessage = post;
      const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: lease.lifetime!, requestSequence: request.requestSequence, closeGeneration: 41n };
      await answer(worker, lease.close(BUDGET), accepted);
      const pending = lease.acknowledge(accepted, BUDGET);
      const observed = afterSettle(pending, () => expect(pending).rejects.toThrow("terminated"));
      const requestId = (worker.sent.at(-1) as { requestId: string }).requestId;
      client.terminate(0);
      await observed;
      client.rebuild(0);
      const replacement = await activateCaptured(client, workers[1]!);
      worker.deliver({ kind: "result", requestId, ok: true, value: { status: { tag: "idle" }, lifecycleReceipt: encodeActorInstanceLifecycle({ ...accepted, kind: "retired" }) } });
      expect(lease.progress()).toEqual({ kind: "blocked", failure: "worker-lost" });
      expect(replacement.lifetime).toBeNull();
      await expect(lease.poll(BUDGET)).rejects.toThrow("actor-lifecycle.worker-lost");
      client.disposeAll();
    });
    it("mints patch authority only from the exact captured turn and rejects forged UI ACKs", async () => {
      const { client, workers } = harness(1);
      const worker = workers[0]!;
      const lease = await activateCaptured(client, worker);
      await openCaptured(worker, lease);
      const patch = { surface: { instance: 7, surface: "main" }, revision: 1n, baseRevision: 0n, ops: [{ tag: "set-root", val: 0n }] };
      const result = { uiPatches: [patch], uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime: lease.lifetime!, patchSequence: 1n }), lifecycleReceipt: undefined, status: { tag: "idle" } };
      const pending = lease.poll(BUDGET);
      worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: result });
      expect(await pending).toBe(result);
      expect(() => lease.captureUiPatchAuthority({ ...result }, 0)).toThrow("actor-lifecycle.foreign-turn");
      expect(() => lease.captureUiPatchAuthority(result, 1)).toThrow("actor-lifecycle.patch-index");
      const source = lease.captureUiPatchAuthority(result, 0);
      expect(lease.captureUiPatchAuthority(result, 0)).toBe(source);
      expect(OwnedNativeUiPatchAuthority.matches(source, lease.activation, lease.lifetime!)).toBe(true);
      expect(OwnedNativeUiPatchAuthority.matches(Object.create(OwnedNativeUiPatchAuthority.prototype), lease.activation, lease.lifetime!)).toBe(false);
      expect(source.operation(0)).toBe(patch.ops[0]);
      expect(source.acceptInput({} as never)).toBe(false);
      expect(source.releaseInput({ ordinal: 0 } as OwnedUiPatchInputRetirement)).toBe(false);
      expect(source.inputRetired).toBe(false);
      expect(source.value).toMatchObject({ surface: "main", revision: 1, baseRevision: 0, operationCount: 1 });
      const before = worker.sent.length;
      await expect(lease.submitUiAcknowledgement(source, {} as OwnedUiPatchAcknowledgement, BUDGET)).rejects.toThrow("actor-lifecycle.ui-ack-mismatch");
      expect(worker.sent).toHaveLength(before);
      expect(OwnedNativeUiPatchSubmissionReceipt.matches(Object.create(OwnedNativeUiPatchSubmissionReceipt.prototype), source, {})).toBe(false);
      client.disposeAll();
    });
    it("requires the exact producer UI patch receipt and retains original claims on malformed or duplicate turns", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../🚪️lifetime/🩹️patch/🧪️tests/🧫️fixtures/🔣️.json", testSource.url), "utf8"));
      const { client, workers } = harness(1); const worker = workers[0]!; const lease = await activateCaptured(client, worker); await openCaptured(worker, lease);
      const receipt = { lifetime: lease.lifetime!, patchSequence: BigInt(fixture.vectors[1].value.patchSequence) };
      const patch = { surface: { instance: 7, surface: fixture.feedback.surface }, revision: 1n, baseRevision: 0n, ops: [] };
      async function returned(uiPatchReceipt: unknown, patches: unknown[] = [patch]) {
        const result = { status: { tag: "idle" }, uiPatches: patches, uiPatchReceipt };
        const pending = lease.poll(BUDGET);
        worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: result });
        expect(await pending).toBe(result); return result;
      }
      const validBytes = encodeActorUiPatchReceipt(receipt);
      const invalid: unknown[] = [undefined, null, new Uint8Array(36), encodeActorUiPatchReceipt({ ...receipt, lifetime: { ...receipt.lifetime, guestLifetime: receipt.lifetime.guestLifetime + 1n } }), encodeActorUiPatchReceipt({ ...receipt, lifetime: { ...receipt.lifetime, activationGeneration: receipt.lifetime.activationGeneration + 1n } })];
      for (const hex of fixture.invalidHex) invalid.push(Buffer.from(hex, "hex"));
      for (const wire of invalid) {
        const result = await returned(wire);
        expect(() => lease.captureUiPatchAuthority(result, 0)).toThrow();
        expect(result.uiPatches[0]).toBe(patch); expect(result.uiPatchReceipt).toBe(wire);
      }
      const [{ default: Ajv }, { actorUiPatchReceiptSchema: receiptSchema }] = await Promise.all([import("ajv"), import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🩹️patch/🟦️.ts")]);
      const identityOracle = new Ajv({ strict: true }).compile(receiptSchema.$defs.PatchIdentityReferences);
      const patchObjects = [patch, { ...patch, surface: { instance: 7, surface: "second" } }];
      for (const [index, row] of fixture.patchIdentity.entries()) {
        expect(identityOracle(row.references)).toBe(row.accepted);
        const output = await returned(encodeActorUiPatchReceipt({ ...receipt, patchSequence: BigInt(index + 1) }), row.references.map((reference: number) => patchObjects[reference]!));
        if (row.accepted) expect(() => lease.captureUiPatchAuthority(output, 0)).not.toThrow();
        else expect(() => lease.captureUiPatchAuthority(output, 0)).toThrow("actor-ui-patch.pairing");
      }
      const overfull = await returned(validBytes, [patch, patch]);
      expect(() => lease.captureUiPatchAuthority(overfull, 0)).toThrow("actor-ui-patch.pairing");
      const original = await returned(validBytes); const source = lease.captureUiPatchAuthority(original, 0);
      expect(source.value.receipt).toEqual(receipt); expect(Object.isFrozen(source.value.receipt)).toBe(true); expect(Object.isFrozen(source.value.receipt.lifetime)).toBe(true);
      expect(lease.captureUiPatchAuthority(original, 0)).toBe(source);
      const duplicate = await returned(validBytes);
      expect(() => lease.captureUiPatchAuthority(duplicate, 0)).toThrow("actor-ui-patch.duplicate-sequence");
      expect(lease.captureUiPatchAuthority(original, 0)).toBe(source); expect(duplicate.uiPatches[0]).toBe(patch);
      validBytes.fill(0); expect(source.value.receipt).toEqual(receipt);
      client.disposeAll();
    });
    it("retains ordinary turn output on its exact captured instance through settlement revocation", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../🪪️activation/🚪️instance/🧪️tests/🧫️fixtures/🔣️.json", testSource.url), "utf8"));
      for (const outcome of fixture.ordinaryOutput) {
        const { client, workers } = harness(1); const worker = workers[0]!; const lease = await activateCaptured(client, worker); await openCaptured(worker, lease);
        const result = { uiPatches: [{ surface: { instance: fixture.instanceId, surface: "main" }, revision: 1n, baseRevision: 0n, ops: [] }], uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime: lease.lifetime!, patchSequence: 1n }), effects: [], status: { tag: "idle" } };
        const pending = client.turn(lease.activation.actorId, [], BUDGET);
        const observed = afterSettle(pending, () => outcome === "revoked" ? expect(pending).rejects.toThrow("actor-activation.revoked") : expect(pending).resolves.toBe(result));
        if (outcome === "revoked") lease.beginClose();
        worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: result });
        await observed;
        const source = lease.captureUiPatchAuthority(result, 0); expect(OwnedNativeUiPatchAuthority.matches(source, lease.activation, lease.lifetime!)).toBe(true);
        if (outcome === "revoked") expect(lease.interruptedTurn).toBe(result);
        client.disposeAll();
      }
    });

    it("retains exact receipt authority on faulted refused clock-fault and malformed ACK turns", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../🪪️activation/🚪️instance/🧪️tests/🧫️fixtures/🔣️.json", testSource.url), "utf8"));
      for (const status of fixture.invalidAckStatuses) {
        const { client, workers } = harness(1);
        const worker = workers[0]!;
        const lease = await activateCaptured(client, worker);
        const captured: ActorInstanceLifecycleReceipt = { kind: "captured", lifetime: { activationGeneration: 1n, instanceId: 7, guestLifetime: 13n }, requestSequence: lease.openRequest.requestSequence };
        await answer(worker, lease.open(openInput, BUDGET), captured);
        const pending = lease.acknowledge(captured, BUDGET);
        const rejected = afterSettle(pending, () => expect(pending).rejects.toThrow("actor-lifecycle.ack-not-admitted"));
        worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: { status } });
        await rejected;
        expect(lease.pendingReceipt).toEqual(captured);
        expect(() => lease.beginClose()).toThrow("actor-lifecycle.capture-pending");
        await answer(worker, lease.acknowledge(captured, BUDGET));
        expect(lease.progress().kind).toBe("open");
        client.disposeAll();
      }
    });
    it("cancels only the captured activation one effect per lifecycle turn", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const fixture = JSON.parse(readFileSync(new URL("../🪪️activation/🚪️instance/🧪️tests/🧫️fixtures/🔣️.json", testSource.url), "utf8"));
      const signals: AbortSignal[] = [];
      const { client, workers } = harness(1, { onHostEffect: (_actor, _effect, _params, signal) => { signals.push(signal); return new Promise(() => {}); } });
      const worker = workers[0]!;
      const lease = await activateCaptured(client, worker);
      await openCaptured(worker, lease);
      for (const requestId of ["one", "two"]) worker.deliver(makeEffectRequestFrame("captured", "storage-read", requestId, {}));
      lease.beginClose();
      const counts = [signals.filter(signal => signal.aborted).length];
      await answer(worker, lease.poll(BUDGET));
      counts.push(signals.filter(signal => signal.aborted).length);
      await answer(worker, lease.poll(BUDGET));
      counts.push(signals.filter(signal => signal.aborted).length);
      expect(counts).toEqual(fixture.closeCancellation);
      
      client.terminate(0);
      client.rebuild(0);
      const replacement = await activateCaptured(client, workers[1]!);
      await openCaptured(workers[1]!, replacement);
      workers[1]!.deliver(makeEffectRequestFrame("captured", "storage-read", "replacement", {}, replacement.activation.activationGeneration));
      expect(() => lease.beginClose()).not.toThrow();
      expect(signals.at(-1)!.aborted).toBe(false);
      client.disposeAll();
    });
    it("cannot close an old open owner through the replacement activation effect ledger", async () => {
      const signals: AbortSignal[] = [];
      const { client, workers } = harness(1, { onHostEffect: (_actor, _effect, _params, signal) => { signals.push(signal); return new Promise(() => {}); } });
      const old = await activateCaptured(client, workers[0]!);
      await openCaptured(workers[0]!, old);
      client.terminate(0);
      client.rebuild(0);
      const fresh = await activateCaptured(client, workers[1]!);
      await openCaptured(workers[1]!, fresh);
      workers[1]!.deliver(makeEffectRequestFrame("captured", "storage-read", "replacement", {}, fresh.activation.activationGeneration));
      expect(() => old.beginClose()).toThrow("actor-lifecycle.worker-lost");
      expect(signals[0]!.aborted).toBe(false);
      client.disposeAll();
    });
  });
}
