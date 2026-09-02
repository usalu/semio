//#region 🧬️JsonStringContract
import { OwnedUiSceneJsonDocument, type OwnedUiSceneJsonDocumentReader, type OwnedUiSceneJsonDocumentRetirement } from "../🧾️value/🟦️.ts";
import type { OwnedUiSceneJsonToken } from "../🟦️.ts";
import type { OwnedUiSceneReader, OwnedUiSceneRetirement } from "../../../🟦️.ts";
import type { RetainedUiWireStep } from "../../../../📦️wire/🟦️.ts";

type Grant = { readonly maxItems: number; readonly maxBytes: number };
export type OwnedUiSceneJsonStringStep = RetainedUiWireStep | { readonly kind: "text"; readonly value: string; readonly phase: string; readonly items: number; readonly bytes: number };
const GRANT = Object.freeze({ maxItems: 1, maxBytes: 4096 });
const admitted = (grant: Grant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
function forward(current: { readonly kind: string; readonly items: number; readonly bytes: number }, phase: string): RetainedUiWireStep {
  const invalid = !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > 4096 || !Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1;
  return { kind: invalid || current.kind === "rejected" ? "rejected" : current.kind === "blocked" ? "blocked" : "pending", phase, items: current.items, bytes: current.bytes };
}
//#endregion 🧬️JsonStringContract

//#region 🔤️JsonStringProjection
/** 🔤️ Projects an exact document token into bounded UTF-16 pages without reconstructing its whole string. */
export class OwnedUiSceneJsonStringCursor {
  readonly #ordinal: number;
  #source: OwnedUiSceneJsonDocument | null;
  #sourceClose: OwnedUiSceneJsonDocumentRetirement | null = null;
  #lookup: OwnedUiSceneJsonDocumentReader | null = null;
  #lookupClose: OwnedUiSceneJsonDocumentRetirement | null = null;
  #token: OwnedUiSceneJsonToken | null = null;
  #reader: OwnedUiSceneReader | null = null;
  #readerClose: OwnedUiSceneRetirement | null = null;
  #chunk: Uint8Array | null = null;
  #chunkOffset = 0;
  #buffer: Uint16Array | null = new Uint16Array(128);
  #length = 0;
  #offset = 0;
  #scalar = 0;
  #remaining = 0;
  #hex = 0;
  #escape = false;
  #eof = false;
  #phase: "lookup" | "lookup-retire" | "lookup-close" | "read" | "ready" = "lookup";
  #closing = false;
  #failure: string | null = null;
  constructor(source: OwnedUiSceneJsonDocument, ordinal: number) { if (!Number.isSafeInteger(ordinal) || ordinal < 0) throw new Error("JSON string requires an exact token ordinal"); this.#ordinal = ordinal; this.#source = OwnedUiSceneJsonDocument.prototype.capture.call(source); Object.freeze(this); }
  get offset(): number { return this.#offset; }
  get failure(): string | null { return this.#failure; }
  #observe(current: RetainedUiWireStep): RetainedUiWireStep { if (current.kind === "rejected") this.#failure = this.#reader?.failure ?? "JSON string child rejected"; return current; }
  #unit(value: number): void { this.#buffer![this.#length++] = value; }
  #point(value: number): void { if (value <= 0xffff) this.#unit(value); else { const scalar = value - 65536; this.#unit(0xd800 + (scalar >>> 10)); this.#unit(0xdc00 + (scalar & 1023)); } }
  #byte(byte: number): void {
    this.#offset++; this.#chunkOffset++;
    if (this.#remaining) { if ((byte & 0xc0) !== 0x80) throw new Error("Captured JSON UTF-8 changed"); this.#scalar = this.#scalar * 64 + (byte & 63); if (--this.#remaining === 0) this.#point(this.#scalar); return; }
    if (this.#hex) { const digit = byte >= 48 && byte <= 57 ? byte - 48 : byte >= 65 && byte <= 70 ? byte - 55 : byte >= 97 && byte <= 102 ? byte - 87 : -1; if (digit < 0) throw new Error("Captured JSON escape changed"); this.#scalar = this.#scalar * 16 + digit; if (--this.#hex === 0) this.#unit(this.#scalar); return; }
    if (this.#escape) {
      this.#escape = false;
      if (byte === 117) { this.#hex = 4; this.#scalar = 0; return; }
      const value = byte === 34 || byte === 92 || byte === 47 ? byte : byte === 98 ? 8 : byte === 102 ? 12 : byte === 110 ? 10 : byte === 114 ? 13 : byte === 116 ? 9 : -1;
      if (value < 0) throw new Error("Captured JSON short escape changed"); this.#unit(value); return;
    }
    if (byte === 92) { this.#escape = true; return; }
    if (byte < 128) { this.#unit(byte); return; }
    if (byte >= 0xc2 && byte <= 0xdf) { this.#scalar = byte & 31; this.#remaining = 1; }
    else if (byte >= 0xe0 && byte <= 0xef) { this.#scalar = byte & 15; this.#remaining = 2; }
    else if (byte >= 0xf0 && byte <= 0xf4) { this.#scalar = byte & 7; this.#remaining = 3; }
    else throw new Error("Captured JSON UTF-8 leader changed");
  }
  #emit(): OwnedUiSceneJsonStringStep { const length = this.#length; const value = String.fromCharCode(...this.#buffer!.subarray(0, length)); this.#length = 0; return { kind: "text", value, phase: "json-string-output", items: 1, bytes: length * 4 + 64 }; }
  advance(grant: Grant): OwnedUiSceneJsonStringStep {
    if (!admitted(grant)) return step("blocked", "json-string"); if (this.#closing || this.#failure) return step("rejected", "json-string"); if (this.#phase === "ready") return step("ready", "json-string");
    try {
      if (this.#phase === "lookup") {
        if (!this.#lookup) { this.#lookup = this.#source!.beginLookup(this.#ordinal); return step("pending", "json-string-lookup", 128); }
        const current = this.#lookup.advance(GRANT); const result = this.#observe(forward(current, "json-string-lookup")); if (result.kind === "rejected" || result.kind === "blocked") return result;
        if (current.kind === "value") this.#token = current.value;
        if (current.kind === "complete") this.#phase = "lookup-retire"; return result;
      }
      if (this.#phase === "lookup-retire") { this.#lookupClose = this.#lookup!.beginClose(); this.#lookup = null; this.#phase = "lookup-close"; return step("pending", "json-string-lookup-close", 128); }
      if (this.#lookupClose) { const current = this.#lookupClose.advance(GRANT); const result = this.#observe(forward(current, "json-string-lookup-close")); if (current.kind === "complete" && result.kind === "pending") this.#lookupClose = null; return result; }
      if (this.#phase === "lookup-close") {
        const token = this.#token; if (!token || token.kind !== "key" && token.kind !== "string" || token.length < 2) throw new Error("Selected JSON token is not a string");
        this.#reader = this.#source!.beginSpan(token.start + 1, token.length - 2); this.#token = null; this.#phase = "read"; return step("pending", "json-string-span", 160);
      }
      if (this.#length >= 126 || this.#eof && this.#length) return this.#emit();
      if (this.#chunk && this.#chunkOffset === this.#chunk.length) { const length = this.#chunk.length; this.#chunk = null; this.#chunkOffset = 0; return step("pending", "json-string-chunk-close", length + 32); }
      if (!this.#chunk) {
        if (this.#eof) { if (this.#remaining || this.#hex || this.#escape) throw new Error("Truncated captured JSON string"); this.#phase = "ready"; return step("ready", "json-string"); }
        const current = this.#reader!.advance(GRANT); const result = this.#observe(forward(current, "json-string-read")); if (result.kind === "rejected" || result.kind === "blocked") return result;
        if (current.kind === "bytes") { this.#chunk = current.value; this.#chunkOffset = 0; } if (current.kind === "complete") this.#eof = true; return result;
      }
      this.#byte(this.#chunk[this.#chunkOffset]!); return step("pending", "json-string-decode", 64);
    } catch (error) { this.#failure = error instanceof Error ? error.message : "JSON string projection failed"; return step("rejected", "json-string", 128); }
  }
  //#region ♻️StringOwnership
  beginClose(): void { this.#closing = true; }
  closeStep(grant: Grant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "json-string-close"); this.#closing = true;
    if (this.#chunk) { const bytes = this.#chunk.length; this.#chunk = null; this.#chunkOffset = 0; return step("pending", "json-string-chunk-close", bytes + 32); }
    if (this.#buffer) { this.#buffer = null; this.#length = 0; return step("pending", "json-string-buffer-close", 256); }
    if (this.#token) { this.#token = null; return step("pending", "json-string-token-close", 96); }
    if (this.#lookupClose) { const current = this.#lookupClose.advance(GRANT); const result = forward(current, "json-string-lookup-close"); if (current.kind === "complete" && result.kind === "pending") this.#lookupClose = null; return result; }
    if (this.#lookup) { this.#lookupClose = this.#lookup.beginClose(); this.#lookup = null; return step("pending", "json-string-lookup-close", 128); }
    if (this.#readerClose) { const current = this.#readerClose.advance(GRANT); const result = forward(current, "json-string-span-close"); if (current.kind === "complete" && result.kind === "pending") this.#readerClose = null; return result; }
    if (this.#reader) { this.#readerClose = this.#reader.beginClose(); this.#reader = null; return step("pending", "json-string-span-close", 128); }
    if (this.#sourceClose) { const current = this.#sourceClose.advance(GRANT); const result = forward(current, "json-string-source-close"); if (current.kind === "complete" && result.kind === "pending") this.#sourceClose = null; return result; }
    if (this.#source) { this.#sourceClose = this.#source.beginClose(); this.#source = null; return step("pending", "json-string-source-close", 128); }
    return step("complete", "json-string-close");
  }
  terminalIsEmpty(): boolean { return this.#closing && this.#source === null && this.#sourceClose === null && this.#lookup === null && this.#lookupClose === null && this.#token === null && this.#reader === null && this.#readerClose === null && this.#chunk === null && this.#buffer === null; }
  //#endregion ♻️StringOwnership
}
//#endregion 🔤️JsonStringProjection
