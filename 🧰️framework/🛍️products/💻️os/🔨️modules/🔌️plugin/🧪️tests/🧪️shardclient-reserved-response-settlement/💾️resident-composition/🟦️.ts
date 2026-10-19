/** 💾️ The shard resident composition registration tests. */
import { fixtureOriginalWorkStock } from "@semio-tech/framework-actor/retained-turn/bootstrap/testing/fixtures";
import { loadActorCompositionFixture } from "@semio-tech/framework-actor/testing/fixtures";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { FakeShardWorker, flushAdmission, spyGetter } from "../🧰️settlement-test-kit/🟦️.ts";
import type { OwnedResidentAdmission, OwnedResidentLedger, OwnedResidentRecord, ResidentStep, ShardClient, ShardClientSettlementTestKit } from "../🧰️settlement-test-kit/🟦️.ts";

/** 🧱 Registers the ledger, activation owner, pool, parent close and funding tests of the shard resident composition. */
export function registerShardResidentCompositionTests(kit: ShardClientSettlementTestKit): void {
  const { BUDGET, OwnedResidentLedger, OwnedResidentRetirement, OwnedUiResidentPool, ShardClient, expect, harness, it, poolControllerEnvelope, prepareResidentFixture, testSource, uiResidentMetadataEnvelope, vi } = kit;
  it("ShardResidentComposition requires the original ledger before creating workers", async () => {
    const { OwnedResidentLedger } = await import("@semio-tech/value-ts/resident");
    const fixture = await loadActorCompositionFixture(); 
    const { residentSchema } = await import("@semio-tech/value-ts/resident/schema"); const { default: Ajv } = await import("ajv");
    
    const ledger = new OwnedResidentLedger(fixture.capacity); let workers = 0;
    const createWorker = () => { workers++; return new FakeShardWorker(0); };
    for (const candidate of [undefined, { capacity: fixture.capacity }, Object.create(OwnedResidentLedger.prototype), new Proxy(ledger, {})]) {
      expect(() => Reflect.construct(ShardClient, [{ originalWorkStock: fixtureOriginalWorkStock(), shardCount: 1, createWorker, residentLedger: candidate }])).toThrow("actor-resident.invalid-ledger");
      expect(workers).toBe(fixture.boundaries.workersBeforeValidLedger);
    }
    const client: ShardClient = Reflect.construct(ShardClient, [{ originalWorkStock: fixtureOriginalWorkStock(), shardCount: 1, createWorker, residentLedger: ledger }]);
    expect(ShardClient.matchesResidentLedger(client, ledger)).toBe(fixture.binding.sameLedger);
    expect(ShardClient.matchesResidentLedger(client, new OwnedResidentLedger(fixture.capacity))).toBe(fixture.binding.foreignLedger);
    expect(ShardClient.matchesResidentLedger(Object.create(ShardClient.prototype), ledger)).toBe(fixture.binding.structuralClient);
    expect(Object.keys(client).includes(fixture.binding.requiredOption)).toBe(fixture.binding.publicLedgerProperty);
    expect(workers).toBe(1); expect(ledger.usage).toEqual({ data: { bytes: 0, slots: 0, owners: 0 }, control: { bytes: 0, slots: 0, owners: 0 } }); client.disposeAll();
  });
  it("ShardResidentComposition matches only the privately captured original activation owner", async () => {
    const fixture = await loadActorCompositionFixture(); 
    const { residentSchema } = await import("@semio-tech/value-ts/resident/schema"); const { default: Ajv } = await import("ajv");
    const { readFile } = await import("node:fs/promises"); const ts = await import("typescript"); const row = fixture.activationBinding;
    
    const ledger = new OwnedResidentLedger(fixture.capacity); const local = harness(1, { residentLedger: ledger }); const foreign = harness(1, { residentLedger: ledger });
    const activate = async (context: typeof local) => { const pending = context.client.activate(row.actorId, "https://fixture.invalid/actor.js", [], BUDGET); await flushAdmission(); const request = context.workers[0]!.sent.at(-1) as { requestId: string }; context.workers[0]!.deliver({ kind: "result", requestId: request.requestId, ok: true, value: undefined }); await pending; return context.client.captureActorActivation(row.actorId); };
    const lease = await activate(local); const other = await activate(foreign); const usage = ledger.usage;
    expect(local.residentLedger === foreign.residentLedger).toBe(row.sameLedger); expect(lease.actorId === other.actorId).toBe(row.sameActor); expect(lease.activationGeneration === other.activationGeneration).toBe(row.sameGeneration);
    expect(ShardClient.matchesActivation(local.client, lease)).toBe(row.own); expect(ShardClient.matchesActivation(foreign.client, lease)).toBe(row.foreign); expect(ShardClient.matchesActivation(local.client, other)).toBe(row.foreign);
    expect(Object.keys(lease)).toEqual(row.publicKeys); expect(Object.isFrozen(lease)).toBe(true);
    let reads = 0; const traps = { get() { reads++; throw new Error("Unexpected public getter"); }, has() { reads++; throw new Error("Unexpected public probe"); }, getPrototypeOf() { reads++; throw new Error("Unexpected prototype probe"); } };
    const structural = Object.defineProperties({}, Object.fromEntries(row.publicKeys.map(key => [key, { get() { reads++; throw new Error("Unexpected structural read"); } }])));
    const revoked = Proxy.revocable(lease, traps); revoked.revoke();
    const candidates: Record<string, unknown> = { null: null, undefined, structural, prototype: Object.create(Object.getPrototypeOf(lease)), proxy: new Proxy(lease, traps), "revoked-proxy": revoked.proxy };
    for (const name of row.refusals) expect(ShardClient.matchesActivation(local.client, candidates[name]), name).toBe(false);
    const clients: Record<string, unknown> = { null: null, undefined, structural, prototype: Object.create(ShardClient.prototype), proxy: new Proxy(local.client, traps) };
    for (const name of row.clientRefusals) expect(ShardClient.matchesActivation(clients[name], lease), name).toBe(false);
    expect(reads).toBe(row.trapReads); expect(ledger.usage).toEqual(usage);
    expect(() => Reflect.construct(Object.getPrototypeOf(lease).constructor, [Symbol("foreign"), local.client, lease.actorId, lease.activationGeneration, lease.assertActive, lease.turn])).toThrow("actor-activation.private-lease");
    const source = ts.createSourceFile("shard.ts", readModuleTreeSource(new URL("./🟦️.ts", testSource.url)), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const declaration = source.statements.find(statement => ts.isClassDeclaration(statement) && statement.name?.text === row.metadata.className); if (!declaration || !ts.isClassDeclaration(declaration)) throw new Error("Actual captured activation declaration missing");
    const fields = declaration.members.filter(ts.isPropertyDeclaration).map(member => member.name.getText(source)); expect(fields).toEqual(row.metadata.fields);
    expect(row.metadata.facadeBytes).toBe(row.metadata.recordBytes + row.metadata.fieldBytes * fields.length); expect(row.metadata.addedPrivateReferences).toBe(fields.filter(name => name.startsWith("#")).length);
    expect(row.metadata.allocationAdmitted).toBe(false); expect(row.metadata.controllerFundsActivation).toBe(false); local.client.disposeAll(); foreign.client.disposeAll();
  });
  it("ShardResidentComposition retains original ownership while revoking replaced routes and workers", async () => {
    const fixture = await loadActorCompositionFixture(); const { produce } = await import("immer"); const row = fixture.activationBinding;
    for (const vector of row.transitions) {
      const ledger = new OwnedResidentLedger(fixture.capacity); const { client, workers } = harness(2, { residentLedger: ledger, exclusiveShardCount: 1 }); const { client: foreign } = harness(1, { residentLedger: ledger });
      const activate = async (worker: FakeShardWorker) => { const pending = client.activate(row.actorId, "https://fixture.invalid/actor.js", [], BUDGET); await flushAdmission(); const request = worker.sent.at(-1) as { requestId: string }; worker.deliver({ kind: "result", requestId: request.requestId, ok: true, value: undefined }); await pending; return client.captureActorActivation(row.actorId); };
      const lease = await activate(workers[0]!); const before = { ownBefore: ShardClient.matchesActivation(client, lease), foreignBefore: ShardClient.matchesActivation(foreign, lease), activeBefore: true };
      expect(() => lease.assertActive()).not.toThrow();
      if (vector.action === "reassign-route") client.leaseExclusive(row.actorId);
      else if (vector.action === "reactivate-name") { client.dispose(row.actorId); const replacement = await activate(workers[0]!); expect(ShardClient.matchesActivation(client, replacement)).toBe(true); expect(() => replacement.assertActive()).not.toThrow(); }
      else if (vector.action === "replace-worker") { client.terminate(0); client.rebuild(0); const replacement = await activate(workers.at(-1)!); expect(workers.at(-1)!.index).toBe(workers[0]!.index); expect(ShardClient.matchesActivation(client, replacement)).toBe(true); expect(() => replacement.assertActive()).not.toThrow(); }
      else client.disposeAll();
      let activeAfter = true; try { lease.assertActive(); } catch { activeAfter = false; }
      const posts = workers.reduce((count, worker) => count + worker.sent.length, 0); await expect(lease.turn([], BUDGET)).rejects.toThrow("actor-activation.revoked");
      const actual = { ...before, ownAfter: ShardClient.matchesActivation(client, lease), foreignAfter: ShardClient.matchesActivation(foreign, lease), activeAfter, newTurns: workers.reduce((count, worker) => count + worker.sent.length, 0) - posts };
      const oracle = produce({ ...before, ownAfter: before.ownBefore, foreignAfter: before.foreignBefore, activeAfter: true, newTurns: 0 }, state => { state.activeAfter = false; });
      expect(actual, vector.action).toEqual(vector.expected); expect(actual).toEqual(oracle); client.disposeAll(); foreign.disposeAll();
    }
  });
  it("ShardResidentComposition preadmits the exact shared pool record without reusing a child grant", async () => {
    const fixture = await loadActorCompositionFixture(); const { produce } = await import("immer");
    const { readFile } = await import("node:fs/promises"); const ts = await import("typescript");
    const ledger = new OwnedResidentLedger(fixture.capacity); const foreign = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); const row = fixture.poolPreparation;
    expect(client.prepareUiResidentPool(foreign, { maxItems: 1, maxBytes: 4096 }).kind).toBe("rejected"); expect(foreign.usage.data).toEqual({ bytes: 0, slots: 0, owners: 0 });
    for (const refused of [{ maxItems: 0, maxBytes: 296 }, { maxItems: 1, maxBytes: 295 }]) expect(client.prepareUiResidentPool(ledger, refused).kind).toBe("blocked");
    expect(ledger.usage.data).toEqual({ bytes: 0, slots: 0, owners: 0 }); prepareResidentFixture(client, ledger, row.prepareBytes);
    expect(ledger.usage.data).toEqual(row.total); expect(uiResidentMetadataEnvelope("pool")).toEqual(row.uiEnvelope);
    const expected = produce({ bytes: 0, slots: 0, owners: 0 }, value => { for (const envelope of [row.controllerEnvelope, row.uiEnvelope, row.intrinsicEnvelope, row.cellEnvelope, row.intrinsicEnvelope, row.cellEnvelope]) { value.bytes += envelope.bytes; value.slots += envelope.slots; value.owners += envelope.owners; } }); expect(expected).toEqual(row.total);
    const source = ts.createSourceFile("shard.ts", readModuleTreeSource(new URL("./🟦️.ts", testSource.url)), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const declaration = source.statements.find(statement => ts.isClassDeclaration(statement) && statement.name?.text === "ResidentAdmissionState"); if (!declaration || !ts.isClassDeclaration(declaration)) throw new Error("Actual resident admission state declaration missing");
    const fields = declaration.members.filter(ts.isPropertyDeclaration).map(member => member.name.getText(source)).filter(name => name.startsWith("uiResident") || name === "clientAdmissionPurpose");
    expect(fields).toEqual(row.controllerFields); expect(row.controllerEnvelope.bytes).toBe(row.controllerModel.recordBytes + row.controllerModel.fieldBytes * fields.length);
    expect(client.prepareUiResidentPool(ledger, { maxItems: 1, maxBytes: 64 })).toMatchObject({ kind: "ready", items: 0, bytes: row.samePreparedBytes });
    expect(client.prepareUiResidentPool(foreign, { maxItems: 1, maxBytes: 4096 }).kind).toBe("rejected"); expect(client.ownsUiResidentPool({})).toBe(false);
    for (const bytes of fixture.unusedClose.releaseBytes) client.closeUiResidentPoolStep({ maxItems: 1, maxBytes: bytes }); expect(ledger.usage.data).toEqual(fixture.poolPreparation.controllerTotal); client.disposeAll();
  });
  it("ShardResidentComposition releases only its actual pool's private terminal witness", async () => {
    const fixture = await loadActorCompositionFixture();
    const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); const { client: foreign } = harness(1, { residentLedger: ledger }); const grant = { maxItems: 1, maxBytes: 4096 }; const row = fixture.poolLifecycle;
    prepareResidentFixture(client, ledger, fixture.poolPreparation.prepareBytes); const admitted = OwnedUiResidentPool.begin(client, ledger, grant); expect(admitted.step.kind).toBe("ready"); const pool = admitted.pool; if (!pool) throw new Error("Actual pool admission missing");
    expect(Object.keys(pool)).toEqual(row.publicCapabilityKeys); expect(client.ownsUiResidentPool(pool)).toBe(true); expect(OwnedUiResidentPool.begin(client, ledger, grant).step.kind).toBe(row.postConstructionRepeat);
    expect(client.releaseUiResidentPool(pool, pool.retirement, grant).kind).toBe(row.premature);
    let reads = 0; const fabricated = { get terminal() { reads++; return true; } }; expect(client.releaseUiResidentPool(pool, fabricated, grant).kind).toBe(row.structural); expect(reads).toBe(0);
    prepareResidentFixture(foreign, ledger, fixture.poolPreparation.prepareBytes); const other = OwnedUiResidentPool.begin(foreign, ledger, grant).pool; if (!other) throw new Error("Foreign actual pool missing"); other.beginClose(); expect(other.closeStep(grant).kind).toBe("complete");
    expect(client.releaseUiResidentPool(pool, other.retirement, grant).kind).toBe(row.foreign); expect(ledger.usage.data.bytes).toBe(fixture.poolPreparation.total.bytes * 2);
    pool.beginClose(); expect(pool.closeStep(grant).kind).toBe("complete"); const witness = pool.retirement; expect(witness).not.toBeNull();
    for (const refused of [{ maxItems: 0, maxBytes: 4096 }, { maxItems: 1, maxBytes: 63 }]) expect(client.releaseUiResidentPool(pool, witness, refused).kind).toBe("blocked");
    for (let index = 0; index < row.releaseBytes.length; index++) { const result = client.releaseUiResidentPool(pool, witness, { maxItems: 1, maxBytes: row.releaseBytes[index]! }); expect(result).toMatchObject({ kind: row.releaseKinds[index], bytes: row.releaseBytes[index], items: 1 }); }
    expect(client.ownsUiResidentPool(pool)).toBe(false); expect(client.releaseUiResidentPool(pool, witness, grant).kind).toBe(row.replay); expect(ledger.usage.data.bytes).toBe(fixture.poolPreparation.total.bytes + fixture.poolPreparation.controllerTotal.bytes);
    for (const bytes of row.releaseBytes) foreign.releaseUiResidentPool(other, other.retirement, { maxItems: 1, maxBytes: bytes }); expect(ledger.usage.data).toEqual({ bytes: fixture.poolPreparation.controllerTotal.bytes * 2, slots: fixture.poolPreparation.controllerTotal.slots * 2, owners: fixture.poolPreparation.controllerTotal.owners * 2 }); client.disposeAll(); foreign.disposeAll();
  });
  it("ShardResidentComposition retains a rejected record and its original fault in the same cell", async () => {
    const { OwnedResidentRecord } = await import("@semio-tech/value-ts/resident"); const fixture = await loadActorCompositionFixture(); const { produce } = await import("immer");
    const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); const { client: peer } = harness(1, { residentLedger: ledger }); const grant = { maxItems: 1, maxBytes: 4096 }; const row = fixture.rejectedPreparation;
    prepareResidentFixture(peer, ledger, fixture.poolPreparation.prepareBytes); const before = ledger.usage.data; prepareResidentFixture(client, ledger, fixture.poolPreparation.prepareBytes.slice(0, fixture.poolPreparation.controllerPrepareBytes.length + 4));
    const original: { record: OwnedResidentRecord | null } = { record: null }; const freeze = Object.freeze;
    const trap = vi.spyOn(Object, "freeze").mockImplementation(value => { const frozen = freeze(value); if (value instanceof OwnedResidentRecord) { original.record = value; throw null; } return frozen; });
    let refused: ResidentStep; try { refused = client.prepareUiResidentPool(ledger, grant); } finally { trap.mockRestore(); }
    expect(original.record).not.toBeNull(); expect(refused).toMatchObject({ kind: row.step, items: 1, bytes: 264 });
    expect(client.prepareUiResidentPool(ledger, { maxItems: 1, maxBytes: 64 }).kind).toBe(row.retry);
    const doubled = produce({ ...before }, usage => { usage.bytes *= row.recordsAfterRefusal; usage.slots *= row.recordsAfterRefusal; usage.owners *= row.recordsAfterRefusal; }); expect(ledger.usage.data).toEqual(doubled);
    expect(OwnedUiResidentPool.begin(client, ledger, grant).pool).toBe(row.pool);
    for (const blocked of [{ maxItems: 0, maxBytes: 4096 }, { maxItems: 1, maxBytes: 63 }]) expect(client.closeUiResidentPoolStep(blocked).kind).toBe("blocked");
    for (let index = 0; index < row.releaseBytes.length; index++) { const bytes = row.releaseBytes[index]!; const current = client.closeUiResidentPoolStep({ maxItems: 1, maxBytes: Math.max(64, bytes) }); expect(current).toMatchObject({ kind: row.releaseKinds[index], items: bytes ? 1 : 0, bytes }); }
    expect(original.record!.terminalIsEmpty()).toBe(row.resourceEmptyAfterAlias);
    expect(ledger.usage.data).toEqual(produce({ ...before }, usage => { usage.bytes += row.retained.bytes; usage.slots += row.retained.slots; usage.owners += row.retained.owners; }));
    expect(peer.prepareUiResidentPool(ledger, grant).kind).toBe("ready");
    for (const bytes of fixture.unusedClose.releaseBytes) peer.closeUiResidentPoolStep({ maxItems: 1, maxBytes: bytes }); expect(ledger.usage.data).toEqual(row.survivingControllersWithFault); client.disposeAll(); peer.disposeAll();
  });
  it("ShardResidentComposition parent closes its installed pool with separate child and proof turns", async () => {
    const fixture = await loadActorCompositionFixture(); const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); const grant = { maxItems: 1, maxBytes: 4096 }; const row = fixture.parentClose;
    prepareResidentFixture(client, ledger, fixture.poolPreparation.prepareBytes); const pool = OwnedUiResidentPool.begin(client, ledger, grant).pool; if (!pool) throw new Error("Original pool missing");
    const close = vi.spyOn(OwnedUiResidentPool.prototype, "closeStep"); const retirement = spyGetter(OwnedUiResidentPool.prototype, "retirement");
    try {
      for (let index = 0; index < row.releaseBytes.length; index++) {
        close.mockClear(); retirement.mockClear(); const current = client.closeUiResidentPoolStep({ maxItems: 1, maxBytes: row.releaseBytes[index]! });
        expect(current).toMatchObject({ kind: row.releaseKinds[index], items: 1, bytes: row.releaseBytes[index] }); expect(close.mock.calls.length).toBeLessThanOrEqual(row.maxChildCallsPerTurn); if (close.mock.calls.length) expect(retirement.mock.calls.length).toBe(0);
      }
      expect(pool.terminalIsEmpty()).toBe(true); expect(client.ownsUiResidentPool(pool)).toBe(false); expect(ledger.usage.data).toEqual(fixture.poolPreparation.controllerTotal); expect(client.closeUiResidentPoolStep(grant)).toMatchObject({ kind: "complete", items: 0, bytes: row.retryBytes });
    } finally { close.mockRestore(); retirement.mockRestore(); client.disposeAll(); }
  });
  it("ShardResidentComposition preserves every thrown value after an actual parent close transition", async () => {
    const fixture = await loadActorCompositionFixture(); const { OwnedResidentRecord } = await import("@semio-tech/value-ts/resident"); const row = fixture.parentFault; const grant = { maxItems: 1, maxBytes: 4096 };
    let getterReads = 0; const values = new Map<string, unknown>([["null", null], ["undefined", undefined], ["false", false], ["zero", 0], ["object", { payload: new Uint8Array(8193), get message() { getterReads++; return "unread"; } }]]);
    for (const name of row.values) {
      const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); const { client: peer } = harness(1, { residentLedger: ledger }); prepareResidentFixture(client, ledger, fixture.poolPreparation.prepareBytes); const before = ledger.usage.data;
      const begin = OwnedResidentRecord.prototype.beginClose; const fault = values.get(name); let transitions = 0;
      const trap = vi.spyOn(OwnedResidentRecord.prototype, "beginClose").mockImplementation(function (this: OwnedResidentRecord) { Reflect.apply(begin, this, []); transitions++; throw fault; });
      try { expect(client.closeUiResidentPoolStep(grant).kind).toBe(row.first); } finally { trap.mockRestore(); }
      expect(transitions).toBe(1); expect(client.closeUiResidentPoolStep(grant)).toMatchObject({ kind: row.retry, phase: row.phase, items: 1, bytes: row.handoffBytes }); expect(ledger.usage.data).toEqual(before); expect(getterReads).toBe(row.getterReads);
      prepareResidentFixture(peer, ledger, fixture.poolPreparation.prepareBytes); for (const bytes of fixture.unusedClose.releaseBytes) peer.closeUiResidentPoolStep({ maxItems: 1, maxBytes: bytes }); expect(ledger.usage.data.bytes).toBe(before.bytes + fixture.poolPreparation.controllerTotal.bytes);
      for (const bytes of [264,64,64]) expect(client.closeUiResidentPoolStep({ maxItems: 1, maxBytes: bytes }).kind).toBe("pending");
      expect(client.closeUiResidentPoolStep(grant)).toMatchObject({ kind: "rejected", phase: "resident-admission-fault-held" }); expect(ledger.usage.data).toEqual(fixture.rejectedPreparation.survivingControllersWithFault); client.disposeAll(); peer.disposeAll();
    }
  });
  it("ShardResidentComposition contains a fault after the actual private detachment observation", async () => {
    const fixture = await loadActorCompositionFixture(); const { OwnedResidentRecord } = await import("@semio-tech/value-ts/resident"); const row = fixture.observationFault;
    const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); const grant = { maxItems: 1, maxBytes: 4096 };
    prepareResidentFixture(client, ledger, fixture.poolPreparation.prepareBytes); const pool = OwnedUiResidentPool.begin(client, ledger, grant).pool; if (!pool) throw new Error("Exact original pool missing"); pool.beginClose(); expect(pool.closeStep(grant).kind).toBe("complete"); const witness = pool.retirement;
    expect(client.releaseUiResidentPool(pool, witness, grant).kind).toBe("pending"); expect(client.releaseUiResidentPool(pool, witness, grant).kind).toBe("pending");
    const read = Object.getOwnPropertyDescriptor(OwnedResidentRecord.prototype, "detachment")!.get!; let observed = false;
    const trap = spyGetter(OwnedResidentRecord.prototype, "detachment").mockImplementation(function (this: OwnedResidentRecord) { observed = Reflect.apply(read, this, []) !== null; throw null; });
    let escaped = false; let current: ResidentStep | null = null;
    try { current = client.releaseUiResidentPool(pool, witness, grant); } catch { escaped = true; } finally { trap.mockRestore(); }
    expect(observed).toBe(row.afterActualDetachmentRead); expect(escaped).toBe(row.escapes); expect(current).toMatchObject({ kind: row.first });
    expect(client.releaseUiResidentPool(pool, witness, grant).kind).toBe(row.faultHandoff);
    expect(client.releaseUiResidentPool(pool, witness, grant).kind).toBe(row.recoveredObservation);
    for (const bytes of [264,64,64]) expect(client.releaseUiResidentPool(pool, witness, { maxItems: 1, maxBytes: bytes }).kind).toBe("pending");
    expect(client.releaseUiResidentPool(pool, witness, grant).kind).toBe(row.final); expect(client.ownsUiResidentPool(pool)).toBe(row.poolStillOwned); expect(ledger.usage.data).toEqual(row.retained); client.disposeAll();
  });
  it("ShardResidentComposition recovers original bootstrap claim and record results after wrapper throws", async () => {
    const fixture = await loadActorCompositionFixture(); const row = fixture.admissionWrappers;
    let getterReads = 0; const values = new Map<string, unknown>([["null", null], ["undefined", undefined], ["false", false], ["zero", 0], ["object", { payload: new Uint8Array(8193), get message() { getterReads++; return "unread"; } }]]);
    for (const scope of row.scopes) for (const stage of row.stages) for (const name of row.values) {
      const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); const { client: peer } = harness(1, { residentLedger: ledger }); const grant = { maxItems: 1, maxBytes: 4096 }; const fault = values.get(name);
      const held: { cell: OwnedResidentAdmission | null; record: OwnedResidentRecord | null } = { cell: null, record: null };
      const count = (scope === "pool" ? fixture.poolPreparation.controllerPrepareBytes.length : 0) + (stage === "bootstrap" ? 0 : stage === "claim" ? 2 : 4); prepareResidentFixture(client, ledger, fixture.poolPreparation.prepareBytes.slice(0,count)); let calls = 0;
      const originalPrepare = OwnedResidentLedger.prototype.prepareAdmission; const originalClaim = OwnedResidentLedger.prototype.claimAdmission; const originalRecord = OwnedResidentLedger.prototype.reserveRecord;
      const trap = stage === "bootstrap" ? vi.spyOn(OwnedResidentLedger.prototype,"prepareAdmission").mockImplementation(function(this: OwnedResidentLedger,...args) { const result = Reflect.apply(originalPrepare,this,args); calls++; held.cell = this.preparedAdmission(client); throw fault; })
        : stage === "claim" ? vi.spyOn(OwnedResidentLedger.prototype,"claimAdmission").mockImplementation(function(this: OwnedResidentLedger,...args) { held.cell = args[1]; Reflect.apply(originalClaim,this,args); calls++; throw fault; })
        : vi.spyOn(OwnedResidentLedger.prototype,"reserveRecord").mockImplementation(function(this: OwnedResidentLedger,...args) { held.cell = args[2]; const result = Reflect.apply(originalRecord,this,args); held.record = result.record; calls++; throw fault; });
      try { expect(client.prepareUiResidentPool(ledger,grant).kind).toBe(row.first); } finally { trap.mockRestore(); }
      expect(calls).toBe(row.resourceCalls); if (!held.cell) throw new Error("Original admission cell lost");
      const cell = held.cell; for (let index = 0; index < fixture.poolPreparation.prepareBytes.length + fixture.unusedClose.releaseBytes.length; index++) { const current = client.closeUiResidentPoolStep(grant); if (current.kind === "rejected" && (current.phase === "resident-admission-fault-held" || current.phase === "actor-resident.controller-fault-held")) break; }
      expect(cell.hasFailure).toBe(true); expect(Object.is(cell.failure,fault)).toBe(true); expect(cell.retirement).toBeNull(); const retained = scope === "pool" ? fixture.rejectedPreparation.retained : stage === "record" ? fixture.poolPreparation.controllerTotal : row.controllerCellFault; expect(ledger.usage.data).toEqual(retained); expect(getterReads).toBe(row.faultsInspected);
      if (held.record) { expect(held.record.terminalIsEmpty()).toBe(scope === "pool"); expect(cell.result?.record).toBe(scope === "pool" ? null : held.record); }
      prepareResidentFixture(peer,ledger,fixture.poolPreparation.prepareBytes); for (const bytes of fixture.unusedClose.releaseBytes) peer.closeUiResidentPoolStep({maxItems:1,maxBytes:bytes}); expect(ledger.usage.data).toEqual({ bytes:retained.bytes+fixture.poolPreparation.controllerTotal.bytes, slots:retained.slots+fixture.poolPreparation.controllerTotal.slots, owners:retained.owners+fixture.poolPreparation.controllerTotal.owners }); client.disposeAll(); peer.disposeAll();
    }
  });
  it("ShardResidentComposition waits for exact result aliases and cell retirement before final release", async () => {
    const fixture = await loadActorCompositionFixture(); const row = fixture.aliasRetirement; const ledger = new OwnedResidentLedger(fixture.capacity); const {client} = harness(1,{residentLedger:ledger});
    const held: { cell: OwnedResidentAdmission | null; record: OwnedResidentRecord | null } = {cell:null,record:null}; const original = OwnedResidentLedger.prototype.reserveRecord;
    const trap = vi.spyOn(OwnedResidentLedger.prototype,"reserveRecord").mockImplementation(function(this:OwnedResidentLedger,...args) { const result=Reflect.apply(original,this,args); held.cell=args[2]; held.record=result.record; return result; });
    try { prepareResidentFixture(client,ledger,fixture.poolPreparation.prepareBytes); } finally { trap.mockRestore(); }
    if(!held.cell || !held.record) throw new Error("Original resource capture absent"); const {cell,record}=held;
    expect(client.closeUiResidentPoolStep({maxItems:1,maxBytes:64}).kind).toBe("pending");
    expect(client.closeUiResidentPoolStep({maxItems:1,maxBytes:row.intrinsicWork}).kind).toBe("pending"); expect(record.terminalIsEmpty()).toBe(row.rootBeforeDetach); expect(cell.result?.record).toBe(record);
    expect(ledger.usage.data.bytes).toBe(fixture.poolPreparation.controllerTotal.bytes+fixture.poolPreparation.cellEnvelope.bytes+row.linkBytes);
    expect(client.closeUiResidentPoolStep({maxItems:1,maxBytes:row.observeWork}).kind).toBe("pending"); expect(record.terminalIsEmpty()).toBe(false);
    expect(client.closeUiResidentPoolStep({maxItems:1,maxBytes:row.detachWork}).kind).toBe("pending"); expect(record.terminalIsEmpty()).toBe(row.rootAfterDetach); expect(cell.result?.record).toBeNull(); expect(cell.terminalIsEmpty()).toBe(row.cellBeforeRefund);
    expect(client.closeUiResidentPoolStep({maxItems:1,maxBytes:row.cellWork}).kind).toBe("pending"); expect(OwnedResidentRetirement.matches(cell.retirement,cell)).toBe(true);
    expect(client.closeUiResidentPoolStep({maxItems:1,maxBytes:row.observeWork}).kind).toBe("complete"); expect(ledger.usage.data).toEqual(fixture.poolPreparation.controllerTotal); client.disposeAll();
  });
  it("ShardResidentComposition cancels every preparation frontier and a closed-ledger refusal", async () => {
    const fixture = await loadActorCompositionFixture(); const row=fixture.cancelledPreparation; const grant={maxItems:1,maxBytes:4096};
    for(const frontier of row.preparationFrontiers) {
      const ledger=new OwnedResidentLedger(fixture.capacity); const {client}=harness(1,{residentLedger:ledger}); prepareResidentFixture(client,ledger,fixture.poolPreparation.prepareBytes.slice(0,frontier));
      let complete=false; for(let index=0;index<fixture.poolPreparation.prepareBytes.length+fixture.unusedClose.releaseBytes.length;index++) { const current=client.closeUiResidentPoolStep(grant); expect(current.items).toBeLessThanOrEqual(1); expect(current.kind).not.toBe("rejected"); if(current.kind==="complete"){complete=true;break;} }
      expect(complete).toBe(true); expect(ledger.usage.data).toEqual(frontier === 0 ? row.unstartedFinal : row.healthyFinal); expect(client.prepareUiResidentPool(ledger,grant).kind).toBe(row.ownerAfterClose); client.disposeAll();
    }
    const ledger=new OwnedResidentLedger(fixture.capacity); ledger.beginClose(); expect(ledger.closeStep(grant).kind).toBe("complete"); const {client}=harness(1,{residentLedger:ledger});
    expect(client.prepareUiResidentPool(ledger,grant).kind).toBe(row.closedLedgerFirst); expect(ledger.usage.data).toEqual(row.unstartedFinal);
    for(const kind of row.closedLedgerRelease) expect(client.closeUiResidentPoolStep(grant).kind).toBe(kind);
    expect(ledger.usage.data).toEqual(row.unstartedFinal); client.disposeAll();
  });
  it("ShardResidentComposition retains actual controller funding after child intrinsic retirement", async () => {
    const fixture = await loadActorCompositionFixture(); const {produce}=await import("immer"); const ledger=new OwnedResidentLedger(fixture.capacity); const {client}=harness(1,{residentLedger:ledger});
    const original=OwnedResidentLedger.prototype.reserveRecord; const held:{record:OwnedResidentRecord|null}={record:null};
    const trap=vi.spyOn(OwnedResidentLedger.prototype,"reserveRecord").mockImplementation(function(this:OwnedResidentLedger,...args){ const result=Reflect.apply(original,this,args); if(args[1]===poolControllerEnvelope)held.record=result.record; return result; });
    try { prepareResidentFixture(client,ledger,fixture.poolPreparation.controllerPrepareBytes); } finally { trap.mockRestore(); }
    if(!held.record) throw new Error("Original controller record missing"); const controller=held.record;
    expect(controller.matchesShell(client)).toBe(fixture.controllerCharge.installExactOriginalShard); expect(controller.retirement).toBeNull(); expect(ledger.usage.data).toEqual(fixture.controllerCharge.controllerBeforePool);
    prepareResidentFixture(client,ledger,fixture.poolPreparation.poolPrepareBytes);
    expect(client.closeUiResidentPoolStep({maxItems:1,maxBytes:64}).kind).toBe("pending");
    expect(client.closeUiResidentPoolStep({maxItems:1,maxBytes:fixture.aliasRetirement.intrinsicWork}).kind).toBe("pending");
    const minimum=produce({bytes:0,slots:0,owners:0},value=>{ for(const envelope of [fixture.poolPreparation.controllerEnvelope,fixture.poolPreparation.cellEnvelope]){value.bytes+=envelope.bytes;value.slots+=envelope.slots;value.owners+=envelope.owners;} });
    expect(ledger.usage.data.bytes).toBeGreaterThanOrEqual(minimum.bytes); expect(ledger.usage.data.slots).toBeGreaterThanOrEqual(minimum.slots); expect(ledger.usage.data.owners).toBeGreaterThanOrEqual(minimum.owners);
    for(const bytes of fixture.unusedClose.releaseBytes.slice(2)) client.closeUiResidentPoolStep({maxItems:1,maxBytes:bytes});
    expect(controller.terminalIsEmpty()).toBe(fixture.controllerCharge.wholeControllerTerminal); expect(controller.matchesShell(client)).toBe(true); expect(ledger.usage.data).toEqual(fixture.poolPreparation.controllerTotal);
    client.disposeAll(); expect(controller.retirement).toBeNull(); expect(ledger.usage.data).toEqual(fixture.poolPreparation.controllerTotal);
  });
}
