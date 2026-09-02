//#region 📦️OwnedWireContract
import { NumericIndex, type NumericIndexEdit, type NumericIndexReader, type NumericIndexRetirement, type NumericIndexGrant } from "../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️.ts";
import { UiSurfaceBytes, UiSurfaceByteBuilder, type UiSurfaceByteRetirement } from "./🔢️bytes/🟦️.ts";
import { takeOwnedNativeBuffer } from "./🔒️transport/🟦️.ts";

export type RetainedUiWireValue = null | boolean | number | string | UiSurfaceBytes | readonly RetainedUiWireValue[] | { readonly [key: string]: RetainedUiWireValue };
export type RetainedUiWireStep = { readonly kind: "blocked" | "pending" | "ready" | "rejected" | "complete"; readonly phase: string; readonly items: number; readonly bytes: number };
type Container = RetainedUiWireValue[] | { [key: string]: RetainedUiWireValue };
type Owned = { value: Container | UiSurfaceBytes | null; next: Owned | null };
type Frame = { owner: Owned; count: number; index: number; key: string | null; previousKey: Uint8Array | null; array: boolean; bytes: UiSurfaceByteBuilder | null; parent: Frame | null };
const TEXT_BYTES = 512;
const COLLECTION_ITEMS = 256;
const MINIMUM_GRANT = 4096;
const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= MINIMUM_GRANT;
const result = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });

function compareBytes(left: Uint8Array, right: Uint8Array): number {
  const count = Math.min(left.length, right.length);
  for (let i = 0; i < count; i++) if (left[i] !== right[i]) return left[i]! - right[i]!;
  return left.length - right.length;
}
//#endregion 📦️OwnedWireContract

//#region 📖️OwnedDecoder
/** 📦️ Consumes an entire transport-owned buffer; every alias is relinquished by this explicit ownership transfer. */
export class RetainedUiWireValueCursor {
  #input: Uint8Array | null;
  #offset = 0;
  #phase = "symbol-count";
  #failure: string | null = null;
  #closing = false;
  #ready = false;
  #root: RetainedUiWireValue | undefined;
  #pending: RetainedUiWireValue | undefined;
  #frame: Frame | null = null;
  #head: Owned | null = null;
  #tail: Owned | null = null;
  #symbols: NumericIndex<string> | null = NumericIndex.empty<string>();
  #symbolEdit: NumericIndexEdit<string> | null = null;
  #symbolReader: NumericIndexReader<string> | null = null;
  #retirement: NumericIndexRetirement<string> | null = null;
  #oldSymbols: NumericIndex<string> | null = null;
  #symbolCount = 0;
  #symbolIndex = 0;
  #previousSymbol: Uint8Array | null = null;
  #text = "";
  #textKind: "symbol" | "value" | "key" = "symbol";
  #textLength = 0;
  #number = 0;
  #natural = 0;
  #multiplier = 1;
  #naturalBytes = 0;
  #array = false;
  #closeOffset = 0;
  #byteRetirement: UiSurfaceByteRetirement | null = null;
  readonly #profile: "value" | "component" | "node";

  constructor(input: unknown, profile: "value" | "component" | "node" = "value") {
    this.#profile = profile;
    this.#input = new Uint8Array(takeOwnedNativeBuffer(input, "Uint8Array", Number.MAX_SAFE_INTEGER));
  }

  get value(): RetainedUiWireValue | undefined { return this.#ready && !this.#closing ? this.#root : undefined; }
  get failure(): string | null { return this.#failure; }

  #byte(): number {
    if (!this.#input || this.#offset >= this.#input.length) throw new Error("Truncated UI wire value");
    return this.#input[this.#offset++]!;
  }

  #nat(next: string): number {
    const byte = this.#byte();
    const digit = byte & 127;
    const value = this.#natural + digit * this.#multiplier;
    if (!Number.isSafeInteger(value) || this.#naturalBytes >= 8) throw new Error("UI wire integer exceeds the exact range");
    this.#natural = value; this.#naturalBytes++;
    if (byte < 128) {
      if (this.#naturalBytes > 1 && digit === 0) throw new Error("Noncanonical UI wire integer");
      this.#number = value; this.#natural = 0; this.#multiplier = 1; this.#naturalBytes = 0; this.#phase = next;
    } else this.#multiplier *= 128;
    return 1;
  }

  #own(value: Container | UiSurfaceBytes | null): Owned {
    const owner: Owned = { value, next: null };
    if (this.#tail) this.#tail.next = owner; else this.#head = owner;
    this.#tail = owner;
    return owner;
  }

  #surfaceBytesPath(): boolean {
    const document = this.#frame;
    const component = document?.parent;
    if (!this.#array || document?.key !== "bytes" || component?.key !== "doc") return false;
    if (this.#profile === "component") return component.parent === null;
    return this.#profile === "node" && component.parent?.key === "component" && component.parent.parent === null;
  }

  #finishBytes(): void {
    const frame = this.#frame!;
    const value = frame.bytes!.takeResult();
    if (!value) throw new Error("Surface byte preparation is incomplete");
    frame.bytes = null; frame.owner.value = value;
    this.#pending = value; this.#frame = frame.parent; frame.parent = null; this.#phase = "attach";
  }

  #advance(grant: NumericIndexGrant): number {
    switch (this.#phase) {
      case "symbol-count": return this.#nat("symbol-count-done");
      case "symbol-count-done":
        if (this.#number > this.#input!.length - this.#offset) throw new Error("Impossible UI symbol count");
        this.#symbolCount = this.#number; this.#textKind = "symbol";
        this.#phase = this.#symbolCount ? "text-length" : "field-count"; return 32;
      case "text-length": return this.#nat("text-length-done");
      case "text-length-done":
        if (this.#number > TEXT_BYTES || this.#number > this.#input!.length - this.#offset) throw new Error("UI text exceeds native bounds or input");
        this.#textLength = this.#number; this.#phase = "text-body"; return 16;
      case "text-body": {
        const bytes = this.#input!.subarray(this.#offset, this.#offset + this.#textLength);
        this.#stepBytes = 4 * bytes.length + 64;
        const text = new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(bytes);
        this.#offset += bytes.length;
        if (this.#textKind === "symbol") {
          if (this.#previousSymbol && compareBytes(this.#previousSymbol, bytes) >= 0) throw new Error("UI symbols must be strictly ordered");
          this.#previousSymbol = bytes; this.#text = text; this.#phase = "symbol-store";
        } else if (this.#textKind === "key") {
          const frame = this.#frame!;
          if (frame.previousKey && compareBytes(frame.previousKey, bytes) >= 0) throw new Error("UI map keys must be strictly ordered");
          frame.previousKey = bytes; frame.key = text; this.#phase = "value-tag";
        } else { this.#pending = text; this.#phase = "attach"; }
        return 4 * bytes.length + 64;
      }
      case "symbol-store":
        this.#symbolEdit = this.#symbols!.beginSet(this.#symbolIndex, this.#text); this.#text = ""; this.#phase = "symbol-edit"; return 128;
      case "symbol-edit": {
        const step = this.#symbolEdit!.advance(grant);
        if (step.kind === "rejected") throw new Error("UI symbol ordinal exhausted");
        if (step.kind === "ready") {
          this.#oldSymbols = this.#symbols; this.#symbols = this.#symbolEdit!.takeResult();
          this.#retirement = this.#symbolEdit!.beginClose(); this.#symbolEdit = null; this.#phase = "symbol-edit-close";
        }
        return step.bytes;
      }
      case "symbol-edit-close": {
        const step = this.#retirement!.advance(grant);
        if (step.kind === "complete") { this.#retirement = this.#oldSymbols!.beginClose(); this.#oldSymbols = null; this.#phase = "symbol-old-close"; }
        return step.bytes;
      }
      case "symbol-old-close": {
        const step = this.#retirement!.advance(grant);
        if (step.kind === "complete") { this.#retirement = null; this.#symbolIndex++; this.#phase = this.#symbolIndex < this.#symbolCount ? "text-length" : "field-count"; }
        return step.bytes;
      }
      case "field-count": return this.#nat("field-count-done");
      case "field-count-done":
        if (this.#number !== 1) throw new Error("UI wire requires one bridge field");
        this.#phase = "field-id"; return 8;
      case "field-id": return this.#nat("field-id-done");
      case "field-id-done":
        if (this.#number !== 1) throw new Error("UI wire bridge field identity differs");
        this.#phase = "outer-tag"; return 8;
      case "outer-tag":
        if (this.#byte() !== 0x11) throw new Error("UI wire bridge tag differs");
        this.#phase = "value-tag"; return 1;
      case "value-tag": {
        const tag = this.#byte();
        if (tag === 0x12 || tag === 0x01 || tag === 0x02) { this.#pending = tag === 0x12 ? null : tag === 0x02; this.#phase = "attach"; }
        else if (tag === 0x05) this.#phase = "float";
        else if (tag === 0x06) this.#phase = "symbol-reference";
        else if (tag === 0x07) { this.#textKind = "value"; this.#phase = "text-length"; }
        else if (tag === 0x0c || tag === 0x10) { this.#array = tag === 0x0c; this.#phase = "collection-count"; }
        else throw new Error("Unknown UI value tag");
        return 1;
      }
      case "float": {
        this.#stepBytes = 16;
        if (this.#input!.length - this.#offset < 8) throw new Error("Truncated UI number");
        const value = new DataView(this.#input!.buffer, this.#offset, 8).getFloat64(0, true);
        if (!Number.isFinite(value) || Object.is(value, -0)) throw new Error("Noncanonical UI number");
        this.#offset += 8; this.#pending = value; this.#phase = "attach"; return 16;
      }
      case "symbol-reference": return this.#nat("symbol-reference-done");
      case "symbol-reference-done":
        if (this.#number >= this.#symbolCount) throw new Error("UI symbol reference exceeds table");
        this.#symbolReader = this.#symbols!.beginLookup(this.#number); this.#phase = "symbol-lookup"; return 64;
      case "symbol-lookup": {
        const step = this.#symbolReader!.advance(grant);
        if (step.kind === "value") this.#pending = step.value;
        if (step.kind === "complete") { this.#retirement = this.#symbolReader!.beginClose(); this.#symbolReader = null; this.#phase = "symbol-lookup-close"; }
        return step.bytes;
      }
      case "symbol-lookup-close": {
        const step = this.#retirement!.advance(grant);
        if (step.kind === "complete") { this.#retirement = null; this.#phase = "attach"; }
        return step.bytes;
      }
      case "collection-count": return this.#nat("collection-create");
      case "collection-create": {
        if (this.#surfaceBytesPath()) {
          const bytes = new UiSurfaceByteBuilder(this.#number);
          this.#frame = { owner: this.#own(null), count: this.#number, index: 0, key: null, previousKey: null, array: true, bytes, parent: this.#frame };
          this.#phase = "surface-bytes-reserve";
          return 128;
        }
        if (this.#number > COLLECTION_ITEMS || this.#number > this.#input!.length - this.#offset) throw new Error("UI collection exceeds native bounds or input");
        const owner = this.#own(this.#array ? new Array<RetainedUiWireValue>(this.#number) : {});
        this.#frame = { owner, count: this.#number, index: 0, key: null, previousKey: null, array: this.#array, bytes: null, parent: this.#frame };
        this.#phase = this.#number ? this.#array ? "value-tag" : "map-key-tag" : "collection-finish";
        return 128 + this.#number * 8;
      }
      case "surface-bytes-reserve": {
        const step = this.#frame!.bytes!.advance(grant);
        if (step.kind === "ready") this.#finishBytes(); else this.#phase = "value-tag";
        return step.bytes + 64;
      }
      case "map-key-tag":
        if (this.#byte() !== 0x07) throw new Error("UI map keys must use inline canonical text");
        this.#textKind = "key"; this.#phase = "text-length"; return 1;
      case "attach": {
        if (this.#pending === undefined) throw new Error("Missing decoded UI value");
        const frame = this.#frame;
        if (!frame) { this.#root = this.#pending; this.#pending = undefined; this.#phase = "finish"; return 32; }
        if (frame.bytes) {
          if (typeof this.#pending !== "number") throw new Error("Surface bytes require unsigned byte values");
          const step = frame.bytes.advance(grant, this.#pending);
          if (step.kind === "rejected") throw new Error("Surface byte value is outside its native range");
          if (step.accepted) { this.#pending = undefined; frame.index++; this.#phase = "value-tag"; }
          if (step.kind === "ready") this.#finishBytes();
          return step.bytes + 64;
        }
        const key = frame.array ? String(frame.index) : frame.key!;
        this.#stepBytes = 128 + frame.count * 8 + key.length * 2;
        Object.defineProperty(frame.owner.value!, key, { value: this.#pending, enumerable: true, configurable: false, writable: false });
        this.#pending = undefined; frame.key = null; frame.index++;
        this.#phase = frame.index === frame.count ? "collection-finish" : frame.array ? "value-tag" : "map-key-tag";
        return 128 + frame.count * 8 + key.length * 2;
      }
      case "collection-finish": {
        const frame = this.#frame!;
        if (frame.array) Object.defineProperty(frame.owner.value, "length", { writable: false });
        Object.preventExtensions(frame.owner.value!);
        this.#pending = frame.owner.value!; this.#frame = frame.parent; frame.parent = null; frame.previousKey = null; this.#phase = "attach";
        return 64 + frame.count * 8;
      }
      case "finish":
        if (this.#offset !== this.#input!.length) throw new Error("Trailing UI wire bytes");
        this.#ready = true; return 16;
      default: throw new Error("Invalid UI decoder phase");
    }
  }

  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return result("blocked", this.#phase);
    if (this.#closing) return result("rejected", "closing");
    if (this.#failure) return result("rejected", this.#phase);
    if (this.#ready) return result("ready", this.#phase);
    const phase = this.#phase;
    this.#stepBytes = 16;
    try { const bytes = this.#advance(grant); return result(this.#ready ? "ready" : "pending", phase, bytes); }
    catch (error) { this.#failure = error instanceof Error ? error.message : "UI wire decoding failed"; return result("rejected", phase, this.#stepBytes); }
  }

  #stepBytes = 0;
//#endregion 📖️OwnedDecoder

//#region ♻️OwnedRetirement
  beginClose(): void { this.#closing = true; this.#ready = false; }

  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return result("blocked", "close");
    if (!this.#closing) throw new Error("UI wire retirement was not started");
    if (this.#byteRetirement) { const step = this.#byteRetirement.advance(grant); if (step.kind === "complete") this.#byteRetirement = null; return result("pending", "close-surface-bytes", step.bytes); }
    if (this.#frame?.bytes) { this.#byteRetirement = this.#frame.bytes.beginClose(); this.#frame.bytes = null; return result("pending", "close-byte-builder", 64); }
    if (this.#frame) { const frame = this.#frame; this.#frame = frame.parent; frame.parent = null; frame.previousKey = null; frame.key = null; return result("pending", "close-frame", 128); }
    if (this.#root !== undefined || this.#pending !== undefined) { this.#root = undefined; this.#pending = undefined; return result("pending", "close-root", 64); }
    if (this.#head) { const owner = this.#head; this.#head = owner.next; owner.next = null; if (owner.value instanceof UiSurfaceBytes) this.#byteRetirement = owner.value.beginClose(); owner.value = null; if (!this.#head) this.#tail = null; return result("pending", "close-owner", 64 + COLLECTION_ITEMS * 8); }
    if (this.#retirement) { const step = this.#retirement.advance(grant); if (step.kind === "complete") this.#retirement = null; return result("pending", "close-index", step.bytes); }
    if (this.#symbolReader) { this.#retirement = this.#symbolReader.beginClose(); this.#symbolReader = null; return result("pending", "close-reader", 64); }
    if (this.#symbolEdit) { this.#retirement = this.#symbolEdit.beginClose(); this.#symbolEdit = null; return result("pending", "close-edit", 64); }
    if (this.#oldSymbols) { this.#retirement = this.#oldSymbols.beginClose(); this.#oldSymbols = null; return result("pending", "close-old", 64); }
    if (this.#symbols) { this.#retirement = this.#symbols.beginClose(); this.#symbols = null; return result("pending", "close-symbols", 64); }
    if (this.#input) {
      this.#text = ""; this.#previousSymbol = null;
      const end = Math.min(this.#input.length, this.#closeOffset + MINIMUM_GRANT);
      const bytes = end - this.#closeOffset;
      this.#input.fill(0, this.#closeOffset, end); this.#closeOffset = end;
      if (end === this.#input.length) this.#input = null;
      return result("pending", "close-bytes", bytes);
    }
    return result("complete", "closed");
  }

  terminalIsEmpty(): boolean {
    return this.#closing && !this.#input && !this.#frame && !this.#head && !this.#tail && !this.#symbols && !this.#symbolReader && !this.#symbolEdit && !this.#retirement && !this.#byteRetirement && !this.#oldSymbols && this.#root === undefined && this.#pending === undefined && !this.#text && !this.#previousSymbol;
  }
}
//#endregion ♻️OwnedRetirement
