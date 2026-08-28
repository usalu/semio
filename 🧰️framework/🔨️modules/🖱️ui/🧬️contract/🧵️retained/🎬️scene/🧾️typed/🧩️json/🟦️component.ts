//#region 🧬️OwnedJsonContract
import { OwnedUiPreparedScene, type OwnedUiPreparedSceneRetirement } from "../🟦️component.ts";
import type { OwnedUiSceneReader, OwnedUiSceneRetirement } from "../../🟦️component.ts";
import type { RetainedUiWireStep } from "../../../📦️wire/🟦️component.ts";

type Grant = { readonly maxItems: number; readonly maxBytes: number };
type State = "first" | "key" | "colon" | "value" | "comma";
type Frame = { readonly kind: "object" | "array"; state: State; parent: Frame | null };
type NumberState = "minus" | "zero" | "integer" | "dot" | "fraction" | "exponent" | "sign" | "digits";
export type OwnedUiSceneJsonToken = { readonly kind: "object" | "array" | "end-object" | "end-array" | "key" | "string" | "number" | "true" | "false" | "null"; readonly start: number; readonly length: number; readonly depth: number };
export type OwnedUiSceneJsonStep = RetainedUiWireStep | { readonly kind: "token"; readonly token: OwnedUiSceneJsonToken; readonly items: number; readonly bytes: number };
const admitted = (grant: Grant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
const whitespace = (byte: number): boolean => byte === 32 || byte === 9 || byte === 10 || byte === 13;
const digit = (byte: number): boolean => byte >= 48 && byte <= 57;
const hex = (byte: number): boolean => digit(byte) || (byte >= 65 && byte <= 70) || (byte >= 97 && byte <= 102);
//#endregion 🧬️OwnedJsonContract

//#region 🧵️OwnedJsonCursor
/** 🧩️ Provisional JSON tokens borrow spans of an exact captured prepared field until explicit close. */
export class OwnedUiSceneJsonCursor {
  readonly #field: number;
  #source: OwnedUiPreparedScene | null;
  #reader: OwnedUiSceneReader | null = null;
  #readerClose: OwnedUiSceneRetirement | null = null;
  #sourceClose: OwnedUiPreparedSceneRetirement | null = null;
  #chunk: Uint8Array | null = null;
  #chunkOffset = 0;
  #offset = 0;
  #eof = false;
  #frames: Frame | null = null;
  #depth = 0;
  #rootDone = false;
  #mode: "idle" | "string" | "number" | "literal" = "idle";
  #start = 0;
  #key = false;
  #escape = false;
  #hex = 0;
  #utf8Remaining = 0;
  #utf8Value = 0;
  #utf8Minimum = 0;
  #number: NumberState = "zero";
  #literal: "true" | "false" | "null" = "null";
  #literalOffset = 0;
  #failure: string | null = null;
  #closing = false;
  #ready = false;
  constructor(source: OwnedUiPreparedScene, field: number) {
    if (!Number.isSafeInteger(field) || field < 0) throw new Error("JSON requires an exact prepared text field");
    this.#field = field; this.#source = OwnedUiPreparedScene.prototype.capture.call(source); Object.freeze(this);
  }
  get offset(): number { return this.#offset; }
  get failure(): string | null { return this.#failure; }
  #consume(): void { this.#chunkOffset++; this.#offset++; }
  #finishValue(): void { if (this.#frames) this.#frames.state = "comma"; else this.#rootDone = true; }
  #token(kind: OwnedUiSceneJsonToken["kind"], start = this.#start): OwnedUiSceneJsonStep {
    return { kind: "token", token: Object.freeze({ kind, start, length: this.#offset - start, depth: this.#depth }), items: 1, bytes: 128 };
  }
  #reject(reason: string): RetainedUiWireStep { this.#failure = reason; return step("rejected", "scene-json", 128); }
  #numberComplete(): boolean { return this.#number === "zero" || this.#number === "integer" || this.#number === "fraction" || this.#number === "digits"; }

  //#region 🔤️LexicalSteps
  #stringByte(byte: number): OwnedUiSceneJsonStep {
    this.#consume();
    if (this.#utf8Remaining) {
      if ((byte & 0xc0) !== 0x80) return this.#reject("Invalid JSON UTF-8 continuation");
      this.#utf8Value = this.#utf8Value * 64 + (byte & 63);
      if (--this.#utf8Remaining === 0 && (this.#utf8Value < this.#utf8Minimum || this.#utf8Value > 0x10ffff || (this.#utf8Value >= 0xd800 && this.#utf8Value <= 0xdfff))) return this.#reject("Invalid JSON Unicode scalar");
    } else if (this.#hex) {
      if (!hex(byte)) return this.#reject("Invalid JSON Unicode escape"); this.#hex--;
    } else if (this.#escape) {
      this.#escape = false;
      if (byte === 117) this.#hex = 4;
      else if (byte !== 34 && byte !== 92 && byte !== 47 && byte !== 98 && byte !== 102 && byte !== 110 && byte !== 114 && byte !== 116) return this.#reject("Invalid JSON escape");
    } else if (byte === 34) {
      this.#mode = "idle";
      if (this.#key) this.#frames!.state = "colon"; else this.#finishValue();
      return this.#token(this.#key ? "key" : "string");
    } else if (byte === 92) this.#escape = true;
    else if (byte < 32) return this.#reject("Unescaped JSON control byte");
    else if (byte >= 128) {
      if (byte >= 0xc2 && byte <= 0xdf) { this.#utf8Remaining = 1; this.#utf8Value = byte & 31; this.#utf8Minimum = 128; }
      else if (byte >= 0xe0 && byte <= 0xef) { this.#utf8Remaining = 2; this.#utf8Value = byte & 15; this.#utf8Minimum = 2048; }
      else if (byte >= 0xf0 && byte <= 0xf4) { this.#utf8Remaining = 3; this.#utf8Value = byte & 7; this.#utf8Minimum = 65536; }
      else return this.#reject("Invalid JSON UTF-8 leader");
    }
    return step("pending", "scene-json-string", 32);
  }
  #numberByte(byte: number): OwnedUiSceneJsonStep {
    let next: NumberState | null = null;
    if (this.#number === "minus" && digit(byte)) next = byte === 48 ? "zero" : "integer";
    else if (this.#number === "integer" && digit(byte)) next = "integer";
    else if ((this.#number === "zero" || this.#number === "integer") && byte === 46) next = "dot";
    else if ((this.#number === "dot" || this.#number === "fraction") && digit(byte)) next = "fraction";
    else if ((this.#number === "zero" || this.#number === "integer" || this.#number === "fraction") && (byte === 101 || byte === 69)) next = "exponent";
    else if (this.#number === "exponent" && (byte === 43 || byte === 45)) next = "sign";
    else if ((this.#number === "exponent" || this.#number === "sign" || this.#number === "digits") && digit(byte)) next = "digits";
    if (next) { this.#number = next; this.#consume(); return step("pending", "scene-json-number", 32); }
    if (!this.#numberComplete()) return this.#reject("Incomplete JSON number");
    this.#mode = "idle"; this.#finishValue(); return this.#token("number");
  }
  #literalByte(byte: number): OwnedUiSceneJsonStep {
    this.#consume();
    if (byte !== this.#literal.charCodeAt(this.#literalOffset++)) return this.#reject("Invalid JSON literal");
    if (this.#literalOffset !== this.#literal.length) return step("pending", "scene-json-literal", 32);
    this.#mode = "idle"; this.#finishValue(); return this.#token(this.#literal);
  }
  //#endregion 🔤️LexicalSteps

  //#region 🌳️GrammarSteps
  #closeContainer(byte: number): OwnedUiSceneJsonStep {
    const frame = this.#frames!;
    if (byte !== (frame.kind === "object" ? 125 : 93)) return this.#reject("Mismatched JSON container");
    const start = this.#offset; this.#consume(); this.#frames = frame.parent; frame.parent = null; this.#depth--; this.#finishValue();
    return this.#token(frame.kind === "object" ? "end-object" : "end-array", start);
  }
  #idleByte(byte: number): OwnedUiSceneJsonStep {
    if (whitespace(byte)) { this.#consume(); return step("pending", "scene-json-whitespace", 16); }
    const frame = this.#frames;
    if (!frame && this.#rootDone) return this.#reject("Trailing JSON content");
    if (frame?.state === "colon") {
      if (byte !== 58) return this.#reject("Missing JSON colon"); frame.state = "value"; this.#consume(); return step("pending", "scene-json-colon", 16);
    }
    if (frame?.state === "comma") {
      if (byte === 125 || byte === 93) return this.#closeContainer(byte);
      if (byte !== 44) return this.#reject("Missing JSON separator");
      frame.state = frame.kind === "object" ? "key" : "value"; this.#consume(); return step("pending", "scene-json-comma", 16);
    }
    if (frame?.state === "first" && (byte === 125 || byte === 93)) return this.#closeContainer(byte);
    const key = frame?.kind === "object" && (frame.state === "first" || frame.state === "key");
    if (key && byte !== 34) return this.#reject("JSON object requires a string key");
    this.#start = this.#offset;
    if (byte === 34) { this.#key = key; this.#mode = "string"; this.#consume(); return step("pending", "scene-json-string", 32); }
    if (byte === 123 || byte === 91) {
      this.#consume(); const kind = byte === 123 ? "object" : "array";
      this.#frames = { kind, state: "first", parent: frame }; this.#depth++; return this.#token(kind);
    }
    if (byte === 45 || digit(byte)) { this.#mode = "number"; this.#number = byte === 45 ? "minus" : byte === 48 ? "zero" : "integer"; this.#consume(); return step("pending", "scene-json-number", 32); }
    if (byte === 116 || byte === 102 || byte === 110) { this.#mode = "literal"; this.#literal = byte === 116 ? "true" : byte === 102 ? "false" : "null"; this.#literalOffset = 0; return this.#literalByte(byte); }
    return this.#reject("Invalid JSON value");
  }
  #end(): OwnedUiSceneJsonStep {
    if (this.#mode === "number" && this.#numberComplete()) { this.#mode = "idle"; this.#finishValue(); return this.#token("number"); }
    if (this.#mode !== "idle" || this.#frames || !this.#rootDone) return this.#reject("Incomplete JSON document");
    this.#ready = true; return step("ready", "scene-json");
  }
  //#endregion 🌳️GrammarSteps

  advance(grant: Grant): OwnedUiSceneJsonStep {
    if (!admitted(grant)) return step("blocked", "scene-json");
    if (this.#closing || this.#failure) return step("rejected", "scene-json");
    if (this.#ready) return step("ready", "scene-json");
    try {
      if (!this.#reader) { this.#reader = this.#source!.beginTextBytes(this.#field); return step("pending", "scene-json-reader", 128); }
      if (this.#chunk && this.#chunkOffset === this.#chunk.length) { const length = this.#chunk.length; this.#chunk = null; this.#chunkOffset = 0; return step("pending", "scene-json-chunk-close", length + 32); }
      if (!this.#chunk) {
        if (this.#eof) return this.#end();
        const current = this.#reader.advance(grant);
        if (current.kind === "bytes") { this.#chunk = current.value; this.#chunkOffset = 0; }
        if (current.kind === "complete") this.#eof = true;
        if (current.kind === "rejected") this.#failure = this.#reader.failure ?? "JSON source read rejected";
        if (current.kind === "blocked") return current;
        return step(current.kind === "rejected" ? "rejected" : "pending", "scene-json-read", current.bytes);
      }
      const byte = this.#chunk[this.#chunkOffset]!;
      if (this.#mode === "string") return this.#stringByte(byte);
      if (this.#mode === "number") return this.#numberByte(byte);
      if (this.#mode === "literal") return this.#literalByte(byte);
      return this.#idleByte(byte);
    } catch (error) { return this.#reject(error instanceof Error ? error.message : "JSON source failed"); }
  }

  //#region ♻️TerminalOwnership
  beginClose(): void { this.#closing = true; }
  closeStep(grant: Grant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "scene-json-close");
    this.#closing = true;
    if (this.#frames) { const frame = this.#frames; this.#frames = frame.parent; frame.parent = null; this.#depth--; return step("pending", "scene-json-frame-close", 64); }
    if (this.#chunk) { const length = this.#chunk.length; this.#chunk = null; this.#chunkOffset = 0; return step("pending", "scene-json-chunk-close", length + 32); }
    if (this.#readerClose) { const current = this.#readerClose.advance(grant); if (current.kind === "complete") this.#readerClose = null; return current.kind === "rejected" || current.kind === "blocked" ? current : step("pending", "scene-json-reader-close", current.bytes); }
    if (this.#reader) { this.#readerClose = this.#reader.beginClose(); this.#reader = null; return step("pending", "scene-json-reader-close", 128); }
    if (this.#sourceClose) { const current = this.#sourceClose.advance(grant); if (current.kind === "complete") this.#sourceClose = null; return current.kind === "rejected" || current.kind === "blocked" ? current : step("pending", "scene-json-source-close", current.bytes); }
    if (this.#source) { this.#sourceClose = this.#source.beginClose(); this.#source = null; return step("pending", "scene-json-source-close", 128); }
    return step("complete", "scene-json-close");
  }
  terminalIsEmpty(): boolean { return this.#closing && this.#frames === null && this.#chunk === null && this.#reader === null && this.#readerClose === null && this.#source === null && this.#sourceClose === null; }
  //#endregion ♻️TerminalOwnership
}
//#endregion 🧵️OwnedJsonCursor
