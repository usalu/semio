//#region 🧬️OwnedValidation
import { NumericIndex, type NumericIndexGrant } from "../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️.ts";
import type { UiContractViolation, UiDocumentLimits } from "../../../../🛂️manifest/🟦️.ts";
import { RetainedUiNumericTable, RetainedUiSiblingKeys } from "../🟦️.ts";
import { OwnedUiNodeIndex, type OwnedUiNodeIndexReader, type OwnedUiNodeIndexRetirement } from "../🗂️nodes/🟦️.ts";
import type { OwnedUiNode, RetainedUiNodeRecord, UiNodeRetirement } from "../📦️wire/🧾️typed/🟦️.ts";
import type { RetainedUiWireStep } from "../📦️wire/🟦️.ts";
import { retainedUiGraphValidation, closeRetainedUiGraphFrame, type RetainedUiGraphFrontier } from "./🔬️graph/🟦️.ts";

type Program<T> = Generator<number, T, void>;
const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const state = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
//#endregion 🧬️OwnedValidation

//#region 📖️AnchoredGraphReads
class GraphNodes {
  #index: OwnedUiNodeIndex | null;
  #reader: OwnedUiNodeIndexReader | null = null;
  #retirement: OwnedUiNodeIndexRetirement | null = null;
  #node: OwnedUiNode | null = null;
  #nodeRetirement: UiNodeRetirement | null = null;
  private readonly grant: () => NumericIndexGrant;
  constructor(source: OwnedUiNodeIndex, grant: () => NumericIndexGrant) { this.grant = grant; this.#index = source.capture(); }
  get size(): number { return this.#index!.size; }

  *#releaseNode(): Program<void> {
    if (this.#node) { this.#nodeRetirement = this.#node.beginClose(); this.#node = null; yield 64; }
    while (this.#nodeRetirement) { const result = this.#nodeRetirement.advance(this.grant()); if (result.kind === "complete") this.#nodeRetirement = null; yield result.bytes; }
  }

  *#releaseReader(): Program<void> {
    this.#retirement = this.#reader!.beginClose(); this.#reader = null; yield 64;
    while (this.#retirement) { const result = this.#retirement.advance(this.grant()); if (result.kind === "complete") this.#retirement = null; yield result.bytes; }
  }

  *lookup(id: number): Program<RetainedUiNodeRecord | undefined> {
    this.#reader = this.#index!.beginLookup(id); yield 64;
    let value: RetainedUiNodeRecord | undefined;
    for (;;) { const result = this.#reader.advance(this.grant()); if (result.kind === "value") { this.#node = result.value; value = result.value.value; } yield result.bytes; if (result.kind === "complete") break; }
    yield* this.#releaseNode(); yield* this.#releaseReader();
    return value;
  }

  *entries(): Generator<number | readonly [number, RetainedUiNodeRecord], void, void> {
    this.#reader = this.#index!.beginRead(); yield 64;
    for (;;) {
      const result = this.#reader.advance(this.grant());
      if (result.kind === "value") { this.#node = result.value; yield result.bytes; yield [result.id, result.value.value]; yield* this.#releaseNode(); }
      else yield result.bytes;
      if (result.kind === "complete") break;
    }
    yield* this.#releaseReader();
  }

  closeStep(grant: NumericIndexGrant): { complete: boolean; bytes: number } {
    if (this.#nodeRetirement) { const result = this.#nodeRetirement.advance(grant); if (result.kind === "complete") this.#nodeRetirement = null; return { complete: false, bytes: result.bytes }; }
    if (this.#node) { this.#nodeRetirement = this.#node.beginClose(); this.#node = null; return { complete: false, bytes: 64 }; }
    if (this.#retirement) { const result = this.#retirement.advance(grant); if (result.kind === "complete") this.#retirement = null; return { complete: false, bytes: result.bytes }; }
    if (this.#reader) { this.#retirement = this.#reader.beginClose(); this.#reader = null; return { complete: false, bytes: 64 }; }
    if (this.#index) { this.#retirement = this.#index.beginClose(); this.#index = null; return { complete: false, bytes: 64 }; }
    return { complete: true, bytes: 0 };
  }
}
//#endregion 📖️AnchoredGraphReads

//#region 🛡️ValidationCursor
/** 🛡️ Captures one immutable index; graph results carry no publication authority. */
export class OwnedUiValidationCursor {
  #grant: NumericIndexGrant = { maxItems: 0, maxBytes: 0 };
  #nodes: GraphNodes;
  #marks: RetainedUiNumericTable<number>;
  #violations: RetainedUiNumericTable<UiContractViolation>;
  #keys: RetainedUiSiblingKeys;
  #frontier: RetainedUiGraphFrontier = { stack: null, count: 0 };
  #program: Program<void> | null;
  #status: "pending" | "ready" | "rejected" | "closing" | "closed" = "pending";
  #failure: string | null = null;
  #taken = false;
  #close = 0;

  constructor(source: OwnedUiNodeIndex, root: number | null, limits: UiDocumentLimits) {
    if (root !== null && (!Number.isSafeInteger(root) || root < 0)) throw new RangeError("Invalid UI graph root");
    const exact = { maxNodes: limits.maxNodes, maxDepth: limits.maxDepth, maxChildren: limits.maxChildren, maxTextBytes: limits.maxTextBytes, maxPatchOps: limits.maxPatchOps, maxPatchBytes: limits.maxPatchBytes };
    for (const limit of [exact.maxNodes, exact.maxDepth, exact.maxChildren, exact.maxTextBytes, exact.maxPatchOps, exact.maxPatchBytes]) if (!Number.isSafeInteger(limit) || limit < 0) throw new RangeError("Invalid UI document limit");
    this.#nodes = new GraphNodes(source, () => this.#grant);
    this.#marks = new RetainedUiNumericTable(NumericIndex.empty<number>(), () => this.#grant);
    this.#violations = new RetainedUiNumericTable(NumericIndex.empty<UiContractViolation>(), () => this.#grant);
    this.#keys = new RetainedUiSiblingKeys(() => this.#grant);
    this.#program = retainedUiGraphValidation(this.#nodes, root, exact, this.#marks, this.#keys, this.#violations, this.#frontier);
    Object.freeze(this);
  }
  get failure(): string | null { return this.#failure; }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (this.#status !== "pending") { if (this.#status === "closing") throw new Error("Owned UI validation is closing"); return state(this.#status === "closed" ? "complete" : this.#status, "validation"); }
    if (!admitted(grant)) return state("blocked", "validation");
    this.#grant = grant;
    try { const result = this.#program!.next(); if (result.done) { this.#program = null; this.#status = "ready"; return state("ready", "validation", 32); } if (result.value > grant.maxBytes) throw new Error("Owned UI validation exceeded its byte grant"); return state("pending", "validation", result.value); }
    catch (error) { this.#failure = error instanceof Error ? error.message : "Owned UI validation failed"; this.#program = null; this.#status = "rejected"; return state("rejected", "validation", 64); }
  }
  takeResult(): NumericIndex<UiContractViolation> | null { if (this.#status !== "ready" || this.#taken) return null; this.#taken = true; return this.#violations.take(); }
  beginClose(): void { if (this.#status === "closing" || this.#status === "closed") return; this.#status = "closing"; this.#program = null; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (this.#status === "closed") return state("complete", "validation-close");
    if (this.#status !== "closing") throw new Error("Owned UI validation close has not begun");
    if (!admitted(grant)) return state("blocked", "validation-close");
    if (closeRetainedUiGraphFrame(this.#frontier)) return state("pending", "validation-stack-close", 48);
    if (this.#close < 4) { const owner = this.#close === 0 ? this.#keys : this.#close === 1 ? this.#marks : this.#close === 2 ? this.#violations : this.#nodes; const result = owner.closeStep(grant); if (result.complete) this.#close++; return state("pending", "validation-owner-close", result.bytes); }
    this.#status = "closed"; return state("complete", "validation-close");
  }
  terminalIsEmpty(): boolean { return this.#status === "closed" && this.#close === 4 && !this.#program && !this.#frontier.stack; }
}
//#endregion 🛡️ValidationCursor
