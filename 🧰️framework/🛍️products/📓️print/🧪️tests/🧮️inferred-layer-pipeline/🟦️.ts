/** 🧮️ Language-neutral inferred layer fixtures executed by the print differential runner. */
import { inferVizLayerTable } from "../../🧬️schema/💡️inferences/🧮transform/🟦️.ts";
import type { VizChartSpecification, VizLayerSpec, VizRow, VizTable } from "../../🧬️schema/📸️snapshot/📊️chart/🟦️.ts";

export type InferredLayerScenario = { readonly name: string; readonly rows: readonly VizRow[]; readonly layer: VizLayerSpec };

/** 🧪️ Runs an authored scenario using the production-owned layer inference. */
export function inferLayerScenario(scenario: InferredLayerScenario): { table: VizTable; deterministic: boolean; immutable: boolean } {
  const spec: VizChartSpecification = { width: 100, height: 80, margin: { left: 0, right: 0, top: 0, bottom: 0 }, language: "en", tables: [{ name: "data", columns: [...new Set(scenario.rows.flatMap(Object.keys))], rows: scenario.rows }], layers: [scenario.layer] };
  const before = JSON.stringify(spec), table = inferVizLayerTable(spec, scenario.layer);
  return { table, deterministic: JSON.stringify(table) === JSON.stringify(inferVizLayerTable(spec, scenario.layer)), immutable: before === JSON.stringify(spec) };
}
