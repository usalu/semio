/** 🧪️ Rewriting document values agree with the shared value owner and independent JSON Schema validation. */
import assert from "node:assert/strict";
import { semioSchemaAjvV1 } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import mapSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/📡️replication/🎮️mutation/🗂️map/🧬️schema/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import { parseRewritingDiff } from "../../🔺️diff/🟦️.ts";
import valueSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json" with { type: "json" };
import vectors from "../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import { parseRewritingArtifact } from "../../🟦️.ts";
import { parseRewritingSnapshot } from "../../📸️snapshot/🟦️.ts";
import propertySchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🌱️value/🧬️schema/🔣️.json" with {type:"json"};
import {parseRewritingJsonValue,parseRewritingDiffJsonValue} from "../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts";
import {rewritingToJsonValue} from "../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts";

/** 🪪️ Checks nested dynamic values, exact layout records and rejection of window-owned fields. */
export function testRewritingDocumentContractOracle(): void {
  const ajv = semioSchemaAjvV1({ allErrors: true });
  const specification = JSON.parse(readFileSync(new URL("../../../🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🌳️typed/🔣️.json", import.meta.url), "utf8"));
  const specificationOwner = new URL("../../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts", import.meta.url);
  for (const path of specification.schemaDocuments) ajv.addSchema(JSON.parse(readFileSync(new URL(path, specificationOwner), "utf8")));
  for (const schema of [valueSchema, mapSchema, propertySchema, artifactSchema]) if (!ajv.getSchema(schema.$id)) ajv.addSchema(schema);
  const base = { ...specification.snapshot, parameterBindings: {}, ruleLayout: { node: { x: {bits:"3ff0000000000000"}, y: {bits:"4000000000000000"} } } };
  const invalidDynamic: Record<string, unknown> = { undefined, "not-a-number": NaN, "positive-infinity": Infinity, function: () => null };
  for (const [schema, parse] of [[artifactSchema, parseRewritingArtifact], [snapshotSchema, parseRewritingSnapshot]] as const) {
    const validate = ajv.compile(schema);
    for (const value of vectors.validBindings) {
      const input = { ...base, parameterBindings: { key: value } };
      assert.equal(validate(input), true, JSON.stringify(validate.errors));
      const decoded=parseRewritingJsonValue(input);
      assert.deepEqual(rewritingToJsonValue(parse(decoded)), input);
    }
    for (const vector of vectors.invalidDocuments) {
      const input = { ...base, [vector.field]: vector.value };
      assert.equal(validate(input), false, `schema accepted invalid ${vector.field}`);
      assert.throws(() => parse(input), `parser accepted invalid ${vector.field}`);
    }
    for (const kind of vectors.invalidDynamic) {
      const input = { ...base, parameterBindings: { key: { nested: invalidDynamic[kind] } } };
      assert.equal(validate(input), false, `schema accepted ${kind}`);
      assert.throws(() => parse(input), `parser accepted ${kind}`);
    }
  }
  const validateDiff = ajv.compile(diffSchema);
  const mutations = join(import.meta.dir, "../../../🧫️fixtures/🧬️mutations");
  const paths = readdirSync(mutations, { recursive: true }).map((path) => String(path).replaceAll("\\", "/")).filter((path) => path.endsWith("/🔺️diff/🔣️.json"));
  assert.ok(paths.length > 0, "Rewriting document contract must validate committed mutation diffs");
  for (const path of paths) {
    const input = JSON.parse(readFileSync(join(mutations, path), "utf8"));
    assert.equal(validateDiff(input), true, JSON.stringify(validateDiff.errors));
    assert.deepEqual(parseRewritingDiffJsonValue(input),parseRewritingDiff(parseRewritingDiffJsonValue(input)));
  }
  const invalidLayout = { ruleLayout: { entries: [{ key: "node", precondition: "any", operation: { kind: "set", value: null } }] } };
  assert.equal(validateDiff(invalidLayout), false);
  assert.throws(() => parseRewritingDiff(invalidLayout));
}
