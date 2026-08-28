//#region 🧬️JsonDocumentContract
import { NumericIndex, type NumericIndexEdit, type NumericIndexReader } from "../../../../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️component.ts";
import { OwnedUiPreparedScene } from "../../🟦️component.ts";
import { OwnedUiSceneJsonCursor, type OwnedUiSceneJsonToken } from "../🟦️component.ts";
import type { OwnedUiSceneReader } from "../../../🟦️component.ts";
import type { RetainedUiWireStep } from "../../../../📦️wire/🟦️component.ts";

type Grant = { readonly maxItems: number; readonly maxBytes: number };
type Retirement = { advance(grant: Grant): { readonly kind: string; readonly items: number; readonly bytes: number }; terminalIsEmpty(): boolean };
type Link = { owner: Retirement | null; next: Link | null; complete: boolean };
type Root = { references: number; readonly field: number; source: OwnedUiPreparedScene | null; records: NumericIndex<OwnedUiSceneJsonToken> | null };
const GRANT = Object.freeze({ maxItems: 1, maxBytes: 4096 });
const MINT = Object.freeze({});
const admitted = (grant: Grant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
function forward(current: { readonly kind: string; readonly items: number; readonly bytes: number }, phase: string, terminal = false): RetainedUiWireStep {
  const bytes = current.bytes + (current.kind === "retired" ? 96 : 0);
  const invalid = !Number.isSafeInteger(bytes) || bytes < 0 || bytes > 4096 || !Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1;
  return { kind: invalid || current.kind === "rejected" ? "rejected" : current.kind === "blocked" ? "blocked" : terminal && current.kind === "complete" ? "complete" : "pending", phase, items: current.items, bytes };
}
let ownDocument: (root: Root) => OwnedUiSceneJsonDocument;
let ownReader: (root: Root, reader: NumericIndexReader<OwnedUiSceneJsonToken>) => OwnedUiSceneJsonDocumentReader;
let ownRetirement: (root: Root, reader?: Retirement | null) => OwnedUiSceneJsonDocumentRetirement;
//#endregion 🧬️JsonDocumentContract

//#region 🎟️JsonDocumentOwnership
export class OwnedUiSceneJsonDocument {
  #root: Root | null;
  private constructor(mint: object, root: Root) { if (mint !== MINT) throw new Error("JSON document requires exact grammar authority"); this.#root = root; Object.freeze(this); }
  static { ownDocument = root => new OwnedUiSceneJsonDocument(MINT, root); }
  #live(): Root { if (!this.#root) throw new Error("JSON document is closed"); return this.#root; }
  #capacity(): Root { const root = this.#live(); if (root.references >= Number.MAX_SAFE_INTEGER) throw new Error("JSON document reader capacity exceeded"); return root; }
  capture(): OwnedUiSceneJsonDocument { const root = this.#capacity(); root.references++; return ownDocument(root); }
  #read(id: number | null): OwnedUiSceneJsonDocumentReader { const root = this.#capacity(); const reader = id === null ? root.records!.beginRead() : root.records!.beginLookup(id); root.references++; return ownReader(root, reader); }
  beginRead(): OwnedUiSceneJsonDocumentReader { return this.#read(null); }
  beginLookup(id: number): OwnedUiSceneJsonDocumentReader { return this.#read(id); }
  beginSpan(offset: number, length: number): OwnedUiSceneReader { const root = this.#live(); return root.source!.beginTextBytes(root.field, offset, length); }
  beginClose(): OwnedUiSceneJsonDocumentRetirement { const root = this.#live(); this.#root = null; return ownRetirement(root); }
  terminalIsEmpty(): boolean { return this.#root === null; }
}
export class OwnedUiSceneJsonDocumentReader {
  #root: Root | null;
  #reader: NumericIndexReader<OwnedUiSceneJsonToken> | null;
  #failed = false;
  private constructor(mint: object, root: Root, reader: NumericIndexReader<OwnedUiSceneJsonToken>) { if (mint !== MINT) throw new Error("JSON reader requires exact document authority"); this.#root = root; this.#reader = reader; Object.freeze(this); }
  static { ownReader = (root, reader) => new OwnedUiSceneJsonDocumentReader(MINT, root, reader); }
  advance(grant: Grant): RetainedUiWireStep | { readonly kind: "value"; readonly id: number; readonly value: OwnedUiSceneJsonToken; readonly items: number; readonly bytes: number } {
    if (!admitted(grant)) return step("blocked", "json-document-read"); if (!this.#reader || this.#failed) return step("rejected", "json-document-read");
    const current = this.#reader.advance(GRANT); const result = forward(current, "json-document-read", true); if (result.kind === "rejected") this.#failed = true;
    return current.kind === "value" && !this.#failed ? current : result;
  }
  beginClose(): OwnedUiSceneJsonDocumentRetirement { if (!this.#root || !this.#reader) throw new Error("JSON reader is closed"); const retirement = ownRetirement(this.#root, this.#reader.beginClose()); this.#root = null; this.#reader = null; return retirement; }
  terminalIsEmpty(): boolean { return this.#root === null && this.#reader === null; }
}
export class OwnedUiSceneJsonDocumentRetirement {
  #root: Root | null;
  #reader: Retirement | null;
  #records: Retirement | null = null;
  #source: Retirement | null = null;
  #released = false;
  private constructor(mint: object, root: Root, reader: Retirement | null) { if (mint !== MINT) throw new Error("JSON retirement requires exact document authority"); this.#root = root; this.#reader = reader; Object.freeze(this); }
  static { ownRetirement = (root, reader = null) => new OwnedUiSceneJsonDocumentRetirement(MINT, root, reader); }
  advance(grant: Grant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "json-document-close");
    if (this.#reader) { const current = this.#reader.advance(GRANT); const result = forward(current, "json-document-reader-close"); if (current.kind === "complete" && result.kind === "pending") this.#reader = null; return result; }
    if (this.#root && !this.#released) { this.#released = true; if (--this.#root.references) this.#root = null; return step("pending", "json-document-release", 32); }
    if (this.#records) { const current = this.#records.advance(GRANT); const result = forward(current, "json-document-record-close"); if (current.kind === "complete" && result.kind === "pending") this.#records = null; return result; }
    if (this.#root?.records) { this.#records = this.#root.records.beginClose(); this.#root.records = null; return step("pending", "json-document-record-close", 64); }
    if (this.#source) { const current = this.#source.advance(GRANT); const result = forward(current, "json-document-source-close"); if (current.kind === "complete" && result.kind === "pending") this.#source = null; return result; }
    if (this.#root?.source) { this.#source = this.#root.source.beginClose(); this.#root.source = null; return step("pending", "json-document-source-close", 64); }
    this.#root = null; return step("complete", "json-document-close");
  }
  terminalIsEmpty(): boolean { return this.#root === null && this.#reader === null && this.#records === null && this.#source === null; }
}
//#endregion 🎟️JsonDocumentOwnership

//#region 🧵️JsonDocumentPreparation
/** 🧾️ Publishes immutable flat JSON token spans only after exact source validation and parser retirement. */
export class OwnedUiSceneJsonDocumentCursor {
  readonly #field: number;
  #source: OwnedUiPreparedScene | null;
  #records: NumericIndex<OwnedUiSceneJsonToken> | null = NumericIndex.empty<OwnedUiSceneJsonToken>();
  #parser: OwnedUiSceneJsonCursor | null = null;
  #token: OwnedUiSceneJsonToken | null = null;
  #edit: NumericIndexEdit<OwnedUiSceneJsonToken> | null = null;
  #retirements: Link | null = null;
  #phase: "start" | "parse" | "edit" | "install" | "parser-close" | "ready" = "start";
  #offset = 0;
  #failure: string | null = null;
  #closing = false;
  #taken = false;
  constructor(source: OwnedUiPreparedScene, field: number) { if (!Number.isSafeInteger(field) || field < 0) throw new Error("JSON document requires an exact field"); this.#field = field; this.#source = OwnedUiPreparedScene.prototype.capture.call(source); Object.freeze(this); }
  get offset(): number { return this.#offset; }
  get failure(): string | null { return this.#failure; }
  #queue(owner: Retirement): void { this.#retirements = { owner, next: this.#retirements, complete: false }; }
  #drain(): RetainedUiWireStep {
    const link = this.#retirements!;
    if (link.complete) { this.#retirements = link.next; link.next = null; link.owner = null; return step("pending", "json-document-owner-close", 32); }
    const current = link.owner!.advance(GRANT); const result = forward(current, "json-document-owner-close");
    if (current.kind === "complete" && result.kind === "pending") { if (!link.owner!.terminalIsEmpty()) return { ...result, kind: "rejected" }; link.complete = true; } return result;
  }
  #observe(current: RetainedUiWireStep): RetainedUiWireStep { if (current.kind === "rejected") this.#failure = this.#parser?.failure ?? "JSON document child rejected"; return current; }
  advance(grant: Grant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "json-document"); if (this.#closing || this.#failure) return step("rejected", "json-document"); if (this.#phase === "ready") return step("ready", "json-document");
    try {
      if (this.#retirements) return this.#observe(this.#drain());
      if (this.#phase === "start") { this.#parser = new OwnedUiSceneJsonCursor(this.#source!, this.#field); this.#phase = "parse"; return step("pending", "json-document-parser", 128); }
      if (this.#phase === "install") { const next = this.#edit!.takeResult(); if (!next) throw new Error("JSON token index lost its result"); this.#queue(this.#edit!.beginClose()); this.#edit = null; this.#queue(this.#records!.beginClose()); this.#records = next; this.#phase = "parse"; return step("pending", "json-document-install", 256); }
      if (this.#edit) { const current = this.#edit.advance(GRANT); const result = this.#observe(forward(current, "json-document-edit")); if (current.kind === "ready" && result.kind === "pending") this.#phase = "install"; return result; }
      if (this.#token) { this.#edit = this.#records!.beginSet(this.#records!.size, this.#token); this.#token = null; this.#phase = "edit"; return step("pending", "json-document-token", 160); }
      if (this.#phase === "parser-close") { const current = this.#parser!.closeStep(GRANT); const result = this.#observe(forward(current, "json-document-parser-close")); if (current.kind === "complete" && result.kind === "pending") { this.#parser = null; this.#phase = "ready"; } return result; }
      const current = this.#parser!.advance(GRANT); this.#offset = this.#parser!.offset; const result = this.#observe(forward(current, "json-document-parse")); if (result.kind === "rejected" || result.kind === "blocked") return result;
      if (current.kind === "token") this.#token = current.token;
      if (current.kind === "ready") { this.#parser!.beginClose(); this.#phase = "parser-close"; }
      return result;
    } catch (error) { this.#failure = error instanceof Error ? error.message : "JSON document preparation failed"; return step("rejected", "json-document", 128); }
  }
  takeResult(): OwnedUiSceneJsonDocument | null { if (this.#phase !== "ready" || this.#closing || this.#failure || this.#taken || !this.#source || !this.#records) return null; const root: Root = { references: 1, field: this.#field, source: this.#source, records: this.#records }; this.#source = null; this.#records = null; this.#taken = true; return ownDocument(root); }
  beginClose(): void { this.#closing = true; this.#parser?.beginClose(); }
  closeStep(grant: Grant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "json-document-close"); this.#closing = true;
    if (this.#parser) { this.#parser.beginClose(); const current = this.#parser.closeStep(GRANT); const result = forward(current, "json-document-parser-close"); if (current.kind === "complete" && result.kind === "pending") this.#parser = null; return result; }
    if (this.#token) { this.#token = null; return step("pending", "json-document-token-close", 96); }
    if (this.#retirements) return this.#drain();
    if (this.#edit) { this.#queue(this.#edit.beginClose()); this.#edit = null; return step("pending", "json-document-edit-close", 64); }
    if (this.#records) { this.#queue(this.#records.beginClose()); this.#records = null; return step("pending", "json-document-record-close", 64); }
    if (this.#source) { this.#queue(this.#source.beginClose()); this.#source = null; return step("pending", "json-document-source-close", 64); }
    return step("complete", "json-document-close");
  }
  terminalIsEmpty(): boolean { return this.#closing && this.#parser === null && this.#token === null && this.#edit === null && this.#retirements === null && this.#source === null && this.#records === null; }
}
//#endregion 🧵️JsonDocumentPreparation
