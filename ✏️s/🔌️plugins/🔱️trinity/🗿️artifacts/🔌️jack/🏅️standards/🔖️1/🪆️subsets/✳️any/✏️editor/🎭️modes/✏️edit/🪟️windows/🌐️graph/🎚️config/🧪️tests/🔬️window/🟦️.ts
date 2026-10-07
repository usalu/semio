import artifactReferenceSchema from "../../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json";
import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { addSemioMutationLeafSchemasV1, semioSchemaAjvV1 } from "../../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import { applyPatch } from "fast-json-patch";

/** 🧪️ Independent JSON Patch reference for exact-window configuration ownership. */
export function testJackGraphWindowConfigOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔬️window/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const ajv = semioSchemaAjvV1({ allErrors: true }).addSchema(artifactReferenceSchema);
  addSemioMutationLeafSchemasV1(ajv, new URL("../../🧬️schema/🧬️mutations", import.meta.url));
  addSemioMutationLeafSchemasV1(ajv, new URL("../../../../📊️results/🫧️transient/🧬️schema/🧬️mutations", import.meta.url));
  for (const document of ["../../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json", "../../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json", "../../../../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json", "../../../../../../../../🧬️schema/📸️snapshot/🔣️.json", "../../../../../../../../🧬️schema/🔣️.json"]) ajv.addSchema(JSON.parse(readFileSync(new URL(document, import.meta.url), "utf8")));
  const validate = ajv.compile(schema);
  const validateMutation = ajv.compile(JSON.parse(readFileSync(new URL("../../🧬️schema/🧬️mutations/🔣️.json", import.meta.url), "utf8")));
  let windows = Object.fromEntries([fixture.leftWindowId, fixture.rightWindowId].map((id: string) => [id, structuredClone(fixture.base)]));
  const generations: Record<string, number> = Object.fromEntries(Object.keys(windows).map((id) => [id, 0]));
  for (const row of fixture.cases) {
    assert(validateMutation(row.mutation), JSON.stringify(validateMutation.errors));
    const field = row.mutation.kind === "set-camera" ? "camera" : "lodMode";
    const value = row.mutation.kind === "set-camera" ? row.mutation.camera : row.mutation.value;
    windows = applyPatch(windows, [{ op: "replace", path: `/${row.windowId}/${field}`, value }], true, false).newDocument;
    generations[row.windowId] += 1;
    assert.deepEqual(windows, row.expected);
    for (const state of Object.values(windows)) assert(validate(state), JSON.stringify(validate.errors));
  }
  assert.deepEqual(generations, fixture.expectedWindowGenerations);

  const query = fixture.resultsOwnership;
  const documentSchema = JSON.parse(readFileSync(new URL("../../../../../../../../🧬️schema/📸️snapshot/🔣️.json", import.meta.url), "utf8"));
  assert(documentSchema.required.includes("query") && documentSchema.properties.query["x-semio-state"] === "artifact", "the Jack query is document content");
  assert(!existsSync(new URL("../../../../📝️editor/🎚️config", import.meta.url)), "the query editor window carries no config lane of its own");
  const resultsSchema = JSON.parse(readFileSync(new URL("../../../../📊️results/🫧️transient/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const resultsMutationSchema = JSON.parse(readFileSync(new URL("../../../../📊️results/🫧️transient/🧬️schema/🧬️mutations/🔣️.json", import.meta.url), "utf8"));
  for (const [root, symbol] of [
    ["../../../../📊️results/🫧️transient/🧬️schema", "JackResultsWindowTransient"],
    ["../../../../📊️results/🫧️transient/🧬️schema/🧬️mutations", "JackResultsWindowTransientMutation"],
  ] as const) {
    for (const leaf of ["🦀️.rs", "🟦️.ts", "🔗️.graphql", "🔣️.json", "🛰️.proto"]) {
      assert(readFileSync(new URL(`${root}/${leaf}`, import.meta.url), "utf8").includes(symbol), `${symbol} missing from ${leaf}`);
    }
  }
  const validateResults = ajv.compile(resultsSchema);
  const validateResultsMutation = ajv.compile(resultsMutationSchema);
  let results = Object.fromEntries(query.pairs.map((pair: { resultsWindowId: string }) => [pair.resultsWindowId, structuredClone(query.resultsBase)]));
  const resultsGenerations: Record<string, number> = Object.fromEntries(Object.keys(results).map((id) => [id, 0]));
  const appConfig = structuredClone(query.appConfig);
  for (const pair of query.pairs) {
    assert(validateResultsMutation(pair.resultMutation), JSON.stringify(validateResultsMutation.errors));
    results = applyPatch(results, [
      { op: "replace", path: `/${pair.resultsWindowId}/queryExecutionId`, value: pair.resultMutation.executionId },
      { op: "replace", path: `/${pair.resultsWindowId}/result`, value: pair.resultMutation.result },
      { op: "replace", path: `/${pair.resultsWindowId}/queryError`, value: pair.resultMutation.error },
    ], true, false).newDocument;
    resultsGenerations[pair.resultsWindowId] += 1;
  }
  for (const state of Object.values(results)) assert(validateResults(state), JSON.stringify(validateResults.errors));
  assert.deepEqual(resultsGenerations, query.expectedResultsGenerations);
  assert.deepEqual(appConfig, query.appConfig);
  const resetResults = Object.fromEntries(Object.keys(results).map((id) => [id, structuredClone(query.resultsBase)]));
  assert(Object.values(resetResults).every((state) => validateResults(state)));
  assert.notDeepEqual(results["results-left"], results["results-right"]);
}
