//#region 🧬️ReturnDriveContract
import { ACTOR_BYTE_PAGE_BYTES, createActorBytePage, readActorBytePage, type ActorBytePage } from "../📃️page/🟦️.ts";
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
  readonly bytes: Uint8Array;
  offset = 0;
  constructor(bytes: Uint8Array, maximum: number) {
    if (!(bytes instanceof Uint8Array) || bytes.length < 1 || bytes.length > maximum) throw new Error("actor-return.envelope");
    this.bytes = bytes;
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
  const { registerTests1 } = await import("./🧪️tests/🧪️actorreturn-codecs-load-natively-in-node-strip-only-mode-and-preserve-ev/🟦️.ts");
  await registerTests1(import.meta.vitest, { ACTOR_RETURN_CONTROL_MAXIMUM_BYTES, ACTOR_RETURN_DRIVE_MAXIMUM_BYTES, ACTOR_RETURN_IDENTITY_MAXIMUM_BYTES, ACTOR_RETURN_ORIGIN_MAXIMUM_BYTES, ACTOR_RETURN_PAGE_RECEIPT_MAXIMUM_BYTES, ACTOR_RETURN_RESULT_MAXIMUM_BYTES, ActorReturnResultFraming, createActorBytePage, decodeActorReturnDrive, decodeActorReturnResult, encodeActorReturnDrive, encodeActorReturnResult, readActorBytePage }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️ReturnDriveLaws
