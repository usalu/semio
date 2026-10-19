/** 🚀️ The shard worker bootstrap registration tests. */
import { loadActorCompositionBootstrapFixture, loadActorCompositionFixture } from "@semio-tech/framework-actor/testing/fixtures";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import type { OwnedResidentAdmission, OwnedResidentLedger, OwnedResidentRecord, ShardClient, ShardClientSettlementTestKit } from "../🧰️settlement-test-kit/🟦️.ts";

/** 🥾 Registers the original metadata preparation, prefix cancellation, gate ending and first value retention tests of the shard worker bootstrap. */
export function registerShardWorkerBootstrapTests(kit: ShardClientSettlementTestKit): void {
  const { OwnedResidentLedger, OwnedResidentRetirement, OwnedUiResidentPool, expect, harness, it, poolControllerEnvelope, poolUiEnvelope, prepareResidentFixture, testSource, vi } = kit;
  async function workerPreparationFixture() { return (await loadActorCompositionBootstrapFixture()); }
  function prepareWorkerFixture(client: ShardClient, rows: readonly (readonly (string | number)[])[]): void {
    for (const row of rows) { const bytes = Number(row[1]); expect(client.prepareWorkerBootstrap({ maxItems: 1, maxBytes: bytes }), String(row[0])).toMatchObject({ kind: "pending", items: 1, bytes }); }
  }
  it("ShardWorkerBootstrap declares only original metadata preparation and close methods", async () => {
    const fixture = await loadActorCompositionBootstrapFixture();
    
    const { residentSchema } = await import("@semio-tech/value-ts/resident/schema"); const { default: Ajv } = await import("ajv");
    
    const { client, workers } = harness(1, { residentLedger: new OwnedResidentLedger(fixture.capacity) }); const posts = workers[0]!.sent.length;
    expect(typeof Reflect.get(client, fixture.methods.prepare)).toBe("function"); expect(typeof Reflect.get(client, fixture.methods.close)).toBe("function");
    expect(workers).toHaveLength(1); expect(workers[0]!.sent.length).toBe(posts); client.disposeAll();
  });
  for (const prefix of [7, 8]) for (const closing of ["ledger", "record", "cell", "fault"]) it(`ShardWorkerBootstrap shared closing prefix ${prefix} ${closing} cannot admit a UI descendant`, async () => {
    const fixture = await loadActorCompositionBootstrapFixture();
    const { OwnedResidentRecord } = await import("@semio-tech/value-ts/resident");
    const ledger = new OwnedResidentLedger(fixture.capacity); const { client, workers } = harness(1, { residentLedger: ledger });
    const held: { cell: OwnedResidentAdmission | null; record: OwnedResidentRecord | null } = { cell: null, record: null };
    const reserve = OwnedResidentLedger.prototype.reserveRecord;
    const capture = vi.spyOn(OwnedResidentLedger.prototype, "reserveRecord").mockImplementation(function (this: OwnedResidentLedger, ...args) { const result = Reflect.apply(reserve, this, args); if (args[1] === poolControllerEnvelope) { held.cell = args[2]; held.record = result.record; } return result; });
    try { for (const [, amount] of fixture.shared.phases.slice(0, prefix)) expect(client.prepareUiResidentPool(ledger, { maxItems: 1, maxBytes: Number(amount) }).kind).toBe("pending"); } finally { capture.mockRestore(); }
    const cell = held.cell, record = held.record; if (!cell || !record) throw new Error("Original shared admission not captured");
    expect(record.matchesShell(client)).toBe(true); const before = ledger.usage; const posts = workers[0]!.sent.length;
    if (closing === "ledger") ledger.beginClose(); else if (closing === "record") record.beginClose(); else if (closing === "cell") cell.beginClose(); else expect(cell.retainFailure({ original: closing }, { maxItems: 1, maxBytes: 64 }).kind).toBe("pending");
    expect(record.matchesShell(client)).toBe(true); expect(record.matchesLiveShell(client)).toBe(false);
    const prepare = vi.spyOn(OwnedResidentLedger.prototype, "prepareAdmission");
    try { expect(client.prepareUiResidentPool(ledger, { maxItems: 1, maxBytes: prefix === 7 ? 64 : 296 }).kind).toBe("rejected"); expect(prepare).not.toHaveBeenCalled(); } finally { prepare.mockRestore(); }
    expect(cell.result?.record).toBe(record); expect(record.retirement).toBeNull(); expect(ledger.usage).toEqual(before); expect(workers).toHaveLength(1); expect(workers[0]!.sent.length).toBe(posts); client.disposeAll();
  });
  it("ShardWorkerBootstrap uses separately granted original records and the declared actual field census", async () => {
    const fixture = await workerPreparationFixture(); const { produce } = await import("immer"); const ts = await import("typescript"); const { readFile } = await import("node:fs/promises");
    const ledger = new OwnedResidentLedger(fixture.capacity); const { client, workers } = harness(1, { residentLedger: ledger }); const posts = workers[0]!.sent.length;
    const messageHandler = workers[0]!.onmessage; const errorHandler = workers[0]!.onerror;
    const held: { record: OwnedResidentRecord; cell: OwnedResidentAdmission }[] = []; const original = OwnedResidentLedger.prototype.reserveRecord;
    const capture = vi.spyOn(OwnedResidentLedger.prototype, "reserveRecord").mockImplementation(function (this: OwnedResidentLedger, ...args) { const result = Reflect.apply(original, this, args); if (result.record) held.push({ record: result.record, cell: args[2] }); return result; });
    try {
      for (const rows of [fixture.shared.phases, fixture.worker.phases]) {
        for (const row of rows) {
          const before = ledger.usage; const bytes = Number(row[1]);
          expect(client.prepareWorkerBootstrap({ maxItems: 0, maxBytes: bytes })).toMatchObject({ kind: "blocked", items: 0, bytes: 0 }); expect(ledger.usage).toEqual(before);
          expect(client.prepareWorkerBootstrap({ maxItems: 1, maxBytes: bytes - 1 })).toMatchObject({ kind: "blocked", items: 0, bytes: 0 }); expect(ledger.usage).toEqual(before);
          expect(client.prepareWorkerBootstrap({ maxItems: 1, maxBytes: bytes }), String(row[0])).toMatchObject({ kind: "pending", items: 1, bytes });
        }
        expect(ledger.usage.data).toEqual(rows === fixture.shared.phases ? fixture.shared.retained : fixture.worker.combined);
      }
    } finally { capture.mockRestore(); }
    expect(held).toHaveLength(2); for (const entry of held) { expect(entry.cell.result?.record).toBe(entry.record); expect(entry.record.matchesLiveShell(client)).toBe(true); expect(entry.record.retirement).toBeNull(); }
    expect(client.prepareWorkerBootstrap({ maxItems: 1, maxBytes: 64 })).toMatchObject({ kind: "ready", items: 0, bytes: 0 });
    expect(fixture.worker.combined).toEqual(produce({ ...fixture.shared.retained }, value => { value.bytes += fixture.worker.retained.bytes; value.slots += fixture.worker.retained.slots; value.owners += fixture.worker.retained.owners; }));
    const source = ts.createSourceFile("shard.ts", readModuleTreeSource(new URL("./🟦️.ts", testSource.url)), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const declaration = source.statements.find(statement => ts.isClassDeclaration(statement) && statement.name?.text === "ResidentAdmissionState"); if (!declaration || !ts.isClassDeclaration(declaration)) throw new Error("Original resident admission state class missing");
    const fields = declaration.members.filter(ts.isPropertyDeclaration).map(member => member.name.getText(source).replace(/^#/, ""));
    expect(fields.filter(name => name.startsWith("uiResident") || name === "clientAdmissionPurpose")).toEqual(fixture.shared.fields);
    expect(fields.filter(name => name.startsWith("workerBootstrap") || name.startsWith("workerAdmission"))).toEqual(fixture.worker.fields);
    expect(fixture.shared.envelope.bytes).toBe(64 + 16 * fixture.shared.fields.length); expect(fixture.worker.envelope.bytes).toBe(16 * fixture.worker.fields.length);
    expect(workers).toHaveLength(1 + fixture.worker.newWorkers); expect(workers[0]!.sent.length).toBe(posts + fixture.worker.newPosts);
    expect(workers[0]!.onmessage).toBe(messageHandler); expect(workers[0]!.onerror).toBe(errorHandler);
    expect(client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 64 }).kind).toBe("pending"); expect(client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 296 }).kind).toBe("blocked");
    expect(ledger.usage.data).toEqual(fixture.worker.combined); client.disposeAll(); expect(ledger.usage.data).toEqual(fixture.worker.combined);
  });
  it("ShardWorkerBootstrap cancels each exact prefix without releasing an admitted worker record", async () => {
    const fixture = await workerPreparationFixture(); const phases = [...fixture.shared.phases, ...fixture.worker.phases];
    for (const row of fixture.cancelFrontiers) {
      const ledger = new OwnedResidentLedger(fixture.capacity); const { client, workers } = harness(1, { residentLedger: ledger }); const posts = workers[0]!.sent.length;
      prepareWorkerFixture(client, phases.slice(0, row.prefix)); let completed = false;
      for (let turn = 0; turn < fixture.worker.phases.length; turn++) {
        const current = client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 296 }); expect(current.items).toBeLessThanOrEqual(1); expect(current.bytes).toBeLessThanOrEqual(296);
        expect(client.prepareWorkerBootstrap({ maxItems: 1, maxBytes: 296 }).kind, `restart ${row.prefix}`).toBe(row.workerRestart);
        if (current.kind === "complete") { completed = true; break; } if (current.kind === "blocked") break;
        expect(current.kind, `close ${row.prefix}/${turn}`).toBe("pending");
      }
      expect(completed, `prefix ${row.prefix}`).toBe(!row.recordHeld);
      if (row.prefix >= fixture.shared.phases.length) expect(ledger.usage.data.bytes, `retained ${row.prefix}`).toBe(fixture.shared.retained.bytes + row.workerRetained);
      let prepared = false;
      for (let turn = 0; turn <= phases.length; turn++) {
        const current = client.prepareUiResidentPool(ledger, { maxItems: 1, maxBytes: 296 }); if (current.kind === "ready") { prepared = true; break; } expect(current.kind, `UI after worker prefix ${row.prefix}/${turn}`).toBe("pending");
      }
      expect(prepared).toBe(true); expect(ledger.usage.data).toEqual({ bytes: fixture.ui.sharedAndPool.bytes + row.workerRetained, slots: fixture.ui.sharedAndPool.slots + (row.recordHeld ? fixture.worker.retained.slots : 0), owners: fixture.ui.sharedAndPool.owners + (row.recordHeld ? fixture.worker.retained.owners : 0) });
      expect(workers).toHaveLength(1); expect(workers[0]!.sent.length).toBe(posts); client.disposeAll();
    }
  });
  it("ShardWorkerBootstrap shares the original prefix and preserves UI-close-then-worker ownership", async () => {
    const fixture = await workerPreparationFixture(); const uiFixture = await loadActorCompositionFixture();
    for (const prefix of [0, ...fixture.shared.phases.map((_, index) => index + 1)]) {
      const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger });
      for (let index = 0; index < fixture.shared.phases.length; index++) {
        const grant = { maxItems: 1, maxBytes: Number(fixture.shared.phases[index]![1]) };
        expect((index < prefix ? client.prepareUiResidentPool(ledger, grant) : client.prepareWorkerBootstrap(grant)).kind).toBe("pending");
      }
      expect(ledger.usage.data).toEqual(fixture.shared.retained); prepareWorkerFixture(client, fixture.worker.phases); expect(ledger.usage.data).toEqual(fixture.worker.combined); client.disposeAll();
    }
    for (const preparedPool of [false, true]) {
      const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger });
      if (preparedPool) { prepareResidentFixture(client, ledger, uiFixture.poolPreparation.prepareBytes); expect(OwnedUiResidentPool.begin(client, ledger, { maxItems: 1, maxBytes: poolUiEnvelope.bytes + 64 }).step.kind).toBe("ready"); }
      let closed = false;
      for (let turn = 0; turn < uiFixture.poolPreparation.prepareBytes.length + uiFixture.unusedClose.releaseBytes.length; turn++) { const current = client.closeUiResidentPoolStep({ maxItems: 1, maxBytes: 592 }); if (current.kind === "complete") { closed = true; break; } expect(current.kind).toBe("pending"); }
      expect(closed).toBe(true); expect(ledger.usage.data.bytes).toBe(preparedPool ? fixture.shared.retained.bytes : 0);
      if (!preparedPool) { expect(client.prepareWorkerBootstrap({ maxItems: 1, maxBytes: 64 }).kind).toBe("pending"); prepareWorkerFixture(client, fixture.shared.phases); }
      prepareWorkerFixture(client, fixture.worker.phases); expect(client.prepareWorkerBootstrap({ maxItems: 1, maxBytes: 64 }).kind).toBe("ready");
      expect(client.prepareUiResidentPool(ledger, { maxItems: 1, maxBytes: 296 }).kind).toBe("rejected"); expect(ledger.usage.data).toEqual(fixture.worker.combined); client.disposeAll();
    }
  });
  it("ShardWorkerBootstrap holds each original purpose until claim or exact unused-cell retirement", async () => {
    const fixture = await workerPreparationFixture();
    for (const first of ["worker", "ui"]) for (const cancel of [false, true]) {
      const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); prepareWorkerFixture(client, fixture.shared.phases);
      const prepare = (bytes: number) => first === "worker" ? client.prepareWorkerBootstrap({ maxItems: 1, maxBytes: bytes }) : client.prepareUiResidentPool(ledger, { maxItems: 1, maxBytes: bytes });
      const other = () => first === "worker" ? client.prepareUiResidentPool(ledger, { maxItems: 1, maxBytes: 296 }) : client.prepareWorkerBootstrap({ maxItems: 1, maxBytes: 296 });
      expect(prepare(296).kind).toBe("pending"); const cell = ledger.preparedAdmission(client); if (!cell) throw new Error("Original purpose cell missing");
      expect(other().kind).toBe("blocked"); expect(ledger.preparedAdmission(client)).toBe(cell); expect(prepare(64).kind).toBe("pending");
      if (cancel) {
        let complete = false;
        for (let turn = 0; turn < fixture.worker.phases.length; turn++) { const current = first === "worker" ? client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 296 }) : client.closeUiResidentPoolStep({ maxItems: 1, maxBytes: 296 }); if (current.kind === "complete") { complete = true; break; } expect(current.kind).toBe("pending"); }
        expect(complete).toBe(true); expect(OwnedResidentRetirement.matches(cell.retirement, cell)).toBe(true);
      } else { expect(prepare(64).kind).toBe("pending"); expect(cell.claimed).toBe(true); expect(other().kind).toBe("blocked"); expect(prepare(64).kind).toBe("pending"); }
      expect(other().kind, `${first}/${cancel}`).toBe("pending"); expect(ledger.preparedAdmission(client)).not.toBe(cell); client.disposeAll();
    }
  });
  it("ShardWorkerBootstrap executes all declared original-cell gate endings", async () => {
    const fixture = await workerPreparationFixture(); const { OwnedResidentAdmission } = await import("@semio-tech/value-ts/resident"); const completed: string[] = [];
    for (const row of fixture.gateCases) {
      const capacityLimited = row.id.startsWith("capacity-blocked") || row.id === "prepare-after-fault-null-not-empty-proof";
      const ledger = new OwnedResidentLedger(capacityLimited ? { ...fixture.capacity, bytes: fixture.capacity.control.bytes + fixture.shared.retained.bytes } : fixture.capacity);
      const { client, workers } = harness(1, { residentLedger: ledger }); const originalReserve = OwnedResidentLedger.prototype.reserveRecord; const shared: { record: OwnedResidentRecord | null } = { record: null };
      const rootCapture = vi.spyOn(OwnedResidentLedger.prototype, "reserveRecord").mockImplementation(function (this: OwnedResidentLedger, ...args) { const result = Reflect.apply(originalReserve, this, args); if (args[1] === poolControllerEnvelope) shared.record = result.record; return result; });
      try { prepareWorkerFixture(client, fixture.shared.phases); } finally { rootCapture.mockRestore(); }
      if (!shared.record) throw new Error("Original live shared record missing");
      const prepare = (bytes = 64) => client.prepareWorkerBootstrap({ maxItems: 1, maxBytes: bytes }); const close = (bytes = 64) => client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: bytes });
      let cell: OwnedResidentAdmission | null = null; const fault = Object.freeze({ original: row.id });
      const take = () => { cell = ledger.preparedAdmission(client); if (!cell) throw new Error(`Original cell missing: ${row.id}`); };
      const start = () => { expect(prepare(296).kind, row.id).toBe("pending"); take(); expect(prepare().kind).toBe("pending"); };
      const claim = () => { start(); expect(prepare().kind).toBe("pending"); expect(cell!.claimed).toBe(true); expect(ledger.preparedAdmission(client)).toBeNull(); };
      const faultPrepare = () => {
        const original = OwnedResidentLedger.prototype.prepareAdmission; const trap = vi.spyOn(OwnedResidentLedger.prototype, "prepareAdmission").mockImplementation(function (this: OwnedResidentLedger, ...args) { Reflect.apply(original, this, args); throw fault; });
        try { expect(prepare(296).kind).toBe("rejected"); } finally { trap.mockRestore(); }
      };
      const faultClaim = () => {
        start(); const original = OwnedResidentLedger.prototype.claimAdmission; const trap = vi.spyOn(OwnedResidentLedger.prototype, "claimAdmission").mockImplementation(function (this: OwnedResidentLedger, ...args) { Reflect.apply(original, this, args); throw fault; });
        try { expect(prepare().kind).toBe("rejected"); } finally { trap.mockRestore(); }
      };
      switch (row.id) {
        case "short-prepare-no-call": expect(prepare(295)).toMatchObject({ kind: "blocked", items: 0 }); break;
        case "capacity-blocked-before-observation": expect(prepare(296).kind).toBe("blocked"); break;
        case "capacity-blocked-known-no-cell": expect(prepare(296).kind).toBe("blocked"); expect(prepare().phase).toBe("actor-worker.empty-admission-observation"); break;
        case "closed-ledger-known-no-cell": {
          const original = OwnedResidentLedger.prototype.prepareAdmission; const trap = vi.spyOn(OwnedResidentLedger.prototype, "prepareAdmission").mockImplementation(function (this: OwnedResidentLedger, ...args) { this.beginClose(); return Reflect.apply(original, this, args); });
          try { expect(prepare(296).kind).toBe("rejected"); } finally { trap.mockRestore(); }
          expect(prepare().phase).toBe("actor-worker.empty-admission-observation"); break;
        }
        case "foreign-purpose-never-clears-original": expect(client.prepareUiResidentPool(ledger, { maxItems: 1, maxBytes: 296 }).kind).toBe("pending"); take(); expect(prepare(296).kind).toBe("blocked"); expect(ledger.preparedAdmission(client)).toBe(cell); break;
        case "prepare-success-awaits-capture": expect(prepare(296).kind).toBe("pending"); take(); break;
        case "prepare-after-fault-recovers-original": faultPrepare(); take(); expect(prepare().phase).toBe("actor-worker.cell-observation"); expect(prepare().kind).toBe("rejected"); break;
        case "prepare-after-fault-null-not-empty-proof": faultPrepare(); expect(ledger.preparedAdmission(client)).toBeNull(); expect(prepare().phase).toBe("actor-worker.admission-handoff"); break;
        case "claim-short-retains-captured-cell": start(); expect(prepare(63)).toMatchObject({ kind: "blocked", items: 0 }); expect(ledger.preparedAdmission(client)).toBe(cell); break;
        case "claim-known-refusal-needs-cancel": start(); cell!.beginClose(); expect(prepare().kind).toBe("rejected"); expect(ledger.preparedAdmission(client)).toBe(cell); break;
        case "claim-return-without-observation": claim(); break;
        case "claim-observed-private-handoff": claim(); expect(prepare().phase).toBe("actor-worker.claim-observation"); break;
        case "claim-after-fault-before-observation": faultClaim(); expect(cell!.claimed).toBe(true); break;
        case "claim-after-fault-observed-but-fenced": faultClaim(); expect(prepare().phase).toBe("actor-worker.claim-observation"); expect(prepare(264).kind).toBe("rejected"); break;
        case "cancel-before-pending-release": start(); expect(close().phase).toBe("actor-worker.cell-begin-close"); expect(ledger.preparedAdmission(client)).toBe(cell); break;
        case "cancel-after-pending-release-not-terminal": start(); close(); expect(close().phase).toBe("resident-admission-bootstrap-release"); expect(close().phase).toBe("actor-worker.pending-release-observation"); expect(cell!.terminalIsEmpty()).toBe(false); break;
        case "cancel-original-terminal-observation": start(); close(); close(); close(); expect(close(296).kind).toBe("pending"); expect(close().kind).toBe("complete"); expect(OwnedResidentRetirement.matches(cell!.retirement, cell!)).toBe(true); break;
        case "cancel-fault-cell-pending-released":
        case "cancel-fault-cell-unobserved": faultPrepare(); take(); expect(close().phase).toBe("actor-worker.cell-observation"); expect(close().phase).toBe("resident-admission-fault-handoff"); expect(close().phase).toBe("actor-worker.cell-begin-close"); expect(close().phase).toBe("resident-admission-bootstrap-release"); if (row.evidence === "pending-release-observed") expect(close().phase).toBe("actor-worker.pending-release-observation"); expect(cell!.failure).toBe(fault); expect(cell!.retirement).toBeNull(); break;
        case "public-phase-is-not-claimed-proof": {
          start(); const trap = vi.spyOn(OwnedResidentLedger.prototype, "claimAdmission").mockReturnValue({ kind: "ready", phase: "resident-admission-claim", items: 1, bytes: 64 });
          try { expect(prepare().kind).toBe("pending"); } finally { trap.mockRestore(); }
          expect(prepare().phase).toBe("actor-worker.unclaimed"); expect(cell!.claimed).toBe(false); expect(ledger.preparedAdmission(client)).toBe(cell); break;
        }
        case "live-root-lost-after-claim": claim(); shared.record.beginClose(); expect(prepare().phase).toBe("actor-worker.claim-observation"); expect(prepare(264).kind).toBe("rejected"); break;
        case "identity-recovery-cannot-open-descendant": claim(); shared.record.beginClose(); expect(shared.record.matchesShell(client)).toBe(true); expect(shared.record.matchesLiveShell(client)).toBe(false); break;
        default: throw new Error(`Missing actual ending ${row.id}`);
      }
      const original = OwnedResidentLedger.prototype.prepareAdmission; const admission = vi.spyOn(OwnedResidentLedger.prototype, "prepareAdmission");
      try {
        if (row.id === "foreign-purpose-never-clears-original") { expect(prepare(296).kind).toBe("blocked"); expect(admission).not.toHaveBeenCalled(); }
        else { client.prepareUiResidentPool(ledger, { maxItems: 1, maxBytes: 296 }); expect(admission.mock.calls.length, row.id).toBe(row.expected.purpose === "none" && row.live ? 1 : 0); }
      } finally { admission.mockRestore(); }
      expect(OwnedResidentLedger.prototype.prepareAdmission).toBe(original);
      if (row.expected.construct) expect(prepare(264).kind).toBe("pending");
      const originalResult = cell === null ? null : (cell as OwnedResidentAdmission).result;
      expect(originalResult !== null, row.id).toBe(row.expected.construct); if (originalResult) expect(originalResult.record).not.toBeNull();
      expect(workers).toHaveLength(1); expect(workers[0]!.sent).toHaveLength(0); client.disposeAll(); completed.push(row.id);
    }
    expect(completed).toEqual(fixture.gateCases.map(row => row.id)); console.log(`[TRACE] ShardWorkerBootstrap actual original-cell endings=${completed.length}`);
  });
  it("ShardWorkerBootstrap retains every first value after real prepare claim reserve and install", async () => {
    const fixture = await workerPreparationFixture(); const { OwnedResidentRecord } = await import("@semio-tech/value-ts/resident"); let observed = 0;
    for (const stage of fixture.faults.stages) for (const name of fixture.faults.values) {
      let reads = 0; const object = Object.defineProperty({}, "message", { get() { reads++; throw new Error("Must not inspect original fault"); } });
      const fault = name === "null" ? null : name === "undefined" ? undefined : name === "false" ? false : name === "zero" ? 0 : object;
      const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); prepareWorkerFixture(client, fixture.shared.phases);
      const prefix = stage === "prepare" ? 0 : stage === "claim" ? 2 : stage === "reserve" ? 4 : 6;
      const held: { cell: OwnedResidentAdmission | null } = { cell: null }; const originalClaim = OwnedResidentLedger.prototype.claimAdmission;
      const capture = vi.spyOn(OwnedResidentLedger.prototype, "claimAdmission").mockImplementation(function (this: OwnedResidentLedger, ...args) { held.cell = args[1]; return Reflect.apply(originalClaim, this, args); });
      try { prepareWorkerFixture(client, fixture.worker.phases.slice(0, prefix)); } finally { capture.mockRestore(); }
      held.cell ??= ledger.preparedAdmission(client);
      const method = stage === "prepare" ? "prepareAdmission" : stage === "claim" ? "claimAdmission" : "reserveRecord";
      const original = OwnedResidentLedger.prototype[method]; const install = OwnedResidentRecord.prototype.install; let calls = 0;
      const trap = stage === "install" ? vi.spyOn(OwnedResidentRecord.prototype, "install").mockImplementation(function (this: OwnedResidentRecord, ...args) { calls++; Reflect.apply(install, this, args); throw fault; }) : vi.spyOn(OwnedResidentLedger.prototype, method).mockImplementation(function (this: OwnedResidentLedger, ...args: unknown[]) { calls++; Reflect.apply(original, this, args); throw fault; });
      try { expect(client.prepareWorkerBootstrap({ maxItems: 1, maxBytes: Number(fixture.worker.phases[prefix]![1]) }).kind).toBe("rejected"); } finally { trap.mockRestore(); }
      held.cell ??= ledger.preparedAdmission(client); const cell = held.cell; if (!cell) throw new Error(`Actual ${stage} cell missing`); const record = cell.result?.record ?? null; const before = ledger.usage;
      expect(client.prepareWorkerBootstrap({ maxItems: 1, maxBytes: 64 }).kind).toBe(stage === "reserve" || stage === "install" ? "rejected" : "pending");
      for (let turn = 0; turn < fixture.worker.phases.length; turn++) { client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 296 }); if (cell.hasFailure) break; }
      expect(cell.hasFailure).toBe(true); expect(Object.is(cell.failure, fault)).toBe(true); expect(cell.result?.record ?? null).toBe(record); expect(cell.retirement).toBeNull(); expect(ledger.usage).toEqual(before); expect(calls).toBe(1); expect(reads).toBe(fixture.faults.getterReads);
      expect(cell.retainFailure(fault, { maxItems: 1, maxBytes: 64 }).kind).toBe("ready"); const distinct = Object.freeze({ distinct: stage }); expect(cell.retainFailure(distinct, { maxItems: 1, maxBytes: 64 }).kind).toBe("rejected"); expect(Object.is(cell.failure, fault)).toBe(true);
      client.disposeAll(); observed++;
    }
    expect(observed).toBe(fixture.faults.stages.length * fixture.faults.values.length); console.log(`[TRACE] ShardWorkerBootstrap real post-call first-fault vectors=${observed}`);
  });
  it("ShardWorkerBootstrap does not infer pending release from a close-wrapper throw and null", async () => {
    const fixture = await workerPreparationFixture(); const { OwnedResidentAdmission } = await import("@semio-tech/value-ts/resident");
    const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); prepareWorkerFixture(client, fixture.shared.phases); prepareWorkerFixture(client, fixture.worker.phases.slice(0, 2));
    const cell = ledger.preparedAdmission(client); if (!cell) throw new Error("Exact original pending worker cell missing"); expect(client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 64 }).kind).toBe("pending");
    const original = OwnedResidentAdmission.prototype.closeStep; const fault = Object.freeze({ original: "close-after-release" });
    const trap = vi.spyOn(OwnedResidentAdmission.prototype, "closeStep").mockImplementation(function (this: OwnedResidentAdmission, grant) { const current = Reflect.apply(original, this, [grant]); expect(current.phase).toBe("resident-admission-bootstrap-release"); throw fault; });
    try { expect(client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 64 }).kind).toBe("rejected"); } finally { trap.mockRestore(); }
    expect(ledger.preparedAdmission(client)).toBeNull(); expect(client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 64 }).phase).toBe("resident-admission-fault-handoff"); expect(cell.failure).toBe(fault);
    expect(client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 296 }).kind).toBe("blocked"); const admission = vi.spyOn(OwnedResidentLedger.prototype, "prepareAdmission");
    try { expect(client.prepareUiResidentPool(ledger, { maxItems: 1, maxBytes: 296 }).kind).toBe("blocked"); expect(admission).not.toHaveBeenCalled(); } finally { admission.mockRestore(); }
    expect(cell.retirement).toBeNull(); expect(ledger.usage.data.bytes).toBe(fixture.shared.retained.bytes + 296); client.disposeAll();
  });
  it("ShardWorkerBootstrap retires the unused original cell after a canonical no-record refusal", async () => {
    const fixture = await workerPreparationFixture(); const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger });
    prepareWorkerFixture(client, fixture.shared.phases); prepareWorkerFixture(client, fixture.worker.phases.slice(0, 2)); const cell = ledger.preparedAdmission(client); if (!cell) throw new Error("Original worker cell missing"); prepareWorkerFixture(client, fixture.worker.phases.slice(2, 4));
    const original = OwnedResidentLedger.prototype.reserveRecord; const trap = vi.spyOn(OwnedResidentLedger.prototype, "reserveRecord").mockImplementation(function (this: OwnedResidentLedger, ...args) { this.beginClose(); return Reflect.apply(original, this, args); });
    try { expect(client.prepareWorkerBootstrap({ maxItems: 1, maxBytes: 264 }).kind).toBe("rejected"); } finally { trap.mockRestore(); }
    expect(cell.result).toBeNull(); expect(client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 64 }).kind).toBe("pending");
    let completed = false; for (let turn = 0; turn < fixture.worker.phases.length; turn++) { const current = client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 296 }); if (current.kind === "complete") { completed = true; break; } expect(current.kind).toBe("pending"); }
    expect(completed).toBe(true); expect(OwnedResidentRetirement.matches(cell.retirement, cell)).toBe(true); expect(ledger.usage.data).toEqual(fixture.shared.retained); client.disposeAll();
  });
  it("ShardWorkerBootstrap leaves a distinct later thrown value with its caller and preserves the first", async () => {
    const fixture = await workerPreparationFixture(); const { OwnedResidentAdmission } = await import("@semio-tech/value-ts/resident");
    const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); prepareWorkerFixture(client, fixture.shared.phases);
    const original = OwnedResidentLedger.prototype.prepareAdmission; const first = Object.freeze({ original: "first" }); const other = Object.freeze({ original: "distinct" });
    const preparation = vi.spyOn(OwnedResidentLedger.prototype, "prepareAdmission").mockImplementation(function (this: OwnedResidentLedger, ...args) { Reflect.apply(original, this, args); throw first; });
    try { expect(client.prepareWorkerBootstrap({ maxItems: 1, maxBytes: 296 }).kind).toBe("rejected"); } finally { preparation.mockRestore(); }
    const cell = ledger.preparedAdmission(client); if (!cell) throw new Error("Original first-fault cell missing"); expect(client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 64 }).kind).toBe("pending");
    const retention = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(() => { throw other; });
    try { let thrown: unknown; try { client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 64 }); } catch (error) { thrown = error; } expect(thrown).toBe(other); } finally { retention.mockRestore(); }
    expect(client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 64 }).phase).toBe("resident-admission-fault-handoff"); expect(cell.failure).toBe(first); expect(cell.retirement).toBeNull(); client.disposeAll();
  });
}
