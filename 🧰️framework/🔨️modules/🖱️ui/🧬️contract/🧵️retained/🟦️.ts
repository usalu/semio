//#region 🧬️Contract
import { NumericIndex, type NumericIndexEdit, type NumericIndexReader, type NumericIndexRetirement, type NumericIndexGrant } from "../../../🌱️value/🗂️ordered/🔢️numeric/🟦️.ts";
import type { AccessibilitySpec, ActionBinding, Component, PatchRejection, UiContractViolation, UiDocumentLimits, UiNodeRecord, UiPatch, UiPatchOp, UiSnapshot } from "../../../🛂️manifest/🟦️.ts";
import { retainedUiGraphValidation, closeRetainedUiGraphFrame, type RetainedUiGraphFrontier } from "./🛡️validation/🔬️graph/🟦️.ts";

export type RetainedUiState = { readonly surface: string; readonly revision: number; readonly root: number | null; readonly nodes: NumericIndex<UiNodeRecord> };
export type RetainedUiStep = { readonly kind: "blocked" | "pending" | "ready" | "rejected" | "cancelled" | "complete"; readonly phase: string; readonly items: number; readonly bytes: number };
export type RetainedUiRejection = Exclude<PatchRejection, { type: "invariantViolated" }> | { readonly type: "invariantViolated"; readonly violations: NumericIndex<UiContractViolation> };
export type RetainedUiResult = { readonly ok: true; readonly state: RetainedUiState; readonly touched: NumericIndex<true> } | { readonly ok: false; readonly rejection: RetainedUiRejection };
type Program<T> = Generator<number, T, void>;
type Link<T> = { value: T | null; next: Link<T> | null };
const granted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 256;
const quota = (quota: "patchOps" | "patchBytes" | "children" | "textBytes", actual: number, max: number): Extract<PatchRejection, { type: "quotaExceeded" }> => ({ type: "quotaExceeded", quota, actual, max });

function copyLimits(value: UiDocumentLimits): UiDocumentLimits {
  return { maxNodes: value.maxNodes, maxDepth: value.maxDepth, maxChildren: value.maxChildren, maxTextBytes: value.maxTextBytes, maxPatchOps: value.maxPatchOps, maxPatchBytes: value.maxPatchBytes };
}

function copyRecord(value: UiNodeRecord): UiNodeRecord {
  return { id: value.id, key: value.key, component: value.component, layout: value.layout, style: value.style, activity: value.activity, disabled: value.disabled, transition: value.transition, accessibility: value.accessibility, bindings: value.bindings, menu: value.menu, children: value.children };
}
//#endregion 🧬️Contract

//#region 🗂️RetainedTable
class Table<V> {
  #index: NumericIndex<V> | null;
  #edit: NumericIndexEdit<V> | null = null;
  #reader: NumericIndexReader<V> | null = null;
  #retirement: NumericIndexRetirement<V> | null = null;
  #old: NumericIndex<V> | null = null;
  readonly grant: () => NumericIndexGrant;
  private readonly retired: (value: V) => void;

  constructor(index: NumericIndex<V>, grant: () => NumericIndexGrant, retired: (value: V) => void = () => {}) { this.grant = grant; this.retired = retired; this.#index = index; }

  get index(): NumericIndex<V> { if (!this.#index) throw new Error("Retained table owner is closed"); return this.#index; }
  get size(): number { return this.index.size; }

  *#drain(): Program<void> {
    while (this.#retirement) {
      const step = this.#retirement.advance(this.grant());
      if (step.kind === "retired") this.retired(step.value);
      if (step.kind === "complete") this.#retirement = null;
      yield step.bytes;
    }
  }

  *lookup(id: number): Program<V | undefined> {
    this.#reader = this.index.beginLookup(id);
    let result: V | undefined;
    for (;;) {
      const step = this.#reader.advance(this.grant());
      if (step.kind === "value") result = step.value;
      yield step.bytes;
      if (step.kind === "complete") break;
    }
    this.#retirement = this.#reader.beginClose(); this.#reader = null;
    yield* this.#drain();
    return result;
  }

  *entries(): Generator<number | readonly [number, V], void, void> {
    this.#reader = this.index.beginRead();
    for (;;) {
      const step = this.#reader.advance(this.grant());
      if (step.kind === "value") yield [step.id, step.value];
      else yield step.bytes;
      if (step.kind === "complete") break;
    }
    this.#retirement = this.#reader.beginClose(); this.#reader = null;
    yield* this.#drain();
  }

  *set(id: number, value: V): Program<void> { yield* this.#change(this.index.beginSet(id, value)); }
  *remove(id: number): Program<void> { yield* this.#change(this.index.beginRemove(id)); }

  *#change(edit: NumericIndexEdit<V>): Program<void> {
    this.#edit = edit;
    for (;;) {
      const step = edit.advance(this.grant());
      if (step.kind === "retired") this.retired(step.value);
      if (step.kind === "rejected") throw new RangeError("Retained UI insertion ordinal exhausted");
      yield step.bytes;
      if (step.kind === "ready") break;
    }
    this.#old = this.#index;
    this.#index = edit.takeResult();
    this.#retirement = edit.beginClose(); this.#edit = null;
    yield* this.#drain();
    this.#retirement = this.#old!.beginClose(); this.#old = null;
    yield* this.#drain();
  }

  take(): NumericIndex<V> {
    if (!this.#index || this.#edit || this.#reader || this.#retirement || this.#old) throw new Error("Retained table is not transferable");
    const result = this.#index; this.#index = null;
    return result;
  }

  closeStep(grant: NumericIndexGrant): { readonly complete: boolean; readonly bytes: number } {
    if (this.#retirement) {
      const step = this.#retirement.advance(grant);
      if (step.kind === "retired") this.retired(step.value);
      if (step.kind === "complete") this.#retirement = null;
      return { complete: false, bytes: step.bytes };
    }
    if (this.#reader) { this.#retirement = this.#reader.beginClose(); this.#reader = null; return { complete: false, bytes: 64 }; }
    if (this.#edit) { this.#retirement = this.#edit.beginClose(); this.#edit = null; return { complete: false, bytes: 128 }; }
    if (this.#old) { this.#retirement = this.#old.beginClose(); this.#old = null; return { complete: false, bytes: 64 }; }
    if (this.#index) { this.#retirement = this.#index.beginClose(); this.#index = null; return { complete: false, bytes: 64 }; }
    return { complete: true, bytes: 0 };
  }
}
export { Table as RetainedUiNumericTable };
//#endregion 🗂️RetainedTable

//#region 🔡️TextAccounting
function* componentStrings(component: Component | import("./📦️wire/🧾️typed/🟦️.ts").RetainedUiComponent): Generator<string> {
  switch (component.type) {
    case "container": yield component.label ?? ""; yield component.description ?? ""; yield component.error ?? ""; break;
    case "text": yield component.value; break;
    case "button": yield component.label; break;
    case "input": yield component.value; yield component.placeholder ?? ""; break;
    case "select": for (const item of component.items) yield item.label; yield component.placeholder ?? ""; break;
    case "toggle": yield component.text ?? ""; break;
    case "keyValueList": for (const entry of component.entries) { yield entry.label; yield entry.value; } break;
    case "treeSection": yield component.label ?? ""; break;
    case "treeItem": yield component.label; yield component.description ?? ""; break;
    case "image": yield component.alt ?? ""; break;
    case "extension": yield component.extension; break;
  }
}

function* accessibilityStrings(value: AccessibilitySpec): Generator<string> { yield value.label ?? ""; yield value.description ?? ""; yield value.shortcut ?? ""; }
function* bindingStrings(values: readonly ActionBinding[]): Generator<string> { for (const value of values) { yield value.action.scope; yield value.action.name; yield value.capability ?? ""; } }

function* opStrings(op: UiPatchOp): Generator<string> {
  switch (op.type) {
    case "upsert": yield op.key; yield* componentStrings(op.component); yield* accessibilityStrings(op.accessibility); yield* bindingStrings(op.bindings ?? []); yield op.menu?.id ?? ""; break;
    case "setComponent": yield* componentStrings(op.component); break;
    case "setAccessibility": yield* accessibilityStrings(op.accessibility); break;
    case "setBindings": yield* bindingStrings(op.bindings); break;
    case "setMenu": yield op.menu?.id ?? ""; break;
  }
}

function* stringBytes(value: string, grant: () => NumericIndexGrant): Program<number> {
  let offset = 0;
  let total = 0;
  yield 16;
  while (offset < value.length) {
    let consumed = 0;
    const budget = Math.min(4096, grant().maxBytes);
    while (offset < value.length && consumed + 8 <= budget) {
      const code = value.charCodeAt(offset++);
      if (code < 0x80) total++;
      else if (code < 0x800) total += 2;
      else if (code >= 0xd800 && code <= 0xdbff && offset < value.length && value.charCodeAt(offset) >= 0xdc00 && value.charCodeAt(offset) <= 0xdfff) { offset++; total += 4; }
      else total += 3;
      consumed += 8;
    }
    yield consumed;
  }
  return total;
}

function* measure(values: Generator<string>, grant: () => NumericIndexGrant): Program<number> {
  let total = 0;
  for (const value of values) total += yield* stringBytes(value, grant);
  return total;
}
export { componentStrings as retainedUiComponentStrings, accessibilityStrings as retainedUiAccessibilityStrings, bindingStrings as retainedUiBindingStrings, measure as measureRetainedUiText };

//#endregion 🔡️TextAccounting

//#region 🔑️SiblingKeys
type KeyCell = { key: string | null; collision: KeyCell | null; ownedNext: KeyCell | null };

class SiblingKeys {
  #table: Table<KeyCell> | null = null;
  #owned: KeyCell | null = null;
  private readonly grant: () => NumericIndexGrant;
  constructor(grant: () => NumericIndexGrant) { this.grant = grant; }

  *insert(key: string): Program<boolean> {
    this.#table ??= new Table(NumericIndex.empty<KeyCell>(), this.grant);
    let hash = 0x811c9dc5;
    let offset = 0;
    yield 32;
    while (offset < key.length) {
      let work = 0;
      while (offset < key.length && work + 8 <= this.grant().maxBytes) {
        const unit = key.charCodeAt(offset++);
        hash = Math.imul(hash ^ (unit & 255), 0x01000193) >>> 0;
        hash = Math.imul(hash ^ (unit >>> 8), 0x01000193) >>> 0;
        work += 8;
      }
      yield work;
    }
    const head = yield* this.#table.lookup(hash);
    let cell = head;
    while (cell) {
      const existing = cell.key!;
      yield 16;
      if (existing.length === key.length) {
        offset = 0;
        let mismatch = false;
        while (offset < key.length && !mismatch) {
          let work = 0;
          while (offset < key.length && work + 8 <= this.grant().maxBytes) {
            const left = key.charCodeAt(offset);
            const right = existing.charCodeAt(offset);
            work += 8;
            if (left !== right) { mismatch = true; break; }
            offset++;
          }
          yield work;
        }
        if (!mismatch) return true;
      }
      cell = cell.collision ?? undefined;
    }
    const inserted = { key, collision: head ?? null, ownedNext: this.#owned };
    this.#owned = inserted;
    yield 48;
    yield* this.#table.set(hash, inserted);
    return false;
  }

  *clear(): Program<void> { for (;;) { const step = this.closeStep(this.grant()); yield step.bytes; if (step.complete) return; } }

  closeStep(grant: NumericIndexGrant): { readonly complete: boolean; readonly bytes: number } {
    if (this.#table) {
      const step = this.#table.closeStep(grant);
      if (step.complete) this.#table = null;
      return { complete: false, bytes: step.bytes };
    }
    if (this.#owned) {
      const cell = this.#owned; this.#owned = cell.ownedNext;
      cell.key = null; cell.collision = null; cell.ownedNext = null;
      return { complete: false, bytes: 48 };
    }
    return { complete: true, bytes: 0 };
  }
}
export { SiblingKeys as RetainedUiSiblingKeys };
//#endregion 🔑️SiblingKeys

//#region 🩹️Transaction
type Resource = { closeStep(grant: NumericIndexGrant): { readonly complete: boolean; readonly bytes: number } };

export class RetainedUiPatchCursor {
  #grant: NumericIndexGrant = { maxItems: 0, maxBytes: 0 };
  #nodes: Table<UiNodeRecord>;
  #touched: Table<true>;
  #marks: Table<number>;
  #violations: Table<UiContractViolation>;
  #keys: SiblingKeys;
  #resources: Link<Resource> | null = null;
  #graph: RetainedUiGraphFrontier = { stack: null, count: 0 };
  #remove: Link<number> | null = null;
  #program: Program<RetainedUiResult | undefined> | null = null;
  #result: RetainedUiResult | null = null;
  #root: number | null;
  #closing = false;
  #closed = false;
  #outcome: "ready" | "rejected" | null = null;
  #phase = "admission";
  private source: RetainedUiState | null;
  private patch: UiPatch | null;
  private readonly limits: UiDocumentLimits;

  constructor(source: RetainedUiState, patch: UiPatch, limits: UiDocumentLimits) {
    this.source = { surface: source.surface, revision: source.revision, root: source.root, nodes: source.nodes };
    this.patch = patch;
    this.limits = copyLimits(limits);
    this.#nodes = this.#table(source.nodes.capture());
    this.#touched = this.#table(NumericIndex.empty<true>());
    this.#marks = this.#table(NumericIndex.empty<number>());
    this.#violations = this.#table(NumericIndex.empty<UiContractViolation>());
    this.#keys = new SiblingKeys(() => this.#grant);
    this.#resources = { value: this.#keys, next: this.#resources };
    this.#root = source.root;
  }

  #table<V>(index: NumericIndex<V>): Table<V> {
    const table = new Table(index, () => this.#grant);
    this.#resources = { value: table, next: this.#resources };
    return table;
  }

  *#apply(op: UiPatchOp): Program<Exclude<PatchRejection, { type: "invariantViolated" }> | null> {
    if (op.type === "setRoot") { this.#root = op.id; yield 16; return null; }
    if (op.type === "upsert" || op.type === "setComponent") {
      if (op.type === "upsert" && (op.children?.length ?? 0) > this.limits.maxChildren) return quota("children", op.children!.length, this.limits.maxChildren);
      const bytes = yield* measure(componentStrings(op.component), () => this.#grant);
      if (bytes > this.limits.maxTextBytes) return quota("textBytes", bytes, this.limits.maxTextBytes);
    }
    if (op.type === "setChildren" && op.children.length > this.limits.maxChildren) return quota("children", op.children.length, this.limits.maxChildren);
    if (op.type === "remove") {
      this.#remove = { value: op.id, next: null };
      while (this.#remove) {
        const item: Link<number> = this.#remove; this.#remove = item.next; item.next = null;
        const id = item.value!; item.value = null;
        yield 32;
        const record = yield* this.#nodes.lookup(id);
        if (!record) continue;
        yield* this.#nodes.remove(id);
        yield* this.#touched.set(id, true);
        for (const child of record.children ?? []) { this.#remove = { value: child, next: this.#remove }; yield 32; }
      }
      return null;
    }
    let record: UiNodeRecord;
    if (op.type === "upsert") {
      record = { id: op.id, key: op.key, component: op.component, layout: op.layout, style: op.style, activity: op.activity, disabled: op.disabled, transition: op.transition, accessibility: op.accessibility, bindings: op.bindings, menu: op.menu, children: op.children };
    } else {
      const current = yield* this.#nodes.lookup(op.id);
      if (!current) return { type: "unknownNode", id: op.id };
      record = copyRecord(current);
      switch (op.type) {
        case "setComponent": record.component = op.component; break;
        case "setLayout": record.layout = op.layout; break;
        case "setActivity": record.activity = op.activity; record.disabled = op.disabled; break;
        case "setChildren": record.children = op.children; break;
        case "setStyle": record.style = op.style; break;
        case "setAccessibility": record.accessibility = op.accessibility; break;
        case "setBindings": record.bindings = op.bindings; break;
        case "setMenu": record.menu = op.menu; break;
      }
    }
    yield 192;
    yield* this.#nodes.set(op.id, record);
    yield* this.#touched.set(op.id, true);
    return null;
  }

  *#validate(): Program<void> {
    yield* retainedUiGraphValidation(this.#nodes, this.#root, this.limits, this.#marks, this.#keys, this.#violations, this.#graph);
  }

  *#run(): Program<RetainedUiResult | undefined> {
    const patch = this.patch!;
    const source = this.source!;
    if (patch.baseRevision !== source.revision) return { ok: false, rejection: { type: "revisionMismatch", expected: source.revision, actual: patch.baseRevision } };
    if (patch.ops.length > this.limits.maxPatchOps) return { ok: false, rejection: quota("patchOps", patch.ops.length, this.limits.maxPatchOps) };
    this.#phase = "accounting";
    let bytes = 0;
    for (const op of patch.ops) {
      bytes += 16 + (op.type === "setChildren" ? op.children.length * 8 : 0);
      yield 32;
      bytes += yield* measure(opStrings(op), () => this.#grant);
    }
    if (bytes > this.limits.maxPatchBytes) return { ok: false, rejection: quota("patchBytes", bytes, this.limits.maxPatchBytes) };
    this.#phase = "application";
    for (const op of patch.ops) {
      yield 16;
      const rejection = yield* this.#apply(op);
      if (rejection) return { ok: false, rejection };
    }
    this.#phase = "validation";
    yield* this.#validate();
    if (this.#graph.count) return { ok: false, rejection: { type: "invariantViolated", violations: this.#violations.take() } };
    this.#phase = "candidate";
    return { ok: true, state: { surface: source.surface, revision: patch.revision, root: this.#root, nodes: this.#nodes.take() }, touched: this.#touched.take() };
  }

  advance(grant: NumericIndexGrant): RetainedUiStep {
    if (this.#closed) return { kind: "complete", phase: "closed", items: 0, bytes: 0 };
    if (this.#closing) throw new Error("Use closeStep after cancelling a retained UI patch");
    if (this.#outcome) return { kind: this.#outcome, phase: this.#phase, items: 0, bytes: 0 };
    if (!granted(grant)) return { kind: "blocked", phase: this.#phase, items: 0, bytes: 0 };
    this.#grant = grant;
    this.#program ??= this.#run();
    const step = this.#program.next();
    if (step.done) {
      if (!step.value) throw new Error("Retained UI work ended without an outcome");
      this.#result = step.value; this.#program = null;
      this.#outcome = step.value.ok ? "ready" : "rejected";
      return { kind: step.value.ok ? "ready" : "rejected", phase: this.#phase, items: 1, bytes: 128 };
    }
    if (step.value > grant.maxBytes) throw new Error("Retained UI work exceeded its byte grant");
    return { kind: "pending", phase: this.#phase, items: 1, bytes: step.value };
  }

  takeResult(): RetainedUiResult | null { const result = this.#result; this.#result = null; return result; }

  beginClose(): void {
    if (this.#closing || this.#closed) return;
    this.#closing = true;
    this.#program?.return(undefined);
    this.#program = null;
    if (this.#result?.ok) {
      this.#table(this.#result.state.nodes);
      this.#table(this.#result.touched);
    } else if (this.#result?.rejection.type === "invariantViolated") this.#table(this.#result.rejection.violations);
    this.#result = null;
    this.source = null; this.patch = null;
  }

  closeStep(grant: NumericIndexGrant): RetainedUiStep {
    if (!this.#closing) throw new Error("Retained UI close has not begun");
    if (!granted(grant)) return { kind: "blocked", phase: "retirement", items: 0, bytes: 0 };
    if (closeRetainedUiGraphFrame(this.#graph)) return { kind: "pending", phase: "retirement", items: 1, bytes: 48 };
    if (this.#remove) { const cell = this.#remove; this.#remove = cell.next; cell.next = null; cell.value = null; return { kind: "pending", phase: "retirement", items: 1, bytes: 32 }; }
    if (this.#resources) {
      const cell = this.#resources;
      const step = cell.value!.closeStep(grant);
      if (step.complete) { this.#resources = cell.next; cell.next = null; cell.value = null; }
      return { kind: "pending", phase: "retirement", items: 1, bytes: step.bytes };
    }
    this.#closed = true;
    return { kind: "complete", phase: "closed", items: 0, bytes: 0 };
  }

  terminalIsEmpty(): boolean { return this.#closed && !this.#resources && !this.#program && !this.#graph.stack && !this.#remove && !this.#result; }
}
//#endregion 🩹️Transaction

//#region 📸️Hydration
export class RetainedUiSnapshotCursor {
  #grant: NumericIndexGrant = { maxItems: 0, maxBytes: 0 };
  #table: Table<UiNodeRecord>;
  #program: Program<void> | null = null;
  #offset = 0;
  #closing = false;
  #closed = false;
  private snapshot: UiSnapshot | null;

  constructor(snapshot: UiSnapshot) { this.snapshot = snapshot; this.#table = new Table(NumericIndex.empty(), () => this.#grant); }

  advance(grant: NumericIndexGrant): RetainedUiStep {
    if (this.#closed) return { kind: "complete", phase: "closed", items: 0, bytes: 0 };
    if (this.#closing) throw new Error("Snapshot hydration is closing");
    if (!granted(grant)) return { kind: "blocked", phase: "hydration", items: 0, bytes: 0 };
    this.#grant = grant;
    if (!this.#program) {
      if (this.#offset === this.snapshot!.nodes.length) return { kind: "ready", phase: "hydration", items: 0, bytes: 0 };
      const record = this.snapshot!.nodes[this.#offset++]!;
      this.#program = this.#table.set(record.id, record);
    }
    const step = this.#program.next();
    if (step.done) this.#program = null;
    return { kind: "pending", phase: "hydration", items: 1, bytes: step.done ? 16 : step.value };
  }

  takeResult(): RetainedUiState | null {
    if (this.#closed || this.#closing || this.#program || this.#offset !== this.snapshot!.nodes.length) return null;
    const snapshot = this.snapshot!;
    const nodes = this.#table.take();
    this.snapshot = null; this.#closed = true;
    return { surface: snapshot.surface, revision: snapshot.revision, root: snapshot.root, nodes };
  }

  beginClose(): void { this.#closing = true; this.#program?.return(); this.#program = null; this.snapshot = null; }

  closeStep(grant: NumericIndexGrant): RetainedUiStep {
    if (!this.#closing) throw new Error("Snapshot close has not begun");
    if (!granted(grant)) return { kind: "blocked", phase: "retirement", items: 0, bytes: 0 };
    const step = this.#table.closeStep(grant);
    if (step.complete) this.#closed = true;
    return { kind: step.complete ? "complete" : "pending", phase: "retirement", items: step.complete ? 0 : 1, bytes: step.bytes };
  }

  terminalIsEmpty(): boolean { return this.#closed && this.snapshot === null && this.#program === null; }
}
//#endregion 📸️Hydration

//#region 📬️Publication
export type RetainedUiSurfaceIdentity = { readonly actor: string; readonly instance: number; readonly surface: string };
export type RetainedUiAcknowledgement = { readonly identity: RetainedUiSurfaceIdentity; readonly revision: number };
let publishPrepared: (owner: RetainedUiSurfaceOwner, transaction: RetainedUiTransaction) => boolean;
let beginTransaction: (owner: RetainedUiSurfaceOwner, source: RetainedUiState, patch: UiPatch, limits: UiDocumentLimits) => RetainedUiTransaction;
let acceptRoot: (owner: RetainedUiSurfaceOwner, source: RetainedUiState, candidate: RetainedUiState) => boolean;

export class RetainedUiSurfaceOwner {
  #current: RetainedUiState | null;
  readonly identity: RetainedUiSurfaceIdentity;
  #limits: UiDocumentLimits;

  static {
    acceptRoot = (owner, source, candidate) => {
      if (owner.#current !== source) return false;
      owner.#current = candidate;
      return true;
    };
  }

  constructor(actor: string, instance: number, initial: RetainedUiState, limits: UiDocumentLimits) {
    if (!Number.isSafeInteger(instance) || instance < 0 || instance > 0xffff_ffff) throw new RangeError("Retained UI instance is not a u32");
    this.identity = Object.freeze({ actor, instance, surface: initial.surface });
    this.#current = Object.freeze({ surface: initial.surface, revision: initial.revision, root: initial.root, nodes: initial.nodes });
    this.#limits = copyLimits(limits);
  }

  capture(): RetainedUiState {
    const current = this.#current;
    if (!current) throw new Error("Retained UI surface is closed");
    return { surface: current.surface, revision: current.revision, root: current.root, nodes: current.nodes.capture() };
  }

  get revision(): number { if (!this.#current) throw new Error("Retained UI surface is closed"); return this.#current.revision; }
  getNode(id: number): UiNodeRecord | undefined { if (!this.#current) throw new Error("Retained UI surface is closed"); return this.#current.nodes.get(id); }

  beginPatch(patch: UiPatch): RetainedUiTransaction {
    if (!this.#current) throw new Error("Retained UI surface is closed");
    return beginTransaction(this, this.#current, patch, this.#limits);
  }

  publish(transaction: RetainedUiTransaction): boolean { return publishPrepared(this, transaction); }

  beginClose(): NumericIndexRetirement<UiNodeRecord> {
    if (!this.#current) throw new Error("Retained UI surface is already closed");
    const retirement = this.#current.nodes.beginClose(); this.#current = null;
    return retirement;
  }

  terminalIsEmpty(): boolean { return this.#current === null; }
}

export class RetainedUiTransaction {
  #owner: RetainedUiSurfaceOwner | null;
  #source: RetainedUiState | null;
  #job: RetainedUiPatchCursor | null;
  #patch: UiPatch | null;
  #identityOffset = 0;
  #identityReady = false;
  #candidate: Extract<RetainedUiResult, { ok: true }> | null = null;
  #rejection: RetainedUiRejection | null = null;
  #ack: RetainedUiAcknowledgement | null = null;
  #nodesRetirement: NumericIndexRetirement<UiNodeRecord> | null = null;
  #touchedRetirement: NumericIndexRetirement<true> | null = null;
  #violationRetirement: NumericIndexRetirement<UiContractViolation> | null = null;
  #previousRetirement: NumericIndexRetirement<UiNodeRecord> | null = null;
  #status: "pending" | "ready" | "rejected" | "published" | "closing" | "closed" = "pending";

  static {
    beginTransaction = (owner, source, patch, limits) => new RetainedUiTransaction(owner, source, patch, limits);
    publishPrepared = (owner, transaction) => transaction.#publish(owner);
  }

  private constructor(owner: RetainedUiSurfaceOwner, source: RetainedUiState, patch: UiPatch, limits: UiDocumentLimits) {
    this.#owner = owner; this.#source = source; this.#patch = patch;
    this.#job = new RetainedUiPatchCursor(source, patch, limits);
  }

  advance(grant: NumericIndexGrant): RetainedUiStep {
    if (this.#status === "closed") return { kind: "complete", phase: "closed", items: 0, bytes: 0 };
    if (this.#status === "closing") throw new Error("Retained transaction is closing");
    if (this.#status !== "pending") return { kind: this.#status === "rejected" ? "rejected" : "ready", phase: this.#status, items: 0, bytes: 0 };
    if (!granted(grant)) return { kind: "blocked", phase: "identity", items: 0, bytes: 0 };
    if (!this.#identityReady) {
      const expected = this.#owner!.identity.surface;
      const actual = this.#patch!.surface;
      if (expected.length !== actual.length) { this.#status = "rejected"; return { kind: "rejected", phase: "identity", items: 1, bytes: 16 }; }
      let bytes = 0;
      while (this.#identityOffset < expected.length && bytes + 8 <= grant.maxBytes) {
        const offset = this.#identityOffset++;
        bytes += 8;
        if (expected.charCodeAt(offset) !== actual.charCodeAt(offset)) { this.#status = "rejected"; return { kind: "rejected", phase: "identity", items: 1, bytes }; }
      }
      this.#identityReady = this.#identityOffset === expected.length;
      return { kind: "pending", phase: "identity", items: 1, bytes };
    }
    const step = this.#job!.advance(grant);
    if (step.kind === "ready" || step.kind === "rejected") {
      const result = this.#job!.takeResult()!;
      if (result.ok) { this.#candidate = result; this.#status = "ready"; }
      else { this.#rejection = result.rejection; this.#status = "rejected"; }
    }
    return step;
  }

  #publish(owner: RetainedUiSurfaceOwner): boolean {
    if (this.#status !== "ready" || owner !== this.#owner || !this.#candidate || !this.#source) return false;
    if (!acceptRoot(owner, this.#source, this.#candidate.state)) { this.#status = "rejected"; return false; }
    this.#previousRetirement = this.#source.nodes.beginClose();
    this.#touchedRetirement = this.#candidate.touched.beginClose();
    this.#ack = { identity: owner.identity, revision: this.#candidate.state.revision };
    this.#candidate = null;
    this.#status = "published";
    return true;
  }

  takeAcknowledgement(): RetainedUiAcknowledgement | null { const ack = this.#ack; this.#ack = null; return ack; }

  beginClose(): void {
    if (this.#status === "closing" || this.#status === "closed") return;
    this.#job!.beginClose();
    if (this.#candidate) {
      this.#nodesRetirement = this.#candidate.state.nodes.beginClose();
      this.#touchedRetirement = this.#candidate.touched.beginClose();
      this.#candidate = null;
    }
    if (this.#rejection?.type === "invariantViolated") this.#violationRetirement = this.#rejection.violations.beginClose();
    this.#rejection = null; this.#ack = null; this.#owner = null; this.#source = null; this.#patch = null;
    this.#status = "closing";
  }

  closeStep(grant: NumericIndexGrant): RetainedUiStep {
    if (this.#status === "closed") return { kind: "complete", phase: "closed", items: 0, bytes: 0 };
    if (this.#status !== "closing") throw new Error("Retained transaction close has not begun");
    if (!granted(grant)) return { kind: "blocked", phase: "retirement", items: 0, bytes: 0 };
    if (this.#job) {
      const step = this.#job.closeStep(grant);
      if (step.kind === "complete") this.#job = null;
      return { ...step, kind: "pending" };
    }
    const advance = <V>(cursor: NumericIndexRetirement<V>) => cursor.advance(grant);
    if (this.#nodesRetirement) { const step = advance(this.#nodesRetirement); if (step.kind === "complete") this.#nodesRetirement = null; return { kind: "pending", phase: "retirement", items: step.items, bytes: step.bytes }; }
    if (this.#touchedRetirement) { const step = advance(this.#touchedRetirement); if (step.kind === "complete") this.#touchedRetirement = null; return { kind: "pending", phase: "retirement", items: step.items, bytes: step.bytes }; }
    if (this.#violationRetirement) { const step = advance(this.#violationRetirement); if (step.kind === "complete") this.#violationRetirement = null; return { kind: "pending", phase: "retirement", items: step.items, bytes: step.bytes }; }
    if (this.#previousRetirement) { const step = advance(this.#previousRetirement); if (step.kind === "complete") this.#previousRetirement = null; return { kind: "pending", phase: "retirement", items: step.items, bytes: step.bytes }; }
    this.#status = "closed";
    return { kind: "complete", phase: "closed", items: 0, bytes: 0 };
  }

  terminalIsEmpty(): boolean { return this.#status === "closed" && !this.#job && !this.#nodesRetirement && !this.#touchedRetirement && !this.#violationRetirement && !this.#previousRetirement; }
}
//#endregion 📬️Publication
