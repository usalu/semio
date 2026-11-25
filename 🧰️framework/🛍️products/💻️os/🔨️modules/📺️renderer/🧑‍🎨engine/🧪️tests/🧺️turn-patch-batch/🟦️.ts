import { fixtureOriginalWorkStock } from "@semio-tech/framework-actor/retained-turn/bootstrap/testing/fixtures";
/** 🧺️ The host decoder's half of the turn patch BATCH, as laws.
 *
 * A reactor turn result now carries EVERY surface that was ready and fits the turn's byte budget
 * under ONE `ui-patch-receipt` (`🎠️kernel/🦀️.rs`'s `UiTurnPatches`, priced by the Rust twin in
 * `⚛️reactor/🧪️tests/🧺️turn-patch-batch`). The host half has to do two things it could not do while
 * the cardinality was one:
 *
 * 1. capture EVERY patch of the turn as its own private authority — one receipt, N patches, in
 *    publication order, each addressed by its own `(surface, revision)`; the duplicate-sequence guard
 *    still refuses that receipt on any OTHER turn;
 * 2. acknowledge the whole batch as one ordered submission — a reactor turn serves exactly one event
 *    slot (`⚛️reactor/🎟️context/🌉️call/🎭️exports` refuses a batch of events), so the batch is N
 *    crossings of one `patch-ack` event each, in publication order, each answer returned to the
 *    caller and the whole submission memoized, so a repeat admits nothing new.
 *
 * See `📓️ui-turn-patch-batching-2026-09-15.md`.
 */
import { describe, expect, it } from "vitest";
import { MAINTENANCE_LANE_DEFAULT_BUDGET, type ShardEventEnvelope } from "@semio-tech/framework-actor/shard-client";
import { ShardClient } from "@semio-tech/framework-actor/shard-client";
import { type ShardWorkerLike } from "@semio-tech/framework-actor/shard-client";
import { encodeActorInstanceLifecycle } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts";
import { encodeActorUiPatchReceipt, validateActorUiPatchPairing } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🩹️patch/🟦️.ts";
import { OwnedUiInstance } from "@semio-tech/ui-contract/retained/instance";
import { OwnedResidentLedger } from "@semio-tech/value-ts/resident";

const SURFACES = ["window", "panel", "catalogue", "inspection", "measures", "artifact"] as const;
const GRANT = { maxItems: 1, maxBytes: 4096 };
const INSTANCE = 7;

type Sent = { kind: string; requestId: string; events?: readonly ShardEventEnvelope[] };

const flush = async (times: number): Promise<void> => {
  for (let index = 0; index < times; index += 1) await Promise.resolve();
};

/** 🧪️ One shard client over a fake worker, opened to a captured lifetime with a bound UI owner. */
async function fixture(actorId: string) {
  const sent: Sent[] = [];
  const worker: ShardWorkerLike = { onmessage: null, onerror: null, postMessage(message) { sent.push(message as Sent); }, terminate() {} };
  const client = new ShardClient({ originalWorkStock: fixtureOriginalWorkStock(), residentLedger: new OwnedResidentLedger({ bytes: 1_048_576, slots: 4_096, owners: 4_096, control: { bytes: 65_536, slots: 256, owners: 256 } }), shardCount: 1, createWorker: () => worker });
  const answerCrossings = async <T>(pending: Promise<T>, count: number, value: unknown): Promise<T> => {
    const base = sent.length;
    for (let index = 0; index < count; index += 1) {
      for (let spin = 0; spin < 4096 && sent.length === base + index; spin += 1) await Promise.resolve();
      worker.onmessage!({ data: { kind: "result", requestId: sent.at(-1)!.requestId, ok: true, value } } as MessageEvent);
    }
    return pending;
  };
  const answer = <T>(pending: Promise<T>, value: unknown): Promise<T> => answerCrossings(pending, 1, value);
  const plain = { uiPatches: [], effects: [], nextWake: null, status: { tag: "idle" } };
  await answer(client.activate(actorId, "/fixture.js", [], MAINTENANCE_LANE_DEFAULT_BUDGET), undefined);
  const lease = client.captureInstanceLifecycle(actorId, INSTANCE);
  const lifetime = { activationGeneration: lease.activation.activationGeneration, instanceId: INSTANCE, guestLifetime: 3n };
  const captured = { kind: "captured" as const, lifetime, requestSequence: lease.openRequest.requestSequence };
  await answer(lease.open({ appId: "fixture", purpose: { tag: "ordinary" }, actor: {}, config: new Uint8Array(), assets: [], capabilities: [], quotas: new Uint8Array() }, MAINTENANCE_LANE_DEFAULT_BUDGET), { ...plain, lifecycleReceipt: encodeActorInstanceLifecycle(captured) });
  const owner = new OwnedUiInstance(lease.activation, lifetime, { maxNodes: 128, maxDepth: 16, maxChildren: 32, maxTextBytes: 4096, maxPatchOps: 128, maxPatchBytes: 65_536 }, { usizeBits: 32 });
  lease.bindHostRetirement(owner);
  await answer(lease.acknowledge(captured, MAINTENANCE_LANE_DEFAULT_BUDGET), plain);
  return { sent, client, answer, answerCrossings, plain, lease, lifetime, owner };
}

/** 🩹️ One turn result carrying `count` surfaces under a single receipt. */
function batchTurn(plain: object, lifetime: { activationGeneration: bigint; instanceId: number; guestLifetime: bigint }, count: number, patchSequence: bigint) {
  return {
    ...plain,
    uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime, patchSequence }),
    uiPatches: SURFACES.slice(0, count).map((surface, index) => ({ surface: { instance: INSTANCE, surface }, revision: BigInt(index + 1), baseRevision: 0n, ops: [] })),
  };
}

describe("🧺️ the turn patch batch, on the host side", () => {
  it("pairs ONE receipt with every patch count a turn may carry", () => {
    const receipt = { lifetime: { activationGeneration: 1n, instanceId: INSTANCE, guestLifetime: 1n }, patchSequence: 9n };
    for (const count of [1, 2, 6, 16, 64]) expect(() => validateActorUiPatchPairing(count, receipt)).not.toThrow();
    expect(() => validateActorUiPatchPairing(0, null)).not.toThrow();
    for (const bad of [[0, receipt] as const, [1, null] as const, [-1, receipt] as const, [1.5, receipt] as const, [Number.NaN, receipt] as const]) {
      expect(() => validateActorUiPatchPairing(bad[0], bad[1])).toThrow("actor-ui-patch.pairing");
    }
  });

  it("captures every patch of one turn as its own authority, in publication order, under one receipt", async () => {
    const { answer, client, plain, lease, lifetime } = await fixture("batch-capture");
    const turn = batchTurn(plain, lifetime, SURFACES.length, 1n);
    await answer(lease.poll(MAINTENANCE_LANE_DEFAULT_BUDGET), turn);
    const captured = SURFACES.map((_, index) => lease.captureUiPatchAuthority(turn, index));
    expect(captured.map((source) => source.value.surface)).toEqual([...SURFACES]);
    expect(captured.map((source) => source.value.revision)).toEqual(SURFACES.map((_, index) => index + 1));
    expect(new Set(captured.map((source) => source.value.receipt.patchSequence))).toEqual(new Set([1n]));
    expect(lease.captureUiPatchAuthority(turn, 0)).toBe(captured[0]);
    expect(() => lease.captureUiPatchAuthority(turn, SURFACES.length)).toThrow("actor-lifecycle.patch-index");
    const stale = batchTurn(plain, lifetime, 1, 1n);
    await answer(lease.poll(MAINTENANCE_LANE_DEFAULT_BUDGET), stale);
    expect(() => lease.captureUiPatchAuthority(stale, 0)).toThrow("actor-ui-patch.duplicate-sequence");
    client.disposeAll();
  });

  it("acknowledges the whole batch as one ordered submission, one crossing and one patch-ack event per surface", async () => {
    const { sent, answer, answerCrossings, client, plain, lease, lifetime, owner } = await fixture("batch-ack");
    const count = 3;
    const turn = batchTurn(plain, lifetime, count, 1n);
    await answer(lease.poll(MAINTENANCE_LANE_DEFAULT_BUDGET), turn);
    const entries = SURFACES.slice(0, count).map((surface, index) => {
      const source = lease.captureUiPatchAuthority(turn, index);
      const lookup = owner.beginSurfaceLookup(lease.activation, lifetime, surface)!;
      for (let step = 0; lookup.advance(GRANT).kind !== "ready"; step += 1) if (step > 1024) throw new Error("fixture lookup did not complete");
      const facade = lookup.takeResult()!;
      lookup.beginClose();
      while (lookup.closeStep(GRANT).kind !== "complete") {}
      const patch = owner.beginPatch(source, facade);
      patch.finishInput();
      for (let step = 0; patch.advance(GRANT).kind !== "ready"; step += 1) if (step > 1024) throw new Error("fixture publication did not complete");
      return { source, token: patch.peekAcknowledgement()!, patch };
    });
    const before = sent.length;
    const submitted = await answerCrossings(lease.submitUiAcknowledgements(entries.map(({ source, token }) => ({ source, token })), MAINTENANCE_LANE_DEFAULT_BUDGET), count, plain);
    expect(sent.length - before).toBe(count);
    const crossings = sent.slice(before).map((message) => message.events!);
    expect(crossings.map((events) => events.length)).toEqual(Array.from({ length: count }, () => 1));
    const posted = crossings.flat();
    expect(posted.map((event) => event.kind)).toEqual(Array.from({ length: count }, () => "patch-ack"));
    expect(posted.map((event) => (event.payload as { surface: { surface: string } }).surface.surface)).toEqual(SURFACES.slice(0, count));
    expect(posted.map((event) => (event.payload as { revision: bigint }).revision)).toEqual(SURFACES.slice(0, count).map((_, index) => BigInt(index + 1)));
    expect(submitted.receipts).toHaveLength(count);
    for (const [index, { patch }] of entries.entries()) {
      expect(patch.acceptAcknowledgement(submitted.receipts[index]!)).toBe(true);
      expect(patch.acceptAcknowledgement(submitted.receipts[index]!)).toBe(false);
    }
    const again = await lease.submitUiAcknowledgements(entries.map(({ source, token }) => ({ source, token })), MAINTENANCE_LANE_DEFAULT_BUDGET);
    expect(sent.length - before).toBe(count);
    expect(again.receipts).toEqual(submitted.receipts);
    client.disposeAll();
  });

  it("refuses an empty batch and a token that belongs to another patch", async () => {
    const { answer, client, plain, lease, lifetime } = await fixture("batch-refusal");
    const turn = batchTurn(plain, lifetime, 2, 1n);
    await answer(lease.poll(MAINTENANCE_LANE_DEFAULT_BUDGET), turn);
    await expect(lease.submitUiAcknowledgements([], MAINTENANCE_LANE_DEFAULT_BUDGET)).rejects.toThrow("actor-lifecycle.ui-ack-mismatch");
    const source = lease.captureUiPatchAuthority(turn, 0);
    await expect(lease.submitUiAcknowledgements([{ source, token: {} as never }], MAINTENANCE_LANE_DEFAULT_BUDGET)).rejects.toThrow("actor-lifecycle.ui-ack-mismatch");
    client.disposeAll();
  });
});
