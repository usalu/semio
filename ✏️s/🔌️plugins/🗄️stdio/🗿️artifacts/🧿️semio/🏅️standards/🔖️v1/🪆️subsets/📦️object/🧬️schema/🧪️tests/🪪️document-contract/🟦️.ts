import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import fg from "fast-glob";
import { applyPatch } from "fast-json-patch";
import { addSemioMutationLeafSchemasV1, semioSchemaAjvV1 } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import * as artifact from "../../🟦️.ts";
import * as snapshot from "../../📸️snapshot/🟦️.ts";
import * as diff from "../../🔺️diff/🟦️.ts";
import {transformFixture} from "../../../../✉️base/🧬️schema/🧮️geometry/🧪️tests/🧰️support/🟦️.ts";
import { testSchemaRecordOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🧪️tests/🔬️unit/🟦️.ts";

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const ownedFixture = (value:any):any => value!==null&&typeof value==="object"&&Object.hasOwn(value,"transform")?{...value,transform:transformFixture(value.transform)}:value;

/** 🧪️ Persisted Object fields and exact child references agree with independent schema admission. */
export function testSemioObjectDocumentContract(): void {
  testSchemaRecordOracle();
  const ajv = semioSchemaAjvV1({ allErrors: true });
  for (const path of ["../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json","../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json","../../../../✉️base/🧬️schema/🪆️child/🔣️.json","../../../../✉️base/🧬️schema/🧮️geometry/🔣️.json"]) ajv.addSchema(read(path));
  const fixtures = read("../../🧫️fixtures/🪪️document-contract/🔣️.json");
  const childIdentity = (value: any): boolean => [["brep","brep"],["mesh","mesh"],["properties","value"]].every(([field,subset]) => !value[field!] || ajv.compile({const:{artifactKind:"s.stdio.semio",standard:"v1",subset}})(value[field!].target.dialect));
  const a = ajv.compile(read("../../🔣️.json"));
  const s = ajv.compile(read("../../📸️snapshot/🔣️.json"));
  const d = ajv.compile(read("../../🔺️diff/🔣️.json"));
  for (const entry of fixtures.snapshotCases) {
    assert.equal(a(entry.input) && childIdentity(entry.input), entry.valid, "artifact oracle");
    assert.equal(s(entry.input) && childIdentity(entry.input), entry.valid, "snapshot oracle");
    if (entry.valid) assert.deepEqual(artifact.parseSemioObjectArtifact(ownedFixture(entry.input)), ownedFixture(entry.input));
    else assert.throws(() => artifact.parseSemioObjectArtifact(ownedFixture(entry.input)), JSON.stringify(entry.input));
  }
  const parseSnapshot = snapshot.parseSemioObjectSnapshot;
  const parseDiff = diff.parseSemioObjectDiff;
  for (const entry of fixtures.snapshotCases) {
    if (entry.valid) assert.deepEqual(parseSnapshot(ownedFixture(entry.input)), ownedFixture(entry.input));
    else assert.throws(() => parseSnapshot(ownedFixture(entry.input)), JSON.stringify(entry.input));
  }
  for (const entry of fixtures.diffCases) {
    assert.equal(d(entry.input) && childIdentity(entry.input), entry.valid, "diff oracle");
    if (entry.valid) assert.deepEqual(parseDiff(ownedFixture(entry.input)), ownedFixture(entry.input));
    else assert.throws(() => parseDiff(ownedFixture(entry.input)), JSON.stringify(entry.input));
  }
  for (const entry of fixtures.patchCases) {
    const expected = applyPatch(structuredClone(entry.before), entry.patch, true).newDocument;
    assert.deepEqual(expected, entry.after, "independent patch oracle");
    assert.deepEqual(diff.applySemioObjectDiff(artifact.parseSemioObjectArtifact(ownedFixture(entry.before)), diff.parseSemioObjectDiff(ownedFixture(entry.diff))), ownedFixture(expected), "parent edit preserves untouched child references");
  }
  addSemioMutationLeafSchemasV1(ajv, new URL("../../🧬️mutations/", import.meta.url));
  const mutationSchema = ajv.compile(read("../../🧬️mutations/🔣️.json"));
  let snapshots = 0, diffs = 0, mutations = 0;
  const corpusRoot = fileURLToPath(new URL("../../../🧫️fixtures/🧬️mutations", import.meta.url));
  for (const file of fg.sync("**/🔣️.json", { cwd: corpusRoot, absolute: true })) {
    const value = JSON.parse(readFileSync(file, "utf8"));
    if (file.includes("/📸️snapshot/")) {
      assert(s(value) && childIdentity(value), file + ": snapshot oracle");
      assert.deepEqual(parseSnapshot(ownedFixture(value)), ownedFixture(value), file);
      snapshots++;
    } else if (file.includes("/🔺️diff/")) {
      assert(d(value) && childIdentity(value), file + ": diff oracle");
      assert.deepEqual(parseDiff(ownedFixture(value)), ownedFixture(value), file);
      diffs++;
    } else if (file.includes("/🦠️mutation/")) {
      assert(mutationSchema(value), file + ": mutation schema oracle");
      const payload = Object.values(value as Record<string, { target?: { artifactId: string }; child_id?: unknown }>)[0]!;
      if (payload.target) assert(typeof payload.child_id === "string", file + ": independent local child identifier");
      mutations++;
    }
  }
  assert(snapshots > 0 && diffs > 0 && mutations > 0, "committed Object corpus must be exercised");
}
