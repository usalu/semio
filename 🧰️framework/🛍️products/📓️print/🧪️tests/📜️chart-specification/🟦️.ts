/** 📜️ Neutral chart admission vectors checked against the independent JSON Schema validator. */
import Ajv from "ajv/dist/2020.js";
import { defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { admitVizChartSpecification, validateVizChartSpecification } from "../../🧬️schema/💡️inferences/✅️validation/🟦️.ts";
import type { VizAuthoredChartSpecification } from "../../🧬️schema/📸️snapshot/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import vectors from "./🧫️fixtures/🔣️.json";

const validate = new Ajv({ strict: false, allowUnionTypes: true }).compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/ChartSpecification" });
const subject = () => vectors.cases.map(({ chart, valid }) => {
  const admitted = validateVizChartSpecification(chart).length === 0;
  if (admitted !== valid) throw new Error(`chart admission disagrees with the neutral validity vector: ${JSON.stringify(chart)}`);
  return admitted;
});
const oracle = () => vectors.cases.map(({ chart }) => Boolean(validate(chart)));
const languageSubject = () => vectors.authoredLanguageCases.map(({ chart, valid }) => {
  const language = "language" in chart ? chart.language : undefined;
  if (language !== undefined && language !== "en" && language !== "de") throw new Error("invalid neutral language control");
  const authored: VizAuthoredChartSpecification = { ...chart, language };
  const before = JSON.stringify(authored);
  const admission = admitVizChartSpecification(authored);
  const admitted = admission.chart !== undefined;
  if (admitted !== valid || JSON.stringify(authored) !== before || (admission.chart !== undefined && (admission.chart === authored || JSON.stringify(admission.chart) !== before))) throw new Error("authored language admission changed persisted input or contradicted the neutral vector");
  return admitted;
});
const languageOracle = () => vectors.authoredLanguageCases.map(({ chart }) => Boolean(validate(chart)));

/** ⚖️ The same neutral vectors are exercised by the inference package differential runner. */
export const chartSpecificationChecks = () => [{ module: "contract", name: "chart-specification", subject, oracle }, { module: "contract", name: "authored-language-admission", subject: languageSubject, oracle: languageOracle }];

export default defineTestAdapter({ implementation: "typescript", scenarios: { "chart-specification-contract": { subject: () => ({ projection: { admitted: subject(), authoredLanguages: languageSubject() } }), oracle: () => ({ projection: { admitted: oracle(), authoredLanguages: languageOracle() } }) } } });
