import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv, { type AnySchema } from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";

type ProjectionFixture = {
  wire: { spaces: Record<string, { indexedDocuments: unknown[] }>; cursor: number; users: Record<string, unknown> };
  malformed: string[];
};

function exactKeys(value: unknown, keys: string[]): boolean {
  return value !== null && typeof value === "object" && !Array.isArray(value) && Object.keys(value).sort().join("|") === [...keys].sort().join("|");
}

function independentProjection(value: unknown): boolean {
  if (!exactKeys(value, ["spaces", "cursor", "users"])) return false;
  const projection = value as ProjectionFixture["wire"];
  if (!Number.isInteger(projection.cursor) || projection.cursor < 0 || !exactKeys(projection.spaces, Object.keys(projection.spaces)) || !exactKeys(projection.users, Object.keys(projection.users))) return false;
  return Object.values(projection.spaces).every((space) => exactKeys(space, ["view", "members", "documents", "indexedDocuments"]) && Array.isArray(space.indexedDocuments));
}

/** 📇️ Proves the persisted Home directory projection uses the shared host's exact camel-case wire. */
export function testHomeDirectoryProjectionSchema(root: string): void {
  const schema = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json"), "utf8")) as AnySchema & { $id: string };
  const fixture = JSON.parse(readFileSync(join(root, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧫️fixtures/📇️projection-persistence-v1/🔣️.json"), "utf8")) as ProjectionFixture;
  const validate = new Ajv({ strict: true, allErrors: true }).addKeyword("x-semio-note").addSchema(schema).getSchema(`${schema.$id}#/$defs/DirectoryReadModel`);
  assert(validate);
  assert(validate(fixture.wire), JSON.stringify(validate.errors));
  assert(independentProjection(fixture.wire));
  const spaceId = Object.keys(fixture.wire.spaces)[0]!;
  const wrongCase = applyPatch(structuredClone(fixture.wire), [{ op: "move", from: `/spaces/${spaceId}/indexedDocuments`, path: `/spaces/${spaceId}/indexed_documents` } as Operation], true, false).newDocument;
  assert.equal(validate(wrongCase), false);
  assert.equal(independentProjection(wrongCase), false);
  assert(fixture.malformed.every((source) => {
    try {
      return !validate(JSON.parse(source));
    } catch {
      return true;
    }
  }));
}
