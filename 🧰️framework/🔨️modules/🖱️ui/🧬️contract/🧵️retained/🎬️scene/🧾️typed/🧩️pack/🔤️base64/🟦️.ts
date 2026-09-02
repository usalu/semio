//#region 🧬️OwnedPackedFieldContract
import { OwnedUiPreparedScene, type OwnedUiPreparedSceneRetirement } from "../../🟦️.ts";
import type { OwnedUiSceneReader, OwnedUiSceneRetirement } from "../../../🟦️.ts";
import { UiSurfaceByteBuilder, type UiSurfaceBytes, type UiSurfaceByteRetirement } from "../../../../📦️wire/🔢️bytes/🟦️.ts";
import type { RetainedUiWireStep } from "../../../../📦️wire/🟦️.ts";

type Grant = { readonly maxItems: number; readonly maxBytes: number };
const admitted = (grant: Grant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
const MINT = Object.freeze({});
let ownField: (bytes: UiSurfaceBytes) => OwnedUiPackedSceneField;
const whitespace = (byte: number): boolean => byte === 32 || byte === 9 || byte === 10 || byte === 12 || byte === 13;
function digit(byte: number): number {
  if (byte >= 65 && byte <= 90) return byte - 65;
  if (byte >= 97 && byte <= 122) return byte - 71;
  if (byte >= 48 && byte <= 57) return byte + 4;
  return byte === 43 ? 62 : byte === 47 ? 63 : -1;
}
//#endregion 🧬️OwnedPackedFieldContract

//#region 🎟️PackedFieldOwnership
/** 🎟️ Only a complete exact captured-field decode can mint this immutable binary owner. */
export class OwnedUiPackedSceneField {
  #bytes: UiSurfaceBytes | null;
  private constructor(mint: object, bytes: UiSurfaceBytes) { if (mint !== MINT) throw new Error("Packed scene field requires exact decode authority"); this.#bytes = bytes; Object.freeze(this); }
  static { ownField = bytes => new OwnedUiPackedSceneField(MINT, bytes); }
  #live(): UiSurfaceBytes { if (!this.#bytes) throw new Error("Packed scene field is closed"); return this.#bytes; }
  get length(): number { return this.#live().length; }
  byteAt(index: number): number { return this.#live().byteAt(index); }
  capture(): OwnedUiPackedSceneField { return ownField(this.#live().capture()); }
  beginClose(): UiSurfaceByteRetirement { const bytes = this.#live(); this.#bytes = null; return bytes.beginClose(); }
  terminalIsEmpty(): boolean { return this.#bytes === null; }
}
//#endregion 🎟️PackedFieldOwnership

//#region 🧵️OwnedBase64Cursor
/** 📦️ Two bounded reads of one captured field prepare independent binary pages without a contiguous copy. */
export class OwnedUiSceneBase64Cursor {
  readonly #field: number;
  #source: OwnedUiPreparedScene | null;
  #reader: OwnedUiSceneReader | null = null;
  #readerClose: OwnedUiSceneRetirement | null = null;
  #sourceClose: OwnedUiPreparedSceneRetirement | null = null;
  #builder: UiSurfaceByteBuilder | null = null;
  #builderClose: UiSurfaceByteRetirement | null = null;
  #chunk: Uint8Array | null = null;
  #chunkOffset = 0;
  #sourceBytesRead = 0;
  #prefix = 0;
  #digits = 0;
  #padding = 0;
  #length = 0;
  #written = 0;
  #bits = 0;
  #value = 0;
  #pending: number | null = null;
  #phase: "scan" | "scan-close" | "allocate" | "decode" | "ready" = "scan";
  #failure: string | null = null;
  #closing = false;
  #taken = false;
  constructor(source: OwnedUiPreparedScene, field: number) {
    if (!Number.isSafeInteger(field) || field < 0) throw new Error("Packed field requires an exact prepared text field");
    this.#field = field; this.#source = OwnedUiPreparedScene.prototype.capture.call(source); Object.freeze(this);
  }
  get failure(): string | null { return this.#failure; }
  get sourceBytesRead(): number { return this.#sourceBytesRead; }
  #reject(reason: string): RetainedUiWireStep { this.#failure = reason; return step("rejected", "scene-base64", 64); }
  #byte(byte: number): RetainedUiWireStep {
    this.#chunkOffset++; this.#sourceBytesRead++;
    if (this.#prefix < 3) {
      if (byte !== (this.#prefix === 0 ? 112 : this.#prefix === 1 ? 107 : 58)) return this.#reject("Packed scene field requires pk: prefix");
      this.#prefix++; return step("pending", "scene-base64-prefix", 16);
    }
    if (whitespace(byte)) return step("pending", "scene-base64-whitespace", 16);
    if (this.#phase === "scan") {
      if (byte === 61) { if (++this.#padding > 2) return this.#reject("Excess base64 padding"); }
      else { if (digit(byte) < 0 || this.#padding) return this.#reject("Invalid base64 alphabet or padding order"); this.#digits++; }
      return step("pending", "scene-base64-scan", 32);
    }
    if (byte === 61) return step("pending", "scene-base64-padding", 16);
    const current = digit(byte);
    if (current < 0) return this.#reject("Captured base64 source changed");
    this.#value = this.#value * 64 + current; this.#bits += 6;
    if (this.#bits >= 8) { this.#bits -= 8; this.#pending = Math.floor(this.#value / 2 ** this.#bits); this.#value %= 2 ** this.#bits; }
    return step("pending", "scene-base64-decode", 32);
  }
  #end(): RetainedUiWireStep {
    if (this.#prefix !== 3) return this.#reject("Truncated packed scene prefix");
    if (this.#phase === "scan") {
      const remainder = this.#digits % 4;
      if (remainder === 1 || (this.#padding > 0 && (this.#digits + this.#padding) % 4 !== 0)) return this.#reject("Invalid base64 terminal quantum");
      this.#length = Math.floor(this.#digits * 6 / 8); this.#phase = "scan-close";
      return step("pending", "scene-base64-scan-finish", 64);
    }
    if (this.#written !== this.#length) return this.#reject("Base64 decoded length mismatch");
    this.#phase = "ready"; return step("ready", "scene-base64");
  }
  advance(grant: Grant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "scene-base64");
    if (this.#closing || this.#failure || this.#taken) return step("rejected", "scene-base64");
    if (this.#phase === "ready") return step("ready", "scene-base64");
    try {
      if (this.#phase === "scan-close") {
        if (this.#reader) { this.#readerClose = this.#reader.beginClose(); this.#reader = null; return step("pending", "scene-base64-scan-close", 128); }
        if (this.#readerClose) { const current = this.#readerClose.advance(grant); if (current.kind === "complete") this.#readerClose = null; return current.kind === "rejected" || current.kind === "blocked" ? current : step("pending", "scene-base64-scan-close", current.bytes); }
        this.#builder = new UiSurfaceByteBuilder(this.#length); this.#phase = "allocate"; return step("pending", "scene-base64-builder", 128);
      }
      if (this.#phase === "allocate") {
        const current = this.#builder!.advance(grant);
        if (current.kind === "rejected") return this.#reject("Base64 page admission rejected");
        if (current.kind === "blocked") return step("blocked", "scene-base64-allocate");
        this.#prefix = 0; this.#phase = "decode"; return step("pending", "scene-base64-allocate", current.bytes);
      }
      if (this.#pending !== null) {
        const current = this.#builder!.advance(grant, this.#pending);
        if (current.kind === "rejected") return this.#reject("Base64 page preparation rejected");
        if (current.accepted) { this.#pending = null; this.#written++; }
        return step(current.kind === "blocked" ? "blocked" : "pending", "scene-base64-write", current.bytes);
      }
      if (!this.#reader) { this.#reader = this.#source!.beginTextBytes(this.#field); return step("pending", "scene-base64-reader", 128); }
      if (this.#chunk && this.#chunkOffset === this.#chunk.length) { const bytes = this.#chunk.length + 32; this.#chunk = null; this.#chunkOffset = 0; return step("pending", "scene-base64-chunk-close", bytes); }
      if (!this.#chunk) {
        const current = this.#reader.advance(grant);
        if (current.kind === "bytes") { this.#chunk = current.value; this.#chunkOffset = 0; }
        if (current.kind === "complete") return this.#end();
        if (current.kind === "rejected") this.#failure = this.#reader.failure ?? "Packed field read rejected";
        return step(current.kind === "rejected" || current.kind === "blocked" ? current.kind : "pending", "scene-base64-read", current.bytes);
      }
      return this.#byte(this.#chunk[this.#chunkOffset]!);
    } catch (error) { return this.#reject(error instanceof Error ? error.message : "Packed field failed"); }
  }
  takeResult(): OwnedUiPackedSceneField | null {
    if (this.#phase !== "ready" || this.#closing || this.#failure || this.#taken) return null;
    const result = this.#builder!.takeResult(); if (!result) return null; this.#taken = true; return ownField(result);
  }

  //#region ♻️TerminalOwnership
  beginClose(): void { this.#closing = true; }
  closeStep(grant: Grant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "scene-base64-close");
    this.#closing = true;
    if (this.#chunk) { const bytes = this.#chunk.length + 32; this.#chunk = null; this.#chunkOffset = 0; return step("pending", "scene-base64-chunk-close", bytes); }
    if (this.#pending !== null) { this.#pending = null; return step("pending", "scene-base64-pending-close", 16); }
    if (this.#readerClose) { const current = this.#readerClose.advance(grant); if (current.kind === "complete") this.#readerClose = null; return current.kind === "rejected" || current.kind === "blocked" ? current : step("pending", "scene-base64-reader-close", current.bytes); }
    if (this.#reader) { this.#readerClose = this.#reader.beginClose(); this.#reader = null; return step("pending", "scene-base64-reader-close", 128); }
    if (this.#builderClose) { const current = this.#builderClose.advance(grant); if (current.kind === "complete") this.#builderClose = null; return step(current.kind === "blocked" || current.kind === "rejected" ? current.kind : "pending", "scene-base64-output-close", current.bytes); }
    if (this.#builder) { this.#builderClose = this.#builder.beginClose(); this.#builder = null; return step("pending", "scene-base64-output-close", 64); }
    if (this.#sourceClose) { const current = this.#sourceClose.advance(grant); if (current.kind === "complete") this.#sourceClose = null; return current.kind === "rejected" || current.kind === "blocked" ? current : step("pending", "scene-base64-source-close", current.bytes); }
    if (this.#source) { this.#sourceClose = this.#source.beginClose(); this.#source = null; return step("pending", "scene-base64-source-close", 128); }
    return step("complete", "scene-base64-close");
  }
  terminalIsEmpty(): boolean { return this.#closing && this.#chunk === null && this.#pending === null && this.#reader === null && this.#readerClose === null && this.#builder === null && this.#builderClose === null && this.#source === null && this.#sourceClose === null; }
  //#endregion ♻️TerminalOwnership
}
//#endregion 🧵️OwnedBase64Cursor
