/** 📊️ `@semio-tech/print-viz-inference` — the TypeScript twin of the `semio-viz` LaTeX kernel.
 * One barrel over the domain modules so the kernel is consumed from code the way d3 is, with no
 * runtime dependency on anything outside this repository.
 * @see ../../../../🖋️latex/semio-viz.sty
 */
export { default as inferencesSchema } from "../../🔣️.json";
export { inferVizChartSnapshot, validateVizChartSpecification } from "../../🟦️.ts";
export type { VizChartInference, VizChartInferenceControl, VizChartInferenceProgress } from "../../🟦️.ts";
export * from "@semio-tech/print/schema/snapshot/chart";
export * from "../../📐scale/🟦️.ts";
export * from "../../🔢format/🟦️.ts";
export * from "../../🧮transform/🟦️.ts";
export * from "../../✒️mark/🟦️.ts";
export * from "../../✒️mark/🪜️curves-linear-and-step/🟦️.ts";
export * from "../../✒️mark/🌀️curves-basis-and-bundle/🟦️.ts";
export * from "../../✒️mark/🎢️curves-cardinal-and-catmull-rom/🟦️.ts";
export * from "../../✒️mark/🏔️curves-monotone-natural-and-bump/🟦️.ts";
export * from "../../✒️mark/💠️curve-registry-and-symbols/🟦️.ts";
export * from "../../🥧shape/🟦️.ts";
export * from "../../🧭coordinate/🟦️.ts";
export * from "../../🌳hierarchy/🟦️.ts";
export * from "../../🌳hierarchy/🌲️tidy-tree-and-cluster/🟦️.ts";
export * from "../../🌳hierarchy/🧱️treemap-tilings-and-partition/🟦️.ts";
export * from "../../🌳hierarchy/🫧️circle-packing/🟦️.ts";
export * from "../../🕸️network/🟦️.ts";
export * from "../../🌊flow/🟦️.ts";
export * from "../../🌍geo/🟦️.ts";
export * from "../../📍spatial/🟦️.ts";
export * from "../../🎨theme/🟦️.ts";
export * from "../../🖼️render/🟦️.ts";
export * from "@semio-tech/print/schema/snapshot";
export * from "@semio-tech/print/schema/diff";
export * from "@semio-tech/print/schema/mutations";
export * from "../../🟦️.ts";
