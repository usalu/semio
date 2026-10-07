import type { VizChartTextOutput } from "../../../../🚪️io/📝️text/💡️inferences/🟦️.ts";
import {vizChartTextOutputToJsonValue} from "../../../../🚪️io/📝️text/💡️inferences/🟦️.ts";
/** 🧵️ Owned chart inference worker; the canonical pure planners also serve numerical probes. */
import type { VizChartSnapshot } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import type {  VizChartInferenceProgress } from "../../../../🧬️schema/💡️inferences/🟦️.ts";
import { planVizChart, renderVizScenePlan } from "../../../../🧬️schema/💡️inferences/🖼️render/🟦️.ts";
import { renderVizTikzPlan } from "../../../../🚪️io/📝️text/💡️inferences/🖋️latex/🟦️.ts";
import { admitVizChartSpecification } from "../../../../🧬️schema/💡️inferences/✅️validation/🟦️.ts";
import { validateVizChartTextOutput } from "../../../../🚪️io/📝️text/💡️inferences/🟦️.ts";
import { renderVizPresetTikz } from "../../../../🚪️io/📝️text/💡️inferences/📚️catalogue/🟦️.ts";
type Message = { readonly kind: "progress"; readonly progress: VizChartInferenceProgress } | { readonly kind: "result"; readonly result: VizChartTextOutput };
type Endpoint = { postMessage(value: Message): void; onmessage: ((event: { readonly data: VizChartSnapshot }) => void) | null };

function infer(snapshot: VizChartSnapshot, post: (message: Message) => void): void {
  try {
    const spec = snapshot.chart;
    const total = (Array.isArray(spec?.tables) ? spec.tables.reduce((sum, table) => sum + (Array.isArray(table?.rows) ? table.rows.length : 0), 0) : 0) + (Array.isArray(spec?.layers) ? spec.layers.length : 0) + (Array.isArray(spec?.guides) ? spec.guides.length : 0) + (Array.isArray(spec?.presets) ? spec.presets.length : 0) + 4;
    let completed = -1;
    const progress = (value: number) => { const next = Math.max(completed, Math.min(total, value)); if (next === completed) return; completed = next; post({ kind: "progress", progress: { completed, total } }); };
    progress(0);
    const admission = admitVizChartSpecification(spec);
    if (admission.chart === undefined) { post({ kind: "result", result: { tikz: "", diagnostics: admission.diagnostics, complete: false } }); return; }
    progress(1);
    const chart = { ...admission.chart, presets: [], ...((spec.presets?.length ?? 0) > 0 ? { title: undefined } : {}) };
    const plan = planVizChart(chart, { onProgress: (done, layers) => progress(1 + Math.floor(done / Math.max(1, layers) * (total - 4))) });
    progress(total - 2);
    const tikz = renderVizPresetTikz(admission.chart, renderVizTikzPlan(plan), () => progress(total - 2));
    progress(total - 1);
    const scene = (spec.presets?.length ?? 0) > 0 ? undefined : renderVizScenePlan(plan);
    const result: VizChartTextOutput = (spec.presets?.length ?? 0) > 0
      ? { tikz, diagnostics: [{ code: "print.chart.scene-unavailable", path: "chart/presets", message: "Catalogue geometry is inferred by the LaTeX implementation; a numerical scene is unavailable for this figure." }], complete: true }
      : { plan, tikz, scene, diagnostics: [], complete: true };
    const output = vizChartTextOutputToJsonValue(result);
    const errors = validateVizChartTextOutput(output);
    if (errors.length > 0) { post({ kind: "result", result: { tikz: "", diagnostics: errors, complete: false } }); return; }
    progress(total);
    post({ kind: "result", result: output });
  } catch (error) { post({ kind: "result", result: { tikz: "", diagnostics: [{ code: "print.chart.inference", path: "chart", message: error instanceof Error ? error.message : String(error) }], complete: false } }); }
}

const endpoint = globalThis as unknown as Endpoint;
if (typeof endpoint.postMessage === "function") endpoint.onmessage = event => infer(event.data, message => endpoint.postMessage(message));
else {
  const module = "node:worker_threads";
  const { parentPort } = await import(module) as typeof import("node:worker_threads");
  if (parentPort === null) throw new Error("chart inference worker requires an owned parent");
  parentPort.on("message", (snapshot: VizChartSnapshot) => infer(snapshot, message => parentPort.postMessage(message)));
}
