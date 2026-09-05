//#region 📨️ReturnResponseContract
import { ACTOR_RETURN_RESULT_MAXIMUM_BYTES, ActorReturnResultFraming, encodeActorReturnResult, type ActorReturnResult, type ActorReturnResultProjection } from "../🟦️.ts";
import { createActorBytePage } from "../../📃️page/🟦️.ts";
export type ActorReturnTransportFault = "requestRefused" | "executionFault" | "resultFault";
type ResponseAuthority = { readonly activationGeneration: bigint; readonly transportRequestSequence: number };
export type ActorReturnResponseHeader = ResponseAuthority & { readonly kind: "result" | "fault" };
export type ActorReturnResponse = ResponseAuthority & ({ readonly kind: "result"; readonly result: ActorReturnResult } | { readonly kind: "fault"; readonly fault: ActorReturnTransportFault });
export type ActorReturnResponseProjection = ResponseAuthority & ({ readonly kind: "result"; readonly result: ActorReturnResultProjection } | { readonly kind: "fault"; readonly fault: ActorReturnTransportFault });
export const ACTOR_RETURN_RESPONSE_MAXIMUM_BYTES = 23 + ACTOR_RETURN_RESULT_MAXIMUM_BYTES;
const MAGIC = [0x73, 0x72, 0x72, 1] as const;
const FAULTS = ["requestRefused", "executionFault", "resultFault"] as const;
const MAX_U64 = 0xffffffffffffffffn;
const MAX_REQUEST = BigInt(Number.MAX_SAFE_INTEGER);
const bufferLength = Object.getOwnPropertyDescriptor(ArrayBuffer.prototype, "byteLength")!.get!;
const bufferResizable = Object.getOwnPropertyDescriptor(ArrayBuffer.prototype, "resizable")?.get;
function fault(reason: string): never { throw new Error(`actor-return-response.${reason}`); }
function uint(value: bigint, maximum: bigint): void { if (typeof value !== "bigint" || value <= 0n || value > maximum) fault("authority"); }
function sameActivation(result: ActorReturnResult | ActorReturnResultProjection, activation: bigint): void {
  let generation: bigint;
  switch (result.kind) {
    case "protocolFault": return;
    case "refused": generation = result.origin.activationGeneration; break;
    case "pending": case "retired": generation = result.identity.origin.activationGeneration; break;
    case "page": generation = result.receipt.identity.origin.activationGeneration; break;
    case "control": generation = (result.control.kind === "inputAck" ? result.control.receipt.identity : result.control.identity).origin.activationGeneration; break;
    default: return fault("result");
  }
  if (generation !== activation) fault("activation-mismatch");
}
//#endregion 📨️ReturnResponseContract

//#region 🔣️ResponseEncoding
/** 🔣️ Encodes a fixed response backing; the caller retains admission and transfer ownership. */
export function encodeActorReturnResponse(value: ActorReturnResponse): ArrayBuffer {
  uint(value.activationGeneration, MAX_U64);
  if (!Number.isSafeInteger(value.transportRequestSequence) || value.transportRequestSequence <= 0) fault("transport-request");
  if (value.kind !== "result" && value.kind !== "fault") fault("tag");
  const header = new Uint8Array(23); header.set(MAGIC); header[4] = value.kind === "result" ? 0 : 1;
  let offset = 5;
  for (const field of [value.activationGeneration, BigInt(value.transportRequestSequence)]) {
    let remainder = field;
    do { const byte = Number(remainder & 127n); remainder >>= 7n; header[offset++] = byte | (remainder ? 128 : 0); } while (remainder);
  }
  let body: Uint8Array;
  if (value.kind === "result") { sameActivation(value.result, value.activationGeneration); body = encodeActorReturnResult(value.result); }
  else { const tag = FAULTS.indexOf(value.fault); if (tag < 0) fault("transport-fault"); body = Uint8Array.of(tag); }
  if (offset + body.length > ACTOR_RETURN_RESPONSE_MAXIMUM_BYTES) fault("length");
  const result = new ArrayBuffer(offset + body.length); const bytes = new Uint8Array(result);
  bytes.set(header.subarray(0, offset)); bytes.set(body, offset); return result;
}
//#endregion 🔣️ResponseEncoding

//#region 📐️ResponseFraming
/** 📐️ Validates one response byte per call; projections contain no payload storage or custody authority. */
export class ActorReturnResponseFraming {
  #stage: "magic" | "tag" | "activation" | "request" | "body" | "fault" | "done" = "magic";
  #offset = 0;
  #bodyOffset = 0;
  #tag = -1;
  #accumulator = 0n;
  #digits = 0;
  #activation = 0n;
  #header: ActorReturnResponseHeader | null = null;
  #result: ActorReturnResultFraming | null = null;
  #faultValue: ActorReturnTransportFault | null = null;
  #failed = false;
  #value: ActorReturnResponseProjection | null = null;
  get header(): ActorReturnResponseHeader | null { return this.#header; }
  get value(): ActorReturnResponseProjection | null { return this.#value; }
  #fail(): never { this.#failed = true; this.#value = null; return fault("framing"); }
  push(byte: number): void {
    if (this.#failed || this.#value !== null || this.#stage === "done" || !Number.isInteger(byte) || byte < 0 || byte > 255 || this.#offset === ACTOR_RETURN_RESPONSE_MAXIMUM_BYTES) this.#fail();
    this.#offset++;
    switch (this.#stage) {
      case "magic":
        if (byte !== MAGIC[this.#offset - 1]) this.#fail();
        if (this.#offset === MAGIC.length) this.#stage = "tag"; return;
      case "tag": if (byte > 1) this.#fail(); this.#tag = byte; this.#stage = "activation"; return;
      case "body":
        try { (this.#result ??= new ActorReturnResultFraming()).push(byte); } catch { this.#fail(); } return;
      case "fault": this.#faultValue = FAULTS[byte] ?? null; if (this.#faultValue === null) this.#fail(); this.#stage = "done"; return;
      default: this.#uint(byte); return;
    }
  }
  #uint(byte: number): void {
    const activation = this.#stage === "activation"; const limit = activation ? 10 : 8;
    if (this.#digits >= limit || this.#digits === 9 && byte > 1) this.#fail();
    this.#accumulator |= BigInt(byte & 127) << BigInt(this.#digits * 7); this.#digits++;
    if (byte & 128) { if (this.#digits === limit) this.#fail(); return; }
    const value = this.#accumulator;
    if (this.#digits > 1 && byte === 0 || value <= 0n || value > (activation ? MAX_U64 : MAX_REQUEST)) this.#fail();
    this.#accumulator = 0n; this.#digits = 0;
    if (activation) { this.#activation = value; this.#stage = "request"; return; }
    this.#header = Object.freeze({ kind: this.#tag === 0 ? "result" : "fault", activationGeneration: this.#activation, transportRequestSequence: Number(value) });
    this.#bodyOffset = this.#offset; this.#stage = this.#tag === 0 ? "body" : "fault";
  }
  finish(): ActorReturnResponseProjection {
    if (this.#failed || this.#header === null) this.#fail();
    if (this.#value !== null) return this.#value;
    const { activationGeneration, transportRequestSequence } = this.#header;
    if (this.#stage === "done" && this.#faultValue !== null) return this.#value = Object.freeze({ kind: "fault", activationGeneration, transportRequestSequence, fault: this.#faultValue });
    if (this.#stage !== "body" || this.#result === null) this.#fail();
    try {
      const result = this.#result.finish(); sameActivation(result, activationGeneration);
      const projection = result.kind === "page" ? Object.freeze({ kind: "page" as const, receipt: result.receipt, payloadOffset: this.#bodyOffset + result.payloadOffset }) : result;
      return this.#value = Object.freeze({ kind: "result", activationGeneration, transportRequestSequence, result: projection });
    } catch { return this.#fail(); }
  }
}
//#endregion 📐️ResponseFraming

//#region 📭️ResponseDecoding
function responseBytes(backing: unknown): Uint8Array {
  let length: number;
  try {
    length = bufferLength.call(backing);
    if (!bufferResizable || bufferResizable.call(backing)) return fault("resizable-backing");
  } catch { return fault("backing"); }
  if (length < 8 || length > ACTOR_RETURN_RESPONSE_MAXIMUM_BYTES) fault("length");
  return new Uint8Array(backing as ArrayBuffer);
}
/** 📬️ Inspects at most 23 header bytes for routing; the original backing and body remain unvalidated owned input. */
export function readActorReturnResponseHeader(backing: unknown): ActorReturnResponseHeader {
  const bytes = responseBytes(backing); const parser = new ActorReturnResponseFraming();
  for (const byte of bytes) { parser.push(byte); if (parser.header !== null) return parser.header; }
  return fault("truncated");
}
/** 📭️ Validates entire fixed backing and canonical fields, without minting captured producer or retirement authority. */
export function decodeActorReturnResponse(backing: unknown): ActorReturnResponse {
  const bytes = responseBytes(backing); const parser = new ActorReturnResponseFraming(); for (const byte of bytes) parser.push(byte);
  const value = parser.finish(); if (value.kind === "fault") return value;
  const result = value.result;
  if (result.kind !== "page") return Object.freeze({ kind: "result", activationGeneration: value.activationGeneration, transportRequestSequence: value.transportRequestSequence, result });
  const page = createActorBytePage(bytes.subarray(result.payloadOffset, result.payloadOffset + result.receipt.length));
  return Object.freeze({ kind: "result", activationGeneration: value.activationGeneration, transportRequestSequence: value.transportRequestSequence, result: Object.freeze({ kind: "page", receipt: result.receipt, page }) });
}
//#endregion 📭️ResponseDecoding

//#region 🧪️ReturnResponseLaws
if (import.meta.vitest) {
  const { it, expect, vi } = import.meta.vitest;
  const hydrate = (value: unknown): any => JSON.parse(JSON.stringify(value), (key, item) => ["activationGeneration", "returnSequence", "pageSequence"].includes(key) ? BigInt(item) : item);
  const backing = (hex: string): ArrayBuffer => Uint8Array.from(Buffer.from(hex, "hex")).buffer;
  const oracle = async () => {
    const name = "@webassemblyjs/leb128/lib/leb.js"; const lib = await import(name); const encode = (lib.default ?? lib).encodeUIntBuffer;
    return (value: number | bigint): Buffer => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(BigInt(value)); return Buffer.from(encode(bytes)); };
  };
  it("ActorReturnResponseFraming uses canonical vectors with no payload copies or backing escape", async () => {
    const api = await import("./🟦️.ts");
    const { default: schema } = await import("./🌿️framing/🧬️schema.json"); const { default: framing } = await import("./🌿️framing/🧪️fixture/🔣️.json");
    const { default: fixture } = await import("./🧪️fixture/🔣️.json"); const { default: returned } = await import("../🧫️fixture/🔣️.json");
    const { default: returnedSchema } = await import("../🧬️schema.json"); const { default: lifetime } = await import("../../🚪️lifetime/🧬️schema.json"); const { default: page } = await import("../../📃️page/🧬️schema.json");
    const { default: Ajv } = await import("ajv"); const uint = await oracle();
    const ajv = new Ajv({ strict: true }).addSchema(lifetime).addSchema(page).addSchema(returnedSchema).addSchema(schema);
    expect(ajv.validate(schema, framing)).toBe(true);
    const cases = fixture.vectors.map(row => ({ bytes: Buffer.from(row.hex, "hex"), expected: hydrate(row.value) }));
    for (const row of returned.pageResultVectors) {
      const origin = row.receipt.identity.origin;
      const header = Buffer.concat([Buffer.from(fixture.magicHex, "hex"), Buffer.of(0), uint(BigInt(origin.activationGeneration)), uint(origin.requestSequence)]);
      const payload = Buffer.alloc(4096); for (let index = 0; index < row.pageLength; index++) payload[index] = (index * 37 + 11) % 256;
      const prefix = Buffer.from(row.prefixHex, "hex"); const result = { kind: "page", receipt: row.receipt, payloadOffset: header.length + prefix.length };
      expect(ajv.validate({ $ref: schema.$id + "#/definitions/pageProjection" }, result)).toBe(true);
      cases.push({ bytes: Buffer.concat([header, prefix, payload]), expected: hydrate({ kind: "result", activationGeneration: origin.activationGeneration, transportRequestSequence: origin.requestSequence, result }) });
    }
    for (const row of cases) {
      const parser = new api.ActorReturnResponseFraming(); let allocations = 0; let header = null; let value;
      expect(parser.header).toBeNull(); expect(parser.value).toBeNull();
      try {
        for (const name of ["ArrayBuffer", "Uint8Array", "BigUint64Array"] as const) vi.stubGlobal(name, new Proxy(globalThis[name], { construct(target, args) { allocations++; return Reflect.construct(target, args); } }));
        for (const byte of row.bytes) { parser.push(byte); if (parser.header !== null) { if (header !== null && header !== parser.header) throw new Error("Framing replaced stable header"); header = parser.header; } }
        value = parser.finish();
      } finally { vi.unstubAllGlobals(); }
      expect(allocations).toBe(framing.maximumPayloadCopies); expect(value).toEqual(row.expected); expect(parser.finish()).toBe(value); expect(parser.value).toBe(value);
      expect(Object.isFrozen(value)).toBe(true); expect(Object.isFrozen(header)).toBe(true); expect(Object.keys(parser)).toEqual([]);
      const { kind, activationGeneration, transportRequestSequence } = row.expected;
      expect(header).toEqual({ kind, activationGeneration, transportRequestSequence });
      if (value.kind === "result" && value.result.kind === "page") {
        expect(Object.keys(value.result).sort()).toEqual(["kind", "payloadOffset", "receipt"]);
        expect(row.bytes.length - value.result.payloadOffset).toBe(4096); expect(Object.isFrozen(value.result)).toBe(true);
      }
    }
  });
  it("ActorReturnResponseFraming keeps malformed bodies and incomplete authority failed", async () => {
    const api = await import("./🟦️.ts"); const { default: fixture } = await import("./🧪️fixture/🔣️.json");
    const { default: returned } = await import("../🧫️fixture/🔣️.json");
    const reject = (bytes: readonly number[]) => {
      const parser = new api.ActorReturnResponseFraming();
      expect(() => { for (const byte of bytes) parser.push(byte); parser.finish(); }).toThrow();
      expect(parser.value).toBeNull(); expect(() => parser.finish()).toThrow(); expect(() => parser.push(0)).toThrow();
    };
    for (const row of fixture.vectors) {
      const bytes = Buffer.from(row.hex, "hex");
      for (let length = 0; length < bytes.length; length++) reject([...bytes.subarray(0, length)]);
      reject([...bytes, 0]);
    }
    for (const hex of fixture.invalidHex) reject([...Buffer.from(hex, "hex")]);
    for (const value of [-1, 256, 1.5, NaN, Infinity]) reject([value]);
    const emptyPage = Buffer.concat([Buffer.from(fixture.vectors[0]!.hex.slice(0, 14) + returned.pageResultVectors[0]!.prefixHex, "hex"), Buffer.alloc(4096)]);
    emptyPage[emptyPage.length - 1] = 1; reject([...emptyPage]);
    const finished = new api.ActorReturnResponseFraming(); for (const byte of Buffer.from(fixture.vectors[0]!.hex, "hex")) finished.push(byte);
    finished.finish(); expect(() => finished.push(0)).toThrow(); expect(finished.value).toBeNull(); expect(() => finished.finish()).toThrow();
    for (const row of fixture.headerCases) {
      const parser = new api.ActorReturnResponseFraming(); const bytes = Buffer.from(row.hex, "hex"); let offset = 0;
      while (parser.header === null && offset < bytes.length) parser.push(bytes[offset++]!);
      expect(parser.header).toEqual(hydrate(row.header)); expect(offset).toBeLessThanOrEqual(23); expect(parser.value).toBeNull();
      expect(() => { while (offset < bytes.length) parser.push(bytes[offset++]!); parser.finish(); }).toThrow(); expect(parser.value).toBeNull();
    }
  });
  it("ActorReturnResponseFraming actual module has no strict TypeScript diagnostics", async () => {
    const { default: ts } = await import("typescript"); const { fileURLToPath } = await import("node:url"); const path = fileURLToPath(import.meta.url);
    const program = ts.createProgram([path], { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, strict: true, noEmit: true, skipLibCheck: true, allowImportingTsExtensions: true, resolveJsonModule: true, esModuleInterop: true, types: ["node", "vitest/importMeta"] });
    const source = program.getSourceFile(path); expect(source).toBeDefined();
    expect([...program.getSyntacticDiagnostics(source), ...program.getSemanticDiagnostics(source)].map(item => ts.flattenDiagnosticMessageText(item.messageText, "\n"))).toEqual([]);
  });
  it("ActorWorkerInboxInventory binds mixed message kinds and logical shells to current source", async () => {
    const { default: schema } = await import("./🎟️credit/📋️metadata/📥️inbox/🧬️schema.json"); const { default: fixture } = await import("./🎟️credit/📋️metadata/📥️inbox/🧪️fixture/🔣️.json");
    const { default: Ajv } = await import("ajv"); const { default: ts } = await import("typescript"); const { readFileSync } = await import("node:fs");
    const validate = new Ajv({ strict: true }).compile(schema); expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const shardPath = new URL("../../📮️shard-client/🟦️.ts", import.meta.url);
    const shard = ts.createSourceFile(shardPath.pathname, readFileSync(shardPath, "utf8"), ts.ScriptTarget.Latest, true);
    const totals = new Map<string, { bytes: bigint; slots: bigint; owners: bigint }>();
    for (const row of fixture.layouts) {
      const declaration = shard.statements.find(node => (ts.isClassDeclaration(node) || ts.isTypeAliasDeclaration(node)) && node.name?.text === row.declaration);
      const members = declaration && ts.isClassDeclaration(declaration) ? declaration.members.filter(ts.isPropertyDeclaration) : declaration && ts.isTypeAliasDeclaration(declaration) && ts.isTypeLiteralNode(declaration.type) ? declaration.type.members : null;
      if (!members) throw new Error("Missing inbox source " + row.declaration);
      const fields = members.map(member => member.name?.getText(shard).replace(/^#/, "")); expect(fields, row.declaration).toEqual(row.fields);
      const previous = totals.get(row.group) ?? { bytes: 0n, slots: 0n, owners: 0n };
      totals.set(row.group, { bytes: previous.bytes + BigInt(fixture.model.recordBytes) + BigInt(fields.length) * BigInt(fixture.model.fieldBytes), slots: previous.slots + 1n, owners: previous.owners + 1n });
    }
    for (const [group, expected] of Object.entries(fixture.minimumShells)) expect(Object.fromEntries(Object.entries(totals.get(group)!).map(([axis, count]) => [axis, Number(count)]))).toEqual(expected);
    const variants = (name: string) => {
      const declaration = shard.statements.find(node => ts.isTypeAliasDeclaration(node) && node.name.text === name);
      if (!declaration || !ts.isTypeAliasDeclaration(declaration) || !ts.isUnionTypeNode(declaration.type)) throw new Error("Missing inbox union " + name);
      return declaration.type.types.map(type => {
        if (!ts.isTypeLiteralNode(type)) throw new Error("Nonliteral inbox variant");
        const properties = type.members.filter(ts.isPropertySignature); const kind = properties.find(item => item.name.getText(shard) === "kind")?.type; const ok = properties.find(item => item.name.getText(shard) === "ok")?.type;
        if (!kind || !ts.isLiteralTypeNode(kind) || !ts.isStringLiteral(kind.literal)) throw new Error("Missing inbox tag");
        return { kind: kind.literal.text, ok: ok && ts.isLiteralTypeNode(ok) ? ok.literal.kind === ts.SyntaxKind.TrueKeyword : null, fields: properties.map(item => item.name.getText(shard)) };
      });
    };
    expect(variants("OutboundMessage")).toEqual(fixture.outbound); expect(variants("InboundMessage")).toEqual(fixture.inbound);
    const producerPath = new URL("../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts", import.meta.url);
    const producer = ts.createSourceFile(producerPath.pathname, readFileSync(producerPath, "utf8"), ts.ScriptTarget.Latest, true);
    const producers = await import("../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts");
    // 🫀️ Inventoried from the EMITTED bytes, not the template's raw text: `shardWorkerSource` now
    // interpolates the schema-owned liveness policy, so its return is a template EXPRESSION and its
    // literal text no longer is the worker. The AST check still fences the shape (one returned
    // template, nothing else), the inventory reads what the browser actually gets.
    const generated = (name: "shardWorkerSource" | "hostShimSource") => {
      const declaration = producer.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === name);
      const returned = declaration && ts.isFunctionDeclaration(declaration) ? declaration.body?.statements.find(ts.isReturnStatement)?.expression : null;
      if (!returned || !(ts.isNoSubstitutionTemplateLiteral(returned) || ts.isTemplateExpression(returned))) throw new Error("Changed generated inbox source " + name);
      return ts.createSourceFile(name + ".js", producers[name](), ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
    };
    const worker = generated("shardWorkerSource"); const shim = generated("hostShimSource"); const requests: string[] = []; const replies: string[] = []; const effects: string[] = []; const effectReplies: string[] = [];
    const visitWorker = (node: import("typescript").Node): void => {
      if (ts.isFunctionDeclaration(node) && node.name?.text === "deliverEffectResult") {
        const visitReply = (child: import("typescript").Node): void => { if (ts.isBinaryExpression(child) && child.left.getText(worker) === "kind" && ts.isStringLiteral(child.right)) effectReplies.push(child.right.text); ts.forEachChild(child, visitReply); };
        ts.forEachChild(node, visitReply); return;
      }
      if (ts.isBinaryExpression(node) && node.left.getText(worker) === "kind" && ts.isStringLiteral(node.right) && !requests.includes(node.right.text)) requests.push(node.right.text);
      if (ts.isSwitchStatement(node) && node.expression.getText(worker) === "kind") for (const clause of node.caseBlock.clauses) if (ts.isCaseClause(clause) && ts.isStringLiteral(clause.expression) && !requests.includes(clause.expression.text)) requests.push(clause.expression.text);
      if (ts.isCallExpression(node) && node.expression.getText(worker) === "self.postMessage" && node.arguments[0] && ts.isObjectLiteralExpression(node.arguments[0])) {
        const tag = node.arguments[0].properties.find(item => ts.isPropertyAssignment(item) && item.name.getText(worker) === "kind");
        if (!tag || !ts.isPropertyAssignment(tag) || !ts.isStringLiteral(tag.initializer)) throw new Error("Uninventoried worker post"); replies.push(tag.initializer.text);
      }
      ts.forEachChild(node, visitWorker);
    };
    const visitShim = (node: import("typescript").Node): void => {
      if (ts.isCallExpression(node) && ["call", "effectRequest"].includes(node.expression.getText(shim)) && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) effects.push(node.arguments[0].text);
      ts.forEachChild(node, visitShim);
    };
    visitWorker(worker); visitShim(shim); expect(requests).toEqual(fixture.workerRequestKinds); expect(replies).toEqual(fixture.workerReplyKinds); expect(effects).toEqual(fixture.hostEffectKinds); expect(effectReplies).toEqual(fixture.hostReplyKinds);
    const client = shard.statements.find(node => ts.isClassDeclaration(node) && node.name?.text === "ShardClient");
    const handler = client && ts.isClassDeclaration(client) ? client.members.find(node => ts.isMethodDeclaration(node) && node.name.getText(shard) === "handleMessage")?.getText(shard) : null;
    expect(handler).toBeTruthy(); expect(handler!.indexOf("message.kind")).toBeLessThan(handler!.indexOf("captureResponse(message)")); expect(handler!.indexOf("this.pending.get(message.requestId)")).toBeLessThan(handler!.indexOf("captureResponse(message)"));
    expect(worker.text.includes("msg.returnDrive")).toBe(false);
  });
  it("ActorWorkerInboxInventory executes generated heartbeat, ordinary reply and awaited effect traffic together", async () => {
    const { default: fixture } = await import("./🎟️credit/📋️metadata/📥️inbox/🧪️fixture/🔣️.json"); const { default: ts } = await import("typescript"); const { readFileSync } = await import("node:fs"); const vm = await import("node:vm");
    const path = new URL("../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts", import.meta.url);
    const source = ts.createSourceFile(path.pathname, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
    const producers = await import("../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts");
    const generated = (name: "shardWorkerSource" | "hostShimSource") => {
      const declaration = source.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === name);
      const returned = declaration && ts.isFunctionDeclaration(declaration) ? declaration.body?.statements.find(ts.isReturnStatement)?.expression : null;
      if (!returned || !(ts.isNoSubstitutionTemplateLiteral(returned) || ts.isTemplateExpression(returned))) throw new Error("Changed generated inbox source " + name); return producers[name]();
    };
    const messages: any[] = []; const receive = (message: unknown) => messages.push(message);
    const shim = vm.createContext({ exports: {}, URL, self: { postMessage: receive } });
    const shimCode = generated("hostShimSource").replace("import.meta.url", JSON.stringify("https://fixture.invalid/host.js?actor=a&activation=1"));
    new vm.Script(ts.transpileModule(shimCode, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS } }).outputText).runInContext(shim);
    let dispatch: ((event: { data: Record<string, unknown> }) => Promise<void>) | null = null;
    const checkpoint = Object.freeze({ ordinary: "checkpoint" }); const effectResult = Object.freeze({ effect: "completed" });
    const context = vm.createContext({ WebAssembly: { Suspending: class {}, promising: (value: unknown) => value }, self: { postMessage: receive, addEventListener: (kind: string, callback: typeof dispatch) => { if (kind === "message") dispatch = callback; } }, apiA: { poll: () => shim.exports.storageRead({ key: "awaited" }), resolveEffect: shim.exports.__resolveEffect, rejectEffect: shim.exports.__rejectEffect }, apiB: { checkpoint: async () => checkpoint } });
    new vm.Script(generated("shardWorkerSource")).runInContext(context);
    new vm.Script('actors.set("a", { api: apiA, activationGeneration: 1n, pendingAssets: [] }); actors.set("b", { api: apiB, activationGeneration: 2n, pendingAssets: [] });').runInContext(context);
    if (!dispatch) throw new Error("Missing generated worker dispatcher");
    const send = dispatch as (event: { data: Record<string, unknown> }) => Promise<void>;
    const pending = send({ data: { kind: "turn", requestId: "r1", actorId: "a", activationGeneration: 1n, events: [], budget: {} } });
    expect(messages.map(message => message.kind)).toEqual(["heartbeat", "frame"]);
    const effectRequest = messages[1].frame.envelope.payload.payload; expect(effectRequest.effect).toBe("storage-read");
    await send({ data: { kind: "checkpoint", requestId: "r2", actorId: "b" } });
    expect(messages.find(message => message.requestId === "r2")?.value).toBe(checkpoint); expect(messages.some(message => message.requestId === "r1")).toBe(false);
    shim.exports.emit({ notification: true }); shim.exports.emitPatch({ patch: true });
    await send({ data: { kind: "frame", actorId: "a", activationGeneration: 1n, frame: { kind: "Envelope", envelope: { to: "a", from: { kind: "kernel" }, payload: { kind: "effect-complete", payload: { requestId: effectRequest.requestId, value: effectResult } } } } } });
    await pending;
    const trace = messages.map(message => message.kind === "heartbeat" ? `heartbeat:${message.turnSeq}` : message.kind === "frame" ? `frame:${message.frame.envelope.payload.kind}` : `result:${message.requestId}:${message.ok ? "success" : "fault"}`);
    expect(trace).toEqual(fixture.mixedTrace); expect(messages.at(-1).value).toBe(effectResult);
    expect(messages.filter(message => message.kind === "frame").map(message => message.frame.envelope.payload.kind)).toEqual(fixture.hostFramePayloadKinds);
    await send({ data: { kind: "unrecognized", requestId: "r3", actorId: "b" } });
    expect(messages.at(-3)).toMatchObject({ kind: "heartbeat", turnSeq: 3 }); expect(messages.at(-2)).toMatchObject({ kind: "worker-fault", source: "handler", phase: "unrecognized", actorId: "b" }); expect(messages.at(-1)).toMatchObject({ kind: "result", requestId: "r3", ok: false });
    const traps: unknown[] = []; const failed = vm.createContext({ WebAssembly: {}, self: { postMessage: (message: unknown) => traps.push(message) } });
    expect(() => new vm.Script(generated("shardWorkerSource")).runInContext(failed)).toThrow(/JSPI/); expect(traps).toHaveLength(1); expect(traps[0]).toMatchObject({ kind: "trap", actorId: "*", activationGeneration: null });
    const postFault = new Error("post-after-observation"); const normalizationFault = Object.freeze({ normalization: "failed" });
    const guestFault = Object.defineProperty({}, "payload", { get() { throw normalizationFault; } });
    for (const mode of ["postThenThrow", "errorNormalizationThrows"] as const) {
      const captured: any[] = []; let callback: ((event: { data: Record<string, unknown> }) => Promise<void>) | null = null;
      const broken = vm.createContext({ WebAssembly: { Suspending: class {}, promising: (value: unknown) => value }, api: { checkpoint: async () => { if (mode === "errorNormalizationThrows") throw guestFault; return checkpoint; } }, self: { addEventListener: (kind: string, handler: typeof callback) => { if (kind === "message") callback = handler; }, postMessage: (message: any) => { captured.push(message); if (mode === "postThenThrow" && message.kind === "result" && message.ok) throw postFault; } } });
      new vm.Script(generated("shardWorkerSource")).runInContext(broken); new vm.Script('actors.set("a", { api, activationGeneration: 1n, pendingAssets: [] });').runInContext(broken);
      if (!callback) throw new Error("Missing fault-probe dispatcher");
      const run = callback as (event: { data: Record<string, unknown> }) => Promise<void>;
      const completed = run({ data: { kind: "checkpoint", requestId: "r4", actorId: "a" } });
      if (mode === "errorNormalizationThrows") await expect(completed).rejects.toBe(normalizationFault); else await completed;
      expect(captured.map(message => message.kind === "result" ? `result-${message.ok ? "success" : "fault"}` : message.kind)).toEqual(fixture.currentFaultTraces[mode]);
      if (mode === "postThenThrow") expect(captured.at(-1)).toMatchObject({ requestId: "r4", error: String(postFault), ok: false });
    }
    expect(fixture.currentFaultTraces.semanticallyAccepted).toBe(false);
  });
  it("ActorReturnResponseMetadata matches actual fixed source records and separately prices the projection graph", async () => {
    const { default: schema } = await import("./🎟️credit/📋️metadata/🧬️schema.json"); const { default: fixture } = await import("./🎟️credit/📋️metadata/🧪️fixture/🔣️.json");
    const { default: resident } = await import("../../../🌱️value/💾️resident/🧬️schema.json");
    const { default: Ajv } = await import("ajv"); const { default: ts } = await import("typescript"); const { readFileSync } = await import("node:fs");
    const ajv = new Ajv({ strict: true }).addSchema(resident); const validate = ajv.compile(schema); expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const paths = { output: "../../🪪️activation/🚪️instance/📥️output/🟦️.ts", shard: "../../📮️shard-client/🟦️.ts", response: "./🟦️.ts", result: "../🟦️.ts" };
    const totals = new Map<string, { bytes: bigint; slots: bigint; owners: bigint }>();
    for (const layout of fixture.layouts) {
      const path = paths[layout.source as keyof typeof paths]; const parsed = ts.createSourceFile(path, readFileSync(new URL(path, import.meta.url), "utf8"), ts.ScriptTarget.Latest, true);
      const declaration = parsed.statements.find(node => (ts.isClassDeclaration(node) || ts.isTypeAliasDeclaration(node)) && node.name?.text === layout.declaration);
      const members = declaration && ts.isClassDeclaration(declaration) ? declaration.members.filter(ts.isPropertyDeclaration) : declaration && ts.isTypeAliasDeclaration(declaration) && ts.isTypeLiteralNode(declaration.type) ? declaration.type.members : null;
      if (!members) throw new Error("Missing declared metadata source " + layout.declaration);
      const fields = members.map(member => member.name?.getText(parsed).replace(/^#/, ""));
      expect(fields, layout.declaration).toEqual(layout.fields);
      const previous = totals.get(layout.group) ?? { bytes: 0n, slots: 0n, owners: 0n };
      totals.set(layout.group, { bytes: previous.bytes + BigInt(fixture.model.recordBytes) + BigInt(fields.length) * BigInt(fixture.model.fieldBytes), slots: previous.slots + 1n, owners: previous.owners + 1n });
    }
    const graph = fixture.projectionMaximum; const framing = totals.get("framing")!;
    totals.set("framing", { bytes: framing.bytes + BigInt(graph.records * fixture.model.recordBytes + graph.fields * fixture.model.fieldBytes), slots: framing.slots + BigInt(graph.records), owners: framing.owners + BigInt(graph.records) });
    for (const [group, expected] of Object.entries(fixture.minimumEnvelopes)) expect(Object.fromEntries(Object.entries(totals.get(group)!).map(([axis, count]) => [axis, Number(count)]))).toEqual(expected);
    expect(fixture.boundaries.finalMountEnvelope).toBe(false); expect(fixture.boundaries.neutralIntrinsicRecordsIncluded).toBe(false); expect(fixture.boundaries.rawBytesIncluded).toBe(false);
    const { default: responses } = await import("./🧪️fixture/🔣️.json"); let maximumRecords = 0; let maximumFields = 0;
    for (const row of responses.vectors) {
      const bytes = Buffer.from(row.hex, "hex"); const parser = new ActorReturnResponseFraming(); const freeze = Object.freeze; const records: object[] = [];
      const spy = vi.spyOn(Object, "freeze").mockImplementation(value => { if (value !== null && typeof value === "object") records.push(value); return freeze(value); });
      try { for (const byte of bytes) parser.push(byte); parser.finish(); } finally { spy.mockRestore(); }
      maximumRecords = Math.max(maximumRecords, records.length); maximumFields = Math.max(maximumFields, records.reduce((count, value) => count + Object.keys(value).length, 0));
    }
    expect({ records: maximumRecords, fields: maximumFields }).toEqual(fixture.projectionMaximum);
  });
  it("ActorReturnResponse declaration matches strict schemas and independent envelope encoding", async () => {
    const { default: schema } = await import("./🧬️schema.json");
    const { default: fixture } = await import("./🧪️fixture/🔣️.json");
    const { default: fixtureSchema } = await import("./📐️schema/🔣️.json");
    const { default: lifetime } = await import("../../🚪️lifetime/🧬️schema.json");
    const { default: page } = await import("../../📃️page/🧬️schema.json");
    const { default: returned } = await import("../🧬️schema.json");
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true }).addSchema(lifetime).addSchema(page).addSchema(returned).addSchema(schema);
    expect(ajv.compile(fixtureSchema)(fixture)).toBe(true);
    const uint = await oracle();
    for (const row of fixture.vectors) {
      const value = row.value;
      const body = value.kind === "result" ? Buffer.from(row.resultHex, "hex") : Buffer.of(fixture.faultTags[value.fault as keyof typeof fixture.faultTags]);
      const encoded = Buffer.concat([Buffer.from(fixture.magicHex, "hex"), Buffer.of(fixture.tags[value.kind as keyof typeof fixture.tags]), uint(BigInt(value.activationGeneration)), uint(value.transportRequestSequence), body]);
      expect(encoded.toString("hex")).toBe(row.hex);
      expect(ajv.validate(schema, value)).toBe(true);
    }
    expect(fixture.vectors[1]!.value.transportRequestSequence).not.toBe(fixture.vectors[1]!.value.result!.control!.receipt!.identity.origin.requestSequence);
    for (const value of [0, -1, 9007199254740992, 1.5]) expect(ajv.validate(schema, { ...fixture.vectors[0]!.value, transportRequestSequence: value })).toBe(false);
  });
  it("ActorReturnResponseCredit declaration validates exact one-reply and retained-fault transitions", async () => {
    const { default: schema } = await import("./🎟️credit/🧬️schema.json");
    const { default: fixture } = await import("./🎟️credit/🧪️fixture/🔣️.json");
    const { default: fixtureSchema } = await import("./🎟️credit/📐️schema/🔣️.json");
    const { default: response } = await import("./🧬️schema.json");
    const { default: lifetime } = await import("../../🚪️lifetime/🧬️schema.json");
    const { default: page } = await import("../../📃️page/🧬️schema.json");
    const { default: returned } = await import("../🧬️schema.json");
    const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");
    const ajv = new Ajv({ strict: true }).addSchema(lifetime).addSchema(page).addSchema(returned).addSchema(response).addSchema(schema);
    const validate = ajv.compile(fixtureSchema); expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    for (const row of fixture.receiverCases) {
      let state = fixture.receiverInitial;
      for (const action of row.actions) {
        state = produce(state, next => {
          if (action === "post") next.phase = "posted";
          else if (action === "cancelEmpty") { next.phase = "cancelled"; next.rawBackingBytes = 0; next.pendingRetained = false; }
          else if (action === "capture") { next.phase = "captured"; next.captures++; next.rawRetained = true; }
          else if (action === "settle") next.pendingRetained = false;
          else {
            next.phase = "heldFault"; next.sealed = true; next.faultRetained = true;
            if (["headerFault", "foreignWorker", "foreignActivation", "foreignRequest", "duplicate"].includes(action)) next.emergencyRoots++;
          }
        });
        expect(ajv.validate({ $ref: schema.$id + "#/definitions/receiver" }, state), JSON.stringify(ajv.errors)).toBe(true);
      }
      expect(state, row.name).toEqual(row.expected);
    }
    for (const row of fixture.workerCases) {
      let state = fixture.workerInitial;
      for (const action of row.actions) {
        state = produce(state, next => {
          if (action === "capture") { next.phase = "captured"; next.rawRetained = true; }
          else if (action === "normalize") { next.phase = "ready"; next.backingRetained = true; }
          else if (action === "post") { next.phase = "posted"; next.transferCreditsSpent++; next.backingRetained = false; }
          else if (action !== "duplicate") { next.phase = "heldFault"; next.faultRetained = true; if (action === "detachedPostFault") { next.transferCreditsSpent++; next.backingRetained = false; } }
        });
        expect(ajv.validate({ $ref: schema.$id + "#/definitions/worker" }, state), JSON.stringify(ajv.errors)).toBe(true);
      }
      expect(state, row.name).toEqual(row.expected);
    }
    for (const delta of [{ captures: 2 }, { emergencyRoots: 2 }, { rawBackingBytes: 0 }, { rawRetained: true }, { pendingRetained: false }]) expect(ajv.validate({ $ref: schema.$id + "#/definitions/receiver" }, { ...fixture.receiverInitial, ...delta })).toBe(false);
    expect(ajv.validate({ $ref: schema.$id + "#/definitions/worker" }, { ...fixture.workerCases[0]!.expected, transferCreditsSpent: 2 })).toBe(false);
    expect(ajv.validate({ $ref: schema.$id + "#/definitions/worker" }, { ...fixture.workerCases[0]!.expected, backingRetained: true })).toBe(false);
    expect(fixture.authority.transportRequestSequence).not.toBe(fixture.returnOriginRequestSequence);
    expect(fixture.maximumBackingBytes).toBe(ACTOR_RETURN_RESPONSE_MAXIMUM_BYTES);
  });
  it("ActorReturnResponse header routing preserves exact authority without decoding malformed bodies", async () => {
    const api = await import("./🟦️.ts");
    const { default: fixture } = await import("./🧪️fixture/🔣️.json");
    const uint = await oracle();
    for (const row of fixture.headerCases) {
      const raw = backing(row.hex); const expected = hydrate(row.header);
      const header = api.readActorReturnResponseHeader(raw);
      expect(header).toEqual(expected); expect(Object.isFrozen(header)).toBe(true);
      const prefix = Buffer.concat([Buffer.from(fixture.magicHex, "hex"), Buffer.of(fixture.tags[row.header.kind as keyof typeof fixture.tags]), uint(BigInt(row.header.activationGeneration)), uint(row.header.transportRequestSequence)]);
      expect(Buffer.from(raw).subarray(0, prefix.length)).toEqual(prefix);
      expect(() => api.decodeActorReturnResponse(raw)).toThrow();
      expect(Object.keys(header).sort()).toEqual(["activationGeneration", "kind", "transportRequestSequence"]);
    }
    for (const row of fixture.vectors) {
      const { kind, activationGeneration, transportRequestSequence } = hydrate(row.value);
      expect(api.readActorReturnResponseHeader(backing(row.hex))).toEqual({ kind, activationGeneration, transportRequestSequence });
    }
    const maximum = new ArrayBuffer(fixture.maximumBytes); new Uint8Array(maximum).set(Buffer.from(fixture.maximumPage.headerHex, "hex"));
    expect(api.readActorReturnResponseHeader(maximum)).toEqual({ kind: "result", activationGeneration: 18446744073709551615n, transportRequestSequence: Number.MAX_SAFE_INTEGER });
    expect(() => api.decodeActorReturnResponse(maximum)).toThrow();
    for (const hex of fixture.invalidHex.slice(0, 7)) expect(() => api.readActorReturnResponseHeader(backing(hex))).toThrow();
    for (const invalid of [new Uint8Array(maximum), new SharedArrayBuffer(16), new Proxy(maximum, {}), new ArrayBuffer(fixture.maximumBytes + 1)]) expect(() => api.readActorReturnResponseHeader(invalid)).toThrow();
  });
  it("ActorReturnResponse codec preserves transport correlation and closed fault variants", async () => {
    const api = await import("./🟦️.ts");
    const { default: fixture } = await import("./🧪️fixture/🔣️.json");
    for (const row of fixture.vectors) {
      const value = hydrate(row.value);
      const encoded = api.encodeActorReturnResponse(value);
      expect(Buffer.from(encoded).toString("hex")).toBe(row.hex);
      expect(Object.getOwnPropertyDescriptor(ArrayBuffer.prototype, "byteLength")!.get!.call(encoded)).toBe(encoded.byteLength);
      expect(api.decodeActorReturnResponse(encoded)).toEqual(value);
      expect(Object.isFrozen(api.decodeActorReturnResponse(encoded))).toBe(true);
      for (let length = 0; length < encoded.byteLength; length++) expect(() => api.decodeActorReturnResponse(encoded.slice(0, length))).toThrow();
    }
    for (const hex of fixture.invalidHex) expect(() => api.decodeActorReturnResponse(backing(hex))).toThrow();
    expect(() => api.encodeActorReturnResponse(hydrate({ kind: "fault", activationGeneration: "7", transportRequestSequence: 1, fault: "arbitraryError" }))).toThrow();
    expect(() => api.encodeActorReturnResponse({ kind: "result", activationGeneration: 8n, transportRequestSequence: 1, result: hydrate(fixture.vectors[0]!.value.result) })).toThrow();
  });
  it("ActorReturnResponse maximum page is exact and decoder rejects non-owning backings", async () => {
    const api = await import("./🟦️.ts");
    const { default: fixture } = await import("./🧪️fixture/🔣️.json");
    const { default: returned } = await import("../🧫️fixture/🔣️.json");
    const { createActorBytePage } = await import("../../📃️page/🟦️.ts");
    const row = returned.pageResultVectors[fixture.maximumPage.sharedPageVector]!;
    const bytes = Uint8Array.from({ length: row.pageLength }, (_, index) => (index * 37 + 11) % 256);
    const result = { kind: "page" as const, receipt: hydrate(row.receipt), page: createActorBytePage(bytes) };
    const encoded = api.encodeActorReturnResponse({ kind: "result", activationGeneration: BigInt(row.receipt.identity.origin.activationGeneration), transportRequestSequence: row.receipt.identity.origin.requestSequence, result });
    expect(encoded.byteLength).toBe(fixture.maximumPage.wireBytes); expect(api.ACTOR_RETURN_RESPONSE_MAXIMUM_BYTES).toBe(fixture.maximumBytes);
    expect(Buffer.from(encoded)).toEqual(Buffer.concat([Buffer.from(fixture.maximumPage.headerHex + row.prefixHex, "hex"), Buffer.from(bytes)]));
    const decoded = api.decodeActorReturnResponse(encoded); if (decoded.kind !== "result") throw new Error("expected result");
    expect(decoded.result).toEqual(result);
    const valid = backing(fixture.vectors[0]!.hex);
    for (const invalid of [new Uint8Array(valid), new Uint8Array(valid, 1), new SharedArrayBuffer(valid.byteLength), new ArrayBuffer(fixture.maximumBytes + 1), new Proxy(valid, {}), {}, null]) expect(() => api.decodeActorReturnResponse(invalid)).toThrow();
    let reads = 0;
    expect(() => api.decodeActorReturnResponse({ get byteLength() { reads++; return 12; } })).toThrow(); expect(reads).toBe(0);
  });
  it("ActorReturnResponse actual Node transfer detaches the whole fixed backing and preserves exact bytes", async () => {
    const { execFileSync } = await import("node:child_process");
    const { default: fixture } = await import("./🧪️fixture/🔣️.json");
    const source = `
      import { MessageChannel } from "node:worker_threads";
      import { once } from "node:events";
      const bytes = Buffer.from(process.argv[1], "hex");
      const raw = Uint8Array.from(bytes).buffer;
      raw.unowned = { bytes: new Uint8Array(8192) };
      const { port1, port2 } = new MessageChannel();
      const incoming = once(port2, "message");
      port1.postMessage(raw, [raw]);
      const [received] = await incoming;
      const length = Object.getOwnPropertyDescriptor(ArrayBuffer.prototype, "byteLength").get.call(received);
      process.stdout.write(JSON.stringify({ senderDetached: raw.detached, receiverResizable: received.resizable, receiverWholeBacking: length === received.byteLength && !(received instanceof SharedArrayBuffer), customPropertyTransferred: Object.hasOwn(received, "unowned"), bytesEqual: Buffer.from(received).equals(bytes) }));
      port1.close(); port2.close();
    `;
    const actual = JSON.parse(execFileSync("node", ["--input-type=module", "--eval", source, fixture.vectors[0]!.hex], { encoding: "utf8", timeout: 10000 }));
    expect(actual).toEqual(fixture.transfer);
  });
  it("ActorReturnResponse actual Node codec rejects shared resizable detached and view backings", async () => {
    const { execFileSync } = await import("node:child_process");
    const { default: fixture } = await import("./🧪️fixture/🔣️.json");
    const source = `
      const api = await import(process.argv[1]);
      const bytes = Buffer.from(process.argv[2], "hex");
      const raw = Uint8Array.from(bytes).buffer;
      const shared = new SharedArrayBuffer(bytes.length); new Uint8Array(shared).set(bytes);
      const resizable = new ArrayBuffer(bytes.length, { maxByteLength: bytes.length + 1 }); new Uint8Array(resizable).set(bytes);
      const detached = raw.slice(0); structuredClone(detached, { transfer: [detached] });
      const cases = { wholeFixed: raw, offsetView: new Uint8Array(raw, 1), wholeView: new Uint8Array(raw), shared, resizable, detached, proxy: new Proxy(raw, {}), oversized: new ArrayBuffer(api.ACTOR_RETURN_RESPONSE_MAXIMUM_BYTES + 1) };
      process.stdout.write(JSON.stringify(Object.fromEntries(Object.entries(cases).map(([name, value]) => { try { api.decodeActorReturnResponse(value); return [name, true]; } catch { return [name, false]; } }))));
    `;
    const actual = JSON.parse(execFileSync("node", ["--experimental-transform-types", "--input-type=module", "--eval", source, import.meta.url, fixture.vectors[0]!.hex], { encoding: "utf8", timeout: 10000 }));
    expect(actual).toEqual(fixture.backings);
  });
}
//#endregion 🧪️ReturnResponseLaws
