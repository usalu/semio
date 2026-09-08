type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { OwnedActorTurnOutput, OwnedActorTurnOutputs, OwnedResidentLedger, cancelEmpty } = dependencies;

  const { describe, expect, it, vi } = vitest;
  const fixtureLedger = () => new OwnedResidentLedger({ bytes: 65536, slots: 256, owners: 256, control: { bytes: 0, slots: 0, owners: 0 } });
  async function fixtureOutput(queue: OwnedActorTurnOutputs): Promise<OwnedActorTurnOutput | null> {
    const { default: fixture } = await import("../../🏘️admission/🧪️fixture/🔣️.json");
    for (let turn = 0; turn < fixture.phases.length + 1; turn++) { const current = queue.reserve({ maxItems: 1, maxBytes: 4096 }); if (current.step.kind === "ready") return current.output; if (current.step.kind === "blocked" || current.step.kind === "rejected") return null; }
    throw new Error("Response admission exceeded declared transitions");
  }
  describe("OwnedActorTurnOutput", () => {
    it("ActorResponseAdmission declares conserved metadata and separate grants without receiver or refund authority", async () => {
      const { default: contract } = await import("../../🏘️admission/🤝️contract.json"); const { default: schema } = await import("../../🏘️admission/🧬️schema/🔣️.json"); const { default: fixture } = await import("../../🏘️admission/🧪️fixture/🔣️.json"); const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");
      const ajv = new Ajv({ strict: true }); expect(ajv.addSchema(schema).getSchema(`${schema.$id}#/$defs/Admission`)!(contract)).toBe(true); expect(ajv.getSchema(`${schema.$id}#/$defs/AdmissionFixture`)!(fixture)).toBe(true);
      const domain = [contract.slotFields, contract.facadeFields, contract.outcomeFields].reduce((total, fields) => produce(total, value => { value.bytes += contract.model.recordBytes + fields.length * contract.model.fieldBytes; value.slots++; value.owners++; }), { bytes: 0, slots: 0, owners: 0 }); expect(domain).toEqual(contract.domain);
      const retained = [domain, contract.intrinsicRecord, contract.admissionCell].reduce((total, charge) => produce(total, value => { value.bytes += charge.bytes; value.slots += charge.slots; value.owners += charge.owners; }), { bytes: 0, slots: 0, owners: 0 }); expect(retained).toEqual(contract.retained);
      expect(contract.model.recordBytes + contract.rosterFields.length * contract.model.fieldBytes).toBe(contract.parentRoster.bytes); expect(fixture.phases.map(row => row.phase)).toEqual(contract.phases); expect(fixture.phases.map(row => row.grant)).toEqual(contract.grants);
      let usage = { bytes: 0, slots: 0, owners: 0 };
      for (const [index, row] of fixture.phases.entries()) { if (index === 0) usage = produce(usage, value => Object.assign(value, contract.admissionCell)); if (index === 4) usage = produce(usage, value => Object.assign(value, retained)); expect(row.resident).toEqual(usage); expect(row.output).toBe(index === fixture.phases.length - 1); }
      expect(fixture.dispatch).toEqual({ beforeReady: false, withoutReceiver: false }); expect(fixture.retainedCancellation).toEqual({ unlink: false, refund: false, capacityRecovered: false });
    });

    it("ActorResponseAdmission binds its declared fields to the actual output source", async () => {
      const { default: contract } = await import("../../🏘️admission/🤝️contract.json"); const ts = await import("typescript"); const { readFile } = await import("node:fs/promises"); const source = ts.createSourceFile("output.ts", await readFile(new URL("./🟦️.ts", source.url), "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
      const fields = (name: string): string[] => { const declaration = source.statements.find(value => (ts.isClassDeclaration(value) || ts.isTypeAliasDeclaration(value)) && value.name?.text === name); if (!declaration) throw new Error(`Missing ${name}`); if (ts.isClassDeclaration(declaration)) return declaration.members.filter(ts.isPropertyDeclaration).map(value => value.name.getText(source).replace(/^#/, "")); if (ts.isTypeAliasDeclaration(declaration) && ts.isTypeLiteralNode(declaration.type)) return declaration.type.members.filter(ts.isPropertySignature).map(value => value.name.getText(source)); throw new Error(`Invalid ${name}`); };
      expect(fields("OwnedActorTurnOutputs")).toEqual(contract.rosterFields); expect(fields("Slot")).toEqual(contract.slotFields); expect(fields("OwnedActorTurnOutput")).toEqual(contract.facadeFields); expect(fields("OwnedActorTurnOutputOutcome")).toEqual(contract.outcomeFields);
    });

    it("retains the exact constructed shell before a finalizer can throw", async () => {
      const { default: fixture } = await import("../../🧪️fixture/🔣️.json"); const { produce } = await import("immer");
      for (const boundary of fixture.construction.faults) {
        const owner = {}; const queue = new OwnedActorTurnOutputs(owner, fixture.capacity, fixtureLedger()); const original = Object.freeze; const failure = new Error(boundary); const captured: OwnedActorTurnOutput[] = [];
        const finalizer = vi.spyOn(Object, "freeze").mockImplementation(value => {
          if (value instanceof OwnedActorTurnOutput) { captured.push(value); if (boundary === "after-finalize") original(value); throw failure; }
          return original(value);
        });
        try { await expect(fixtureOutput(queue)).rejects.toBe(failure); } finally { finalizer.mockRestore(); }
        expect(captured).toHaveLength(1); const output = captured[0]!;
        expect(queue.pending).toBe(fixture.construction.retainedSlotsAfterFault); expect(queue.peek()).toBe(output); expect(OwnedActorTurnOutput.matches(output, owner)).toBe(true);
        expect(Object.keys(output)).toEqual(fixture.construction.publicCapabilityKeys); const retained = produce(fixture.trace[0]!, state => { state.retained = true; }); expect(output.state).toEqual(retained);
        expect(await fixtureOutput(queue)).toBeNull(); expect(queue.peek()).toBe(output);
        expect(output.cancelEmpty()).toBe(fixture.construction.cancelUnused); expect(output.cancelEmpty()).toBe(fixture.construction.cancelReplay); expect(queue.peek()).toBe(output);
        expect(output.state).toEqual(retained); expect(queue.pending).toBe(1);
      }
    });
    it("ActorOutputFault retains exact constructor failures and rejects empty cancellation or dispatch", async () => {
      const { default: fixture } = await import("../../🧯️fault/🧪️fixture/🔣️.json"); const { default: schema } = await import("../../🧯️fault/🧬️schema/🔣️.json"); const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");
      expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true); const matches = Reflect.get(OwnedActorTurnOutput, "matchesFault"); expect(typeof matches).toBe("function");
      for (const boundary of fixture.boundaries) for (const kind of fixture.values) {
        const queue = new OwnedActorTurnOutputs({}, 1, fixtureLedger()); let reads = 0; const fault = kind === "null" ? null : kind === "undefined" ? undefined : kind === "false" ? false : kind === "zero" ? 0 : { payload: new Uint8Array(fixture.unknownBytes), get message() { reads++; throw new Error("Foreign constructor fault getter"); } };
        const original = Object.freeze; const spy = vi.spyOn(Object, "freeze").mockImplementation(value => { if (value === queue.peek() && value !== null) { if (boundary === "after-finalize") original(value); throw fault; } return original(value); }); let threw = false;
        try { await fixtureOutput(queue); } catch (error) { threw = true; expect(Object.is(error, fault)).toBe(true); } finally { spy.mockRestore(); }
        const output = queue.peek(); if (!output) throw new Error("Original faulted shell lost"); expect(threw).toBe(true); expect(Reflect.apply(matches, OwnedActorTurnOutput, [output, fault])).toBe(fixture.constructor.originalFaultRetained);
        expect(output.state).toEqual(produce({ capacity: 1, sequence: "1", phase: "reserved", retained: false }, value => { value.retained = true; })); expect(output.cancelEmpty()).toBe(fixture.constructor.cancelEmpty);
        let called = false; await expect(output.run(async () => { called = true; return null; })).rejects.toThrow("actor-output.faulted"); expect(called).toBe(fixture.constructor.runAllowed); expect(queue.peek()).toBe(output); expect(queue.pending).toBe(fixture.constructor.retainedSlots); expect(reads).toBe(0);
      }
    });
    it("ActorOutputFault installs returned and refused outcomes before finalization without replacing either root", async () => {
      const { default: fixture } = await import("../../🧯️fault/🧪️fixture/🔣️.json"); const matches = Reflect.get(OwnedActorTurnOutput, "matchesFault");
      for (const outcome of fixture.outcomes) for (const boundary of fixture.boundaries) for (const kind of fixture.values) {
        const queue = new OwnedActorTurnOutputs({}, 1, fixtureLedger()); const output = (await fixtureOutput(queue))!; const raw = { payload: new Uint8Array(fixture.unknownBytes), outcome }; let reads = 0;
        const fault = kind === "null" ? null : kind === "undefined" ? undefined : kind === "false" ? false : kind === "zero" ? 0 : { payload: new Uint8Array(fixture.unknownBytes), get message() { reads++; throw new Error("Foreign outcome fault getter"); } };
        const original = Object.freeze; let observed = false; const spy = vi.spyOn(Object, "freeze").mockImplementation(value => { if (value !== null && typeof value === "object" && Object.getOwnPropertyDescriptor(value, "value")?.value === raw && Object.getOwnPropertyDescriptor(value, "kind")?.value === outcome) { observed = true; expect(output.outcome === value).toBe(true); if (boundary === "after-finalize") original(value); throw fault; } return original(value); });
        try { await expect(output.run(async () => { if (outcome === "refused") throw raw; return raw; })).rejects.toBe(fault); } finally { spy.mockRestore(); }
        expect(observed).toBe(fixture.outcome.installBeforeFinalizer); expect(output.outcome?.kind).toBe(outcome); expect(output.outcome?.value === raw).toBe(fixture.outcome.originalOutcomeRetained); expect(Reflect.apply(matches, OwnedActorTurnOutput, [output, fault])).toBe(fixture.outcome.originalFaultRetained);
        expect(output.cancelEmpty()).toBe(fixture.outcome.cancelEmpty); expect(queue.peek()).toBe(output); expect(queue.pending).toBe(1); expect(reads).toBe(0);
      }
    });
    it("pre-admits a strong exact output owner before dispatch and retains it across caller faults", async () => {
      const { default: fixture } = await import("../../🧪️fixture/🔣️.json");
      const { default: schema } = await import("../../🧬️schema/🔣️.json");
      const { default: lifetimeSchema } = await import("../../../../../🚪️lifetime/🧬️schema/🔣️.json");
      const { default: Ajv } = await import("ajv");
      const { produce } = await import("immer");
      const validate = new Ajv({ strict: true }).addSchema(lifetimeSchema).compile(schema);
      const owner = Object.freeze({});
      const queue = new OwnedActorTurnOutputs(owner, fixture.capacity, fixtureLedger());
      const output = (await fixtureOutput(queue))!;
      const trace = [output.state];
      expect(OwnedActorTurnOutput.matches(output, owner)).toBe(true);
      expect(OwnedActorTurnOutput.matches(output, {})).toBe(false);
      let resolve!: (value: object) => void;
      const raw = { unknown: { marker: fixture.mutablePayload.before }, uiPatches: [{ ops: [new Uint8Array(8192)] }] };
      const pending = output.run(() => new Promise<object>(done => { resolve = done; }));
      trace.push(output.state);
      const observer = pending.then(() => { throw new Error("caller publication fault"); });
      const observed = expect(observer).rejects.toThrow("caller publication fault");
      resolve(raw); await observed; trace.push(output.state);
      expect(trace).toEqual(fixture.trace); expect(trace.every(state => validate(state))).toBe(true);
      const oracle = produce(fixture.trace[0]!, state => { state.phase = "returned"; state.retained = true; });
      expect(output.state).toEqual(oracle);
      expect(queue.peek()).toBe(output); expect(output.outcome).toEqual({ kind: "returned", value: raw });
      expect(output.outcome?.value).toBe(raw); expect(queue.pending).toBe(1);
      raw.unknown.marker = fixture.mutablePayload.after; expect((output.outcome?.value as typeof raw).unknown.marker).toBe(fixture.mutablePayload.after);
      queue.beginClose(); expect(await fixtureOutput(queue)).toBeNull(); expect(output.cancelEmpty()).toBe(false); expect(queue.peek()).toBe(output);
    });

    it("refuses full admission and sequence exhaustion without discarding retained outputs", async () => {
      const { default: fixture } = await import("../../🧪️fixture/🔣️.json");
      const ledger = fixtureLedger(); const queue = new OwnedActorTurnOutputs({}, fixture.capacity, ledger);
      const first = (await fixtureOutput(queue))!;
      const refused = new Error("post refused");
      await expect(first.run(() => { throw refused; })).rejects.toBe(refused);
      expect(first.outcome?.value).toBe(refused); expect(first.outcome?.kind).toBe("refused"); expect(first.cancelEmpty()).toBe(false);
      const second = (await fixtureOutput(queue))!; expect(second.state.sequence).toBe("2"); const retained = ledger.usage;
      expect(second.cancelEmpty()).toBe(true); expect(second.cancelEmpty()).toBe(false); expect(queue.pending).toBe(fixture.capacity); expect(await fixtureOutput(queue)).toBeNull(); expect(ledger.usage).toEqual(retained); expect(queue.peek()).toBe(first);
      let calls = 0; await expect(first.run(async () => { calls++; return {}; })).rejects.toThrow("actor-output.already-submitted"); expect(calls).toBe(0);
      const exhausted = new OwnedActorTurnOutputs({}, fixture.capacity, fixtureLedger(), BigInt(fixture.maximumSequence) - 1n);
      const last = (await fixtureOutput(exhausted))!; expect(last.state.sequence).toBe(fixture.maximumSequence); expect(last.cancelEmpty()).toBe(true); expect(await fixtureOutput(exhausted)).toBeNull(); expect(exhausted.peek()).toBe(last); expect(queue.peek()).toBe(first);
    });

    it("rejects fabricated reservations without consulting public getters", () => {
      let reads = 0;
      const forged = { get owner() { reads++; throw new Error("Unowned getter"); } };
      expect(OwnedActorTurnOutput.matches(forged, {})).toBe(false);
      expect(OwnedActorTurnOutput.matches(Object.create(OwnedActorTurnOutput.prototype), {})).toBe(false);
      expect(() => Reflect.construct(OwnedActorTurnOutput, [forged])).toThrow("actor-output.private-mint"); expect(reads).toBe(0);
    });
    it("captures the original response envelope before settlement or failure extraction can throw", async () => {
      const { default: fixture } = await import("../../🧪️fixture/🔣️.json");
      const { default: schema } = await import("../../🧬️schema/🔣️.json");
      const { default: lifetimeSchema } = await import("../../../../../🚪️lifetime/🧬️schema/🔣️.json");
      const { default: Ajv } = await import("ajv");
      const { produce } = await import("immer");
      const validate = new Ajv({ strict: true }).addSchema(lifetimeSchema).compile(schema);
      for (const kind of fixture.responseSettlement.outcomes) {
        const owner = Object.freeze({}); const queue = new OwnedActorTurnOutputs(owner, fixture.capacity, fixtureLedger()); const output = (await fixtureOutput(queue))!;
        const raw = { kind: "result", ok: kind === "success", value: { uiPatches: [] }, framesBytes: new Uint8Array(fixture.responseSettlement.unknownPayloadBytes), unknown: { retained: true } };
        const observed: string[] = [];
        const pending = output.run(async () => {
          expect(output.captureResponse(raw)).toBe(true); observed.push(fixture.responseSettlement.phases[0]!);
          expect(output.responseEnvelope).toBe(raw); expect(output.state.phase).toBe("returned"); expect(validate(output.state)).toBe(true);
          expect(output.captureResponse({ ...raw })).toBe(fixture.responseSettlement.replaceCapturedResponse);
          for (const phase of fixture.responseSettlement.phases.slice(1)) {
            observed.push(phase); expect(queue.peek()?.responseEnvelope).toBe(raw);
            if (kind === "graft-fault" && phase === "error-graft") throw new Error("graft fixture fault");
          }
          return raw.value;
        });
        if (kind === "graft-fault") await expect(pending).rejects.toThrow("graft fixture fault");
        else if (kind === "callback-fault") await expect(pending.then(() => { throw new Error("caller fixture fault"); })).rejects.toThrow("caller fixture fault");
        else expect(await pending).toBe(raw.value);
        expect(output.responseEnvelope).toBe(raw); expect(queue.peek()).toBe(output); expect(raw.framesBytes.byteLength).toBe(fixture.responseSettlement.unknownPayloadBytes);
        expect(observed).toEqual(kind === "graft-fault" ? fixture.responseSettlement.phases.slice(0, 4) : fixture.responseSettlement.phases);
        expect(output.state).toEqual(produce(fixture.trace[0]!, state => { state.phase = "returned"; state.retained = true; }));
        expect(output.cancelEmpty()).toBe(false); queue.beginClose(); expect(output.responseEnvelope).toBe(raw);
      }
    });
  });

}
