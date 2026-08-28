//#region 📄️PrivatePagedInputAuthority
import { OwnedUiResidentPayload, retainOwnedUiResidentBuilder, type OwnedUiResidentPage } from "../../../💾️resident/🟦️component.ts";
import type { OwnedUiInstance } from "../../../🏘️instance/🟦️component.ts";
import type { NumericIndexGrant } from "../../../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️component.ts";
import type { RetainedUiWireStep } from "../../../📦️wire/🟦️component.ts";
import type { ActorInstanceLifetime } from "../../../../../../🎭️actor/🚪️lifetime/🟦️component.ts";
import type { ShardActorActivationLease } from "../../../../../../🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts";
import { OwnedKernelReturnInputField, OwnedKernelReturnInputFragment, OwnedKernelReturnInputRelease } from "../../../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️component.ts";

const MINT = Object.freeze({});
type Evidence = { readonly fragment: object; readonly field: object; readonly builder: OwnedUiOperationPayloadBuilder; readonly offset: bigint; readonly length: number };
const admitted = (grant: NumericIndexGrant, bytes: number): boolean => Number.isSafeInteger(grant.maxItems) && grant.maxItems >= 1 && Number.isSafeInteger(grant.maxBytes) && grant.maxBytes >= bytes;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
let cancelledProof: (proof: Evidence) => OwnedUiOperationInputCancelled;
let copiedProof: (proof: Evidence) => OwnedUiOperationInputCopied;
type PageCell = { readonly page: OwnedUiResidentPage; readonly length: number; next: PageCell | null };
let createReader: (builder: OwnedUiOperationPayloadBuilder, first: PageCell | null) => OwnedUiOperationPayloadReader;
let installReader: (builder: OwnedUiOperationPayloadBuilder, reader: OwnedUiOperationPayloadReader) => boolean;
export type OwnedUiOperationPayloadAdmission = { readonly step: RetainedUiWireStep; readonly builder: OwnedUiOperationPayloadBuilder | null };
export type OwnedUiOperationPayloadReadAdmission = { readonly step: RetainedUiWireStep; readonly reader: OwnedUiOperationPayloadReader | null };
export type OwnedUiOperationPayloadReadStep = RetainedUiWireStep | { readonly kind: "byte"; readonly value: number; readonly items: number; readonly bytes: number };
function childStep(current: RetainedUiWireStep, grant: NumericIndexGrant): RetainedUiWireStep {
  if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes) return { ...current, kind: "rejected" };
  return current.kind === "ready" || current.kind === "complete" ? { ...current, kind: "pending" } : current;
}
function exact(proof: Evidence, fragment: object, field: object, builder: object, offset: bigint, length: number): boolean {
  return typeof offset === "bigint" && offset >= 0n && offset <= 18446744073709551615n && Number.isSafeInteger(length) && length >= 0 && length <= 4096 && offset + BigInt(length) <= 18446744073709551615n
    && proof.fragment === fragment && proof.field === field && proof.builder === builder && proof.offset === offset && proof.length === length && OwnedUiOperationPayloadBuilder.matchesField(builder, field);
}

/** 📄️ Exact native field admission is retained by its resident parent before exposure to a caller. */
export class OwnedUiOperationPayloadBuilder {
  #field: OwnedKernelReturnInputField | null;
  #resident: OwnedUiResidentPayload | null;
  #bound = false;
  #closing = false;
  #phase: "open" | "proof" | "release" | "receipt" | "pages" | "owner" | "closed" = "open";
  #fragment: OwnedKernelReturnInputFragment | null = null;
  #proof: OwnedUiOperationInputCancelled | null = null;
  #release: OwnedKernelReturnInputRelease | null = null;
  #failure: string | null = null;
  #copyPhase: "idle" | "allocate" | "copy" | "seal" | "proof" | "release" | "receipt" | "continue" | "observe" | "ready" = "idle";
  #sourceKind: "pending" | "complete" | null = null;
  #input: OwnedKernelReturnInputFragment | null = null;
  #copyFragment: OwnedKernelReturnInputFragment | null = null;
  #lastFragment: OwnedKernelReturnInputFragment | null = null;
  #inputOffset = 0;
  #copied = 0n;
  #copyProof: OwnedUiOperationInputCopied | null = null;
  #copyRelease: OwnedKernelReturnInputRelease | null = null;
  #head: PageCell | null = null;
  #tail: PageCell | null = null;
  #writer: PageCell | null = null;
  #written = 0;
  #reader: OwnedUiOperationPayloadReader | null = null;
  #readerReserved = false;
  static { installReader = (builder, reader) => { if (!builder.#readerReserved || builder.#reader) return false; builder.#reader = reader; return true; }; }
  private constructor(mint: object, field: OwnedKernelReturnInputField, resident: OwnedUiResidentPayload) {
    if (mint !== MINT) throw new Error("Invalid paged builder authority"); this.#field = field; this.#resident = resident;
    if (!retainOwnedUiResidentBuilder(resident, this)) { this.#field = null; this.#resident = null; this.#closing = true; this.#phase = "closed"; throw new Error("Paged builder registration refused"); }
    Object.freeze(this);
  }
  static matchesField(builder: unknown, field: object): builder is OwnedUiOperationPayloadBuilder {
    return builder !== null && typeof builder === "object" && #field in builder && builder.#field === field && builder.#resident !== null;
  }
  static matchesResident(builder: unknown, resident: OwnedUiResidentPayload): builder is OwnedUiOperationPayloadBuilder { return builder !== null && typeof builder === "object" && #resident in builder && builder.#resident === resident; }
  static readerIsEmpty(builder: OwnedUiOperationPayloadBuilder): boolean { return builder.#reader === null || builder.#reader.terminalIsEmpty(); }
  static begin(owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime, field: unknown, resident: unknown, grant: NumericIndexGrant): OwnedUiOperationPayloadAdmission {
    if (!admitted(grant, 1024)) return { step: step("blocked", "paged-admission"), builder: null };
    if (!OwnedKernelReturnInputField.matchesOwner(field, owner, activation, lifetime) || !OwnedUiResidentPayload.matchesOwner(resident, owner, activation, lifetime)) return { step: step("rejected", "paged-admission-owner"), builder: null };
    if (!resident.reserveBuilder()) return { step: step("blocked", "paged-admission-resident"), builder: null };
    let builder: OwnedUiOperationPayloadBuilder | null = null;
    try {
      builder = new OwnedUiOperationPayloadBuilder(MINT, field, resident);
      if (!field.bind(builder)) throw new Error("Native field binding refused");
      builder.#bound = OwnedKernelReturnInputField.matchesBuilder(field, builder); if (!builder.#bound) throw new Error("Native field binding identity differs"); return { step: step("ready", "paged-admission", 1024), builder };
    } catch { if (builder) { builder.#failure = "Paged builder admission failed"; builder.beginClose(); } return { step: step("rejected", "paged-admission-fault", 1024), builder: null }; }
  }
  get failure(): string | null { return this.#failure; }

  //#region 📥️FixedPageCopy
  #copyChild(current: RetainedUiWireStep, grant: NumericIndexGrant): RetainedUiWireStep {
    const result = childStep(current, grant); if (result.kind === "rejected") this.#failure = "Paged payload child failed"; return result;
  }
  #settleCopy(): RetainedUiWireStep {
    const fragment = this.#copyFragment!;
    if (this.#copyPhase === "proof") { this.#copyProof = copiedProof({ fragment, field: this.#field!, builder: this, offset: fragment.offset, length: fragment.length }); this.#copyPhase = "release"; return step("pending", "paged-input-copy-proof", 128); }
    if (this.#copyPhase === "release") { const receipt = fragment.release(this.#copyProof); if (!receipt) return step("rejected", "paged-input-copy-release-refused", 128); this.#copyRelease = receipt; this.#copyPhase = "receipt"; return step("pending", "paged-input-copy-release", 128); }
    if (!OwnedKernelReturnInputRelease.matches(this.#copyRelease, fragment, this.#copyProof)) return step("rejected", "paged-input-copy-release-authority");
    this.#lastFragment = fragment; this.#copyFragment = null; this.#copyProof = null; this.#copyRelease = null; this.#copyPhase = "continue"; return step("pending", "paged-input-copy-release-retire", 128);
  }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, this.#copyPhase === "observe" ? 128 : 256)) return step("blocked", "paged-copy"); if (this.#closing || !this.#bound || this.#failure) return step("rejected", "paged-copy");
    try {
      if (this.#copyPhase === "ready") return step("ready", "paged-copy");
      if (this.#copyPhase === "proof" || this.#copyPhase === "release" || this.#copyPhase === "receipt") return this.#settleCopy();
      if (this.#copyPhase === "continue") {
        const current = this.#field!.advance(grant, this); const forwarded = this.#copyChild({ ...current, phase: "paged-source-advance" }, grant); if (forwarded.kind === "rejected" || forwarded.kind === "blocked") return forwarded;
        if (current.kind !== "pending" && current.kind !== "complete") { this.#failure = "Paged source step differs"; return { ...forwarded, kind: "rejected" }; }
        this.#sourceKind = current.kind; this.#copyPhase = "observe";
        return forwarded;
      }
      if (this.#copyPhase === "observe") {
        const complete = this.#field!.complete; const consumed = this.#field!.consumed; const fragment = this.#field!.fragment;
        if (complete !== (this.#sourceKind === "complete") || consumed > this.#copied || complete && (this.#copied !== this.#field!.value.byteLength || consumed !== this.#copied) || fragment !== this.#lastFragment && consumed !== this.#copied) { this.#failure = "Paged source completion differs"; return step("rejected", "paged-source-observe", 128); }
        this.#sourceKind = null; this.#copyPhase = complete ? "ready" : fragment !== this.#lastFragment ? "idle" : "continue"; return step("pending", "paged-source-observe", 128);
      }
      if (this.#copyPhase === "idle") {
        const fragment = this.#field!.fragment; if (!fragment || fragment === this.#lastFragment) return step("blocked", "paged-source-continuation");
        if (!OwnedKernelReturnInputFragment.matches(fragment, this.#field!) || fragment.offset !== this.#copied || fragment.offset + BigInt(fragment.length) > this.#field!.value.byteLength) return step("rejected", "paged-source-range");
        this.#input = fragment; this.#copyFragment = fragment; this.#inputOffset = 0; this.#copyPhase = "copy"; return step("pending", "paged-input-admit", 128);
      }
      if (this.#copyPhase === "allocate") { const current = this.#writer!.page.allocate(grant); const result = this.#copyChild(current, grant); if (result.kind !== "rejected" && current.kind === "ready") this.#copyPhase = "copy"; return result; }
      if (this.#copyPhase === "seal") { const current = this.#writer!.page.seal(grant); const result = this.#copyChild(current, grant); if (result.kind !== "rejected" && current.kind === "ready") { this.#writer = null; this.#copyPhase = "copy"; } return result; }
      if (this.#inputOffset === this.#input!.length) { this.#input = null; this.#copyPhase = "proof"; return step("pending", "paged-input-copy-detach", 128); }
      if (!this.#writer) {
        const remaining = this.#field!.value.byteLength - this.#copied; const length = Number(remaining < 256n ? remaining : 256n); const page = this.#resident!.reservePage(length); if (!page) return step("blocked", "paged-page-resident");
        const cell: PageCell = { page, length, next: null }; if (this.#tail) this.#tail.next = cell; else this.#head = cell; this.#tail = cell; this.#writer = cell; this.#written = 0; this.#copyPhase = "allocate"; return step("pending", "paged-page-reserve", 128);
      }
      const byte = this.#input!.byteAt(this.#inputOffset, this); const current = this.#writer.page.writeByte(byte, grant); const result = this.#copyChild(current, grant);
      if (current.kind === "pending" && current.items === 1 && current.bytes === 1) { this.#inputOffset++; this.#copied++; this.#written++; if (this.#written === this.#writer.length) this.#copyPhase = "seal"; }
      return result;
    } catch { this.#failure = "Paged payload copy failed"; return step("rejected", "paged-copy-fault"); }
  }
  beginRead(grant: NumericIndexGrant): OwnedUiOperationPayloadReadAdmission {
    if (!admitted(grant, 1024) || this.#closing || this.#failure || this.#copyPhase !== "ready" || this.#reader || this.#readerReserved) return { step: step("blocked", "paged-reader-admission"), reader: null };
    if (!this.#resident!.reserveReader(this)) return { step: step("blocked", "paged-reader-resident"), reader: null }; this.#readerReserved = true;
    try { this.#reader = createReader(this, this.#head); return { step: step("ready", "paged-reader-admission", 1024), reader: this.#reader }; } catch { this.#failure = "Paged reader admission failed"; return { step: step("rejected", "paged-reader-admission", 1024), reader: null }; }
  }
  //#endregion 📥️FixedPageCopy

  //#region ♻️BoundedClose
  beginClose(): void { this.#closing = true; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 128)) return step("blocked", "paged-builder-close"); if (!this.#closing) throw new Error("Paged builder close has not begun");
    try {
      if (this.#phase === "closed") return step("complete", "paged-builder-close");
      if (this.#reader && !this.#reader.terminalIsEmpty()) { this.#reader.beginClose(); return childStep(this.#reader.closeStep(grant), grant); }
      if (this.#readerReserved) { const current = this.#resident!.releaseReader(this, grant); if (current.kind === "complete") { this.#readerReserved = false; this.#reader = null; } return childStep(current, grant); }
      if (this.#copyPhase === "proof" || this.#copyPhase === "release" || this.#copyPhase === "receipt") return this.#settleCopy();
      if (this.#phase === "open") { this.#bound = OwnedKernelReturnInputField.matchesBuilder(this.#field, this); if (this.#bound) { const fragment = this.#copyFragment ?? this.#field!.fragment; this.#fragment = fragment === this.#lastFragment ? null : fragment; this.#field!.beginClose(); } this.#input = null; this.#copyFragment = null; this.#phase = this.#fragment ? "proof" : "pages"; return step("pending", "paged-input-detach", 128); }
      if (this.#phase === "proof") { const fragment = this.#fragment!; this.#proof = cancelledProof({ fragment, field: this.#field!, builder: this, offset: fragment.offset, length: fragment.length }); this.#phase = "release"; return step("pending", "paged-input-cancel-proof", 128); }
      if (this.#phase === "release") { const receipt = this.#fragment!.release(this.#proof); if (!receipt) return step("rejected", "paged-input-release-refused", 128); this.#release = receipt; this.#phase = "receipt"; return step("pending", "paged-input-release", 128); }
      if (this.#phase === "receipt") { if (!OwnedKernelReturnInputRelease.matches(this.#release, this.#fragment!, this.#proof)) return step("rejected", "paged-input-release-authority"); this.#fragment = null; this.#proof = null; this.#release = null; this.#bound = false; this.#phase = "pages"; return step("pending", "paged-input-release-retire", 128); }
      if (this.#phase === "pages" && this.#head) {
        const cell = this.#head; if (!cell.page.terminalIsEmpty()) { cell.page.beginClose(); return childStep(cell.page.closeStep(grant), grant); }
        this.#head = cell.next; cell.next = null; if (this.#tail === cell) this.#tail = null; if (this.#writer === cell) this.#writer = null; return step("pending", "paged-page-cell-close", 64);
      }
      this.#lastFragment = null; this.#reader = null; this.#sourceKind = null;
      this.#field = null; this.#resident = null; this.#bound = false; this.#phase = "closed"; return step("complete", "paged-builder-close", 128);
    } catch { this.#failure = "Paged builder close failed"; return step("rejected", "paged-builder-close-fault"); }
  }
  terminalIsEmpty(): boolean { return this.#closing && this.#phase === "closed" && !this.#field && !this.#resident && !this.#fragment && !this.#proof && !this.#release && !this.#input && !this.#copyFragment && !this.#lastFragment && !this.#copyProof && !this.#copyRelease && !this.#head && !this.#tail && !this.#writer && !this.#reader && !this.#readerReserved; }
  //#endregion ♻️BoundedClose
}

//#region 📖️RegisteredSequentialReader
/** 📖️ The charged builder retains this reader before exposure; values are individual bytes, never storage aliases. */
export class OwnedUiOperationPayloadReader {
  #builder: OwnedUiOperationPayloadBuilder | null;
  #cell: PageCell | null;
  #offset = 0;
  #closing = false;
  #failed = false;
  private constructor(mint: object, builder: OwnedUiOperationPayloadBuilder, first: PageCell | null) {
    if (mint !== MINT) throw new Error("Invalid paged reader authority"); this.#builder = builder; this.#cell = first;
    if (!installReader(builder, this)) { this.#builder = null; this.#cell = null; this.#closing = true; throw new Error("Paged reader registration refused"); }
    Object.freeze(this);
  }
  static { createReader = (builder, first) => new OwnedUiOperationPayloadReader(MINT, builder, first); }
  advance(grant: NumericIndexGrant): OwnedUiOperationPayloadReadStep {
    if (!admitted(grant, 128)) return step("blocked", "paged-reader"); if (this.#closing || this.#failed) return step("rejected", "paged-reader"); if (!this.#cell) return step("complete", "paged-reader");
    if (this.#offset === this.#cell.length) { this.#cell = this.#cell.next; this.#offset = 0; return step("pending", "paged-reader-next", 64); }
    try { const value = this.#cell.page.byteAt(this.#offset); this.#offset++; return { kind: "byte", value, items: 1, bytes: 1 }; }
    catch { this.#failed = true; return step("rejected", "paged-reader-fault"); }
  }
  beginClose(): void { this.#closing = true; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 128)) return step("blocked", "paged-reader-close"); if (!this.#closing) throw new Error("Paged reader close has not begun"); if (!this.#builder) return step("complete", "paged-reader-close");
    this.#cell = null; this.#builder = null; return step("complete", "paged-reader-close", 128);
  }
  terminalIsEmpty(): boolean { return this.#closing && !this.#cell && !this.#builder; }
}
//#endregion 📖️RegisteredSequentialReader

/** 📥️ Exact copied-input evidence; no public mint or successful copy path is exposed by this staging boundary. */
export class OwnedUiOperationInputCopied {
  readonly #proof: Evidence;
  private constructor(mint: object, proof: Evidence) { if (mint !== MINT) throw new Error("Invalid copied input authority"); this.#proof = proof; Object.freeze(this); }
  static { copiedProof = proof => new OwnedUiOperationInputCopied(MINT, proof); }
  static matches(token: unknown, fragment: object, field: object, builder: object, offset: bigint, length: number): token is OwnedUiOperationInputCopied {
    return token !== null && typeof token === "object" && #proof in token && exact(token.#proof, fragment, field, builder, offset, length);
  }
}

/** ♻️ Exact detached-input cancellation evidence, distinct from copied bytes and semantic publication. */
export class OwnedUiOperationInputCancelled {
  readonly #proof: Evidence;
  private constructor(mint: object, proof: Evidence) { if (mint !== MINT) throw new Error("Invalid cancelled input authority"); this.#proof = proof; Object.freeze(this); }
  static { cancelledProof = proof => new OwnedUiOperationInputCancelled(MINT, proof); }
  static matches(token: unknown, fragment: object, field: object, builder: object, offset: bigint, length: number): token is OwnedUiOperationInputCancelled {
    return token !== null && typeof token === "object" && #proof in token && exact(token.#proof, fragment, field, builder, offset, length);
  }
}
//#endregion 📄️PrivatePagedInputAuthority
