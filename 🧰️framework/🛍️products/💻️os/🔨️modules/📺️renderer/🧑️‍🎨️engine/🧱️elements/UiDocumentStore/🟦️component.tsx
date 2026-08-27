/// <reference types="vitest/importMeta" />
// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/UiDocumentStore/component.tsx
/** @emoji 🗄️ `UiDocumentStore` — the per-surface retained store for the semantic UI contract
 * (`semio-framework-ui-contract`). Holds `{ revision, root, nodes: Map<UiNodeId, UiNodeRecord> }` and
 * applies a `UiPatch` **transactionally**: every op lands on a draft copy first, the draft is
 * validated against the same invariants the Rust `validate_snapshot`/`apply_patch` (`🦀️limits.rs`)
 * enforce, and only a fully-valid draft is swapped in. On ANY rejection the store is left
 * reference-identical to before — not just value-equal — so a rejected patch can never be mistaken
 * for a partial apply. This file is the TypeScript twin of that Rust algorithm: the React DOM
 * renderer, the GPU renderer, and every future renderer apply patches through logic that must agree,
 * and this is React's half of that agreement (see `🦀️limits.rs`'s own header doc for the Rust half).
 *
 * Subscriptions are per-node (`subscribeNode`/`useUiNode`, via `useSyncExternalStore`), so a
 * `SetComponent` on one node re-renders exactly the one component reading that node — never the
 * whole tree. `subscribeRoot`/`subscribeRevision` are separate channels for the rare consumer that
 * cares about tree shape or the document's own version rather than one node's content. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useCallback, useSyncExternalStore } from "react";
import { RetainedUiPatchCursor, RetainedUiSnapshotCursor, RetainedUiSurfaceOwner, type RetainedUiTransaction, type RetainedUiState } from "@semio-tech/framework";
import {
  type AccessibilitySpec,
  type ActionBinding,
  type BuiltNode,
  type Component,
  type MenuRef,
  type PatchRejection,
  type QuotaKind,
  type StyleSpec,
  type SurfaceId,
  type UiContractViolation,
  type UiDocumentLimits,
  type UiNodeId,
  type UiNodeRecord,
  type UiPatch,
  type UiPatchOp,
  type UiRevision,
  type UiSnapshot,
  type UiTrigger,
  type UiActionId,
  type UiIntent,
  type UiValue,
} from "@semio-tech/framework";
// #endregion 🔌️Adapters

//#region 🔖️Limits
/** 🛡️ Mirrors `semio-framework-ui-contract`'s `UiDocumentLimits::default()` (`🦀️limits.rs`) field for
 * field — the two MUST stay numerically identical or the two renderers would disagree about which
 * documents are valid. */
export const DEFAULT_UI_DOCUMENT_LIMITS: UiDocumentLimits = {
  maxNodes: 20_000,
  maxDepth: 128,
  maxChildren: 4_096,
  maxTextBytes: 65_536,
  maxPatchOps: 4_096,
  maxPatchBytes: 1_048_576,
};

/** 🔢️ UTF-8 byte length, matching Rust's `str::len()` — `String.prototype.length` counts UTF-16 code
 * units and would silently disagree with the Rust side on any non-ASCII text. */
function utf8ByteLength(value: string): number {
  return new TextEncoder().encode(value).length;
}

function labelBytes(label: string | null | undefined): number {
  return label ? utf8ByteLength(label) : 0;
}

/** 📝️ Mirrors `🦦️limits.rs`'s `component_text_bytes` — scope-limited to the fields most likely to
 * carry large user-authored strings, not exhaustive over every field of every variant. */
function componentTextBytes(component: Component): number {
  switch (component.type) {
    case "container":
      return labelBytes(component.label) + (component.description?.length ? utf8ByteLength(component.description) : 0) + (component.error?.length ? utf8ByteLength(component.error) : 0);
    case "text":
      return utf8ByteLength(component.value);
    case "button":
      return utf8ByteLength(component.label);
    case "input":
      return utf8ByteLength(component.value) + labelBytes(component.placeholder);
    case "select":
      return component.items.reduce((sum, item) => sum + utf8ByteLength(item.label), 0) + labelBytes(component.placeholder);
    case "toggle":
      return labelBytes(component.text);
    case "keyValueList":
      return component.entries.reduce((sum, entry) => sum + utf8ByteLength(entry.label) + utf8ByteLength(entry.value), 0);
    case "treeSection":
      return labelBytes(component.label);
    case "treeItem":
      return utf8ByteLength(component.label) + (component.description?.length ? utf8ByteLength(component.description) : 0);
    case "image":
      return labelBytes(component.alt);
    case "extension":
      return utf8ByteLength(component.extension);
    case "separator":
    case "slider":
    case "numberStepper":
    case "ring":
    case "iconSelect":
    case "tree":
    case "surface":
      return 0;
  }
}

function accessibilityTextBytes(spec: AccessibilitySpec): number {
  return labelBytes(spec.label) + labelBytes(spec.description) + (spec.shortcut ? utf8ByteLength(spec.shortcut) : 0);
}

function bindingsTextBytes(bindings: readonly ActionBinding[]): number {
  return bindings.reduce((sum, binding) => sum + utf8ByteLength(binding.action.scope) + utf8ByteLength(binding.action.name) + (binding.capability ? utf8ByteLength(binding.capability) : 0), 0);
}

function menuTextBytes(menu: MenuRef | null | undefined): number {
  return menu ? utf8ByteLength(menu.id) : 0;
}

/** 🧮️ Mirrors `🦀️limits.rs`'s `patch_byte_estimate` — a rough, dependency-free proxy for wire cost. */
export function patchByteEstimate(patch: UiPatch): number {
  const OP_OVERHEAD_BYTES = 16;
  return patch.ops.reduce((sum, op) => sum + OP_OVERHEAD_BYTES + opTextBytes(op), 0);
}

function opTextBytes(op: UiPatchOp): number {
  switch (op.type) {
    case "upsert":
      return utf8ByteLength(op.key) + componentTextBytes(op.component) + accessibilityTextBytes(op.accessibility) + bindingsTextBytes(op.bindings ?? []) + menuTextBytes(op.menu);
    case "setComponent":
      return componentTextBytes(op.component);
    case "setChildren":
      return op.children.length * 8;
    case "setAccessibility":
      return accessibilityTextBytes(op.accessibility);
    case "setBindings":
      return bindingsTextBytes(op.bindings);
    case "setMenu":
      return menuTextBytes(op.menu);
    case "setLayout":
    case "setActivity":
    case "setStyle":
    case "remove":
    case "setRoot":
      return 0;
  }
}

function isFiniteOrUndefined(value: number | null | undefined): boolean {
  return value === null || value === undefined || Number.isFinite(value);
}

/** 🔢️ Mirrors `🦀️limits.rs`'s `component_is_finite`. */
function componentIsFinite(component: Component): boolean {
  switch (component.type) {
    case "slider":
      return [component.value, component.min, component.max, component.step].every(Number.isFinite);
    case "numberStepper":
      return [component.value, component.step].every(Number.isFinite);
    case "ring":
      return Number.isFinite(component.t);
    case "input":
      return isFiniteOrUndefined(component.min) && isFiniteOrUndefined(component.max) && isFiniteOrUndefined(component.step);
    default:
      return true;
  }
}

function isSection(component: Component): boolean {
  return component.type === "container" && component.role === "section";
}
//#endregion 🔖️Limits

//#region 🔖️Validate
type WalkFrame = { readonly kind: "enter"; readonly id: UiNodeId; readonly depth: number; readonly parentInSection: boolean } | { readonly kind: "exit"; readonly id: UiNodeId };

/** 🌲️ Mirrors `🦀️limits.rs`'s `validate_core` — same iterative preorder walk (explicit stack, never
 * recursive), same violation set, same short-circuit on an already-oversized node table. Generic over
 * a plain `ReadonlyMap` so it runs unmodified against either a freshly-received `UiSnapshot` or a
 * store's own retained state. */
export function validateUiDocumentCore(root: UiNodeId | null, nodes: ReadonlyMap<UiNodeId, UiNodeRecord>, limits: UiDocumentLimits): UiContractViolation[] {
  const violations: UiContractViolation[] = [];
  if (nodes.size > limits.maxNodes) {
    violations.push({ type: "nodeQuota", count: nodes.size, max: limits.maxNodes });
    return violations;
  }

  const visited = new Set<UiNodeId>();
  const onPath = new Set<UiNodeId>();

  if (root !== null && nodes.has(root)) {
    const stack: WalkFrame[] = [{ kind: "enter", id: root, depth: 0, parentInSection: false }];
    while (stack.length > 0) {
      const frame = stack.pop()!;
      if (frame.kind === "exit") {
        onPath.delete(frame.id);
        continue;
      }
      const { id, depth, parentInSection } = frame;
      if (onPath.has(id)) {
        violations.push({ type: "cycle", node: id });
        continue;
      }
      if (visited.has(id)) continue;
      visited.add(id);
      const record = nodes.get(id);
      if (!record) continue;

      const inSection = parentInSection || isSection(record.component);
      if (parentInSection && isSection(record.component)) violations.push({ type: "sectionNested", node: id });
      if (!componentIsFinite(record.component)) violations.push({ type: "nonFiniteNumber", node: id });
      if (depth > limits.maxDepth) {
        violations.push({ type: "depthQuota", node: id, depth, max: limits.maxDepth });
        continue;
      }

      onPath.add(id);
      stack.push({ kind: "exit", id });

      const seenKeys = new Set<string>();
      for (const childId of record.children ?? []) {
        const child = nodes.get(childId);
        if (!child) {
          violations.push({ type: "orphanChild", parent: id, child: childId });
          continue;
        }
        if (seenKeys.has(child.key)) violations.push({ type: "duplicateSiblingKey", parent: id, key: child.key });
        else seenKeys.add(child.key);
        stack.push({ kind: "enter", id: childId, depth: depth + 1, parentInSection: inSection });
      }
    }
  }

  for (const id of nodes.keys()) {
    if (!visited.has(id)) violations.push({ type: "danglingRoot", node: id });
  }
  return violations;
}
//#endregion 🔖️Validate

//#region 🔖️Apply
export type UiDocumentState = {
  readonly surface: SurfaceId;
  readonly revision: UiRevision;
  readonly root: UiNodeId | null;
  readonly nodes: ReadonlyMap<UiNodeId, UiNodeRecord>;
};

/** 🌱️ An empty document for `surface`, at revision zero with no root yet — mirrors
 * `crate::UiSnapshotState::new`. */
export function emptyUiDocumentState(surface: SurfaceId): UiDocumentState {
  return { surface, revision: 0, root: null, nodes: new Map() };
}

/** 📸️ Builds a retained state directly from a full `UiSnapshot` — the "whole-body replace" every
 * consumer uses to hydrate or resynchronize a surface, mirroring `From<UiSnapshot> for
 * UiSnapshotState` exactly, including that this does NOT validate (see `UiDocumentStore.loadSnapshot`'s
 * own doc for why that is the deliberately-correct behavior, not an oversight). A caller with a
 * genuinely untrusted whole snapshot in hand may still run {@link validateUiDocumentCore} itself. */
export function uiDocumentStateFromSnapshot(snapshot: UiSnapshot): UiDocumentState {
  const nodes = new Map<UiNodeId, UiNodeRecord>();
  for (const record of snapshot.nodes) nodes.set(record.id, record);
  return { surface: snapshot.surface, revision: snapshot.revision, root: snapshot.root, nodes };
}

function rejectQuota(quota: QuotaKind, actual: number, max: number): PatchRejection {
  return { type: "quotaExceeded", quota, actual, max };
}

/** 🩹️ Applies one op to `draft` (a fresh `Map`, safe to mutate in place — the caller already cloned
 * it), mirroring `🦀️limits.rs`'s `apply_op`. Returns a rejection instead of throwing so the caller can
 * bail out of the whole patch without having mutated the CALLER's real state (only `draft`, which is
 * discarded on rejection). */
function applyOp(draft: { root: UiNodeId | null; nodes: Map<UiNodeId, UiNodeRecord> }, op: UiPatchOp, limits: UiDocumentLimits): PatchRejection | null {
  const mutate = (id: UiNodeId): UiNodeRecord | PatchRejection => {
    const record = draft.nodes.get(id);
    return record ?? { type: "unknownNode", id };
  };

  switch (op.type) {
    case "upsert": {
      if ((op.children?.length ?? 0) > limits.maxChildren) return rejectQuota("children", op.children!.length, limits.maxChildren);
      const bytes = componentTextBytes(op.component);
      if (bytes > limits.maxTextBytes) return rejectQuota("textBytes", bytes, limits.maxTextBytes);
      const { type: _discardTag, ...record } = op;
      draft.nodes.set(op.id, record as UiNodeRecord);
      return null;
    }
    case "setComponent": {
      const bytes = componentTextBytes(op.component);
      if (bytes > limits.maxTextBytes) return rejectQuota("textBytes", bytes, limits.maxTextBytes);
      const record = mutate(op.id);
      if ("type" in record && record.type === "unknownNode") return record;
      draft.nodes.set(op.id, { ...(record as UiNodeRecord), component: op.component });
      return null;
    }
    case "setLayout": {
      const record = mutate(op.id);
      if ("type" in record && record.type === "unknownNode") return record;
      draft.nodes.set(op.id, { ...(record as UiNodeRecord), layout: op.layout });
      return null;
    }
    case "setActivity": {
      const record = mutate(op.id);
      if ("type" in record && record.type === "unknownNode") return record;
      draft.nodes.set(op.id, { ...(record as UiNodeRecord), activity: op.activity, disabled: op.disabled });
      return null;
    }
    case "setChildren": {
      if (op.children.length > limits.maxChildren) return rejectQuota("children", op.children.length, limits.maxChildren);
      const record = mutate(op.id);
      if ("type" in record && record.type === "unknownNode") return record;
      draft.nodes.set(op.id, { ...(record as UiNodeRecord), children: op.children });
      return null;
    }
    case "setStyle": {
      const record = mutate(op.id);
      if ("type" in record && record.type === "unknownNode") return record;
      draft.nodes.set(op.id, { ...(record as UiNodeRecord), style: op.style });
      return null;
    }
    case "setAccessibility": {
      const record = mutate(op.id);
      if ("type" in record && record.type === "unknownNode") return record;
      draft.nodes.set(op.id, { ...(record as UiNodeRecord), accessibility: op.accessibility });
      return null;
    }
    case "setBindings": {
      const record = mutate(op.id);
      if ("type" in record && record.type === "unknownNode") return record;
      draft.nodes.set(op.id, { ...(record as UiNodeRecord), bindings: op.bindings });
      return null;
    }
    case "setMenu": {
      const record = mutate(op.id);
      if ("type" in record && record.type === "unknownNode") return record;
      draft.nodes.set(op.id, { ...(record as UiNodeRecord), menu: op.menu });
      return null;
    }
    case "remove": {
      const stack: UiNodeId[] = [op.id];
      while (stack.length > 0) {
        const current = stack.pop()!;
        const record = draft.nodes.get(current);
        if (record) {
          draft.nodes.delete(current);
          stack.push(...(record.children ?? []));
        }
      }
      return null;
    }
    case "setRoot":
      draft.root = op.id;
      return null;
  }
}

/** 🩹️ Mirrors `🦀️limits.rs`'s `apply_patch` — totally transactional. `base_revision` must equal
 * `state.revision`; every op then applies to a draft `Map` CLONED from `state.nodes` up front, never
 * to `state` itself; the draft is validated via {@link validateUiDocumentCore}; only on success does
 * the caller's state swap to the draft. On any rejection path this function returns `{ok:false}` and
 * never mutates `state` — the returned object on success is a NEW `UiDocumentState`, so a caller that
 * discards it on rejection has touched nothing. */
export function applyUiPatch(state: UiDocumentState, patch: UiPatch, limits: UiDocumentLimits = DEFAULT_UI_DOCUMENT_LIMITS): { readonly ok: true; readonly state: UiDocumentState } | { readonly ok: false; readonly rejection: PatchRejection } {
  if (patch.baseRevision !== state.revision) return { ok: false, rejection: { type: "revisionMismatch", expected: state.revision, actual: patch.baseRevision } };
  if (patch.ops.length > limits.maxPatchOps) return { ok: false, rejection: rejectQuota("patchOps", patch.ops.length, limits.maxPatchOps) };
  const estimatedBytes = patchByteEstimate(patch);
  if (estimatedBytes > limits.maxPatchBytes) return { ok: false, rejection: rejectQuota("patchBytes", estimatedBytes, limits.maxPatchBytes) };

  const draft = { root: state.root, nodes: new Map(state.nodes) };
  for (const op of patch.ops) {
    const rejection = applyOp(draft, op, limits);
    if (rejection) return { ok: false, rejection };
  }

  const violations = validateUiDocumentCore(draft.root, draft.nodes, limits);
  if (violations.length > 0) return { ok: false, rejection: { type: "invariantViolated", violations } };

  return { ok: true, state: { surface: state.surface, revision: patch.revision, root: draft.root, nodes: draft.nodes } };
}
//#endregion 🔖️Apply

//#region 🔖️Reconcile
/** @emoji 🧬️ Mints one flat retained snapshot from an authored {@link BuiltNode} tree. Node ids are DFS-local to this full-body reconciliation; patch-time transition hints intentionally start empty. */
export function builtNodeToSnapshot(surface: SurfaceId, root: BuiltNode, revision: UiRevision = 0, layoutEpoch: bigint = 0n): UiSnapshot {
  const nodes: UiNodeRecord[] = [];
  let nextId = 1;
  const mint = (node: BuiltNode): UiNodeId => {
    const id = nextId++;
    const children = node.children.map(mint);
    nodes.push({
      id,
      key: node.key,
      component: node.component,
      layout: node.layout,
      style: node.style,
      activity: node.activity,
      disabled: node.disabled,
      transition: null,
      accessibility: node.accessibility,
      bindings: node.bindings,
      menu: node.menu,
      children,
    });
    return id;
  };
  const rootId = mint(root);
  return { surface, revision, root: rootId, nodes, layoutEpoch };
}
//#endregion 🔖️Reconcile

//#region 🔖️Store
type Listener = () => void;

/** 🗄️ A per-surface retained store — see this file's header doc for the transactional/subscription
 * contract. One instance per live surface; the caller (typically `PluginRuntime`) owns its lifetime. */
export class UiDocumentStore {
  private state: UiDocumentState;
  private readonly limits: UiDocumentLimits;
  private readonly nodeListeners = new Map<UiNodeId, Set<Listener>>();
  private readonly rootListeners = new Set<Listener>();
  private readonly revisionListeners = new Set<Listener>();
  private seq = 0n;

  constructor(surface: SurfaceId, limits: UiDocumentLimits = DEFAULT_UI_DOCUMENT_LIMITS) {
    this.state = emptyUiDocumentState(surface);
    this.limits = limits;
  }

  getState(): UiDocumentState {
    return this.state;
  }

  /** 📸️ Hydrates/resynchronizes the whole store from a freshly-received `UiSnapshot` — mirrors
   * `crate::UiSnapshotState`'s `From<UiSnapshot>` EXACTLY, including that conversion's own lack of
   * validation: this crate's one validated entry point is `apply_patch`'s draft-then-validate flow,
   * never a bare snapshot conversion (a `UiSnapshot` only ever originates from this contract's own
   * authoritative runtime, which built it through validated patches in the first place — a `UiPatch`
   * is the untrusted-input boundary this contract defends, not a whole-body replace). Confirmed
   * against the shared conformance corpus: every `🚫️rejection` fixture loads its `.snapshot.json`
   * unchecked and only asserts the FOLLOWING `.patch.json` is rejected. */
  loadSnapshot(snapshot: UiSnapshot): void {
    const previous = this.state;
    this.state = uiDocumentStateFromSnapshot(snapshot);
    this.notifyDiff(previous, this.state);
  }

  /** 🩹️ Applies `patch` transactionally via {@link applyUiPatch}. On success, notifies exactly the
   * per-node listeners whose record reference actually changed (never the whole tree), plus the
   * root/revision channels when those moved. On rejection, notifies nobody — the store is
   * reference-identical to before. */
  applyPatch(patch: UiPatch): { readonly ok: true } | { readonly ok: false; readonly rejection: PatchRejection } {
    const result = applyUiPatch(this.state, patch, this.limits);
    if (!result.ok) return result;
    const previous = this.state;
    this.state = result.state;
    this.notifyDiff(previous, this.state);
    return { ok: true };
  }

  private notifyDiff(previous: UiDocumentState, next: UiDocumentState): void {
    if (previous.revision !== next.revision) for (const listener of this.revisionListeners) listener();
    if (previous.root !== next.root) for (const listener of this.rootListeners) listener();
    const touched = new Set<UiNodeId>();
    for (const [id, record] of next.nodes) if (previous.nodes.get(id) !== record) touched.add(id);
    for (const id of previous.nodes.keys()) if (!next.nodes.has(id)) touched.add(id);
    for (const id of touched) {
      const listeners = this.nodeListeners.get(id);
      if (listeners) for (const listener of listeners) listener();
    }
  }

  subscribeNode(id: UiNodeId): (onStoreChange: Listener) => () => void {
    return (onStoreChange: Listener) => {
      let listeners = this.nodeListeners.get(id);
      if (!listeners) {
        listeners = new Set();
        this.nodeListeners.set(id, listeners);
      }
      listeners.add(onStoreChange);
      return () => {
        listeners!.delete(onStoreChange);
        if (listeners!.size === 0) this.nodeListeners.delete(id);
      };
    };
  }

  getNodeSnapshot = (id: UiNodeId): UiNodeRecord | undefined => this.state.nodes.get(id);

  subscribeRoot = (onStoreChange: Listener): (() => void) => {
    this.rootListeners.add(onStoreChange);
    return () => this.rootListeners.delete(onStoreChange);
  };

  getRootSnapshot = (): UiNodeId | null => this.state.root;

  subscribeRevision = (onStoreChange: Listener): (() => void) => {
    this.revisionListeners.add(onStoreChange);
    return () => this.revisionListeners.delete(onStoreChange);
  };

  getRevisionSnapshot = (): UiRevision => this.state.revision;

  /** 🎬️ Builds a `UiIntent` carrying this store's OWN current `revision` and a per-surface monotonic
   * `seq` — the two fields that let a host recognise and drop a stale interaction instead of
   * misapplying it against geometry the user never actually saw. Replaces the old per-control
   * `dispatch(controllerId, action, args)` plumbing: one helper, called once per user gesture. */
  buildIntent(record: UiNodeRecord, binding: ActionBinding, input?: UiValue): UiIntent {
    this.seq += 1n;
    return {
      surface: this.state.surface,
      revision: this.state.revision,
      node: record.id,
      nodeKey: record.key,
      trigger: binding.trigger,
      action: binding.action,
      args: binding.args,
      input: input ?? null,
      seq: this.seq,
    };
  }
}

/** 🎬️ Finds `record`'s own binding for `trigger` and builds the `UiIntent`, or `undefined` when the
 * node declares no binding for that lifecycle moment — the one place a component decides "do I even
 * have an action to fire here" before asking the store to mint the intent. */
export function emitIntent(store: UiDocumentStore, record: UiNodeRecord, trigger: UiTrigger, input?: UiValue): UiIntent | undefined {
  const binding = (record.bindings ?? []).find((candidate) => candidate.trigger === trigger);
  if (!binding) return undefined;
  return store.buildIntent(record, binding, input);
}

/** 🔎️ The `UiActionId` a node binds for `trigger`, or `undefined` — used by the conformance suite to
 * assert reachable action ids without duplicating {@link emitIntent}'s lookup. */
export function actionIdForTrigger(record: UiNodeRecord, trigger: UiTrigger): UiActionId | undefined {
  return (record.bindings ?? []).find((candidate) => candidate.trigger === trigger)?.action;
}
//#endregion 🔖️Store

//#region 🔖️Hooks
/** 🪝️ Subscribes to exactly node `id` — re-renders only when THAT node's record reference changes. */
export function useUiNode(store: UiDocumentStore, id: UiNodeId): UiNodeRecord | undefined {
  const subscribe = useCallback((onStoreChange: () => void) => store.subscribeNode(id)(onStoreChange), [store, id]);
  return useSyncExternalStore(subscribe, () => store.getNodeSnapshot(id));
}

export function useUiDocumentRoot(store: UiDocumentStore): UiNodeId | null {
  return useSyncExternalStore(store.subscribeRoot, store.getRootSnapshot);
}

export function useUiDocumentRevision(store: UiDocumentStore): UiRevision {
  return useSyncExternalStore(store.subscribeRevision, store.getRevisionSnapshot);
}
//#endregion 🔖️Hooks

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it, vi } = import.meta.vitest;
  const { default: retainedFixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️patch.json");
  const { default: retainedSchema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️patch.schema.json");
  const { default: Ajv } = await import("ajv");
  const { produce } = await import("immer");
  const { RetainedUiWireValueCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🟦️component.ts");
  const { default: wireFixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️decode.json");
  const { default: wireSchema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️decode.schema.json");

  //#region 🧾️TypedWireTests
  describe("TypedWire", () => {
    const grant = { maxItems: 1, maxBytes: 4096 };
    function finish(cursor: { advance: (budget: typeof grant) => { kind: string; items: number; bytes: number }; failure: string | null }): string {
      for (let i = 0; i < 500_000; i++) { const step = cursor.advance(grant); expect(step.items).toBeLessThanOrEqual(1); expect(step.bytes).toBeLessThanOrEqual(4096); if (step.kind === "ready" || step.kind === "rejected") return step.kind; }
      throw new Error("Typed UI preparation did not terminate");
    }
    function close(cursor: { beginClose: () => void; closeStep: (budget: typeof grant) => { kind: string; items: number; bytes: number }; terminalIsEmpty: () => boolean }): void {
      cursor.beginClose();
      for (let i = 0; i < 500_000; i++) { const step = cursor.closeStep(grant); expect(step.items).toBeLessThanOrEqual(1); expect(step.bytes).toBeLessThanOrEqual(4096); if (step.kind === "complete") { expect(cursor.terminalIsEmpty()).toBe(true); return; } }
      throw new Error("Typed UI close did not terminate");
    }
    function retire(cursor: { advance: (budget: typeof grant) => { kind: string; items: number; bytes: number }; terminalIsEmpty: () => boolean }): void {
      for (let i = 0; i < 500_000; i++) { const step = cursor.advance(grant); expect(step.items).toBeLessThanOrEqual(1); expect(step.bytes).toBeLessThanOrEqual(4096); if (step.kind === "complete") { expect(cursor.terminalIsEmpty()).toBe(true); return; } }
      throw new Error("Typed UI payload retirement did not terminate");
    }

    it("normalizes all native component variants and defaults with strict neutral and Immer oracles", async () => {
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️typed.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️typed.schema.json");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true);
      for (const vector of fixture.components) {
        const cursor = new RetainedUiTypedCursor(encodePackValue(vector.wire), "component");
        expect(cursor.takeResult()).toBeNull(); expect(finish(cursor), cursor.failure ?? vector.wire.type).toBe("ready");
        const payload = cursor.takeResult()!; close(cursor);
        const value = payload.value;
        const projected = value.type === "surface" ? { ...value, doc: { bytes: Array.from({ length: value.doc.bytes.length }, (_, i) => value.doc.bytes.byteAt(i)) } } : value;
        expect(projected).toEqual(JSON.parse(JSON.stringify(produce(vector.expected, () => {}))));
        expect(Object.isFrozen(value)).toBe(true); expect(Reflect.set(value, "type", "forged")).toBe(false);
        if (value.type === "surface") expect("beginClose" in value.doc.bytes).toBe(false);
        retire(payload.beginClose());
      }
      const style = new RetainedUiTypedCursor(encodePackValue({}), "style"); expect(finish(style)).toBe("ready"); const styled = style.takeResult()!; expect(styled.value).toEqual(fixture.defaults.style); close(style); retire(styled.beginClose());
      const accessibility = new RetainedUiTypedCursor(encodePackValue({}), "accessibility"); expect(finish(accessibility)).toBe("ready"); const accessible = accessibility.takeResult()!; expect(accessible.value).toEqual(fixture.defaults.accessibility); close(accessibility); retire(accessible.beginClose());
      for (const vector of fixture.hostile) { const cursor = new RetainedUiTypedCursor(encodePackValue(vector), "component"); expect(finish(cursor)).toBe("rejected"); expect(cursor.takeResult()).toBeNull(); close(cursor); }
    });

    it("validates real upsert and set-component entry roots, safe IDs, and every layout variant", async () => {
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const layouts = [
        { kind: "leaf", width: "hug", height: "fill" },
        { kind: "stack", axis: "vertical", gap: "sm", padding: { all: "none" }, align: "stretch", justify: "start", grow: false, wrap: true },
        { kind: "grid", columns: [{ fraction: 2 }, "auto"], rows: ["maxContent"], columnGap: "xs", rowGap: "md", padding: { symmetric: { vertical: "xs", horizontal: "sm" } }, align: "center", justify: "spaceBetween" },
        { kind: "overlay", anchor: "bottomEnd", inset: { each: { top: "none", right: "xs", bottom: "sm", left: "md" } }, dismissible: true },
        { kind: "scroll", axes: "both", padding: { all: "none" }, sizing: { fixed: "xl" } },
        { kind: "absolute", sizingWidth: "fill", sizingHeight: "hug" },
      ];
      for (const layout of layouts) { const cursor = new RetainedUiTypedCursor(encodePackValue(layout), "layout"); expect(finish(cursor), cursor.failure ?? "").toBe("ready"); const owner = cursor.takeResult()!; expect(owner.value).toEqual(layout); close(cursor); retire(owner.beginClose()); }
      const wire = { tag: "upsert", val: { node: encodePackValue({ id: Number.MAX_SAFE_INTEGER, key: "surface", component: { type: "surface", kind: "node-graph", docSchema: "node-graph@1", doc: { bytes: [5, 7, 9] } }, layout: layouts[0], style: {}, activity: "idle", accessibility: {} }) } };
      const node = new RetainedUiTypedCursor(wire.val.node, "node"); expect(finish(node), node.failure ?? "").toBe("ready"); const owner = node.takeResult()!; close(node);
      expect(owner.value.id).toBe(Number.MAX_SAFE_INTEGER); expect(owner.value.disabled).toBe(false); expect(owner.value.children).toEqual([]); expect(owner.value.transition).toBeNull();
      expect(owner.value.component.type).toBe("surface"); if (owner.value.component.type === "surface") expect(owner.value.component.doc.bytes.byteAt(2)).toBe(9);
      retire(owner.beginClose());
      for (const malformed of [{ ...layouts[0], width: "100px" }, { ...layouts[2], columns: [{ fraction: 256 }] }, { ...layouts[0], width: { fixed: "xs", stray: true } }]) { const cursor = new RetainedUiTypedCursor(encodePackValue(malformed), "layout"); expect(finish(cursor)).toBe("rejected"); close(cursor); }
    });

    it("captures exact surface bytes before decoder close and retains older snapshots until their own final retirement", async () => {
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const expected = Buffer.alloc(32768); for (let i = 0; i < expected.length; i++) expected[i] = (i * 17 + 3) & 255;
      const wire = { tag: "set-component", val: { node: 7, component: encodePackValue({ type: "surface", kind: "node-graph", docSchema: "node-graph@1", doc: { bytes: Array.from(expected) } }) } };
      const cursor = new RetainedUiTypedCursor(wire.val.component, "component"); expect(finish(cursor), cursor.failure ?? "").toBe("ready"); const published = cursor.takeResult()!; const oldReader = published.capture(); const concurrentReader = published.capture(); close(cursor);
      const value = oldReader.value; if (value.type !== "surface") throw new Error("Expected retained surface"); const borrowed = value.doc.bytes;
      const publicationRetirement = published.beginClose(); expect(publicationRetirement.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(borrowed.byteAt(32767)).toBe(expected[32767]); retire(publicationRetirement);
      retire(concurrentReader.beginClose()); expect(Buffer.from(Array.from({ length: borrowed.length }, (_, i) => borrowed.byteAt(i)))).toEqual(expected);
      const cancelled = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "node-graph", docSchema: "node-graph@1", doc: { bytes: [4] } }), "component"); expect(finish(cancelled)).toBe("ready"); close(cancelled); expect(borrowed.byteAt(0)).toBe(3);
      retire(oldReader.beginClose()); expect(() => borrowed.byteAt(0)).toThrow(); expect(oldReader.terminalIsEmpty()).toBe(true);
    });

    it("TypedNodeFields retains seven direct fields without ancestor chains across repeated replacement", async () => {
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.schema.json");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true);
      const input = new RetainedUiTypedCursor(encodePackValue(fixture.node), "node"); expect(finish(input)).toBe("ready"); const payload = input.takeResult()!;
      expect(() => Object.defineProperty(payload, "value", { value: { ...payload.value, id: 1 } })).toThrow();
      let current = OwnedUiNode.captureFrom(payload); const oldReader = current.capture(); close(input); retire(payload.beginClose());
      expect(() => Object.defineProperty(current, "value", { value: { ...current.value, id: 1 } })).toThrow();
      const old = oldReader.value; if (old.component.type !== "surface") throw new Error("Expected old surface"); const oldBytes = old.component.doc.bytes;
      const replacement = new RetainedUiTypedCursor(encodePackValue(fixture.replacement), "component"); expect(finish(replacement)).toBe("ready"); const reusable = replacement.takeResult()!; close(replacement);
      for (let i = 0; i < fixture.repeat; i++) {
        const moved = reusable.capture(); const next = current.replace({ field: "component", payload: moved }); expect(moved.terminalIsEmpty()).toBe(true); retire(current.beginClose()); current = next;
        expect(current.value.layout).toBe(old.layout); expect(current.value.accessibility).toBe(old.accessibility); expect(current.value.children).toBe(old.children); expect(current.value.component).toBe(reusable.value);
      }
      expect(Array.from({ length: oldBytes.length }, (_, i) => oldBytes.byteAt(i))).toEqual(fixture.expected.oldBytesAfterReplacement);
      expect(current.value.id).toBe(fixture.expected.id); expect(current.value.component).toEqual(produce(fixture.expected.component, () => {}));
      retire(oldReader.beginClose()); expect(() => oldBytes.byteAt(0)).toThrow(); expect(current.value.layout).toBe(old.layout);
      retire(current.beginClose()); retire(reusable.beginClose());
    });

    it("TypedNodeFields rejects rebound field authority before moving it and cancels without releasing current", async () => {
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const input = new RetainedUiTypedCursor(encodePackValue(fixture.node), "node"); expect(finish(input)).toBe("ready"); const payload = input.takeResult()!; const current = OwnedUiNode.captureFrom(payload); close(input); retire(payload.beginClose());
      const style = new RetainedUiTypedCursor(encodePackValue({}), "style"); expect(finish(style)).toBe("ready"); const wrong = style.takeResult()!; close(style);
      expect(() => Reflect.apply(current.replace, current, [{ field: "component", payload: wrong }])).toThrow(); expect(wrong.terminalIsEmpty()).toBe(false);
      const changed = current.replace({ field: "style", payload: wrong }); const discard = changed.beginClose(); expect(discard.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); retire(discard);
      const value = current.value; if (value.component.type !== "surface") throw new Error("Expected unchanged surface"); expect(value.component.doc.bytes.byteAt(0)).toBe(3); retire(current.beginClose());
    });

    it("TypedNodeFields closes each partial field builder and rejects forged node projection owners", async () => {
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const reference = new RetainedUiTypedCursor(encodePackValue(fixture.node), "node"); let count = 0;
      for (let n = 0; n < 10000; n++) { const current = reference.advance(grant); if (current.phase === "typed-normalize") count++; if (current.kind === "ready") break; }
      close(reference); expect(count).toBeGreaterThan(50);
      for (let cutoff = 0; cutoff <= count; cutoff++) {
        const cursor = new RetainedUiTypedCursor(encodePackValue(fixture.node), "node"); let observed = 0;
        for (let n = 0; n < 10000; n++) { const current = cursor.advance(grant); if (current.phase === "typed-normalize" && observed++ === cutoff) break; if (current.kind === "ready") break; }
        close(cursor);
      }
      const wrong = new RetainedUiTypedCursor(encodePackValue(fixture.node.component), "component"); expect(finish(wrong)).toBe("ready"); const owner = wrong.takeResult()!; close(wrong);
      expect(() => Reflect.apply(OwnedUiNode.captureFrom, OwnedUiNode, [owner])).toThrow(); expect(owner.terminalIsEmpty()).toBe(false); retire(owner.beginClose());
    });

    it("OwnedNodeIndex captures exact readers across removal and closes final surface owners in bounded steps", async () => {
      const { OwnedUiNodeIndex } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🗂️nodes/🟦️component.ts");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-nodes.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-nodes.schema.json");
      const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true);
      const node = (id: number) => { const cursor = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, id }), "node"); expect(finish(cursor)).toBe("ready"); const payload = cursor.takeResult()!; const result = OwnedUiNode.captureFrom(payload); close(cursor); retire(payload.beginClose()); return result; };
      let index = OwnedUiNodeIndex.empty();
      const oracle = new Map<number, true>();
      for (const id of fixture.ids) { const input = node(id); const edit = index.beginSet(input); retire(input.beginClose()); expect(finish(edit)).toBe("ready"); const next = edit.takeResult()!; retire(edit.beginClose()); retire(index.beginClose()); index = next; oracle.set(id, true); }
      const old = index.capture(); const lookup = old.beginLookup(fixture.removed);
      let held: ReturnType<typeof OwnedUiNode.captureFrom> | null = null;
      for (;;) { const result = lookup.advance(grant); if (result.kind === "value") held = result.value; if (result.kind === "complete") break; }
      retire(lookup.beginClose()); if (!held || held.value.component.type !== "surface") throw new Error("Expected captured surface node"); const bytes = held.value.component.doc.bytes;
      const removal = index.beginRemove(fixture.removed); expect(finish(removal)).toBe("ready"); const removed = removal.takeResult()!; retire(removal.beginClose()); retire(index.beginClose()); index = removed; oracle.delete(fixture.removed);
      const input = node(fixture.removed); const insertion = index.beginSet(input); retire(input.beginClose()); expect(finish(insertion)).toBe("ready"); const reinserted = insertion.takeResult()!; retire(insertion.beginClose()); retire(index.beginClose()); index = reinserted; oracle.set(fixture.removed, true);
      const order: number[] = []; const reader = index.beginRead();
      for (;;) { const result = reader.advance(grant); expect(result.items).toBeLessThanOrEqual(1); expect(result.bytes).toBeLessThanOrEqual(4096); if (result.kind === "value") { order.push(result.value.value.id); retire(result.value.beginClose()); } if (result.kind === "complete") break; }
      retire(reader.beginClose()); expect(order).toEqual(produce(fixture.expectedReinsertedOrder, () => {})); expect(order).toEqual(Array.from(oracle.keys()));
      retire(index.beginClose()); const closing = old.beginClose(); for (const budget of fixture.grants.slice(0, 2)) expect(closing.advance(budget).kind).toBe("blocked"); retire(closing);
      expect(Array.from({ length: bytes.length }, (_, i) => bytes.byteAt(i))).toEqual(fixture.oldBytes); retire(held.beginClose()); expect(() => bytes.byteAt(0)).toThrow();
      const rejected = node(17); expect(() => index.beginSet(rejected)).toThrow(); expect(rejected.terminalIsEmpty()).toBe(false); retire(rejected.beginClose());
    });

    it("OwnedNodeIndex cancels every persistent edit phase without closing a concurrently captured source", async () => {
      const { OwnedUiNodeIndex } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🗂️nodes/🟦️component.ts");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const cursor = new RetainedUiTypedCursor(encodePackValue(fields.node), "node"); expect(finish(cursor)).toBe("ready"); const payload = cursor.takeResult()!; const input = OwnedUiNode.captureFrom(payload); close(cursor); retire(payload.beginClose());
      const source = OwnedUiNodeIndex.empty(); const reference = source.beginSet(input); let count = 0;
      for (;;) { count++; if (reference.advance(grant).kind === "ready") break; } retire(reference.beginClose());
      for (let cutoff = 0; cutoff <= count; cutoff++) { const snapshot = source.capture(); const edit = source.beginSet(input); for (let i = 0; i < cutoff; i++) edit.advance(grant); const closing = edit.beginClose(); expect(edit.takeResult()).toBeNull(); expect(closing.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); retire(closing); expect(snapshot.size).toBe(0); retire(snapshot.beginClose()); if (input.value.component.type !== "surface") throw new Error("Expected retained original"); expect(input.value.component.doc.bytes.byteAt(0)).toBe(3); }
      retire(source.beginClose()); retire(input.beginClose());
    });

    it("OwnedOperation applies typed field owners and removes subtrees while preserving concurrent source snapshots", async () => {
      const { OwnedUiOperation, OwnedUiOperationCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️component.ts");
      const { OwnedUiNodeIndex } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🗂️nodes/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-operations.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-operations.schema.json");
      const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true);
      let source = OwnedUiNodeIndex.empty();
      for (const id of [fixture.parent, fixture.child]) {
        const input = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, id, children: id === fixture.parent ? [fixture.child] : [] }), "node"); expect(finish(input)).toBe("ready"); const payload = input.takeResult()!; const op = OwnedUiOperation.upsert(payload); retire(payload.beginClose()); close(input);
        const cursor = new OwnedUiOperationCursor(source, fixture.parent, op); expect(op.terminalIsEmpty()).toBe(true); expect(finish(cursor)).toBe("ready"); const result = cursor.takeResult()!; close(cursor); retire(source.beginClose()); source = result.nodes; retire(result.touched.beginClose());
      }
      const old = source.capture(); const reader = old.beginLookup(fixture.child); let captured: ReturnType<typeof reader.advance> | null = null;
      for (;;) { const result = reader.advance(grant); if (result.kind === "value") captured = result; if (result.kind === "complete") break; } retire(reader.beginClose());
      if (captured?.kind !== "value" || captured.value.value.component.type !== "surface") throw new Error("Expected old surface owner"); const bytes = captured.value.value.component.doc.bytes;
      const input = new RetainedUiTypedCursor(encodePackValue(fixture.replacement), "component"); expect(finish(input)).toBe("ready"); const payload = input.takeResult()!; const op = OwnedUiOperation.field(fixture.child, { field: "component", payload }); retire(payload.beginClose()); close(input);
      const changed = new OwnedUiOperationCursor(source, fixture.parent, op); expect(finish(changed)).toBe("ready"); const replacement = changed.takeResult()!; close(changed); retire(source.beginClose()); source = replacement.nodes;
      expect(Array.from(replacement.touched, ([id]) => id)).toEqual([fixture.child]); retire(replacement.touched.beginClose());
      const check = source.beginLookup(fixture.child); for (;;) { const result = check.advance(grant); if (result.kind === "value") { expect(result.value.value.component).toEqual(produce({ ...fixture.replacement, emphasize: null, dataAttributes: null }, () => {})); retire(result.value.beginClose()); } if (result.kind === "complete") break; } retire(check.beginClose());
      const reference = new OwnedUiOperationCursor(source, fixture.parent, OwnedUiOperation.remove(fixture.parent)); let count = 0; for (;;) { count++; if (reference.advance(grant).kind === "ready") break; } const removed = reference.takeResult()!; expect(removed.nodes.size).toBe(0); expect(Array.from(removed.touched, ([id]) => id)).toEqual(fixture.expected.removedIds); close(reference); retire(removed.nodes.beginClose()); retire(removed.touched.beginClose());
      for (let cutoff = 0; cutoff <= count; cutoff++) { const cursor = new OwnedUiOperationCursor(source, fixture.parent, OwnedUiOperation.remove(fixture.parent)); for (let i = 0; i < cutoff; i++) cursor.advance(grant); cursor.beginClose(); expect(cursor.takeResult()).toBeNull(); expect(cursor.closeStep({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); close(cursor); expect(source.size).toBe(2); }
      const activity = new RetainedUiTypedCursor(encodePackValue({ activity: "loading", disabled: false }), "activity"); expect(finish(activity)).toBe("ready"); const state = activity.takeResult()!; close(activity); const unknown = new OwnedUiOperationCursor(source, fixture.parent, OwnedUiOperation.activity(123, state)); retire(state.beginClose());
      expect(finish(unknown)).toBe("rejected"); close(unknown); retire(source.beginClose()); retire(old.beginClose()); expect(bytes.byteAt(0)).toBe(3); retire(captured.value.beginClose()); expect(() => bytes.byteAt(0)).toThrow();
    });

    it("OwnedValidation preserves graph violation order, captured byte lifetimes and every cancellation frontier", async () => {
      const { OwnedUiValidationCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🛡️validation/🟦️component.ts");
      const { OwnedUiNodeIndex } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🗂️nodes/🟦️component.ts");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-validation.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-validation.schema.json");
      const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true);
      let prefixes = 0;
      for (const vector of fixture.cases) {
        let source = OwnedUiNodeIndex.empty(); const reference = new Map<number, UiNodeRecord>();
        for (const node of vector.nodes) {
          const raw = { ...fields.node, ...node }; const decoder = new RetainedUiTypedCursor(encodePackValue(raw), "node"); expect(finish(decoder)).toBe("ready"); const payload = decoder.takeResult()!; const owner = OwnedUiNode.captureFrom(payload); reference.set(node.id, { ...payload.value, component: { type: "text", value: "oracle", emphasize: null, dataAttributes: null } }); close(decoder); retire(payload.beginClose());
          const edit = source.beginSet(owner); retire(owner.beginClose()); expect(finish(edit)).toBe("ready"); const next = edit.takeResult()!; retire(edit.beginClose()); retire(source.beginClose()); source = next;
        }
        const limits = { ...DEFAULT_UI_DOCUMENT_LIMITS, maxNodes: vector.maxNodes, maxDepth: vector.maxDepth };
        const expected = validateUiDocumentCore(vector.root, new Map(produce(Array.from(reference), () => {})), limits); expect(expected.map(value => value.type)).toEqual(vector.expected);
        const validator = new OwnedUiValidationCursor(source, vector.root, limits); let count = 0;
        for (;;) { count++; const result = validator.advance(grant); expect(result.bytes).toBeLessThanOrEqual(4096); expect(result.items).toBeLessThanOrEqual(1); if (result.kind === "ready") break; if (result.kind === "rejected") throw new Error(validator.failure ?? "Unexpected validation failure"); }
        const violations = validator.takeResult()!; expect(Array.from(violations, ([, value]) => value)).toEqual(expected); retire(violations.beginClose()); close(validator);
        if (vector.name === "valid-safe53") {
          for (let cutoff = 0; cutoff <= count; cutoff++) { const cursor = new OwnedUiValidationCursor(source, vector.root, limits); for (let i = 0; i < cutoff; i++) cursor.advance(grant); cursor.beginClose(); expect(cursor.takeResult()).toBeNull(); expect(cursor.closeStep({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); close(cursor); expect(source.size).toBe(2); prefixes++; }
          const reader = source.beginLookup(vector.root!); let held: ReturnType<typeof OwnedUiNode.captureFrom> | null = null;
          for (;;) { const result = reader.advance(grant); if (result.kind === "value") held = result.value; if (result.kind === "complete") break; } retire(reader.beginClose());
          if (!held || held.value.component.type !== "surface") throw new Error("Expected surface byte owner"); const bytes = held.value.component.doc.bytes;
          const cursor = new OwnedUiValidationCursor(source, vector.root, limits); retire(source.beginClose()); expect(finish(cursor)).toBe("ready"); retire(cursor.takeResult()!.beginClose()); close(cursor); expect(bytes.byteAt(0)).toBe(3); retire(held.beginClose()); expect(() => bytes.byteAt(0)).toThrow();
        } else retire(source.beginClose());
      }
      expect(prefixes).toBeGreaterThan(100);
      console.info(`[DEBUG] OwnedValidation vectors=${fixture.cases.length} cancellationPrefixes=${prefixes} grants=1/4096`);
    });

    it("OwnedHash streams exact insertion-ordered JSON bytes while retaining old surface owners through cancellation", async () => {
      const { OwnedUiSnapshotHashCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🔢️hash/🟦️component.ts");
      const { OwnedUiNodeIndex } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🗂️nodes/🟦️component.ts");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-hash.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-hash.schema.json");
      const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true);
      let source = OwnedUiNodeIndex.empty(); const records: unknown[] = []; let nested: unknown = fixture.text; for (let i = 0; i < fixture.depth; i++) nested = { next: nested };
      for (const [ordinal, id] of fixture.ids.entries()) {
        const component = ordinal === 0 ? { ...fields.node.component, doc: { bytes: Array.from({ length: fixture.surfaceBytes }, (_, i) => i % 251) } } : { type: "extension", extension: "hash", props: { nested, numbers: [-0, 1e-7, 1e21], "0": true, ["__proto__"]: null } };
        const decoder = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, id, component }), "node"); expect(finish(decoder)).toBe("ready"); const payload = decoder.takeResult()!; const node = OwnedUiNode.captureFrom(payload);
        const record = node.value; records.push({ ...record, component: record.component.type === "surface" ? { ...record.component, doc: { bytes: Array.from({ length: record.component.doc.bytes.length }, (_, i) => record.component.type === "surface" ? record.component.doc.bytes.byteAt(i) : 0) } } : record.component });
        close(decoder); retire(payload.beginClose()); const edit = source.beginSet(node); retire(node.beginClose()); expect(finish(edit)).toBe("ready"); const next = edit.takeResult()!; retire(edit.beginClose()); retire(source.beginClose()); source = next;
      }
      const expected = Buffer.from(JSON.stringify(produce({ surface: fixture.surface, revision: fixture.revision, root: fixture.root, nodes: records, layoutEpoch: "0" }, () => {})), "utf8");
      const cursor = new OwnedUiSnapshotHashCursor(source, fixture); const chunks: Uint8Array[] = []; let calls = 0;
      expect(cursor.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked");
      for (;;) { calls++; const result = cursor.advance(grant); expect(result.bytes).toBeLessThanOrEqual(4096); expect(result.items).toBeLessThanOrEqual(1); if (result.chunk) chunks.push(result.chunk); if (result.kind === "rejected") throw new Error(cursor.failure ?? "Hash failed"); if (result.kind === "ready") break; }
      expect(Buffer.concat(chunks)).toEqual(expected); let hash = 0x811c9dc5; for (const byte of expected) hash = Math.imul(hash ^ byte, 0x01000193) >>> 0;
      expect(cursor.takeResult()).toEqual({ hash: `${hash.toString(16)}:${fixture.revision}`, byteLength: expected.length }); close(cursor);
      for (const cutoff of [0, 1, 3, Math.floor(calls / 3), Math.floor(calls / 2), calls - 1, calls]) { const cancelled = new OwnedUiSnapshotHashCursor(source, fixture); for (let i = 0; i < cutoff; i++) cancelled.advance(grant); cancelled.beginClose(); expect(cancelled.takeResult()).toBeNull(); close(cancelled); expect(source.size).toBe(fixture.ids.length); }
      const retained = new OwnedUiSnapshotHashCursor(source, fixture); retire(source.beginClose()); expect(finish(retained)).toBe("ready"); expect(retained.takeResult()!.byteLength).toBe(expected.length); close(retained);
    });

    it("ReadLease binds exact consumer commits, keeps stable snapshots and backpressures until retired roots are empty", async () => {
      const { OwnedUiNodeReadLease } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️component.ts");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️read-lease.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️read-lease.schema.json");
      const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true);
      const make = () => { const decoder = new RetainedUiTypedCursor(encodePackValue(fields.node), "node"); expect(finish(decoder)).toBe("ready"); const payload = decoder.takeResult()!; const node = OwnedUiNode.captureFrom(payload); close(decoder); retire(payload.beginClose()); return node; };
      const input = make(); const lease = new OwnedUiNodeReadLease(fixture.node, fixture.versions[0]!, input); const independent = new OwnedUiNodeReadLease(fixture.node, fixture.versions[0]!, input); retire(input.beginClose());
      const initial = lease.snapshot; expect(Reflect.apply(lease.acknowledge, lease, [null])).toBe(false); expect(lease.snapshot).toBe(initial); expect(lease.snapshot).toBe(initial); const firstBytes = initial.record!.component; if (firstBytes.type !== "surface") throw new Error("Expected surface");
      const second = make(); expect(lease.offer(fixture.versions[1]!, second)).toBe(true); retire(second.beginClose()); const later = lease.snapshot;
      expect(lease.acknowledge(independent.snapshot)).toBe(false); expect(lease.acknowledge(initial)).toBe(true); expect(lease.snapshot).toBe(later);
      const third = make(); expect(lease.offer(fixture.versions[2]!, third)).toBe(false); expect(third.terminalIsEmpty()).toBe(false); expect(lease.acknowledge(later)).toBe(true); expect(lease.offer(fixture.versions[2]!, third)).toBe(false);
      expect(lease.advanceRetirement({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); while (lease.retirementPending) { const result = lease.advanceRetirement(grant); expect(result.bytes).toBeLessThanOrEqual(4096); }
      expect(firstBytes.doc.bytes.byteAt(0)).toBe(3); close(independent); expect(() => firstBytes.doc.bytes.byteAt(0)).toThrow(); expect(() => initial.record).toThrow();
      expect(lease.offer(fixture.versions[2]!, third)).toBe(true); retire(third.beginClose()); const newest = lease.snapshot; expect(lease.acknowledge(initial)).toBe(false); expect(lease.acknowledge(later)).toBe(true); expect(lease.snapshot).toBe(newest);
      const newestComponent = newest.record!.component; if (newestComponent.type !== "surface") throw new Error("Expected surface"); expect(newestComponent.doc.bytes.byteAt(0)).toBe(3); close(lease); expect(() => newestComponent.doc.bytes.byteAt(0)).toThrow(); expect(() => lease.snapshot).toThrow();
      expect(produce(fixture.versions, () => {})).toEqual([0,1,2]);
    });

    it("ReadLease React subscriptions own StrictMode remounts and independently acknowledge exact committed snapshots", async () => {
      const { useOwnedUiNode } = await import("./📖️owned/🟦️component.tsx");
      const { OwnedUiNodeReadLease } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️component.ts");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json");
      const { encodePackValue } = await import("@semio-tech/framework-os"); const { render, act, cleanup } = await import("@testing-library/react"); const { StrictMode, createElement } = await import("react");
      const decoder = new RetainedUiTypedCursor(encodePackValue(fields.node), "node"); expect(finish(decoder)).toBe("ready"); const payload = decoder.takeResult()!; const node = OwnedUiNode.captureFrom(payload); close(decoder); retire(payload.beginClose());
      type Subscription = import("./📖️owned/🟦️component.tsx").OwnedUiReadSubscription & { readonly lease: InstanceType<typeof OwnedUiNodeReadLease>; readonly notify: () => void };
      const active = new Set<Subscription>(); const closing: Subscription[] = []; const acknowledgements: number[] = [];
      const source: import("./📖️owned/🟦️component.tsx").OwnedUiReadSource = {
        subscribeNode: (id, notify) => { const subscription = { lease: new OwnedUiNodeReadLease(id, 0, node), notify, get snapshot() { return this.lease.snapshot; } }; active.add(subscription); return subscription; },
        acknowledgeRead: (subscription, snapshot) => { for (const value of active) if (value === subscription && value.lease.acknowledge(snapshot)) acknowledgements.push(snapshot.version); },
        unsubscribeNode: subscription => { for (const value of active) if (value === subscription) { active.delete(value); value.lease.beginClose(); closing.push(value); break; } },
      };
      function View({ label }: { label: string }) { const record = useOwnedUiNode(source, fields.node.id); return createElement("span", { "aria-label": label }, record?.component.type === "surface" ? String(record.component.doc.bytes.byteAt(0)) : "pending"); }
      const first = render(createElement(StrictMode, null, createElement(View, { label: "Erste Ansicht" }))); const second = render(createElement(View, { label: "Second view" }));
      expect(first.getByLabelText("Erste Ansicht").textContent).toBe("3"); expect(second.getByLabelText("Second view").textContent).toBe("3"); expect(active.size).toBe(2); expect(closing.length).toBeGreaterThanOrEqual(1);
      for (const subscription of closing.splice(0)) close(subscription.lease);
      const before = acknowledgements.length; await act(async () => { for (const subscription of active) { expect(subscription.lease.offer(1, node)).toBe(true); subscription.notify(); } });
      expect(acknowledgements.slice(before)).toEqual([1,1]); for (const subscription of active) while (subscription.lease.retirementPending) subscription.lease.advanceRetirement(grant);
      first.unmount(); expect(active.size).toBe(1); for (const subscription of closing.splice(0)) close(subscription.lease);
      expect(second.getByLabelText("Second view").textContent).toBe("3"); second.unmount(); expect(active.size).toBe(0); for (const subscription of closing.splice(0)) close(subscription.lease); retire(node.beginClose()); cleanup();
    });

    it("ReadLease source and node replacement reject stale layout acknowledgements while retaining detached owners", async () => {
      const { useOwnedUiNode } = await import("./📖️owned/🟦️component.tsx");
      const { OwnedUiNodeReadLease } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️component.ts");
      const { render, cleanup } = await import("@testing-library/react"); const { createElement } = await import("react");
      type Source = import("./📖️owned/🟦️component.tsx").OwnedUiReadSource;
      type Subscription = import("./📖️owned/🟦️component.tsx").OwnedUiReadSubscription & { readonly lease: InstanceType<typeof OwnedUiNodeReadLease> };
      const owners = () => {
        const active = new Set<Subscription>(); const detached: Subscription[] = []; const accepted: object[] = [];
        const source: Source = {
          subscribeNode: id => { const subscription = { lease: new OwnedUiNodeReadLease(id, 0, null), get snapshot() { return this.lease.snapshot; } }; active.add(subscription); return subscription; },
          acknowledgeRead: (subscription, snapshot) => { for (const value of active) if (value === subscription && value.lease.acknowledge(snapshot)) accepted.push(snapshot); },
          unsubscribeNode: subscription => { for (const value of active) if (value === subscription && active.delete(value)) detached.push(value); },
        };
        return { source, active, detached, accepted };
      };
      const a = owners(); const b = owners();
      function View({ source, id }: { source: Source; id: number }) { useOwnedUiNode(source, id); return createElement("span", null, id); }
      const root = render(createElement(View, { source: a.source, id: 7 })); const old = [...a.active][0]!; const oldToken = old.lease.snapshot;
      root.rerender(createElement(View, { source: a.source, id: 8 })); const replacement = [...a.active][0]!;
      expect(a.detached).toEqual([old]); expect(old.lease.terminalIsEmpty()).toBe(false); expect(replacement.lease.acknowledge(oldToken)).toBe(false);
      const before = a.accepted.length; a.source.acknowledgeRead(old, oldToken); a.source.acknowledgeRead(replacement, oldToken); expect(a.accepted.length).toBe(before);
      const replacementToken = replacement.lease.snapshot; root.rerender(createElement(View, { source: b.source, id: 8 })); const current = [...b.active][0]!;
      expect(a.active.size).toBe(0); expect(a.detached).toEqual([old, replacement]); expect(current.lease.acknowledge(replacementToken)).toBe(false);
      close(old.lease); close(replacement.lease); expect(current.lease.snapshot).not.toBe(replacementToken); expect(current.lease.terminalIsEmpty()).toBe(false);
      root.unmount(); expect(b.active.size).toBe(0); expect(b.detached).toEqual([current]); close(current.lease); cleanup();
    });

    it("ReadPublication exposes all staged reads at one epoch and retains cancelled captures until bounded close", async () => {
      const { OwnedUiNodeReadLease, OwnedUiReadPublication } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️read-publication.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️read-publication.schema.json");
      expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true);
      const publication = new OwnedUiReadPublication(fixture.versions[0]!); const other = new OwnedUiReadPublication(0);
      const a = new OwnedUiNodeReadLease(7, 0, null, publication); const b = new OwnedUiNodeReadLease(8, 0, null, publication);
      const initialA = a.snapshot; const initialB = b.snapshot; const first = publication.begin(1); const foreign = other.begin(1);
      expect(() => a.stage(foreign, null)).toThrow(); expect(a.hasCapacity).toBe(true); expect(() => publication.begin(2)).toThrow();
      expect(a.stage(first, null)).toBe(true); expect(a.snapshot).toBe(initialA); expect(b.stage(first, null)).toBe(true); expect(b.snapshot).toBe(initialB);
      expect(publication.publish(first)).toBe(true); expect([a.snapshot.version, b.snapshot.version]).toEqual(produce([1,1], () => {})); expect(publication.cancel(first)).toBe(false);
      expect(a.acknowledge(a.snapshot)).toBe(true); expect(b.acknowledge(b.snapshot)).toBe(true); while (a.retirementPending) a.advanceRetirement(grant); while (b.retirementPending) b.advanceRetirement(grant);
      const stable = a.snapshot; const next = publication.begin(2); expect(a.stage(next, null)).toBe(true); expect(a.snapshot).toBe(stable); expect(publication.cancel(next)).toBe(true); expect(publication.publish(next)).toBe(false);
      expect(a.snapshot).toBe(stable); expect(a.hasCapacity).toBe(false); expect(a.retirementPending).toBe(true); expect(a.advanceRetirement({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked");
      while (a.retirementPending) a.advanceRetirement(grant); expect(a.hasCapacity).toBe(true); expect(a.snapshot).toBe(stable);
      const closing = publication.begin(2); expect(a.stage(closing, null)).toBe(true); close(a); expect(publication.cancel(closing)).toBe(true); close(b); expect(other.cancel(foreign)).toBe(true);
    });

    //#region 🎬️OwnedSceneTests
    it("OwnedScene prepares native tag vectors and long Unicode slices with strict neutral and Node oracles", async () => {
      const { OwnedUiSceneCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-scene.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-scene.schema.json");
      const { default: rawFixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-scene.json?raw");
      const { encodePackValue, decodeScenePackValue } = await import("@semio-tech/framework-os");
      const { Buffer } = await import("node:buffer");
      const validate = new Ajv({ strict: true, allErrors: true }).compile<typeof fixture>(schema); const exactFixture: unknown = JSON.parse(rawFixture); if (!validate(exactFixture)) throw new Error(JSON.stringify(validate.errors));
      const prepare = (bytes: Uint8Array) => {
        const typed = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "canvas-2d", docSchema: "canvas2d@1", doc: { bytes: Array.from(bytes) }, bindings: [] }), "component");
        expect(finish(typed)).toBe("ready"); const input = typed.takeResult()!; close(typed);
        const scene = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(scene)).toBe("ready"); const document = scene.takeResult()!; close(scene); return document;
      };
      type Document = ReturnType<typeof prepare>;
      const lookup = (document: Document, id: number) => {
        const reader = document.beginRead(id); let value: Extract<ReturnType<typeof reader.advance>, { kind: "value" }>["value"] | undefined;
        for (;;) { const current = reader.advance(grant); if (current.kind === "value") value = current.value; if (current.kind === "complete") break; }
        retire(reader.beginClose()); if (!value) throw new Error("Missing scene record"); return value;
      };
      const text = (document: Document, id: number): string => {
        const reader = document.beginText(id); const chunks: string[] = [];
        for (;;) { const current = reader.advance(grant); if (current.kind === "text") chunks.push(current.value); if (current.kind === "complete") break; }
        retire(reader.beginClose()); return chunks.join("");
      };
      const materialize = (document: Document, id = 0): unknown => {
        const node = lookup(document, id);
        if (node.kind === "unit" || node.kind === "none") return null;
        if (node.kind === "boolean" || node.kind === "float" || node.kind === "char") return node.value;
        if (node.kind === "integer") return Number(node.value);
        if (node.kind === "text") return text(document, id);
        if (node.kind === "bytes") {
          const reader = document.beginBytes(id); const bytes: number[] = [];
          for (;;) { const current = reader.advance(grant); if (current.kind === "bytes") bytes.push(...current.value); if (current.kind === "complete") break; }
          retire(reader.beginClose()); return bytes;
        }
        if (node.kind === "some") return materialize(document, node.first);
        if (node.kind === "variant") { const name = text(document, node.first); const payload = lookup(document, node.first).end; return lookup(document, payload).kind === "unit" ? name : { [name]: materialize(document, payload) }; }
        if (node.kind === "sequence") { const values: unknown[] = []; let child = node.first; for (let i = 0; i < node.count; i++) { values.push(materialize(document, child)); child = lookup(document, child).end; } return values; }
        if (node.kind !== "map") throw new Error("Unknown scene oracle kind");
        const object: Record<string, unknown> = {}; let child = node.first;
        for (let i = 0; i < node.count; i++) { const key = text(document, child); child = lookup(document, child).end; Object.defineProperty(object, key, { value: materialize(document, child), enumerable: true }); child = lookup(document, child).end; } return object;
      };
      for (const vector of exactFixture.cases) {
        const bytes = Buffer.from(vector.hex, "hex"); const document = prepare(bytes); const actual = materialize(document);
        if (vector.name === "prototype-key") { expect(actual).toEqual(vector.value); expect(Object.getPrototypeOf(actual)).toBe(Object.prototype); }
        else expect(actual, vector.name).toEqual(produce(vector.value, () => {}));
        if (vector.name !== "bom-preserved" && vector.name !== "prototype-key") expect(actual, vector.name).toEqual(decodeScenePackValue(bytes));
        if (vector.name === "double") expect(actual).toBe(bytes.readDoubleLE(1));
        if (vector.name === "unicode" || vector.name === "bom-preserved") expect(actual).toBe(new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(bytes.subarray(2)));
        retire(document.beginClose());
      }
      const textValue = fixture.large.unit.repeat(fixture.large.repeat); const raw = Buffer.from(textValue); const header: number[] = [6]; let length = raw.length;
      do { const byte = length & 127; length = Math.floor(length / 128); header.push(byte | (length ? 128 : 0)); } while (length);
      const document = prepare(Buffer.concat([Buffer.from(header), raw])); const reader = document.beginText(0); const captured = document.capture(); retire(document.beginClose());
      let reconstructed = ""; for (;;) { const current = reader.advance(grant); expect(current.bytes).toBeLessThanOrEqual(4096); if (current.kind === "text") { expect(current.value.length).toBeLessThanOrEqual(256); reconstructed += current.value; } if (current.kind === "complete") break; }
      expect(reconstructed).toBe(raw.toString("utf8")); retire(captured.beginClose()); retire(reader.beginClose());
    });
    it("OwnedScene rejects malformed native packets and forged or rebound scene ownership without retiring the source", async () => {
      const { OwnedUiSceneCursor, OwnedUiSceneDocument, OwnedUiSceneReader, OwnedUiSceneRetirement } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-scene.json");
      const { encodePackValue } = await import("@semio-tech/framework-os"); const { Buffer } = await import("node:buffer");
      const source = (bytes: Uint8Array) => { const typed = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "canvas-2d", docSchema: "canvas2d@1", doc: { bytes: Array.from(bytes) }, bindings: [] }), "component"); expect(finish(typed)).toBe("ready"); const input = typed.takeResult()!; close(typed); return input; };
      for (const vector of fixture.hostile) {
        const input = source(Buffer.from(vector.hex, "hex")); const scene = new OwnedUiSceneCursor(input);
        expect(finish(scene), vector.name).toBe("rejected"); expect(scene.takeResult()).toBeNull(); expect(scene.failure).not.toBeNull(); close(scene);
        if (input.value.type !== "surface") throw new Error("Expected source component"); expect(input.value.doc.bytes.length).toBe(vector.hex.length / 2); retire(input.beginClose());
      }
      const style = new RetainedUiTypedCursor(encodePackValue({}), "style"); expect(finish(style)).toBe("ready"); const wrong = style.takeResult()!; close(style);
      expect(() => Reflect.construct(OwnedUiSceneCursor, [wrong])).toThrow(); expect(wrong.terminalIsEmpty()).toBe(false); retire(wrong.beginClose());
      const input = source(new Uint8Array([2]));
      for (const constructor of [OwnedUiSceneDocument, OwnedUiSceneReader, OwnedUiSceneRetirement]) expect(() => Reflect.construct(constructor, [{ references: 1, source: input, index: {} }])).toThrow();
      expect(input.terminalIsEmpty()).toBe(false); retire(input.beginClose());
    });

    it("OwnedScene cancellation owns every frame and reader across exact source lifetimes and maximum page envelope", async () => {
      const { OwnedUiSceneCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-scene.json");
      const { encodePackValue } = await import("@semio-tech/framework-os"); const { Buffer } = await import("node:buffer");
      const source = (bytes: Uint8Array) => { const typed = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "canvas-2d", docSchema: "canvas2d@1", doc: { bytes: Array.from(bytes) }, bindings: [] }), "component"); expect(finish(typed)).toBe("ready"); const input = typed.takeResult()!; close(typed); return input; };
      const raw = Buffer.from(fixture.cases.find(vector => vector.name === "nested-map")!.hex, "hex"); const referenceInput = source(raw); const reference = new OwnedUiSceneCursor(referenceInput); const phases = new Map<string, number>(); let count = 0;
      for (;;) { const current = reference.advance(grant); count++; phases.set(current.phase, count); if (current.kind === "ready") break; if (current.kind === "rejected") throw new Error(reference.failure!); }
      close(reference); retire(referenceInput.beginClose()); expect(count).toBeGreaterThan(100);
      const cutoffs = new Set([0, 1, count, ...phases.values()]);
      for (const cutoff of cutoffs) {
        const input = source(raw); if (input.value.type !== "surface") throw new Error("Expected surface"); const view = input.value.doc.bytes; const cursor = new OwnedUiSceneCursor(input); retire(input.beginClose());
        for (let index = 0; index < cutoff; index++) cursor.advance(grant);
        const progress = cursor.completedBytes; expect(cursor.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(cursor.completedBytes).toBe(progress);
        cursor.beginClose(); expect(cursor.takeResult()).toBeNull(); expect(cursor.closeStep({ maxItems: 1, maxBytes: 4095 }).kind).toBe("blocked"); expect(view.byteAt(0)).toBe(raw[0]); close(cursor); expect(() => view.byteAt(0)).toThrow();
      }
      const prefix = Buffer.from([6, 0xfc, 0xff, 1]); const bytes = Buffer.concat([prefix, Buffer.alloc(fixture.large.maximumBytes - prefix.length, 97)]); expect(bytes.length).toBe(32768);
      const input = source(bytes); if (input.value.type !== "surface") throw new Error("Expected surface"); const view = input.value.doc.bytes; const cursor = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(cursor)).toBe("ready"); const document = cursor.takeResult()!; close(cursor);
      const reader = document.beginText(0); const other = document.beginText(0); retire(document.beginClose()); let length = 0;
      for (;;) { const current = reader.advance(grant); if (current.kind === "text") { expect(current.value).toBe("a".repeat(current.value.length)); length += current.value.length; } if (current.kind === "complete") break; }
      expect(length).toBe(32764); retire(reader.beginClose()); expect(view.byteAt(0)).toBe(6); retire(other.beginClose()); expect(() => view.byteAt(0)).toThrow();
      const deep = source(Buffer.concat([Buffer.alloc(fixture.large.depth, 9), Buffer.from([0])])); const parser = new OwnedUiSceneCursor(deep); retire(deep.beginClose()); let prepared = false;
      const records = fixture.large.depth + 1; const height = 2 * Math.ceil(Math.log2(records + 1)); const allocationNodes = 3; const referenceSlots = allocationNodes * 3;
      const allocationTurns = referenceSlots * (referenceSlots + 1) + referenceSlots + allocationNodes * 2 + referenceSlots + 4;
      const recordTurns = 2 * (height + 1) * (allocationTurns + 16) + 3 * height + 64;
      const maximumTurns = records * recordTurns + records * 4;
      let previousBytes = 0; let previousRecords = 0; let stalled = 0;
      for (let index = 0; index < maximumTurns; index++) {
        const current = parser.advance(grant); const bytes = parser.completedBytes; const completed = parser.completedRecords;
        if (current.items > 1 || current.bytes > 4096) throw new Error("Scene step exceeded its actual grant");
        if (bytes < previousBytes || completed < previousRecords) throw new Error("Scene preparation progress regressed");
        stalled = bytes > previousBytes || completed > previousRecords ? 0 : stalled + 1; if (stalled > recordTurns) throw new Error("Scene record exceeded derived dual-AVL continuation bound");
        previousBytes = bytes; previousRecords = completed;
        if (current.kind === "rejected") throw new Error(parser.failure!); if (current.kind === "ready") { prepared = true; break; }
      }
      expect(prepared, JSON.stringify({ bytes: parser.completedBytes, records: parser.completedRecords, expected: fixture.large.depth + 1 })).toBe(true); const nested = parser.takeResult()!; close(parser); expect(nested.size).toBe(fixture.large.depth + 1); retire(nested.beginClose());
    });

    it("OwnedScene cancels every small-packet prefix and rejects long duplicate keys after real incremental comparison", async () => {
      const { OwnedUiSceneCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-scene.json");
      const { encodePackValue } = await import("@semio-tech/framework-os"); const { Buffer } = await import("node:buffer");
      const source = (bytes: Uint8Array) => { const typed = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "canvas-2d", docSchema: "canvas2d@1", doc: { bytes: Array.from(bytes) }, bindings: [] }), "component"); expect(finish(typed)).toBe("ready"); const input = typed.takeResult()!; close(typed); return input; };
      const original = source(Buffer.from([9, 2])); const reference = new OwnedUiSceneCursor(original); let count = 0;
      for (;;) { count++; if (reference.advance(grant).kind === "ready") break; } close(reference);
      for (let cutoff = 0; cutoff <= count; cutoff++) { const cursor = new OwnedUiSceneCursor(original); for (let index = 0; index < cutoff; index++) cursor.advance(grant); close(cursor); expect(original.terminalIsEmpty()).toBe(false); }
      const copy = source(Buffer.from([9, 2])); if (copy.value.type !== "surface") throw new Error("Expected surface"); const copiedBytes = copy.value.doc.bytes; retire(original.beginClose()); expect(copiedBytes.byteAt(0)).toBe(9); retire(copy.beginClose()); expect(() => copiedBytes.byteAt(0)).toThrow();
      const bytes = Buffer.from(fixture.large.unit.repeat(fixture.large.repeat)); const prefix: number[] = [6]; let length = bytes.length;
      do { const byte = length & 127; length = Math.floor(length / 128); prefix.push(byte | (length ? 128 : 0)); } while (length);
      const input = source(Buffer.concat([Buffer.from([13, 2]), Buffer.from(prefix), bytes, Buffer.from([1]), Buffer.from(prefix), bytes, Buffer.from([2])]));
      const cursor = new OwnedUiSceneCursor(input); let compared = 0;
      for (;;) { const current = cursor.advance(grant); if (current.phase === "scene-key-compare" && current.bytes === 2) compared++; if (current.kind === "rejected") break; if (current.kind === "ready") throw new Error("Duplicate key was admitted"); }
      expect(compared).toBe(bytes.length); expect(cursor.failure).toBe("Duplicate scene map key"); close(cursor);
      const cancelled = new OwnedUiSceneCursor(input); for (;;) { const current = cancelled.advance(grant); if (current.phase === "scene-key-compare" && current.bytes === 2) break; } retire(input.beginClose()); close(cancelled);
      const full = source(Buffer.from("03ffffffffffffffffff01", "hex")); const parser = new OwnedUiSceneCursor(full); retire(full.beginClose()); expect(finish(parser)).toBe("ready"); const document = parser.takeResult()!; close(parser); const reader = document.beginRead(); let actual = 0n;
      for (;;) { const current = reader.advance(grant); if (current.kind === "value" && current.value.kind === "integer") actual = current.value.value; if (current.kind === "complete") break; }
      expect(actual).toBe(Buffer.from("ffffffffffffffff", "hex").readBigUInt64LE()); retire(document.beginClose()); retire(reader.beginClose());
    });
    //#endregion 🎬️OwnedSceneTests

    it("OwnedSurface publishes a validated hashed flat root before ACK and retires exact subscription owners", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts");
      const { OwnedUiOperation } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-surface.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-surface.schema.json");
      const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true);
      const surface = new OwnedUiSurface(fixture.identity, DEFAULT_UI_DOCUMENT_LIMITS); const independent = new OwnedUiSurface(fixture.identity, DEFAULT_UI_DOCUMENT_LIMITS);
      let notices = 0; const subscription = surface.subscribeNode(fixture.root, () => notices++); const foreign = independent.subscribeNode(fixture.root, () => {});
      expect(subscription.snapshot).toBeNull(); while (surface.maintenancePending) surface.advanceMaintenance(grant); expect(subscription.snapshot!.record).toBeUndefined(); expect(notices).toBe(1);
      const input = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, component: { type: "text", value: fixture.label } }), "node"); expect(finish(input)).toBe("ready"); const payload = input.takeResult()!; const record = payload.value;
      const patch = surface.beginPatch(0, 1); patch.pushOperation(OwnedUiOperation.upsert(payload)); retire(payload.beginClose()); close(input); expect(finish(patch)).toBe("ready"); expect(patch.takeAcknowledgement()).toBeNull();
      patch.pushOperation(OwnedUiOperation.setRoot(fixture.root)); expect(finish(patch)).toBe("ready"); patch.finishInput();
      for (;;) { const before = notices; const result = patch.advance(grant); expect(notices - before).toBeLessThanOrEqual(1); expect(result.bytes).toBeLessThanOrEqual(4096); expect(result.items).toBeLessThanOrEqual(1); if (surface.view.revision === 0) expect(subscription.snapshot!.record).toBeUndefined(); if (result.kind === "ready") break; if (result.kind === "rejected") throw new Error(patch.failure ?? "Surface rejected"); }
      expect(surface.view.revision).toBe(1); expect(surface.view.root).toBe(fixture.root); expect(subscription.snapshot!.record).toEqual(record); expect(independent.view.revision).toBe(0);
      const bytes = Buffer.from(JSON.stringify(produce({ surface: fixture.identity.surface, revision: 1, root: fixture.root, nodes: [record], layoutEpoch: "0" }, () => {}))); let hash = 0x811c9dc5; for (const byte of bytes) hash = Math.imul(hash ^ byte, 0x01000193) >>> 0;
      expect(surface.view.hash).toBe(`${hash.toString(16)}:1`); expect(patch.takeAcknowledgement()).toEqual({ ...fixture.identity, revision: 1, hash: surface.view.hash }); expect(patch.takeAcknowledgement()).toBeNull(); close(patch);
      surface.acknowledgeRead(subscription, subscription.snapshot!); surface.acknowledgeRead(foreign, subscription.snapshot!); while (surface.maintenancePending) surface.advanceMaintenance(grant);
      expect(() => surface.beginPatch(0, 2)).toThrow(); const unchanged = surface.view; const invalid = surface.beginPatch(1, 2); invalid.pushOperation(OwnedUiOperation.setRoot(8)); expect(finish(invalid)).toBe("ready"); invalid.finishInput(); expect(finish(invalid)).toBe("rejected"); expect(surface.view).toBe(unchanged); expect(invalid.takeAcknowledgement()).toBeNull(); close(invalid);
      surface.beginClose(); expect(surface.closeStep(grant)).toMatchObject({ kind: "blocked", phase: "surface-readers" }); expect(subscription.snapshot!.record).toEqual(record);
      surface.unsubscribeNode(subscription); expect(surface.maintenancePending).toBe(true); expect(surface.closeStep({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); close(surface); expect(() => subscription.snapshot).toThrow(); independent.unsubscribeNode(foreign); close(independent);
    });

    it("OwnedSurface React reads initialize incrementally and exact unmount releases the final published byte owner", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts");
      const { OwnedUiOperation } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { useOwnedUiNode } = await import("./📖️owned/🟦️component.tsx");
      const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json");
      const { encodePackValue } = await import("@semio-tech/framework-os"); const { render, act, cleanup } = await import("@testing-library/react"); const { createElement } = await import("react");
      const surface = new OwnedUiSurface({ actor: "DOM", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS);
      const decoder = new RetainedUiTypedCursor(encodePackValue(fields.node), "node"); expect(finish(decoder)).toBe("ready"); const payload = decoder.takeResult()!;
      const seed = surface.beginPatch(0, 1); seed.pushOperation(OwnedUiOperation.upsert(payload)); retire(payload.beginClose()); close(decoder); expect(finish(seed)).toBe("ready"); seed.pushOperation(OwnedUiOperation.setRoot(fields.node.id)); expect(finish(seed)).toBe("ready"); seed.finishInput(); expect(finish(seed)).toBe("ready"); expect(seed.takeAcknowledgement()?.revision).toBe(1); close(seed);
      let last: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts").RetainedUiNodeRecord | undefined;
      function View() { const record = useOwnedUiNode(surface, fields.node.id); last = record; return createElement("span", { "aria-label": "Ansicht / View" }, record?.component.type === "surface" ? String(record.component.doc.bytes.byteAt(0)) : record?.component.type === "text" ? record.component.value : "pending"); }
      const root = render(createElement(View)); expect(root.getByLabelText("Ansicht / View").textContent).toBe("pending");
      await act(async () => { while (surface.maintenancePending) surface.advanceMaintenance(grant); }); expect(root.getByLabelText("Ansicht / View").textContent).toBe("3");
      const component = last!.component; if (component.type !== "surface") throw new Error("Expected captured bytes"); const oldBytes = component.doc.bytes;
      const input = new RetainedUiTypedCursor(encodePackValue({ type: "text", value: "Neu / New" }), "component"); expect(finish(input)).toBe("ready"); const field = input.takeResult()!;
      const patch = surface.beginPatch(1, 2); patch.pushOperation(OwnedUiOperation.field(fields.node.id, { field: "component", payload: field })); retire(field.beginClose()); close(input); expect(finish(patch)).toBe("ready"); patch.finishInput();
      await act(async () => { expect(finish(patch)).toBe("ready"); }); expect(root.getByLabelText("Ansicht / View").textContent).toBe("Neu / New"); expect(oldBytes.byteAt(0)).toBe(3); expect(patch.takeAcknowledgement()?.revision).toBe(2); close(patch);
      await act(async () => { while (surface.maintenancePending) surface.advanceMaintenance(grant); }); expect(() => oldBytes.byteAt(0)).toThrow();
      surface.beginClose(); expect(surface.closeStep(grant)).toMatchObject({ kind: "blocked", phase: "surface-readers" }); root.unmount(); close(surface); cleanup();
    });

    it("OwnedSurface notifications use an exact cutoff across unsubscribe, reentrant subscribe and thrown callbacks", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts");
      const { OwnedUiOperation } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const surface = new OwnedUiSurface({ actor: "notices", instance: 1, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS); const events: string[] = [];
      let late: import("./📖️owned/🟦️component.tsx").OwnedUiReadSubscription | null = null;
      let failedOnce = false;
      const first = surface.subscribeNode(fields.node.id, () => { events.push(`a:${surface.view.revision}`); if (surface.view.revision === 1 && !failedOnce) { failedOnce = true; surface.unsubscribeNode(second); late = surface.subscribeNode(fields.node.id, () => events.push(`c:${surface.view.revision}`)); throw new Error("intentional observer failure"); } if (first.snapshot) surface.acknowledgeRead(first, first.snapshot); });
      const second = surface.subscribeNode(fields.node.id, () => events.push(`b:${surface.view.revision}`)); while (surface.maintenancePending) surface.advanceMaintenance(grant);
      const decoder = new RetainedUiTypedCursor(encodePackValue(fields.node), "node"); expect(finish(decoder)).toBe("ready"); const payload = decoder.takeResult()!; const patch = surface.beginPatch(0, 1); patch.pushOperation(OwnedUiOperation.upsert(payload)); retire(payload.beginClose()); close(decoder); expect(finish(patch)).toBe("ready"); patch.pushOperation(OwnedUiOperation.setRoot(fields.node.id)); expect(finish(patch)).toBe("ready"); patch.finishInput();
      for (;;) { const before = events.length; const step = patch.advance(grant); expect(events.length - before).toBeLessThanOrEqual(1); if (step.kind === "ready") break; if (step.kind === "rejected") throw new Error(patch.failure!); }
      expect(events).toEqual(["a:0", "b:0", "a:1"]); expect(surface.notificationFailures).toBe(1); expect(patch.takeAcknowledgement()?.revision).toBe(1); close(patch);
      while (surface.maintenancePending) surface.advanceMaintenance(grant); expect(events).toEqual(["a:0", "b:0", "a:1", "c:1"]);
      expect(surface.takeNotificationFailure()).toEqual({ subscription: first, reason: "callback-threw" }); expect(surface.takeNotificationFailure()).toBeNull(); expect(surface.retryNotification(first)).toBe(true);
      while (surface.maintenancePending) surface.advanceMaintenance(grant); expect(events).toEqual(["a:0", "b:0", "a:1", "c:1", "a:1"]); expect(surface.retryNotification(first)).toBe(false); expect(first.snapshot!.record!.id).toBe(fields.node.id);
      surface.unsubscribeNode(first); if (!late) throw new Error("Expected reentrant subscriber"); surface.unsubscribeNode(late); close(surface);
    });

    it("OwnedSurface rejects text and patch quotas without publishing and preserves rejected input ownership", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts");
      const { OwnedUiOperation } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      for (const limit of ["maxTextBytes", "maxPatchBytes", "maxPatchOps"] as const) {
        const surface = new OwnedUiSurface({ actor: "quota", instance: 1, surface: "window" }, { ...DEFAULT_UI_DOCUMENT_LIMITS, [limit]: 0 }); const previous = surface.view;
        const decoder = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, component: { type: "text", value: "Ü" } }), "node"); expect(finish(decoder)).toBe("ready"); const payload = decoder.takeResult()!; const operation = OwnedUiOperation.upsert(payload); retire(payload.beginClose()); close(decoder); const patch = surface.beginPatch(0, 1);
        if (limit === "maxPatchOps") { expect(() => patch.pushOperation(operation)).toThrow(); expect(operation.terminalIsEmpty()).toBe(false); retire(operation.beginClose()); }
        else { patch.pushOperation(operation); expect(finish(patch)).toBe("rejected"); }
        expect(surface.view).toBe(previous); expect(patch.takeAcknowledgement()).toBeNull(); close(patch); close(surface);
      }
    });

    it("OwnedSurface every cancellation prefix preserves bytes and completes committed notifications and receipt obligations", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts");
      const { OwnedUiOperation } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const run = (cursor: { advance(budget: typeof grant): { kind: string; bytes: number; items: number } }) => { for (let n = 0; n < 10000; n++) { const step = cursor.advance(grant); if (step.bytes > 4096 || step.items > 1) throw new Error("Exceeded grant"); if (step.kind === "ready") return; if (step.kind === "rejected") throw new Error("Unexpected rejection"); } throw new Error("Cursor failed to finish"); };
      const setup = () => {
        const surface = new OwnedUiSurface({ actor: "prefix", instance: 1, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS);
        const make = (byte: number) => { const decoder = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, component: { ...fields.node.component, doc: { bytes: [byte,7,11] } } }), "node"); run(decoder); const payload = decoder.takeResult()!; const component = payload.value.component; if (component.type !== "surface") throw new Error("Expected bytes"); const operation = OwnedUiOperation.upsert(payload); retire(payload.beginClose()); close(decoder); return { operation, bytes: component.doc.bytes }; };
        const original = make(3); const seed = surface.beginPatch(0, 1); seed.pushOperation(original.operation); run(seed); seed.pushOperation(OwnedUiOperation.setRoot(fields.node.id)); run(seed); seed.finishInput(); run(seed); seed.takeAcknowledgement(); close(seed);
        const notices: number[] = []; const subscription = surface.subscribeNode(fields.node.id, () => notices.push(surface.view.revision)); while (surface.maintenancePending) surface.advanceMaintenance(grant);
        const candidate = make(23); const patch = surface.beginPatch(1, 2); patch.pushOperation(candidate.operation);
        return { surface, patch, subscription, notices, original: original.bytes, candidate: candidate.bytes };
      };
      let steps = 0; const phases = new Set<string>();
      const reference = setup(); let finishedInput = false;
      for (;;) { steps++; const step = reference.patch.advance(grant); phases.add(reference.patch.phase); if (step.kind === "rejected") throw new Error(reference.patch.failure!); if (step.kind === "ready") { if (!finishedInput) { reference.patch.finishInput(); finishedInput = true; } else break; } }
      reference.patch.takeAcknowledgement(); close(reference.patch); reference.surface.unsubscribeNode(reference.subscription); close(reference.surface);
      expect(phases).toEqual(new Set(["input", "validation", "hash", "staging", "notifications", "accepted"]));
      for (let cutoff = 0; cutoff <= steps; cutoff++) {
        const owner = setup(); let inputDone = false;
        for (let n = 0; n < cutoff; n++) { const step = owner.patch.advance(grant); if (step.kind === "ready" && !inputDone) { owner.patch.finishInput(); inputDone = true; } }
        const committed = owner.surface.view.revision === 2; owner.patch.beginClose(); expect(owner.patch.closeStep({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); let receipts = 0;
        for (let n = 0; n < 20000; n++) { const step = owner.patch.closeStep(grant); if (step.bytes > 4096 || step.items > 1) throw new Error("Close exceeded grant"); if (step.kind === "blocked") { expect(step.phase).toBe("surface-acknowledgement"); expect(owner.patch.takeAcknowledgement()?.revision).toBe(2); receipts++; } if (step.kind === "complete") break; if (n === 19999) throw new Error("Close failed to finish"); }
        expect(owner.patch.terminalIsEmpty()).toBe(true); expect(receipts).toBe(committed ? 1 : 0); expect(owner.surface.view.revision).toBe(committed ? 2 : 1); expect(owner.notices).toEqual(committed ? [1,2] : [1]); expect(owner.original.byteAt(0)).toBe(3);
        if (committed) expect(owner.candidate.byteAt(0)).toBe(23); else expect(() => owner.candidate.byteAt(0)).toThrow();
        owner.surface.unsubscribeNode(owner.subscription); close(owner.surface); expect(() => owner.original.byteAt(0)).toThrow(); expect(() => owner.candidate.byteAt(0)).toThrow();
      }
      expect(steps).toBeGreaterThan(100);
      console.info(`[DEBUG] OwnedSurface cancellation prefixes: ${steps + 1}; phases: ${[...phases].join(",")}`);
    });

    it("ReadLease aborted speculative rendering never creates a subscription owner", async () => {
      const { useOwnedUiNode } = await import("./📖️owned/🟦️component.tsx"); const { render, cleanup } = await import("@testing-library/react"); const { Suspense, createElement } = await import("react");
      const subscribe = vi.fn(() => { throw new Error("Aborted render must not acquire a read lease"); });
      const source: import("./📖️owned/🟦️component.tsx").OwnedUiReadSource = { subscribeNode: subscribe, acknowledgeRead: vi.fn(), unsubscribeNode: vi.fn() };
      const suspended = new Promise<void>(() => {});
      function View(): never { useOwnedUiNode(source, 7); throw suspended; }
      const root = render(createElement(Suspense, { fallback: createElement("span", null, "Warten") }, createElement(View)));
      expect(root.getByText("Warten")).toBeTruthy(); expect(subscribe).not.toHaveBeenCalled(); root.unmount(); expect(source.unsubscribeNode).not.toHaveBeenCalled(); cleanup();
    });

    it("closes cancellation at every normalization step and traverses deep UiValue without recursive execution", async () => {
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const wire = { type: "surface", kind: "node-graph", docSchema: "node-graph@1", doc: { bytes: [2, 4] }, bindings: [{ trigger: "activate", action: { scope: "a", name: "b", version: 1 }, args: { nested: ["Ü", true] } }] };
      let normalizationSteps = 0;
      const reference = new RetainedUiTypedCursor(encodePackValue(wire), "component");
      for (let n = 0; n < 10000; n++) { const current = reference.advance(grant); if (current.phase === "typed-normalize") normalizationSteps++; if (current.kind === "ready") break; }
      close(reference); expect(normalizationSteps).toBeGreaterThan(20);
      for (let cutoff = -1; cutoff <= normalizationSteps; cutoff++) {
        const cursor = new RetainedUiTypedCursor(encodePackValue(wire), "component"); let observed = 0;
        if (cutoff >= 0) for (let n = 0; n < 10000; n++) { const current = cursor.advance(grant); if (current.phase === "typed-normalize" && observed++ === cutoff) break; if (current.kind === "ready") break; }
        cursor.beginClose(); expect(cursor.takeResult()).toBeNull(); expect(cursor.closeStep({ maxItems: 1, maxBytes: 4095 }).kind).toBe("blocked"); close(cursor);
      }
      let nested: unknown = "last"; for (let i = 0; i < 500; i++) nested = { next: nested };
      const deep = new RetainedUiTypedCursor(encodePackValue({ type: "extension", extension: "deep", props: nested }), "component"); expect(finish(deep), deep.failure ?? "").toBe("ready"); const payload = deep.takeResult()!; close(deep); expect(payload.value.type).toBe("extension"); retire(payload.beginClose());
    });
  });
  //#endregion 🧾️TypedWireTests

  //#region 📦️OwnedWireTests
  function wireClose(cursor: InstanceType<typeof RetainedUiWireValueCursor>): void {
    cursor.beginClose();
    for (let n = 0; n < 100_000; n++) {
      const step = cursor.closeStep({ maxItems: 1, maxBytes: 4096 });
      expect(step.items).toBeLessThanOrEqual(1); expect(step.bytes).toBeLessThanOrEqual(4096);
      if (step.kind === "complete") { expect(cursor.terminalIsEmpty()).toBe(true); return; }
    }
    throw new Error("Owned wire retirement failed to terminate");
  }

  function wireReady(cursor: InstanceType<typeof RetainedUiWireValueCursor>): "ready" | "rejected" {
    for (let n = 0; n < 100_000; n++) {
      const step = cursor.advance({ maxItems: 1, maxBytes: 4096 });
      expect(step.items).toBeLessThanOrEqual(1); expect(step.bytes).toBeLessThanOrEqual(4096);
      if (step.kind === "ready" || step.kind === "rejected") return step.kind;
    }
    throw new Error("Owned wire decode failed to terminate");
  }

  describe("OwnedWire", () => {
    it("validates strict neutral native bounds and canonical Rust byte vectors", async () => {
      const { decodePackValue } = await import("@semio-tech/framework-os");
      expect(new Ajv({ strict: true, allErrors: true }).compile(wireSchema)(wireFixture)).toBe(true);
      for (const vector of wireFixture.vectors) {
        const input = Uint8Array.from(Buffer.from(vector.hex, "hex"));
        const reference = decodePackValue(input);
        const cursor = new RetainedUiWireValueCursor(input);
        expect(input.byteLength).toBe(0);
        expect(cursor.advance({ maxItems: 1, maxBytes: 4096 }).bytes).toBeGreaterThan(0);
        expect(wireReady(cursor)).toBe("ready");
        expect(cursor.value).toEqual(vector.expected); expect(cursor.value).toEqual(reference);
        wireClose(cursor);
      }
    });

    it("rejects malformed canonical framing and values without publishing a partial root", () => {
      for (const vector of wireFixture.hostile) {
        const cursor = new RetainedUiWireValueCursor(Uint8Array.from(Buffer.from(vector.hex, "hex")));
        expect(wireReady(cursor), vector.name).toBe("rejected"); expect(cursor.value).toBeUndefined();
        wireClose(cursor);
      }
    });

    it("owns large nested immutable values and preserves __proto__ as data", async () => {
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const value = Array.from({ length: wireFixture.large.rows }, (_, id) => ({ id, text: wireFixture.large.text.repeat(wireFixture.large.repeat), nested: { enabled: true } }));
      const input = encodePackValue(value);
      expect(input.length).toBeGreaterThan(4096);
      const cursor = new RetainedUiWireValueCursor(input);
      expect(wireReady(cursor)).toBe("ready"); expect(cursor.value).toEqual(JSON.parse(JSON.stringify(produce(value, () => {}))));
      expect(Object.isFrozen(cursor.value)).toBe(true);
      const result = cursor.value;
      expect(Array.isArray(result)).toBe(true);
      if (Array.isArray(result)) {
        expect(Reflect.set(result, "0", null)).toBe(false);
        expect(Reflect.set(result[0], "text", "forged")).toBe(false);
        expect(Object.isFrozen(result[0].nested)).toBe(true);
      }
      wireClose(cursor);
      const hostileName = new RetainedUiWireValueCursor(encodePackValue(JSON.parse('{"__proto__":{"safe":true}}')));
      expect(wireReady(hostileName)).toBe("ready"); expect(Object.hasOwn(hostileName.value as object, "__proto__")).toBe(true);
      expect(Object.getPrototypeOf(hostileName.value)).toBe(Object.prototype);
      wireClose(hostileName);
    });

    it("cancels at every decoding phase and rejects shared or aliased input ownership", async () => {
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const value = { a: [false, 42, { deep: "unicode😀" }], b: null };
      const phases = new Set<string>();
      for (let stop = 0; stop < 300; stop++) {
        const cursor = new RetainedUiWireValueCursor(encodePackValue(value));
        expect(cursor.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked");
        let ready = false;
        for (let n = 0; n < stop; n++) { const step = cursor.advance({ maxItems: 1, maxBytes: 4096 }); phases.add(step.phase); if (step.kind === "ready") { ready = true; break; } }
        wireClose(cursor); expect(cursor.value).toBeUndefined();
        if (ready) break;
      }
      expect(phases.size).toBeGreaterThanOrEqual(7);
      const shared = new Uint8Array(new SharedArrayBuffer(8));
      expect(() => new RetainedUiWireValueCursor(shared)).toThrow(/ownership/);
      const entire = new Uint8Array(32); const subview = entire.subarray(4, 12);
      expect(() => new RetainedUiWireValueCursor(subview)).toThrow(/ownership/); expect(entire.byteLength).toBe(32);
      const memory = new WebAssembly.Memory({ initial: 1 });
      expect(() => new RetainedUiWireValueCursor(new Uint8Array(memory.buffer))).toThrow(/ownership/); expect(memory.buffer.byteLength).toBe(65536);
      const bytes = encodePackValue(value); const alias = new Uint8Array(bytes.buffer);
      const first = new RetainedUiWireValueCursor(bytes);
      expect(alias.byteLength).toBe(0);
      expect(() => new RetainedUiWireValueCursor(alias)).toThrow(/ownership/);
      wireClose(first);
    });

    it("links admitted scalar and collection bounds to the owning native schema", async () => {
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const { readFileSync } = await import("node:fs");
      const { resolve } = await import("node:path");
      const action = readFileSync(resolve(process.cwd(), "../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️action.rs"), "utf8");
      expect(action).toContain(`UI_TEXT_MAX_BYTES: usize = ${wireFixture.nativeBounds.textBytes};`);
      expect(action).toContain(`UI_VALUE_MAX_ITEMS: usize = ${wireFixture.nativeBounds.collectionItems};`);
      for (const value of ["x".repeat(512), Array.from({ length: 256 }, (_, i) => i)]) {
        const cursor = new RetainedUiWireValueCursor(encodePackValue(value));
        expect(cursor.advance({ maxItems: 1, maxBytes: 4095 }).kind).toBe("blocked");
        expect(wireReady(cursor)).toBe("ready"); expect(cursor.value).toEqual(value);
        cursor.beginClose(); expect(cursor.closeStep({ maxItems: 1, maxBytes: 4095 }).kind).toBe("blocked"); wireClose(cursor);
      }
      const overflow = new RetainedUiWireValueCursor(encodePackValue("x".repeat(513)));
      expect(wireReady(overflow)).toBe("rejected"); wireClose(overflow);
    });
  });
  //#endregion 📦️OwnedWireTests

  //#region 📦️SurfaceBytePagesTests
  describe("SurfaceBytePages", () => {
    it("matches Node Buffer at every native page boundary and preserves exact captured owners", async () => {
      const { UiSurfaceByteBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🔢️bytes/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️surface-bytes.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️surface-bytes.schema.json");
      expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
      for (const length of fixture.lengths) {
        const expected = Buffer.from(Array.from({ length }, (_, i) => (i * fixture.pattern.multiply + fixture.pattern.add) % fixture.pattern.modulo));
        const builder = new UiSurfaceByteBuilder(length);
        expect(builder.advance({ maxItems: 1, maxBytes: 4095 }, 3).kind).toBe("blocked");
        let offset = 0;
        for (let turn = 0; turn < 100_000; turn++) {
          const step = builder.advance({ maxItems: 1, maxBytes: 4096 }, expected[offset]);
          expect(step.bytes).toBeLessThanOrEqual(4096); expect(step.items).toBeLessThanOrEqual(1);
          if (step.accepted) offset++;
          if (step.kind === "ready") break;
        }
        expect(offset).toBe(length);
        const value = builder.takeResult(); expect(value).not.toBeNull(); expect(builder.terminalIsEmpty()).toBe(true);
        const captured = value!.capture(); const retirement = value!.beginClose();
        expect(value!.terminalIsEmpty()).toBe(true); expect(() => value!.byteAt(0)).toThrow(/closed/);
        expect(retirement.advance({ maxItems: 1, maxBytes: 4096 }).kind).toBe("complete");
        expect(Buffer.from(Array.from({ length }, (_, index) => captured.byteAt(index)))).toEqual(expected);
        const final = captured.beginClose(); let released = 0;
        for (let turn = 0; turn < 1000; turn++) {
          const step = final.advance({ maxItems: 1, maxBytes: 4096 }); released += step.bytes;
          expect(step.bytes).toBeLessThanOrEqual(4096); expect(step.items).toBeLessThanOrEqual(1);
          if (step.kind === "complete") break;
        }
        expect(final.terminalIsEmpty()).toBe(true); expect(released).toBeGreaterThanOrEqual(length);
      }
      for (const length of fixture.invalidLengths) expect(() => new UiSurfaceByteBuilder(length)).toThrow(RangeError);
    });

    it("retires cancellation before allocation, between pages, and after preparation without transferring partial bytes", async () => {
      const { UiSurfaceByteBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🔢️bytes/🟦️component.ts");
      for (const stop of [0, 1, 2, 3, 255, 256, 257, 258, 260, 600]) {
        const builder = new UiSurfaceByteBuilder(513);
        for (let n = 0; n < stop; n++) builder.advance({ maxItems: 1, maxBytes: 4096 }, n % 256);
        const close = builder.beginClose(); expect(builder.takeResult()).toBeNull();
        expect(close.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked");
        for (let n = 0; n < 100; n++) if (close.advance({ maxItems: 1, maxBytes: 4096 }).kind === "complete") break;
        expect(close.terminalIsEmpty()).toBe(true); expect(builder.terminalIsEmpty()).toBe(true);
      }
      const invalid = new UiSurfaceByteBuilder(1);
      expect(invalid.advance({ maxItems: 1, maxBytes: 4096 }, 256).kind).toBe("rejected");
      expect(invalid.takeResult()).toBeNull(); expect(invalid.beginClose().advance({ maxItems: 1, maxBytes: 4096 }).kind).toBe("complete");
    });

    it("decodes the full native SurfaceDoc numeric-list wire into pages without changing ordinary UiValue bounds", async () => {
      const { UiSurfaceBytes } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🔢️bytes/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️surface-bytes.json");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      const expected = Buffer.from(Array.from({ length: fixture.maximumBytes }, (_, i) => (i * fixture.pattern.multiply + fixture.pattern.add) % fixture.pattern.modulo));
      const component = { type: "surface", kind: "node-graph", docSchema: "node-graph@1", doc: { bytes: Array.from(expected) }, bindings: [] };
      const denied = new RetainedUiWireValueCursor(encodePackValue(component));
      expect(wireReady(denied)).toBe("rejected"); wireClose(denied);
      for (const profile of ["component", "node"] as const) {
        const cursor = new RetainedUiWireValueCursor(encodePackValue(profile === "node" ? { component } : component), profile);
        expect(wireReady(cursor), cursor.failure ?? "").toBe("ready");
        const decoded = cursor.value;
        if (!decoded || typeof decoded !== "object") throw new Error("Expected owned component record");
        const resolved: unknown = profile === "node" ? Reflect.get(decoded, "component") : decoded;
        if (!resolved || typeof resolved !== "object") throw new Error("Expected component");
        const document: unknown = Reflect.get(resolved, "doc");
        if (!document || typeof document !== "object") throw new Error("Expected SurfaceDoc");
        const bytes: unknown = Reflect.get(document, "bytes");
        expect(bytes).toBeInstanceOf(UiSurfaceBytes);
        if (!(bytes instanceof UiSurfaceBytes)) throw new Error("Expected paged bytes");
        const capture = bytes.capture(); wireClose(cursor);
        expect(bytes.terminalIsEmpty()).toBe(true);
        expect(Buffer.from(Array.from({ length: capture.length }, (_, i) => capture.byteAt(i)))).toEqual(expected);
        const close = capture.beginClose();
        for (let n = 0; n < 1000; n++) if (close.advance({ maxItems: 1, maxBytes: 4096 }).kind === "complete") break;
        expect(close.terminalIsEmpty()).toBe(true);
      }
      for (const bytes of [[256], [false], [1.5], Array.from({ length: fixture.maximumBytes + 1 }, () => 0)]) {
        const cursor = new RetainedUiWireValueCursor(encodePackValue({ doc: { bytes }, type: "surface" }), "component");
        expect(wireReady(cursor)).toBe("rejected"); wireClose(cursor);
      }
    });
  });
  //#endregion 📦️SurfaceBytePagesTests

  const TEST_STYLE: StyleSpec = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
  const TEST_ACCESSIBILITY: AccessibilitySpec = { label: null, description: null, live: "off", shortcut: null, hidden: false };

  // 🐛 `{ type: "separator" }` alone does NOT satisfy `Component` here — `SeparatorProps` is
  // `Record<string, never>` (a genuinely empty Rust struct), and TypeScript's structural check on an
  // object LITERAL treats that index signature as applying to every key of the intersection,
  // including the discriminant itself (`Property 'type' is incompatible with index signature: Type
  // '"separator"' is not assignable to type 'never'`) — reproduced standalone, not this file's bug,
  // and not fixable by loosening a type. A minimal `container` literal has real fields and sidesteps
  // it; see the packet report for the finding (applies to any bare `{ type: "separator" }` literal
  // asserted directly against `Component`, repo-wide).
  function leaf(id: number, key: string): UiNodeRecord {
    return {
      id,
      key,
      component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
      layout: { kind: "leaf", width: "hug", height: "hug" },
      style: TEST_STYLE,
      activity: "idle",
      disabled: false,
      transition: null,
      accessibility: TEST_ACCESSIBILITY,
      bindings: [],
      menu: null,
      children: [],
    };
  }

  function container(id: number, key: string, children: readonly number[]): UiNodeRecord {
    return { ...leaf(id, key), component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null }, children: [...children] };
  }

  function snapshot(root: number, nodes: readonly UiNodeRecord[]): UiSnapshot {
    return { surface: "s", revision: 0, root, nodes: [...nodes], layoutEpoch: 0n };
  }

  //#region 🧵️RetainedTests
  function retainedHydrate(value: UiSnapshot, bytes: number): RetainedUiState {
    const cursor = new RetainedUiSnapshotCursor(value);
    for (let n = 0; n < 2_000_000; n++) {
      const step = cursor.advance({ maxItems: 1, maxBytes: bytes });
      expect(step.items).toBeLessThanOrEqual(1);
      expect(step.bytes).toBeLessThanOrEqual(bytes);
      if (step.kind === "ready") {
        const result = cursor.takeResult();
        expect(result).not.toBeNull();
        expect(cursor.terminalIsEmpty()).toBe(true);
        return result!;
      }
    }
    throw new Error("Retained hydration did not terminate");
  }

  function retainedClose(cursor: RetainedUiPatchCursor, bytes: number): void {
    cursor.beginClose();
    for (let n = 0; n < 2_000_000; n++) {
      const step = cursor.closeStep({ maxItems: 1, maxBytes: bytes });
      expect(step.bytes).toBeLessThanOrEqual(bytes);
      if (step.kind === "complete") { expect(cursor.terminalIsEmpty()).toBe(true); return; }
    }
    throw new Error("Retained patch close did not terminate");
  }

  function closeIndex<V>(index: import("@semio-tech/framework").NumericIndex<V>, bytes: number): void {
    const retirement = index.beginClose();
    for (let n = 0; n < 2_000_000; n++) {
      const step = retirement.advance({ maxItems: 1, maxBytes: bytes });
      if (step.kind === "complete") return;
    }
    throw new Error("Retained test root did not retire");
  }

  describe("Retained UI patch preparation", () => {
    it("strictly validates its language-neutral lifecycle and UTF8 fixture", () => {
      const validate = new Ajv({ strict: true, allErrors: true }).compile(retainedSchema);
      expect(validate(retainedFixture), JSON.stringify(validate.errors)).toBe(true);
      expect(validate({ ...retainedFixture, unknown: true })).toBe(false);
      expect(new TextEncoder().encode(retainedFixture.text.unit.repeat(retainedFixture.text.repeat)).length).toBe(retainedFixture.text.utf8Bytes);
    });

    for (const bytes of retainedFixture.grants) for (const vector of retainedFixture.cases) {
      it(`${vector.name} at one item/${bytes} bytes matches the reference rejection order and Immer value oracle`, () => {
        const records = Array.from({ length: vector.size }, (_, id) => leaf(id, `node:${id}`));
        records[0] = container(0, "root", records.slice(1).map((record) => record.id));
        const text = retainedFixture.text.unit.repeat(retainedFixture.text.repeat);
        const patch: UiPatch = { surface: "s", baseRevision: vector.kind === "stale" ? 99 : 0, revision: 1, ops: [] };
        switch (vector.kind) {
          case "replace": patch.ops.push({ type: "setComponent", id: 1, component: { type: "text", value: text, emphasize: null, dataAttributes: null } }); break;
          case "remove": patch.ops.push({ type: "remove", id: 0 }); break;
          case "cycle": patch.ops.push({ type: "setChildren", id: 1, children: [0] }); break;
          case "orphan": patch.ops.push({ type: "setChildren", id: 1, children: [9999] }); break;
          case "duplicate": records[1] = { ...records[1]!, key: text }; records[2] = { ...records[2]!, key: text }; break;
          case "dangling": records[0] = container(0, "root", []); break;
          case "depth": for (let id = 0; id < records.length; id++) records[id] = container(id, `node:${id}`, id + 1 < records.length ? [id + 1] : []); break;
        }
        const initial = snapshot(0, records);
        const reference = applyUiPatch(uiDocumentStateFromSnapshot(initial), patch);
        const state = retainedHydrate(initial, bytes);
        const captured = state.nodes.capture();
        const cursor = new RetainedUiPatchCursor(state, patch, DEFAULT_UI_DOCUMENT_LIMITS);
        expect(cursor.advance({ maxItems: 0, maxBytes: bytes }).kind).toBe("blocked");
        let complete = false;
        for (let n = 0; n < 4_000_000; n++) {
          const step = cursor.advance({ maxItems: 1, maxBytes: bytes });
          if (step.bytes > bytes || step.items > 1) throw new Error("Retained patch exceeded its grant");
          if (step.kind !== "ready" && step.kind !== "rejected") continue;
          const result = cursor.takeResult()!;
          expect(result.ok).toBe(reference.ok);
          if (result.ok && reference.ok) {
            expect([...result.state.nodes]).toEqual([...reference.state.nodes]);
            const oracle = produce(initial.nodes, (draft) => {
              if (vector.kind === "replace") draft[1]!.component = { type: "text", value: text, emphasize: null, dataAttributes: null };
              if (vector.kind === "remove") draft.splice(0);
            });
            expect([...result.state.nodes].map((entry) => entry[1])).toEqual(oracle);
            closeIndex(result.state.nodes, bytes); closeIndex(result.touched, bytes);
          } else if (!result.ok && !reference.ok) {
            if (result.rejection.type === "invariantViolated") {
              expect({ type: result.rejection.type, violations: [...result.rejection.violations].map((entry) => entry[1]) }).toEqual(reference.rejection);
              closeIndex(result.rejection.violations, bytes);
            } else expect(result.rejection).toEqual(reference.rejection);
          }
          retainedClose(cursor, bytes);
          expect([...captured].map((entry) => entry[1])).toEqual(initial.nodes);
          complete = true; break;
        }
        expect(complete).toBe(true);
        closeIndex(state.nodes, bytes); closeIndex(captured, bytes);
      });
    }

    it("cancels every semantic phase without publishing or invalidating an old captured reader", () => {
      const initial = snapshot(0, [container(0, "root", [1]), leaf(1, "leaf")]);
      const state = retainedHydrate(initial, 256);
      const patch: UiPatch = { surface: "s", baseRevision: 0, revision: 1, ops: [{ type: "setComponent", id: 1, component: { type: "text", value: retainedFixture.text.unit.repeat(32), emphasize: null, dataAttributes: null } }] };
      for (const phase of ["admission", "accounting", "application", "validation", "candidate"]) {
        const reader = state.nodes.capture();
        const cursor = new RetainedUiPatchCursor(state, patch, DEFAULT_UI_DOCUMENT_LIMITS);
        if (phase !== "admission") {
          let reached = false;
          for (let n = 0; n < 100_000; n++) { const step = cursor.advance({ maxItems: 1, maxBytes: 256 }); if (step.phase === phase) { reached = true; break; } }
          expect(reached).toBe(true);
        }
        retainedClose(cursor, 256);
        expect([...reader].map((entry) => entry[1])).toEqual(initial.nodes);
        expect(state.revision).toBe(0);
        closeIndex(reader, 256);
      }
      closeIndex(state.nodes, 256);
    });
  });

  describe("Retained UI atomic publication", () => {
    function prepare(transaction: RetainedUiTransaction): "ready" | "rejected" {
      for (let n = 0; n < 100_000; n++) {
        expect(transaction.takeAcknowledgement()).toBeNull();
        const step = transaction.advance({ maxItems: 1, maxBytes: 256 });
        if (step.kind === "ready" || step.kind === "rejected") return step.kind;
      }
      throw new Error("Retained publication did not prepare");
    }

    function retire(transaction: RetainedUiTransaction): void {
      transaction.beginClose();
      for (let n = 0; n < 100_000; n++) {
        const step = transaction.closeStep({ maxItems: 1, maxBytes: 256 });
        expect(step.bytes).toBeLessThanOrEqual(256);
        if (step.kind === "complete") { expect(transaction.terminalIsEmpty()).toBe(true); return; }
      }
      throw new Error("Retained publication did not retire");
    }

    function closeOwner(owner: RetainedUiSurfaceOwner): void {
      const retirement = owner.beginClose();
      for (let n = 0; n < 100_000; n++) if (retirement.advance({ maxItems: 1, maxBytes: 256 }).kind === "complete") return;
      throw new Error("Retained surface did not retire");
    }

    it("mints an acknowledgement only after exact root and revision publication", () => {
      const owner = new RetainedUiSurfaceOwner("actor", 7, retainedHydrate(snapshot(0, [leaf(0, "root")]), 256), DEFAULT_UI_DOCUMENT_LIMITS);
      const captured = owner.capture();
      const transaction = owner.beginPatch({ surface: "s", baseRevision: 0, revision: 1, ops: [{ type: "setComponent", id: 0, component: { type: "text", value: "published", emphasize: null, dataAttributes: null } }] });
      expect(owner.publish(transaction)).toBe(false);
      expect(prepare(transaction)).toBe("ready");
      expect(owner.revision).toBe(0);
      expect(owner.publish(transaction)).toBe(true);
      expect(owner.revision).toBe(1);
      expect(owner.getNode(0)?.component).toMatchObject({ value: "published" });
      expect(transaction.takeAcknowledgement()).toEqual({ identity: { actor: "actor", instance: 7, surface: "s" }, revision: 1 });
      expect(transaction.takeAcknowledgement()).toBeNull();
      expect(owner.publish(transaction)).toBe(false);
      retire(transaction);
      expect(captured.nodes.get(0)?.component.type).toBe("container");
      closeOwner(owner);
      expect(captured.nodes.get(0)?.key).toBe("root");
      closeIndex(captured.nodes, 256);
    });

    it("rejects rebound owners and stale concurrent candidates without emitting ACK", () => {
      const initial = snapshot(0, [leaf(0, "root")]);
      const owner = new RetainedUiSurfaceOwner("same-actor", 7, retainedHydrate(initial, 256), DEFAULT_UI_DOCUMENT_LIMITS);
      const other = new RetainedUiSurfaceOwner("same-actor", 8, retainedHydrate(initial, 256), DEFAULT_UI_DOCUMENT_LIMITS);
      const patch: UiPatch = { surface: "s", baseRevision: 0, revision: 1, ops: [] };
      const first = owner.beginPatch(patch);
      const second = owner.beginPatch(patch);
      expect(prepare(first)).toBe("ready"); expect(prepare(second)).toBe("ready");
      expect(other.publish(first)).toBe(false);
      expect(first.takeAcknowledgement()).toBeNull();
      expect(owner.publish(first)).toBe(true);
      expect(owner.publish(second)).toBe(false);
      expect(second.takeAcknowledgement()).toBeNull();
      expect(other.revision).toBe(0);
      retire(first); retire(second); closeOwner(owner); closeOwner(other);
    });

    it("rejects a different surface and cancellation of a ready candidate", () => {
      const owner = new RetainedUiSurfaceOwner("actor", 7, retainedHydrate(snapshot(0, [leaf(0, "root")]), 256), DEFAULT_UI_DOCUMENT_LIMITS);
      const wrong = owner.beginPatch({ surface: "other", baseRevision: 0, revision: 1, ops: [] });
      expect(prepare(wrong)).toBe("rejected");
      expect(owner.publish(wrong)).toBe(false); retire(wrong);
      const cancelled = owner.beginPatch({ surface: "s", baseRevision: 0, revision: 1, ops: [] });
      expect(prepare(cancelled)).toBe("ready"); retire(cancelled);
      expect(owner.publish(cancelled)).toBe(false);
      expect(cancelled.takeAcknowledgement()).toBeNull();
      expect(owner.revision).toBe(0); closeOwner(owner);
    });
  });
  //#endregion 🧵️RetainedTests

  describe("UiDocumentStore transactions", () => {
    it("applies every op kind and advances the revision", () => {
      const store = new UiDocumentStore("s");
      store.loadSnapshot(snapshot(0, [container(0, "root", [1]), leaf(1, "a")]));
      const result = store.applyPatch({
        surface: "s",
        baseRevision: 0,
        revision: 1,
        ops: [
          { type: "upsert", ...leaf(2, "b") },
          { type: "setChildren", id: 0, children: [1, 2] },
          { type: "setComponent", id: 1, component: { type: "text", value: "hi", emphasize: null, dataAttributes: null } },
        ],
      });
      expect(result.ok).toBe(true);
      expect(store.getState().revision).toBe(1);
      expect(store.getNodeSnapshot(0)?.children).toEqual([1, 2]);
      expect(store.getNodeSnapshot(1)?.component).toEqual({ type: "text", value: "hi", emphasize: null, dataAttributes: null });
      expect(store.getNodeSnapshot(2)).toBeDefined();
    });

    it("rejects a stale baseRevision and leaves state reference-identical", () => {
      const store = new UiDocumentStore("s");
      store.loadSnapshot(snapshot(0, [leaf(0, "root")]));
      const before = store.getState();
      const result = store.applyPatch({ surface: "s", baseRevision: 5, revision: 6, ops: [] });
      expect(result.ok).toBe(false);
      if (!result.ok) expect(result.rejection).toEqual({ type: "revisionMismatch", expected: 0, actual: 5 });
      expect(store.getState()).toBe(before);
    });

    it("rejects a cycle and leaves state unchanged", () => {
      const store = new UiDocumentStore("s");
      store.loadSnapshot(snapshot(0, [container(0, "root", [1]), container(1, "a", [])]));
      const before = store.getState();
      const result = store.applyPatch({ surface: "s", baseRevision: 0, revision: 1, ops: [{ type: "setChildren", id: 1, children: [0] }] });
      expect(result.ok).toBe(false);
      if (!result.ok) expect(result.rejection).toEqual({ type: "invariantViolated", violations: [{ type: "cycle", node: 0 }] });
      expect(store.getState()).toBe(before);
      expect(store.getNodeSnapshot(1)?.children).toEqual([]);
    });

    it("rejects an unknown node target", () => {
      const store = new UiDocumentStore("s");
      store.loadSnapshot(snapshot(0, [leaf(0, "root")]));
      const result = store.applyPatch({ surface: "s", baseRevision: 0, revision: 1, ops: [{ type: "setComponent", id: 99, component: { type: "text", value: "x", emphasize: null, dataAttributes: null } }] });
      expect(result.ok).toBe(false);
      if (!result.ok) expect(result.rejection).toEqual({ type: "unknownNode", id: 99 });
    });

    it("rejects an oversized patch by op count", () => {
      const store = new UiDocumentStore("s", { ...DEFAULT_UI_DOCUMENT_LIMITS, maxPatchOps: 1 });
      store.loadSnapshot(snapshot(0, [leaf(0, "root")]));
      const result = store.applyPatch({ surface: "s", baseRevision: 0, revision: 1, ops: [{ type: "setLayout", id: 0, layout: { kind: "leaf", width: "fill", height: "hug" } }, { type: "setStyle", id: 0, style: TEST_STYLE }] });
      expect(result.ok).toBe(false);
      if (!result.ok) expect(result.rejection).toEqual({ type: "quotaExceeded", quota: "patchOps", actual: 2, max: 1 });
    });

    it("removes a whole orphaned subtree", () => {
      const store = new UiDocumentStore("s");
      store.loadSnapshot(snapshot(0, [container(0, "root", [1]), container(1, "mid", [2]), leaf(2, "leaf")]));
      const result = store.applyPatch({ surface: "s", baseRevision: 0, revision: 1, ops: [{ type: "remove", id: 1 }, { type: "setChildren", id: 0, children: [] }] });
      expect(result.ok).toBe(true);
      expect(store.getNodeSnapshot(1)).toBeUndefined();
      expect(store.getNodeSnapshot(2)).toBeUndefined();
    });
  });

  describe("UiDocumentStore per-node subscription granularity", () => {
    it("notifies only the changed node's listeners, not siblings", () => {
      const store = new UiDocumentStore("s");
      store.loadSnapshot(snapshot(0, [container(0, "root", [1, 2]), leaf(1, "a"), leaf(2, "b")]));
      const onA = vi.fn();
      const onB = vi.fn();
      const onRoot = vi.fn();
      store.subscribeNode(1)(onA);
      store.subscribeNode(2)(onB);
      store.subscribeNode(0)(onRoot);
      store.applyPatch({ surface: "s", baseRevision: 0, revision: 1, ops: [{ type: "setComponent", id: 1, component: { type: "text", value: "changed", emphasize: null, dataAttributes: null } }] });
      expect(onA).toHaveBeenCalledTimes(1);
      expect(onB).toHaveBeenCalledTimes(0);
      expect(onRoot).toHaveBeenCalledTimes(0);
    });

    it("does not notify any node listener on a rejected patch", () => {
      const store = new UiDocumentStore("s");
      store.loadSnapshot(snapshot(0, [leaf(0, "root")]));
      const onRoot = vi.fn();
      store.subscribeNode(0)(onRoot);
      store.applyPatch({ surface: "s", baseRevision: 99, revision: 100, ops: [] });
      expect(onRoot).toHaveBeenCalledTimes(0);
    });
  });

  describe("emitIntent", () => {
    it("carries the store's current revision and a monotonic per-surface seq", () => {
      const store = new UiDocumentStore("s");
      const button: UiNodeRecord = {
        ...leaf(0, "btn"),
        component: { type: "button", icon: "save", label: "Save" },
        bindings: [{ trigger: "activate", action: { scope: "app", name: "save", version: 1 }, args: null, capability: null }],
      };
      store.loadSnapshot(snapshot(0, [button]));
      const first = emitIntent(store, button, "activate");
      const second = emitIntent(store, button, "activate");
      expect(first?.revision).toBe(0);
      expect(first?.nodeKey).toBe("btn");
      expect(first?.action).toEqual({ scope: "app", name: "save", version: 1 });
      expect(second!.seq).toBe(first!.seq + 1n);
    });

    it("returns undefined when the node has no binding for that trigger", () => {
      const store = new UiDocumentStore("s");
      const record = leaf(0, "x");
      store.loadSnapshot(snapshot(0, [record]));
      expect(emitIntent(store, record, "commit")).toBeUndefined();
    });
  });
}
//#endregion 🧪️Tests
