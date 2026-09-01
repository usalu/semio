import type { ResidentCapacity } from "../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts";

type Counts<T extends number | bigint> = { readonly bytes: T; readonly slots: T; readonly owners: T };
type Capacity<T extends number | bigint> = Counts<T> & { readonly control: Counts<T> };
export type PluginPollCompositionWit = Capacity<bigint>;

function field(value: unknown, key: string): unknown {
  if (value === null || typeof value !== "object") throw new Error("plugin-composition.field");
  const descriptor = Object.getOwnPropertyDescriptor(value, key);
  if (!descriptor || !("value" in descriptor)) throw new Error("plugin-composition.field");
  return descriptor.value;
}
function wireCount(value: unknown): bigint {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0) throw new Error("plugin-composition.number");
  return BigInt(value);
}
function hostCount(value: unknown): number {
  if (typeof value !== "bigint" || value < 0n || value > 9007199254740991n) throw new Error("plugin-composition.u64");
  return Number(value);
}
function mapCapacity<T extends number | bigint>(value: unknown, scalar: (value: unknown) => T): Capacity<T> {
  const bytes = scalar(field(value, "bytes")); const slots = scalar(field(value, "slots")); const owners = scalar(field(value, "owners")); const source = field(value, "control");
  const control = { bytes: scalar(field(source, "bytes")), slots: scalar(field(source, "slots")), owners: scalar(field(source, "owners")) };
  if (control.bytes > bytes || control.slots > slots || control.owners > owners) throw new Error("plugin-composition.partition");
  return Object.freeze({ bytes, slots, owners, control: Object.freeze(control) });
}

/** 🏘️ Maps the existing required poll field; original owner, raw roots and allocation admission remain with the caller. */
export function pluginPollCompositionToWit(value: unknown): PluginPollCompositionWit { return mapCapacity(value, wireCount); }

/** 🏘️ Checks WIT u64 values before host conversion; equal configuration never certifies private composition identity. */
export function pluginPollCompositionFromWit(value: unknown): ResidentCapacity { return mapCapacity(value, hostCount); }

//#region 🧪️CompositionMapping
if (import.meta.vitest) {
  const { it, expect } = import.meta.vitest;

  it("PluginPollCompositionWit preserves the canonical six scalars and exact nested field names", async () => {
    const api = await import("./🟦️component.ts"); const { default: fixture } = await import("../../../../../../🔨️modules/🎠️kernel/📥️poll/🏘️composition/🧪️fixture/🔣️.json"); const { default: contract } = await import("../../../../../../🔨️modules/🎠️kernel/📥️poll/🏘️composition/🧬️contract/🔣️.json");
    const { default: schema } = await import("../../../../../../🔨️modules/🎠️kernel/📥️poll/🏘️composition/🧬️schema/🔣️.json"); const { default: fixtureSchema } = await import("../../../../../../🔨️modules/🎠️kernel/📥️poll/🏘️composition/🧪️schema/🔣️.json"); const { default: capacitySchema } = await import("../../../../../../🔨️modules/🌱️value/💾️resident/🧬️schema.json");
    const { default: Ajv } = await import("ajv"); const library = "lodash"; const { default: _ } = await import(library); const { Buffer } = await import("node:buffer");
    const oracle = new Ajv({ strict: true }).addSchema(capacitySchema).addSchema(schema); expect(oracle.compile(fixtureSchema)(fixture)).toBe(true);
    for (const row of fixture.valid) {
      const bytes = Buffer.alloc(48); const expected = {};
      contract.wireOrder.forEach((path, index) => { bytes.writeBigUInt64LE(BigInt(_.get(row.input.composition, path)), index * 8); _.set(expected, path, bytes.readBigUInt64LE(index * 8)); });
      expect(Array.from({ length: 6 }, (_, index) => bytes.readBigUInt64LE(index * 8).toString())).toEqual(row.words);
      const wire = api.pluginPollCompositionToWit(row.input.composition); expect(wire).toEqual(expected); expect(api.pluginPollCompositionFromWit(wire)).toEqual(row.input.composition);
      expect(Object.isFrozen(wire)).toBe(true); expect(Object.isFrozen(wire.control)).toBe(true); expect(Object.keys(wire)).toEqual(["bytes", "slots", "owners", "control"]);
      expect(Object.keys(wire.control)).toEqual(["bytes", "slots", "owners"]); expect("identity" in wire).toBe(false);
    }
    const validate = oracle.getSchema("semio.kernel.poll.composition.v1")!;
    for (const row of fixture.invalid) { expect(validate(row.input)).toBe(false); expect(() => api.pluginPollCompositionToWit(_.get(row.input, "composition"))).toThrow(/composition/); }
    for (const row of fixture.partitionRefusals) { expect(() => api.pluginPollCompositionToWit(row)).toThrow(/partition/); const wire = _.cloneDeep(row); for (const path of contract.wireOrder) _.set(wire, path, BigInt(_.get(row, path))); expect(() => api.pluginPollCompositionFromWit(wire)).toThrow(/partition/); }
  });

  it("PluginPollCompositionWit checks every WIT u64 before conversion and keeps number and bigint dialects distinct", async () => {
    const api = await import("./🟦️component.ts"); const { default: fixture } = await import("../../../../../../🔨️modules/🎠️kernel/📥️poll/🏘️composition/🧪️fixture/🔣️.json"); const { default: contract } = await import("../../../../../../🔨️modules/🎠️kernel/📥️poll/🏘️composition/🧬️contract/🔣️.json"); const library = "lodash"; const { default: _ } = await import(library);
    const original = fixture.valid[1]!.input.composition; const wire = api.pluginPollCompositionToWit(original);
    for (const path of contract.wireOrder) {
      for (const invalid of [-1n, BigInt(contract.scalar.maximum) + 1n, 0x10000000000000000n, 1, "1", null, undefined]) { const value = _.cloneDeep(wire); _.set(value, path, invalid); expect(() => api.pluginPollCompositionFromWit(value)).toThrow(/composition/); }
      for (const invalid of [-1, 0.5, contract.scalar.maximum + 1, Infinity, NaN, 1n, "1", null, undefined]) { const value = _.cloneDeep(original); _.set(value, path, invalid); expect(() => api.pluginPollCompositionToWit(value)).toThrow(/composition/); }
    }
  });

  it("PluginPollCompositionWit reads only own data fields without claiming unknown-root retirement or private owner authority", async () => {
    const api = await import("./🟦️component.ts"); const { default: fixture } = await import("../../../../../../🔨️modules/🎠️kernel/📥️poll/🏘️composition/🧪️fixture/🔣️.json"); const original = fixture.valid[1]!.input.composition; let reads = 0;
    const wrapper = { ...original, payload: new Uint8Array(8193), get extra() { reads++; throw new Error("unread"); } };
    expect(api.pluginPollCompositionToWit(wrapper)).toEqual(api.pluginPollCompositionToWit(original)); expect(wrapper.payload.byteLength).toBe(8193); expect(reads).toBe(0);
    for (const key of ["bytes", "slots", "owners", "control"]) { const accessor = { ...original }; Object.defineProperty(accessor, key, { get() { reads++; throw new Error("accessor"); } }); expect(() => api.pluginPollCompositionToWit(accessor)).toThrow(/field/); }
    expect(() => api.pluginPollCompositionToWit(Object.create(original))).toThrow(/field/); expect(reads).toBe(0);
    const same = api.pluginPollCompositionToWit({ ...original, control: { ...original.control } }); expect(same).toEqual(api.pluginPollCompositionToWit(original)); expect(Object.keys(same)).not.toContain("owner");
  });

  it("PluginPollCompositionWit actual module passes strict TypeScript diagnostics", async () => {
    const { default: ts } = await import("typescript"); const { fileURLToPath } = await import("node:url"); const path = fileURLToPath(import.meta.url);
    const program = ts.createProgram([path], { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, strict: true, noEmit: true, skipLibCheck: true, allowImportingTsExtensions: true, resolveJsonModule: true, esModuleInterop: true, types: ["node", "vitest/importMeta"] }); const source = program.getSourceFile(path); expect(source).toBeDefined();
    expect([...program.getSyntacticDiagnostics(source), ...program.getSemanticDiagnostics(source)].map(item => ts.flattenDiagnosticMessageText(item.messageText, "\n"))).toEqual([]);
  });
}
//#endregion 🧪️CompositionMapping
