type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { ACTOR_BYTE_PAGE_BYTES, MAINTENANCE_LANE_DEFAULT_BUDGET, MAX_SEGMENTED_DOWNLOAD_OPERATION_ID, NO_RESIDENT_FAULT, OwnedActorTurnOutput, OwnedActorTurnOutputs, OwnedKernelReturnContent, OwnedNativeUiPatchAuthority, OwnedNativeUiPatchSubmissionReceipt, OwnedResidentLedger, OwnedResidentRetirement, OwnedShardReturn, OwnedShardReturnPage, OwnedUiInstance, OwnedUiInstanceRetirement, OwnedUiPatchAcknowledgement, OwnedUiPatchInputRetirement, OwnedUiResidentPool, SHARD_FRAME_VARIANT_FIELDS, SHARD_JSPI_FAULT_CODE, SHARD_LIVENESS_POLICY, ShardClient, ShardJspiUnavailableError, assertShardJspiAvailable, capturedReturnState, createActorBytePage, createGrantedBudgetTracker, createShardCommandIngressPages, describeShardWorkerError, encodeActorInstanceLifecycle, encodeActorUiPatchReceipt, interpretShardFrame, isShardLostError, orderEnvelopesByLane, poolControllerEnvelope, poolUiEnvelope, shardJspiAvailable, uiResidentMetadataEnvelope } = dependencies;
  type ActorInstanceLifecycleReceipt = any;
  type ActorInstanceLifetime = any;
  type InboundMessage = any;
  type Lane = any;
  type OutboundMessage = any;
  type OwnedResidentAdmission = any;
  type OwnedResidentRecord = any;
  type OwnedUiResidentInstance = any;
  type OwnedUiResidentPayload = any;
  type PendingEntry = any;
  type ResidentStep = any;
  type ShardBudget = any;
  type ShardClientOptions = any;
  type ShardEnvelope = any;
  type ShardEventEnvelope = any;
  type ShardFrame = any;
  type ShardInstanceLifecycleLease = any;
  type ShardInstanceOwner = any;
  type ShardJspiScope = any;
  type ShardReturnAdmission = any;
  type ShardSlot = any;
  type ShardWorkerLike = any;
  type UiResidentSourceProof = any;

  const { describe, expect, it, vi } = vitest;

  it("ShardWorkerBootstrap declares only original metadata preparation and close methods", async () => {
    const { default: fixture } = await import("../../../🏘️composition/🏗️bootstrap/🧪️fixture/🔣️.json");
    const { default: schema } = await import("../../../🏘️composition/🏗️bootstrap/🧬️schema/🔣️.json");
    const { default: residentSchema } = await import("../../../../🌱️value/💾️resident/🧬️schema/🔣️.json"); const { default: Ajv } = await import("ajv");
    expect(new Ajv({ strict: true }).addSchema(residentSchema).compile(schema)(fixture)).toBe(true);
    const { client, workers } = harness(1, { residentLedger: new OwnedResidentLedger(fixture.capacity) }); const posts = workers[0]!.sent.length;
    expect(typeof Reflect.get(client, fixture.methods.prepare)).toBe("function"); expect(typeof Reflect.get(client, fixture.methods.close)).toBe("function");
    expect(workers).toHaveLength(1); expect(workers[0]!.sent.length).toBe(posts); client.disposeAll();
  });

  for (const prefix of [7, 8]) for (const closing of ["ledger", "record", "cell", "fault"]) it(`ShardWorkerBootstrap shared closing prefix ${prefix} ${closing} cannot admit a UI descendant`, async () => {
    const { default: fixture } = await import("../../../🏘️composition/🏗️bootstrap/🧪️fixture/🔣️.json");
    const { OwnedResidentRecord } = await import("../../../../🌱️value/💾️resident/🟦️.ts");
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

  async function workerPreparationFixture() { return (await import("../../../🏘️composition/🏗️bootstrap/🧪️fixture/🔣️.json")).default; }
  function prepareWorkerFixture(client: ShardClient, rows: readonly (readonly (string | number)[])[]): void {
    for (const row of rows) { const bytes = Number(row[1]); expect(client.prepareWorkerBootstrap({ maxItems: 1, maxBytes: bytes }), String(row[0])).toMatchObject({ kind: "pending", items: 1, bytes }); }
  }

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
    const source = ts.createSourceFile("shard.ts", await readFile(new URL("./🟦️.ts", source.url), "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const declaration = source.statements.find(statement => ts.isClassDeclaration(statement) && statement.name?.text === "ShardClient"); if (!declaration || !ts.isClassDeclaration(declaration)) throw new Error("Original Shard class missing");
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
    const fixture = await workerPreparationFixture(); const { default: uiFixture } = await import("../../../🏘️composition/🧪️fixture/🔣️.json");
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
    const fixture = await workerPreparationFixture(); const { OwnedResidentAdmission } = await import("../../../../🌱️value/💾️resident/🟦️.ts"); const completed: string[] = [];
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
    expect(completed).toEqual(fixture.gateCases.map(row => row.id)); console.log(`[DEBUG] ShardWorkerBootstrap actual original-cell endings=${completed.length}`);
  });

  it("ShardWorkerBootstrap retains every first value after real prepare claim reserve and install", async () => {
    const fixture = await workerPreparationFixture(); const { OwnedResidentRecord } = await import("../../../../🌱️value/💾️resident/🟦️.ts"); let observed = 0;
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
    expect(observed).toBe(fixture.faults.stages.length * fixture.faults.values.length); console.log(`[DEBUG] ShardWorkerBootstrap real post-call first-fault vectors=${observed}`);
  });

  it("ShardWorkerBootstrap does not infer pending release from a close-wrapper throw and null", async () => {
    const fixture = await workerPreparationFixture(); const { OwnedResidentAdmission } = await import("../../../../🌱️value/💾️resident/🟦️.ts");
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
    const fixture = await workerPreparationFixture(); const { OwnedResidentAdmission } = await import("../../../../🌱️value/💾️resident/🟦️.ts");
    const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); prepareWorkerFixture(client, fixture.shared.phases);
    const original = OwnedResidentLedger.prototype.prepareAdmission; const first = Object.freeze({ original: "first" }); const other = Object.freeze({ original: "distinct" });
    const preparation = vi.spyOn(OwnedResidentLedger.prototype, "prepareAdmission").mockImplementation(function (this: OwnedResidentLedger, ...args) { Reflect.apply(original, this, args); throw first; });
    try { expect(client.prepareWorkerBootstrap({ maxItems: 1, maxBytes: 296 }).kind).toBe("rejected"); } finally { preparation.mockRestore(); }
    const cell = ledger.preparedAdmission(client); if (!cell) throw new Error("Original first-fault cell missing"); expect(client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 64 }).kind).toBe("pending");
    const retention = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(() => { throw other; });
    try { let thrown: unknown; try { client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 64 }); } catch (error) { thrown = error; } expect(thrown).toBe(other); } finally { retention.mockRestore(); }
    expect(client.closeWorkerBootstrapStep({ maxItems: 1, maxBytes: 64 }).phase).toBe("resident-admission-fault-handoff"); expect(cell.failure).toBe(first); expect(cell.retirement).toBeNull(); client.disposeAll();
  });

  it("ShardResidentComposition requires the original ledger before creating workers", async () => {
    const { OwnedResidentLedger } = await import("../../../../🌱️value/💾️resident/🟦️.ts");
    const { default: fixture } = await import("../../../🏘️composition/🧪️fixture/🔣️.json"); const { default: schema } = await import("../../../🏘️composition/🧬️schema/🔣️.json");
    const { default: residentSchema } = await import("../../../../🌱️value/💾️resident/🧬️schema/🔣️.json"); const { default: Ajv } = await import("ajv");
    expect(new Ajv({ strict: true }).addSchema(residentSchema).compile(schema)(fixture)).toBe(true);
    const ledger = new OwnedResidentLedger(fixture.capacity); let workers = 0;
    const createWorker = () => { workers++; return new FakeShardWorker(0); };
    for (const candidate of [undefined, { capacity: fixture.capacity }, Object.create(OwnedResidentLedger.prototype), new Proxy(ledger, {})]) {
      expect(() => Reflect.construct(ShardClient, [{ shardCount: 1, createWorker, residentLedger: candidate }])).toThrow("actor-resident.invalid-ledger");
      expect(workers).toBe(fixture.boundaries.workersBeforeValidLedger);
    }
    const client: ShardClient = Reflect.construct(ShardClient, [{ shardCount: 1, createWorker, residentLedger: ledger }]);
    expect(ShardClient.matchesResidentLedger(client, ledger)).toBe(fixture.binding.sameLedger);
    expect(ShardClient.matchesResidentLedger(client, new OwnedResidentLedger(fixture.capacity))).toBe(fixture.binding.foreignLedger);
    expect(ShardClient.matchesResidentLedger(Object.create(ShardClient.prototype), ledger)).toBe(fixture.binding.structuralClient);
    expect(Object.keys(client).includes(fixture.binding.requiredOption)).toBe(fixture.binding.publicLedgerProperty);
    expect(workers).toBe(1); expect(ledger.usage).toEqual({ data: { bytes: 0, slots: 0, owners: 0 }, control: { bytes: 0, slots: 0, owners: 0 } }); client.disposeAll();
  });

  it("ShardResidentComposition matches only the privately captured original activation owner", async () => {
    const { default: fixture } = await import("../../../🏘️composition/🧪️fixture/🔣️.json"); const { default: schema } = await import("../../../🏘️composition/🧬️schema/🔣️.json");
    const { default: residentSchema } = await import("../../../../🌱️value/💾️resident/🧬️schema/🔣️.json"); const { default: Ajv } = await import("ajv");
    const { readFile } = await import("node:fs/promises"); const ts = await import("typescript"); const row = fixture.activationBinding;
    expect(new Ajv({ strict: true }).addSchema(residentSchema).compile(schema)(fixture)).toBe(true);
    const ledger = new OwnedResidentLedger(fixture.capacity); const local = harness(1, { residentLedger: ledger }); const foreign = harness(1, { residentLedger: ledger });
    const activate = async (context: typeof local) => { const pending = context.client.activate(row.actorId, "https://fixture.invalid/actor.js", [], BUDGET); const request = context.workers[0]!.sent.at(-1) as { requestId: string }; context.workers[0]!.deliver({ kind: "result", requestId: request.requestId, ok: true, value: undefined }); await pending; return context.client.captureActorActivation(row.actorId); };
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
    const source = ts.createSourceFile("shard.ts", await readFile(new URL("./🟦️.ts", source.url), "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const declaration = source.statements.find(statement => ts.isClassDeclaration(statement) && statement.name?.text === row.metadata.className); if (!declaration || !ts.isClassDeclaration(declaration)) throw new Error("Actual captured activation declaration missing");
    const fields = declaration.members.filter(ts.isPropertyDeclaration).map(member => member.name.getText(source)); expect(fields).toEqual(row.metadata.fields);
    expect(row.metadata.facadeBytes).toBe(row.metadata.recordBytes + row.metadata.fieldBytes * fields.length); expect(row.metadata.addedPrivateReferences).toBe(fields.filter(name => name.startsWith("#")).length);
    expect(row.metadata.allocationAdmitted).toBe(false); expect(row.metadata.controllerFundsActivation).toBe(false); local.client.disposeAll(); foreign.client.disposeAll();
  });

  it("ShardResidentComposition retains original ownership while revoking replaced routes and workers", async () => {
    const { default: fixture } = await import("../../../🏘️composition/🧪️fixture/🔣️.json"); const { produce } = await import("immer"); const row = fixture.activationBinding;
    for (const vector of row.transitions) {
      const ledger = new OwnedResidentLedger(fixture.capacity); const { client, workers } = harness(2, { residentLedger: ledger, exclusiveShardCount: 1 }); const { client: foreign } = harness(1, { residentLedger: ledger });
      const activate = async (worker: FakeShardWorker) => { const pending = client.activate(row.actorId, "https://fixture.invalid/actor.js", [], BUDGET); const request = worker.sent.at(-1) as { requestId: string }; worker.deliver({ kind: "result", requestId: request.requestId, ok: true, value: undefined }); await pending; return client.captureActorActivation(row.actorId); };
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

  function prepareResidentFixture(client: ShardClient, ledger: OwnedResidentLedger, bytes: readonly number[]): void {
    for (const amount of bytes) expect(client.prepareUiResidentPool(ledger, { maxItems: 1, maxBytes: amount })).toMatchObject({ kind: "pending", items: 1, bytes: amount });
  }

  it("ShardResidentComposition preadmits the exact shared pool record without reusing a child grant", async () => {
    const { default: fixture } = await import("../../../🏘️composition/🧪️fixture/🔣️.json"); const { produce } = await import("immer");
    const { readFile } = await import("node:fs/promises"); const ts = await import("typescript");
    const ledger = new OwnedResidentLedger(fixture.capacity); const foreign = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); const row = fixture.poolPreparation;
    expect(client.prepareUiResidentPool(foreign, { maxItems: 1, maxBytes: 4096 }).kind).toBe("rejected"); expect(foreign.usage.data).toEqual({ bytes: 0, slots: 0, owners: 0 });
    for (const refused of [{ maxItems: 0, maxBytes: 296 }, { maxItems: 1, maxBytes: 295 }]) expect(client.prepareUiResidentPool(ledger, refused).kind).toBe("blocked");
    expect(ledger.usage.data).toEqual({ bytes: 0, slots: 0, owners: 0 }); prepareResidentFixture(client, ledger, row.prepareBytes);
    expect(ledger.usage.data).toEqual(row.total); expect(uiResidentMetadataEnvelope("pool")).toEqual(row.uiEnvelope);
    const expected = produce({ bytes: 0, slots: 0, owners: 0 }, value => { for (const envelope of [row.controllerEnvelope, row.uiEnvelope, row.intrinsicEnvelope, row.cellEnvelope, row.intrinsicEnvelope, row.cellEnvelope]) { value.bytes += envelope.bytes; value.slots += envelope.slots; value.owners += envelope.owners; } }); expect(expected).toEqual(row.total);
    const source = ts.createSourceFile("shard.ts", await readFile(new URL("./🟦️.ts", source.url), "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const declaration = source.statements.find(statement => ts.isClassDeclaration(statement) && statement.name?.text === "ShardClient"); if (!declaration || !ts.isClassDeclaration(declaration)) throw new Error("Actual Shard declaration missing");
    const fields = declaration.members.filter(ts.isPropertyDeclaration).map(member => member.name.getText(source)).filter(name => name.startsWith("#uiResident") || name === "#clientAdmissionPurpose").map(name => name.slice(1));
    expect(fields).toEqual(row.controllerFields); expect(row.controllerEnvelope.bytes).toBe(row.controllerModel.recordBytes + row.controllerModel.fieldBytes * fields.length);
    expect(client.prepareUiResidentPool(ledger, { maxItems: 1, maxBytes: 64 })).toMatchObject({ kind: "ready", items: 0, bytes: row.samePreparedBytes });
    expect(client.prepareUiResidentPool(foreign, { maxItems: 1, maxBytes: 4096 }).kind).toBe("rejected"); expect(client.ownsUiResidentPool({})).toBe(false);
    for (const bytes of fixture.unusedClose.releaseBytes) client.closeUiResidentPoolStep({ maxItems: 1, maxBytes: bytes }); expect(ledger.usage.data).toEqual(fixture.poolPreparation.controllerTotal); client.disposeAll();
  });

  it("ShardResidentComposition releases only its actual pool's private terminal witness", async () => {
    const { default: fixture } = await import("../../../🏘️composition/🧪️fixture/🔣️.json");
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
    const { OwnedResidentRecord } = await import("../../../../🌱️value/💾️resident/🟦️.ts"); const { default: fixture } = await import("../../../🏘️composition/🧪️fixture/🔣️.json"); const { produce } = await import("immer");
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
    const { default: fixture } = await import("../../../🏘️composition/🧪️fixture/🔣️.json"); const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); const grant = { maxItems: 1, maxBytes: 4096 }; const row = fixture.parentClose;
    prepareResidentFixture(client, ledger, fixture.poolPreparation.prepareBytes); const pool = OwnedUiResidentPool.begin(client, ledger, grant).pool; if (!pool) throw new Error("Original pool missing");
    const close = vi.spyOn(OwnedUiResidentPool.prototype, "closeStep"); const retirement = vi.spyOn(OwnedUiResidentPool.prototype, "retirement", "get");
    try {
      for (let index = 0; index < row.releaseBytes.length; index++) {
        close.mockClear(); retirement.mockClear(); const current = client.closeUiResidentPoolStep({ maxItems: 1, maxBytes: row.releaseBytes[index]! });
        expect(current).toMatchObject({ kind: row.releaseKinds[index], items: 1, bytes: row.releaseBytes[index] }); expect(close.mock.calls.length).toBeLessThanOrEqual(row.maxChildCallsPerTurn); if (close.mock.calls.length) expect(retirement).not.toHaveBeenCalled();
      }
      expect(pool.terminalIsEmpty()).toBe(true); expect(client.ownsUiResidentPool(pool)).toBe(false); expect(ledger.usage.data).toEqual(fixture.poolPreparation.controllerTotal); expect(client.closeUiResidentPoolStep(grant)).toMatchObject({ kind: "complete", items: 0, bytes: row.retryBytes });
    } finally { close.mockRestore(); retirement.mockRestore(); client.disposeAll(); }
  });

  it("ShardResidentComposition preserves every thrown value after an actual parent close transition", async () => {
    const { default: fixture } = await import("../../../🏘️composition/🧪️fixture/🔣️.json"); const { OwnedResidentRecord } = await import("../../../../🌱️value/💾️resident/🟦️.ts"); const row = fixture.parentFault; const grant = { maxItems: 1, maxBytes: 4096 };
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
    const { default: fixture } = await import("../../../🏘️composition/🧪️fixture/🔣️.json"); const { OwnedResidentRecord } = await import("../../../../🌱️value/💾️resident/🟦️.ts"); const row = fixture.observationFault;
    const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); const grant = { maxItems: 1, maxBytes: 4096 };
    prepareResidentFixture(client, ledger, fixture.poolPreparation.prepareBytes); const pool = OwnedUiResidentPool.begin(client, ledger, grant).pool; if (!pool) throw new Error("Exact original pool missing"); pool.beginClose(); expect(pool.closeStep(grant).kind).toBe("complete"); const witness = pool.retirement;
    expect(client.releaseUiResidentPool(pool, witness, grant).kind).toBe("pending"); expect(client.releaseUiResidentPool(pool, witness, grant).kind).toBe("pending");
    const read = Object.getOwnPropertyDescriptor(OwnedResidentRecord.prototype, "detachment")!.get!; let observed = false;
    const trap = vi.spyOn(OwnedResidentRecord.prototype, "detachment", "get").mockImplementation(function (this: OwnedResidentRecord) { observed = Reflect.apply(read, this, []) !== null; throw null; });
    let escaped = false; let current: ResidentStep | null = null;
    try { current = client.releaseUiResidentPool(pool, witness, grant); } catch { escaped = true; } finally { trap.mockRestore(); }
    expect(observed).toBe(row.afterActualDetachmentRead); expect(escaped).toBe(row.escapes); expect(current).toMatchObject({ kind: row.first });
    expect(client.releaseUiResidentPool(pool, witness, grant).kind).toBe(row.faultHandoff);
    expect(client.releaseUiResidentPool(pool, witness, grant).kind).toBe(row.recoveredObservation);
    for (const bytes of [264,64,64]) expect(client.releaseUiResidentPool(pool, witness, { maxItems: 1, maxBytes: bytes }).kind).toBe("pending");
    expect(client.releaseUiResidentPool(pool, witness, grant).kind).toBe(row.final); expect(client.ownsUiResidentPool(pool)).toBe(row.poolStillOwned); expect(ledger.usage.data).toEqual(row.retained); client.disposeAll();
  });

  it("ShardResidentComposition recovers original bootstrap claim and record results after wrapper throws", async () => {
    const { default: fixture } = await import("../../../🏘️composition/🧪️fixture/🔣️.json"); const row = fixture.admissionWrappers;
    let getterReads = 0; const values = new Map<string, unknown>([["null", null], ["undefined", undefined], ["false", false], ["zero", 0], ["object", { payload: new Uint8Array(8193), get message() { getterReads++; return "unread"; } }]]);
    for (const scope of row.scopes) for (const stage of row.stages) for (const name of row.values) {
      const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); const { client: peer } = harness(1, { residentLedger: ledger }); const grant = { maxItems: 1, maxBytes: 4096 }; const fault = values.get(name);
      const held: { cell: import("../../🌱️value/💾️resident/🟦️.ts").OwnedResidentAdmission | null; record: OwnedResidentRecord | null } = { cell: null, record: null };
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
    const { default: fixture } = await import("../../../🏘️composition/🧪️fixture/🔣️.json"); const row = fixture.aliasRetirement; const ledger = new OwnedResidentLedger(fixture.capacity); const {client} = harness(1,{residentLedger:ledger});
    const held: { cell: import("../../🌱️value/💾️resident/🟦️.ts").OwnedResidentAdmission | null; record: OwnedResidentRecord | null } = {cell:null,record:null}; const original = OwnedResidentLedger.prototype.reserveRecord;
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
    const {default:fixture}=await import("../../../🏘️composition/🧪️fixture/🔣️.json"); const row=fixture.cancelledPreparation; const grant={maxItems:1,maxBytes:4096};
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
    const {default:fixture}=await import("../../../🏘️composition/🧪️fixture/🔣️.json"); const {produce}=await import("immer"); const ledger=new OwnedResidentLedger(fixture.capacity); const {client}=harness(1,{residentLedger:ledger});
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

  class FakeShardWorker implements ShardWorkerLike {
    readonly index: number;
    onmessage: ((event: { readonly data: unknown }) => void) | null = null;
    onerror: ((event: unknown) => void) | null = null;
    readonly sent: unknown[] = [];
    terminated = false;
    constructor(index: number) {
      this.index = index;
    }
    postMessage(message: unknown): void {
      this.sent.push(message);
    }
    terminate(): void {
      this.terminated = true;
    }
    deliver(message: InboundMessage): void {
      this.onmessage?.({ data: message });
    }
  }

  function harness(shardCount = 2, extra?: Partial<ShardClientOptions>) {
    const workers: FakeShardWorker[] = [];
    let nowMs = 0;
    const residentLedger = extra?.residentLedger ?? new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } });
    const client = new ShardClient({
      residentLedger,
      shardCount,
      createWorker: (index) => {
        const worker = new FakeShardWorker(index);
        workers.push(worker);
        return worker;
      },
      now: () => nowMs,
      ...extra,
    });
    return { client, residentLedger, workers, advance: (ms: number) => (nowMs += ms), setNow: (ms: number) => (nowMs = ms) };
  }

  async function fixtureResidentPool(client: ShardClient, ledger: OwnedResidentLedger): Promise<OwnedUiResidentPool> {
    const { default: fixture } = await import("../../../🏘️composition/🧪️fixture/🔣️.json");
    const { produce } = await import("immer"); const before = ledger.usage.data;
    expect(ShardClient.matchesResidentLedger(client, ledger)).toBe(true);
    for (const bytes of fixture.poolPreparation.prepareBytes) {
      const result = OwnedUiResidentPool.begin(client, ledger, { maxItems: 1, maxBytes: bytes });
      expect(result.pool).toBeNull(); expect(result.step.kind).toBe("pending"); expect(result.step.items).toBe(1); expect(result.step.bytes).toBe(bytes);
    }
    const result = OwnedUiResidentPool.begin(client, ledger, { maxItems: 1, maxBytes: uiResidentMetadataEnvelope("pool").bytes + 64 });
    expect(result.step.kind).toBe("ready"); expect(result.pool).not.toBeNull();
    const pool = result.pool; if (!pool) throw new Error("Exact fixture resident pool was not admitted");
    expect(OwnedUiResidentPool.matchesComposition(pool, client, ledger)).toBe(true); expect(client.ownsUiResidentPool(pool)).toBe(true);
    expect(ledger.usage.data).toEqual(produce(before, value => { value.bytes += fixture.poolPreparation.total.bytes; value.slots += fixture.poolPreparation.total.slots; value.owners += fixture.poolPreparation.total.owners; }));
    return pool;
  }

  async function fixtureResidentScope(pool: OwnedUiResidentPool, ledger: OwnedResidentLedger, lease: ShardInstanceLifecycleLease): Promise<OwnedUiResidentInstance> {
    const { OwnedUiResidentInstance } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts");
    const { default: fixture } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture/🔣️.json");
    const { default: schema } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema/🔣️.json"); const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");
    expect(new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/SlotFixture`)!(fixture)).toBe(true);
    const owner = fixtureHosts.get(lease); const lifetime = lease.lifetime; if (!owner || !lifetime) throw new Error("Original fixture host has not been captured");
    const before = ledger.usage.data; let scope: OwnedUiResidentInstance | null = null;
    for (let index = 0; index < fixture.firstChild.prepareBytes.length; index++) {
      const bytes = fixture.firstChild.prepareBytes[index]!; const last = index === fixture.firstChild.prepareBytes.length - 1;
      const result = pool.bindInstance(owner, lease.activation, lifetime, { maxItems: 1, maxBytes: bytes });
      expect(result.step).toMatchObject({ kind: last ? "ready" : "pending", items: 1, bytes });
      if (last) { expect(result.scope).not.toBeNull(); scope = result.scope; } else expect(result.scope).toBeNull();
    }
    if (!scope) throw new Error("Exact fixture resident scope was not admitted");
    expect(OwnedUiResidentInstance.matches(scope, owner, lease.activation, lifetime)).toBe(true);
    const repeated = pool.bindInstance(owner, lease.activation, lifetime, { maxItems: 1, maxBytes: 64 }); expect(repeated.scope).toBe(scope); expect(repeated.step).toMatchObject({ kind: "ready", items: 0, bytes: 0 });
    const expected = produce({ ...before }, usage => { usage.bytes += fixture.firstChild.total.bytes; usage.slots += fixture.firstChild.total.slots; usage.owners += fixture.firstChild.total.owners; }); expect(ledger.usage.data).toEqual(expected);
    return scope;
  }

  async function fixtureResidentPayload(scope: OwnedUiResidentInstance, ledger: OwnedResidentLedger, field: NonNullable<OwnedKernelReturnContent["field"]>): Promise<OwnedUiResidentPayload> {
    const { OwnedUiResidentPayload } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts");
    const { OwnedKernelReturnInputField } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
    const { default: fixture } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️fixture/🔣️.json"); const { default: schema } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv"); const { produce } = await import("immer"); expect(new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/PayloadFixture`)!(fixture)).toBe(true);
    const before = ledger.usage.data; let payload: OwnedUiResidentPayload | null = null;
    for (let index = 0; index < fixture.admissionBytes.length; index++) {
      const bytes = fixture.admissionBytes[index]!; const last = index === fixture.admissionBytes.length - 1;
      const short = scope.beginPayload(field, { maxItems: 0, maxBytes: bytes }); expect(short.payload).toBeNull(); expect(short.step).toMatchObject({ kind: "blocked", items: 0, bytes: 0 });
      const result = scope.beginPayload(field, { maxItems: 1, maxBytes: bytes }); expect(result.step, fixture.admissionPhases[index]).toMatchObject({ kind: last ? "ready" : "pending", items: 1, bytes });
      if (last) { expect(result.payload).not.toBeNull(); payload = result.payload; } else expect(result.payload).toBeNull();
    }
    if (!payload) throw new Error("Exact fixture resident payload was not admitted");
    expect(OwnedUiResidentPayload.matchesScope(payload, scope)).toBe(true); expect(OwnedUiResidentPayload.matchesField(payload, field)).toBe(true); expect(OwnedKernelReturnInputField.matchesResidentPayload(field, payload)).toBe(true);
    expect(field.residentPayload(scope)).toBe(payload); const repeated = scope.beginPayload(field, { maxItems: 1, maxBytes: 64 }); expect(repeated.payload).toBe(payload); expect(repeated.step).toMatchObject({ kind: "ready", items: 0, bytes: 0 });
    const expected = produce({ ...before }, usage => { usage.bytes += fixture.expectedPayloadTotal.bytes; usage.slots += fixture.expectedPayloadTotal.slots; usage.owners += fixture.expectedPayloadTotal.owners; }); expect(ledger.usage.data).toEqual(expected);
    return payload;
  }

  async function fixtureResidentBuilder(ledger: OwnedResidentLedger, field: NonNullable<OwnedKernelReturnContent["field"]>, resident: OwnedUiResidentPayload) {
    const { OwnedUiOperationPayloadBuilder } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🟦️.ts");
    const { OwnedKernelReturnInputField } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
    const { default: fixture } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture/🔣️.json"); const { default: schema } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv"); const { produce } = await import("immer"); expect(new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/BuilderFixture`)!(fixture)).toBe(true);
    const before = ledger.usage.data;
    for (let index = 0; index < fixture.grants.length; index++) {
      const bytes = fixture.grants[index]!; const last = index === fixture.grants.length - 1;
      const short = OwnedUiOperationPayloadBuilder.begin(field.owner, field.activation, field.lifetime, field, resident, { maxItems: 0, maxBytes: bytes }); expect(short.builder).toBeNull(); expect(short.step).toMatchObject({ kind: "blocked", items: 0, bytes: 0 });
      const result = OwnedUiOperationPayloadBuilder.begin(field.owner, field.activation, field.lifetime, field, resident, { maxItems: 1, maxBytes: bytes });
      if (result.step.kind === "rejected") { expect(result.builder).toBeNull(); return result; }
      expect(result.step, fixture.phases[index]).toMatchObject({ kind: last ? "ready" : "pending", items: 1, bytes });
      if (!last) { expect(result.builder).toBeNull(); continue; }
      const builder = result.builder; if (!builder) throw new Error("Exact fixture resident builder was not admitted");
      expect(OwnedUiOperationPayloadBuilder.matchesField(builder, field)).toBe(true); expect(OwnedKernelReturnInputField.matchesBuilder(field, builder)).toBe(true);
      const repeated = resident.beginBuilder(field, { maxItems: 1, maxBytes: 64 }); expect(repeated.builder).toBe(builder); expect(repeated.step).toMatchObject({ kind: "ready", items: 0, bytes: 0 });
      const expected = produce({ ...before }, usage => { usage.bytes += fixture.total.bytes; usage.slots += fixture.total.slots; usage.owners += fixture.total.owners; }); expect(ledger.usage.data).toEqual(expected);
      return result;
    }
    throw new Error("Exact fixture builder phase sequence did not publish");
  }

  const BUDGET: ShardBudget = { fuel: 1000, wallMs: 4, memoryBytes: 1 << 20, uiNodes: 100, mailboxLen: 16, maxEffects: 8, maxPatchBytes: 1 << 16 };
  const fixtureHosts = new WeakMap<ShardInstanceLifecycleLease, OwnedUiInstance>();
  function bindFixtureHost(lease: ShardInstanceLifecycleLease): OwnedUiInstance {
    const host = new OwnedUiInstance(lease.activation, lease.lifetime!, { maxNodes: 128, maxDepth: 16, maxChildren: 32, maxTextBytes: 4096, maxPatchOps: 128, maxPatchBytes: 65536 }, { usizeBits: 32 });
    lease.bindHostRetirement(host);
    fixtureHosts.set(lease, host);
    return host;
  }
  function fixtureRetirement(lease: ShardInstanceLifecycleLease): OwnedUiInstanceRetirement {
    const host = fixtureHosts.get(lease)!;
    host.beginClose();
    for (let step = 0; step < 32 && !host.terminalIsEmpty(); step += 1) host.closeStep({ maxItems: 1, maxBytes: 4096 });
    const witness = host.takeRetirementWitness();
    if (!witness) throw new Error("fixture host retirement did not complete");
    return witness;
  }

  async function answerLifecycle(worker: FakeShardWorker, pending: Promise<unknown>, receipt?: ActorInstanceLifecycleReceipt): Promise<unknown> {
    const message = worker.sent.at(-1) as { requestId: string };
    worker.deliver({ kind: "result", requestId: message.requestId, ok: true, value: { status: { tag: "idle" }, lifecycleReceipt: receipt ? encodeActorInstanceLifecycle(receipt) : undefined } });
    return pending;
  }

  async function captureFixtureInstance(client: ShardClient, worker: FakeShardWorker, actorId: string, instanceId = 7, guestLifetime = 13n): Promise<ShardInstanceLifecycleLease> {
    const lease = client.captureInstanceLifecycle(actorId, instanceId);
    const captured: ActorInstanceLifecycleReceipt = { kind: "captured", lifetime: { activationGeneration: lease.activation.activationGeneration, instanceId, guestLifetime }, requestSequence: lease.openRequest.requestSequence };
    await answerLifecycle(worker, lease.open({ appId: "fixture", actor: "fixture", config: [], assets: [], capabilities: [], quotas: [] }, BUDGET), captured);
    await answerLifecycle(worker, lease.acknowledge(captured, BUDGET));
    bindFixtureHost(lease);
    return lease;
  }

  async function retireFixtureInstance(worker: FakeShardWorker, lease: ShardInstanceLifecycleLease): Promise<void> {
    const request = lease.beginClose();
    const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: lease.lifetime!, requestSequence: request.requestSequence, closeGeneration: 31n };
    if (!lease.pendingReceipt) await answerLifecycle(worker, lease.close(BUDGET), accepted);
    const retired: ActorInstanceLifecycleReceipt = { ...accepted, kind: "retired" };
    await answerLifecycle(worker, lease.acknowledge(lease.pendingReceipt!, BUDGET), retired);
    await answerLifecycle(worker, lease.acknowledge(retired, BUDGET, fixtureRetirement(lease)));
  }

  //#region 🚪️CapturedLifecycle
  async function fixtureOutputReservation(queue: OwnedActorTurnOutputs): Promise<OwnedActorTurnOutput> {
    const { default: fixture } = await import("../../../🪪️activation/🚪️instance/📥️output/🏘️admission/🧪️fixture/🔣️.json");
    for (let turn = 0; turn < fixture.phases.length + 1; turn++) { const current = queue.reserve({ maxItems: 1, maxBytes: 4096 }); if (current.step.kind === "ready" && current.output) return current.output; expect(current.step.kind).toBe("pending"); }
    throw new Error("Output admission exceeded declared transitions");
  }
  describe("ShardClient reserved response settlement", () => {
    it("captures the exact response before actual pending removal, heartbeat recomputation and caller settlement", async () => {
      const { default: fixture } = await import("../../../🪪️activation/🚪️instance/📥️output/🧪️fixture/🔣️.json");
      const { client, workers, residentLedger } = harness(1); const worker = workers[0]!;
      const slot: ShardSlot = Reflect.get(client, "shards")[0];
      const send = Reflect.get(client, "send").bind(client) as (slot: ShardSlot, message: OutboundMessage, request: string, posted: undefined, output: OwnedActorTurnOutput) => Promise<unknown>;
      const pendingEntries: Map<string, PendingEntry> = Reflect.get(client, "pending");
      const queue = new OwnedActorTurnOutputs({}, fixture.capacity, residentLedger); const output = await fixtureOutputReservation(queue);
      const raw = { kind: "result" as const, requestId: "reserved-fixture", ok: true as const, value: { uiPatches: [] }, unknown: new Uint8Array(fixture.responseSettlement.unknownPayloadBytes) };
      const order: string[] = [];
      const remove = pendingEntries.delete.bind(pendingEntries);
      pendingEntries.delete = key => { expect(output.responseEnvelope).toBe(raw); order.push("pending-removal"); return remove(key); };
      const recompute = Reflect.get(client, "recomputeOldestPending").bind(client);
      Reflect.set(client, "recomputeOldestPending", (target: ShardSlot) => { expect(output.responseEnvelope).toBe(raw); order.push("heartbeat-recompute"); return recompute(target); });
      const result = output.run(() => send(slot, { kind: "turn", requestId: raw.requestId, actorId: "reserved-fixture", activationGeneration: 1n, events: [], budget: BUDGET }, raw.requestId, undefined, output));
      const observed = expect(result.then(() => { expect(output.responseEnvelope).toBe(raw); order.push("external-continuation"); throw new Error("fixture publication fault"); })).rejects.toThrow("fixture publication fault");
      worker.deliver(raw); await observed;
      expect(order).toEqual(fixture.responseSettlement.phases.filter(phase => phase !== "capture" && phase !== "error-graft"));
      expect(output.outcome?.value).toBe(raw.value); expect(queue.peek()?.responseEnvelope).toBe(raw);
      expect(pendingEntries.has(raw.requestId)).toBe(false); client.disposeAll();
    });

    it("retains the original failed response when actual worker error grafting throws", async () => {
      const { default: fixture } = await import("../../../🪪️activation/🚪️instance/📥️output/🧪️fixture/🔣️.json");
      const { client, workers, residentLedger } = harness(1); const worker = workers[0]!;
      const slot: ShardSlot = Reflect.get(client, "shards")[0];
      const send = Reflect.get(client, "send").bind(client) as (slot: ShardSlot, message: OutboundMessage, request: string, posted: undefined, output: OwnedActorTurnOutput) => Promise<unknown>;
      const queue = new OwnedActorTurnOutputs({}, fixture.capacity, residentLedger); const output = await fixtureOutputReservation(queue);
      const raw = { kind: "result" as const, requestId: "graft-fixture", ok: false as const, error: "native refusal", stack: "native stack", framesBytes: 8192, unknown: new Uint8Array(fixture.responseSettlement.unknownPayloadBytes) };
      const result = output.run(() => send(slot, { kind: "turn", requestId: raw.requestId, actorId: "graft-fixture", activationGeneration: 1n, events: [], budget: BUDGET }, raw.requestId, undefined, output));
      const failure = new Error("fixture diagnostic fault"); const observed = expect(result).rejects.toBe(failure);
      const log = vi.spyOn(console, "log").mockImplementation(() => { expect(output.responseEnvelope).toBe(raw); throw failure; });
      try { expect(() => worker.deliver(raw)).not.toThrow(); await observed; } finally { log.mockRestore(); }
      expect(output.responseEnvelope).toBe(raw); expect(output.outcome?.value).toBe(failure); expect(raw.unknown.byteLength).toBe(fixture.responseSettlement.unknownPayloadBytes);
      expect(queue.peek()).toBe(output); client.disposeAll();
    });
  });

  describe("ShardClient captured return authority", () => {
    it("CapturedReturnAdmission validates its exact parent phases and independent fixed ledger inventory", async () => {
      const { default: contract } = await import("../../../🪪️activation/📤️return/🏘️admission/🤝️contract.json"); const { default: schema } = await import("../../../🪪️activation/📤️return/🏘️admission/🧬️schema/🔣️.json");
      const { default: fixture } = await import("../../../🪪️activation/📤️return/🏘️admission/🧪️fixture/🔣️.json"); const fixtureSchema = schema;
      const { default: Ajv } = await import("ajv"); const { produce } = await import("immer"); const ajv = new Ajv({ strict: true }); expect(ajv.addSchema(schema).getSchema(`${schema.$id}#/$defs/Admission`)!(contract)).toBe(true); expect(ajv.getSchema(`${schema.$id}#/$defs/AdmissionFixture`)!(fixture)).toBe(true);
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
      const { default: contract } = await import("../../../🪪️activation/📤️return/🏘️admission/🤝️contract.json"); const ts = await import("typescript"); const { readFile } = await import("node:fs/promises");
      const ast = ts.createSourceFile("shard-client.ts", await readFile(new URL("./🟦️.ts", source.url), "utf8"), ts.ScriptTarget.Latest, true);
      const fields = (name: string) => { const node = ast.statements.find(item => ts.isTypeAliasDeclaration(item) && item.name.text === name); if (!node || !ts.isTypeAliasDeclaration(node) || !ts.isTypeLiteralNode(node.type)) throw new Error("Missing return owner declaration"); return node.type.members.map(item => item.name?.getText(ast)); };
      const instance = fields("ShardInstanceOwner"); expect(instance.slice(-contract.parentFields.length)).toEqual(contract.parentFields); expect(fields("CapturedReturn")).toEqual(contract.stateFields);
      const facade = ast.statements.find(item => ts.isClassDeclaration(item) && item.name?.text === "OwnedShardReturn"); if (!facade || !ts.isClassDeclaration(facade)) throw new Error("Missing return facade"); expect(facade.members.filter(ts.isPropertyDeclaration).map(item => item.name.getText(ast).slice(1))).toEqual(contract.facadeFields);
      const output = ts.createSourceFile("output.ts", await readFile(new URL("../🪪️activation/🚪️instance/📥️output/🟦️.ts", source.url), "utf8"), ts.ScriptTarget.Latest, true);
      const roster = output.statements.find(item => ts.isClassDeclaration(item) && item.name?.text === "OwnedActorTurnOutputs"); if (!roster || !ts.isClassDeclaration(roster)) throw new Error("Missing output roster"); expect(roster.members.filter(ts.isPropertyDeclaration).map(item => item.name.getText(output).slice(1))).toEqual(contract.rosterFields);
      const constructor = roster.members.find(ts.isConstructorDeclaration); expect(constructor?.body?.getText(output)).not.toContain("Object.freeze");
      const lease = ast.statements.find(item => ts.isInterfaceDeclaration(item) && item.name.text === "ShardInstanceLifecycleLease"); if (!lease || !ts.isInterfaceDeclaration(lease)) throw new Error("Missing captured lease");
      const method = lease.members.find(item => ts.isMethodSignature(item) && item.name.getText(ast) === "reserveReturn"); if (!method || !ts.isMethodSignature(method)) throw new Error("Missing phased return method"); expect(method.parameters.map(item => item.name.getText(ast))).toEqual(["maximumResponses", "grant"]);
    });

    it("CapturedReturnAdmission refuses the first zero grant before original parent construction", async () => {
      const { default: fixture } = await import("../../../🪪️activation/📤️return/🏘️admission/🧪️fixture/🔣️.json"); const { client, residentLedger, worker, instance } = await captured(); const before = residentLedger.usage; const sent = worker.sent.length;
      const admission = Reflect.apply(instance.reserveReturn, instance, [fixture.capacity, { maxItems: 0, maxBytes: 0 }]);
      expect(admission).toMatchObject({ step: { kind: fixture.shortGrant.kind, items: fixture.shortGrant.items, bytes: fixture.shortGrant.bytes }, source: null });
      expect(instance.pendingReturn).toBeNull(); expect(residentLedger.usage).toEqual(before); expect(worker.sent.length).toBe(sent); client.disposeAll();
    });
    it("CapturedReturnAdmission installs the actual parent before all twelve granted construction phases", async () => {
      const { default: fixture } = await import("../../../🪪️activation/📤️return/🏘️admission/🧪️fixture/🔣️.json"); const { produce } = await import("immer");
      const { client, residentLedger, worker, instance } = await captured(); const owner: ShardInstanceOwner = Reflect.get(client, "instanceLifecycles").get(instance.openRequest.requestSequence); const initial = residentLedger.usage.data; const posts = worker.sent.length;
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
      const { default: fixture } = await import("../../../🪪️activation/📤️return/🏘️admission/🧪️fixture/🔣️.json"); const { OwnedResidentRecord } = await import("../../../../🌱️value/💾️resident/🟦️.ts");
      for (const boundary of fixture.faultBoundaries) for (const kind of fixture.faultValues) {
        const { client, residentLedger, worker, instance } = await captured(); const owner: ShardInstanceOwner = Reflect.get(client, "instanceLifecycles").get(instance.openRequest.requestSequence); const posts = worker.sent.length; let reads = 0;
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
      const { default: fixture } = await import("../../../🪪️activation/📤️return/🏘️admission/🧪️fixture/🔣️.json");
      for (const prefix of fixture.closing.prefixes) {
        const { client, residentLedger, worker, instance } = await captured(); const owner: ShardInstanceOwner = Reflect.get(client, "instanceLifecycles").get(instance.openRequest.requestSequence);
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
      const { default: fixture } = await import("../../../🪪️activation/🚪️instance/📥️output/🏘️admission/🧪️fixture/🔣️.json");
      const { client, residentLedger, worker, instance } = await captured(); const source = await fixtureCapturedReturn(instance, fixture.capacity); const state = capturedReturnState(source); const before = residentLedger.usage; const posts = worker.sent.length;
      if (!state.outputs) throw new Error("Missing original output roster");
      Reflect.apply(state.outputs.reserve, state.outputs, [{ maxItems: 0, maxBytes: 4096 }]);
      expect(state.outputs.pending).toBe(fixture.short.linked); expect(residentLedger.usage).toEqual(before); expect(worker.sent.length).toBe(posts); expect(source.retainedResponses).toBe(0); client.disposeAll();
    });

    it("ActorResponseAdmission charges each original output before construction and retains cancelled metadata", async () => {
      const { default: fixture } = await import("../../../🪪️activation/🚪️instance/📥️output/🏘️admission/🧪️fixture/🔣️.json"); const { produce } = await import("immer"); const { OwnedResidentRecord } = await import("../../../../🌱️value/💾️resident/🟦️.ts");
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
      const { default: fixture } = await import("../../../🪪️activation/🚪️instance/📥️output/🏘️admission/🧪️fixture/🔣️.json");
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
      const { default: fixture } = await import("../../../🪪️activation/🚪️instance/📥️output/🏘️admission/🧪️fixture/🔣️.json"); const { default: contract } = await import("../../../🪪️activation/🚪️instance/📥️output/🏘️admission/🤝️contract.json"); const { OwnedResidentRecord } = await import("../../../../🌱️value/💾️resident/🟦️.ts");
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
      const { OwnedActorTurnOutput } = await import("../../../🪪️activation/🚪️instance/📥️output/🟦️.ts");
      const { default: fixture } = await import("../../../🪪️activation/🚪️instance/📥️output/🧯️fault/🧪️fixture/🔣️.json"); const { default: schema } = await import("../../../🪪️activation/🚪️instance/📥️output/🧯️fault/🧬️schema/🔣️.json"); const { default: Ajv } = await import("ajv"); expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
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
      const { default: fixture } = await import("../../../🪪️activation/📤️return/🧪️fixture/🔣️.json");
      const { default: schema } = await import("../../../🪪️activation/📤️return/🧬️schema/🔣️.json");
      const { default: Ajv } = await import("ajv"); const { produce } = await import("immer"); const ts = await import("typescript"); const { readFile } = await import("node:fs/promises");
      expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
      for (const boundary of fixture.construction.boundaries) for (const kind of fixture.construction.faults) {
        const { row, client, worker, instance } = await captured(); let reads = 0;
        const fault = kind === "null" ? null : kind === "undefined" ? undefined : kind === "false" ? false : kind === "zero" ? 0 : { payload: new Uint8Array(fixture.construction.unknownBytes), get message() { reads++; throw new Error("Unowned fault getter"); } };
        const observed: { source: OwnedShardReturn | null; before: boolean; fault: unknown } = { source: null, before: false, fault: NO_RESIDENT_FAULT };
        const freeze = Object.freeze; const count = worker.sent.length;
        const finalizer = vi.spyOn(Object, "freeze").mockImplementation(value => {
          if (value instanceof OwnedShardReturn) { observed.source = value; observed.before = instance.pendingReturn === value; if (boundary === "after-finalize") freeze(value); throw fault; }
          return freeze(value);
        });
        const { default: admission } = await import("../../../🪪️activation/📤️return/🏘️admission/🧪️fixture/🔣️.json"); let rejected = false;
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
      const ast = ts.createSourceFile("shard-client.ts", await readFile(new URL("./🟦️.ts", source.url), "utf8"), ts.ScriptTarget.Latest, true);
      const declaration = ast.statements.find(node => ts.isTypeAliasDeclaration(node) && node.name.text === "CapturedReturn");
      if (!declaration || !ts.isTypeAliasDeclaration(declaration) || !ts.isTypeLiteralNode(declaration.type)) throw new Error("Captured return state declaration missing");
      expect(declaration.type.members.map(member => member.name?.getText(ast))).toEqual(fixture.construction.stateFields);
    });

    async function fixtureCapturedReturn(instance: ShardInstanceLifecycleLease, capacity: number): Promise<OwnedShardReturn> {
      const { default: fixture } = await import("../../../🪪️activation/📤️return/🏘️admission/🧪️fixture/🔣️.json");
      let source: OwnedShardReturn | null = null;
      for (const phase of fixture.phases) {
        const current = instance.reserveReturn(capacity, { maxItems: 1, maxBytes: phase.grant });
        expect(current.step.kind).toBe(phase.kind); expect(current.step.bytes).toBe(phase.grant); expect(current.source !== null).toBe(phase.source); source = current.source;
      }
      if (!source) throw new Error("Original captured return was not published"); return source;
    }

    async function fixtureResponse(source: OwnedShardReturn): Promise<void> {
      const { default: fixture } = await import("../../../🪪️activation/🚪️instance/📥️output/🏘️admission/🧪️fixture/🔣️.json");
      for (let turn = 0; turn < fixture.phases.length + 1; turn++) { const current = source.reserveResponse({ maxItems: 1, maxBytes: 4096 }); if (current.kind === "ready") return; expect(current.kind).toBe("pending"); }
      throw new Error("Captured response admission exceeded declared transitions");
    }

    async function captured() {
      const { default: row } = await import("../../../🪪️activation/📤️return/🧪️fixture/🔣️.json");
      const { client, residentLedger, workers } = harness(2, { exclusiveShardCount: 1 });
      const pending = client.activate(row.actorId, "https://fixture.invalid/component.js", [], BUDGET);
      const worker = workers[0]!;
      worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: undefined });
      await pending;
      const instance = await captureFixtureInstance(client, worker, row.actorId);
      return { row, client, residentLedger, workers, worker, instance };
    }

    async function inputStream(lifetime: ActorInstanceLifetime, payloadBytes?: number) {
      const { default: vector } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧫️fixture/🔣️.json");
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
      const { encodeActorReturnResult } = await import("../../../📤️return/🟦️.ts");
      const identity = { origin: source.origin!, returnSequence: BigInt(row.returnSequence) };
      const pageBytes = pageLength === undefined ? stream.bytes : stream.bytes.subarray(0, pageLength);
      const response = { kind: "result" as const, requestId: request.requestId, ok: true as const, value: encodeActorReturnResult({ kind: "page", receipt: { identity, pageSequence: 1n, length: pageBytes.length, final: pageBytes.length === stream.bytes.length }, page: createActorBytePage(pageBytes) }) };
      worker.deliver(response); const report = await pending;
      return { ...context, ...stream, source, response, report };
    }
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
      const { OwnedKernelReturnContent, OwnedKernelReturnInputField, OwnedKernelReturnInputFragment } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
      const { default: schema } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧬️schema/🔣️.json");
      const { default: Ajv } = await import("ajv");
      const { client, instance, source, response, receipt, payload, vector } = await deliveredInput();
      expect(new Ajv({ strict: true }).compile(schema)(vector)).toBe(true);
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
      const { OwnedKernelReturnContent } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
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
      const { OwnedKernelReturnContent } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
      const { default: vector } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧫️fixture/🔣️.json");
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
      const { default: contract } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/📜️contract/🔣️.json"); const { default: schema } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/🧬️schema/🔣️.json");
      const { default: fixture } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/🧫️fixture/🔣️.json");
      const { default: Ajv } = await import("ajv"); const { produce } = await import("immer"); const validator = new Ajv({ strict: true });
      expect(validator.addSchema(schema).getSchema(`${schema.$id}#/$defs/Builder`)!(contract)).toBe(true); expect(validator.getSchema(`${schema.$id}#/$defs/BuilderFixture`)!(fixture)).toBe(true);
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
      const { default: contract } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🏗️builder/📜️contract/🔣️.json"); const ts = await import("typescript"); const { readFile } = await import("node:fs/promises");
      const text = await readFile(new URL("../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts", source.url), "utf8"); const input = ts.createSourceFile("input.ts", text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
      const uiText = await readFile(new URL("../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts", source.url), "utf8"); const ui = ts.createSourceFile("resident.ts", uiText, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
      const field = input.statements.find(item => ts.isClassDeclaration(item) && item.name?.text === "OwnedKernelReturnInputField"); const witness = ui.statements.find(item => ts.isClassDeclaration(item) && item.name?.text === "BuilderWitness");
      if (!field || !ts.isClassDeclaration(field) || !witness || !ts.isClassDeclaration(witness)) throw new Error("Actual private field/witness missing");
      expect(field.members.filter(ts.isPropertyDeclaration).map(item => item.name.getText(input).slice(1))).toEqual(contract.fieldFields); expect(witness.members.filter(ts.isPropertyDeclaration).map(item => item.name.getText(ui).slice(1))).toEqual(contract.witnessFields);
      for (const name of ["detachBuilder", "settleBuilder", "matchesBuilderDetached", "matchesBuilderSettled"]) { const method = field.members.find(item => ts.isMethodDeclaration(item) && item.name.getText(input) === name); expect(method, name).toBeDefined(); }
      const binding = field.members.find(item => ts.isPropertyDeclaration(item) && item.name.getText(input) === "#builder"); expect(binding?.getText(input)).toContain("OwnedUiResidentBuilderRetirement");
    });

    it("OwnedKernelReturnBuilderBinding rejects premature and forged witnesses without releasing the original binding", async () => {
      const { OwnedKernelReturnInputField } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { OwnedUiResidentBuilderRetirement } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts");
      const { client, residentLedger, instance, source, response } = await deliveredInput(); const input = new OwnedKernelReturnContent(source, fixtureHosts.get(instance)!, instance.activation, instance.lifetime!);
      for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 }); const field = input.field!; const fragment = field.fragment;
      const detach = Reflect.get(field, "detachBuilder"); const settle = Reflect.get(field, "settleBuilder"); const detached = Reflect.get(OwnedKernelReturnInputField, "matchesBuilderDetached"); const settled = Reflect.get(OwnedKernelReturnInputField, "matchesBuilderSettled");
      expect([detach, settle, detached, settled].map(value => typeof value)).toEqual(["function", "function", "function", "function"]);
      const pool = await fixtureResidentPool(client, residentLedger); const scope = await fixtureResidentScope(pool, residentLedger, instance); const resident = await fixtureResidentPayload(scope, residentLedger, field);
      const held: { proof: unknown } = { proof: null }; const original = Object.freeze; const spy = vi.spyOn(Object, "freeze").mockImplementation(value => { if (value instanceof OwnedUiResidentBuilderRetirement) held.proof = value; return original(value); });
      let builder: import("../../🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🟦️.ts").OwnedUiOperationPayloadBuilder | null = null;
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
      const { default: contract } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🧾️release/📜️contract/🔣️.json"); const { default: schema } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🧾️release/🧬️schema/🔣️.json");
      const { default: fixture } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🧾️release/🧫️fixture/🔣️.json");
      const { default: Ajv } = await import("ajv"); const { produce } = await import("immer"); const validator = new Ajv({ strict: true });
      expect(validator.addSchema(schema).getSchema(`${schema.$id}#/$defs/Release`)!(contract)).toBe(true); expect(validator.getSchema(`${schema.$id}#/$defs/ReleaseFixture`)!(fixture)).toBe(true);
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
      const { default: contract } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🧾️release/📜️contract/🔣️.json"); const ts = await import("typescript"); const { readFile } = await import("node:fs/promises");
      const text = await readFile(new URL("../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts", source.url), "utf8"); const source = ts.createSourceFile("input.ts", text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
      for (const [name, fields] of [["OwnedKernelReturnInputField", contract.fieldFields], ["OwnedKernelReturnInputFragment", contract.fragmentFields], ["OwnedKernelReturnInputRelease", contract.releaseFields]] as const) {
        const declaration = source.statements.find(item => ts.isClassDeclaration(item) && item.name?.text === name);
        if (!declaration || !ts.isClassDeclaration(declaration)) throw new Error("Actual input class missing");
        expect(declaration.members.filter(ts.isPropertyDeclaration).map(item => item.name.getText(source).slice(1)), name).toEqual(fields);
      }
    });

    it("OwnedKernelReturnInputEvidence rejects unissued or foreign observations without consulting their getters", async () => {
      const { OwnedKernelReturnInputRelease } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
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
      const { default: contract } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/📦️payload/📜️contract/🔣️.json"); const { default: schema } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧬️schema/🔣️.json");
      const { default: fixture } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧫️fixture/🔣️.json");
      const { default: Ajv } = await import("ajv"); const { produce } = await import("immer"); const validator = new Ajv({ strict: true });
      expect(validator.addSchema(schema).getSchema(`${schema.$id}#/$defs/Payload`)!(contract)).toBe(true); expect(validator.getSchema(`${schema.$id}#/$defs/PayloadFixture`)!(fixture)).toBe(true);
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
      const { OwnedKernelReturnContent, OwnedKernelReturnInputField, OwnedKernelReturnPayloadDetachment } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
      const { default: contract } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/📦️payload/📜️contract/🔣️.json"); const { default: fixture } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧫️fixture/🔣️.json"); const ts = await import("typescript"); const { readFile } = await import("node:fs/promises");
      const { client, instance, source, response } = await deliveredInput(); const owner = fixtureHosts.get(instance)!;
      const input = new OwnedKernelReturnContent(source, owner, instance.activation, instance.lifetime!); for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 }); const field = input.field!;
      expect(OwnedKernelReturnInputField.matchesResidentPayload(field, {})).toBe(false); expect(field.residentPayloadDetachment).toBeNull();
      expect(Reflect.apply(field.installResidentPayload, field, [{}, { maxItems: 1, maxBytes: 63 }])).toEqual({ kind: "blocked", items: 0, bytes: 0 });
      expect(Reflect.apply(field.installResidentPayload, field, [{}, { maxItems: 1, maxBytes: 64 }])).toEqual({ kind: "rejected", items: 0, bytes: 0 });
      expect(Reflect.apply(field.residentPayload, field, [{}])).toBeNull(); expect(Reflect.apply(field.detachResidentPayload, field, [{}, {}, { maxItems: 1, maxBytes: 64 }])).toEqual({ kind: "rejected", items: 0, bytes: 0 });
      expect(Reflect.apply(field.settleResidentPayload, field, [{}, {}, { maxItems: 1, maxBytes: 64 }])).toEqual({ kind: "rejected", items: 0, bytes: 0 });
      expect(OwnedKernelReturnPayloadDetachment.matches({}, field, {})).toBe(false); expect(OwnedKernelReturnPayloadDetachment.matchesSettled({}, {})).toBe(false);
      expect(() => Reflect.construct(OwnedKernelReturnPayloadDetachment, [{}, field])).toThrow("return-input.private-payload-detachment");
      const text = await readFile(new URL("../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts", source.url), "utf8"); const parsed = ts.createSourceFile("input.ts", text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
      for (const [name, expected] of [["OwnedKernelReturnInputField", contract.sourceFields], ["OwnedKernelReturnPayloadDetachment", contract.observationFields]] as const) { const declaration = parsed.statements.find(item => ts.isClassDeclaration(item) && item.name?.text === name); if (!declaration || !ts.isClassDeclaration(declaration)) throw new Error("Actual source class missing"); expect(declaration.members.filter(ts.isPropertyDeclaration).map(item => item.name.getText(parsed).slice(1))).toEqual(expected); }
      const ownerType = parsed.statements.find(item => ts.isTypeAliasDeclaration(item) && item.name.text === "InputOwner"); if (!ownerType || !ts.isTypeAliasDeclaration(ownerType) || !ts.isTypeLiteralNode(ownerType.type)) throw new Error("Actual input owner type missing");
      expect(ownerType.type.members.filter(ts.isPropertySignature).map(item => item.name.getText(parsed))).toEqual(fixture.construction.inputOwnerFields);
      const ownerMetadata = fixture.construction.inputOwnerSubset; expect(ownerMetadata.bytes).toBe(ownerMetadata.recordBytes + ownerMetadata.fieldBytes * fixture.construction.inputOwnerFields.length); expect(ownerMetadata.additionalBytes).toBe(ownerMetadata.fieldBytes * (ownerMetadata.fields - ownerMetadata.priorFields));
      expect(field.residentPayloadDetachment).toBeNull(); expect(input.field).toBe(field); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response); input.beginClose(); client.disposeAll();
    });

    it("OwnedKernelReturnInput retains exact field construction roots and arbitrary finalizer failures", async () => {
      const { OwnedKernelReturnContent, OwnedKernelReturnInputField, OwnedKernelReturnInputFragment, OwnedKernelReturnPayloadDetachment } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
      const { default: fixture } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧫️fixture/🔣️.json"); const row = fixture.construction; const freeze = Object.freeze;
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

    it("OwnedKernelReturnInput admits its exact instance through the released nine-phase shared scope", async () => {
      const { default: slot } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture/🔣️.json");
      const { client, residentLedger, instance, source, response } = await deliveredInput();
      const pool = await fixtureResidentPool(client, residentLedger); const before = residentLedger.usage.data;
      const scope = await fixtureResidentScope(pool, residentLedger, instance); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response);
      scope.beginClose(); for (let turn = 0; turn < slot.firstChild.retirementTurns && !scope.terminalIsEmpty(); turn++) expect(scope.closeStep({ maxItems: 1, maxBytes: 4096 }).kind).not.toBe("rejected");
      expect(scope.terminalIsEmpty()).toBe(true); expect(residentLedger.usage.data).toEqual(before); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response); client.disposeAll();
    });

    it("OwnedKernelReturnInput settles the genuine payload in exact charged phases after operation revocation", async () => {
      const { OwnedKernelReturnInputField, OwnedKernelReturnPayloadDetachment } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
      const { OwnedUiResidentPayloadSourceRelease } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts");
      const { default: fixture } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧫️fixture/🔣️.json"); const { produce } = await import("immer");
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
      const { OwnedKernelReturnInputField, OwnedKernelReturnPayloadDetachment } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
      const { OwnedUiResidentPayloadSourceRelease } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts");
      const { default: fixture } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/📦️payload/🧫️fixture/🔣️.json"); const { default: ui } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️fixture/🔣️.json");
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
      const { OwnedUiOperationPayloadBuilder } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🟦️.ts");
      const { client, residentLedger, instance, source, response, worker } = await deliveredInput(); const owner = fixtureHosts.get(instance)!;
      const input = new OwnedKernelReturnContent(source, owner, instance.activation, instance.lifetime!); for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 }); const field = input.field!;
      const pool = await fixtureResidentPool(client, residentLedger); const scope = await fixtureResidentScope(pool, residentLedger, instance); const resident = await fixtureResidentPayload(scope, residentLedger, field); const posts = worker.sent.length;
      const admitted = await fixtureResidentBuilder(residentLedger, field, resident); const builder = admitted.builder; expect(admitted.step.kind).toBe("ready"); expect(builder).not.toBeNull();
      if (!builder) throw new Error("Original builder was not admitted"); expect(OwnedUiOperationPayloadBuilder.healthy(builder)).toBe(true); expect(OwnedUiOperationPayloadBuilder.empty(builder)).toBe(false);
      expect(worker.sent.length).toBe(posts); expect(capturedReturnState(source).outputs?.peek()?.responseEnvelope).toBe(response); expect(field.consumed).toBe(0n); expect(field.fragment?.field).toBe(field);
      input.beginClose(); builder.beginClose(); expect(OwnedUiOperationPayloadBuilder.empty(builder)).toBe(false); client.disposeAll();
    });

    it("OwnedKernelReturnInput privately identifies its bound builder after the real bind call throws", async () => {
      const { OwnedKernelReturnContent, OwnedKernelReturnInputField } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
      const { OwnedUiResidentPool } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts");
      const { OwnedUiOperationPayloadBuilder } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🟦️.ts");
      const { default: schema } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧬️schema/🔣️.json");
      const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");
      const { client, residentLedger, instance, source, response, vector } = await deliveredInput();
      expect(new Ajv({ strict: true }).compile(schema)(vector)).toBe(true);
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
      const { OwnedKernelReturnContent } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
      const { OwnedUiOperationInputCancelled } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🟦️.ts");
      const { default: evidenceFixture } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️fixture/🔣️.json");
      const { default: builderFixture } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture/🔣️.json");
      const { default: payloadFixture } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️fixture/🔣️.json");
      const { default: scopeFixture } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture/🔣️.json");
      const { default: schema } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧬️schema/🔣️.json");
      const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");
      const { client, residentLedger, instance, source, response, vector, payload } = await deliveredInput();
      expect(new Ajv({ strict: true }).compile(schema)(vector)).toBe(true);
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
      const { OwnedKernelReturnInputFragment } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
      const { OwnedUiOperationInputCopied } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🟦️.ts");
      const { default: vector } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧫️fixture/🔣️.json");
      const { default: readers } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture/🔣️.json");
      const { default: pages } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧪️fixture/🔣️.json");
      const { default: binding } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧪️fixture/🔣️.json");
      const { default: evidence } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️fixture/🔣️.json");
      const { default: copied } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧪️fixture/🔣️.json");
      const { default: builders } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture/🔣️.json");
      const { default: payloads } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️fixture/🔣️.json");
      const { default: scopes } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture/🔣️.json");
      const { default: copiedSchema } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema/🔣️.json");
      const { default: Ajv } = await import("ajv"); const { Buffer } = await import("node:buffer"); const { produce } = await import("immer");
      expect(new Ajv({ strict: true }).addSchema(copiedSchema).getSchema(`${copiedSchema.$id}#/$defs/CopiedFixture`)!(copied)).toBe(true);
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
      const { default: vector } = await import("../../../../🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧫️fixture/🔣️.json");
      const { default: readers } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture/🔣️.json");
      const { default: pages } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🧪️fixture/🔣️.json");
      const { default: binding } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧪️fixture/🔣️.json");
      const { default: evidence } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️fixture/🔣️.json");
      const { default: copied } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧪️fixture/🔣️.json");
      const { default: builders } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture/🔣️.json");
      const { default: payloads } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️fixture/🔣️.json");
      const { default: scopes } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture/🔣️.json");
      const { default: bindingSchema } = await import("../../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/📃️page/🔗️binding/🧬️schema/🔣️.json");
      const { default: Ajv } = await import("ajv"); const { Buffer } = await import("node:buffer");
      expect(new Ajv({ strict: true }).addSchema(bindingSchema).getSchema(`${bindingSchema.$id}#/$defs/BindingFixture`)!(binding)).toBe(true);
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

    it("mints a page only from the original captured response and keeps controls on its old worker", async () => {
      const { default: Ajv } = await import("ajv");
      const { default: schema } = await import("../../../🪪️activation/📤️return/🧬️schema/🔣️.json");
      const { encodeActorReturnResult, decodeActorReturnDrive } = await import("../../../📤️return/🟦️.ts");
      const { row, client, workers, worker, instance } = await captured();
      const oracle = new Ajv({ strict: true }); expect(oracle.validate(schema, row)).toBe(true);
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
      const entries: Map<string, PendingEntry> = Reflect.get(client, "pending"); const remove = entries.delete.bind(entries);
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
      expect(actual).toEqual(row.expected); expect(oracle.validate({ const: row.expected }, actual)).toBe(true);
      expect(pageOutput?.responseEnvelope).toBe(pageResponse);
      expect(source.reserveResponse({ maxItems: 1, maxBytes: 4096 }).kind).toBe("pending"); expect(source.reserveResponse({ maxItems: 1, maxBytes: 4096 }).kind).toBe("blocked"); await expect(source.poll(BUDGET)).rejects.toThrow("actor-return.response-admission-required");
      expect(source.page).toBe(page); expect(source.retainedResponses).toBe(row.responseSlots);
      expect(() => instance.dispose()).toThrow("actor-close.native-retirement-pending");
      client.disposeAll();
    });

    it("retains foreign replies and refuses replaced workers without redirecting cancellation", async () => {
      const { encodeActorReturnResult, decodeActorReturnDrive } = await import("../../../📤️return/🟦️.ts");
      const { row, client, workers, worker, instance } = await captured();
      const source = await fixtureCapturedReturn(instance, row.responseSlots);
      await fixtureResponse(source); const running = source.execute([], BUDGET); const observed = expect(running).rejects.toThrow("actor-return.foreign-origin");
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
      const { encodeActorReturnResult, decodeActorReturnDrive } = await import("../../../📤️return/🟦️.ts");
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
      const { encodeActorReturnResult } = await import("../../../📤️return/🟦️.ts");
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
      const { encodeActorReturnResult } = await import("../../../📤️return/🟦️.ts");
      const { row, client, worker, instance } = await captured();
      const source = await fixtureCapturedReturn(instance, row.responseSlots); await fixtureResponse(source); const running = source.execute([], BUDGET);
      let request = worker.sent.at(-1) as { requestId: string };
      const identity = { origin: source.origin!, returnSequence: BigInt(row.returnSequence) };
      worker.deliver({ kind: "result", requestId: request.requestId, ok: true, value: encodeActorReturnResult({ kind: "pending", identity, reason: "working" }) }); await running;
      await fixtureResponse(source); const polling = source.poll(BUDGET); const observed = expect(polling).rejects.toThrow();
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
  });

  describe("ShardClient captured instance lifecycle", () => {
    const openInput = { appId: "fixture", actor: "fixture", config: new Uint8Array(), assets: [], capabilities: [], quotas: new Uint8Array() };
    async function activateCaptured(client: ShardClient, worker: FakeShardWorker, actorId = "captured") {
      const activated = client.activate(actorId, "https://fixture.invalid/actor.js", [], BUDGET);
      worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: undefined });
      await activated;
      return client.captureInstanceLifecycle(actorId, 7);
    }
    async function answer(worker: FakeShardWorker, pending: Promise<unknown>, receipt?: ActorInstanceLifecycleReceipt) {
      const message = worker.sent.at(-1) as { requestId: string };
      worker.deliver({ kind: "result", requestId: message.requestId, ok: true, value: { status: { tag: "idle" }, lifecycleReceipt: receipt ? encodeActorInstanceLifecycle(receipt) : undefined } });
      return pending;
    }
    async function openCaptured(worker: FakeShardWorker, lease: ShardInstanceLifecycleLease, guestLifetime = 13n) {
      const captured: ActorInstanceLifecycleReceipt = { kind: "captured", lifetime: { activationGeneration: lease.activation.activationGeneration, instanceId: 7, guestLifetime }, requestSequence: lease.openRequest.requestSequence };
      await answer(worker, lease.open(openInput, BUDGET), captured);
      await answer(worker, lease.acknowledge(captured, BUDGET));
      return captured;
    }
    it("joins canonical captured accepted retired and exact ACK with host retirement", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const fixture = JSON.parse(readFileSync(new URL("../🪪️activation/🚪️instance/🧪️fixture/🔣️.json", source.url), "utf8"));
      const schema = JSON.parse(readFileSync(new URL("../🪪️activation/🚪️instance/🧬️schema/🔣️.json", source.url), "utf8"));
      const oracle = new Ajv({ strict: true });
      expect(oracle.validate(schema, fixture)).toBe(true);
      const { client, workers } = harness(1);
      const worker = workers[0]!;
      const lease = await activateCaptured(client, worker);
      const phases = [lease.progress().kind];
      expect(lease.lifetime).toBeNull();
      expect(() => lease.beginClose()).toThrow("actor-lifecycle.capture-pending");
      expect(() => client.dispose("captured")).toThrow("actor-close.native-retirement-pending");
      const captured: ActorInstanceLifecycleReceipt = { kind: "captured", lifetime: { activationGeneration: 1n, instanceId: fixture.instanceId, guestLifetime: BigInt(fixture.guestLifetime) }, requestSequence: lease.openRequest.requestSequence };
      await answer(worker, lease.open(openInput, BUDGET), captured);
      phases.push(lease.progress().kind);
      expect(lease.lifetime).toEqual(captured.lifetime);
      expect(Object.isFrozen(lease.lifetime)).toBe(true);
      await answer(worker, lease.acknowledge(captured, BUDGET));
      phases.push(lease.progress().kind);
      const host = new OwnedUiInstance(lease.activation, lease.lifetime!, { maxNodes: 128, maxDepth: 16, maxChildren: 32, maxTextBytes: 4096, maxPatchOps: 128, maxPatchBytes: 65536 }, { usizeBits: 32 });
      lease.bindHostRetirement(host);
      const close = lease.beginClose();
      phases.push(lease.progress().kind);
      expect(lease.beginClose()).toBe(close);
      await expect(lease.activation.turn([{ kind: "request", payload: {} }], BUDGET)).rejects.toThrow("actor-activation.revoked");
      const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: captured.lifetime, requestSequence: close.requestSequence, closeGeneration: 41n };
      await answer(worker, lease.close(BUDGET), accepted);
      phases.push(lease.progress().kind);
      const retired: ActorInstanceLifecycleReceipt = { ...accepted, kind: "retired" };
      await answer(worker, lease.acknowledge(accepted, BUDGET), retired);
      phases.push(lease.progress().kind);
      const before = worker.sent.length;
      await expect(lease.acknowledge(retired, BUDGET)).rejects.toThrow("actor-lifecycle.host-retirement-pending");
      expect(worker.sent).toHaveLength(before);
      expect(() => client.dispose("captured")).toThrow("actor-close.native-retirement-pending");
      host.beginClose();
      expect(host.closeStep({ maxItems: 1, maxBytes: 4096 }).kind).toBe("complete");
      const witness = host.takeRetirementWitness()!;
      await expect(lease.acknowledge(retired, BUDGET, Object.create(OwnedUiInstanceRetirement.prototype))).rejects.toThrow("actor-lifecycle.host-retirement-pending");
      await answer(worker, lease.acknowledge(retired, BUDGET, witness));
      phases.push(lease.progress().kind);
      expect(phases).toEqual(fixture.phases);
      expect(oracle.validate({ const: fixture.phases }, phases)).toBe(true);
      const events = worker.sent.flatMap(value => (value as { events?: ShardEventEnvelope[] }).events ?? []);
      expect(events.map(event => event.kind)).toEqual(["instance-open", "instance-lifecycle-ack", "instance-close", "instance-lifecycle-ack", "instance-lifecycle-ack"]);
      expect(events[1]!.payload).toEqual({ kind: "ack", receipt: captured });
      client.dispose("captured");
    });
    it("keeps exact lifecycle authority after route revocation and rejects a replacement guest receipt", async () => {
      const { client, workers } = harness(2, { exclusiveShardCount: 1 });
      const worker = workers[0]!;
      const lease = await activateCaptured(client, worker);
      await openCaptured(worker, lease);
      client.leaseExclusive("captured");
      const close = lease.beginClose();
      const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: lease.lifetime!, requestSequence: close.requestSequence, closeGeneration: 41n };
      const pending = lease.close(BUDGET);
      const requestId = (worker.sent.at(-1) as { requestId: string }).requestId;
      workers[1]!.deliver({ kind: "result", requestId, ok: true, value: { status: { tag: "idle" }, lifecycleReceipt: encodeActorInstanceLifecycle(accepted) } });
      expect(lease.pendingReceipt).toBeNull();
      await answer(worker, pending, accepted);
      expect(workers[1]!.sent).toHaveLength(0);
      await expect(answer(worker, lease.acknowledge(accepted, BUDGET), { ...accepted, kind: "retired", lifetime: { ...accepted.lifetime, guestLifetime: 14n } })).rejects.toThrow("actor-lifecycle.receipt-mismatch");
      expect(lease.progress().kind).not.toBe("complete");
      client.disposeAll();
    });
    it("retains close authority after transport failure and worker loss without admitting commands", async () => {
      const { client, workers } = harness(1);
      const worker = workers[0]!;
      const lease = await activateCaptured(client, worker);
      await openCaptured(worker, lease);
      const request = lease.beginClose();
      const post = worker.postMessage.bind(worker);
      worker.postMessage = () => { throw new Error("fixture transport refused"); };
      await expect(lease.close(BUDGET)).rejects.toThrow("fixture transport refused");
      expect(lease.progress()).toEqual({ kind: "blocked", failure: "transport-refused" });
      expect(lease.beginClose()).toBe(request);
      worker.postMessage = post;
      const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: lease.lifetime!, requestSequence: request.requestSequence, closeGeneration: 41n };
      await answer(worker, lease.close(BUDGET), accepted);
      const pending = lease.acknowledge(accepted, BUDGET);
      const observed = expect(pending).rejects.toThrow("terminated");
      const requestId = (worker.sent.at(-1) as { requestId: string }).requestId;
      client.terminate(0);
      await observed;
      client.rebuild(0);
      const replacement = await activateCaptured(client, workers[1]!);
      worker.deliver({ kind: "result", requestId, ok: true, value: { status: { tag: "idle" }, lifecycleReceipt: encodeActorInstanceLifecycle({ ...accepted, kind: "retired" }) } });
      expect(lease.progress()).toEqual({ kind: "blocked", failure: "worker-lost" });
      expect(replacement.lifetime).toBeNull();
      await expect(lease.poll(BUDGET)).rejects.toThrow("actor-lifecycle.worker-lost");
      client.disposeAll();
    });
    it("mints patch authority only from the exact captured turn and rejects forged UI ACKs", async () => {
      const { client, workers } = harness(1);
      const worker = workers[0]!;
      const lease = await activateCaptured(client, worker);
      await openCaptured(worker, lease);
      const patch = { surface: { instance: 7, surface: "main" }, revision: 1n, baseRevision: 0n, ops: [{ tag: "set-root", val: 0n }] };
      const result = { uiPatches: [patch], uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime: lease.lifetime!, patchSequence: 1n }), lifecycleReceipt: undefined, status: { tag: "idle" } };
      const pending = lease.poll(BUDGET);
      worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: result });
      expect(await pending).toBe(result);
      expect(() => lease.captureUiPatchAuthority({ ...result }, 0)).toThrow("actor-lifecycle.foreign-turn");
      expect(() => lease.captureUiPatchAuthority(result, 1)).toThrow("actor-lifecycle.patch-index");
      const source = lease.captureUiPatchAuthority(result, 0);
      expect(lease.captureUiPatchAuthority(result, 0)).toBe(source);
      expect(OwnedNativeUiPatchAuthority.matches(source, lease.activation, lease.lifetime!)).toBe(true);
      expect(OwnedNativeUiPatchAuthority.matches(Object.create(OwnedNativeUiPatchAuthority.prototype), lease.activation, lease.lifetime!)).toBe(false);
      expect(source.operation(0)).toBe(patch.ops[0]);
      expect(source.acceptInput({} as never)).toBe(false);
      expect(source.releaseInput({ ordinal: 0 } as OwnedUiPatchInputRetirement)).toBe(false);
      expect(source.inputRetired).toBe(false);
      expect(source.value).toMatchObject({ surface: "main", revision: 1, baseRevision: 0, operationCount: 1 });
      const before = worker.sent.length;
      await expect(lease.submitUiAcknowledgement(source, {} as OwnedUiPatchAcknowledgement, BUDGET)).rejects.toThrow("actor-lifecycle.ui-ack-mismatch");
      expect(worker.sent).toHaveLength(before);
      expect(OwnedNativeUiPatchSubmissionReceipt.matches(Object.create(OwnedNativeUiPatchSubmissionReceipt.prototype), source, {})).toBe(false);
      client.disposeAll();
    });
    it("requires the exact producer UI patch receipt and retains original claims on malformed or duplicate turns", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../🚪️lifetime/🩹️patch/🧫️fixture/🔣️.json", source.url), "utf8"));
      const { client, workers } = harness(1); const worker = workers[0]!; const lease = await activateCaptured(client, worker); await openCaptured(worker, lease);
      const receipt = { lifetime: lease.lifetime!, patchSequence: BigInt(fixture.vectors[1].value.patchSequence) };
      const patch = { surface: { instance: 7, surface: fixture.feedback.surface }, revision: 1n, baseRevision: 0n, ops: [] };
      async function returned(uiPatchReceipt: unknown, patches: unknown[] = [patch]) {
        const result = { status: { tag: "idle" }, uiPatches: patches, uiPatchReceipt };
        const pending = lease.poll(BUDGET);
        worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: result });
        expect(await pending).toBe(result); return result;
      }
      const validBytes = encodeActorUiPatchReceipt(receipt);
      const invalid: unknown[] = [undefined, null, new Uint8Array(36), encodeActorUiPatchReceipt({ ...receipt, lifetime: { ...receipt.lifetime, guestLifetime: receipt.lifetime.guestLifetime + 1n } }), encodeActorUiPatchReceipt({ ...receipt, lifetime: { ...receipt.lifetime, activationGeneration: receipt.lifetime.activationGeneration + 1n } })];
      for (const hex of fixture.invalidHex) invalid.push(Buffer.from(hex, "hex"));
      for (const wire of invalid) {
        const result = await returned(wire);
        expect(() => lease.captureUiPatchAuthority(result, 0)).toThrow();
        expect(result.uiPatches[0]).toBe(patch); expect(result.uiPatchReceipt).toBe(wire);
      }
      const overfull = await returned(validBytes, [patch, patch]);
      expect(() => lease.captureUiPatchAuthority(overfull, 0)).toThrow("actor-ui-patch.pairing");
      const original = await returned(validBytes); const source = lease.captureUiPatchAuthority(original, 0);
      expect(source.value.receipt).toEqual(receipt); expect(Object.isFrozen(source.value.receipt)).toBe(true); expect(Object.isFrozen(source.value.receipt.lifetime)).toBe(true);
      expect(lease.captureUiPatchAuthority(original, 0)).toBe(source);
      const duplicate = await returned(validBytes);
      expect(() => lease.captureUiPatchAuthority(duplicate, 0)).toThrow("actor-ui-patch.duplicate-sequence");
      expect(lease.captureUiPatchAuthority(original, 0)).toBe(source); expect(duplicate.uiPatches[0]).toBe(patch);
      validBytes.fill(0); expect(source.value.receipt).toEqual(receipt);
      client.disposeAll();
    });
    it("retains ordinary turn output on its exact captured instance through settlement revocation", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../🪪️activation/🚪️instance/🧪️fixture/🔣️.json", source.url), "utf8"));
      for (const outcome of fixture.ordinaryOutput) {
        const { client, workers } = harness(1); const worker = workers[0]!; const lease = await activateCaptured(client, worker); await openCaptured(worker, lease);
        const result = { uiPatches: [{ surface: { instance: fixture.instanceId, surface: "main" }, revision: 1n, baseRevision: 0n, ops: [] }], uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime: lease.lifetime!, patchSequence: 1n }), effects: [], status: { tag: "idle" } };
        const pending = client.turn(lease.activation.actorId, [], BUDGET);
        const observed = outcome === "revoked" ? expect(pending).rejects.toThrow("actor-activation.revoked") : expect(pending).resolves.toBe(result);
        if (outcome === "revoked") lease.beginClose();
        worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: result });
        await observed;
        const source = lease.captureUiPatchAuthority(result, 0); expect(OwnedNativeUiPatchAuthority.matches(source, lease.activation, lease.lifetime!)).toBe(true);
        if (outcome === "revoked") expect(lease.interruptedTurn).toBe(result);
        client.disposeAll();
      }
    });

    it("retains exact receipt authority on faulted refused clock-fault and malformed ACK turns", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../🪪️activation/🚪️instance/🧪️fixture/🔣️.json", source.url), "utf8"));
      for (const status of fixture.invalidAckStatuses) {
        const { client, workers } = harness(1);
        const worker = workers[0]!;
        const lease = await activateCaptured(client, worker);
        const captured: ActorInstanceLifecycleReceipt = { kind: "captured", lifetime: { activationGeneration: 1n, instanceId: 7, guestLifetime: 13n }, requestSequence: lease.openRequest.requestSequence };
        await answer(worker, lease.open(openInput, BUDGET), captured);
        const pending = lease.acknowledge(captured, BUDGET);
        const rejected = expect(pending).rejects.toThrow("actor-lifecycle.ack-not-admitted");
        worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: { status } });
        await rejected;
        expect(lease.pendingReceipt).toEqual(captured);
        expect(() => lease.beginClose()).toThrow("actor-lifecycle.capture-pending");
        await answer(worker, lease.acknowledge(captured, BUDGET));
        expect(lease.progress().kind).toBe("open");
        client.disposeAll();
      }
    });
    it("cancels only the captured activation one effect per lifecycle turn", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const fixture = JSON.parse(readFileSync(new URL("../🪪️activation/🚪️instance/🧪️fixture/🔣️.json", source.url), "utf8"));
      const signals: AbortSignal[] = [];
      const { client, workers } = harness(1, { onHostEffect: (_actor, _effect, _params, signal) => { signals.push(signal); return new Promise(() => {}); } });
      const worker = workers[0]!;
      const lease = await activateCaptured(client, worker);
      await openCaptured(worker, lease);
      for (const requestId of ["one", "two"]) worker.deliver(makeEffectRequestFrame("captured", "storage-read", requestId, {}));
      lease.beginClose();
      const counts = [signals.filter(signal => signal.aborted).length];
      await answer(worker, lease.poll(BUDGET));
      counts.push(signals.filter(signal => signal.aborted).length);
      await answer(worker, lease.poll(BUDGET));
      counts.push(signals.filter(signal => signal.aborted).length);
      expect(counts).toEqual(fixture.closeCancellation);
      expect(new Ajv().validate({ const: fixture.closeCancellation }, counts)).toBe(true);
      client.terminate(0);
      client.rebuild(0);
      const replacement = await activateCaptured(client, workers[1]!);
      await openCaptured(workers[1]!, replacement);
      workers[1]!.deliver(makeEffectRequestFrame("captured", "storage-read", "replacement", {}, replacement.activation.activationGeneration));
      expect(() => lease.beginClose()).not.toThrow();
      expect(signals.at(-1)!.aborted).toBe(false);
      client.disposeAll();
    });
    it("cannot close an old open owner through the replacement activation effect ledger", async () => {
      const signals: AbortSignal[] = [];
      const { client, workers } = harness(1, { onHostEffect: (_actor, _effect, _params, signal) => { signals.push(signal); return new Promise(() => {}); } });
      const old = await activateCaptured(client, workers[0]!);
      await openCaptured(workers[0]!, old);
      client.terminate(0);
      client.rebuild(0);
      const fresh = await activateCaptured(client, workers[1]!);
      await openCaptured(workers[1]!, fresh);
      workers[1]!.deliver(makeEffectRequestFrame("captured", "storage-read", "replacement", {}, fresh.activation.activationGeneration));
      expect(() => old.beginClose()).toThrow("actor-lifecycle.worker-lost");
      expect(signals[0]!.aborted).toBe(false);
      client.disposeAll();
    });
  });
  //#endregion 🚪️CapturedLifecycle

  //#region 🪪️ActivationLease
  describe("ShardClient activation lease", () => {
    async function fixture() {
      const { readFileSync } = await import("node:fs");
      return JSON.parse(readFileSync(new URL("../🪪️activation/🧪️fixture/🔣️.json", source.url), "utf8")) as { actorId: string; instanceId: number; revocations: Array<{ action: string; expected: { activeBefore: boolean; activeAfter: boolean; newTurns: number } }> };
    }

    async function activate(client: ShardClient, worker: FakeShardWorker, actorId: string) {
      const pending = client.activate(actorId, "https://fixture.invalid/actor.js", [], BUDGET);
      const message = worker.sent.at(-1) as { requestId: string };
      worker.deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
      await pending;
      return client.captureActorActivation(actorId);
    }

    it("validates the neutral cases and captures only a ready immutable activation", async () => {
      const { default: Ajv } = await import("ajv");
      const { readFileSync } = await import("node:fs");
      const row = await fixture();
      const schema = JSON.parse(readFileSync(new URL("../🪪️activation/🧬️schema/🔣️.json", source.url), "utf8"));
      expect(new Ajv().validate(schema, row)).toBe(true);
      const { client, workers } = harness(1);
      expect(() => client.captureActorActivation(row.actorId)).toThrow("actor-activation.not-ready");
      const pending = client.activate(row.actorId, "https://fixture.invalid/actor.js", [], BUDGET);
      expect(() => client.captureActorActivation(row.actorId)).toThrow("actor-activation.not-ready");
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await pending;
      const lease = client.captureActorActivation(row.actorId);
      expect(Object.isFrozen(lease)).toBe(true);
      expect(lease.actorId).toBe(row.actorId);
      expect(lease.activationGeneration).toBe(1n);
      expect(() => lease.assertActive()).not.toThrow();
      expect(workers[0]!.sent).toHaveLength(1);
      client.dispose(row.actorId);
    });

    it("revokes each close loss or disposal without dispatching another operation", async () => {
      const { default: Ajv } = await import("ajv");
      const row = await fixture();
      const oracle = new Ajv();
      for (const test of row.revocations) {
        const { client, workers } = harness(1);
        const worker = workers[0]!;
        const lease = await activate(client, worker, row.actorId);
        lease.assertActive();
        const close = test.action.startsWith("close") ? await captureFixtureInstance(client, worker, row.actorId, row.instanceId) : null;
        const post = worker.postMessage.bind(worker);
        if (test.action === "close-refused") worker.postMessage = () => { throw new Error("fixture transport refused"); };
        if (close) {
          close.beginClose();
          if (test.action === "close-refused") await expect(close.close(BUDGET)).rejects.toThrow("fixture transport refused");
        }
        else if (test.action === "dispose") client.dispose(row.actorId);
        else if (test.action === "worker-error") worker.onerror?.(new Error("fixture worker lost"));
        else if (test.action === "terminate") client.terminate(0);
        else client.disposeAll();
        const before = worker.sent.length;
        let activeAfter = true;
        try { lease.assertActive(); } catch { activeAfter = false; }
        await expect(lease.turn([], BUDGET)).rejects.toThrow("actor-activation.revoked");
        const actual = { activeBefore: true, activeAfter, newTurns: worker.sent.length - before };
        expect(actual, test.action).toEqual(test.expected);
        expect(oracle.validate({ const: test.expected }, actual), test.action).toBe(true);
        if (close) {
          expect(close.progress().kind).toBe(test.action === "close" ? "closing" : "blocked");
          worker.postMessage = post;
          await retireFixtureInstance(worker, close);
          client.dispose(row.actorId);
        }
      }
    });

    it("never refreshes an old lease when the same actor name is activated again", async () => {
      const { actorId } = await fixture();
      const { client, workers } = harness(1);
      const old = await activate(client, workers[0]!, actorId);
      client.dispose(actorId);
      const fresh = await activate(client, workers[0]!, actorId);
      expect(fresh.activationGeneration).toBe(old.activationGeneration + 1n);
      expect(() => old.assertActive()).toThrow("actor-activation.revoked");
      expect(() => fresh.assertActive()).not.toThrow();
      client.dispose(actorId);
    });

    it("never refreshes an old lease after replacement of its worker at the same slot", async () => {
      const { actorId } = await fixture();
      const { client, workers } = harness(1);
      const old = await activate(client, workers[0]!, actorId);
      client.terminate(0);
      client.rebuild(0);
      const fresh = await activate(client, workers[1]!, actorId);
      expect(workers[1]!.index).toBe(workers[0]!.index);
      expect(() => old.assertActive()).toThrow("actor-activation.revoked");
      expect(() => fresh.assertActive()).not.toThrow();
      client.dispose(actorId);
    });

    it("permanently revokes a released route even when the same worker route is reacquired", async () => {
      const { actorId, instanceId } = await fixture();
      const { client, workers } = harness(2, { exclusiveShardCount: 1 });
      client.leaseExclusive(actorId);
      const operation = await activate(client, workers[1]!, actorId);
      const close = await captureFixtureInstance(client, workers[1]!, actorId, instanceId);
      client.releaseExclusive(actorId);
      expect(client.leaseExclusive(actorId)).toBe(1);
      expect(() => operation.assertActive()).toThrow("actor-activation.revoked");
      expect(() => client.captureActorActivation(actorId)).toThrow("actor-activation.revoked");
      close.beginClose();
      expect(close.progress().kind).toBe("closing");
      await retireFixtureInstance(workers[1]!, close);
      client.dispose(actorId);
    });

    it("keeps the old close root after moving operations onto an exclusive route", async () => {
      const { actorId, instanceId } = await fixture();
      const { client, workers } = harness(2, { exclusiveShardCount: 1 });
      const operation = await activate(client, workers[0]!, actorId);
      const close = await captureFixtureInstance(client, workers[0]!, actorId, instanceId);
      client.leaseExclusive(actorId);
      expect(() => operation.assertActive()).toThrow("actor-activation.revoked");
      close.beginClose();
      expect(workers[1]!.sent).toHaveLength(0);
      await retireFixtureInstance(workers[0]!, close);
      expect(close.progress().kind).toBe("complete");
      client.disposeAll();
    });

    it("disposes the captured worker after a moved or released route and preserves refusal for retry", async () => {
      const { default: rows } = await import("../../../🪪️activation/🧪️fixture/🔣️.json");
      const { default: schema } = await import("../../../🪪️activation/🧬️schema/🔣️.json");
      const { default: Ajv } = await import("ajv");
      const oracle = new Ajv({ strict: true });
      expect(oracle.validate(schema, rows)).toBe(true);
      for (const row of rows.disposals) {
        const { client, workers } = harness(2, { exclusiveShardCount: 1 });
        const original = workers[row.route === "released" ? 1 : 0]!;
        const other = workers[row.route === "released" ? 0 : 1]!;
        if (row.route === "released") client.leaseExclusive(rows.actorId);
        const operation = await activate(client, original, rows.actorId);
        const owner = await captureFixtureInstance(client, original, rows.actorId, rows.instanceId);
        if (row.route === "released") client.releaseExclusive(rows.actorId); else client.leaseExclusive(rows.actorId);
        expect(() => operation.assertActive()).toThrow("actor-activation.revoked");
        await retireFixtureInstance(original, owner);
        const route = client.shardIndexFor(rows.actorId);
        const post = original.postMessage.bind(original);
        if (row.refuseFirst) {
          original.postMessage = () => { throw new Error("fixture dispose refused"); };
          expect(() => client.dispose(rows.actorId)).toThrow("fixture dispose refused");
          expect(client.shardIndexFor(rows.actorId)).toBe(route);
          expect(owner.progress().kind).toBe("complete");
          await expect(client.activate(rows.actorId, "https://fixture.invalid/replacement.js", [], BUDGET)).rejects.toThrow("actor-close.activation-already-owned");
          original.postMessage = post;
        }
        client.dispose(rows.actorId);
        const isDisposal = (value: unknown): value is Extract<OutboundMessage, { kind: "dispose" }> => value !== null && typeof value === "object" && Reflect.get(value, "kind") === "dispose";
        const messages = original.sent.filter(isDisposal);
        expect(messages.length, row.route).toBe(row.expected.originalPosts);
        expect(other.sent.filter(isDisposal).length, row.route).toBe(row.expected.otherPosts);
        const routeReleased = client.shardIndexFor(rows.actorId) === undefined;
        const fresh = await activate(client, workers[0]!, rows.actorId);
        const before = workers.reduce((count, worker) => count + worker.sent.length, 0);
        owner.dispose();
        let replacementActive = true;
        try { fresh.assertActive(); } catch { replacementActive = false; }
        const actual = { originalPosts: messages.length, otherPosts: other.sent.filter(isDisposal).length, sameGeneration: messages[0]?.activationGeneration === operation.activationGeneration, routeReleased, oldLeasePosts: workers.reduce((count, worker) => count + worker.sent.length, 0) - before, replacementActive };
        expect(actual, JSON.stringify(row)).toEqual(row.expected);
        expect(oracle.validate({ const: row.expected }, actual)).toBe(true);
        client.dispose(rows.actorId);
      }
    });

    it("refuses captured disposal before exact close and never addresses a replacement worker", async () => {
      const { client, workers } = harness(1);
      await activate(client, workers[0]!, "captured-dispose");
      const owner = await captureFixtureInstance(client, workers[0]!, "captured-dispose");
      expect(() => owner.dispose()).toThrow("actor-close.native-retirement-pending");
      await retireFixtureInstance(workers[0]!, owner);
      client.terminate(0);
      client.rebuild(0);
      const fresh = await activate(client, workers[1]!, "captured-dispose");
      const before = workers[1]!.sent.length;
      expect(() => owner.dispose()).toThrow("actor-close.worker-lost");
      expect(workers[1]!.sent.length).toBe(before);
      expect(() => fresh.assertActive()).not.toThrow();
      client.dispose("captured-dispose");
    });

    it("refuses a reply from a foreign or replaced worker incarnation", async () => {
      const { actorId } = await fixture();
      const { client, workers } = harness(2, { exclusiveShardCount: 1 });
      await activate(client, workers[0]!, actorId);
      client.terminate(0);
      client.rebuild(0);
      const lease = await activate(client, workers[2]!, actorId);
      let settled = false;
      const turning = lease.turn([], BUDGET).then((result) => { settled = true; return result; });
      const message = workers[2]!.sent.at(-1) as { requestId: string };
      for (const foreign of [workers[0]!, workers[1]!]) foreign.deliver({ kind: "result", requestId: message.requestId, ok: true, value: "foreign" });
      await Promise.resolve();
      await Promise.resolve();
      expect(settled).toBe(false);
      workers[2]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: "owned" });
      await expect(turning).resolves.toBe("owned");
      client.dispose(actorId);
    });

    it("rejects a settled result after close begins without acknowledging or releasing the close root", async () => {
      const { actorId, instanceId } = await fixture();
      const { client, workers } = harness(1);
      const worker = workers[0]!;
      const lease = await activate(client, worker, actorId);
      const close = await captureFixtureInstance(client, worker, actorId, instanceId);
      const turning = lease.turn([{ kind: "completed", payload: { req: 41n } }], BUDGET);
      const observed = expect(turning).rejects.toThrow("actor-activation.revoked");
      const turn = worker.sent.at(-1) as { requestId: string };
      const before = worker.sent.length;
      close.beginClose();
      worker.deliver({ kind: "result", requestId: turn.requestId, ok: true, value: { uiPatches: ["retained"] } });
      await observed;
      expect(worker.sent).toHaveLength(before);
      expect(close.progress().kind).toBe("closing");
      expect(() => client.dispose(actorId)).toThrow("actor-close.native-retirement-pending");
      await retireFixtureInstance(worker, close);
      client.dispose(actorId);
    });
  });
  //#endregion 🪪️ActivationLease

  describe("ShardClient exact instance close transport", () => {
    it("retains and retries the same close authority after transport refusal", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../🚪️lifetime/🧪️fixture/🔣️.json", source.url), "utf8"));
      for (const failure of fixture.leaseFailures) {
        const { client, workers } = harness(1);
        const worker = workers[0]!;
        const activation = client.activate("retry", "https://x/retry.js", [], BUDGET);
        worker.deliver({ kind: "result", requestId: (worker.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
        await activation;
        const lease = await captureFixtureInstance(client, worker, "retry");
        const request = lease.beginClose();
        const send = worker.postMessage.bind(worker);
        let original: unknown;
        worker.postMessage = (message) => {
          original = message;
          if (failure === "postMessage-throw") throw new Error("fixture transport refusal");
          send(message);
        };
        const pending = lease.close(BUDGET);
        const observed = expect(pending).rejects.toThrow("fixture");
        if (failure === "worker-error") worker.deliver({ kind: "result", requestId: (original as { requestId: string }).requestId, ok: false, error: "fixture worker refusal" });
        await observed;
        expect(lease.progress()).toEqual({ kind: "blocked", failure: failure === "postMessage-throw" ? "transport-refused" : "worker-refused" });
        expect(() => client.dispose("retry")).toThrow("actor-close.native-retirement-pending");
        worker.postMessage = send;
        const retried = lease.close(BUDGET);
        const message = worker.sent.at(-1) as { events: ShardEventEnvelope[] };
        expect(message.events[0]!.payload).toBe(request);
        expect(message.events).toEqual((original as { events: ShardEventEnvelope[] }).events);
        await answerLifecycle(worker, retried);
        await retireFixtureInstance(worker, lease);
        expect(lease.progress()).toEqual({ kind: "complete", failure: null });
        client.dispose("retry");
      }
    });

    it("waits for the captured worker's exact accepted and retired receipts", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../🚪️lifetime/🧪️fixture/🔣️.json", source.url), "utf8"));
      for (const row of fixture.leaseReceipts) {
        const { client, workers } = harness(2);
        const worker = workers[0]!;
        const activation = client.activate("close-owner", "https://x/close.js", [], BUDGET);
        worker.deliver({ kind: "result", requestId: (worker.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
        await activation;
        const lease = await captureFixtureInstance(client, worker, "close-owner");
        const request = lease.beginClose();
        await answerLifecycle(worker, lease.close(BUDGET));
        const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: lease.lifetime!, requestSequence: request.requestSequence, closeGeneration: 17n };
        for (const event of row.events) {
          const receipt: ActorInstanceLifecycleReceipt = { ...accepted, kind: event.endsWith("retired") ? "retired" : "accepted", ...(event === "old-activation-accepted" ? { lifetime: { ...lease.lifetime!, activationGeneration: lease.lifetime!.activationGeneration + 1n } } : {}), ...(event === "wrong-request-accepted" ? { requestSequence: request.requestSequence + 1 } : {}), ...(event === "wrong-generation-retired" ? { closeGeneration: 16n } : {}) };
          const pending = lease.poll(BUDGET);
          if (event === "foreign-worker-accepted") {
            const requestId = (worker.sent.at(-1) as { requestId: string }).requestId;
            workers[1]!.deliver({ kind: "result", requestId, ok: true, value: { status: { tag: "idle" }, lifecycleReceipt: encodeActorInstanceLifecycle(receipt) } });
            await answerLifecycle(worker, pending);
          } else {
            const delivered = answerLifecycle(worker, pending, receipt);
            if (event.startsWith("wrong-") || event === "old-activation-accepted" || receipt.kind === "retired" && lease.progress().kind !== "accepted") await expect(delivered).rejects.toThrow("actor-lifecycle.receipt-mismatch");
            else await delivered;
          }
          if (lease.pendingReceipt?.kind === "accepted") await answerLifecycle(worker, lease.acknowledge(lease.pendingReceipt, BUDGET));
        }
        expect(lease.pendingReceipt?.kind === "retired", row.name).toBe(row.settled);
        expect(client.shardIndexFor("close-owner")).toBe(0);
        if (lease.pendingReceipt?.kind !== "retired") {
          await answerLifecycle(worker, lease.poll(BUDGET), accepted);
          await answerLifecycle(worker, lease.acknowledge(accepted, BUDGET), { ...accepted, kind: "retired" });
        }
        await answerLifecycle(worker, lease.acknowledge(lease.pendingReceipt!, BUDGET, fixtureRetirement(lease)));
        expect(lease.progress().kind).toBe("complete");
        client.dispose("close-owner");
      }
    });

    it("does not revive a completed owner when the numeric ID or actor activation is reused", async () => {
      const { client, workers } = harness(1);
      const worker = workers[0]!;
      const activate = async () => {
        const pending = client.activate("reused", "https://x/close.js", [], BUDGET);
        worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: undefined });
        await pending;
      };
      await activate();
      const old = await captureFixtureInstance(client, worker, "reused");
      await retireFixtureInstance(worker, old);
      const sameActivation = await captureFixtureInstance(client, worker, "reused", 7, 14n);
      expect(sameActivation.lifetime!.activationGeneration).toBe(old.lifetime!.activationGeneration);
      expect(sameActivation.lifetime!.guestLifetime).not.toBe(old.lifetime!.guestLifetime);
      expect(() => old.activation.assertActive()).toThrow("actor-activation.revoked");
      const before = worker.sent.length;
      await expect(old.poll(BUDGET)).rejects.toThrow("actor-lifecycle.already-complete");
      expect(worker.sent).toHaveLength(before);
      await retireFixtureInstance(worker, sameActivation);
      client.dispose("reused");
      await activate();
      const fresh = await captureFixtureInstance(client, worker, "reused");
      expect(fresh.lifetime!.activationGeneration).toBe(old.lifetime!.activationGeneration + 1n);
      await retireFixtureInstance(worker, fresh);
      client.dispose("reused");
    });
  });

  describe("ShardClient activation + turn round-trip", () => {
    it("routes activate then turn to the same shard, resolving the reply by requestId", async () => {
      const { client, workers } = harness(2);
      const activatePromise = client.activate("actor-1", "https://x/plugin.js", [], BUDGET);
      const activateMsg = workers[0]!.sent[0] as { readonly kind: string; readonly requestId: string };
      expect(activateMsg.kind).toBe("activate");
      workers[0]!.deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
      await activatePromise;

      const turnPromise = client.turn("actor-1", [{ kind: "wake", payload: {} }], BUDGET);
      const turnMsg = workers[0]!.sent[1] as { readonly kind: string; readonly requestId: string };
      expect(turnMsg.kind).toBe("turn");
      workers[0]!.deliver({ kind: "result", requestId: turnMsg.requestId, ok: true, value: { effects: [] } });
      await expect(turnPromise).resolves.toEqual({ effects: [] });
    });

    it("rejects turn() for an actor never activated", async () => {
      const { client } = harness(2);
      await expect(client.turn("ghost", [], BUDGET)).rejects.toThrow(/not activated/);
    });
  });

  describe("ShardClient segmented-download transport", () => {
    async function activatedHarness() {
      const state = harness(1);
      const activation = state.client.activate("actor-download", "https://x/plugin.js", [], BUDGET);
      const message = state.workers[0]!.sent[0] as { readonly requestId: string };
      state.workers[0]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
      await activation;
      return state;
    }

    it("preserves operation identity and last-Some then None ordering", async () => {
      const { client, workers } = await activatedHarness();
      for (const expected of [new Uint8Array([1, 2]), new Uint8Array([3]), undefined]) {
        const read = client.takeSegmentedDownloadChunk("actor-download", 17, 91n);
        const message = workers[0]!.sent.at(-1) as { readonly kind: string; readonly requestId: string; readonly instanceId: number; readonly operationId: bigint };
        expect(message).toMatchObject({ kind: "takeSegmentedDownloadChunk", instanceId: 17, operationId: 91n });
        workers[0]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: expected });
        await expect(read).resolves.toEqual(expected);
      }
    });

    it("propagates unknown-operation errors without manufacturing a terminal None", async () => {
      const { client, workers } = await activatedHarness();
      const read = client.takeSegmentedDownloadChunk("actor-download", 17, 404n);
      const message = workers[0]!.sent.at(-1) as { readonly requestId: string };
      workers[0]!.deliver({ kind: "result", requestId: message.requestId, ok: false, error: "interactive-job.unknown-segmented-download" });
      await expect(read).rejects.toThrow("interactive-job.unknown-segmented-download");
    });

    it("rejects oversized or empty response items", async () => {
      const { client, workers } = await activatedHarness();
      for (const invalid of [new Uint8Array(4_097), new Uint8Array(0)]) {
        const read = client.takeSegmentedDownloadChunk("actor-download", 17, 91n);
        const message = workers[0]!.sent.at(-1) as { readonly requestId: string };
        workers[0]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: invalid });
        await expect(read).rejects.toThrow("segmented-download-transport-limit");
      }
    });

    it("rejects an in-flight read when actor disposal cancels its transport ownership", async () => {
      const { client } = await activatedHarness();
      const read = client.takeSegmentedDownloadChunk("actor-download", 17, 91n);
      client.dispose("actor-download");
      await expect(read).rejects.toThrow("ShardClient actor disposed");
    });

    it("preserves the complete u64 operation authority through structured clone", async () => {
      const { client, workers } = await activatedHarness();
      for (const operationId of [(1n << 53n) + 1n, MAX_SEGMENTED_DOWNLOAD_OPERATION_ID]) {
        const read = client.takeSegmentedDownloadChunk("actor-download", 17, operationId);
        const message = structuredClone(workers[0]!.sent.at(-1)) as { readonly requestId: string; readonly operationId: bigint };
        expect(message.operationId).toBe(operationId);
        workers[0]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
        await expect(read).resolves.toBeUndefined();
      }
    });

    it("rejects zero and overflowing operation authorities", async () => {
      const { client } = await activatedHarness();
      await expect(client.takeSegmentedDownloadChunk("actor-download", 17, 0n)).rejects.toThrow("segmented-download-authority-invalid");
      await expect(client.takeSegmentedDownloadChunk("actor-download", 17, 1n << 64n)).rejects.toThrow("segmented-download-authority-invalid");
    });
  });

  describe("ShardClient actor-id multiplexing", () => {
    it("distinguishes two actors' replies on the same shard even when they resolve out of order", async () => {
      const { client, workers } = harness(1);
      const p1 = client.activate("a", "https://x/a.js", [], BUDGET);
      const p2 = client.activate("b", "https://x/b.js", [], BUDGET);
      const [msgA, msgB] = workers[0]!.sent as { readonly requestId: string }[];
      // out-of-order reply: b's activate lands before a's
      workers[0]!.deliver({ kind: "result", requestId: msgB!.requestId, ok: true, value: undefined });
      workers[0]!.deliver({ kind: "result", requestId: msgA!.requestId, ok: true, value: undefined });
      await expect(p1).resolves.toBeUndefined();
      await expect(p2).resolves.toBeUndefined();
      expect(client.shardIndexFor("a")).toBe(0);
      expect(client.shardIndexFor("b")).toBe(0);
    });

    it("round-robins fresh actors across shards, skipping the reserved exclusive tail", async () => {
      const { client } = harness(4, { exclusiveShardCount: 1 });
      const indices = ["a", "b", "c"].map((id) => {
        void client.activate(id, "https://x/y.js", [], BUDGET);
        return client.shardIndexFor(id);
      });
      expect(new Set(indices).has(3)).toBe(false); // shard 3 reserved exclusive
      expect(indices).toEqual([0, 1, 2]);
    });
  });

  describe("ShardClient heartbeat watchdog (postMessage path)", () => {
    it("does not miss while idle (no pending turn)", () => {
      const { client, advance } = harness(1);
      advance(100_000);
      client.checkHeartbeats(100_000);
      // no throw, no rebuild callback would have fired — nothing to assert but absence of an exception
      expect(true).toBe(true);
    });

    it("terminates + rebuilds after 3 consecutive missed-heartbeat windows on a stuck turn", async () => {
      const lost: Array<{ index: number; actorIds: readonly string[] }> = [];
      const { client, workers, advance, setNow } = harness(1, { heartbeatTimeoutMs: 1000, onShardLost: (index, actorIds) => lost.push({ index, actorIds }) });
      setNow(0);
      const activatePromise = client.activate("stuck", "https://x/stuck.js", [], BUDGET);
      const activateMsg = workers[0]!.sent[0] as { readonly requestId: string };
      workers[0]!.deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
      await activatePromise;

      const originalWorker = workers[0]!;
      client.turn("stuck", [], BUDGET).catch(() => {}); // never replies — simulates a hung guest; terminate() rejects it

      advance(1001);
      client.checkHeartbeats();
      expect(lost).toEqual([]);
      expect(originalWorker.terminated).toBe(false);

      advance(1001);
      client.checkHeartbeats();
      advance(1001);
      client.checkHeartbeats();

      expect(originalWorker.terminated).toBe(true);
      expect(lost).toEqual([{ index: 0, actorIds: ["stuck"] }]);
      expect(client.shardIndexFor("stuck")).toBeUndefined(); // rebuild cleared routing; caller must re-activate
    });

    it("a fresh heartbeat resets the miss count", async () => {
      const { client, workers, advance, setNow } = harness(1, { heartbeatTimeoutMs: 1000 });
      setNow(0);
      const activatePromise = client.activate("busy", "https://x/busy.js", [], BUDGET);
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;
      void client.turn("busy", [], BUDGET);

      advance(1001);
      client.checkHeartbeats();
      advance(1001);
      client.checkHeartbeats();
      // guest emits a heartbeat right before the 3rd window would elapse
      workers[0]!.deliver({ kind: "heartbeat", turnSeq: 7 });
      advance(1001);
      client.checkHeartbeats();
      expect(workers[0]!.terminated).toBe(false);
    });
  });

  describe("ShardClient liveness watchdog (schema-owned timelines)", () => {
    type LivenessScenario = {
      readonly id: string;
      readonly heartbeatTimeoutMs: number;
      readonly timeline: readonly { readonly atMs: number; readonly event: string }[];
      readonly expected: { readonly missesAtCheck: readonly number[]; readonly terminatedAtMs: number | null };
    };
    type LivenessFixture = {
      readonly policy: Readonly<Record<string, number>>;
      readonly worker: { readonly activationPhases: readonly string[]; readonly progressPhase: string };
      readonly jspi: { readonly faultCode: string; readonly vectors: readonly { readonly id: string; readonly webAssembly: boolean; readonly suspending: boolean; readonly promising: boolean; readonly available: boolean }[] };
      readonly scenarios: readonly LivenessScenario[];
    };

    async function livenessFixture(): Promise<LivenessFixture> {
      const { default: fixture } = await import("../../🧪️fixture/🔣️.json");
      const { default: schema } = await import("../../🧬️schema/🔣️.json");
      const { default: Ajv } = await import("ajv");
      const validate = new Ajv({ strict: true }).compile(schema);
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      return fixture as unknown as LivenessFixture;
    }

    /** 🔮️ Independent oracle — the watchdog rule re-derived from its prose statement ("a busy shard is
     * one that has said SOMETHING since `heartbeatTimeoutMs` ago; every further silent window costs one
     * miss; `missedLimit` in a row kills it") as a flat simulation over the timeline, sharing no code
     * with {@link evaluateShardLiveness} or `ShardClient`. Agreement between this, the fixture's own
     * recorded expectations and the real client is what makes the rule trustworthy. */
    function simulateLiveness(scenario: LivenessScenario, missedLimit: number): { readonly missesAtCheck: number[]; readonly terminatedAtMs: number | null } {
      let pendingSince: number | null = null;
      let livenessAtMs = 0;
      let missed = 0;
      let lastMissAtMs = 0;
      const missesAtCheck: number[] = [];
      let terminatedAtMs: number | null = null;
      for (const step of scenario.timeline) {
        if (terminatedAtMs !== null) break;
        if (step.event === "pending") { pendingSince ??= step.atMs; continue; }
        if (step.event === "idle" || step.event === "liveness") {
          if (step.event === "idle") pendingSince = null;
          livenessAtMs = step.atMs;
          missed = 0;
          lastMissAtMs = step.atMs;
          continue;
        }
        const silent = pendingSince !== null && step.atMs - Math.max(livenessAtMs, pendingSince) > scenario.heartbeatTimeoutMs && step.atMs - lastMissAtMs >= scenario.heartbeatTimeoutMs;
        if (silent) { missed += 1; lastMissAtMs = step.atMs; }
        missesAtCheck.push(missed);
        if (missed >= missedLimit) terminatedAtMs = step.atMs;
      }
      return { missesAtCheck, terminatedAtMs };
    }

    it("mirrors the schema-owned policy record in every consumer of it", async () => {
      const fixture = await livenessFixture();
      expect({ ...SHARD_LIVENESS_POLICY }).toEqual(fixture.policy);
      const { SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS } = await import("../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts");
      expect(SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS).toBe(fixture.policy.progressIntervalMs);
    });

    it("serves the shard worker from the schema-owned distribution route, not a transliteration of it", async () => {
      const { SHARD_WORKER_URL } = await import("../../../🧵️shard-runtime/🟦️.ts");
      const { MODULE_PLUGIN_ROUTE, MODULE_SHARD_DIRECTORY } = await import("../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts");
      const { SHARD_WORKER_FILE } = await import("../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts");
      expect(SHARD_WORKER_URL).toBe(`${MODULE_PLUGIN_ROUTE}/${MODULE_SHARD_DIRECTORY}/${SHARD_WORKER_FILE}`);
      // 🩺️ Every browser-side spawner must use THAT url. A private ASCII transliteration
      // (`/plugin-modules/_shard/…`) is answered by Vite's SPA fallback with `text/html`, which a
      // module Worker rejects as a message-less `error` event — the 2026-09-05 four-dead-shard boot.
      const { readFileSync } = await import("node:fs");
      for (const spawner of ["../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx", "../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts"]) {
        const source = readFileSync(new URL(spawner, source.url), "utf8");
        expect(source, spawner).not.toContain("/plugin-modules/_shard/");
        if (source.includes("new Worker(")) expect(source, spawner).toContain("SHARD_WORKER_URL");
      }
    });

    it("classifies JSPI availability exactly as the fixture's vectors say", async () => {
      const fixture = await livenessFixture();
      expect(SHARD_JSPI_FAULT_CODE).toBe(fixture.jspi.faultCode);
      const observed = fixture.jspi.vectors.map((vector) => {
        const scope: ShardJspiScope = vector.webAssembly ? { WebAssembly: { Suspending: vector.suspending ? class {} : undefined, promising: vector.promising ? () => undefined : undefined } } : {};
        const available = shardJspiAvailable(scope);
        // 🔮️ Independent oracle: "JSPI is present when BOTH lifting primitives are callable", stated
        // directly against the vector rather than through the implementation under test.
        expect(available, vector.id).toBe(vector.webAssembly && vector.suspending && vector.promising);
        let thrown: unknown = null;
        try { assertShardJspiAvailable(scope); } catch (error) { thrown = error; }
        expect(thrown === null, vector.id).toBe(vector.available);
        if (thrown) {
          expect(thrown).toBeInstanceOf(ShardJspiUnavailableError);
          expect((thrown as ShardJspiUnavailableError).code).toBe(fixture.jspi.faultCode);
          expect(Object.keys((thrown as ShardJspiUnavailableError).text)).toEqual(["en", "de"]);
        }
        return { id: vector.id, available };
      });
      expect(observed).toEqual(fixture.jspi.vectors.map((vector) => ({ id: vector.id, available: vector.available })));
      console.log(`[DEBUG] ShardClient JSPI vectors=${observed.map((row) => `${row.id}:${row.available}`).join(",")}`);
    });

    it("names a worker onerror instead of logging a bare Event", async () => {
      expect(describeShardWorkerError({ type: "error", message: "Uncaught TypeError: WebAssembly.Suspending is not a constructor", filename: "http://localhost:6076/plugin-modules/_shard/🟨️shard-worker.js", lineno: 18, colno: 9 })).toBe(
        "Uncaught TypeError: WebAssembly.Suspending is not a constructor at http://localhost:6076/plugin-modules/_shard/🟨️shard-worker.js:18:9",
      );
      expect(describeShardWorkerError({ type: "error", message: "" })).toContain('redacted "error" event');
      expect(describeShardWorkerError({ error: { message: "boom" } })).toBe("boom");
      const { client, workers } = harness(1);
      void client.activate("crashy", "https://x/c.js", [], BUDGET).catch(() => {});
      workers[0]!.onerror?.({ type: "error", message: "top-level throw", filename: "worker.js", lineno: 18, colno: 9 });
      expect(isShardLostError(new Error("shard 0 worker crashed: top-level throw at worker.js:18:9"))).toBe(true);
      client.disposeAll();
    });

    it("agrees with the independent oracle and the fixture on every watchdog timeline", async () => {
      const fixture = await livenessFixture();
      const observed: string[] = [];
      for (const scenario of fixture.scenarios) {
        const lost: number[] = [];
        const { client, workers, setNow } = harness(1, { heartbeatTimeoutMs: scenario.heartbeatTimeoutMs, onShardLost: (index) => lost.push(index) });
        setNow(0);
        const activatePromise = client.activate("live", "https://x/live.js", [], BUDGET);
        const activateId = (workers[0]!.sent.find((message) => (message as { kind: string }).kind === "activate") as { requestId: string }).requestId;
        workers[0]!.deliver({ kind: "result", requestId: activateId, ok: true, value: undefined });
        await activatePromise;
        const missesAtCheck: number[] = [];
        const outstanding: string[] = [];
        let terminatedAtMs: number | null = null;
        let beat = 0;
        for (const step of scenario.timeline) {
          if (terminatedAtMs !== null) break;
          setNow(step.atMs);
          if (step.event === "pending") {
            void client.checkpoint("live").catch(() => {});
            outstanding.push((workers[0]!.sent.at(-1) as { requestId: string }).requestId);
            continue;
          }
          if (step.event === "idle") {
            for (const requestId of outstanding.splice(0)) workers[0]!.deliver({ kind: "result", requestId, ok: true, value: new Uint8Array() });
            continue;
          }
          if (step.event === "liveness") {
            beat += 1;
            workers[0]!.deliver({ kind: "heartbeat", turnSeq: beat, phase: fixture.worker.progressPhase });
            continue;
          }
          // 🔎️ Captured BEFORE the tick: a terminating check replaces `shards[0]` with a rebuilt slot
          // whose count is legitimately zero, and the count under test is the retired slot's.
          const slot = (Reflect.get(client, "shards") as readonly { readonly heartbeat: { readonly missedCount: number } }[])[0]!;
          client.checkHeartbeats();
          missesAtCheck.push(slot.heartbeat.missedCount);
          if (lost.length > 0) terminatedAtMs = step.atMs;
        }
        expect({ id: scenario.id, missesAtCheck, terminatedAtMs }).toEqual({ id: scenario.id, ...simulateLiveness(scenario, fixture.policy.missedLimit!) });
        expect(missesAtCheck, scenario.id).toEqual(scenario.expected.missesAtCheck);
        expect(terminatedAtMs, scenario.id).toBe(scenario.expected.terminatedAtMs);
        client.disposeAll();
        observed.push(scenario.id);
      }
      expect(observed).toEqual(fixture.scenarios.map((scenario) => scenario.id));
      console.log(`[DEBUG] ShardClient liveness timelines replayed against client+oracle=${observed.length}`);
    });

    it("beats at every generated-worker activation boundary and tickers on through a stalled import", async () => {
      const fixture = await livenessFixture();
      const { default: ts } = await import("typescript");
      const { readFileSync } = await import("node:fs");
      const vm = await import("node:vm");
      const path = new URL("../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts", source.url);
      const parsed = ts.createSourceFile(path.pathname, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
      const declaration = parsed.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "shardWorkerSource");
      const returned = declaration && ts.isFunctionDeclaration(declaration) ? declaration.body?.statements.find(ts.isReturnStatement)?.expression : null;
      if (!returned || !ts.isTemplateExpression(returned)) throw new Error("Changed generated shard worker source");
      const { shardWorkerSource } = await import("../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts");
      // 🧪️ `await import(url)` has no meaning inside a bare `vm` context; the ONE call is redirected to
      // an injected loader so the same generated bytes can be driven with that boundary held open.
      const source = shardWorkerSource().replace("await import(/* @vite-ignore */ moduleUrl)", "await __import(moduleUrl)");
      expect(source).not.toContain("@vite-ignore");
      const beats: (string | undefined)[] = [];
      const results: string[] = [];
      const intervals: { readonly delayMs: number; readonly tick: () => void }[] = [];
      const cleared: number[] = [];
      let bridge: (value: { createActorApi: (actorId: string, generation: bigint) => Promise<unknown> }) => void = () => {};
      let api: (value: unknown) => void = () => {};
      const bridgeReady = new Promise<{ createActorApi: (actorId: string, generation: bigint) => Promise<unknown> }>((resolve) => { bridge = resolve; });
      const apiReady = new Promise<unknown>((resolve) => { api = resolve; });
      let dispatch: ((event: { data: Record<string, unknown> }) => Promise<void>) | null = null;
      const context = vm.createContext({
        WebAssembly: { Suspending: class {}, promising: (value: unknown) => value },
        __import: () => bridgeReady,
        setInterval: (tick: () => void, delayMs: number) => intervals.push({ delayMs, tick }),
        clearInterval: (handle: number) => cleared.push(handle),
        self: {
          postMessage: (message: { kind: string; phase?: string; requestId?: string }) => {
            if (message.kind === "heartbeat") beats.push(message.phase);
            if (message.kind === "result") results.push(message.requestId ?? "");
          },
          addEventListener: (_kind: string, handler: typeof dispatch) => { dispatch = handler; },
        },
      });
      new vm.Script(source).runInContext(context);
      if (!dispatch) throw new Error("Missing generated worker dispatcher");
      const send = dispatch as (event: { data: Record<string, unknown> }) => Promise<void>;
      const activating = send({ data: { kind: "activate", requestId: "a1", actorId: "a", activationGeneration: 1n, moduleUrl: "https://fixture.invalid/a.js", assets: [] } });
      await Promise.resolve();
      expect(beats).toEqual([undefined, fixture.worker.activationPhases[0]]);
      expect(intervals).toHaveLength(1);
      expect(intervals[0]!.delayMs).toBe(fixture.policy.progressIntervalMs);
      intervals[0]!.tick();
      intervals[0]!.tick();
      expect(results).toEqual([]);
      bridge({ createActorApi: () => apiReady as Promise<unknown> });
      await Promise.resolve();
      await Promise.resolve();
      api({ poll: async () => undefined });
      await activating;
      expect(beats).toEqual([undefined, "module-fetch", fixture.worker.progressPhase, fixture.worker.progressPhase, "module-ready", "actor-ready"]);
      expect(beats.slice(4)).toEqual(fixture.worker.activationPhases.slice(1));
      expect(results).toEqual(["a1"]);
      expect(cleared).toHaveLength(1);
      console.log(`[DEBUG] ShardWorkerLiveness activation beats=${beats.map((phase) => phase ?? "request").join(",")}`);
    });
  });

  describe("ShardClient SAB heartbeat path", () => {
    it("pollHeartbeatSab reads Atomics-stored turnSeq and feeds the same miss-count state machine", async () => {
      const sab = new SharedArrayBuffer(4 * Int32Array.BYTES_PER_ELEMENT);
      const view = new Int32Array(sab);
      const { client, workers, advance, setNow } = harness(1, { heartbeatSab: sab, heartbeatTimeoutMs: 1000 });
      setNow(0);
      const activatePromise = client.activate("sab-actor", "https://x/s.js", [], BUDGET);
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent.find((m) => (m as { kind: string }).kind === "activate") as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;
      void client.turn("sab-actor", [], BUDGET);

      advance(1001);
      client.checkHeartbeats();
      expect(workers[0]!.terminated).toBe(false);

      Atomics.store(view, 0, 5);
      client.pollHeartbeatSab();
      advance(1001);
      client.checkHeartbeats();
      expect(workers[0]!.terminated).toBe(false); // heartbeat seen via SAB reset the window
    });
  });

  describe("ShardClient leaseExclusive", () => {
    it("moves an actor onto a reserved exclusive shard and back", async () => {
      const { client } = harness(4, { exclusiveShardCount: 2 });
      void client.activate("heavy", "https://x/h.js", [], BUDGET);
      expect(client.shardIndexFor("heavy")).toBe(0);
      const exclusiveIndex = client.leaseExclusive("heavy");
      expect([2, 3]).toContain(exclusiveIndex);
      expect(client.shardIndexFor("heavy")).toBe(exclusiveIndex);
      client.releaseExclusive("heavy");
      expect(client.shardIndexFor("heavy")).toBeUndefined();
    });

    it("throws once every exclusive shard is leased and force is not set", () => {
      const { client } = harness(3, { exclusiveShardCount: 1 });
      void client.activate("first", "https://x/1.js", [], BUDGET);
      client.leaseExclusive("first");
      void client.activate("second", "https://x/2.js", [], BUDGET);
      expect(() => client.leaseExclusive("second")).toThrow(/no free exclusive shard/);
    });

    it("is idempotent for the same actor already leased", () => {
      const { client } = harness(2, { exclusiveShardCount: 1 });
      void client.activate("a", "https://x/a.js", [], BUDGET);
      const first = client.leaseExclusive("a");
      const second = client.leaseExclusive("a");
      expect(first).toBe(second);
    });
  });

  describe("ShardClient.startWatchdog / stopWatchdog", () => {
    it("self-ticks checkHeartbeats + pollHeartbeatSab with no external caller, detects a missed heartbeat, and rebuilds", async () => {
      vi.useFakeTimers();
      try {
        const lost: Array<{ index: number; actorIds: readonly string[] }> = [];
        const workers: FakeShardWorker[] = [];
        const client = new ShardClient({
          residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }),
          shardCount: 1,
          createWorker: (index) => {
            const worker = new FakeShardWorker(index);
            workers.push(worker);
            return worker;
          },
          heartbeatTimeoutMs: 1000,
          onShardLost: (index, actorIds) => lost.push({ index, actorIds }),
        });

        const activatePromise = client.activate("stuck", "https://x/stuck.js", [], BUDGET);
        const activateMsg = workers[0]!.sent[0] as { readonly requestId: string };
        workers[0]!.deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
        await activatePromise;

        client.turn("stuck", [], BUDGET).catch(() => {}); // never replies — nothing external ticks the watchdog for it
        client.startWatchdog(1001); // same cadence the pre-existing manual test used, driven this time by the self-tick

        vi.advanceTimersByTime(1001); // window 1
        expect(lost).toEqual([]);
        vi.advanceTimersByTime(1001); // window 2
        expect(lost).toEqual([]);
        vi.advanceTimersByTime(1001); // window 3 — self-ticked, no manual checkHeartbeats() call anywhere in this test
        expect(workers[0]!.terminated).toBe(true);
        expect(lost).toEqual([{ index: 0, actorIds: ["stuck"] }]);
        expect(client.shardIndexFor("stuck")).toBeUndefined();

        client.stopWatchdog();
        const lostCountAfterStop = lost.length;
        vi.advanceTimersByTime(10_000);
        expect(lost.length).toBe(lostCountAfterStop); // stopped — no further ticks fire
      } finally {
        vi.useRealTimers();
      }
    });

    it("is idempotent to call twice, and stopWatchdog before ever starting is a no-op", () => {
      vi.useFakeTimers();
      try {
        const { client } = harness(1);
        client.startWatchdog(500);
        client.startWatchdog(500); // second call while running: no-op, does not leak a second interval
        client.stopWatchdog();
        client.stopWatchdog(); // already stopped: no-op, does not throw
        expect(true).toBe(true);
      } finally {
        vi.useRealTimers();
      }
    });
  });

  describe("ShardClient failShard clears routing", () => {
    it("clears actorShard + slot.actorIds immediately on a worker crash (onerror), before any terminate()/rebuild()", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("x", "https://x/x.js", [], BUDGET);
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;
      expect(client.shardIndexFor("x")).toBe(0);

      workers[0]!.onerror?.(new Error("boom")); // failShard runs; onerror does NOT call terminate()/rebuild()

      // routing cleared right away — a dead shard must stop receiving newly routed work immediately,
      // not only once a later rebuild() happens to run.
      expect(client.shardIndexFor("x")).toBeUndefined();
      await expect(client.turn("x", [], BUDGET)).rejects.toThrow(/not activated/);
    });
  });

  describe("ShardClient terminate/rebuild", () => {
    it("rejects in-flight requests on terminate and spawns a fresh worker on rebuild", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("x", "https://x/x.js", [], BUDGET);
      const rejection = expect(activatePromise).rejects.toThrow(/terminated/);
      const oldWorker = workers[0]!;
      const actorIds = client.terminate(0);
      expect(actorIds).toEqual(["x"]);
      await rejection;
      expect(oldWorker.terminated).toBe(true);
      client.rebuild(0);
      expect(workers.length).toBe(2);
      expect(client.shardIndexFor("x")).toBeUndefined();
    });
  });

  describe("ShardClient worker crash", () => {
    it("onerror fails every pending request on that shard", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("crashy", "https://x/c.js", [], BUDGET);
      workers[0]!.onerror?.(new Error("boom"));
      await expect(activatePromise).rejects.toThrow(/crashed/);
    });
  });

  describe("ShardClient.shardMetricsSamples", () => {
    it("reports zero actors/busyRatio and an infinite heartbeat age for a fresh, never-touched shard", () => {
      const { client } = harness(2);
      const samples = client.shardMetricsSamples(1_000);
      expect(samples).toHaveLength(2);
      for (const sample of samples) {
        expect(sample.metrics.actors).toBe(0);
        expect(sample.metrics.busyRatio).toBe(0);
        expect(sample.metrics.heartbeatAgeMs).toBe(Number.POSITIVE_INFINITY);
      }
    });

    it("counts resident actors and in-flight turns as busyRatio, and ages the heartbeat off the injected clock", async () => {
      const { client, workers, setNow } = harness(1);
      setNow(0);
      const activateA = client.activate("a", "https://x/a.js", [], BUDGET);
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activateA;
      const activateB = client.activate("b", "https://x/b.js", [], BUDGET); // second actor, same (only) shard
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[1] as { requestId: string }).requestId, ok: true, value: undefined });
      await activateB; // both activations settled — only the turn below should count toward busyRatio

      workers[0]!.deliver({ kind: "heartbeat", turnSeq: 1 });
      void client.turn("a", [], BUDGET); // one in-flight turn out of two resident actors, deliberately never replied

      setNow(300);
      const [sample] = client.shardMetricsSamples(300);
      expect(sample!.metrics.actors).toBe(2);
      expect(sample!.metrics.busyRatio).toBeCloseTo(0.5);
      expect(sample!.metrics.heartbeatAgeMs).toBe(300);
    });
  });

  //#region 📨️ShardFrame tests
  function makeEnvelope(to: string, lane: Lane, seq: number, kind = "wake"): ShardEnvelope {
    return { to, from: { kind: "kernel" }, lane, seq, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind, payload: {} } };
  }

  describe("orderEnvelopesByLane", () => {
    it("sorts by Lane priority, not arrival order, stable within a tied lane", () => {
      const envelopes = [makeEnvelope("a", "Background", 1), makeEnvelope("a", "Interactive", 2), makeEnvelope("a", "Maintenance", 3), makeEnvelope("a", "Interactive", 4)];
      const ordered = orderEnvelopesByLane(envelopes);
      expect(ordered.map((envelope) => envelope.seq)).toEqual([2, 4, 1, 3]);
    });

    it("is a no-op for an already-lane-sorted, single-lane batch", () => {
      const envelopes = [makeEnvelope("a", "Interactive", 1), makeEnvelope("a", "Interactive", 2)];
      expect(orderEnvelopesByLane(envelopes).map((envelope) => envelope.seq)).toEqual([1, 2]);
    });
  });

  describe("GrantedBudgetTracker + interpretShardFrame", () => {
    it("a Grant records its budget and hands back envelopes in lane-priority order", () => {
      const tracker = createGrantedBudgetTracker();
      const envelopes = [makeEnvelope("a", "Background", 1), makeEnvelope("a", "Interactive", 2)];
      const grantBudget: ShardBudget = { ...BUDGET, fuel: 999 };
      const result = interpretShardFrame({ kind: "Grant", actor: "a", budget: grantBudget, envelopes }, tracker);
      expect(result).toEqual({ action: "runEnvelopes", actor: "a", budget: grantBudget, envelopes: [envelopes[1], envelopes[0]] });
      expect(tracker.granted("a")).toBe(grantBudget);
    });

    it("an Envelope with no prior Grant runs under the Maintenance-lane default, never an invented constant", () => {
      const tracker = createGrantedBudgetTracker();
      const lonelyEnvelope = makeEnvelope("never-granted", "Interactive", 1);
      const result = interpretShardFrame({ kind: "Envelope", envelope: lonelyEnvelope }, tracker);
      expect(result).toEqual({ action: "runEnvelopes", actor: "never-granted", budget: MAINTENANCE_LANE_DEFAULT_BUDGET, envelopes: [lonelyEnvelope] });
    });

    it("an Envelope AFTER a Grant for the same actor runs under THAT granted budget — proving the old constant no longer influences it", () => {
      const tracker = createGrantedBudgetTracker();
      const grantBudget: ShardBudget = { ...BUDGET, fuel: 42 };
      interpretShardFrame({ kind: "Grant", actor: "a", budget: grantBudget, envelopes: [] }, tracker);
      const followUp = makeEnvelope("a", "Interactive", 5);
      const result = interpretShardFrame({ kind: "Envelope", envelope: followUp }, tracker);
      expect(result.action).toBe("runEnvelopes");
      expect((result as { readonly budget: ShardBudget }).budget).toBe(grantBudget);
      expect((result as { readonly budget: ShardBudget }).budget).not.toBe(MAINTENANCE_LANE_DEFAULT_BUDGET);
    });

    it("Register/Unregister are pure bookkeeping; Unregister forgets a previously granted budget", () => {
      const tracker = createGrantedBudgetTracker();
      interpretShardFrame({ kind: "Grant", actor: "a", budget: BUDGET, envelopes: [] }, tracker);
      expect(tracker.granted("a")).toBe(BUDGET);
      expect(interpretShardFrame({ kind: "Register", actor: "a" }, tracker)).toEqual({ action: "register", actor: "a" });
      expect(interpretShardFrame({ kind: "Unregister", actor: "a" }, tracker)).toEqual({ action: "unregister", actor: "a" });
      expect(tracker.granted("a")).toEqual(MAINTENANCE_LANE_DEFAULT_BUDGET); // forgotten — back to the floor, not a stale grant
    });

    it("an unknown/future frame variant resolves to 'unknown' instead of throwing (forward-compat)", () => {
      const tracker = createGrantedBudgetTracker();
      const futureFrame = { kind: "Checkpoint", actor: "a" } as unknown as ShardFrame;
      expect(() => interpretShardFrame(futureFrame, tracker)).not.toThrow();
      expect(interpretShardFrame(futureFrame, tracker)).toEqual({ action: "unknown", frame: futureFrame });
    });
  });

  describe("ShardClient.grant / ShardClient.envelope wire adoption", () => {
    it("grant() sends a ShardFrame::Grant frame with envelopes pre-sorted by lane, budget carried alongside them", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("a", "https://x/a.js", [], BUDGET);
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;

      const grantBudget: ShardBudget = { ...BUDGET, fuel: 12345 };
      const envelopes = [makeEnvelope("a", "Background", 1), makeEnvelope("a", "Interactive", 2)];
      void client.grant("a", grantBudget, envelopes);

      const sent = workers[0]!.sent[1] as { readonly kind: string; readonly frame: ShardFrame };
      expect(sent.kind).toBe("frame");
      expect(sent.frame.kind).toBe("Grant");
      const grantFrame = sent.frame as { readonly kind: "Grant"; readonly actor: string; readonly budget: ShardBudget; readonly envelopes: readonly ShardEnvelope[] };
      expect(grantFrame.budget).toBe(grantBudget);
      expect(grantFrame.envelopes.map((envelope) => envelope.seq)).toEqual([2, 1]); // Interactive dispatched before Background
    });

    it("envelope() sends a ShardFrame::Envelope frame with NO budget field on the wire at all", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("a", "https://x/a.js", [], BUDGET);
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;

      void client.envelope(makeEnvelope("a", "Interactive", 1));
      const sent = workers[0]!.sent[1] as { readonly kind: string; readonly frame: ShardFrame };
      expect(sent.kind).toBe("frame");
      expect(sent.frame.kind).toBe("Envelope");
      expect(Object.keys(sent.frame)).toEqual(["kind", "envelope"]); // structurally budget-less on the wire
    });

    it("turn()/activate() keep working completely unchanged alongside the new frame wire (incremental adoption really is incremental)", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("legacy", "https://x/legacy.js", [], BUDGET);
      const activateMsg = workers[0]!.sent[0] as { readonly kind: string; readonly requestId: string };
      expect(activateMsg.kind).toBe("activate");
      workers[0]!.deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
      await activatePromise;

      const turnPromise = client.turn("legacy", [{ kind: "wake", payload: {} }], BUDGET);
      const turnMsg = workers[0]!.sent[1] as { readonly kind: string; readonly requestId: string };
      expect(turnMsg.kind).toBe("turn");
      workers[0]!.deliver({ kind: "result", requestId: turnMsg.requestId, ok: true, value: { effects: [] } });
      await expect(turnPromise).resolves.toEqual({ effects: [] });
    });
  });

  describe("fixed command ingress pages", () => {
    it("matches a DataView little-endian oracle across a full page boundary", () => {
      const command = Uint8Array.from({ length: ACTOR_BYTE_PAGE_BYTES + 5 }, (_, index) => index & 0xff);
      const pages = createShardCommandIngressPages({ owner: 7n, generation: 11n, commandIndex: 1, commandCount: 3, instance: 13, seq: 17n, command });
      expect(pages).toHaveLength(2);
      expect(pages.map((page) => page.page.length)).toEqual([ACTOR_BYTE_PAGE_BYTES, 5]);
      expect(pages.map((page) => Object.keys(page))).toEqual([["cursor", "page"], ["cursor", "page"]]);
      expect(pages[0]!.cursor).toMatchObject({ owner: 7n, generation: 11n, commandIndex: 1, commandCount: 3, instance: 13, seq: 17n, kind: 0, pageIndex: 0, pageCount: 2 });
      const oracle = new DataView(command.buffer, command.byteOffset, command.byteLength);
      expect(pages[0]!.page.block00.word0).toBe(oracle.getBigUint64(0, true));
      expect(pages[0]!.page.block63.word7).toBe(oracle.getBigUint64(ACTOR_BYTE_PAGE_BYTES - 8, true));
      expect(pages[1]!.page.block00.word0).toBe(0x0000000403020100n);
      expect(pages[1]!.page.block00.word1).toBe(0n);
      expect(pages[1]!.page.block63.word7).toBe(0n);
    });

    it("forwards the fixed page as the dedicated turn argument", async () => {
      const { client, workers } = harness(1);
      await activateActor(client, workers, "paged");
      const page = createShardCommandIngressPages({ owner: 1n, generation: 1n, commandIndex: 0, commandCount: 1, instance: 1, seq: 1n, command: Uint8Array.of(9, 8, 7) })[0]!;
      void client.turn("paged", [], BUDGET, page);
      expect(workers[0]!.sent.at(-1)).toMatchObject({ kind: "turn", actorId: "paged", events: [], commandPage: page });
      expect(Object.keys(page)).toEqual(["cursor", "page"]);
    });
  });

  describe("ShardFrame parity with Rust component.rs", () => {
    it("TS ShardFrame variant/field names match the live Rust enum in 🖥️host/🧵️shard/🦀️.rs", async () => {
      const { readFileSync } = await import("node:fs");
      const rustUrl = new URL("../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs", source.url);
      const source = readFileSync(rustUrl, "utf8");
      const enumMatch = source.match(/pub enum ShardFrame \{([\s\S]*?)\n\}\s*\n\s*impl ShardFrame/);
      expect(enumMatch).not.toBeNull(); // [DEBUG] `pub enum ShardFrame { ... } impl ShardFrame` shape not found — Rust source changed, update this test's regex
      const body = enumMatch![1]!.replace(/\/\/\/.*$/gm, "").replace(/\/\/.*$/gm, "");
      const variantPattern = /(\w+)\s*(?:\{([^{}]*)\}|\(([^()]*)\))?\s*,/g;
      const rustVariants: Array<{ readonly name: string; readonly fields: readonly string[] | null }> = [];
      let match: RegExpExecArray | null;
      while ((match = variantPattern.exec(body)) !== null) {
        const [, name, structFields, tupleType] = match;
        if (structFields !== undefined) {
          const fields = structFields
            .split(",")
            .map((part) => part.trim())
            .filter((part) => part.length > 0)
            .map((part) => part.split(":")[0]!.trim());
          rustVariants.push({ name: name!, fields });
        } else if (tupleType !== undefined) {
          rustVariants.push({ name: name!, fields: null });
        }
      }

      // Same variant NAMES, in the same order, as this file's own runtime twin.
      expect(rustVariants.map((variant) => variant.name)).toEqual(SHARD_FRAME_VARIANT_FIELDS.map((variant) => variant.kind));
      for (const rustVariant of rustVariants) {
        if (rustVariant.fields === null) continue; // tuple variant (`Envelope(Envelope)`) — Rust has no field name to diff against; see SHARD_FRAME_VARIANT_FIELDS's own doc
        const tsVariant = SHARD_FRAME_VARIANT_FIELDS.find((variant) => variant.kind === rustVariant.name)!;
        expect(tsVariant.fields).toEqual(rustVariant.fields);
      }
    });
  });
  //#endregion 📨️ShardFrame tests

  //#region 🌉️HostEffectBridge tests
  function makeEffectRequestFrame(actorId: string, effect: string, requestId: string, params: unknown, activationGeneration = 1n): InboundMessage {
    return { kind: "frame", actorId, activationGeneration, frame: { kind: "Envelope", envelope: { to: "kernel", from: { kind: "actor", id: actorId }, lane: "Background", seq: 1, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind: "effect-request", payload: { effect, requestId, params } } } } };
  }

  type EffectReplyPayload = { readonly requestId: string; readonly value?: unknown; readonly message?: string };
  type EffectReplyMessage = { readonly kind: "frame"; readonly frame: { readonly kind: "Envelope"; readonly envelope: { readonly payload: { readonly kind: "effect-complete" | "effect-error"; readonly payload: EffectReplyPayload } } } };

  function findEffectReply(sent: readonly unknown[], requestId: string, kind: "effect-complete" | "effect-error"): EffectReplyMessage | undefined {
    return (sent as readonly { readonly kind: string; readonly frame?: { readonly kind: string; readonly envelope?: { readonly payload?: { readonly kind?: string; readonly payload?: { readonly requestId?: string } } } } }[]).find(
      (message) => message.kind === "frame" && message.frame?.kind === "Envelope" && message.frame.envelope?.payload?.kind === kind && message.frame.envelope.payload?.payload?.requestId === requestId,
    ) as EffectReplyMessage | undefined;
  }

  async function activateActor(client: ShardClient, workers: readonly FakeShardWorker[], actorId: string, shardIndex = 0): Promise<void> {
    const promise = client.activate(actorId, `https://x/${actorId}.js`, [], BUDGET);
    const message = workers[shardIndex]!.sent.at(-1) as { readonly requestId: string };
    workers[shardIndex]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
    await promise;
  }

  function flushMicrotasks(): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, 0));
  }

  describe("ShardClient host-effect bridge — handler success", () => {
    it("resolves an effect-request through onHostEffect and posts an effect-complete frame back to the worker", async () => {
      const { client, workers } = harness(1, { onHostEffect: async (actorId, effect, params) => ({ actorId, effect, params, from: "handler" }) });
      await activateActor(client, workers, "a");

      workers[0]!.deliver(makeEffectRequestFrame("a", "http-fetch", "a:http-fetch:1", { url: "https://example.test" }));
      await flushMicrotasks();

      const reply = findEffectReply(workers[0]!.sent, "a:http-fetch:1", "effect-complete");
      expect(reply).toBeDefined();
      expect(reply?.frame.envelope.payload.payload.value).toEqual({ actorId: "a", effect: "http-fetch", params: { url: "https://example.test" }, from: "handler" });
    });
  });

  describe("ShardClient host-effect bridge — handler error", () => {
    it("a rejected onHostEffect settles as effect-error, never a hang", async () => {
      const { client, workers } = harness(1, { onHostEffect: async () => { throw new Error("boom"); } });
      await activateActor(client, workers, "a");

      workers[0]!.deliver(makeEffectRequestFrame("a", "blob-read", "a:blob-read:1", { hash: "x" }));
      await flushMicrotasks();

      const reply = findEffectReply(workers[0]!.sent, "a:blob-read:1", "effect-error");
      expect(reply?.frame.envelope.payload.payload.message).toBe("boom");
    });
  });

  describe("ShardClient host-effect bridge — no handler installed", () => {
    it("fails FAST with an explicit effect-error, synchronously, never a silent hang", async () => {
      const { client, workers } = harness(1); // no onHostEffect
      await activateActor(client, workers, "a");

      workers[0]!.deliver(makeEffectRequestFrame("a", "storage-read", "a:storage-read:1", {}));
      // no `await flushMicrotasks()` — the reply must already be sent, proving this path never touches a Promise chain
      const reply = findEffectReply(workers[0]!.sent, "a:storage-read:1", "effect-error");
      expect(reply?.frame.envelope.payload.payload.message).toBe("no host effect handler installed");
    });
  });

  describe("ShardClient host-effect bridge — backpressure cap", () => {
    it("rejects an effect-request beyond maxOutstandingEffectsPerActor with a quota-shaped effect-error, while the earlier one stays pending", async () => {
      const { client, workers } = harness(1, { maxOutstandingEffectsPerActor: 1, onHostEffect: () => new Promise(() => {}) });
      await activateActor(client, workers, "a");

      workers[0]!.deliver(makeEffectRequestFrame("a", "spawn-job", "a:spawn-job:1", {}));
      workers[0]!.deliver(makeEffectRequestFrame("a", "spawn-job", "a:spawn-job:2", {}));

      expect(findEffectReply(workers[0]!.sent, "a:spawn-job:1", "effect-error")).toBeUndefined();
      expect(findEffectReply(workers[0]!.sent, "a:spawn-job:1", "effect-complete")).toBeUndefined();
      const reply = findEffectReply(workers[0]!.sent, "a:spawn-job:2", "effect-error");
      expect(reply?.frame.envelope.payload.payload.message).toMatch(/outstandingRequests.*limit=1.*actual=1/);
    });
  });

  describe("ShardClient host-effect bridge — shard-loss settlement", () => {
    it("terminate() aborts every outstanding effect for its actors, and a late handler resolution posts no reply to the dead worker", async () => {
      let capturedSignal: AbortSignal | undefined;
      const { client, workers } = harness(1, {
        onHostEffect: (_actorId, _effect, _params, signal) =>
          new Promise((resolve) => {
            capturedSignal = signal;
            signal.addEventListener("abort", () => resolve("too-late"));
          }),
      });
      await activateActor(client, workers, "a");

      workers[0]!.deliver(makeEffectRequestFrame("a", "http-fetch", "a:http-fetch:1", {}));
      expect(capturedSignal?.aborted).toBe(false);

      client.terminate(0);
      expect(capturedSignal?.aborted).toBe(true);

      const sentBeforeLateResolve = workers[0]!.sent.length;
      await flushMicrotasks();
      expect(workers[0]!.sent.length).toBe(sentBeforeLateResolve); // the abort-triggered resolve did NOT produce a late effect-complete/effect-error post
      expect(findEffectReply(workers[0]!.sent, "a:http-fetch:1", "effect-complete")).toBeUndefined();
      expect(findEffectReply(workers[0]!.sent, "a:http-fetch:1", "effect-error")).toBeUndefined();
    });

    it("dispose(actorId) aborts that actor's outstanding effects without touching a sibling actor's", async () => {
      const signals: Record<string, AbortSignal> = {};
      const { client, workers } = harness(1, {
        onHostEffect: (actorId, _effect, _params, signal) => {
          signals[actorId] = signal;
          return new Promise(() => {});
        },
      });
      await activateActor(client, workers, "a");
      await activateActor(client, workers, "b");

      workers[0]!.deliver(makeEffectRequestFrame("a", "http-fetch", "a:http-fetch:1", {}));
      workers[0]!.deliver(makeEffectRequestFrame("b", "http-fetch", "b:http-fetch:1", {}, client.captureActorActivation("b").activationGeneration));

      client.dispose("a");
      expect(signals.a?.aborted).toBe(true);
      expect(signals.b?.aborted).toBe(false);
    });
  });
  //#endregion 🌉️HostEffectBridge tests

  //#region 🪪️InboundActivation
  describe("ShardClient inbound activation authority", () => {
    it("matches the neutral stale-source matrix without consuming exact close receipts", async () => {
      const { default: Ajv } = await import("ajv");
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../🪪️activation/📨️inbound/🧪️fixture/🔣️.json", source.url), "utf8")) as { actorId: string; requestId: string; cases: Array<{ name: string; effects: number; traps: number }> };
      const schema = JSON.parse(readFileSync(new URL("../🪪️activation/📨️inbound/🧬️schema/🔣️.json", source.url), "utf8"));
      const oracle = new Ajv();
      expect(oracle.validate(schema, fixture)).toBe(true);
      for (const row of fixture.cases) {
        const effects: string[] = [];
        const traps: string[] = [];
        const { client, workers } = harness(1, { onHostEffect: async (_actor, effect) => { effects.push(effect); return "ok"; }, onActorTrap: (_actor, message) => traps.push(message) });
        await activateActor(client, workers, fixture.actorId);
        const source = workers[0]!;
        const generation = client.captureActorActivation(fixture.actorId).activationGeneration;
        let close: ShardInstanceLifecycleLease | undefined;
        if (row.name === "replaced-worker") {
          client.terminate(0);
          client.rebuild(0);
          await activateActor(client, workers, fixture.actorId, 1);
        } else if (row.name === "reactivated") {
          client.dispose(fixture.actorId);
          await activateActor(client, workers, fixture.actorId);
        } else if (row.name === "closing") {
          close = await captureFixtureInstance(client, source, fixture.actorId);
          close.beginClose();
        }
        const frame = makeEffectRequestFrame(fixture.actorId, "storage-read", fixture.requestId, {});
        const message: Record<string, unknown> = { ...frame, activationGeneration: row.name === "foreign-generation" ? generation + 1n : generation };
        const trap: Record<string, unknown> = { kind: "trap", actorId: fixture.actorId, activationGeneration: message.activationGeneration, message: "fixture-trap" };
        if (row.name === "missing-generation") { delete message.activationGeneration; delete trap.activationGeneration; }
        if (row.name === "foreign-envelope" && frame.kind === "frame" && frame.frame.kind === "Envelope") {
          message.frame = { ...frame.frame, envelope: { ...frame.frame.envelope, from: { kind: "actor", id: "another-actor" } } };
        }
        source.onmessage?.({ data: message });
        source.onmessage?.({ data: trap });
        await flushMicrotasks();
        const actual = { name: row.name, effects: effects.length, traps: traps.length };
        expect(actual, row.name).toEqual(row);
        expect(oracle.validate({ const: row }, actual), row.name).toBe(true);
        if (close) {
          await retireFixtureInstance(source, close);
          expect(close.progress().kind).toBe("complete");
        }
        client.dispose(fixture.actorId);
      }
    });

    it("keeps a replacement effect with a reused request id when the old handler settles", async () => {
      const settlements: Array<(value: unknown) => void> = [];
      const signals: AbortSignal[] = [];
      const { client, workers } = harness(1, { onHostEffect: (_actor, _effect, _params, signal) => new Promise((resolve) => { signals.push(signal); settlements.push(resolve); }) });
      await activateActor(client, workers, "a");
      const oldGeneration = client.captureActorActivation("a").activationGeneration;
      workers[0]!.onmessage?.({ data: { ...makeEffectRequestFrame("a", "storage-read", "shared", {}), activationGeneration: oldGeneration } });
      client.dispose("a");
      expect(signals[0]!.aborted).toBe(true);
      await activateActor(client, workers, "a");
      const generation = client.captureActorActivation("a").activationGeneration;
      workers[0]!.onmessage?.({ data: { ...makeEffectRequestFrame("a", "storage-read", "shared", {}), activationGeneration: generation } });
      settlements[0]!("old");
      await flushMicrotasks();
      expect(findEffectReply(workers[0]!.sent, "shared", "effect-complete")).toBeUndefined();
      expect(signals[1]!.aborted).toBe(false);
      settlements[1]!("new");
      await flushMicrotasks();
      const reply = findEffectReply(workers[0]!.sent, "shared", "effect-complete");
      expect(reply?.frame.envelope.payload.payload.value).toBe("new");
      expect((reply as unknown as { activationGeneration: bigint }).activationGeneration).toBe(generation);
      client.dispose("a");
    });

    it("aborts the admitted host effect on close while retaining the native close owner", async () => {
      let signal: AbortSignal | undefined;
      let settle!: (value: unknown) => void;
      const { client, workers } = harness(1, { onHostEffect: (_actor, _effect, _params, captured) => new Promise((resolve) => { signal = captured; settle = resolve; }) });
      await activateActor(client, workers, "a");
      workers[0]!.onmessage?.({ data: { ...makeEffectRequestFrame("a", "storage-read", "closing", {}), activationGeneration: client.captureActorActivation("a").activationGeneration } });
      const lease = await captureFixtureInstance(client, workers[0]!, "a");
      void lease.beginClose();
      expect(signal?.aborted).toBe(false);
      await answerLifecycle(workers[0]!, lease.poll(BUDGET));
      expect(signal?.aborted).toBe(true);
      settle("late");
      await flushMicrotasks();
      expect(findEffectReply(workers[0]!.sent, "closing", "effect-complete")).toBeUndefined();
      expect(lease.progress().kind).toBe("closing");
      expect(() => client.dispose("a")).toThrow("actor-close.native-retirement-pending");
    });

    it("ignores late old-worker errors and worker traps after slot replacement", async () => {
      const traps: string[] = [];
      const { client, workers } = harness(1, { onActorTrap: (_actor, message) => traps.push(message) });
      await activateActor(client, workers, "a");
      const staleError = workers[0]!.onerror!;
      client.terminate(0);
      client.rebuild(0);
      await activateActor(client, workers, "a", 1);
      const lease = client.captureActorActivation("a");
      staleError(new Error("old-worker"));
      workers[0]!.onmessage?.({ data: { kind: "trap", actorId: "*", activationGeneration: null, message: "old-trap" } });
      expect(() => lease.assertActive()).not.toThrow();
      expect(traps).toEqual([]);
      workers[1]!.onmessage?.({ data: { kind: "trap", actorId: "*", activationGeneration: null, message: "current-worker-trap" } });
      expect(traps).toEqual(["current-worker-trap"]);
      client.dispose("a");
    });
  });
  //#endregion 🪪️InboundActivation

  void vi;

}
