/** 🧾️ The reserved response settlement tests. */
import { loadInstanceOutputAdmissionFixture, loadActivationInstanceOutputFixture } from "@semio-tech/framework-actor/testing/fixtures";
import { afterSettle, shardClientInternals } from "../🧰️settlement-test-kit/🟦️.ts";
import type { OutboundMessage, OwnedActorTurnOutput, OwnedActorTurnOutputs, PendingEntry, ShardClientSettlementTestKit, ShardSlot } from "../🧰️settlement-test-kit/🟦️.ts";

/** 💬 Registers the reserved response settlement tests of the shard client. */
export function registerReservedResponseSettlementTests(kit: ShardClientSettlementTestKit): void {
  const { BUDGET, OwnedActorTurnOutputs, describe, expect, harness, it } = kit;
  async function fixtureOutputReservation(queue: OwnedActorTurnOutputs): Promise<OwnedActorTurnOutput> {
    const fixture = await loadInstanceOutputAdmissionFixture();
    for (let turn = 0; turn < fixture.phases.length + 1; turn++) { const current = queue.reserve({ maxItems: 1, maxBytes: 4096 }); if (current.step.kind === "ready" && current.output) return current.output; expect(current.step.kind).toBe("pending"); }
    throw new Error("Output admission exceeded declared transitions");
  }
  describe("ShardClient reserved response settlement", () => {
    it("captures the exact response before actual pending removal, heartbeat recomputation and caller settlement", async () => {
      const fixture = await loadActivationInstanceOutputFixture();
      const { client, workers, residentLedger } = harness(1); const worker = workers[0]!;
      const slot: ShardSlot = shardClientInternals(client).state.shards[0];
      const send = Reflect.get(client, "send").bind(client) as (slot: ShardSlot, message: OutboundMessage, request: string, posted: undefined, output: OwnedActorTurnOutput) => Promise<unknown>;
      const pendingEntries: Map<string, PendingEntry> = shardClientInternals(client).state.pending;
      const queue = new OwnedActorTurnOutputs({}, fixture.capacity, residentLedger); const output = await fixtureOutputReservation(queue);
      const raw = { kind: "result" as const, requestId: "reserved-fixture", ok: true as const, value: { uiPatches: [] }, unknown: new Uint8Array(fixture.responseSettlement.unknownPayloadBytes) };
      const order: string[] = [];
      const remove = pendingEntries.delete.bind(pendingEntries);
      pendingEntries.delete = key => { expect(output.responseEnvelope).toBe(raw); order.push("pending-removal"); return remove(key); };
      const transport = shardClientInternals(client).hub.transport; const recompute = (Reflect.get(transport, "recomputeOldestPending") as (target: ShardSlot) => unknown).bind(transport);
      Reflect.set(transport, "recomputeOldestPending", (target: ShardSlot) => { expect(output.responseEnvelope).toBe(raw); order.push("heartbeat-recompute"); return recompute(target); });
      const result = output.run(() => send(slot, { kind: "turn", requestId: raw.requestId, actorId: "reserved-fixture", activationGeneration: 1n, events: [], budget: BUDGET }, raw.requestId, undefined, output));
      const chained = result.then(() => { expect(output.responseEnvelope).toBe(raw); order.push("external-continuation"); throw new Error("fixture publication fault"); }); const observed = afterSettle(chained, () => expect(chained).rejects.toThrow("fixture publication fault"));
      worker.deliver(raw); await observed;
      expect(order).toEqual(fixture.responseSettlement.phases.filter(phase => phase !== "capture" && phase !== "error-graft"));
      expect(output.outcome?.value).toBe(raw.value); expect(queue.peek()?.responseEnvelope).toBe(raw);
      expect(pendingEntries.has(raw.requestId)).toBe(false); client.disposeAll();
    });

    it("retains the original failed response when actual worker error grafting throws", async () => {
      const fixture = await loadActivationInstanceOutputFixture();
      const { client, workers, residentLedger } = harness(1); const worker = workers[0]!;
      const slot: ShardSlot = shardClientInternals(client).state.shards[0];
      const send = Reflect.get(client, "send").bind(client) as (slot: ShardSlot, message: OutboundMessage, request: string, posted: undefined, output: OwnedActorTurnOutput) => Promise<unknown>;
      const queue = new OwnedActorTurnOutputs({}, fixture.capacity, residentLedger); const output = await fixtureOutputReservation(queue);
      const raw = { kind: "result" as const, requestId: "graft-fixture", ok: false as const, error: "native refusal", stack: "native stack", framesBytes: 8192, unknown: new Uint8Array(fixture.responseSettlement.unknownPayloadBytes) };
      const result = output.run(() => send(slot, { kind: "turn", requestId: raw.requestId, actorId: "graft-fixture", activationGeneration: 1n, events: [], budget: BUDGET }, raw.requestId, undefined, output));
      const failure = new Error("fixture diagnostic fault"); const observed = afterSettle(result, () => expect(result).rejects.toBe(failure));
      Object.defineProperty(raw, "stack", { get: () => { expect(output.responseEnvelope).toBe(raw); throw failure; } });
      expect(() => worker.deliver(raw)).not.toThrow(); await observed;
      expect(output.responseEnvelope).toBe(raw); expect(output.outcome?.value).toBe(failure); expect(raw.unknown.byteLength).toBe(fixture.responseSettlement.unknownPayloadBytes);
      expect(queue.peek()).toBe(output); client.disposeAll();
    });
  });
}
