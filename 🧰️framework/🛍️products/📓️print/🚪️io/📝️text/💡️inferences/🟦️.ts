import type {VizChartInference} from "../../../🧬️schema/💡️inferences/🟦️.ts";

/** 🧾️ Projects one owned inference result into its physical JSON value. */
export function vizChartInferenceToJsonValue(value:VizChartInference):VizChartInference{return JSON.parse(JSON.stringify(value)) as VizChartInference;}
