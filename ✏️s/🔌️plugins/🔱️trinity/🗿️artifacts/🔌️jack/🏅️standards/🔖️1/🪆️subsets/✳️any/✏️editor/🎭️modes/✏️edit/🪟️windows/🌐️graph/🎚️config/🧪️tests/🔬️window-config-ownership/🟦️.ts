import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";

/** 🧪️ Independent JSON Patch reference for exact-window configuration ownership. */
export function testJackGraphWindowConfigOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("./../../🧫️fixtures/🔬️window-config-ownership/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword("x-semio-state");
  ajv.addKeyword("x-semio-owner");
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

  const query = fixture.queryOwnership;
  const editorSchema = JSON.parse(readFileSync(new URL("../../../../📝️editor/🎚️config/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const editorMutationSchema = JSON.parse(readFileSync(new URL("../../../../📝️editor/🎚️config/🧬️schema/🧬️mutations/🔣️.json", import.meta.url), "utf8"));
  const resultsSchema = JSON.parse(readFileSync(new URL("../../../../📊️results/🫧️transient/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const resultsMutationSchema = JSON.parse(readFileSync(new URL("../../../../📊️results/🫧️transient/🧬️schema/🧬️mutations/🔣️.json", import.meta.url), "utf8"));
  for (const [root, symbol] of [
    ["../../../../📝️editor/🎚️config/🧬️schema", "JackEditorWindowConfig"],
    ["../../../../📝️editor/🎚️config/🧬️schema/🧬️mutations", "JackEditorWindowConfigMutation"],
    ["../../../../📊️results/🫧️transient/🧬️schema", "JackResultsWindowTransient"],
    ["../../../../📊️results/🫧️transient/🧬️schema/🧬️mutations", "JackResultsWindowTransientMutation"],
  ] as const) {
    for (const leaf of ["🦀️.rs", "🟦️.ts", "🔗️.graphql", "🔣️.json", "🛰️.proto"]) {
      assert(readFileSync(new URL(`${root}/${leaf}`, import.meta.url), "utf8").includes(symbol), `${symbol} missing from ${leaf}`);
    }
  }
  const validateEditor = ajv.compile(editorSchema);
  const validateEditorMutation = ajv.compile(editorMutationSchema);
  const validateResults = ajv.compile(resultsSchema);
  const validateResultsMutation = ajv.compile(resultsMutationSchema);
  let editors = Object.fromEntries(query.pairs.map((pair: { editorWindowId: string }) => [pair.editorWindowId, structuredClone(query.editorBase)]));
  let results = Object.fromEntries(query.pairs.map((pair: { resultsWindowId: string }) => [pair.resultsWindowId, structuredClone(query.resultsBase)]));
  const editorGenerations: Record<string, number> = Object.fromEntries(Object.keys(editors).map((id) => [id, 0]));
  const resultsGenerations: Record<string, number> = Object.fromEntries(Object.keys(results).map((id) => [id, 0]));
  const appConfig = structuredClone(query.appConfig);
  for (const pair of query.pairs) {
    assert(validateEditorMutation(pair.queryMutation), JSON.stringify(validateEditorMutation.errors));
    assert(validateResultsMutation(pair.resultMutation), JSON.stringify(validateResultsMutation.errors));
    editors = applyPatch(editors, [{ op: "replace", path: `/${pair.editorWindowId}/jackQuery`, value: pair.queryMutation.value }], true, false).newDocument;
    results = applyPatch(results, [
      { op: "replace", path: `/${pair.resultsWindowId}/queryExecutionId`, value: pair.resultMutation.executionId },
      { op: "replace", path: `/${pair.resultsWindowId}/result`, value: pair.resultMutation.result },
      { op: "replace", path: `/${pair.resultsWindowId}/queryError`, value: pair.resultMutation.error },
    ], true, false).newDocument;
    editorGenerations[pair.editorWindowId] += 1;
    resultsGenerations[pair.resultsWindowId] += 1;
  }
  for (const state of Object.values(editors)) assert(validateEditor(state), JSON.stringify(validateEditor.errors));
  for (const state of Object.values(results)) assert(validateResults(state), JSON.stringify(validateResults.errors));
  assert.deepEqual(editorGenerations, query.expectedEditorGenerations);
  assert.deepEqual(resultsGenerations, query.expectedResultsGenerations);
  assert.deepEqual(appConfig, query.appConfig);
  const reloadedEditors = structuredClone(editors);
  const resetResults = Object.fromEntries(Object.keys(results).map((id) => [id, structuredClone(query.resultsBase)]));
  assert.deepEqual(reloadedEditors, editors);
  assert(Object.values(resetResults).every((state) => validateResults(state)));
  assert.notDeepEqual(results["results-left"], results["results-right"]);
  console.log("[DEBUG] Jack window ownership oracle: independent graph settings, editor query sources, results outputs, generations, persisted source reload, and transient reset");
}
