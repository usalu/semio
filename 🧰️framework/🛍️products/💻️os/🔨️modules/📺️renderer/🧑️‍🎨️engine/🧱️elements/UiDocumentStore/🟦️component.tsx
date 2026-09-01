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

  //#region 🔢️ScalarSourceImports
  const { test } = import.meta.vitest;
  const assert: typeof import("node:assert/strict") = (await import("node:assert/strict")).default;
  const { Buffer } = await import("node:buffer");
  const { createHash } = await import("node:crypto");
  const { createRequire } = await import("node:module");
  const { default: scalarDeclarationJson } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🔣️.json");
  const { default: scalarDeclarationSchemaJson } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧬️schema/🔣️.json");
  const { default: scalarTestsJson } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧪️tests/🔣️.json");
  const { default: scalarTestsSchemaJson } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧪️tests/🧬️schema/🔣️.json");
  const { default: actorWordSchemaJson } = await import("../../../../../../../🔨️modules/🎭️actor/📄️page/🧬️schema.json");
  const { default: scalarReaderFixtureRaw } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json?raw");
  const { default: scalarPageFixtureRaw } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json?raw");
  //#endregion 🔢️ScalarSourceImports

  //#region 🧾️TypedWireTests
  describe("TypedWire", () => {
    //#region 🔢️ScalarSourceOracles
    {
      type AnySchema = import("ajv").AnySchema;
      //#region Scalar Declaration Types
      type Resources = { bytes: number; slots: number; owners: number };
      type Primitive = string | number | boolean | null;
      type ReceiptState = Record<string, Primitive | undefined> & {
       binding: string; receipt: string; sourceCursor: number; parsedCursor: number;
       serial: string; expectedSerial: string; resultSerial: string; latch: number | null;
       value: number | null; decoder: string; remaining: number; availability: string;
       fault: string | null; witness: string; receiptDependencies: boolean; readerTerminal: boolean;
       payloadLink: boolean; pending: boolean; bodyEmpty: boolean; recordPhase: string; cellPhase: string;
       used: number; childPrefix: number; aliasPrefix: number; aliasRequired: boolean; aliasTerminal: boolean;
       parentCharge: number; faultOwner: string | null;
      };
      type Grant = { maxItems: number; maxBytes: number };
      type ReceiptRow = {
       [key: string]: unknown;
       action: string; grant: Grant; expected: { kind: string; state?: Partial<ReceiptState> };
       byte?: number; resultSerial?: string; consumer?: string; faultAt?: string; fault?: string; availability?: string;
      };
      type Guard = { field: string; op: string; value: Primitive | string[] };
      type Effect = { field: string; op: string; value?: Primitive; from?: string };
      type Transition = { bytes: number; items: number; kind: string; guards: Guard[]; effects: Effect[] };
      type CloseStep = { phase: string; bytes: number; refund: number };
      type ConstructionPrefix = {
       completed: number; charged: number; originalParentCharge: number; shellInstalled: boolean;
       steps: CloseStep[]; turns: number; bytes: number;
      };
      type ChildClose = {
       path: string; sha256: string; grants: number[];
       prefixSums: { completed: number; remainingTurns: number; remainingBytes: number }[];
      };
      type ScalarDeclaration = {
       records: { name: string; bytes: number; fields: string[] }[];
       domain: Resources; intrinsic: { record: Resources; admission: Resources }; total: Resources;
       machine: { identity: Record<string, string>; initial: ReceiptState; transitions: Record<string, Transition> };
       closePlan: {
       reader: ChildClose; readerAlias: ChildClose; page: ChildClose;
       decoderOwned: { phase: string; bytes: number; items: number }[];
       fullReaderReady: { phase: string; bytes: number }[];
       constructionPrefixes: ConstructionPrefix[];
       };
       readerAdmission: number[]; readerAdmissionOrder: { phase: string; bytes: number }[];
       readerBinding: { phase: string; readerAdmissionIndex: number; bytes: number }[];
      };
      type ScalarVector = { id: string; profile: string; hex: string; accepted: boolean; expected: string | null };
      type DeclarationNegative = { id: string; mutation: string; target?: string; accepted?: boolean; source?: string; profile?: string };
      type AliasExpected = {
       [key: string]: unknown;
       completed: number; initialPayloadScalar: string | null; initialPendingEntry: string | null;
       finalPayloadScalar: null; finalPendingEntry: null; finalPendingRecord: null; finalPendingCell: null;
       finalCellResult: null; finalRecordShell: null; finalWitnessState: null;
       finalUsed: number; parentCharge: number; unlinkTurns: number; workBytes: number; turns: number;
      };
      type ScalarFixture = {
       vectors: ScalarVector[];
       windowPlans: { prefix: { hex: string; repeat: number }[]; tail: { hex: string }; crosses: number }[];
       declarationNegatives: DeclarationNegative[];
       ownershipTraces: { id: string; initial: Partial<ReceiptState>; steps: ReceiptRow[] }[];
       closeChecks: { decoderOnlyTurns: number; decoderOnlyBytes: number; readerReadyTurns: number; readerReadyBytes: number; aliasExtraBytes: number; pageExtraBytes: number };
       maintenanceCases: { kind: string; items: number; bytes: number; maxItems: number; maxBytes: number; expected: string; observe: boolean }[];
       constructionAliasExpected: AliasExpected[];
       constructionAliasCases: { id: string; phase: string; mutation: Record<string, Primitive | undefined>; grant: number; expected: string }[];
      };
      type ReceiptResult = { state: ReceiptState; kind: string; bytes: number; items: number };
      type ReceiptChange = (state: ReceiptState, recipe: (draft: ReceiptState) => void) => ReceiptState;
      type ArithmeticCloseState = { used: number; parent: number; fault: object | null };
      type AliasState = {
       [key: string]: unknown;
       used: number; parentCharge: number; payloadScalar: OriginalIdentity | null;
       pendingEntry: OriginalIdentity | null; pendingCell: OriginalIdentity | null; pendingRecord: OriginalIdentity | null;
       stateCell: OriginalIdentity | null; stateRecord: OriginalIdentity | null; witnessState: OriginalIdentity | null;
       recordShell: OriginalIdentity | null; cellResult: OriginalIdentity | null;
       bodyEmpty: boolean; domainProof: boolean; aliasPhase: string; fault: object | null; faultOwner: string | null;
      };
      class OriginalIdentity { constructor(readonly id: string) { Object.freeze(this); } }
      //#endregion
      const importedLeb: unknown = createRequire(import.meta.url)("@webassemblyjs/leb128/lib/leb.js");
      assert(importedLeb!==null&&typeof importedLeb==="object");
      const lebModule: unknown = "default" in importedLeb ? importedLeb.default : importedLeb;
      assert(lebModule!==null&&typeof lebModule==="object"&&"decodeUIntBuffer" in lebModule&&"encodeUIntBuffer" in lebModule);
      assert(typeof lebModule.decodeUIntBuffer==="function"&&typeof lebModule.encodeUIntBuffer==="function");
      const leb = {decodeUIntBuffer: lebModule.decodeUIntBuffer, encodeUIntBuffer: lebModule.encodeUIntBuffer};
      //#region Scalar Input Binding
      const scalarInputAjv = new Ajv({strict: true, allErrors: true});
      scalarInputAjv.addSchema(actorWordSchemaJson);
      const scalarDeclarationCheck = scalarInputAjv.compile<ScalarDeclaration>(scalarDeclarationSchemaJson);
      const scalarFixtureCheck = scalarInputAjv.compile<ScalarFixture>(scalarTestsSchemaJson);
      assert(scalarDeclarationCheck(scalarDeclarationJson), JSON.stringify(scalarDeclarationCheck.errors));
      assert(scalarFixtureCheck(scalarTestsJson), JSON.stringify(scalarFixtureCheck.errors));
      const scalarChildSources: Readonly<Record<string, Uint8Array>> = {
       "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json": new TextEncoder().encode(scalarReaderFixtureRaw),
       "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json": new TextEncoder().encode(scalarPageFixtureRaw),
      };
      //#endregion
      //#region Scalar Source Oracles
      function validateScalarDeclarationCold(c: ScalarDeclaration,f: ScalarFixture,contractSchema: AnySchema,fixtureSchema: AnySchema,wordSchema: AnySchema) {
        //#region Values
        const ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(wordSchema);for(const [schema,value] of [[contractSchema,c],[fixtureSchema,f]]){const check=ajv.compile(schema);assert(check(value),JSON.stringify(check.errors));} for(const r of c.records)assert.equal(r.bytes,16+8*r.fields.length);assert.equal(new Set(c.records.map(r=>r.name)).size,5);const sum=c.records.reduce((a,r)=>a+r.bytes,0);assert.equal(sum,c.domain.bytes);assert.deepEqual(c.total,{bytes:sum+c.intrinsic.record.bytes+c.intrinsic.admission.bytes,slots:5+3+6,owners:5+3+6}); const oracle=(v: ScalarVector): string=>{const bytes=Buffer.from(v.hex,"hex");if(v.profile==="ui-value-tag"){if(bytes.length!==1||![1,2,5,6,7,12,16,18].includes(bytes[0]))throw Error("tag");return String(bytes[0]);}if(v.profile==="utf8-codepoint"){const decoded=new TextDecoder("utf-8",{fatal:true,ignoreBOM:true}).decode(bytes);const points=Array.from(decoded);if(points.length!==1)throw Error("extent");return String(points[0].codePointAt(0));}if(v.profile==="ui-f64"){if(bytes.length!==8)throw Error("extent");const n=bytes.readDoubleLE();if(!Number.isFinite(n)||Object.is(n,-0))throw Error("domain");return String(n);}if(!bytes.length||bytes.length>10||(bytes[bytes.length-1]&128))throw Error("extent");const decoded: unknown=leb.decodeUIntBuffer(bytes,0);assert(decoded!==null&&typeof decoded==="object"&&"nextIndex" in decoded&&"value" in decoded);assert(decoded.value instanceof Uint8Array);if(decoded.nextIndex!==bytes.length)throw Error("extent");const raw=Buffer.from(decoded.value);if(raw.length>8&&raw.subarray(8).some(x=>x!==0))throw Error("u64");const fixed=Buffer.alloc(8);raw.copy(fixed,0,0,8);const n=fixed.readBigUInt64LE();const encodedValue: unknown=leb.encodeUIntBuffer(fixed);assert(encodedValue instanceof Uint8Array);const encoded=Buffer.from(encodedValue);if(!encoded.equals(bytes))throw Error("canonical");if(v.profile==="natural-safe53"&&n>BigInt(Number.MAX_SAFE_INTEGER))throw Error("safe53");return String(n);}; let accepted=0,rejected=0;for(const v of f.vectors){let actual: string | null=null;try{actual=oracle(v);}catch{}assert.equal(actual,v.expected,v.id);assert.equal(actual!==null,v.accepted,v.id+" acceptance");v.accepted?accepted++:rejected++;} for(const w of f.windowPlans){const start=w.prefix.reduce((n,p)=>n+p.hex.length/2*p.repeat,0);assert.equal(start,255);assert(start+w.tail.hex.length/2>w.crosses);} const before: {used: Resources; held: boolean; fault?: string}={used:{bytes:0,slots:0,owners:0},held:false};const admitted=produce(before,d=>{for(const k of (["bytes","slots","owners"] as const))d.used[k]+=c.total[k];d.held=true;});const faulted=produce(admitted,d=>{d.fault="exact-root";});assert.deepEqual(faulted.used,admitted.used);const closed=produce(admitted,d=>{d.held=false;for(const k of (["bytes","slots","owners"] as const))d.used[k]-=c.total[k];});assert.deepEqual(closed,before);const census={strictAjv:2,recordCensus:c.records.length,vectors:f.vectors.length,accepted,rejected,plannedWindowShapes:f.windowPlans.length,immerChargeTransitions:3,runtimeDecoderExecuted:false};console.log("[DEBUG] "+JSON.stringify(census));
        //#endregion
        //#region Declaration Negatives
        { const s=fixtureSchema;
        const ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(wordSchema);const shape=ajv.compile(s);const valid=(x: ScalarFixture)=>shape(x)&&new Set(x.vectors.map(v=>v.id)).size===x.vectors.length&&new Set(x.declarationNegatives.map(v=>v.id)).size===x.declarationNegatives.length;assert(valid(f),JSON.stringify(shape.errors));for(const row of f.declarationNegatives){const x=structuredClone(f);if(row.mutation==="flip-accepted"){const target=x.vectors.find(v=>v.id===row.target);assert(target);assert(typeof row.accepted==="boolean");target.accepted=row.accepted;}if(row.mutation==="duplicate-id"){const target=x.vectors.find(v=>v.id===row.target);assert(target);assert(typeof row.source==="string");target.id=row.source;}if(row.mutation==="remove-profile")x.vectors=x.vectors.filter(v=>v.profile!==row.profile);assert.equal(valid(x),false,row.id);console.log("[DEBUG] "+row.id+" accepted=false");}assert.equal(JSON.stringify(f.vectors),"[{\"id\":\"tag-01\",\"profile\":\"ui-value-tag\",\"hex\":\"01\",\"accepted\":true,\"expected\":\"1\"},{\"id\":\"tag-02\",\"profile\":\"ui-value-tag\",\"hex\":\"02\",\"accepted\":true,\"expected\":\"2\"},{\"id\":\"tag-05\",\"profile\":\"ui-value-tag\",\"hex\":\"05\",\"accepted\":true,\"expected\":\"5\"},{\"id\":\"tag-06\",\"profile\":\"ui-value-tag\",\"hex\":\"06\",\"accepted\":true,\"expected\":\"6\"},{\"id\":\"tag-07\",\"profile\":\"ui-value-tag\",\"hex\":\"07\",\"accepted\":true,\"expected\":\"7\"},{\"id\":\"tag-0c\",\"profile\":\"ui-value-tag\",\"hex\":\"0c\",\"accepted\":true,\"expected\":\"12\"},{\"id\":\"tag-10\",\"profile\":\"ui-value-tag\",\"hex\":\"10\",\"accepted\":true,\"expected\":\"16\"},{\"id\":\"tag-12\",\"profile\":\"ui-value-tag\",\"hex\":\"12\",\"accepted\":true,\"expected\":\"18\"},{\"id\":\"invalid-tag-00\",\"profile\":\"ui-value-tag\",\"hex\":\"00\",\"accepted\":false,\"expected\":null},{\"id\":\"invalid-tag-11\",\"profile\":\"ui-value-tag\",\"hex\":\"11\",\"accepted\":false,\"expected\":null},{\"id\":\"invalid-tag-13\",\"profile\":\"ui-value-tag\",\"hex\":\"13\",\"accepted\":false,\"expected\":null},{\"id\":\"invalid-tag-ff\",\"profile\":\"ui-value-tag\",\"hex\":\"ff\",\"accepted\":false,\"expected\":null},{\"id\":\"u64-zero\",\"profile\":\"natural-u64\",\"hex\":\"00\",\"accepted\":true,\"expected\":\"0\"},{\"id\":\"u64-127\",\"profile\":\"natural-u64\",\"hex\":\"7f\",\"accepted\":true,\"expected\":\"127\"},{\"id\":\"u64-128\",\"profile\":\"natural-u64\",\"hex\":\"8001\",\"accepted\":true,\"expected\":\"128\"},{\"id\":\"u64-max\",\"profile\":\"natural-u64\",\"hex\":\"ffffffffffffffffff01\",\"accepted\":true,\"expected\":\"18446744073709551615\"},{\"id\":\"u64-overflow\",\"profile\":\"natural-u64\",\"hex\":\"ffffffffffffffffff02\",\"accepted\":false,\"expected\":null},{\"id\":\"u64-redundant\",\"profile\":\"natural-u64\",\"hex\":\"8000\",\"accepted\":false,\"expected\":null},{\"id\":\"u64-truncated\",\"profile\":\"natural-u64\",\"hex\":\"80\",\"accepted\":false,\"expected\":null},{\"id\":\"safe53-max\",\"profile\":\"natural-safe53\",\"hex\":\"ffffffffffffff0f\",\"accepted\":true,\"expected\":\"9007199254740991\"},{\"id\":\"safe53-overflow\",\"profile\":\"natural-safe53\",\"hex\":\"8080808080808010\",\"accepted\":false,\"expected\":null},{\"id\":\"utf8-nul\",\"profile\":\"utf8-codepoint\",\"hex\":\"00\",\"accepted\":true,\"expected\":\"0\"},{\"id\":\"utf8-ascii\",\"profile\":\"utf8-codepoint\",\"hex\":\"7f\",\"accepted\":true,\"expected\":\"127\"},{\"id\":\"utf8-two\",\"profile\":\"utf8-codepoint\",\"hex\":\"c280\",\"accepted\":true,\"expected\":\"128\"},{\"id\":\"utf8-euro\",\"profile\":\"utf8-codepoint\",\"hex\":\"e282ac\",\"accepted\":true,\"expected\":\"8364\"},{\"id\":\"utf8-emoji\",\"profile\":\"utf8-codepoint\",\"hex\":\"f09f9880\",\"accepted\":true,\"expected\":\"128512\"},{\"id\":\"utf8-max\",\"profile\":\"utf8-codepoint\",\"hex\":\"f48fbfbf\",\"accepted\":true,\"expected\":\"1114111\"},{\"id\":\"utf8-bom\",\"profile\":\"utf8-codepoint\",\"hex\":\"efbbbf\",\"accepted\":true,\"expected\":\"65279\"},{\"id\":\"utf8-overlong\",\"profile\":\"utf8-codepoint\",\"hex\":\"c080\",\"accepted\":false,\"expected\":null},{\"id\":\"utf8-surrogate\",\"profile\":\"utf8-codepoint\",\"hex\":\"eda080\",\"accepted\":false,\"expected\":null},{\"id\":\"utf8-overmax\",\"profile\":\"utf8-codepoint\",\"hex\":\"f4908080\",\"accepted\":false,\"expected\":null},{\"id\":\"utf8-continuation\",\"profile\":\"utf8-codepoint\",\"hex\":\"80\",\"accepted\":false,\"expected\":null},{\"id\":\"utf8-truncated\",\"profile\":\"utf8-codepoint\",\"hex\":\"e282\",\"accepted\":false,\"expected\":null},{\"id\":\"f64-zero\",\"profile\":\"ui-f64\",\"hex\":\"0000000000000000\",\"accepted\":true,\"expected\":\"0\"},{\"id\":\"f64-one\",\"profile\":\"ui-f64\",\"hex\":\"000000000000f03f\",\"accepted\":true,\"expected\":\"1\"},{\"id\":\"f64-negative-one\",\"profile\":\"ui-f64\",\"hex\":\"000000000000f0bf\",\"accepted\":true,\"expected\":\"-1\"},{\"id\":\"f64-min-subnormal\",\"profile\":\"ui-f64\",\"hex\":\"0100000000000000\",\"accepted\":true,\"expected\":\"5e-324\"},{\"id\":\"f64-max\",\"profile\":\"ui-f64\",\"hex\":\"ffffffffffffef7f\",\"accepted\":true,\"expected\":\"1.7976931348623157e+308\"},{\"id\":\"f64-negative-zero\",\"profile\":\"ui-f64\",\"hex\":\"0000000000000080\",\"accepted\":false,\"expected\":null},{\"id\":\"f64-infinity\",\"profile\":\"ui-f64\",\"hex\":\"000000000000f07f\",\"accepted\":false,\"expected\":null},{\"id\":\"f64-negative-infinity\",\"profile\":\"ui-f64\",\"hex\":\"000000000000f0ff\",\"accepted\":false,\"expected\":null},{\"id\":\"f64-nan\",\"profile\":\"ui-f64\",\"hex\":\"010000000000f07f\",\"accepted\":false,\"expected\":null},{\"id\":\"f64-truncated\",\"profile\":\"ui-f64\",\"hex\":\"00000000000000\",\"accepted\":false,\"expected\":null}]");console.log("[DEBUG] schemaNegative=8 preservedValues=43 semanticUniqueId=1 runtimeDecoderExecuted=false");for(const word of["18446744073709551616","01"]){const x=structuredClone(f);x.ownershipTraces[0].initial.serial=word;assert.equal(valid(x),false,word);}console.log("[DEBUG] serialSchemaNegatives=2");
        }
        //#endregion
      }
      function runScalarReceiptModelCold(c: ScalarDeclaration,f: ScalarFixture,contractSchema: AnySchema,fixtureSchema: AnySchema,wordSchema: AnySchema) {
        const ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(wordSchema);for(const [p,v]of[[contractSchema,c],[fixtureSchema,f]]){const check=ajv.compile(p);assert(check(v),JSON.stringify(check.errors));}assert.equal(new Set(f.ownershipTraces.map(x=>x.id)).size,f.ownershipTraces.length);const guard=(s: ReceiptState,r: ReceiptRow,g: Guard): boolean=>{const a=s[g.field],b=g.value;if(g.op==="eq")return a===b;if(g.op==="in"){assert(Array.isArray(b));assert(typeof a==="string");return b.includes(a);}if(g.op==="gt"){assert(typeof a==="number"&&typeof b==="number");return a>b;}if(g.op==="word-lt"){assert(typeof a==="string"&&typeof b==="string");return BigInt(a)<BigInt(b);}if(g.op==="same"){assert(typeof b==="string");return a===s[b];}if(g.op==="event-eq"){assert(typeof b==="string");return a===r[b];}throw Error("Unknown guard "+g.op);};const write=(d: ReceiptState,r: ReceiptRow,e: Effect): void=>{
      if(e.op==="set"){assert(e.value!==undefined);d[e.field]=e.value;}
      else if(e.op==="add"){const previous=d[e.field];assert(typeof previous==="number"&&typeof e.value==="number");d[e.field]=previous+e.value;}
      else if(e.op==="copy"){assert(typeof e.from==="string");const value=e.from.startsWith("$")?r[e.from.slice(1)]:d[e.from];assert(value===null||typeof value==="string"||typeof value==="number"||typeof value==="boolean");d[e.field]=value;}
      else if(e.op==="word-inc"){const previous=d[e.field];assert(typeof previous==="string");d[e.field]=String(BigInt(previous)+1n);}
      else throw Error("Unknown effect "+e.op);};const plain: ReceiptChange=(s,fn)=>{const d={...s};fn(d);return d;};const apply=(s: ReceiptState,r: ReceiptRow,change: ReceiptChange): ReceiptResult=>{const t=c.machine.transitions[r.action];assert(t);if(Object.entries(c.machine.identity).some(([k,v])=>r[k]!==undefined&&r[k]!==v)||s.fault!==null||!t.guards.every(g=>guard(s,r,g)))return{state:s,kind:"rejected",bytes:0,items:0};if(r.grant.maxItems<t.items||r.grant.maxBytes<t.bytes)return{state:s,kind:"blocked",bytes:0,items:0};if(r.faultAt==="before")return{state:change(s,d=>{assert(typeof r.fault==="string");d.fault=r.fault;d.faultOwner=d.used>0?"decoder":"parent";}),kind:"rejected",bytes:0,items:0};const next=change(s,d=>{for(const e of t.effects)write(d,r,e);if(r.faultAt==="after"){assert(typeof r.fault==="string");d.fault=r.fault;d.faultOwner=d.used>0?"decoder":"parent";}});return{state:next,kind:r.faultAt==="after"?"rejected":t.kind,bytes:t.bytes,items:t.items};};let rows=0;for(const trace of f.ownershipTraces){let state={...c.machine.initial,...trace.initial};for(const [i,r]of trace.steps.entries()){const before={...state};const actual=apply(state,r,plain),oracle=apply(state,r,produce);assert.deepEqual(actual,oracle,trace.id+"/"+i+" Immer");assert.equal(actual.kind,r.expected.kind,trace.id+"/"+i+" "+r.action);for(const[k,v]of Object.entries(r.expected.state??{}))assert.deepEqual(actual.state[k],v,trace.id+"/"+i+" "+k);assert(actual.bytes<=r.grant.maxBytes&&actual.items<=r.grant.maxItems);assert(actual.state.sourceCursor>=state.sourceCursor&&actual.state.parsedCursor>=state.parsedCursor);assert(actual.state.parsedCursor<=actual.state.sourceCursor);assert(actual.state.used>=0);if(actual.kind==="blocked")assert.deepEqual(actual.state,before);state=actual.state;rows++;}}console.log("[DEBUG] closedTraces="+f.ownershipTraces.length+" rows="+rows+" strictAjv=2 ImmerTransitions="+rows+" runtimeDecoderExecuted=false");
      }
      function runScalarCloseModelCold(c: ScalarDeclaration,f: ScalarFixture,childSources: Readonly<Record<string, Uint8Array>>) {
        let suffixes=0;for(const name of (["reader","readerAlias","page"] as const)){const x=c.closePlan[name];const bytes=childSources[x.path];assert(bytes,name+" original source");assert.equal(createHash("sha256").update(bytes).digest("hex"),x.sha256,name+" source");const actual: unknown=JSON.parse(new TextDecoder().decode(bytes));assert(actual!==null&&typeof actual==="object");assert("closing" in actual&&actual.closing!==null&&typeof actual.closing==="object"&&"grants" in actual.closing);assert("aliasCloseGrants" in actual||name!=="readerAlias");assert.deepEqual(x.grants,name==="readerAlias"&&"aliasCloseGrants" in actual?actual.aliasCloseGrants:actual.closing.grants);for(const p of x.prefixSums){assert.equal(p.remainingTurns,x.grants.length-p.completed);assert.equal(p.remainingBytes,x.grants.slice(p.completed).reduce((a,b)=>a+b,0));suffixes++;}} assert.equal(c.closePlan.decoderOwned.length,f.closeChecks.decoderOnlyTurns);assert.equal(c.closePlan.decoderOwned.reduce((n,s)=>n+s.bytes,0),f.closeChecks.decoderOnlyBytes);assert.equal(c.closePlan.fullReaderReady.length,f.closeChecks.readerReadyTurns);assert.equal(c.closePlan.fullReaderReady.reduce((n,s)=>n+s.bytes,0),f.closeChecks.readerReadyBytes);let constructorRows=0,faultRows=0,shortRows=0; for(const p of c.closePlan.constructionPrefixes){assert.equal(p.completed>=5?992:p.completed>0?296:0,p.charged);assert.equal(p.steps.length,p.turns);assert.equal(p.steps.reduce((n,s)=>n+s.bytes,0),p.bytes);let state: ArithmeticCloseState={used:p.charged,parent:p.originalParentCharge,fault:null};for(const [i,s]of p.steps.entries()){assert(s.bytes>0&&s.bytes<=4096);for(const grant of[0,s.bytes-1]){const before={...state};const result=grant<s.bytes?{kind:"blocked",state}:null;assert(result);assert.equal(result.kind,"blocked");assert.deepEqual(result.state,before);shortRows++;}for(const at of["before","after"]){const original=Object.freeze({id:p.completed+"/"+i+"/"+at});const faulted=produce(state,d=>{if(at==="after")d.used-=s.refund;d.fault=original;});assert.equal(faulted.fault,original);assert(faulted.used>=0&&faulted.used+faulted.parent>0);assert.equal(faulted.parent,p.originalParentCharge);faultRows++;}const next=produce(state,d=>{d.used-=s.refund;});assert(next.used>=0);state=next;constructorRows++;}assert.equal(state.used,0);assert.equal(state.parent,880);} const refund=[{bytes:688,slots:8,owners:8},{bytes:8,slots:0,owners:0},{bytes:296,slots:6,owners:6}].reduce((a,b)=>({bytes:a.bytes+b.bytes,slots:a.slots+b.slots,owners:a.owners+b.owners}),{bytes:0,slots:0,owners:0});assert.deepEqual(refund,c.total); for(const row of f.maintenanceCases){const original={receipt:"prepared",cursor:0,serial:"1",latch:null};const bad=row.items>row.maxItems||row.bytes>row.maxBytes;const result={kind:bad?"rejected":row.kind==="complete"?"pending":row.kind,items:row.items,bytes:row.bytes};assert.equal(result.kind,row.expected);assert.equal(result.bytes,row.bytes);assert.equal(result.items,row.items);assert.deepEqual(original,{receipt:"prepared",cursor:0,serial:"1",latch:null});assert.equal(!bad&&row.kind==="complete",row.observe);for(const grant of[0,63,64])assert.equal(row.observe&&grant>=64,row.observe?grant===64:false);} console.log("[DEBUG] "+JSON.stringify({childSuffixes:suffixes,constructionPrefixes:c.closePlan.constructionPrefixes.length,constructorRows,faultRows,shortRows,maintenanceCases:f.maintenanceCases.length,decoderClose:f.closeChecks.decoderOnlyBytes,readerReadyClose:f.closeChecks.readerReadyBytes,aliasExtra:f.closeChecks.aliasExtraBytes,pageExtra:f.closeChecks.pageExtraBytes,privateRuntimeExecuted:false}));assert.equal(c.readerAdmissionOrder.length,16);assert.deepEqual(c.readerAdmissionOrder.map(x=>x.bytes),c.readerAdmission);assert(c.readerBinding[0].readerAdmissionIndex<c.readerAdmissionOrder.findIndex(x=>x.phase==="reader-finalize"));assert(c.readerBinding[1].readerAdmissionIndex<c.readerAdmissionOrder.findIndex(x=>x.phase==="reader-publication"));console.log("[DEBUG] exactReaderConstructionOrder=16 bindingBeforeFinalizer=true");
      let exactRows=0,aliasNegativeRows=0,aliasFaultRows=0,aliasShortRows=0,aliasReplayRows=0;
      const identities=Object.fromEntries(["original-state","foreign-state","original-cell","foreign-cell","original-record","foreign-record"].map(id=>[id,new OriginalIdentity(id)]));
      const S=identities["original-state"],C=identities["original-cell"],R=identities["original-record"];
      const decode=(v: Primitive | undefined): Primitive | OriginalIdentity | undefined=>typeof v==="string"&&v in identities?identities[v]:v;
      const initial=(p: ConstructionPrefix): AliasState=>({used:p.charged,parentCharge:880,payloadScalar:p.completed>=7?S:null,pendingEntry:p.completed>=7&&p.completed<13?S:null,pendingCell:p.completed>0&&p.completed<13?C:null,pendingRecord:p.completed>=5&&p.completed<13?R:null,stateCell:p.completed>=7?C:null,stateRecord:p.completed>=7?R:null,witnessState:p.completed>=11?S:null,recordShell:p.shellInstalled?S:null,cellResult:p.completed>=5?R:null,bodyEmpty:p.completed<7,domainProof:false,aliasPhase:p.completed>=7?"linked":"never-installed",fault:null,faultOwner:null});
      const guard=(s: AliasState,p: ConstructionPrefix,step: CloseStep,grant: number): string=>{
       if(s.fault!==null)return "rejected";if(grant<step.bytes)return "blocked";
       if(step.phase==="capture-original-admission-frontier"&&p.completed>=7&&!(s.payloadScalar===S&&(s.pendingEntry===null||s.pendingEntry===S)&&s.stateCell===C&&s.stateRecord===R))return "rejected";
       if(step.phase==="clear-original-fixed-state"&&!(s.pendingCell===C&&s.pendingRecord===R&&s.payloadScalar===S&&s.pendingEntry===S))return "rejected";
       if(step.phase==="unlink-original-state-aliases"&&!(s.aliasPhase==="captured"&&s.bodyEmpty&&s.payloadScalar===S&&s.pendingEntry===S&&s.pendingCell===C&&s.pendingRecord===R&&(!p.shellInstalled||(s.domainProof&&s.witnessState===S))))return "rejected";
       if(step.phase==="record-close"&&!(s.pendingRecord===R&&s.pendingCell===C&&s.recordShell===null&&s.bodyEmpty&&s.payloadScalar===null&&s.pendingEntry===null&&(p.completed<7||s.aliasPhase==="unlinked")))return "rejected";
       return "pending";
      };
      const effects=(d: AliasState,p: ConstructionPrefix,step: CloseStep): void=>{
       switch(step.phase){
       case "capture-original-admission-frontier":d.pendingCell=p.completed>0?C:null;d.pendingRecord=p.completed>=5?R:null;if(p.completed>=7){d.pendingEntry=S;d.aliasPhase="captured";}break;
       case "install-original-preadmitted-witness":d.witnessState=S;break;
       case "clear-original-fixed-state":d.bodyEmpty=true;d.stateCell=null;d.stateRecord=null;break;
       case "original-domain-proof":assert.equal(d.witnessState,S);d.domainProof=true;break;
       case "unlink-original-state-aliases":d.payloadScalar=null;d.pendingEntry=null;d.aliasPhase="unlinked";break;
       case "exact-original-shell-detach":assert.equal(d.witnessState,S);assert.equal(d.recordShell,S);assert(d.domainProof);d.recordShell=null;break;
       case "resource-result-alias":d.cellResult=null;break;
       case "parent-observe":case "original-empty-slot-observation":d.pendingCell=null;d.pendingRecord=null;d.witnessState=null;break;
       }
       d.used-=step.refund;
      };
      for(const p of c.closePlan.constructionPrefixes){
       const expected=f.constructionAliasExpected.find(x=>x.completed===p.completed);assert(expected);let state=initial(p),unlink=0,work=0;const frontiers=new Map<string, AliasState>();
       assert.equal(state.payloadScalar,decode(expected.initialPayloadScalar));assert.equal(state.pendingEntry,decode(expected.initialPendingEntry));
       for(const step of p.steps){
        frontiers.set(step.phase,{...state});assert.equal(guard(state,p,step,step.bytes),"pending",p.completed+"/"+step.phase);
        for(const grant of[0,step.bytes-1]){assert.equal(guard(state,p,step,grant),"blocked");aliasShortRows++;}
        const plain={...state};effects(plain,p,step);const next=produce(state,d=>{effects(d,p,step);});assert.deepEqual(next,plain);assert(next.used>=0);
        for(const at of["before","after"]){const original=Object.freeze({prefix:p.completed,phase:step.phase,at});const faulted=produce(at==="before"?state:next,d=>{d.fault=original;d.faultOwner=d.used>0?"original-pending-or-cell":"original-payload";});assert.equal(faulted.fault,original);assert.equal(faulted.parentCharge,880);assert.equal(guard(faulted,p,step,step.bytes),"rejected");assert.equal(faulted.payloadScalar,(at==="before"?state:next).payloadScalar);assert.equal(faulted.pendingEntry,(at==="before"?state:next).pendingEntry);if(step.phase==="unlink-original-state-aliases"){assert.equal(faulted.pendingCell,C);assert.equal(faulted.pendingRecord,R);assert.equal(faulted.aliasPhase,at==="before"?"captured":"unlinked");}aliasFaultRows++;}
        if(step.phase==="unlink-original-state-aliases"){unlink++;assert.equal(guard(next,p,step,step.bytes),"rejected");aliasReplayRows++;}
        if(step.refund>0){assert.equal(state.payloadScalar,null);assert.equal(state.pendingEntry,null);}
        state=next;work+=step.bytes;exactRows++;
       }
       assert.equal(state.used,expected.finalUsed);assert.equal(state.parentCharge,expected.parentCharge);assert.equal(work,expected.workBytes);assert.equal(unlink,expected.unlinkTurns);assert.equal(p.steps.length,expected.turns);
       for(const key of["PayloadScalar","PendingEntry","PendingRecord","PendingCell","CellResult","RecordShell","WitnessState"])assert.equal(state[key[0].toLowerCase()+key.slice(1)],expected["final"+key],p.completed+"/"+key);
       if(p.completed>=7){
        for(const row of f.constructionAliasCases){
         if(row.id==="missing-proof-unlink"&&!p.shellInstalled)continue;
         const base=frontiers.get(row.phase);assert(base);const bad={...base,...Object.fromEntries(Object.entries(row.mutation).map(([k,v])=>[k,decode(v)]))},before={...bad};const step=p.steps.find(x=>x.phase===row.phase);assert(step);
         assert.equal(guard(bad,p,step,row.grant),row.expected,p.completed+"/"+row.id);assert.deepEqual(bad,before);assert.equal(bad.used,base.used);aliasNegativeRows++;
        }
        let omitted=initial(p),refused=0;
        for(const step of p.steps.filter(x=>x.phase!=="unlink-original-state-aliases")){if(guard(omitted,p,step,step.bytes)!=="pending"){assert.equal(step.phase,"record-close");assert.equal(omitted.used,992);assert.equal(omitted.payloadScalar,S);assert.equal(omitted.pendingEntry,S);refused++;break;}effects(omitted,p,step);}
        assert.equal(refused,1);
       }
      }
      assert.equal(new Set(f.constructionAliasCases.map(x=>x.id)).size,f.constructionAliasCases.length);
      console.log("[DEBUG] "+JSON.stringify({exactAliasPrefixes:14,exactAliasRows:exactRows,aliasNegativeRows,aliasFaultRows,aliasShortRows,aliasReplayRows,omittedUnlinkRefusals:7,constructorChargeUnchanged:992,originalParentCharge:880,runtimeExecuted:false}));
      
      }
      //#endregion
      
      //#region Scalar Tests
      test("OwnedResidentScalarDeclaration", () => {
        validateScalarDeclarationCold(scalarDeclarationJson, scalarTestsJson, scalarDeclarationSchemaJson, scalarTestsSchemaJson, actorWordSchemaJson);
        //#region Canonical Provenance Extension
        const provenanceAjv = new Ajv({strict: true, allErrors: true});
        provenanceAjv.addSchema(actorWordSchemaJson);
        const declaration = provenanceAjv.compile(scalarDeclarationSchemaJson);
        const fixture = provenanceAjv.compile(scalarTestsSchemaJson);
        assert.equal(declaration(scalarTestsJson), false, "fixture is not a declaration");
        assert.equal(fixture(scalarDeclarationJson), false, "declaration is not a fixture");
        for (const [check, value] of [[declaration, scalarDeclarationJson], [fixture, scalarTestsJson]] as const) {
          assert.equal(check({...value, $id: "foreign.scalar.source"}), false, "foreign provenance");
          assert.equal(check({...value, status: "runtime-mounted"}), false, "unissued runtime readiness");
        }
        assert.equal(scalarDeclarationJson.registeredTestProposal.status, "source-oracle-contract");
        assert.equal(scalarDeclarationJson.status, "runtime-unmounted-source-declaration");
        assert.equal(scalarTestsJson.status, "runtime-unmounted-source-oracle");
        console.log("[DEBUG] canonicalCrossPairRejections=2 provenanceRejections=4 runtimeMounted=false");
        //#endregion
      });
      test("OwnedResidentScalarReceiptModel", () => {
        runScalarReceiptModelCold(scalarDeclarationJson, scalarTestsJson, scalarDeclarationSchemaJson, scalarTestsSchemaJson, actorWordSchemaJson);
      });
      test("OwnedResidentScalarCloseModel", () => {
        runScalarCloseModelCold(scalarDeclarationJson, scalarTestsJson, scalarChildSources);
      });
      //#endregion
    }
    //#endregion 🔢️ScalarSourceOracles
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

    //#region 🪪️NativeInstanceFixture
    async function nativeInstanceFixture(sharedLedger?: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentLedger) {
      const { ShardClient } = await import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts"); const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🟦️component.ts");
      const { encodeActorUiPatchReceipt } = await import("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️component.ts"); let nativePatchSequence = 50n;
      class Worker {
        onmessage: ((event: { readonly data: unknown }) => void) | null = null;
        onerror: ((event: unknown) => void) | null = null;
        readonly sent: unknown[] = [];
        refuse = false;
        postMessage(value: unknown): void { if (this.refuse) { this.refuse = false; throw new Error("Fixture transport refusal"); } this.sent.push(value); }
        terminate(): void {}
      }
      const { OwnedResidentLedger } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: residentFixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧪️fixture.json");
      const residentLedger = sharedLedger ?? new OwnedResidentLedger(residentFixture.compositionCapacity); const worker = new Worker(); const client = new ShardClient({ residentLedger, shardCount: 1, createWorker: () => worker }); const budget = { fuel: 1000, wallMs: 4, memoryBytes: 1 << 20, uiNodes: 128, mailboxLen: 16, maxEffects: 8, maxPatchBytes: 1 << 16 };
      async function answer<T>(pending: Promise<T>, value: unknown): Promise<T> { const sent = worker.sent.at(-1); if (!sent || typeof sent !== "object" || !("requestId" in sent) || typeof sent.requestId !== "string") throw new Error("Missing exact fixture request"); worker.onmessage?.({ data: { kind: "result", requestId: sent.requestId, ok: true, value } }); return pending; }
      await answer(client.activate("native-ui", "https://fixture.invalid/native-ui.js", [], budget), undefined); const lease = client.captureInstanceLifecycle("native-ui", 7);
      const captured = { kind: "captured" as const, lifetime: { activationGeneration: lease.activation.activationGeneration, instanceId: 7, guestLifetime: 13n }, requestSequence: lease.openRequest.requestSequence };
      await answer(lease.open({ appId: "fixture", actor: "fixture", config: [], assets: [], capabilities: [], quotas: [] }, budget), { lifecycleReceipt: encodeActorInstanceLifecycle(captured), status: { tag: "idle" } }); await answer(lease.acknowledge(captured, budget), { status: { tag: "idle" } }); const capturedLifetime = lease.lifetime; if (!capturedLifetime) throw new Error("Native fixture did not capture lifetime"); const lifetime = capturedLifetime;
      async function source(operations: readonly unknown[], baseRevision = 0, revision = 1) { const result = { status: { tag: "idle" }, uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime, patchSequence: ++nativePatchSequence }), uiPatches: [{ surface: { instance: 7, surface: "window" }, baseRevision: BigInt(baseRevision), revision: BigInt(revision), ops: operations }] }; await answer(lease.poll(budget), result); return lease.captureUiPatchAuthority(result, 0); }
      return { client, worker, lease, lifetime, activation: lease.activation, budget, answer, source, residentLedger };
    }
    //#endregion 🪪️NativeInstanceFixture

    //#region 📄️PagedPayloadSchemaTests
    async function nativePagedFieldFixture(fixtureIndex = 2, sharedLedger?: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentLedger) {
      const native = await nativeInstanceFixture(sharedLedger); const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { OwnedKernelReturnContent } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { encodeActorReturnResult } = await import("../../../../../../../🔨️modules/🎭️actor/📤️return/🟦️component.ts"); const { createActorBytePage } = await import("../../../../../../../🔨️modules/🎭️actor/📄️page/🟦️component.ts"); const { encodeActorUiPatchReceipt } = await import("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️component.ts"); const { Buffer } = await import("node:buffer");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🧪️fixture.json"); const row = fixture.valid[fixtureIndex]!; const payload = Buffer.from(Array.from({ length: Number(row.declaredBytes) }, (_, i) => (73 * i + 19) & 255));
      const name = "@webassemblyjs/leb128/lib/leb.js"; const imported: unknown = await import(name); if (!imported || typeof imported !== "object") throw new Error("Missing LEB128 oracle"); const lib: unknown = "default" in imported ? imported.default : imported; if (!lib || typeof lib !== "object" || !("encodeUIntBuffer" in lib) || typeof lib.encodeUIntBuffer !== "function") throw new Error("Invalid LEB128 oracle");
      const encode = lib.encodeUIntBuffer; const uint = (value: number) => { const input = Buffer.alloc(8); input.writeBigUInt64LE(BigInt(value)); const output: unknown = encode(input); if (!(output instanceof Uint8Array)) throw new Error("Invalid LEB128 output"); return Buffer.from(output); }; const frame = (tag: number, body: Uint8Array) => Buffer.concat([Buffer.of(tag), uint(body.length), body]);
      const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const { default: returnAdmission } = await import("../../../../../../../🔨️modules/🎭️actor/🪪️activation/📤️return/🏘️admission/🧪️fixture.json"); let source: import("../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts").OwnedShardReturn | null = null;
      for (const phase of returnAdmission.phases) { const current = native.lease.reserveReturn(4, { maxItems: 1, maxBytes: phase.grant }); expect(current.step.kind, phase.phase).toBe(phase.kind); source = current.source; } if (!source) throw new Error("Native fixture did not admit original return owner"); const { default: responseAdmission } = await import("../../../../../../../🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🏘️admission/🧪️fixture.json"); for (const phase of responseAdmission.phases) expect(source.reserveResponse({ maxItems: 1, maxBytes: phase.grant }).kind, phase.phase).toBe(phase.kind); const pending = source.execute([], native.budget); const origin = source.origin; if (!origin) throw new Error("Native fixture did not issue return origin");
      const receipt = { lifetime: native.lifetime, patchSequence: 51n }; const surface = Buffer.from("window"); const bytes = Buffer.concat([Buffer.from("73727401", "hex"), frame(0, Buffer.of(0, 0, 1, 0, 0)), frame(2, Buffer.concat([encodeActorUiPatchReceipt(receipt), uint(surface.length), surface, Buffer.of(0, 1, 1)])), frame(3, Buffer.concat([Buffer.of(1), uint(7), uint(payload.length), payload])), frame(4, Buffer.alloc(0)), frame(7, Buffer.alloc(13)), frame(9, Buffer.alloc(0))]);
      await native.answer(pending, encodeActorReturnResult({ kind: "page", receipt: { identity: { origin, returnSequence: 1n }, pageSequence: 1n, length: bytes.length, final: true }, page: createActorBytePage(Uint8Array.from(bytes)) })); const content = new OwnedKernelReturnContent(source, owner, native.activation, native.lifetime);
      for (let i = 0; i < bytes.length && !content.field; i++) { const current = content.advance(grant); expect(current.kind).not.toBe("rejected"); expect(current.bytes).toBeLessThanOrEqual(1); } const field = content.field; if (!field || !field.fragment) throw new Error("Native fixture did not issue exact field"); return { ...native, owner, source, content, field, fragment: field.fragment, payload, fixture };
    }

    it("OwnedPagedPayloadSchema admits only exact canonical u64 field lengths", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🧪️fixture.json"); const { default: fixtureSchema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🧪️schema.json"); const { default: domain } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🧬️schema.json"); const { default: words } = await import("../../../../../../../🔨️modules/🎭️actor/📄️page/🧬️schema.json"); const { Buffer } = await import("node:buffer");
      const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(words); expect(ajv.compile(fixtureSchema)(fixture)).toBe(true); const validate = ajv.compile(domain);
      for (const row of fixture.declaredBoundaries) {
        let oracle = false; try { const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(BigInt(row.value)); oracle = bytes.readBigUInt64LE().toString() === row.value; } catch {}
        expect(oracle, row.value).toBe(row.accepted); expect(validate({ declaredBytes: row.value, destinationPageBytes: fixture.pageBytes, inputOwner: "native-private-field-and-fragment", residentOwner: "shared-host-exact-instance-payload", completion: "copied-input-is-not-publication" }), row.value).toBe(oracle);
      }
      for (const row of fixture.valid) { const count = row.fragments.reduce((sum, length) => sum + BigInt(length), 0n); expect(count.toString()).toBe(row.declaredBytes); expect(Math.ceil(Number(count) / fixture.pageBytes)).toBe(row.pages); expect(row.pages * fixture.pageBytes).toBe(row.chargedBytes); }
    });
    it("OwnedPagedInputBrands rejects structural builders and forged copied or cancelled evidence", async () => {
      const { OwnedUiOperationPayloadBuilder, OwnedUiOperationInputCopied, OwnedUiOperationInputCancelled } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts");
      const field = Object.freeze({}); const fragment = Object.freeze({}); const builder = Object.freeze({ field }); let reads = 0; const hostile = { get field() { reads++; return field; }, get offset() { reads++; return 0n; } };
      expect(typeof OwnedUiOperationPayloadBuilder).toBe("function"); expect(OwnedUiOperationPayloadBuilder.matchesField(builder, field)).toBe(false); expect(OwnedUiOperationPayloadBuilder.matchesField(hostile, field)).toBe(false); expect(() => Reflect.construct(OwnedUiOperationPayloadBuilder, [{}, field, {}])).toThrow(/authority/);
      for (const token of [OwnedUiOperationInputCopied, OwnedUiOperationInputCancelled]) { expect(token.matches(hostile, fragment, field, builder, 0n, 1)).toBe(false); expect(token.matches({}, fragment, field, builder, 18446744073709551616n, 1)).toBe(false); expect(() => Reflect.construct(token, [{}, fragment, field, builder, 0n, 1])).toThrow(/authority/); }
      expect(reads).toBe(0);
    });
    it("OwnedPagedAdmission roots the actual field builder before exposure and cancels abandoned input under the shared owner grant", async () => {
      const { OwnedUiOperationPayloadBuilder, OwnedUiOperationInputCopied, OwnedUiOperationInputCancelled } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { OwnedKernelReturnInputFragment } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { default: builders } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { default: evidence } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️fixture.json"); const { default: cancellation } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧪️fixture.json"); const { default: scopes } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture.json");
      const { native, pool, scope, fixture, baseline } = await residentPayloadFixture(); let payload: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayload | null = null; for (const bytes of fixture.admissionBytes) payload = scope.beginPayload(native.field, { maxItems: 1, maxBytes: bytes }).payload; if (!payload) throw new Error("Missing admitted payload"); const before = pool.usage;
      expect(OwnedUiOperationPayloadBuilder.begin(native.owner, native.activation, native.lifetime, native.field, payload, { maxItems: 0, maxBytes: 4096 })).toMatchObject({ step: { kind: "blocked", items: 0, bytes: 0 }, builder: null }); expect(pool.usage).toEqual(before);
      let builder: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts").OwnedUiOperationPayloadBuilder | null = null;
      for (let index = 0; index < builders.grants.length; index++) { const bytes = builders.grants[index]!; const current = OwnedUiOperationPayloadBuilder.begin(native.owner, native.activation, native.lifetime, native.field, payload, { maxItems: 1, maxBytes: bytes }); expect(current.step).toMatchObject({ kind: index === builders.grants.length - 1 ? "ready" : "pending", bytes }); builder = current.builder; } if (!builder) throw new Error("Missing admitted builder"); const original = builder;
      expect(OwnedUiOperationPayloadBuilder.matchesField(original, native.field)).toBe(true); expect(pool.usage).toEqual(produce(before, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += builders.total[axis]; })); expect(OwnedUiOperationPayloadBuilder.begin(native.owner, native.activation, native.lifetime, native.field, payload, grant)).toMatchObject({ step: { kind: "ready", bytes: 0 }, builder: original });
      expect(original.advance({ maxItems: 1, maxBytes: cancellation.detachBytes })).toMatchObject({ kind: "pending", bytes: cancellation.detachBytes }); let cancelled = 0; const release = OwnedKernelReturnInputFragment.prototype.release; const probe = vi.spyOn(OwnedKernelReturnInputFragment.prototype, "release").mockImplementation(function(this: typeof native.fragment, proof) { expect(this).toBe(native.fragment); expect(OwnedUiOperationInputCopied.matches(proof, this, native.field, original, 0n, native.payload.length)).toBe(false); expect(OwnedUiOperationInputCancelled.matches(proof, this, native.field, original, 0n, native.payload.length)).toBe(true); const result = release.call(this, proof); expect(result?.kind).toBe("cancelled"); cancelled++; return result; }); const read = vi.spyOn(OwnedKernelReturnInputFragment.prototype, "byteAt");
      const maximum = cancellation.closeBound.inputDetachGrants.length + evidence.grants.length + evidence.retirement.grants.length + evidence.retirement.registrationGrants.length + builders.bindingGrants.length + fixture.sourceReleaseTurns.length + cancellation.closeBound.payloadRegistrationGrants.length + scopes.firstChild.retirementTurns + 2; let complete = false;
      try { expect(() => { throw new Error("Caller lost admitted builder"); }).toThrow(); native.owner.beginClose(); for (let turn = 0; turn < maximum; turn++) { const current = native.owner.closeStep(grant); expect(current.kind, current.phase).not.toBe("rejected"); expect(current.kind, current.phase).not.toBe("blocked"); expect(current.bytes).toBeLessThanOrEqual(grant.maxBytes); if (current.kind === "complete") { complete = true; break; } } expect(complete).toBe(true); expect(original.terminalIsEmpty()).toBe(true); expect(payload.terminalIsEmpty()).toBe(true); expect(scope.terminalIsEmpty()).toBe(true); expect(cancelled).toBe(1); expect(read).not.toHaveBeenCalled(); expect(native.field.consumed).toBe(0n); expect(native.field.complete).toBe(false); expect(pool.usage).toEqual(baseline); } finally { probe.mockRestore(); read.mockRestore(); }
    });
    it("OwnedPagedAdmission keeps rejected binding and source release failures owned and separates terminal child work from parent release", async () => {
      const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { OwnedKernelReturnInputFragment } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { default: builders } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { native, pool, scope, payload, builder, evidenceFixture: evidence } = await residentEvidenceFixture(); const foreign = await residentBuilderFixture(); const before = pool.usage;
      expect(OwnedUiOperationPayloadBuilder.begin(native.owner, native.activation, { ...native.lifetime, guestLifetime: native.lifetime.guestLifetime + 1n }, native.field, payload, grant).step.kind).toBe("rejected"); expect(OwnedUiOperationPayloadBuilder.begin(native.owner, native.activation, native.lifetime, { value: native.field.value }, payload, grant).step.kind).toBe("rejected"); expect(OwnedUiOperationPayloadBuilder.begin(native.owner, native.activation, native.lifetime, foreign.native.field, payload, grant).step.kind).toBe("rejected"); expect(scope.beginPayload(native.field, grant)).toMatchObject({ step: { kind: "ready", bytes: 0 }, payload }); expect(pool.usage).toEqual(before);
      for (const bytes of evidence.grants) payload.beginEvidence(builder, { maxItems: 1, maxBytes: bytes }); expect(payload.advanceEvidence(builder, { maxItems: 1, maxBytes: evidence.retirement.grants[0]! }).kind).toBe("pending"); const held = pool.usage; const refused = vi.spyOn(OwnedKernelReturnInputFragment.prototype, "release").mockReturnValue(null);
      try { expect(payload.advanceEvidence(builder, { maxItems: 1, maxBytes: evidence.retirement.grants[1]! })).toMatchObject({ kind: "rejected", phase: "paged-evidence-release-refused", bytes: 128 }); expect(pool.usage).toEqual(held); expect(builder.terminalIsEmpty()).toBe(false); expect(native.field.fragment).toBe(native.fragment); } finally { refused.mockRestore(); }
      for (const bytes of [...evidence.retirement.grants.slice(1), ...evidence.retirement.registrationGrants]) { const current = payload.advanceEvidence(builder, { maxItems: 1, maxBytes: bytes }); expect(current.kind).not.toBe("rejected"); expect(current.bytes).toBe(bytes); } expect(pool.usage).toEqual(before); payload.beginClose();
      for (let index = 0; index < 7; index++) expect(payload.closeStep({ maxItems: 1, maxBytes: builders.bindingGrants[index]! }).kind).toBe("pending"); const terminal = OwnedUiOperationPayloadBuilder.finishRetirement; const probe = vi.spyOn(OwnedUiOperationPayloadBuilder, "finishRetirement").mockImplementation((value, resident, budget) => { const current = terminal(value, resident, budget); expect(current.kind).toBe("complete"); return { ...current, items: 1, bytes: budget.maxBytes }; });
      try { expect(payload.closeStep(grant)).toMatchObject({ kind: "pending", bytes: 4096 }); expect(pool.usage).toEqual(before); expect(probe).toHaveBeenCalledTimes(1); } finally { probe.mockRestore(); }
      expect(payload.closeStep({ maxItems: 1, maxBytes: builders.bindingGrants[8]! })).toMatchObject({ kind: "pending", bytes: 64 }); expect(pool.usage).toEqual(before); for (const bytes of builders.bindingGrants.slice(9)) expect(payload.closeStep({ maxItems: 1, maxBytes: bytes }).kind).toBe("pending"); expect(pool.usage).toEqual(produce(before, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] -= builders.total[axis]; })); expect(builder.terminalIsEmpty()).toBe(true);
    });
    async function residentAuthoredWindowFixture() {
      const base = await residentBuilderFixture(); const { default: binding } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🔗️binding/🧪️fixture.json"); const { default: pages } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); expect(base.builder.advance({ maxItems: 1, maxBytes: binding.producer.fragmentCapture })).toMatchObject({ kind: "pending", bytes: binding.producer.fragmentCapture });
      for (const bytes of [...pages.storageOwnerGrants, ...binding.admissionGrants, binding.producer.pageObservation]) expect(base.builder.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); const page = base.payload.beginPage(base.builder, binding.stream.windows[0]!, grant).page; if (!page) throw new Error("Missing original producer page"); return { ...base, page, binding, pages };
    }
    async function residentAuthoredSourceFixture() { const base = await residentCopiedRangeFixture(); for (const bytes of [...base.evidenceFixture.grants, 64, 128]) expect(base.builder.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); return base; }
    it("OwnedPagedCopy copies the genuine first fragment into admitted fixed pages and strongly owns the sequential reader", async () => {
      const { Buffer } = await import("node:buffer"); const { default: copied } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧪️fixture.json"); const { default: cancellation } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧪️fixture.json"); const { default: builders } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { OwnedUiOperationInputCopied, OwnedUiOperationInputCancelled } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { OwnedKernelReturnInputFragment } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { native, payload, builder, reader, actual, pool, evidenceFixture: evidence, readerFixture: readers, fixture: payloadFixture, pageFixture: pages } = await residentCopiedRangeFixture(); const baseline = pool.usage;
      expect(Buffer.from(actual)).toEqual(native.payload); expect(native.field.consumed).toBe(0n); expect(builder.beginRead(grant)).toMatchObject({ step: { kind: "ready", bytes: 0 }, reader }); expect(reader.advance(grant).kind).toBe("blocked"); let issued = 0; const original = OwnedKernelReturnInputFragment.prototype.release; const release = vi.spyOn(OwnedKernelReturnInputFragment.prototype, "release").mockImplementation(function(this: typeof native.fragment, token) { expect(OwnedUiOperationInputCopied.matches(token, this, native.field, builder, this.offset, this.length)).toBe(true); expect(OwnedUiOperationInputCancelled.matches(token, this, native.field, builder, this.offset, this.length)).toBe(false); issued++; return original.call(this, token); });
      try { for (const bytes of [...evidence.grants, 64, 128]) builder.advance({ maxItems: 1, maxBytes: bytes }); for (let index = 0; index < native.payload.length; index++) { builder.advance({ maxItems: 1, maxBytes: copied.sourceStepBytes }); builder.advance({ maxItems: 1, maxBytes: copied.sourceObservationBytes }); } for (const bytes of [...copied.remainingRetirementGrants, ...evidence.retirement.registrationGrants, copied.rangeObservationBytes]) builder.advance({ maxItems: 1, maxBytes: bytes }); } finally { release.mockRestore(); } expect(issued).toBe(1); expect(native.field.complete).toBe(true); expect(builder.advance(grant)).toMatchObject({ kind: "ready", bytes: 0 }); expect(reader.advance(grant)).toMatchObject({ kind: "complete", bytes: 0 }); expect(pool.usage).toEqual(baseline);
      expect(() => { throw new Error("Caller lost registered reader"); }).toThrow(); native.lease.beginClose(); payload.beginClose(); const maximum = readers.closing.grants.length + builders.bindingGrants.length + payloadFixture.sourceReleaseTurns.length + cancellation.closeBound.payloadRegistrationGrants.length + cancellation.closeBound.storageGrants.length; let complete = false;
      for (let turn = 0; turn < maximum; turn++) { const current = payload.closeStep(grant); expect(current.kind, current.phase).not.toBe("rejected"); expect(current.kind, current.phase).not.toBe("blocked"); expect(current.bytes).toBeLessThanOrEqual(grant.maxBytes); if (current.kind === "complete") { complete = true; break; } } expect(complete).toBe(true); expect(reader.terminalIsEmpty()).toBe(true); expect(builder.terminalIsEmpty()).toBe(true); expect(pool.usage).toEqual(produce(baseline, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] -= readers.total[axis] + builders.total[axis] + payloadFixture.expectedPayloadTotal[axis] + pages.storageOwnerTotal[axis]; }));
    });
    it("OwnedPagedFault preserves child refusal and raw over-grant accounting without advancing the producer", async () => {
      const { OwnedUiResidentPage } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🧪️fixture.json");
      for (const fault of fixture.childFailures) {
        const { builder, payload, pool, native } = await residentAuthoredWindowFixture(); const held = pool.usage; const original = OwnedUiResidentPage.prototype.allocate; const raw = Object.freeze({ fault, backing: Uint8Array.of(73) }); const probe = vi.spyOn(OwnedUiResidentPage.prototype, "allocate").mockImplementation(function(this: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage, budget) { if (fault === "throw") throw raw; if (fault === "blocked" || fault === "rejected") return { kind: fault, phase: "authored-page-refusal", items: 0, bytes: 0 }; const result = original.call(this, budget); return { ...result, items: 1, bytes: budget.maxBytes + (fault === "over-grant" ? 1 : 0) }; });
        try { expect(builder.advance(grant)).toMatchObject({ kind: fault === "blocked" ? "blocked" : fault === "terminal-full-grant" ? "pending" : "rejected", bytes: fault === "over-grant" ? 4097 : fault === "terminal-full-grant" ? 4096 : 0 }); } finally { probe.mockRestore(); } expect(pool.usage).toEqual(held); expect(native.field.consumed).toBe(0n);
        if (fault === "blocked" || fault === "terminal-full-grant") expect(builder.advance(grant).kind).toBe("pending"); else { expect(builder.advance(grant)).toMatchObject({ kind: "rejected", bytes: 0 }); expect(builder.beginRead(grant).reader).toBeNull(); payload.beginClose(); expect(payload.closeStep(grant).kind).not.toBe("complete"); expect(builder.terminalIsEmpty()).toBe(false); expect(pool.usage).toEqual(held); }
      }
    });
    it("OwnedPagedFault retains a registered reader after an exact page read throws without skipping the failed byte", async () => {
      const { OwnedResidentReader, OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: readers } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json"); const { native, pool, payload, builder, binding } = await residentAuthoredWindowFixture(); let reader: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayloadReader | null = null;
      for (const bytes of readers.admissionGrants) reader = builder.beginRead({ maxItems: 1, maxBytes: bytes }).reader; if (!reader) throw new Error("Missing early reader"); for (const bytes of [binding.producer.allocation, binding.producer.allocationObservation]) builder.advance({ maxItems: 1, maxBytes: bytes }); for (let index = 0; index < binding.stream.windows[0]!; index++) { builder.advance({ maxItems: 1, maxBytes: 1 }); builder.advance({ maxItems: 1, maxBytes: 1 }); } for (const bytes of [binding.producer.seal, binding.producer.sealObservation]) builder.advance({ maxItems: 1, maxBytes: bytes }); reader.advance({ maxItems: 1, maxBytes: 64 }); for (const bytes of readers.aliasGrants) reader.advance({ maxItems: 1, maxBytes: bytes }); expect(reader.advance({ maxItems: 1, maxBytes: 1 })).toMatchObject({ kind: "byte", value: native.payload[0] });
      const held = pool.usage; const fault = Object.freeze({ backing: Uint8Array.of(73) }); const read = vi.spyOn(OwnedResidentReader.prototype, "byteAt").mockImplementation(() => { throw fault; }); try { expect(reader.advance(grant)).toMatchObject({ kind: "rejected", bytes: 0 }); expect(read.mock.calls).toEqual([[1]]); } finally { read.mockRestore(); } expect(reader.advance(grant)).toMatchObject({ kind: "rejected", bytes: 0 }); const retained: unknown[] = []; const original = OwnedResidentAdmission.prototype.retainFailure; const probe = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return original.call(this, error, budget); }); try { payload.beginClose(); expect(payload.closeStep(grant).kind).toBe("pending"); expect(payload.closeStep(grant).kind).toBe("rejected"); expect(payload.closeStep(grant).kind).toBe("pending"); expect(payload.closeStep(grant).kind).toBe("rejected"); } finally { probe.mockRestore(); } expect(retained).toEqual([fault]); expect(reader.terminalIsEmpty()).toBe(false); expect(pool.usage).toEqual(held);
    });
    it("OwnedPagedPartialFault cannot resume after a page write throws before or after mutation", async () => {
      const { OwnedUiResidentPage } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { OwnedKernelReturnInputFragment } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🧪️fixture.json");
      for (const row of fixture.writeFaults) { const { builder, payload, pool, native, binding } = await residentAuthoredWindowFixture(); for (const bytes of [binding.producer.allocation, binding.producer.allocationObservation]) builder.advance({ maxItems: 1, maxBytes: bytes }); const held = pool.usage; const fault = Object.freeze({ row, backing: Uint8Array.of(73) }); let written = 0; const original = OwnedUiResidentPage.prototype.writeByte; const write = vi.spyOn(OwnedUiResidentPage.prototype, "writeByte").mockImplementation(function(this: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage, value, budget) { if (row === "after-write") { expect(original.call(this, value, budget)).toMatchObject({ kind: "pending", bytes: 1 }); written++; } throw fault; }); const read = vi.spyOn(OwnedKernelReturnInputFragment.prototype, "byteAt"); const release = vi.spyOn(OwnedKernelReturnInputFragment.prototype, "release");
        try { expect(builder.advance({ maxItems: 1, maxBytes: 1 })).toMatchObject({ kind: "pending", bytes: 1 }); expect(builder.advance({ maxItems: 1, maxBytes: 1 })).toMatchObject({ kind: "rejected", bytes: 0 }); write.mockRestore(); expect(builder.advance(grant)).toMatchObject({ kind: "rejected", bytes: 0 }); expect(read.mock.calls.map(call => call[0])).toEqual([0]); expect(written).toBe(row === "after-write" ? 1 : 0); expect(release).not.toHaveBeenCalled(); } finally { write.mockRestore(); read.mockRestore(); release.mockRestore(); } expect(builder.failure).toBe(fault); payload.beginClose(); expect(payload.closeStep(grant).kind).toBe("pending"); expect(payload.closeStep(grant).kind).toBe("rejected"); expect(builder.failure).toBe(fault); expect(pool.usage).toEqual(held); expect(native.field.consumed).toBe(0n); expect(builder.terminalIsEmpty()).toBe(false);
      }
    });
    async function residentAuthoredClosePayload(base: Awaited<ReturnType<typeof residentBuilderFixture>>) {
      const { default: readers } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json"); const { default: binding } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🔗️binding/🧪️fixture.json"); const { default: builders } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { default: cancellation } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧪️fixture.json"); const maximum = readers.admissionGrants.length + readers.closing.grants.length + readers.aliasCloseGrants.length + binding.admissionGrants.length + binding.closeGrants.length + base.evidenceFixture.grants.length + base.evidenceFixture.retirement.grants.length + base.evidenceFixture.retirement.registrationGrants.length + builders.bindingGrants.length + base.fixture.sourceReleaseTurns.length + cancellation.closeBound.payloadRegistrationGrants.length + cancellation.closeBound.storageGrants.length + cancellation.closeBound.inputDetachGrants.length;
      base.payload.beginClose(); for (let turn = 0; turn < maximum; turn++) { const current = base.payload.closeStep(grant); expect(current.kind, current.phase).not.toBe("rejected"); expect(current.kind, current.phase).not.toBe("blocked"); expect(current.items).toBeLessThanOrEqual(grant.maxItems); expect(current.bytes).toBeLessThanOrEqual(grant.maxBytes); if (current.kind === "complete") return; } throw new Error("Declared child retirement phases exhausted");
    }
    it("OwnedPagedCancel retains every first-fragment cancellation prefix and distinguishes copied from detached input", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🧪️fixture.json"); const { default: readers } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json"); const { default: binding } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🔗️binding/🧪️fixture.json"); const { default: pages } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); const { default: evidence } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️fixture.json"); const { default: copied } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧪️fixture.json"); const { default: builders } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { OwnedUiOperationInputCopied, OwnedUiOperationInputCancelled } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { OwnedKernelReturnInputFragment } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { Buffer } = await import("node:buffer");
      const plan: Array<{ driver: "admit-reader" | "builder" | "reader"; bytes: number; stage: string }> = []; const append = (driver: "admit-reader" | "builder" | "reader", values: readonly number[], stage: string) => { for (const bytes of values) plan.push({ driver, bytes, stage }); };
      append("admit-reader", readers.admissionGrants, "reader-admission"); append("builder", [binding.producer.fragmentCapture], "offer");
      for (let window = 0; window < binding.stream.windows.length; window++) { const length = binding.stream.windows[window]!; append("builder", [...(window === 0 ? pages.storageOwnerGrants : []), ...binding.admissionGrants, binding.producer.pageObservation], "page-admission"); append("builder", [binding.producer.allocation, binding.producer.allocationObservation], "allocation"); for (let index = 0; index < length; index++) { append("builder", [1], "source-read"); append("builder", [1], "destination-write"); } append("builder", [binding.producer.seal, binding.producer.sealObservation], "seal"); append("reader", [64, ...readers.aliasGrants], "reader-alias"); append("reader", Array.from({ length }, () => 1), "reader-byte"); append("reader", [64, ...readers.aliasCloseGrants, ...binding.closeGrants, 64], "page-retirement"); }
      append("builder", [binding.producer.inputDetach], "input-detach"); append("builder", evidence.grants, "evidence-admission"); append("builder", [64, 128], "evidence-release"); for (let index = 0; index < copied.length; index++) { append("builder", [copied.sourceStepBytes], "source-consume"); append("builder", [copied.sourceObservationBytes], "source-observe"); } append("builder", [...copied.remainingRetirementGrants, ...evidence.retirement.registrationGrants], "evidence-retirement"); append("builder", [copied.rangeObservationBytes], "range-observe"); append("builder", [4096], "ready"); expect(new Set(plan.map(row => row.stage))).toEqual(new Set(fixture.cancellationPhases)); expect(fixture.cancellation.everyPrefix).toBe(true); let executed = 0;
      for (let prefix = 0; prefix <= plan.length; prefix++) {
        const base = await residentBuilderFixture(fixture.cancellation.fixtureIndex); const { builder, payload, native, pool } = base; const baseline = pool.usage; let reader: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayloadReader | null = null; const actual: number[] = []; let releases = 0; let copiedReleases = 0; let cancelledReleases = 0; const original = OwnedKernelReturnInputFragment.prototype.release; const release = vi.spyOn(OwnedKernelReturnInputFragment.prototype, "release").mockImplementation(function(this: typeof native.fragment, token) { expect(this).toBe(native.fragment); if (OwnedUiOperationInputCopied.matches(token, this, native.field, builder, this.offset, this.length)) copiedReleases++; else if (OwnedUiOperationInputCancelled.matches(token, this, native.field, builder, this.offset, this.length)) cancelledReleases++; else throw new Error("Unissued original input evidence"); releases++; return original.call(this, token); });
        try {
          for (const row of plan.slice(0, prefix)) { const budget = { maxItems: 1, maxBytes: row.bytes }; const current = row.driver === "admit-reader" ? (() => { const admission = builder.beginRead(budget); if (admission.reader) reader = admission.reader; return admission.step; })() : row.driver === "builder" ? builder.advance(budget) : reader!.advance(budget); expect(current.kind, prefix + ":" + row.stage).not.toBe("rejected"); expect(current.kind, prefix + ":" + row.stage).not.toBe("blocked"); expect(current.bytes).toBeLessThanOrEqual(row.bytes); if (current.kind === "byte") actual.push(current.value); }
          expect(Buffer.from(actual)).toEqual(native.payload.subarray(0, actual.length)); const consumed = native.field.consumed; const complete = native.field.complete; const held = pool.usage; expect(builder.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(pool.usage).toEqual(held); native.lease.beginClose(); const reads = vi.spyOn(OwnedKernelReturnInputFragment.prototype, "byteAt");
          try { await residentAuthoredClosePayload(base); expect(reads).not.toHaveBeenCalled(); } finally { reads.mockRestore(); } expect(native.field.consumed).toBe(consumed); expect(native.field.complete).toBe(complete); const offered = prefix > readers.admissionGrants.length; expect(releases, String(prefix)).toBe(offered ? 1 : 0); expect(copiedReleases + cancelledReleases).toBe(releases); expect(builder.terminalIsEmpty()).toBe(true); expect(reader === null || reader.terminalIsEmpty()).toBe(true); expect(pool.usage).toEqual(produce(baseline, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] -= builders.total[axis] + base.fixture.expectedPayloadTotal[axis]; })); executed++;
        } finally { release.mockRestore(); }
      }
      expect(executed).toBe(plan.length + 1); console.log("[DEBUG] Authored cancellation prefixes", executed);
    });
    async function residentAuthoredBuilderFault(frontier: "builder-finalizer" | "source-bind-before" | "source-bind-after") {
      const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { OwnedKernelReturnInputField, OwnedKernelReturnInputFragment } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: builders } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { default: wire } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🧪️fixture.json"); const row = wire.bindingFaults.find(value => value.name === (frontier === "builder-finalizer" ? "constructor-finalization" : frontier === "source-bind-before" ? "before-bind" : "after-bind")); if (!row) throw new Error("Missing authored binding fault"); expect(builders.faults).toContain(frontier); const { native, scope, pool, fixture } = await residentPayloadFixture(); let payload: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayload | null = null; for (const bytes of fixture.admissionBytes) payload = scope.beginPayload(native.field, { maxItems: 1, maxBytes: bytes }).payload; if (!payload) throw new Error("Missing original payload"); const parent = payload; const begin = (bytes: number) => OwnedUiOperationPayloadBuilder.begin(native.owner, native.activation, native.lifetime, native.field, parent, { maxItems: 1, maxBytes: bytes }); const prefix = frontier === "builder-finalizer" ? 11 : 8; for (const bytes of builders.grants.slice(0, prefix)) begin(bytes); const held = pool.usage; const fault = Object.freeze({ frontier, backing: Uint8Array.of(73) }); const captured: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts").OwnedUiOperationPayloadBuilder[] = []; const freeze = Object.freeze; const bind = OwnedKernelReturnInputField.prototype.bind;
      const freezing = vi.spyOn(Object, "freeze").mockImplementation(value => { if (frontier === "builder-finalizer" && OwnedUiOperationPayloadBuilder.matchesResident(value, parent)) { captured.push(value); throw fault; } return freeze(value); }); const binding = vi.spyOn(OwnedKernelReturnInputField.prototype, "bind").mockImplementation(function(this: typeof native.field, value) { if (frontier !== "builder-finalizer") { if (frontier === "source-bind-after") expect(bind.call(this, value)).toBe(true); captured.push(value); throw fault; } return bind.call(this, value); });
      try { expect(begin(builders.grants[prefix]!)).toMatchObject({ step: { kind: "rejected" }, builder: null }); } finally { freezing.mockRestore(); binding.mockRestore(); } expect(captured).toHaveLength(1); const builder = captured[0]!; expect(OwnedKernelReturnInputField.matchesBuilder(native.field, builder)).toBe(row.sourceBound); expect(begin(4096).step.kind).toBe("rejected"); expect(pool.usage).toEqual(held); const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const observe = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); }); const release = vi.spyOn(OwnedKernelReturnInputFragment.prototype, "release");
      try { parent.beginClose(); expect(parent.closeStep(grant).kind).toBe("pending"); expect(parent.closeStep(grant).kind).toBe("rejected"); expect(release).toHaveBeenCalledTimes(row.cancelledReleases); } finally { observe.mockRestore(); release.mockRestore(); } expect(retained).toEqual(row.originalFaultHeld ? [fault] : []); expect(builder.terminalIsEmpty()).toBe(!row.parentRetainsBuilder); expect(parent.terminalIsEmpty()).toBe(false); expect(native.field.fragment).toBe(native.fragment); expect(pool.usage).toEqual(held);
    }
    it("OwnedPagedRegistration roots an allocated private builder before constructor finalization or field binding can throw", async () => {
      await residentAuthoredBuilderFault("builder-finalizer"); await residentAuthoredBuilderFault("source-bind-before");
    });
    it("OwnedPagedBoundFault retains the original privately bound source and raw exception after binding changed its owner", async () => {
      await residentAuthoredBuilderFault("source-bind-after");
    });
    it("OwnedPagedContinuation completes only after the original field consumes its released range", async () => {
      const { default: copied } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧪️fixture.json"); const { builder, reader, native, evidenceFixture: evidence } = await residentAuthoredSourceFixture(); let consumed = 0;
      for (let index = 0; index < native.payload.length; index++) { const before = native.field.consumed; expect(builder.advance({ maxItems: 1, maxBytes: copied.sourceStepBytes })).toMatchObject({ kind: "pending", phase: "paged-evidence-source-advance", bytes: copied.sourceStepBytes }); expect(native.field.consumed - before).toBe(1n); consumed++; expect(reader.advance(grant).kind).toBe("blocked"); expect(builder.advance({ maxItems: 1, maxBytes: copied.sourceObservationBytes })).toMatchObject({ kind: "pending", bytes: copied.sourceObservationBytes }); }
      expect(consumed).toBe(native.payload.length); expect(native.field.complete).toBe(true); expect(native.field.fragment).toBe(native.fragment); for (const bytes of [...copied.remainingRetirementGrants, ...evidence.retirement.registrationGrants, copied.rangeObservationBytes]) expect(builder.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); expect(native.field.fragment).toBeNull(); expect(builder.advance(grant)).toMatchObject({ kind: "ready", bytes: 0 }); expect(reader.advance(grant)).toMatchObject({ kind: "complete", bytes: 0 });
    });
    it("OwnedPagedContinuation preserves source child faults and separates terminal work from readiness", async () => {
      const { OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🧪️fixture.json"); const { default: copied } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧪️fixture.json");
      for (const row of fixture.continuationFailures) { const { builder, native, pool, payload } = await residentAuthoredSourceFixture(); if (row === "terminal-full-grant") for (let index = 0; index < native.payload.length - 1; index++) { builder.advance({ maxItems: 1, maxBytes: copied.sourceStepBytes }); builder.advance({ maxItems: 1, maxBytes: copied.sourceObservationBytes }); } const held = pool.usage; const before = native.field.consumed; const fault = Object.freeze({ row, backing: Uint8Array.of(73) }); const original = OwnedKernelReturnInputField.prototype.advance; const probe = vi.spyOn(OwnedKernelReturnInputField.prototype, "advance").mockImplementation(function(this: typeof native.field, budget, consumer) { expect(consumer).toBe(builder); if (row === "blocked" || row === "rejected") return { kind: row, items: 0, bytes: 0 }; if (row === "throw") throw fault; const current = original.call(this, budget, consumer); if (row === "after-work-throw") throw fault; if (row === "terminal-full-grant") expect(current.kind).toBe("complete"); return { ...current, items: 1, bytes: budget.maxBytes + (row === "over-grant" ? 1 : 0) }; });
        try { expect(builder.advance(grant)).toMatchObject({ kind: row === "blocked" ? "blocked" : row === "terminal-full-grant" ? "pending" : "rejected", bytes: row === "over-grant" ? 4097 : row === "terminal-full-grant" ? 4096 : 0 }); expect(probe).toHaveBeenCalledTimes(1); } finally { probe.mockRestore(); } const after = native.field.consumed; expect(after - before).toBe(row === "blocked" || row === "rejected" || row === "throw" ? 0n : 1n); expect(pool.usage).toEqual(held);
        if (row === "blocked") expect(builder.advance({ maxItems: 1, maxBytes: 1 })).toMatchObject({ kind: "pending", bytes: 1 }); else if (row === "terminal-full-grant") { expect(builder.advance({ maxItems: 1, maxBytes: copied.sourceObservationBytes })).toMatchObject({ kind: "pending", bytes: copied.sourceObservationBytes }); expect(native.field.consumed).toBe(after); } else { expect(builder.advance(grant)).toMatchObject({ kind: "rejected", bytes: 0 }); expect(native.field.consumed).toBe(after); expect(builder.terminalIsEmpty()).toBe(false); if (row === "throw" || row === "after-work-throw") { const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const capture = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); }); try { expect(payload.advanceEvidence(builder, grant)).toMatchObject({ kind: "pending", bytes: 64 }); expect(payload.advanceEvidence(builder, grant).kind).toBe("rejected"); } finally { capture.mockRestore(); } expect(retained).toEqual([fault]); expect(pool.usage).toEqual(held); } }
      }
    });
    it("OwnedResidentReaderConstructor keeps the preinstalled reader and exact finalization fault charged", async () => {
      const { OwnedUiResidentPayloadReader } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json"); const { payload, builder, pool } = await residentBuilderFixture();
      for (let index = 0; index < 11; index++) expect(builder.beginRead({ maxItems: 1, maxBytes: fixture.admissionGrants[index]! }).step.kind).toBe("pending"); const held = pool.usage; const fault = Object.freeze({ backing: Uint8Array.of(73) }); const captured: Array<import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayloadReader> = []; const freeze = Object.freeze;
      const probe = vi.spyOn(Object, "freeze").mockImplementation(value => { if (value instanceof OwnedUiResidentPayloadReader) { captured.push(value); throw fault; } return freeze(value); });
      try { expect(builder.beginRead(grant)).toMatchObject({ step: { kind: "rejected" }, reader: null }); expect(captured).toHaveLength(1); } finally { probe.mockRestore(); }
      expect(builder.beginRead(grant).step.kind).toBe("rejected"); const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const observe = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); }); try { payload.beginClose(); expect(payload.closeStep(grant).kind).toBe("pending"); expect(payload.closeStep(grant).kind).toBe("rejected"); } finally { observe.mockRestore(); }
      expect(retained).toEqual([fault]); expect(captured[0]!.terminalIsEmpty()).toBe(false); expect(pool.usage).toEqual(held);
    });
    it("OwnedPagedBoundary defers exact source completion metadata until after a full-grant child turn", async () => {
      const { OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { default: copied } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧪️fixture.json"); const { builder, native } = await residentAuthoredSourceFixture(); for (let index = 0; index < native.payload.length - 1; index++) { builder.advance({ maxItems: 1, maxBytes: copied.sourceStepBytes }); builder.advance({ maxItems: 1, maxBytes: copied.sourceObservationBytes }); }
      const original = OwnedKernelReturnInputField.prototype.advance; const probe = vi.spyOn(OwnedKernelReturnInputField.prototype, "advance").mockImplementation(function(this: typeof native.field, budget, consumer) { const current = original.call(this, budget, consumer); expect(current.kind).toBe("complete"); return { ...current, items: 1, bytes: budget.maxBytes }; }); const completion = vi.spyOn(OwnedKernelReturnInputField.prototype, "complete", "get"); const consumed = vi.spyOn(OwnedKernelReturnInputField.prototype, "consumed", "get");
      try { expect(builder.advance(grant)).toMatchObject({ kind: "pending", bytes: 4096 }); expect(probe).toHaveBeenCalledTimes(1); expect(completion).not.toHaveBeenCalled(); expect(consumed).not.toHaveBeenCalled(); probe.mockRestore(); expect(builder.advance({ maxItems: 1, maxBytes: copied.sourceObservationBytes - 1 })).toMatchObject({ kind: "blocked", bytes: 0 }); expect(completion).not.toHaveBeenCalled(); expect(consumed).not.toHaveBeenCalled(); expect(builder.advance({ maxItems: 1, maxBytes: copied.sourceObservationBytes })).toMatchObject({ kind: "pending", phase: "paged-evidence-source-observe", bytes: copied.sourceObservationBytes }); expect(completion).toHaveBeenCalled(); expect(consumed).toHaveBeenCalled(); } finally { probe.mockRestore(); completion.mockRestore(); consumed.mockRestore(); } expect(native.field.consumed).toBe(BigInt(native.payload.length)); expect(native.field.complete).toBe(true);
    });
    //#endregion 📄️PagedPayloadSchemaTests

    //#region 💾️ResidentPoolTests
    it("OwnedResidentPool reserves shared pages before allocation and returns credit only after final alias retirement", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️resident.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️resident.schema.json"); const { default: domain } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧬️schema.json"); const { default: neutral } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🧬️schema.json"); const { default: contract } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧬️contract.json"); const { default: metadata } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧪️fixture.json"); const { default: pages } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); const { default: binding } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🔗️binding/🧪️fixture.json"); const { default: readers } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json"); const { default: builders } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { Buffer } = await import("node:buffer");
      const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(neutral); expect(ajv.validate(schema, fixture)).toBe(true); const capacity = ajv.compile(domain); expect(capacity(metadata.compositionCapacity)).toBe(true); for (const invalid of fixture.invalidCapacities) expect(capacity(invalid)).toBe(false); expect(capacity({ bytes: 0, slots: 0, owners: 0, control: { bytes: 0, slots: 0, owners: 0 } })).toBe(true); expect(contract.composition.defaultCapacity).toBeNull(); expect(contract.finalInstanceWitness.counterOnlyAuthorityAccepted).toBe(false); expect(contract.composition.parallelUiCapacity).toBe(false);
      const left = await residentBuilderFixture(); const right = await residentBuilderFixture(2, left.native.residentLedger); expect(left.native.residentLedger).toBe(right.native.residentLedger); let oracle = left.pool.usage; let event = 0;
      const check = (name: string, delta: { bytes: number; slots: number; owners: number }) => { expect(fixture.trace[event++]).toBe(name); oracle = produce(oracle, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += delta[axis]; }); expect(left.pool.usage).toEqual(oracle); expect(right.pool.usage).toEqual(oracle); };
      const sum = (...values: Array<{ bytes: number; slots: number; owners: number }>) => ({ bytes: values.reduce((total, value) => total + value.bytes, 0), slots: values.reduce((total, value) => total + value.slots, 0), owners: values.reduce((total, value) => total + value.owners, 0) }); const minus = (value: { bytes: number; slots: number; owners: number }) => ({ bytes: -value.bytes, slots: -value.slots, owners: -value.owners });
      let x: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage | null = null; let y: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage | null = null;
      for (const bytes of [...pages.storageOwnerGrants, ...pages.admissionGrants]) x = left.payload.beginPage(left.builder, fixture.bytes.length, { maxItems: 1, maxBytes: bytes }).page; if (!x) throw new Error("Missing first page"); check("reserve-first-page", sum(pages.storageOwnerTotal, pages.total));
      for (const bytes of [...pages.storageOwnerGrants, ...pages.admissionGrants]) y = right.payload.beginPage(right.builder, 1, { maxItems: 1, maxBytes: bytes }).page; if (!y) throw new Error("Missing second page"); check("reserve-second-page", sum(pages.storageOwnerTotal, pages.total)); expect(x.allocate({ maxItems: 1, maxBytes: fixture.pageBytes - 1 }).kind).toBe("blocked"); expect(x.allocate(grant)).toMatchObject({ kind: "ready", bytes: fixture.pageBytes }); y.allocate(grant); for (const byte of fixture.bytes) x.writeByte(byte, { maxItems: 1, maxBytes: 1 }); y.writeByte(42, grant); x.seal(grant); y.seal(grant);
      let reader: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayloadReader | null = null; for (const bytes of readers.admissionGrants) reader = left.builder.beginRead({ maxItems: 1, maxBytes: bytes }).reader; if (!reader) throw new Error("Missing original reader"); check("admit-first-reader", readers.total); reader.advance({ maxItems: 1, maxBytes: 64 }); for (const bytes of readers.aliasGrants) reader.advance({ maxItems: 1, maxBytes: bytes }); check("capture-first-alias", sum(readers.intrinsicReader, metadata.neutralAdmission));
      const actual: number[] = []; for (const _ of fixture.bytes) { const current = reader.advance({ maxItems: 1, maxBytes: 1 }); if (current.kind !== "byte") throw new Error("Missing original alias byte"); actual.push(current.value); } expect(Buffer.from(actual)).toEqual(Buffer.from(fixture.bytes)); expect(reader.advance({ maxItems: 1, maxBytes: 64 }).kind).toBe("pending"); for (const bytes of readers.aliasCloseGrants) reader.advance({ maxItems: 1, maxBytes: bytes }); check("retire-first-alias", minus(sum(readers.intrinsicReader, metadata.neutralAdmission))); for (const bytes of [...binding.closeGrants, 64]) reader.advance({ maxItems: 1, maxBytes: bytes }); expect(x.terminalIsEmpty()).toBe(true); check("retire-first-page", minus(pages.total)); expect(y.terminalIsEmpty()).toBe(false); await residentAuthoredClosePayload(left); check("retire-first-payload", minus(sum(readers.total, builders.total, left.fixture.expectedPayloadTotal, pages.storageOwnerTotal))); await residentAuthoredClosePayload(right); check("retire-second-payload", minus(sum(builders.total, right.fixture.expectedPayloadTotal, pages.storageOwnerTotal, pages.total))); expect(event).toBe(fixture.trace.length);
    });
    it("OwnedResidentPool preserves ownership on saturation, cancellation, forged handles and revoked operations", async () => {
      const { OwnedUiResidentPool, OwnedUiResidentPage, OwnedUiResidentPayload, OwnedUiResidentInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { OwnedResidentLedger } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: neutral } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🧪️fixture.json"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️resident.json"); const { default: metadata } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧪️fixture.json"); const { default: pages } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); for (const invalid of fixture.invalidCapacities) expect(() => new OwnedResidentLedger(invalid)).toThrow(); for (const Type of [OwnedUiResidentPool, OwnedUiResidentPage, OwnedUiResidentPayload, OwnedUiResidentInstance]) expect(() => Reflect.construct(Type, [{}, {}])).toThrow();
      const base = await residentBuilderFixture(); const ledger = base.native.residentLedger; const baseline = ledger.usage; const claim = (partition: "data" | "control") => { const consumer = Object.freeze({ partition }); expect(ledger.prepareAdmission(consumer, partition, { maxItems: 1, maxBytes: metadata.neutralAdmission.bytes }).kind).toBe("pending"); const cell = ledger.preparedAdmission(consumer); if (!cell) throw new Error("Missing original saturation cell"); expect(ledger.claimAdmission(consumer, cell, { maxItems: 1, maxBytes: 64 }).kind).toBe("ready"); return cell; };
      const ownerCell = claim("data"); const owner = ledger.beginOwner("data", ownerCell, { maxItems: 1, maxBytes: pages.storageOwnerTotal.bytes - metadata.neutralAdmission.bytes }).owner; if (!owner) throw new Error("Missing saturation storage owner"); const cell = claim("data"); const externalBytes = neutral.recordCharges.storage.bytes + 2 * neutral.recordCharges.witness.bytes + 8; const remaining = ledger.capacity.bytes - ledger.capacity.control.bytes - ledger.usage.data.bytes - externalBytes; const slot = owner.reserveExternalBacking(remaining, cell, { maxItems: 1, maxBytes: externalBytes }).slot; if (!slot) throw new Error("Missing original unsubmitted backing reservation"); expect(ledger.usage.data.bytes).toBe(ledger.capacity.bytes - ledger.capacity.control.bytes); const saturated = ledger.usage;
      expect(base.payload.beginPage(base.builder, 1, grant)).toMatchObject({ step: { kind: "blocked", bytes: 0 }, page: null }); expect(ledger.usage).toEqual(saturated); const control = claim("control"); expect(ledger.usage.control.bytes).toBe(metadata.neutralAdmission.bytes); control.beginClose(); expect(control.closeStep(grant).kind).toBe("complete"); cell.beginClose(); for (const bytes of [externalBytes, 64, metadata.neutralAdmission.bytes]) expect(cell.closeStep({ maxItems: 1, maxBytes: bytes }).kind).not.toBe("rejected"); expect(slot.terminalIsEmpty()).toBe(true); ownerCell.beginClose(); for (const bytes of [pages.storageOwnerTotal.bytes - metadata.neutralAdmission.bytes, 64, metadata.neutralAdmission.bytes]) ownerCell.closeStep({ maxItems: 1, maxBytes: bytes }); expect(ledger.usage).toEqual(baseline);
      for (const prefix of fixture.cancelPrefixes) { const current = await residentBuilderFixture(); const before = current.pool.usage; let page: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage | null = null; for (const bytes of [...pages.storageOwnerGrants, ...pages.admissionGrants]) page = current.payload.beginPage(current.builder, fixture.bytes.length, { maxItems: 1, maxBytes: bytes }).page; if (!page) throw new Error("Missing cancellation page"); expect(OwnedUiResidentPayload.matchesOwner(current.payload, current.native.owner, current.native.activation, current.native.lifetime)).toBe(true); expect(OwnedUiResidentPayload.matchesOwner(Object.create(OwnedUiResidentPayload.prototype), current.native.owner, current.native.activation, current.native.lifetime)).toBe(false); expect(OwnedUiResidentPayload.matchesOwner(current.payload, current.native.owner, current.native.activation, { ...current.native.lifetime, guestLifetime: current.native.lifetime.guestLifetime + 1n })).toBe(false); expect(current.payload.beginPage(current.builder, 257, grant).step.kind).toBe("rejected");
        for (let index = 0; index < prefix; index++) { if (index === 0) page.allocate(grant); else if (index <= fixture.bytes.length) page.writeByte(fixture.bytes[index - 1]!, { maxItems: 1, maxBytes: 1 }); else page.seal(grant); } const held = current.pool.usage; expect(current.payload.closePage(page, { maxItems: 0, maxBytes: 0 }).kind).toBe("blocked"); expect(current.pool.usage).toEqual(held); current.native.lease.beginClose(); expect(page.allocate(grant).kind).toBe("rejected"); expect(current.payload.beginPage(current.builder, 1, grant).step.kind).toBe("rejected"); await residentAuthoredClosePayload(current); expect(page.terminalIsEmpty()).toBe(true); expect(current.pool.usage.bytes).toBeLessThan(before.bytes);
      }
    });
    it("OwnedResidentPool roots abandoned reservations and delays the exact instance witness for live read aliases", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️resident.json"); const { default: pages } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); const { default: readers } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json"); const { default: scopes } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture.json"); const base = await residentBuilderFixture(); const { native, payload, builder, scope } = base; let page: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage | null = null;
      expect(() => { for (const bytes of [...pages.storageOwnerGrants, ...pages.admissionGrants]) page = payload.beginPage(builder, 2, { maxItems: 1, maxBytes: bytes }).page; if (!page) throw new Error("Missing abandoned page"); page.allocate(grant); page.writeByte(77, grant); page.writeByte(88, grant); page.seal(grant); throw new Error("Caller failed after owned reservation"); }).toThrow(/Caller failed/); const original = payload.beginPage(builder, 2, grant).page; if (!original) throw new Error("Parent lost original page");
      let reader: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayloadReader | null = null; for (const bytes of readers.admissionGrants) reader = builder.beginRead({ maxItems: 1, maxBytes: bytes }).reader; if (!reader) throw new Error("Missing original registered reader"); reader.advance({ maxItems: 1, maxBytes: 64 }); for (const bytes of readers.aliasGrants) reader.advance({ maxItems: 1, maxBytes: bytes }); native.lease.beginClose(); expect(reader.advance({ maxItems: 1, maxBytes: 1 })).toMatchObject({ kind: "byte", value: 77 }); expect(native.owner.takeRetirementWitness() !== null).toBe(fixture.earlyInstanceWitness); native.owner.beginClose(); expect(reader.advance({ maxItems: 1, maxBytes: 1 })).toMatchObject({ kind: "byte", value: 88 }); payload.beginClose(); expect(reader.advance(grant).kind).toBe("rejected"); expect(native.owner.closeStep({ maxItems: 0, maxBytes: 0 }).kind).toBe("blocked"); expect(original.terminalIsEmpty()).toBe(false); await residentAuthoredClosePayload(base); expect(reader.terminalIsEmpty()).toBe(true); expect(original.terminalIsEmpty()).toBe(fixture.lostWriterRecovered); expect(native.owner.takeRetirementWitness()).toBeNull(); let complete = false;
      for (let turn = 0; turn < scopes.firstChild.retirementTurns + 2; turn++) { const current = native.owner.closeStep(grant); expect(current.kind).not.toBe("rejected"); expect(current.bytes).toBeLessThanOrEqual(grant.maxBytes); if (current.kind === "complete") { complete = true; break; } } expect(complete).toBe(true); expect(scope.terminalIsEmpty()).toBe(true); expect(native.owner.takeRetirementWitness()).not.toBeNull();
    });
    //#endregion 💾️ResidentPoolTests

    //#region 🪪️ResidentMetadataTests
    it("OwnedResidentSlotContract declares the funded parent handoff and immutable concurrent roster oracle", async () => {
      const { default: contract } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️contract.json"); const { default: domain } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧬️schema.json");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️schema.json");
      const ajv = new Ajv({ strict: true, allErrors: true }); expect(ajv.compile(domain)(contract)).toBe(true); expect(ajv.compile(schema)(fixture)).toBe(true); expect(fixture.widths.map(field => field.name)).toEqual(contract.fields);
      expect(BigInt(contract.recordBytes) + fixture.widths.reduce((sum, field) => sum + BigInt(field.bytes), 0n)).toBe(BigInt(contract.charge.bytes));
      for (const axis of ["bytes", "slots", "owners"] as const) expect(BigInt(fixture.basePool[axis]) + BigInt(contract.charge[axis])).toBe(BigInt(fixture.proposedPool[axis]));
      let state: { slot: string | null; roster: string[]; fault: string | null } = { slot: null, roster: [], fault: null };
      for (const row of fixture.trace) {
        const previous = state; const previousRoster = [...state.roster]; state = produce(state, draft => {
          if (row.event === "admit" || row.event === "capture-retirement") { expect(draft.slot).toBeNull(); expect(draft.fault).toBeNull(); if (row.event === "capture-retirement") expect(draft.roster).toContain(row.owner); draft.slot = row.owner; }
          else { expect(draft.slot).toBe(row.owner); if (row.event === "register") { draft.roster.push(row.owner); draft.slot = null; } else if (row.event === "unlink-domain-empty") draft.roster.splice(draft.roster.indexOf(row.owner), 1); else if (row.event === "refunded-observed") draft.slot = null; else if (row.event === "fault") draft.fault = row.owner; }
        }); expect(previous.roster).toEqual(previousRoster); expect(state.slot).toBe(row.slot); expect(state.roster).toEqual(row.roster);
      }
      expect(state).toEqual({ slot: "b", roster: ["b"], fault: "b" }); expect(contract.reuse).toBe("only-empty-private-slot-after-exact-handoff-observation");
    });

    it("OwnedResidentConstruction installs the original pool before fallible field construction and retains exact faults", async () => {
      const { OwnedUiResidentPool, OwnedUiResidentPoolRetirement } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧪️fixture.json"); const { default: composition } = await import("../../../../../../../🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json");
      for (const frontier of fixture.poolConstructionFaults) {
        const native = await nativeInstanceFixture(); for (const bytes of composition.poolPreparation.prepareBytes) expect(native.client.prepareUiResidentPool(native.residentLedger, { maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes });
        type OwnedPool = import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPool;
        const retained = native.residentLedger.usage; const originalInstall = native.client.installUiResidentPool; const originalCapture = native.client.captureUiResidentPoolFault; const installed: { pool: OwnedPool | null } = { pool: null };
        const fault = Object.freeze({ frontier, bytes: Uint8Array.from([73, 19, 2]) }); let captured: unknown; let captureCount = 0;
        native.client.installUiResidentPool = function (pool, budget) { const current = originalInstall.call(this, pool, budget); if (this.ownsUiResidentPool(pool)) installed.pool = pool; return current; };
        native.client.captureUiResidentPoolFault = function (error) { originalCapture.call(this, error); captured = error; captureCount++; };
        const NativeMap = globalThis.WeakMap; const freeze = Object.freeze;
        if (frontier === "bindings-constructor") globalThis.WeakMap = new Proxy(NativeMap, { construct() { throw fault; } });
        else Object.freeze = <T,>(value: T): Readonly<T> => { if (frontier === "pool-finalizer" ? value instanceof OwnedUiResidentPool : value instanceof OwnedUiResidentPoolRetirement) throw fault; return freeze(value); };
        let result: ReturnType<typeof OwnedUiResidentPool.begin>; try { result = OwnedUiResidentPool.begin(native.client, native.residentLedger, grant); } finally { globalThis.WeakMap = NativeMap; Object.freeze = freeze; native.client.installUiResidentPool = originalInstall; native.client.captureUiResidentPoolFault = originalCapture; }
        expect(result.step.kind).toBe("rejected"); expect(result.pool).toBeNull(); expect(installed.pool).not.toBeNull(); const pool = installed.pool; if (!pool) throw new Error("Original pool shell was lost before field construction");
        expect(native.client.ownsUiResidentPool(pool)).toBe(true); expect(captureCount).toBe(1); expect(captured).toBe(fault); expect(native.residentLedger.usage).toEqual(retained);
        expect(native.client.closeUiResidentPoolStep({ maxItems: 1, maxBytes: 63 }).kind).toBe("blocked"); expect(native.client.closeUiResidentPoolStep(grant)).toMatchObject({ kind: "pending", phase: "resident-admission-fault-handoff", bytes: 64 });
        for (let turn = 0; turn < 4; turn++) { const current = native.client.closeUiResidentPoolStep(grant); expect(current.items).toBeLessThanOrEqual(1); expect(current.bytes).toBeLessThanOrEqual(4096); }
        expect(pool.retirement).toBeNull(); expect(pool.terminalIsEmpty()).toBe(false); expect(native.client.ownsUiResidentPool(pool)).toBe(true); expect(native.residentLedger.usage).toEqual(retained); native.client.disposeAll();
      }
    });

    it("OwnedResidentScope admits its original native lifetime through the parent-funded slot and retires exact registrations", async () => {
      const { OwnedUiResidentPool, OwnedUiResidentInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture.json");
      const { default: composition } = await import("../../../../../../../🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json"); const native = await nativeInstanceFixture(); for (const bytes of composition.poolPreparation.prepareBytes) expect(native.client.prepareUiResidentPool(native.residentLedger, { maxItems: 1, maxBytes: bytes }).kind).toBe("pending");
      const pool = OwnedUiResidentPool.begin(native.client, native.residentLedger, grant).pool; if (!pool) throw new Error("Missing original parent pool"); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const before = native.residentLedger.usage;
      const refused: unknown = Reflect.apply(pool.bindInstance, pool, [owner, native.activation, native.lifetime, { maxItems: 0, maxBytes: 4096 }]); expect(refused).toMatchObject({ step: { kind: "blocked", items: 0, bytes: 0 }, scope: null }); expect(native.residentLedger.usage).toEqual(before);
      let scope: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentInstance | null = null;
      for (let index = 0; index < fixture.firstChild.prepareBytes.length; index++) { const bytes = fixture.firstChild.prepareBytes[index]!; const last = index === fixture.firstChild.prepareBytes.length - 1; const current: unknown = Reflect.apply(pool.bindInstance, pool, [owner, native.activation, native.lifetime, { maxItems: 1, maxBytes: bytes }]); expect(current).toMatchObject({ step: { kind: last ? "ready" : "pending", items: 1, bytes } }); if (current && typeof current === "object" && "scope" in current && current.scope instanceof OwnedUiResidentInstance) scope = current.scope; if (!last) expect(scope).toBeNull(); }
      if (!scope) throw new Error("Original lifetime scope was not admitted"); expect(native.residentLedger.usage.data).toEqual(produce(before.data, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += fixture.firstChild.total[axis]; }));
      expect(Reflect.apply(pool.bindInstance, pool, [owner, native.activation, native.lifetime, grant])).toMatchObject({ step: { kind: "ready", items: 0, bytes: 0 }, scope }); scope.beginClose();
      for (let turn = 0; turn < fixture.firstChild.retirementTurns && !scope.terminalIsEmpty(); turn++) { const current = scope.closeStep(grant); expect(current.items).toBeLessThanOrEqual(1); expect(current.bytes).toBeLessThanOrEqual(4096); expect(current.kind).not.toBe("rejected"); }
      expect(scope.terminalIsEmpty()).toBe(true); expect(native.residentLedger.usage).toEqual(before);
      native.client.disposeAll();
    });

    it("OwnedResidentScope cancels every admitted prefix through its original parent slot", async () => {
      const { OwnedUiResidentPool } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture.json"); const { default: composition } = await import("../../../../../../../🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json");
      for (const prefix of fixture.cancellationPrefixes) {
        const native = await nativeInstanceFixture(); for (const bytes of composition.poolPreparation.prepareBytes) native.client.prepareUiResidentPool(native.residentLedger, { maxItems: 1, maxBytes: bytes }); const pool = OwnedUiResidentPool.begin(native.client, native.residentLedger, grant).pool; if (!pool) throw new Error("Missing original pool");
        const before = native.residentLedger.usage; const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
        for (const bytes of fixture.firstChild.prepareBytes.slice(0, prefix)) expect(pool.bindInstance(owner, native.activation, native.lifetime, { maxItems: 1, maxBytes: bytes }).step.kind).not.toBe("rejected");
        pool.beginClose(); for (let turn = 0; turn < fixture.firstChild.retirementTurns + 3 && !pool.terminalIsEmpty(); turn++) { const current = pool.closeStep(grant); expect(current.kind, `prefix ${prefix}, turn ${turn}: ${current.phase}`).not.toBe("rejected"); expect(current.bytes).toBeLessThanOrEqual(grant.maxBytes); expect(current.items).toBeLessThanOrEqual(1); }
        expect(pool.terminalIsEmpty(), `prefix ${prefix}`).toBe(true); expect(native.residentLedger.usage).toEqual(before); native.client.disposeAll();
      }
    });

    it("OwnedResidentScope retains exact installed shells and first faults across constructor and handoff failures", async () => {
      const { OwnedUiResidentPool, OwnedUiResidentInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { OwnedResidentRecord, OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture.json"); const { default: composition } = await import("../../../../../../../🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json");
      for (const frontier of fixture.constructionFaults) {
        const native = await nativeInstanceFixture(); for (const bytes of composition.poolPreparation.prepareBytes) native.client.prepareUiResidentPool(native.residentLedger, { maxItems: 1, maxBytes: bytes }); const pool = OwnedUiResidentPool.begin(native.client, native.residentLedger, grant).pool; if (!pool) throw new Error("Missing original pool"); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
        const prefix = frontier.startsWith("owner") ? 7 : frontier.startsWith("binding") ? 8 : 6; for (const bytes of fixture.firstChild.prepareBytes.slice(0, prefix)) pool.bindInstance(owner, native.activation, native.lifetime, { maxItems: 1, maxBytes: bytes }); const before = native.residentLedger.usage; const fault = { frontier, bytes: Uint8Array.of(73) }; const originalInstall = OwnedResidentRecord.prototype.install; const originalAttach = OwnedUiInstance.prototype.attachResidentScope; const originalSet = WeakMap.prototype.set; const originalFreeze = Object.freeze; let originalScope: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentInstance | null = null;
        if (frontier.startsWith("record")) OwnedResidentRecord.prototype.install = function(shell, budget) { if (shell instanceof OwnedUiResidentInstance) { originalScope = shell; if (frontier.endsWith("before")) throw fault; originalInstall.call(this, shell, budget); throw fault; } return originalInstall.call(this, shell, budget); };
        if (frontier.startsWith("owner")) OwnedUiInstance.prototype.attachResidentScope = function(scope) { if (this === owner) { originalScope = scope; if (frontier.endsWith("after")) originalAttach.call(this, scope); throw fault; } return originalAttach.call(this, scope); };
        if (frontier.startsWith("binding")) WeakMap.prototype.set = function(key, value) { if (key === owner && value instanceof OwnedUiResidentInstance) { originalScope = value; if (frontier.endsWith("after")) originalSet.call(this, key, value); throw fault; } return originalSet.call(this, key, value); };
        if (frontier === "scope-finalizer") Object.freeze = <T,>(value: T): Readonly<T> => { if (value instanceof OwnedUiResidentInstance) { originalScope = value; throw fault; } return originalFreeze(value); };
        try { expect(pool.bindInstance(owner, native.activation, native.lifetime, grant)).toMatchObject({ step: { kind: "rejected" }, scope: null }); } finally { OwnedResidentRecord.prototype.install = originalInstall; OwnedUiInstance.prototype.attachResidentScope = originalAttach; WeakMap.prototype.set = originalSet; Object.freeze = originalFreeze; }
        expect(originalScope).not.toBeNull(); expect(native.residentLedger.usage).toEqual(before); expect(pool.bindInstance(owner, native.activation, native.lifetime, grant)).toMatchObject({ step: { kind: "rejected" }, scope: null });
        const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const probe = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); });
        try { pool.beginClose(); for (let turn = 0; turn < fixture.firstChild.retirementTurns + 3; turn++) { const current = pool.closeStep(grant); expect(current.bytes).toBeLessThanOrEqual(grant.maxBytes); expect(current.items).toBeLessThanOrEqual(1); } } finally { probe.mockRestore(); }
        expect(retained).toContain(fault); expect(pool.terminalIsEmpty()).toBe(false); expect(pool.retirement).toBeNull(); expect(native.residentLedger.usage.data.bytes).toBeGreaterThanOrEqual(296); native.client.disposeAll();
      }
    });

    it("OwnedResidentScope serializes transitions without conflating independent native lifetimes", async () => {
      const { OwnedUiResidentPool } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture.json"); const { default: composition } = await import("../../../../../../../🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json");
      const native = await nativeInstanceFixture(); await native.answer(native.client.activate("native-ui-second", "https://fixture.invalid/native-ui.js", [], native.budget), undefined); const lease = native.client.captureInstanceLifecycle("native-ui-second", fixture.concurrent.instanceId); const captured = { kind: "captured" as const, lifetime: { activationGeneration: lease.activation.activationGeneration, instanceId: fixture.concurrent.instanceId, guestLifetime: BigInt(fixture.concurrent.guestLifetime) }, requestSequence: lease.openRequest.requestSequence };
      await native.answer(lease.open({ appId: "fixture", actor: "fixture", config: [], assets: [], capabilities: [], quotas: [] }, native.budget), { lifecycleReceipt: encodeActorInstanceLifecycle(captured), status: { tag: "idle" } }); await native.answer(lease.acknowledge(captured, native.budget), { status: { tag: "idle" } }); const lifetime = lease.lifetime; if (!lifetime) throw new Error("Missing second native lifetime");
      for (const bytes of composition.poolPreparation.prepareBytes) native.client.prepareUiResidentPool(native.residentLedger, { maxItems: 1, maxBytes: bytes }); const pool = OwnedUiResidentPool.begin(native.client, native.residentLedger, grant).pool; if (!pool) throw new Error("Missing original pool"); const before = native.residentLedger.usage;
      const ownerA = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const ownerB = new OwnedUiInstance(lease.activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
      for (const bytes of fixture.firstChild.prepareBytes) pool.bindInstance(ownerA, native.activation, native.lifetime, { maxItems: 1, maxBytes: bytes }); const scopeA = pool.bindInstance(ownerA, native.activation, native.lifetime, grant).scope; if (!scopeA) throw new Error("Missing first scope");
      pool.bindInstance(ownerB, lease.activation, lifetime, { maxItems: 1, maxBytes: fixture.firstChild.prepareBytes[0]! }); const busy = native.residentLedger.usage; scopeA.beginClose(); expect(scopeA.closeStep(grant)).toMatchObject({ kind: fixture.concurrent.busyKind, bytes: 0 }); expect(native.residentLedger.usage).toEqual(busy);
      for (const bytes of fixture.firstChild.prepareBytes.slice(1)) pool.bindInstance(ownerB, lease.activation, lifetime, { maxItems: 1, maxBytes: bytes }); const scopeB = pool.bindInstance(ownerB, lease.activation, lifetime, grant).scope; if (!scopeB) throw new Error("Missing second scope"); expect(scopeB).not.toBe(scopeA);
      expect(native.residentLedger.usage.data).toEqual(produce(before.data, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += fixture.firstChild.total[axis] * fixture.concurrent.liveScopes; }));
      expect(scopeA.closeStep(grant)).toMatchObject({ kind: "pending", bytes: 64 }); expect(pool.bindInstance(ownerB, lease.activation, lifetime, grant)).toMatchObject({ scope: scopeB, step: { kind: "ready", bytes: 0 } }); scopeB.beginClose(); expect(scopeB.closeStep(grant).kind).toBe(fixture.concurrent.busyKind);
      for (let turn = 0; turn < fixture.firstChild.retirementTurns && !scopeA.terminalIsEmpty(); turn++) scopeA.closeStep(grant); expect(scopeA.terminalIsEmpty()).toBe(true); expect(scopeB.terminalIsEmpty()).toBe(false); for (let turn = 0; turn < fixture.firstChild.retirementTurns && !scopeB.terminalIsEmpty(); turn++) scopeB.closeStep(grant); expect(scopeB.terminalIsEmpty()).toBe(true); expect(native.residentLedger.usage).toEqual(before); native.client.disposeAll();
    });

    it("OwnedResidentScope closes known no-cell admission outcomes without consuming unrelated owners", async () => {
      const { OwnedUiResidentPool } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture.json"); const { default: composition } = await import("../../../../../../../🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json");
      for (const row of fixture.noCell) {
        const native = await nativeInstanceFixture(); for (const bytes of composition.poolPreparation.prepareBytes) native.client.prepareUiResidentPool(native.residentLedger, { maxItems: 1, maxBytes: bytes }); const pool = OwnedUiResidentPool.begin(native.client, native.residentLedger, grant).pool; if (!pool) throw new Error("Missing original pool"); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
        if (row.reason === "ledger-closing") native.residentLedger.beginClose(); else { const consumer = {}; expect(native.residentLedger.prepareAdmission(consumer, "data", grant).kind).toBe("pending"); const cell = native.residentLedger.preparedAdmission(consumer); if (!cell) throw new Error("Missing independent admission"); native.residentLedger.claimAdmission(consumer, cell, grant); const capacity = native.residentLedger.capacity; const used = native.residentLedger.usage.data; const amount = capacity.bytes - capacity.control.bytes - used.bytes - 264; expect(native.residentLedger.reserveRecord("data", { bytes: amount, slots: 1, owners: 1 }, cell, grant).step.kind).toBe("ready"); }
        const before = native.residentLedger.usage; expect(pool.bindInstance(owner, native.activation, native.lifetime, grant)).toMatchObject({ step: { kind: row.kind, bytes: 0 }, scope: null }); expect(native.residentLedger.usage).toEqual(before);
        pool.beginClose(); for (let turn = 0; turn < 3 && !pool.terminalIsEmpty(); turn++) pool.closeStep(grant); expect(pool.terminalIsEmpty(), row.reason).toBe(true); expect(native.residentLedger.usage).toEqual(before); native.client.disposeAll();
      }
    });

    it("OwnedResidentScope retains before-and-after canonical admission wrapper faults without a false pending close", async () => {
      const { OwnedUiResidentPool } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { OwnedResidentLedger, OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture.json"); const { default: composition } = await import("../../../../../../../🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json");
      for (const row of fixture.admissionFaults) {
        const native = await nativeInstanceFixture(); for (const bytes of composition.poolPreparation.prepareBytes) native.client.prepareUiResidentPool(native.residentLedger, { maxItems: 1, maxBytes: bytes }); const pool = OwnedUiResidentPool.begin(native.client, native.residentLedger, grant).pool; if (!pool) throw new Error("Missing original pool"); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
        for (const bytes of fixture.firstChild.prepareBytes.slice(0, row.prefix)) pool.bindInstance(owner, native.activation, native.lifetime, { maxItems: 1, maxBytes: bytes }); const fault = { row, bytes: Uint8Array.of(73) }; const prepare = OwnedResidentLedger.prototype.prepareAdmission; const claim = OwnedResidentLedger.prototype.claimAdmission; const record = OwnedResidentLedger.prototype.reserveRecord;
        if (row.method === "prepareAdmission") OwnedResidentLedger.prototype.prepareAdmission = function(...args) { if (this !== native.residentLedger) return prepare.apply(this, args); if (row.after) prepare.apply(this, args); throw fault; };
        if (row.method === "claimAdmission") OwnedResidentLedger.prototype.claimAdmission = function(...args) { if (this !== native.residentLedger) return claim.apply(this, args); if (row.after) claim.apply(this, args); throw fault; };
        if (row.method === "reserveRecord") OwnedResidentLedger.prototype.reserveRecord = function(...args) { if (this !== native.residentLedger) return record.apply(this, args); if (row.after) record.apply(this, args); throw fault; };
        try { expect(pool.bindInstance(owner, native.activation, native.lifetime, grant)).toMatchObject({ step: { kind: "rejected" }, scope: null }); } finally { OwnedResidentLedger.prototype.prepareAdmission = prepare; OwnedResidentLedger.prototype.claimAdmission = claim; OwnedResidentLedger.prototype.reserveRecord = record; }
        const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const probe = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); });
        try { pool.beginClose(); let failureSeen = false; for (let turn = 0; turn < fixture.firstChild.retirementTurns + 3; turn++) { const current = pool.closeStep(grant); expect(current.kind, `${row.method}/${row.after}: ${current.phase}`).not.toBe("blocked"); if (current.kind === "rejected") failureSeen = true; expect(current.bytes).toBeLessThanOrEqual(grant.maxBytes); } expect(failureSeen).toBe(true); } finally { probe.mockRestore(); }
        if (row.method !== "prepareAdmission" || row.after) expect(retained).toContain(fault); expect(pool.terminalIsEmpty()).toBe(false); expect(pool.retirement).toBeNull(); native.client.disposeAll();
      }
    });

    it("OwnedResidentScope rejects a genuine foreign composition even when it shares the same ledger and numeric lifetime", async () => {
      const { OwnedUiResidentPool } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture.json"); const { default: composition } = await import("../../../../../../../🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json");
      const native = await nativeInstanceFixture(); const foreign = await nativeInstanceFixture(native.residentLedger); expect(foreign.residentLedger === native.residentLedger).toBe(fixture.foreignComposition.sameLedger); expect(foreign.lifetime).toEqual(native.lifetime);
      for (const bytes of composition.poolPreparation.prepareBytes) native.client.prepareUiResidentPool(native.residentLedger, { maxItems: 1, maxBytes: bytes }); const pool = OwnedUiResidentPool.begin(native.client, native.residentLedger, grant).pool; if (!pool) throw new Error("Missing original pool"); const owner = new OwnedUiInstance(foreign.activation, foreign.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const before = native.residentLedger.usage;
      expect(pool.bindInstance(owner, foreign.activation, foreign.lifetime, grant)).toMatchObject({ step: { kind: fixture.foreignComposition.kind, bytes: fixture.foreignComposition.workBytes }, scope: null });
      let proxyReads = 0; const proxy = new Proxy(foreign.activation, { get() { proxyReads++; throw new Error("Foreign activation was inspected"); } }); expect(pool.bindInstance(owner, proxy, foreign.lifetime, grant).step.kind).toBe("rejected"); expect(proxyReads).toBe(fixture.foreignComposition.proxyReads);
      expect(native.residentLedger.usage).toEqual(before); pool.beginClose(); expect(pool.closeStep(grant).kind).toBe("complete"); expect(pool.terminalIsEmpty()).toBe(true); native.client.disposeAll(); foreign.client.disposeAll();
    });

    it("OwnedResidentComposition retains the exact prepared pool and separates metadata admission from construction and final refund", async () => {
      const { OwnedUiResidentPool } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { uiResidentMetadataEnvelope } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🟦️component.ts");
      const { default: composition } = await import("../../../../../../../🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json"); const native = await nativeInstanceFixture(); const before = native.residentLedger.usage.data;
      for (const bytes of composition.poolPreparation.prepareBytes) expect(OwnedUiResidentPool.begin(native.client, native.residentLedger, { maxItems: 1, maxBytes: bytes })).toMatchObject({ step: { kind: "pending", items: 1, bytes }, pool: null });
      expect(uiResidentMetadataEnvelope("pool")).toEqual(composition.poolPreparation.uiEnvelope); expect(native.residentLedger.usage.data).toEqual(produce(before, draft => { draft.bytes += composition.poolPreparation.total.bytes; draft.slots += composition.poolPreparation.total.slots; draft.owners += composition.poolPreparation.total.owners; }));
      const admitted = OwnedUiResidentPool.begin(native.client, native.residentLedger, grant); expect(admitted.step.kind).toBe("ready"); const pool = admitted.pool; if (!pool) throw new Error("Missing prepared pool");
      expect(OwnedUiResidentPool.begin(native.client, native.residentLedger, grant)).toMatchObject({ step: { kind: "rejected" }, pool: null }); expect(() => Reflect.construct(OwnedUiResidentPool, [{ bytes: 65536, slots: 512, owners: 512 }])).toThrow();
      expect(pool.retirement).toBeNull(); close(pool); const witness = pool.retirement; expect(witness).not.toBeNull(); expect(native.client.releaseUiResidentPool(pool, {}, grant).kind).toBe("rejected");
      for (let index = 0; index < composition.poolLifecycle.releaseBytes.length; index++) { const bytes = composition.poolLifecycle.releaseBytes[index]!; expect(native.client.releaseUiResidentPool(pool, witness, { maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: composition.poolLifecycle.releaseKinds[index], bytes }); }
      const retained = produce(before, draft => { draft.bytes += composition.poolPreparation.controllerTotal.bytes; draft.slots += composition.poolPreparation.controllerTotal.slots; draft.owners += composition.poolPreparation.controllerTotal.owners; });
      expect(native.client.ownsUiResidentPool(pool)).toBe(false); expect(native.residentLedger.usage.data).toEqual(retained); native.client.disposeAll(); expect(native.residentLedger.usage.data).toEqual(retained);
    });

    async function residentPayloadFixture(fixtureIndex = 2, sharedLedger?: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentLedger) {
      const { OwnedUiResidentPool } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️fixture.json"); const { default: scopes } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture.json"); const { default: composition } = await import("../../../../../../../🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json");
      const native = await nativePagedFieldFixture(fixtureIndex, sharedLedger); for (const bytes of composition.poolPreparation.prepareBytes) expect(native.client.prepareUiResidentPool(native.residentLedger, { maxItems: 1, maxBytes: bytes }).kind).toBe("pending"); const pool = OwnedUiResidentPool.begin(native.client, native.residentLedger, grant).pool; if (!pool) throw new Error("Missing original pool"); const baseline = pool.usage;
      let scope: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentInstance | null = null; for (const bytes of scopes.firstChild.prepareBytes) scope = pool.bindInstance(native.owner, native.activation, native.lifetime, { maxItems: 1, maxBytes: bytes }).scope; if (!scope) throw new Error("Missing original scope");
      return { native, pool, scope, fixture, baseline };
    }
    async function residentBuilderFixture(fixtureIndex = 2, sharedLedger?: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentLedger) {
      const base = await residentPayloadFixture(fixtureIndex, sharedLedger); const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { default: builderFixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { default: evidenceFixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️fixture.json");
      let payload: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayload | null = null; for (const bytes of base.fixture.admissionBytes) payload = base.scope.beginPayload(base.native.field, { maxItems: 1, maxBytes: bytes }).payload; if (!payload) throw new Error("Missing original payload"); let builder: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts").OwnedUiOperationPayloadBuilder | null = null; for (const bytes of builderFixture.grants) builder = OwnedUiOperationPayloadBuilder.begin(base.native.owner, base.native.activation, base.native.lifetime, base.native.field, payload, { maxItems: 1, maxBytes: bytes }).builder; if (!builder) throw new Error("Missing original builder"); return { ...base, payload, builder, evidenceFixture };
    }
    async function residentEvidenceFixture() { const base = await residentBuilderFixture(); base.builder.beginClose(); return base; }
    it("OwnedResidentPayloadAdmission admits an exact captured field in separate grants and waits for two-way source retirement", async () => {
      const { OwnedUiResidentPool, OwnedUiResidentPayload } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️fixture.json"); const { default: scopes } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📨️slot/🧪️fixture.json"); const { default: composition } = await import("../../../../../../../🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json");
      const native = await nativePagedFieldFixture(); for (const bytes of composition.poolPreparation.prepareBytes) expect(native.client.prepareUiResidentPool(native.residentLedger, { maxItems: 1, maxBytes: bytes }).kind).toBe("pending"); const pool = OwnedUiResidentPool.begin(native.client, native.residentLedger, grant).pool; if (!pool) throw new Error("Missing original pool");
      let scope: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentInstance | null = null; for (const bytes of scopes.firstChild.prepareBytes) scope = pool.bindInstance(native.owner, native.activation, native.lifetime, { maxItems: 1, maxBytes: bytes }).scope; if (!scope) throw new Error("Missing original scope");
      const before = pool.usage; expect(scope.beginPayload(native.field, { maxItems: 0, maxBytes: 4096 })).toMatchObject({ step: { kind: "blocked", bytes: 0 }, payload: null }); expect(pool.usage).toEqual(before);
      let payload: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayload | null = null;
      for (let index = 0; index < fixture.admissionBytes.length; index++) { const bytes = fixture.admissionBytes[index]!; const admitted = scope.beginPayload(native.field, { maxItems: 1, maxBytes: bytes }); expect(admitted.step).toMatchObject({ kind: index === fixture.admissionBytes.length - 1 ? "ready" : "pending", bytes }); expect(admitted.payload !== null).toBe(index === fixture.admissionBytes.length - 1); payload = admitted.payload; }
      if (!payload) throw new Error("Exact field did not publish its payload"); expect(OwnedUiResidentPayload.matchesField(payload, native.field)).toBe(true); expect(OwnedUiResidentPayload.matchesScope(payload, scope)).toBe(true); expect(native.field.residentPayload(scope)).toBe(payload); expect(scope.beginPayload(native.field, grant)).toMatchObject({ step: { kind: "ready", bytes: 0 }, payload });
      const charged = produce(before, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += fixture.expectedPayloadTotal[axis]; }); expect(pool.usage).toEqual(charged); close(payload); expect(native.field.residentPayload(scope)).toBeNull(); expect(payload.terminalIsEmpty()).toBe(true); expect(pool.usage).toEqual(before); close(scope); close(pool); native.client.disposeAll();
    });

    it("OwnedResidentPayloadSettledField rejects the original settled field before another admission", async () => {
      const { native, pool, scope, fixture } = await residentPayloadFixture(); let payload: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayload | null = null;
      for (const bytes of fixture.admissionBytes) payload = scope.beginPayload(native.field, { maxItems: 1, maxBytes: bytes }).payload; if (!payload) throw new Error("Missing original payload"); close(payload);
      const observation = native.field.residentPayloadDetachment; expect(observation).not.toBeNull(); const expected = produce(pool.usage, () => {});
      for (let attempt = 0; attempt < fixture.settledField.attempts; attempt++) { const current = scope.beginPayload(native.field, grant); expect(current).toMatchObject({ step: { kind: fixture.settledField.kind, bytes: fixture.settledField.bytes }, payload: null }); expect(pool.usage).toEqual(expected); expect(native.field.residentPayloadDetachment).toBe(observation); }
      close(scope); close(pool); native.client.disposeAll();
    });


it("OwnedResidentPageBinding keeps the original page in its builder until parent-authorized detachment", async () => {
      const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🔗️binding/🧪️fixture.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🔗️binding/🧪️schema.json"); const { default: contract } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🔗️binding/🧬️contract.json"); const { default: domain } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🔗️binding/🧬️schema.json"); const { default: Ajv } = await import("ajv"); const ajv = new Ajv({ strict: true }); expect(ajv.validate(schema, fixture)).toBe(true); expect(ajv.validate(domain, contract)).toBe(true);
      const { payload, builder, page, pool, pageFixture: pages } = await residentPageFixture(); expect(OwnedUiOperationPayloadBuilder.matchesPage(builder, page, payload)).toBe(true); const held = pool.usage; expect(payload.beginPage(builder, 1, grant)).toMatchObject({ step: { kind: "ready", bytes: 0 }, page }); expect(pool.usage).toEqual(held);
      expect(page.allocate(grant).kind).toBe("ready"); expect(page.writeByte(73, grant).kind).toBe("pending"); expect(page.seal(grant).kind).toBe("ready");
      for (let index = 0; index < fixture.closeGrants.length; index++) { const current = payload.closePage(page, { maxItems: 1, maxBytes: fixture.closeGrants[index]! }); expect(current, fixture.closePhases[index]).toMatchObject({ kind: index === fixture.closeGrants.length - 1 ? "complete" : "pending", bytes: fixture.closeGrants[index] }); expect(OwnedUiOperationPayloadBuilder.matchesPage(builder, page, payload)).toBe(index < 1); if (index <= 2) expect(pool.usage).toEqual(held); }
      expect(page.terminalIsEmpty()).toBe(true); expect(pool.usage).toEqual(produce(held, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] -= pages.total[axis]; }));
    });

it("OwnedResidentPageProducer admits its original page through the shared parent before allocation", async () => {
      const { builder, payload, pool, reader } = await residentReaderFixture(); const { default: pages } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); const { default: binding } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🔗️binding/🧪️fixture.json"); const before = pool.usage; expect(builder.advance(grant)).toMatchObject({ kind: "pending", bytes: 128 });
      for (const bytes of [...pages.storageOwnerGrants, ...binding.admissionGrants]) expect(builder.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes });
      expect(pool.usage).toEqual(produce(before, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += pages.storageOwnerTotal[axis] + pages.total[axis]; })); expect(payload.beginReader(builder, grant).reader).toBe(reader); expect(reader.advance(grant).kind).toBe("blocked");
    });

it("OwnedResidentPageProducerWindow copies two original windows with separate scalar grants before source EOF", async () => {
      const { payload, builder, reader, pool, native, readerFixture: readers } = await residentReaderFixture(); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🔗️binding/🧪️fixture.json"); const { default: pages } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); const { Buffer } = await import("node:buffer"); const before = pool.usage; const actual: number[] = []; const work = fixture.producer;
      expect(builder.advance({ maxItems: 1, maxBytes: work.fragmentCapture })).toMatchObject({ kind: "pending", bytes: work.fragmentCapture });
      for (let window = 0; window < fixture.stream.windows.length; window++) {
        const length = fixture.stream.windows[window]!; for (const bytes of [...(window === 0 ? pages.storageOwnerGrants : []), ...fixture.admissionGrants]) expect(builder.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); const page = payload.beginPage(builder, length, grant).page; if (!page) throw new Error("Missing automatic original page");
        for (const bytes of [work.pageObservation, work.allocation, work.allocationObservation]) expect(builder.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); for (let index = 0; index < length; index++) { expect(builder.advance({ maxItems: 1, maxBytes: work.sourceRead })).toMatchObject({ kind: "pending", bytes: work.sourceRead, phase: "paged-input-byte" }); expect(builder.advance({ maxItems: 1, maxBytes: work.destinationWrite })).toMatchObject({ kind: "pending", bytes: work.destinationWrite }); }
        for (const bytes of [work.seal, work.sealObservation]) expect(builder.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); if (window === 0) expect(builder.advance(grant)).toMatchObject({ kind: "blocked", bytes: 0, phase: "paged-page-window" });
        reader.advance({ maxItems: 1, maxBytes: 64 }); for (const bytes of readers.aliasGrants) expect(reader.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); for (let index = 0; index < length; index++) { const current = reader.advance({ maxItems: 1, maxBytes: 1 }); if (current.kind !== "byte") throw new Error("Missing automatic byte"); actual.push(current.value); } reader.advance({ maxItems: 1, maxBytes: 64 }); for (const bytes of readers.aliasCloseGrants) expect(reader.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); for (const bytes of fixture.closeGrants) expect(reader.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); expect(reader.advance({ maxItems: 1, maxBytes: 64 }).kind).toBe("pending"); expect(reader.advance(grant).kind).toBe("blocked");
        expect(page.terminalIsEmpty()).toBe(true); expect(pool.usage).toEqual(produce(before, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += pages.storageOwnerTotal[axis]; })); expect(native.field.consumed).toBe(BigInt(fixture.stream.sourceConsumptionAtFirstWindow)); expect(native.field.complete).toBe(false);
      }
      expect(Buffer.from(actual)).toEqual(Buffer.from(native.payload)); expect(builder.advance({ maxItems: 1, maxBytes: work.inputDetach })).toMatchObject({ kind: "pending", bytes: work.inputDetach }); expect(builder.advance(grant)).toMatchObject({ kind: "pending", bytes: 296, phase: "resident-admission-bootstrap" }); expect(native.field.fragment).toBe(native.fragment); expect(native.field.consumed).toBe(0n);
    });

it("OwnedResidentPageBindingFault retains original install and detach wrapper failures without refund", async () => {
      const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🔗️binding/🧪️fixture.json"); const { default: pages } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json");
      for (const row of fixture.faults) {
        const { payload, builder, pool } = await residentBuilderFixture(); const count = row.method === "install" ? fixture.admissionPhases.indexOf("builder-install") : fixture.admissionGrants.length; let page: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage | null = null; for (const bytes of [...pages.storageOwnerGrants, ...fixture.admissionGrants.slice(0, count)]) page = payload.beginPage(builder, 1, { maxItems: 1, maxBytes: bytes }).page; const held = pool.usage; if (row.method === "detach") { if (!page) throw new Error("Missing original binding page"); expect(payload.closePage(page, grant).kind).toBe("pending"); }
        const fault = Object.freeze({ row, backing: Uint8Array.of(73) }); const install = OwnedUiOperationPayloadBuilder.installPage; const detach = OwnedUiOperationPayloadBuilder.detachPage; const probe = row.method === "install" ? vi.spyOn(OwnedUiOperationPayloadBuilder, "installPage").mockImplementation((builder, page, resident, budget) => { if (row.after) expect(install(builder, page, resident, budget).bytes).toBe(64); throw fault; }) : vi.spyOn(OwnedUiOperationPayloadBuilder, "detachPage").mockImplementation((builder, page, proof, resident, budget) => { if (row.after) expect(detach(builder, page, proof, resident, budget).bytes).toBe(64); throw fault; });
        try { expect(row.method === "install" ? payload.beginPage(builder, 1, grant).step.kind : payload.closePage(page!, grant).kind).toBe("rejected"); } finally { probe.mockRestore(); } expect(payload.beginPage(builder, 1, grant).step.kind).toBe("rejected");
        const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const capture = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); }); try { payload.beginClose(); expect(payload.closeStep(grant).kind).toBe("pending"); expect(payload.closeStep(grant).kind).toBe("rejected"); } finally { capture.mockRestore(); } expect(retained).toEqual([fault]); expect(pool.usage).toEqual(held); if (page) expect(page.terminalIsEmpty()).toBe(false);
      }
    });
    it("OwnedResidentPageBindingIdentity rejects foreign and replayed proof before original alias mutation", async () => {
      const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const base = await residentPageFixture(); const foreign = await residentPageFixture(); const { builder, page, payload, pool, pageFixture } = base; const held = pool.usage; expect(OwnedUiOperationPayloadBuilder.detachPage(builder, page, {}, payload, grant).kind).toBe("rejected"); expect(OwnedUiOperationPayloadBuilder.matchesPage(builder, page, payload)).toBe(true); payload.closePage(page, grant); let proof: unknown; const original = OwnedUiOperationPayloadBuilder.detachPage;
      const probe = vi.spyOn(OwnedUiOperationPayloadBuilder, "detachPage").mockImplementation((owner, actual, witness, resident, budget) => { proof = witness; expect(original(foreign.builder, actual, witness, resident, budget).kind).toBe("rejected"); expect(original(owner, foreign.page, witness, resident, budget).kind).toBe("rejected"); expect(original(owner, actual, witness, foreign.payload, budget).kind).toBe("rejected"); expect(original(owner, actual, witness, resident, { maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(OwnedUiOperationPayloadBuilder.matchesPage(owner, actual, resident)).toBe(true); return original(owner, actual, witness, resident, budget); }); try { expect(payload.closePage(page, grant)).toMatchObject({ kind: "pending", bytes: 64 }); } finally { probe.mockRestore(); } expect(pool.usage).toEqual(held);
      for (let index = 2; index < pageFixture.closing.grants.length; index++) if (index !== pageFixture.closing.unallocatedSkip) payload.closePage(page, { maxItems: 1, maxBytes: pageFixture.closing.grants[index]! }); expect(page.terminalIsEmpty()).toBe(true); expect(original(builder, page, proof, payload, grant).kind).toBe("rejected"); let replacement: typeof page | null = null; for (const bytes of pageFixture.admissionGrants) replacement = payload.beginPage(builder, 1, { maxItems: 1, maxBytes: bytes }).page; if (!replacement) throw new Error("Missing replacement page"); const nextHeld = pool.usage; expect(original(builder, replacement, proof, payload, grant).kind).toBe("rejected"); expect(OwnedUiOperationPayloadBuilder.matchesPage(builder, replacement, payload)).toBe(true); expect(pool.usage).toEqual(nextHeld);
    });

it("OwnedResidentPageBindingChild preserves refusal and separates terminal child work from observation", async () => {
      const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🔗️binding/🧪️fixture.json"); const { default: pages } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json");
      for (const method of ["install", "detach"] as const) for (const row of fixture.childResults) {
        const { payload, builder, pool } = await residentBuilderFixture(); const count = method === "install" ? fixture.admissionPhases.indexOf("builder-install") : fixture.admissionGrants.length; let page: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage | null = null; for (const bytes of [...pages.storageOwnerGrants, ...fixture.admissionGrants.slice(0, count)]) page = payload.beginPage(builder, 1, { maxItems: 1, maxBytes: bytes }).page; if (method === "detach") { if (!page) throw new Error("Missing child page"); payload.closePage(page, grant); } const held = pool.usage; const install = OwnedUiOperationPayloadBuilder.installPage; const detach = OwnedUiOperationPayloadBuilder.detachPage; const current = () => method === "install" ? payload.beginPage(builder, 1, grant).step : payload.closePage(page!, grant);
        const returned = () => ({ kind: row.kind === "complete" ? "complete" as const : row.kind === "blocked" ? "blocked" as const : "rejected" as const, bytes: row.bytes, items: row.bytes ? 1 : 0, phase: "exact-page-binding-child" }); const probe = method === "install" ? vi.spyOn(OwnedUiOperationPayloadBuilder, "installPage").mockImplementation((builder, page, resident, budget) => { if (row.actual) expect(install(builder, page, resident, budget).bytes).toBe(64); return returned(); }) : vi.spyOn(OwnedUiOperationPayloadBuilder, "detachPage").mockImplementation((builder, page, proof, resident, budget) => { if (row.actual) expect(detach(builder, page, proof, resident, budget).bytes).toBe(64); return returned(); }); try { expect(current()).toMatchObject({ kind: row.expected, bytes: row.bytes }); expect(probe).toHaveBeenCalledTimes(1); } finally { probe.mockRestore(); } expect(pool.usage).toEqual(held);
        if (row.actual) expect(current()).toMatchObject({ kind: "pending", bytes: 64, phase: method === "install" ? "resident-page-builder-observation" : "resident-page-builder-detach-observation" }); else if (row.kind === "blocked") expect(current()).toMatchObject({ kind: "pending", bytes: 64 }); expect(pool.usage).toEqual(held);
      }
    });

it("OwnedResidentPageProducerFault retains the exact read or write fault without replaying an offset", async () => {
      const { OwnedKernelReturnInputFragment } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { OwnedUiResidentPage } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🔗️binding/🧪️fixture.json"); const { default: pages } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json");
      for (const row of fixture.producerFaults) {
        const { builder, payload, reader, pool } = await residentReaderFixture(); builder.advance(grant); for (const bytes of [...pages.storageOwnerGrants, ...fixture.admissionGrants, fixture.producer.pageObservation, fixture.producer.allocation, fixture.producer.allocationObservation]) expect(builder.advance({ maxItems: 1, maxBytes: bytes }).kind).toBe("pending"); if (row.method === "destination-write") expect(builder.advance({ maxItems: 1, maxBytes: 1 }).kind).toBe("pending");
        const held = pool.usage; const fault = Object.freeze({ row, bytes: Uint8Array.of(73) }); const offsets: number[] = []; const read = OwnedKernelReturnInputFragment.prototype.byteAt; const write = OwnedUiResidentPage.prototype.writeByte; const probe = row.method === "source-read" ? vi.spyOn(OwnedKernelReturnInputFragment.prototype, "byteAt").mockImplementation(function(this: import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts").OwnedKernelReturnInputFragment, index, owner) { offsets.push(index); if (row.after) expect(read.call(this, index, owner)).toBe(19); throw fault; }) : vi.spyOn(OwnedUiResidentPage.prototype, "writeByte").mockImplementation(function(this: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage, value, budget) { if (row.after) expect(write.call(this, value, budget)).toMatchObject({ kind: "pending", bytes: 1 }); throw fault; });
        try { expect(builder.advance({ maxItems: 1, maxBytes: 1 }).kind).toBe("rejected"); expect(builder.failure).toBe(fault); expect(builder.advance(grant).kind).toBe("rejected"); expect(probe).toHaveBeenCalledTimes(1); } finally { probe.mockRestore(); } expect(offsets).toEqual(row.method === "source-read" ? [0] : []); expect(pool.usage).toEqual(held); payload.beginClose(); for (let turn = 0; turn < 3; turn++) payload.closeStep(grant); expect(pool.usage).toEqual(held); expect(builder.failure).toBe(fault); expect(reader.terminalIsEmpty()).toBe(false); expect(payload.terminalIsEmpty()).toBe(false);
      }
    });

    it("OwnedResidentPageAdmission charges the original storage owner and complete fixed page before allocation", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️schema.json"); const { default: Ajv } = await import("ajv"); const { Buffer } = await import("node:buffer"); const ajv = new Ajv({ strict: true }); expect(ajv.validate(schema, fixture)).toBe(true); expect(48n + 8n * BigInt(fixture.stateFields.length + fixture.facadeFields.length + fixture.witnessFields.length)).toBe(BigInt(fixture.domain.bytes));
      for (const length of fixture.lengths) {
        const { payload, builder, pool } = await residentBuilderFixture(); const baseline = pool.usage; expect(payload.beginPage(builder, length, { maxItems: 0, maxBytes: 4096 })).toMatchObject({ step: { kind: "blocked", bytes: 0 }, page: null }); expect(pool.usage).toEqual(baseline);
        for (const bytes of fixture.storageOwnerGrants) { expect(payload.beginPage(builder, length, { maxItems: 1, maxBytes: bytes - 1 })).toMatchObject({ step: { kind: "blocked", bytes: 0 }, page: null }); expect(payload.beginPage(builder, length, { maxItems: 1, maxBytes: bytes })).toMatchObject({ step: { kind: "pending", bytes }, page: null }); }
        expect(pool.usage).toEqual(produce(baseline, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += fixture.storageOwnerTotal[axis]; }));
        let page: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage | null = null;
        for (let index = 0; index < fixture.admissionGrants.length; index++) { const bytes = fixture.admissionGrants[index]!; const held = pool.usage; expect(payload.beginPage(builder, length, { maxItems: 1, maxBytes: bytes - 1 })).toMatchObject({ step: { kind: "blocked", bytes: 0 }, page: null }); expect(pool.usage).toEqual(held); const current = payload.beginPage(builder, length, { maxItems: 1, maxBytes: bytes }); expect(current.step, fixture.admissionPhases[index]).toMatchObject({ kind: index === fixture.admissionGrants.length - 1 ? "ready" : "pending", bytes }); expect(current.page !== null).toBe(index === fixture.admissionGrants.length - 1); page = current.page; }
        if (!page) throw new Error("Missing original admitted page"); const held = produce(baseline, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += fixture.storageOwnerTotal[axis] + fixture.total[axis]; }); expect(pool.usage).toEqual(held); expect(Reflect.get(page, "capture")).toBeUndefined(); expect(Reflect.get(page, "byteAt")).toBeUndefined();
        expect(page.allocate({ maxItems: 1, maxBytes: fixture.allocateGrant - 1 })).toMatchObject({ kind: "blocked", bytes: 0 }); expect(page.allocate(grant)).toMatchObject({ kind: "ready", bytes: fixture.allocateGrant }); const oracle = Buffer.from(Array.from({ length }, (_, index) => (73 * index + 19) & 255)); for (const value of oracle) expect(page.writeByte(value, grant)).toMatchObject({ kind: "pending", bytes: 1 }); expect(page.seal(grant)).toMatchObject({ kind: "ready", bytes: 64 }); expect(page.writeByte(1, grant).kind).toBe("rejected"); expect(pool.usage).toEqual(held);
      }
    });

    it("OwnedResidentPageRetirement preserves zero-grant authority and refunds each original alias separately", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️schema.json"); const { default: Ajv } = await import("ajv"); expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
      for (const length of fixture.lengths) for (const allocated of [false, true]) {
        const { payload, builder, pool } = await residentBuilderFixture(); const before = pool.usage;
        let page: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage | null = null; for (const bytes of [...fixture.storageOwnerGrants, ...fixture.admissionGrants]) page = payload.beginPage(builder, length, { maxItems: 1, maxBytes: bytes }).page; if (!page) throw new Error("Missing admitted page");
        if (allocated) { expect(page.allocate(grant).kind).toBe("ready"); for (let index = 0; index < length; index++) expect(page.writeByte(index & 255, grant).kind).toBe("pending"); expect(page.seal(grant).kind).toBe("ready"); }
        let expected = pool.usage; expect(Reflect.get(page, "closeStep")).toBeUndefined(); expect(payload.closePage(page, { maxItems: 0, maxBytes: 4096 })).toMatchObject({ kind: "blocked", bytes: 0 }); expect(pool.usage).toEqual(expected);
        if (allocated) expect(page.seal(grant).kind).toBe("ready");
        for (let index = 0; index < fixture.closing.grants.length; index++) {
          if (!allocated && index === fixture.closing.unallocatedSkip) continue;
          const bytes = fixture.closing.grants[index]!; expect(payload.closePage(page, { maxItems: 0, maxBytes: bytes })).toMatchObject({ kind: "blocked", bytes: 0 }); expect(pool.usage).toEqual(expected);
          const result = payload.closePage(page, { maxItems: 1, maxBytes: bytes }); expect(result, fixture.closing.phases[index]).toMatchObject({ kind: index === fixture.closing.grants.length - 1 ? "complete" : "pending", bytes });
          expected = produce(expected, draft => { for (const refund of fixture.closing.refunds) if (refund.at === index) for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] -= refund[axis]; }); expect(pool.usage, fixture.closing.phases[index]).toEqual(expected); expect(page.terminalIsEmpty()).toBe(index === fixture.closing.grants.length - 1);
        }
        expect(pool.usage).toEqual(produce(before, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += fixture.storageOwnerTotal[axis]; })); expect(payload.closePage(page, grant)).toMatchObject({ kind: fixture.replay.retiredRepeat, bytes: 0 });
      }
    });

    it("OwnedResidentPageCancellation closes every original owner and page admission prefix through its payload", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); const { default: builderFixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json");
      const admission = [...fixture.storageOwnerGrants, ...fixture.admissionGrants]; expect(fixture.cancellation.prefixes).toEqual(Array.from({ length: admission.length + 1 }, (_, index) => index));
      for (const prefix of fixture.cancellation.prefixes) {
        const { payload, builder, pool, native, fixture: payloadFixture } = await residentBuilderFixture(); const baseline = pool.usage; for (let index = 0; index < prefix; index++) expect(payload.beginPage(builder, 1, { maxItems: 1, maxBytes: admission[index]! }).step.kind).not.toBe("rejected");
        payload.beginClose(); let terminal = false; const maximum = admission.length + fixture.closing.grants.length + fixture.cancellation.storageCloseGrants.length + builderFixture.bindingGrants.length + payloadFixture.sourceReleaseTurns.length + fixture.cancellation.payloadRegistrationTurns;
        for (let turn = 0; turn < maximum; turn++) { const current = payload.closeStep(grant); expect(current.kind, `${prefix}:${turn}:${current.phase}`).not.toBe("blocked"); expect(current.kind, `${prefix}:${turn}:${current.phase}`).not.toBe("rejected"); expect(current.items).toBeLessThanOrEqual(1); expect(current.bytes).toBeLessThanOrEqual(4096); if (current.kind === "complete") { terminal = true; break; } }
        expect(terminal, String(prefix)).toBe(true); expect(payload.terminalIsEmpty()).toBe(true); expect(builder.terminalIsEmpty()).toBe(true); expect(native.field.fragment).toBe(native.fragment); expect(pool.usage).toEqual(produce(baseline, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] -= builderFixture.total[axis] + payloadFixture.expectedPayloadTotal[axis]; }));
      }
    });

    async function residentPageFixture() {
      const base = await residentBuilderFixture(); const { default: pageFixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); let page: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage | null = null;
      for (const bytes of [...pageFixture.storageOwnerGrants, ...pageFixture.admissionGrants]) page = base.payload.beginPage(base.builder, 1, { maxItems: 1, maxBytes: bytes }).page; if (!page) throw new Error("Missing original page"); return { ...base, page, pageFixture };
    }
    it("OwnedResidentPageQuarantine retains parent faults across direct page calls and exact close", async () => {
      const { OwnedResidentPage, OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { payload, page, pool, pageFixture } = await residentPageFixture(); const held = pool.usage; const fault = Object.freeze({ backing: Uint8Array.of(73) });
      expect(payload.closeStep({ get maxItems(): number { throw fault; }, maxBytes: 4096 }).kind).toBe("rejected");
      const probe = vi.spyOn(OwnedResidentPage.prototype, "allocate"); try { expect(page.allocate(grant).kind).toBe(pageFixture.quarantine.kind); expect(page.writeByte(73, grant).kind).toBe(pageFixture.quarantine.kind); expect(page.seal(grant).kind).toBe(pageFixture.quarantine.kind); expect(payload.closePage(page, grant).kind).toBe(pageFixture.quarantine.kind); expect(probe).toHaveBeenCalledTimes(pageFixture.quarantine.forwardCalls); } finally { probe.mockRestore(); }
      const retained: unknown[] = []; const original = OwnedResidentAdmission.prototype.retainFailure; const capture = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return original.call(this, error, budget); }); try { payload.beginClose(); expect(payload.closeStep(grant).kind).toBe("pending"); expect(payload.closeStep(grant).kind).toBe("rejected"); } finally { capture.mockRestore(); } expect(retained).toEqual([fault]); expect(pool.usage).toEqual(held); expect(page.terminalIsEmpty()).toBe(false);
    });
    it("OwnedResidentPageChild preserves exact child refusal and terminal grant before parent observation", async () => {
      const { OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json");
      for (const row of fixture.childResults) {
        const { payload, page, pool } = await residentPageFixture(); expect(page.allocate(grant).kind).toBe("ready"); const terminal = fixture.closing.phases.indexOf("intrinsic-cell"); expect(terminal).toBeGreaterThan(0); for (let index = 0; index < terminal; index++) expect(payload.closePage(page, { maxItems: 1, maxBytes: fixture.closing.grants[index]! }).kind).toBe("pending"); const held = pool.usage;
        const original = OwnedResidentAdmission.prototype.closeStep; const probe = vi.spyOn(OwnedResidentAdmission.prototype, "closeStep").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, budget) { if (row.actual) expect(original.call(this, budget).kind).toBe("complete"); return { kind: row.kind === "complete" ? "complete" : row.kind === "blocked" ? "blocked" : "rejected", phase: "private-page-child", items: row.bytes ? 1 : 0, bytes: row.bytes }; });
        try { expect(payload.closePage(page, grant)).toMatchObject({ kind: row.expected, bytes: row.bytes }); } finally { probe.mockRestore(); }
        expect(pool.usage).toEqual(produce(held, draft => { if (row.actual) for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] -= fixture.neutralCell[axis]; })); expect(page.terminalIsEmpty()).toBe(false);
        if (row.actual) { const before = pool.usage; expect(payload.closePage(page, { maxItems: 1, maxBytes: 63 })).toMatchObject({ kind: "blocked", bytes: 0 }); expect(payload.closePage(page, { maxItems: 1, maxBytes: 64 })).toMatchObject({ kind: "pending", bytes: 64, phase: "resident-page-storage-release-observation" }); expect(pool.usage).toEqual(before); }
      }
    });

    it("OwnedResidentPageFault retains exact constructor and admission wrapper failures in the original parent cell", async () => {
      const { OwnedResidentLedger, OwnedResidentOwner, OwnedResidentRecord, OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { OwnedUiResidentPage } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json");
      for (const row of fixture.faults) {
        const { payload, builder, pool } = await residentBuilderFixture(); const grants = [...fixture.storageOwnerGrants, ...fixture.admissionGrants]; for (let index = 0; index < row.prefix; index++) expect(payload.beginPage(builder, 1, { maxItems: 1, maxBytes: grants[index]! }).step.kind).toBe("pending"); const before = pool.usage; const fault = Object.freeze({ row, bytes: Uint8Array.of(73) }); const freeze = Object.freeze;
        const install = OwnedResidentRecord.prototype.install; const prepare = OwnedResidentLedger.prototype.prepareAdmission; const claim = OwnedResidentLedger.prototype.claimAdmission; const reserve = OwnedResidentOwner.prototype.reservePage;
        const probe = row.method === "record-install" ? vi.spyOn(OwnedResidentRecord.prototype, "install").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentRecord, shell, budget) { if (row.after) install.call(this, shell, budget); throw fault; }) : row.method === "storage-prepare" ? vi.spyOn(OwnedResidentLedger.prototype, "prepareAdmission").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentLedger, consumer, partition, budget) { if (row.after) prepare.call(this, consumer, partition, budget); throw fault; }) : row.method === "storage-claim" ? vi.spyOn(OwnedResidentLedger.prototype, "claimAdmission").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentLedger, consumer, cell, budget) { if (row.after) claim.call(this, consumer, cell, budget); throw fault; }) : row.method === "storage-reserve" ? vi.spyOn(OwnedResidentOwner.prototype, "reservePage").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentOwner, length, cell, budget) { if (row.after) reserve.call(this, length, cell, budget); throw fault; }) : null;
        if (!probe) Object.freeze = <T,>(value: T): Readonly<T> => { if (row.method === "page-freeze" ? value instanceof OwnedUiResidentPage : value !== null && typeof value === "object" && Object.getPrototypeOf(value)?.constructor.name === "PageWitness") throw fault; return freeze(value); };
        try { expect(payload.beginPage(builder, 1, grant)).toMatchObject({ step: { kind: "rejected" }, page: null }); } finally { probe?.mockRestore(); Object.freeze = freeze; }
        const expected = produce(before, draft => { const charge = row.after && row.method === "storage-prepare" ? fixture.neutralCell : row.after && row.method === "storage-reserve" ? fixture.neutralPage : null; if (charge) for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += charge[axis]; }); expect(pool.usage, row.method).toEqual(expected); expect(payload.beginPage(builder, 1, grant).step.kind).toBe("rejected");
        const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const capture = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); });
        try { payload.beginClose(); expect(payload.closeStep(grant).kind).toBe("pending"); expect(payload.closeStep(grant).kind).toBe("rejected"); } finally { capture.mockRestore(); } expect(retained).toEqual([fault]); expect(pool.usage).toEqual(expected); expect(payload.terminalIsEmpty()).toBe(false);
      }
    });

    it("OwnedResidentPageBodyFault never retries past a before or after mutation failure", async () => {
      const { OwnedResidentPage, OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json");
      for (const row of fixture.bodyFaults) {
        const { payload, builder, page, pool } = await residentPageFixture(); if (row.method === "writeByte") expect(page.allocate(grant).kind).toBe("ready"); const held = pool.usage; const fault = Object.freeze({ row, bytes: Uint8Array.of(73) }); const allocate = OwnedResidentPage.prototype.allocate; const write = OwnedResidentPage.prototype.writeByte;
        const probe = row.method === "allocate" ? vi.spyOn(OwnedResidentPage.prototype, "allocate").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentPage, budget) { if (row.after) expect(allocate.call(this, budget).kind).toBe("ready"); throw fault; }) : vi.spyOn(OwnedResidentPage.prototype, "writeByte").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentPage, value, budget) { if (row.after) expect(write.call(this, value, budget).kind).toBe("pending"); throw fault; });
        try { expect((row.method === "allocate" ? page.allocate(grant) : page.writeByte(73, grant)).kind).toBe("rejected"); } finally { probe.mockRestore(); }
        expect(payload.beginPage(builder, 1, grant)).toMatchObject({ step: { kind: fixture.replay.faultHeldRetry }, page: null }); expect(page.allocate(grant).kind).toBe("rejected"); expect(page.writeByte(74, grant).kind).toBe("rejected"); expect(page.seal(grant).kind).toBe("rejected"); const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const capture = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); });
        try { expect(payload.closePage(page, grant).kind).toBe("pending"); expect(payload.closePage(page, grant).kind).toBe("rejected"); expect(payload.closePage(page, grant).kind).toBe("pending"); expect(payload.closePage(page, grant).kind).toBe("rejected"); } finally { capture.mockRestore(); } expect(retained).toEqual([fault]); expect(pool.usage).toEqual(held); expect(page.terminalIsEmpty()).toBe(false);
      }
    });
    it("OwnedResidentPageInventory matches actual fixed fields and the shared metadata envelope", async () => {
      const ts = await import("typescript"); const { readFileSync } = await import("node:fs"); const { resolve } = await import("node:path"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); const { default: payloadFixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️fixture.json"); const { uiResidentMetadataEnvelope } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🟦️component.ts");
      const source = ts.createSourceFile("resident.ts", readFileSync(resolve(process.cwd(), "../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"), "utf8"), ts.ScriptTarget.Latest, true);
      function fields(name: string): string[] { for (const node of source.statements) { if (ts.isClassDeclaration(node) && node.name?.text === name) return node.members.filter(ts.isPropertyDeclaration).map(member => member.name.getText(source).replace(/^#/, "")); if (ts.isTypeAliasDeclaration(node) && node.name.text === name && ts.isTypeLiteralNode(node.type)) return node.type.members.filter(ts.isPropertySignature).map(member => member.name.getText(source)); } throw new Error(`Missing exact ${name}`); }
      expect(fields("Page")).toEqual(fixture.stateFields); expect(fields("OwnedUiResidentPage")).toEqual(fixture.facadeFields); expect(fields("PageWitness")).toEqual(fixture.witnessFields); expect(fields("PageSlot")).toEqual(["requestOwner", "cell", "record", "entry", "phase", "failure", "witness"]); expect(fields("Payload")).toEqual(payloadFixture.payloadFields); expect(uiResidentMetadataEnvelope("page")).toEqual(fixture.domain);
      for (const axis of ["bytes", "slots", "owners"] as const) expect(BigInt(fixture.domain[axis]) + BigInt(fixture.neutralRecord[axis]) + 2n * BigInt(fixture.neutralCell[axis]) + BigInt(fixture.neutralPage[axis])).toBe(BigInt(fixture.total[axis]));
      const count = (BigInt(fixture.wholeField.extent) + BigInt(fixture.wholeField.pageLength) - 1n) / BigInt(fixture.wholeField.pageLength); expect(count).toBe(BigInt(fixture.wholeField.pages)); expect(count * BigInt(fixture.total.bytes)).toBe(BigInt(fixture.wholeField.pageResidentBytes)); expect(count * BigInt(fixture.total.bytes) <= BigInt(fixture.wholeField.compositionBytes)).toBe(fixture.wholeField.wholeFieldFits);
    });

    it("OwnedResidentPageForeignRetirement rejects terminal authority after the exact original pairing is discharged", async () => {
      const original = await residentPageFixture(); const foreign = await residentPageFixture(); const held = foreign.pool.usage; expect(foreign.payload.closePage(original.page, grant)).toMatchObject({ kind: "rejected", bytes: 0 }); expect(foreign.pool.usage).toEqual(held);
      for (let index = 0; index < original.pageFixture.closing.grants.length; index++) if (index !== original.pageFixture.closing.unallocatedSkip) original.payload.closePage(original.page, { maxItems: 1, maxBytes: original.pageFixture.closing.grants[index]! });
      expect(original.page.terminalIsEmpty()).toBe(true); expect(foreign.payload.closePage(original.page, grant)).toMatchObject({ kind: original.pageFixture.replay.foreignRetired, bytes: 0 }); expect(foreign.pool.usage).toEqual(held); expect(foreign.payload.beginPage(foreign.builder, 1, grant)).toMatchObject({ step: { kind: "ready", bytes: 0 }, page: foreign.page });
      expect(original.payload.closePage(original.page, grant)).toMatchObject({ kind: original.pageFixture.replay.retiredRepeat, bytes: 0 });
    });

it("OwnedResidentReaderPageFence invalidates the exact sealed page when the reader captures retirement", async () => {
      const { payload, builder, reader, readerFixture: fixture } = await residentReaderFixture(); const { default: pages } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); let page: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage | null = null;
      for (const bytes of [...pages.storageOwnerGrants, ...pages.admissionGrants]) page = payload.beginPage(builder, 0, { maxItems: 1, maxBytes: bytes }).page; if (!page) throw new Error("Missing empty original page"); expect(page.allocate(grant).kind).toBe("ready"); expect(page.seal(grant).kind).toBe("ready"); reader.advance({ maxItems: 1, maxBytes: 64 }); for (const bytes of fixture.aliasGrants) reader.advance({ maxItems: 1, maxBytes: bytes }); reader.advance({ maxItems: 1, maxBytes: 64 }); for (const bytes of fixture.aliasCloseGrants) reader.advance({ maxItems: 1, maxBytes: bytes });
      expect(reader.advance({ maxItems: 1, maxBytes: fixture.pageCloseFence.captureBytes })).toMatchObject({ kind: "pending", bytes: fixture.pageCloseFence.captureBytes }); expect(payload.beginPage(builder, 0, grant)).toMatchObject({ step: { kind: fixture.pageCloseFence.heldKind }, page: null }); expect(page.seal(grant).kind).toBe(fixture.pageCloseFence.sealedKind);
    });

it("OwnedResidentReaderInventory matches the exact state facade witness and existing parent words", async () => {
      const ts = await import("typescript"); const { readFileSync } = await import("node:fs"); const { resolve } = await import("node:path"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json"); const { default: payloadFixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️fixture.json"); const { default: builders } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { uiResidentMetadataEnvelope } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🟦️component.ts");
      const root = "../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/"; const source = ts.createSourceFile("resident.ts", readFileSync(resolve(process.cwd(), root + "💾️resident/🟦️component.ts"), "utf8"), ts.ScriptTarget.Latest, true); const builderSource = ts.createSourceFile("builder.ts", readFileSync(resolve(process.cwd(), root + "🩹️operations/📥️wire/📄️pages/🟦️component.ts"), "utf8"), ts.ScriptTarget.Latest, true);
      function fields(name: string, file = source): string[] { for (const node of file.statements) { if (ts.isClassDeclaration(node) && node.name?.text === name) return node.members.filter(ts.isPropertyDeclaration).map(member => member.name.getText(file).replace(/^#/, "")); if (ts.isTypeAliasDeclaration(node) && node.name.text === name && ts.isTypeLiteralNode(node.type)) return node.type.members.filter(ts.isPropertySignature).map(member => member.name.getText(file)); } throw new Error("Missing exact " + name); }
      expect(fields("Reader")).toEqual(fixture.stateFields); expect(fields("OwnedUiResidentPayloadReader")).toEqual(fixture.facadeFields); expect(fields("OwnedUiResidentReaderRetirement")).toEqual(fixture.witnessFields); expect(fields("ReaderSlot")).toEqual(["requestOwner", "cell", "record", "entry", "phase", "failure", "witness"]); expect(fields("Payload")).toEqual(payloadFixture.payloadFields); expect(fields("OwnedUiOperationPayloadBuilder", builderSource)).toEqual(builders.fields); expect(uiResidentMetadataEnvelope("reader")).toEqual(fixture.domain);
      expect([fixture.stateFields, fixture.facadeFields, fixture.witnessFields].reduce((sum, fields) => sum + 16n + 8n * BigInt(fields.length), 0n)).toBe(BigInt(fixture.domain.bytes)); for (const axis of ["bytes", "slots", "owners"] as const) { expect(BigInt(fixture.domain[axis]) + BigInt(fixture.neutralRecord[axis]) + BigInt(fixture.neutralCell[axis])).toBe(BigInt(fixture.total[axis])); expect(BigInt(fixture.intrinsicReader[axis]) + BigInt(fixture.neutralCell[axis])).toBe(BigInt(fixture.readAliasTotal[axis])); }
    });
    it("OwnedResidentReaderBindingFault keeps exact before and after detachment failures charged", async () => {
      const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json");
      for (const row of fixture.bindingFaults) {
        const { payload, reader, pool } = await residentReaderFixture(); for (let index = 0; index < row.prefix; index++) expect(payload.closeReader(reader, { maxItems: 1, maxBytes: fixture.closing.grants[index]! }).kind).toBe("pending"); const held = pool.usage; const fault = Object.freeze({ row, backing: Uint8Array.of(73) }); const method = row.method === "detachReader" ? "detachReader" : "settleReader"; const original = OwnedUiOperationPayloadBuilder[method];
        const probe = vi.spyOn(OwnedUiOperationPayloadBuilder, method).mockImplementation((builder, reader, proof, resident, budget) => { if (row.after) expect(original(builder, reader, proof, resident, budget).bytes).toBe(64); throw fault; }); try { expect(payload.closeReader(reader, grant).kind).toBe("rejected"); } finally { probe.mockRestore(); }
        const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const capture = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); }); try { expect(payload.closeReader(reader, grant).kind).toBe("pending"); expect(payload.closeReader(reader, grant).kind).toBe("rejected"); } finally { capture.mockRestore(); } expect(retained).toEqual([fault]); expect(pool.usage).toEqual(held); expect(reader.terminalIsEmpty()).toBe(false); expect(reader.advance(grant).kind).toBe("rejected");
      }
    });
    it("OwnedResidentReaderBindingGrant forwards child refusal and terminal work before separate observation", async () => {
      const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json");
      for (const method of ["detachReader", "settleReader"] as const) for (const row of fixture.childReturns) {
        const { payload, reader, pool } = await residentReaderFixture(); const prefix = method === "detachReader" ? 2 : 4; for (let index = 0; index < prefix; index++) payload.closeReader(reader, { maxItems: 1, maxBytes: fixture.closing.grants[index]! }); const held = pool.usage; const original = OwnedUiOperationPayloadBuilder[method];
        const probe = vi.spyOn(OwnedUiOperationPayloadBuilder, method).mockImplementation((builder, reader, proof, resident, budget) => { if (row.kind === "complete") expect(original(builder, reader, proof, resident, budget).bytes).toBe(64); return { kind: row.kind === "complete" ? "complete" : row.kind === "blocked" ? "blocked" : "rejected", items: row.items, bytes: row.bytes, phase: "exact-reader-child" }; }); try { expect(payload.closeReader(reader, grant)).toMatchObject({ kind: row.expected, items: row.items, bytes: row.bytes }); expect(probe).toHaveBeenCalledTimes(1); } finally { probe.mockRestore(); }
        expect(pool.usage).toEqual(held); expect(reader.terminalIsEmpty()).toBe(false); if (row.kind === "complete") expect(payload.closeReader(reader, { maxItems: 1, maxBytes: 64 })).toMatchObject({ kind: "pending", bytes: 64, phase: method === "detachReader" ? "resident-reader-binding-detach-observation" : "resident-reader-binding-settle-observation" }); expect(pool.usage).toEqual(held);
      }
    });

it("OwnedResidentReaderRevocation preserves the original read alias and parent close after operations revoke", async () => {
      const { payload, builder, reader, pool, native, readerFixture: fixture } = await residentReaderFixture(); const { default: pages } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); let page: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage | null = null;
      for (const bytes of [...pages.storageOwnerGrants, ...pages.admissionGrants]) page = payload.beginPage(builder, 1, { maxItems: 1, maxBytes: bytes }).page; if (!page) throw new Error("Missing revocation page"); page.allocate(grant); page.writeByte(fixture.revocation.value, grant); page.seal(grant); reader.advance({ maxItems: 1, maxBytes: 64 }); for (const bytes of fixture.aliasGrants) reader.advance({ maxItems: 1, maxBytes: bytes }); const held = pool.usage;
      native.lease.beginClose(); expect(() => native.activation.assertActive()).toThrow(/revoked/); expect(builder.beginRead(grant).step.kind).toBe("rejected"); expect(reader.advance({ maxItems: 1, maxBytes: 1 })).toMatchObject({ kind: "byte", value: fixture.revocation.value, bytes: 1 }); expect(pool.usage).toEqual(held);
      expect(payload.closeReader(reader, { maxItems: 1, maxBytes: 64 }).kind).toBe("pending"); for (const bytes of fixture.aliasCloseGrants) expect(payload.closeReader(reader, { maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); for (let index = 1; index < fixture.closing.grants.length; index++) expect(payload.closeReader(reader, { maxItems: 1, maxBytes: fixture.closing.grants[index]! }).kind).toBe(index === fixture.closing.grants.length - 1 ? "complete" : "pending"); expect(reader.terminalIsEmpty()).toBe(true); expect(page.terminalIsEmpty()).toBe(false);
      expect(pool.usage).toEqual(produce(held, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] -= fixture.total[axis] + fixture.readAliasTotal[axis]; }));
    });
    it("OwnedResidentReaderQuarantine preserves the original parent failure across reads and binding close", async () => {
      const { OwnedResidentAdmission, OwnedResidentReader } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { payload, reader, pool, readerFixture: fixture } = await residentReaderFixture(); const held = pool.usage; const fault = Object.freeze({ backing: Uint8Array.of(73) });
      expect(payload.closeStep({ get maxItems(): number { throw fault; }, maxBytes: 4096 }).kind).toBe("rejected"); const probe = vi.spyOn(OwnedResidentReader.prototype, "byteAt"); try { expect(reader.advance(grant).kind).toBe(fixture.quarantine.kind); expect(payload.closeReader(reader, grant).kind).toBe(fixture.quarantine.kind); expect(probe).toHaveBeenCalledTimes(fixture.quarantine.forwardCalls); } finally { probe.mockRestore(); }
      const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const capture = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); }); try { payload.beginClose(); expect(payload.closeStep(grant).kind).toBe("pending"); expect(payload.closeStep(grant).kind).toBe("rejected"); } finally { capture.mockRestore(); } expect(retained).toEqual([fault]); expect(pool.usage).toEqual(held); expect(reader.terminalIsEmpty()).toBe(false);
    });

    it("OwnedResidentReaderAdmission installs the exact reader before EOF without charging another page or owner", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️schema.json"); const { default: Ajv } = await import("ajv"); expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
      const { payload, builder, pool, native } = await residentBuilderFixture(); const before = pool.usage; expect(native.field.complete).toBe(false); expect(payload.beginReader(builder, { maxItems: 0, maxBytes: 4096 })).toMatchObject({ step: { kind: "blocked", bytes: 0 }, reader: null }); expect(pool.usage).toEqual(before);
      let reader: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayloadReader | null = null;
      for (let index = 0; index < fixture.admissionGrants.length; index++) { const bytes = fixture.admissionGrants[index]!; const held = pool.usage; expect(payload.beginReader(builder, { maxItems: 1, maxBytes: bytes - 1 })).toMatchObject({ step: { kind: "blocked", bytes: 0 }, reader: null }); expect(pool.usage).toEqual(held); const current = payload.beginReader(builder, { maxItems: 1, maxBytes: bytes }); expect(current.step, fixture.admissionPhases[index]).toMatchObject({ kind: index === fixture.admissionGrants.length - 1 ? "ready" : "pending", bytes }); expect(current.reader !== null).toBe(index === fixture.admissionGrants.length - 1); reader = current.reader; }
      if (!reader) throw new Error("Missing original reader"); expect(pool.usage).toEqual(produce(before, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += fixture.total[axis]; })); expect(payload.beginReader(builder, grant)).toMatchObject({ step: { kind: "ready", bytes: 0 }, reader }); expect(builder.beginRead(grant)).toMatchObject({ step: { kind: "ready", bytes: 0 }, reader }); expect(reader.advance(grant)).toMatchObject({ kind: "blocked", bytes: 0 }); expect(native.field.complete).toBe(false);
    });

    async function residentReaderFixture(fixtureIndex = 2) {
      const base = await residentBuilderFixture(fixtureIndex); const { default: readerFixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json"); let reader: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayloadReader | null = null;
      for (const bytes of readerFixture.admissionGrants) reader = base.payload.beginReader(base.builder, { maxItems: 1, maxBytes: bytes }).reader; if (!reader) throw new Error("Missing original reader"); return { ...base, reader, readerFixture };
    }
    it("OwnedResidentReaderRetirement uses its exact original binding witness and rejects ambiguous replay", async () => {
      const base = await residentReaderFixture(); const other = await residentReaderFixture(); const { payload, reader, pool, readerFixture: fixture } = base; let expected = pool.usage; expect(other.payload.closeReader(reader, grant)).toMatchObject({ kind: "rejected", bytes: 0 });
      for (let index = 0; index < fixture.closing.grants.length; index++) { const bytes = fixture.closing.grants[index]!; expect(payload.closeReader(reader, { maxItems: 1, maxBytes: bytes - 1 })).toMatchObject({ kind: "blocked", bytes: 0 }); expect(pool.usage).toEqual(expected); expect(payload.closeReader(reader, { maxItems: 1, maxBytes: bytes }), fixture.closing.phases[index]).toMatchObject({ kind: index === fixture.closing.grants.length - 1 ? "complete" : "pending", bytes }); expected = produce(expected, draft => { for (const refund of fixture.closing.refunds) if (refund.at === index) for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] -= refund[axis]; }); expect(pool.usage).toEqual(expected); expect(reader.terminalIsEmpty()).toBe(index === fixture.closing.grants.length - 1); }
      expect(other.payload.closeReader(reader, grant)).toMatchObject({ kind: "rejected", bytes: 0 }); expect(payload.closeReader(reader, grant)).toMatchObject({ kind: "rejected", bytes: 0 }); expect(payload.beginReader(base.builder, grant)).toMatchObject({ step: { kind: "rejected" }, reader: null }); expect(other.payload.beginReader(other.builder, grant)).toMatchObject({ step: { kind: "ready" }, reader: other.reader });
    });
    it("OwnedResidentReaderWindow consumes two original byte pages before source EOF without retaining the whole extent", async () => {
      const { payload, builder, reader, pool, native, readerFixture: fixture } = await residentReaderFixture(); const { default: pages } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); const { Buffer } = await import("node:buffer"); const expected = Buffer.from(native.payload); const actual: number[] = []; const baseline = pool.usage;
      for (const length of [256, 1]) {
        let page: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage | null = null; for (const bytes of actual.length === 0 ? [...pages.storageOwnerGrants, ...pages.admissionGrants] : pages.admissionGrants) page = payload.beginPage(builder, length, { maxItems: 1, maxBytes: bytes }).page; if (!page) throw new Error("Missing streaming page"); expect(page.allocate(grant).kind).toBe("ready"); for (let index = 0; index < length; index++) expect(page.writeByte(native.fragment.byteAt(actual.length + index, builder), grant).kind).toBe("pending"); expect(page.seal(grant).kind).toBe("ready");
        expect(reader.advance({ maxItems: 1, maxBytes: 64 })).toMatchObject({ kind: "pending", bytes: 64 }); for (const bytes of fixture.aliasGrants) expect(reader.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); expect(payload.closePage(page, grant).kind).toBe("pending"); expect(payload.closePage(page, grant).kind).toBe("blocked");
        for (let index = 0; index < length; index++) { const current = reader.advance({ maxItems: 1, maxBytes: 1 }); expect(current.kind).toBe("byte"); if (current.kind !== "byte") throw new Error("Missing exact byte"); actual.push(current.value); }
        expect(reader.advance({ maxItems: 1, maxBytes: 64 })).toMatchObject({ kind: "pending", bytes: 64 }); for (const bytes of fixture.aliasCloseGrants) expect(reader.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes });
        for (let index = 1; index < pages.closing.grants.length; index++) expect(reader.advance({ maxItems: 1, maxBytes: pages.closing.grants[index]! })).toMatchObject({ kind: "pending", bytes: pages.closing.grants[index] }); expect(reader.advance({ maxItems: 1, maxBytes: 64 })).toMatchObject({ kind: "pending", bytes: 64 }); expect(reader.advance(grant)).toMatchObject({ kind: "blocked", bytes: 0 }); expect(page.terminalIsEmpty()).toBe(true);
        expect(pool.usage).toEqual(produce(baseline, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += pages.storageOwnerTotal[axis]; })); expect(native.field.complete).toBe(false);
      }
      expect(Buffer.from(actual)).toEqual(expected); expect(native.field.consumed).toBe(0n);
    });

    it("OwnedResidentReaderHeld preserves original handle identity throughout its admitted read phases", async () => {
      const { payload, builder, reader, pageFixture: pages, page } = await (async () => { const base = await residentReaderFixture(); const { default: pageFixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); let page: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage | null = null; for (const bytes of [...pageFixture.storageOwnerGrants, ...pageFixture.admissionGrants]) page = base.payload.beginPage(base.builder, 1, { maxItems: 1, maxBytes: bytes }).page; if (!page) throw new Error("Missing held-reader page"); return { ...base, page, pageFixture }; })();
      expect(page.allocate(grant).kind).toBe("ready"); expect(page.writeByte(73, grant).kind).toBe("pending"); expect(page.seal(grant).kind).toBe("ready"); expect(reader.advance({ maxItems: 1, maxBytes: 64 }).kind).toBe("pending");
      expect(payload.beginReader(builder, grant)).toMatchObject({ step: { kind: "ready", bytes: 0 }, reader }); expect(builder.beginRead(grant)).toMatchObject({ step: { kind: "ready", bytes: 0 }, reader }); expect(pages.total.bytes).toBe(1536);
    });
    it("OwnedResidentReaderCancellation retires every admitted prefix under the exact original parent", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json"); const { default: builders } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json");
      for (const prefix of fixture.prefixes) {
        const { payload, builder, pool, fixture: payloadFixture } = await residentBuilderFixture(); const before = pool.usage; for (let index = 0; index < prefix; index++) expect(payload.beginReader(builder, { maxItems: 1, maxBytes: fixture.admissionGrants[index]! }).step.kind).toBe(index === fixture.admissionGrants.length - 1 ? "ready" : "pending");
        payload.beginClose(); let terminal = false; const maximum = fixture.admissionGrants.length + fixture.closing.grants.length + builders.bindingGrants.length + payloadFixture.sourceReleaseTurns.length + fixture.cancellation.payloadRegistrationTurns;
        for (let turn = 0; turn < maximum; turn++) { const current = payload.closeStep(grant); expect(current.kind, `${prefix}:${current.phase}`).not.toBe("blocked"); expect(current.kind, `${prefix}:${current.phase}`).not.toBe("rejected"); expect(current.bytes).toBeLessThanOrEqual(4096); if (current.kind === "complete") { terminal = true; break; } }
        expect(terminal, String(prefix)).toBe(true); expect(pool.usage).toEqual(produce(before, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] -= builders.total[axis] + payloadFixture.expectedPayloadTotal[axis]; }));
      }
    });

    it("OwnedResidentReaderFault retains exact shell and binding failures without replacing the original admission", async () => {
      const { OwnedResidentRecord, OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { OwnedUiResidentPayloadReader } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json");
      for (const row of fixture.faults) {
        const { payload, builder, pool } = await residentBuilderFixture(); for (let index = 0; index < row.prefix; index++) payload.beginReader(builder, { maxItems: 1, maxBytes: fixture.admissionGrants[index]! }); const held = pool.usage; const fault = Object.freeze({ row, backing: Uint8Array.of(73) }); const freeze = Object.freeze; const install = OwnedResidentRecord.prototype.install; const bind = OwnedUiOperationPayloadBuilder.installReader;
        const probe = row.method === "record-install" ? vi.spyOn(OwnedResidentRecord.prototype, "install").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentRecord, shell, budget) { if (row.after) install.call(this, shell, budget); throw fault; }) : row.method === "builder-install" ? vi.spyOn(OwnedUiOperationPayloadBuilder, "installReader").mockImplementation((builder, reader, resident, budget) => { if (row.after) expect(bind(builder, reader, resident, budget).kind).toBe("pending"); throw fault; }) : null;
        if (!probe) Object.freeze = <T,>(value: T): Readonly<T> => { if (row.method === "reader-freeze" ? value instanceof OwnedUiResidentPayloadReader : value !== null && typeof value === "object" && Object.getPrototypeOf(value)?.constructor.name === "OwnedUiResidentReaderRetirement") throw fault; return freeze(value); };
        try { expect(payload.beginReader(builder, grant)).toMatchObject({ step: { kind: "rejected" }, reader: null }); } finally { probe?.mockRestore(); Object.freeze = freeze; }
        expect(payload.beginReader(builder, grant).step.kind).toBe("rejected"); const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const observe = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); });
        try { payload.beginClose(); expect(payload.closeStep(grant).kind).toBe("pending"); expect(payload.closeStep(grant).kind).toBe("rejected"); } finally { observe.mockRestore(); } expect(retained).toEqual([fault]); expect(pool.usage).toEqual(held); expect(payload.terminalIsEmpty()).toBe(false);
      }
    });
    it("OwnedResidentReaderBodyFault preserves original alias and byte offset after read or detach wrapper failure", async () => {
      const { OwnedResidentReader, OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json"); const { default: pages } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json");
      for (const row of fixture.bodyFaults) {
        const { payload, builder, reader, pool } = await residentReaderFixture(); let page: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPage | null = null; for (const bytes of [...pages.storageOwnerGrants, ...pages.admissionGrants]) page = payload.beginPage(builder, 1, { maxItems: 1, maxBytes: bytes }).page; if (!page) throw new Error("Missing fault page"); page.allocate(grant); page.writeByte(73, grant); page.seal(grant); reader.advance({ maxItems: 1, maxBytes: 64 }); for (const bytes of fixture.aliasGrants) reader.advance({ maxItems: 1, maxBytes: bytes });
        if (row.method === "alias-close") { expect(reader.advance(grant)).toMatchObject({ kind: "byte", value: 73 }); expect(reader.advance(grant).kind).toBe("pending"); }
        const held = pool.usage; const fault = Object.freeze({ row, backing: Uint8Array.of(73) }); const read = OwnedResidentReader.prototype.byteAt; const close = OwnedResidentAdmission.prototype.closeStep; const offsets: number[] = [];
        const probe = row.method === "read" ? vi.spyOn(OwnedResidentReader.prototype, "byteAt").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentReader, index) { offsets.push(index); if (row.after) expect(read.call(this, index)).toBe(73); throw fault; }) : vi.spyOn(OwnedResidentAdmission.prototype, "closeStep").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, budget) { if (row.after) expect(close.call(this, budget)).toMatchObject({ kind: "pending", bytes: 136 }); throw fault; });
        try { expect(reader.advance(grant).kind).toBe("rejected"); expect(reader.advance(grant).kind).toBe("rejected"); expect(probe).toHaveBeenCalledTimes(1); } finally { probe.mockRestore(); } expect(offsets).toEqual(row.method === "read" ? [0] : []); expect(payload.beginReader(builder, grant).step.kind).toBe("rejected");
        const expected = produce(held, draft => { if (row.method === "alias-close" && row.after) { draft.bytes -= fixture.intrinsicReader.bytes - 8; draft.slots -= fixture.intrinsicReader.slots; draft.owners -= fixture.intrinsicReader.owners; } }); expect(pool.usage).toEqual(expected);
        const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const capture = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); }); try { expect(payload.closeReader(reader, grant).kind).toBe("pending"); expect(payload.closeReader(reader, grant).kind).toBe("rejected"); expect(payload.closeReader(reader, grant).kind).toBe("pending"); expect(payload.closeReader(reader, grant).kind).toBe("rejected"); } finally { capture.mockRestore(); } expect(retained).toEqual([fault]); expect(pool.usage).toEqual(expected); expect(reader.terminalIsEmpty()).toBe(false); expect(page.terminalIsEmpty()).toBe(false);
      }
    });

    it("OwnedResidentBuilderAdmission charges and registers the original builder before source binding and publication", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️schema.json"); const { default: contract } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️contract.json"); const { default: contractSchema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧬️schema.json"); const { default: Ajv } = await import("ajv"); const ajv = new Ajv({ strict: true, allErrors: true }); expect(ajv.validate(schema, fixture), ajv.errorsText()).toBe(true); expect(ajv.validate(contractSchema, contract), ajv.errorsText()).toBe(true);
      expect([fixture.fields, fixture.entryFields, fixture.witnessFields].reduce((sum, fields) => sum + 16n + BigInt(fields.length) * 8n, 0n)).toBe(BigInt(fixture.domain.bytes)); for (const axis of ["bytes", "slots", "owners"] as const) expect(BigInt(fixture.domain[axis]) + BigInt(fixture.neutralCell[axis]) + BigInt(fixture.neutralRecord[axis])).toBe(BigInt(fixture.total[axis]));
      const capacity = fixture.wholeFieldCapacity; const pages = (BigInt(capacity.extent) + BigInt(capacity.pageLength) - 1n) / BigInt(capacity.pageLength); expect(pages).toBe(BigInt(capacity.pages)); const required = pages * BigInt(capacity.pageCharge); expect(required).toBe(BigInt(capacity.required)); expect(required <= BigInt(capacity.composition)).toBe(capacity.fits);
      const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { native, pool, scope, fixture: payloadFixture } = await residentPayloadFixture(); let payload: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayload | null = null; for (const bytes of payloadFixture.admissionBytes) payload = scope.beginPayload(native.field, { maxItems: 1, maxBytes: bytes }).payload; if (!payload) throw new Error("Missing original payload");
      const before = pool.usage; const begin = (maxBytes: number) => OwnedUiOperationPayloadBuilder.begin(native.owner, native.activation, native.lifetime, native.field, payload, { maxItems: 1, maxBytes }); expect(begin(0)).toMatchObject({ step: { kind: "blocked", bytes: 0 }, builder: null }); expect(pool.usage).toEqual(before); let builder: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts").OwnedUiOperationPayloadBuilder | null = null;
      for (let index = 0; index < fixture.grants.length; index++) { const bytes = fixture.grants[index]!; const current = begin(bytes); expect(current.step, fixture.phases[index]).toMatchObject({ kind: index === fixture.grants.length - 1 ? "ready" : "pending", bytes }); expect(current.builder !== null).toBe(index === fixture.grants.length - 1); builder = current.builder; }
      if (!builder) throw new Error("Missing published builder"); const expected = produce(before, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += fixture.total[axis]; }); expect(pool.usage).toEqual(expected); expect(begin(4096)).toMatchObject({ step: { kind: "ready", bytes: 0 }, builder }); expect(pool.usage).toEqual(expected); expect(native.field.fragment).not.toBeNull();
    });

    it("OwnedResidentBuilderGrant preserves every phase when one byte short", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { native, pool, scope, fixture: payloadFixture } = await residentPayloadFixture(); let payload: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayload | null = null;
      for (const bytes of payloadFixture.admissionBytes) payload = scope.beginPayload(native.field, { maxItems: 1, maxBytes: bytes }).payload; if (!payload) throw new Error("Missing original payload");
      for (let index = 0; index < fixture.grants.length; index++) { const maxBytes = fixture.grants[index]!; const before = pool.usage; const short = OwnedUiOperationPayloadBuilder.begin(native.owner, native.activation, native.lifetime, native.field, payload, { maxItems: 1, maxBytes: maxBytes - 1 }); expect(short.step, fixture.phases[index]).toMatchObject({ kind: "blocked", bytes: 0 }); expect(pool.usage).toEqual(before); const current = OwnedUiOperationPayloadBuilder.begin(native.owner, native.activation, native.lifetime, native.field, payload, { maxItems: 1, maxBytes }); expect(current.step).toMatchObject({ kind: index === fixture.grants.length - 1 ? "ready" : "pending", bytes: maxBytes }); }
    });

    it("OwnedResidentBuilderCancellation retires every prefix through the exact source binding", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts");
      for (const prefix of fixture.prefixes) {
        const { native, pool, scope, fixture: payloadFixture } = await residentPayloadFixture(); const original = pool.usage; let payload: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayload | null = null; for (const bytes of payloadFixture.admissionBytes) payload = scope.beginPayload(native.field, { maxItems: 1, maxBytes: bytes }).payload; if (!payload) throw new Error("Missing original payload");
        for (let index = 0; index < prefix; index++) OwnedUiOperationPayloadBuilder.begin(native.owner, native.activation, native.lifetime, native.field, payload, { maxItems: 1, maxBytes: fixture.grants[index]! }); payload.beginClose(); let complete = false;
        const maximum = fixture.grants.length + fixture.bindingGrants.length + payloadFixture.sourceReleaseTurns.length + 8; for (let turn = 0; turn < maximum; turn++) { const current = payload.closeStep(grant); expect(current.kind, `${prefix}:${current.phase}`).not.toBe("rejected"); expect(current.kind, `${prefix}:${current.phase}`).not.toBe("blocked"); expect(current.bytes).toBeLessThanOrEqual(grant.maxBytes); if (current.kind === "complete") { complete = true; break; } }
        expect(complete, String(prefix)).toBe(true); expect(pool.usage).toEqual(original); expect(native.field.fragment).not.toBeNull(); close(scope); close(pool); native.client.disposeAll();
      }
    });

    it("OwnedResidentBuilderFault retains original entry, install, source and finalizer failures", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { OwnedResidentAdmission, OwnedResidentRecord } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts");
      for (const frontier of fixture.faults) {
        const { native, pool, scope, fixture: payloadFixture } = await residentPayloadFixture(); let payload: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayload | null = null; for (const bytes of payloadFixture.admissionBytes) payload = scope.beginPayload(native.field, { maxItems: 1, maxBytes: bytes }).payload; if (!payload) throw new Error("Missing original payload"); const begin = (maxBytes: number) => OwnedUiOperationPayloadBuilder.begin(native.owner, native.activation, native.lifetime, native.field, payload, { maxItems: 1, maxBytes }); const prefix = frontier.startsWith("source-") ? 8 : frontier === "witness-finalizer" ? 10 : frontier === "builder-finalizer" ? 11 : 7;
        for (let index = 0; index < prefix; index++) begin(fixture.grants[index]!); const held = pool.usage; const fault = Object.freeze({ frontier, backing: Uint8Array.of(73) }); const install = OwnedResidentRecord.prototype.install; const bind = OwnedKernelReturnInputField.prototype.bind; const freeze = Object.freeze;
        const probe = frontier === "entry-before-shell" ? vi.spyOn(OwnedUiOperationPayloadBuilder, "construct").mockImplementation(() => { throw fault; }) : frontier.startsWith("record-") ? vi.spyOn(OwnedResidentRecord.prototype, "install").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentRecord, shell, budget) { if (frontier.endsWith("after")) install.call(this, shell, budget); throw fault; }) : frontier.startsWith("source-") ? vi.spyOn(OwnedKernelReturnInputField.prototype, "bind").mockImplementation(function(this: typeof native.field, builder) { if (frontier.endsWith("after")) bind.call(this, builder); throw fault; }) : null;
        if (!probe) Object.freeze = <T,>(value: T): Readonly<T> => { if (frontier === "builder-finalizer" ? value instanceof OwnedUiOperationPayloadBuilder : value !== null && typeof value === "object" && Object.getPrototypeOf(value)?.constructor.name === "BuilderWitness") throw fault; return freeze(value); };
        try { expect(begin(4096)).toMatchObject({ step: { kind: "rejected" }, builder: null }); } finally { probe?.mockRestore(); Object.freeze = freeze; }
        expect(begin(4096).step.kind).toBe("rejected"); expect(pool.usage).toEqual(held); const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const capture = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); });
        try { payload.beginClose(); for (let turn = 0; turn < 8; turn++) expect(payload.closeStep(grant).kind).not.toBe("complete"); } finally { capture.mockRestore(); } expect(retained).toEqual([fault]); expect(pool.usage).toEqual(held); expect(payload.terminalIsEmpty()).toBe(false);
      }
    });

    it("OwnedResidentBuilderInventory matches actual compiler-parsed fields and the neutral metadata oracle", async () => {
      const ts = await import("typescript"); const { readFileSync } = await import("node:fs"); const { resolve } = await import("node:path"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { default: evidence } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️fixture.json"); const root = resolve(process.cwd(), "../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained");
      const pages = ts.createSourceFile("pages.ts", readFileSync(resolve(root, "🩹️operations/📥️wire/📄️pages/🟦️component.ts"), "utf8"), ts.ScriptTarget.Latest, true); const resident = ts.createSourceFile("resident.ts", readFileSync(resolve(root, "💾️resident/🟦️component.ts"), "utf8"), ts.ScriptTarget.Latest, true);
      function fields(source: import("typescript").SourceFile, name: string): string[] { for (const node of source.statements) { if (ts.isClassDeclaration(node) && node.name?.text === name) return node.members.filter(ts.isPropertyDeclaration).map(member => member.name.getText(source).replace(/^#/, "")); if (ts.isTypeAliasDeclaration(node) && node.name.text === name && ts.isTypeLiteralNode(node.type)) return node.type.members.filter(ts.isPropertySignature).map(member => member.name.getText(source)); } throw new Error(`Missing exact ${name}`); }
      expect(fields(pages, "OwnedUiOperationPayloadBuilder")).toEqual(fixture.fields); expect(fields(resident, "BuilderEntry")).toEqual(fixture.entryFields); expect(fields(resident, "BuilderWitness")).toEqual(fixture.witnessFields); expect(fields(pages, "Evidence")).toEqual(evidence.capsuleFields); expect(fields(pages, "OwnedUiOperationInputCopied")).toEqual(["proof"]); expect(fields(pages, "OwnedUiOperationInputCancelled")).toEqual(["proof"]);
      expect(fields(resident, "EvidenceEntry")).toEqual(fixture.entryFields); expect(fields(resident, "EvidenceWitness")).toEqual(["evidence", "terminal"]);
    });

    async function residentCopiedRangeFixture() {
      const base = await residentReaderFixture(); const { default: binding } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🔗️binding/🧪️fixture.json"); const { default: pages } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); const { builder, reader, readerFixture: readers } = base; const actual: number[] = []; builder.advance({ maxItems: 1, maxBytes: binding.producer.fragmentCapture });
      for (let window = 0; window < binding.stream.windows.length; window++) { const length = binding.stream.windows[window]!; for (const bytes of [...(window === 0 ? pages.storageOwnerGrants : []), ...binding.admissionGrants, binding.producer.pageObservation, binding.producer.allocation, binding.producer.allocationObservation]) expect(builder.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); for (let index = 0; index < length; index++) { builder.advance({ maxItems: 1, maxBytes: 1 }); builder.advance({ maxItems: 1, maxBytes: 1 }); } for (const bytes of [binding.producer.seal, binding.producer.sealObservation]) builder.advance({ maxItems: 1, maxBytes: bytes }); reader.advance({ maxItems: 1, maxBytes: 64 }); for (const bytes of readers.aliasGrants) reader.advance({ maxItems: 1, maxBytes: bytes }); for (let index = 0; index < length; index++) { const current = reader.advance({ maxItems: 1, maxBytes: 1 }); if (current.kind !== "byte") throw new Error("Missing original copied byte"); actual.push(current.value); } reader.advance({ maxItems: 1, maxBytes: 64 }); for (const bytes of [...readers.aliasCloseGrants, ...binding.closeGrants, 64]) expect(reader.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); }
      expect(builder.advance({ maxItems: 1, maxBytes: binding.producer.inputDetach })).toMatchObject({ kind: "pending", bytes: binding.producer.inputDetach }); return { ...base, actual, pageFixture: pages };
    }
    it("OwnedResidentCopiedRange admits one original copied token and observes every real source byte before EOF", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧪️fixture.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧪️schema.json"); const { default: contract } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️contract.json"); const { default: domain } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧬️schema.json"); const { default: Ajv } = await import("ajv"); const ajv = new Ajv({ strict: true }); expect(ajv.validate(schema, fixture)).toBe(true); expect(ajv.validate(domain, contract)).toBe(true); const { Buffer } = await import("node:buffer"); const { OwnedUiOperationInputCopied } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { OwnedKernelReturnInputFragment } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { builder, reader, native, payload, pool, actual, evidenceFixture: evidence } = await residentCopiedRangeFixture(); const baseline = pool.usage; expect(Buffer.from(actual)).toEqual(native.payload); expect(actual.length).toBe(fixture.length); expect(native.field.consumed).toBe(0n); expect(reader.advance(grant).kind).toBe("blocked");
      for (const bytes of evidence.grants) expect(builder.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); expect(pool.usage).toEqual(produce(baseline, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += evidence.total[axis]; })); const original = OwnedKernelReturnInputFragment.prototype.release; let proof: unknown = null; const probe = vi.spyOn(OwnedKernelReturnInputFragment.prototype, "release").mockImplementation(function(this: typeof native.fragment, token) { proof = token; return original.call(this, token); });
      try { expect(builder.advance({ maxItems: 1, maxBytes: 64 })).toMatchObject({ kind: "pending", bytes: 64 }); expect(builder.advance({ maxItems: 1, maxBytes: 128 })).toMatchObject({ kind: "pending", bytes: 128 }); } finally { probe.mockRestore(); } expect(OwnedUiOperationInputCopied.matches(proof, native.fragment, native.field, builder, native.fragment.offset, native.fragment.length)).toBe(true);
      for (let index = 0; index < fixture.length; index++) { expect(builder.advance({ maxItems: 1, maxBytes: fixture.sourceStepBytes })).toMatchObject({ kind: "pending", bytes: fixture.sourceStepBytes, phase: "paged-evidence-source-advance" }); expect(native.field.consumed).toBe(BigInt(index + 1)); expect(builder.advance({ maxItems: 1, maxBytes: fixture.sourceObservationBytes - 1 })).toMatchObject({ kind: "blocked", bytes: 0 }); expect(builder.advance({ maxItems: 1, maxBytes: fixture.sourceObservationBytes })).toMatchObject({ kind: "pending", bytes: fixture.sourceObservationBytes, phase: "paged-evidence-source-observe" }); expect(reader.advance(grant).kind).toBe("blocked"); }
      expect(native.field.complete).toBe(true); for (const bytes of [...fixture.remainingRetirementGrants, ...evidence.retirement.registrationGrants]) expect(builder.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); expect(pool.usage).toEqual(baseline); expect(native.field.fragment).toBeNull(); expect(builder.advance({ maxItems: 1, maxBytes: fixture.rangeObservationBytes })).toMatchObject({ kind: "pending", bytes: fixture.rangeObservationBytes }); expect(builder.advance(grant)).toMatchObject({ kind: "ready", bytes: 0 }); expect(reader.advance(grant)).toMatchObject({ kind: "complete", bytes: 0 }); expect(payload.terminalIsEmpty()).toBe(false);
    });


    it("OwnedResidentCopiedCancellation preserves the same copied token without advancing source progress during close", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧪️fixture.json"); const { default: cancellation } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧪️fixture.json"); const { default: builders } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { OwnedKernelReturnInputFragment } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
      for (const row of fixture.cancellationPrefixes) for (const revoked of fixture.revoked) {
        const { payload, builder, pool, native, readerFixture: readers, evidenceFixture: evidence, fixture: payloadFixture, pageFixture: pages } = await residentCopiedRangeFixture(); const baseline = pool.usage; for (const bytes of evidence.grants) builder.advance({ maxItems: 1, maxBytes: bytes }); let token: unknown = null; let releases = 0; const original = OwnedKernelReturnInputFragment.prototype.release; const release = vi.spyOn(OwnedKernelReturnInputFragment.prototype, "release").mockImplementation(function(this: typeof native.fragment, proof) { if (token !== null) expect(proof).toBe(token); token = proof; releases++; return original.call(this, proof); }); const construct = vi.spyOn(OwnedUiOperationPayloadBuilder, "constructEvidence");
        try { for (const bytes of row.grants) builder.advance({ maxItems: 1, maxBytes: bytes }); expect(native.field.consumed).toBe(BigInt(row.consumed)); payload.beginClose(); if (revoked) native.lease.beginClose(); let complete = false; const maximum = readers.closing.grants.length + evidence.retirement.grants.length + evidence.retirement.registrationGrants.length + builders.bindingGrants.length + payloadFixture.sourceReleaseTurns.length + cancellation.closeBound.payloadRegistrationGrants.length + cancellation.closeBound.storageGrants.length + 1;
          for (let turn = 0; turn < maximum; turn++) { const current = payload.closeStep(grant); expect(current.kind, row.name + ":" + current.phase).not.toBe("rejected"); expect(current.kind, row.name + ":" + current.phase).not.toBe("blocked"); expect(native.field.consumed, row.name + ":" + current.phase).toBe(BigInt(row.consumed)); expect(native.field.complete).toBe(false); if (current.kind === "complete") { complete = true; break; } } expect(complete, row.name).toBe(true); expect(construct).not.toHaveBeenCalled(); expect(releases).toBe(1);
        } finally { release.mockRestore(); construct.mockRestore(); } expect(builder.terminalIsEmpty()).toBe(true); expect(payload.terminalIsEmpty()).toBe(true); expect(pool.usage).toEqual(produce(baseline, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] -= builders.total[axis] + payloadFixture.expectedPayloadTotal[axis] + readers.total[axis] + pages.storageOwnerTotal[axis]; }));
      }
    });


    it("OwnedResidentCopiedEmpty obtains actual zero-byte source completion without allocating a destination page", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧪️fixture.json"); const { Buffer } = await import("node:buffer"); const { OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { builder, reader, native, pool, evidenceFixture: evidence } = await residentReaderFixture(fixture.empty.fixtureIndex); const baseline = pool.usage; expect(native.payload).toEqual(Buffer.alloc(0)); builder.advance({ maxItems: 1, maxBytes: 128 }); builder.advance({ maxItems: 1, maxBytes: 128 }); expect(pool.usage).toEqual(baseline); expect(native.field.complete).toBe(false); for (const bytes of evidence.grants) builder.advance({ maxItems: 1, maxBytes: bytes }); builder.advance({ maxItems: 1, maxBytes: 64 }); builder.advance({ maxItems: 1, maxBytes: 128 }); const probe = vi.spyOn(OwnedKernelReturnInputField.prototype, "advance");
      try { expect(builder.advance({ maxItems: 1, maxBytes: 1 })).toMatchObject({ kind: "pending", bytes: fixture.empty.sourceStepBytes, phase: "paged-evidence-source-advance" }); expect(probe).toHaveBeenCalledTimes(fixture.empty.sourceSteps); } finally { probe.mockRestore(); } expect(native.field.complete).toBe(true); expect(reader.advance(grant).kind).toBe("blocked"); builder.advance({ maxItems: 1, maxBytes: fixture.sourceObservationBytes }); for (const bytes of [...fixture.remainingRetirementGrants, ...evidence.retirement.registrationGrants, fixture.rangeObservationBytes]) expect(builder.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); expect(builder.advance(grant).kind).toBe("ready"); expect(reader.advance(grant)).toMatchObject({ kind: "complete", bytes: 0 }); expect(pool.usage).toEqual(baseline);
    });

    it("OwnedResidentCopiedChild preserves source refusal and full-grant results before independent observation", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧪️fixture.json"); const { OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
      for (const row of fixture.childResults) { const { builder, native, pool, evidenceFixture: evidence } = await residentCopiedRangeFixture(); for (const bytes of [...evidence.grants, 64, 128]) builder.advance({ maxItems: 1, maxBytes: bytes }); const held = pool.usage; const original = OwnedKernelReturnInputField.prototype.advance; const probe = vi.spyOn(OwnedKernelReturnInputField.prototype, "advance").mockImplementation(function(this: typeof native.field, budget, value) { if (row.actual) original.call(this, budget, value); return { kind: row.kind === "pending" ? "pending" : row.kind === "blocked" ? "blocked" : "rejected", items: row.bytes ? 1 : 0, bytes: row.bytes }; });
        try { expect(builder.advance(grant)).toMatchObject({ kind: row.expected, bytes: row.bytes }); } finally { probe.mockRestore(); } expect(pool.usage).toEqual(produce(held, () => {})); expect(native.field.consumed).toBe(row.actual ? 1n : 0n); if (row.expected === "pending") { expect(builder.advance({ maxItems: 1, maxBytes: fixture.sourceObservationBytes - 1 })).toMatchObject({ kind: "blocked", bytes: 0 }); expect(builder.advance({ maxItems: 1, maxBytes: fixture.sourceObservationBytes })).toMatchObject({ kind: "pending", bytes: fixture.sourceObservationBytes }); expect(native.field.consumed).toBe(1n); } else if (row.expected === "blocked") { expect(builder.advance({ maxItems: 1, maxBytes: 1 })).toMatchObject({ kind: "pending", bytes: 1 }); expect(native.field.consumed).toBe(1n); } else { expect(builder.advance(grant).kind).toBe("rejected"); expect(native.field.consumed).toBe(row.actual ? 1n : 0n); }
      }
    });

    it("OwnedResidentCopiedFault retains exact source-advance wrapper exceptions before or after the original byte", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/📋️copied/🧪️fixture.json"); const { OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts");
      for (const row of fixture.faults) { const { payload, builder, native, pool, evidenceFixture: evidence } = await residentCopiedRangeFixture(); for (const bytes of [...evidence.grants, 64, 128]) builder.advance({ maxItems: 1, maxBytes: bytes }); const held = pool.usage; const fault = Object.freeze({ row, backing: Uint8Array.of(73) }); const original = OwnedKernelReturnInputField.prototype.advance; const probe = vi.spyOn(OwnedKernelReturnInputField.prototype, "advance").mockImplementation(function(this: typeof native.field, budget, value) { if (row.after) original.call(this, budget, value); throw fault; }); try { expect(builder.advance(grant).kind).toBe("rejected"); } finally { probe.mockRestore(); } const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const capture = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); });
        try { expect(payload.advanceEvidence(builder, grant)).toMatchObject({ kind: "pending", bytes: 64 }); expect(payload.advanceEvidence(builder, grant).kind).toBe("rejected"); expect(builder.advance(grant).kind).toBe("rejected"); } finally { capture.mockRestore(); } expect(retained).toEqual([fault]); expect(pool.usage).toEqual(produce(held, () => {})); expect(native.field.consumed).toBe(row.after ? 1n : 0n); expect(native.field.fragment).toBe(native.fragment); expect(payload.terminalIsEmpty()).toBe(false);
      }
    });


    it("OwnedResidentActiveInputCancellation detaches the captured input before admitting its exact cancelled evidence", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧪️fixture.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧪️schema.json"); const { default: contract } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️contract.json"); const { default: domain } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧬️schema.json"); const { default: Ajv } = await import("ajv"); const ajv = new Ajv({ strict: true }); expect(ajv.validate(schema, fixture)).toBe(true); expect(ajv.validate(domain, contract)).toBe(true);
      const { OwnedUiOperationInputCancelled } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { default: builders } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json");
      for (const revoked of fixture.revoked) {
        const { payload, builder, native, pool, evidenceFixture: evidence, fixture: payloadFixture } = await residentBuilderFixture(); const baseline = pool.usage; const fragment = native.field.fragment; if (!fragment) throw new Error("Missing original active fragment"); expect(builder.advance({ maxItems: 1, maxBytes: fixture.detachBytes })).toMatchObject({ kind: "pending", bytes: fixture.detachBytes }); payload.beginClose(); if (revoked) native.lease.beginClose();
        expect(payload.closeStep({ maxItems: 1, maxBytes: fixture.detachBytes - 1 })).toMatchObject({ kind: "blocked", bytes: 0 }); expect(pool.usage).toEqual(produce(baseline, () => {})); expect(payload.closeStep({ maxItems: 1, maxBytes: fixture.detachBytes })).toMatchObject({ kind: "pending", bytes: fixture.detachBytes, phase: "paged-active-input-detach" }); expect(native.field.fragment).toBe(fragment); expect(native.field.consumed).toBe(BigInt(fixture.sourceConsumption));
        let token: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts").OwnedUiOperationInputCopied | import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts").OwnedUiOperationInputCancelled | null = null; for (const bytes of evidence.grants) { expect(payload.closeStep({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); } token = payload.beginEvidence(builder, grant).evidence; expect(OwnedUiOperationInputCancelled.matches(token, fragment, native.field, builder, fragment.offset, fragment.length)).toBe(true);
        for (const bytes of [...evidence.retirement.grants, ...evidence.retirement.registrationGrants]) expect(payload.closeStep({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes }); expect(pool.usage).toEqual(baseline); expect(native.field.fragment).toBeNull(); expect(native.field.consumed).toBe(BigInt(fixture.sourceConsumption));
        let complete = false; const maximum = builders.bindingGrants.length + payloadFixture.sourceReleaseTurns.length + 8; for (let turn = 0; turn < maximum; turn++) { const current = payload.closeStep(grant); expect(current.kind, current.phase).not.toBe("rejected"); expect(current.kind, current.phase).not.toBe("blocked"); if (current.kind === "complete") { complete = true; break; } } expect(complete).toBe(true); expect(builder.terminalIsEmpty()).toBe(true); expect(payload.terminalIsEmpty()).toBe(true); expect(pool.usage).toEqual(produce(baseline, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] -= builders.total[axis] + payloadFixture.expectedPayloadTotal[axis]; }));
      }
    });

    it("OwnedResidentActiveInputWindows closes latched and written bytes behind the exact reader and page aliases", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧪️fixture.json"); const { default: binding } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🔗️binding/🧪️fixture.json"); const { default: pages } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📄️page/🧪️fixture.json"); const { default: readers } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📖️reader/🧪️fixture.json"); const { default: builders } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { OwnedUiOperationInputCancelled } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { OwnedKernelReturnInputFragment } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
      for (const row of fixture.cases.filter(row => row.page)) for (const revoked of fixture.revoked) {
        const { payload, builder, native, pool, evidenceFixture: evidence, fixture: payloadFixture } = await residentBuilderFixture(); const baseline = pool.usage; let reader: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayloadReader | null = null; for (const bytes of readers.admissionGrants) reader = builder.beginRead({ maxItems: 1, maxBytes: bytes }).reader; if (!reader) throw new Error("Missing original reader"); builder.advance({ maxItems: 1, maxBytes: binding.producer.fragmentCapture }); for (const bytes of [...pages.storageOwnerGrants, ...binding.admissionGrants, binding.producer.pageObservation, binding.producer.allocation, binding.producer.allocationObservation]) expect(builder.advance({ maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: "pending", bytes });
        const page = payload.beginPage(builder, 256, grant).page; if (!page) throw new Error("Missing original automatic page"); expect(builder.advance({ maxItems: 1, maxBytes: 1 })).toMatchObject({ kind: "pending", bytes: 1, phase: "paged-input-byte" }); if (row.write) builder.advance({ maxItems: 1, maxBytes: 1 });
        if (row.name === "reader-window-active") { for (let index = 1; index < 256; index++) { builder.advance({ maxItems: 1, maxBytes: 1 }); builder.advance({ maxItems: 1, maxBytes: 1 }); } for (const bytes of [binding.producer.seal, binding.producer.sealObservation]) builder.advance({ maxItems: 1, maxBytes: bytes }); expect(builder.advance(grant)).toMatchObject({ kind: "blocked", phase: "paged-page-window" }); reader.advance({ maxItems: 1, maxBytes: 64 }); for (const bytes of readers.aliasGrants) reader.advance({ maxItems: 1, maxBytes: bytes }); expect(reader.advance({ maxItems: 1, maxBytes: 1 })).toMatchObject({ kind: "byte", value: native.payload[0] }); }
        const original = OwnedKernelReturnInputFragment.prototype.release; let releases = 0; let cancelled = false; const probe = vi.spyOn(OwnedKernelReturnInputFragment.prototype, "release").mockImplementation(function(this: typeof native.fragment, proof) { releases++; cancelled = OwnedUiOperationInputCancelled.matches(proof, this, native.field, builder, this.offset, this.length); return original.call(this, proof); }); payload.beginClose(); if (revoked) native.lease.beginClose(); let detached = false; let complete = false;
        const maximum = readers.closing.grants.length + readers.aliasCloseGrants.length + binding.closeGrants.length + evidence.grants.length + evidence.retirement.grants.length + evidence.retirement.registrationGrants.length + builders.bindingGrants.length + payloadFixture.sourceReleaseTurns.length + fixture.closeBound.inputDetachGrants.length + fixture.closeBound.payloadRegistrationGrants.length + fixture.closeBound.storageGrants.length;
        try { for (let turn = 0; turn < maximum; turn++) { const current = payload.closeStep(grant); expect(current.kind, row.name + ":" + current.phase).not.toBe("rejected"); expect(current.kind, row.name + ":" + current.phase).not.toBe("blocked"); expect(current.bytes).toBeLessThanOrEqual(grant.maxBytes); if (current.phase === "paged-active-input-detach") { detached = true; expect(page.terminalIsEmpty()).toBe(true); expect(reader.terminalIsEmpty()).toBe(true); expect(releases).toBe(0); expect(current.bytes).toBe(fixture.detachBytes); } if (current.kind === "complete") { complete = true; break; } } } finally { probe.mockRestore(); }
        expect(detached, row.name).toBe(true); expect(complete, row.name).toBe(true); expect(releases).toBe(1); expect(cancelled).toBe(true); expect(native.field.consumed).toBe(BigInt(fixture.sourceConsumption)); expect(native.field.complete).toBe(false); expect(native.field.fragment).toBeNull(); expect(builder.terminalIsEmpty()).toBe(true); expect(payload.terminalIsEmpty()).toBe(true); expect(pool.usage).toEqual(produce(baseline, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] -= builders.total[axis] + payloadFixture.expectedPayloadTotal[axis]; }));
      }
    });


    it("OwnedResidentActiveInputFault preserves the original before and after detachment exception without source release", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🚫️cancellation/🧪️fixture.json"); const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { OwnedKernelReturnInputFragment } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts");
      for (const row of fixture.faults) { const { payload, builder, native, pool } = await residentBuilderFixture(); builder.advance({ maxItems: 1, maxBytes: fixture.detachBytes }); const held = pool.usage; const fault = Object.freeze({ row, backing: Uint8Array.of(73) }); const original = OwnedUiOperationPayloadBuilder.prepareInputCancellation; const probe = vi.spyOn(OwnedUiOperationPayloadBuilder, "prepareInputCancellation").mockImplementation((value, resident, budget) => { if (row.after) original(value, resident, budget); throw fault; }); const release = vi.spyOn(OwnedKernelReturnInputFragment.prototype, "release"); const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const capture = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); });
        payload.beginClose(); try { expect(payload.closeStep(grant)).toMatchObject({ kind: "rejected", bytes: 0 }); probe.mockRestore(); expect(payload.closeStep(grant)).toMatchObject({ kind: "pending", bytes: 64 }); expect(payload.closeStep(grant).kind).toBe("rejected"); expect(payload.beginEvidence(builder, grant).step.kind).toBe("rejected"); expect(builder.advance(grant).kind).toBe("rejected"); expect(release).not.toHaveBeenCalled(); } finally { probe.mockRestore(); release.mockRestore(); capture.mockRestore(); } expect(retained).toEqual([fault]); expect(pool.usage).toEqual(produce(held, () => {})); expect(native.field.fragment).toBe(native.fragment); expect(native.field.consumed).toBe(0n); expect(payload.terminalIsEmpty()).toBe(false); expect(builder.terminalIsEmpty()).toBe(false);
      }
    });


    it("OwnedResidentEvidenceBrands exposes only exact private release predicates", async () => {
      const { OwnedUiOperationInputCopied, OwnedUiOperationInputCancelled } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️fixture.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️schema.json"); const { default: Ajv } = await import("ajv"); const ajv = new Ajv({ strict: true }); expect(ajv.validate(schema, fixture)).toBe(true); expect(fixture.records.reduce((sum, row) => sum + 16n + 8n * BigInt(row.fields), 0n)).toBe(BigInt(fixture.domain.bytes)); let reads = 0; const hostile = new Proxy({}, { get() { reads++; throw new Error("Untrusted evidence read"); } });
      for (const token of [OwnedUiOperationInputCopied, OwnedUiOperationInputCancelled]) for (const name of fixture.predicates) { const predicate = Reflect.get(token, name); expect(typeof predicate).toBe("function"); for (const candidate of [null, {}, Object.create(token.prototype), hostile]) expect(Reflect.apply(predicate, token, [candidate, hostile])).toBe(false); } expect(reads).toBe(0);
    });

    it("OwnedResidentEvidenceAdmission registers original cancellation evidence before source release", async () => {
      const { OwnedUiOperationPayloadBuilder, OwnedUiOperationInputCancelled } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { default: builderFixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { default: evidenceFixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️fixture.json");
      const { native, pool, scope, fixture } = await residentPayloadFixture(); let payload: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts").OwnedUiResidentPayload | null = null; for (const bytes of fixture.admissionBytes) payload = scope.beginPayload(native.field, { maxItems: 1, maxBytes: bytes }).payload; if (!payload) throw new Error("Missing original payload"); let builder: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts").OwnedUiOperationPayloadBuilder | null = null; for (const bytes of builderFixture.grants) builder = OwnedUiOperationPayloadBuilder.begin(native.owner, native.activation, native.lifetime, native.field, payload, { maxItems: 1, maxBytes: bytes }).builder; if (!builder) throw new Error("Missing original builder");
      const baseline = pool.usage; const fragment = native.field.fragment; if (!fragment) throw new Error("Missing actual fragment"); builder.beginClose(); let evidence: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts").OwnedUiOperationInputCancelled | import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts").OwnedUiOperationInputCopied | null = null;
      for (let index = 0; index < evidenceFixture.grants.length; index++) { const current = payload.beginEvidence(builder, { maxItems: 1, maxBytes: evidenceFixture.grants[index]! }); expect(current.step, String(index)).toMatchObject({ kind: index === evidenceFixture.grants.length - 1 ? "ready" : "pending", items: 1, bytes: evidenceFixture.grants[index] }); evidence = current.evidence; if (index !== evidenceFixture.grants.length - 1) expect(evidence).toBeNull(); }
      expect(evidence).not.toBeNull(); expect(OwnedUiOperationInputCancelled.matches(evidence, fragment, native.field, builder, fragment.offset, fragment.length)).toBe(true); expect(pool.usage).toEqual(produce(baseline, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += evidenceFixture.total[axis]; })); expect(payload.beginEvidence(builder, grant)).toMatchObject({ step: { kind: "ready", items: 0, bytes: 0 }, evidence }); expect(fragment.field).toBe(native.field);
    });

    it("OwnedResidentEvidenceRetirement separates original source settlement from exact registration refund", async () => {
      const { OwnedUiOperationInputCancelled } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { OwnedKernelReturnInputRelease, OwnedKernelReturnInputFragment, OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { payload, builder, pool, native, evidenceFixture: fixture } = await residentEvidenceFixture(); const baseline = pool.usage; let token: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts").OwnedUiOperationInputCancelled | import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts").OwnedUiOperationInputCopied | null = null;
      for (const bytes of fixture.grants) token = payload.beginEvidence(builder, { maxItems: 1, maxBytes: bytes }).evidence; if (!token) throw new Error("Missing original evidence"); const original = token; const charged = pool.usage; let release: import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts").OwnedKernelReturnInputRelease | null = null; const issue = OwnedKernelReturnInputFragment.prototype.release; const observe = vi.spyOn(OwnedKernelReturnInputFragment.prototype, "release").mockImplementation(function(this: typeof native.fragment, proof) { const current = issue.call(this, proof); if (current) release = current; return current; });
      try {
        for (let index = 0; index < fixture.retirement.grants.length; index++) { const bytes = fixture.retirement.grants[index]!; expect(payload.advanceEvidence(builder, { maxItems: 1, maxBytes: bytes - 1 }), fixture.retirement.phases[index]).toMatchObject({ kind: "blocked", bytes: 0 }); const current = payload.advanceEvidence(builder, { maxItems: 1, maxBytes: bytes }); expect(current, fixture.retirement.phases[index]).toMatchObject({ kind: "pending", items: 1, bytes }); expect(pool.usage).toEqual(produce(charged, () => {})); if (index === 1) { expect(OwnedKernelReturnInputRelease.matches(release, native.fragment, original)).toBe(true); expect(OwnedUiOperationInputCancelled.matchesRelease(original, release)).toBe(true); } if (index === 2) { expect(OwnedKernelReturnInputRelease.matchesSourceDetached(release, native.field, original)).toBe(true); expect(native.fragment.field).toBeNull(); expect(OwnedUiOperationInputCancelled.matchesSourceDetached(original, release)).toBe(false); } if (index === 3) expect(OwnedUiOperationInputCancelled.matchesSourceDetached(original, release)).toBe(true); if (index === 4) { expect(OwnedKernelReturnInputRelease.matchesSettled(release, original)).toBe(true); expect(native.field.fragment).toBeNull(); } }
      } finally { observe.mockRestore(); }
      expect(OwnedUiOperationInputCancelled.matchesRelease(original, release)).toBe(false); for (let index = 0; index < fixture.retirement.registrationGrants.length; index++) { const bytes = fixture.retirement.registrationGrants[index]!; expect(payload.advanceEvidence(builder, { maxItems: 1, maxBytes: bytes - 1 })).toMatchObject({ kind: "blocked", bytes: 0 }); expect(payload.advanceEvidence(builder, { maxItems: 1, maxBytes: bytes })).toMatchObject({ kind: index === fixture.retirement.registrationGrants.length - 1 ? "complete" : "pending", items: 1, bytes }); } expect(pool.usage).toEqual(baseline); expect(OwnedKernelReturnInputField.matchesBuilder(native.field, builder)).toBe(true); expect(builder.terminalIsEmpty()).toBe(false); expect(payload.advanceEvidence(builder, grant)).toMatchObject({ kind: "complete", bytes: 0 }); expect(payload.beginEvidence(builder, grant).step.kind).toBe("rejected");
    });

    it("OwnedResidentEvidenceAbandoned keeps parent-driven evidence retirement independent from builder binding", async () => {
      const { payload, builder, pool, native, evidenceFixture: fixture } = await residentEvidenceFixture(); const baseline = pool.usage; for (const bytes of fixture.grants) payload.beginEvidence(builder, { maxItems: 1, maxBytes: bytes }); payload.beginClose();
      for (const bytes of [...fixture.retirement.grants, ...fixture.retirement.registrationGrants]) { const current = payload.closeStep({ maxItems: 1, maxBytes: bytes }); expect(current.kind, current.phase).toBe("pending"); expect(current.bytes).toBe(bytes); } expect(pool.usage).toEqual(baseline); expect(native.field.fragment).toBeNull(); expect(builder.terminalIsEmpty()).toBe(false); close(payload); expect(builder.terminalIsEmpty()).toBe(true); expect(payload.terminalIsEmpty()).toBe(true);
    });

    it("OwnedResidentEvidenceCancellation retires every admitted prefix without replacing the original token", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️fixture.json"); const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts");
      const { default: builderFixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json");
      for (const prefix of fixture.cancellationPrefixes) { const { payload, builder, pool, native, fixture: payloadFixture } = await residentEvidenceFixture(); const baseline = pool.usage; for (let index = 0; index < prefix; index++) payload.beginEvidence(builder, { maxItems: 1, maxBytes: fixture.grants[index]! }); const construct = vi.spyOn(OwnedUiOperationPayloadBuilder, "constructEvidence"); payload.beginClose(); let complete = false; try { const maximum = fixture.grants.length + fixture.retirement.grants.length + fixture.retirement.registrationGrants.length + builderFixture.bindingGrants.length + payloadFixture.sourceReleaseTurns.length + 8; for (let turn = 0; turn < maximum; turn++) { const current = payload.closeStep(grant); expect(current.kind, `${prefix}:${current.phase}`).not.toBe("rejected"); expect(current.kind, `${prefix}:${current.phase}`).not.toBe("blocked"); expect(current.bytes).toBeLessThanOrEqual(grant.maxBytes); if (current.kind === "complete") { complete = true; break; } } expect(complete, String(prefix)).toBe(true); expect(construct).not.toHaveBeenCalled(); } finally { construct.mockRestore(); } expect(pool.usage).toEqual(produce(baseline, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] -= builderFixture.total[axis] + payloadFixture.expectedPayloadTotal[axis]; })); expect(native.field.fragment === null).toBe(prefix >= fixture.tokenInstalledPrefix); expect(payload.terminalIsEmpty()).toBe(true); }
    });

    it("OwnedResidentEvidenceQuarantine rejects direct child drive after the original payload holds a fault", async () => {
      const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { OwnedKernelReturnInputFragment } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { payload, builder, pool, evidenceFixture: fixture } = await residentEvidenceFixture(); let token: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts").OwnedUiOperationInputCopied | import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts").OwnedUiOperationInputCancelled | null = null; for (const bytes of fixture.grants) token = payload.beginEvidence(builder, { maxItems: 1, maxBytes: bytes }).evidence; if (!token) throw new Error("Missing original evidence"); for (let index = 0; index < fixture.parentQuarantine.prefix; index++) payload.advanceEvidence(builder, grant); const held = pool.usage; const fault = Object.freeze({ bytes: Uint8Array.of(73) }); payload.beginClose(); expect(payload.closeStep({ get maxItems(): number { throw fault; }, maxBytes: 4096 }).kind).toBe("rejected"); const probe = vi.spyOn(OwnedKernelReturnInputFragment.prototype, "release"); try { expect(OwnedUiOperationPayloadBuilder.advanceEvidence(token, builder, payload, grant).kind).toBe(fixture.parentQuarantine.kind); expect(probe).toHaveBeenCalledTimes(fixture.parentQuarantine.sourceReleaseCalls); } finally { probe.mockRestore(); } const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const capture = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); }); try { expect(payload.closeStep(grant).kind).toBe("pending"); expect(payload.closeStep(grant).kind).toBe("rejected"); } finally { capture.mockRestore(); } expect(retained).toEqual([fault]); expect(pool.usage).toEqual(produce(held, () => {}));
    });

    it("OwnedResidentEvidenceChild forwards exact source refusal and full-grant work before its own observation", async () => {
      const { OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️fixture.json");
      for (const row of fixture.childResults) { const { payload, builder, pool, native } = await residentEvidenceFixture(); for (const bytes of fixture.grants) payload.beginEvidence(builder, { maxItems: 1, maxBytes: bytes }); for (let index = 0; index < 4; index++) payload.advanceEvidence(builder, { maxItems: 1, maxBytes: fixture.retirement.grants[index]! }); const held = pool.usage; const original = OwnedKernelReturnInputField.prototype.settleInputRelease; const probe = vi.spyOn(OwnedKernelReturnInputField.prototype, "settleInputRelease").mockImplementation(function(this: typeof native.field, release, proof, budget) { if (row.actual) original.call(this, release, proof, budget); return { kind: row.kind === "complete" ? "complete" : row.kind === "blocked" ? "blocked" : "rejected", items: row.bytes === 0 ? 0 : 1, bytes: row.bytes }; }); try { expect(payload.advanceEvidence(builder, grant)).toMatchObject({ kind: row.expected, bytes: row.bytes }); expect(pool.usage).toEqual(held); expect(native.field.fragment === null).toBe(row.actual); } finally { probe.mockRestore(); } if (row.actual) { expect(payload.advanceEvidence(builder, { maxItems: 1, maxBytes: 63 })).toMatchObject({ kind: "blocked", bytes: 0 }); expect(payload.advanceEvidence(builder, { maxItems: 1, maxBytes: 64 })).toMatchObject({ kind: "pending", bytes: 64 }); expect(pool.usage).toEqual(held); } }
    });

    it("OwnedResidentEvidenceSourceFault keeps exact before and after mutation faults under the original cell", async () => {
      const { OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️fixture.json");
      for (const row of fixture.retirementFaults) { const { payload, builder, pool, native } = await residentEvidenceFixture(); for (const bytes of fixture.grants) payload.beginEvidence(builder, { maxItems: 1, maxBytes: bytes }); for (let index = 0; index < row.prefix; index++) payload.advanceEvidence(builder, { maxItems: 1, maxBytes: fixture.retirement.grants[index]! }); const held = pool.usage; const fault = Object.freeze({ row, bytes: Uint8Array.of(73) }); const method = row.method === "detachInputRelease" ? "detachInputRelease" : "settleInputRelease"; const original = OwnedKernelReturnInputField.prototype[method]; const probe = vi.spyOn(OwnedKernelReturnInputField.prototype, method).mockImplementation(function(this: typeof native.field, release, proof, budget) { if (row.after) original.call(this, release, proof, budget); throw fault; }); try { expect(payload.advanceEvidence(builder, grant).kind).toBe("rejected"); } finally { probe.mockRestore(); } const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const capture = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); }); try { expect(payload.advanceEvidence(builder, grant).kind).toBe("pending"); expect(payload.advanceEvidence(builder, grant).kind).toBe("rejected"); payload.beginClose(); expect(payload.closeStep(grant).kind).toBe("rejected"); } finally { capture.mockRestore(); } expect(retained).toEqual([fault]); expect(pool.usage).toEqual(held); expect(payload.terminalIsEmpty()).toBe(false); }
    });

    it("OwnedResidentEvidenceFault retains the installed token shell, capsule and exact original fault", async () => {
      const { OwnedUiOperationInputCancelled, OwnedUiOperationInputCopied } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts"); const { OwnedResidentRecord, OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🧾️evidence/🧪️fixture.json");
      for (const frontier of fixture.faults) {
        const { payload, builder, pool, native } = await residentEvidenceFixture(); for (let index = 0; index < frontier.prefix; index++) payload.beginEvidence(builder, { maxItems: 1, maxBytes: fixture.grants[index]! }); const held = pool.usage; const fault = Object.freeze({ frontier: frontier.name, backing: Uint8Array.of(73) }); const install = OwnedResidentRecord.prototype.install; const freeze = Object.freeze; const seal = Object.seal; let token: unknown = null; let reads = 0; const hostile = new Proxy({}, { get() { reads++; throw new Error("Temporary token probed builder as capsule"); } });
        const probe = frontier.name.startsWith("token-install") ? vi.spyOn(OwnedResidentRecord.prototype, "install").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentRecord, shell, budget) { token = shell; for (const type of [OwnedUiOperationInputCopied, OwnedUiOperationInputCancelled]) { expect(type.matchesRelease(shell, hostile)).toBe(false); expect(type.matchesSourceDetached(shell, hostile)).toBe(false); expect(type.matches(shell, native.fragment, native.field, builder, native.fragment.offset, native.fragment.length)).toBe(false); } if (frontier.name.endsWith("after")) install.call(this, shell, budget); throw fault; }) : null;
        if (frontier.name.startsWith("capsule-")) Object.seal = <T,>(value: T): T => { if (value !== null && typeof value === "object" && Object.hasOwn(value, "token")) { token = Reflect.get(value, "token"); if (frontier.name === "capsule-after-finalization") seal(value); throw fault; } return seal(value); };
        if (frontier.name.endsWith("finalization") && !frontier.name.startsWith("capsule-")) Object.freeze = <T,>(value: T): Readonly<T> => { if (frontier.name === "token-finalization" ? value instanceof OwnedUiOperationInputCancelled : value !== null && typeof value === "object" && Object.getPrototypeOf(value)?.constructor.name === "EvidenceWitness") { token = value; throw fault; } return freeze(value); };
        try { expect(payload.beginEvidence(builder, grant)).toMatchObject({ step: { kind: "rejected" }, evidence: null }); } finally { probe?.mockRestore(); Object.freeze = freeze; Object.seal = seal; }
        expect(token).not.toBeNull(); expect(reads).toBe(0); expect(payload.beginEvidence(builder, grant).step.kind).toBe("rejected"); expect(pool.usage).toEqual(held); const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const capture = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); });
        try { payload.beginClose(); for (let turn = 0; turn < 4; turn++) expect(payload.closeStep(grant).kind).not.toBe("complete"); } finally { capture.mockRestore(); } expect(retained).toEqual([fault]); expect(pool.usage).toEqual(held); expect(payload.terminalIsEmpty()).toBe(false);
      }
    });

    it("OwnedResidentBuilderBindingRetirement closes only the exact original two-sided source binding before refund", async () => {
      const { OwnedUiResidentBuilderRetirement } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { payload, builder, native, pool } = await residentEvidenceFixture(); const held = pool.usage; const fragment = native.field.fragment; let witness: unknown = null; const original = OwnedKernelReturnInputField.prototype.detachBuilder; const probe = vi.spyOn(OwnedKernelReturnInputField.prototype, "detachBuilder").mockImplementation(function(this: typeof native.field, value, proof, budget) { witness = proof; return original.call(this, value, proof, budget); }); payload.beginClose();
      try { for (let index = 0; index < fixture.bindingGrants.length; index++) { const bytes = fixture.bindingGrants[index]!; expect(payload.closeStep({ maxItems: 1, maxBytes: bytes - 1 }), String(index)).toMatchObject({ kind: "blocked", bytes: 0 }); expect(payload.closeStep({ maxItems: 1, maxBytes: bytes }), String(index)).toMatchObject({ kind: "pending", items: 1, bytes }); if (index === 2) { expect(OwnedKernelReturnInputField.matchesBuilder(native.field, builder)).toBe(false); expect(OwnedKernelReturnInputField.matchesBuilderDetached(native.field, witness)).toBe(true); expect(OwnedUiResidentBuilderRetirement.matchesBody(witness, builder, native.field)).toBe(true); } if (index === 3) expect(OwnedUiResidentBuilderRetirement.matchesDetached(witness, native.field)).toBe(false); if (index === 4) expect(OwnedUiResidentBuilderRetirement.matchesDetached(witness, native.field)).toBe(true); if (index === 5) expect(OwnedKernelReturnInputField.matchesBuilderSettled(native.field, witness)).toBe(true); if (index < 12) expect(pool.usage).toEqual(held); } } finally { probe.mockRestore(); }
      expect(builder.terminalIsEmpty()).toBe(true); expect(pool.usage).toEqual(produce(held, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] -= fixture.total[axis]; })); expect(native.field.fragment).toBe(fragment); expect(native.field.bind(builder)).toBe(false); expect(OwnedUiResidentBuilderRetirement.matchesSourceBinding(witness, native.field)).toBe(false); close(payload); expect(payload.terminalIsEmpty()).toBe(true);
    });

    it("OwnedResidentBuilderBindingQuarantine invalidates source proof when its original parent holds a fault", async () => {
      const { OwnedUiResidentBuilderRetirement } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { payload, native, pool } = await residentEvidenceFixture(); let witness: unknown = null; const original = OwnedKernelReturnInputField.prototype.detachBuilder; const probe = vi.spyOn(OwnedKernelReturnInputField.prototype, "detachBuilder").mockImplementation(function(this: typeof native.field, builder, proof, budget) { witness = proof; return original.call(this, builder, proof, budget); }); payload.beginClose(); try { for (let index = 0; index < fixture.bindingQuarantine.prefix; index++) expect(payload.closeStep(grant).kind).toBe("pending"); } finally { probe.mockRestore(); } if (!OwnedUiResidentBuilderRetirement.matchesDetached(witness, native.field)) throw new Error("Missing original detached proof"); const held = pool.usage; const fault = Object.freeze({ bytes: Uint8Array.of(73) }); expect(payload.closeStep({ get maxItems(): number { throw fault; }, maxBytes: 4096 }).kind).toBe("rejected"); expect(OwnedUiResidentBuilderRetirement.matchesDetached(witness, native.field)).toBe(false); expect(native.field.settleBuilder(witness, grant).kind).toBe(fixture.bindingQuarantine.kind); expect(pool.usage).toEqual(produce(held, () => {}));
    });


    it("OwnedResidentBuilderBindingChild forwards source refusal and terminal full-grant work without a same-turn observation", async () => {
      const { OwnedUiResidentBuilderRetirement } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json");
      for (const row of fixture.bindingChildResults) { const { payload, native, pool } = await residentEvidenceFixture(); let witness: unknown = null; payload.beginClose(); for (let index = 0; index < 5; index++) payload.closeStep(grant); const held = pool.usage; const original = OwnedKernelReturnInputField.prototype.settleBuilder; const probe = vi.spyOn(OwnedKernelReturnInputField.prototype, "settleBuilder").mockImplementation(function(this: typeof native.field, proof, budget) { witness = proof; if (row.actual) original.call(this, proof, budget); return { kind: row.kind === "complete" ? "complete" : row.kind === "blocked" ? "blocked" : "rejected", items: row.bytes === 0 ? 0 : 1, bytes: row.bytes }; }); try { expect(payload.closeStep(grant)).toMatchObject({ kind: row.expected, bytes: row.bytes }); expect(pool.usage).toEqual(produce(held, () => {})); expect(OwnedKernelReturnInputField.matchesBuilderSettled(native.field, witness)).toBe(row.actual); expect(OwnedUiResidentBuilderRetirement.matchesDetached(witness, native.field)).toBe(true); } finally { probe.mockRestore(); } if (row.actual) { expect(payload.closeStep({ maxItems: 1, maxBytes: 63 })).toMatchObject({ kind: "blocked", bytes: 0 }); expect(payload.closeStep({ maxItems: 1, maxBytes: 64 })).toMatchObject({ kind: "pending", bytes: 64 }); expect(OwnedUiResidentBuilderRetirement.matchesDetached(witness, native.field)).toBe(false); expect(pool.usage).toEqual(held); } }
    });

    it("OwnedResidentBuilderBindingFault retains exact before and after source-mutation faults without refund", async () => {
      const { OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json");
      for (const row of fixture.bindingFaults) { const { payload, pool, native } = await residentEvidenceFixture(); payload.beginClose(); for (let index = 0; index < row.prefix; index++) payload.closeStep(grant); const held = pool.usage; const fault = Object.freeze({ row, bytes: Uint8Array.of(73) }); const detach = OwnedKernelReturnInputField.prototype.detachBuilder; const settle = OwnedKernelReturnInputField.prototype.settleBuilder;
        const probe = row.method === "detachBuilder" ? vi.spyOn(OwnedKernelReturnInputField.prototype, "detachBuilder").mockImplementation(function(this: typeof native.field, builder, proof, budget) { if (row.after) detach.call(this, builder, proof, budget); throw fault; }) : vi.spyOn(OwnedKernelReturnInputField.prototype, "settleBuilder").mockImplementation(function(this: typeof native.field, proof, budget) { if (row.after) settle.call(this, proof, budget); throw fault; }); try { expect(payload.closeStep(grant).kind).toBe("rejected"); } finally { probe.mockRestore(); } const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const capture = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); }); try { expect(payload.closeStep(grant).kind).toBe("pending"); expect(payload.closeStep(grant).kind).toBe("rejected"); } finally { capture.mockRestore(); } expect(retained).toEqual([fault]); expect(pool.usage).toEqual(held); expect(payload.terminalIsEmpty()).toBe(false);
      }
    });


    it("OwnedResidentBuilderBindingRevocation closes its original binding after operation authority is revoked", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const { OwnedUiOperationPayloadBuilder } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts");
      for (const prefix of fixture.bindingRevocationPrefixes) { const { payload, builder, native, pool, fixture: payloadFixture } = await residentEvidenceFixture(); const held = pool.usage; const fragment = native.field.fragment; payload.beginClose(); for (let index = 0; index < prefix; index++) expect(payload.closeStep(grant).kind).toBe("pending"); native.lease.beginClose(); expect(() => native.activation.assertActive()).toThrow(/revoked/); expect(OwnedUiOperationPayloadBuilder.begin(native.owner, native.activation, native.lifetime, native.field, payload, grant).step.kind).toBe("rejected"); let complete = false; const maximum = fixture.bindingGrants.length - prefix + payloadFixture.sourceReleaseTurns.length + 8; for (let turn = 0; turn < maximum; turn++) { const current = payload.closeStep(grant); expect(current.kind, current.phase).not.toBe("rejected"); expect(current.kind, current.phase).not.toBe("blocked"); if (current.kind === "complete") { complete = true; break; } } expect(complete, String(prefix)).toBe(true); expect(builder.terminalIsEmpty()).toBe(true); expect(pool.usage).toEqual(produce(held, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] -= fixture.total[axis] + payloadFixture.expectedPayloadTotal[axis]; })); expect(native.field.fragment).toBe(fragment); }
    });

    it("OwnedResidentBuilderBindingBrands requires the original privately phased body and source witness", async () => {
      const resident = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🏗️builder/🧪️fixture.json"); const witness: unknown = Reflect.get(resident, "OwnedUiResidentBuilderRetirement"); expect(typeof witness).toBe("function"); if (typeof witness !== "function") throw new Error("Missing original builder witness"); let reads = 0; const hostile = new Proxy({}, { get() { reads++; throw new Error("Forged builder binding read"); } });
      for (const name of fixture.bindingPredicates) { const predicate: unknown = Reflect.get(witness, name); expect(typeof predicate).toBe("function"); if (typeof predicate !== "function") throw new Error("Missing exact binding predicate"); for (const candidate of [null, {}, Object.create(witness.prototype), hostile]) expect(Reflect.apply(predicate, witness, [candidate, hostile, hostile])).toBe(false); } expect(reads).toBe(0);
    });

    it("OwnedResidentPayloadCancellation closes all admitted prefixes through the original parent slot", async () => {
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️fixture.json");
      for (const prefix of fixture.cancellationPrefixes) {
        const { native, pool, scope, baseline } = await residentPayloadFixture(); for (let index = 0; index < prefix; index++) expect(scope.beginPayload(native.field, { maxItems: 1, maxBytes: fixture.admissionBytes[index]! }).step.kind).toBe(index === fixture.admissionBytes.length - 1 ? "ready" : "pending");
        const held = pool.usage; scope.beginClose(); expect(scope.closeStep({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(pool.usage).toEqual(held); let complete = false;
        for (let turn = 0; turn < 96; turn++) { const current = scope.closeStep(grant); expect(current.kind, `prefix ${prefix} ${current.phase}`).not.toBe("rejected"); expect(current.bytes).toBeLessThanOrEqual(grant.maxBytes); expect(current.items).toBeLessThanOrEqual(1); if (current.kind === "complete") { complete = true; break; } }
        expect(complete, `prefix ${prefix}`).toBe(true); expect(scope.terminalIsEmpty()).toBe(true); expect(pool.usage).toEqual(baseline); expect(native.field.residentPayload(scope)).toBeNull(); close(pool); native.client.disposeAll();
      }
    });

    it("OwnedResidentPayloadRefusedSource requires original private unbound proof after clean source installation refusal", async () => {
      const { native, pool, scope, fixture, baseline } = await residentPayloadFixture(); const row = fixture.refusedSource; for (let index = 0; index < row.prefix; index++) scope.beginPayload(native.field, { maxItems: 1, maxBytes: fixture.admissionBytes[index]! }); native.field.beginClose();
      expect(scope.beginPayload(native.field, grant).step.kind).toBe(row.installKind); expect(native.field.residentPayload(scope)).toBeNull(); const held = pool.usage; scope.beginClose(); let completed = false;
      for (let turn = 0; turn < 96; turn++) { const current = scope.closeStep(grant); expect(current.kind, current.phase).not.toBe("rejected"); expect(current.bytes).toBeLessThanOrEqual(grant.maxBytes); if (current.kind === "complete") { completed = true; break; } }
      expect(completed).toBe(row.closeComplete); expect(held.bytes).toBeGreaterThan(baseline.bytes); expect(pool.usage).toEqual(baseline); expect(native.field.residentPayloadDetachment).not.toBeNull(); close(pool); native.client.disposeAll();
    });

    it("OwnedResidentPayloadFault retains exact source-install and finalizer faults under the original charged cell", async () => {
      const { OwnedUiResidentPayload, OwnedUiResidentPayloadSourceRelease } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { OwnedKernelReturnInputField } = await import("../../../../../../../🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts"); const { OwnedResidentAdmission } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️fixture.json");
      for (const frontier of fixture.constructionFaults) {
        const { native, pool, scope } = await residentPayloadFixture(); for (let index = 0; index < frontier.prefix; index++) scope.beginPayload(native.field, { maxItems: 1, maxBytes: fixture.admissionBytes[index]! }); const before = pool.usage; const fault = Object.freeze({ frontier, data: Uint8Array.of(73, 9) }); const freeze = Object.freeze; const install = OwnedKernelReturnInputField.prototype.installResidentPayload;
        const probe = frontier.method === "installResidentPayload" ? vi.spyOn(OwnedKernelReturnInputField.prototype, "installResidentPayload").mockImplementation(function(this: typeof native.field, payload, budget) { if (frontier.after) install.call(this, payload, budget); throw fault; }) : null;
        if (!probe) Object.freeze = <T,>(value: T): Readonly<T> => { if (frontier.method === "payload-finalizer" ? value instanceof OwnedUiResidentPayload : value instanceof OwnedUiResidentPayloadSourceRelease) throw fault; return freeze(value); };
        try { expect(scope.beginPayload(native.field, grant)).toMatchObject({ step: { kind: "rejected" }, payload: null }); } finally { Object.freeze = freeze; probe?.mockRestore(); }
        expect(scope.beginPayload(native.field, grant).step.kind).toBe("rejected"); expect(pool.usage).toEqual(before); const retained: unknown[] = []; const retain = OwnedResidentAdmission.prototype.retainFailure; const capture = vi.spyOn(OwnedResidentAdmission.prototype, "retainFailure").mockImplementation(function(this: import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts").OwnedResidentAdmission, error, budget) { retained.push(error); return retain.call(this, error, budget); });
        try { scope.beginClose(); for (let turn = 0; turn < 8; turn++) expect(scope.closeStep(grant).kind).not.toBe("complete"); } finally { capture.mockRestore(); }
        expect(retained).toEqual([fault]); expect(scope.terminalIsEmpty()).toBe(false); expect(pool.usage).toEqual(before); native.client.disposeAll();
      }
    });

    it("OwnedResidentPayloadBrands rejects forged field, scope and two-way source release authority", async () => {
      const { OwnedUiResidentPayload, OwnedUiResidentPayloadSourceRelease } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts");
      expect(typeof OwnedUiResidentPayload.matchesField).toBe("function"); expect(typeof OwnedUiResidentPayload.matchesScope).toBe("function"); expect(typeof OwnedUiResidentPayload.matchesSourceDetachment).toBe("function"); expect(typeof OwnedUiResidentPayloadSourceRelease).toBe("function");
      let reads = 0; const hostile = { get field() { reads++; throw new Error("Forged field read"); }, get payload() { reads++; throw new Error("Forged payload read"); } }; const field = {}; const scope = {}; const observation = {};
      for (const payload of [hostile, Object.create(OwnedUiResidentPayload.prototype), new Proxy(hostile, { get() { reads++; throw new Error("Forged proxy read"); } })]) { expect(OwnedUiResidentPayload.matchesField(payload, field)).toBe(false); expect(OwnedUiResidentPayload.matchesScope(payload, scope)).toBe(false); expect(OwnedUiResidentPayload.matchesSourceDetachment(payload, observation)).toBe(false); }
      expect(OwnedUiResidentPayloadSourceRelease.matches(hostile, hostile, field)).toBe(false); expect(OwnedUiResidentPayloadSourceRelease.matchesDetached(hostile, hostile)).toBe(false); expect(() => Reflect.construct(OwnedUiResidentPayloadSourceRelease, [{}, hostile])).toThrow(/authority/); expect(reads).toBe(0);
    });

    it("OwnedResidentPayloadContract accounts each future parent slot and field-owned payload record before construction", async () => {
      const { default: domain } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️schema.json"); const { default: contract } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧬️contract.json"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️fixture.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/📦️payload/🧪️schema.json");
      const ajv = new Ajv({ strict: true, allErrors: true }); expect(ajv.compile(domain)(contract)).toBe(true); expect(ajv.compile(schema)(fixture)).toBe(true);
      expect(fixture.payloadFields.length).toBe(fixture.payloadRecords[0]!.fields); const bytes = fixture.payloadRecords.reduce((sum, row) => sum + BigInt(fixture.recordBytes) + BigInt(fixture.fieldBytes) * BigInt(row.fields), 0n); expect(bytes).toBe(BigInt(contract.payloadEnvelope.bytes)); expect(fixture.payloadRecords.length).toBe(contract.payloadEnvelope.slots);
      expect(produce(fixture.instanceBase, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += fixture.slot[axis]; })).toEqual(fixture.expectedInstance); expect(fixture.expectedInstance).toEqual(contract.instanceEnvelope); expect(fixture.expectedPayload).toEqual(contract.payloadEnvelope);
      expect(produce(fixture.expectedPayload, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += fixture.neutralCell[axis] + fixture.neutralRecord[axis]; })).toEqual(fixture.expectedPayloadTotal);
    });

    it("OwnedResidentMetadata charges each schema-defined UI record envelope separately from neutral registration and backing", async () => {
      const { uiResidentMetadataEnvelope } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧪️fixture.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧪️schema.json");
      const { default: domain } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🧬️schema.json");
      const { OwnedResidentLedger } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🟦️component.ts");
      const { default: capacitySchema } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/🧬️schema.json");
      const { default: admissionContract } = await import("../../../../../../../🔨️modules/🌱️value/💾️resident/📨️admission/🧬️contract.json"); expect(fixture.neutralAdmission).toEqual(admissionContract.charge);
      expect(new Ajv({ strict: true, allErrors: true }).addSchema(domain).addSchema(capacitySchema).compile(schema)(fixture)).toBe(true);
      const kinds = ["pool", "instance", "payload", "builder", "reader", "page", "evidence"] as const;
      for (const invalid of ["__proto__", "constructor", "unknown", null, { toString() { throw new Error("Metadata kind was coerced"); } }]) expect(() => Reflect.apply(uiResidentMetadataEnvelope, undefined, [invalid])).toThrow("Invalid UI resident metadata kind");
      const capacity = fixture.expected.reduce((total, row) => ({ bytes: total.bytes + row.bytes + fixture.neutralRegistration.bytes + fixture.neutralAdmission.bytes, slots: total.slots + row.slots + fixture.neutralRegistration.slots + fixture.neutralAdmission.slots, owners: total.owners + row.owners + fixture.neutralRegistration.owners + fixture.neutralAdmission.owners }), { bytes: 0, slots: 0, owners: 0 });
      const ledger = new OwnedResidentLedger({ ...capacity, control: { bytes: 0, slots: 0, owners: 0 } }); let oracle = { bytes: 0, slots: 0, owners: 0 };
      for (const kind of kinds) {
        const definition = fixture.catalogue.find(row => row.kind === kind)!; const expected = fixture.expected.find(row => row.kind === kind)!;
        const bytes = definition.records.reduce((sum, record) => sum + BigInt(fixture.logicalRecordBytes) + BigInt(record.fields.length) * BigInt(fixture.logicalFieldBytes), 0n);
        expect(bytes).toBe(BigInt(expected.bytes)); expect(definition.records.length).toBe(expected.slots); expect(expected.slots).toBe(expected.owners);
        const envelope = uiResidentMetadataEnvelope(kind); expect(envelope).toEqual({ bytes: expected.bytes, slots: expected.slots, owners: expected.owners }); expect(Object.isFrozen(envelope)).toBe(true);
        const consumer = Object.freeze({ kind }); expect(ledger.prepareAdmission(consumer, "data", grant).kind).toBe("pending"); const cell = ledger.preparedAdmission(consumer); if (!cell) throw new Error("Missing original metadata admission cell"); expect(ledger.claimAdmission(consumer, cell, grant).kind).toBe("ready");
        const admission = ledger.reserveRecord("data", envelope, cell, grant); expect(admission.step.kind).toBe("ready"); expect(admission.record).not.toBeNull(); expect(cell.result?.record).toBe(admission.record);
        oracle = produce(oracle, draft => { draft.bytes += envelope.bytes + fixture.neutralRegistration.bytes + fixture.neutralAdmission.bytes; draft.slots += envelope.slots + fixture.neutralRegistration.slots + fixture.neutralAdmission.slots; draft.owners += envelope.owners + fixture.neutralRegistration.owners + fixture.neutralAdmission.owners; }); expect(ledger.usage.data).toEqual(oracle);
      }
      expect(oracle).toEqual(capacity); expect(ledger.prepareAdmission(Object.freeze({}), "data", grant).kind).toBe("blocked"); expect(ledger.usage.data).toEqual(oracle); close(ledger); expect(ledger.usage.data).toEqual({ bytes: 0, slots: 0, owners: 0 });
    });
    //#endregion 🪪️ResidentMetadataTests

    //#region 🏘️InstanceMaintenanceTests
    it("OwnedInstanceMaintenance preserves child refusal and callback failure without losing the queued owner", async () => {
      const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️instance-maintenance.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️instance-maintenance.schema.json");
      expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true);
      for (const vector of fixture.cases) {
        const native = await nativeInstanceFixture(); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const lookup = owner.beginSurfaceLookup(native.activation, native.lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup); let notified = 0; const subscription = surface.subscribeView(() => { notified++; });
        const probe = vi.spyOn(OwnedUiSurface.prototype, "advanceMaintenance").mockImplementationOnce(() => { if (vector.kind === "throw") throw new Error(vector.name); if (vector.kind !== "blocked" && vector.kind !== "rejected" && vector.kind !== "pending") throw new Error("Invalid maintenance fixture kind"); return { kind: vector.kind, phase: vector.name, items: vector.items, bytes: vector.bytes }; });
        try { const current = owner.advanceMaintenance(grant); expect(current.kind, vector.name).toBe(vector.expected); expect(current.bytes).toBe(vector.bytes); expect(current.items).toBe(vector.items); expect(owner.maintenancePending, vector.name).toBe(true); expect(notified).toBe(0); if (vector.kind === "throw") expect(owner.maintenanceFailure).toBe(vector.name); } finally { probe.mockRestore(); }
        for (let turn = 0; owner.maintenancePending && turn < 128; turn++) owner.advanceMaintenance(grant); expect(owner.maintenancePending).toBe(false); expect(notified).toBe(produce(fixture.retryNotifications, () => {})); surface.unsubscribeNode(subscription); close(owner); native.client.disposeAll();
      }
      const native = await nativeInstanceFixture(); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const lookup = owner.beginSurfaceLookup(native.activation, native.lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup); const subscription = surface.subscribeView(() => {});
      const probe = vi.spyOn(OwnedUiSurface.prototype, "advanceMaintenance").mockReturnValueOnce({ kind: "complete", phase: "terminal-child", items: 1, bytes: fixture.terminalChildBytes });
      try { expect(owner.advanceMaintenance(grant)).toMatchObject({ kind: "pending", bytes: fixture.terminalChildBytes }); expect(owner.advanceMaintenance(grant)).toMatchObject({ kind: "pending", bytes: fixture.queueTransitionBytes }); expect(probe).toHaveBeenCalledTimes(1); } finally { probe.mockRestore(); }
      surface.unsubscribeNode(subscription); close(owner); native.client.disposeAll();
    });
    //#endregion 🏘️InstanceMaintenanceTests

    //#region 🏁️InstanceCloseTests
    it("OwnedInstanceCloseAccounting preserves full-grant surface completion before separate wrapper release", async () => {
      const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️instance-close.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️instance-close.schema.json"); expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
      const native = await nativeInstanceFixture(); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const lookup = owner.beginSurfaceLookup(native.activation, native.lifetime, "window")!; expect(finish(lookup)).toBe("ready"); lookup.takeResult(); close(lookup); const original = OwnedUiSurface.prototype.closeStep; let completed = false;
      const probe = vi.spyOn(OwnedUiSurface.prototype, "closeStep").mockImplementation(function(this: InstanceType<typeof OwnedUiSurface>, budget) { const current = original.call(this, budget); if (current.kind === "complete" && !completed) { completed = true; return { ...current, items: 1, bytes: fixture.childBytes }; } return current; }); owner.beginClose();
      try { for (let turn = 0; !completed && turn < 256; turn++) { const current = owner.closeStep(grant); if (completed) expect(current).toMatchObject({ kind: "pending", bytes: fixture.childBytes }); } expect(completed).toBe(true); expect(owner.takeRetirementWitness() !== null).toBe(fixture.earlyWitness); expect(owner.closeStep(grant)).toMatchObject({ kind: "pending", bytes: produce(fixture.surfaceReleaseBytes, () => {}) }); } finally { probe.mockRestore(); }
      close(owner); native.client.disposeAll();
    });

    it("OwnedInstanceCloseAccounting issues input retirement separately from full-grant wire completion", async () => {
      const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { OwnedUiWirePatchCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️instance-close.json");
      for (const mode of fixture.wireOwners) {
        const native = await nativeInstanceFixture(); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const lookup = owner.beginSurfaceLookup(native.activation, native.lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup); const source = await native.source([{ tag: "set-root", val: 1n }]); const patch = owner.beginPatch(source, surface); expect(patch.offer(0)).toBe(true); const closing = mode === "patch" ? patch : owner; const original = OwnedUiWirePatchCursor.prototype.closeStep; let completed = false;
        const probe = vi.spyOn(OwnedUiWirePatchCursor.prototype, "closeStep").mockImplementation(function(this: InstanceType<typeof OwnedUiWirePatchCursor>, budget) { const current = original.call(this, budget); if (this.terminalIsEmpty() && !completed) { completed = true; return { ...current, kind: "complete", items: 1, bytes: fixture.childBytes }; } return current; }); closing.beginClose();
        try { for (let turn = 0; !completed && turn < 512; turn++) { const current = closing.closeStep(grant); if (completed) expect(current, mode).toMatchObject({ kind: "pending", bytes: fixture.childBytes }); } expect(completed).toBe(true); expect(closing.closeStep(grant)).toMatchObject({ kind: "pending", bytes: fixture.inputTokenBytes }); } finally { probe.mockRestore(); }
        const receipt = patch.peekInputReceipt()!; expect(receipt).not.toBeNull(); expect(patch.releaseInputReceipt(receipt)).toBe(true); close(closing); if (mode === "patch") close(owner); native.client.disposeAll();
      }
    });

    it("OwnedInstanceCloseAccounting retains lookup refusal diagnostics and its exact child for retry", async () => {
      const { OwnedUiInstance, OwnedUiSurfaceLookup } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️instance-close.json");
      for (const kind of fixture.lookupRefusals) { if (kind !== "blocked" && kind !== "rejected") throw new Error("Invalid lookup fixture kind"); const native = await nativeInstanceFixture(); const owner = new OwnedUiInstance(native.activation, native.lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const lookup = owner.beginSurfaceLookup(native.activation, native.lifetime, "window")!; owner.beginClose(); const probe = vi.spyOn(OwnedUiSurfaceLookup.prototype, "closeStep").mockReturnValueOnce({ kind, phase: "exact-lookup-refusal", items: 0, bytes: 0 }); try { expect(owner.closeStep(grant)).toEqual({ kind, phase: "exact-lookup-refusal", items: 0, bytes: 0 }); expect(lookup.terminalIsEmpty()).toBe(false); expect(owner.takeRetirementWitness()).toBeNull(); } finally { probe.mockRestore(); } close(owner); expect(lookup.terminalIsEmpty()).toBe(true); native.client.disposeAll(); }
    });
    //#endregion 🏁️InstanceCloseTests

    //#region 🖼️SurfaceChildTests
    it("OwnedSurfaceChild preserves private lookup and retirement refusal without losing retry ownership", async () => {
      const { OwnedUiSurface, OwnedUiSurfacePatch } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts"); const { OwnedUiSceneBindingIndexReader, OwnedUiSceneBindingIndexRetirement } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🗂️index/🟦️component.ts"); const { OwnedUiNodeIndexRetirement } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🗂️nodes/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️surface-child.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️surface-child.schema.json"); expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
      for (const layer of fixture.layers) for (const kind of fixture.refusals) {
        if (kind !== "blocked" && kind !== "rejected") throw new Error("Invalid surface refusal fixture"); const surface = new OwnedUiSurface({ actor: "surface-child", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const maintenance = layer === "lookup" || layer === "reader-retirement"; const subscription = maintenance ? surface.subscribeNode(7, () => {}) : null; if (layer === "patch-close") surface.beginPatch(0, 1); if (!maintenance) surface.beginClose(); let hit = false;
        const refusal = (): import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🟦️component.ts").RetainedUiWireStep => { hit = true; return { kind, phase: layer, items: 0, bytes: 0 }; };
        const probe = layer === "lookup" ? vi.spyOn(OwnedUiSceneBindingIndexReader.prototype, "advance").mockImplementationOnce(refusal) : layer === "reader-retirement" || layer === "binding-retirement" ? vi.spyOn(OwnedUiSceneBindingIndexRetirement.prototype, "advance").mockImplementationOnce(refusal) : layer === "node-retirement" ? vi.spyOn(OwnedUiNodeIndexRetirement.prototype, "advance").mockImplementationOnce(refusal) : vi.spyOn(OwnedUiSurfacePatch.prototype, "closeStep").mockImplementationOnce(refusal);
        try { for (let turn = 0; !hit && turn < 256; turn++) { const current = maintenance ? surface.advanceMaintenance(grant) : surface.closeStep(grant); if (hit) expect(current).toEqual(produce({ kind, phase: layer, items: 0, bytes: 0 }, () => {})); } expect(hit).toBe(true); expect(surface.terminalIsEmpty()).toBe(false); if (maintenance) expect(surface.maintenancePending).toBe(true); } finally { probe.mockRestore(); }
        if (subscription) surface.unsubscribeNode(subscription); close(surface);
      }
    });

    it("OwnedSurfaceChild keeps a full-grant reader completion separate from queue work and lease minting", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts"); const { OwnedUiSceneBindingIndexReader } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🗂️index/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️surface-child.json");
      const surface = new OwnedUiSurface({ actor: "surface-child", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const subscription = surface.subscribeNode(7, () => {}); const original = OwnedUiSceneBindingIndexReader.prototype.advance; let hit = false;
      const probe = vi.spyOn(OwnedUiSceneBindingIndexReader.prototype, "advance").mockImplementation(function(this: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🗂️index/🟦️component.ts").OwnedUiSceneBindingIndexReader, budget) { const current = original.call(this, budget); if (current.kind === "complete" && !hit) { hit = true; return { ...current, items: 1, bytes: fixture.childBytes }; } return current; }); const closing = vi.spyOn(OwnedUiSceneBindingIndexReader.prototype, "beginClose");
      try { for (let turn = 0; !hit && turn < 256; turn++) { const current = surface.advanceMaintenance(grant); if (hit) expect(current).toMatchObject({ kind: "pending", bytes: fixture.childBytes }); } expect(hit).toBe(true); expect(closing.mock.calls.length > 0).toBe(fixture.earlyReaderClose); expect(surface.advanceMaintenance(grant)).toMatchObject({ kind: "pending", bytes: fixture.queueBytes }); expect(surface.advanceMaintenance(grant)).toMatchObject({ kind: "pending", phase: "subscription-read-captured", bytes: fixture.leaseBytes }); expect(closing).toHaveBeenCalledTimes(1); } finally { probe.mockRestore(); closing.mockRestore(); }
      surface.unsubscribeNode(subscription); close(surface);
    });
    it("OwnedSurfaceChild retains a throwing or over-grant private reader for cancellation", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts"); const { OwnedUiSceneBindingIndexReader } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🗂️index/🟦️component.ts");
      for (const mode of ["throw", "overgrant"]) {
        const surface = new OwnedUiSurface({ actor: "reader-fault", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const subscription = surface.subscribeNode(7, () => {}); let hit = false;
        const probe = vi.spyOn(OwnedUiSceneBindingIndexReader.prototype, "advance").mockImplementationOnce(() => { hit = true; if (mode === "throw") throw new Error("Exact reader failed"); return { kind: "pending", phase: "reader-overgrant", items: 1, bytes: 4097 }; });
        try { for (let turn = 0; !hit && turn < 32; turn++) { const current = surface.advanceMaintenance(grant); if (hit) { expect(current.kind).toBe("rejected"); expect(current.bytes).toBe(mode === "throw" ? 0 : 4097); } } expect(hit).toBe(true); expect(surface.maintenancePending).toBe(true); expect(surface.maintenanceFailure).not.toBeNull(); } finally { probe.mockRestore(); }
        surface.unsubscribeNode(subscription); close(surface);
      }
    });
    //#endregion 🖼️SurfaceChildTests

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
      const { OwnedUiNodeReadLease, OwnedUiReadPublication, OwnedUiReadCommit } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️read-publication.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️read-publication.schema.json");
      expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true);
      const publication = new OwnedUiReadPublication(fixture.versions[0]!); const other = new OwnedUiReadPublication(0);
      const a = new OwnedUiNodeReadLease(7, 0, null, publication); const b = new OwnedUiNodeReadLease(8, 0, null, publication);
      const initialA = a.snapshot; const initialB = b.snapshot; const first = publication.begin(1); const foreign = other.begin(1);
      for (const version of fixture.forgedVersions) {
        let staged = false; let forged: typeof first | null = null;
        try { forged = Reflect.construct(OwnedUiReadCommit, [publication, version]); staged = a.stage(forged!, null); } catch { }
        expect(staged).toBe(false); expect(a.hasCapacity).toBe(true); expect(a.snapshot).toBe(initialA); if (forged) expect(publication.publish(forged)).toBe(false);
      }
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
      const recordTurns = 2 * (height + 1) * (allocationTurns + 16) + 9 * height + 64;
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
    it("TypedScene validates all fifteen native schema roots and nested records without reconstructing source strings", async () => {
      const { OwnedUiSceneProjectionCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🟦️component.ts");
      const { OwnedUiSceneCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️typed-scene.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️typed-scene.schema.json");
      const { default: catalog } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🔣️catalog.json");
      const { default: catalogSchema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🔣️catalog.schema.json");
      const { encodePackValue } = await import("@semio-tech/framework-os"); const { Buffer } = await import("node:buffer");
      expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true); expect(new Ajv({ strict: true, allErrors: true }).compile(catalogSchema)(catalog)).toBe(true);
      const size = (value: number): number[] => { const result: number[] = []; do { const byte = value & 127; value = Math.floor(value / 128); result.push(byte | (value ? 128 : 0)); } while (value); return result; };
      const encode = (value: unknown): Uint8Array => {
        if (value === null) return Buffer.from([8]); if (typeof value === "boolean") return Buffer.from([value ? 2 : 1]);
        if (typeof value === "number") { const bytes = Buffer.alloc(9); bytes[0] = 5; bytes.writeDoubleLE(value, 1); return bytes; }
        if (typeof value === "string") { const bytes = Buffer.from(value); return Buffer.concat([Buffer.from([6, ...size(bytes.length)]), bytes]); }
        if (Array.isArray(value)) return Buffer.concat([Buffer.from([10, ...size(value.length)]), ...value.map(encode)]);
        if (typeof value !== "object" || !value) throw new Error("Invalid neutral scene fixture");
        if (Object.hasOwn(value, "$some")) return Buffer.concat([Buffer.from([9]), encode(Reflect.get(value, "$some"))]);
        const entries = Object.entries(value); return Buffer.concat([Buffer.from([13, ...size(entries.length)]), ...entries.flatMap(([key, child]) => [encode(key), encode(child)])]);
      };
      const prepared = (vector: { kind: string; schema: string; value: unknown }) => {
        const wire = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: vector.kind, docSchema: vector.schema, doc: { bytes: Array.from(encode(vector.value)) }, bindings: [] }), "component"); expect(finish(wire)).toBe("ready"); const input = wire.takeResult()!; close(wire);
        const parser = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(parser)).toBe("ready"); const document = parser.takeResult()!; close(parser);
        const cursor = new OwnedUiSceneProjectionCursor(document, { usizeBits: 64 }); retire(document.beginClose()); return cursor;
      };
      expect(fixture.cases.map(value => value.kind)).toEqual(produce(catalog.surfaces.map(value => value.kind), () => {}));
      for (const vector of fixture.cases) {
        const cursor = prepared(vector); expect(finish(cursor), vector.kind).toBe("ready"); const document = cursor.takeResult()!; close(cursor); expect(document.kind).toBe(vector.kind); expect(document.schema).toBe(vector.schema);
        const reader = document.beginRecord(); retire(document.beginClose()); let fields: readonly { readonly name: string; readonly source: number | null }[] = [];
        for (;;) { const current = reader.advance(grant); if (current.kind === "value") fields = current.value.fields; if (current.kind === "complete") break; }
        expect(fields.length).toBeGreaterThan(0); expect(fields.length).toBeLessThanOrEqual(32); retire(reader.beginClose());
      }
      for (const vector of fixture.hostile) { const cursor = prepared(vector); expect(finish(cursor), `${vector.kind}/${vector.schema}`).toBe("rejected"); expect(cursor.failure).not.toBeNull(); expect(cursor.takeResult()).toBeNull(); close(cursor); }
    });
    it("TypedScene cancels every observed projection prefix while retaining the exact source and rejecting forged owners", async () => {
      const typedScene = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🟦️component.ts");
      const { OwnedUiSceneCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️typed-scene.json");
      const { encodePackValue } = await import("@semio-tech/framework-os"); const { Buffer } = await import("node:buffer");
      const packet = Buffer.concat([Buffer.from([13, 1, 6, 6]), Buffer.from("buffer"), Buffer.from([6, 0])]);
      const wire = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "text-editor", docSchema: "text-editor@1", doc: { bytes: Array.from(packet) }, bindings: [] }), "component"); expect(finish(wire)).toBe("ready"); const input = wire.takeResult()!; close(wire);
      const parser = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(parser)).toBe("ready"); const source = parser.takeResult()!; close(parser);
      const observed = new typedScene.OwnedUiSceneProjectionCursor(source, { usizeBits: 64 }); let count = 0; const phases = new Set<string>();
      for (;;) { const current = observed.advance(grant); count++; phases.add(current.phase); expect(current.items).toBeLessThanOrEqual(fixture.lifecycle.maxItems); expect(current.bytes).toBeLessThanOrEqual(fixture.lifecycle.maxBytes); if (current.kind === "ready") break; expect(current.kind).toBe("pending"); }
      close(observed); expect(phases.has("scene-typed-record")).toBe(true); expect(count).toBeGreaterThan(100);
      for (let cutoff = 0; cutoff <= count; cutoff++) {
        const cursor = new typedScene.OwnedUiSceneProjectionCursor(source, { usizeBits: 64 }); expect(cursor.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(cursor.advance({ maxItems: 1, maxBytes: 4095 }).kind).toBe("blocked");
        for (let index = 0; index < cutoff; index++) cursor.advance(grant);
        cursor.beginClose(); expect(cursor.takeResult()).toBeNull(); expect(cursor.closeStep({ maxItems: 1, maxBytes: 0 }).kind).toBe("blocked"); close(cursor); expect(source.terminalIsEmpty()).toBe(false);
      }
      for (const constructor of [typedScene.OwnedUiPreparedScene, typedScene.OwnedUiPreparedSceneReader, typedScene.OwnedUiPreparedSceneRetirement]) expect(() => Reflect.construct(constructor, [{}, {}, null])).toThrow(/authority/);
      const final = new typedScene.OwnedUiSceneProjectionCursor(source, { usizeBits: 64 }); retire(source.beginClose()); expect(finish(final)).toBe("ready"); const prepared = final.takeResult()!; close(final); expect(() => Object.defineProperty(prepared, "schema", { value: "forged@1" })).toThrow(); retire(prepared.beginClose());
    });

    it("TypedScene retains long Unicode field slices across independent prepared and raw readers without rebinding source pages", async () => {
      const { OwnedUiSceneProjectionCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🟦️component.ts");
      const { OwnedUiSceneCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️typed-scene.json");
      const { encodePackValue } = await import("@semio-tech/framework-os"); const { Buffer } = await import("node:buffer");
      const text = fixture.lifecycle.text.repeat(fixture.lifecycle.repeat); const bytes = Buffer.from(text); const prefix: number[] = [13, 1, 6, 6, ...Buffer.from("buffer"), 6]; let remaining = bytes.length;
      do { const byte = remaining & 127; remaining = Math.floor(remaining / 128); prefix.push(byte | (remaining ? 128 : 0)); } while (remaining);
      const create = () => {
        const packet = Buffer.concat([Buffer.from(prefix), bytes]); const wire = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "text-editor", docSchema: "text-editor@1", doc: { bytes: Array.from(packet) }, bindings: [] }), "component"); expect(finish(wire)).toBe("ready"); const input = wire.takeResult()!; close(wire); if (input.value.type !== "surface") throw new Error("Expected surface"); const pages = input.value.doc.bytes;
        const parser = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(parser)).toBe("ready"); const raw = parser.takeResult()!; close(parser); const cursor = new OwnedUiSceneProjectionCursor(raw, { usizeBits: 64 }); retire(raw.beginClose()); expect(finish(cursor)).toBe("ready"); const prepared = cursor.takeResult()!; close(cursor); return { prepared, pages };
      };
      const first = create(); const equal = create(); const record = first.prepared.beginRecord(); let buffer: number | null = null;
      for (;;) { const current = record.advance(grant); if (current.kind === "value") { const field = current.value.fields.find(value => value.name === "buffer"); expect(field?.literal).toBeNull(); buffer = field?.source ?? null; } if (current.kind === "complete") break; }
      if (buffer === null) throw new Error("Expected a sliced buffer field");
      const reader = first.prepared.beginText(buffer); const alias = first.prepared.capture(); const second = alias.beginText(buffer); retire(first.prepared.beginClose()); retire(alias.beginClose()); retire(record.beginClose()); retire(equal.prepared.beginClose()); expect(() => equal.pages.byteAt(0)).toThrow(); expect(first.pages.byteAt(0)).toBe(13);
      const chunks: string[] = []; for (;;) { const current = reader.advance(grant); if (current.kind === "text") { expect(Buffer.byteLength(current.value)).toBeLessThanOrEqual(fixture.lifecycle.maximumEmittedTextBytes); chunks.push(current.value); } expect(current.bytes).toBeLessThanOrEqual(4096); if (current.kind === "complete") break; }
      expect(chunks.join("")).toBe(new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(bytes)); expect(chunks.length).toBeGreaterThan(100); retire(reader.beginClose()); expect(first.pages.byteAt(0)).toBe(13);
      expect(second.advance(grant).kind).not.toBe("rejected"); retire(second.beginClose()); expect(() => first.pages.byteAt(0)).toThrow();
    });

    it("SceneTextBytes reads exact owned UTF-8 ranges without whole-text conversion and retires cancelled readers", async () => {
      const { OwnedUiSceneProjectionCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🟦️component.ts"); const { OwnedUiSceneCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️component.ts"); const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts"); const { encodePackValue } = await import("@semio-tech/framework-os"); const { Buffer } = await import("node:buffer");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-text-bytes.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-text-bytes.schema.json"); expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true); expect(Buffer.from(fixture.text).toString("hex")).toBe(fixture.utf8Hex);
      function create(text: string) {
        const bytes = Buffer.from(text); const prefix = [13, 1, 6, 6, ...Buffer.from("buffer"), 6]; let remaining = bytes.length; do { const digit = remaining % 128; remaining = Math.floor(remaining / 128); prefix.push(digit | (remaining ? 128 : 0)); } while (remaining);
        const wire = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "text-editor", docSchema: "text-editor@1", doc: { bytes: Array.from(Buffer.concat([Buffer.from(prefix), bytes])) }, bindings: [] }), "component"); expect(finish(wire)).toBe("ready"); const input = wire.takeResult()!; close(wire); if (input.value.type !== "surface") throw new Error("Missing fixture Surface"); const pages = input.value.doc.bytes;
        const parser = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(parser)).toBe("ready"); const raw = parser.takeResult()!; close(parser); const projection = new OwnedUiSceneProjectionCursor(raw, { usizeBits: 64 }); retire(raw.beginClose()); expect(finish(projection)).toBe("ready"); const prepared = projection.takeResult()!; close(projection); const record = prepared.beginRecord(); let source: number | null = null; for (;;) { const current = record.advance(grant); if (current.kind === "value") source = current.value.fields.find(field => field.name === "buffer")?.source ?? null; if (current.kind === "complete") break; } retire(record.beginClose()); if (source === null) throw new Error("Missing buffer field source"); return { prepared, source, pages };
      }
      const small = create(fixture.text);
      for (const vector of fixture.ranges) { const reader = small.prepared.beginTextBytes(small.source, vector.offset, vector.length); const chunks: Uint8Array[] = []; for (;;) { const current = reader.advance(grant); if (current.kind === "bytes") { expect(current.value.length).toBeLessThanOrEqual(fixture.maximumChunkBytes); chunks.push(current.value); } expect(current.bytes).toBeLessThanOrEqual(4096); if (current.kind === "rejected") throw new Error(reader.failure!); if (current.kind === "complete") break; } expect(Buffer.concat(chunks).toString("hex")).toBe(vector.hex); expect(Buffer.concat(chunks)).toEqual(Buffer.from(fixture.text).subarray(vector.offset, vector.offset + vector.length)); retire(reader.beginClose()); }
      for (const vector of fixture.invalid) { if (vector.offset < 0 || vector.length < 0) expect(() => small.prepared.beginTextBytes(small.source, vector.offset, vector.length)).toThrow(/range/); else { const reader = small.prepared.beginTextBytes(small.source, vector.offset, vector.length); let rejected = false; for (let i = 0; i < 256; i++) { const current = reader.advance(grant); if (current.kind === "rejected") { rejected = true; break; } expect(current.kind).not.toBe("bytes"); } expect(rejected).toBe(true); retire(reader.beginClose()); } } retire(small.prepared.beginClose()); expect(() => small.pages.byteAt(0)).toThrow();
      const large = create(fixture.text.repeat(fixture.repeat)); const reader = large.prepared.beginTextBytes(large.source); const cancelled = large.prepared.beginTextBytes(large.source, 1, 4); retire(large.prepared.beginClose()); expect(reader.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); const chunks: Uint8Array[] = []; for (;;) { const current = reader.advance(grant); if (current.kind === "bytes") chunks.push(current.value); if (current.kind === "complete") break; if (current.kind === "rejected") throw new Error(reader.failure!); } expect(Buffer.concat(chunks)).toEqual(Buffer.from(fixture.text.repeat(fixture.repeat))); expect(chunks.length).toBeGreaterThan(1); retire(reader.beginClose()); expect(large.pages.byteAt(0)).toBe(13); const closing = cancelled.beginClose(); expect(closing.advance({ maxItems: 1, maxBytes: 0 }).kind).toBe("blocked"); retire(closing); expect(() => large.pages.byteAt(0)).toThrow();
    });
    it("OwnedSceneJson streams exact syntax from captured prepared UTF-8 with JSON.parse and Node Buffer oracles", async () => {
      const { OwnedUiSceneJsonCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🟦️component.ts"); const { OwnedUiSceneProjectionCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🟦️component.ts"); const { OwnedUiSceneCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️component.ts"); const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts"); const { encodePackValue } = await import("@semio-tech/framework-os"); const { Buffer } = await import("node:buffer");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-json.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-json.schema.json"); expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
      let forgedCaptureCalls = 0; expect(() => Reflect.construct(OwnedUiSceneJsonCursor, [{ capture: () => { forgedCaptureCalls++; return null; } }, 0])).toThrow(); expect(forgedCaptureCalls).toBe(fixture.forgedSourceCaptureCalls);
      function create(text: string, shared = false) {
        const bytes = Buffer.from(text); const prefix = [13, 1, 6, 6, ...Buffer.from("buffer"), 6]; let remaining = bytes.length; do { const digit = remaining % 128; remaining = Math.floor(remaining / 128); prefix.push(digit | (remaining ? 128 : 0)); } while (remaining);
        const typed = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "text-editor", docSchema: "text-editor@1", doc: { bytes: Array.from(Buffer.concat([Buffer.from(prefix), bytes])) }, bindings: [] }), "component"); expect(finish(typed)).toBe("ready"); const input = typed.takeResult()!; close(typed); if (input.value.type !== "surface") throw new Error("Expected scene"); const pages = input.value.doc.bytes; const raw = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(raw)).toBe("ready"); const document = raw.takeResult()!; close(raw); const projection = new OwnedUiSceneProjectionCursor(document, { usizeBits: 64 }); retire(document.beginClose()); expect(finish(projection)).toBe("ready"); const prepared = projection.takeResult()!; close(projection);
        const reader = prepared.beginRecord(); let field: number | null = null; for (;;) { const current = reader.advance(grant); if (current.kind === "value") field = current.value.fields.find(value => value.name === "buffer")?.source ?? null; if (current.kind === "complete") break; } retire(reader.beginClose()); if (field === null) throw new Error("Missing text source"); const parser = new OwnedUiSceneJsonCursor(prepared, field); const sibling = shared ? new OwnedUiSceneJsonCursor(prepared, field) : null; retire(prepared.beginClose()); return { parser, sibling, bytes, pages };
      }
      const largeText = JSON.stringify({ [fixture.large.key.repeat(fixture.large.repeat)]: fixture.large.value.repeat(fixture.large.repeat) }); const texts = [...fixture.valid.map(value => value.text), largeText];
      for (const text of texts) {
        const { parser, bytes, pages } = create(text); expect(parser.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); let result: unknown; const stack: { value: unknown[] | Record<string, unknown>; key: string | null }[] = [];
        const put = (value: unknown) => { const parent = stack.at(-1); if (!parent) result = value; else if (Array.isArray(parent.value)) parent.value.push(value); else { if (parent.key === null) throw new Error("Missing JSON key"); Object.defineProperty(parent.value, parent.key, { value, writable: true, configurable: true, enumerable: true }); parent.key = null; } };
        for (let turns = 0;; turns++) { expect(turns).toBeLessThan(bytes.length * 4 + 512); const before = parser.offset; const current = parser.advance(grant); expect(parser.offset).toBeGreaterThanOrEqual(before); expect(parser.offset - before).toBeLessThanOrEqual(1); expect(current.items).toBeLessThanOrEqual(1); expect(current.bytes).toBeLessThanOrEqual(4096); if (current.kind === "rejected") throw new Error(parser.failure!);
          if (current.kind === "token") { const token = current.token; const raw = bytes.subarray(token.start, token.start + token.length).toString("utf8"); if (token.kind === "object" || token.kind === "array") { const value = token.kind === "array" ? [] : {}; put(value); stack.push({ value, key: null }); } else if (token.kind === "end-object" || token.kind === "end-array") stack.pop(); else if (token.kind === "key") stack.at(-1)!.key = JSON.parse(raw); else put(JSON.parse(raw)); }
          if (current.kind === "ready") break;
        }
        expect(result).toEqual(JSON.parse(text)); expect(parser.offset).toBe(bytes.length); close(parser); expect(() => pages.byteAt(0)).toThrow();
      }
      for (const text of fixture.invalid) { expect(() => JSON.parse(text)).toThrow(); const { parser } = create(text); expect(finish(parser)).toBe("rejected"); close(parser); }
      const nested = "[".repeat(fixture.depth) + "0" + "]".repeat(fixture.depth); const deep = create(nested); let stalled = 0;
      for (let turns = 0;; turns++) { expect(turns).toBeLessThan(deep.bytes.length * 4 + 512); const before = deep.parser.offset; const current = deep.parser.advance(grant); expect(current.kind).not.toBe("rejected"); expect(deep.parser.offset).toBeGreaterThanOrEqual(before); expect(deep.parser.offset - before).toBeLessThanOrEqual(1); stalled = deep.parser.offset > before || current.kind === "token" ? 0 : stalled + 1; expect(stalled).toBeLessThan(512); if (current.kind === "ready") break; }
      close(deep.parser); expect(() => deep.pages.byteAt(0)).toThrow();
      for (const cutoff of fixture.cancelPrefixes) { const value = create(nested); for (let n = 0; n < cutoff; n++) value.parser.advance(grant); value.parser.beginClose(); expect(value.parser.closeStep({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); close(value.parser); expect(() => value.pages.byteAt(0)).toThrow(); }
      const shared = create(largeText, true); shared.parser.advance(grant); close(shared.parser); expect(shared.pages.byteAt(0)).toBe(13); expect(finish(shared.sibling!)).toBe("ready"); expect(shared.sibling!.offset).toBe(shared.bytes.length); close(shared.sibling!); expect(() => shared.pages.byteAt(0)).toThrow();
    });

    it("OwnedSceneJsonDocument publishes flat validated tokens with independent exact span and reader ownership", async () => {
      const { OwnedUiSceneJsonDocumentCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🧾️value/🟦️component.ts"); const { OwnedUiSceneProjectionCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🟦️component.ts"); const { OwnedUiSceneCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️component.ts"); const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts"); const { encodePackValue } = await import("@semio-tech/framework-os"); const { Buffer } = await import("node:buffer");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-json-document.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-json-document.schema.json"); expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
      function create(text: string) {
        const bytes = Buffer.from(text); const prefix = [13, 1, 6, 6, ...Buffer.from("buffer"), 6]; let remaining = bytes.length; do { const digit = remaining % 128; remaining = Math.floor(remaining / 128); prefix.push(digit | (remaining ? 128 : 0)); } while (remaining);
        const typed = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "text-editor", docSchema: "text-editor@1", doc: { bytes: Array.from(Buffer.concat([Buffer.from(prefix), bytes])) }, bindings: [] }), "component"); expect(finish(typed)).toBe("ready"); const input = typed.takeResult()!; close(typed); if (input.value.type !== "surface") throw new Error("Expected exact scene"); const pages = input.value.doc.bytes; const raw = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(raw)).toBe("ready"); const document = raw.takeResult()!; close(raw); const projection = new OwnedUiSceneProjectionCursor(document, { usizeBits: 64 }); retire(document.beginClose()); expect(finish(projection)).toBe("ready"); const prepared = projection.takeResult()!; close(projection);
        const reader = prepared.beginRecord(); let field: number | null = null; for (;;) { const current = reader.advance(grant); if (current.kind === "value") field = current.value.fields.find(value => value.name === "buffer")?.source ?? null; if (current.kind === "complete") break; } retire(reader.beginClose()); if (field === null) throw new Error("Missing JSON source"); const cursor = new OwnedUiSceneJsonDocumentCursor(prepared, field); retire(prepared.beginClose()); return { cursor, bytes, pages };
      }
      function bound(bytes: number): number { const height = 2 * Math.ceil(Math.log2(bytes + 2)); const allocation = 9 * 10 + 9 + 6 + 9 + 4; return (bytes + 4) * (2 * (height + 1) * (allocation + 16) + 9 * height + 64) + 1024; }
      const { NumericIndexRetirement } = await import("../../../../../../../🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🟦️component.ts"); const terminal = create("0"); const originalAdvance = NumericIndexRetirement.prototype.advance; let injected = false;
      for (let turn = 0;; turn++) { expect(turn).toBeLessThan(bound(1)); if (terminal.cursor.advance(grant).phase === "json-document-install") break; }
      const terminalSpy = vi.spyOn(NumericIndexRetirement.prototype, "advance").mockImplementation(function (this: InstanceType<typeof NumericIndexRetirement>, budget) { const current = originalAdvance.call(this, budget); if (!injected && current.kind === "complete") { expect(this.terminalIsEmpty()).toBe(true); injected = true; return { kind: "complete", items: 1, bytes: fixture.terminalChildBytes }; } return current; });
      try { for (let turn = 0; !injected; turn++) { expect(turn).toBeLessThan(bound(1)); const current = terminal.cursor.advance(grant); if (injected) { expect(current.kind).toBe("pending"); expect(current.bytes).toBe(fixture.terminalChildBytes); } } } finally { terminalSpy.mockRestore(); }
      const terminalOffset = terminal.cursor.offset; const released = terminal.cursor.advance(grant); expect(released.kind).toBe("pending"); expect(released.bytes).toBe(fixture.terminalReleaseBytes); expect(terminal.cursor.offset).toBe(terminalOffset); expect(finish(terminal.cursor)).toBe("ready"); retire(terminal.cursor.takeResult()!.beginClose()); close(terminal.cursor); expect(() => terminal.pages.byteAt(0)).toThrow();
      const phases = new Map<string, number>();
      const large = JSON.stringify({ [fixture.large.key.repeat(fixture.large.repeat)]: fixture.large.value.repeat(fixture.large.repeat) });
      for (const vector of [...fixture.valid, { name: "large", text: large, keys: [fixture.large.key.repeat(fixture.large.repeat)] }]) {
        const value = create(vector.text); expect(value.cursor.takeResult()).toBeNull(); expect(value.cursor.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked");
        for (let turn = 0;; turn++) { expect(turn).toBeLessThan(bound(value.bytes.length)); const before = value.cursor.offset; const current = value.cursor.advance(grant); expect(value.cursor.offset).toBeGreaterThanOrEqual(before); expect(value.cursor.offset - before).toBeLessThanOrEqual(1); expect(current.items).toBeLessThanOrEqual(fixture.maxItems); expect(current.bytes).toBeLessThanOrEqual(fixture.maxBytes); expect(current.kind, value.cursor.failure ?? vector.name).not.toBe("rejected"); if (!phases.has(current.phase)) phases.set(current.phase, turn); if (current.kind === "ready") break; }
        const document = value.cursor.takeResult()!; expect(value.cursor.takeResult()).toBeNull(); close(value.cursor); const alias = document.capture(); const readers = Array.from({ length: fixture.readerCount }, () => document.beginRead()); const span = document.beginSpan(0, value.bytes.length); const missing = document.beginLookup(Number.MAX_SAFE_INTEGER); retire(document.beginClose());
        let missingCount = 0; for (;;) { const current = missing.advance(grant); if (current.kind === "value") missingCount++; if (current.kind === "complete") break; } expect(missingCount).toBe(0); retire(missing.beginClose());
        const keys: string[] = []; let result: unknown; const stack: { value: unknown[] | Record<string, unknown>; key: string | null }[] = [];
        const put = (item: unknown) => { const parent = stack.at(-1); if (!parent) result = item; else if (Array.isArray(parent.value)) parent.value.push(item); else { if (parent.key === null) throw new Error("Missing owned JSON key"); Object.defineProperty(parent.value, parent.key, { value: item, writable: true, configurable: true, enumerable: true }); parent.key = null; } };
        for (;;) { const current = readers[0]!.advance(grant); expect(current.kind).not.toBe("rejected"); if (current.kind === "value") { const token = current.value; const text = value.bytes.subarray(token.start, token.start + token.length).toString("utf8"); if (token.kind === "object" || token.kind === "array") { const item = token.kind === "array" ? [] : {}; put(item); stack.push({ value: item, key: null }); } else if (token.kind === "end-object" || token.kind === "end-array") stack.pop(); else if (token.kind === "key") { const key: string = JSON.parse(text); keys.push(key); stack.at(-1)!.key = key; } else put(JSON.parse(text)); } if (current.kind === "complete") break; }
        expect(result).toEqual(JSON.parse(vector.text)); expect(keys).toEqual(vector.keys); retire(readers[0]!.beginClose()); const first = alias.beginLookup(0); let firstCount = 0; for (;;) { const current = first.advance(grant); if (current.kind === "value") firstCount++; if (current.kind === "complete") break; } expect(firstCount).toBe(1); retire(first.beginClose());
        const prototype: unknown = Object.getPrototypeOf(alias); if (!prototype || typeof prototype !== "object" || !("constructor" in prototype) || typeof prototype.constructor !== "function") throw new Error("Missing JSON owner constructor"); let forged = false; try { Reflect.construct(prototype.constructor, [{}, {}]); forged = true; } catch {} expect(forged).toBe(fixture.forgedRootAccepted); retire(alias.beginClose()); retire(readers[1]!.beginClose()); expect(value.pages.byteAt(0)).toBe(13);
        const chunks: Uint8Array[] = []; for (;;) { const current = span.advance(grant); if (current.kind === "bytes") chunks.push(current.value); if (current.kind === "complete") break; if (current.kind === "rejected") throw new Error(span.failure!); } expect(Buffer.concat(chunks)).toEqual(value.bytes); retire(span.beginClose()); expect(() => value.pages.byteAt(0)).toThrow();
      }
      for (const text of fixture.invalid) { const value = create(text); expect(finish(value.cursor)).toBe("rejected"); expect(value.cursor.takeResult()).toBeNull(); close(value.cursor); expect(() => value.pages.byteAt(0)).toThrow(); }
      const nested = "[".repeat(fixture.depth) + "0" + "]".repeat(fixture.depth); const deep = create(nested); let stalled = 0; const perRecord = bound(1) + 4 * Math.ceil(Math.log2(deep.bytes.length + 2)) * 512;
      for (let turn = 0;; turn++) { expect(turn).toBeLessThan(bound(deep.bytes.length)); const before = deep.cursor.offset; const current = deep.cursor.advance(grant); expect(current.kind, deep.cursor.failure ?? "deep").not.toBe("rejected"); stalled = deep.cursor.offset > before ? 0 : stalled + 1; expect(stalled).toBeLessThan(perRecord); if (current.kind === "ready") break; } const deepDocument = deep.cursor.takeResult()!; close(deep.cursor); retire(deepDocument.beginClose()); expect(() => deep.pages.byteAt(0)).toThrow();
      for (const cutoff of new Set([...fixture.cancelPrefixes, ...phases.values()])) { const value = create(vectorForCancel()); for (let turn = 0; turn < cutoff; turn++) value.cursor.advance(grant); value.cursor.beginClose(); expect(value.cursor.closeStep({ maxItems: 1, maxBytes: 0 }).kind).toBe("blocked"); close(value.cursor); expect(value.cursor.takeResult()).toBeNull(); expect(() => value.pages.byteAt(0)).toThrow(); }
      function vectorForCancel(): string { return fixture.valid[0]!.text; }
      let forgedCalls = 0; expect(() => Reflect.construct(OwnedUiSceneJsonDocumentCursor, [{ capture: () => { forgedCalls++; return null; } }, 0])).toThrow(); expect(forgedCalls).toBe(0);
    });

    it("OwnedSceneJsonString projects selected exact string tokens into bounded UTF-16 pages", async () => {
      const { OwnedUiSceneJsonStringCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🔤️string/🟦️component.ts"); const { OwnedUiSceneJsonDocumentCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🧾️value/🟦️component.ts"); const { OwnedUiSceneProjectionCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🟦️component.ts"); const { OwnedUiSceneCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️component.ts"); const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts"); const { encodePackValue } = await import("@semio-tech/framework-os"); const { Buffer } = await import("node:buffer");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-json-string.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-json-string.schema.json"); expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
      function create(text: string, ordinal: number, sibling = false) {
        const bytes = Buffer.from(text); const prefix = [13, 1, 6, 6, ...Buffer.from("buffer"), 6]; let remaining = bytes.length; do { const digit = remaining % 128; remaining = Math.floor(remaining / 128); prefix.push(digit | (remaining ? 128 : 0)); } while (remaining);
        const typed = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "text-editor", docSchema: "text-editor@1", doc: { bytes: Array.from(Buffer.concat([Buffer.from(prefix), bytes])) }, bindings: [] }), "component"); expect(finish(typed)).toBe("ready"); const input = typed.takeResult()!; close(typed); if (input.value.type !== "surface") throw new Error("Expected exact scene"); const pages = input.value.doc.bytes; const raw = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(raw)).toBe("ready"); const document = raw.takeResult()!; close(raw); const projection = new OwnedUiSceneProjectionCursor(document, { usizeBits: 64 }); retire(document.beginClose()); expect(finish(projection)).toBe("ready"); const prepared = projection.takeResult()!; close(projection);
        const reader = prepared.beginRecord(); let field: number | null = null; for (;;) { const current = reader.advance(grant); if (current.kind === "value") field = current.value.fields.find(value => value.name === "buffer")?.source ?? null; if (current.kind === "complete") break; } retire(reader.beginClose()); if (field === null) throw new Error("Missing JSON source"); const parser = new OwnedUiSceneJsonDocumentCursor(prepared, field); retire(prepared.beginClose()); expect(finish(parser)).toBe("ready"); const json = parser.takeResult()!; close(parser); const cursor = new OwnedUiSceneJsonStringCursor(json, ordinal); const other = sibling ? new OwnedUiSceneJsonStringCursor(json, ordinal) : null; retire(json.beginClose()); return { cursor, other, bytes, pages };
      }
      const { OwnedUiSceneJsonDocumentReader } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🧾️value/🟦️component.ts"); const terminal = create("\"x\"", 0); const originalAdvance = OwnedUiSceneJsonDocumentReader.prototype.advance; let injected = false;
      const terminalSpy = vi.spyOn(OwnedUiSceneJsonDocumentReader.prototype, "advance").mockImplementation(function (this: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️json/🧾️value/🟦️component.ts").OwnedUiSceneJsonDocumentReader, budget) { const current = originalAdvance.call(this, budget); if (!injected && current.kind === "complete") { injected = true; return { kind: "complete", phase: current.phase, items: 1, bytes: fixture.terminalChildBytes }; } return current; });
      try { for (let turn = 0; !injected; turn++) { expect(turn).toBeLessThan(2048); const current = terminal.cursor.advance(grant); if (injected) { expect(current.kind).toBe("pending"); expect(current.bytes).toBe(fixture.terminalChildBytes); } } } finally { terminalSpy.mockRestore(); }
      const released = terminal.cursor.advance(grant); expect(released.kind).toBe("pending"); expect(released.bytes).toBe(fixture.terminalReleaseBytes); close(terminal.cursor); expect(() => terminal.pages.byteAt(0)).toThrow();
      const phases = new Map<string, number>(); const large = fixture.large.value.repeat(fixture.large.repeat); const vectors = [...fixture.valid, { name: "large", text: JSON.stringify(large), ordinal: 0, value: large }];
      for (const vector of vectors) {
        const value = create(vector.text, vector.ordinal); expect(value.cursor.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); const chunks: string[] = [];
        for (let turn = 0;; turn++) { expect(turn).toBeLessThan(value.bytes.length * 4 + 2048); const before = value.cursor.offset; const current = value.cursor.advance(grant); expect(value.cursor.offset).toBeGreaterThanOrEqual(before); expect(value.cursor.offset - before).toBeLessThanOrEqual(1); expect(current.items).toBeLessThanOrEqual(fixture.maxItems); expect(current.bytes).toBeLessThanOrEqual(fixture.maxBytes); expect(current.kind, value.cursor.failure ?? vector.name).not.toBe("rejected"); if (vector.name === "large" && !phases.has(current.phase)) phases.set(current.phase, turn); if (current.kind === "text") { expect(current.value.length).toBeLessThanOrEqual(fixture.chunkCodeUnits); chunks.push(current.value); } if (current.kind === "ready") break; }
        expect(chunks.join("")).toBe(vector.value); expect(Buffer.from(chunks.join(""), "utf16le")).toEqual(Buffer.from(vector.value, "utf16le")); if (vector.ordinal === 0) expect(chunks.join("")).toBe(JSON.parse(vector.text)); close(value.cursor); expect(() => value.pages.byteAt(0)).toThrow();
      }
      for (const vector of fixture.invalid) { const value = create(vector.text, vector.ordinal); expect(finish(value.cursor)).toBe("rejected"); close(value.cursor); expect(() => value.pages.byteAt(0)).toThrow(); }
      for (const cutoff of new Set([...fixture.cancelPrefixes, ...phases.values()])) { const value = create(JSON.stringify(large), 0); for (let turn = 0; turn < cutoff; turn++) value.cursor.advance(grant); value.cursor.beginClose(); expect(value.cursor.closeStep({ maxItems: 1, maxBytes: 0 }).kind).toBe("blocked"); close(value.cursor); expect(() => value.pages.byteAt(0)).toThrow(); }
      const shared = create(JSON.stringify(large), 0, true); shared.cursor.advance(grant); close(shared.cursor); expect(shared.pages.byteAt(0)).toBe(13); const chunks: string[] = []; for (;;) { const current = shared.other!.advance(grant); if (current.kind === "text") chunks.push(current.value); if (current.kind === "ready") break; if (current.kind === "rejected") throw new Error(shared.other!.failure!); } expect(chunks.join("")).toBe(large); close(shared.other!); expect(() => shared.pages.byteAt(0)).toThrow();
      let captureCalls = 0; expect(() => Reflect.construct(OwnedUiSceneJsonStringCursor, [{ capture: () => { captureCalls++; return null; } }, 0])).toThrow(); expect(captureCalls).toBe(fixture.forgedCaptureCalls);
    });

    it("OwnedScenePackedField decodes captured base64 into independently retired pages with atob and Node Buffer oracles", async () => {
      const { OwnedUiSceneBase64Cursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️pack/🔤️base64/🟦️component.ts"); const { OwnedUiSceneProjectionCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🟦️component.ts"); const { OwnedUiSceneCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️component.ts"); const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts"); const { encodePackValue } = await import("@semio-tech/framework-os"); const { Buffer } = await import("node:buffer");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-pack-field.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-pack-field.schema.json"); expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
      let forgedCaptureCalls = 0; expect(() => Reflect.construct(OwnedUiSceneBase64Cursor, [{ capture: () => { forgedCaptureCalls++; return null; } }, 0])).toThrow(); expect(forgedCaptureCalls).toBe(0);
      function create(text: string) {
        const bytes = Buffer.from(text); const prefix = [13, 1, 6, 6, ...Buffer.from("buffer"), 6]; let remaining = bytes.length; do { const digit = remaining % 128; remaining = Math.floor(remaining / 128); prefix.push(digit | (remaining ? 128 : 0)); } while (remaining);
        const wire = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "text-editor", docSchema: "text-editor@1", doc: { bytes: Array.from(Buffer.concat([Buffer.from(prefix), bytes])) }, bindings: [] }), "component"); expect(finish(wire)).toBe("ready"); const input = wire.takeResult()!; close(wire); if (input.value.type !== "surface") throw new Error("Expected scene"); const pages = input.value.doc.bytes; const raw = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(raw)).toBe("ready"); const document = raw.takeResult()!; close(raw); const projection = new OwnedUiSceneProjectionCursor(document, { usizeBits: 64 }); retire(document.beginClose()); expect(finish(projection)).toBe("ready"); const prepared = projection.takeResult()!; close(projection);
        const reader = prepared.beginRecord(); let field: number | null = null; for (;;) { const current = reader.advance(grant); if (current.kind === "value") field = current.value.fields.find(value => value.name === "buffer")?.source ?? null; if (current.kind === "complete") break; } retire(reader.beginClose()); if (field === null) throw new Error("Missing text source"); const cursor = new OwnedUiSceneBase64Cursor(prepared, field); retire(prepared.beginClose()); return { cursor, pages, bytes };
      }
      const large = Buffer.from(Array.from({ length: fixture.largeBytes }, (_, index) => index % 256)); const vectors = [...fixture.valid, { name: "large", text: `pk:${large.toString("base64")}`, hex: large.toString("hex") }]; const phases = new Map<string, number>();
      for (const vector of vectors) { const oracle = Buffer.from(atob(vector.text.slice(3)), "latin1"); expect(oracle.toString("hex")).toBe(vector.hex); expect(Buffer.from(vector.text.slice(3), "base64")).toEqual(oracle); const { cursor, pages, bytes } = create(vector.text); expect(cursor.takeResult()).toBeNull(); expect(cursor.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked");
        for (let turns = 0;; turns++) { expect(turns).toBeLessThan(bytes.length * 5 + 1024); const before = cursor.sourceBytesRead; const current = cursor.advance(grant); expect(cursor.sourceBytesRead).toBeGreaterThanOrEqual(before); expect(cursor.sourceBytesRead - before).toBeLessThanOrEqual(1); expect(current.items).toBeLessThanOrEqual(1); expect(current.bytes).toBeLessThanOrEqual(4096); expect(current.kind, cursor.failure ?? vector.name).not.toBe("rejected"); if (vector.name === "large" && !phases.has(current.phase)) phases.set(current.phase, turns); if (current.kind === "ready") break; }
        const result = cursor.takeResult()!; expect(cursor.takeResult()).toBeNull(); close(cursor); expect(() => pages.byteAt(0)).toThrow(); expect(result.length).toBe(oracle.length); expect(Buffer.from(Array.from({ length: result.length }, (_, index) => result.byteAt(index)))).toEqual(oracle); const prototype: unknown = Object.getPrototypeOf(result); if (!prototype || typeof prototype !== "object" || !("constructor" in prototype) || typeof prototype.constructor !== "function") throw new Error("Missing packed root constructor"); let forgedRootAccepted = false; try { Reflect.construct(prototype.constructor, [{}, {}]); forgedRootAccepted = true; } catch {} expect(forgedRootAccepted).toBe(fixture.forgedRootAccepted); const alias = result.capture(); retire(result.beginClose()); if (alias.length) expect(alias.byteAt(0)).toBe(oracle[0]); retire(alias.beginClose()); expect(() => alias.byteAt(0)).toThrow();
      }
      for (const text of fixture.invalid) { if (text.startsWith("pk:")) expect(() => atob(text.slice(3))).toThrow(); const { cursor, pages } = create(text); expect(finish(cursor)).toBe("rejected"); expect(cursor.takeResult()).toBeNull(); close(cursor); expect(() => pages.byteAt(0)).toThrow(); }
      for (const cutoff of new Set([...fixture.cancelPrefixes, ...phases.values()])) { const { cursor, pages } = create(`pk:${large.toString("base64")}`); for (let index = 0; index < cutoff; index++) cursor.advance(grant); cursor.beginClose(); expect(cursor.closeStep({ maxItems: 1, maxBytes: 0 }).kind).toBe("blocked"); close(cursor); expect(cursor.takeResult()).toBeNull(); expect(() => pages.byteAt(0)).toThrow(); }
    });

    it("OwnedSceneGenericPack retains original symbols, map pairs and exact scalar spans with native-wire and Node oracles", async () => {
      const { OwnedUiGenericPackCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️pack/🧾️value/🟦️component.ts"); const { OwnedUiSceneBase64Cursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🧩️pack/🔤️base64/🟦️component.ts"); const { OwnedUiSceneProjectionCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🟦️component.ts"); const { OwnedUiSceneCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️component.ts"); const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts"); const { encodePackValue, decodePackValue } = await import("@semio-tech/framework-os"); const { Buffer } = await import("node:buffer");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-generic-pack.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-generic-pack.schema.json"); expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
      function create(binary: Uint8Array) {
        const bytes = Buffer.from(`pk:${Buffer.from(binary).toString("base64")}`); const prefix = [13, 1, 6, 6, ...Buffer.from("buffer"), 6]; let remaining = bytes.length; do { const digit = remaining % 128; remaining = Math.floor(remaining / 128); prefix.push(digit | (remaining ? 128 : 0)); } while (remaining);
        const wire = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: "text-editor", docSchema: "text-editor@1", doc: { bytes: Array.from(Buffer.concat([Buffer.from(prefix), bytes])) }, bindings: [] }), "component"); expect(finish(wire)).toBe("ready"); const input = wire.takeResult()!; close(wire); const raw = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(raw)).toBe("ready"); const document = raw.takeResult()!; close(raw); const projection = new OwnedUiSceneProjectionCursor(document, { usizeBits: 64 }); retire(document.beginClose()); expect(finish(projection)).toBe("ready"); const prepared = projection.takeResult()!; close(projection);
        const reader = prepared.beginRecord(); let field: number | null = null; for (;;) { const current = reader.advance(grant); if (current.kind === "value") field = current.value.fields.find(value => value.name === "buffer")?.source ?? null; if (current.kind === "complete") break; } retire(reader.beginClose()); if (field === null) throw new Error("Missing packed source"); const base64 = new OwnedUiSceneBase64Cursor(prepared, field); retire(prepared.beginClose()); expect(finish(base64)).toBe("ready"); const source = base64.takeResult()!; close(base64); const cursor = new OwnedUiGenericPackCursor(source); retire(source.beginClose()); return cursor;
      }
      function drive(cursor: InstanceType<typeof OwnedUiGenericPackCursor>, bytes: number): string {
        const height = 2 * Math.ceil(Math.log2(bytes + 1)); const allocations = 9 * 10 + 9 + 6 + 9 + 4; const perRecord = 2 * (height + 1) * (allocations + 16) + 9 * height + 64; const bound = (bytes * 3 + 4) * perRecord;
        for (let turn = 0; turn < bound; turn++) { const before = cursor.offset; const current = cursor.advance(grant); if (cursor.offset < before || cursor.offset - before > 1 || current.items > 1 || current.bytes > 4096) throw new Error("Generic pack exceeded exact grant"); if (current.kind === "ready" || current.kind === "rejected") return current.kind; } throw new Error("Generic pack exceeded byte-derived dual-index bound");
      }
      const { NumericIndexRetirement, NumericIndexReader } = await import("../../../../../../../🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🟦️component.ts"); const terminal = create(Buffer.from("0001011112", "hex")); terminal.beginClose(); expect(terminal.closeStep(grant).phase).toBe("generic-pack-symbol-close");
      const originalAdvance = NumericIndexRetirement.prototype.advance; const terminalSpy = vi.spyOn(NumericIndexRetirement.prototype, "advance").mockImplementationOnce(function (this: InstanceType<typeof NumericIndexRetirement>, budget) { const current = originalAdvance.call(this, budget); expect(current.kind).toBe("complete"); expect(this.terminalIsEmpty()).toBe(true); return { kind: "complete", items: 1, bytes: fixture.terminalChildBytes }; });
      try { const current = terminal.closeStep(grant); expect(current.kind).toBe("pending"); expect(current.bytes).toBe(fixture.terminalChildBytes); } finally { terminalSpy.mockRestore(); }
      const released = terminal.closeStep(grant); expect(released.kind).toBe("pending"); expect(released.bytes).toBe(fixture.terminalReleaseBytes); close(terminal);
      const largeValues: unknown[] = [{ [fixture.large.key.repeat(fixture.large.repeat)]: true }, fixture.large.text.repeat(fixture.large.repeat), Array.from({ length: fixture.large.items }, (_, index) => index % 2 === 0)];
      const vectors = [...fixture.valid.map(value => ({ bytes: Buffer.from(value.hex, "hex"), expected: value.expected, keys: value.keys ?? null })), ...largeValues.map(expected => ({ bytes: Buffer.from(encodePackValue(expected)), expected, keys: null }))];
      for (const vector of vectors) { expect(decodePackValue(vector.bytes)).toEqual(vector.expected); const cursor = create(vector.bytes); expect(cursor.takeResult()).toBeNull(); expect(cursor.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(drive(cursor, vector.bytes.length), cursor.failure ?? "generic pack").toBe("ready"); const document = cursor.takeResult()!; close(cursor); const source = document.captureSource(); const reader = document.beginRead(); retire(document.beginClose()); let field = 0; let result: unknown = null; const keys: string[] = []; const stack: { value: unknown[] | Record<string, unknown>; key: string | null }[] = [];
        const put = (value: unknown) => { const parent = stack.at(-1); if (!parent) { if (field === 1) result = value; } else if (Array.isArray(parent.value)) parent.value.push(value); else { if (parent.key === null) throw new Error("Missing generic pack key"); Object.defineProperty(parent.value, parent.key, { value, writable: true, configurable: true, enumerable: true }); parent.key = null; } };
        for (;;) { const current = reader.advance(grant); expect(current.bytes).toBeLessThanOrEqual(4096); if (current.kind === "value") { const token = current.value; if (token.kind === "field") field = token.field; else if (token.kind === "array" || token.kind === "map") { const value = token.kind === "array" ? [] : {}; put(value); stack.push({ value, key: null }); } else if (token.kind === "end-array" || token.kind === "end-map") stack.pop(); else if (token.kind === "key" || token.kind === "string") { const text = Buffer.from(Array.from({ length: token.length }, (_, index) => source.byteAt(token.start + index))).toString("utf8"); if (token.kind === "key") { stack.at(-1)!.key = text; keys.push(text); } else put(text); } else put(token.kind === "number" ? token.value : token.kind === "null" ? null : token.kind === "true"); } if (current.kind === "complete") break; } expect(result).toEqual(JSON.parse(JSON.stringify(vector.expected))); if (vector.keys) expect(keys).toEqual(vector.keys); retire(reader.beginClose()); retire(source.beginClose());
      }
      for (const bits of fixture.floatBits) { const bytes = Buffer.from(`0001011105${bits}`, "hex"); const cursor = create(bytes); expect(drive(cursor, bytes.length)).toBe("ready"); const document = cursor.takeResult()!; close(cursor); const reader = document.beginRead(); retire(document.beginClose()); let observed = false; for (;;) { const current = reader.advance(grant); if (current.kind === "value" && current.value.kind === "number") observed = Object.is(current.value.value, bytes.readDoubleLE(5)); if (current.kind === "complete") break; } expect(observed).toBe(true); retire(reader.beginClose()); }
      for (const vector of fixture.invalid) { const bytes = Buffer.from(vector.hex, "hex"); const cursor = create(bytes); expect(drive(cursor, bytes.length)).toBe("rejected"); expect(cursor.takeResult()).toBeNull(); close(cursor); }
      const deepBytes = Buffer.concat([Buffer.from([0,1,1,17]), Buffer.from(Array.from({ length: fixture.depth * 2 }, (_, index) => index % 2 === 0 ? 12 : 1)), Buffer.from([18])]); const deep = create(deepBytes); expect(drive(deep, deepBytes.length)).toBe("ready"); retire(deep.takeResult()!.beginClose()); close(deep);
      for (const cutoff of fixture.cancelPrefixes) { const cursor = create(deepBytes); for (let index = 0; index < cutoff; index++) cursor.advance(grant); close(cursor); expect(cursor.takeResult()).toBeNull(); }
      let called = false; expect(() => Reflect.construct(OwnedUiGenericPackCursor, [{ capture: () => { called = true; return null; } }])).toThrow(); expect(called).toBe(fixture.forgedRootAccepted);
      for (const target of ["reader", "records"] as const) for (const vector of fixture.childSteps) { const cursor = create(Buffer.from("0001011112", "hex")); expect(drive(cursor, 5)).toBe("ready"); const document = cursor.takeResult()!; close(cursor); const reader = target === "reader" ? document.beginRead() : null; const owner = reader ? reader.beginClose() : document.beginClose(); if (!reader) { owner.advance(grant); owner.advance(grant); } const spy = vi.spyOn(NumericIndexRetirement.prototype, "advance").mockImplementationOnce(() => vector.kind === "blocked" ? { kind: "blocked", items: vector.items, bytes: vector.bytes } : vector.kind === "rejected" ? { kind: "rejected", reason: "ordinal-exhausted", items: vector.items, bytes: vector.bytes } : { kind: "pending", items: vector.items, bytes: vector.bytes }); try { const current = owner.advance(grant); expect(current.kind).toBe(vector.expected); expect(current.items).toBe(vector.items); expect(current.bytes).toBe(vector.bytes); expect(owner.terminalIsEmpty()).toBe(false); } finally { spy.mockRestore(); } retire(owner); if (reader) retire(document.beginClose()); }
      for (const vector of fixture.childSteps.filter(value => value.kind !== "rejected")) { const cursor = create(Buffer.from("0001011112", "hex")); expect(drive(cursor, 5)).toBe("ready"); const document = cursor.takeResult()!; close(cursor); const reader = document.beginRead(); const spy = vi.spyOn(NumericIndexReader.prototype, "advance").mockImplementationOnce(() => vector.kind === "blocked" ? { kind: "blocked", items: vector.items, bytes: vector.bytes } : { kind: "pending", items: vector.items, bytes: vector.bytes }); try { const current = reader.advance(grant); expect(current.kind).toBe(vector.expected); expect(current.bytes).toBe(vector.bytes); } finally { spy.mockRestore(); } retire(reader.beginClose()); retire(document.beginClose()); }
    });

    it("SceneBinding atomically owns the exact node and prepared scene or a surfaced diagnostic", async () => {
      const { OwnedUiSceneBindingCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🟦️component.ts");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-binding.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-binding.schema.json");
      const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true);
      for (const vector of fixture.cases) {
        const record = { ...produce(fields.node, draft => { draft.id = 2 ** 40; }), children: [] };
        const source = { ...record, component: { type: "surface", kind: vector.kind, docSchema: vector.schema, doc: { bytes: vector.packet }, bindings: [] } };
        const typed = new RetainedUiTypedCursor(encodePackValue(source), "node"); expect(finish(typed)).toBe("ready"); const payload = typed.takeResult()!; close(typed); const node = OwnedUiNode.captureFrom(payload); retire(payload.beginClose());
        const cursor = new OwnedUiSceneBindingCursor(node, { usizeBits: 64 }); retire(node.beginClose()); expect(finish(cursor)).toBe("ready"); const binding = cursor.takeResult()!; close(cursor); expect(binding.value.id).toBe(2 ** 40); expect(binding.diagnostic?.code ?? null).toBe(vector.diagnostic);
        const alias = binding.capture(); const reader = alias.beginRecord(); retire(binding.beginClose()); retire(alias.beginClose());
        if (vector.diagnostic) expect(reader).toBeNull(); else { expect(reader).not.toBeNull(); let seen = false; for (;;) { const current = reader!.advance(grant); if (current.kind === "value") seen = current.value.schema === "TextEditorScene"; if (current.kind === "complete") break; } expect(seen).toBe(true); retire(reader!.beginClose()); }
      }
    });
    it("SceneBinding reuses only identical component roots and closes cancellation independently of old issued readers", async () => {
      const { OwnedUiSceneBindingCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🟦️component.ts");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-binding.json");
      const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const vector = fixture.cases[0]!; const wire = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, children: [], component: { type: "surface", kind: vector.kind, docSchema: vector.schema, doc: { bytes: vector.packet }, bindings: [] } }), "node"); expect(finish(wire)).toBe("ready"); const payload = wire.takeResult()!; close(wire); const node = OwnedUiNode.captureFrom(payload); retire(payload.beginClose());
      const first = new OwnedUiSceneBindingCursor(node, { usizeBits: 64 }); const phases = new Map<string, number>(); let count = 0;
      for (;;) { const current = first.advance(grant); if (!phases.has(current.phase)) phases.set(current.phase, count); count++; if (current.kind === "ready") break; expect(current.kind).toBe("pending"); }
      const binding = first.takeResult()!; close(first); const oldReader = binding.beginRecord()!;
      for (const cutoff of [...phases.values(), count]) { const cancelled = new OwnedUiSceneBindingCursor(node, { usizeBits: 64 }); for (let index = 0; index < cutoff; index++) cancelled.advance(grant); close(cancelled); expect(node.terminalIsEmpty()).toBe(false); }
      const styleWire = new RetainedUiTypedCursor(encodePackValue({}), "style"); expect(finish(styleWire)).toBe("ready"); const style = styleWire.takeResult()!; close(styleWire); const changedStyle = node.replace({ field: "style", payload: style });
      const reuse = new OwnedUiSceneBindingCursor(changedStyle, { usizeBits: 64 }); reuse.considerPrevious(binding); retire(changedStyle.beginClose()); const reusedPhases = new Set<string>(); for (;;) { const current = reuse.advance(grant); reusedPhases.add(current.phase); if (current.kind === "ready") break; expect(current.kind).toBe("pending"); }
      expect(reusedPhases.has("scene-binding-packet")).toBe(false); const reused = reuse.takeResult()!; close(reuse); expect(reused.value.component).toBe(binding.value.component); retire(reused.beginClose());
      const replacement = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind: vector.kind, docSchema: vector.schema, doc: { bytes: [13, 0] }, bindings: [] }), "component"); expect(finish(replacement)).toBe("ready"); const component = replacement.takeResult()!; close(replacement); const changed = node.replace({ field: "component", payload: component });
      const fresh = new OwnedUiSceneBindingCursor(changed, { usizeBits: 64 }); fresh.considerPrevious(binding); retire(changed.beginClose()); retire(node.beginClose()); retire(binding.beginClose()); expect(finish(fresh)).toBe("ready"); const rejectedScene = fresh.takeResult()!; close(fresh); expect(rejectedScene.diagnostic?.code).toBe("invalid-scene-fields"); retire(rejectedScene.beginClose());
      let oldValid = false; for (;;) { const current = oldReader.advance(grant); if (current.kind === "value") oldValid = current.value.schema === "TextEditorScene"; if (current.kind === "complete") break; } expect(oldValid).toBe(true); retire(oldReader.beginClose());
    });
    it("TypedScene separates exact numeric wire bits from finite geometry and owning-host usize width", async () => {
      const { OwnedUiSceneProjectionCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/🟦️component.ts");
      const { OwnedUiSceneCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-numeric.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-numeric.schema.json");
      const { Buffer } = await import("node:buffer"); const { encodePackValue } = await import("@semio-tech/framework-os"); expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true);
      const text = (value: string) => { const bytes = Buffer.from(value); return Buffer.concat([Buffer.from([6, bytes.length]), bytes]); };
      const map = (values: readonly (readonly [string, Uint8Array])[]) => Buffer.concat([Buffer.from([13, values.length]), ...values.flatMap(([key, value]) => [text(key), value])]);
      const scalar = (value: number) => { const bytes = Buffer.alloc(9); bytes[0] = 5; bytes.writeDoubleLE(value, 1); return bytes; };
      const zero = Buffer.from([3, 0]); const some = (value: Uint8Array) => Buffer.concat([Buffer.from([9]), value]);
      const source = (bytes: Uint8Array, kind: string) => { const typed = new RetainedUiTypedCursor(encodePackValue({ type: "surface", kind, docSchema: `${kind}@1`, doc: { bytes: Array.from(bytes) }, bindings: [] }), "component"); expect(finish(typed)).toBe("ready"); const input = typed.takeResult()!; close(typed); const parser = new OwnedUiSceneCursor(input); retire(input.beginClose()); expect(finish(parser)).toBe("ready"); const document = parser.takeResult()!; close(parser); return document; };
      for (const vector of fixture.cases) {
        const raw = Buffer.from(vector.hex, "hex"); const generic = source(raw, "canvas-2d"); const read = generic.beginRead(); retire(generic.beginClose());
        for (;;) { const current = read.advance(grant); if (current.kind === "value") { if (current.value.kind === "float") expect(Object.is(current.value.value, raw.readDoubleLE(1))).toBe(true); else expect(current.value.kind).toBe("integer"); } if (current.kind === "complete") break; } retire(read.beginClose());
        if (vector.type === "i64") continue;
        let kind = "canvas-2d"; let packet: Uint8Array;
        if (vector.type === "usize") {
          kind = "node-graph"; const variadic = map([["slotKey", text("slot")], ["min", raw]]); const operator = map([["id", text("op")], ["extension", text("ext")], ["name", text("name")], ["abbreviation", text("op")], ["icon", text("")], ["summary", text("")], ["variadicInput", some(variadic)]]); packet = map([["operators", Buffer.concat([Buffer.from([10, 1]), operator])]]);
        } else {
          const snapshot = map([["slot", vector.type === "u8" ? raw : zero], ["epoch", vector.type === "u64" ? raw : zero], ["revision", zero], ["generation", zero], ["pageCount", zero], ["byteCount", vector.type === "u32" ? raw : zero]]);
          packet = map([["cameraX", scalar(0)], ["cameraY", scalar(0)], ["zoom", vector.type === "f64" ? raw : scalar(1)], ["layersJson", text("[]")], ["snapshot", some(snapshot)]]);
        }
        const document = source(packet, kind);
        for (const usizeBits of [32, 64] as const) {
          const profile: { usizeBits: 32 | 64 } = { usizeBits }; const cursor = new OwnedUiSceneProjectionCursor(document, profile); profile.usizeBits = usizeBits === 32 ? 64 : 32;
          const expected = (usizeBits === 32 ? vector.native32 : vector.native64) && vector.finite; expect(finish(cursor), `${vector.name}/${usizeBits}`).toBe(expected ? "ready" : "rejected"); if (expected) retire(cursor.takeResult()!.beginClose()); close(cursor);
        }
        retire(document.beginClose());
      }
    });
    it("SceneBinding refuses forged typed source constructors before reading caller-owned roots", async () => {
      const { OwnedUiNode, OwnedUiPayload } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      let reads = 0; const forged = Object.defineProperty({}, "value", { get() { reads++; throw new Error("Untrusted value getter"); } });
      expect(() => Reflect.construct(OwnedUiPayload, [forged])).toThrow(/authority/);
      expect(() => Reflect.construct(OwnedUiNode, [forged])).toThrow(/authority/);
      expect(reads).toBe(0);
    });
    it("SceneBindingIndex keeps paired roots through concurrent edit cancellation and insertion-order snapshots", async () => {
      const { OwnedUiNodeReadLease } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️component.ts");
      const { OwnedUiSceneBindingCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🟦️component.ts");
      const { OwnedUiSceneBindingIndex } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🗂️index/🟦️component.ts");
      const { RetainedUiTypedCursor, OwnedUiNode } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-binding.json");
      const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const create = (id: number) => { const vector = fixture.cases[0]!; const typed = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, id, children: [], component: { type: "surface", kind: vector.kind, docSchema: vector.schema, doc: { bytes: vector.packet }, bindings: [] } }), "node"); expect(finish(typed)).toBe("ready"); const payload = typed.takeResult()!; close(typed); const node = OwnedUiNode.captureFrom(payload); retire(payload.beginClose()); const cursor = new OwnedUiSceneBindingCursor(node, { usizeBits: 64 }); retire(node.beginClose()); expect(finish(cursor)).toBe("ready"); const result = cursor.takeResult()!; close(cursor); return result; };
      let index = OwnedUiSceneBindingIndex.empty(); const map = new Map<number, true>();
      for (const id of fixture.indexIds) { const binding = create(id); const edit = index.beginSet(binding); retire(binding.beginClose()); expect(finish(edit)).toBe("ready"); const next = edit.takeResult()!; retire(edit.beginClose()); retire(index.beginClose()); index = next; map.set(id, true); }
      const oldReader = index.beginRead(); const first = fixture.indexIds[0]!; const cancelled = index.beginRemove(first); expect(cancelled.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); cancelled.advance(grant); retire(cancelled.beginClose()); expect(index.size).toBe(2);
      const remove = index.beginRemove(first); expect(finish(remove)).toBe("ready"); const removed = remove.takeResult()!; retire(remove.beginClose()); retire(index.beginClose()); index = removed; map.delete(first);
      const replacement = create(first); const insert = index.beginSet(replacement); retire(replacement.beginClose()); expect(finish(insert)).toBe("ready"); const next = insert.takeResult()!; retire(insert.beginClose()); retire(index.beginClose()); index = next; map.set(first, true);
      const current = index.beginRead(); retire(index.beginClose()); const ids: number[] = [];
      for (;;) { const step = current.advance(grant); if (step.kind === "value") { ids.push(step.id); retire(step.value.beginClose()); } if (step.kind === "complete") break; } expect(ids).toEqual(Array.from(map.keys())); retire(current.beginClose());
      const oldIds: number[] = []; for (;;) { const step = oldReader.advance(grant); if (step.kind === "value") { oldIds.push(step.id); const scene = step.value.beginRecord()!; retire(step.value.beginClose()); let seen = false; for (;;) { const result = scene.advance(grant); if (result.kind === "value") seen = true; if (result.kind === "complete") break; } expect(seen).toBe(true); retire(scene.beginClose()); } if (step.kind === "complete") break; } expect(oldIds).toEqual(fixture.indexIds); retire(oldReader.beginClose());
      const paired = create(first); const lease = new OwnedUiNodeReadLease(first, 0, paired); retire(paired.beginClose()); const oldSnapshot = lease.snapshot;
      const initialChild = oldSnapshot.beginSceneRecord()!; const otherChild = oldSnapshot.beginSceneRecord()!; expect(oldSnapshot.beginSceneRecord()).toBeNull();
      const closingChild = initialChild.beginClose(); expect(oldSnapshot.beginSceneRecord()).toBeNull(); retire(closingChild); const replacementChild = oldSnapshot.beginSceneRecord()!; expect(replacementChild).not.toBeNull();
      const newer = create(first); expect(lease.offer(1, newer)).toBe(true); retire(newer.beginClose()); const latest = lease.snapshot; expect(latest).not.toBe(oldSnapshot); const child = latest.beginSceneRecord()!;
      expect(lease.acknowledge(latest)).toBe(true); lease.advanceRetirement(grant); expect(lease.acknowledge(oldSnapshot)).toBe(false); expect(() => oldSnapshot.beginSceneRecord()).toThrow(/retired/); expect(lease.hasCapacity).toBe(false);
      retire(otherChild.beginClose()); retire(replacementChild.beginClose()); for (let turn = 0; lease.retirementPending && turn < 1000; turn++) lease.advanceRetirement(grant); expect(lease.hasCapacity).toBe(true); expect(latest.hasPreparedScene).toBe(true);
      lease.beginClose(); let completedBeforeChild = false;
      for (let turn = 0; turn < 1000; turn++) { const current = lease.closeStep(grant); if (current.kind === "complete") { completedBeforeChild = true; break; } if (current.kind === "blocked") break; }
      retire(child.beginClose()); close(lease); expect(completedBeforeChild).toBe(false);
    });
    //#endregion 🎬️OwnedSceneTests

    it("OwnedSurfaceScene publishes node and prepared scene atomically and waits for issued scene children on close", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts");
      const { OwnedUiOperation } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-binding.json"); const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const surface = new OwnedUiSurface({ actor: "scene", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const id = fields.node.id; let notifiedPrepared = false;
      const subscription = surface.subscribeNode(id, () => { if (surface.view.revision) notifiedPrepared = subscription.snapshot!.hasPreparedScene; }); while (surface.maintenancePending) surface.advanceMaintenance(grant); const previous = subscription.snapshot;
      const vector = fixture.cases[0]!; const typed = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, children: [], component: { type: "surface", kind: vector.kind, docSchema: vector.schema, doc: { bytes: vector.packet }, bindings: [] } }), "node"); expect(finish(typed)).toBe("ready"); const payload = typed.takeResult()!; close(typed);
      const patch = surface.beginPatch(0, 1); patch.pushOperation(OwnedUiOperation.upsert(payload)); retire(payload.beginClose()); expect(finish(patch)).toBe("ready"); patch.pushOperation(OwnedUiOperation.setRoot(id)); expect(finish(patch)).toBe("ready"); patch.finishInput();
      for (;;) { const current = patch.advance(grant); if (!surface.view.revision) expect(subscription.snapshot).toBe(previous); if (current.kind === "ready") break; if (current.kind === "rejected") throw new Error(patch.failure ?? "Surface rejected"); }
      const snapshot = subscription.snapshot!; const prepared = snapshot.hasPreparedScene; const reader = snapshot.beginSceneRecord(); expect(patch.takeAcknowledgement()?.revision).toBe(1); close(patch); surface.acknowledgeRead(subscription, snapshot); while (surface.maintenancePending) surface.advanceMaintenance(grant);
      const managed = surface.openSceneRecord(subscription, snapshot); expect(managed).not.toBeNull(); expect(surface.openSceneRecord(subscription, snapshot)).toBeNull(); expect(subscription.snapshot).toBe(snapshot);
      surface.unsubscribeNode(subscription); surface.beginClose(); let early = false; for (let turn = 0; turn < 1000; turn++) { if (surface.closeStep(grant).kind === "complete") { early = true; break; } }
      expect(() => managed!.advance(grant)).toThrow();
      if (!reader) throw new Error("Expected prepared reader");
      const foreign = new OwnedUiSurface(surface.identity, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const other = foreign.subscribeNode(id, () => {});
      expect(foreign.retireSceneRead(other, reader)).toBe(false); expect(reader.terminalIsEmpty()).toBe(false); expect(surface.retireSceneRead(subscription, reader)).toBe(true); expect(reader.terminalIsEmpty()).toBe(true); expect(surface.retireSceneRead(subscription, reader)).toBe(false);
      close(surface); foreign.unsubscribeNode(other); close(foreign); expect(prepared).toBe(true); expect(notifiedPrepared).toBe(true); expect(early).toBe(false);
    });

    it("OwnedSurfaceScene React effect cleanup queues exact children across source replacement and unmount", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts");
      const { OwnedUiOperation } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { useOwnedUiScene } = await import("./📖️owned/🟦️component.tsx");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️scene-binding.json"); const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const { render, act, cleanup } = await import("@testing-library/react"); const { createElement } = await import("react"); const vector = fixture.cases[0]!;
      const create = () => {
        const surface = new OwnedUiSurface({ actor: "effect", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
        const typed = new RetainedUiTypedCursor(encodePackValue({ ...fields.node, children: [], component: { type: "surface", kind: vector.kind, docSchema: vector.schema, doc: { bytes: vector.packet }, bindings: [] } }), "node"); expect(finish(typed)).toBe("ready"); const payload = typed.takeResult()!; close(typed);
        const patch = surface.beginPatch(0, 1); patch.pushOperation(OwnedUiOperation.upsert(payload)); retire(payload.beginClose()); expect(finish(patch)).toBe("ready"); patch.pushOperation(OwnedUiOperation.setRoot(fields.node.id)); expect(finish(patch)).toBe("ready"); patch.finishInput(); expect(finish(patch)).toBe("ready"); expect(patch.takeAcknowledgement()?.revision).toBe(1); close(patch); return surface;
      };
      const first = create(); const second = create(); const views: import("./📖️owned/🟦️component.tsx").OwnedUiSceneEffectView[] = []; const readers: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️component.ts").OwnedUiSceneRecordView[] = []; let cleanups = 0;
      const consume = (view: import("./📖️owned/🟦️component.tsx").OwnedUiSceneEffectView) => { views.push(view); const a = view.openRecord(); const b = view.openRecord(); expect(a).not.toBeNull(); expect(b).not.toBeNull(); expect(view.openRecord()).toBeNull(); readers.push(a!, b!); return () => { cleanups++; }; };
      function View({ source }: { source: typeof first }) { const record = useOwnedUiScene(source, fields.node.id, consume); return createElement("span", { "aria-label": "Szene / Scene" }, record ? String(record.id) : "pending"); }
      const root = render(createElement(View, { source: first })); await act(async () => { while (first.maintenancePending) first.advanceMaintenance(grant); }); expect(root.getByLabelText("Szene / Scene").textContent).toBe(String(fields.node.id)); expect(views.length).toBe(1);
      root.rerender(createElement(View, { source: second })); close(first); expect(() => readers[0]!.advance(grant)).toThrow(); expect(views[0]!.openRecord()).toBeNull();
      await act(async () => { while (second.maintenancePending) second.advanceMaintenance(grant); }); expect(views.length).toBe(2); expect(readers[0]!.close()).toBe(false); expect(readers[2]!.advance(grant).kind).not.toBe("rejected");
      second.beginClose(); root.unmount(); close(second); expect(() => readers[2]!.advance(grant)).toThrow(); expect(cleanups).toBe(2); expect(first.terminalIsEmpty() && second.terminalIsEmpty()).toBe(true); cleanup();
    });

    //#region 📥️NativeChildTests
    it("OwnedNativeChild preserves decode refusal and raw over-grant work through both operation layers", async () => {
      const { OwnedUiWireOperationCursor, OwnedUiWirePatchCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts"); const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️native-child.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️native-child.schema.json"); const { encodePackValue } = await import("@semio-tech/framework-os"); const deepStrictEqual: typeof import("node:assert").deepStrictEqual = (await import("node:assert")).deepStrictEqual;
      expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
      for (const layer of ["operation-decode", "stream-decode"]) for (const kind of fixture.refusals) for (const bytes of [0, fixture.overGrantBytes]) {
        const current: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🟦️component.ts").RetainedUiWireStep = { kind: kind === "blocked" ? "blocked" : "rejected", phase: layer, items: bytes ? 1 : 0, bytes };
        const surface = new OwnedUiSurface({ actor: "child", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
        const cursor = layer === "operation-decode" ? new OwnedUiWireOperationCursor("set-component", 7, encodePackValue({ type: "text", value: "Child" })) : new OwnedUiWirePatchCursor(surface, 0, 1, 1);
        if (cursor instanceof OwnedUiWirePatchCursor) expect(cursor.offer(0, { tag: "set-root", val: 7n })).toBe(true);
        const spy = layer === "operation-decode" ? vi.spyOn(RetainedUiTypedCursor.prototype, "advance").mockReturnValueOnce(current) : vi.spyOn(OwnedUiWireOperationCursor.prototype, "advance").mockReturnValueOnce(current);
        try { deepStrictEqual(cursor.advance(grant), { ...current, kind: bytes > grant.maxBytes ? "rejected" : current.kind }); expect(cursor.terminalIsEmpty()).toBe(false); }
        finally { spy.mockRestore(); close(cursor); close(surface); }
      }
    });

    it("OwnedNativeChild preserves exact close refusal without unlinking real payload and decoder owners", async () => {
      const { OwnedUiWireOperationCursor, OwnedUiWirePatchCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️component.ts"); const { RetainedUiTypedCursor, UiPayloadRetirement } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { OwnedUiOperationRetirement } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️component.ts"); const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️native-child.json"); const { encodePackValue } = await import("@semio-tech/framework-os"); const deepStrictEqual: typeof import("node:assert").deepStrictEqual = (await import("node:assert")).deepStrictEqual;
      for (const layer of fixture.layers.filter(value => !value.endsWith("decode"))) for (const kind of fixture.refusals) {
        const current: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🟦️component.ts").RetainedUiWireStep = { kind: kind === "blocked" ? "blocked" : "rejected", phase: layer, items: 0, bytes: 0 };
        const surface = new OwnedUiSurface({ actor: "child-close", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
        const cursor = layer.startsWith("stream") ? new OwnedUiWirePatchCursor(surface, 0, 1, 1) : new OwnedUiWireOperationCursor("set-component", 7, encodePackValue({ type: "text", value: "Close" }));
        if (cursor instanceof OwnedUiWirePatchCursor) { expect(cursor.offer(0, { tag: "set-component", val: { node: 7n, component: encodePackValue({ type: "text", value: "Stream close" }) } })).toBe(true); if (layer === "stream-operation-retirement") { let captured = false; for (let turn = 0; !captured && turn < 10000; turn++) captured = cursor.advance(grant).phase === "native-patch-result"; expect(captured).toBe(true); } }
        else if (layer === "payload-retirement") { let reached = false; for (let turn = 0; !reached && turn < 10000; turn++) reached = cursor.advance(grant).phase === "native-payload-release"; expect(reached).toBe(true); }
        else if (layer === "operation-retirement") expect(finish(cursor)).toBe("ready");
        if (layer !== "payload-retirement") cursor.beginClose(); let called = false;
        const fail = () => { called = true; return current; };
        const spy = layer === "payload-retirement" ? vi.spyOn(UiPayloadRetirement.prototype, "advance").mockImplementation(fail) : layer === "decoder-close" ? vi.spyOn(RetainedUiTypedCursor.prototype, "closeStep").mockImplementation(fail) : layer === "stream-input-close" ? vi.spyOn(OwnedUiWireOperationCursor.prototype, "closeStep").mockImplementation(fail) : vi.spyOn(OwnedUiOperationRetirement.prototype, "advance").mockImplementation(fail);
        try { let observed: typeof current | null = null; for (let turn = 0; !called && turn < 10000; turn++) observed = layer === "payload-retirement" ? cursor.advance(grant) : cursor.closeStep(grant); expect(called, layer).toBe(true); deepStrictEqual(observed, current); expect(cursor.terminalIsEmpty()).toBe(false); }
        finally { spy.mockRestore(); close(cursor); close(surface); }
      }
    });
    it("OwnedNativeChild separates terminal full-grant work and retains thrown close ownership", async () => {
      const { OwnedUiWireOperationCursor, OwnedUiWirePatchCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️component.ts"); const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts"); const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️native-child.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      for (const layer of fixture.terminalLayers) {
        const surface = new OwnedUiSurface({ actor: "terminal-child", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const cursor = layer === "decoder-close" ? new OwnedUiWireOperationCursor("set-component", 7, encodePackValue({ type: "text", value: "Terminal" })) : new OwnedUiWirePatchCursor(surface, 0, 1, 1); if (cursor instanceof OwnedUiWirePatchCursor) expect(cursor.offer(0, { tag: "set-component", val: { node: 7n, component: encodePackValue({ type: "text", value: "Terminal" }) } })).toBe(true); cursor.beginClose(); let terminal = false;
        const typed = RetainedUiTypedCursor.prototype.closeStep; const native = OwnedUiWireOperationCursor.prototype.closeStep;
        const spy = layer === "decoder-close" ? vi.spyOn(RetainedUiTypedCursor.prototype, "closeStep").mockImplementation(function (this: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts").RetainedUiTypedCursor<"component">, budget) { const result = typed.call(this, budget); if (this.terminalIsEmpty()) { terminal = true; return { ...result, kind: "complete", items: 1, bytes: fixture.childBytes }; } return result; }) : vi.spyOn(OwnedUiWireOperationCursor.prototype, "closeStep").mockImplementation(function (this: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️component.ts").OwnedUiWireOperationCursor, budget) { const result = native.call(this, budget); if (this.terminalIsEmpty()) { terminal = true; return { ...result, kind: "complete", items: 1, bytes: fixture.childBytes }; } return result; });
        try { let observed = cursor.closeStep(grant); for (let turn = 0; !terminal && turn < 10000; turn++) observed = cursor.closeStep(grant); expect(terminal).toBe(true); expect(observed).toMatchObject({ kind: "pending", items: 1, bytes: fixture.childBytes }); expect(cursor.terminalIsEmpty()).toBe(false); expect(cursor.closeStep(grant)).toMatchObject({ kind: "pending", bytes: fixture.releaseBytes }); }
        finally { spy.mockRestore(); close(cursor); close(surface); }
      }
      const cursor = new OwnedUiWireOperationCursor("set-component", 7, encodePackValue({ type: "text", value: "Fault" })); cursor.beginClose(); const spy = vi.spyOn(RetainedUiTypedCursor.prototype, "closeStep").mockImplementationOnce(() => { throw new Error(fixture.throwReason); });
      try { expect(cursor.closeStep(grant)).toMatchObject({ kind: "rejected", items: 0, bytes: 0 }); expect(cursor.failure).toBe(fixture.throwReason); expect(cursor.terminalIsEmpty()).toBe(false); }
      finally { spy.mockRestore(); close(cursor); }
    });
    //#endregion 📥️NativeChildTests

    it("OwnedWireOperation consumes all eleven native tags into paired publication before its exact ACK", async () => {
      const { OwnedUiWireOperationCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️component.ts");
      const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️wire-operations.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️wire-operations.schema.json"); const { encodePackValue } = await import("@semio-tech/framework-os"); const { Buffer } = await import("node:buffer");
      expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true);
      const surface = new OwnedUiSurface({ actor: "wire", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const subscription = surface.subscribeNode(fixture.root, () => {}); while (surface.maintenancePending) surface.advanceMaintenance(grant); const original = surface.view; const patch = surface.beginPatch(0, 1); const tags = new Set<string>();
      const apply = (tag: string, target: unknown, payload?: unknown) => { const input = new OwnedUiWireOperationCursor(tag, target, payload); expect(input.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(finish(input)).toBe("ready"); patch.pushOperation(input.takeResult()!); close(input); expect(finish(patch)).toBe("ready"); expect(surface.view).toBe(original); expect(patch.takeAcknowledgement()).toBeNull(); tags.add(tag); };
      const rootBytes = encodePackValue(fixture.node); const alias = new Uint8Array(rootBytes.buffer); apply("upsert", undefined, rootBytes); expect(alias.byteLength).toBe(0);
      apply("upsert", undefined, encodePackValue({ ...fixture.node, id: fixture.child, key: "child" }));
      for (const field of fixture.fields) apply(field.tag, BigInt(fixture.root), encodePackValue(field.value));
      apply("set-children", BigInt(fixture.root), new BigUint64Array()); apply("set-root", BigInt(fixture.root)); apply("remove", BigInt(fixture.child));
      patch.finishInput(); expect(finish(patch)).toBe("ready"); const record = subscription.snapshot!.record!; expect(record.component).toMatchObject({ type: "text", value: "Neu / New 😀" }); expect(record.disabled).toBe(true); expect(record.accessibility.label).toBe("Ansicht / View 😀"); expect(tags.size).toBe(11);
      const bytes = Buffer.from(JSON.stringify(produce({ surface: "window", revision: 1, root: fixture.root, nodes: [record], layoutEpoch: "0" }, () => {}))); let hash = 0x811c9dc5; for (const byte of bytes) hash = Math.imul(hash ^ byte, 0x01000193) >>> 0;
      expect(patch.takeAcknowledgement()).toEqual({ actor: "wire", instance: 7, surface: "window", revision: 1, hash: `${hash.toString(16)}:1` }); close(patch); surface.unsubscribeNode(subscription); close(surface);
    });

    it("OwnedWireOperation rejects hostile transport inputs before transfer and closes every admitted prefix", async () => {
      const { OwnedUiWireOperationCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️component.ts");
      const { RetainedUiChildIdsCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️wire-operations.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const hostile = new Set<string>();
      const refuse = (name: string, tag: string, target: unknown, payload?: unknown) => { const bytes = payload instanceof Uint8Array ? payload.byteLength : null; expect(() => new OwnedUiWireOperationCursor(tag, target, payload)).toThrow(); if (payload instanceof Uint8Array) expect(payload.byteLength).toBe(bytes); hostile.add(name); };
      refuse("unknown-tag", "unsupported", 7, encodePackValue(null)); refuse("unexpected-payload", "remove", 7, encodePackValue(null)); refuse("unsafe-node-id", "set-component", 9007199254740992n, encodePackValue(null)); refuse("negative-node-id", "set-root", -1); refuse("wrong-payload-kind", "upsert", undefined, []);
      const backing = new Uint8Array(32); refuse("payload-subview", "upsert", undefined, backing.subarray(1)); expect(backing.byteLength).toBe(32);
      const shared = new Uint8Array(new SharedArrayBuffer(32)); refuse("payload-shared-buffer", "upsert", undefined, shared);
      refuse("children-over-capacity", "set-children", 7, new BigUint64Array(fixture.nativeChildren.maximum + 1));
      let getterCalls = 0; const accessor: unknown[] = []; Object.defineProperty(accessor, 0, { enumerable: true, get() { getterCalls++; return 1; } });
      refuse("children-accessor", "set-children", 7, accessor);
      const unsafeChildren = new OwnedUiWireOperationCursor("set-children", 7, new BigUint64Array([9007199254740992n])); expect(finish(unsafeChildren)).toBe("rejected"); expect(unsafeChildren.takeResult()).toBeNull(); close(unsafeChildren); hostile.add("children-unsafe-id");
      expect(getterCalls).toBe(0); expect([...hostile].sort()).toEqual([...fixture.hostile].sort());
      const maximum = BigUint64Array.from({ length: fixture.nativeChildren.maximum }, (_, index) => BigInt(fixture.nativeChildren.values[index % 3]!)); const expected = JSON.parse(JSON.stringify(Array.from(maximum, Number))); const alias = new BigUint64Array(maximum.buffer);
      const child = new RetainedUiChildIdsCursor(maximum); expect(alias.byteLength).toBe(0); expect(finish(child)).toBe("ready"); const payload = child.takeResult()!; expect(payload.value).toEqual(expected); expect(Object.is(payload.value[0], -0)).toBe(false); retire(payload.beginClose()); close(child);
      const phases = new Set<string>(); let prefixes = 0;
      for (let prefix = 0; ; prefix++) {
        const input = new OwnedUiWireOperationCursor("upsert", undefined, encodePackValue(fixture.node)); let ready = false;
        for (let index = 0; index < prefix; index++) { const current = input.advance(grant); phases.add(current.phase); expect(current.bytes).toBeLessThanOrEqual(grant.maxBytes); expect(current.items).toBeLessThanOrEqual(1); if (current.kind === "ready") { ready = true; break; } if (current.kind === "rejected") throw new Error(input.failure!); }
        input.beginClose(); expect(input.takeResult()).toBeNull(); expect(input.closeStep({ maxItems: 1, maxBytes: 0 }).kind).toBe("blocked"); close(input); expect(input.terminalIsEmpty()).toBe(true); prefixes++; if (ready) break; expect(prefix).toBeLessThan(4096);
      }
      expect(prefixes).toBeGreaterThan(20); expect([...phases]).toEqual(expect.arrayContaining(["typed-ready", "native-operation-retire", "native-payload-release", "native-operation-ready"]));
    });

    it("OwnedWireOperation transfers intrinsic buffer ownership without reading shadowed view metadata", async () => {
      const { OwnedUiWireOperationCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️component.ts"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const bytes = encodePackValue({ type: "text", value: "Sicher / Safe" }); const original = new Uint8Array(bytes.buffer); const unrelated = new ArrayBuffer(bytes.byteLength); let getters = 0;
      Object.defineProperty(bytes, "buffer", { get() { getters++; return unrelated; } });
      const input = new OwnedUiWireOperationCursor("set-component", 7, bytes); close(input); expect(original.byteLength).toBe(0); expect(unrelated.byteLength).toBeGreaterThan(0); expect(getters).toBe(0);
    });

    it("OwnedWireOperation accepts intrinsic native fields from another realm at the public entry", async () => {
      const { OwnedUiWireOperationCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️component.ts"); const { runInNewContext } = await import("node:vm"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const packet = encodePackValue({ type: "text", value: "Bereich / Realm 😀" }); const bytes: unknown = runInNewContext("new Uint8Array(bytes)", { bytes: Array.from(packet) }); const children: unknown = runInNewContext("new BigUint64Array([0n, 1099511627776n, 9007199254740991n])");
      if (!ArrayBuffer.isView(bytes) || !ArrayBuffer.isView(children)) throw new Error("VM did not create native views"); expect(bytes instanceof Uint8Array).toBe(false); expect(children instanceof BigUint64Array).toBe(false);
      for (const [tag, payload] of [["set-component", bytes], ["set-children", children]] as const) { const input = new OwnedUiWireOperationCursor(tag, 7, payload); expect(payload.byteLength).toBe(0); expect(finish(input)).toBe("ready"); retire(input.takeResult()!.beginClose()); close(input); }
    });

    it("OwnedWireOperation mint refuses arbitrary normalized roots before touching source getters", async () => {
      const { OwnedUiOperation, OwnedUiOperationRetirement } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️component.ts");
      let reads = 0; const source = { get kind() { reads++; return "root"; }, get id() { reads++; return 7; } };
      let rejected = false; let forged: ReturnType<typeof OwnedUiOperation.remove> | undefined;
      try { forged = Reflect.construct(OwnedUiOperation, [source]); } catch { rejected = true; }
      if (forged) retire(forged.beginClose()); expect(rejected).toBe(true); expect(reads).toBe(0);
      expect(() => Reflect.construct(OwnedUiOperationRetirement, [source])).toThrow(/authority/); expect(reads).toBe(0);
    });

    it("OwnedWireOperation stream retains one exact ordinal through close and preserves committed late-cancel ACK", async () => {
      const { OwnedUiWirePatchCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️component.ts"); const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️wire-operations.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const surface = new OwnedUiSurface({ actor: "stream", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const original = surface.view;
      const patch = new OwnedUiWirePatchCursor(surface, 0, 1, fixture.stream.operations); let reads = 0; const forbidden = { get tag() { reads++; throw new Error("Unadmitted page access"); } };
      expect(patch.offer(1, forbidden)).toBe(false); expect(reads).toBe(0); const bytes = encodePackValue(fixture.node); expect(patch.offer(0, { tag: "upsert", val: { node: bytes } })).toBe(true); expect(bytes.byteLength).toBe(0); expect(patch.offer(0, forbidden)).toBe(false); expect(patch.offer(1, forbidden)).toBe(false); expect(reads).toBe(0);
      expect(finish(patch)).toBe("ready"); expect(surface.view).toBe(original); expect(patch.takeAcknowledgement()).toBeNull(); expect(patch.offer(1, forbidden)).toBe(false); expect(patch.takePageReceipt()).toEqual({ ordinal: 0 }); expect(patch.takePageReceipt()).toBeNull(); expect(patch.offer(0, forbidden)).toBe(false);
      expect(patch.offer(1, { tag: "set-root", val: BigInt(fixture.root) })).toBe(true); expect(finish(patch)).toBe("ready"); expect(patch.takePageReceipt()).toEqual({ ordinal: 1 }); expect(surface.view).toBe(original); patch.finishInput();
      while (surface.view === original) { const current = patch.advance(grant); if (current.kind === "rejected") throw new Error(patch.failure!); expect(current.bytes).toBeLessThanOrEqual(4096); }
      patch.beginClose(); let blocked = false; for (let i = 0; i < 100_000; i++) { const current = patch.closeStep(grant); if (current.kind === "blocked") { expect(current.phase).toBe("surface-acknowledgement"); blocked = true; break; } if (current.kind === "rejected" || current.kind === "complete") throw new Error("Committed receipt was lost"); }
      expect(blocked).toBe(true); expect(patch.takeAcknowledgement()).toMatchObject({ actor: "stream", instance: 7, surface: "window", revision: 1 }); close(patch);
      const cancelled = new OwnedUiWirePatchCursor(surface, 1, 2, 1); const pending = encodePackValue(fixture.node); expect(cancelled.offer(0, { tag: "upsert", val: { node: pending } })).toBe(true); cancelled.beginClose(); close(cancelled); expect(cancelled.takeAcknowledgement()).toBeNull(); expect(surface.view.revision).toBe(1); close(surface);
    });

    it("OwnedWireOperation native retirement closes known fields without claiming unknown wrapper ownership", async () => {
      const { OwnedUiWireOperationCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️wire-operations.json"); const { encodePackValue } = await import("@semio-tech/framework-os"); const { Buffer } = await import("node:buffer");
      const bytes = encodePackValue(fixture.node); const unknown = { bytes: new Uint8Array(fixture.rawRetirement.unknownBytes).fill(17) }; const original = { tag: "upsert", val: { node: bytes, unknown }, unknown }; const cursor = OwnedUiWireOperationCursor.fromNative(original); expect(bytes.byteLength).toBe(0); close(cursor); expect(original.unknown).toBe(unknown); expect(original.val.unknown).toBe(unknown); expect(Buffer.from(unknown.bytes)).toEqual(Buffer.alloc(fixture.rawRetirement.unknownBytes, 17));
      const children = BigUint64Array.from(fixture.nativeChildren.values, BigInt); const child = OwnedUiWireOperationCursor.fromNative({ tag: "set-children", val: { node: 7n, children } }); expect(children.byteLength).toBe(0); close(child); close(OwnedUiWireOperationCursor.fromNative({ tag: "remove", val: 7n }));
      const untouched = encodePackValue({}); expect(() => OwnedUiWireOperationCursor.fromNative({ tag: "set-style", val: { node: 9007199254740992n, style: untouched } })).toThrow(/range/); expect(untouched.byteLength).toBeGreaterThan(0); let reads = 0; expect(() => OwnedUiWireOperationCursor.fromNative({ tag: "upsert", get val() { reads++; return { node: untouched }; } })).toThrow(/accessor/); expect(reads).toBe(0); expect(untouched.byteLength).toBeGreaterThan(0);
    });

    it("OwnedInstance binds the exact activation and guest lifetime while closing old readers independently", async () => {
      const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️instance-owner.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️instance-owner.schema.json");
      expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true); let active = true;
      const activation = Object.freeze({ actorId: fixture.actor, activationGeneration: BigInt(fixture.activationGeneration), assertActive() { if (!active) throw new Error("Revoked activation"); }, async turn() { return {}; } }); const foreign = Object.freeze({ ...activation });
      const oldLife = Object.freeze({ activationGeneration: activation.activationGeneration, instanceId: fixture.instanceId, guestLifetime: BigInt(fixture.guestLifetimes[0]!) }); const nextLife = produce(oldLife, draft => { draft.guestLifetime = BigInt(fixture.guestLifetimes[1]!); });
      const old = new OwnedUiInstance(activation, oldLife, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const next = new OwnedUiInstance(activation, nextLife, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
      expect(old.beginSurfaceLookup(foreign, oldLife, "window")).toBeNull(); expect(old.beginSurfaceLookup(activation, nextLife, "window")).toBeNull();
      const first = old.beginSurfaceLookup(activation, oldLife, "window")!; expect(old.beginSurfaceLookup(activation, oldLife, "inspector")).toBeNull(); expect(first.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(finish(first)).toBe("ready"); const view = first.takeResult()!; close(first);
      const again = old.beginSurfaceLookup(activation, oldLife, "window")!; expect(finish(again)).toBe("ready"); expect(again.takeResult()).toBe(view); close(again);
      const other = next.beginSurfaceLookup(activation, nextLife, "window")!; expect(finish(other)).toBe("ready"); const replacement = other.takeResult()!; close(other); expect(replacement).not.toBe(view);
      const reader = view.subscribeNode(7, () => {}); while (old.maintenancePending) old.advanceMaintenance(grant); expect(reader.snapshot?.record).toBeUndefined();
      expect(OwnedUiInstance.matches(old, activation, nextLife)).toBe(false); active = false; expect(() => old.beginSurfaceLookup(activation, oldLife, "inspector")).toThrow(/Revoked/);
      old.beginClose(); let waiting = false; for (let i = 0; i < 100; i++) { const current = old.closeStep(grant); if (current.kind === "blocked") { waiting = true; expect(current.phase).toBe("surface-readers"); break; } } expect(waiting).toBe(true); view.unsubscribeNode(reader); close(old); expect(old.terminalIsEmpty()).toBe(true); expect(next.terminalIsEmpty()).toBe(false); expect(replacement.view.revision).toBe(0); close(next);
    });

    it("OwnedRootSource renders exact native roots through independent consumers and retains close until unmount", async () => {
      const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { useOwnedUiView, useOwnedUiNode } = await import("./📖️owned/🟦️component.tsx");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️root-source.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️root-source.schema.json"); const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️wire-operations.json");
      const { encodePackValue } = await import("@semio-tech/framework-os"); const { render, act, cleanup } = await import("@testing-library/react"); const { createElement, StrictMode } = await import("react"); expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
      const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const lookup = owner.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const source = lookup.takeResult()!; close(lookup);
      function Node({ id }: { id: number }) { const record = useOwnedUiNode(source, id); return createElement("span", null, record?.component.type === "text" ? record.component.value : "pending"); }
      function View({ label }: { label: string }) { const current = useOwnedUiView(source); return createElement("section", { "aria-label": label }, current.root === null ? "pending" : createElement(Node, { key: current.root, id: current.root })); }
      const first = render(createElement(StrictMode, null, createElement(View, { label: fixture.labels[0]! }))); const second = render(createElement(View, { label: fixture.labels[1]! }));
      const maintenance = async () => { await act(async () => { for (let n = 0; owner.maintenancePending; n++) { expect(n).toBeLessThan(10000); const current = owner.advanceMaintenance(grant); expect(current.items).toBeLessThanOrEqual(1); expect(current.bytes).toBeLessThanOrEqual(4096); } }); };
      await maintenance(); const initial = source.view; expect(source.view).toBe(initial); expect(first.getByLabelText(fixture.labels[0]!).textContent).toBe("pending");
      const publish = async (id: number, text: string, revision: number, cancelled = false) => {
        const input = produce(fields.node, draft => { draft.id = id; draft.component.value = text; }); const previous = source.view; const operations: unknown[] = previous.root === null ? [] : [{ tag: "remove", val: BigInt(previous.root) }]; operations.push({ tag: "upsert", val: { node: encodePackValue(input) } }, { tag: "set-root", val: BigInt(id) }); const authority = await native.source(operations, previous.revision, revision); const patch = owner.beginPatch(authority, source);
        for (let ordinal = 0; ordinal < authority.value.operationCount; ordinal++) { expect(patch.offer(ordinal)).toBe(true); expect(finish(patch), patch.failure ?? "native input").toBe("ready"); expect(patch.releaseInputReceipt(patch.peekInputReceipt()!)).toBe(true); expect(source.view).toBe(previous); }
        if (cancelled) { close(patch); expect(source.view).toBe(previous); return; }
        patch.finishInput(); await act(async () => { expect(finish(patch), patch.failure ?? "native publication").toBe("ready"); }); const token = patch.peekAcknowledgement()!; expect(token.value.revision).toBe(revision); const submitted = await native.answer(native.lease.submitUiAcknowledgement(authority, token, native.budget), { status: { tag: "idle" } }); expect(patch.acceptAcknowledgement(submitted.receipt)).toBe(true); close(patch); await maintenance();
      };
      const oracle = JSON.parse(JSON.stringify(produce(fixture.nodes, () => {}))); const a = fixture.nodes[0]!; const b = fixture.nodes[1]!;
      await publish(a.id, a.text, 1); expect(source.view).not.toBe(initial); expect(source.view).toBe(source.view); expect(source.view.root).toBe(a.id); expect(first.getByLabelText(fixture.labels[0]!).textContent).toBe(oracle[0].text); expect(second.getByLabelText(fixture.labels[1]!).textContent).toBe(oracle[0].text);
      const published = source.view; await publish(b.id, fixture.cancelled, 2, true); expect(source.view).toBe(published); expect(first.getByLabelText(fixture.labels[0]!).textContent).toBe(oracle[0].text);
      await publish(b.id, b.text, 2); expect(source.view.root).toBe(b.id); expect(first.getByLabelText(fixture.labels[0]!).textContent).toBe(oracle[1].text); expect(second.getByLabelText(fixture.labels[1]!).textContent).toBe(oracle[1].text);
      first.unmount(); await maintenance(); expect(second.getByLabelText(fixture.labels[1]!).textContent).toBe(oracle[1].text); native.lease.beginClose(); owner.beginClose(); let waiting = false;
      for (let n = 0; n < 100; n++) { const current = owner.closeStep(grant); expect(current.kind).not.toBe("complete"); if (current.kind === "blocked") { expect(current.phase).toBe("surface-readers"); waiting = true; break; } }
      expect(waiting).toBe(true); expect(owner.takeRetirementWitness()).toBeNull(); second.unmount(); close(owner); expect(owner.takeRetirementWitness()).not.toBeNull(); native.client.disposeAll(); cleanup();
    });

    it("OwnedIntake retains one lookup and exact native source until successful publication receipt", async () => {
      const { OwnedUiPatchIntake } = await import("./📥️intake/🟦️component.ts"); const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️intake.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️intake.schema.json"); const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️wire-operations.json"); const { encodePackValue } = await import("@semio-tech/framework-os"); expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
      const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const foreign = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const source = await native.source([{ tag: "upsert", val: { node: encodePackValue(fields.node) } }, { tag: "set-root", val: BigInt(fields.root) }]); expect(source.value.operationCount).toBe(fixture.operationCount); expect(() => new OwnedUiPatchIntake(foreign, source)).toThrow(/owner/);
      const busy = owner.beginSurfaceLookup(activation, lifetime, "window")!; const intake = new OwnedUiPatchIntake(owner, source); expect(intake.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(intake.advance(grant)).toMatchObject({ kind: "blocked", phase: "intake-lookup-capacity" }); expect(intake.takeSurface()).toBeNull(); expect(source.inputRetired).toBe(false); close(busy);
      for (let n = 0; !intake.peekAcknowledgement(); n++) { expect(n).toBeLessThan(10000); const current = intake.advance(grant); expect(current.kind, intake.failure ?? "intake").not.toBe("rejected"); expect(current.items).toBeLessThanOrEqual(1); expect(current.bytes).toBeLessThanOrEqual(4096); expect(intake.takeSurface()).toBeNull(); }
      const token = intake.peekAcknowledgement()!; expect(source.inputRetired).toBe(true); expect(intake.advance(grant).kind).toBe("blocked"); native.worker.refuse = true; await expect(native.lease.submitUiAcknowledgement(source, token, native.budget)).rejects.toThrow(/refusal/); expect(intake.peekAcknowledgement()).toBe(token); expect(intake.acceptAcknowledgement(Object.create(Object.getPrototypeOf(token)))).toBe(false);
      const receipt = await native.answer(native.lease.submitUiAcknowledgement(source, token, native.budget), { status: { tag: "idle" } }); expect(intake.acceptAcknowledgement(receipt.receipt)).toBe(true); expect(intake.acceptAcknowledgement(receipt.receipt)).toBe(false); expect(finish(intake)).toBe("ready"); const view = intake.takeSurface()!; expect(view.view).toMatchObject(produce({ root: fields.root, revision: 1 }, () => {})); expect(intake.takeSurface()).toBeNull(); close(intake); expect(owner.terminalIsEmpty()).toBe(false); expect(view.view.root).toBe(fields.root); close(owner); close(foreign); native.client.disposeAll();
    });

    it("OwnedIntake cancellation retires each accepted prefix without closing the whole instance", async () => {
      const { OwnedUiPatchIntake } = await import("./📥️intake/🟦️component.ts"); const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️intake.json"); const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️wire-operations.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      for (const prefix of fixture.cancelPrefixes) {
        const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const source = await native.source([{ tag: "upsert", val: { node: encodePackValue(fields.node) } }, { tag: "set-root", val: BigInt(fields.root) }]); const intake = new OwnedUiPatchIntake(owner, source);
        for (let n = 0; n < prefix; n++) { const current = intake.advance(grant); expect(current.kind, intake.failure ?? "prefix").not.toBe("rejected"); if (intake.peekAcknowledgement()) break; }
        intake.beginClose(); expect(intake.closeStep({ maxItems: 1, maxBytes: 0 }).kind).toBe("blocked"); expect(intake.takeSurface()).toBeNull();
        for (let n = 0; !intake.terminalIsEmpty(); n++) { expect(n).toBeLessThan(10000); const token = intake.peekAcknowledgement(); if (token) { const receipt = await native.answer(native.lease.submitUiAcknowledgement(source, token, native.budget), { status: { tag: "idle" } }); expect(intake.acceptAcknowledgement(receipt.receipt)).toBe(true); } const current = intake.closeStep(grant); expect(current.items).toBeLessThanOrEqual(1); expect(current.bytes).toBeLessThanOrEqual(4096); }
        expect(owner.terminalIsEmpty()).toBe(false); close(owner); native.client.disposeAll();
      }
    });

    it("OwnedIntake late cancellation preserves committed notification and ACK under the shared close driver", async () => {
      const { OwnedUiPatchIntake } = await import("./📥️intake/🟦️component.ts"); const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️wire-operations.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const lookup = owner.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup); let notifications = 0; const subscription = surface.subscribeView(() => { notifications++; }); while (owner.maintenancePending) owner.advanceMaintenance(grant); const before = notifications;
      const source = await native.source([{ tag: "upsert", val: { node: encodePackValue(fields.node) } }, { tag: "set-root", val: BigInt(fields.root) }]); const intake = new OwnedUiPatchIntake(owner, source);
      for (let n = 0; surface.view.revision === 0; n++) { expect(n).toBeLessThan(10000); const current = intake.advance(grant); expect(current.kind, intake.failure ?? "publication").not.toBe("rejected"); }
      expect(intake.peekAcknowledgement()).toBeNull(); expect(notifications).toBe(before); native.lease.beginClose(); owner.beginClose(); intake.beginClose();
      for (let n = 0; !intake.peekAcknowledgement(); n++) { expect(n).toBeLessThan(10000); expect(owner.closeStep(grant).kind).not.toBe("complete"); const current = intake.closeStep(grant); expect(current.bytes).toBeLessThanOrEqual(4096); }
      const token = intake.peekAcknowledgement()!; expect(notifications).toBeGreaterThan(before); expect(owner.takeRetirementWitness()).toBeNull(); native.worker.refuse = true; await expect(native.lease.submitUiAcknowledgement(source, token, native.budget)).rejects.toThrow(/refusal/); expect(intake.peekAcknowledgement()).toBe(token); expect(intake.closeStep(grant).kind).toBe("blocked");
      const receipt = await native.answer(native.lease.submitUiAcknowledgement(source, token, native.budget), { status: { tag: "idle" } }); expect(intake.acceptAcknowledgement(receipt.receipt)).toBe(true); surface.unsubscribeNode(subscription);
      for (let n = 0; !intake.terminalIsEmpty() || !owner.terminalIsEmpty(); n++) { expect(n).toBeLessThan(10000); if (!owner.terminalIsEmpty()) owner.closeStep(grant); if (!intake.terminalIsEmpty()) intake.closeStep(grant); }
      expect(intake.takeSurface()).toBeNull(); expect(owner.takeRetirementWitness()).not.toBeNull(); native.client.disposeAll();
    });

    it("OwnedIntake preserves an injected committed close rejection and its exact retained owner", async () => {
      const { OwnedUiPatchIntake } = await import("./📥️intake/🟦️component.ts"); const { OwnedUiInstance, OwnedUiInstancePatch } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️intake-close-fault.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️intake-close-fault.schema.json"); const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️wire-operations.json"); const { encodePackValue } = await import("@semio-tech/framework-os"); expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
      const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const lookup = owner.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup); const source = await native.source([{ tag: "upsert", val: { node: encodePackValue(fields.node) } }, { tag: "set-root", val: BigInt(fields.root) }]); const intake = new OwnedUiPatchIntake(owner, source);
      for (let n = 0; surface.view.revision === 0; n++) { expect(n).toBeLessThan(10000); expect(intake.advance(grant).kind).not.toBe("rejected"); } intake.beginClose();
      const fault = vi.spyOn(OwnedUiInstancePatch.prototype, "closeStep").mockReturnValueOnce({ kind: "rejected", phase: fixture.phase, items: fixture.items, bytes: fixture.bytes });
      try { const actual = intake.closeStep(grant); expect(actual).toEqual(produce({ kind: fixture.kind, phase: fixture.phase, items: fixture.items, bytes: fixture.bytes }, () => {})); expect(intake.failure).toContain(fixture.phase); expect(intake.terminalIsEmpty()).toBe(!fixture.retainsOwner); expect(intake.peekAcknowledgement() !== null).toBe(fixture.issuesAcknowledgement); expect(intake.takeSurface()).toBeNull(); } finally { fault.mockRestore(); }
      for (let n = 0; !intake.terminalIsEmpty(); n++) { expect(n).toBeLessThan(10000); const token = intake.peekAcknowledgement(); if (token) { const receipt = await native.answer(native.lease.submitUiAcknowledgement(source, token, native.budget), { status: { tag: "idle" } }); expect(intake.acceptAcknowledgement(receipt.receipt)).toBe(true); } intake.closeStep(grant); }
      expect(intake.failure).toContain(fixture.phase); close(owner); native.client.disposeAll();
    });

    it("OwnedIntake preserves failed notification ownership and permits exact consumer retry before later publication", async () => {
      const { OwnedUiPatchIntake } = await import("./📥️intake/🟦️component.ts"); const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️intake-notification.json"); const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️intake-notification.schema.json"); const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️wire-operations.json"); const { encodePackValue } = await import("@semio-tech/framework-os"); expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
      const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const lookup = owner.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup); const events: string[] = []; let fail = true;
      const subscription: import("./📖️owned/🟦️component.tsx").OwnedUiReadSubscription = surface.subscribeNode(fields.root, () => { const snapshot = subscription.snapshot!; if (snapshot.version === 1 && fail) { fail = false; events.push("rejected"); throw new Error("Temporary consumer failure"); } events.push(String(snapshot.version)); surface.acknowledgeRead(subscription, snapshot); }); while (owner.maintenancePending) owner.advanceMaintenance(grant);
      const publish = async (operations: readonly unknown[], revision: number) => { const source = await native.source(operations, revision - 1, revision); const intake = new OwnedUiPatchIntake(owner, source); for (let n = 0; !intake.peekAcknowledgement(); n++) { expect(n).toBeLessThan(10000); expect(intake.advance(grant).kind, intake.failure ?? "notification").not.toBe("rejected"); } const token = intake.peekAcknowledgement()!; const receipt = await native.answer(native.lease.submitUiAcknowledgement(source, token, native.budget), { status: { tag: "idle" } }); expect(intake.acceptAcknowledgement(receipt.receipt)).toBe(true); expect(finish(intake)).toBe("ready"); expect(intake.takeSurface()).toBe(surface); close(intake); };
      await publish([{ tag: "upsert", val: { node: encodePackValue(fields.node) } }, { tag: "set-root", val: BigInt(fields.root) }], 1); expect(events).toEqual(fixture.events.slice(0, 2)); expect(owner.terminalIsEmpty()).toBe(fixture.retireBeforeRetry); expect(surface.retryNotification(subscription)).toBe(fixture.retryAccepted); expect(surface.retryNotification(subscription)).toBe(fixture.duplicateRetryAccepted); while (owner.maintenancePending) owner.advanceMaintenance(grant); expect(events).toEqual(fixture.events.slice(0, 3));
      await publish([{ tag: "set-component", val: { node: BigInt(fields.root), component: encodePackValue({ type: "text", value: "Recovered / Wiederhergestellt" }) } }], 2); expect(events).toEqual(produce(fixture.events, () => {})); surface.unsubscribeNode(subscription); close(owner); native.client.disposeAll();
    });

    it("OwnedInstance old patch facades cannot read or release a successor input receipt", async () => {
      const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️wire-operations.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const instance = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(instance);
      const lookup = instance.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const view = lookup.takeResult()!; close(lookup);
      const old = instance.beginPatch(await native.source([]), view); close(old); const next = instance.beginPatch(await native.source([{ tag: "upsert", val: { node: encodePackValue(fields.node) } }]), view); expect(next.offer(0)).toBe(true); expect(finish(next)).toBe("ready"); const receipt = next.peekInputReceipt()!;
      expect(() => old.peekInputReceipt()).toThrow(/retired/); expect(() => old.releaseInputReceipt(receipt)).toThrow(/retired/); expect(next.peekInputReceipt()).toBe(receipt); expect(next.releaseInputReceipt(receipt)).toBe(true); close(next); close(instance); native.client.disposeAll();
    });

    it("OwnedInstance acknowledgement token rejects forged roots without consulting public properties", async () => {
      const { OwnedUiPatchAcknowledgement } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); let reads = 0; const source = Object.freeze({}); const forged = { get value() { reads++; throw new Error("Forged value access"); }, matches() { reads++; return true; } };
      expect(OwnedUiPatchAcknowledgement.matches(forged, source)).toBe(false); expect(() => Reflect.construct(OwnedUiPatchAcknowledgement, [forged, source, forged])).toThrow(/authority/); expect(reads).toBe(0);
    });

    it("OwnedInstance input-retirement token rejects forged ordinals and preserves cancelled exact source ownership", async () => {
      const { OwnedUiInstance, OwnedUiPatchInputRetirement } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { encodePackValue } = await import("@semio-tech/framework-os"); const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️wire-operations.json");
      let reads = 0; const forged = { get ordinal() { reads++; return 0; }, matches() { reads++; return true; } }; expect(OwnedUiPatchInputRetirement.matches(forged, {}, 0, {})).toBe(false); expect(() => Reflect.construct(OwnedUiPatchInputRetirement, [forged, {}, 0, {}])).toThrow(/authority/); expect(reads).toBe(0);
      const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const lookup = owner.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup);
      const bytes = encodePackValue(fields.node); const operation = { tag: "upsert", val: { node: bytes } }; const source = await native.source([operation]); const patch = owner.beginPatch(source, surface); expect(patch.offer(0)).toBe(true); expect(patch.peekInputReceipt()).toBeNull(); native.lease.beginClose(); patch.beginClose(); expect(patch.closeStep({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(patch.peekInputReceipt()).toBeNull();
      let waiting = false; for (let i = 0; i < 500_000; i++) { const current = patch.closeStep(grant); expect(current.bytes).toBeLessThanOrEqual(4096); if (current.kind === "blocked") { expect(current.phase).toBe("instance-input-retirement"); waiting = true; break; } if (current.kind === "complete") throw new Error("Cancelled input was discarded before exact native release"); } expect(waiting).toBe(true);
      const token = patch.peekInputReceipt()!; expect(token.ordinal).toBe(0); expect(patch.peekInputReceipt()).toBe(token); expect(OwnedUiPatchInputRetirement.matches(token, source, 0, operation)).toBe(true); expect(OwnedUiPatchInputRetirement.matches(token, source, 1, operation)).toBe(false); expect(OwnedUiPatchInputRetirement.matches(token, source, 0, { ...operation })).toBe(false); expect(OwnedUiPatchInputRetirement.matches(token, {}, 0, operation)).toBe(false); expect(patch.releaseInputReceipt(Object.create(OwnedUiPatchInputRetirement.prototype))).toBe(false); expect(patch.peekInputReceipt()).toBe(token); expect(patch.releaseInputReceipt(token)).toBe(true); expect(patch.releaseInputReceipt(token)).toBe(false); expect(patch.peekInputReceipt()).toBeNull(); close(patch); close(owner); native.client.disposeAll();
    });

    it("OwnedInstance input acceptance is privately minted only after successful wire admission", async () => {
      const { OwnedUiInstance, OwnedUiPatchInputAcceptance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); let reads = 0; const forged = { get ordinal() { reads++; return 0; }, matches() { reads++; return true; } }; expect(OwnedUiPatchInputAcceptance.matches(forged, {}, 0, {})).toBe(false); expect(() => Reflect.construct(OwnedUiPatchInputAcceptance, [forged, {}, 0, {}])).toThrow(/authority/); expect(reads).toBe(0);
      const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const lookup = owner.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup);
      const allocation = new Uint8Array(10); const partial = allocation.subarray(2, 4); const operation = { tag: "upsert", val: { node: partial } }; const source = await native.source([operation]); const patch = owner.beginPatch(source, surface); expect(() => patch.offer(0)).toThrow("Native ownership requires an entire non-shared admitted buffer"); expect(allocation.byteLength).toBe(10); expect(patch.peekInputReceipt()).toBeNull(); expect(source.inputRetired).toBe(false); close(patch); expect(patch.peekInputReceipt()).toBeNull(); close(owner); native.client.disposeAll();
    });

    it("OwnedInstance retirement witness requires exact terminal descendants and transfers once after revocation", async () => {
      const { OwnedUiInstance, OwnedUiInstanceRetirement } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️instance-owner.json");
      let active = true; const activation = Object.freeze({ actorId: fixture.actor, activationGeneration: 9n, assertActive() { if (!active) throw new Error("Revoked activation"); }, async turn() { return {}; } }); const lifetime = Object.freeze({ activationGeneration: 9n, instanceId: fixture.instanceId, guestLifetime: BigInt(fixture.guestLifetimes[0]!) }); const nextLife = produce(lifetime, draft => { draft.guestLifetime = BigInt(fixture.guestLifetimes[1]!); });
      const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const foreign = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
      const lookup = owner.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup); const reader = surface.subscribeNode(7, () => {}); while (owner.maintenancePending) owner.advanceMaintenance(grant);
      expect(owner.takeRetirementWitness()).toBeNull(); active = false; owner.beginClose(); expect(owner.closeStep({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked"); expect(owner.takeRetirementWitness()).toBeNull();
      let blocked = false; for (let i = 0; i < 100; i++) { const current = owner.closeStep(grant); if (current.kind === "blocked") { blocked = true; break; } } expect(blocked).toBe(true); expect(owner.takeRetirementWitness()).toBeNull(); surface.unsubscribeNode(reader); close(owner);
      const witness = owner.takeRetirementWitness(); expect(witness).not.toBeNull(); expect(owner.takeRetirementWitness()).toBeNull(); expect(OwnedUiInstanceRetirement.matches(witness, owner, activation, lifetime)).toBe(true); expect(OwnedUiInstanceRetirement.matches(witness, foreign, activation, lifetime)).toBe(false); expect(OwnedUiInstanceRetirement.matches(witness, owner, Object.freeze({ ...activation }), lifetime)).toBe(false); expect(OwnedUiInstanceRetirement.matches(witness, owner, activation, nextLife)).toBe(false);
      let reads = 0; const forged = { isRetired() { reads++; return true; }, get owner() { reads++; throw new Error("Forged retirement access"); } }; expect(OwnedUiInstanceRetirement.matches(forged, owner, activation, lifetime)).toBe(false); expect(() => Reflect.construct(OwnedUiInstanceRetirement, [forged, owner, activation, lifetime])).toThrow(/authority/); expect(reads).toBe(0); close(foreign); expect(OwnedUiInstanceRetirement.matches(foreign.takeRetirementWitness(), owner, activation, lifetime)).toBe(false);
    });

    it("OwnedInstance cancels one-cell lookups and rejects forged facades at the native name boundary", async () => {
      const { OwnedUiInstance, OwnedUiInstanceSurface, OwnedUiSurfaceLookup } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️instance-owner.json");
      const activation = Object.freeze({ actorId: fixture.actor, activationGeneration: 9n, assertActive() {}, async turn() { return {}; } }); const lifetime = { activationGeneration: 9n, instanceId: 7, guestLifetime: 13n }; const instance = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const independent = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
      const surfaces: import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts").OwnedUiInstanceSurface[] = [];
      for (const name of [...fixture.surfaceNames, "😀".repeat(fixture.nativeSurfaceMaximumBytes / 4)]) { const lookup = instance.beginSurfaceLookup(activation, lifetime, name)!; expect(finish(lookup)).toBe("ready"); surfaces.push(lookup.takeResult()!); close(lookup); }
      expect(() => instance.beginSurfaceLookup(activation, lifetime, "😀".repeat(fixture.nativeSurfaceMaximumBytes / 4 + 1))).toThrow(/capacity/);
      let reads = 0; const forged = { get owner() { reads++; throw new Error("Forged owner access"); } }; expect(() => Reflect.construct(OwnedUiInstanceSurface, [forged])).toThrow(/authority/); expect(() => Reflect.construct(OwnedUiSurfaceLookup, [forged])).toThrow(/authority/); expect(reads).toBe(0);
      const pending = instance.beginSurfaceLookup(activation, lifetime, "unpublished")!; expect(pending.advance(grant)).toMatchObject({ kind: "pending", bytes: 2112 }); instance.beginClose(); expect(instance.closeStep({ maxItems: 1, maxBytes: 0 }).kind).toBe("blocked"); close(instance); expect(pending.terminalIsEmpty()).toBe(true); expect(pending.takeResult()).toBeNull(); expect(() => surfaces[0]!.view).toThrow(/retired/); close(independent);
    });

    it("OwnedInstance binds native patch source to issued ACK and retries the exact pair after revocation", async () => {
      const { OwnedUiInstance, OwnedUiPatchAcknowledgement, OwnedUiInstanceRetirement } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️wire-operations.json"); const { encodePackValue } = await import("@semio-tech/framework-os");
      const native = await nativeInstanceFixture(); const { activation, lifetime } = native; const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const foreign = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner);
      const lookup = owner.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const surface = lookup.takeResult()!; close(lookup); const bytes = encodePackValue(fields.node); const source = await native.source([{ tag: "upsert", val: { node: bytes } }, { tag: "set-root", val: BigInt(fields.root) }]);
      expect(() => foreign.beginPatch(source, surface)).toThrow(/owner/); const patch = owner.beginPatch(source, surface); expect(patch.peekAcknowledgement()).toBeNull(); let notifications = 0; const reader = surface.subscribeNode(fields.root, () => { notifications++; }); while (owner.maintenancePending) owner.advanceMaintenance(grant); const initial = reader.snapshot;
      for (let ordinal = 0; ordinal < source.value.operationCount; ordinal++) { expect(patch.offer(ordinal)).toBe(true); expect(finish(patch)).toBe("ready"); expect(patch.releaseInputReceipt(patch.peekInputReceipt()!)).toBe(true); expect(patch.peekAcknowledgement()).toBeNull(); }
      patch.finishInput(); expect(finish(patch)).toBe("ready"); expect(surface.view.revision).toBe(1); expect(reader.snapshot).not.toBe(initial); expect(notifications).toBeGreaterThan(1); const token = patch.peekAcknowledgement(); if (!token) throw new Error("Missing owned publication token"); expect(patch.peekAcknowledgement()).toBe(token); expect(OwnedUiPatchAcknowledgement.matches(token, source)).toBe(true); expect(OwnedUiPatchAcknowledgement.matches(token, {})).toBe(false); expect(token.value).toMatchObject({ actor: activation.actorId, instance: 7, surface: "window", revision: 1, lifetime }); expect(bytes.byteLength).toBe(0);
      expect(token.value.receipt).toEqual(source.value.receipt); expect(token.value.receipt.patchSequence).toBe(51n); expect(token.value.receipt.patchSequence).not.toBe(BigInt(token.value.revision)); expect(Object.isFrozen(token.value.receipt)).toBe(true); expect(Object.isFrozen(token.value.receipt.lifetime)).toBe(true);
      const closeRequest = native.lease.beginClose(); owner.beginClose(); expect(() => activation.assertActive()).toThrow(/revoked/); expect(owner.closeStep(grant).kind).toBe("blocked"); expect(owner.takeRetirementWitness()).toBeNull(); expect(patch.acceptAcknowledgement({ success: true })).toBe(false); native.worker.refuse = true; await expect(native.lease.submitUiAcknowledgement(source, token, native.budget)).rejects.toThrow(/refusal/); expect(patch.peekAcknowledgement()).toBe(token);
      await expect(native.answer(native.lease.submitUiAcknowledgement(source, token, native.budget), { status: { tag: "fault", val: "fixture" } })).rejects.toThrow(/not-admitted/); expect(patch.peekAcknowledgement()).toBe(token); const submitted = await native.answer(native.lease.submitUiAcknowledgement(source, token, native.budget), { status: { tag: "idle" } }); expect(patch.acceptAcknowledgement(submitted.receipt)).toBe(true); expect(patch.acceptAcknowledgement(submitted.receipt)).toBe(false); expect(patch.peekAcknowledgement()).toBeNull();
      const { encodeActorInstanceLifecycle } = await import("../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🟦️component.ts"); const accepted = { kind: "accepted" as const, lifetime, requestSequence: closeRequest.requestSequence, closeGeneration: 17n }; const retired = { ...accepted, kind: "retired" as const };
      await native.answer(native.lease.close(native.budget), { status: { tag: "idle" }, lifecycleReceipt: encodeActorInstanceLifecycle(accepted) }); await native.answer(native.lease.acknowledge(accepted, native.budget), { status: { tag: "idle" }, lifecycleReceipt: encodeActorInstanceLifecycle(retired) }); close(foreign); const foreignWitness = foreign.takeRetirementWitness()!; await expect(native.lease.acknowledge(retired, native.budget, foreignWitness)).rejects.toThrow(/host-retirement-pending/); expect(owner.takeRetirementWitness()).toBeNull();
      surface.unsubscribeNode(reader); close(owner); const witness = owner.takeRetirementWitness()!; expect(OwnedUiInstanceRetirement.matches(witness, owner, activation, lifetime)).toBe(true); native.worker.refuse = true; await expect(native.lease.acknowledge(retired, native.budget, witness)).rejects.toThrow(/refusal/); expect(native.lease.pendingReceipt).toEqual(retired); await native.answer(native.lease.acknowledge(retired, native.budget, witness), { status: { tag: "idle" } }); expect(native.lease.progress().kind).toBe("complete"); native.client.dispose("native-ui"); native.client.disposeAll();
    });

    it("OwnedInstance refuses the same native source on an independently owned same-lifetime surface", async () => {
      const { OwnedUiInstance } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts"); const native = await nativeInstanceFixture(); const { activation, lifetime } = native;
      const owner = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const foreign = new OwnedUiInstance(activation, lifetime, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); native.lease.bindHostRetirement(owner); const source = await native.source([]); const lookup = foreign.beginSurfaceLookup(activation, lifetime, "window")!; expect(finish(lookup)).toBe("ready"); const facade = lookup.takeResult()!; close(lookup);
      expect(() => foreign.beginPatch(source, facade)).toThrow(/owner/); expect(facade.view.revision).toBe(0); close(foreign); close(owner); native.client.disposeAll();
    });

    it("OwnedSurfaceMint refuses forged subscription detachment and private patch source access", async () => {
      const { OwnedUiSurface, OwnedUiSurfacePatch } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts");
      const surface = new OwnedUiSurface({ actor: "mint", instance: 1, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const real = surface.subscribeNode(7, () => {}); while (surface.maintenancePending) surface.advanceMaintenance(grant); const token = real.snapshot;
      const prototype: unknown = Object.getPrototypeOf(real); if (!prototype || typeof prototype !== "object" || !("constructor" in prototype) || typeof prototype.constructor !== "function") throw new Error("Missing reflected constructor");
      let rejected = false;
      try { const forged: typeof real = Reflect.construct(prototype.constructor, [{ owner: surface, active: true, previous: null, next: null, failureQueued: false, queued: false, queueNext: null, lease: null, sceneClose: [null,null,null,null], sceneActive: [null,null,null,null] }]); surface.unsubscribeNode(forged); } catch { rejected = true; }
      surface.beginClose(); expect(surface.closeStep(grant)).toMatchObject({ kind: "blocked", phase: "surface-readers" }); expect(rejected).toBe(true); expect(real.snapshot).toBe(token);
      let reads = 0; const source = { get nodes() { reads++; throw new Error("Forged source access"); } };
      expect(() => Reflect.construct(OwnedUiSurfacePatch, [surface, source, 1, DEFAULT_UI_DOCUMENT_LIMITS])).toThrow(/authority/); expect(reads).toBe(0);
      surface.unsubscribeNode(real); close(surface);
    });

    it("OwnedSurface publishes a validated hashed flat root before ACK and retires exact subscription owners", async () => {
      const { OwnedUiSurface } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️component.ts");
      const { OwnedUiOperation } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/🟦️component.ts");
      const { RetainedUiTypedCursor } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts");
      const { default: fixture } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-surface.json");
      const { default: schema } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-surface.schema.json");
      const { default: fields } = await import("../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧪️fixtures/🔣️fields.json");
      const { encodePackValue } = await import("@semio-tech/framework-os");
      expect(new Ajv({ strict: true, allErrors: true }).compile(schema)(fixture)).toBe(true);
      const surface = new OwnedUiSurface(fixture.identity, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const independent = new OwnedUiSurface(fixture.identity, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
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
      const surface = new OwnedUiSurface({ actor: "DOM", instance: 7, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
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
      const surface = new OwnedUiSurface({ actor: "notices", instance: 1, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 }); const events: string[] = [];
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
        const surface = new OwnedUiSurface({ actor: "quota", instance: 1, surface: "window" }, { ...DEFAULT_UI_DOCUMENT_LIMITS, [limit]: 0 }, { usizeBits: 64 }); const previous = surface.view;
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
        const surface = new OwnedUiSurface({ actor: "prefix", instance: 1, surface: "window" }, DEFAULT_UI_DOCUMENT_LIMITS, { usizeBits: 64 });
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
      expect(phases).toEqual(new Set(["input", "validation", "scenes", "hash", "staging", "notifications", "accepted"]));
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
