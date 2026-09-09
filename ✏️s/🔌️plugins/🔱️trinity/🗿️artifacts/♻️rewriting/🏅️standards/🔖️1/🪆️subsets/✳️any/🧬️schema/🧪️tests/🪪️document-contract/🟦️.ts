/** 🧪️ Rewriting document values agree with the shared value owner and independent JSON Schema validation. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import mapSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/📡️replication/🎮️mutation/🗂️map/🧬️schema/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import { parseRewritingDiff } from "../../🔺️diff/🟦️.ts";
import valueSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json" with { type: "json" };
import vectors from "./../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import { parseRewritingArtifact } from "../../🟦️.ts";
import { parseRewritingSnapshot } from "../../📸️snapshot/🟦️.ts";

/** 🪪️ Checks nested dynamic values, exact layout records and rejection of window-owned fields. */
export function testRewritingDocumentContractOracle(): void {
  const ajv = new Ajv({ strict: false, strictNumbers: true, allErrors: true, validateFormats: false });
  ajv.addSchema(valueSchema).addSchema(mapSchema).addSchema(artifactSchema);
  const base = { beforeFixtureJson: "{}", lhsJson: "{}", rhsJson: "{}", parameterBindings: {}, ruleLayout: { node: { x: 1, y: 2 } } };
  const invalidDynamic: Record<string, unknown> = { undefined, "not-a-number": NaN, "positive-infinity": Infinity, function: () => null };
  for (const [schema, parse] of [[artifactSchema, parseRewritingArtifact], [snapshotSchema, parseRewritingSnapshot]] as const) {
    const validate = ajv.compile(schema);
    for (const value of vectors.validBindings) {
      const input = { ...base, parameterBindings: { key: value } };
      assert.equal(validate(input), true, JSON.stringify(validate.errors));
      assert.deepEqual(parse(input), input);
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
    assert.deepEqual(parseRewritingDiff(input), input);
  }
  const invalidLayout = { ruleLayout: { entries: [{ key: "node", precondition: "any", operation: { kind: "set", value: null } }] } };
  assert.equal(validateDiff(invalidLayout), false);
  assert.throws(() => parseRewritingDiff(invalidLayout));
  console.log(`[DEBUG] Rewriting shared map diff matched ${paths.length} committed inputs and rejected a null layout-point payload`);
  console.log("[DEBUG] Rewriting document/snapshot accepted six shared JSON value kinds and rejected window fields, malformed layouts and non-JSON nested values");
}
