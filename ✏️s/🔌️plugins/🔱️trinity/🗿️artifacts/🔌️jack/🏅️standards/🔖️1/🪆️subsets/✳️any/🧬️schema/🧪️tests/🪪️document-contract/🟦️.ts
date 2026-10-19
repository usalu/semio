import { artifactReferenceSchema as artifactReferenceSchema } from "@semio-tech/framework/schema/artifact-reference";
import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import type { AnySchema } from "ajv";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import { parseJackArtifact } from "../../🟦️.ts";
import { parseJackSnapshot } from "../../📸️snapshot/🟦️.ts";
import { parseJackDiff } from "../../🔺️diff/🟦️.ts";
import { parseJackJsonValue } from "../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts";
import { frameworkSourceUrl } from "@semio-tech/framework/test/source-text";

const json = (url: URL): unknown => JSON.parse(readFileSync(url, "utf8"));

/** 🧬️ Reads one JSON Schema document — the same `JSON.parse`, typed as what Ajv actually takes. */
const schema = (url: URL): AnySchema => JSON.parse(readFileSync(url, "utf8"));

/** 🪪️ Proves Jack's production parsers and Ajv share the native child-handle document boundary. */
export function testJackDocumentContract(): void {
  const cases = json(new URL("../../../🧪️tests/🧫️fixtures/🔬️contract/🪪️document/🔣️.json", import.meta.url)) as {
    snapshotFixture: string;
    diffFixture: string;
    expectedContentCapability: string;
    expectedSelectedKind: string;
    invalidDocuments: unknown[];
    invalidDiffs: unknown[];
  };
  const ioSchema = schema(frameworkSourceUrl("🔨️modules/🚪️io/🧬️schema/🔣️.json"));
  const childSchema = schema(frameworkSourceUrl("🔨️modules/🧬️schema/🗿️artifact-reference/🪆️child/🧬️schema/🔣️.json"));
  const artifactSchema = schema(new URL("../../🔣️.json", import.meta.url));
  const snapshotSchema = schema(new URL("../../📸️snapshot/🔣️.json", import.meta.url));
  const diffSchema = schema(new URL("../../🔺️diff/🔣️.json", import.meta.url));
  const snapshot = json(new URL(cases.snapshotFixture, import.meta.url));
  const diff = json(new URL(cases.diffFixture, import.meta.url));
  const ajv = semioSchemaAjvV1({ allErrors: true }).addSchema(artifactReferenceSchema);
  ajv.addSchema(schema(frameworkSourceUrl("🔨️modules/🌱️value/🧬️schema/🔣️.json")));
  ajv.addSchema(schema(frameworkSourceUrl("🔨️modules/🌱️value/🏷️type/🧬️schema/🔣️.json")));
  ajv.addSchema(ioSchema);
  ajv.addSchema(childSchema);
  ajv.addSchema(artifactSchema);
  const validateArtifact = ajv.getSchema("https://json.schemas.assets.semio-tech.com/s/trinity/jack/artifact.json");
  const validateSnapshot = ajv.compile(snapshotSchema);
  const validateDiff = ajv.compile(diffSchema);
  assert(validateArtifact?.(snapshot), JSON.stringify(validateArtifact?.errors));
  assert(validateSnapshot(snapshot), JSON.stringify(validateSnapshot.errors));
  assert(validateDiff(diff), JSON.stringify(validateDiff.errors));
  const artifact = parseJackJsonValue(snapshot);
  const parsedSnapshot = parseJackSnapshot(artifact);
  const parsedDiff = parseJackDiff(diff);
  assert.equal(artifact.content.childId, parsedSnapshot.content.childId);
  assert.equal(artifact.content.target.dialect.artifactKind, cases.expectedSelectedKind);
  for (const schema of [artifactSchema, snapshotSchema, diffSchema] as Array<{ properties: { content: Record<string, unknown> } }>) {
    assert.equal(schema.properties.content["x-semio-child-capability"], cases.expectedContentCapability);
  }
  const fixtureRoot = join(import.meta.dir, "../../../🧪️tests/🧫️fixtures/🧬️mutations");
  const snapshotFixtures = readdirSync(fixtureRoot, { recursive: true })
    .map((path) => String(path).replaceAll("\\", "/"))
    .filter((path) => path.includes("/📸️snapshot/") && path.endsWith("/🔣️.json"));
  assert.equal(snapshotFixtures.length, 2);
  for (const path of snapshotFixtures) {
    const parsed = parseJackSnapshot(parseJackJsonValue(json(new URL(join(fixtureRoot, path), import.meta.url))));
    assert.equal(parsed.content.target.dialect.artifactKind, cases.expectedSelectedKind, path);
    assert.equal(parsed.content.target.artifactId, parsed.content.childId, path);
  }
  assert.equal(parsedDiff.content, null);
  for (const invalid of cases.invalidDocuments) {
    assert.equal(validateArtifact?.(invalid), false);
    assert.throws(() => parseJackJsonValue(invalid));
    assert.equal(validateSnapshot(invalid), false);
    assert.throws(() => parseJackSnapshot(invalid));
  }
  for (const invalid of cases.invalidDiffs) {
    assert.equal(validateDiff(invalid), false);
    assert.throws(() => parseJackDiff(invalid));
  }
}
