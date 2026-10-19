/** 🔓️ The captured return admission, response admission and construction tests. */
import { loadInstanceOutputAdmissionFixture, loadActivationReturnRetirementFixture, loadActivationReturnAdmissionFixture, loadActivationReturnAdmissionContract, loadInstanceOutputAdmissionContract, loadInstanceOutputFaultFixture, loadActorActivationReturnFixture } from "@semio-tech/framework-actor/testing/fixtures";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { shardClientInternals } from "../🧰️settlement-test-kit/🟦️.ts";
import type { OwnedResidentLedger, OwnedResidentRecord, OwnedShardReturn, ShardClientSettlementTestKit, ShardInstanceOwner, ShardReturnAdmission } from "../🧰️settlement-test-kit/🟦️.ts";
import type { CapturedReturnHelpers } from "../🛠️captured-return-helpers/🟦️.ts";

/** 🗝 Registers the unused return retirement, return admission, response admission and return construction tests. */
export function registerCapturedReturnAdmissionTests(kit: ShardClientSettlementTestKit, helpers: CapturedReturnHelpers): void {
  const { BUDGET, NO_RESIDENT_FAULT, OwnedKernelReturnContent, OwnedResidentLedger, OwnedShardReturn, capturedReturnState, expect, it, testSource, vi } = kit;
  const { captured, deliveredInput, fixtureCapturedReturn, fixtureResponse } = helpers;
  it("CapturedUnusedReturnRetirement releases each original admission prefix without posting or retaining stale authority", async () => {
    const fixture = await loadActivationReturnRetirementFixture();
    
    const admission = await loadActivationReturnAdmissionFixture();
    const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");
    
    for (const prefix of fixture.prefixes) {
      const { client, residentLedger, worker, instance } = await captured();
      const owner = shardClientInternals(client).state.instanceLifecycles.get(instance.openRequest.requestSequence)!;
      const before = residentLedger.usage.data, posts = worker.sent.length;
      for (const row of admission.phases.slice(0, prefix)) instance.reserveReturn(admission.capacity, { maxItems: 1, maxBytes: row.grant });
      const source = instance.pendingReturn;
      if (prefix === admission.phases.length && source) await fixtureResponse(source);
      const held = residentLedger.usage.data;
      expect(instance.retireUnusedReturn({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked");
      expect(residentLedger.usage.data).toEqual(held);
      let complete = false;
      for (let index = 0; index < fixture.maximumSteps; index++) {
        const step = instance.retireUnusedReturn(fixture.grant);
        expect(step.items).toBeLessThanOrEqual(fixture.grant.maxItems); expect(step.bytes).toBeLessThanOrEqual(fixture.grant.maxBytes);
        expect(step.kind, step.phase).not.toBe("rejected"); expect(step.kind, step.phase).not.toBe("blocked");
        if (step.kind === "complete") { complete = true; break; }
      }
      expect(complete, String(prefix)).toBe(true);
      expect({ phase: owner.returnPhase, capacity: owner.returnCapacity, pendingReturn: instance.pendingReturn, posts: worker.sent.length - posts }).toEqual(fixture.terminal);
      expect(residentLedger.usage.data).toEqual(produce(before, () => {}));
      expect(owner.returnCell).toBeNull(); expect(owner.returnRecord).toBeNull(); expect(owner.activation.returned).toBeNull();
      if (source) {
        expect(source.reserveResponse(fixture.grant).kind).toBe("rejected");
        await expect(source.execute([], BUDGET)).rejects.toThrow("actor-return.closed");
        const successor = await fixtureCapturedReturn(instance, admission.capacity); expect(successor).not.toBe(source);
        expect(instance.pendingReturn).toBe(successor);
      }
      client.disposeAll();
    }
  });
  it("CapturedUnusedReturnRetirement does not discard an executed original return or its private page", async () => {
    const fixture = await loadActivationReturnRetirementFixture();
    const { client, residentLedger, worker, instance, source, response } = await deliveredInput();
    const before = residentLedger.usage, page = source.page, posts = worker.sent.length;
    for (let index = 0; index < 3; index++) expect(instance.retireUnusedReturn(fixture.grant).kind).toBe("blocked");
    expect(instance.pendingReturn).toBe(source); expect(source.page).toBe(page); expect(source.retainedResponses).toBe(1);
    expect(residentLedger.usage).toEqual(before); expect(worker.sent.length).toBe(posts);
    const latest = capturedReturnState(source).latest;
    if (latest === null) throw new Error("Executed original return lost its latest response");
    expect(latest.responseEnvelope).toBe(response); client.disposeAll();
  });
  it("CapturedUnusedReturnRetirement retains executing, content and construction-fault owners", async () => {
    const fixture = await loadActivationReturnRetirementFixture();
    const admission = await loadActivationReturnAdmissionFixture();
    for (const phase of ["in-flight", "content-owned", "faulted"]) {
      const { client, residentLedger, worker, instance, row } = await captured();
      const owner = shardClientInternals(client).state.instanceLifecycles.get(instance.openRequest.requestSequence)!;
      if (phase === "faulted") {
        const fault = { payload: new Uint8Array(8192) };
        const freeze = Object.freeze;
        const spy = vi.spyOn(Object, "freeze").mockImplementation(value => { if (value instanceof OwnedShardReturn) throw fault; return freeze(value); });
        try { for (const item of admission.phases) instance.reserveReturn(admission.capacity, { maxItems: 1, maxBytes: item.grant }); } finally { spy.mockRestore(); }
        const before = residentLedger.usage;
        expect(instance.retireUnusedReturn(fixture.grant).kind).toBe("blocked"); expect(owner.returnFault).toBe(fault);
        expect(residentLedger.usage).toEqual(before); client.disposeAll(); continue;
      }
      const source = await fixtureCapturedReturn(instance, admission.capacity);
      if (phase === "content-owned") {
        const content = new OwnedKernelReturnContent(source, owner.host!, instance.activation, instance.lifetime!);
        const before = residentLedger.usage;
        expect(instance.retireUnusedReturn(fixture.grant).kind).toBe("blocked"); expect(source.content).toBe(content);
        expect(instance.pendingReturn).toBe(source); expect(residentLedger.usage).toEqual(before); client.disposeAll(); continue;
      }
      await fixtureResponse(source);
      const pending = source.execute([], BUDGET); const request = worker.sent.at(-1) as { requestId: string };
      const before = residentLedger.usage;
      expect(instance.retireUnusedReturn(fixture.grant).kind).toBe("blocked"); expect(instance.pendingReturn).toBe(source);
      expect(residentLedger.usage).toEqual(before);
      const { encodeActorReturnResult } = await import("@semio-tech/framework-actor/return");
      worker.deliver({ kind: "result", requestId: request.requestId, ok: true, value: encodeActorReturnResult({ kind: "pending", identity: { origin: source.origin!, returnSequence: BigInt(row.returnSequence) }, reason: "working" }) });
      await pending;
      expect(instance.retireUnusedReturn(fixture.grant).kind).toBe("blocked"); expect(instance.pendingReturn).toBe(source); client.disposeAll();
    }
  });
  it("CapturedReturnAdmission validates its exact parent phases and independent fixed ledger inventory", async () => {
    const contract = await loadActivationReturnAdmissionContract(); 
    const fixture = await loadActivationReturnAdmissionFixture(); 
    const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");  
    const words = [contract.parentFields, contract.stateFields, contract.rosterFields, contract.facadeFields]; const bytes = words.reduce((sum, fields) => sum + BigInt(contract.model.recordBytes) + BigInt(fields.length) * BigInt(contract.model.fieldBytes), 0n);
    expect({ bytes: Number(bytes), slots: words.length, owners: words.length }).toEqual(contract.domain);
    const retained = produce({ bytes: 0, slots: 0, owners: 0 }, value => { for (const envelope of [contract.domain, contract.intrinsicRecord, contract.admissionCell]) { value.bytes += envelope.bytes; value.slots += envelope.slots; value.owners += envelope.owners; } });
    expect(retained).toEqual(fixture.retained); let state = { bytes: 0, slots: 0, owners: 0, published: false };
    for (const [index, phase] of fixture.phases.entries()) {
      state = produce(state, value => { if (index === 0) Object.assign(value, contract.admissionCell); if (index === 4) Object.assign(value, retained); value.published = index === fixture.phases.length - 1; });
      expect(phase.phase).toBe(contract.phases[index]); expect(phase.grant).toBe(contract.grants[index]); expect(phase.kind).toBe(state.published ? "ready" : "pending"); expect(phase.source).toBe(state.published);
      expect({ bytes: state.bytes, slots: state.slots, owners: state.owners }).toEqual(phase.resident);
    }
    expect(contract.boundaries.liveDispatch).toBe(false); expect(contract.boundaries.wholeReturnRetirement).toBe(false); expect(new Set(contract.constructionOrder).size).toBe(fixture.phases.length);
  });
  it("CapturedReturnAdmission binds the parent, state, roster and facade inventory to actual source", async () => {
    const contract = await loadActivationReturnAdmissionContract(); const ts = await import("typescript"); const { readFile } = await import("node:fs/promises");
    const ast = ts.createSourceFile("shard-client.ts", readModuleTreeSource(new URL("./🟦️.ts", testSource.url)), ts.ScriptTarget.Latest, true);
    const fields = (name: string) => { const node = ast.statements.find(item => ts.isTypeAliasDeclaration(item) && item.name.text === name); if (!node || !ts.isTypeAliasDeclaration(node) || !ts.isTypeLiteralNode(node.type)) throw new Error("Missing return owner declaration"); return node.type.members.map(item => item.name?.getText(ast)); };
    const instance = fields("ShardInstanceOwner"); expect(instance.slice(-contract.parentFields.length)).toEqual(contract.parentFields); expect(fields("CapturedReturn")).toEqual(contract.stateFields);
    const facade = ast.statements.find(item => ts.isClassDeclaration(item) && item.name?.text === "OwnedShardReturn"); if (!facade || !ts.isClassDeclaration(facade)) throw new Error("Missing return facade"); expect(facade.members.filter(ts.isPropertyDeclaration).map(item => item.name.getText(ast).slice(1))).toEqual(contract.facadeFields);
    const output = ts.createSourceFile("output.ts", await readFile(new URL("../🪪️activation/🚪️instance/📥️output/🟦️.ts", testSource.url), "utf8"), ts.ScriptTarget.Latest, true);
    const roster = output.statements.find(item => ts.isClassDeclaration(item) && item.name?.text === "OwnedActorTurnOutputs"); if (!roster || !ts.isClassDeclaration(roster)) throw new Error("Missing output roster"); expect(roster.members.filter(ts.isPropertyDeclaration).map(item => item.name.getText(output).slice(1))).toEqual(contract.rosterFields);
    const constructor = roster.members.find(ts.isConstructorDeclaration); expect(constructor?.body?.getText(output)).not.toContain("Object.freeze");
    const lease = ast.statements.find(item => ts.isInterfaceDeclaration(item) && item.name.text === "ShardInstanceLifecycleLease"); if (!lease || !ts.isInterfaceDeclaration(lease)) throw new Error("Missing captured lease");
    const method = lease.members.find(item => ts.isMethodSignature(item) && item.name.getText(ast) === "reserveReturn"); if (!method || !ts.isMethodSignature(method)) throw new Error("Missing phased return method"); expect(method.parameters.map(item => item.name.getText(ast))).toEqual(["maximumResponses", "grant"]);
  });
  it("CapturedReturnAdmission refuses the first zero grant before original parent construction", async () => {
    const fixture = await loadActivationReturnAdmissionFixture(); const { client, residentLedger, worker, instance } = await captured(); const before = residentLedger.usage; const sent = worker.sent.length;
    const admission = Reflect.apply(instance.reserveReturn, instance, [fixture.capacity, { maxItems: 0, maxBytes: 0 }]);
    expect(admission).toMatchObject({ step: { kind: fixture.shortGrant.kind, items: fixture.shortGrant.items, bytes: fixture.shortGrant.bytes }, source: null });
    expect(instance.pendingReturn).toBeNull(); expect(residentLedger.usage).toEqual(before); expect(worker.sent.length).toBe(sent); client.disposeAll();
  });
  it("CapturedReturnAdmission installs the actual parent before all twelve granted construction phases", async () => {
    const fixture = await loadActivationReturnAdmissionFixture(); const { produce } = await import("immer");
    const { client, residentLedger, worker, instance } = await captured(); const owner: ShardInstanceOwner = shardClientInternals(client).state.instanceLifecycles.get(instance.openRequest.requestSequence)!; const initial = residentLedger.usage.data; const posts = worker.sent.length;
    for (const capacity of fixture.invalidCapacities) expect(instance.reserveReturn(capacity, { maxItems: 1, maxBytes: 4096 })).toMatchObject({ step: { kind: "rejected", bytes: 0 }, source: null });
    expect(owner.returnCell).toBeNull(); expect(owner.returnCapacity).toBe(0); let source: OwnedShardReturn | null = null;
    for (const [index, phase] of fixture.phases.entries()) {
      const before = residentLedger.usage; const parent = owner.activation.returned;
      for (const grant of [{ maxItems: 0, maxBytes: phase.grant }, { maxItems: 1, maxBytes: phase.grant - 1 }]) expect(instance.reserveReturn(fixture.capacity, grant)).toMatchObject({ step: { kind: "blocked", bytes: 0 }, source: null });
      expect(residentLedger.usage).toEqual(before); expect(owner.activation.returned).toBe(parent);
      const current = instance.reserveReturn(fixture.capacity, { maxItems: 1, maxBytes: phase.grant }); expect(current.step.kind).toBe(phase.kind); expect(current.step.bytes).toBe(phase.grant); expect(current.source !== null).toBe(phase.source); expect(owner.returnPhase).toBe(phase.phase);
      expect(residentLedger.usage.data).toEqual(produce(initial, value => { value.bytes += phase.resident.bytes; value.slots += phase.resident.slots; value.owners += phase.resident.owners; }));
      if (index >= 6) expect(owner.returnRecord?.matchesShell(owner)).toBe(true); if (index >= 8) expect(Object.isSealed(owner.activation.returned)).toBe(true); if (index >= 9) expect(Object.isFrozen(owner.activation.returned?.outputs)).toBe(true);
      if (index === 10) { expect(instance.pendingReturn).not.toBeNull(); await expect(instance.pendingReturn!.execute([], BUDGET)).rejects.toThrow("actor-return.construction-pending"); }
      source = current.source;
    }
    if (!source || !owner.returnRecord) throw new Error("Original return was not retained");
    expect(instance.reserveReturn(fixture.capacity, { maxItems: 1, maxBytes: 64 })).toMatchObject({ step: { kind: "ready", bytes: 0 }, source }); expect(instance.reserveReturn(fixture.capacity + 1, { maxItems: 1, maxBytes: 64 }).step.kind).toBe("rejected");
    expect(Object.keys(source)).toEqual([]); expect(source.retainedResponses).toBe(0); const retained = residentLedger.usage;
    owner.returnRecord.beginClose(); expect(owner.returnRecord.closeStep({ maxItems: 1, maxBytes: 264 }).kind).toBe("blocked"); expect(residentLedger.usage).toEqual(retained);
    client.disposeAll(); expect(instance.pendingReturn).toBe(source); expect(residentLedger.usage).toEqual(retained); expect(worker.sent.length).toBe(posts);
  });
  it("CapturedReturnAdmission recovers genuine lost returns and finalizer faults under the original charged owner", async () => {
    const fixture = await loadActivationReturnAdmissionFixture(); const { OwnedResidentRecord } = await import("@semio-tech/value-ts/resident");
    for (const boundary of fixture.faultBoundaries) for (const kind of fixture.faultValues) {
      const { client, residentLedger, worker, instance } = await captured(); const owner: ShardInstanceOwner = shardClientInternals(client).state.instanceLifecycles.get(instance.openRequest.requestSequence)!; const posts = worker.sent.length; let reads = 0;
      const fault = kind === "null" ? null : kind === "undefined" ? undefined : kind === "false" ? false : kind === "zero" ? 0 : { payload: new Uint8Array(8193), get message() { reads++; throw new Error("Foreign fault getter"); } };
      const restorations: Array<() => void> = []; let observed = false;
      if (boundary === "prepare-after-return") { const original = OwnedResidentLedger.prototype.prepareAdmission; const spy = vi.spyOn(OwnedResidentLedger.prototype, "prepareAdmission").mockImplementation(function (this: OwnedResidentLedger, consumer, partition, grant) { const result = original.call(this, consumer, partition, grant); if (this === residentLedger && consumer === owner) { observed = true; throw fault; } return result; }); restorations.push(() => spy.mockRestore()); }
      if (boundary === "claim-after-return") { const original = OwnedResidentLedger.prototype.claimAdmission; const spy = vi.spyOn(OwnedResidentLedger.prototype, "claimAdmission").mockImplementation(function (this: OwnedResidentLedger, consumer, cell, grant) { const result = original.call(this, consumer, cell, grant); if (this === residentLedger && consumer === owner) { observed = true; throw fault; } return result; }); restorations.push(() => spy.mockRestore()); }
      if (boundary === "record-after-return") { const original = OwnedResidentLedger.prototype.reserveRecord; const spy = vi.spyOn(OwnedResidentLedger.prototype, "reserveRecord").mockImplementation(function (this: OwnedResidentLedger, partition, envelope, cell, grant) { const result = original.call(this, partition, envelope, cell, grant); if (this === residentLedger && cell === owner.returnCell) { observed = true; throw fault; } return result; }); restorations.push(() => spy.mockRestore()); }
      if (boundary === "install-after-call") { const original = OwnedResidentRecord.prototype.install; const spy = vi.spyOn(OwnedResidentRecord.prototype, "install").mockImplementation(function (this: OwnedResidentRecord, shell, grant) { const result = original.call(this, shell, grant); if (shell === owner) { observed = true; throw fault; } return result; }); restorations.push(() => spy.mockRestore()); }
      if (boundary.startsWith("state-")) { const original = Object.seal; const spy = vi.spyOn(Object, "seal").mockImplementation(value => { if (value === owner.activation.returned && value !== null) { observed = true; expect(owner.returnRecord?.matchesShell(owner)).toBe(true); if (boundary === "state-after-seal") original(value); throw fault; } return original(value); }); restorations.push(() => spy.mockRestore()); }
      if (boundary.startsWith("roster-") || boundary.startsWith("facade-")) { const original = Object.freeze; const spy = vi.spyOn(Object, "freeze").mockImplementation(value => { const state = owner.activation.returned; const selected = boundary.startsWith("roster-") ? state?.outputs === value : state?.facade === value; if (state && selected && value !== null) { observed = true; expect(owner.returnRecord?.matchesShell(owner)).toBe(true); if (boundary.includes("after")) original(value); throw fault; } return original(value); }); restorations.push(() => spy.mockRestore()); }
      let rejected: ShardReturnAdmission | null = null; let spent = 0;
      try { for (const phase of fixture.phases) { const current = instance.reserveReturn(fixture.capacity, { maxItems: 1, maxBytes: phase.grant }); if (current.step.kind === "rejected") { rejected = current; spent = phase.grant; break; } } } finally { for (const restore of restorations) restore(); }
      expect(observed).toBe(true); expect(rejected?.source).toBeNull(); expect(Object.is(owner.returnFault, fault)).toBe(true); expect(rejected?.step.bytes).toBe(spent);
      const state = owner.activation.returned; const outputs = state?.outputs; const facade = state?.facade;
      for (let turn = 0; turn < 3; turn++) instance.reserveReturn(fixture.capacity, { maxItems: 1, maxBytes: 4096 });
      expect(owner.returnCell?.hasFailure).toBe(true); expect(Object.is(owner.returnCell?.failure, fault)).toBe(true); expect(owner.activation.returned).toBe(state); expect(state?.outputs).toBe(outputs); expect(state?.facade).toBe(facade);
      expect(instance.reserveReturn(fixture.capacity + 1, { maxItems: 1, maxBytes: 4096 }).step.kind).toBe("rejected"); expect(worker.sent.length).toBe(posts); expect(reads).toBe(0); const usage = residentLedger.usage;
      client.disposeAll(); expect(residentLedger.usage).toEqual(usage); expect(owner.activation.returned).toBe(state); expect(Object.is(owner.returnFault, fault)).toBe(true);
    }
  });
  it("CapturedReturnAdmission stops forward child construction at every closing ledger prefix", async () => {
    const fixture = await loadActivationReturnAdmissionFixture();
    for (const prefix of fixture.closing.prefixes) {
      const { client, residentLedger, worker, instance } = await captured(); const owner: ShardInstanceOwner = shardClientInternals(client).state.instanceLifecycles.get(instance.openRequest.requestSequence)!;
      for (const phase of fixture.phases.slice(0, prefix)) expect(instance.reserveReturn(fixture.capacity, { maxItems: 1, maxBytes: phase.grant }).step.kind).toBe(phase.kind);
      const state = owner.activation.returned; const outputs = state?.outputs; const source = state?.facade; const before = residentLedger.usage; const posts = worker.sent.length; residentLedger.beginClose();
      for (const phase of fixture.phases.slice(prefix)) {
        const current = instance.reserveReturn(fixture.capacity, { maxItems: 1, maxBytes: phase.grant });
        expect(current.source, String(prefix)).toBeNull(); expect(owner.activation.returned === state, String(prefix)).toBe(true); expect(state?.outputs === outputs, String(prefix)).toBe(true); expect(state?.facade === source, String(prefix)).toBe(true); expect(residentLedger.usage, String(prefix)).toEqual(before);
      }
      expect(worker.sent.length).toBe(posts); client.disposeAll();
    }
  });
  it("ActorResponseAdmission refuses an ungranted output allocation in the actual captured roster", async () => {
    const fixture = await loadInstanceOutputAdmissionFixture();
    const { client, residentLedger, worker, instance } = await captured(); const source = await fixtureCapturedReturn(instance, fixture.capacity); const state = capturedReturnState(source); const before = residentLedger.usage; const posts = worker.sent.length;
    if (!state.outputs) throw new Error("Missing original output roster");
    Reflect.apply(state.outputs.reserve, state.outputs, [{ maxItems: 0, maxBytes: 4096 }]);
    expect(state.outputs.pending).toBe(fixture.short.linked); expect(residentLedger.usage).toEqual(before); expect(worker.sent.length).toBe(posts); expect(source.retainedResponses).toBe(0); client.disposeAll();
  });
  it("ActorResponseAdmission charges each original output before construction and retains cancelled metadata", async () => {
    const fixture = await loadInstanceOutputAdmissionFixture(); const { produce } = await import("immer"); const { OwnedResidentRecord } = await import("@semio-tech/value-ts/resident");
    const { client, residentLedger, worker, instance } = await captured(); const source = await fixtureCapturedReturn(instance, fixture.capacity); const state = capturedReturnState(source); const before = residentLedger.usage.data; const posts = worker.sent.length; let record: OwnedResidentRecord | null = null;
    const install = OwnedResidentRecord.prototype.install; const spy = vi.spyOn(OwnedResidentRecord.prototype, "install").mockImplementation(function (this: OwnedResidentRecord, shell, grant) { const result = install.call(this, shell, grant); if (shell === state.outputs) record = this; return result; });
    try {
      for (const [index, row] of fixture.phases.entries()) {
        const usage = residentLedger.usage; const linked = source.retainedResponses;
        for (const grant of [{ maxItems: 0, maxBytes: row.grant }, { maxItems: 1, maxBytes: row.grant - 1 }]) { expect(source.reserveResponse(grant).kind).toBe("blocked"); expect(source.retainedResponses).toBe(linked); expect(residentLedger.usage).toEqual(usage); }
        const current = source.reserveResponse({ maxItems: 1, maxBytes: row.grant }); expect(current.kind, row.phase).toBe(row.kind); expect(current.bytes, row.phase).toBe(row.grant); expect(source.retainedResponses).toBe(row.linked);
        expect(residentLedger.usage.data).toEqual(produce(before, value => { value.bytes += row.resident.bytes; value.slots += row.resident.slots; value.owners += row.resident.owners; }));
        if (index < fixture.phases.length - 1) { await expect(source.execute([], BUDGET)).rejects.toThrow("actor-return.response-admission-required"); expect(worker.sent.length).toBe(posts); }
        if (index === fixture.phases.length - 2) { const output = state.outputs?.peek(); if (!output) throw new Error("Missing retained facade before publication"); let called = false; await expect(output.run(async () => { called = true; return null; })).rejects.toThrow("actor-output.closed"); expect(called).toBe(false); }
      }
    } finally { spy.mockRestore(); }
    const output = state.latest; if (!output || !record) throw new Error("Missing original admitted output"); const originalRecord: OwnedResidentRecord = record;
    expect(source.reserveResponse({ maxItems: 1, maxBytes: 64 })).toMatchObject({ kind: "ready", bytes: fixture.repeat.bytes }); expect(state.latest).toBe(output); expect(originalRecord.matchesShell(state.outputs)).toBe(true);
    const retained = residentLedger.usage; expect(output.cancelEmpty()).toBe(true); expect(output.cancelEmpty()).toBe(false); expect(source.retainedResponses).toBe(1); expect(state.outputs?.peek()).toBe(output); expect(residentLedger.usage).toEqual(retained);
    originalRecord.beginClose(); expect(originalRecord.closeStep({ maxItems: 1, maxBytes: 264 }).kind).toBe("blocked"); expect(residentLedger.usage).toEqual(retained); expect(worker.sent.length).toBe(posts); client.disposeAll();
  });
  it("ActorResponseAdmission preserves original roots at every closing prefix", async () => {
    const fixture = await loadInstanceOutputAdmissionFixture();
    for (const prefix of fixture.closingPrefixes) for (const close of ["ledger", "roster"] as const) {
      const { client, residentLedger, worker, instance } = await captured(); const source = await fixtureCapturedReturn(instance, fixture.capacity); const state = capturedReturnState(source);
      for (const row of fixture.phases.slice(0, prefix)) expect(source.reserveResponse({ maxItems: 1, maxBytes: row.grant }).kind).toBe(row.kind);
      const head = state.outputs?.peek(); const latest = state.latest; const linked = source.retainedResponses; const usage = residentLedger.usage; const posts = worker.sent.length;
      if (close === "ledger") residentLedger.beginClose(); else state.outputs!.beginClose();
      for (const row of fixture.phases.slice(prefix)) { expect(source.reserveResponse({ maxItems: 1, maxBytes: row.grant }).kind).not.toBe("ready"); expect(state.outputs?.peek()).toBe(head); expect(state.latest).toBe(latest); expect(source.retainedResponses).toBe(linked); expect(residentLedger.usage).toEqual(usage); }
      await expect(source.execute([], BUDGET)).rejects.toThrow(); expect(worker.sent.length).toBe(posts); client.disposeAll();
    }
  });
  it("ActorResponseAdmission recovers exact wrapper and finalizer faults without replacement or post", async () => {
    const fixture = await loadInstanceOutputAdmissionFixture(); const contract = await loadInstanceOutputAdmissionContract(); const { OwnedResidentRecord } = await import("@semio-tech/value-ts/resident");
    for (const boundary of contract.faultBoundaries) for (const kind of fixture.faultValues) {
      const { client, residentLedger, worker, instance } = await captured(); const source = await fixtureCapturedReturn(instance, fixture.capacity); const state = capturedReturnState(source); const posts = worker.sent.length; let reads = 0; let observed = false;
      const fault = kind === "null" ? null : kind === "undefined" ? undefined : kind === "false" ? false : kind === "zero" ? 0 : { payload: new Uint8Array(fixture.unknownBytes), get message() { reads++; throw new Error("Foreign admission fault getter"); } }; const restore: Array<() => void> = [];
      if (boundary === "prepare-after-return") { const original = OwnedResidentLedger.prototype.prepareAdmission; const spy = vi.spyOn(OwnedResidentLedger.prototype, "prepareAdmission").mockImplementation(function (this: OwnedResidentLedger, consumer, partition, grant) { const result = original.call(this, consumer, partition, grant); if (this === residentLedger && consumer === state.outputs) { observed = true; throw fault; } return result; }); restore.push(() => spy.mockRestore()); }
      if (boundary === "claim-after-return") { const original = OwnedResidentLedger.prototype.claimAdmission; const spy = vi.spyOn(OwnedResidentLedger.prototype, "claimAdmission").mockImplementation(function (this: OwnedResidentLedger, consumer, cell, grant) { const result = original.call(this, consumer, cell, grant); if (this === residentLedger && consumer === state.outputs) { observed = true; throw fault; } return result; }); restore.push(() => spy.mockRestore()); }
      if (boundary === "record-after-return") { const original = OwnedResidentLedger.prototype.reserveRecord; const spy = vi.spyOn(OwnedResidentLedger.prototype, "reserveRecord").mockImplementation(function (this: OwnedResidentLedger, partition, envelope, cell, grant) { const result = original.call(this, partition, envelope, cell, grant); if (this === residentLedger) { observed = true; throw fault; } return result; }); restore.push(() => spy.mockRestore()); }
      if (boundary === "install-after-call") { const original = OwnedResidentRecord.prototype.install; const spy = vi.spyOn(OwnedResidentRecord.prototype, "install").mockImplementation(function (this: OwnedResidentRecord, shell, grant) { const result = original.call(this, shell, grant); if (shell === state.outputs) { observed = true; throw fault; } return result; }); restore.push(() => spy.mockRestore()); }
      if (boundary.startsWith("facade-")) { const original = Object.freeze; const spy = vi.spyOn(Object, "freeze").mockImplementation(value => { if (value !== null && value === state.outputs?.peek()) { observed = true; if (boundary === "facade-after-freeze") original(value); throw fault; } return original(value); }); restore.push(() => spy.mockRestore()); }
      try { await expect(fixtureResponse(source)).rejects.toBe(fault); } finally { for (const reset of restore) reset(); }
      expect(observed).toBe(true); expect(Object.is(state.fault, fault)).toBe(true); expect(state.failed).toBe(true); const head = state.outputs?.peek(); const latest = state.latest; const linked = source.retainedResponses; const usage = residentLedger.usage;
      for (let turn = 0; turn < 4; turn++) expect(source.reserveResponse({ maxItems: 1, maxBytes: 4096 }).kind).not.toBe("ready");
      expect(state.outputs?.peek()).toBe(head); expect(state.latest).toBe(latest); expect(source.retainedResponses).toBe(linked); expect(residentLedger.usage).toEqual(usage); await expect(source.execute([], BUDGET)).rejects.toThrow("actor-return.owner-fault"); expect(worker.sent.length).toBe(posts); expect(reads).toBe(0); client.disposeAll();
    }
  });
  it("CapturedReturnConstruction fences the original parent after a retained child finalizer fault", async () => {
    const { OwnedActorTurnOutput } = await import("@semio-tech/framework-actor/activation/instance/output");
    const fixture = await loadInstanceOutputFaultFixture();  const { default: Ajv } = await import("ajv"); 
    for (const boundary of fixture.boundaries) for (const kind of fixture.values) {
      const { client, worker, instance } = await captured(); const source = await fixtureCapturedReturn(instance, 2); const state = capturedReturnState(source); const posts = worker.sent.length; let reads = 0;
      const fault = kind === "null" ? null : kind === "undefined" ? undefined : kind === "false" ? false : kind === "zero" ? 0 : { payload: new Uint8Array(fixture.unknownBytes), get message() { reads++; throw new Error("Foreign child fault getter"); } };
      const original = Object.freeze; const spy = vi.spyOn(Object, "freeze").mockImplementation(value => { if (value !== null && value === state.outputs?.peek()) { if (boundary === "after-finalize") original(value); throw fault; } return original(value); });
      try { await expect(fixtureResponse(source)).rejects.toBe(fault); } finally { spy.mockRestore(); }
      const output = state.outputs?.peek(); expect(output).not.toBeNull(); expect(OwnedActorTurnOutput.matchesFault(output, fault)).toBe(true); expect(state.failed).toBe(fixture.parent.faulted); expect(Object.is(state.fault, fault)).toBe(true);
      let calls = 0; const post = vi.spyOn(worker, "postMessage").mockImplementation(() => { calls++; throw new Error("Unexpected forward dispatch"); });
      try { await expect(source.execute([], BUDGET)).rejects.toThrow(fixture.parent.secondOutcome); } finally { post.mockRestore(); }
      expect(calls).toBe(fixture.parent.dispatchCalls); expect(source.retainedResponses).toBe(fixture.parent.retainedResponses); expect(state.outputs?.peek()).toBe(output); expect(worker.sent.length).toBe(posts); expect(reads).toBe(0); client.disposeAll();
    }
  });
  it("CapturedReturnConstruction retains the original parent and raw fault before facade finalization", async () => {
    const fixture = await loadActorActivationReturnFixture();
    
    const { default: Ajv } = await import("ajv"); const { produce } = await import("immer"); const ts = await import("typescript"); const { readFile } = await import("node:fs/promises");
    
    for (const boundary of fixture.construction.boundaries) for (const kind of fixture.construction.faults) {
      const { row, client, worker, instance } = await captured(); let reads = 0;
      const fault = kind === "null" ? null : kind === "undefined" ? undefined : kind === "false" ? false : kind === "zero" ? 0 : { payload: new Uint8Array(fixture.construction.unknownBytes), get message() { reads++; throw new Error("Unowned fault getter"); } };
      const observed: { source: OwnedShardReturn | null; before: boolean; fault: unknown } = { source: null, before: false, fault: NO_RESIDENT_FAULT };
      const freeze = Object.freeze; const count = worker.sent.length;
      const finalizer = vi.spyOn(Object, "freeze").mockImplementation(value => {
        if (value instanceof OwnedShardReturn) { observed.source = value; observed.before = instance.pendingReturn === value; if (boundary === "after-finalize") freeze(value); throw fault; }
        return freeze(value);
      });
      const admission = await loadActivationReturnAdmissionFixture(); let rejected = false;
      try { for (const phase of admission.phases) { const result = instance.reserveReturn(row.responseSlots, { maxItems: 1, maxBytes: phase.grant }); if (result.step.kind === "rejected") { rejected = true; break; } } } finally { finalizer.mockRestore(); }
      expect(rejected).toBe(true);
      expect(observed.before).toBe(fixture.construction.parentBeforeFinalize);
      const source = observed.source; if (!source) throw new Error("Original captured-return shell was not observed");
      expect(instance.pendingReturn === source).toBe(fixture.construction.sameFacadeAfterFault);
      const state = capturedReturnState(source); expect(Object.is(state.fault, fault)).toBe(fixture.construction.sameFaultAfterFault);
      expect(state.failed).toBe(true); expect(state.retired).toBe(fixture.construction.retired); expect(source.retainedResponses).toBe(fixture.construction.retainedResponses);
      expect(instance.reserveReturn(row.responseSlots + 1, { maxItems: 1, maxBytes: 64 }).step.kind).toBe("rejected");
      await expect(source.execute([], BUDGET)).rejects.toThrow("actor-return.owner-fault");
      expect(worker.sent.length - count).toBe(fixture.construction.postedRequests); expect(reads).toBe(fixture.construction.publicFaultReads);
      const oracle = produce({ retained: false, failed: false }, current => { current.retained = true; current.failed = true; });
      expect({ retained: instance.pendingReturn === source, failed: state.failed }).toEqual(oracle);
      client.disposeAll(); expect(instance.pendingReturn).toBe(source); expect(Object.is(state.fault, fault)).toBe(true);
    }
    const ast = ts.createSourceFile("shard-client.ts", readModuleTreeSource(new URL("./🟦️.ts", testSource.url)), ts.ScriptTarget.Latest, true);
    const declaration = ast.statements.find(node => ts.isTypeAliasDeclaration(node) && node.name.text === "CapturedReturn");
    if (!declaration || !ts.isTypeAliasDeclaration(declaration) || !ts.isTypeLiteralNode(declaration.type)) throw new Error("Captured return state declaration missing");
    expect(declaration.type.members.map(member => member.name?.getText(ast))).toEqual(fixture.construction.stateFields);
  });
}
