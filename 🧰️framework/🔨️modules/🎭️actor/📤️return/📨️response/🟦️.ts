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
  const { registerTests1 } = await import("./🧪️tests/🧪️actorreturnresponseframing-uses-canonical-vectors-with-no-payload-copies/🟦️.ts");
  const { fileURLToPath } = await import("node:url");
  await registerTests1(import.meta.vitest, { ACTOR_RETURN_RESPONSE_MAXIMUM_BYTES, ActorReturnResponseFraming, createActorBytePage, decodeActorReturnResponse, encodeActorReturnResponse, fault, readActorReturnResponseHeader, uint }, { directory: fileURLToPath(new URL(".", import.meta.url)), url: import.meta.url });
}
//#endregion 🧪️ReturnResponseLaws
