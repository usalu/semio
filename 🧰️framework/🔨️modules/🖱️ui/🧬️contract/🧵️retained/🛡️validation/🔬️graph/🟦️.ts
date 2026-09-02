//#region 🧬️GraphContract
import type { Component, UiContractViolation, UiDocumentLimits, UiNodeRecord } from "../../../../../🛂️manifest/🟦️.ts";
import type { RetainedUiComponent, RetainedUiNodeRecord } from "../../📦️wire/🧾️typed/🟦️.ts";
import type { RetainedUiNumericTable, RetainedUiSiblingKeys } from "../../🟦️.ts";

type Program<T> = Generator<number, T, void>;
type Record = UiNodeRecord | RetainedUiNodeRecord;
type Walk = { readonly kind: "enter" | "exit"; readonly id: number; readonly depth: number; readonly section: boolean };
type Link = { value: Walk | null; next: Link | null };
export type RetainedUiGraphFrontier = { stack: Link | null; count: number };
export type RetainedUiGraphNodes = { readonly size: number; lookup(id: number): Program<Record | undefined>; entries(): Generator<number | readonly [number, Record], void, void> };

function finite(component: Component | RetainedUiComponent): boolean {
  switch (component.type) {
    case "slider": return Number.isFinite(component.value) && Number.isFinite(component.min) && Number.isFinite(component.max) && Number.isFinite(component.step);
    case "numberStepper": return Number.isFinite(component.value) && Number.isFinite(component.step);
    case "ring": return Number.isFinite(component.t);
    case "input": return (component.min == null || Number.isFinite(component.min)) && (component.max == null || Number.isFinite(component.max)) && (component.step == null || Number.isFinite(component.step));
    default: return true;
  }
}

function* violation(value: UiContractViolation, frontier: RetainedUiGraphFrontier, violations: RetainedUiNumericTable<UiContractViolation>): Program<void> {
  if (frontier.count === Number.MAX_SAFE_INTEGER) throw new RangeError("Retained UI violation ordinal exhausted");
  yield* violations.set(frontier.count++, value);
}

export function closeRetainedUiGraphFrame(frontier: RetainedUiGraphFrontier): boolean {
  const cell = frontier.stack;
  if (!cell) return false;
  frontier.stack = cell.next; cell.next = null; cell.value = null;
  return true;
}
//#endregion 🧬️GraphContract

//#region 🚶️GraphTraversal
/** 🛡️ Shared explicit graph frontier preserves the native depth-first violation order. */
export function* retainedUiGraphValidation(nodes: RetainedUiGraphNodes, root: number | null, limits: UiDocumentLimits, marks: RetainedUiNumericTable<number>, keys: RetainedUiSiblingKeys, violations: RetainedUiNumericTable<UiContractViolation>, frontier: RetainedUiGraphFrontier): Program<void> {
  if (nodes.size > limits.maxNodes) { yield* violation({ type: "nodeQuota", count: nodes.size, max: limits.maxNodes }, frontier, violations); return; }
  if (root !== null && (yield* nodes.lookup(root))) frontier.stack = { value: { kind: "enter", id: root, depth: 0, section: false }, next: null };
  while (frontier.stack) {
    const cell: Link = frontier.stack; frontier.stack = cell.next; cell.next = null;
    const frame = cell.value!; cell.value = null; yield 48;
    const flags = (yield* marks.lookup(frame.id)) ?? 0;
    if (frame.kind === "exit") { yield* marks.set(frame.id, flags & ~2); continue; }
    if (flags & 2) { yield* violation({ type: "cycle", node: frame.id }, frontier, violations); continue; }
    if (flags & 1) continue;
    yield* marks.set(frame.id, 1);
    const record = yield* nodes.lookup(frame.id);
    if (!record) continue;
    const section = record.component.type === "container" && record.component.role === "section";
    if (frame.section && section) yield* violation({ type: "sectionNested", node: frame.id }, frontier, violations);
    if (!finite(record.component)) yield* violation({ type: "nonFiniteNumber", node: frame.id }, frontier, violations);
    if (frame.depth > limits.maxDepth) { yield* violation({ type: "depthQuota", node: frame.id, depth: frame.depth, max: limits.maxDepth }, frontier, violations); continue; }
    yield* marks.set(frame.id, 3);
    frontier.stack = { value: { ...frame, kind: "exit" }, next: frontier.stack }; yield 48;
    for (const childId of record.children ?? []) {
      const child = yield* nodes.lookup(childId);
      if (!child) { yield* violation({ type: "orphanChild", parent: frame.id, child: childId }, frontier, violations); continue; }
      if (yield* keys.insert(child.key)) yield* violation({ type: "duplicateSiblingKey", parent: frame.id, key: child.key }, frontier, violations);
      frontier.stack = { value: { kind: "enter", id: childId, depth: frame.depth + 1, section: frame.section || section }, next: frontier.stack }; yield 64;
    }
    yield* keys.clear();
  }
  const entries = nodes.entries();
  for (;;) {
    const step = entries.next(); if (step.done) break;
    if (typeof step.value === "number") { yield step.value; continue; }
    const id = step.value[0]; yield 64;
    if (!((yield* marks.lookup(id)) ?? 0)) yield* violation({ type: "danglingRoot", node: id }, frontier, violations);
  }
}
//#endregion 🚶️GraphTraversal
