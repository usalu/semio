/** 🔀️ Guarded chart event replay preserves preconditions and atomicity. */
import type { VizChartSnapshot, VizChartValue } from "../📸️snapshot/🟦️.ts";
import { validateVizChartSpecification } from "../💡️inferences/✅️validation/🟦️.ts";

export type VizChartEdit = { readonly path: readonly string[]; readonly before?: VizChartValue; readonly after?: VizChartValue };
export type VizChartDiff = { readonly edits: readonly VizChartEdit[] };
export type VizChartMutationMessage = { readonly code: string; readonly message: string; readonly target: readonly string[] };
export type VizChartApplyOutcome = { readonly snapshot: VizChartSnapshot; readonly messages: readonly VizChartMutationMessage[] };
const ROOTS = ["width", "height", "margin", "theme", "language", "tables", "scales", "coordinate", "layers", "guides", "title", "annotations", "presets"];

export function validVizChartPath(path: readonly string[]): boolean {
  return path.length > 0 && path.length <= 64 && ROOTS.includes(path[0]!) && path.every((part) => part.length > 0 && !["__proto__", "constructor", "prototype"].includes(part));
}
export function readVizChartPath(root: unknown, path: readonly string[]): VizChartValue | undefined {
  let value = root;
  for (const part of path) {
    if (value === null || typeof value !== "object" || !Object.hasOwn(value, part)) return undefined;
    if (Array.isArray(value) && String(Number(part)) !== part) return undefined;
    value = (value as Record<string, unknown>)[part];
  }
  return value as VizChartValue | undefined;
}
export function equalVizChartValue(left: unknown, right: unknown): boolean {
  if (left === right) return true;
  if (left === null || right === null || typeof left !== "object" || typeof right !== "object") return false;
  if (Array.isArray(left) !== Array.isArray(right)) return false;
  const a = Object.keys(left).sort(), b = Object.keys(right).sort();
  return a.length === b.length && a.every((key, index) => key === b[index] && equalVizChartValue((left as Record<string, unknown>)[key], (right as Record<string, unknown>)[key]));
}

export function applyVizChartDiff(base: VizChartSnapshot, diff: VizChartDiff): VizChartApplyOutcome {
  const snapshot = structuredClone(base);
  for (const edit of diff.edits) {
    const reject = (code: string, message: string): VizChartApplyOutcome => ({ snapshot: base, messages: [{ code, message, target: edit.path }] });
    if (!validVizChartPath(edit.path)) return reject("print.chart.path", "invalid chart address");
    const parent = readVizChartPath(snapshot.chart, edit.path.slice(0, -1));
    if (parent === null || typeof parent !== "object") return reject("print.chart.parent", "chart address parent is missing or scalar");
    const key = edit.path.at(-1)!;
    if (Array.isArray(parent) && (!/^(0|[1-9][0-9]*)$/.test(key) || Number(key) > parent.length)) return reject("print.chart.index", "array index must be canonical and in range");
    if (!equalVizChartValue(readVizChartPath(snapshot.chart, edit.path), edit.before)) return reject("print.chart.precondition", "chart field changed since diff was authored");
    if (Array.isArray(parent)) {
      const items = parent as VizChartValue[];
      if (edit.after === undefined) { if (Number(key) < items.length) items.splice(Number(key), 1); }
      else items[Number(key)] = structuredClone(edit.after);
    } else {
      const object = parent as Record<string, VizChartValue>;
      if (edit.after === undefined) delete object[key];
      else object[key] = structuredClone(edit.after);
    }
  }
  const diagnostics = diff.edits.length > 0 ? validateVizChartSpecification(snapshot.chart) : [];
  return diagnostics.length > 0 ? { snapshot: base, messages: diagnostics.map((diagnostic) => ({code:diagnostic.code,message:diagnostic.message,target:[]})) } : { snapshot, messages: [] };
}

export function absorbVizChartDiff(first: VizChartDiff, second: VizChartDiff): VizChartDiff { return { edits: [...first.edits, ...second.edits] }; }
export function betweenVizChartSnapshots(base: VizChartSnapshot, other: VizChartSnapshot): VizChartDiff {
  return { edits: [...new Set([...Object.keys(base.chart), ...Object.keys(other.chart)])].sort().flatMap((key) => {
    const before = readVizChartPath(base.chart, [key]), after = readVizChartPath(other.chart, [key]);
    return equalVizChartValue(before, after) ? [] : [{ path: [key], ...(before === undefined ? {} : { before }), ...(after === undefined ? {} : { after }) }];
  }) };
}
export function inverseVizChartDiff(base: VizChartSnapshot, diff: VizChartDiff): VizChartDiff {
  const outcome = applyVizChartDiff(base, diff);
  return outcome.messages.length === 0 ? betweenVizChartSnapshots(outcome.snapshot, base) : { edits: [] };
}
export function vizChartDiffTouches(diff: VizChartDiff): readonly string[] { return diff.edits.map((edit) => `chart/${edit.path.join("/")}`); }
