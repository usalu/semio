//#region 🧬️OwnedGenericPackContract
import { NumericIndex, type NumericIndexEdit, type NumericIndexReader } from "../../../../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️.ts";
import { OwnedUiPackedSceneField } from "../🔤️base64/🟦️.ts";
import type { RetainedUiWireStep } from "../../../../📦️wire/🟦️.ts";

type Grant = { readonly maxItems: number; readonly maxBytes: number };
type Span = { readonly start: number; readonly length: number };
export type OwnedUiGenericPackToken =
  | { readonly kind: "field"; readonly field: number }
  | { readonly kind: "null" | "true" | "false" | "array" | "map" | "end-array" | "end-map" }
  | ({ readonly kind: "key" | "string" } & Span)
  | ({ readonly kind: "number"; readonly value: number } & Span);
type Frame = { readonly kind: "array" | "map"; remaining: number; key: boolean; parent: Frame | null };
type Retirement = { advance(grant: Grant): { readonly kind: string; readonly items: number; readonly bytes: number }; terminalIsEmpty(): boolean };
type Link = { owner: Retirement | null; next: Link | null; complete: boolean };
type Root = { references: number; source: OwnedUiPackedSceneField | null; records: NumericIndex<OwnedUiGenericPackToken> | null };
type Program<T> = Generator<number | RetainedUiWireStep, T, void>;
const GRANT = Object.freeze({ maxItems: 1, maxBytes: 4096 });
const MINT = Object.freeze({});
const admitted = (grant: Grant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
const charge = (current: { readonly kind: string; readonly bytes: number }): number => current.bytes + (current.kind === "retired" ? 96 : 0);
function forward(current: { readonly kind: string; readonly items: number; readonly bytes: number }, phase: string, terminal = false): RetainedUiWireStep {
  const bytes = charge(current);
  const invalid = !Number.isSafeInteger(bytes) || bytes < 0 || bytes > 4096 || !Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1;
  return { kind: invalid || current.kind === "rejected" ? "rejected" : current.kind === "blocked" ? "blocked" : terminal && current.kind === "complete" ? "complete" : "pending", phase, items: current.items, bytes };
}
let ownDocument: (root: Root) => OwnedUiGenericPackDocument;
let ownReader: (root: Root, reader: NumericIndexReader<OwnedUiGenericPackToken>) => OwnedUiGenericPackReader;
let ownRetirement: (root: Root, reader?: Retirement | null) => OwnedUiGenericPackRetirement;
//#endregion 🧬️OwnedGenericPackContract

//#region 🎟️GenericPackOwnership
export class OwnedUiGenericPackDocument {
  #root: Root | null;
  private constructor(mint: object, root: Root) { if (mint !== MINT) throw new Error("Generic pack document requires exact mint authority"); this.#root = root; Object.freeze(this); }
  static { ownDocument = root => new OwnedUiGenericPackDocument(MINT, root); }
  #live(): Root { if (!this.#root) throw new Error("Generic pack document is closed"); return this.#root; }
  captureSource(): OwnedUiPackedSceneField { return this.#live().source!.capture(); }
  beginRead(): OwnedUiGenericPackReader {
    const root = this.#live(); if (root.references === Number.MAX_SAFE_INTEGER) throw new Error("Generic pack reader capacity exceeded");
    const reader = root.records!.beginRead(); root.references++; return ownReader(root, reader);
  }
  beginClose(): OwnedUiGenericPackRetirement { const root = this.#live(); this.#root = null; return ownRetirement(root); }
  terminalIsEmpty(): boolean { return this.#root === null; }
}
export class OwnedUiGenericPackReader {
  #root: Root | null;
  #reader: NumericIndexReader<OwnedUiGenericPackToken> | null;
  #failed = false;
  private constructor(mint: object, root: Root, reader: NumericIndexReader<OwnedUiGenericPackToken>) { if (mint !== MINT) throw new Error("Generic pack reader requires exact mint authority"); this.#root = root; this.#reader = reader; Object.freeze(this); }
  static { ownReader = (root, reader) => new OwnedUiGenericPackReader(MINT, root, reader); }
  advance(grant: Grant): RetainedUiWireStep | { readonly kind: "value"; readonly value: OwnedUiGenericPackToken; readonly items: number; readonly bytes: number } {
    if (!admitted(grant)) return step("blocked", "generic-pack-read");
    if (!this.#reader || this.#failed) return step("rejected", "generic-pack-read");
    const current = this.#reader.advance(GRANT);
    const result = forward(current, "generic-pack-read", true); if (result.kind === "rejected") this.#failed = true;
    return current.kind === "value" && !this.#failed ? current : result;
  }
  beginClose(): OwnedUiGenericPackRetirement { if (!this.#root || !this.#reader) throw new Error("Generic pack reader is closed"); const result = ownRetirement(this.#root, this.#reader.beginClose()); this.#root = null; this.#reader = null; return result; }
  terminalIsEmpty(): boolean { return this.#root === null && this.#reader === null; }
}
export class OwnedUiGenericPackRetirement {
  #root: Root | null;
  #reader: Retirement | null;
  #records: Retirement | null = null;
  #source: Retirement | null = null;
  #released = false;
  private constructor(mint: object, root: Root, reader: Retirement | null) { if (mint !== MINT) throw new Error("Generic pack retirement requires exact mint authority"); this.#root = root; this.#reader = reader; Object.freeze(this); }
  static { ownRetirement = (root, reader = null) => new OwnedUiGenericPackRetirement(MINT, root, reader); }
  advance(grant: Grant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "generic-pack-close");
    if (this.#reader) { const current = this.#reader.advance(GRANT); const result = forward(current, "generic-pack-reader-close"); if (current.kind === "complete" && result.kind === "pending") this.#reader = null; return result; }
    if (this.#root && !this.#released) { this.#released = true; if (--this.#root.references) this.#root = null; return step("pending", "generic-pack-release", 32); }
    if (this.#records) { const current = this.#records.advance(GRANT); const result = forward(current, "generic-pack-record-close"); if (current.kind === "complete" && result.kind === "pending") this.#records = null; return result; }
    if (this.#root?.records) { this.#records = this.#root.records.beginClose(); this.#root.records = null; return step("pending", "generic-pack-record-close", 64); }
    if (this.#source) { const current = this.#source.advance(GRANT); const result = forward(current, "generic-pack-source-close"); if (current.kind === "complete" && result.kind === "pending") this.#source = null; return result; }
    if (this.#root?.source) { this.#source = this.#root.source.beginClose(); this.#root.source = null; return step("pending", "generic-pack-source-close", 64); }
    this.#root = null; return step("complete", "generic-pack-close");
  }
  terminalIsEmpty(): boolean { return this.#root === null && this.#reader === null && this.#records === null && this.#source === null; }
}
//#endregion 🎟️GenericPackOwnership

//#region 🧵️GenericPackGrammar
/** 🧾️ Keeps native generic-pack values as immutable token and text spans over exact captured pages. */
export class OwnedUiGenericPackCursor {
  #source: OwnedUiPackedSceneField | null;
  #symbols: NumericIndex<Span> | null = NumericIndex.empty<Span>();
  #records: NumericIndex<OwnedUiGenericPackToken> | null = NumericIndex.empty<OwnedUiGenericPackToken>();
  #symbolEdit: NumericIndexEdit<Span> | null = null;
  #recordEdit: NumericIndexEdit<OwnedUiGenericPackToken> | null = null;
  #symbolReader: NumericIndexReader<Span> | null = null;
  #retirements: Link | null = null;
  #frames: Frame | null = null;
  #program: Program<void> | null = null;
  #offset = 0;
  #phase = "generic-pack-start";
  #ready = false;
  #closing = false;
  #taken = false;
  #failure: string | null = null;
  readonly #float = new Uint8Array(8);
  constructor(source: OwnedUiPackedSceneField) { this.#source = OwnedUiPackedSceneField.prototype.capture.call(source); Object.freeze(this); }
  get offset(): number { return this.#offset; }
  get failure(): string | null { return this.#failure; }
  #byte(): number { if (this.#offset >= this.#source!.length) throw new Error("Truncated generic pack"); return this.#source!.byteAt(this.#offset++); }
  #queue(owner: Retirement): void { this.#retirements = { owner, next: this.#retirements, complete: false }; }
  #drain(): RetainedUiWireStep {
    if (this.#retirements!.complete) { const link = this.#retirements!; this.#retirements = link.next; link.next = null; link.owner = null; return step("pending", "generic-pack-owner-close", 32); }
    const link = this.#retirements!; const current = link.owner!.advance(GRANT);
    const result = forward(current, "generic-pack-owner-close"); if (result.kind === "rejected" || result.kind === "blocked") return result;
    if (current.kind === "complete") { if (!link.owner!.terminalIsEmpty()) return { ...result, kind: "rejected" }; link.complete = true; }
    return result;
  }
  *#natural(): Program<number> {
    let value = 0; let factor = 1;
    for (let count = 0; count < 8; count++) { const byte = this.#byte(); value += (byte & 127) * factor; yield 1; if (!Number.isSafeInteger(value)) throw new Error("Generic pack integer exceeds exact admitted range"); if (byte < 128) return value; factor *= 128; }
    throw new Error("Generic pack varint overflow");
  }
  *#text(): Program<Span> {
    const length = yield* this.#natural(); if (length > this.#source!.length - this.#offset) throw new Error("Generic pack string exceeds exact source");
    const start = this.#offset; let remaining = 0; let scalar = 0; let minimum = 0;
    for (let index = 0; index < length; index++) {
      const byte = this.#byte();
      if (remaining) { if ((byte & 0xc0) !== 0x80) throw new Error("Invalid generic pack UTF-8 continuation"); scalar = scalar * 64 + (byte & 63); if (--remaining === 0 && (scalar < minimum || scalar > 0x10ffff || (scalar >= 0xd800 && scalar <= 0xdfff))) throw new Error("Invalid generic pack Unicode scalar"); }
      else if (byte >= 128) { if (byte >= 0xc2 && byte <= 0xdf) { remaining = 1; scalar = byte & 31; minimum = 128; } else if (byte >= 0xe0 && byte <= 0xef) { remaining = 2; scalar = byte & 15; minimum = 2048; } else if (byte >= 0xf0 && byte <= 0xf4) { remaining = 3; scalar = byte & 7; minimum = 65536; } else throw new Error("Invalid generic pack UTF-8 leader"); }
      yield 32;
    }
    if (remaining) throw new Error("Truncated generic pack Unicode scalar"); return Object.freeze({ start, length });
  }
  *#saveSymbol(id: number, value: Span): Program<void> {
    this.#phase = "generic-pack-symbol-index"; this.#symbolEdit = this.#symbols!.beginSet(id, value); yield 128;
    for (;;) { const current = this.#symbolEdit.advance(GRANT); yield forward(current, this.#phase); if (current.kind === "ready") break; if (current.kind === "rejected") throw new Error("Generic pack symbol index rejected"); }
    const next = this.#symbolEdit.takeResult()!; this.#queue(this.#symbolEdit.beginClose()); this.#symbolEdit = null; this.#queue(this.#symbols!.beginClose()); this.#symbols = next; yield 128;
    while (this.#retirements) yield this.#drain();
  }
  *#emit(value: OwnedUiGenericPackToken): Program<void> {
    this.#phase = "generic-pack-token-index"; this.#recordEdit = this.#records!.beginSet(this.#records!.size, Object.freeze(value)); yield 128;
    for (;;) { const current = this.#recordEdit.advance(GRANT); yield forward(current, this.#phase); if (current.kind === "ready") break; if (current.kind === "rejected") throw new Error("Generic pack token index rejected"); }
    const next = this.#recordEdit.takeResult()!; this.#queue(this.#recordEdit.beginClose()); this.#recordEdit = null; this.#queue(this.#records!.beginClose()); this.#records = next; yield 128;
    while (this.#retirements) yield this.#drain();
  }
  *#string(tag: number): Program<Span> {
    if (tag === 7) return yield* this.#text();
    if (tag !== 6) throw new Error("Generic pack map key requires a string");
    const id = yield* this.#natural(); this.#symbolReader = this.#symbols!.beginLookup(id); let value: Span | null = null; yield 64;
    for (;;) { const current = this.#symbolReader.advance(GRANT); if (current.kind === "value") value = current.value; yield forward(current, this.#phase); if (current.kind === "complete") break; }
    this.#queue(this.#symbolReader.beginClose()); this.#symbolReader = null; yield 64; while (this.#retirements) yield this.#drain();
    if (!value) throw new Error("Generic pack symbol is missing"); return value;
  }
  #finishValue(): boolean { if (!this.#frames) return true; this.#frames.remaining--; this.#frames.key = this.#frames.kind === "map"; return false; }
  *#value(): Program<void> {
    let done = false;
    while (!done) {
      if (this.#frames?.remaining === 0) { const frame = this.#frames; this.#frames = frame.parent; frame.parent = null; yield 64; yield* this.#emit({ kind: frame.kind === "array" ? "end-array" : "end-map" }); done = this.#finishValue(); continue; }
      if (this.#frames?.kind === "map" && this.#frames.key) { const tag = this.#byte(); yield 1; const span = yield* this.#string(tag); yield* this.#emit({ kind: "key", ...span }); this.#frames.key = false; yield 16; continue; }
      this.#phase = "generic-pack-value"; const tag = this.#byte(); yield 1;
      if (tag === 12 || tag === 16) { const count = yield* this.#natural(); if (count > this.#source!.length - this.#offset) throw new Error("Generic pack collection exceeds exact source"); const kind = tag === 12 ? "array" : "map"; this.#frames = { kind, remaining: count, key: kind === "map", parent: this.#frames }; yield 64; yield* this.#emit({ kind }); continue; }
      if (tag === 18 || tag === 1 || tag === 2) yield* this.#emit({ kind: tag === 18 ? "null" : tag === 1 ? "false" : "true" });
      else if (tag === 5) { const start = this.#offset; for (let index = 0; index < 8; index++) { this.#float[index] = this.#byte(); yield 1; } const value = new DataView(this.#float.buffer).getFloat64(0, true); yield 32; yield* this.#emit({ kind: "number", value, start, length: 8 }); }
      else if (tag === 6 || tag === 7) { const span = yield* this.#string(tag); yield* this.#emit({ kind: "string", ...span }); }
      else throw new Error("Unsupported generic pack value tag");
      done = this.#finishValue(); yield 16;
    }
  }
  *#run(): Program<void> {
    const symbols = yield* this.#natural(); if (symbols > this.#source!.length - this.#offset) throw new Error("Generic pack symbol count exceeds source");
    for (let index = 0; index < symbols; index++) { const span = yield* this.#text(); yield* this.#saveSymbol(index, span); }
    const fields = yield* this.#natural(); if (fields > this.#source!.length - this.#offset) throw new Error("Generic pack field count exceeds source");
    for (let index = 0; index < fields; index++) { const field = yield* this.#natural(); const tag = this.#byte(); yield 1; if (tag !== 17) throw new Error("Generic scene field requires DSL value tag"); yield* this.#emit({ kind: "field", field }); yield* this.#value(); }
    if (this.#offset !== this.#source!.length) throw new Error("Trailing generic pack bytes");
    this.#queue(this.#symbols!.beginClose()); this.#symbols = null; yield 64; while (this.#retirements) yield this.#drain();
  }
  advance(grant: Grant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", this.#phase);
    if (this.#closing || this.#failure || this.#taken) return step("rejected", this.#phase);
    if (this.#ready) return step("ready", this.#phase);
    try { if (!this.#program) { this.#program = this.#run(); return step("pending", this.#phase, 64); } const current = this.#program.next(); if (current.done) { this.#ready = true; this.#program = null; return step("ready", this.#phase); } const result = typeof current.value === "number" ? forward({ kind: "pending", items: current.value ? 1 : 0, bytes: current.value }, this.#phase) : current.value; if (result.kind === "rejected") this.#failure = "Generic pack child rejected or exceeded grant"; return result; }
    catch (error) { this.#failure = error instanceof Error ? error.message : "Generic pack failed"; return step("rejected", this.#phase, 128); }
  }
  takeResult(): OwnedUiGenericPackDocument | null {
    if (!this.#ready || this.#closing || this.#failure || this.#taken || !this.#source || !this.#records) return null;
    const root: Root = { references: 1, source: this.#source, records: this.#records }; this.#source = null; this.#records = null; this.#taken = true; return ownDocument(root);
  }
  beginClose(): void { this.#closing = true; }
  closeStep(grant: Grant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "generic-pack-cursor-close"); this.#closing = true;
    if (this.#program) { this.#program = null; return step("pending", "generic-pack-program-close", 64); }
    if (this.#frames) { const frame = this.#frames; this.#frames = frame.parent; frame.parent = null; return step("pending", "generic-pack-frame-close", 64); }
    if (this.#retirements) return this.#drain();
    if (this.#symbolReader) { this.#queue(this.#symbolReader.beginClose()); this.#symbolReader = null; return step("pending", "generic-pack-symbol-reader-close", 64); }
    if (this.#symbolEdit) { this.#queue(this.#symbolEdit.beginClose()); this.#symbolEdit = null; return step("pending", "generic-pack-symbol-edit-close", 64); }
    if (this.#recordEdit) { this.#queue(this.#recordEdit.beginClose()); this.#recordEdit = null; return step("pending", "generic-pack-record-edit-close", 64); }
    if (this.#symbols) { this.#queue(this.#symbols.beginClose()); this.#symbols = null; return step("pending", "generic-pack-symbol-close", 64); }
    if (this.#records) { this.#queue(this.#records.beginClose()); this.#records = null; return step("pending", "generic-pack-record-close", 64); }
    if (this.#source) { this.#queue(this.#source.beginClose()); this.#source = null; return step("pending", "generic-pack-source-close", 64); }
    return step("complete", "generic-pack-cursor-close");
  }
  terminalIsEmpty(): boolean { return this.#closing && this.#program === null && this.#frames === null && this.#retirements === null && this.#symbolReader === null && this.#symbolEdit === null && this.#recordEdit === null && this.#symbols === null && this.#records === null && this.#source === null; }
}
//#endregion 🧵️GenericPackGrammar
