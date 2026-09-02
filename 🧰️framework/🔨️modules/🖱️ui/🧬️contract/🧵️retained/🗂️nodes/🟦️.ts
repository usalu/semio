//#region 🧬️OwnedIndexContract
import { NumericIndex, type NumericIndexGrant, type NumericIndexEdit, type NumericIndexReader, type NumericIndexRetirement, type NumericIndexOrdinal } from "../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️.ts";
import { OwnedUiNode, type UiNodeRetirement } from "../📦️wire/🧾️typed/🟦️.ts";
import type { RetainedUiWireStep } from "../📦️wire/🟦️.ts";

export type OwnedUiNodeReadStep = RetainedUiWireStep | { readonly kind: "value"; readonly id: number; readonly ordinal: NumericIndexOrdinal; readonly value: OwnedUiNode; readonly items: number; readonly bytes: number };
const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const state = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
let adoptIndex: (index: NumericIndex<OwnedUiNode>) => OwnedUiNodeIndex;
let adoptRetirement: (index: NumericIndexRetirement<OwnedUiNode>, node?: UiNodeRetirement | null) => OwnedUiNodeIndexRetirement;
let adoptEdit: (edit: NumericIndexEdit<OwnedUiNode>) => OwnedUiNodeIndexEdit;
let adoptRead: (read: NumericIndexReader<OwnedUiNode>) => OwnedUiNodeIndexReader;
//#endregion 🧬️OwnedIndexContract

//#region 🗂️OwnedIndex
/** 🗂️ Persistent node snapshots own exact node handles; reads transfer independent captures. */
export class OwnedUiNodeIndex {
  #index: NumericIndex<OwnedUiNode> | null;
  private constructor(index: NumericIndex<OwnedUiNode>) { this.#index = index; Object.freeze(this); }
  static { adoptIndex = index => new OwnedUiNodeIndex(index); }
  static empty(): OwnedUiNodeIndex { return adoptIndex(NumericIndex.empty<OwnedUiNode>()); }
  #live(): NumericIndex<OwnedUiNode> { if (!this.#index) throw new Error("Owned UI node index is closed"); return this.#index; }
  get size(): number { return this.#live().size; }
  capture(): OwnedUiNodeIndex { return adoptIndex(this.#live().capture()); }
  beginSet(node: OwnedUiNode): OwnedUiNodeIndexEdit { const index = this.#live(); const id = node.value.id; index.assertCaptureCapacity(); return adoptEdit(index.beginSet(id, node.capture())); }
  beginRemove(id: number): OwnedUiNodeIndexEdit { return adoptEdit(this.#live().beginRemove(id)); }
  beginRead(): OwnedUiNodeIndexReader { return adoptRead(this.#live().beginRead()); }
  beginSortedRead(): OwnedUiNodeIndexReader { return adoptRead(this.#live().beginSortedRead()); }
  beginLookup(id: number): OwnedUiNodeIndexReader { return adoptRead(this.#live().beginLookup(id)); }
  beginClose(): OwnedUiNodeIndexRetirement { const index = this.#live(); this.#index = null; return adoptRetirement(index.beginClose()); }
  terminalIsEmpty(): boolean { return this.#index === null; }
}
//#endregion 🗂️OwnedIndex

//#region ♻️EntryRetirement
export class OwnedUiNodeIndexRetirement {
  #index: NumericIndexRetirement<OwnedUiNode> | null;
  #node: UiNodeRetirement | null;
  private constructor(index: NumericIndexRetirement<OwnedUiNode>, node: UiNodeRetirement | null) { this.#index = index; this.#node = node; Object.freeze(this); }
  static { adoptRetirement = (index, node = null) => new OwnedUiNodeIndexRetirement(index, node); }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return state("blocked", "node-index-close");
    if (this.#node) { const result = this.#node.advance(grant); if (result.kind === "complete") this.#node = null; return { ...result, kind: "pending" }; }
    if (this.#index) {
      const result = this.#index.advance(grant);
      if (result.kind === "retired") { this.#node = result.value.beginClose(); return state("pending", "node-index-entry-close", result.bytes + 64); }
      if (result.kind === "complete") this.#index = null;
      return { kind: "pending", phase: "node-index-close", items: result.items, bytes: result.bytes };
    }
    return state("complete", "node-index-close");
  }
  terminalIsEmpty(): boolean { return this.#index === null && this.#node === null; }
}
//#endregion ♻️EntryRetirement

//#region ✏️RetainedEdits
export class OwnedUiNodeIndexEdit {
  #edit: NumericIndexEdit<OwnedUiNode> | null;
  #node: UiNodeRetirement | null = null;
  #failure: string | null = null;
  private constructor(edit: NumericIndexEdit<OwnedUiNode>) { this.#edit = edit; Object.freeze(this); }
  static { adoptEdit = edit => new OwnedUiNodeIndexEdit(edit); }
  get failure(): string | null { return this.#failure; }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return state("blocked", "node-index-edit");
    if (!this.#edit) throw new Error("Owned UI node edit is closed");
    if (this.#node) { const result = this.#node.advance(grant); if (result.kind === "complete") this.#node = null; return { ...result, kind: "pending" }; }
    const result = this.#edit.advance(grant);
    if (result.kind === "retired") { this.#node = result.value.beginClose(); return state("pending", "node-index-entry-close", result.bytes + 64); }
    if (result.kind === "rejected") this.#failure = result.reason;
    return { kind: result.kind, phase: "node-index-edit", items: result.items, bytes: result.bytes };
  }
  takeResult(): OwnedUiNodeIndex | null { if (!this.#edit || this.#node) return null; const result = this.#edit.takeResult(); return result ? adoptIndex(result) : null; }
  beginClose(): OwnedUiNodeIndexRetirement {
    if (!this.#edit) throw new Error("Owned UI node edit is already closed");
    const result = adoptRetirement(this.#edit.beginClose(), this.#node); this.#edit = null; this.#node = null; return result;
  }
  terminalIsEmpty(): boolean { return this.#edit === null && this.#node === null; }
}
//#endregion ✏️RetainedEdits

//#region 📖️CapturedReads
export class OwnedUiNodeIndexReader {
  #reader: NumericIndexReader<OwnedUiNode> | null;
  private constructor(reader: NumericIndexReader<OwnedUiNode>) { this.#reader = reader; Object.freeze(this); }
  static { adoptRead = reader => new OwnedUiNodeIndexReader(reader); }
  advance(grant: NumericIndexGrant): OwnedUiNodeReadStep {
    if (!admitted(grant)) return state("blocked", "node-index-read");
    if (!this.#reader) throw new Error("Owned UI node reader is closed");
    const result = this.#reader.advance(grant);
    if (result.kind === "value") return { ...result, value: result.value.capture(), bytes: result.bytes + 64 };
    return { ...result, phase: "node-index-read" };
  }
  beginClose(): OwnedUiNodeIndexRetirement { if (!this.#reader) throw new Error("Owned UI node reader is already closed"); const result = adoptRetirement(this.#reader.beginClose()); this.#reader = null; return result; }
  terminalIsEmpty(): boolean { return this.#reader === null; }
}
//#endregion 📖️CapturedReads
