//#region 🧬️BindingIndexContract
import { NumericIndex, type NumericIndexEdit, type NumericIndexGrant, type NumericIndexOrdinal, type NumericIndexReader, type NumericIndexRetirement } from "../../../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️.ts";
import { OwnedUiSceneBinding, type OwnedUiSceneBindingRetirement } from "../🟦️.ts";
import type { RetainedUiWireStep } from "../../../📦️wire/🟦️.ts";
export type OwnedUiSceneBindingReadStep = RetainedUiWireStep | { readonly kind: "value"; readonly id: number; readonly ordinal: NumericIndexOrdinal; readonly value: OwnedUiSceneBinding; readonly items: number; readonly bytes: number };
const MINT = Object.freeze({});
const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
let own: (index: NumericIndex<OwnedUiSceneBinding>) => OwnedUiSceneBindingIndex;
let edit: (owner: NumericIndexEdit<OwnedUiSceneBinding>) => OwnedUiSceneBindingIndexEdit;
let read: (owner: NumericIndexReader<OwnedUiSceneBinding>) => OwnedUiSceneBindingIndexReader;
let retire: (owner: NumericIndexRetirement<OwnedUiSceneBinding>, active?: OwnedUiSceneBindingRetirement | null) => OwnedUiSceneBindingIndexRetirement;
//#endregion 🧬️BindingIndexContract

//#region 🗂️OwnedBindings
export class OwnedUiSceneBindingIndex {
  #index: NumericIndex<OwnedUiSceneBinding> | null;
  private constructor(mint: object, index: NumericIndex<OwnedUiSceneBinding>) { if (mint !== MINT) throw new Error("Binding index requires exact mint authority"); this.#index = index; Object.freeze(this); }
  static { own = index => new OwnedUiSceneBindingIndex(MINT, index); }
  static empty(): OwnedUiSceneBindingIndex { return own(NumericIndex.empty<OwnedUiSceneBinding>()); }
  #live(): NumericIndex<OwnedUiSceneBinding> { if (!this.#index) throw new Error("Binding index is closed"); return this.#index; }
  get size(): number { return this.#live().size; }
  capture(): OwnedUiSceneBindingIndex { return own(this.#live().capture()); }
  beginSet(binding: OwnedUiSceneBinding): OwnedUiSceneBindingIndexEdit { const index = this.#live(); const id = binding.value.id; index.assertCaptureCapacity(); return edit(index.beginSet(id, binding.capture())); }
  beginRemove(id: number): OwnedUiSceneBindingIndexEdit { return edit(this.#live().beginRemove(id)); }
  beginLookup(id: number): OwnedUiSceneBindingIndexReader { return read(this.#live().beginLookup(id)); }
  beginRead(): OwnedUiSceneBindingIndexReader { return read(this.#live().beginRead()); }
  beginClose(): OwnedUiSceneBindingIndexRetirement { const index = this.#live(); this.#index = null; return retire(index.beginClose()); }
  terminalIsEmpty(): boolean { return this.#index === null; }
}
//#endregion 🗂️OwnedBindings

//#region ♻️BindingEntryRetirement
export class OwnedUiSceneBindingIndexRetirement {
  #owner: NumericIndexRetirement<OwnedUiSceneBinding> | null;
  #active: OwnedUiSceneBindingRetirement | null;
  private constructor(mint: object, owner: NumericIndexRetirement<OwnedUiSceneBinding>, active: OwnedUiSceneBindingRetirement | null) { if (mint !== MINT) throw new Error("Binding index retirement requires exact mint authority"); this.#owner = owner; this.#active = active; Object.freeze(this); }
  static { retire = (owner, active = null) => new OwnedUiSceneBindingIndexRetirement(MINT, owner, active); }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "binding-index-close");
    if (this.#active) { const current = this.#active.advance(grant); if (current.kind === "complete") this.#active = null; return { ...current, kind: "pending" }; }
    if (!this.#owner) return step("complete", "binding-index-close");
    const current = this.#owner.advance(grant);
    if (current.kind === "retired") { this.#active = current.value.beginClose(); return step("pending", "binding-index-entry-close", current.bytes + 64); }
    if (current.kind === "complete") this.#owner = null;
    return { kind: "pending", phase: "binding-index-close", items: current.items, bytes: current.bytes };
  }
  terminalIsEmpty(): boolean { return this.#owner === null && this.#active === null; }
}
//#endregion ♻️BindingEntryRetirement

//#region ✏️BindingEdits
export class OwnedUiSceneBindingIndexEdit {
  #owner: NumericIndexEdit<OwnedUiSceneBinding> | null;
  #active: OwnedUiSceneBindingRetirement | null = null;
  #failure: string | null = null;
  private constructor(mint: object, owner: NumericIndexEdit<OwnedUiSceneBinding>) { if (mint !== MINT) throw new Error("Binding edit requires exact mint authority"); this.#owner = owner; Object.freeze(this); }
  static { edit = owner => new OwnedUiSceneBindingIndexEdit(MINT, owner); }
  get failure(): string | null { return this.#failure; }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "binding-index-edit");
    if (!this.#owner) throw new Error("Binding edit is closed");
    if (this.#active) { const current = this.#active.advance(grant); if (current.kind === "complete") this.#active = null; return { ...current, kind: "pending" }; }
    const current = this.#owner.advance(grant);
    if (current.kind === "retired") { this.#active = current.value.beginClose(); return step("pending", "binding-index-entry-close", current.bytes + 64); }
    if (current.kind === "rejected") this.#failure = current.reason;
    return { kind: current.kind, phase: "binding-index-edit", items: current.items, bytes: current.bytes };
  }
  takeResult(): OwnedUiSceneBindingIndex | null { if (!this.#owner || this.#active) return null; const result = this.#owner.takeResult(); return result ? own(result) : null; }
  beginClose(): OwnedUiSceneBindingIndexRetirement { if (!this.#owner) throw new Error("Binding edit is closed"); const result = retire(this.#owner.beginClose(), this.#active); this.#owner = null; this.#active = null; return result; }
  terminalIsEmpty(): boolean { return this.#owner === null && this.#active === null; }
}
//#endregion ✏️BindingEdits

//#region 📖️BindingReads
export class OwnedUiSceneBindingIndexReader {
  #owner: NumericIndexReader<OwnedUiSceneBinding> | null;
  private constructor(mint: object, owner: NumericIndexReader<OwnedUiSceneBinding>) { if (mint !== MINT) throw new Error("Binding reader requires exact mint authority"); this.#owner = owner; Object.freeze(this); }
  static { read = owner => new OwnedUiSceneBindingIndexReader(MINT, owner); }
  advance(grant: NumericIndexGrant): OwnedUiSceneBindingReadStep {
    if (!admitted(grant)) return step("blocked", "binding-index-read");
    if (!this.#owner) throw new Error("Binding reader is closed");
    const current = this.#owner.advance(grant);
    if (current.kind === "value") return { ...current, value: current.value.capture(), bytes: current.bytes + 64 };
    return { ...current, phase: "binding-index-read" };
  }
  beginClose(): OwnedUiSceneBindingIndexRetirement { if (!this.#owner) throw new Error("Binding reader is closed"); const result = retire(this.#owner.beginClose()); this.#owner = null; return result; }
  terminalIsEmpty(): boolean { return this.#owner === null; }
}
//#endregion 📖️BindingReads
