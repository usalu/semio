/** 🎚️ Semantic chart customization is an event producing a guarded diff. */
import type { VizChartSnapshot, VizChartValue } from "../📸️snapshot/🟦️.ts";
import { applyVizChartDiff, readVizChartPath, equalVizChartValue, validVizChartPath, type VizChartDiff, type VizChartMutationMessage } from "../🔀️diff/🟦️.ts";
export type ChangeVizChartValue = { readonly path: readonly string[]; readonly value?: VizChartValue };
export type VizChartMutationOutcome = { readonly diff: VizChartDiff; readonly messages: readonly VizChartMutationMessage[] };

export function changeVizChartValue(base: VizChartSnapshot, mutation: ChangeVizChartValue): VizChartMutationOutcome {
  if (!validVizChartPath(mutation.path)) return { diff: { edits: [] }, messages: [{ code: "mutation.invariant", message: "invalid chart address", target: mutation.path }] };
  const before = readVizChartPath(base.chart, mutation.path);
  const diff: VizChartDiff = { edits: [{ path: mutation.path, ...(before === undefined ? {} : { before }), ...(mutation.value === undefined ? {} : { after: mutation.value }) }] };
  const result = applyVizChartDiff(base, diff);
  if (result.messages.length > 0) return { diff: { edits: [] }, messages: result.messages };
  return { diff: { edits: equalVizChartValue(before, mutation.value) ? [] : diff.edits }, messages: [] };
}
export function inverseChangeVizChartValue(base: VizChartSnapshot, mutation: ChangeVizChartValue): readonly ChangeVizChartValue[] {
  if (changeVizChartValue(base, mutation).diff.edits.length === 0) return [];
  const parentPath = mutation.path.slice(0, -1);
  const parent = readVizChartPath(base.chart, parentPath);
  const index = Number(mutation.path.at(-1));
  if (mutation.value === undefined && Array.isArray(parent) && index < parent.length) {
    return parent.slice(index).map((item, offset) => ({ path: [...parentPath, String(index + offset)], value: item as VizChartValue }));
  }
  const value = readVizChartPath(base.chart, mutation.path);
  return [{ path: mutation.path, ...(value === undefined ? {} : { value }) }];
}
