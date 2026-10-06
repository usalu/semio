/** 💡️ Semantic chart inference results, work controls and field dependencies. */
import type { VizChartSnapshot } from "../📸️snapshot/🟦️.ts";
import type { VizRenderPlan, renderVizScenePlan } from "./🖼️render/🟦️.ts";
import type { VizChartDiagnostic } from "./✅️validation/🟦️.ts";
export { validateVizChartSpecification } from "./✅️validation/🟦️.ts";
export type VizChartInference = { readonly plan?: VizRenderPlan; readonly tikz: string; readonly scene?: ReturnType<typeof renderVizScenePlan>; readonly diagnostics: readonly VizChartDiagnostic[]; readonly complete: true } | { readonly plan?: never; readonly tikz: ""; readonly scene?: never; readonly diagnostics: readonly VizChartDiagnostic[]; readonly complete: false };
export type VizChartInferenceProgress = { readonly completed: number; readonly total: number };
export type VizChartInferenceControl = { readonly signal?: AbortSignal; readonly onProgress?: (progress: VizChartInferenceProgress) => void };
export const VIZ_CHART_INFERENCE_FIELDS = [{ id: "framework.print.chart.inference.plan", reads: ["chart"] }, { id: "framework.print.chart.inference.tikz", reads: ["chart"] }, { id: "framework.print.chart.inference.scene", reads: ["chart"] }] as const;
