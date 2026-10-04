/** 📸️ Persisted print authoring state; derived drawings never enter the event log. */
import type { VizChartSpecification } from "./📊️chart/🟦️.ts";
export type VizChartSnapshot = { readonly chart: VizChartSpecification };
export type VizChartValue = null | boolean | number | string | readonly VizChartValue[] | { readonly [key: string]: VizChartValue };
