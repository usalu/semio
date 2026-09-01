//#region 🔌️Adapters
import Ajv from "ajv/dist/2020";
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { projectTaxonomySourceAdmission } from "../../🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
const vectors = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")) as {
  cases: readonly { id: string; input: unknown; expected: unknown }[];
  schemaRejections: readonly { id: string; candidate: unknown; expectedKeyword: string }[];
  schemaCases: readonly { id: string; subject: "candidate" | "sourceAdmissionInput" | "observation" | "sourceAdmission"; value: unknown; valid: boolean }[];
};
const validator = new Ajv({ strict: true, allErrors: true });
validator.addSchema(schema);
const validateCases = validator.getSchema(schema.$id + "#/$defs/sourceAdmissionCases")!;
const validateCandidate = validator.getSchema(schema.$id + "#/$defs/candidate")!;
const validateResult = validator.getSchema(schema.$id + "#/$defs/sourceAdmission")!;
//#endregion 🧬️Contract

//#region 🧪️Projection
describe("taxonomy source admission projection", () => {
  test("neutral records satisfy the independent schema implementation", () => {
    expect(validateCases(vectors), JSON.stringify(validateCases.errors)).toBe(true);
    const identities = [...vectors.cases, ...vectors.schemaRejections, ...vectors.schemaCases].map((row) => row.id);
    expect(new Set(identities).size).toBe(identities.length);
  });
  for (const row of vectors.cases) test(row.id, () => {
    const actual = projectTaxonomySourceAdmission(row.input);
    expect(actual).toEqual(row.expected);
    expect(validateResult(actual), JSON.stringify(validateResult.errors)).toBe(true);
  });
  for (const row of vectors.schemaRejections) test(row.id, () => {
    expect(validateCandidate(row.candidate)).toBe(false);
    expect(validateCandidate.errors?.some((error) => error.keyword === row.expectedKeyword)).toBe(true);
    expect(projectTaxonomySourceAdmission({ scope: null, opaquePrefixes: [], generatorOutputRoots: [], candidates: [row.candidate] }).status).toBe("rejected");
  });
  for (const row of vectors.schemaCases) test(row.id, () => {
    const validate = validator.getSchema(schema.$id + "#/$defs/" + row.subject)!;
    expect(validate(row.value), JSON.stringify(validate.errors)).toBe(row.valid);
  });
});
//#endregion 🧪️Projection
