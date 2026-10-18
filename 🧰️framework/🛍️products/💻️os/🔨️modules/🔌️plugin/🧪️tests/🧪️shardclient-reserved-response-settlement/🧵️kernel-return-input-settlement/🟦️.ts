/** 🧵️ The kernel return input resident payload, settlement and framing tests. */
import { loadRetainedResidentSlotFixture, loadRetainedResidentPayloadFixture, loadRetainedResidentBuilderFixture, loadRetainedResidentEvidenceFixture, loadRetainedResidentReaderFixture, loadRetainedResidentPageFixture, loadResidentPageBindingFixture, loadResidentEvidenceCopiedFixture, loadResidentEvidenceCopiedSchema, loadResidentPageBindingSchema } from "../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🟦️.ts";
import { loadContentInputAuthorityFixture, loadContentInputPayloadFixture } from "../../../../../../../🔨️modules/🎠️kernel/🧪️tests/🧫️fixtures/🟦️.ts";
import type { OwnedUiResidentPayload, ShardClientSettlementTestKit, UiResidentSourceProof } from "../🧰️settlement-test-kit/🟦️.ts";
import type { CapturedReturnHelpers } from "../🛠️captured-return-helpers/🟦️.ts";

/** 🪡 Registers the kernel return input payload, builder admission, settlement and framing tests. */
export function registerKernelReturnInputSettlementTests(kit: ShardClientSettlementTestKit, helpers: CapturedReturnHelpers): void {
  const { OwnedKernelReturnContent, capturedReturnState, expect, fixtureHosts, fixtureResidentBuilder, fixtureResidentPayload, fixtureResidentPool, fixtureResidentScope, it, vi } = kit;
  const { deliveredInput } = helpers;
  it("OwnedKernelReturnInput admits its exact instance through the released nine-phase shared scope", async () => {
    const slot = await loadRetainedResidentSlotFixture();
    const { client, residentLedger, instance, source, response } = await deliveredInput();
    const pool = await fixtureResidentPool(client, residentLedger); const before = residentLedger.usage.data;
    const scope = await fixtureResidentScope(pool, residentLedger, instance); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response);
    scope.beginClose(); for (let turn = 0; turn < slot.firstChild.retirementTurns && !scope.terminalIsEmpty(); turn++) expect(scope.closeStep({ maxItems: 1, maxBytes: 4096 }).kind).not.toBe("rejected");
    expect(scope.terminalIsEmpty()).toBe(true); expect(residentLedger.usage.data).toEqual(before); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response); client.disposeAll();
  });
  it("OwnedKernelReturnInput settles the genuine payload in exact charged phases after operation revocation", async () => {
    const { OwnedKernelReturnInputField, OwnedKernelReturnPayloadDetachment } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
    const { OwnedUiResidentPayloadSourceRelease } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts");
    const fixture = await loadContentInputPayloadFixture(); const { produce } = await import("immer");
    for (const activationState of fixture.runtimeClose.activationStates) {
      const { client, residentLedger, instance, source, response, worker } = await deliveredInput(); const owner = fixtureHosts.get(instance)!;
      const input = new OwnedKernelReturnContent(source, owner, instance.activation, instance.lifetime!); for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 }); const field = input.field!;
      const pool = await fixtureResidentPool(client, residentLedger); const scope = await fixtureResidentScope(pool, residentLedger, instance); const baseline = residentLedger.usage.data;
      const payload = await fixtureResidentPayload(scope, residentLedger, field); let expected = { ...residentLedger.usage.data }; const fragment = field.fragment; const page = source.page; const posts = worker.sent.length;
      const captured: { proof: UiResidentSourceProof | null } = { proof: null };
      const originalDetach = OwnedKernelReturnInputField.prototype.detachResidentPayload;
      const detach = vi.spyOn(OwnedKernelReturnInputField.prototype, "detachResidentPayload").mockImplementation(function (this: typeof field, actual, proof, grant) {
        if (this !== field) return originalDetach.call(this, actual, proof, grant);
        expect(actual).toBe(payload); expect(OwnedUiResidentPayloadSourceRelease.matches(proof, payload, field)).toBe(true); captured.proof = proof;
        expect(originalDetach.call(this, actual, proof, { maxItems: 1, maxBytes: 63 })).toEqual({ kind: "blocked", items: 0, bytes: 0 });
        expect(Reflect.apply(originalDetach, this, [actual, {}, grant])).toEqual({ kind: "rejected", items: 0, bytes: 0 }); return originalDetach.call(this, actual, proof, grant);
      });
      if (activationState === "revoked") { instance.beginClose(); input.beginClose(); expect(() => instance.activation.assertActive()).toThrow("actor-activation.revoked"); }
      payload.beginClose();
      try {
        for (let index = 0; index < fixture.runtimeClose.close.length; index++) {
          const row = fixture.runtimeClose.close[index]!; const last = index === fixture.runtimeClose.close.length - 1;
          expect(payload.closeStep({ maxItems: 0, maxBytes: row.bytes })).toMatchObject({ kind: "blocked", items: 0, bytes: 0 }); expect(residentLedger.usage.data).toEqual(expected);
          expect(payload.closeStep({ maxItems: 1, maxBytes: row.bytes }), row.phase).toEqual({ kind: last ? "complete" : "pending", phase: row.phase, items: 1, bytes: row.bytes });
          const refund = fixture.runtimeClose.refunds.find(item => item.phase === row.phase); if (refund) expected = produce(expected, usage => { usage.bytes -= refund.released.bytes; usage.slots -= refund.released.slots; usage.owners -= refund.released.owners; }); expect(residentLedger.usage.data).toEqual(expected);
          const observation = field.residentPayloadDetachment;
          if (row.phase === "resident-payload-source-detach") { expect(OwnedKernelReturnPayloadDetachment.matches(observation, field, payload)).toBe(true); expect(field.residentPayload(scope)).toBeNull(); expect(field.settleResidentPayload(observation!, captured.proof!, { maxItems: 1, maxBytes: 64 })).toEqual({ kind: "rejected", items: 0, bytes: 0 }); }
          if (row.phase === "resident-payload-source-settle" || row.phase === "resident-payload-settle-observation") { expect(OwnedKernelReturnPayloadDetachment.matchesSettled(observation, payload)).toBe(true); expect(field.settleResidentPayload(observation!, captured.proof!, { maxItems: 1, maxBytes: 64 })).toEqual({ kind: "rejected", items: 0, bytes: 0 }); }
          expect(payload.terminalIsEmpty()).toBe(last); expect(input.field).toBe(field); expect(field.fragment).toBe(fragment); expect(source.page).toBe(page); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response);
        }
      } finally { detach.mockRestore(); }
      expect(expected).toEqual(baseline); expect(OwnedKernelReturnPayloadDetachment.matchesSettled(field.residentPayloadDetachment, payload)).toBe(false);
      expect(field.installResidentPayload(payload, { maxItems: 1, maxBytes: 64 })).toEqual({ kind: "rejected", items: 0, bytes: 0 }); expect(field.complete).toBe(fixture.runtimeClose.sourceComplete);
      expect("acknowledgeInput" in source).toBe(fixture.runtimeClose.rawInputAck); expect(worker.sent.length).toBe(posts); client.disposeAll();
    }
  });
  it("OwnedKernelReturnInput abandons a genuine unbound payload only after clean source refusal", async () => {
    const { OwnedKernelReturnInputField, OwnedKernelReturnPayloadDetachment } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
    const { OwnedUiResidentPayloadSourceRelease } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts");
    const fixture = await loadContentInputPayloadFixture(); const ui = await loadRetainedResidentPayloadFixture();
    for (const driver of fixture.abandonment.drivers) {
    const { client, residentLedger, instance, source, response } = await deliveredInput(); const owner = fixtureHosts.get(instance)!;
    const input = new OwnedKernelReturnContent(source, owner, instance.activation, instance.lifetime!); for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 }); const field = input.field!;
    const pool = await fixtureResidentPool(client, residentLedger); const scope = await fixtureResidentScope(pool, residentLedger, instance); const before = residentLedger.usage.data;
    for (const bytes of ui.admissionBytes.slice(0, fixture.abandonment.admissionPrefix)) expect(scope.beginPayload(field, { maxItems: 1, maxBytes: bytes }).step).toMatchObject({ kind: "pending", bytes });
    const captured: { payload: OwnedUiResidentPayload | null; proof: UiResidentSourceProof | null } = { payload: null, proof: null }; const install = OwnedKernelReturnInputField.prototype.installResidentPayload;
    const attempted = vi.spyOn(OwnedKernelReturnInputField.prototype, "installResidentPayload").mockImplementation(function (this: typeof field, payload, grant) { if (this === field) captured.payload = payload; return install.call(this, payload, grant); });
    field.beginClose(); try { expect(scope.beginPayload(field, { maxItems: 1, maxBytes: 64 }).step).toMatchObject({ kind: "rejected", items: 0, bytes: 0 }); } finally { attempted.mockRestore(); }
    const payload = captured.payload; if (!payload) throw new Error("Original refused payload was not retained at actual source attempt");
    expect(field.residentPayload(scope)).toBeNull(); expect(field.residentPayloadDetachment).toBeNull(); const freeze = Object.freeze;
    const mint = vi.spyOn(Object, "freeze").mockImplementation(value => { if (value instanceof OwnedUiResidentPayloadSourceRelease) captured.proof = value; return freeze(value); });
    payload.beginClose(); try { expect(payload.closeStep({ maxItems: 1, maxBytes: fixture.abandonment.witnessGrantBytes })).toEqual({ kind: "pending", phase: "resident-payload-witness", items: 1, bytes: fixture.abandonment.witnessConstructionBytes }); } finally { mint.mockRestore(); }
    expect(payload.closeStep({ maxItems: 1, maxBytes: fixture.abandonment.bodyProofBytes })).toMatchObject({ kind: "pending", phase: "resident-payload-body-proof" });
    const proof = captured.proof; if (!OwnedUiResidentPayloadSourceRelease.matches(proof, payload, field)) throw new Error("Actual body-empty witness was not issued");
    expect(field.detachResidentPayload(payload, proof, { maxItems: 1, maxBytes: 63 })).toEqual({ kind: "blocked", items: 0, bytes: 0 });
    expect(Reflect.apply(field.detachResidentPayload, field, [payload, {}, { maxItems: 1, maxBytes: 64 }])).toEqual({ kind: "rejected", items: 0, bytes: 0 });
    if (driver === "source") expect(field.detachResidentPayload(payload, proof, { maxItems: 1, maxBytes: 64 })).toEqual({ kind: "pending", items: 1, bytes: 64 });
    else expect(payload.closeStep({ maxItems: 1, maxBytes: 64 })).toEqual({ kind: "pending", phase: "resident-payload-source-detach", items: 1, bytes: 64 });
    const observation = field.residentPayloadDetachment; expect(OwnedKernelReturnPayloadDetachment.matches(observation, field, payload)).toBe(true);
    expect(field.settleResidentPayload(observation!, proof, { maxItems: 1, maxBytes: 64 })).toEqual({ kind: "rejected", items: 0, bytes: 0 });
    for (const row of fixture.runtimeClose.close.slice(3)) expect(payload.closeStep({ maxItems: 1, maxBytes: row.bytes }), row.phase).toEqual({ kind: row.phase === "resident-payload-slot-close" ? "complete" : "pending", phase: row.phase, items: 1, bytes: row.bytes });
    expect(payload.terminalIsEmpty()).toBe(true); expect(residentLedger.usage.data).toEqual(before); expect(field.residentPayloadDetachment).toBe(observation);
    expect(field.installResidentPayload(payload, { maxItems: 1, maxBytes: 64 })).toEqual({ kind: "rejected", items: 0, bytes: 0 }); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response); client.disposeAll();
    }
  });
  it("OwnedKernelReturnInput admits its original builder through thirteen independent shared-ledger phases", async () => {
    const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🟦️.ts");
    const { client, residentLedger, instance, source, response, worker } = await deliveredInput(); const owner = fixtureHosts.get(instance)!;
    const input = new OwnedKernelReturnContent(source, owner, instance.activation, instance.lifetime!); for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 }); const field = input.field!;
    const pool = await fixtureResidentPool(client, residentLedger); const scope = await fixtureResidentScope(pool, residentLedger, instance); const resident = await fixtureResidentPayload(scope, residentLedger, field); const posts = worker.sent.length;
    const admitted = await fixtureResidentBuilder(residentLedger, field, resident); const builder = admitted.builder; expect(admitted.step.kind).toBe("ready"); expect(builder).not.toBeNull();
    if (!builder) throw new Error("Original builder was not admitted"); expect(OwnedUiOperationPayloadBuilder.healthy(builder)).toBe(true); expect(OwnedUiOperationPayloadBuilder.empty(builder)).toBe(false);
    expect(worker.sent.length).toBe(posts); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response); expect(field.consumed).toBe(0n); expect(field.fragment?.field).toBe(field);
    input.beginClose(); builder.beginClose(); expect(OwnedUiOperationPayloadBuilder.empty(builder)).toBe(false); client.disposeAll();
  });
  it("OwnedKernelReturnInput privately identifies its bound builder after the real bind call throws", async () => {
    const { OwnedKernelReturnContent, OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
    const { OwnedUiResidentPool } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts");
    const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🟦️.ts");
    
    const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");
    const { client, residentLedger, instance, source, response, vector } = await deliveredInput();
    
    const owner = fixtureHosts.get(instance)!; const activation = instance.activation; const lifetime = instance.lifetime!;
    const input = new OwnedKernelReturnContent(source, owner, activation, lifetime);
    for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 });
    const field = input.field!; const fragment = field.fragment!;
    const pool = await fixtureResidentPool(client, residentLedger);
    const resident = await fixtureResidentPayload(await fixtureResidentScope(pool, residentLedger, instance), residentLedger, field);
    expect(OwnedKernelReturnInputField.matchesBuilder(field, null)).toBe(vector.binding.nullBuilder);
    expect(OwnedKernelReturnInputField.matchesBuilder(field, {})).toBe(vector.binding.before);
    const captured: { builder: object | null; bound: boolean } = { builder: null, bound: false };
    const bind = OwnedKernelReturnInputField.prototype.bind;
    const intercept = vi.spyOn(OwnedKernelReturnInputField.prototype, "bind").mockImplementation(function (this: typeof field, builder) {
      captured.builder = builder; captured.bound = bind.call(this, builder);
      if (captured.bound) throw new Error("fixture after actual native binding"); return false;
    });
    try {
      const admission = await fixtureResidentBuilder(residentLedger, field, resident);
      expect(admission.builder).toBeNull(); expect(admission.step.kind).toBe("rejected");
    } finally { intercept.mockRestore(); }
    expect(captured.bound).toBe(true); expect(captured.builder).not.toBeNull();
    expect(OwnedKernelReturnInputField.matchesBuilder(field, captured.builder)).toBe(vector.binding.afterBindThenThrow);
    const observed = produce({ bound: vector.binding.before }, state => { state.bound = captured.bound; });
    expect(OwnedKernelReturnInputField.matchesBuilder(field, captured.builder)).toBe(observed.bound);
    field.beginClose(); expect(OwnedKernelReturnInputField.matchesBuilder(field, captured.builder)).toBe(vector.binding.afterBeginClose);
    expect(OwnedKernelReturnInputField.matchesBuilder(field, {})).toBe(vector.binding.foreignBuilder);
    expect(OwnedKernelReturnInputField.matchesBuilder(field, null)).toBe(vector.binding.nullBuilder);
    let reads = 0; const forged = { get builder() { reads++; return captured.builder; } };
    expect(OwnedKernelReturnInputField.matchesBuilder(forged, captured.builder)).toBe(vector.binding.forgedField);
    expect(OwnedKernelReturnInputField.matchesBuilder(Object.create(OwnedKernelReturnInputField.prototype), captured.builder)).toBe(false);
    expect(OwnedKernelReturnInputField.matchesBuilder(new Proxy(field, { has() { reads++; return true; }, get() { reads++; return captured.builder; } }), captured.builder)).toBe(false);
    expect(reads).toBe(vector.binding.publicReads); expect(field.fragment).toBe(fragment);
    expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response); client.disposeAll();
  });
  it("OwnedKernelReturnInput advances no framing on unread or genuinely cancelled fragments", async () => {
    const { OwnedKernelReturnContent } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
    const { OwnedUiOperationInputCancelled } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🟦️.ts");
    const evidenceFixture = await loadRetainedResidentEvidenceFixture();
    const builderFixture = await loadRetainedResidentBuilderFixture();
    const payloadFixture = await loadRetainedResidentPayloadFixture();
    const scopeFixture = await loadRetainedResidentSlotFixture();
    
    const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");
    const { client, residentLedger, instance, source, response, vector, payload } = await deliveredInput();
    
    const owner = fixtureHosts.get(instance)!; const activation = instance.activation; const lifetime = instance.lifetime!;
    const input = new OwnedKernelReturnContent(source, owner, activation, lifetime);
    for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 });
    const field = input.field!; const fragment = field.fragment!;
    const pool = await fixtureResidentPool(client, residentLedger);
    const scope = await fixtureResidentScope(pool, residentLedger, instance); const resident = await fixtureResidentPayload(scope, residentLedger, field);
    const builder = (await fixtureResidentBuilder(residentLedger, field, resident)).builder!;
    expect(builder).not.toBeNull();
    expect(Buffer.from(Array.from({ length: fragment.length }, (_, index) => fragment.byteAt(index, builder)))).toEqual(payload);
    expect(field.advance({ maxItems: 1, maxBytes: 4096 }, builder)).toEqual({ kind: vector.continuation.beforeProof, items: 0, bytes: 0 });
    expect(field.consumed).toBe(0n); expect(field.complete).toBe(false);
    const baseline = pool.usage;
    builder.beginClose();
    for (const [index, bytes] of evidenceFixture.grants.entries()) {
      const current = resident.beginEvidence(builder, { maxItems: 1, maxBytes: bytes });
      expect(current.step).toMatchObject({ kind: index === evidenceFixture.grants.length - 1 ? "ready" : "pending", items: 1, bytes });
      if (current.evidence) expect(OwnedUiOperationInputCancelled.matches(current.evidence, fragment, field, builder, fragment.offset, fragment.length)).toBe(true);
    }
    expect(pool.usage).toEqual(produce(baseline, value => { for (const axis of ["bytes", "slots", "owners"] as const) value[axis] += evidenceFixture.total[axis]; }));
    const evidenceTurns = [...evidenceFixture.retirement.grants, ...evidenceFixture.retirement.registrationGrants];
    for (const [index, bytes] of evidenceTurns.entries()) expect(resident.advanceEvidence(builder, { maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: index === evidenceTurns.length - 1 ? "complete" : "pending", items: 1, bytes });
    expect(builder.terminalIsEmpty()).toBe(false); expect(pool.usage).toEqual(baseline);
    expect(field.advance({ maxItems: 1, maxBytes: 4096 }, builder)).toEqual({ kind: vector.continuation.afterCancellation, items: 0, bytes: 0 });
    expect(field.consumed.toString()).toBe(vector.continuation.cancelledConsumed); expect(field.complete).toBe(vector.continuation.cancelledComplete);
    expect(field.fragment === null).toBe(vector.cancellation.fragmentDetached); expect(() => fragment.byteAt(0, builder)).toThrow();
    resident.beginClose();
    for (const bytes of builderFixture.bindingGrants) expect(resident.closeStep({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", items: 1, bytes });
    expect(builder.terminalIsEmpty()).toBe(vector.cancellation.builderDetached);
    expect(field.advance({ maxItems: 1, maxBytes: 4096 }, builder)).toEqual({ kind: vector.cancellation.afterBindingRelease, items: 0, bytes: 0 });
    expect(pool.usage).toEqual(produce(baseline, value => { for (const axis of ["bytes", "slots", "owners"] as const) value[axis] -= builderFixture.total[axis]; }));
    const payloadTurns = payloadFixture.sourceReleaseTurns.length + vector.cancellation.payloadRegistrationTurns.length;
    for (let turn = 0; turn < payloadTurns; turn++) expect(resident.closeStep({ maxItems: 1, maxBytes: 4096 }).kind).toBe(turn === payloadTurns - 1 ? "complete" : "pending");
    expect(resident.terminalIsEmpty()).toBe(true);
    expect(pool.usage).toEqual(produce(baseline, value => { for (const axis of ["bytes", "slots", "owners"] as const) value[axis] -= builderFixture.total[axis] + payloadFixture.expectedPayloadTotal[axis]; }));
    expect("acknowledgeInput" in source).toBe(vector.continuation.pageInputAck);
    expect(source.page).not.toBeNull(); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response);
    pool.beginClose(); for (let turn = 0; turn < scopeFixture.firstChild.retirementTurns + vector.cancellation.poolOwnTurns && !pool.terminalIsEmpty(); turn++) { const current = pool.closeStep({ maxItems: 1, maxBytes: 4096 }); expect(current.kind, current.phase).not.toBe("blocked"); expect(current.kind, current.phase).not.toBe("rejected"); }
    expect(pool.terminalIsEmpty()).toBe(true); expect(input.field).toBe(field); client.disposeAll();
  });
  it("OwnedKernelReturnInput consumes only privately copied bytes and retains the containing raw page", async () => {
    const { OwnedKernelReturnInputFragment } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
    const { OwnedUiOperationInputCopied } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🟦️.ts");
    const vector = await loadContentInputAuthorityFixture();
    const readers = await loadRetainedResidentReaderFixture();
    const pages = await loadRetainedResidentPageFixture();
    const binding = await loadResidentPageBindingFixture();
    const evidence = await loadRetainedResidentEvidenceFixture();
    const copied = await loadResidentEvidenceCopiedFixture();
    const builders = await loadRetainedResidentBuilderFixture();
    const payloads = await loadRetainedResidentPayloadFixture();
    const scopes = await loadRetainedResidentSlotFixture();
    const copiedSchema = await loadResidentEvidenceCopiedSchema();
    const { default: Ajv } = await import("ajv"); const { Buffer } = await import("node:buffer"); const { produce } = await import("immer");
    
    for (const length of vector.continuation.copiedPayloadBytes) {
      const { client, residentLedger, instance, source, response, payload, worker } = await deliveredInput(undefined, length);
      const input = new OwnedKernelReturnContent(source, fixtureHosts.get(instance)!, instance.activation, instance.lifetime!);
      for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 });
      const field = input.field!; const fragment = field.fragment!; const rawPage = source.page; const posts = worker.sent.length;
      const pool = await fixtureResidentPool(client, residentLedger); const scope = await fixtureResidentScope(pool, residentLedger, instance); const baseline = pool.usage;
      const resident = await fixtureResidentPayload(scope, residentLedger, field); const builder = (await fixtureResidentBuilder(residentLedger, field, resident)).builder!;
      let admission = resident.beginReader(builder, { maxItems: 0, maxBytes: 4096 }); expect(admission.step.kind).toBe("blocked");
      for (const [index, bytes] of readers.admissionGrants.entries()) { admission = resident.beginReader(builder, { maxItems: 1, maxBytes: bytes }); expect(admission.step, readers.admissionPhases[index]).toMatchObject({ kind: index === readers.admissionGrants.length - 1 ? "ready" : "pending", bytes }); }
      const reader = admission.reader; if (!reader) throw new Error("Original early reader was not admitted");
      expect(builder.beginRead({ maxItems: 1, maxBytes: 4096 }).reader).toBe(reader); expect(field.complete).toBe(false); expect(reader.advance({ maxItems: 1, maxBytes: 4096 }).kind).toBe("blocked");
      const advance = (bytes: number) => { const current = builder.advance({ maxItems: 1, maxBytes: bytes }); expect(current, current.phase).toMatchObject({ kind: "pending", bytes }); };
      const readPending = (bytes: number) => { const current = reader.advance({ maxItems: 1, maxBytes: bytes }); if (current.kind === "byte") throw new Error("Unexpected byte during reader ownership phase"); expect(current, current.phase).toMatchObject({ kind: "pending", bytes }); };
      const beforeWindows = pool.usage; const actual: number[] = []; advance(binding.producer.fragmentCapture);
      for (let offset = 0; offset < length; offset += pages.wholeField.pageLength) {
        const count = Math.min(pages.wholeField.pageLength, length - offset);
        for (const bytes of [...(offset === 0 ? pages.storageOwnerGrants : []), ...binding.admissionGrants, binding.producer.pageObservation, binding.producer.allocation, binding.producer.allocationObservation]) advance(bytes);
        for (let index = 0; index < count; index++) { advance(binding.producer.sourceRead); advance(binding.producer.destinationWrite); }
        advance(binding.producer.seal); advance(binding.producer.sealObservation);
        readPending(64); for (const bytes of readers.aliasGrants) readPending(bytes);
        for (let index = 0; index < count; index++) { const current = reader.advance({ maxItems: 1, maxBytes: 1 }); expect(current.kind).toBe("byte"); if (current.kind !== "byte") throw new Error("Original window byte was not returned"); actual.push(current.value); }
        readPending(64); for (const bytes of [...readers.aliasCloseGrants, ...binding.closeGrants, 64]) readPending(bytes);
        expect(pool.usage).toEqual(produce(beforeWindows, value => { for (const axis of ["bytes", "slots", "owners"] as const) value[axis] += pages.storageOwnerTotal[axis]; }));
        expect(field.consumed).toBe(0n); expect(field.complete).toBe(false); expect(reader.advance({ maxItems: 1, maxBytes: 4096 }).kind).toBe("blocked");
      }
      expect(Buffer.from(actual)).toEqual(payload); advance(binding.producer.inputDetach);
      for (const bytes of evidence.grants) advance(bytes);
      let proof: unknown = null; const original = OwnedKernelReturnInputFragment.prototype.release;
      const release = vi.spyOn(OwnedKernelReturnInputFragment.prototype, "release").mockImplementation(function (this: typeof fragment, token) { expect(this).toBe(fragment); proof = token; return original.call(this, token); });
      try { for (const bytes of evidence.retirement.grants.slice(0, 2)) advance(bytes); } finally { release.mockRestore(); }
      expect(OwnedUiOperationInputCopied.matches(proof, fragment, field, builder, fragment.offset, fragment.length)).toBe(true);
      expect(field.consumed).toBe(0n); expect(field.complete).toBe(false); expect(() => fragment.byteAt(0, builder)).toThrow();
      expect(field.advance({ maxItems: 1, maxBytes: 4096 }, {})).toEqual({ kind: vector.driver.foreignAfterCopy, items: 0, bytes: 0 });
      expect(field.consumed.toString()).toBe(vector.driver.foreignConsumed); const unchanged = field.consumed; input.advance({ maxItems: 1, maxBytes: 4096 });
      expect(field.consumed !== unchanged).toBe(vector.driver.contentDrivesField);
      expect(field.advance({ maxItems: 0, maxBytes: 4096 }, builder).kind).toBe("blocked"); expect(field.advance({ maxItems: 1, maxBytes: 0 }, builder).kind).toBe("blocked");
      const sourceTurns = length === 0 ? copied.empty.sourceSteps : length;
      for (let index = 0; index < sourceTurns; index++) {
        const current = builder.advance({ maxItems: 1, maxBytes: copied.sourceStepBytes });
        expect(current).toMatchObject({ kind: "pending", phase: "paged-evidence-source-advance", bytes: length === 0 ? copied.empty.sourceStepBytes : copied.sourceStepBytes });
        expect(current.items).toBeLessThanOrEqual(vector.continuation.maximumItems); expect(field.consumed).toBe(BigInt(length === 0 ? 0 : index + 1));
        expect(builder.advance({ maxItems: 1, maxBytes: copied.sourceObservationBytes - 1 })).toMatchObject({ kind: "blocked", bytes: 0 });
        advance(copied.sourceObservationBytes); expect(reader.advance({ maxItems: 1, maxBytes: 4096 }).kind).toBe("blocked");
      }
      for (const bytes of [...copied.remainingRetirementGrants, ...evidence.retirement.registrationGrants, copied.rangeObservationBytes]) advance(bytes);
      expect(builder.advance({ maxItems: 1, maxBytes: 4096 })).toMatchObject({ kind: "ready", bytes: 0 }); expect(builder.failure).toBeNull();
      expect(field.complete).toBe(vector.continuation.copiedComplete); expect(field.consumed).toBe(BigInt(length)); expect(field.fragment).toBeNull();
      expect(field.advance({ maxItems: 1, maxBytes: 4096 }, builder)).toEqual({ kind: "complete", items: 0, bytes: 0 }); expect(reader.advance({ maxItems: 1, maxBytes: 4096 })).toMatchObject({ kind: "complete", bytes: 0 });
      expect(input.field).toBe(field); expect(source.page).toBe(rawPage); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response);
      expect("acknowledgeInput" in source).toBe(vector.continuation.pageInputAck); expect(worker.sent.length).toBe(posts);
      resident.beginClose(); const closeTurns = readers.closing.grants.length + builders.bindingGrants.length + payloads.sourceReleaseTurns.length + vector.cancellation.payloadRegistrationTurns.length + (length === 0 ? 0 : pages.cancellation.storageCloseGrants.length);
      for (let turn = 0; turn < closeTurns && !resident.terminalIsEmpty(); turn++) { const current = resident.closeStep({ maxItems: 1, maxBytes: 4096 }); expect(current.kind, current.phase).not.toBe("blocked"); expect(current.kind, current.phase).not.toBe("rejected"); }
      expect(reader.terminalIsEmpty()).toBe(true); expect(builder.terminalIsEmpty()).toBe(true); expect(resident.terminalIsEmpty()).toBe(true); expect(pool.usage).toEqual(baseline);
      pool.beginClose(); for (let turn = 0; turn < scopes.firstChild.retirementTurns + vector.cancellation.poolOwnTurns && !pool.terminalIsEmpty(); turn++) { const current = pool.closeStep({ maxItems: 1, maxBytes: 4096 }); expect(current.kind, current.phase).not.toBe("blocked"); expect(current.kind, current.phase).not.toBe("rejected"); }
      expect(pool.terminalIsEmpty()).toBe(true); expect(source.page).toBe(rawPage); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response); expect(worker.sent.length).toBe(posts); client.disposeAll();
    }
  });
  it("OwnedKernelReturnInput stops at the exact copied page boundary without fabricating a next range", async () => {
    const vector = await loadContentInputAuthorityFixture();
    const readers = await loadRetainedResidentReaderFixture();
    const pages = await loadRetainedResidentPageFixture();
    const binding = await loadResidentPageBindingFixture();
    const evidence = await loadRetainedResidentEvidenceFixture();
    const copied = await loadResidentEvidenceCopiedFixture();
    const builders = await loadRetainedResidentBuilderFixture();
    const payloads = await loadRetainedResidentPayloadFixture();
    const scopes = await loadRetainedResidentSlotFixture();
    const bindingSchema = await loadResidentPageBindingSchema();
    const { default: Ajv } = await import("ajv"); const { Buffer } = await import("node:buffer");
    
    const { client, residentLedger, instance, source, response, payload, worker } = await deliveredInput(undefined, vector.crossPage.payloadBytes, vector.crossPage.firstPageBytes);
    const input = new OwnedKernelReturnContent(source, fixtureHosts.get(instance)!, instance.activation, instance.lifetime!);
    for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 });
    const field = input.field!; const fragment = field.fragment!; const rawPage = source.page; const posts = worker.sent.length;
    const pool = await fixtureResidentPool(client, residentLedger); const scope = await fixtureResidentScope(pool, residentLedger, instance); const baseline = pool.usage;
    const resident = await fixtureResidentPayload(scope, residentLedger, field); const builder = (await fixtureResidentBuilder(residentLedger, field, resident)).builder!;
    let admission = resident.beginReader(builder, { maxItems: 0, maxBytes: 4096 }); expect(admission.step.kind).toBe("blocked");
    for (const [index, bytes] of readers.admissionGrants.entries()) { admission = resident.beginReader(builder, { maxItems: 1, maxBytes: bytes }); expect(admission.step, readers.admissionPhases[index]).toMatchObject({ kind: index === readers.admissionGrants.length - 1 ? "ready" : "pending", bytes }); }
    const reader = admission.reader; if (!reader) throw new Error("Original boundary reader was not admitted");
    const advance = (bytes: number) => { const current = builder.advance({ maxItems: 1, maxBytes: bytes }); expect(current, current.phase).toMatchObject({ kind: "pending", bytes }); };
    const readPending = (bytes: number) => { const current = reader.advance({ maxItems: 1, maxBytes: bytes }); if (current.kind === "byte") throw new Error("Unexpected byte during reader ownership phase"); expect(current, current.phase).toMatchObject({ kind: "pending", bytes }); };
    const actual: number[] = []; advance(binding.producer.fragmentCapture);
    const wholeWindows = Math.floor(fragment.length / pages.wholeField.pageLength);
    for (let offset = 0; offset < fragment.length; offset += pages.wholeField.pageLength) {
      const count = Math.min(pages.wholeField.pageLength, fragment.length - offset);
      for (const bytes of [...(offset === 0 ? pages.storageOwnerGrants : []), ...binding.admissionGrants, binding.producer.pageObservation, binding.producer.allocation, binding.producer.allocationObservation]) advance(bytes);
      for (let index = 0; index < count; index++) { advance(binding.producer.sourceRead); advance(binding.producer.destinationWrite); }
      if (count === pages.wholeField.pageLength) {
        advance(binding.producer.seal); advance(binding.producer.sealObservation); readPending(64); for (const bytes of readers.aliasGrants) readPending(bytes);
        for (let index = 0; index < count; index++) { const current = reader.advance({ maxItems: 1, maxBytes: 1 }); expect(current.kind).toBe("byte"); if (current.kind !== "byte") throw new Error("Original boundary window byte was not returned"); actual.push(current.value); }
        readPending(64); for (const bytes of [...readers.aliasCloseGrants, ...binding.closeGrants, 64]) readPending(bytes);
      }
      expect(field.consumed).toBe(0n); expect(field.complete).toBe(false); expect(reader.advance({ maxItems: 1, maxBytes: 4096 }).kind).toBe("blocked");
    }
    expect(actual.length).toBe(wholeWindows * pages.wholeField.pageLength); expect(Buffer.from(actual)).toEqual(payload.subarray(0, actual.length));
    advance(binding.producer.inputDetach); for (const bytes of [...evidence.grants, ...evidence.retirement.grants.slice(0, 2)]) advance(bytes);
    expect(() => fragment.byteAt(0, builder)).toThrow(); expect(field.consumed).toBe(0n);
    for (let index = 0; index < fragment.length; index++) {
      expect(builder.advance({ maxItems: 1, maxBytes: copied.sourceStepBytes })).toMatchObject({ kind: "pending", bytes: copied.sourceStepBytes, phase: "paged-evidence-source-advance" });
      expect(field.consumed).toBe(BigInt(index + 1)); advance(copied.sourceObservationBytes); expect(field.complete).toBe(false);
    }
    for (const bytes of [...copied.remainingRetirementGrants, ...evidence.retirement.registrationGrants, copied.rangeObservationBytes]) advance(bytes);
    expect(builder.advance({ maxItems: 1, maxBytes: 4096 })).toMatchObject({ kind: "blocked", bytes: 0, phase: "paged-source-continuation" }); expect(builder.failure).toBeNull();
    expect(field.consumed).toBe(BigInt(fragment.length)); expect(field.complete).toBe(vector.continuation.pageBoundaryComplete); expect(field.fragment).toBeNull();
    expect(field.advance({ maxItems: 1, maxBytes: 4096 }, builder)).toEqual({ kind: "blocked", items: 0, bytes: 0 }); expect(builder.beginRead({ maxItems: 1, maxBytes: 4096 }).reader).toBe(reader); expect(reader.advance({ maxItems: 1, maxBytes: 4096 }).kind).toBe("blocked");
    const unchanged = field.consumed; input.advance({ maxItems: 1, maxBytes: 4096 }); expect(field.consumed).toBe(unchanged); expect(field.fragment).toBeNull();
    expect(source.page).toBe(rawPage); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response); expect("acknowledgeInput" in source).toBe(vector.continuation.pageInputAck); expect(worker.sent.length).toBe(posts);
    resident.beginClose(); const partialPage = fragment.length % pages.wholeField.pageLength !== 0;
    const closeTurns = readers.closing.grants.length + (partialPage ? binding.closeGrants.length : 0) + builders.bindingGrants.length + payloads.sourceReleaseTurns.length + vector.cancellation.payloadRegistrationTurns.length + pages.cancellation.storageCloseGrants.length;
    for (let turn = 0; turn < closeTurns && !resident.terminalIsEmpty(); turn++) { const current = resident.closeStep({ maxItems: 1, maxBytes: 4096 }); expect(current.kind, current.phase).not.toBe("blocked"); expect(current.kind, current.phase).not.toBe("rejected"); expect(field.consumed).toBe(unchanged); }
    expect(reader.terminalIsEmpty()).toBe(true); expect(builder.terminalIsEmpty()).toBe(true); expect(resident.terminalIsEmpty()).toBe(true); expect(pool.usage).toEqual(baseline);
    pool.beginClose(); for (let turn = 0; turn < scopes.firstChild.retirementTurns + vector.cancellation.poolOwnTurns && !pool.terminalIsEmpty(); turn++) { const current = pool.closeStep({ maxItems: 1, maxBytes: 4096 }); expect(current.kind, current.phase).not.toBe("blocked"); expect(current.kind, current.phase).not.toBe("rejected"); }
    expect(pool.terminalIsEmpty()).toBe(true); expect(field.complete).toBe(false); expect(source.page).toBe(rawPage); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response); expect(worker.sent.length).toBe(posts); client.disposeAll();
  });
}
