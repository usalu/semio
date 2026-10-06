/** ✅️ Chart validity is inferred with the framework's owned schema validator. */
import resultDocument from "../🔣️.json";
import document from "../../🔣️.json";
import type { VizAuthoredChartSpecification } from "../../📸️snapshot/🟦️.ts";
import type { VizChartSpecification } from "../../📸️snapshot/📊️chart/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
export type VizChartDiagnostic = { readonly code: string; readonly path: string; readonly message: string };
export function validateVizChartSpecification(chart: unknown): readonly VizChartDiagnostic[] {
  try { return validateJsonSchemaSubset(document.$defs.ChartSpecification, chart, document).map((message) => ({ code: "print.chart.schema", path: "chart", message })); }
  catch (error) { return [{ code: "print.chart.schema", path: "chart", message: error instanceof Error ? error.message : String(error) }]; }
}

/** 🚦️ Admits authored input for inference without selecting a language or changing persisted state. */
export function admitVizChartSpecification(chart: VizAuthoredChartSpecification): { readonly chart?: undefined; readonly diagnostics: readonly VizChartDiagnostic[] } | { readonly chart: VizChartSpecification; readonly diagnostics: readonly VizChartDiagnostic[] } {
  const diagnostics = validateVizChartSpecification(chart);
  if (diagnostics.length > 0) return { diagnostics };
  if (chart.language === undefined) return { diagnostics: [{ code: "print.chart.schema", path: "chart", message: "language is required for inference" }] };
  return { chart: { ...chart, language: chart.language }, diagnostics };
}

/** 🪪️ Admits canonical wire output against every owned inference result variant. */
export function validateVizChartInference(value: unknown): readonly VizChartDiagnostic[] {
  try { return validateJsonSchemaSubset(resultDocument, value).map(message => ({ code: "print.chart.inference-schema", path: "inference", message })); }
  catch (error) { return [{ code: "print.chart.inference-schema", path: "inference", message: error instanceof Error ? error.message : String(error) }]; }
}
