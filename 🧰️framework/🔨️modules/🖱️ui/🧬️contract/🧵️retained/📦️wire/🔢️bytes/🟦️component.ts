//#region 📦️SurfaceByteContract
import type { NumericIndexGrant } from "../../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️component.ts";

const PAGE_BYTES = 256;
const MAXIMUM_BYTES = 32768;
const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
type Root = { readonly length: number; pages: (Uint8Array | null)[]; references: number };
export type UiSurfaceByteStep = { readonly kind: "blocked" | "pending" | "ready" | "rejected" | "complete"; readonly accepted: boolean; readonly items: number; readonly bytes: number };
const step = (kind: UiSurfaceByteStep["kind"], bytes = 0, accepted = false): UiSurfaceByteStep => ({ kind, accepted, items: bytes ? 1 : 0, bytes });
let own: (root: Root) => UiSurfaceBytes;
let retire: (root: Root | null) => UiSurfaceByteRetirement;
//#endregion 📦️SurfaceByteContract

//#region 🔒️ImmutableByteOwner
/** 📦️ Exact immutable surface bytes; capture an owner before retaining a read across publication. */
export class UiSurfaceBytes {
  #root: Root | null;
  private constructor(root: Root) { this.#root = root; }
  static { own = (root) => new UiSurfaceBytes(root); }
  get length(): number { if (!this.#root) throw new Error("Surface byte owner is closed"); return this.#root.length; }

  byteAt(index: number): number {
    if (!this.#root) throw new Error("Surface byte owner is closed");
    if (!Number.isSafeInteger(index) || index < 0 || index >= this.#root.length) throw new RangeError("Surface byte index is outside the exact range");
    return this.#root.pages[Math.floor(index / PAGE_BYTES)]![index % PAGE_BYTES]!;
  }

  capture(): UiSurfaceBytes {
    if (!this.#root) throw new Error("Surface byte owner is closed");
    if (this.#root.references === Number.MAX_SAFE_INTEGER) throw new RangeError("Surface byte owner reference overflow");
    this.#root.references++;
    return own(this.#root);
  }

  beginClose(): UiSurfaceByteRetirement {
    const root = this.#root; this.#root = null;
    return retire(root);
  }

  terminalIsEmpty(): boolean { return this.#root === null; }
}
//#endregion 🔒️ImmutableByteOwner

//#region 🧱️BytePagePreparation
export class UiSurfaceByteBuilder {
  #root: Root | null = null;
  #written = 0;
  #taken = false;
  #closing = false;
  #failed = false;
  readonly #length: number;

  constructor(length: number) {
    if (!Number.isSafeInteger(length) || length < 0 || length > MAXIMUM_BYTES) throw new RangeError("Surface byte length exceeds the native envelope");
    this.#length = length;
  }

  advance(grant: NumericIndexGrant, value?: number): UiSurfaceByteStep {
    if (!admitted(grant)) return step("blocked");
    if (this.#closing || this.#taken || this.#failed) return step("rejected");
    if (value !== undefined && (!Number.isInteger(value) || value < 0 || value > 255)) { this.#failed = true; return step("rejected", 16); }
    if (!this.#root) {
      const count = Math.ceil(this.#length / PAGE_BYTES);
      this.#root = { length: this.#length, pages: new Array<Uint8Array | null>(count).fill(null), references: 1 };
      return step(this.#length ? "pending" : "ready", 64 + count * 8);
    }
    if (this.#written === this.#length) return step("ready");
    if (value === undefined) return step("blocked");
    const index = Math.floor(this.#written / PAGE_BYTES);
    if (!this.#root.pages[index]) {
      const length = Math.min(PAGE_BYTES, this.#length - index * PAGE_BYTES);
      this.#root.pages[index] = new Uint8Array(length);
      return step("pending", length + 16);
    }
    this.#root.pages[index]![this.#written % PAGE_BYTES] = value; this.#written++;
    return step(this.#written === this.#length ? "ready" : "pending", 1, true);
  }

  takeResult(): UiSurfaceBytes | null {
    if (!this.#root || this.#written !== this.#length || this.#closing || this.#failed || this.#taken) return null;
    const root = this.#root; this.#root = null; this.#taken = true;
    return own(root);
  }

  beginClose(): UiSurfaceByteRetirement {
    this.#closing = true;
    const root = this.#root; this.#root = null;
    return retire(root);
  }

  terminalIsEmpty(): boolean { return (this.#taken || this.#closing) && this.#root === null; }
}
//#endregion 🧱️BytePagePreparation

//#region ♻️BytePageRetirement
export class UiSurfaceByteRetirement {
  #root: Root | null;
  #released = false;
  #page = 0;
  #offset = 0;
  private constructor(root: Root | null) { this.#root = root; }
  static { retire = (root) => new UiSurfaceByteRetirement(root); }

  advance(grant: NumericIndexGrant): UiSurfaceByteStep {
    if (!admitted(grant)) return step("blocked");
    if (!this.#root) return step("complete");
    if (!this.#released) {
      this.#root.references--;
      this.#released = true;
      if (this.#root.references > 0) this.#root = null;
      return step(this.#root ? "pending" : "complete", 32);
    }
    if (this.#page < this.#root.pages.length) {
      const page = this.#root.pages[this.#page];
      if (!page) { this.#page++; return step("pending", 8); }
      const end = Math.min(page.length, this.#offset + grant.maxBytes - 16);
      const bytes = end - this.#offset;
      page.fill(0, this.#offset, end); this.#offset = end;
      if (end === page.length) { this.#root.pages[this.#page++] = null; this.#offset = 0; }
      return step("pending", bytes + 16);
    }
    const bytes = this.#root.pages.length * 8 + 32;
    this.#root.pages = []; this.#root = null;
    return step("complete", bytes);
  }

  terminalIsEmpty(): boolean { return this.#root === null; }
}
//#endregion ♻️BytePageRetirement
