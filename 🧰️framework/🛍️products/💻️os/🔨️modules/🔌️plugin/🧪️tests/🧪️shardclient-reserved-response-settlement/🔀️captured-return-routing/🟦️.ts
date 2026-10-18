/** 🔀️ The captured return page minting, worker routing and loss tests. */
import { afterSettle, shardClientInternals } from "../🧰️settlement-test-kit/🟦️.ts";
import type { PendingEntry, ShardClientSettlementTestKit } from "../🧰️settlement-test-kit/🟦️.ts";
import type { CapturedReturnHelpers } from "../🛠️captured-return-helpers/🟦️.ts";

/** 🧭 Registers the page minting, foreign reply, retry, raw envelope, worker loss and cancellation tests. */
export function registerCapturedReturnRoutingTests(kit: ShardClientSettlementTestKit, helpers: CapturedReturnHelpers): void {
  const { BUDGET, OwnedShardReturnPage, capturedReturnState, createActorBytePage, expect, fixtureHosts, it } = kit;
  const { captured, fixtureCapturedReturn, fixtureResponse } = helpers;
  it("mints a page only from the original captured response and keeps controls on its old worker", async () => {
    const { default: Ajv } = await import("ajv");
    
    const { encodeActorReturnResult, decodeActorReturnDrive } = await import("../../../../../../../🔨️modules/🎭️actor/📤️return/🟦️.ts");
    const { row, client, workers, worker, instance } = await captured();
    const oracle = new Ajv({ strict: true }); 
    const source = await fixtureCapturedReturn(instance, row.responseSlots);
    expect(instance.pendingReturn).toBe(source);
    expect(instance.reserveReturn(row.responseSlots, { maxItems: 1, maxBytes: 64 }).source).toBe(source);
    await fixtureResponse(source); const running = source.execute([], BUDGET);
    const execute = worker.sent.at(-1) as { requestId: string; returnDrive: Uint8Array };
    const drive = decodeActorReturnDrive(execute.returnDrive);
    if (drive.kind !== "execute") throw new Error("expected execute");
    expect(drive.origin.requestSequence).toBe(Number(execute.requestId.slice(1)));
    const identity = { origin: drive.origin, returnSequence: BigInt(row.returnSequence) };
    const first = { kind: "result" as const, requestId: execute.requestId, ok: true as const, value: encodeActorReturnResult({ kind: "pending", identity, reason: "working" }) };
    const entries: Map<string, PendingEntry> = shardClientInternals(client).state.pending; const remove = entries.delete.bind(entries);
    entries.delete = key => { if (key === execute.requestId) expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(first); return remove(key); };
    worker.deliver(first); await running;
    client.leaseExclusive(row.actorId);
    await fixtureResponse(source); const polling = source.poll(BUDGET);
    const poll = worker.sent.at(-1) as { requestId: string; returnDrive: Uint8Array; events: unknown[] };
    expect(poll.events).toEqual([]); expect(decodeActorReturnDrive(poll.returnDrive)).toEqual({ kind: "control", control: { kind: "poll", identity } });
    const receipt = { identity, pageSequence: BigInt(row.pageSequence), length: row.pageBytes.length, final: true };
    const pageResponse = { kind: "result" as const, requestId: poll.requestId, ok: true as const, value: encodeActorReturnResult({ kind: "page", receipt, page: createActorBytePage(Uint8Array.from(row.pageBytes)) }) };
    worker.deliver(pageResponse); await polling;
    const pageOutput = capturedReturnState(source).latest;
    const page = source.page!;
    expect(row.pageBytes.map((_, index) => page.byteAt(index))).toEqual(row.pageBytes);
    expect(OwnedShardReturnPage.matchesOwner(page, fixtureHosts.get(instance)!, instance.activation, instance.lifetime!)).toBe(true);
    expect(OwnedShardReturnPage.matchesOwner({ receipt }, fixtureHosts.get(instance)!, instance.activation, instance.lifetime!)).toBe(false);
    expect(() => Reflect.construct(OwnedShardReturnPage, [receipt])).toThrow("actor-return.private-page");
    await expect(source.execute([], BUDGET)).rejects.toThrow("actor-return.execute-already-owned");
    await fixtureResponse(source); const cancelling = source.cancel(BUDGET);
    const cancel = worker.sent.at(-1) as { requestId: string; returnDrive: Uint8Array; events: unknown[] };
    expect(cancel.events).toEqual([]);
    worker.deliver({ kind: "result", requestId: cancel.requestId, ok: true, value: encodeActorReturnResult({ kind: "control", control: { kind: "cancel", identity }, outcome: "accepted", fault: "none" }) });
    await cancelling;
    const actual = { originalControlPosts: worker.sent.filter(value => value !== null && typeof value === "object" && Reflect.has(value, "returnDrive")).length - 1, replacementControlPosts: workers[1]!.sent.length, sameOrigin: source.origin?.requestSequence === drive.origin.requestSequence, sameParent: source.page === page && instance.pendingReturn === source, retainedResponses: source.retainedResponses, inputAckAvailable: "acknowledgeInput" in source };
    expect(actual).toEqual(row.expected); 
    expect(pageOutput?.responseEnvelope).toBe(pageResponse);
    expect(source.reserveResponse({ maxItems: 1, maxBytes: 4096 }).kind).toBe("pending"); expect(source.reserveResponse({ maxItems: 1, maxBytes: 4096 }).kind).toBe("blocked"); await expect(source.poll(BUDGET)).rejects.toThrow("actor-return.response-admission-required");
    expect(source.page).toBe(page); expect(source.retainedResponses).toBe(row.responseSlots);
    expect(() => instance.dispose()).toThrow("actor-close.native-retirement-pending");
    client.disposeAll();
  });
  it("retains foreign replies and refuses replaced workers without redirecting cancellation", async () => {
    const { encodeActorReturnResult, decodeActorReturnDrive } = await import("../../../../../../../🔨️modules/🎭️actor/📤️return/🟦️.ts");
    const { row, client, workers, worker, instance } = await captured();
    const source = await fixtureCapturedReturn(instance, row.responseSlots);
    await fixtureResponse(source); const running = source.execute([], BUDGET); const observed = afterSettle(running, () => expect(running).rejects.toThrow("actor-return.foreign-origin"));
    const execute = worker.sent.at(-1) as { requestId: string; returnDrive: Uint8Array };
    const drive = decodeActorReturnDrive(execute.returnDrive); if (drive.kind !== "execute") throw new Error("expected execute");
    const raw = { kind: "result" as const, requestId: execute.requestId, ok: true as const, value: encodeActorReturnResult({ kind: "pending", identity: { origin: { ...drive.origin, requestSequence: drive.origin.requestSequence + 1 }, returnSequence: 1n }, reason: "working" }), unknown: new Uint8Array(8192) };
    worker.deliver(raw); await observed;
    expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(raw); expect(raw.unknown.byteLength).toBe(8192);
    expect(source.page).toBeNull(); expect(instance.pendingReturn).toBe(source);
    client.terminate(0); client.rebuild(0);
    const before = workers[2]!.sent.length;
    await expect(source.cancel(BUDGET)).rejects.toThrow("actor-return.worker-lost");
    expect(workers[2]!.sent.length).toBe(before); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(raw);
    client.disposeAll();
  });
  it("retries a refused execute with its frozen original origin and retains the refused owner", async () => {
    const { encodeActorReturnResult, decodeActorReturnDrive } = await import("../../../../../../../🔨️modules/🎭️actor/📤️return/🟦️.ts");
    const { row, client, worker, instance } = await captured();
    const source = await fixtureCapturedReturn(instance, row.responseSlots); const post = worker.postMessage.bind(worker);
    await fixtureResponse(source); worker.postMessage = () => { throw new Error("fixture return post refusal"); };
    await expect(source.execute([], BUDGET)).rejects.toThrow("fixture return post refusal");
    const origin = source.origin!; expect(source.retainedResponses).toBe(1); expect(instance.pendingReturn).toBe(source);
    worker.postMessage = post;
    await fixtureResponse(source); const retried = source.retry(BUDGET); const message = worker.sent.at(-1) as { requestId: string; returnDrive: Uint8Array };
    expect(Number(message.requestId.slice(1))).toBeGreaterThan(origin.requestSequence);
    expect(decodeActorReturnDrive(message.returnDrive)).toEqual({ kind: "execute", origin });
    worker.deliver({ kind: "result", requestId: message.requestId, ok: true, value: encodeActorReturnResult({ kind: "pending", identity: { origin, returnSequence: BigInt(row.returnSequence) }, reason: "working" }) });
    await retried; expect(source.origin).toBe(origin); expect(source.retainedResponses).toBe(2);
    client.disposeAll();
  });
  it("keeps raw envelopes private and blocks other turn paths while the captured return is owned", async () => {
    const { row, client, instance } = await captured();
    const source = await fixtureCapturedReturn(instance, row.responseSlots);
    expect("firstResponse" in source || "latestResponse" in source).toBe(row.boundaries.publicRawResponse);
    await expect(instance.activation.turn([], BUDGET)).rejects.toThrow("actor-return.already-owned");
    await expect(instance.poll(BUDGET)).rejects.toThrow("actor-return.retirement-pending");
    client.disposeAll();
  });
  it("observes original worker loss even after the routing roster moved away", async () => {
    const { encodeActorReturnResult } = await import("../../../../../../../🔨️modules/🎭️actor/📤️return/🟦️.ts");
    const { row, client, worker, instance } = await captured();
    const source = await fixtureCapturedReturn(instance, row.responseSlots); await fixtureResponse(source); const running = source.execute([], BUDGET);
    const request = worker.sent.at(-1) as { requestId: string };
    const identity = { origin: source.origin!, returnSequence: BigInt(row.returnSequence) };
    worker.deliver({ kind: "result", requestId: request.requestId, ok: true, value: encodeActorReturnResult({ kind: "pending", identity, reason: "working" }) });
    await running; client.leaseExclusive(row.actorId);
    worker.onerror?.(new Error("original worker failed after route move"));
    const before = worker.sent.length;
    const failure = source.cancel(BUDGET).then(() => null, error => error);
    if (worker.sent.length !== before) {
      const request = worker.sent.at(-1) as { requestId: string };
      worker.deliver({ kind: "result", requestId: request.requestId, ok: true, value: encodeActorReturnResult({ kind: "control", control: { kind: "cancel", identity }, outcome: "accepted", fault: "none" }) });
    }
    expect((await failure)?.message).toBe("actor-return.worker-lost");
    expect(worker.sent.length - before).toBe(row.boundaries.lostOriginalWorkerControlPosts);
    expect(instance.pendingReturn).toBe(source); client.disposeAll();
  });
  it("retains cancellation authority after a malformed response with a previously captured identity", async () => {
    const { encodeActorReturnResult } = await import("../../../../../../../🔨️modules/🎭️actor/📤️return/🟦️.ts");
    const { row, client, worker, instance } = await captured();
    const source = await fixtureCapturedReturn(instance, row.responseSlots); await fixtureResponse(source); const running = source.execute([], BUDGET);
    let request = worker.sent.at(-1) as { requestId: string };
    const identity = { origin: source.origin!, returnSequence: BigInt(row.returnSequence) };
    worker.deliver({ kind: "result", requestId: request.requestId, ok: true, value: encodeActorReturnResult({ kind: "pending", identity, reason: "working" }) }); await running;
    await fixtureResponse(source); const polling = source.poll(BUDGET); const observed = afterSettle(polling, () => expect(polling).rejects.toThrow());
    request = worker.sent.at(-1) as { requestId: string };
    const malformed = { kind: "result" as const, requestId: request.requestId, ok: true as const, value: Uint8Array.of(255), unknown: new Uint8Array(8192) };
    worker.deliver(malformed); await observed;
    const retained = capturedReturnState(source).latest;
    client.leaseExclusive(row.actorId);
    await fixtureResponse(source); const cancellation = source.cancel(BUDGET).then(() => true, () => false);
    request = worker.sent.at(-1) as { requestId: string };
    if (request.requestId !== malformed.requestId) worker.deliver({ kind: "result", requestId: request.requestId, ok: true, value: encodeActorReturnResult({ kind: "control", control: { kind: "cancel", identity }, outcome: "accepted", fault: "none" }) });
    expect(await cancellation).toBe(row.boundaries.cancellationAfterMalformedResult);
    expect(retained?.responseEnvelope).toBe(malformed); expect(malformed.unknown.byteLength).toBe(8192); client.disposeAll();
  });
}
