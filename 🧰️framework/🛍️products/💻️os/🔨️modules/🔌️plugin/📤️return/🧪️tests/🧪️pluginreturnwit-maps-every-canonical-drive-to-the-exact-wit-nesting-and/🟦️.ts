type TestSource = { readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { encodePluginReturnResult, generation, pluginReturnDriveToWit } = dependencies;

  const { it, expect } = vitest;
  const hydrate = (value: unknown): any => JSON.parse(JSON.stringify(value), (key, item) => ["activationGeneration", "returnSequence", "pageSequence"].includes(key) ? BigInt(item) : item);
  const origin = (value: any) => ({ activationGeneration: value.activationGeneration, requestSequence: BigInt(value.requestSequence) });
  const identity = (value: any) => ({ origin: origin(value.origin), returnSequence: value.returnSequence });
  const receipt = (value: any) => ({ identity: identity(value.identity), pageSequence: value.pageSequence, length: value.length, final: value.final });
  const kebab = (value: string): string => value.replace(/[A-Z]/g, letter => "-" + letter.toLowerCase());
  const control = (value: any) => ({ tag: kebab(value.kind), val: value.kind === "inputAck" ? receipt(value.receipt) : identity(value.identity) });
  const result = (value: any): unknown => {
    switch (value.kind) {
      case "protocolFault": return { tag: "protocol-fault", val: kebab(value.fault) };
      case "refused": return { tag: value.kind, val: { origin: origin(value.origin), fault: kebab(value.fault) } };
      case "pending": return { tag: value.kind, val: { identity: identity(value.identity), reason: kebab(value.reason) } };
      case "retired": return { tag: value.kind, val: { identity: identity(value.identity), completion: value.completion } };
      case "control": return { tag: value.kind, val: { control: control(value.control), outcome: value.outcome, fault: kebab(value.fault) } };
      case "page": return { tag: value.kind, val: { receipt: receipt(value.receipt), page: value.page } };
      default: throw new Error("unknown fixture result");
    }
  };

  it("PluginReturnWit maps every canonical drive to the exact WIT nesting and u64 request", async () => {
    const api = await import("../../🟦️.ts");
    const { default: fixture } = await import("../../../../../../../🔨️modules/🎭️actor/📤️return/🧫️fixture/🔣️.json");
    const { decodeActorReturnDrive } = await import("../../../../../../../🔨️modules/🎭️actor/📤️return/🟦️.ts");
    const name = "@webassemblyjs/leb128/lib/leb.js";
    const module = await import(name);
    const encode = (module.default ?? module).encodeUIntBuffer;
    const uint = (value: bigint | number): Buffer => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(BigInt(value)); return Buffer.from(encode(bytes)); };
    for (const row of fixture.wireVectors) {
      const value = hydrate(row.value);
      const generation = value.kind === "execute" ? value.origin.activationGeneration : (value.control.kind === "inputAck" ? value.control.receipt.identity : value.control.identity).origin.activationGeneration;
      const expected = value.kind === "execute" ? { tag: "execute", val: origin(value.origin) } : { tag: "control", val: control(value.control) };
      expect(api.pluginReturnDriveToWit(value, generation)).toEqual(expected);
      expect(decodeActorReturnDrive(Uint8Array.from(Buffer.from(row.hex, "hex")))).toEqual(value);
      const ack = value.kind === "control" && value.control.kind === "inputAck" ? value.control.receipt : null;
      const id = value.kind === "execute" ? null : (ack ? ack.identity : value.control.identity);
      const authority = value.kind === "execute" ? value.origin : id.origin;
      const tags = value.kind === "execute" ? [0] : [1, fixture.controlTags[value.control.kind as keyof typeof fixture.controlTags]];
      const fields = [uint(authority.activationGeneration), uint(authority.requestSequence), ...(id ? [uint(id.returnSequence)] : []), ...(ack ? [uint(ack.pageSequence), uint(ack.length), Buffer.of(ack.final ? 1 : 0)] : [])];
      expect(Buffer.concat([Buffer.from(tags), ...fields]).toString("hex")).toBe(row.hex);
      expect(() => api.pluginReturnDriveToWit(value, generation === 1n ? 2n : 1n)).toThrow(/activation/);
    }
  });

  it("PluginReturnWit matches the shared fixed result vectors and exact enum subset", async () => {
    const api = await import("../../🟦️.ts");
    const { default: fixture } = await import("../../../../../../../🔨️modules/🎭️actor/📤️return/🧫️fixture/🔣️.json");
    const { default: schema } = await import("../../../../../../../🔨️modules/🎭️actor/📤️return/🧬️schema/🔣️.json");
    const { default: lifetimeSchema } = await import("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧬️schema/🔣️.json");
    const { default: pageSchema } = await import("../../../../../../../🔨️modules/🎭️actor/📃️page/🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv");
    const validate = new Ajv({ strict: true }).addSchema(lifetimeSchema).addSchema(pageSchema).addSchema(schema).getSchema("https://semio.tech/schema/framework/actor/return/schema.json#/$defs/Return")!;
    const { decodeActorReturnResult } = await import("../../../../../../../🔨️modules/🎭️actor/📤️return/🟦️.ts");
    for (const row of fixture.resultVectors) {
      expect(validate(row.value)).toBe(true);
      const value = hydrate(row.value);
      const authority = value.kind === "refused" ? value.origin : value.kind === "control" ? (value.control.kind === "inputAck" ? value.control.receipt.identity : value.control.identity).origin : value.kind === "protocolFault" ? null : value.identity.origin;
      const bytes = api.encodePluginReturnResult(result(value), authority?.activationGeneration ?? 1n);
      expect(Buffer.from(bytes).toString("hex")).toBe(row.hex);
      expect(decodeActorReturnResult(bytes)).toEqual(value);
    }
    for (const row of fixture.resultContradictions) expect(() => api.encodePluginReturnResult(result(hydrate(row)), 7n)).toThrow();
    for (const fault of fixture.resultEnums.fault) {
      const call = () => api.encodePluginReturnResult({ tag: "protocol-fault", val: kebab(fault) }, 7n);
      if (fault === "malformedControl" || fault === "mixedControl") expect(call()).toHaveLength(2);
      else expect(call).toThrow();
    }
    for (const tag of ["ok", "err", "protocolFault", "turn-result", "", null]) expect(() => api.encodePluginReturnResult({ tag, val: {} }, 7n)).toThrow();
  });

  it("PluginReturnWit actual mapping module has no strict semantic or syntactic TypeScript diagnostics", async () => {
    const { default: ts } = await import("typescript");
    const { fileURLToPath } = await import("node:url");
    const path = fileURLToPath(source.url);
    const program = ts.createProgram([path], { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, jsx: ts.JsxEmit.ReactJSX, strict: true, noEmit: true, skipLibCheck: true, allowImportingTsExtensions: true, resolveJsonModule: true, esModuleInterop: true, types: ["node", "vitest/importMeta"] });
    const sourceFile = program.getSourceFile(path);
    expect(sourceFile).toBeDefined();
    const diagnostics = [...program.getSyntacticDiagnostics(sourceFile), ...program.getSemanticDiagnostics(sourceFile)];
    expect(diagnostics.map(item => ts.flattenDiagnosticMessageText(item.messageText, "\n"))).toEqual([]);
  });

  it("PluginReturnWit preserves all neutral page words without interpreting semantic bytes", async () => {
    const api = await import("../../🟦️.ts");
    const { default: fixture } = await import("../../../../../../../🔨️modules/🎭️actor/📤️return/🧫️fixture/🔣️.json");
    const { createActorBytePage } = await import("../../../../../../../🔨️modules/🎭️actor/📃️page/🟦️.ts");
    for (const row of fixture.pageResultVectors) {
      const bytes = Buffer.alloc(fixture.maximumPageBytes);
      if (row.pattern === "mod37plus11") for (let index = 0; index < row.pageLength; index++) bytes[index] = (index * 37 + 11) % 256;
      const page = createActorBytePage(Uint8Array.from(bytes.subarray(0, row.pageLength)));
      const source = result({ kind: "page", receipt: hydrate(row.receipt), page });
      const encoded = api.encodePluginReturnResult(source, BigInt(row.receipt.identity.origin.activationGeneration));
      expect(Buffer.from(encoded)).toEqual(Buffer.concat([Buffer.from(row.prefixHex, "hex"), bytes]));
      expect(encoded.length).toBe(row.wireBytes);
      if (row.pageLength === 0) expect(() => api.encodePluginReturnResult(result({ kind: "page", receipt: hydrate(row.receipt), page: createActorBytePage(Uint8Array.of(1)) }), BigInt(row.receipt.identity.origin.activationGeneration))).toThrow();
    }
  });

  it("PluginReturnWit rejects unsafe WIT integers and accessors while leaving unknown roots owned by its caller", async () => {
    const api = await import("../../🟦️.ts");
    const { default: fixture } = await import("../../../../../../../🔨️modules/🎭️actor/📤️return/🧫️fixture/🔣️.json");
    const source: any = result(hydrate(fixture.resultVectors.find(row => row.value.kind === "refused")!.value));
    const generation = source.val.origin.activationGeneration;
    for (const value of [0n, -1n, 9007199254740992n, 1, "1", null, undefined]) expect(() => api.encodePluginReturnResult({ ...source, val: { ...source.val, origin: { ...source.val.origin, requestSequence: value } } }, generation)).toThrow();
    for (const value of [0n, -1n, 0x10000000000000000n, 1, "1", null, undefined]) expect(() => api.encodePluginReturnResult(source, value as bigint)).toThrow();
    let reads = 0;
    const unknown = { buffer: new Uint8Array(8192) };
    const original = { ...source, unknown, get extra() { reads++; throw new Error("unknown getter"); } };
    expect(api.encodePluginReturnResult(original, generation).length).toBeGreaterThan(0);
    expect(original.unknown).toBe(unknown); expect(unknown.buffer.byteLength).toBe(8192); expect(reads).toBe(0);
    const accessor = { get tag() { reads++; throw new Error("tag getter"); }, val: source.val };
    expect(() => api.encodePluginReturnResult(accessor, generation)).toThrow(/field/);
    expect(reads).toBe(0);
    expect(() => api.encodePluginReturnResult(source, generation === 1n ? 2n : 1n)).toThrow(/activation/);
  });

}
