/** 🛠️ The captured return, response, input stream and delivered input fixtures shared by the captured return authority parts. */
import { loadInstanceOutputAdmissionFixture, loadActivationReturnAdmissionFixture, loadActorActivationReturnFixture } from "@semio-tech/framework-actor/testing/fixtures";
import { loadContentInputAuthorityFixture } from "@semio-tech/framework-kernel/testing/fixtures";
import { flushAdmission } from "../🧰️settlement-test-kit/🟦️.ts";
import type { ActorInstanceLifetime, OwnedShardReturn, ShardClientSettlementTestKit, ShardInstanceLifecycleLease } from "../🧰️settlement-test-kit/🟦️.ts";

/** 🔧 Creates the captured return fixtures shared by the captured return authority parts. */
export function createCapturedReturnHelpers(kit: ShardClientSettlementTestKit) {
  const { BUDGET, captureFixtureInstance, createActorBytePage, encodeActorUiPatchReceipt, expect, harness } = kit;
  async function fixtureCapturedReturn(instance: ShardInstanceLifecycleLease, capacity: number): Promise<OwnedShardReturn> {
    const fixture = await loadActivationReturnAdmissionFixture();
    let source: OwnedShardReturn | null = null;
    for (const phase of fixture.phases) {
      const current = instance.reserveReturn(capacity, { maxItems: 1, maxBytes: phase.grant });
      expect(current.step.kind).toBe(phase.kind); expect(current.step.bytes).toBe(phase.grant); expect(current.source !== null).toBe(phase.source); source = current.source;
    }
    if (!source) throw new Error("Original captured return was not published"); return source;
  }
  async function fixtureResponse(source: OwnedShardReturn): Promise<void> {
    const fixture = await loadInstanceOutputAdmissionFixture();
    for (let turn = 0; turn < fixture.phases.length + 1; turn++) { const current = source.reserveResponse({ maxItems: 1, maxBytes: 4096 }); if (current.kind === "ready") return; expect(current.kind).toBe("pending"); }
    throw new Error("Captured response admission exceeded declared transitions");
  }
  async function captured() {
    const row = await loadActorActivationReturnFixture();
    const { client, residentLedger, workers } = harness(2, { exclusiveShardCount: 1 });
    const pending = client.activate(row.actorId, "https://fixture.invalid/component.js", [], BUDGET);
    await flushAdmission();
    const worker = workers[0]!;
    worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: undefined });
    await pending;
    const instance = await captureFixtureInstance(client, worker, row.actorId);
    return { row, client, residentLedger, workers, worker, instance };
  }
  async function inputStream(lifetime: ActorInstanceLifetime, payloadBytes?: number) {
    const vector = await loadContentInputAuthorityFixture();
    const name = "@webassemblyjs/leb128/lib/leb.js"; const lib = await import(name); const encode = (lib.default ?? lib).encodeUIntBuffer;
    const uint = (n: bigint | number) => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(BigInt(n)); return Buffer.from(encode(bytes)); };
    const frame = (tag: number, body: Buffer) => Buffer.concat([Buffer.of(tag), uint(body.length), body]);
    const receipt = { lifetime, patchSequence: BigInt(vector.patchSequence) };
    const authority = Buffer.concat([uint(lifetime.activationGeneration), uint(lifetime.instanceId), uint(lifetime.guestLifetime), uint(receipt.patchSequence)]);
    expect(authority).toEqual(Buffer.from(encodeActorUiPatchReceipt(receipt)));
    const surface = Buffer.from(vector.surface); const payload = payloadBytes === undefined ? Buffer.from(vector.payloadHex, "hex") : Buffer.alloc(payloadBytes);
    if (payloadBytes !== undefined) for (let index = 0; index < payload.length; index++) payload[index] = (index * 37 + 11) % 256;
    const bytes = Buffer.concat([Buffer.from("73727401", "hex"), frame(0, Buffer.of(0, 0, 1, 0, 0)), frame(2, Buffer.concat([authority, uint(surface.length), surface, Buffer.of(0, 1, 1)])), frame(3, Buffer.concat([Buffer.of(vector.opcode), uint(BigInt(vector.node)), uint(payload.length), payload])), frame(4, Buffer.alloc(0)), frame(7, Buffer.alloc(13)), frame(9, Buffer.alloc(0))]);
    return { bytes, receipt, vector, payload };
  }
  async function deliveredInput(foreign?: "activationGeneration" | "instanceId" | "guestLifetime", payloadBytes?: number, pageLength?: number) {
    const context = await captured(); const { instance, worker, row } = context;
    const lifetime = { ...instance.lifetime! };
    if (foreign === "instanceId") lifetime.instanceId++; else if (foreign) lifetime[foreign]++;
    const stream = await inputStream(lifetime, payloadBytes);
    const source = await fixtureCapturedReturn(instance, row.responseSlots); await fixtureResponse(source); const pending = source.execute([], BUDGET);
    const request = worker.sent.at(-1) as { requestId: string };
    const { encodeActorReturnResult } = await import("@semio-tech/framework-actor/return");
    const identity = { origin: source.origin!, returnSequence: BigInt(row.returnSequence) };
    const pageBytes = pageLength === undefined ? stream.bytes : stream.bytes.subarray(0, pageLength);
    const response = { kind: "result" as const, requestId: request.requestId, ok: true as const, value: encodeActorReturnResult({ kind: "page", receipt: { identity, pageSequence: 1n, length: pageBytes.length, final: pageBytes.length === stream.bytes.length }, page: createActorBytePage(pageBytes) }) };
    worker.deliver(response); const report = await pending;
    return { ...context, ...stream, source, response, report };
  }
  return { fixtureCapturedReturn, fixtureResponse, captured, inputStream, deliveredInput };
}
export type CapturedReturnHelpers = ReturnType<typeof createCapturedReturnHelpers>;
