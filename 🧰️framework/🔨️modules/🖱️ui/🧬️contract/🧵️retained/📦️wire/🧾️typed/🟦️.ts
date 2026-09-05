//#region 🧾️TypedContract
import type * as Contract from "../../../../../🛂️manifest/🟦️.ts";
import type { NumericIndexGrant } from "../../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️.ts";
import { RetainedUiWireValueCursor, type RetainedUiWireStep } from "../🟦️.ts";
import { UiSurfaceBytes, type UiSurfaceByteRetirement } from "../🔢️bytes/🟦️.ts";
import { takeOwnedNativeBuffer } from "../🔒️transport/🟦️.ts";

export interface UiSurfaceByteView { readonly length: number; byteAt(index: number): number }
export type RetainedUiComponent = Exclude<Contract.Component, { type: "surface" }> | { readonly type: "surface"; readonly kind: Contract.UiSurfaceKind; readonly docSchema: string; readonly doc: { readonly bytes: UiSurfaceByteView }; readonly bindings: Contract.ActionBinding[] };
export type RetainedUiNodeRecord = Omit<Required<Contract.UiNodeRecord>, "component"> & { readonly component: RetainedUiComponent };
export type RetainedUiTypedValues = { component: RetainedUiComponent; node: RetainedUiNodeRecord; layout: Contract.LayoutSpec; style: Contract.StyleSpec; activity: { readonly activity: Contract.Activity; readonly disabled: boolean }; accessibility: Contract.AccessibilitySpec; bindings: Contract.ActionBinding[]; menu: Contract.MenuRef | null; children: number[] };
type Profile = keyof RetainedUiTypedValues;
type Field = "component" | "layout" | "style" | "accessibility" | "bindings" | "menu" | "children";
type NodeFields = { [K in Field]: OwnedUiPayload<RetainedUiTypedValues[K]> };
export type RetainedUiFieldChange = { [K in Field]: { readonly field: K; readonly payload: OwnedUiPayload<RetainedUiTypedValues[K]> } }[Field];
type Program<T> = Generator<number, T, void>;
type Owned = { value: object | null; next: Owned | null };
type Bytes = { value: UiSurfaceBytes | null; next: Bytes | null };
type PayloadLink = { value: OwnedUiPayload<unknown> | null; next: PayloadLink | null };
type Root<T> = { value: T | undefined; references: number; owned: Owned | null; bytes: Bytes | null; children: PayloadLink | null; fields: NodeFields | null; kind: Profile | null };
const OWNER_MINT = Object.freeze({});
const admitted = (grant: NumericIndexGrant): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= 4096;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
function isRecord(value: unknown): value is Readonly<Record<string, unknown>> { return typeof value === "object" && value !== null && !Array.isArray(value) && Object.getPrototypeOf(value) === Object.prototype; }
function text(value: unknown): string { if (typeof value !== "string") throw new Error("Expected UI text"); return value; }
function boolean(value: unknown): boolean { if (typeof value !== "boolean") throw new Error("Expected UI boolean"); return value; }
function number(value: unknown): number { if (typeof value !== "number" || !Number.isFinite(value)) throw new Error("Expected finite UI number"); return value; }
function natural(value: unknown, max = Number.MAX_SAFE_INTEGER): number { const result = number(value); if (!Number.isSafeInteger(result) || result < 0 || result > max) throw new Error("UI integer exceeds its exact domain"); return result === 0 ? 0 : result; }
function optional<T>(value: unknown, read: (value: unknown) => T): T | null { return value == null ? null : read(value); }
function choice<const T extends string>(value: unknown, choices: readonly T[]): T { for (const candidate of choices) if (value === candidate) return candidate; throw new Error("Unknown UI schema discriminator"); }
function defaulted<T>(value: unknown, fallback: T, read: (value: unknown) => T): T { return value === undefined ? fallback : read(value); }
const space = (value: unknown): Contract.SpaceToken => choice(value, ["none", "xs", "sm", "md", "lg", "xl", "xxl"]);
const activity = (value: unknown): Contract.Activity => choice(value, ["waiting", "loading", "idle", "finished"]);
//#endregion 🧾️TypedContract

//#region 📚️PayloadOwnership
class ByteView implements UiSurfaceByteView {
  readonly #source: UiSurfaceBytes;
  constructor(source: UiSurfaceBytes) { this.#source = source; Object.freeze(this); }
  get length(): number { return this.#source.length; }
  byteAt(index: number): number { return this.#source.byteAt(index); }
}

let ownPayload: <T>(root: Root<T>) => OwnedUiPayload<T>;
let retirePayload: <T>(root: Root<T>) => UiPayloadRetirement<T>;
let payloadFields: (payload: OwnedUiPayload<RetainedUiNodeRecord>) => NodeFields;
let movedPayload: <T>(payload: OwnedUiPayload<T>, kind: Profile) => OwnedUiPayload<T>;
let checkPayload: (payload: OwnedUiPayload<unknown>, kind: Profile) => void;
let checkCapture: (payload: OwnedUiPayload<unknown>, kind: Profile) => void;
let exactPayload: <T>(payload: OwnedUiPayload<T>) => T;
let nodeFields: (node: OwnedUiNode) => NodeFields;
type CaptureProbe = { readonly operation: string; readonly field: Field; readonly rejected: boolean; readonly preserved: boolean };
let saturationProbe: ((source: OwnedUiPayload<RetainedUiNodeRecord>, replacement: OwnedUiPayload<RetainedUiComponent>, activity: OwnedUiPayload<RetainedUiTypedValues["activity"]>) => readonly CaptureProbe[]) | undefined;

/** 📚️ Captured immutable typed payload; readers retain this owner, not a borrowed value alone. */
export class OwnedUiPayload<T> {
  #root: Root<T> | null;
  private constructor(mint: object, root: Root<T>) { if (mint !== OWNER_MINT) throw new Error("Typed payload requires exact mint authority"); this.#root = root; Object.freeze(this); }
  static {
    ownPayload = <V>(root: Root<V>) => new OwnedUiPayload(OWNER_MINT, root);
    payloadFields = payload => { if (payload.#root?.kind !== "node" || !payload.#root.fields) throw new Error("Expected an exact typed node field owner"); return payload.#root.fields; };
    checkPayload = (payload, kind) => { if (!payload.#root || payload.#root.kind !== kind) throw new Error("Typed UI payload field authority mismatch"); };
    checkCapture = (payload, kind) => { checkPayload(payload, kind); if (payload.#root!.references === Number.MAX_SAFE_INTEGER) throw new Error("Typed UI payload cannot be captured"); };
    exactPayload = <V>(payload: OwnedUiPayload<V>): V => { if (!payload.#root || payload.#root.value === undefined) throw new Error("Typed UI payload owner is closed"); return payload.#root.value; };
    movedPayload = <V>(payload: OwnedUiPayload<V>, kind: Profile): OwnedUiPayload<V> => { if (!payload.#root || payload.#root.kind !== kind) throw new Error("Typed UI payload field authority mismatch"); const root = payload.#root; payload.#root = null; return new OwnedUiPayload(OWNER_MINT, root); };
    if (import.meta.vitest) saturationProbe = (source, replacement, activity) => {
      const results: CaptureProbe[] = [];
      const keys: readonly Field[] = ["component", "layout", "style", "accessibility", "bindings", "menu", "children"];
      for (const operation of ["captureFrom", "replace", "withActivity"]) for (const field of keys) {
        if (operation === "replace" && field === "component") continue;
        const node = OwnedUiNode.captureFrom(source);
        const fields = nodeFields(node);
        const roots = keys.map(key => fields[key].#root!);
        const previous = roots.map(root => root.references);
        const changed = replacement.capture();
        const changedRoot = changed.#root!;
        const saturated = keys.indexOf(field);
        roots[saturated]!.references = Number.MAX_SAFE_INTEGER;
        let outcome: OwnedUiNode | undefined;
        let rejected = false;
        try { outcome = operation === "captureFrom" ? OwnedUiNode.captureFrom(source) : operation === "replace" ? node.replace({ field: "component", payload: changed }) : node.withActivity(activity); }
        catch { rejected = true; }
        const preserved = changed.#root === changedRoot && roots.every((root, index) => root.references === (index === saturated ? Number.MAX_SAFE_INTEGER : previous[index]));
        if (outcome) { roots[saturated]!.references = previous[saturated]! + (operation === "replace" && field === "component" ? 0 : 1); const retirement = outcome.beginClose(); while (!retirement.terminalIsEmpty()) retirement.advance({ maxItems: 1, maxBytes: 4096 }); }
        roots.forEach((root, index) => { root.references = previous[index]!; }); changed.#root = changedRoot;
        const changedClose = changed.beginClose(); while (!changedClose.terminalIsEmpty()) changedClose.advance({ maxItems: 1, maxBytes: 4096 });
        const nodeClose = node.beginClose(); while (!nodeClose.terminalIsEmpty()) nodeClose.advance({ maxItems: 1, maxBytes: 4096 });
        results.push({ operation, field, rejected, preserved });
      }
      return results;
    };
  }
  get value(): T { if (!this.#root || this.#root.value === undefined) throw new Error("Typed UI payload owner is closed"); return this.#root.value; }
  capture(): OwnedUiPayload<T> { if (!this.#root || this.#root.references === Number.MAX_SAFE_INTEGER) throw new Error("Typed UI payload cannot be captured"); this.#root.references++; return ownPayload(this.#root); }
  beginClose(): UiPayloadRetirement<T> { if (!this.#root) throw new Error("Typed UI payload already closed"); const root = this.#root; this.#root = null; return retirePayload(root); }
  terminalIsEmpty(): boolean { return this.#root === null; }
}

export class UiPayloadRetirement<T> {
  #root: Root<T> | null;
  #released = false;
  #bytes: UiSurfaceByteRetirement | null = null;
  #child: UiPayloadRetirement<unknown> | null = null;
  private constructor(root: Root<T>) { this.#root = root; }
  static { retirePayload = <V>(root: Root<V>) => new UiPayloadRetirement(root); }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "typed-retire");
    if (!this.#root) return step("complete", "typed-retire");
    const root = this.#root;
    if (!this.#released) { this.#released = true; if (--root.references) this.#root = null; else { root.value = undefined; root.fields = null; } return step("pending", "typed-release", 128); }
    if (root.owned) { const owned = root.owned; root.owned = owned.next; owned.value = null; owned.next = null; return step("pending", "typed-object-retire", 2112); }
    if (this.#bytes) { const result = this.#bytes.advance(grant); if (result.kind === "complete") this.#bytes = null; return { ...result, kind: "pending", phase: "typed-bytes-retire" }; }
    if (root.bytes) { const bytes = root.bytes; root.bytes = bytes.next; bytes.next = null; this.#bytes = bytes.value!.beginClose(); bytes.value = null; return step("pending", "typed-bytes-retire", 64); }
    if (this.#child) { const result = this.#child.advance(grant); if (result.kind === "complete") this.#child = null; return { ...result, kind: "pending" }; }
    if (root.children) { const child = root.children; root.children = child.next; child.next = null; this.#child = child.value!.beginClose(); child.value = null; return step("pending", "typed-field-retire", 64); }
    this.#root = null; return step("complete", "typed-retire");
  }
  terminalIsEmpty(): boolean { return this.#root === null && this.#bytes === null && this.#child === null; }
}
//#endregion 📚️PayloadOwnership

//#region 🗂️DirectNodeFields
type NodeRoot = { value: RetainedUiNodeRecord | null; fields: NodeFields | null; references: number };
let retireNode: (root: NodeRoot) => UiNodeRetirement;

function capturedChange(change: RetainedUiFieldChange | null): RetainedUiFieldChange | null {
  if (!change) return null;
  const field = change.field;
  switch (field) {
    case "component": return { field, payload: change.payload };
    case "layout": return { field, payload: change.payload };
    case "style": return { field, payload: change.payload };
    case "accessibility": return { field, payload: change.payload };
    case "bindings": return { field, payload: change.payload };
    case "menu": return { field, payload: change.payload };
    case "children": return { field, payload: change.payload };
    default: throw new Error("Unknown typed UI field change");
  }
}

/** 🎟️ Captures a field only after matching its private normalization profile. */
export function captureTypedUiPayload<P extends Profile>(kind: P, payload: OwnedUiPayload<RetainedUiTypedValues[P]>): OwnedUiPayload<RetainedUiTypedValues[P]> { checkPayload(payload, kind); return payload.capture(); }

export function captureUiFieldChange(requested: RetainedUiFieldChange): RetainedUiFieldChange {
  const change = capturedChange(requested)!;
  switch (change.field) {
    case "component": return { field: change.field, payload: captureTypedUiPayload(change.field, change.payload) };
    case "layout": return { field: change.field, payload: captureTypedUiPayload(change.field, change.payload) };
    case "style": return { field: change.field, payload: captureTypedUiPayload(change.field, change.payload) };
    case "accessibility": return { field: change.field, payload: captureTypedUiPayload(change.field, change.payload) };
    case "bindings": return { field: change.field, payload: captureTypedUiPayload(change.field, change.payload) };
    case "menu": return { field: change.field, payload: captureTypedUiPayload(change.field, change.payload) };
    case "children": return { field: change.field, payload: captureTypedUiPayload(change.field, change.payload) };
  }
}

function copyFields(fields: NodeFields, requested: RetainedUiFieldChange | null): NodeFields {
  const change = capturedChange(requested);
  if (change) checkPayload(change.payload, change.field);
  if (change?.field !== "component") checkCapture(fields.component, "component");
  if (change?.field !== "layout") checkCapture(fields.layout, "layout");
  if (change?.field !== "style") checkCapture(fields.style, "style");
  if (change?.field !== "accessibility") checkCapture(fields.accessibility, "accessibility");
  if (change?.field !== "bindings") checkCapture(fields.bindings, "bindings");
  if (change?.field !== "menu") checkCapture(fields.menu, "menu");
  if (change?.field !== "children") checkCapture(fields.children, "children");
  return {
    component: change?.field === "component" ? movedPayload(change.payload, "component") : fields.component.capture(),
    layout: change?.field === "layout" ? movedPayload(change.payload, "layout") : fields.layout.capture(),
    style: change?.field === "style" ? movedPayload(change.payload, "style") : fields.style.capture(),
    accessibility: change?.field === "accessibility" ? movedPayload(change.payload, "accessibility") : fields.accessibility.capture(),
    bindings: change?.field === "bindings" ? movedPayload(change.payload, "bindings") : fields.bindings.capture(),
    menu: change?.field === "menu" ? movedPayload(change.payload, "menu") : fields.menu.capture(),
    children: change?.field === "children" ? movedPayload(change.payload, "children") : fields.children.capture(),
  };
}

function nodeRoot(base: RetainedUiNodeRecord, fields: NodeFields, state?: RetainedUiTypedValues["activity"]): NodeRoot {
  return { references: 1, fields, value: Object.freeze({ id: base.id, key: base.key, component: fields.component.value, layout: fields.layout.value, style: fields.style.value, activity: state?.activity ?? base.activity, disabled: state?.disabled ?? base.disabled, transition: base.transition, accessibility: fields.accessibility.value, bindings: fields.bindings.value, menu: fields.menu.value, children: fields.children.value }) };
}

/** 🗂️ One immutable node owns seven direct field roots and never retains a previous node as an ancestor. */
export class OwnedUiNode {
  #root: NodeRoot | null;
  private constructor(mint: object, root: NodeRoot) { if (mint !== OWNER_MINT) throw new Error("Typed node requires exact mint authority"); this.#root = root; Object.freeze(this); }
  static { nodeFields = node => { if (!node.#root?.fields) throw new Error("Retained UI node owner is closed"); return node.#root.fields; }; }
  static captureFrom(payload: OwnedUiPayload<RetainedUiNodeRecord>): OwnedUiNode { const fields = payloadFields(payload); return new OwnedUiNode(OWNER_MINT, nodeRoot(exactPayload(payload), copyFields(fields, null))); }
  #value(): RetainedUiNodeRecord { if (!this.#root?.value) throw new Error("Retained UI node owner is closed"); return this.#root.value; }
  get value(): RetainedUiNodeRecord { return this.#value(); }
  capture(): OwnedUiNode { if (!this.#root || this.#root.references === Number.MAX_SAFE_INTEGER) throw new Error("Retained UI node cannot be captured"); this.#root.references++; return new OwnedUiNode(OWNER_MINT, this.#root); }
  captureComponent(): OwnedUiPayload<RetainedUiComponent> { return nodeFields(this).component.capture(); }
  replace(change: RetainedUiFieldChange): OwnedUiNode { const value = this.#value(); return new OwnedUiNode(OWNER_MINT, nodeRoot(value, copyFields(this.#root!.fields!, change))); }
  withActivity(payload: OwnedUiPayload<RetainedUiTypedValues["activity"]>): OwnedUiNode { checkPayload(payload, "activity"); return new OwnedUiNode(OWNER_MINT, nodeRoot(this.#value(), copyFields(this.#root!.fields!, null), exactPayload(payload))); }
  beginClose(): UiNodeRetirement { if (!this.#root) throw new Error("Retained UI node already closed"); const root = this.#root; this.#root = null; return retireNode(root); }
  terminalIsEmpty(): boolean { return this.#root === null; }
}

export class UiNodeRetirement {
  #root: NodeRoot | null;
  #child: UiPayloadRetirement<unknown> | null = null;
  #released = false;
  #index = 0;
  private constructor(root: NodeRoot) { this.#root = root; }
  static { retireNode = root => new UiNodeRetirement(root); }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "node-retire");
    if (!this.#root) return step("complete", "node-retire");
    const root = this.#root;
    if (!this.#released) { this.#released = true; if (--root.references) this.#root = null; else root.value = null; return step("pending", "node-release", 128); }
    if (this.#child) { const result = this.#child.advance(grant); if (result.kind === "complete") this.#child = null; return { ...result, kind: "pending" }; }
    const fields = root.fields!;
    const next: OwnedUiPayload<unknown> | undefined = this.#index === 0 ? fields.component : this.#index === 1 ? fields.layout : this.#index === 2 ? fields.style : this.#index === 3 ? fields.accessibility : this.#index === 4 ? fields.bindings : this.#index === 5 ? fields.menu : this.#index === 6 ? fields.children : undefined;
    if (next) { this.#index++; this.#child = next.beginClose(); return step("pending", "node-field-retire", 64); }
    root.fields = null; this.#root = null; return step("complete", "node-retire", 64);
  }
  terminalIsEmpty(): boolean { return this.#root === null && this.#child === null; }
}
//#endregion 🗂️DirectNodeFields

//#region 🏗️TypedBuilder
type JsonFrame = { index: number; count: number; parent: JsonFrame | null } & ({ input: readonly unknown[]; output: Contract.UiValue[]; keys: null } | { input: Readonly<Record<string, unknown>>; output: { [key: string]: Contract.UiValue }; keys: readonly string[] });

class Builder {
  owned: Owned | null = null;
  bytes: Bytes | null = null;
  json: JsonFrame | null = null;
  children: PayloadLink | null = null;
  fields: NodeFields | null = null;
  active: Builder | null = null;

  own<T extends object>(value: T): T { this.owned = { value, next: this.owned }; return value; }
  fixed<T extends object>(value: T): T { return this.own(Object.freeze(value)); }

  *field<K extends Field>(kind: K, value: unknown): Program<OwnedUiPayload<RetainedUiTypedValues[K]>> {
    const builder = new Builder(); this.active = builder;
    yield 128;
    const result = yield* readers[kind](builder, value);
    const payload = ownPayload<RetainedUiTypedValues[K]>({ value: result, references: 1, owned: builder.owned, bytes: builder.bytes, children: builder.children, fields: null, kind });
    builder.owned = null; builder.bytes = null; builder.children = null; this.active = null;
    this.children = { value: payload, next: this.children };
    yield 128;
    return payload;
  }

  *record(value: unknown, fields: readonly string[]): Program<Readonly<Record<string, unknown>>> {
    yield 64;
    if (!isRecord(value)) throw new Error("Expected a UI schema object");
    const keys = Object.keys(value);
    if (keys.length > 256) throw new Error("UI schema object exceeds native slots");
    yield 64 + keys.length * 8;
    for (const key of keys) { yield 512; if (!fields.includes(key)) throw new Error(`Unknown UI field: ${key}`); }
    return value;
  }

  *list<T>(value: unknown, read: (value: unknown) => Program<T>, maximum = 256): Program<T[]> {
    yield 64;
    if (!Array.isArray(value) || value.length > maximum) throw new Error("UI list exceeds native slots or is missing");
    const input: readonly unknown[] = value;
    const output: T[] = new Array(input.length);
    yield 64 + input.length * 8;
    for (let i = 0; i < input.length; i++) { const item = yield* read(input[i]); Object.defineProperty(output, i, { value: item, enumerable: true }); yield 32; }
    Object.defineProperty(output, "length", { writable: false }); Object.preventExtensions(output);
    return this.own(output);
  }

  *stringMap(value: unknown): Program<{ [key: string]: string }> {
    yield 64;
    if (!isRecord(value)) throw new Error("Expected UI text map");
    const keys = Object.keys(value);
    if (keys.length > 256) throw new Error("UI text map exceeds native slots");
    yield 64 + keys.length * 8;
    const output: { [key: string]: string } = {};
    for (const key of keys) { Object.defineProperty(output, key, { value: text(value[key]), enumerable: true }); yield 64; }
    Object.preventExtensions(output); return this.own(output);
  }

  *value(value: unknown): Program<Contract.UiValue> {
    let input = value;
    let output: Contract.UiValue | undefined;
    for (;;) {
      yield 64;
      if (input === null || typeof input === "boolean" || typeof input === "string") output = input;
      else if (typeof input === "number") output = number(input);
      else {
        if (!Array.isArray(input) && !isRecord(input)) throw new Error("Invalid native UI value");
        const source: unknown = input;
        const keys = Array.isArray(source) ? null : isRecord(source) ? Object.keys(source) : null;
        const count = Array.isArray(source) ? source.length : keys!.length;
        if (count > 256) throw new Error("UI value exceeds native slots");
        yield 64 + count * 8;
        if (Array.isArray(source)) this.json = { input: source, output: new Array<Contract.UiValue>(count), keys: null, index: 0, count, parent: this.json };
        else if (isRecord(source)) this.json = { input: source, output: {}, keys: keys!, index: 0, count, parent: this.json };
        if (count) { input = this.jsonInput(this.json!); continue; }
        output = this.finishJson();
      }
      for (;;) {
        yield 64;
        if (!this.json) return output;
        const frame = this.json;
        Object.defineProperty(frame.output, frame.keys ? frame.keys[frame.index]! : frame.index, { value: output, enumerable: true });
        frame.index++;
        if (frame.index < frame.count) { input = this.jsonInput(frame); break; }
        output = this.finishJson();
      }
    }
  }

  jsonInput(frame: JsonFrame): unknown { return frame.keys === null ? frame.input[frame.index] : frame.input[frame.keys[frame.index]!]; }
  finishJson(): Contract.UiValue { const frame = this.json!; this.json = frame.parent; frame.parent = null; if (Array.isArray(frame.output)) Object.defineProperty(frame.output, "length", { writable: false }); Object.preventExtensions(frame.output); return this.own(frame.output); }

  *binding(value: unknown): Program<Contract.ActionBinding> {
    const v = yield* this.record(value, ["trigger", "action", "args", "capability"]);
    const action = yield* this.record(v.action, ["scope", "name", "version"]);
    const address = this.fixed({ scope: text(action.scope), name: text(action.name), version: natural(action.version, 65535) });
    yield 128;
    const args = v.args == null ? null : yield* this.value(v.args);
    yield 128;
    return this.fixed({ trigger: choice(v.trigger, ["activate", "change", "commit", "delta", "drop", "submit", "abort", "repeatLast", "hoverPreview"]), action: address, args, capability: optional(v.capability, text) });
  }

  *bindings(value: unknown): Program<Contract.ActionBinding[]> { return yield* this.list(value, item => this.binding(item), 32); }
  *menu(value: unknown): Program<Contract.MenuRef | null> { yield 32; if (value === null) return null; const v = yield* this.record(value, ["id", "args"]); const args = v.args == null ? null : yield* this.value(v.args); yield 64; return this.fixed({ id: text(v.id), args }); }

  *component(value: unknown): Program<RetainedUiComponent> {
    yield 64;
    if (!isRecord(value)) throw new Error("Expected UI component");
    switch (value.type) {
      case "container": {
        const v = yield* this.record(value, ["type", "role", "label", "description", "required", "error", "defaultOpen", "dropOverlay"]);
        let dropOverlay: Contract.DropOverlaySpec | null = null;
        if (v.dropOverlay != null) { const d = yield* this.record(v.dropOverlay, ["title", "hint", "accept"]); dropOverlay = this.fixed({ title: text(d.title), hint: text(d.hint), accept: optional(d.accept, text) }); yield 128; }
        return this.fixed({ type: "container", role: defaulted(v.role, "plain", v => choice(v, ["plain", "section", "group", "field", "form", "toolbar"])), label: optional(v.label, text), description: optional(v.description, text), required: optional(v.required, boolean), error: optional(v.error, text), defaultOpen: optional(v.defaultOpen, boolean), dropOverlay });
      }
      case "text": { const v = yield* this.record(value, ["type", "value", "emphasize", "dataAttributes"]); const dataAttributes = v.dataAttributes == null ? null : yield* this.stringMap(v.dataAttributes); yield 128; return this.fixed({ type: "text", value: text(v.value), emphasize: optional(v.emphasize, boolean), dataAttributes }); }
      case "button": { const v = yield* this.record(value, ["type", "icon", "label"]); return this.fixed({ type: "button", icon: text(v.icon), label: text(v.label) }); }
      case "separator": { yield* this.record(value, ["type"]); return this.fixed({ type: "separator" }); }
      case "input": { const v = yield* this.record(value, ["type", "kind", "value", "placeholder", "commit", "min", "max", "step", "accept"]); return this.fixed({ type: "input", kind: defaulted(v.kind, "text", v => choice(v, ["text", "longText", "number", "date", "color", "file"])), value: text(v.value), placeholder: optional(v.placeholder, text), commit: optional(v.commit, text), min: optional(v.min, number), max: optional(v.max, number), step: optional(v.step, number), accept: optional(v.accept, text) }); }
      case "select": { const v = yield* this.record(value, ["type", "value", "items", "placeholder"]); const items = yield* this.list(v.items, item => this.selectItem(item)); yield 128; return this.fixed({ type: "select", value: text(v.value), items, placeholder: optional(v.placeholder, text) }); }
      case "toggle": { const v = yield* this.record(value, ["type", "on", "icon", "text"]); return this.fixed({ type: "toggle", on: boolean(v.on), icon: text(v.icon), text: optional(v.text, text) }); }
      case "keyValueList": { const v = yield* this.record(value, ["type", "entries"]); const entries = yield* this.list(v.entries, item => this.entry(item)); yield 128; return this.fixed({ type: "keyValueList", entries }); }
      case "slider": { const v = yield* this.record(value, ["type", "value", "min", "max", "step", "unit"]); return this.fixed({ type: "slider", value: number(v.value), min: number(v.min), max: number(v.max), step: number(v.step), unit: optional(v.unit, text) }); }
      case "numberStepper": { const v = yield* this.record(value, ["type", "value", "step", "uniform"]); return this.fixed({ type: "numberStepper", value: number(v.value), step: number(v.step), uniform: boolean(v.uniform) }); }
      case "ring": { const v = yield* this.record(value, ["type", "orbId", "t"]); return this.fixed({ type: "ring", orbId: text(v.orbId), t: number(v.t) }); }
      case "iconSelect": { const v = yield* this.record(value, ["type", "value", "uniform", "classifierKind"]); return this.fixed({ type: "iconSelect", value: text(v.value), uniform: boolean(v.uniform), classifierKind: text(v.classifierKind) }); }
      case "tree": { const v = yield* this.record(value, ["type", "interactionDomain"]); return this.fixed({ type: "tree", interactionDomain: optional(v.interactionDomain, text) }); }
      case "treeSection": { const v = yield* this.record(value, ["type", "label", "defaultOpen"]); return this.fixed({ type: "treeSection", label: optional(v.label, text), defaultOpen: optional(v.defaultOpen, boolean) }); }
      case "treeItem": { const v = yield* this.record(value, ["type", "label", "description", "icon", "defaultOpen", "draggable", "dragData", "dimmed", "rowActions"]); const dragData = v.dragData == null ? null : yield* this.stringMap(v.dragData); const rowActions = yield* this.list(v.rowActions === undefined ? [] : v.rowActions, item => this.rowAction(item)); yield 128; return this.fixed({ type: "treeItem", label: text(v.label), description: optional(v.description, text), icon: optional(v.icon, text), defaultOpen: optional(v.defaultOpen, boolean), draggable: optional(v.draggable, boolean), dragData, dimmed: optional(v.dimmed, boolean), rowActions }); }
      case "image": { const v = yield* this.record(value, ["type", "src", "alt"]); return this.fixed({ type: "image", src: text(v.src), alt: optional(v.alt, text) }); }
      case "surface": {
        const v = yield* this.record(value, ["type", "kind", "docSchema", "doc", "bindings"]);
        const d = yield* this.record(v.doc, ["bytes"]);
        if (!(d.bytes instanceof UiSurfaceBytes)) throw new Error("Surface document requires exact owned byte pages");
        const source = d.bytes.capture(); this.bytes = { value: source, next: this.bytes }; const bytes = this.fixed(new ByteView(source)); const doc = this.fixed({ bytes });
        yield 128;
        const bindings = yield* this.bindings(v.bindings === undefined ? [] : v.bindings);
        yield 128;
        return this.fixed({ type: "surface", kind: choice(v.kind, ["canvas-2d", "world-3d", "node-graph", "text-editor", "table", "paint-2d", "virtual-file-system", "tiled-map", "board-2d", "icon-render", "ink-canvas", "graph-timeline", "block-list", "diff-view", "event-feed"]), docSchema: text(v.docSchema), doc, bindings });
      }
      case "extension": { const v = yield* this.record(value, ["type", "extension", "props"]); const props = yield* this.value(v.props); yield 128; return this.fixed({ type: "extension", extension: text(v.extension), props }); }
      default: throw new Error("Unknown UI component type");
    }
  }

  *selectItem(value: unknown): Program<Contract.SelectItem> { const v = yield* this.record(value, ["value", "label"]); return this.fixed({ value: text(v.value), label: text(v.label) }); }
  *entry(value: unknown): Program<Contract.KeyValueEntry> { const v = yield* this.record(value, ["label", "value"]); return this.fixed({ label: text(v.label), value: text(v.value) }); }
  *rowAction(value: unknown): Program<Contract.RowAction> { const v = yield* this.record(value, ["icon", "label", "action", "placement"]); const action = yield* this.binding(v.action); yield 128; return this.fixed({ icon: text(v.icon), label: optional(v.label, text), action, placement: defaulted(v.placement, "row", v => choice(v, ["row", "menu"])) }); }

  *sizing(value: unknown): Program<Contract.Sizing> { yield 32; if (value === "hug" || value === "fill") return value; const v = yield* this.record(value, ["fixed"]); return this.fixed({ fixed: space(v.fixed) }); }
  *edges(value: unknown): Program<Contract.EdgeSpace> {
    yield 32;
    if (!isRecord(value) || Object.keys(value).length !== 1) throw new Error("Expected exactly one edge-space variant");
    if (Object.hasOwn(value, "all")) return this.fixed({ all: space(value.all) });
    if (Object.hasOwn(value, "symmetric")) { const v = yield* this.record(value.symmetric, ["vertical", "horizontal"]); return this.fixed({ symmetric: this.fixed({ vertical: space(v.vertical), horizontal: space(v.horizontal) }) }); }
    const v = yield* this.record(value.each, ["top", "right", "bottom", "left"]); return this.fixed({ each: this.fixed({ top: space(v.top), right: space(v.right), bottom: space(v.bottom), left: space(v.left) }) });
  }
  *track(value: unknown): Program<Contract.GridTrack> { yield 32; if (value === "auto" || value === "minContent" || value === "maxContent") return value; if (!isRecord(value) || Object.keys(value).length !== 1) throw new Error("Expected exactly one grid-track variant"); if (Object.hasOwn(value, "fraction")) return this.fixed({ fraction: natural(value.fraction, 255) }); const v = yield* this.record(value, ["fixed"]); return this.fixed({ fixed: space(v.fixed) }); }

  *layout(value: unknown): Program<Contract.LayoutSpec> {
    yield 64;
    if (!isRecord(value)) throw new Error("Expected UI layout");
    const align = (value: unknown): Contract.Align => choice(value, ["start", "center", "end", "stretch", "baseline"]);
    const justify = (value: unknown): Contract.Justify => choice(value, ["start", "center", "end", "spaceBetween", "spaceAround", "spaceEvenly"]);
    switch (value.kind) {
      case "leaf": { const v = yield* this.record(value, ["kind", "width", "height"]); const width = yield* this.sizing(v.width); const height = yield* this.sizing(v.height); yield 128; return this.fixed({ kind: "leaf", width, height }); }
      case "stack": { const v = yield* this.record(value, ["kind", "axis", "gap", "padding", "align", "justify", "grow", "wrap"]); const padding = yield* this.edges(v.padding); yield 128; return this.fixed({ kind: "stack", axis: choice(v.axis, ["horizontal", "vertical"]), gap: space(v.gap), padding, align: align(v.align), justify: justify(v.justify), grow: boolean(v.grow), wrap: boolean(v.wrap) }); }
      case "grid": { const v = yield* this.record(value, ["kind", "columns", "rows", "columnGap", "rowGap", "padding", "align", "justify"]); const columns = yield* this.list(v.columns, item => this.track(item), 32); const rows = yield* this.list(v.rows, item => this.track(item), 32); const padding = yield* this.edges(v.padding); yield 128; return this.fixed({ kind: "grid", columns, rows, columnGap: space(v.columnGap), rowGap: space(v.rowGap), padding, align: align(v.align), justify: justify(v.justify) }); }
      case "overlay": { const v = yield* this.record(value, ["kind", "anchor", "inset", "dismissible"]); const inset = yield* this.edges(v.inset); yield 128; return this.fixed({ kind: "overlay", anchor: choice(v.anchor, ["topStart", "top", "topEnd", "start", "center", "end", "bottomStart", "bottom", "bottomEnd"]), inset, dismissible: boolean(v.dismissible) }); }
      case "scroll": { const v = yield* this.record(value, ["kind", "axes", "padding", "sizing"]); const padding = yield* this.edges(v.padding); const sizing = yield* this.sizing(v.sizing); yield 128; return this.fixed({ kind: "scroll", axes: choice(v.axes, ["none", "horizontal", "vertical", "both"]), padding, sizing }); }
      case "absolute": { const v = yield* this.record(value, ["kind", "sizingWidth", "sizingHeight"]); const sizingWidth = yield* this.sizing(v.sizingWidth); const sizingHeight = yield* this.sizing(v.sizingHeight); yield 128; return this.fixed({ kind: "absolute", sizingWidth, sizingHeight }); }
      default: throw new Error("Unknown UI layout kind");
    }
  }

  *style(value: unknown): Program<Contract.StyleSpec> { const v = yield* this.record(value, ["variant", "size", "density", "tone", "emphasis"]); return this.fixed({ variant: defaulted(v.variant, "solid", v => choice(v, ["solid", "outline", "ghost", "plain"])), size: defaulted(v.size, "md", v => choice(v, ["xs", "sm", "md", "lg", "xl"])), density: defaulted(v.density, "standard", v => choice(v, ["compact", "standard", "touch"])), tone: defaulted(v.tone, "neutral", v => choice(v, ["neutral", "primary", "secondary", "tertiary", "info", "success", "warning", "danger"])), emphasis: defaulted(v.emphasis, "regular", v => choice(v, ["subtle", "regular", "strong"])) }); }
  *accessibility(value: unknown): Program<Contract.AccessibilitySpec> { const v = yield* this.record(value, ["label", "description", "live", "shortcut", "hidden"]); return this.fixed({ label: optional(v.label, text), description: optional(v.description, text), live: defaulted(v.live, "off", v => choice(v, ["off", "polite", "assertive"])), shortcut: optional(v.shortcut, text), hidden: defaulted(v.hidden, false, boolean) }); }
  *activity(value: unknown): Program<RetainedUiTypedValues["activity"]> { const v = yield* this.record(value, ["activity", "disabled"]); return this.fixed({ activity: activity(v.activity), disabled: boolean(v.disabled) }); }
  *childIds(value: unknown): Program<number[]> { return yield* this.list(value, function* (item) { yield 32; return natural(item); }, 128); }
  *node(value: unknown): Program<RetainedUiNodeRecord> {
    const v = yield* this.record(value, ["id", "key", "component", "layout", "style", "activity", "disabled", "transition", "accessibility", "bindings", "menu", "children"]);
    const component = yield* this.field("component", v.component); const layout = yield* this.field("layout", v.layout); const style = yield* this.field("style", v.style); const accessibility = yield* this.field("accessibility", v.accessibility);
    const bindings = yield* this.field("bindings", v.bindings === undefined ? [] : v.bindings); const menu = yield* this.field("menu", v.menu === undefined ? null : v.menu); const children = yield* this.field("children", v.children === undefined ? [] : v.children);
    this.fields = { component, layout, style, accessibility, bindings, menu, children };
    yield 128;
    return this.fixed({ id: natural(v.id), key: text(v.key), component: component.value, layout: layout.value, style: style.value, activity: activity(v.activity), disabled: defaulted(v.disabled, false, boolean), transition: optional(v.transition, v => choice(v, ["introducing", "celebrating"])), accessibility: accessibility.value, bindings: bindings.value, menu: menu.value, children: children.value });
  }
}
//#endregion 🏗️TypedBuilder

//#region 🚶️TypedCursor
const readers: { [P in Profile]: (builder: Builder, value: unknown) => Program<RetainedUiTypedValues[P]> } = {
  component: (b, v) => b.component(v), node: (b, v) => b.node(v), layout: (b, v) => b.layout(v), style: (b, v) => b.style(v), activity: (b, v) => b.activity(v), accessibility: (b, v) => b.accessibility(v), bindings: (b, v) => b.bindings(v), menu: (b, v) => b.menu(v), children: (b, v) => b.childIds(v),
};

export class RetainedUiTypedCursor<P extends Profile> {
  #decoder: RetainedUiWireValueCursor | null;
  #builder = new Builder();
  #program: Program<RetainedUiTypedValues[P]> | null = null;
  #payload: OwnedUiPayload<RetainedUiTypedValues[P]> | null = null;
  #retirement: UiPayloadRetirement<RetainedUiTypedValues[P]> | null = null;
  #closing = false;
  #failure: string | null = null;
  #phase = "typed-decode";
  readonly #profile: P;
  constructor(input: unknown, profile: P) { this.#profile = profile; this.#decoder = new RetainedUiWireValueCursor(input, profile === "node" || profile === "component" ? profile : "value"); }
  get profile(): P { return this.#profile; }
  get failure(): string | null { return this.#failure; }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", this.#phase);
    if (this.#closing || this.#failure) return step("rejected", this.#phase);
    if (this.#payload) return step("ready", this.#phase);
    try {
      if (this.#phase === "typed-decode") { const result = this.#decoder!.advance(grant); if (result.kind === "rejected") throw new Error(this.#decoder!.failure!); if (result.kind === "ready") { this.#program = readers[this.#profile](this.#builder, this.#decoder!.value); this.#phase = "typed-normalize"; } return { ...result, kind: result.kind === "ready" ? "pending" : result.kind }; }
      if (this.#phase === "typed-normalize") {
        const result = this.#program!.next();
        if (!result.done) return step("pending", this.#phase, result.value);
        this.#program = null;
        this.#payload = ownPayload({ value: result.value, references: 1, owned: this.#builder.owned, bytes: this.#builder.bytes, children: this.#builder.children, fields: this.#builder.fields, kind: this.#profile });
        this.#builder.owned = null; this.#builder.bytes = null; this.#builder.children = null; this.#builder.fields = null; this.#phase = "typed-ready";
        return step("ready", this.#phase, 128);
      }
      throw new Error("Invalid typed UI cursor phase");
    } catch (error) { this.#failure = error instanceof Error ? error.message : "Typed UI normalization failed"; return step("rejected", this.#phase, 2112); }
  }
  takeResult(): OwnedUiPayload<RetainedUiTypedValues[P]> | null { if (this.#closing || this.#failure || this.#phase !== "typed-ready") return null; const payload = this.#payload; this.#payload = null; this.#phase = "typed-taken"; return payload; }
  beginClose(): void { this.#closing = true; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "typed-close");
    if (!this.#closing) throw new Error("Begin typed UI close before advancing retirement");
    const builder = this.#builder.active ?? this.#builder;
    if (builder.json) { const frame = builder.json; builder.json = frame.parent; frame.parent = null; return step("pending", "typed-json-frame-close", 2112); }
    if (this.#program) { this.#program = null; return step("pending", "typed-program-close", 2112); }
    if (this.#payload) { this.#retirement = this.#payload.beginClose(); this.#payload = null; return step("pending", "typed-payload-close", 64); }
    if (this.#retirement) { const result = this.#retirement.advance(grant); if (result.kind === "complete") this.#retirement = null; return { ...result, kind: "pending" }; }
    if (builder.owned || builder.bytes || builder.children) { this.#retirement = retirePayload<RetainedUiTypedValues[P]>({ value: undefined, references: 1, owned: builder.owned, bytes: builder.bytes, children: builder.children, fields: null, kind: null }); builder.owned = null; builder.bytes = null; builder.children = null; builder.fields = null; return step("pending", "typed-partial-close", 128); }
    if (this.#builder.active) { this.#builder.active = null; return step("pending", "typed-field-builder-close", 64); }
    if (this.#decoder) { this.#decoder.beginClose(); const result = this.#decoder.closeStep(grant); if (result.kind === "complete") this.#decoder = null; return { ...result, kind: "pending" }; }
    return step("complete", "typed-close");
  }
  terminalIsEmpty(): boolean { return this.#closing && !this.#decoder && !this.#program && !this.#payload && !this.#retirement && !this.#builder.owned && !this.#builder.bytes && !this.#builder.json && !this.#builder.active && !this.#builder.children && !this.#builder.fields; }
}
//#endregion 🚶️TypedCursor

//#region 🔢️NativeChildField
/** 🔢️ Consumes the exact native 128-slot u64 buffer, detaching all caller aliases before scalar traversal. */
export class RetainedUiChildIdsCursor {
  #input: BigUint64Array | null;
  #output: number[] | null;
  #index = 0;
  #payload: OwnedUiPayload<number[]> | null = null;
  #retirement: UiPayloadRetirement<number[]> | null = null;
  #failure: string | null = null;
  #closing = false;
  #ready = false;
  constructor(input: unknown) {
    this.#input = new BigUint64Array(takeOwnedNativeBuffer(input, "BigUint64Array", 1024)); this.#output = new Array(this.#input.length);
    Object.freeze(this);
  }
  get failure(): string | null { return this.#failure; }
  advance(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "native-child-field");
    if (this.#closing || this.#failure) return step("rejected", "native-child-field");
    if (this.#ready) return step("ready", "native-child-field");
    try {
      if (this.#index < this.#input!.length) {
        const value = this.#input![this.#index]!;
        if (value > 9007199254740991n) throw new Error("Native child ID exceeds the exact renderer range");
        Object.defineProperty(this.#output!, this.#index++, { value: Number(value), enumerable: true }); return step("pending", "native-child-field", 64);
      }
      const output = this.#output!; Object.defineProperty(output, "length", { writable: false }); Object.preventExtensions(output);
      this.#payload = ownPayload({ value: output, references: 1, owned: { value: output, next: null }, bytes: null, children: null, fields: null, kind: "children" });
      this.#output = null; this.#input = null; this.#ready = true; return step("ready", "native-child-field", 1280);
    } catch (error) { this.#failure = error instanceof Error ? error.message : "Native child field failed"; return step("rejected", "native-child-field", 64); }
  }
  takeResult(): OwnedUiPayload<number[]> | null { if (!this.#ready || this.#closing || this.#failure) return null; const result = this.#payload; this.#payload = null; return result; }
  beginClose(): void { this.#closing = true; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant)) return step("blocked", "native-child-close"); if (!this.#closing) throw new Error("Native child close has not begun");
    if (this.#payload) { this.#retirement = this.#payload.beginClose(); this.#payload = null; return step("pending", "native-child-close", 64); }
    if (this.#retirement) { const current = this.#retirement.advance(grant); if (current.kind === "complete") this.#retirement = null; return { ...current, kind: "pending" }; }
    if (this.#input || this.#output) { this.#input = null; this.#output = null; return step("pending", "native-child-close", 2112); }
    return step("complete", "native-child-close");
  }
  terminalIsEmpty(): boolean { return this.#closing && !this.#input && !this.#output && !this.#payload && !this.#retirement; }
}
//#endregion 🔢️NativeChildField

//#region 🧪️PrivateOwnershipProbe
if (import.meta.vitest) {
  const { it, expect } = import.meta.vitest;
      const { default: fixture } = await import("../🧪️fixtures/🏷️fields/🔣️.json");

  function prepared<P extends Profile>(kind: P, value: unknown): OwnedUiPayload<RetainedUiTypedValues[P]> {
    const builder = new Builder();
    const program = readers[kind](builder, value);
    for (let i = 0; i < 100_000; i++) {
      const result = program.next();
      if (result.done) return ownPayload({ value: result.value, references: 1, owned: builder.owned, bytes: builder.bytes, children: builder.children, fields: builder.fields, kind });
    }
    throw new Error("Private ownership fixture did not terminate");
  }

  it("TypedNodeFields preflights every capture before transfer under private reference saturation", () => {
    const source = prepared("node", { ...fixture.node, component: fixture.replacement });
    const replacement = prepared("component", fixture.replacement);
    const state = prepared("activity", { activity: "loading", disabled: false });
    const outcomes = saturationProbe!(source, replacement, state);
    for (const owner of [source, replacement, state]) { const retirement = owner.beginClose(); while (!retirement.terminalIsEmpty()) retirement.advance({ maxItems: 1, maxBytes: 4096 }); }
    expect(outcomes).toHaveLength(20);
    expect(outcomes.every(row => row.rejected && row.preserved), JSON.stringify(outcomes)).toBe(true);
  });
}
//#endregion 🧪️PrivateOwnershipProbe
