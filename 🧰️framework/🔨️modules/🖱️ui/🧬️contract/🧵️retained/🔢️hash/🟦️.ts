//#region 🧬️SnapshotHashContract
import type { NumericIndexGrant } from "../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️.ts";
import { OwnedUiNodeIndex, type OwnedUiNodeIndexReader, type OwnedUiNodeIndexRetirement } from "../🗂️nodes/🟦️.ts";
import type { OwnedUiNode, UiNodeRetirement, UiSurfaceByteView } from "../📦️wire/🧾️typed/🟦️.ts";
import type { RetainedUiWireStep } from "../📦️wire/🟦️.ts";

export type OwnedUiSnapshotHashStep = RetainedUiWireStep & { readonly chunk?: Uint8Array };
export type OwnedUiSnapshotHash = { readonly hash: string; readonly byteLength: number };
type Metadata = { readonly surface: string; readonly revision: number; readonly root: number | null };
type Program = Generator<OwnedUiSnapshotHashStep, void, void>;
const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const state = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): OwnedUiSnapshotHashStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
//#endregion 🧬️SnapshotHashContract

//#region 🔡️CanonicalJsonBytes
type Frame = { next: Frame | null } & (
  | { kind: "value"; value: unknown }
  | { kind: "raw" | "text"; value: string; offset: number; opened: boolean }
  | { kind: "array"; value: readonly unknown[]; offset: number; opened: boolean }
  | { kind: "object"; value: Readonly<Record<string, unknown>>; keys: string[]; offset: number; opened: boolean }
  | { kind: "bytes"; value: UiSurfaceByteView; offset: number; opened: boolean }
);
type Atom = { cost: number; bytes: readonly number[] };

function utf8(code: number): number[] {
  if (code < 128) return [code];
  if (code < 2048) return [192 | code >>> 6, 128 | code & 63];
  if (code < 65536) return [224 | code >>> 12, 128 | code >>> 6 & 63, 128 | code & 63];
  return [240 | code >>> 18, 128 | code >>> 12 & 63, 128 | code >>> 6 & 63, 128 | code & 63];
}

function escaped(code: number): number[] {
  const digits = "0123456789abcdef";
  return [92, 117, digits.charCodeAt(code >>> 12), digits.charCodeAt(code >>> 8 & 15), digits.charCodeAt(code >>> 4 & 15), digits.charCodeAt(code & 15)];
}

class JsonBytes {
  #stack: Frame | null;
  private readonly surface: UiSurfaceByteView | null;
  constructor(value: unknown, surface: UiSurfaceByteView | null = null, raw = false) { this.surface = surface; this.#stack = raw && typeof value === "string" ? { kind: "raw", value, offset: 0, opened: true, next: null } : { kind: "value", value, next: null }; }
  #pop(): void { const frame = this.#stack!; this.#stack = frame.next; frame.next = null; }
  #raw(value: string): void { this.#stack = { kind: "raw", value, offset: 0, opened: true, next: this.#stack }; }
  #text(value: string): void { this.#stack = { kind: "text", value, offset: 0, opened: false, next: this.#stack }; }

  #atom(): Atom {
    const frame = this.#stack!;
    if (frame.kind === "value") {
      this.#pop(); const value = frame.value;
      if (value === null) this.#raw("null");
      else if (typeof value === "string") this.#text(value);
      else if (typeof value === "boolean") this.#raw(value ? "true" : "false");
      else if (typeof value === "number" && Number.isFinite(value)) this.#raw(String(value));
      else if (value === this.surface) this.#stack = { kind: "bytes", value: this.surface!, offset: 0, opened: false, next: this.#stack };
      else if (Array.isArray(value)) this.#stack = { kind: "array", value, offset: 0, opened: false, next: this.#stack };
      else if (typeof value === "object" && value !== null) {
        const keys = Object.keys(value); if (keys.length > 256) throw new Error("Owned JSON object exceeds its normalized native envelope");
        this.#stack = { kind: "object", value: value as Readonly<Record<string, unknown>>, keys, offset: 0, opened: false, next: this.#stack };
        return { cost: 64 + keys.length * 8, bytes: [] };
      } else throw new Error("Owned UI JSON contains an unsupported value");
      return { cost: 96, bytes: [] };
    }
    if (frame.kind === "raw" || frame.kind === "text") {
      if (!frame.opened) { frame.opened = true; return { cost: 32, bytes: [34] }; }
      if (frame.offset === frame.value.length) { this.#pop(); return { cost: 64, bytes: frame.kind === "text" ? [34] : [] }; }
      const code = frame.value.charCodeAt(frame.offset++);
      if (frame.kind === "raw") return { cost: 32, bytes: [code] };
      if (code === 34 || code === 92) return { cost: 48, bytes: [92, code] };
      if (code === 8 || code === 9 || code === 10 || code === 12 || code === 13) return { cost: 48, bytes: [92, code === 8 ? 98 : code === 9 ? 116 : code === 10 ? 110 : code === 12 ? 102 : 114] };
      if (code < 32) return { cost: 64, bytes: escaped(code) };
      if (code >= 0xd800 && code <= 0xdbff) {
        const low = frame.value.charCodeAt(frame.offset);
        if (low >= 0xdc00 && low <= 0xdfff) { frame.offset++; return { cost: 64, bytes: utf8(0x10000 + (code - 0xd800) * 1024 + low - 0xdc00) }; }
        return { cost: 64, bytes: escaped(code) };
      }
      return { cost: 64, bytes: code >= 0xdc00 && code <= 0xdfff ? escaped(code) : utf8(code) };
    }
    if (!frame.opened) { frame.opened = true; return { cost: 32, bytes: [frame.kind === "object" ? 123 : 91] }; }
    const length = frame.kind === "object" ? frame.keys.length : frame.value.length;
    if (frame.offset === length) { this.#pop(); return { cost: 64 + (frame.kind === "object" ? frame.keys.length * 8 : 0), bytes: [frame.kind === "object" ? 125 : 93] }; }
    const index = frame.offset++;
    if (frame.kind === "object") {
      const key = frame.keys[index]!;
      this.#stack = { kind: "value", value: frame.value[key], next: this.#stack }; this.#raw(":"); this.#text(key);
      if (index) this.#raw(",");
      return { cost: 256, bytes: [] };
    }
    this.#stack = { kind: "value", value: frame.kind === "bytes" ? frame.value.byteAt(index) : frame.value[index], next: this.#stack };
    if (index) this.#raw(",");
    return { cost: 128, bytes: [] };
  }

  advance(grant: NumericIndexGrant): { done: boolean; cost: number; chunk: Uint8Array; failure: string | null } {
    const output = new Uint8Array(256); let written = 0; let cost = 256;
    let failure: string | null = null;
    while (this.#stack && cost + 2200 <= grant.maxBytes && written + 16 <= output.length) {
      try { const atom = this.#atom(); cost += atom.cost + atom.bytes.length * 3; for (const byte of atom.bytes) output[written++] = byte; }
      catch (error) { failure = error instanceof Error ? error.message : "Owned JSON encoding failed"; cost += 64; break; }
    }
    return { done: this.#stack === null, cost, chunk: output.subarray(0, written), failure };
  }
  closeStep(): { done: boolean; cost: number } { const frame = this.#stack; if (!frame) return { done: true, cost: 0 }; const cost = 64 + (frame.kind === "object" ? frame.keys.length * 8 : 0); this.#pop(); return { done: false, cost }; }
}
//#endregion 🔡️CanonicalJsonBytes

//#region 🔢️CapturedSnapshotDigest
/** 🔢️ Streams the exact captured snapshot into FNV-1a without materializing its node array or JSON. */
export class OwnedUiSnapshotHashCursor {
  #source: OwnedUiNodeIndex | null;
  #reader: OwnedUiNodeIndexReader | null = null;
  #retirement: OwnedUiNodeIndexRetirement | null = null;
  #node: OwnedUiNode | null = null;
  #nodeRetirement: UiNodeRetirement | null = null;
  #encoder: JsonBytes | null = null;
  #program: Program | null = null;
  #grant: NumericIndexGrant = { maxItems: 0, maxBytes: 0 };
  #hash = 0x811c9dc5;
  #bytes = 0;
  #metadata: Metadata | null;
  #status: "pending" | "ready" | "rejected" | "closing" | "closed" = "pending";
  #failure: string | null = null;
  #taken = false;

  constructor(source: OwnedUiNodeIndex, metadata: Metadata) {
    const exact = { surface: metadata.surface, revision: metadata.revision, root: metadata.root };
    if (typeof exact.surface !== "string" || !Number.isSafeInteger(exact.revision) || exact.revision < 0 || (exact.root !== null && (!Number.isSafeInteger(exact.root) || exact.root < 0))) throw new Error("Invalid owned UI snapshot identity");
    this.#metadata = exact; this.#source = source.capture(); Object.freeze(this);
  }
  get failure(): string | null { return this.#failure; }

  *#encode(value: unknown, raw = false, surface: UiSurfaceByteView | null = null): Program {
    this.#encoder = new JsonBytes(value, surface, raw); yield state("pending", "hash-encoder", 64);
    for (;;) {
      const result = this.#encoder.advance(this.#grant);
      if (!Number.isSafeInteger(this.#bytes + result.chunk.length)) throw new RangeError("Owned snapshot byte count exhausted");
      for (const byte of result.chunk) this.#hash = Math.imul(this.#hash ^ byte, 0x01000193) >>> 0;
      this.#bytes += result.chunk.length;
      yield { ...state("pending", "hash-bytes", result.cost), chunk: result.chunk };
      if (result.failure) throw new Error(result.failure);
      if (result.done) break;
    }
    this.#encoder = null; yield state("pending", "hash-encoder-close", 64);
  }

  *#run(): Program {
    const metadata = this.#metadata!;
    yield* this.#encode('{"surface":', true); yield* this.#encode(metadata.surface);
    yield* this.#encode(',"revision":', true); yield* this.#encode(metadata.revision);
    yield* this.#encode(',"root":', true); yield* this.#encode(metadata.root ?? 0);
    yield* this.#encode(',"nodes":[', true);
    this.#reader = this.#source!.beginRead(); yield state("pending", "hash-reader", 64);
    let first = true;
    for (;;) {
      const result = this.#reader.advance(this.#grant);
      if (result.kind === "value") {
        this.#node = result.value; yield state("pending", "hash-node", result.bytes);
        if (!first) yield* this.#encode(",", true); first = false;
        const record = this.#node.value;
        yield* this.#encode(record, false, record.component.type === "surface" ? record.component.doc.bytes : null);
        this.#nodeRetirement = this.#node.beginClose(); this.#node = null; yield state("pending", "hash-node-close", 64);
        while (this.#nodeRetirement) { const closed = this.#nodeRetirement.advance(this.#grant); if (closed.kind === "complete") this.#nodeRetirement = null; yield { ...closed, kind: "pending" }; }
      } else yield state("pending", "hash-reader", result.bytes);
      if (result.kind === "complete") break;
    }
    this.#retirement = this.#reader.beginClose(); this.#reader = null; yield state("pending", "hash-reader-close", 64);
    while (this.#retirement) { const closed = this.#retirement.advance(this.#grant); if (closed.kind === "complete") this.#retirement = null; yield { ...closed, kind: "pending" }; }
    yield* this.#encode('],"layoutEpoch":"0"}', true);
  }

  advance(grant: NumericIndexGrant): OwnedUiSnapshotHashStep {
    if (this.#status !== "pending") { if (this.#status === "closing") throw new Error("Owned UI hash is closing"); return state(this.#status === "closed" ? "complete" : this.#status, "hash"); }
    if (!admitted(grant)) return state("blocked", "hash");
    this.#grant = grant; this.#program ??= this.#run();
    try { const result = this.#program.next(); if (result.done) { this.#program = null; this.#status = "ready"; return state("ready", "hash", 32); } if (result.value.bytes > grant.maxBytes || result.value.items > grant.maxItems) throw new Error("Owned UI hash exceeded its grant"); return result.value; }
    catch (error) { this.#failure = error instanceof Error ? error.message : "Owned UI hash failed"; this.#program = null; this.#status = "rejected"; return state("rejected", "hash", 64); }
  }
  takeResult(): OwnedUiSnapshotHash | null { if (this.#status !== "ready" || this.#taken) return null; this.#taken = true; return { hash: `${this.#hash.toString(16)}:${this.#metadata!.revision}`, byteLength: this.#bytes }; }
  beginClose(): void { if (this.#status === "closing" || this.#status === "closed") return; this.#program = null; this.#status = "closing"; }
  closeStep(grant: NumericIndexGrant): OwnedUiSnapshotHashStep {
    if (this.#status === "closed") return state("complete", "hash-close");
    if (this.#status !== "closing") throw new Error("Owned UI hash close has not begun");
    if (!admitted(grant)) return state("blocked", "hash-close");
    if (this.#encoder) { const result = this.#encoder.closeStep(); if (result.done) this.#encoder = null; return state("pending", "hash-frame-close", result.cost); }
    if (this.#nodeRetirement) { const result = this.#nodeRetirement.advance(grant); if (result.kind === "complete") this.#nodeRetirement = null; return { ...result, kind: "pending" }; }
    if (this.#node) { this.#nodeRetirement = this.#node.beginClose(); this.#node = null; return state("pending", "hash-node-close", 64); }
    if (this.#retirement) { const result = this.#retirement.advance(grant); if (result.kind === "complete") this.#retirement = null; return { ...result, kind: "pending" }; }
    if (this.#reader) { this.#retirement = this.#reader.beginClose(); this.#reader = null; return state("pending", "hash-reader-close", 64); }
    if (this.#source) { this.#retirement = this.#source.beginClose(); this.#source = null; return state("pending", "hash-source-close", 64); }
    this.#metadata = null; this.#status = "closed"; return state("complete", "hash-close");
  }
  terminalIsEmpty(): boolean { return this.#status === "closed" && !this.#encoder && !this.#node && !this.#nodeRetirement && !this.#reader && !this.#retirement && !this.#source && !this.#metadata && !this.#program; }
}
//#endregion 🔢️CapturedSnapshotDigest
