/** 🪪️ Selection vocabulary of the spatial kernel: picked targets, selection events and specs, accept expansion and object deletion from a selection. */
import type { Vec3 } from "@semio-tech/framework-3d-js";
import { Model, resolveKernelTopologyKind } from "../🧊️model/🟦️.ts";
import type { EditableEntityKind, ModelEntityKind, ObjectRef } from "../🟦️.ts";

// #region 🎮️InteractionEvent
/** 🧭️ Interaction input envelope; `kind` selects `machine.states[*].on` keys. */
export type InteractionEvent = { readonly kind: string; readonly [k: string]: unknown };
// #endregion 🎮️InteractionEvent

// #region 🪪️Selection
export const MODEL_ENTITY_KINDS = new Set<string>(["anchor", "vertex", "edge", "wire", "face", "shell", "solid", "object", "geometry", "attribute"]);

/** 🪪️ One picked geometry or derived view target for `selection.changed`. */
export interface SelectionTarget {
  readonly kind: ModelEntityKind;
  readonly id: string;
  readonly editable: boolean;
  readonly derivedFrom?: readonly { kind: EditableEntityKind; id: string }[];
}

/** 🪪️ Host selection payload; `targets` filtered by `SelectionSpec.accept`. */
export interface SelectionEvent extends InteractionEvent {
  readonly kind: "selection.changed";
  readonly targets: readonly SelectionTarget[];
  readonly point?: Vec3;
}

/** 🪪️ Per-state declarative filter for model vs extension-view picking. */
export interface SelectionSpec {
  readonly accept: readonly ModelEntityKind[];
  readonly multiple?: boolean;
  readonly prompt?: string;
}

/** 🧭️ Returns `targets` whose `kind` is listed in `spec.accept`. */
export function filterSelectionTargets(spec: SelectionSpec, targets: readonly SelectionTarget[]): SelectionTarget[] {
  return targets.filter((t) => spec.accept.includes(t.kind));
}

/** 🧭️ Maps object picks to wire/edge primitives when `spec.accept` lists curve geometry kinds. */
export function expandSelectionTargetsForAccept(model: Model, spec: SelectionSpec, targets: readonly SelectionTarget[]): SelectionTarget[] {
  const accept = new Set(spec.accept);
  const out: SelectionTarget[] = [];
  const seen = new Set<string>();
  const push = (kind: ModelEntityKind, id: string): void => {
    const key = `${kind}:${id}`;
    if (seen.has(key)) return;
    seen.add(key);
    out.push({ kind, id, editable: true });
  };
  for (const target of targets) {
    if (accept.has(target.kind)) {
      push(target.kind, target.id);
      continue;
    }
    if (target.kind !== "object") continue;
    const row = model.objects[target.id as ObjectRef];
    if (!row) continue;
    const curveRef = row.primitives.curve;
    if (!curveRef) continue;
    const topo = resolveKernelTopologyKind(model, String(curveRef));
    if (accept.has("wire") && (topo === "wire" || topo === null)) push("wire", String(curveRef));
    else if (accept.has("edge") && (topo === "edge" || topo === null)) push("edge", String(curveRef));
  }
  return out;
}

/** 🧭️ True when every target is accepted (and at least one target exists). */
export function selectionEventMatches(spec: SelectionSpec, ev: SelectionEvent): boolean {
  if (!ev.targets || ev.targets.length === 0) return false;
  const xs = filterSelectionTargets(spec, ev.targets);
  if (xs.length !== ev.targets.length) return false;
  if (!spec.multiple && xs.length > 1) return false;
  return true;
}

/** 🗑️ Removes object rows by id; geometry primitives stay intact so linked topology remains valid. */
export function deleteObjectsFromModel(model: Model, objectIds: readonly string[]): readonly string[] {
  const removed: string[] = [];
  for (const id of objectIds) {
    if (!model.objects[id]) continue;
    delete model.objects[id];
    removed.push(id);
  }
  if (removed.length > 0) model.bump();
  return removed;
}

/** 🪪️ Object ids from selection eligible for deletion (excludes geometry primitives). */
export function deletableObjectIdsFromSelection(selection: readonly SelectionTarget[]): readonly string[] {
  const ids: string[] = [];
  const seen = new Set<string>();
  for (const target of selection) {
    if (target.kind !== "object" || seen.has(target.id)) continue;
    seen.add(target.id);
    ids.push(target.id);
  }
  return ids;
}
//#endregion 🪪️Selection
