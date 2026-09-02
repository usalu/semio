//#region 🧬️WitReturnContract
import { encodeActorReturnDrive, encodeActorReturnResult, type ActorReturnControl, type ActorReturnDrive, type ActorReturnFault, type ActorReturnIdentity, type ActorReturnOrigin, type ActorReturnPageReceipt, type ActorReturnResult } from "../../../../../🔨️modules/🎭️actor/📤️return/🟦️.ts";
import type { ActorBytePage } from "../../../../../🔨️modules/🎭️actor/📄️page/🟦️.ts";
type WitOrigin = { readonly activationGeneration: bigint; readonly requestSequence: bigint };
type WitIdentity = { readonly origin: WitOrigin; readonly returnSequence: bigint };
type WitPageReceipt = { readonly identity: WitIdentity; readonly pageSequence: bigint; readonly length: number; readonly final: boolean };
type WitControl = { readonly tag: "poll" | "cancel" | "retired-ack"; readonly val: WitIdentity } | { readonly tag: "input-ack"; readonly val: WitPageReceipt };
export type PluginReturnWitDrive = { readonly tag: "execute"; readonly val: WitOrigin } | { readonly tag: "control"; readonly val: WitControl };
function mappingFault(reason: string): never { throw new Error(`plugin-return.${reason}`); }
function generation(value: bigint): void { if (typeof value !== "bigint" || value <= 0n || value > 0xffffffffffffffffn) mappingFault("activation"); }
function field(value: unknown, name: string): unknown {
  if (value === null || typeof value !== "object") return mappingFault("field");
  const descriptor = Object.getOwnPropertyDescriptor(value, name);
  if (!descriptor || !("value" in descriptor)) return mappingFault("field");
  return descriptor.value;
}
function controlOrigin(value: ActorReturnControl): ActorReturnOrigin { return (value.kind === "inputAck" ? value.receipt.identity : value.identity).origin; }
function toOrigin(value: ActorReturnOrigin): WitOrigin { return Object.freeze({ activationGeneration: value.activationGeneration, requestSequence: BigInt(value.requestSequence) }); }
function toIdentity(value: ActorReturnIdentity): WitIdentity { return Object.freeze({ origin: toOrigin(value.origin), returnSequence: value.returnSequence }); }
function toReceipt(value: ActorReturnPageReceipt): WitPageReceipt { return Object.freeze({ identity: toIdentity(value.identity), pageSequence: value.pageSequence, length: value.length, final: value.final }); }
function toControl(value: ActorReturnControl): WitControl {
  if (value.kind === "inputAck") return Object.freeze({ tag: "input-ack", val: toReceipt(value.receipt) });
  return Object.freeze({ tag: value.kind === "retiredAck" ? "retired-ack" : value.kind, val: toIdentity(value.identity) });
}

/** 🧵️ Maps one validated canonical drive to the type-only WIT contract; the captured caller retains dispatch authority. */
export function pluginReturnDriveToWit(drive: ActorReturnDrive, activationGeneration: bigint): PluginReturnWitDrive {
  generation(activationGeneration);
  encodeActorReturnDrive(drive);
  const origin = drive.kind === "execute" ? drive.origin : controlOrigin(drive.control);
  if (origin.activationGeneration !== activationGeneration) mappingFault("activation-mismatch");
  return drive.kind === "execute" ? Object.freeze({ tag: "execute", val: toOrigin(drive.origin) }) : Object.freeze({ tag: "control", val: toControl(drive.control) });
}
//#endregion 🧬️WitReturnContract

//#region 📤️WitReturnEncoding
function fromOrigin(value: unknown, activation: bigint): ActorReturnOrigin {
  const requestSequence = field(value, "requestSequence");
  if (typeof requestSequence !== "bigint" || requestSequence <= 0n || requestSequence > BigInt(Number.MAX_SAFE_INTEGER)) return mappingFault("request-sequence");
  const activationGeneration = field(value, "activationGeneration");
  if (activationGeneration !== activation) return mappingFault("activation-mismatch");
  return { activationGeneration, requestSequence: Number(requestSequence) };
}
function fromIdentity(value: unknown, activation: bigint): ActorReturnIdentity { return { origin: fromOrigin(field(value, "origin"), activation), returnSequence: field(value, "returnSequence") as bigint }; }
function fromReceipt(value: unknown, activation: bigint): ActorReturnPageReceipt {
  return { identity: fromIdentity(field(value, "identity"), activation), pageSequence: field(value, "pageSequence") as bigint, length: field(value, "length") as number, final: field(value, "final") as boolean };
}
function fromControl(value: unknown, activation: bigint): ActorReturnControl {
  const tag = field(value, "tag"); const body = field(value, "val");
  switch (tag) {
    case "poll": return { kind: "poll", identity: fromIdentity(body, activation) };
    case "cancel": return { kind: "cancel", identity: fromIdentity(body, activation) };
    case "retired-ack": return { kind: "retiredAck", identity: fromIdentity(body, activation) };
    case "input-ack": return { kind: "inputAck", receipt: fromReceipt(body, activation) };
    default: return mappingFault("control-tag");
  }
}
function fromFault(value: unknown): ActorReturnFault {
  switch (value) {
    case "none": case "capacity": case "deadline": return value;
    case "sequence-exhausted": return "sequenceExhausted";
    case "stale-origin": return "staleOrigin";
    case "stale-identity": return "staleIdentity";
    case "wrong-page": return "wrongPage";
    case "input-not-retired": return "inputNotRetired";
    case "not-retired": return "notRetired";
    case "clock-unavailable": return "clockUnavailable";
    case "clock-backward": return "clockBackward";
    case "owner-fault": return "ownerFault";
    case "malformed-control": return "malformedControl";
    case "mixed-control": return "mixedControl";
    default: return mappingFault("fault-enum");
  }
}
function fromReason(value: unknown): Extract<ActorReturnResult, { kind: "pending" }>["reason"] {
  switch (value) {
    case "working": case "blocked": case "closing": return value;
    case "awaiting-input": return "awaitingInput";
    default: return mappingFault("pending-reason");
  }
}

/** 📮️ Encodes selected fixed WIT fields; the caller must retain the original response and pre-admit all format allocations. */
export function encodePluginReturnResult(original: unknown, activationGeneration: bigint): Uint8Array {
  generation(activationGeneration);
  const tag = field(original, "tag"); const body = field(original, "val");
  let result: ActorReturnResult;
  switch (tag) {
    case "protocol-fault": {
      const fault = fromFault(body);
      if (fault !== "malformedControl" && fault !== "mixedControl") return mappingFault("protocol-fault");
      result = { kind: "protocolFault", fault }; break;
    }
    case "refused": result = { kind: "refused", origin: fromOrigin(field(body, "origin"), activationGeneration), fault: fromFault(field(body, "fault")) }; break;
    case "pending": result = { kind: "pending", identity: fromIdentity(field(body, "identity"), activationGeneration), reason: fromReason(field(body, "reason")) }; break;
    case "page": result = { kind: "page", receipt: fromReceipt(field(body, "receipt"), activationGeneration), page: field(body, "page") as ActorBytePage }; break;
    case "retired": result = { kind: "retired", identity: fromIdentity(field(body, "identity"), activationGeneration), completion: field(body, "completion") as Extract<ActorReturnResult, { kind: "retired" }>["completion"] }; break;
    case "control": result = { kind: "control", control: fromControl(field(body, "control"), activationGeneration), outcome: field(body, "outcome") as Extract<ActorReturnResult, { kind: "control" }>["outcome"], fault: fromFault(field(body, "fault")) }; break;
    default: return mappingFault("result-tag");
  }
  return encodeActorReturnResult(result);
}
//#endregion 📤️WitReturnEncoding

//#region 🧪️WitReturnLaws
if (import.meta.vitest) {
  const { it, expect } = import.meta.vitest;
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
    const api = await import("./🟦️.ts");
    const { default: fixture } = await import("../../../../../🔨️modules/🎭️actor/📤️return/🧪️fixture.json");
    const { decodeActorReturnDrive } = await import("../../../../../🔨️modules/🎭️actor/📤️return/🟦️.ts");
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
    const api = await import("./🟦️.ts");
    const { default: fixture } = await import("../../../../../🔨️modules/🎭️actor/📤️return/🧪️fixture.json");
    const { default: schema } = await import("../../../../../🔨️modules/🎭️actor/📤️return/🧬️schema.json");
    const { default: lifetimeSchema } = await import("../../../../../🔨️modules/🎭️actor/🚪️lifetime/🧬️schema.json");
    const { default: pageSchema } = await import("../../../../../🔨️modules/🎭️actor/📄️page/🧬️schema.json");
    const { default: Ajv } = await import("ajv");
    const validate = new Ajv({ strict: true }).addSchema(lifetimeSchema).addSchema(pageSchema).addSchema(schema).getSchema("semio.actor.retained-return.v1#/definitions/result")!;
    const { decodeActorReturnResult } = await import("../../../../../🔨️modules/🎭️actor/📤️return/🟦️.ts");
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
    const path = fileURLToPath(import.meta.url);
    const program = ts.createProgram([path], { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, jsx: ts.JsxEmit.ReactJSX, strict: true, noEmit: true, skipLibCheck: true, allowImportingTsExtensions: true, resolveJsonModule: true, esModuleInterop: true, types: ["node", "vitest/importMeta"] });
    const source = program.getSourceFile(path);
    expect(source).toBeDefined();
    const diagnostics = [...program.getSyntacticDiagnostics(source), ...program.getSemanticDiagnostics(source)];
    expect(diagnostics.map(item => ts.flattenDiagnosticMessageText(item.messageText, "\n"))).toEqual([]);
  });

  it("PluginReturnWit preserves all neutral page words without interpreting semantic bytes", async () => {
    const api = await import("./🟦️.ts");
    const { default: fixture } = await import("../../../../../🔨️modules/🎭️actor/📤️return/🧪️fixture.json");
    const { createActorBytePage } = await import("../../../../../🔨️modules/🎭️actor/📄️page/🟦️.ts");
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
    const api = await import("./🟦️.ts");
    const { default: fixture } = await import("../../../../../🔨️modules/🎭️actor/📤️return/🧪️fixture.json");
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
//#endregion 🧪️WitReturnLaws
