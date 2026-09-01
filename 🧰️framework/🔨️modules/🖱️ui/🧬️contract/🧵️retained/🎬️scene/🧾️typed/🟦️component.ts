//#region 🧬️ProjectionContract
import catalog from "./🔣️catalog.json" with { type: "json" };
import { NumericIndex, type NumericIndexEdit, type NumericIndexGrant, type NumericIndexReader } from "../../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️component.ts";
import { OwnedUiSceneDocument, type OwnedUiSceneReader, type OwnedUiSceneValue } from "../🟦️component.ts";
import type { RetainedUiWireStep } from "../../📦️wire/🟦️component.ts";

type Literal = string | number | boolean | readonly unknown[] | null;
type Spec = { readonly fields: readonly (readonly string[])[]; readonly defaults?: Readonly<Record<string, Exclude<Literal, null>>> };
export type OwnedUiPreparedSceneField = { readonly name: string; readonly type: string; readonly source: number | null; readonly literal: Literal };
export type OwnedUiPreparedSceneRecord = { readonly schema: string; readonly source: number; readonly fields: readonly OwnedUiPreparedSceneField[] };
export type OwnedUiSceneHostProfile = { readonly usizeBits: 32 | 64 };
export type OwnedUiPreparedSceneReadStep = RetainedUiWireStep | { readonly kind: "value"; readonly value: OwnedUiPreparedSceneRecord; readonly items: number; readonly bytes: number };
type Root = { references: number; source: OwnedUiSceneDocument | null; records: NumericIndex<OwnedUiPreparedSceneRecord> | null };
type Retirement = { advance(grant: NumericIndexGrant): { readonly kind: string; readonly items: number; readonly bytes: number }; terminalIsEmpty(): boolean };
type Link = { owner: Retirement | null; next: Link | null };
type Task = ({ readonly kind: "value"; readonly source: number; readonly type: string }
  | { readonly kind: "sequence"; readonly type: string; remaining: number; position: number }
  | { readonly kind: "record"; readonly source: number; readonly name: string; readonly spec: Spec; readonly fields: (OwnedUiPreparedSceneField | null)[]; remaining: number; position: number; missing: number }) & { next: Task | null };
type Program<T> = Generator<number, T, void>;
const SPECS: Readonly<Record<string, Spec>> = catalog.records;
const GRANT = Object.freeze({ maxItems: 1, maxBytes: 4096 });
const MINT = Object.freeze({});
const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
let ownDocument: (root: Root) => OwnedUiPreparedScene;
let ownReader: (root: Root, reader: NumericIndexReader<OwnedUiPreparedSceneRecord>) => OwnedUiPreparedSceneReader;
let ownRetirement: (root: Root, reader?: Retirement | null) => OwnedUiPreparedSceneRetirement;

function freezeCatalog(): void {
  for (const spec of Object.values(SPECS)) {
    if (spec.fields.length > 32) throw new Error("Static scene schema exceeds fixed field metadata capacity");
    for (const field of spec.fields) { if (field.length !== 2 || field[0]!.length > 64 || field[1]!.length > 64) throw new Error("Static scene schema field exceeds its metadata bound"); Object.freeze(field); }
    if (spec.defaults) { for (const value of Object.values(spec.defaults)) if (Array.isArray(value)) Object.freeze(value); Object.freeze(spec.defaults); }
    Object.freeze(spec.fields); Object.freeze(spec);
  }
  for (const surface of catalog.surfaces) Object.freeze(surface);
  Object.freeze(catalog.surfaces); Object.freeze(catalog.records); Object.freeze(catalog);
}
freezeCatalog();
//#endregion 🧬️ProjectionContract

//#region 🎟️PreparedOwnership
/** 🎟️ Native-schema-checked fields reference the original scene arena without reconstructing text. */
export class OwnedUiPreparedScene {
  #root: Root | null;
  private constructor(mint: object, root: Root) { if (mint !== MINT) throw new Error("Prepared scene requires exact mint authority"); this.#root = root; Object.freeze(this); }
  static { ownDocument = root => new OwnedUiPreparedScene(MINT, root); }
  #live(): Root { if (!this.#root) throw new Error("Prepared scene is closed"); return this.#root; }
  get kind(): string { return this.#live().source!.kind; }
  get schema(): string { return this.#live().source!.schema; }
  capture(): OwnedUiPreparedScene { const root = this.#live(); if (root.references === Number.MAX_SAFE_INTEGER) throw new Error("Prepared scene reference overflow"); root.references++; return ownDocument(root); }
  beginRecord(source = 0): OwnedUiPreparedSceneReader {
    const root = this.#live(); if (root.references === Number.MAX_SAFE_INTEGER) throw new Error("Prepared scene reference overflow");
    const reader = root.records!.beginLookup(source); root.references++; return ownReader(root, reader);
  }
  beginText(source: number): OwnedUiSceneReader { return this.#live().source!.beginText(source); }
  beginTextBytes(source: number, offset = 0, length?: number): OwnedUiSceneReader { return this.#live().source!.beginTextBytes(source, offset, length); }
  beginValue(source: number): OwnedUiSceneReader { return this.#live().source!.beginRead(source); }
  beginClose(): OwnedUiPreparedSceneRetirement { const root = this.#live(); this.#root = null; return ownRetirement(root); }
  terminalIsEmpty(): boolean { return this.#root === null; }
}

export class OwnedUiPreparedSceneReader {
  #root: Root | null;
  #reader: NumericIndexReader<OwnedUiPreparedSceneRecord> | null;
  private constructor(mint: object, root: Root, reader: NumericIndexReader<OwnedUiPreparedSceneRecord>) { if (mint !== MINT) throw new Error("Prepared scene reader requires exact mint authority"); this.#root = root; this.#reader = reader; Object.freeze(this); }
  static { ownReader = (root, reader) => new OwnedUiPreparedSceneReader(MINT, root, reader); }
  advance(grant: NumericIndexGrant): OwnedUiPreparedSceneReadStep {
    if (!admitted(grant)) return step("blocked", "prepared-scene-read");
    if (!this.#reader) return step("rejected", "prepared-scene-read");
    const current = this.#reader.advance(GRANT);
    return current.kind === "value" ? { ...current, bytes: current.bytes + 64 } : { ...current, phase: "prepared-scene-read" };
  }
  beginClose(): OwnedUiPreparedSceneRetirement { if (!this.#root || !this.#reader) throw new Error("Prepared scene reader is closed"); const result = ownRetirement(this.#root, this.#reader.beginClose()); this.#root = null; this.#reader = null; return result; }
  terminalIsEmpty(): boolean { return this.#root === null && this.#reader === null; }
}

export class OwnedUiPreparedSceneRetirement {
  #root: Root | null;
  #reader: Retirement | null;
  #records: Retirement | null = null;
  #source: Retirement | null = null;
  #released = false;
  private constructor(mint: object, root: Root, reader: Retirement | null) { if (mint !== MINT) throw new Error("Prepared scene retirement requires exact mint authority"); this.#root = root; this.#reader = reader; Object.freeze(this); }
  static { ownRetirement = (root, reader = null) => new OwnedUiPreparedSceneRetirement(MINT, root, reader); }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "prepared-scene-close");
    if (this.#reader) { const current = this.#reader.advance(GRANT); if (current.kind === "complete") this.#reader = null; return step("pending", "prepared-scene-reader-close", current.bytes + (current.kind === "retired" ? 3072 : 0)); }
    if (this.#root && !this.#released) { this.#released = true; if (--this.#root.references) this.#root = null; return step("pending", "prepared-scene-release", 32); }
    if (this.#records) { const current = this.#records.advance(GRANT); if (current.kind === "complete") this.#records = null; return step("pending", "prepared-scene-record-close", current.bytes + (current.kind === "retired" ? 3072 : 0)); }
    if (this.#root?.records) { this.#records = this.#root.records.beginClose(); this.#root.records = null; return step("pending", "prepared-scene-record-close", 64); }
    if (this.#source) { const current = this.#source.advance(GRANT); if (current.kind === "complete") this.#source = null; return step("pending", "prepared-scene-source-close", current.bytes); }
    if (this.#root?.source) { this.#source = this.#root.source.beginClose(); this.#root.source = null; return step("pending", "prepared-scene-source-close", 64); }
    this.#root = null; return step("complete", "prepared-scene-close");
  }
  terminalIsEmpty(): boolean { return this.#root === null && this.#reader === null && this.#records === null && this.#source === null; }
}
//#endregion 🎟️PreparedOwnership

//#region 🧵️TypedProjection
export class OwnedUiSceneProjectionCursor {
  readonly #usizeBits: 32 | 64;
  #source: OwnedUiSceneDocument | null;
  #records: NumericIndex<OwnedUiPreparedSceneRecord> | null = NumericIndex.empty<OwnedUiPreparedSceneRecord>();
  #reader: OwnedUiSceneReader | null = null;
  #edit: NumericIndexEdit<OwnedUiPreparedSceneRecord> | null = null;
  #retirements: Link | null = null;
  #tasks: Task | null = null;
  #program: Program<void> | null = null;
  #closing = false;
  #complete = false;
  #taken = false;
  #failure: string | null = null;
  #phase = "scene-schema";
  constructor(source: OwnedUiSceneDocument, profile: OwnedUiSceneHostProfile) { const bits = profile.usizeBits; if (bits !== 32 && bits !== 64) throw new Error("Scene projection requires an owning host width"); this.#usizeBits = bits; this.#source = source.capture(); Object.freeze(this); }
  get failure(): string | null { return this.#failure; }
  #queue(owner: Retirement): void { this.#retirements = { owner, next: this.#retirements }; }
  #push(task: Task): void { task.next = this.#tasks; this.#tasks = task; }
  #drain(): number { const link = this.#retirements!; const current = link.owner!.advance(GRANT); if (current.kind === "complete") { this.#retirements = link.next; link.next = null; link.owner = null; } return current.bytes + (current.kind === "retired" ? 3072 : 32); }
  *#lookup(source: number): Program<OwnedUiSceneValue> {
    this.#reader = this.#source!.beginRead(source); let value: OwnedUiSceneValue | null = null; yield 64;
    for (;;) { const current = this.#reader.advance(GRANT); if (current.kind === "value") value = current.value; yield current.bytes; if (current.kind === "complete") break; if (current.kind === "rejected") throw new Error("scene-read-failed"); }
    this.#queue(this.#reader.beginClose()); this.#reader = null; yield 64; while (this.#retirements) yield this.#drain();
    if (!value) throw new Error("scene-record-missing"); return value;
  }
  *#fieldName(value: OwnedUiSceneValue): Program<string | null> {
    if (value.kind !== "text") throw new Error("scene-field-name-invalid");
    if (value.length > 64) return null;
    this.#reader = this.#source!.beginText(value.start); let name = ""; yield 64;
    for (;;) { const current = this.#reader.advance(GRANT); if (current.kind === "text") name += current.value; yield current.bytes; if (current.kind === "complete") break; if (current.kind === "rejected") throw new Error("scene-field-name-invalid"); }
    this.#queue(this.#reader.beginClose()); this.#reader = null; yield 64; while (this.#retirements) yield this.#drain(); return name;
  }
  *#save(record: OwnedUiPreparedSceneRecord): Program<void> {
    this.#phase = "scene-typed-record"; this.#edit = this.#records!.beginSet(record.source, record); yield 64;
    for (;;) { const current = this.#edit.advance(GRANT); yield current.bytes + (current.kind === "retired" ? 3072 : 0); if (current.kind === "ready") break; if (current.kind === "rejected") throw new Error("scene-typed-index-exhausted"); }
    const next = this.#edit.takeResult()!; this.#queue(this.#edit.beginClose()); this.#edit = null; this.#queue(this.#records!.beginClose()); this.#records = next; yield 128;
    while (this.#retirements) yield this.#drain();
  }
  *#value(task: Extract<Task, { kind: "value" }>): Program<void> {
    const value = yield* this.#lookup(task.source); const type = task.type;
    if (type.startsWith("?")) {
      if (value.kind === "none") return;
      if (value.kind !== "some") throw new Error("scene-option-tag-invalid");
      this.#push({ kind: "value", source: value.first, type: type.slice(1), next: null }); yield 64; return;
    }
    if (type.startsWith("[")) {
      if (value.kind !== "sequence") throw new Error("scene-sequence-type-invalid");
      this.#push({ kind: "sequence", type: type.slice(1, -1), remaining: value.count, position: value.first, next: null }); yield 80; return;
    }
    if (type.startsWith("#")) {
      const name = type.slice(1); const spec = SPECS[name];
      if (!spec || value.kind !== "map") throw new Error("scene-record-type-invalid");
      this.#push({ kind: "record", source: value.start, name, spec, fields: new Array<OwnedUiPreparedSceneField | null>(spec.fields.length).fill(null), remaining: value.count, position: value.first, missing: 0, next: null }); yield 128 + spec.fields.length * 8; return;
    }
    if (type === "text" && value.kind === "text") return;
    if (type === "bool" && value.kind === "boolean") return;
    if (type === "f64" && value.kind === "float" && Number.isFinite(value.value)) return;
    if (value.kind === "integer") {
      const maximum = type === "u8" ? 255n : type === "u16" ? 65535n : type === "u32" || type === "usize" && this.#usizeBits === 32 ? 4294967295n : type === "u64" || type === "usize" ? 18446744073709551615n : -1n;
      if (value.value >= 0n && value.value <= maximum) return;
    }
    throw new Error("scene-field-type-invalid");
  }
  *#record(task: Extract<Task, { kind: "record" }>): Program<void> {
    if (task.remaining) {
      const key = yield* this.#lookup(task.position); const name = yield* this.#fieldName(key); const value = yield* this.#lookup(key.end);
      task.position = value.end; task.remaining--; let field = -1;
      for (let index = 0; index < task.spec.fields.length; index++) { if (task.spec.fields[index]![0] === name) field = index; yield 64; }
      this.#push(task);
      if (field >= 0) { const spec = task.spec.fields[field]!; task.fields[field] = Object.freeze({ name: spec[0]!, type: spec[1]!, source: value.start, literal: null }); this.#push({ kind: "value", source: value.start, type: spec[1]!, next: null }); }
      yield 160; return;
    }
    if (task.missing < task.spec.fields.length) {
      const index = task.missing++; const field = task.spec.fields[index]!;
      if (!task.fields[index]) {
        const name = field[0]!; const type = field[1]!; const literal = task.spec.defaults && Object.hasOwn(task.spec.defaults, name) ? task.spec.defaults[name]! : null;
        if (literal === null && !type.startsWith("?")) throw new Error("scene-required-field-missing");
        task.fields[index] = Object.freeze({ name, type, source: null, literal });
      }
      this.#push(task); yield 128; return;
    }
    const fields: OwnedUiPreparedSceneField[] = [];
    for (const field of task.fields) { if (!field) throw new Error("scene-field-normalization-incomplete"); fields.push(field); }
    yield 64 + fields.length * 16;
    yield* this.#save(Object.freeze({ schema: task.name, source: task.source, fields: Object.freeze(fields) }));
  }
  *#prepare(): Program<void> {
    let record: string | null = null;
    for (const surface of catalog.surfaces) { if (surface.kind === this.#source!.kind && surface.schema === this.#source!.schema) record = surface.record; yield 1152; }
    if (!record) throw new Error("unsupported-scene-schema");
    this.#push({ kind: "value", source: 0, type: `#${record}`, next: null }); yield 96;
    while (this.#tasks) {
      const task = this.#tasks; this.#tasks = task.next; task.next = null; this.#phase = "scene-typed-validate"; yield 64;
      if (task.kind === "value") yield* this.#value(task);
      else if (task.kind === "record") yield* this.#record(task);
      else if (task.remaining) { const value = yield* this.#lookup(task.position); task.position = value.end; task.remaining--; this.#push(task); this.#push({ kind: "value", source: value.start, type: task.type, next: null }); yield 128; }
    }
  }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", this.#phase);
    if (this.#closing || this.#taken || this.#failure) return step("rejected", this.#phase);
    if (this.#complete) return step("ready", "scene-typed-ready");
    try { this.#program ??= this.#prepare(); const current = this.#program.next(); if (current.done) { this.#complete = true; this.#program = null; return step("ready", "scene-typed-ready", 32); } return step("pending", this.#phase, current.value); }
    catch (error) { this.#failure = error instanceof Error ? error.message : "scene-projection-failed"; return step("rejected", this.#phase, 128); }
  }
  takeResult(): OwnedUiPreparedScene | null {
    if (!this.#complete || this.#closing || this.#failure || this.#taken || !this.#source || !this.#records) return null;
    const document = ownDocument({ references: 1, source: this.#source, records: this.#records }); this.#source = null; this.#records = null; this.#taken = true; return document;
  }
  beginClose(): void { this.#closing = true; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant) || !this.#closing) return step("blocked", "scene-typed-close");
    if (this.#program) { this.#program.return(undefined); this.#program = null; return step("pending", "scene-typed-program-close", 3072); }
    if (this.#tasks) { const task = this.#tasks; this.#tasks = task.next; task.next = null; return step("pending", "scene-typed-frame-close", 3072); }
    if (this.#reader) { this.#queue(this.#reader.beginClose()); this.#reader = null; return step("pending", "scene-typed-reader-close", 64); }
    if (this.#edit) { this.#queue(this.#edit.beginClose()); this.#edit = null; return step("pending", "scene-typed-edit-close", 64); }
    if (this.#retirements) return step("pending", "scene-typed-retirement", this.#drain());
    if (this.#records) { this.#queue(this.#records.beginClose()); this.#records = null; return step("pending", "scene-typed-records-close", 64); }
    if (this.#source) { this.#queue(this.#source.beginClose()); this.#source = null; return step("pending", "scene-typed-source-close", 64); }
    return step("complete", "scene-typed-close");
  }
  terminalIsEmpty(): boolean { return this.#closing && this.#program === null && this.#tasks === null && this.#reader === null && this.#edit === null && this.#retirements === null && this.#source === null && this.#records === null; }
}
//#endregion 🧵️TypedProjection
