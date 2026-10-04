/** 📸️ Persisted print authoring state; derived drawings never enter the event log. */
import type { VizChartSpecification,VizLanguage,VizCoordinateKind,VizOptionValue } from "./📊️chart/🟦️.ts";
/** 🌱️ Authored persistence permits an unselected language; inference still requires a complete specification. */
export type VizAuthoredChartSpecification = Omit<VizChartSpecification,"language"|"coordinate"> & {readonly language?:VizLanguage;readonly coordinate?:{readonly kind:VizCoordinateKind;readonly options?:Readonly<Record<string,VizOptionValue|readonly VizOptionValue[]>>}};
export type VizChartSnapshot = { readonly chart: VizAuthoredChartSpecification };
export type VizChartValue = null | boolean | number | string | readonly VizChartValue[] | { readonly [key: string]: VizChartValue };

export {CHART_SQLITE_SCHEMA,chartToSqliteDatabase,chartFromSqliteDatabase} from "./🪶️sqlite/🟦️.ts";
