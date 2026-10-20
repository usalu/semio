/** 🔗️ The kernel return input page, content, builder binding and evidence tests. */
import { loadContentInputAuthorityFixture, loadContentInputBuilderDocument, loadContentInputBuilderFixture, loadContentInputReleaseDocument, loadContentInputReleaseFixture, loadContentInputPayloadDocument, loadContentInputPayloadFixture } from "@semio-tech/framework-kernel/testing/fixtures";
import { OwnedKernelReturnContent } from "@semio-tech/framework-kernel/return/content/input";
import type { ShardClientSettlementTestKit } from "../🧰️settlement-test-kit/🟦️.ts";
import type { CapturedReturnHelpers } from "../🛠️captured-return-helpers/🟦️.ts";

/** ⛓ Registers the kernel return input page, content, builder binding and detach evidence tests. */
export function registerKernelReturnInputBindingTests(kit: ShardClientSettlementTestKit, helpers: CapturedReturnHelpers): void {
  const { OwnedShardReturnPage, OwnedUiInstance, capturedReturnState, expect, fixtureHosts, fixtureResidentBuilder, fixtureResidentPayload, fixtureResidentPool, fixtureResidentScope, it, testSource, vi } = kit;
  const { deliveredInput } = helpers;
  it("OwnedKernelReturnInput keeps decoded page storage behind its private facade", async () => {
    const { client, instance, source, response, report, vector } = await deliveredInput();
    expect(report.kind).toBe("page");
    expect("page" in report).toBe(vector.boundaries.publicDecodedPage);
    expect(Object.isFrozen(report)).toBe(true);
    expect(source.page?.byteAt(0)).toBe(0x73);
    expect(OwnedShardReturnPage.matchesOwner(source.page, fixtureHosts.get(instance)!, instance.activation, instance.lifetime!)).toBe(true);
    expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response);
    client.disposeAll();
  });
  it("OwnedKernelReturnInput captures one content owner and exact grammar-selected private field", async () => {
    const { OwnedKernelReturnContent, OwnedKernelReturnInputField, OwnedKernelReturnInputFragment } = await import("@semio-tech/framework-kernel/return/content/input");
    
    const { default: Ajv } = await import("ajv");
    const { client, instance, source, response, receipt, payload, vector } = await deliveredInput();
    
    const host = fixtureHosts.get(instance)!;
    const input = new OwnedKernelReturnContent(source, host, instance.activation, instance.lifetime!);
    expect(source.content).toBe(input);
    expect(() => new OwnedKernelReturnContent(source, host, instance.activation, instance.lifetime!)).toThrow(/content-owned/);
    expect(input.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked");
    for (let turn = 0; turn < 256 && input.field === null; turn++) {
      const step = input.advance({ maxItems: 1, maxBytes: 4096 });
      expect(step.items).toBeLessThanOrEqual(1); expect(step.bytes).toBeLessThanOrEqual(1);
    }
    const field = input.field!;
    expect(field.value).toEqual({ operation: 0, opcode: vector.opcode, node: BigInt(vector.node), name: vector.field, byteLength: BigInt(payload.length), receipt });
    expect(OwnedKernelReturnInputField.matchesOwner(field, host, instance.activation, instance.lifetime!)).toBe(true);
    expect(OwnedKernelReturnInputField.matchesOwner({ value: field.value }, host, instance.activation, instance.lifetime!)).toBe(false);
    expect(() => Reflect.construct(OwnedKernelReturnInputField, [field.value])).toThrow();
    const fragment = field.fragment!;
    expect(fragment.offset).toBe(0n); expect(fragment.length).toBe(payload.length); expect(fragment.field).toBe(field);
    expect(OwnedKernelReturnInputFragment.matches(fragment, field)).toBe(true);
    expect(OwnedKernelReturnInputFragment.matches({ field }, field)).toBe(false);
    expect(() => fragment.byteAt(0, {})).toThrow(/builder/);
    expect(fragment.release({})).toBeNull(); expect(field.fragment).toBe(fragment);
    input.beginClose(); expect(input.advance({ maxItems: 1, maxBytes: 4096 }).kind).toBe("blocked");
    expect(() => fragment.byteAt(0, {})).toThrow();
    expect(source.content).toBe(input); expect(input.field).toBe(field); expect(field.fragment).toBe(fragment);
    expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response); client.disposeAll();
  });
  it("OwnedKernelReturnInput refuses foreign concrete hosts and patch lifetime fields before field mint", async () => {
    const { OwnedKernelReturnContent } = await import("@semio-tech/framework-kernel/return/content/input");
    for (const key of ["activationGeneration", "instanceId", "guestLifetime"] as const) {
      const { client, instance, source, response } = await deliveredInput(key);
      const host = fixtureHosts.get(instance)!;
      const foreign = new OwnedUiInstance(instance.activation, instance.lifetime!, { maxNodes: 1, maxDepth: 1, maxChildren: 1, maxTextBytes: 16, maxPatchOps: 1, maxPatchBytes: 256 }, { usizeBits: 32 });
      expect(() => new OwnedKernelReturnContent(source, foreign, instance.activation, instance.lifetime!)).toThrow(/owner/);
      expect(source.content).toBeNull();
      const input = new OwnedKernelReturnContent(source, host, instance.activation, instance.lifetime!);
      for (let turn = 0; turn < 256 && input.failure === null; turn++) input.advance({ maxItems: 1, maxBytes: 4096 });
      expect(input.failure).toMatch(/patch-lifetime/); expect(input.field).toBeNull();
      expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response); client.disposeAll();
    }
  });
  it("OwnedKernelReturnInput bounds a large field to the exact currently captured page range", async () => {
    const { OwnedKernelReturnContent } = await import("@semio-tech/framework-kernel/return/content/input");
    const vector = await loadContentInputAuthorityFixture();
    const { client, instance, source, bytes, payload, response } = await deliveredInput(undefined, vector.crossPage.payloadBytes, vector.crossPage.firstPageBytes);
    const input = new OwnedKernelReturnContent(source, fixtureHosts.get(instance)!, instance.activation, instance.lifetime!);
    for (let turn = 0; turn < 256 && input.field === null; turn++) input.advance({ maxItems: 1, maxBytes: 4096 });
    const field = input.field!; const fragment = field.fragment!;
    const start = bytes.indexOf(payload);
    expect(start).toBeGreaterThan(0);
    expect(field.value.byteLength).toBe(BigInt(vector.crossPage.payloadBytes));
    expect(fragment.length).toBe(vector.crossPage.firstPageBytes - start);
    expect(fragment.length).toBeLessThanOrEqual(vector.crossPage.maximumFragmentBytes);
    expect(fragment.offset).toBe(0n);
    expect("acknowledgeInput" in source).toBe(vector.crossPage.inputAckBeforeCopy);
    expect(input.advance({ maxItems: 1, maxBytes: 4096 }).kind).toBe("ready");
    expect(field.fragment).toBe(fragment); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response);
    input.beginClose(); expect(field.fragment).toBe(fragment); client.disposeAll();
  });
  it("OwnedKernelReturnBuilderBinding validates two-way close traces with an independent state oracle", async () => {
    const contract = await loadContentInputBuilderDocument(); 
    const fixture = await loadContentInputBuilderFixture();
    const { default: Ajv } = await import("ajv"); const { produce } = await import("immer"); const validator = new Ajv({ strict: true });
     
    const price = (fields: number) => BigInt(contract.metadata.recordBytes) + BigInt(contract.metadata.fieldBytes) * BigInt(fields);
    expect(price(contract.fieldFields.length)).toBe(BigInt(contract.metadata.fieldBytesTotal)); expect(price(contract.witnessFields.length)).toBe(BigInt(contract.metadata.witnessBytes));
    for (const vector of fixture.traces) {
      let state = { ...vector.initial };
      for (const row of vector.trace) {
        let kind = "rejected"; let bytes = 0;
        state = produce(state, draft => {
          if (row.action.endsWith("-short")) { kind = "blocked"; return; }
          if (row.action === "body-retire") { draft.body = true; kind = "pending"; bytes = 64; }
          else if (row.action === "evidence-settled") { draft.evidence = false; kind = "pending"; bytes = 64; }
          else if (row.action === "operation-revoked") kind = "pending";
          else if (row.action === "detach" && (draft.phase === "bound" || draft.phase === "unbound") && draft.body && draft.observation && !draft.evidence) { draft.phase = "detached"; draft.binding = "originalWitness"; kind = "pending"; bytes = 64; }
          else if (row.action === "observe-detached-after-wrapper-fault" && draft.phase === "detached" && draft.observation) kind = "ready";
          else if (row.action === "ui-detach" && draft.phase === "detached" && draft.observation) { draft.uiSource = false; kind = "pending"; bytes = 64; }
          else if (row.action === "settle" && draft.phase === "detached" && !draft.uiSource && draft.observation) { draft.phase = "settled"; draft.binding = "consumedSentinel"; kind = "complete"; bytes = 64; }
          else if (row.action === "observe-settled-after-wrapper-fault" && draft.phase === "settled" && !draft.uiSource && draft.observation) kind = "ready";
          else if (row.action === "ui-forget" && draft.phase === "settled" && !draft.uiSource) { draft.observation = false; kind = "complete"; bytes = 64; }
        });
        const { evidence, ...visible } = state; expect({ kind, bytes, ...visible }, vector.mode + ":" + row.action).toEqual(row.expected);
      }
    }
    expect(contract.metadata.additionalPerFieldAllocations).toBe(0); expect(contract.metadata.sourceAdmitted).toBe(false); expect(fixture.readiness.runtimeBodyWitness).toBe(false);
  });
  it("OwnedKernelReturnBuilderBinding reuses the actual field word and original two-field UI witness", async () => {
    const contract = await loadContentInputBuilderDocument(); const ts = await import("typescript"); const { readFile } = await import("node:fs/promises");
    const text = await readFile(new URL("../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts", testSource.url), "utf8"); const input = ts.createSourceFile("input.ts", text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const uiText = await readFile(new URL("../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts", testSource.url), "utf8"); const ui = ts.createSourceFile("resident.ts", uiText, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const field = input.statements.find(item => ts.isClassDeclaration(item) && item.name?.text === "OwnedKernelReturnInputField"); const witness = ui.statements.find(item => ts.isClassDeclaration(item) && item.name?.text === "BuilderWitness");
    if (!field || !ts.isClassDeclaration(field) || !witness || !ts.isClassDeclaration(witness)) throw new Error("Actual private field/witness missing");
    expect(field.members.filter(ts.isPropertyDeclaration).map(item => item.name.getText(input).slice(1))).toEqual(contract.fieldFields); expect(witness.members.filter(ts.isPropertyDeclaration).map(item => item.name.getText(ui).slice(1))).toEqual(contract.witnessFields);
    for (const name of ["detachBuilder", "settleBuilder", "matchesBuilderDetached", "matchesBuilderSettled"]) { const method = field.members.find(item => ts.isMethodDeclaration(item) && item.name.getText(input) === name); expect(method, name).toBeDefined(); }
    const binding = field.members.find(item => ts.isPropertyDeclaration(item) && item.name.getText(input) === "#builder"); expect(binding?.getText(input)).toContain("OwnedUiResidentBuilderRetirement");
  });
  it("OwnedKernelReturnBuilderBinding rejects premature and forged witnesses without releasing the original binding", async () => {
    const { OwnedKernelReturnInputField } = await import("@semio-tech/framework-kernel/return/content/input"); const { OwnedUiResidentBuilderRetirement } = await import("@semio-tech/ui-contract/retained/resident");
    const { client, residentLedger, instance, source, response } = await deliveredInput(); const input = new OwnedKernelReturnContent(source, fixtureHosts.get(instance)!, instance.activation, instance.lifetime!);
    for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 }); const field = input.field!; const fragment = field.fragment;
    const detach = Reflect.get(field, "detachBuilder"); const settle = Reflect.get(field, "settleBuilder"); const detached = Reflect.get(OwnedKernelReturnInputField, "matchesBuilderDetached"); const settled = Reflect.get(OwnedKernelReturnInputField, "matchesBuilderSettled");
    expect([detach, settle, detached, settled].map(value => typeof value)).toEqual(["function", "function", "function", "function"]);
    const pool = await fixtureResidentPool(client, residentLedger); const scope = await fixtureResidentScope(pool, residentLedger, instance); const resident = await fixtureResidentPayload(scope, residentLedger, field);
    const held: { proof: unknown } = { proof: null }; const original = Object.freeze; const spy = vi.spyOn(Object, "freeze").mockImplementation(value => { if (value instanceof OwnedUiResidentBuilderRetirement) held.proof = value; return original(value); });
    let builder: import("@semio-tech/ui-contract/retained/operations/wire/pages").OwnedUiOperationPayloadBuilder | null = null;
    try { builder = (await fixtureResidentBuilder(residentLedger, field, resident)).builder; } finally { spy.mockRestore(); }
    if (!builder || !held.proof) throw new Error("Genuine original builder/witness missing"); expect(OwnedUiResidentBuilderRetirement.matchesBody(held.proof, builder, field)).toBe(false);
    let reads = 0; const forged = { get phase() { reads++; throw new Error("Foreign builder proof getter"); } }; const proxy = new Proxy({}, { get() { reads++; throw new Error("Foreign builder proof proxy"); } }); const before = residentLedger.usage;
    for (const proof of [held.proof, forged, proxy, Object.create(OwnedUiResidentBuilderRetirement.prototype), builder, null]) {
      expect(Reflect.apply(detach, field, [builder, proof, { maxItems: 1, maxBytes: 63 }])).toEqual({ kind: "blocked", items: 0, bytes: 0 });
      expect(Reflect.apply(detach, field, [builder, proof, { maxItems: 1, maxBytes: 64 }])).toEqual({ kind: "rejected", items: 0, bytes: 0 }); expect(Reflect.apply(settle, field, [proof, { maxItems: 1, maxBytes: 64 }])).toEqual({ kind: "rejected", items: 0, bytes: 0 });
      for (const candidate of [field, forged, proxy, Object.create(OwnedKernelReturnInputField.prototype), null]) { expect(Reflect.apply(detached, OwnedKernelReturnInputField, [candidate, proof])).toBe(false); expect(Reflect.apply(settled, OwnedKernelReturnInputField, [candidate, proof])).toBe(false); }
    }
    expect(reads).toBe(0); expect(OwnedKernelReturnInputField.matchesBuilder(field, builder)).toBe(true); expect(field.fragment).toBe(fragment); expect(residentLedger.usage).toEqual(before); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response); input.beginClose(); client.disposeAll();
  });
  it("OwnedKernelReturnInputEvidence validates exact detach phases with an independent state oracle", async () => {
    const contract = await loadContentInputReleaseDocument(); 
    const fixture = await loadContentInputReleaseFixture();
    const { default: Ajv } = await import("ajv"); const { produce } = await import("immer"); const validator = new Ajv({ strict: true });
     
    const metadata = contract.metadata; const price = (fields: number) => BigInt(metadata.recordBytes) + BigInt(metadata.fieldBytes) * BigInt(fields);
    expect(price(contract.releaseFields.length)).toBe(BigInt(metadata.releaseBytes)); expect(price(contract.fragmentFields.length)).toBe(BigInt(metadata.fragmentBytes)); expect(price(contract.fieldFields.length)).toBe(BigInt(metadata.fieldBytesTotal));
    expect(price(metadata.priorReleaseFields)).toBe(BigInt(metadata.priorReleaseBytes)); expect(metadata.releaseBytes - metadata.priorReleaseBytes).toBe(metadata.additionalBytes);
    for (const vector of fixture.traces) {
      let state = { phase: "empty", bound: false, consumed: false, closing: vector.mode === "cancelled", forgotten: false, sourceToken: false, sourceFragment: false, fragmentField: true, fieldSlots: true, uiBody: true, uiObservation: false, nextRange: false };
      for (const row of vector.trace) {
        let kind = "rejected"; let bytes = 0;
        state = produce(state, draft => {
          if (row.action.endsWith("-short")) { kind = "blocked"; return; }
          if (row.action === "release" && draft.phase === "empty") { draft.phase = "issued"; draft.sourceToken = true; draft.sourceFragment = true; kind = "ready"; }
          else if (row.action === "ui-bind" && draft.phase === "issued") { draft.bound = true; draft.uiObservation = true; kind = "pending"; bytes = 64; }
          else if (row.action === "consume" && draft.phase === "issued" && !draft.closing) { draft.consumed = true; kind = "complete"; bytes = 1; }
          else if (row.action === "close") { draft.closing = true; kind = "pending"; }
          else if (row.action === "detach" && draft.phase === "issued" && draft.bound && (draft.consumed || draft.closing)) { draft.phase = "sourceDetached"; draft.sourceToken = false; draft.sourceFragment = false; draft.fragmentField = false; kind = "pending"; bytes = 64; }
          else if (row.action === "observe-after-wrapper-fault" && draft.phase === "sourceDetached" && draft.bound) kind = "ready";
          else if (row.action === "ui-detach" && draft.phase === "sourceDetached" && draft.bound) { draft.uiBody = false; kind = "pending"; bytes = 64; }
          else if (row.action === "settle" && draft.phase === "sourceDetached" && draft.bound && !draft.uiBody) { draft.phase = "settled"; draft.fieldSlots = false; draft.nextRange = !draft.closing; kind = "complete"; bytes = 64; }
          else if (row.action === "observe-settled-after-wrapper-fault" && draft.phase === "settled" && draft.bound && !draft.forgotten) kind = "ready";
          else if (row.action === "ui-forget" && draft.phase === "settled" && !draft.uiBody) { draft.uiObservation = false; draft.forgotten = true; kind = "complete"; bytes = 64; }
        });
        const { bound, consumed, closing, forgotten, ...visible } = state;
        expect({ kind, bytes, ...visible }, vector.mode + ":" + row.action).toEqual(row.expected);
        if (visible.phase !== "settled") expect(visible.fieldSlots).toBe(true);
        if (visible.uiObservation) expect(fixture.readiness.evidenceRefundBeforeUiForget).toBe(false);
      }
    }
    expect(contract.rules.rawPageAcknowledged).toBe(false); expect(contract.rules.wholeSourceRetired).toBe(false); expect(contract.metadata.sourceAdmitted).toBe(false);
  });
  it("OwnedKernelReturnInputEvidence binds its declared metadata to the actual source fields", async () => {
    const contract = await loadContentInputReleaseDocument(); const ts = await import("typescript"); const { readFile } = await import("node:fs/promises");
    const text = await readFile(new URL("../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts", testSource.url), "utf8"); const source = ts.createSourceFile("input.ts", text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    for (const [name, fields] of [["OwnedKernelReturnInputField", contract.fieldFields], ["OwnedKernelReturnInputFragment", contract.fragmentFields], ["OwnedKernelReturnInputRelease", contract.releaseFields]] as const) {
      const declaration = source.statements.find(item => ts.isClassDeclaration(item) && item.name?.text === name);
      if (!declaration || !ts.isClassDeclaration(declaration)) throw new Error("Actual input class missing");
      expect(declaration.members.filter(ts.isPropertyDeclaration).map(item => item.name.getText(source).slice(1)), name).toEqual(fields);
    }
  });
  it("OwnedKernelReturnInputEvidence rejects unissued or foreign observations without consulting their getters", async () => {
    const { OwnedKernelReturnInputRelease } = await import("@semio-tech/framework-kernel/return/content/input");
    const context = await deliveredInput(); const { client, instance, source, response } = context; const input = new OwnedKernelReturnContent(source, fixtureHosts.get(instance)!, instance.activation, instance.lifetime!);
    for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 }); const field = input.field!; const fragment = field.fragment; let reads = 0;
    const detach = Reflect.get(field, "detachInputRelease"); const settle = Reflect.get(field, "settleInputRelease"); const detached = Reflect.get(OwnedKernelReturnInputRelease, "matchesSourceDetached"); const settled = Reflect.get(OwnedKernelReturnInputRelease, "matchesSettled");
    expect([detach, settle, detached, settled].map(value => typeof value)).toEqual(["function", "function", "function", "function"]);
    const forged = { get kind() { reads++; throw new Error("Foreign observation getter"); }, get fragment() { reads++; throw new Error("Foreign fragment getter"); } }; const proxy = new Proxy({}, { get() { reads++; throw new Error("Foreign observation proxy"); } });
    for (const observation of [forged, proxy, Object.create(OwnedKernelReturnInputRelease.prototype), null]) {
      expect(Reflect.apply(detach, field, [observation, forged, { maxItems: 1, maxBytes: 63 }])).toEqual({ kind: "blocked", items: 0, bytes: 0 });
      expect(Reflect.apply(detach, field, [observation, forged, { maxItems: 1, maxBytes: 64 }])).toEqual({ kind: "rejected", items: 0, bytes: 0 });
      expect(Reflect.apply(settle, field, [observation, forged, { maxItems: 1, maxBytes: 64 }])).toEqual({ kind: "rejected", items: 0, bytes: 0 });
      expect(Reflect.apply(detached, OwnedKernelReturnInputRelease, [observation, field, forged])).toBe(false); expect(Reflect.apply(settled, OwnedKernelReturnInputRelease, [observation, forged])).toBe(false);
    }
    expect(reads).toBe(0); expect(field.fragment).toBe(fragment); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response); input.beginClose(); client.disposeAll();
  });
  it("OwnedKernelReturnInput validates the two-way resident payload declaration with an independent state oracle", async () => {
    const contract = await loadContentInputPayloadDocument(); 
    const fixture = await loadContentInputPayloadFixture();
    const { default: Ajv } = await import("ajv"); const { produce } = await import("immer"); const validator = new Ajv({ strict: true });
     
    const metadata = contract.fixedSubset; expect(metadata.fieldBytesTotal).toBe(metadata.recordBytes + metadata.fieldBytes * contract.sourceFields.length); expect(metadata.observationBytesTotal).toBe(metadata.recordBytes + metadata.fieldBytes * contract.observationFields.length); expect(metadata.total.bytes).toBe(metadata.fieldBytesTotal + metadata.observationBytesTotal);
    let state: { phase: string; sourcePayload: string | null; observedPayload: string | null; observedProof: string | null; uiDetached: boolean } = { phase: "unbound", sourcePayload: null, observedPayload: null, observedProof: null, uiDetached: false };
    for (const row of fixture.trace) {
      let kind = "rejected"; let bytes = 0;
      state = produce(state, draft => {
        if (row.action.endsWith("-short")) { kind = "blocked"; return; }
        if (row.action === "install" && draft.phase === "unbound") { draft.sourcePayload = fixture.identities.payload; draft.observedPayload = fixture.identities.payload; draft.phase = "bound"; kind = "ready"; bytes = 64; }
        else if (row.action === "install-same" && draft.phase === "bound") kind = "ready";
        else if (row.action === "detach" && draft.phase === "bound") { draft.sourcePayload = null; draft.observedProof = fixture.identities.proof; draft.phase = "detached"; kind = "pending"; bytes = 64; }
        else if (row.action === "ui-detach" && draft.phase === "detached") { draft.uiDetached = true; kind = "pending"; bytes = 64; }
        else if (row.action === "settle" && draft.phase === "detached" && draft.uiDetached) { draft.observedPayload = null; draft.observedProof = null; draft.phase = "settled"; kind = "complete"; bytes = 64; }
      });
      expect({ kind, phase: state.phase, sourcePayload: state.sourcePayload, observedPayload: state.observedPayload, observedProof: state.observedProof, bytes }, row.action).toEqual(row.expected);
    }
    let abandoned = { phase: "unbound", uiDetached: false };
    for (const row of fixture.abandonment.trace) {
      let kind = "rejected"; let bytes = 0;
      abandoned = produce(abandoned, draft => {
        if (row.action === "abandon-short") kind = "blocked";
        else if (row.action === "abandon" && draft.phase === "unbound") { draft.phase = "detached"; kind = "pending"; bytes = 64; }
        else if (row.action === "ui-detach" && draft.phase === "detached") { draft.uiDetached = true; kind = "pending"; bytes = 64; }
        else if (row.action === "settle" && draft.phase === "detached" && draft.uiDetached) { draft.phase = "settled"; kind = "complete"; bytes = 64; }
      });
      expect({ kind, phase: abandoned.phase, bytes }, row.action).toEqual({ kind: row.kind, phase: row.phase, bytes: row.bytes });
    }
    expect(contract.rules.closeOnlyUnboundAbandonment).toBe(true); expect(contract.rules.sourceQuotaAdmitted).toBe(false); expect(contract.rules.rawPageAcknowledged).toBe(false); expect(contract.rules.observationFieldBacklink).toBe(false);
  });
  it("OwnedKernelReturnInput refuses fabricated resident payload associations on its actual private field", async () => {
    const { OwnedKernelReturnContent, OwnedKernelReturnInputField, OwnedKernelReturnPayloadDetachment } = await import("@semio-tech/framework-kernel/return/content/input");
    const contract = await loadContentInputPayloadDocument(); const fixture = await loadContentInputPayloadFixture(); const ts = await import("typescript"); const { readFile } = await import("node:fs/promises");
    const { client, instance, source, response } = await deliveredInput(); const owner = fixtureHosts.get(instance)!;
    const input = new OwnedKernelReturnContent(source, owner, instance.activation, instance.lifetime!); for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 }); const field = input.field!;
    expect(OwnedKernelReturnInputField.matchesResidentPayload(field, {})).toBe(false); expect(field.residentPayloadDetachment).toBeNull();
    expect(Reflect.apply(field.installResidentPayload, field, [{}, { maxItems: 1, maxBytes: 63 }])).toEqual({ kind: "blocked", items: 0, bytes: 0 });
    expect(Reflect.apply(field.installResidentPayload, field, [{}, { maxItems: 1, maxBytes: 64 }])).toEqual({ kind: "rejected", items: 0, bytes: 0 });
    expect(Reflect.apply(field.residentPayload, field, [{}])).toBeNull(); expect(Reflect.apply(field.detachResidentPayload, field, [{}, {}, { maxItems: 1, maxBytes: 64 }])).toEqual({ kind: "rejected", items: 0, bytes: 0 });
    expect(Reflect.apply(field.settleResidentPayload, field, [{}, {}, { maxItems: 1, maxBytes: 64 }])).toEqual({ kind: "rejected", items: 0, bytes: 0 });
    expect(OwnedKernelReturnPayloadDetachment.matches({}, field, {})).toBe(false); expect(OwnedKernelReturnPayloadDetachment.matchesSettled({}, {})).toBe(false);
    expect(() => Reflect.construct(OwnedKernelReturnPayloadDetachment, [{}, field])).toThrow("return-input.private-payload-detachment");
    const text = await readFile(new URL("../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts", testSource.url), "utf8"); const parsed = ts.createSourceFile("input.ts", text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    for (const [name, expected] of [["OwnedKernelReturnInputField", contract.sourceFields], ["OwnedKernelReturnPayloadDetachment", contract.observationFields]] as const) { const declaration = parsed.statements.find(item => ts.isClassDeclaration(item) && item.name?.text === name); if (!declaration || !ts.isClassDeclaration(declaration)) throw new Error("Actual source class missing"); expect(declaration.members.filter(ts.isPropertyDeclaration).map(item => item.name.getText(parsed).slice(1))).toEqual(expected); }
    const ownerType = parsed.statements.find(item => ts.isTypeAliasDeclaration(item) && item.name.text === "InputOwner"); if (!ownerType || !ts.isTypeAliasDeclaration(ownerType) || !ts.isTypeLiteralNode(ownerType.type)) throw new Error("Actual input owner type missing");
    expect(ownerType.type.members.filter(ts.isPropertySignature).map(item => item.name.getText(parsed))).toEqual(fixture.construction.inputOwnerFields);
    const ownerMetadata = fixture.construction.inputOwnerSubset; expect(ownerMetadata.bytes).toBe(ownerMetadata.recordBytes + ownerMetadata.fieldBytes * fixture.construction.inputOwnerFields.length); expect(ownerMetadata.additionalBytes).toBe(ownerMetadata.fieldBytes * (ownerMetadata.fields - ownerMetadata.priorFields));
    expect(field.residentPayloadDetachment).toBeNull(); expect(input.field).toBe(field); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response); input.beginClose(); client.disposeAll();
  });
  it("OwnedKernelReturnInput retains exact field construction roots and arbitrary finalizer failures", async () => {
    const { OwnedKernelReturnContent, OwnedKernelReturnInputField, OwnedKernelReturnInputFragment, OwnedKernelReturnPayloadDetachment } = await import("@semio-tech/framework-kernel/return/content/input");
    const fixture = await loadContentInputPayloadFixture(); const row = fixture.construction; const freeze = Object.freeze;
    for (const stage of row.stages) for (const position of row.positions) for (const kind of row.faults) {
      const { client, instance, source, response } = await deliveredInput(); const owner = fixtureHosts.get(instance)!; let reads = 0;
      const faults: Record<string, unknown> = { null: null, undefined, false: false, zero: 0, object: { payload: new Uint8Array(row.faultPayloadBytes), get message() { reads++; throw new Error("Arbitrary fault getter read"); } } }; const fault = faults[kind];
      const input = new OwnedKernelReturnContent(source, owner, instance.activation, instance.lifetime!);
      const capture: { value: object | null; field: typeof input.field; before: boolean } = { value: null, field: null, before: false };
      const trap = vi.spyOn(Object, "freeze").mockImplementation(value => {
        const selected = stage === "field" ? value instanceof OwnedKernelReturnInputField : stage === "fragment" ? value instanceof OwnedKernelReturnInputFragment : value instanceof OwnedKernelReturnPayloadDetachment;
        if (!selected || capture.value !== null) return freeze(value);
        capture.value = value as object; capture.field = input.field;
        capture.before = capture.field !== null && (stage === "field" ? capture.field === value : stage === "fragment" ? capture.field.fragment === value : OwnedKernelReturnPayloadDetachment.matchesOwner(value, capture.field));
        if (position === "after") freeze(value); throw fault;
      });
      try { for (let turn = 0; turn < 256 && input.failure === null; turn++) input.advance({ maxItems: 1, maxBytes: 4096 }); } finally { trap.mockRestore(); }
      expect(capture.value, stage + position + kind).not.toBeNull(); expect(capture.before).toBe(row.parentBeforeFinalizer); expect(input.field).toBe(capture.field);
      expect(OwnedKernelReturnContent.matchesFault(input, fault)).toBe(row.firstFaultRetained); expect(input.failure).toBe("return-input.fault"); expect(reads).toBe(row.publicFaultReads);
      expect(input.advance({ maxItems: 1, maxBytes: 4096 }).kind).toBe("rejected"); expect(input.field).toBe(capture.field); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response);
      input.beginClose(); expect(OwnedKernelReturnContent.matchesFault(input, fault)).toBe(true); client.disposeAll();
    }
  });
}
