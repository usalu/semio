/** 🪪️ Document backbone worker wire: admitting inference openings, requests and responses of the backbone worker.
 *
 * @see ../🧮️directory-client-helpers/🟦️.ts — the shared helpers
 * @see ../../🟦️.ts — the package entry the suite reads fixtures relative to
 */
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { describe, expect, it, vi } from "vitest";
import { BACKBONE_WORKER_WIRE_MAGIC, decodeBackboneMessage, decodeBackboneWorkerRequest, decodeBackboneWorkerResponse, encodeBackboneMessage, encodeBackboneWorkerRequest, encodeBackboneWorkerResponse, parseDocumentBackboneMessage, parseInboundDocumentBackboneMessage } from "@semio-tech/framework/backbone";
import { encodePackValue } from "@semio-tech/framework/packs";
import { source, type BackboneWorkerRequest } from "../🧮️directory-client-helpers/🟦️.ts";

describe("document backbone worker wire", () => {
  it("admits inference opening only after its exact worker receipt", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/💡️inference/🚪️opening/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    const { InferencePortOpeningMailboxV1, parseInferencePortClosedV1, parseInferencePortOpeningResultV1 } = await import("@semio-tech/framework/inference/opening");
    expect(equal(parseInferencePortClosedV1(fixture.closed), fixture.closed)).toBe(true);
    expect(equal(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(fixture.closed)), fixture.closed)).toBe(true);
    for (const extra of [{ authority: "forged" }, { operationEpoch: 0 }, { kind: "inference-port-opened" }]) expect(() => parseInferencePortClosedV1({ ...fixture.closed, ...extra })).toThrow();
    const sent: unknown[] = [];
    const mailbox = new InferencePortOpeningMailboxV1((request) => { sent.push(request); });
    try {
      const pending = mailbox.open(fixture.request);
      expect(equal(sent, [fixture.request])).toBe(true);
      expect(mailbox.settle({ ...fixture.opened, operationEpoch: 20 })).toBe(false);
      expect(mailbox.settle({ ...fixture.opened, scope: { ...fixture.opened.scope, documentId: "foreign" } })).toBe(false);
      await expect(mailbox.open(fixture.request)).rejects.toThrow("pending");
      expect(mailbox.settle(fixture.opened)).toBe(true);
      expect(equal(await pending, fixture.opened)).toBe(true);
      expect(mailbox.settle(fixture.opened)).toBe(false);
      await expect(mailbox.open(fixture.request)).rejects.toThrow("replayed");
      const refused = expect(mailbox.open({ ...fixture.request, operationEpoch: 22 })).rejects.toThrow("inference.capacity");
      expect(mailbox.settle(fixture.opened)).toBe(false);
      expect(mailbox.settle({ ...fixture.refused, operationEpoch: 22 })).toBe(true);
      await refused;
      const closed = expect(mailbox.open({ ...fixture.request, operationEpoch: 23 })).rejects.toThrow("retired");
      mailbox.close("retired");
      await closed;
      expect(mailbox.settle(fixture.opened)).toBe(false);
      expect(equal(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(fixture.opened)), fixture.opened)).toBe(true);
      for (const extra of [{ code: "inference.capacity" }, { authority: "forged" }, { operationEpoch: 0 }]) expect(() => parseInferencePortOpeningResultV1({ ...fixture.opened, ...extra })).toThrow();
      for (const [field, code] of [["indeterminate", "inference.capacity"], ["refused", "inference.transport"]]) {
        const hostile = { ...fixture[field!], code };
        expect(() => parseInferencePortOpeningResultV1(hostile)).toThrow();
      }
    } finally { mailbox.close("test cleanup"); }
  });

  it("projects real browser intent publications and preserves bounded owned inference effects", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧬️schema/🔣️.json", source.url), "utf8"));

    const { decodeBrowserActorCommandPublicationV1, decodeBrowserActorIntentPublicationV1, encodeBrowserActorHostEffectV1, decodeBrowserActorHostEffectsV1, publishBrowserActorHostEffectsV1, requireBrowserActorCommandBackboneProjectionV1 } = await import("@semio-tech/framework/browser-bundle/action-handoff/publication");
    const { encodeAppFrame } = await import("@semio-tech/framework/app-channel");
    const { encodePackValue } = await import("@semio-tech/framework/packs");
    const { parseBrowserActorActionResultV1 } = await import("@semio-tech/framework/browser-bundle/action-handoff");
    const emit = encodeAppFrame(fixture.publication.emit);
    expect(decodeBrowserActorIntentPublicationV1(emit)).toEqual({ kind: "emit" });
    expect(decodeBrowserActorIntentPublicationV1(encodeAppFrame(fixture.publication.error)).kind).toBe("error");
    expect(() => decodeBrowserActorIntentPublicationV1(encodeAppFrame({ Error: { ...fixture.publication.error.Error, in_reply_to: 0 } }))).toThrow();
    expect(() => decodeBrowserActorIntentPublicationV1(new Uint8Array([...emit, 0]))).toThrow();
    expect(() => decodeBrowserActorIntentPublicationV1(encodeAppFrame({ Emit: { ...fixture.publication.emit.Emit, in_reply_to: 1 } }))).toThrow();
    const invocation = encodeAppFrame({ Invocation: { in_reply_to: fixture.commandRequest.actionSequence, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } });
    const emptyInvocation = decodeBrowserActorCommandPublicationV1(invocation, fixture.commandRequest.actionSequence);
    expect(emptyInvocation.kind).toBe("invocation");
    if (emptyInvocation.kind !== "invocation") throw new Error("expected invocation");
    expect(() => requireBrowserActorCommandBackboneProjectionV1(emptyInvocation, [])).not.toThrow();
    const namedEmpty = decodeBrowserActorCommandPublicationV1(encodeAppFrame({ Invocation: { in_reply_to: fixture.commandRequest.actionSequence, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: Array.from(encodePackValue([])), inverse_group: Array.from(encodePackValue({ invocationId: "clearSelection:0", mutations: [], inverseMutations: [], memberEdits: [] })) } }), fixture.commandRequest.actionSequence);
    if (namedEmpty.kind !== "invocation") throw new Error("expected invocation");
    expect(namedEmpty.projection.inverseGroup.invocationId).toBe("clearSelection:0");
    expect(() => requireBrowserActorCommandBackboneProjectionV1(namedEmpty, [])).not.toThrow();
    expect(decodeBrowserActorCommandPublicationV1(encodeAppFrame({ Error: { in_reply_to: fixture.commandRequest.actionSequence, fault: [], report: [] } }), fixture.commandRequest.actionSequence)).toEqual({ kind: "error", reason: "action-guest-refused", detail: "unknown fault" });
    expect(() => decodeBrowserActorCommandPublicationV1(invocation, fixture.commandRequest.actionSequence + 1)).toThrow();
    expect(() => decodeBrowserActorCommandPublicationV1(new Uint8Array([...invocation, 0]), fixture.commandRequest.actionSequence)).toThrow();
    expect(() => decodeBrowserActorCommandPublicationV1(encodeAppFrame({ Invocation: { in_reply_to: fixture.commandRequest.actionSequence, output: [], diagnostics: [], ui_scope: [], history_patch: [1], messages: [], mutations: [], inverse_group: [] } }), fixture.commandRequest.actionSequence)).toThrow();
    expect(decodeBrowserActorCommandPublicationV1(encodeAppFrame({ Invocation: { in_reply_to: fixture.commandRequest.actionSequence, output: [], diagnostics: [], ui_scope: [], history_patch: Array.from(encodePackValue(null)), messages: [], mutations: [], inverse_group: [] } }), fixture.commandRequest.actionSequence).kind).toBe("invocation");
    const ephemeral = encodeAppFrame({ Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [], tool_run: [], history_edit: [] } });
    const ephemeralSnapshot = { kind: "ephemeral", snapshot: { presence: [1, 2], presenceGeneration: 3, transientGeneration: 4, interaction: [], toolRun: [], historyEdit: [] } };
    expect(decodeBrowserActorCommandPublicationV1(ephemeral, fixture.commandRequest.actionSequence)).toEqual(ephemeralSnapshot);
    expect(decodeBrowserActorIntentPublicationV1(ephemeral)).toEqual(ephemeralSnapshot);
    const progress = encodeAppFrame({ Invocation: { in_reply_to: 0, output: [1, 2], diagnostics: [], ui_scope: [3], history_patch: [], messages: [], mutations: [], inverse_group: [] } });
    expect(decodeBrowserActorCommandPublicationV1(encodeAppFrame({ OperationCompleted: { operation: 3n, revision: 0xfedc_ba98_7654_3210n, ui_scope: [1], history_patch: [] } }), fixture.commandRequest.actionSequence)).toEqual({ kind: "operation-completed", operation: 3n, revision: 0xfedc_ba98_7654_3210n, historyPatch: null });
    const historyBytes = Array.from(encodePackValue({ cursor: 1, upserts: [] }));
    expect(decodeBrowserActorCommandPublicationV1(encodeAppFrame({ OperationCompleted: { operation: 3n, revision: 1n, ui_scope: [], history_patch: historyBytes } }), fixture.commandRequest.actionSequence)).toEqual({ kind: "operation-completed", operation: 3n, revision: 1n, historyPatch: historyBytes });
    expect(() => decodeBrowserActorCommandPublicationV1(encodeAppFrame({ OperationCompleted: { operation: 3n, revision: 1n, ui_scope: [], history_patch: Array.from(encodePackValue([1])) } }), fixture.commandRequest.actionSequence)).toThrow("invalid history patch");
    const progressPublication = decodeBrowserActorCommandPublicationV1(progress, fixture.commandRequest.actionSequence);
    expect(progressPublication.kind).toBe("completion");
    if (progressPublication.kind !== "completion") throw new Error("expected completion");
    expect(() => requireBrowserActorCommandBackboneProjectionV1(progressPublication, [])).not.toThrow();
    expect(decodeBrowserActorIntentPublicationV1(progress).kind).toBe("completion");
    const reservedCompletion = decodeBrowserActorCommandPublicationV1(encodeAppFrame({ Invocation: { in_reply_to: 0, output: [], diagnostics: Array.from(encodePackValue([])), ui_scope: [], history_patch: Array.from(encodePackValue(null)), messages: [], mutations: Array.from(encodePackValue([])), inverse_group: Array.from(encodePackValue({ invocationId: "clearSelection:0", mutations: [], inverseMutations: [], memberEdits: [] })) } }), fixture.commandRequest.actionSequence);
    if (reservedCompletion.kind !== "completion") throw new Error("expected completion");
    expect(() => requireBrowserActorCommandBackboneProjectionV1(reservedCompletion, [])).not.toThrow();
    expect(() => decodeBrowserActorCommandPublicationV1(encodeAppFrame({ Invocation: { in_reply_to: 0, output: [], diagnostics: [], ui_scope: [], history_patch: Array.from(encodePackValue("rows")), messages: [], mutations: [], inverse_group: [] } }), fixture.commandRequest.actionSequence)).toThrow("invalid history patch");
    expect(() => decodeBrowserActorCommandPublicationV1(new Uint8Array([...ephemeral, 0]), fixture.commandRequest.actionSequence)).toThrow();
    const replyHistory = Array.from(encodePackValue({ cursor: 2, upserts: [] })),
      replyWithHistory = decodeBrowserActorCommandPublicationV1(encodeAppFrame({ Invocation: { in_reply_to: fixture.commandRequest.actionSequence, output: [], diagnostics: [], ui_scope: [], history_patch: replyHistory, messages: [], mutations: [], inverse_group: [] } }), fixture.commandRequest.actionSequence);
    expect(replyWithHistory.kind === "invocation" ? replyWithHistory.historyPatch : undefined).toEqual(replyHistory);
    const mutationId = "command-mutation-1",
      invocationId = "command:0:1",
      forward = [1, 2],
      inverse = [3, 4],
      inverseVector = [1, 2, 3, 4],
      mutation = {
        id: mutationId,
        document: "0",
        baseVersion: 0,
        invocationId,
        diff: { schema: "demo/v1.operation", payload: forward },
        inverse: { targetMutation: mutationId, inverseDiff: { schema: "demo/v1.operation.inverse", payload: inverseVector }, baseVersion: 0, dependencies: [], undoPolicy: "ExactBaseOnly" },
        dependencies: ["prior"],
        author: "actor-1",
        timestamp: { actor: 7, physical_ms: 8, logical: 9 },
      },
      inverseGroup = { invocationId, mutations: [mutationId], inverseMutations: [mutation.inverse] },
      mutationFrame = encodeAppFrame({ Invocation: { in_reply_to: fixture.commandRequest.actionSequence, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: Array.from(encodePackValue([mutation])), inverse_group: Array.from(encodePackValue(inverseGroup)) } }),
      mutationPublication = decodeBrowserActorCommandPublicationV1(mutationFrame, fixture.commandRequest.actionSequence),
      envelope = { mutation_id: mutationId, actor: "actor-1", dependencies: ["prior"], observed: null, target: [], diff: { schema: "demo/v1", payload: Uint8Array.from(forward) }, inverse: { schema: "demo/v1", payload: Uint8Array.from(inverse) }, timestamp: { actor: 7n, physical_ms: 8n, logical: 9n } };
    expect(mutationPublication.kind).toBe("invocation");
    if (mutationPublication.kind !== "invocation") throw new Error("expected mutation invocation");
    expect(() => requireBrowserActorCommandBackboneProjectionV1(mutationPublication, [envelope])).not.toThrow();
    expect(() => requireBrowserActorCommandBackboneProjectionV1(mutationPublication, [])).toThrow("action-publication-unprojected");
    expect(() => requireBrowserActorCommandBackboneProjectionV1(mutationPublication, [{ ...envelope, diff: { ...envelope.diff, payload: Uint8Array.of(9) } }])).toThrow("action-publication-unprojected");
    expect(() => requireBrowserActorCommandBackboneProjectionV1(mutationPublication, [{ ...envelope, inverse: { ...envelope.inverse, payload: Uint8Array.of(9) } }])).toThrow("action-publication-unprojected");
    for (const vector of fixture.publication.servicePayloads) {
      const value = { ...fixture.publication.hostEffect, val: { ...fixture.publication.hostEffect.val, payload: vector.pack } };
      if (!vector.valid) { expect(() => encodeBrowserActorHostEffectV1(value), vector.name).toThrow(); continue; }
      const encoded = encodeBrowserActorHostEffectV1(value);
      expect(equal(decodeBrowserActorHostEffectsV1([encoded]), [{ requestServiceOperation: { ...fixture.publication.projectedEffect.requestServiceOperation, payload: vector.expected } }]), vector.name).toBe(true);
    }
    const encoded = encodeBrowserActorHostEffectV1(fixture.publication.hostEffect);
    expect(equal(encoded, Array.from(encodePackValue(fixture.publication.projectedEffect)))).toBe(true);
    expect(equal(decodeBrowserActorHostEffectsV1([encoded]), [fixture.publication.projectedEffect])).toBe(true);
    const external = encodeBrowserActorHostEffectV1(fixture.publication.externalHostEffect);
    expect(equal(external, Array.from(encodePackValue(fixture.publication.externalProjectedEffect)))).toBe(true);
    expect(equal(decodeBrowserActorHostEffectsV1([external]), [fixture.publication.externalProjectedEffect])).toBe(true);
    for (const url of ["javascript:alert(1)", "data:text/html,script", "file:///secret", "/relative", "https://user:secret@example.invalid", "https://example.invalid/\nforged", "https://example.invalid/" + "a".repeat(2048)]) {
      expect(() => encodeBrowserActorHostEffectV1({ tag: "open-external-url", val: { url } })).toThrow();
    }
    const result = { ...fixture.acknowledged, hostEffects: [encoded] };
    expect(equal(parseBrowserActorActionResultV1(result), result)).toBe(true);
    expect(result.outcome).toBe("guest-applied");
    expect(() => parseBrowserActorActionResultV1({ ...result, outcome: "acknowledged" })).toThrow();
    expect(() => parseBrowserActorActionResultV1({ ...fixture.rejected, hostEffects: [encoded] })).toThrow();
    for (const effect of [
      { ...fixture.publication.hostEffect, val: { kind: "foreign" } },
      { ...fixture.publication.hostEffect, val: { ...fixture.publication.hostEffect.val, documentId: "forged" } },
      { tag: "invoke-extension", val: {} },
      { tag: "publish-event", val: {} },
    ]) expect(() => encodeBrowserActorHostEffectV1(effect)).toThrow();
    expect(() => decodeBrowserActorHostEffectsV1([[...encoded, 0]])).toThrow();
    expect(() => decodeBrowserActorHostEffectsV1(new Array(65).fill(encoded))).toThrow();
    expect(() => parseBrowserActorActionResultV1({ ...result, hostEffects: [new Array(262144).fill(1), [1]] })).toThrow();
    const published: unknown[] = [];
    const publish = (effect: unknown) => { published.push(effect); };
    await expect(publishBrowserActorHostEffectsV1([encoded], () => false, publish)).rejects.toThrow("retired");
    expect(published).toEqual([]);
    await expect(publishBrowserActorHostEffectsV1([encoded, [...encoded, 0]], () => true, publish)).rejects.toThrow();
    expect(published).toEqual([]);
    await publishBrowserActorHostEffectsV1([encoded], () => true, publish);
    expect(equal(published, [fixture.publication.projectedEffect])).toBe(true);
    await expect(publishBrowserActorHostEffectsV1([encoded, encoded], () => true, publish)).rejects.toThrow();
    expect(equal(published, [fixture.publication.projectedEffect])).toBe(true);
  });

  it("settles only the exact live browser intent mailbox and retires pending work", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    const { BrowserActorActionMailboxV1, browserActorActionRefusalReasonV1 } = await import("@semio-tech/framework/browser-bundle/action-handoff/requests");
    const { actionSequence: _sequence, ...owner } = fixture.request;
    const intent = { ...fixture.uiIntent, seq: BigInt(fixture.uiIntent.seq) };
    const sent: any[] = [];
    const mailbox = new BrowserActorActionMailboxV1((request) => { sent.push(request); }, 1000);
    try {
      const pending = mailbox.dispatchIntent(owner, "map", intent);
      expect(sent).toHaveLength(1);
      expect(sent[0].actionSequence).toBe(1);
      expect(mailbox.pending()).toBe(1);
      // 📬️ A second intent during the first is QUEUED, not refused: it owns sequence 2 but is not sent yet, and a
      // receipt naming that unsent sequence is foreign.
      const queued = mailbox.dispatchIntent(owner, "map", intent);
      expect(sent).toHaveLength(1);
      expect(mailbox.pending()).toBe(2);
      expect(mailbox.settle({ ...fixture.acknowledged, actionSequence: 2 })).toBe(false);
      for (const hostile of fixture.hostileResults.slice(0, 3)) expect(mailbox.settle({ ...hostile, actionSequence: 1 })).toBe(false);
      expect(mailbox.settle(fixture.acknowledged)).toBe(false);
      const acknowledged = { ...fixture.acknowledged, actionSequence: 1 };
      expect(mailbox.settle(acknowledged)).toBe(true);
      expect(equal(await pending, acknowledged)).toBe(true);
      expect(mailbox.settle(acknowledged)).toBe(false);
      // 🚚️ Settling the first sends the queued one; its receipt settles it alone.
      expect(sent).toHaveLength(2);
      expect(sent[1].actionSequence).toBe(2);
      expect(mailbox.pending()).toBe(1);
      const queuedReceipt = { ...fixture.acknowledged, actionSequence: 2 };
      expect(mailbox.settle(queuedReceipt)).toBe(true);
      expect(equal(await queued, queuedReceipt)).toBe(true);
      expect(mailbox.pending()).toBe(0);
      const next = mailbox.dispatchIntent(owner, "map", intent);
      const retired = expect(next).rejects.toThrow("retired");
      expect(sent[2].actionSequence).toBe(3);
      mailbox.close("retired");
      await retired;
      expect(mailbox.settle({ ...fixture.acknowledged, actionSequence: 3 })).toBe(false);
      await expect(mailbox.dispatchIntent(owner, "map", intent)).rejects.toThrow("closed");
      expect(mailbox.pending()).toBe(0);
    } finally {
      mailbox.close("test cleanup");
    }
    // 🎛️ Three rapid commands: sent one at a time in issue order, each settled by its own receipt, resolved in order.
    const rapidSent: number[] = [];
    const rapid = new BrowserActorActionMailboxV1((request) => { rapidSent.push(request.actionSequence); });
    try {
      const settledOrder: number[] = [];
      const commands = [1, 2, 3].map((sequence) => rapid.dispatchCommand(owner, fixture.actionInvocation, fixture.commandViewState).then((result) => { settledOrder.push(sequence); return result; }));
      expect(rapidSent).toEqual([1]);
      expect(rapid.pending()).toBe(3);
      expect(rapid.settle({ ...fixture.acknowledged, actionSequence: 3 })).toBe(false);
      expect(rapid.settle({ ...fixture.acknowledged, actionSequence: 2 })).toBe(false);
      expect(rapid.settle({ ...fixture.acknowledged, actionSequence: 1 })).toBe(true);
      expect(rapidSent).toEqual([1, 2]);
      expect(rapid.settle({ ...fixture.acknowledged, actionSequence: 2 })).toBe(true);
      expect(rapidSent).toEqual([1, 2, 3]);
      expect(rapid.settle({ ...fixture.acknowledged, actionSequence: 3 })).toBe(true);
      const results = await Promise.all(commands);
      expect(results.map((result) => result.actionSequence)).toEqual([1, 2, 3]);
      expect(settledOrder).toEqual([1, 2, 3]);
      expect(rapid.pending()).toBe(0);
    } finally {
      rapid.close("test cleanup");
    }
    // 🧮️ Capacity bounds the queue behind the in-flight request; the overflow is a typed `queue-full` refusal.
    const boundedSent: number[] = [];
    const bounded = new BrowserActorActionMailboxV1((request) => { boundedSent.push(request.actionSequence); }, 1000, 1);
    try {
      const first = bounded.dispatchCommand(owner, fixture.actionInvocation, fixture.commandViewState);
      const second = bounded.dispatchCommand(owner, fixture.actionInvocation, fixture.commandViewState);
      const overflow = bounded.dispatchCommand(owner, fixture.actionInvocation, fixture.commandViewState);
      await expect(overflow).rejects.toThrow("queue full");
      expect(await overflow.catch((error) => browserActorActionRefusalReasonV1(error))).toBe("queue-full");
      expect(boundedSent).toEqual([1]);
      expect(bounded.pending()).toBe(2);
      // 🛑️ `close` rejects the in-flight AND the queued waiter, once.
      const firstClosed = expect(first).rejects.toThrow("owner retired");
      const secondClosed = expect(second).rejects.toThrow("owner retired");
      bounded.close("browser-actor-action: owner retired");
      await firstClosed;
      await secondClosed;
      expect(await first.catch((error) => browserActorActionRefusalReasonV1(error))).toBe("owner-mismatch");
      expect(bounded.pending()).toBe(0);
      expect(bounded.settle({ ...fixture.acknowledged, actionSequence: 1 })).toBe(false);
    } finally {
      bounded.close("test cleanup");
    }
    expect(() => new BrowserActorActionMailboxV1(() => {}, 1000, 0)).toThrow("invalid capacity");
    expect(browserActorActionRefusalReasonV1(new Error("browser-actor-action: mailbox closed"))).toBe("dispatch-failed");
    expect(browserActorActionRefusalReasonV1(new Error("browser-actor-action: completion unconfirmed"))).toBe("dispatch-failed");
    expect(browserActorActionRefusalReasonV1(new Error("action-owner-mismatch"))).toBe("owner-mismatch");
    expect(browserActorActionRefusalReasonV1(new Error("action-busy"))).toBe("owner-mismatch");
    expect(browserActorActionRefusalReasonV1(new Error("action-state-unconfirmed"))).toBe("not-applied");
    expect(browserActorActionRefusalReasonV1(new Error("action-catching-up"))).toBe("catching-up");
    expect(browserActorActionRefusalReasonV1(new Error("action-refused"))).toBe("dispatch-failed");
    expect(browserActorActionRefusalReasonV1(new Error("action-command-ingress-refused"))).toBe("dispatch-failed");
    expect(browserActorActionRefusalReasonV1(new Error("action-guest-refused"))).toBeNull();
    expect(browserActorActionRefusalReasonV1(new Error("enqueue failed"))).toBeNull();
    expect(browserActorActionRefusalReasonV1(42)).toBeNull();
    const failing = new BrowserActorActionMailboxV1(() => { throw new Error("enqueue failed"); });
    await expect(failing.dispatchIntent(owner, "map", intent)).rejects.toThrow("enqueue failed");
    expect(failing.pending()).toBe(0);
    failing.close("test cleanup");
    const retried: number[] = [];
    const retry = new BrowserActorActionMailboxV1((request) => { retried.push(request.actionSequence); });
    try {
      await expect(retry.dispatchIntent(owner, "map", { ...intent, revision: 4 })).rejects.toThrow("stale surface");
      expect(retried).toEqual([]);
      const refused = expect(retry.dispatchIntent(owner, "map", intent)).rejects.toThrow(fixture.rejected.reason);
      expect(retried).toEqual([2]);
      expect(retry.settle({ ...fixture.rejected, actionSequence: 2 })).toBe(true);
      await refused;
      const next = retry.dispatchIntent(owner, "map", intent);
      expect(retried).toEqual([2, 3]);
      expect(retry.settle({ ...fixture.acknowledged, actionSequence: 3 })).toBe(true);
      await next;
    } finally {
      retry.close("test cleanup");
    }
    vi.useFakeTimers();
    const timedSent: number[] = [];
    const timed = new BrowserActorActionMailboxV1((request) => { timedSent.push(request.actionSequence); }, 10);
    try {
      // ⏱️ The completion timer starts when a request is SENT: the queued second request cannot expire while it
      // waits behind the first, and the first's expiry settles it alone and sends the next.
      const expired = expect(timed.dispatchIntent(owner, "map", intent)).rejects.toThrow("unconfirmed");
      const behind = timed.dispatchIntent(owner, "map", intent);
      expect(timedSent).toEqual([1]);
      await vi.advanceTimersByTimeAsync(9);
      expect(timedSent).toEqual([1]);
      await vi.advanceTimersByTimeAsync(1);
      await expired;
      expect(timedSent).toEqual([1, 2]);
      expect(timed.pending()).toBe(1);
      await vi.advanceTimersByTimeAsync(9);
      expect(timed.settle({ ...fixture.acknowledged, actionSequence: 2 })).toBe(true);
      expect((await behind).actionSequence).toBe(2);
      await vi.advanceTimersByTimeAsync(10);
      expect(timed.pending()).toBe(0);
      const later = timed.dispatchIntent(owner, "map", intent);
      expect(timedSent).toEqual([1, 2, 3]);
      expect(timed.settle({ ...fixture.acknowledged, actionSequence: 3 })).toBe(true);
      await later;
    } finally {
      timed.close("test cleanup");
      vi.useRealTimers();
    }
  });

  it("preserves the complete browser UI intent and its unsigned sequence above JSON precision", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧬️schema/🔣️.json", source.url), "utf8"));
    const validRequest = semioSchemaAjvV1({ strict: true }).compile({ ...schema, $ref: "#/definitions/request" });
    const validIntent = semioSchemaAjvV1({ strict: true }).compile({ ...schema, $ref: "#/definitions/uiIntent" });

    const { createBrowserActorUiIntentRequestV1 } = await import("@semio-tech/framework/browser-bundle/action-handoff/intent");
    const { decodePackValue, packUInt } = await import("@semio-tech/framework/packs");
    const intent = { ...fixture.uiIntent, seq: BigInt(fixture.uiIntent.seq) };
    const request = createBrowserActorUiIntentRequestV1(fixture.request, "map", intent);
    expect(request.actionSequence).toBe(9);
    expect(equal(decodePackValue(new Uint8Array(request.payload.bytes)), { ...intent, surface: "0:map", revision: packUInt(3n), node: packUInt(42n), action: { ...intent.action, version: packUInt(1n) }, seq: packUInt(0xffffffffffffffffn) })).toBe(true);
    expect(intent.surface).toBe("map");
    expect(validIntent({ ...fixture.uiIntent, seq: "0" })).toBe(true);
    expect((decodePackValue(new Uint8Array(createBrowserActorUiIntentRequestV1(fixture.request, "map", { ...intent, seq: 0n }).payload.bytes)) as Readonly<Record<string, unknown>>).seq).toEqual(packUInt(0n));
    for (const extra of [{ surface: "other" }, { revision: 4 }, { node: -1 }, { seq: 0x10000000000000000n }]) expect(() => createBrowserActorUiIntentRequestV1(fixture.request, "map", { ...intent, ...extra })).toThrow();
    for (const extra of [{ activationGeneration: "18446744073709551616" }, { actionSequence: 9007199254740992 }, { surfaceRevision: 9007199254740992 }]) {
      expect(validRequest({ ...fixture.request, ...extra })).toBe(false);
      expect(() => createBrowserActorUiIntentRequestV1({ ...fixture.request, ...extra }, "map", intent)).toThrow();
    }
  });

  it("admits only exact direct browser intent owners on the private worker wire", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    const request = { ...fixture.request, clientInstanceId: "12345678-1234-4123-8123-123456789abc" };
    const response = { ...fixture.acknowledged, clientInstanceId: "12345678-1234-4123-8123-123456789abc" };
    expect(equal(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request)), request)).toBe(true);
    expect(equal(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response)), response)).toBe(true);
    for (const extra of [{ clientInstanceId: "" }, { authority: "forged" }, { command: [1] }, { surfaceRevision: 0 }, { payload: { kind: "ui-intent", bytes: [] } }]) {
      expect(() => decodeBackboneWorkerRequest(encodeBackboneWorkerRequest({ ...request, ...extra }))).toThrow();
    }
    for (const extra of [{ clientInstanceId: "" }, { authority: "forged" }, { outcome: "rejected" }, { actionSequence: 0 }]) {
      expect(() => decodeBackboneWorkerResponse(encodeBackboneWorkerResponse({ ...response, ...extra }))).toThrow();
    }
  });

  it("admits only closed semantically valid directory administration requests", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/📇️directory/🏛️administration-worker-wire-v1.json", source.url), "utf8")) as {
      valid: readonly BackboneWorkerRequest[];
      invalid: readonly { name: string; request: unknown }[];
    };
    const commandFixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🏛️administration/🛂️command-admission/🔣️.json", source.url), "utf8")) as {
      allowed: readonly { command: unknown }[];
      unrelated: readonly unknown[];
      malformed: readonly unknown[];
    };
    const directorySchema = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧬️schema/🔣️.json", source.url), "utf8"));
    const commandOracle = semioSchemaAjvV1({ strict: true }).addSchema(directorySchema).getSchema(`${directorySchema.$id}#/$defs/DirectoryCommand`)!;
    const wire = (request: unknown): Uint8Array => new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...encodePackValue(request)]);

    for (const request of fixture.valid) expect(equal(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request)), request)).toBe(true);
    for (const command of [...commandFixture.allowed.map((row) => row.command), ...commandFixture.unrelated]) {
      expect(commandOracle(command), JSON.stringify(commandOracle.errors)).toBe(true);
      const request = { kind: "directory-administration-submit", operationEpoch: 7, requestId: "9f8e7d6c5b4a39281706f5e4d3c2b1a0", command };
      const decoded = decodeBackboneWorkerRequest(wire(request));
      expect(equal(decoded, request)).toBe(true);
      expect(JSON.stringify((decoded as { command: unknown }).command)).toBe(JSON.stringify(command));
    }
    for (const row of fixture.invalid) expect(() => decodeBackboneWorkerRequest(wire(row.request)), row.name).toThrow();
    for (const command of commandFixture.malformed) {
      if (commandOracle(command)) continue;
      expect(() => decodeBackboneWorkerRequest(wire({ kind: "directory-administration-submit", operationEpoch: 7, requestId: "9f8e7d6c5b4a39281706f5e4d3c2b1a0", command }))).toThrow();
    }
    for (const request of [
      { kind: "directory-administration-open", operationEpoch: 1, spaceId: "s".repeat(257) },
      { kind: "directory-administration-refresh", operationEpoch: 1, cursor: "a".repeat(1025) },
    ]) expect(() => decodeBackboneWorkerRequest(wire(request))).toThrow();
  });

  const fromHex = (hex: string): Uint8Array => new Uint8Array(Buffer.from(hex, "hex"));

  it("retains one exact causal OpBinary through actor send and receive frames", () => {
    const batch = fromHex("01016d01640161000000017301aa016902bbcc03ffffffffffffffffff010500");
    const message = encodeBackboneMessage({ kind: "mutations", envelopes: batch });
    const clientInstanceId = "12345678-1234-4123-8123-123456789abc";
    const request = { kind: "send", documentId: "d", clientInstanceId, message: { kind: "documentBackbone", message } } as const;
    const response = { kind: "event", documentId: "d", clientInstanceId, event: { kind: "documentBackbone", message } } as const;
    const requestRound = decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request));
    const responseRound = decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response));
    expect(requestRound).toEqual(request);
    expect(responseRound).toEqual(response);
    expect(parseDocumentBackboneMessage((requestRound as typeof request).message.message).envelopes[0]?.timestamp.physical_ms).toBe(0xffff_ffff_ffff_ffffn);
  });

  it("encodes a retraction as the Rust store's exact vector and admits it only toward the store", () => {
    const toHex = (bytes: Uint8Array) => Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
    for (const [mutationIds, hex] of [[[], "01040001000c00"], [["a"], "010401016101000c010600"]] as const) {
      const encoded = encodeBackboneMessage({ kind: "retract", mutationIds });
      expect(toHex(encoded)).toBe(hex);
      expect(decodeBackboneMessage(encoded)).toEqual({ kind: "retract", mutationIds });
      expect(parseInboundDocumentBackboneMessage(encoded)).toEqual(encoded);
      expect(() => parseDocumentBackboneMessage(encoded)).toThrow("mutations required");
    }
    expect(() => decodeBackboneMessage(fromHex("01030001000c00"))).toThrow("member lanes are Rust-only");
  });

  it("rejects non-mutation, noncanonical and over-cap actor messages", () => {
    const genesis = encodeBackboneMessage({ kind: "genesis", pack: new Uint8Array() });
    const ack = encodeBackboneMessage({ kind: "ack", opIds: [] });
    const trailing = encodeBackboneMessage({ kind: "mutations", envelopes: fromHex("0000") });
    expect(() => parseDocumentBackboneMessage(genesis)).toThrow("mutations required");
    expect(() => parseDocumentBackboneMessage(ack)).toThrow("mutations required");
    expect(() => parseDocumentBackboneMessage(trailing)).toThrow("trailing-bytes");
    expect(() => parseDocumentBackboneMessage(new Uint8Array(262_145))).toThrow("hot byte limit");

    const malformedFields = encodePackValue({ kind: "send", documentId: "d", clientInstanceId: "12345678-1234-4123-8123-123456789abc", message: { kind: "documentBackbone", message: Array.from(encodeBackboneMessage({ kind: "mutations", envelopes: Uint8Array.of(0) })), legacy: [] } });
    expect(() => decodeBackboneWorkerRequest(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...malformedFields]))).toThrow("invalid document backbone message");
  });
});
