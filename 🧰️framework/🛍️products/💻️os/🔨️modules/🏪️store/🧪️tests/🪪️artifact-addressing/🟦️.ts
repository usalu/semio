/** 🧪️ Independent JSON Schema checks for shared artifact addressing and child wire identity. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🪪️artifact-addressing/🔣️.json" with { type: "json" };
import ioSchema from "../../../../../../🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import linkSchema from "../../🔗️link/🧬️schema/🔣️.json" with { type: "json" };
import blobSchema from "../../📦️blob/🧬️schema/🔣️.json" with { type: "json" };
import { parseArtifactLink } from "../../🔗️link/🧬️schema/🟦️.ts";
import childSchema from "../../🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import manifestSchema from "../../../../../../🔨️modules/🛂️manifest/🧬️schema/🔣️.json" with { type: "json" };
import { dialectCoordinate, parseDialectCoordinate, artifactRefUri, parseArtifactRefUri } from "../../../../../../🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import { contentId, parseArtifactChild } from "../../🪆️child/🧬️schema/🟦️.ts";
import { sha256Hex } from "../../../../../../🔨️modules/🔏️hash/🟦️.ts";

import openingSchema from "../../../../🎚️config/🧬️schema/🔣️.json" with { type: "json" };
import setDefaultSchema from "../../../../🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🧬️schema/🔣️.json" with { type: "json" };
import clearDefaultSchema from "../../../../🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/🧬️schema/🔣️.json" with { type: "json" };
import { surfaceAppId, parseSurfaceAppId } from "../../../../../../🔨️modules/🛂️manifest/🧬️schema/🟦️.ts";

export function testSharedArtifactAddressingOracle(): void {
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema(ioSchema).addSchema(manifestSchema).addSchema(blobSchema);
  const validateLink = ajv.compile(linkSchema);
  for (const row of fixture.validLinks) {
    assert.equal(validateLink(row), true, JSON.stringify(validateLink.errors));
    assert.deepEqual(parseArtifactLink(row), row);
  }
  for (const row of fixture.invalidLinks) {
    assert.equal(validateLink(row), false, JSON.stringify(row));
    assert.throws(() => parseArtifactLink(row));
  }
  const validate = ajv.compile(childSchema);
  const opening = ajv.compile(openingSchema);
  const setDefault = ajv.compile(setDefaultSchema);
  const clearDefault = ajv.compile(clearDefaultSchema);
  for (const row of fixture.valid) {
    assert.equal(validate(row.child), true, JSON.stringify(validate.errors));
    assert.deepEqual(parseArtifactChild(row.child), row.child);
    assert.equal(dialectCoordinate(row.child.target.dialect), row.coordinate);
    assert.deepEqual(parseDialectCoordinate(row.coordinate), row.child.target.dialect);
    assert.equal(artifactRefUri(row.child.target), row.uri);
    assert.deepEqual(parseArtifactRefUri(row.uri), row.child.target);
    const dialect = row.child.target.dialect;
    const pin = { dialect, app: row.app, role: row.role };
    assert.equal(opening({ defaults: [pin] }), true, JSON.stringify(opening.errors));
    assert.equal(setDefault(pin), true, JSON.stringify(setDefault.errors));
    assert.equal(clearDefault({ dialect, role: row.role }), true, JSON.stringify(clearDefault.errors));
    assert.equal(setDefault({ ...pin, role: "foreign" }), false);
    assert.deepEqual(parseSurfaceAppId(surfaceAppId(dialect, "editor")), { dialect, role: "editor" });
  }
  for (const row of fixture.invalidChildren) {
    assert.equal(validate(row), false);
    assert.throws(() => parseArtifactChild(row));
  }
  for (const coordinate of fixture.invalidCoordinates) assert.throws(() => parseDialectCoordinate(coordinate));
  for (const uri of fixture.invalidUris) assert.throws(() => parseArtifactRefUri(uri));
}

/** 🆔️ `contentId` (TS twin of `store::content_id`) equals Python `hashlib`'s committed answers, every committed id is a
 * schema `ContentId`, and the first-party synchronous SHA-256 equals Node's `crypto` (third-party) on every vector and on
 * inputs that cross the 55/56/64-byte padding boundaries. */
export function testContentIdOracle(): void {
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema(ioSchema).addSchema(childSchema);
  const validateContentId = ajv.compile({ $ref: `${childSchema.$id}#/$defs/ContentId` });
  const encoder = new TextEncoder();
  for (const row of fixture.contentIds) {
    const bytes = encoder.encode(row.text);
    assert.equal(contentId(row.prefix, bytes), row.id);
    assert.equal(validateContentId(row.id), true, JSON.stringify(validateContentId.errors));
    assert.equal(sha256Hex(bytes), createHash("sha256").update(bytes).digest("hex"));
  }
  for (const length of [0, 1, 55, 56, 63, 64, 65, 119, 120, 1000]) {
    const bytes = Uint8Array.from({ length }, (_, index) => (index * 131 + 7) & 0xff);
    assert.equal(sha256Hex(bytes), createHash("sha256").update(bytes).digest("hex"), `length ${length}`);
  }
  for (const invalid of ["catalog", "catalog-4F53CDA18C2BAA0C", "catalog-4f53cda18c2baa0", "-4f53cda18c2baa0c"]) assert.equal(validateContentId(invalid), false, invalid);
}
