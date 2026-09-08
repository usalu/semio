type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { ACTOR_RETURN_CONTROL_MAXIMUM_BYTES, ACTOR_RETURN_DRIVE_MAXIMUM_BYTES, ACTOR_RETURN_IDENTITY_MAXIMUM_BYTES, ACTOR_RETURN_ORIGIN_MAXIMUM_BYTES, ACTOR_RETURN_PAGE_RECEIPT_MAXIMUM_BYTES, ACTOR_RETURN_RESULT_MAXIMUM_BYTES, ActorReturnResultFraming, createActorBytePage, decodeActorReturnDrive, decodeActorReturnResult, encodeActorReturnDrive, encodeActorReturnResult, readActorBytePage } = dependencies;
  type ActorReturnControl = any;
  type ActorReturnDrive = any;
  type ActorReturnIdentity = any;
  type ActorReturnOrigin = any;
  type ActorReturnPageReceipt = any;
  type ActorReturnResult = any;

  const { it, expect, vi } = vitest;
  const hydrate = (value: unknown): ActorReturnDrive => JSON.parse(JSON.stringify(value), (key, item) => ["activationGeneration", "returnSequence", "pageSequence"].includes(key) ? BigInt(item) : item);
  const hydrateResult = (value: unknown): ActorReturnResult => JSON.parse(JSON.stringify(value), (key, item) => ["activationGeneration", "returnSequence", "pageSequence"].includes(key) ? BigInt(item) : item);
  const resultOracle = async (): Promise<(value: ActorReturnResult) => Buffer> => {
    const { default: fixture } = await import("../../🧫️fixture/🔣️.json");
    const moduleName = "@webassemblyjs/leb128/lib/leb.js";
    const module = await import(moduleName);
    const encode = (module.default ?? module).encodeUIntBuffer;
    const uint = (value: bigint | number): Buffer => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(BigInt(value)); return Buffer.from(encode(bytes)); };
    const origin = (value: ActorReturnOrigin): Buffer[] => [uint(value.activationGeneration), uint(value.requestSequence)];
    const identity = (value: ActorReturnIdentity): Buffer[] => [...origin(value.origin), uint(value.returnSequence)];
    const receipt = (value: ActorReturnPageReceipt): Buffer[] => [...identity(value.identity), uint(value.pageSequence), uint(value.length), Buffer.from([value.final ? 1 : 0])];
    const control = (value: ActorReturnControl): Buffer[] => [Buffer.from([fixture.controlTags[value.kind]]), ...(value.kind === "inputAck" ? receipt(value.receipt) : identity(value.identity))];
    return value => {
      const tag = Buffer.from([fixture.resultTags[value.kind]]);
      switch (value.kind) {
        case "protocolFault": return Buffer.concat([tag, Buffer.from([fixture.resultEnums.fault.indexOf(value.fault)])]);
        case "refused": return Buffer.concat([tag, ...origin(value.origin), Buffer.from([fixture.resultEnums.fault.indexOf(value.fault)])]);
        case "pending": return Buffer.concat([tag, ...identity(value.identity), Buffer.from([fixture.resultEnums.reason.indexOf(value.reason)])]);
        case "retired": return Buffer.concat([tag, ...identity(value.identity), Buffer.from([fixture.resultEnums.completion.indexOf(value.completion)])]);
        case "control": return Buffer.concat([tag, ...control(value.control), Buffer.from([fixture.resultEnums.outcome.indexOf(value.outcome), fixture.resultEnums.fault.indexOf(value.fault)])]);
        case "page": {
          const bytes = Buffer.alloc(fixture.maximumPageBytes);
          for (let block = 0; block < 64; block++) for (let word = 0; word < 8; word++) bytes.writeBigUInt64LE(Reflect.get(Reflect.get(value.page, `block${String(block).padStart(2, "0")}`), `word${word}`), block * 64 + word * 8);
          return Buffer.concat([tag, ...receipt(value.receipt), bytes]);
        }
      }
    };
  };

  it("ActorReturn codecs load natively in Node strip-only mode and preserve every shared vector", async () => {
    const { spawnSync } = await import("node:child_process");
    const source = new URL(source.url).href;
    const fixture = new URL("./🧫️fixture/🔣️.json", source).href;
    const page = new URL("../📃️page/🟦️.ts", source).href;
    const program = `
      import assert from "node:assert/strict";
      import { readFileSync } from "node:fs";
      import { fileURLToPath } from "node:url";
      const api = await import(${JSON.stringify(source)});
      const pageApi = await import(${JSON.stringify(page)});
      const fixture = JSON.parse(readFileSync(fileURLToPath(${JSON.stringify(fixture)}), "utf8"));
      const hydrate = value => JSON.parse(JSON.stringify(value), (key, item) => ["activationGeneration", "returnSequence", "pageSequence"].includes(key) ? BigInt(item) : item);
      for (const row of fixture.wireVectors) {
        const value = hydrate(row.value);
        const bytes = api.encodeActorReturnDrive(value);
        assert.equal(Buffer.from(bytes).toString("hex"), row.hex);
        assert.deepStrictEqual(api.decodeActorReturnDrive(bytes), value);
      }
      for (const row of fixture.resultVectors) {
        const value = hydrate(row.value);
        const bytes = api.encodeActorReturnResult(value);
        assert.equal(Buffer.from(bytes).toString("hex"), row.hex);
        assert.deepStrictEqual(api.decodeActorReturnResult(bytes), value);
      }
      for (const row of fixture.pageResultVectors) {
        const payload = Uint8Array.from({ length: row.pageLength }, (_, index) => row.pattern === "mod37plus11" ? (index * 37 + 11) % 256 : 0);
        const receipt = hydrate(row.receipt);
        const value = { kind: "page", receipt, page: pageApi.createActorBytePage(payload) };
        const bytes = api.encodeActorReturnResult(value);
        const expectedPage = Buffer.alloc(fixture.maximumPageBytes);
        expectedPage.set(payload);
        const expected = Buffer.concat([Buffer.from(row.prefixHex, "hex"), expectedPage]);
        assert.equal(bytes.length, row.wireBytes);
        assert.deepStrictEqual(Buffer.from(bytes), expected);
        const decoded = api.decodeActorReturnResult(bytes);
        assert.equal(decoded.kind, "page");
        assert.deepStrictEqual(decoded.receipt, receipt);
        assert.deepStrictEqual(pageApi.readActorBytePage(decoded.page), payload);
      }
    `;
    const environment = { ...process.env }; delete environment.NO_COLOR;
    const child = spawnSync("node", ["--experimental-strip-types", "--input-type=module", "--eval", program], { encoding: "utf8", env: environment });
    if (child.status !== 0) throw new Error(child.stderr);
    expect({ status: child.status, signal: child.signal, stdout: child.stdout }).toEqual({ status: 0, signal: null, stdout: "" });
  });

  it("ActorReturnDrive matches the shared canonical vectors and independent LEB128 bytes", async () => {
    const { default: fixture } = await import("../../🧫️fixture/🔣️.json");
    const { default: schema } = await import("../../🧬️schema/🔣️.json");
    const { default: lifetimeSchema } = await import("../../../🚪️lifetime/🧬️schema/🔣️.json");
    const { default: pageSchema } = await import("../../../📃️page/🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true }).addSchema(lifetimeSchema).addSchema(pageSchema).addSchema(schema);
    expect(ajv.getSchema(`${schema.$id}#/$defs/ReturnFixture`)!(fixture)).toBe(true);
    expect(ACTOR_RETURN_ORIGIN_MAXIMUM_BYTES).toBe(fixture.maximumOriginBytes);
    expect(ACTOR_RETURN_IDENTITY_MAXIMUM_BYTES).toBe(fixture.maximumIdentityBytes);
    expect(ACTOR_RETURN_PAGE_RECEIPT_MAXIMUM_BYTES).toBe(fixture.maximumPageReceiptBytes);
    expect(ACTOR_RETURN_CONTROL_MAXIMUM_BYTES).toBe(fixture.maximumControlBytes);
    expect(ACTOR_RETURN_DRIVE_MAXIMUM_BYTES).toBe(fixture.maximumDriveBytes);
    const moduleName = "@webassemblyjs/leb128/lib/leb.js";
    const module: unknown = await import(moduleName);
    const oracle: unknown = module && typeof module === "object" ? Reflect.get(module, "default") ?? module : null;
    const encode: unknown = oracle && typeof oracle === "object" ? Reflect.get(oracle, "encodeUIntBuffer") : null;
    if (typeof encode !== "function") throw new Error("missing independent LEB128 oracle");
    for (const row of fixture.wireVectors) {
      const drive = hydrate(row.value);
      const bytes = encodeActorReturnDrive(drive);
      expect(Buffer.from(bytes).toString("hex")).toBe(row.hex);
      expect(decodeActorReturnDrive(bytes)).toEqual(drive);
      const identity = drive.kind === "execute" ? null : drive.control.kind === "inputAck" ? drive.control.receipt.identity : drive.control.identity;
      const origin = drive.kind === "execute" ? drive.origin : identity!.origin;
      const values = [origin.activationGeneration, BigInt(origin.requestSequence), ...(identity ? [identity.returnSequence] : [])];
      const tag = drive.kind === "execute" ? [fixture.driveTags.execute] : [fixture.driveTags.control, fixture.controlTags[drive.control.kind]];
      const receipt = drive.kind === "control" && drive.control.kind === "inputAck" ? drive.control.receipt : null;
      if (receipt) values.push(receipt.pageSequence, BigInt(receipt.length));
      const independent = values.map(value => { const buffer = Buffer.alloc(8); buffer.writeBigUInt64LE(value); return Buffer.from(encode(buffer)); });
      expect(Buffer.concat([Buffer.from(tag), ...independent, ...(receipt ? [Buffer.from([receipt.final ? 1 : 0])] : [])])).toEqual(Buffer.from(bytes));
      for (let length = 0; length < bytes.length; length++) expect(() => decodeActorReturnDrive(bytes.subarray(0, length))).toThrow();
      const offset = new Uint8Array(bytes.length + 3); offset.set(bytes, 2);
      expect(decodeActorReturnDrive(offset.subarray(2, bytes.length + 2))).toEqual(drive);
    }
    expect(fixture.wireVectors.at(-1)!.hex.length / 2).toBe(ACTOR_RETURN_DRIVE_MAXIMUM_BYTES);
  });

  it("ActorReturnDrive rejects malformed, noncanonical and trailing input without mutating the source", async () => {
    const { default: fixture } = await import("../../🧫️fixture/🔣️.json");
    for (const hex of fixture.malformedWire) {
      const bytes = Uint8Array.from(Buffer.from(hex, "hex")); const original = bytes.slice();
      expect(() => decodeActorReturnDrive(bytes)).toThrow(); expect(bytes).toEqual(original);
    }
    const valid = encodeActorReturnDrive(hydrate(fixture.wireVectors[0]!.value));
    for (const tail of [0, 1, 255]) expect(() => decodeActorReturnDrive(Uint8Array.from([...valid, tail]))).toThrow();
    expect(() => decodeActorReturnDrive(new Uint8Array(fixture.maximumDriveBytes + 1))).toThrow();
    for (const value of [null, undefined, [], {}, "000709"]) expect(() => decodeActorReturnDrive(value as Uint8Array)).toThrow();
  });

  it("ActorReturnDrive enforces exact unsigned authority, safe transport ids and final-page length", () => {
    const origin = { activationGeneration: 7n, requestSequence: 9 };
    const identity = { origin, returnSequence: 11n };
    const receipt = { identity, pageSequence: 1n, length: 3, final: true };
    const makeAck = (value: ActorReturnPageReceipt): ActorReturnDrive => ({ kind: "control", control: { kind: "inputAck", receipt: value } });
    for (const value of [0n, -1n, 0x10000000000000000n, 1, "1", null, undefined]) {
      expect(() => encodeActorReturnDrive({ kind: "execute", origin: { ...origin, activationGeneration: value as bigint } })).toThrow();
      expect(() => encodeActorReturnDrive(makeAck({ ...receipt, identity: { ...identity, returnSequence: value as bigint } }))).toThrow();
      expect(() => encodeActorReturnDrive(makeAck({ ...receipt, pageSequence: value as bigint }))).toThrow();
    }
    for (const requestSequence of [0, -1, 1.5, Number.MAX_SAFE_INTEGER + 1, Infinity, NaN, 9n, "9"]) expect(() => encodeActorReturnDrive({ kind: "execute", origin: { ...origin, requestSequence: requestSequence as number } })).toThrow();
    for (const length of [-1, 4097, 0.5, Infinity, NaN, 3n, "3"]) expect(() => encodeActorReturnDrive(makeAck({ ...receipt, length: length as number }))).toThrow();
    const invalidFinals: readonly unknown[] = [0, 1, "true", null, undefined];
    for (const final of invalidFinals) expect(() => encodeActorReturnDrive(makeAck({ ...receipt, final: final as boolean }))).toThrow();
    expect(() => encodeActorReturnDrive(makeAck({ ...receipt, length: 0, final: false }))).toThrow();
    expect(decodeActorReturnDrive(encodeActorReturnDrive(makeAck({ ...receipt, length: 0, final: true })))).toEqual(makeAck({ ...receipt, length: 0, final: true }));
    for (const kind of ["input-ack", "retired-ack", "unknown"]) expect(() => encodeActorReturnDrive({ kind: "control", control: { kind, identity } } as ActorReturnDrive)).toThrow();
  });

  it("ActorReturnResult matches all shared fixed results and the independent LEB128 oracle", async () => {
    const { default: fixture } = await import("../../🧫️fixture/🔣️.json");
    const oracle = await resultOracle();
    for (const row of fixture.resultVectors) {
      const value = hydrateResult(row.value);
      const encoded = encodeActorReturnResult(value);
      expect(Buffer.from(encoded).toString("hex")).toBe(row.hex);
      expect(Buffer.from(encoded)).toEqual(oracle(value));
      expect(decodeActorReturnResult(encoded)).toEqual(value);
      expect(Object.isFrozen(decodeActorReturnResult(encoded))).toBe(true);
      for (let length = 0; length < encoded.length; length++) expect(() => decodeActorReturnResult(encoded.subarray(0, length))).toThrow();
      expect(() => decodeActorReturnResult(Uint8Array.from([...encoded, 0]))).toThrow();
    }
  });

  it("ActorReturnResult preserves exact fixed page bytes, lengths and the 4138 byte maximum", async () => {
    const { default: fixture } = await import("../../🧫️fixture/🔣️.json");
    const oracle = await resultOracle();
    expect(ACTOR_RETURN_RESULT_MAXIMUM_BYTES).toBe(fixture.maximumResultBytes);
    for (const row of fixture.pageResultVectors) {
      const payload = Buffer.alloc(fixture.maximumPageBytes);
      if (row.pattern === "mod37plus11") for (let index = 0; index < row.pageLength; index++) payload[index] = (index * 37 + 11) % 256;
      const source = Uint8Array.from(payload.subarray(0, row.pageLength));
      const receipt = hydrateResult({ kind: "control", control: { kind: "inputAck", receipt: row.receipt }, outcome: "accepted", fault: "none" });
      if (receipt.kind !== "control" || receipt.control.kind !== "inputAck") throw new Error("invalid fixture");
      const value: ActorReturnResult = { kind: "page", receipt: receipt.control.receipt, page: createActorBytePage(source) };
      const expected = Buffer.concat([Buffer.from(row.prefixHex, "hex"), payload]);
      const bytes = encodeActorReturnResult(value);
      expect(bytes.length).toBe(row.wireBytes); expect(Buffer.from(bytes)).toEqual(expected); expect(oracle(value)).toEqual(expected);
      const backing = new Uint8Array(bytes.length + 7); backing.set(bytes, 3);
      const decoded = decodeActorReturnResult(backing.subarray(3, bytes.length + 3));
      expect(decoded).toEqual(value);
      if (decoded.kind !== "page") throw new Error("missing page result");
      expect(Object.isFrozen(decoded.page)).toBe(true); expect(Object.isFrozen(decoded.receipt.identity.origin)).toBe(true);
      expect(readActorBytePage(decoded.page)).toEqual(source);
      for (let length = 0; length < bytes.length; length++) expect(() => decodeActorReturnResult(bytes.subarray(0, length))).toThrow();
      expect(() => decodeActorReturnResult(Uint8Array.from([...bytes, 0]))).toThrow();
      source.fill(0); expect(Buffer.from(encodeActorReturnResult(value))).toEqual(expected);
      if (row.pageLength === 0) {
        const dirty = bytes.slice(); dirty[dirty.length - 1] = 1;
        expect(() => decodeActorReturnResult(dirty)).toThrow();
        expect(() => encodeActorReturnResult({ ...value, page: createActorBytePage(new Uint8Array(1)) })).toThrow();
      }
    }
  });

  it("ActorReturnResult rejects shared contradictions and every enum boundary in both directions", async () => {
    const { default: fixture } = await import("../../🧫️fixture/🔣️.json");
    const { default: schema } = await import("../../🧬️schema/🔣️.json");
    const { default: lifetimeSchema } = await import("../../../🚪️lifetime/🧬️schema/🔣️.json");
    const { default: pageSchema } = await import("../../../📃️page/🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv");
    const validate = new Ajv({ strict: true }).addSchema(lifetimeSchema).addSchema(pageSchema).addSchema(schema).getSchema("https://semio.tech/schema/framework/actor/return/schema.json#/$defs/Result")!;
    const oracle = await resultOracle();
    for (const row of fixture.resultContradictions) {
      expect(validate(row)).toBe(false);
      const value = hydrateResult(row);
      expect(() => encodeActorReturnResult(value)).toThrow();
      const bytes = Uint8Array.from(oracle(value)); const original = bytes.slice();
      expect(() => decodeActorReturnResult(bytes)).toThrow(); expect(bytes).toEqual(original);
    }
    for (const drive of fixture.wireVectors.slice(1, 5)) {
      for (const outcome of fixture.resultEnums.outcome) for (const fault of fixture.resultEnums.fault) {
        const row = { kind: "control", control: drive.value.control, outcome, fault };
        const value = hydrateResult(row); const bytes = oracle(value);
        if (validate(row)) { expect(Buffer.from(encodeActorReturnResult(value))).toEqual(bytes); expect(decodeActorReturnResult(bytes)).toEqual(value); }
        else { expect(() => encodeActorReturnResult(value)).toThrow(); expect(() => decodeActorReturnResult(bytes)).toThrow(); }
      }
    }
    for (const row of fixture.resultVectors) {
      const bytes = Uint8Array.from(Buffer.from(row.hex, "hex")); bytes[bytes.length - 1] = 255;
      expect(() => decodeActorReturnResult(bytes)).toThrow();
    }
    for (const value of [null, undefined, [], {}, "00070901"]) expect(() => decodeActorReturnResult(value as Uint8Array)).toThrow();
    expect(() => decodeActorReturnResult(new Uint8Array(fixture.maximumResultBytes + 1))).toThrow();
    expect(() => decodeActorReturnResult(Uint8Array.of(5))).toThrow();
    expect(() => decodeActorReturnResult(Uint8Array.of(fixture.resultTags.protocolFault + 1, 12))).toThrow();
  });

  it("ActorReturnResult encodes pre-admission protocol faults without inventing return authority", async () => {
    const { default: fixture } = await import("../../🧫️fixture/🔣️.json");
    const oracle = await resultOracle();
    for (const row of fixture.preAdmissionFaults) {
      const bytes = Uint8Array.from(Buffer.from(row.invalidDriveHex, "hex")); const original = bytes.slice();
      expect(() => decodeActorReturnDrive(bytes)).toThrow(); expect(bytes).toEqual(original);
      const result = decodeActorReturnResult(Uint8Array.from(Buffer.from(row.resultHex, "hex")));
      expect(result).toEqual({ kind: "protocolFault", fault: "malformedControl" });
      expect(Object.keys(result)).toEqual(["kind", "fault"]);
      expect(Buffer.from(encodeActorReturnResult(result)).toString("hex")).toBe(row.resultHex);
      expect(oracle(result).toString("hex")).toBe(row.resultHex);
    }
    for (const fault of fixture.resultEnums.fault) {
      const value = hydrateResult({ kind: "protocolFault", fault });
      const bytes = oracle(value);
      if (fault === "malformedControl" || fault === "mixedControl") {
        expect(encodeActorReturnResult(value)).toHaveLength(2);
        expect(Buffer.from(encodeActorReturnResult(value))).toEqual(bytes);
        expect(decodeActorReturnResult(bytes)).toEqual(value);
      } else {
        expect(() => encodeActorReturnResult(value)).toThrow();
        expect(() => decodeActorReturnResult(bytes)).toThrow();
      }
    }
  });

  it("ActorReturnResultFraming validates shared vectors without allocating or exposing page storage", async () => {
    const api = await import("../../🟦️.ts");
    const { default: fixture } = await import("../../🧫️fixture/🔣️.json");
    const { default: law } = await import("../../🌿️framing/🧪️fixture/🔣️.json");
    const { default: schema } = await import("../../🌿️framing/🧬️schema/🔣️.json");
    const { default: returned } = await import("../../🧬️schema/🔣️.json");
    const { default: lifetime } = await import("../../../🚪️lifetime/🧬️schema/🔣️.json");
    const { default: page } = await import("../../../📃️page/🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true }).addSchema(lifetime).addSchema(page).addSchema(returned).addSchema(schema);
    expect(ajv.validate(schema, law)).toBe(true); const oracle = await resultOracle();
    for (const row of fixture.resultVectors) {
      const value = hydrateResult(row.value); const bytes = oracle(value);
      expect(bytes.toString("hex")).toBe(row.hex);
      const parser = new api.ActorReturnResultFraming();
      for (const byte of bytes) parser.push(byte);
      expect(parser.finish()).toEqual(value); expect(parser.finish()).toBe(parser.value);
    }
    for (const row of fixture.pageResultVectors) {
      const receiptResult = hydrateResult({ kind: "control", control: { kind: "inputAck", receipt: row.receipt }, outcome: "accepted", fault: "none" });
      if (receiptResult.kind !== "control" || receiptResult.control.kind !== "inputAck") throw new Error("fixture receipt");
      const payload = Uint8Array.from({ length: row.pageLength }, (_, index) => (index * 37 + 11) % 256);
      const bytes = oracle({ kind: "page", receipt: receiptResult.control.receipt, page: createActorBytePage(payload) });
      const allocations: number[] = [];
      for (const name of ["Uint8Array", "BigUint64Array", "ArrayBuffer"] as const) {
        const original = globalThis[name]; vi.stubGlobal(name, new Proxy(original, { construct(target, args, newTarget) { allocations.push(Number(args[0])); return Reflect.construct(target, args, newTarget); } }));
      }
      let projection;
      try {
        const parser = new api.ActorReturnResultFraming();
        for (let index = 0; index < bytes.length; index++) { expect(parser.value).toBeNull(); parser.push(bytes[index]!); }
        projection = parser.finish(); expect(parser.finish()).toBe(projection); expect(allocations).toHaveLength(law.maximumPayloadCopies);
      } finally { vi.unstubAllGlobals(); }
      expect(projection).toEqual({ kind: "page", receipt: receiptResult.control.receipt, payloadOffset: Buffer.from(row.prefixHex, "hex").length });
      expect(ajv.validate({ $ref: schema.$id + "#/definitions/pageProjection" }, JSON.parse(JSON.stringify(projection, (_, value) => typeof value === "bigint" ? value.toString() : value)))).toBe(true);
      if (projection!.kind !== "page") throw new Error("fixture page projection");
      expect(bytes.subarray(projection!.payloadOffset, projection!.payloadOffset + row.pageLength)).toEqual(Buffer.from(payload));
      expect(Object.keys(projection!).sort()).toEqual(["kind", "payloadOffset", "receipt"]);
    }
  });

  it("ActorReturnResultFraming retains failure across malformed, trailing and truncated input", async () => {
    const api = await import("../../🟦️.ts");
    const { default: fixture } = await import("../../🧫️fixture/🔣️.json"); const oracle = await resultOracle();
    const malformed = fixture.resultContradictions.map(row => oracle(hydrateResult(row)));
    malformed.push(Buffer.of(6), Buffer.of(1, 0, 1, 1, 0), Buffer.of(1, 0x81, 0, 1, 1, 0));
    for (const row of fixture.resultVectors) { const bytes = Buffer.from(row.hex, "hex"); malformed.push(bytes.subarray(0, bytes.length - 1), Buffer.concat([bytes, Buffer.of(0)])); }
    const emptyPage = fixture.pageResultVectors[0]!; const padding = Buffer.alloc(4096); padding[4095] = 1; malformed.push(Buffer.concat([Buffer.from(emptyPage.prefixHex, "hex"), padding]));
    for (const bytes of malformed) {
      const parser = new api.ActorReturnResultFraming();
      expect(() => { for (const byte of bytes) parser.push(byte); parser.finish(); }).toThrow();
      expect(() => parser.push(0)).toThrow(); expect(() => parser.finish()).toThrow(); expect(parser.value).toBeNull();
    }
    for (const byte of [-1, 256, 0.5, NaN]) { const parser = new api.ActorReturnResultFraming(); expect(() => parser.push(byte)).toThrow(); expect(() => parser.finish()).toThrow(); }
  });

  it("ActorReturnResult does not allocate a page for a fixed control or any whole semantic result", async () => {
    const { default: fixture } = await import("../../🧫️fixture/🔣️.json");
    const input = hydrateResult(fixture.resultVectors[0]!.value);
    const allocations: number[] = []; const original = Uint8Array;
    vi.stubGlobal("Uint8Array", new Proxy(original, { construct(target, args, newTarget) {
      if (typeof args[0] !== "number") throw new Error("unexpected whole-buffer constructor");
      allocations.push(args[0]); return Reflect.construct(target, args, newTarget);
    } }));
    let bytes: Uint8Array;
    try { bytes = encodeActorReturnResult(input); expect(decodeActorReturnResult(bytes)).toEqual(input); }
    finally { vi.unstubAllGlobals(); }
    expect(allocations).toHaveLength(1); expect(allocations[0]).toBeLessThanOrEqual(fixture.maximumControlBytes + 3);
    expect(bytes!).toHaveLength(4);
  });

}
