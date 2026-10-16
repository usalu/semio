/** 🪆️ The authored Semio child schema agrees with the framework-generated declared-dialect stand-in that consumer plugins test against. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import schema from "../🔣️.json";
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json";
import { artifactReferenceSchema as referenceSchema } from "@semio-tech/framework/schema/artifact-reference";
import { declaredChildDialectSchema } from "@semio-tech/framework/schema/oracles/document";
import { parseSemioChild } from "../🟦️.ts";

const subsets = Object.keys(schema.$defs);
const dialect = (subset: string) => ({ artifactKind: "s.stdio.semio", standard: "v1", subset });
const stand = declaredChildDialectSchema(schema.$id, childSchema.$id, Object.fromEntries(subsets.map((subset) => [subset, dialect(subset)]))) as { $defs: Record<string, unknown> };

test("the generated declared-dialect definitions equal the authored Semio child definitions", () => {
  expect(subsets.length).toBeGreaterThan(0);
  expect(stand.$defs).toEqual(schema.$defs);
});

for (const subset of subsets) test(`the parser and the independent Ajv oracle agree on the ${subset} child`, () => {
  const ajv = new Ajv({ strict: false }).addSchema(referenceSchema).addSchema(childSchema).addSchema(schema);
  const validate = ajv.compile({ $ref: `${schema.$id}#/$defs/${subset}` });
  const accepted = { childId: "c", target: { artifactId: "a", dialect: dialect(subset) } };
  const wrongSubset = { childId: "c", target: { artifactId: "a", dialect: dialect(`${subset}-other`) } };
  const wrongKind = { childId: "c", target: { artifactId: "a", dialect: { ...dialect(subset), artifactKind: "s.other.semio" } } };
  expect(validate(accepted)).toBe(true);
  expect(parseSemioChild(accepted, subset)).toEqual(accepted);
  for (const refused of [wrongSubset, wrongKind]) {
    expect(validate(refused)).toBe(false);
    expect(() => parseSemioChild(refused, subset)).toThrow("artifact dialect mismatch");
  }
});
