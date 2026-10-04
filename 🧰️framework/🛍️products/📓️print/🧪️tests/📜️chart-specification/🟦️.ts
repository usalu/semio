/** 📜️ Neutral chart admission vectors checked against the independent JSON Schema validator. */
import Ajv from "ajv/dist/2020.js";
import { defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { validateVizChartSpecification } from "../../🧬️schema/💡️inferences/✅️validation/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import vectors from "./🧫️fixtures/🔣️.json";

const validate = new Ajv({ strict: false, allowUnionTypes: true }).compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/ChartSpecification" });
const subject = () => vectors.cases.map(({ chart, valid }) => {
  const admitted = validateVizChartSpecification(chart).length === 0;
  if (admitted !== valid) throw new Error(`chart admission disagrees with the neutral validity vector: ${JSON.stringify(chart)}`);
  return admitted;
});
const oracle = () => vectors.cases.map(({ chart }) => Boolean(validate(chart)));

/** ⚖️ The same neutral vectors are exercised by the inference package differential runner. */
export const chartSpecificationChecks = () => [{ module: "contract", name: "chart-specification", subject, oracle }];

export default defineTestAdapter({ implementation: "typescript", scenarios: { "chart-specification-contract": { subject: () => ({ projection: { admitted: subject() } }), oracle: () => ({ projection: { admitted: oracle() } }) } } });
