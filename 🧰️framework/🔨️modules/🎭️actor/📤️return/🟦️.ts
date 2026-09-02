//#region 🧬️ReturnDriveContract
import { ACTOR_BYTE_PAGE_BYTES, createActorBytePage, readActorBytePage, type ActorBytePage } from "../📄️page/🟦️.ts";
export type ActorReturnOrigin = { readonly activationGeneration: bigint; readonly requestSequence: number };
export type ActorReturnIdentity = { readonly origin: ActorReturnOrigin; readonly returnSequence: bigint };
export type ActorReturnPageReceipt = { readonly identity: ActorReturnIdentity; readonly pageSequence: bigint; readonly length: number; readonly final: boolean };
export type ActorReturnControl =
  | { readonly kind: "poll"; readonly identity: ActorReturnIdentity }
  | { readonly kind: "inputAck"; readonly receipt: ActorReturnPageReceipt }
  | { readonly kind: "cancel"; readonly identity: ActorReturnIdentity }
  | { readonly kind: "retiredAck"; readonly identity: ActorReturnIdentity };
export type ActorReturnDrive =
  | { readonly kind: "execute"; readonly origin: ActorReturnOrigin }
  | { readonly kind: "control"; readonly control: ActorReturnControl };
export type ActorReturnResult =
  | { readonly kind: "protocolFault"; readonly fault: "malformedControl" | "mixedControl" }
  | { readonly kind: "refused"; readonly origin: ActorReturnOrigin; readonly fault: ActorReturnFault }
  | { readonly kind: "pending"; readonly identity: ActorReturnIdentity; readonly reason: "working" | "blocked" | "awaitingInput" | "closing" }
  | { readonly kind: "page"; readonly receipt: ActorReturnPageReceipt; readonly page: ActorBytePage }
  | { readonly kind: "retired"; readonly identity: ActorReturnIdentity; readonly completion: "complete" | "cancelled" | "faulted" }
  | { readonly kind: "control"; readonly control: ActorReturnControl; readonly outcome: "accepted" | "duplicate" | "blocked" | "refused"; readonly fault: ActorReturnFault };
export type ActorReturnResultProjection = Exclude<ActorReturnResult, { kind: "page" }> | { readonly kind: "page"; readonly receipt: ActorReturnPageReceipt; readonly payloadOffset: number };
export type ActorReturnFault = "none" | "capacity" | "sequenceExhausted" | "staleOrigin" | "staleIdentity" | "wrongPage" | "inputNotRetired" | "notRetired" | "clockUnavailable" | "clockBackward" | "deadline" | "ownerFault" | "malformedControl" | "mixedControl";
export const ACTOR_RETURN_ORIGIN_MAXIMUM_BYTES = 18;
export const ACTOR_RETURN_IDENTITY_MAXIMUM_BYTES = 28;
export const ACTOR_RETURN_PAGE_RECEIPT_MAXIMUM_BYTES = 41;
export const ACTOR_RETURN_CONTROL_MAXIMUM_BYTES = 42;
export const ACTOR_RETURN_DRIVE_MAXIMUM_BYTES = 43;
export const ACTOR_RETURN_RESULT_MAXIMUM_BYTES = 1 + ACTOR_RETURN_PAGE_RECEIPT_MAXIMUM_BYTES + ACTOR_BYTE_PAGE_BYTES;
const U64_MAXIMUM = 0xffffffffffffffffn;
const RETURN_REASONS = ["working", "blocked", "awaitingInput", "closing"] as const;
const RETURN_COMPLETIONS = ["complete", "cancelled", "faulted"] as const;
const RETURN_OUTCOMES = ["accepted", "duplicate", "blocked", "refused"] as const;
const RETURN_FAULTS = ["none", "capacity", "sequenceExhausted", "staleOrigin", "staleIdentity", "wrongPage", "inputNotRetired", "notRetired", "clockUnavailable", "clockBackward", "deadline", "ownerFault", "malformedControl", "mixedControl"] as const;
//#endregion 🧬️ReturnDriveContract

//#region 📦️FixedControlCodec
class ReturnWriter {
  readonly bytes: Uint8Array;
  length = 0;
  constructor(maximum: number) { this.bytes = new Uint8Array(maximum); }
  byte(value: number): void {
    if (this.length === this.bytes.length) throw new Error("actor-return.envelope");
    this.bytes[this.length++] = value;
  }
  uint(value: bigint, maximum = U64_MAXIMUM, positive = true): void {
    if (typeof value !== "bigint" || value < (positive ? 1n : 0n) || value > maximum) throw new Error("actor-return.authority");
    do {
      const byte = Number(value & 127n); value >>= 7n;
      this.byte(byte | (value === 0n ? 0 : 128));
    } while (value !== 0n);
  }
  finish(): Uint8Array { return this.bytes.subarray(0, this.length); }
}

class ReturnReader {
  offset = 0;
  constructor(readonly bytes: Uint8Array, maximum: number) {
    if (!(bytes instanceof Uint8Array) || bytes.length < 1 || bytes.length > maximum) throw new Error("actor-return.envelope");
  }
  byte(): number {
    const byte = this.bytes[this.offset++];
    if (byte === undefined) throw new Error("actor-return.truncated");
    return byte;
  }
  uint(maximum = U64_MAXIMUM, positive = true): bigint {
    let value = 0n;
    for (let index = 0; index < 10; index++) {
      const byte = this.byte(); value |= BigInt(byte & 127) << BigInt(index * 7);
      if ((byte & 128) === 0) {
        if (index > 0 && byte === 0 || value > maximum || positive && value === 0n) throw new Error("actor-return.noncanonical-authority");
        return value;
      }
    }
    throw new Error("actor-return.overlong");
  }
  finish(): void { if (this.offset !== this.bytes.length) throw new Error("actor-return.trailing"); }
}

function writeOrigin(writer: ReturnWriter, origin: ActorReturnOrigin): void {
  if (!Number.isSafeInteger(origin.requestSequence) || origin.requestSequence < 1) throw new Error("actor-return.request-sequence");
  writer.uint(origin.activationGeneration); writer.uint(BigInt(origin.requestSequence), BigInt(Number.MAX_SAFE_INTEGER));
}
function readOrigin(reader: ReturnReader): ActorReturnOrigin {
  return Object.freeze({ activationGeneration: reader.uint(), requestSequence: Number(reader.uint(BigInt(Number.MAX_SAFE_INTEGER))) });
}
function writeIdentity(writer: ReturnWriter, identity: ActorReturnIdentity): void { writeOrigin(writer, identity.origin); writer.uint(identity.returnSequence); }
function readIdentity(reader: ReturnReader): ActorReturnIdentity { return Object.freeze({ origin: readOrigin(reader), returnSequence: reader.uint() }); }
function writePageReceipt(writer: ReturnWriter, receipt: ActorReturnPageReceipt): void {
  if (!Number.isInteger(receipt.length) || receipt.length < 0 || receipt.length > ACTOR_BYTE_PAGE_BYTES || typeof receipt.final !== "boolean" || !receipt.final && receipt.length === 0) throw new Error("actor-return.page-receipt");
  writeIdentity(writer, receipt.identity); writer.uint(receipt.pageSequence); writer.uint(BigInt(receipt.length), BigInt(ACTOR_BYTE_PAGE_BYTES), false); writer.byte(receipt.final ? 1 : 0);
}
function readPageReceipt(reader: ReturnReader): ActorReturnPageReceipt {
  const identity = readIdentity(reader); const pageSequence = reader.uint(); const length = Number(reader.uint(BigInt(ACTOR_BYTE_PAGE_BYTES), false)); const final = reader.byte();
  if (final > 1 || final === 0 && length === 0) throw new Error("actor-return.page-receipt");
  return Object.freeze({ identity, pageSequence, length, final: final === 1 });
}
function writeControl(writer: ReturnWriter, control: ActorReturnControl): void {
  switch (control.kind) {
    case "poll": writer.byte(0); writeIdentity(writer, control.identity); return;
    case "inputAck": writer.byte(1); writePageReceipt(writer, control.receipt); return;
    case "cancel": writer.byte(2); writeIdentity(writer, control.identity); return;
    case "retiredAck": writer.byte(3); writeIdentity(writer, control.identity); return;
    default: throw new Error("actor-return.control-tag");
  }
}
function readControl(reader: ReturnReader): ActorReturnControl {
  switch (reader.byte()) {
    case 0: return Object.freeze({ kind: "poll", identity: readIdentity(reader) });
    case 1: return Object.freeze({ kind: "inputAck", receipt: readPageReceipt(reader) });
    case 2: return Object.freeze({ kind: "cancel", identity: readIdentity(reader) });
    case 3: return Object.freeze({ kind: "retiredAck", identity: readIdentity(reader) });
    default: throw new Error("actor-return.control-tag");
  }
}

/** 📤️ Encodes one canonical fixed drive; wire identity does not grant execution or retirement authority. */
export function encodeActorReturnDrive(drive: ActorReturnDrive): Uint8Array {
  const writer = new ReturnWriter(ACTOR_RETURN_DRIVE_MAXIMUM_BYTES);
  switch (drive.kind) {
    case "execute": writer.byte(0); writeOrigin(writer, drive.origin); break;
    case "control": writer.byte(1); writeControl(writer, drive.control); break;
    default: throw new Error("actor-return.drive-tag");
  }
  return writer.finish();
}

/** 📬️ Decodes exact fixed control bytes without interpreting or releasing a retained content owner. */
export function decodeActorReturnDrive(bytes: Uint8Array): ActorReturnDrive {
  const reader = new ReturnReader(bytes, ACTOR_RETURN_DRIVE_MAXIMUM_BYTES);
  const tag = reader.byte();
  const drive: ActorReturnDrive = tag === 0 ? Object.freeze({ kind: "execute", origin: readOrigin(reader) }) : tag === 1 ? Object.freeze({ kind: "control", control: readControl(reader) }) : (() => { throw new Error("actor-return.drive-tag"); })();
  reader.finish(); return drive;
}
//#endregion 📦️FixedControlCodec

//#region 📤️FixedResultCodec
function writeEnum(writer: ReturnWriter, value: string, values: readonly string[]): void {
  const index = values.indexOf(value);
  if (index < 0) throw new Error("actor-return.result-enum");
  writer.byte(index);
}
function validateControlResult(control: ActorReturnControl, outcome: string, fault: ActorReturnFault): void {
  const success = outcome === "accepted" || outcome === "duplicate";
  if (success ? fault !== "none" || control.kind === "poll" : fault === "none") throw new Error("actor-return.control-outcome");
}

/** 📮️ Encodes one fixed result envelope; variable semantic content is a separate retained stream. */
export function encodeActorReturnResult(result: ActorReturnResult): Uint8Array {
  const writer = new ReturnWriter(result.kind === "page" ? ACTOR_RETURN_RESULT_MAXIMUM_BYTES : ACTOR_RETURN_CONTROL_MAXIMUM_BYTES + 3);
  switch (result.kind) {
    case "protocolFault":
      if (result.fault !== "malformedControl" && result.fault !== "mixedControl") throw new Error("actor-return.protocol-fault");
      writer.byte(5); writeEnum(writer, result.fault, RETURN_FAULTS); break;
    case "refused":
      if (result.fault === "none") throw new Error("actor-return.refused-fault");
      writer.byte(0); writeOrigin(writer, result.origin); writeEnum(writer, result.fault, RETURN_FAULTS); break;
    case "pending": writer.byte(1); writeIdentity(writer, result.identity); writeEnum(writer, result.reason, RETURN_REASONS); break;
    case "page": {
      writer.byte(2); writePageReceipt(writer, result.receipt);
      const bytes = readActorBytePage(result.page);
      if (bytes.length !== result.receipt.length) throw new Error("actor-return.page-length");
      writer.bytes.set(bytes, writer.length); writer.length += ACTOR_BYTE_PAGE_BYTES; break;
    }
    case "retired": writer.byte(3); writeIdentity(writer, result.identity); writeEnum(writer, result.completion, RETURN_COMPLETIONS); break;
    case "control":
      validateControlResult(result.control, result.outcome, result.fault);
      writer.byte(4); writeControl(writer, result.control); writeEnum(writer, result.outcome, RETURN_OUTCOMES); writeEnum(writer, result.fault, RETURN_FAULTS); break;
    default: throw new Error("actor-return.result-tag");
  }
  return writer.finish();
}

type ResultFramingStage = "tag" | "control" | "activation" | "request" | "return" | "page" | "length" | "final" | "padding" | "reason" | "completion" | "outcome" | "fault" | "done";

/** 📐️ One canonical byte per call validates fixed metadata and page padding without copying payload storage. */
export class ActorReturnResultFraming {
  #stage: ResultFramingStage = "tag";
  #tag = -1;
  #controlTag = -1;
  #offset = 0;
  #payloadOffset = 0;
  #payloadRead = 0;
  #accumulator = 0n;
  #digits = 0;
  #activation = 0n;
  #request = 0;
  #returnSequence = 0n;
  #pageSequence = 0n;
  #length = 0;
  #final = false;
  #reason: typeof RETURN_REASONS[number] = "working";
  #completion: typeof RETURN_COMPLETIONS[number] = "complete";
  #outcome: typeof RETURN_OUTCOMES[number] = "accepted";
  #faultValue: ActorReturnFault = "none";
  #failed = false;
  #value: ActorReturnResultProjection | null = null;
  get value(): ActorReturnResultProjection | null { return this.#value; }
  #fail(): never { this.#failed = true; this.#value = null; throw new Error("actor-return.result-framing"); }
  #enum<T extends string>(byte: number, values: readonly T[]): T { const value = values[byte]; if (value === undefined) return this.#fail(); return value; }
  push(byte: number): void {
    if (this.#failed || this.#value !== null || this.#stage === "done" || !Number.isInteger(byte) || byte < 0 || byte > 255 || this.#offset === ACTOR_RETURN_RESULT_MAXIMUM_BYTES) this.#fail();
    this.#offset++;
    switch (this.#stage) {
      case "tag": if (byte > 5) this.#fail(); this.#tag = byte; this.#stage = byte === 4 ? "control" : byte === 5 ? "fault" : "activation"; return;
      case "control": if (byte > 3) this.#fail(); this.#controlTag = byte; this.#stage = "activation"; return;
      case "final":
        if (byte > 1 || byte === 0 && this.#length === 0) this.#fail();
        this.#final = byte === 1; this.#payloadOffset = this.#offset; this.#stage = this.#tag === 2 ? "padding" : "outcome"; return;
      case "padding":
        if (this.#payloadRead >= this.#length && byte !== 0) this.#fail();
        this.#payloadRead++; if (this.#payloadRead === ACTOR_BYTE_PAGE_BYTES) this.#stage = "done"; return;
      case "reason": this.#reason = this.#enum(byte, RETURN_REASONS); this.#stage = "done"; return;
      case "completion": this.#completion = this.#enum(byte, RETURN_COMPLETIONS); this.#stage = "done"; return;
      case "outcome": this.#outcome = this.#enum(byte, RETURN_OUTCOMES); this.#stage = "fault"; return;
      case "fault": this.#faultValue = this.#enum(byte, RETURN_FAULTS); this.#stage = "done"; return;
      default: this.#uint(byte); return;
    }
  }
  #uint(byte: number): void {
    const stage = this.#stage;
    const limit = stage === "request" ? 8 : 10;
    if (this.#digits >= limit || this.#digits === 9 && byte > 1) this.#fail();
    this.#accumulator |= BigInt(byte & 127) << BigInt(this.#digits * 7); this.#digits++;
    if (byte & 128) { if (this.#digits === limit) this.#fail(); return; }
    const value = this.#accumulator;
    const maximum = stage === "request" ? BigInt(Number.MAX_SAFE_INTEGER) : stage === "length" ? BigInt(ACTOR_BYTE_PAGE_BYTES) : U64_MAXIMUM;
    if (this.#digits > 1 && byte === 0 || value > maximum || stage !== "length" && value === 0n) this.#fail();
    this.#accumulator = 0n; this.#digits = 0;
    switch (stage) {
      case "activation": this.#activation = value; this.#stage = "request"; return;
      case "request": this.#request = Number(value); this.#stage = this.#tag === 0 ? "fault" : "return"; return;
      case "return": this.#returnSequence = value; this.#stage = this.#tag === 2 || this.#controlTag === 1 ? "page" : this.#tag === 1 ? "reason" : this.#tag === 3 ? "completion" : "outcome"; return;
      case "page": this.#pageSequence = value; this.#stage = "length"; return;
      case "length": this.#length = Number(value); this.#stage = "final"; return;
      default: this.#fail();
    }
  }
  finish(): ActorReturnResultProjection {
    if (this.#failed || this.#stage !== "done") this.#fail();
    if (this.#value !== null) return this.#value;
    const fault = this.#faultValue;
    if (this.#tag === 5) {
      if (fault !== "malformedControl" && fault !== "mixedControl") this.#fail();
      return this.#value = Object.freeze({ kind: "protocolFault", fault });
    }
    if (this.#tag === 0 && fault === "none") this.#fail();
    if (this.#tag === 4) { const success = this.#outcome === "accepted" || this.#outcome === "duplicate"; if (success ? fault !== "none" || this.#controlTag === 0 : fault === "none") this.#fail(); }
    const origin = Object.freeze({ activationGeneration: this.#activation, requestSequence: this.#request });
    if (this.#tag === 0) return this.#value = Object.freeze({ kind: "refused", origin, fault });
    const identity = Object.freeze({ origin, returnSequence: this.#returnSequence });
    if (this.#tag === 1) return this.#value = Object.freeze({ kind: "pending", identity, reason: this.#reason });
    if (this.#tag === 3) return this.#value = Object.freeze({ kind: "retired", identity, completion: this.#completion });
    if (this.#tag === 2 || this.#controlTag === 1) {
      const receipt = Object.freeze({ identity, pageSequence: this.#pageSequence, length: this.#length, final: this.#final });
      if (this.#tag === 2) return this.#value = Object.freeze({ kind: "page", receipt, payloadOffset: this.#payloadOffset });
      return this.#value = Object.freeze({ kind: "control", control: Object.freeze({ kind: "inputAck", receipt }), outcome: this.#outcome, fault });
    }
    const kind = this.#controlTag === 0 ? "poll" : this.#controlTag === 2 ? "cancel" : "retiredAck";
    return this.#value = Object.freeze({ kind: "control", control: Object.freeze({ kind, identity }), outcome: this.#outcome, fault });
  }
}

/** 📭️ Materializes the canonical framing projection for non-retained callers; this whole conversion grants no ownership. */
export function decodeActorReturnResult(bytes: Uint8Array): ActorReturnResult {
  if (!(bytes instanceof Uint8Array) || bytes.length < 1 || bytes.length > ACTOR_RETURN_RESULT_MAXIMUM_BYTES) throw new Error("actor-return.envelope");
  const parser = new ActorReturnResultFraming(); for (const byte of bytes) parser.push(byte);
  const value = parser.finish();
  return value.kind === "page" ? Object.freeze({ kind: "page", receipt: value.receipt, page: createActorBytePage(bytes.subarray(value.payloadOffset, value.payloadOffset + value.receipt.length)) }) : value;
}
//#endregion 📤️FixedResultCodec

//#region 🧪️ReturnDriveLaws
if (import.meta.vitest) {
  const { it, expect, vi } = import.meta.vitest;
  const hydrate = (value: unknown): ActorReturnDrive => JSON.parse(JSON.stringify(value), (key, item) => ["activationGeneration", "returnSequence", "pageSequence"].includes(key) ? BigInt(item) : item);
  const hydrateResult = (value: unknown): ActorReturnResult => JSON.parse(JSON.stringify(value), (key, item) => ["activationGeneration", "returnSequence", "pageSequence"].includes(key) ? BigInt(item) : item);
  const resultOracle = async (): Promise<(value: ActorReturnResult) => Buffer> => {
    const { default: fixture } = await import("./🧪️fixture.json");
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

  it("ActorReturnDrive matches the shared canonical vectors and independent LEB128 bytes", async () => {
    const { default: fixture } = await import("./🧪️fixture.json");
    const { default: schema } = await import("./🧬️schema.json");
    const { default: fixtureSchema } = await import("./🧪️schema.json");
    const { default: lifetimeSchema } = await import("../🚪️lifetime/🧬️schema.json");
    const { default: pageSchema } = await import("../📄️page/🧬️schema.json");
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true }).addSchema(lifetimeSchema).addSchema(pageSchema).addSchema(schema);
    expect(ajv.compile(fixtureSchema)(fixture)).toBe(true);
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
    const { default: fixture } = await import("./🧪️fixture.json");
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
    const { default: fixture } = await import("./🧪️fixture.json");
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
    const { default: fixture } = await import("./🧪️fixture.json");
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
    const { default: fixture } = await import("./🧪️fixture.json");
    const { default: schema } = await import("./🧬️schema.json");
    const { default: lifetimeSchema } = await import("../🚪️lifetime/🧬️schema.json");
    const { default: pageSchema } = await import("../📄️page/🧬️schema.json");
    const { default: Ajv } = await import("ajv");
    const validate = new Ajv({ strict: true }).addSchema(lifetimeSchema).addSchema(pageSchema).addSchema(schema).getSchema("semio.actor.retained-return.v1#/definitions/result")!;
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
    const { default: fixture } = await import("./🧪️fixture.json");
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
    const api = await import("./🟦️.ts");
    const { default: fixture } = await import("./🧪️fixture.json");
    const { default: law } = await import("./📄️framing/🧪️fixture.json");
    const { default: schema } = await import("./📄️framing/🧬️schema.json");
    const { default: returned } = await import("./🧬️schema.json");
    const { default: lifetime } = await import("../🚪️lifetime/🧬️schema.json");
    const { default: page } = await import("../📄️page/🧬️schema.json");
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
    const api = await import("./🟦️.ts");
    const { default: fixture } = await import("./🧪️fixture.json"); const oracle = await resultOracle();
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
    const { default: fixture } = await import("./🧪️fixture.json");
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
//#endregion 🧪️ReturnDriveLaws
