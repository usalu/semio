//#region 🧬️SceneContract
import { NumericIndex, type NumericIndexEdit, type NumericIndexGrant, type NumericIndexReader } from "../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️component.ts";
import { captureTypedUiPayload, type OwnedUiPayload, type RetainedUiComponent, type UiPayloadRetirement, type UiSurfaceByteView } from "../📦️wire/🧾️typed/🟦️component.ts";
import type { RetainedUiWireStep } from "../📦️wire/🟦️component.ts";

type Span = { readonly start: number; readonly end: number };
export type OwnedUiSceneValue = Span & (
  | { readonly kind: "unit" | "none" }
  | { readonly kind: "boolean"; readonly value: boolean }
  | { readonly kind: "integer"; readonly value: bigint }
  | { readonly kind: "float"; readonly value: number }
  | { readonly kind: "char"; readonly value: string }
  | { readonly kind: "text" | "bytes"; readonly offset: number; readonly length: number }
  | { readonly kind: "some" | "variant"; readonly first: number }
  | { readonly kind: "sequence" | "map"; readonly first: number; readonly count: number }
);
export type OwnedUiSceneReadStep = RetainedUiWireStep
  | { readonly kind: "value"; readonly value: OwnedUiSceneValue; readonly items: number; readonly bytes: number }
  | { readonly kind: "text"; readonly value: string; readonly items: number; readonly bytes: number }
  | { readonly kind: "bytes"; readonly value: Uint8Array; readonly items: number; readonly bytes: number };
type Entry = { readonly value: OwnedUiSceneValue; readonly collision: number | null };
type Frame = { readonly start: number; readonly kind: "some" | "variant" | "sequence" | "map"; readonly first: number; readonly count: number; remaining: number; parent: Frame | null };
type Root = { references: number; index: NumericIndex<Entry> | null; source: OwnedUiPayload<RetainedUiComponent> | null };
type Retirement = { advance(grant: NumericIndexGrant): { readonly kind: string; readonly items: number; readonly bytes: number }; terminalIsEmpty(): boolean };
type RetireLink = { owner: Retirement | null; next: RetireLink | null };
type Program<T> = Generator<number, T, void>;
type ReadMode = "value" | "text" | "bytes" | "text-bytes";
const GRANT = Object.freeze({ maxItems: 1, maxBytes: 4096 });
const MINT = Object.freeze({});
const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
let ownDocument: (root: Root) => OwnedUiSceneDocument;
let ownReader: (root: Root, reader: NumericIndexReader<Entry>, mode: ReadMode, offset: number, length: number | null) => OwnedUiSceneReader;
let ownRetirement: (root: Root | null, reader?: Retirement | null) => OwnedUiSceneRetirement;
function sourceComponent(root: Root) { const value = root.source?.value; if (value?.type !== "surface") throw new Error("Scene source owner is closed"); return value; }
function sourceBytes(root: Root): UiSurfaceByteView { return sourceComponent(root).doc.bytes; }
function captureRoot(root: Root): Root { if (root.references === Number.MAX_SAFE_INTEGER || !root.source || !root.index) throw new Error("Scene root cannot be captured"); root.references++; return root; }
//#endregion 🧬️SceneContract

//#region 🔤️UnicodeScalar
class Utf8Scalar {
  #remaining = 0;
  #value = 0;
  #minimum = 0;
  get complete(): boolean { return this.#remaining === 0; }
  push(byte: number): number | null {
    if (this.#remaining) {
      if ((byte & 0xc0) !== 0x80) throw new Error("Invalid scene UTF-8 continuation");
      this.#value = this.#value * 64 + (byte & 63);
      if (--this.#remaining) return null;
      if (this.#value < this.#minimum || this.#value > 0x10ffff || (this.#value >= 0xd800 && this.#value <= 0xdfff)) throw new Error("Invalid scene Unicode scalar");
      return this.#value;
    }
    if (byte < 128) return byte;
    if (byte >= 0xc2 && byte <= 0xdf) { this.#remaining = 1; this.#value = byte & 31; this.#minimum = 128; return null; }
    if (byte >= 0xe0 && byte <= 0xef) { this.#remaining = 2; this.#value = byte & 15; this.#minimum = 2048; return null; }
    if (byte >= 0xf0 && byte <= 0xf4) { this.#remaining = 3; this.#value = byte & 7; this.#minimum = 65536; return null; }
    throw new Error("Invalid scene UTF-8 leader");
  }
}
//#endregion 🔤️UnicodeScalar

//#region 🎬️CapturedScene
/** 🎬️ Flat immutable scene records retain the exact component pages until all readers close. */
export class OwnedUiSceneDocument {
  #root: Root | null;
  private constructor(mint: object, root: Root) { if (mint !== MINT) throw new Error("Scene document requires exact mint authority"); this.#root = root; Object.freeze(this); }
  static { ownDocument = root => new OwnedUiSceneDocument(MINT, root); }
  #live(): Root { if (!this.#root) throw new Error("Scene document is closed"); return this.#root; }
  get size(): number { return this.#live().index!.size; }
  get kind(): string { return sourceComponent(this.#live()).kind; }
  get schema(): string { return sourceComponent(this.#live()).docSchema; }
  capture(): OwnedUiSceneDocument { return ownDocument(captureRoot(this.#live())); }
  #read(id: number, mode: ReadMode, offset = 0, length: number | null = null): OwnedUiSceneReader {
    const root = this.#live();
    if (!Number.isSafeInteger(id) || id < 0 || root.references === Number.MAX_SAFE_INTEGER) throw new Error("Scene read cannot be admitted");
    if (!Number.isSafeInteger(offset) || offset < 0 || (length !== null && (!Number.isSafeInteger(length) || length < 0))) throw new Error("Invalid scene text byte range");
    const reader = root.index!.beginLookup(id);
    return ownReader(captureRoot(root), reader, mode, offset, length);
  }
  beginRead(id = 0): OwnedUiSceneReader { return this.#read(id, "value"); }
  beginText(id: number): OwnedUiSceneReader { return this.#read(id, "text"); }
  beginBytes(id: number): OwnedUiSceneReader { return this.#read(id, "bytes"); }
  beginTextBytes(id: number, offset = 0, length?: number): OwnedUiSceneReader { return this.#read(id, "text-bytes", offset, length ?? null); }
  beginClose(): OwnedUiSceneRetirement { const root = this.#live(); this.#root = null; return ownRetirement(root); }
  terminalIsEmpty(): boolean { return this.#root === null; }
}

export class OwnedUiSceneReader {
  #root: Root | null;
  #reader: NumericIndexReader<Entry> | null;
  #readerClose: Retirement | null = null;
  #value: OwnedUiSceneValue | null = null;
  #offset = 0;
  #done = false;
  #failure: string | null = null;
  readonly #mode: ReadMode;
  readonly #start: number;
  readonly #length: number | null;
  readonly #utf8 = new Utf8Scalar();
  private constructor(mint: object, root: Root, reader: NumericIndexReader<Entry>, mode: ReadMode, offset: number, length: number | null) { if (mint !== MINT) throw new Error("Scene reader requires exact mint authority"); this.#root = root; this.#reader = reader; this.#mode = mode; this.#start = offset; this.#length = length; Object.freeze(this); }
  static { ownReader = (root, reader, mode, offset, length) => new OwnedUiSceneReader(MINT, root, reader, mode, offset, length); }
  get failure(): string | null { return this.#failure; }
  advance(grant: NumericIndexGrant): OwnedUiSceneReadStep {
    if (!admitted(grant)) return step("blocked", "scene-read");
    if (!this.#root || this.#failure) return step("rejected", "scene-read");
    if (this.#done) return step("complete", "scene-read");
    if (this.#reader) {
      const current = this.#reader.advance(GRANT);
      if (current.kind === "value") this.#value = current.value.value;
      if (current.kind === "complete") { this.#readerClose = this.#reader.beginClose(); this.#reader = null; }
      return step("pending", "scene-read-lookup", current.bytes + 64);
    }
    if (this.#readerClose) { const current = this.#readerClose.advance(GRANT); if (current.kind === "complete") this.#readerClose = null; return step("pending", "scene-read-lookup-close", current.bytes); }
    const value = this.#value;
    if (!value) { this.#done = true; return step("complete", "scene-read"); }
    if (this.#mode === "value") { this.#done = true; return { kind: "value", value, items: 1, bytes: 80 }; }
    if ((value.kind !== "text" && value.kind !== "bytes") || (this.#mode === "text-bytes" ? value.kind !== "text" : value.kind !== this.#mode)) { this.#failure = "Scene read kind mismatch"; return step("rejected", "scene-read-kind", 16); }
    const length = this.#length ?? value.length - this.#start;
    if (this.#start > value.length || length > value.length - this.#start) { this.#failure = "Scene text byte range exceeds its exact field"; return step("rejected", "scene-read-range", 32); }
    if (this.#offset === length) { this.#done = true; return step("complete", "scene-read"); }
    const source = sourceBytes(this.#root);
    if (value.kind === "bytes" || this.#mode === "text-bytes") {
      const count = Math.min(256, length - this.#offset); const bytes = new Uint8Array(count);
      for (let index = 0; index < count; index++) bytes[index] = source.byteAt(value.offset + this.#start + this.#offset++);
      return { kind: "bytes", value: bytes, items: 1, bytes: count * 2 + 32 };
    }
    let text = ""; let read = 0;
    while (read < 128 && this.#offset < value.length) {
      const scalar = this.#utf8.push(source.byteAt(value.offset + this.#offset++)); read++;
      if (scalar !== null) text += String.fromCodePoint(scalar);
    }
    return { kind: "text", value: text, items: 1, bytes: read + text.length * 2 + 64 };
  }
  beginClose(): OwnedUiSceneRetirement {
    if (!this.#root) throw new Error("Scene reader is already closed");
    const retirement = ownRetirement(this.#root, this.#reader ? this.#reader.beginClose() : this.#readerClose);
    this.#root = null; this.#reader = null; this.#readerClose = null; this.#value = null; return retirement;
  }
  terminalIsEmpty(): boolean { return this.#root === null && this.#reader === null && this.#readerClose === null && this.#value === null; }
}

export class OwnedUiSceneRetirement {
  #root: Root | null;
  #reader: Retirement | null;
  #released = false;
  #index: Retirement | null = null;
  #source: UiPayloadRetirement<RetainedUiComponent> | null = null;
  private constructor(mint: object, root: Root | null, reader: Retirement | null) { if (mint !== MINT) throw new Error("Scene retirement requires exact mint authority"); this.#root = root; this.#reader = reader; Object.freeze(this); }
  static { ownRetirement = (root, reader = null) => new OwnedUiSceneRetirement(MINT, root, reader); }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "scene-close");
    if (this.#reader) { const current = this.#reader.advance(GRANT); if (current.kind === "complete") this.#reader = null; return step("pending", "scene-read-close", current.bytes); }
    if (this.#root && !this.#released) { this.#released = true; if (--this.#root.references) this.#root = null; return step("pending", "scene-root-release", 32); }
    if (this.#index) { const current = this.#index.advance(GRANT); if (current.kind === "complete") this.#index = null; return step("pending", "scene-index-close", current.bytes); }
    if (this.#root?.index) { this.#index = this.#root.index.beginClose(); this.#root.index = null; return step("pending", "scene-index-close", 64); }
    if (this.#source) { const current = this.#source.advance(GRANT); if (current.kind === "complete") this.#source = null; return { ...current, kind: "pending" }; }
    if (this.#root?.source) { this.#source = this.#root.source.beginClose(); this.#root.source = null; return step("pending", "scene-source-close", 64); }
    this.#root = null; return step("complete", "scene-close");
  }
  terminalIsEmpty(): boolean { return this.#root === null && this.#reader === null && this.#index === null && this.#source === null; }
}
//#endregion 🎬️CapturedScene

//#region 🧵️ScenePreparation
/** 🧵️ Parses the native scene packet without a whole-buffer copy or recursive value allocation. */
export class OwnedUiSceneCursor {
  #source: OwnedUiPayload<RetainedUiComponent> | null;
  #entries: NumericIndex<Entry> | null = NumericIndex.empty<Entry>();
  #buckets: NumericIndex<number> | null = NumericIndex.empty<number>();
  #entryEdit: NumericIndexEdit<Entry> | null = null;
  #bucketEdit: NumericIndexEdit<number> | null = null;
  #entryReader: NumericIndexReader<Entry> | null = null;
  #bucketReader: NumericIndexReader<number> | null = null;
  #retirements: RetireLink | null = null;
  #frames: Frame | null = null;
  #program: Program<void> | null = null;
  #position = 0;
  #phase = "scene-start";
  #complete = false;
  #closing = false;
  #taken = false;
  #failure: string | null = null;
  constructor(source: OwnedUiPayload<RetainedUiComponent>) {
    if (source.value.type !== "surface") throw new Error("Scene preparation requires a Surface component");
    this.#source = captureTypedUiPayload("component", source); Object.freeze(this);
  }
  get failure(): string | null { return this.#failure; }
  get completedBytes(): number { return this.#position; }
  get completedRecords(): number { return this.#entries?.size ?? 0; }
  #bytes(): UiSurfaceByteView { const value = this.#source?.value; if (value?.type !== "surface") throw new Error("Scene preparation source is closed"); return value.doc.bytes; }
  #byte(): number { if (this.#position >= this.#bytes().length) throw new Error("Truncated scene packet"); return this.#bytes().byteAt(this.#position++); }
  #queue(owner: Retirement): void { this.#retirements = { owner, next: this.#retirements }; }
  #drain(): number {
    const cell = this.#retirements!; const result = cell.owner!.advance(GRANT);
    if (result.kind === "complete") { this.#retirements = cell.next; cell.next = null; cell.owner = null; }
    return result.bytes + 32;
  }
  *#varint(): Program<bigint> {
    let value = 0n;
    for (let index = 0; index < 10; index++) {
      this.#phase = "scene-varint"; const byte = this.#byte(); const digit = byte & 127;
      if (index === 9 && digit > 1) throw new Error("Scene integer exceeds u64");
      if (index > 0 && !(byte & 128) && digit === 0) throw new Error("Noncanonical scene varint");
      value |= BigInt(digit) << BigInt(index * 7); yield 32;
      if (!(byte & 128)) return value;
    }
    throw new Error("Scene varint exceeds ten bytes");
  }
  *#length(): Program<number> { const value = yield* this.#varint(); if (value > BigInt(this.#bytes().length - this.#position)) throw new Error("Scene length exceeds remaining bytes"); return Number(value); }
  *#lookup(id: number): Program<Entry> {
    this.#entryReader = this.#entries!.beginLookup(id); let value: Entry | null = null; yield 64;
    for (;;) { const current = this.#entryReader.advance(GRANT); if (current.kind === "value") value = current.value; yield current.bytes; if (current.kind === "complete") break; }
    this.#queue(this.#entryReader.beginClose()); this.#entryReader = null; yield 64;
    while (this.#retirements) yield this.#drain();
    if (!value) throw new Error("Scene key arena is inconsistent"); return value;
  }
  *#key(value: OwnedUiSceneValue, hash: number): Program<number | null> {
    const frame = this.#frames!;
    if (value.kind !== "text") throw new Error("Scene map key is not text");
    const bucket = frame.start * 4294967296 + hash;
    this.#phase = "scene-key-lookup"; this.#bucketReader = this.#buckets!.beginLookup(bucket); let head: number | null = null; yield 64;
    for (;;) { const current = this.#bucketReader.advance(GRANT); if (current.kind === "value") head = current.value; yield current.bytes; if (current.kind === "complete") break; }
    this.#queue(this.#bucketReader.beginClose()); this.#bucketReader = null; yield 64;
    while (this.#retirements) yield this.#drain();
    let previous = head;
    while (previous !== null) {
      const entry = yield* this.#lookup(previous); const candidate = entry.value;
      if (candidate.kind !== "text") throw new Error("Scene key arena kind mismatch");
      if (candidate.length === value.length) {
        let equal = true; this.#phase = "scene-key-compare";
        for (let offset = 0; offset < value.length; offset++) { const same = this.#bytes().byteAt(candidate.offset + offset) === this.#bytes().byteAt(value.offset + offset); yield 2; if (!same) { equal = false; break; } }
        if (equal) throw new Error("Duplicate scene map key");
      }
      previous = entry.collision; yield 32;
    }
    this.#phase = "scene-key-insert"; this.#bucketEdit = this.#buckets!.beginSet(bucket, value.start); yield 64;
    for (;;) { const current = this.#bucketEdit.advance(GRANT); yield current.bytes; if (current.kind === "ready") break; if (current.kind === "rejected") throw new Error("Scene key ordinal exhausted"); }
    const next = this.#bucketEdit.takeResult()!; this.#queue(this.#bucketEdit.beginClose()); this.#bucketEdit = null; this.#queue(this.#buckets!.beginClose()); this.#buckets = next; yield 128;
    while (this.#retirements) yield this.#drain();
    return head;
  }
  *#save(value: OwnedUiSceneValue, hash = 0): Program<void> {
    const parent = this.#frames; let collision: number | null = null;
    if (parent?.kind === "map" && parent.remaining % 2 === 0) collision = yield* this.#key(value, hash);
    if (parent?.kind === "variant" && parent.remaining === 2 && value.kind !== "text") throw new Error("Scene variant name is not text");
    this.#phase = "scene-record-insert"; this.#entryEdit = this.#entries!.beginSet(value.start, Object.freeze({ value: Object.freeze(value), collision })); yield 160;
    for (;;) { const current = this.#entryEdit.advance(GRANT); yield current.bytes; if (current.kind === "ready") break; if (current.kind === "rejected") throw new Error("Scene record ordinal exhausted"); }
    const next = this.#entryEdit.takeResult()!; this.#queue(this.#entryEdit.beginClose()); this.#entryEdit = null; this.#queue(this.#entries!.beginClose()); this.#entries = next; yield 128;
    while (this.#retirements) yield this.#drain();
    if (parent) parent.remaining--;
  }
  *#parse(): Program<void> {
    let root = false;
    while (!root) {
      const frame = this.#frames;
      if (frame && frame.remaining === 0) {
        this.#frames = frame.parent; frame.parent = null;
        const value: OwnedUiSceneValue = frame.kind === "map" || frame.kind === "sequence" ? { kind: frame.kind, start: frame.start, end: this.#position, first: frame.first, count: frame.count } : { kind: frame.kind, start: frame.start, end: this.#position, first: frame.first };
        yield 96; yield* this.#save(value); root = this.#frames === null; continue;
      }
      this.#phase = "scene-tag"; const start = this.#position; const tag = this.#byte(); yield 16;
      let value: OwnedUiSceneValue; let hash = 2166136261;
      if (tag === 0 || tag === 8) value = { kind: tag === 0 ? "unit" : "none", start, end: this.#position };
      else if (tag === 1 || tag === 2) value = { kind: "boolean", start, end: this.#position, value: tag === 2 };
      else if (tag === 3 || tag === 4 || tag === 11) {
        const raw = yield* this.#varint();
        if (tag === 11) { if (raw > 0x10ffffn || (raw >= 0xd800n && raw <= 0xdfffn)) throw new Error("Invalid scene character"); value = { kind: "char", start, end: this.#position, value: String.fromCodePoint(Number(raw)) }; }
        else value = { kind: "integer", start, end: this.#position, value: tag === 3 ? raw : (raw >> 1n) ^ -(raw & 1n) };
      } else if (tag === 5) {
        const bytes = new Uint8Array(8); yield 24;
        for (let index = 0; index < 8; index++) { bytes[index] = this.#byte(); yield 1; }
        value = { kind: "float", start, end: this.#position, value: new DataView(bytes.buffer).getFloat64(0, true) };
      } else if (tag === 6 || tag === 7) {
        const length = yield* this.#length(); const offset = this.#position;
        if (tag === 6) {
          const utf8 = new Utf8Scalar(); this.#phase = "scene-text"; yield 32;
          for (let index = 0; index < length; index++) { const byte = this.#byte(); utf8.push(byte); hash = Math.imul(hash ^ byte, 16777619) >>> 0; yield 16; }
          if (!utf8.complete) throw new Error("Truncated scene Unicode scalar");
        } else { this.#position += length; yield 32; }
        value = { kind: tag === 6 ? "text" : "bytes", start, end: this.#position, offset, length };
      } else if (tag === 9 || tag === 10 || tag === 12 || tag === 13) {
        const count = tag === 10 || tag === 13 ? yield* this.#length() : 1;
        this.#frames = { start, kind: tag === 9 ? "some" : tag === 10 ? "sequence" : tag === 12 ? "variant" : "map", first: this.#position, count, remaining: tag === 12 ? 2 : tag === 13 ? count * 2 : count, parent: this.#frames };
        this.#phase = "scene-frame"; yield 80; continue;
      } else throw new Error("Unknown scene packet tag");
      yield* this.#save(value, hash); root = this.#frames === null;
    }
    if (this.#position !== this.#bytes().length) throw new Error("Trailing scene packet bytes");
    this.#phase = "scene-prepare-close"; this.#queue(this.#buckets!.beginClose()); this.#buckets = null; yield 64;
    while (this.#retirements) yield this.#drain();
  }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", this.#phase);
    if (this.#closing || this.#taken || this.#failure) return step("rejected", this.#phase);
    if (this.#complete) return step("ready", "scene-ready");
    try {
      this.#program ??= this.#parse(); const current = this.#program.next();
      if (current.done) { this.#program = null; this.#complete = true; return step("ready", "scene-ready", 32); }
      return step("pending", this.#phase, current.value);
    } catch (error) { this.#failure = error instanceof Error ? error.message : "Scene preparation failed"; return step("rejected", this.#phase, 128); }
  }
  takeResult(): OwnedUiSceneDocument | null {
    if (!this.#complete || this.#closing || this.#failure || this.#taken || !this.#source || !this.#entries) return null;
    const document = ownDocument({ references: 1, index: this.#entries, source: this.#source }); this.#entries = null; this.#source = null; this.#taken = true; return document;
  }
  beginClose(): void { this.#closing = true; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "scene-close");
    if (!this.#closing) return step("blocked", "scene-close-not-started");
    if (this.#program) { this.#program.return(undefined); this.#program = null; return step("pending", "scene-program-close", 128); }
    if (this.#frames) { const frame = this.#frames; this.#frames = frame.parent; frame.parent = null; return step("pending", "scene-frame-close", 80); }
    if (this.#entryReader) { this.#queue(this.#entryReader.beginClose()); this.#entryReader = null; return step("pending", "scene-reader-close", 64); }
    if (this.#bucketReader) { this.#queue(this.#bucketReader.beginClose()); this.#bucketReader = null; return step("pending", "scene-reader-close", 64); }
    if (this.#entryEdit) { this.#queue(this.#entryEdit.beginClose()); this.#entryEdit = null; return step("pending", "scene-edit-close", 64); }
    if (this.#bucketEdit) { this.#queue(this.#bucketEdit.beginClose()); this.#bucketEdit = null; return step("pending", "scene-edit-close", 64); }
    if (this.#retirements) return step("pending", "scene-arena-close", this.#drain());
    if (this.#buckets) { this.#queue(this.#buckets.beginClose()); this.#buckets = null; return step("pending", "scene-key-index-close", 64); }
    if (this.#entries) { this.#queue(this.#entries.beginClose()); this.#entries = null; return step("pending", "scene-record-index-close", 64); }
    if (this.#source) { this.#queue(this.#source.beginClose()); this.#source = null; return step("pending", "scene-source-close", 64); }
    return step("complete", "scene-close");
  }
  terminalIsEmpty(): boolean { return this.#closing && this.#program === null && this.#frames === null && this.#entryReader === null && this.#bucketReader === null && this.#entryEdit === null && this.#bucketEdit === null && this.#retirements === null && this.#buckets === null && this.#entries === null && this.#source === null; }
}
//#endregion 🧵️ScenePreparation
