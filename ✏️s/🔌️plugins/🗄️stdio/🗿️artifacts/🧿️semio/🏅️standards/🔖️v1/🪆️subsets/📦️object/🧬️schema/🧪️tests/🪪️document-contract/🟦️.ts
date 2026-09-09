import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import fg from "fast-glob";
import { applyPatch } from "fast-json-patch";
import * as artifact from "../../🟦️.ts";
import * as snapshot from "../../📸️snapshot/🟦️.ts";
import * as diff from "../../🔺️diff/🟦️.ts";
import { testSchemaRecordOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🧪️tests/🔬️unit/🟦️.ts";

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));

/** 🧪️ Persisted Object fields and exact child references agree with independent schema admission. */
export function testSemioObjectDocumentContract(): void {
  testSchemaRecordOracle();
  const ajv = new Ajv({ strict: true, allErrors: true });
  for (const key of ["x-semio-state", "x-semio-child"]) ajv.addKeyword(key);
  for (const path of ["../../../../../../../../../../../..//🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json","../../../../../../../../../../../..//🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json","../../../../✉️base/🧬️schema/🪆️child/🔣️.json","../../../../✉️base/🧬️schema/🧮️geometry/🔣️.json"]) ajv.addSchema(read(path));
  const fixtures = read("./🧫️fixtures/🔣️.json");
  const childIdentity = (value: any): boolean => ["brep", "mesh", "properties"].every((field) => !value[field] || ajv.compile({ const: value[field].target.artifactId })(value[field].childId));
  const a = ajv.compile(read("../../🔣️.json"));
  const s = ajv.compile(read("../../📸️snapshot/🔣️.json"));
  const d = ajv.compile(read("../../🔺️diff/🔣️.json"));
  for (const entry of fixtures.snapshotCases) {
    assert.equal(a(entry.input) && childIdentity(entry.input), entry.valid, "artifact oracle");
    assert.equal(s(entry.input) && childIdentity(entry.input), entry.valid, "snapshot oracle");
    if (entry.valid) assert.deepEqual(artifact.parseSemioObjectArtifact(entry.input), entry.input);
    else assert.throws(() => artifact.parseSemioObjectArtifact(entry.input), JSON.stringify(entry.input));
  }
  const parseSnapshot = (snapshot as Record<string, (input: unknown) => unknown>).parseSemioObjectSnapshot!;
  const parseDiff = (diff as Record<string, (input: unknown) => unknown>).parseSemioObjectDiff!;
  for (const entry of fixtures.snapshotCases) {
    if (entry.valid) assert.deepEqual(parseSnapshot(entry.input), entry.input);
    else assert.throws(() => parseSnapshot(entry.input), JSON.stringify(entry.input));
  }
  for (const entry of fixtures.diffCases) {
    assert.equal(d(entry.input) && childIdentity(entry.input), entry.valid, "diff oracle");
    if (entry.valid) assert.deepEqual(parseDiff(entry.input), entry.input);
    else assert.throws(() => parseDiff(entry.input), JSON.stringify(entry.input));
  }
  for (const entry of fixtures.patchCases) {
    const expected = applyPatch(structuredClone(entry.before), entry.patch, true).newDocument;
    assert.deepEqual(expected, entry.after, "independent patch oracle");
    assert.deepEqual(diff.applySemioObjectDiff(artifact.parseSemioObjectArtifact(entry.before), diff.parseSemioObjectDiff(entry.diff)), expected, "parent edit preserves untouched child references");
  }
  const mutationRoot = fileURLToPath(new URL("../../🧬️mutations/", import.meta.url));
  for (const file of fg.sync("*/🧬️schema/🔣️.json", { cwd: mutationRoot, absolute: true })) ajv.addSchema(JSON.parse(readFileSync(file, "utf8")));
  const mutationSchema = ajv.compile(read("../../🧬️mutations/🔣️.json"));
  let snapshots = 0, diffs = 0, mutations = 0;
  const corpusRoot = fileURLToPath(new URL("../../../🧫️fixtures/🧬️mutations/", import.meta.url));
  for (const file of fg.sync("**/🔣️.json", { cwd: corpusRoot, absolute: true })) {
    const value = JSON.parse(readFileSync(file, "utf8"));
    if (file.includes("/📸️snapshot/")) {
      assert(s(value) && childIdentity(value), file + ": snapshot oracle");
      assert.deepEqual(parseSnapshot(value), value, file);
      snapshots++;
    } else if (file.includes("/🔺️diff/")) {
      assert(d(value) && childIdentity(value), file + ": diff oracle");
      assert.deepEqual(parseDiff(value), value, file);
      diffs++;
    } else if (file.includes("/🦠️mutation/")) {
      assert(mutationSchema(value), file + ": mutation schema oracle");
      const payload = Object.values(value)[0] as Record<string, any>;
      if (payload.target) assert(ajv.compile({ const: payload.target.artifactId })(payload.child_id), file + ": mutation child identity");
      mutations++;
    }
  }
  assert(snapshots > 0 && diffs > 0 && mutations > 0, "committed Object corpus must be exercised");
  console.log("[DEBUG] Stdio Object exact contracts: " + fixtures.snapshotCases.length + " snapshot vectors, " + fixtures.diffCases.length + " diff vectors, " + snapshots + " committed snapshots, " + diffs + " committed diffs");
}
