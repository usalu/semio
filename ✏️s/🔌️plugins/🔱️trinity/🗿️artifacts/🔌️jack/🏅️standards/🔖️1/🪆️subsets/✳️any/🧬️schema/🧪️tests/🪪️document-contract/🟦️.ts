import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { parseJackArtifact } from "../../🟦️.ts";
import { parseJackSnapshot } from "../../📸️snapshot/🟦️.ts";
import { parseJackDiff } from "../../🔺️diff/🟦️.ts";

const json = (url: URL): unknown => JSON.parse(readFileSync(url, "utf8"));

/** 🪪️ Proves Jack's production parsers and Ajv share the native child-handle document boundary. */
export function testJackDocumentContract(): void {
  const cases = json(new URL("./../../🧫️fixtures/🪪️document-contract/🔣️.json", import.meta.url)) as {
    snapshotFixture: string;
    diffFixture: string;
    expectedChildKind: string;
    invalidDocuments: unknown[];
    invalidDiffs: unknown[];
  };
  const ioSchema = json(new URL("../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json", import.meta.url));
  const childSchema = json(new URL("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json", import.meta.url));
  const artifactSchema = json(new URL("../../🔣️.json", import.meta.url));
  const snapshotSchema = json(new URL("../../📸️snapshot/🔣️.json", import.meta.url));
  const diffSchema = json(new URL("../../🔺️diff/🔣️.json", import.meta.url));
  const snapshot = json(new URL(cases.snapshotFixture, import.meta.url));
  const diff = json(new URL(cases.diffFixture, import.meta.url));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword("x-semio-state");
  ajv.addKeyword("x-semio-child-kind");
  ajv.addSchema(ioSchema);
  ajv.addSchema(childSchema);
  ajv.addSchema(artifactSchema);
  const validateArtifact = ajv.getSchema("https://semio.tech/schema/s/trinity/jack/artifact.json");
  const validateSnapshot = ajv.compile(snapshotSchema);
  const validateDiff = ajv.compile(diffSchema);
  assert(validateArtifact?.(snapshot), JSON.stringify(validateArtifact?.errors));
  assert(validateSnapshot(snapshot), JSON.stringify(validateSnapshot.errors));
  assert(validateDiff(diff), JSON.stringify(validateDiff.errors));
  const artifact = parseJackArtifact(snapshot);
  const parsedSnapshot = parseJackSnapshot(snapshot);
  const parsedDiff = parseJackDiff(diff);
  assert.equal(artifact.content.childId, parsedSnapshot.content.childId);
  assert.equal(artifact.content.target.dialect.artifactKind, cases.expectedChildKind);
  assert.equal(parsedDiff.content, null);
  for (const invalid of cases.invalidDocuments) {
    assert.equal(validateArtifact?.(invalid), false);
    assert.throws(() => parseJackArtifact(invalid));
    assert.equal(validateSnapshot(invalid), false);
    assert.throws(() => parseJackSnapshot(invalid));
  }
  for (const invalid of cases.invalidDiffs) {
    assert.equal(validateDiff(invalid), false);
    assert.throws(() => parseJackDiff(invalid));
  }
  console.log("[DEBUG] Jack artifact, snapshot, and diff accept the committed shared-child fixture and refuse embedded graph and replacement-artifact shapes");
}
