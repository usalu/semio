/** 🧪️ Independent JSON Schema checks for shared artifact addressing and child wire identity. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🪪️artifact-addressing/🔣️.json" with { type: "json" };
import ioSchema from "../../../../../../🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import linkSchema from "../../🔗️link/🧬️schema/🔣️.json" with { type: "json" };
import blobSchema from "../../📦️blob/🧬️schema/🔣️.json" with { type: "json" };
import { parseArtifactLink } from "../../🔗️link/🧬️schema/🟦️.ts";
import childSchema from "../../🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import manifestSchema from "../../../../../../🔨️modules/🛂️manifest/🧬️schema/🔣️.json" with { type: "json" };
import { dialectCoordinate, parseDialectCoordinate, artifactRefUri, parseArtifactRefUri } from "../../../../../../🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import { parseArtifactChild } from "../../🪆️child/🧬️schema/🟦️.ts";

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
  console.log(`[DEBUG] shared link identities matched ${fixture.validLinks.length} pin variants and rejected ${fixture.invalidLinks.length} foreign or malformed records with independent Ajv`);
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
  console.log(`[DEBUG] shared artifact addressing: ${fixture.valid.length} canonical identities, ${fixture.invalidChildren.length} rejected child records, coordinate/URI codec parity with independent Ajv`);
}
