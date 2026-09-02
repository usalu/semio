//#region 🧬️NativeOperationContract
import type { NumericIndexGrant } from "../../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️.ts";
import { OwnedUiOperation, type OwnedUiOperationRetirement } from "../🟦️.ts";
import { RetainedUiTypedCursor, RetainedUiChildIdsCursor, type RetainedUiTypedValues, type OwnedUiPayload, type UiPayloadRetirement } from "../../📦️wire/🧾️typed/🟦️.ts";
import type { RetainedUiWireStep } from "../../📦️wire/🟦️.ts";
import { OwnedUiSurface, type OwnedUiSurfacePatch, type OwnedUiSurfaceAcknowledgement } from "../../🖼️surface/🟦️.ts";

type Profile = Exclude<keyof RetainedUiTypedValues, "children">;
type Packed = { [P in Profile]: { readonly kind: P; readonly cursor: RetainedUiTypedCursor<P> } }[Profile];
const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
function childStep(current: RetainedUiWireStep, grant: NumericIndexGrant): RetainedUiWireStep {
  if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes) return { ...current, kind: "rejected" };
  return current.kind === "complete" || current.kind === "ready" ? { ...current, kind: "pending" } : current;
}
function nodeId(value: unknown): number {
  if (typeof value === "bigint") { if (value < 0n || value > 9007199254740991n) throw new Error("Native node ID exceeds the exact renderer range"); return Number(value); }
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0) throw new Error("Native node ID is not a nonnegative safe integer"); return value === 0 ? 0 : value;
}
function data(value: unknown, key: string): unknown {
  if (!value || typeof value !== "object") throw new Error("Native operation requires its exact data record");
  const descriptor = Object.getOwnPropertyDescriptor(value, key);
  if (!descriptor || !("value" in descriptor)) throw new Error("Native operation field cannot be an accessor or inherited");
  return descriptor.value;
}
function nativeOperation(value: unknown): OwnedUiWireOperationCursor {
  const tag = data(value, "tag"); if (typeof tag !== "string") throw new Error("Native operation tag is missing");
  const payload = data(value, "val");
  if (tag === "remove" || tag === "set-root") return new OwnedUiWireOperationCursor(tag, payload);
  if (tag === "upsert") return new OwnedUiWireOperationCursor(tag, undefined, data(payload, "node"));
  switch (tag) {
    case "set-component": return new OwnedUiWireOperationCursor(tag, data(payload, "node"), data(payload, "component"));
    case "set-layout": return new OwnedUiWireOperationCursor(tag, data(payload, "node"), data(payload, "layout"));
    case "set-activity": return new OwnedUiWireOperationCursor(tag, data(payload, "node"), data(payload, "activity"));
    case "set-children": return new OwnedUiWireOperationCursor(tag, data(payload, "node"), data(payload, "children"));
    case "set-style": return new OwnedUiWireOperationCursor(tag, data(payload, "node"), data(payload, "style"));
    case "set-accessibility": return new OwnedUiWireOperationCursor(tag, data(payload, "node"), data(payload, "accessibility"));
    case "set-bindings": return new OwnedUiWireOperationCursor(tag, data(payload, "node"), data(payload, "bindings"));
    case "set-menu": return new OwnedUiWireOperationCursor(tag, data(payload, "node"), data(payload, "menu"));
    default: throw new Error("Unknown native patch tag");
  }
}
//#endregion 🧬️NativeOperationContract

//#region 📥️OwnedOperationIntake
/** 📥️ Consumes one native tag's exact field payload, retaining decoder and operation ownership until terminal transfer. */
export class OwnedUiWireOperationCursor {
  static fromNative(value: unknown): OwnedUiWireOperationCursor { return nativeOperation(value); }
  readonly #id: number | null;
  readonly #tag: string;
  #packed: Packed | null = null;
  #children: RetainedUiChildIdsCursor | null = null;
  #payload: OwnedUiPayload<unknown> | null = null;
  #payloadClose: UiPayloadRetirement<unknown> | null = null;
  #operation: OwnedUiOperation | null = null;
  #operationClose: OwnedUiOperationRetirement | null = null;
  #phase = "native-operation-decode";
  #failure: string | null = null;
  #closing = false;
  #ready = false;

  constructor(tag: string, target: unknown, payload?: unknown) {
    if (typeof tag !== "string" || tag.length > 32) throw new Error("Invalid native patch tag");
    this.#tag = tag; this.#id = tag === "upsert" ? null : nodeId(target);
    if (tag === "upsert" && target !== undefined) throw new Error("Upsert identity is inside its exact node payload");
    if (tag === "remove" || tag === "set-root") { if (payload !== undefined) throw new Error("Scalar native operation cannot carry a payload"); }
    else if (tag === "set-children") this.#children = new RetainedUiChildIdsCursor(payload);
    else {
      switch (tag) {
        case "upsert": this.#packed = { kind: "node", cursor: new RetainedUiTypedCursor(payload, "node") }; break;
        case "set-component": this.#packed = { kind: "component", cursor: new RetainedUiTypedCursor(payload, "component") }; break;
        case "set-layout": this.#packed = { kind: "layout", cursor: new RetainedUiTypedCursor(payload, "layout") }; break;
        case "set-activity": this.#packed = { kind: "activity", cursor: new RetainedUiTypedCursor(payload, "activity") }; break;
        case "set-style": this.#packed = { kind: "style", cursor: new RetainedUiTypedCursor(payload, "style") }; break;
        case "set-accessibility": this.#packed = { kind: "accessibility", cursor: new RetainedUiTypedCursor(payload, "accessibility") }; break;
        case "set-bindings": this.#packed = { kind: "bindings", cursor: new RetainedUiTypedCursor(payload, "bindings") }; break;
        case "set-menu": this.#packed = { kind: "menu", cursor: new RetainedUiTypedCursor(payload, "menu") }; break;
        default: throw new Error("Unknown native patch tag");
      }
    }
    Object.freeze(this);
  }
  get failure(): string | null { return this.#failure; }
  #capture<T>(payload: OwnedUiPayload<T> | null, create: (value: OwnedUiPayload<T>) => OwnedUiOperation): void {
    if (!payload) throw new Error("Native patch decoder lost its exact payload"); this.#payload = payload; this.#operation = create(payload);
  }
  #capturePacked(packed: Packed): void {
    switch (packed.kind) {
      case "node": this.#capture(packed.cursor.takeResult(), value => OwnedUiOperation.upsert(value)); break;
      case "component": this.#capture(packed.cursor.takeResult(), value => OwnedUiOperation.field(this.#id!, { field: "component", payload: value })); break;
      case "layout": this.#capture(packed.cursor.takeResult(), value => OwnedUiOperation.field(this.#id!, { field: "layout", payload: value })); break;
      case "style": this.#capture(packed.cursor.takeResult(), value => OwnedUiOperation.field(this.#id!, { field: "style", payload: value })); break;
      case "accessibility": this.#capture(packed.cursor.takeResult(), value => OwnedUiOperation.field(this.#id!, { field: "accessibility", payload: value })); break;
      case "bindings": this.#capture(packed.cursor.takeResult(), value => OwnedUiOperation.field(this.#id!, { field: "bindings", payload: value })); break;
      case "menu": this.#capture(packed.cursor.takeResult(), value => OwnedUiOperation.field(this.#id!, { field: "menu", payload: value })); break;
      case "activity": this.#capture(packed.cursor.takeResult(), value => OwnedUiOperation.activity(this.#id!, value)); break;
    }
  }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", this.#phase); if (this.#closing || this.#failure) return step("rejected", this.#phase); if (this.#ready) return step("ready", this.#phase);
    try {
      if (this.#phase === "native-operation-decode") {
        const decoder = this.#packed?.cursor ?? this.#children;
        if (decoder) { const current = decoder.advance(grant); const forwarded = childStep(current, grant); if (forwarded.kind === "rejected") this.#failure = decoder.failure ?? "Native patch payload failed or exceeded its grant"; else if (current.kind === "ready") this.#phase = "native-operation-capture"; return forwarded; }
        this.#phase = "native-operation-capture"; return step("pending", this.#phase, 32);
      }
      if (this.#phase === "native-operation-capture") {
        if (this.#packed) this.#capturePacked(this.#packed);
        else if (this.#children) this.#capture(this.#children.takeResult(), value => OwnedUiOperation.field(this.#id!, { field: "children", payload: value }));
        else this.#operation = this.#tag === "remove" ? OwnedUiOperation.remove(this.#id!) : OwnedUiOperation.setRoot(this.#id!);
        this.#packed?.cursor.beginClose(); this.#children?.beginClose(); this.#phase = "native-operation-retire"; return step("pending", this.#phase, 1024);
      }
      const cleanup = this.#closeInput(grant); if (cleanup) return cleanup;
      this.#ready = true; this.#phase = "native-operation-ready"; return step("ready", this.#phase, 32);
    } catch (error) { this.#failure = error instanceof Error ? error.message : "Native operation intake failed"; return step("rejected", this.#phase, 128); }
  }
  #closeInput(grant: NumericIndexGrant): RetainedUiWireStep | null {
    if (this.#payload) { this.#payloadClose = this.#payload.beginClose(); this.#payload = null; return step("pending", "native-payload-release", 64); }
    if (this.#payloadClose) { if (this.#payloadClose.terminalIsEmpty()) { this.#payloadClose = null; return step("pending", "native-payload-retirement-release", 64); } return childStep(this.#payloadClose.advance(grant), grant); }
    const decoder = this.#packed?.cursor ?? this.#children;
    if (decoder) { if (decoder.terminalIsEmpty()) { this.#packed = null; this.#children = null; return step("pending", "native-decoder-release", 64); } return childStep(decoder.closeStep(grant), grant); }
    return null;
  }
  takeResult(): OwnedUiOperation | null { if (!this.#ready || this.#closing || this.#failure) return null; const result = this.#operation; this.#operation = null; return result; }
  beginClose(): void { if (this.#closing) return; this.#closing = true; this.#packed?.cursor.beginClose(); this.#children?.beginClose(); }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "native-operation-close"); if (!this.#closing) throw new Error("Native operation close has not begun");
    try { return this.#closeStep(grant); } catch (error) { this.#failure = error instanceof Error ? error.message : "Native operation close failed"; return step("rejected", "native-operation-close-failed"); }
  }
  #closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (this.#operation) { this.#operationClose = this.#operation.beginClose(); this.#operation = null; return step("pending", "native-operation-close", 64); }
    if (this.#operationClose) { if (this.#operationClose.terminalIsEmpty()) { this.#operationClose = null; return step("pending", "native-operation-retirement-release", 64); } return childStep(this.#operationClose.advance(grant), grant); }
    return this.#closeInput(grant) ?? step("complete", "native-operation-close");
  }
  terminalIsEmpty(): boolean { return this.#closing && !this.#packed && !this.#children && !this.#payload && !this.#payloadClose && !this.#operation && !this.#operationClose; }
}
//#endregion 📥️OwnedOperationIntake

//#region 🩹️OwnedPatchStream
export type OwnedUiWirePageReceipt = { readonly ordinal: number };
/** 🩹️ One native-operation slot bound to an exact surface candidate, with separate input and publication receipts. */
export class OwnedUiWirePatchCursor {
  #patch: OwnedUiSurfacePatch | null;
  readonly #count: number;
  #next = 0;
  #input: OwnedUiWireOperationCursor | null = null;
  #operation: OwnedUiOperation | null = null;
  #operationClose: OwnedUiOperationRetirement | null = null;
  #receipt: OwnedUiWirePageReceipt | null = null;
  #phase: "input" | "decode" | "decode-result" | "input-close" | "transfer" | "apply" | "apply-receipt" | "publish" | "ready" = "input";
  #closing = false;
  #failure: string | null = null;
  constructor(surface: OwnedUiSurface, baseRevision: number, revision: number, count: number) {
    if (!Number.isSafeInteger(count) || count < 0) throw new Error("Invalid native operation count");
    this.#count = count; this.#patch = surface.beginPatch(baseRevision, revision); Object.freeze(this);
  }
  get failure(): string | null { return this.#failure; }
  offer(ordinal: number, value: unknown): boolean {
    if (this.#closing || this.#failure || this.#phase !== "input" || this.#receipt || this.#input || ordinal !== this.#next || ordinal >= this.#count) return false;
    this.#input = OwnedUiWireOperationCursor.fromNative(value); this.#phase = "decode"; return true;
  }
  finishInput(): void {
    if (this.#closing || this.#failure || this.#phase !== "input" || this.#receipt || this.#next !== this.#count) throw new Error("Native patch still owns unconsumed input obligations");
    this.#patch!.finishInput(); this.#phase = "publish";
  }
  takePageReceipt(): OwnedUiWirePageReceipt | null { const result = this.#receipt; this.#receipt = null; return result; }
  takeAcknowledgement(): OwnedUiSurfaceAcknowledgement | null { return this.#patch?.takeAcknowledgement() ?? null; }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "native-patch");
    if (this.#closing || this.#failure) return step("rejected", "native-patch");
    if (this.#phase === "input" || this.#phase === "ready") return step("ready", `native-patch-${this.#phase}`);
    try {
      if (this.#phase === "decode") {
        const current = this.#input!.advance(grant);
        const forwarded = childStep(current, grant);
        if (forwarded.kind === "rejected") this.#failure = this.#input!.failure ?? "Native operation rejected or exceeded its grant";
        else if (current.kind === "ready") this.#phase = "decode-result";
        return forwarded;
      }
      if (this.#phase === "decode-result") { this.#operation = this.#input!.takeResult(); if (!this.#operation) throw new Error("Native operation transfer is missing"); this.#input!.beginClose(); this.#phase = "input-close"; return step("pending", "native-patch-result", 128); }
      if (this.#phase === "input-close") {
        if (this.#input!.terminalIsEmpty()) { this.#input = null; this.#phase = "transfer"; return step("pending", "native-input-release", 64); }
        return childStep(this.#input!.closeStep(grant), grant);
      }
      if (this.#phase === "transfer") { this.#patch!.pushOperation(this.#operation!); this.#operation = null; this.#phase = "apply"; return step("pending", "native-patch-transfer", 128); }
      if (this.#phase === "apply-receipt") { this.#receipt = Object.freeze({ ordinal: this.#next++ }); this.#phase = "input"; return step("ready", "native-patch-receipt", 128); }
      const current = this.#patch!.advance(grant);
      const forwarded = childStep(current, grant);
      if (forwarded.kind === "rejected") { this.#failure = this.#patch!.failure ?? "Native patch rejected or exceeded its grant"; return forwarded; }
      if (current.kind === "ready") {
        if (this.#phase === "apply") { this.#phase = "apply-receipt"; return forwarded; }
        else this.#phase = "ready";
      }
      return current.kind === "ready" ? current : forwarded;
    } catch (error) { this.#failure = error instanceof Error ? error.message : "Native patch intake failed"; return step("rejected", "native-patch", 128); }
  }
  beginClose(): void { if (this.#closing) return; this.#closing = true; this.#input?.beginClose(); this.#patch?.beginClose(); }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "native-patch-close"); if (!this.#closing) throw new Error("Native patch close has not begun");
    try { return this.#closeStep(grant); } catch (error) { this.#failure = error instanceof Error ? error.message : "Native patch close failed"; return step("rejected", "native-patch-close-failed"); }
  }
  #closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (this.#receipt) return step("blocked", "native-page-receipt");
    if (this.#operation) { this.#operationClose = this.#operation.beginClose(); this.#operation = null; return step("pending", "native-operation-release", 64); }
    if (this.#operationClose) { if (this.#operationClose.terminalIsEmpty()) { this.#operationClose = null; return step("pending", "native-operation-retirement-release", 64); } return childStep(this.#operationClose.advance(grant), grant); }
    if (this.#input) { if (this.#input.terminalIsEmpty()) { this.#input = null; return step("pending", "native-input-release", 64); } return childStep(this.#input.closeStep(grant), grant); }
    if (this.#patch) { if (this.#patch.terminalIsEmpty()) { this.#patch = null; return step("pending", "native-surface-release", 64); } return childStep(this.#patch.closeStep(grant), grant); }
    return step("complete", "native-patch-close");
  }
  terminalIsEmpty(): boolean { return this.#closing && !this.#patch && !this.#input && !this.#operation && !this.#operationClose && !this.#receipt; }
}
//#endregion 🩹️OwnedPatchStream
