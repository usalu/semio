//#region 🧬️OwnedOperations
import { NumericIndex, type NumericIndexGrant } from "../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️component.ts";
import { RetainedUiNumericTable, retainedUiComponentStrings, retainedUiAccessibilityStrings, retainedUiBindingStrings, measureRetainedUiText } from "../🟦️component.ts";
import { OwnedUiNodeIndex, type OwnedUiNodeIndexEdit, type OwnedUiNodeIndexReader, type OwnedUiNodeIndexRetirement } from "../🗂️nodes/🟦️component.ts";
import { OwnedUiNode, captureTypedUiPayload, captureUiFieldChange, type OwnedUiPayload, type RetainedUiNodeRecord, type RetainedUiTypedValues, type RetainedUiFieldChange, type UiNodeRetirement, type UiPayloadRetirement } from "../📦️wire/🧾️typed/🟦️component.ts";
import type { RetainedUiWireStep } from "../📦️wire/🟦️component.ts";

type Operation = { kind: "upsert"; node: OwnedUiNode } | { kind: "field"; id: number; change: RetainedUiFieldChange } | { kind: "activity"; id: number; payload: OwnedUiPayload<RetainedUiTypedValues["activity"]> } | { kind: "remove"; id: number } | { kind: "root"; id: number | null };
type Program = Generator<number, void, void>;
type Link = { id: number; next: Link | null };
export type OwnedUiOperationResult = { readonly nodes: OwnedUiNodeIndex; readonly root: number | null; readonly touched: NumericIndex<true>; readonly estimatedBytes: number };
const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const state = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
function id(value: number): number { if (!Number.isSafeInteger(value) || value < 0) throw new RangeError("UI node ID is not a nonnegative safe integer"); return value === 0 ? 0 : value; }
let takeOperation: (owner: OwnedUiOperation) => Operation;
let checkOperation: (owner: OwnedUiOperation) => void;

function* operationStrings(operation: Operation): Generator<string> {
  if (operation.kind === "upsert") { const node = operation.node.value; yield node.key; yield* retainedUiComponentStrings(node.component); yield* retainedUiAccessibilityStrings(node.accessibility); yield* retainedUiBindingStrings(node.bindings); yield node.menu?.id ?? ""; }
  if (operation.kind === "field") {
    const change = operation.change;
    if (change.field === "component") yield* retainedUiComponentStrings(change.payload.value);
    if (change.field === "accessibility") yield* retainedUiAccessibilityStrings(change.payload.value);
    if (change.field === "bindings") yield* retainedUiBindingStrings(change.payload.value);
    if (change.field === "menu") yield change.payload.value?.id ?? "";
  }
}

/** 🩹️ A typed operation captures exact normalized field owners, never arbitrary borrowed JSON. */
export class OwnedUiOperation {
  #value: Operation | null;
  private constructor(value: Operation) { this.#value = value; Object.freeze(this); }
  static {
    checkOperation = owner => { if (!owner.#value) throw new Error("Owned UI operation is closed"); };
    takeOperation = owner => { checkOperation(owner); const value = owner.#value!; owner.#value = null; return value; };
  }
  static upsert(payload: OwnedUiPayload<RetainedUiNodeRecord>): OwnedUiOperation { return new OwnedUiOperation({ kind: "upsert", node: OwnedUiNode.captureFrom(payload) }); }
  static field(node: number, change: RetainedUiFieldChange): OwnedUiOperation { const exact = id(node); return new OwnedUiOperation({ kind: "field", id: exact, change: captureUiFieldChange(change) }); }
  static activity(node: number, payload: OwnedUiPayload<RetainedUiTypedValues["activity"]>): OwnedUiOperation { const exact = id(node); return new OwnedUiOperation({ kind: "activity", id: exact, payload: captureTypedUiPayload("activity", payload) }); }
  static remove(node: number): OwnedUiOperation { return new OwnedUiOperation({ kind: "remove", id: id(node) }); }
  static setRoot(node: number | null): OwnedUiOperation { return new OwnedUiOperation({ kind: "root", id: node === null ? null : id(node) }); }
  beginClose(): OwnedUiOperationRetirement { return new OwnedUiOperationRetirement(takeOperation(this)); }
  terminalIsEmpty(): boolean { return this.#value === null; }
}

export class OwnedUiOperationRetirement {
  #retirement: UiNodeRetirement | UiPayloadRetirement<unknown> | null;
  constructor(value: Operation) {
    const owner = value.kind === "upsert" ? value.node : value.kind === "field" ? value.change.payload : value.kind === "activity" ? value.payload : null;
    this.#retirement = owner && !owner.terminalIsEmpty() ? owner.beginClose() : null;
    Object.freeze(this);
  }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return state("blocked", "operation-close");
    if (!this.#retirement) return state("complete", "operation-close");
    const result = this.#retirement.advance(grant); if (result.kind === "complete") this.#retirement = null;
    return { ...result, kind: "pending" };
  }
  terminalIsEmpty(): boolean { return this.#retirement === null; }
}
//#endregion 🧬️OwnedOperations

//#region 🚶️CandidateOperation
/** 🚶️ Applies one operation to a private candidate; its result is not a validation/publication token. */
export class OwnedUiOperationCursor {
  #nodes: OwnedUiNodeIndex | null;
  #root: number | null;
  #operation: Operation | null;
  #program: Program | null = null;
  #reader: OwnedUiNodeIndexReader | null = null;
  #edit: OwnedUiNodeIndexEdit | null = null;
  #old: OwnedUiNodeIndex | null = null;
  #retirement: OwnedUiNodeIndexRetirement | null = null;
  #node: OwnedUiNode | null = null;
  #replacement: OwnedUiNode | null = null;
  #nodeRetirement: UiNodeRetirement | null = null;
  #operationRetirement: OwnedUiOperationRetirement | null = null;
  #stack: Link | null = null;
  #grant: NumericIndexGrant = { maxItems: 0, maxBytes: 0 };
  #touched: RetainedUiNumericTable<true>;
  #status: "pending" | "ready" | "rejected" | "closing" | "closed" = "pending";
  #failure: string | null = null;
  #estimatedBytes = 16;
  readonly #maxChildren: number;
  readonly #maxTextBytes: number;

  constructor(source: OwnedUiNodeIndex, root: number | null, operation: OwnedUiOperation, limits: { readonly maxChildren: number; readonly maxTextBytes: number } = { maxChildren: Number.MAX_SAFE_INTEGER, maxTextBytes: Number.MAX_SAFE_INTEGER }) {
    this.#maxChildren = id(limits.maxChildren); this.#maxTextBytes = id(limits.maxTextBytes);
    checkOperation(operation); this.#root = root === null ? null : id(root);
    this.#nodes = source.capture(); this.#operation = takeOperation(operation);
    this.#touched = new RetainedUiNumericTable(NumericIndex.empty<true>(), () => this.#grant);
    Object.freeze(this);
  }
  get failure(): string | null { return this.#failure; }

  *#drain(): Program { while (this.#retirement) { const result = this.#retirement.advance(this.#grant); if (result.kind === "complete") this.#retirement = null; yield result.bytes; } }
  *#releaseNode(): Program { if (this.#node) { this.#nodeRetirement = this.#node.beginClose(); this.#node = null; yield 64; } while (this.#nodeRetirement) { const result = this.#nodeRetirement.advance(this.#grant); if (result.kind === "complete") this.#nodeRetirement = null; yield result.bytes; } }

  *#lookup(node: number): Program {
    this.#reader = this.#nodes!.beginLookup(node); yield 64;
    for (;;) { const result = this.#reader.advance(this.#grant); if (result.kind === "value") this.#node = result.value; yield result.bytes; if (result.kind === "complete") break; }
    this.#retirement = this.#reader.beginClose(); this.#reader = null; yield 64; yield* this.#drain();
  }

  *#change(edit: OwnedUiNodeIndexEdit): Program {
    this.#edit = edit; yield 64;
    for (;;) { const result = edit.advance(this.#grant); if (result.kind === "rejected") throw new Error(edit.failure ?? "Owned UI index rejected operation"); yield result.bytes; if (result.kind === "ready") break; }
    this.#old = this.#nodes; this.#nodes = edit.takeResult()!; this.#retirement = edit.beginClose(); this.#edit = null; yield 128; yield* this.#drain();
    this.#retirement = this.#old!.beginClose(); this.#old = null; yield 64; yield* this.#drain();
  }

  *#run(): Program {
    const operation = this.#operation!;
    const children = operation.kind === "upsert" ? operation.node.value.children : operation.kind === "field" && operation.change.field === "children" ? operation.change.payload.value : null;
    if (children && children.length > this.#maxChildren) throw new Error("Owned UI children quota exceeded");
    const component = operation.kind === "upsert" ? operation.node.value.component : operation.kind === "field" && operation.change.field === "component" ? operation.change.payload.value : null;
    if (component && (yield* measureRetainedUiText(retainedUiComponentStrings(component), () => this.#grant)) > this.#maxTextBytes) throw new Error("Owned UI text quota exceeded");
    this.#estimatedBytes += yield* measureRetainedUiText(operationStrings(operation), () => this.#grant);
    if (operation.kind === "field" && operation.change.field === "children") this.#estimatedBytes += operation.change.payload.value.length * 8;
    if (operation.kind === "root") { this.#root = operation.id; yield 16; }
    else if (operation.kind === "upsert") { yield* this.#change(this.#nodes!.beginSet(operation.node)); yield* this.#touched.set(operation.node.value.id, true); }
    else if (operation.kind === "remove") {
      this.#stack = { id: operation.id, next: null }; yield 32;
      while (this.#stack) {
        const cell: Link = this.#stack; this.#stack = cell.next; cell.next = null; yield 32;
        yield* this.#lookup(cell.id);
        if (!this.#node) continue;
        const children = this.#node.value.children;
        yield* this.#change(this.#nodes!.beginRemove(cell.id)); yield* this.#touched.set(cell.id, true);
        for (const child of children) { this.#stack = { id: child, next: this.#stack }; yield 32; }
        yield* this.#releaseNode();
      }
    } else {
      yield* this.#lookup(operation.id);
      if (!this.#node) throw new Error(`Unknown UI node: ${operation.id}`);
      this.#replacement = operation.kind === "field" ? this.#node.replace(operation.change) : this.#node.withActivity(operation.payload); yield 512;
      yield* this.#change(this.#nodes!.beginSet(this.#replacement)); yield* this.#touched.set(operation.id, true);
      yield* this.#releaseNode(); this.#node = this.#replacement; this.#replacement = null; yield* this.#releaseNode();
    }
    this.#operationRetirement = new OwnedUiOperationRetirement(operation); this.#operation = null; yield 64;
    while (this.#operationRetirement) { const result = this.#operationRetirement.advance(this.#grant); if (result.kind === "complete") this.#operationRetirement = null; yield result.bytes; }
  }

  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (this.#status !== "pending") { if (this.#status === "closing") throw new Error("Owned UI operation is closing"); return state(this.#status === "closed" ? "complete" : this.#status, this.#status); }
    if (!admitted(grant)) return state("blocked", "operation");
    this.#grant = grant; this.#program ??= this.#run();
    try { const result = this.#program.next(); if (result.done) { this.#program = null; this.#status = "ready"; return state("ready", "operation", 32); } if (result.value > grant.maxBytes) throw new Error("Owned UI operation exceeded its grant"); return state("pending", "operation", result.value); }
    catch (error) { this.#failure = error instanceof Error ? error.message : "Owned UI operation failed"; this.#status = "rejected"; this.#program = null; return state("rejected", "operation", 64); }
  }

  takeResult(): OwnedUiOperationResult | null { if (this.#status !== "ready" || !this.#nodes) return null; const nodes = this.#nodes; this.#nodes = null; return { nodes, root: this.#root, touched: this.#touched.take(), estimatedBytes: this.#estimatedBytes }; }
  beginClose(): void { if (this.#status === "closed" || this.#status === "closing") return; this.#status = "closing"; this.#program = null; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (this.#status === "closed") return state("complete", "operation-close");
    if (this.#status !== "closing") throw new Error("Owned UI close has not begun");
    if (!admitted(grant)) return state("blocked", "operation-close");
    if (this.#stack) { const cell = this.#stack; this.#stack = cell.next; cell.next = null; return state("pending", "operation-stack-close", 32); }
    if (this.#retirement) { const result = this.#retirement.advance(grant); if (result.kind === "complete") this.#retirement = null; return { ...result, kind: "pending" }; }
    if (this.#reader) { this.#retirement = this.#reader.beginClose(); this.#reader = null; return state("pending", "operation-reader-close", 64); }
    if (this.#edit) { this.#retirement = this.#edit.beginClose(); this.#edit = null; return state("pending", "operation-edit-close", 64); }
    if (this.#old) { this.#retirement = this.#old.beginClose(); this.#old = null; return state("pending", "operation-old-close", 64); }
    if (this.#nodeRetirement) { const result = this.#nodeRetirement.advance(grant); if (result.kind === "complete") this.#nodeRetirement = null; return { ...result, kind: "pending" }; }
    if (this.#node) { this.#nodeRetirement = this.#node.beginClose(); this.#node = null; return state("pending", "operation-node-close", 64); }
    if (this.#replacement) { this.#nodeRetirement = this.#replacement.beginClose(); this.#replacement = null; return state("pending", "operation-replacement-close", 64); }
    if (this.#operationRetirement) { const result = this.#operationRetirement.advance(grant); if (result.kind === "complete") this.#operationRetirement = null; return { ...result, kind: "pending" }; }
    if (this.#operation) { this.#operationRetirement = new OwnedUiOperationRetirement(this.#operation); this.#operation = null; return state("pending", "operation-payload-close", 64); }
    if (this.#nodes) { this.#retirement = this.#nodes.beginClose(); this.#nodes = null; return state("pending", "operation-index-close", 64); }
    const touched = this.#touched.closeStep(grant); if (!touched.complete) return state("pending", "operation-touched-close", touched.bytes);
    this.#status = "closed"; return state("complete", "operation-close");
  }
  terminalIsEmpty(): boolean { return this.#status === "closed" && !this.#program && !this.#stack && !this.#retirement && !this.#reader && !this.#edit && !this.#old && !this.#nodeRetirement && !this.#node && !this.#replacement && !this.#operationRetirement && !this.#operation && !this.#nodes; }
}
//#endregion 🚶️CandidateOperation
