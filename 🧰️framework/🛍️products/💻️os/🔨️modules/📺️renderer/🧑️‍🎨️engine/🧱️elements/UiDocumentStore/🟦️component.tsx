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
import {
  type AccessibilitySpec,
  type ActionBinding,
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
