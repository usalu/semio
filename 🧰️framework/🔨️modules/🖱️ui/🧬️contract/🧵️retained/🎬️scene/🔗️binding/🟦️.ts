//#region 🧬️BindingContract
import type { NumericIndexGrant } from "../../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️.ts";
import { OwnedUiNode, type OwnedUiPayload, type RetainedUiComponent, type RetainedUiNodeRecord } from "../../📦️wire/🧾️typed/🟦️.ts";
import type { RetainedUiWireStep } from "../../📦️wire/🟦️.ts";
import { OwnedUiSceneCursor, type OwnedUiSceneDocument, type OwnedUiSceneReader } from "../🟦️.ts";
import { OwnedUiSceneProjectionCursor, type OwnedUiPreparedScene, type OwnedUiPreparedSceneReader, type OwnedUiSceneHostProfile } from "../🧾️typed/🟦️.ts";

export type OwnedUiSceneDiagnostic = { readonly code: "invalid-scene-packet" | "unsupported-scene-schema" | "invalid-scene-fields" };
type Root = { references: number; readonly usizeBits: 32 | 64; node: OwnedUiNode | null; scene: OwnedUiPreparedScene | null; diagnostic: OwnedUiSceneDiagnostic | null };
type Retirement = { advance(grant: NumericIndexGrant): RetainedUiWireStep; terminalIsEmpty(): boolean };
type Link = { owner: Retirement | null; next: Link | null };
type Program = Generator<number, void, void>;
const GRANT = Object.freeze({ maxItems: 1, maxBytes: 4096 });
const MINT = Object.freeze({});
const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
let own: (root: Root) => OwnedUiSceneBinding;
let retire: (root: Root) => OwnedUiSceneBindingRetirement;
let exact: (owner: OwnedUiSceneBinding) => Root;
//#endregion 🧬️BindingContract

//#region 🔗️PairedReadOwner
/** 🔗️ A single captured root binds a wire node and its exact prepared scene without exposing destructive owners. */
export class OwnedUiSceneBinding {
  #root: Root | null;
  private constructor(mint: object, root: Root) { if (mint !== MINT) throw new Error("Scene binding requires exact mint authority"); this.#root = root; Object.freeze(this); }
  static { own = root => new OwnedUiSceneBinding(MINT, root); exact = owner => owner.#live(); }
  #live(): Root { if (!this.#root) throw new Error("Scene binding is closed"); return this.#root; }
  get value(): RetainedUiNodeRecord { return this.#live().node!.value; }
  get diagnostic(): OwnedUiSceneDiagnostic | null { return this.#live().diagnostic; }
  get prepared(): boolean { return this.#live().scene !== null; }
  capture(): OwnedUiSceneBinding { const root = this.#live(); if (root.references === Number.MAX_SAFE_INTEGER) throw new Error("Scene binding reference overflow"); root.references++; return own(root); }
  beginRecord(source = 0): OwnedUiPreparedSceneReader | null { return this.#live().scene?.beginRecord(source) ?? null; }
  beginText(source: number): OwnedUiSceneReader | null { return this.#live().scene?.beginText(source) ?? null; }
  beginValue(source: number): OwnedUiSceneReader | null { return this.#live().scene?.beginValue(source) ?? null; }
  beginClose(): OwnedUiSceneBindingRetirement { const root = this.#live(); this.#root = null; return retire(root); }
  terminalIsEmpty(): boolean { return this.#root === null; }
}

export class OwnedUiSceneBindingRetirement {
  #root: Root | null;
  #active: Retirement | null = null;
  #released = false;
  private constructor(mint: object, root: Root) { if (mint !== MINT) throw new Error("Scene binding retirement requires exact mint authority"); this.#root = root; Object.freeze(this); }
  static { retire = root => new OwnedUiSceneBindingRetirement(MINT, root); }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "scene-binding-close");
    if (this.#active) { const current = this.#active.advance(GRANT); if (current.kind === "complete") this.#active = null; return { ...current, kind: "pending" }; }
    if (!this.#root) return step("complete", "scene-binding-close");
    if (!this.#released) { this.#released = true; if (--this.#root.references) this.#root = null; return step("pending", "scene-binding-release", 32); }
    if (this.#root.scene) { this.#active = this.#root.scene.beginClose(); this.#root.scene = null; return step("pending", "scene-binding-projection-close", 64); }
    if (this.#root.node) { this.#active = this.#root.node.beginClose(); this.#root.node = null; return step("pending", "scene-binding-node-close", 64); }
    this.#root.diagnostic = null; this.#root = null; return step("complete", "scene-binding-close", 64);
  }
  terminalIsEmpty(): boolean { return this.#root === null && this.#active === null; }
}
//#endregion 🔗️PairedReadOwner

//#region 🧵️BindingPreparation
export class OwnedUiSceneBindingCursor {
  readonly #profile: OwnedUiSceneHostProfile;
  #node: OwnedUiNode | null;
  #previous: OwnedUiSceneBinding | null = null;
  #component: OwnedUiPayload<RetainedUiComponent> | null = null;
  #parser: OwnedUiSceneCursor | null = null;
  #raw: OwnedUiSceneDocument | null = null;
  #projection: OwnedUiSceneProjectionCursor | null = null;
  #scene: OwnedUiPreparedScene | null = null;
  #retirements: Link | null = null;
  #diagnostic: OwnedUiSceneDiagnostic | null = null;
  #program: Program | null = null;
  #started = false;
  #ready = false;
  #closing = false;
  #taken = false;
  #failure: string | null = null;
  #phase = "scene-binding-admission";
  constructor(node: OwnedUiNode, profile: OwnedUiSceneHostProfile) { const bits = profile.usizeBits; if (bits !== 32 && bits !== 64) throw new Error("Scene binding requires an owning host width"); this.#profile = Object.freeze({ usizeBits: bits }); this.#node = node.capture(); Object.freeze(this); }
  get failure(): string | null { return this.#failure; }
  considerPrevious(previous: OwnedUiSceneBinding): void { if (this.#started || this.#closing || this.#previous) throw new Error("Scene binding reuse admission is closed"); this.#previous = previous.capture(); }
  #queue(owner: Retirement): void { this.#retirements = { owner, next: this.#retirements }; }
  #drain(): number { const link = this.#retirements!; if (!link.owner) { this.#retirements = link.next; link.next = null; return 32; } const current = link.owner.advance(GRANT); if (current.kind === "complete") link.owner = null; return current.bytes; }
  *#prepare(): Program {
    this.#started = true;
    if (this.#previous) {
      const previous = exact(this.#previous);
      if (previous.usizeBits === this.#profile.usizeBits && previous.node!.value.component === this.#node!.value.component) { this.#scene = previous.scene?.capture() ?? null; this.#diagnostic = previous.diagnostic; }
      this.#queue(this.#previous.beginClose()); this.#previous = null; yield 128;
      while (this.#retirements) yield this.#drain();
      if (this.#scene || this.#diagnostic) return;
    }
    if (this.#node!.value.component.type !== "surface") return;
    this.#component = this.#node!.captureComponent(); yield 64;
    this.#parser = new OwnedUiSceneCursor(this.#component); this.#queue(this.#component.beginClose()); this.#component = null; yield 128;
    while (this.#retirements) yield this.#drain();
    this.#phase = "scene-binding-packet";
    for (;;) { const current = this.#parser.advance(GRANT); yield current.bytes; if (current.kind === "ready") { this.#raw = this.#parser.takeResult()!; break; } if (current.kind === "rejected") { this.#diagnostic = Object.freeze({ code: "invalid-scene-packet" }); break; } }
    this.#parser.beginClose(); yield 32;
    while (this.#parser) { const current = this.#parser.closeStep(GRANT); if (current.kind === "complete") this.#parser = null; yield current.bytes; }
    if (!this.#raw) return;
    this.#projection = new OwnedUiSceneProjectionCursor(this.#raw, this.#profile); this.#queue(this.#raw.beginClose()); this.#raw = null; yield 128;
    while (this.#retirements) yield this.#drain();
    this.#phase = "scene-binding-fields";
    for (;;) { const current = this.#projection.advance(GRANT); yield current.bytes; if (current.kind === "ready") { this.#scene = this.#projection.takeResult()!; break; } if (current.kind === "rejected") { this.#diagnostic = Object.freeze({ code: this.#projection.failure === "unsupported-scene-schema" ? "unsupported-scene-schema" : "invalid-scene-fields" }); break; } }
    this.#projection.beginClose(); yield 32;
    while (this.#projection) { const current = this.#projection.closeStep(GRANT); if (current.kind === "complete") this.#projection = null; yield current.bytes; }
  }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", this.#phase);
    if (this.#closing || this.#taken || this.#failure) return step("rejected", this.#phase);
    if (this.#ready) return step("ready", "scene-binding-ready");
    try { this.#program ??= this.#prepare(); const current = this.#program.next(); if (current.done) { this.#program = null; this.#ready = true; return step("ready", "scene-binding-ready", 32); } return step("pending", this.#phase, current.value); }
    catch (error) { this.#failure = error instanceof Error ? error.message : "scene-binding-failed"; return step("rejected", this.#phase, 128); }
  }
  takeResult(): OwnedUiSceneBinding | null {
    if (!this.#ready || this.#taken || this.#closing || this.#failure || !this.#node) return null;
    const result = own({ references: 1, usizeBits: this.#profile.usizeBits, node: this.#node, scene: this.#scene, diagnostic: this.#diagnostic }); this.#node = null; this.#scene = null; this.#diagnostic = null; this.#taken = true; return result;
  }
  beginClose(): void { this.#closing = true; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!this.#closing || !admitted(grant)) return step("blocked", "scene-binding-close");
    if (this.#program) { this.#program.return(undefined); this.#program = null; return step("pending", "scene-binding-program-close", 128); }
    if (this.#parser) { this.#parser.beginClose(); const current = this.#parser.closeStep(GRANT); if (current.kind === "complete") this.#parser = null; return { ...current, kind: "pending" }; }
    if (this.#projection) { this.#projection.beginClose(); const current = this.#projection.closeStep(GRANT); if (current.kind === "complete") this.#projection = null; return { ...current, kind: "pending" }; }
    if (this.#retirements) return step("pending", "scene-binding-retirement", this.#drain());
    if (this.#raw) { this.#queue(this.#raw.beginClose()); this.#raw = null; return step("pending", "scene-binding-raw-close", 64); }
    if (this.#component) { this.#queue(this.#component.beginClose()); this.#component = null; return step("pending", "scene-binding-component-close", 64); }
    if (this.#scene) { this.#queue(this.#scene.beginClose()); this.#scene = null; return step("pending", "scene-binding-scene-close", 64); }
    if (this.#previous) { this.#queue(this.#previous.beginClose()); this.#previous = null; return step("pending", "scene-binding-previous-close", 64); }
    if (this.#node) { this.#queue(this.#node.beginClose()); this.#node = null; return step("pending", "scene-binding-node-close", 64); }
    this.#diagnostic = null; return step("complete", "scene-binding-close");
  }
  terminalIsEmpty(): boolean { return this.#closing && this.#program === null && this.#parser === null && this.#projection === null && this.#retirements === null && this.#raw === null && this.#component === null && this.#scene === null && this.#previous === null && this.#node === null && this.#diagnostic === null; }
}
//#endregion 🧵️BindingPreparation
