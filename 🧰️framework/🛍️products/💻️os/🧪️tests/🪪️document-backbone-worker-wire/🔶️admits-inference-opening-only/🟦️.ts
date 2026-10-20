/** 🔶️ The admits inference opening only tests of the document backbone worker wire suite. */
import { expect, it } from "vitest";
import { decodeBackboneWorkerResponse, encodeBackboneWorkerResponse } from "@semio-tech/framework/backbone";
import { encodePackValue } from "@semio-tech/framework/packs";
import { source } from "../../🧮️directory-client-helpers/🟦️.ts";

/** 🔷 Registers the admits inference opening only tests of the document backbone worker wire suite. */
export function registerAdmitsInferenceOpeningOnlyTests(): void {
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
}
