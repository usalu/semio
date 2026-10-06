/** 🏛️ Verifies the shared representation boundary against an independent path oracle. */
import assert from "node:assert/strict";
import { readFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";
import { minimatch } from "minimatch";
import Ajv from "ajv";

const owner = resolve(import.meta.dir, "../../..");
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🏛️ownership/🔣️.json"), "utf8"));
const admitted = new Ajv().compile({
  type: "object", additionalProperties: false, required: ["traits", "binaryOwner", "semanticOwner"],
  properties: {
    traits: {type: "array", minItems: 5, maxItems: 5, items: {type: "object", additionalProperties: false, required: ["name", "owner"], properties: {name: {enum: ["OpText", "OpBinary", "DiffText", "DiffBinary", "DiffCodec"]}, owner: {type: "string"}}}},
    binaryOwner: {type: "string"}, semanticOwner: {type: "string"}
  }
});
assert.equal(admitted(fixture), true);
assert.equal(new Set(fixture.traits.map((row: {name: string}) => row.name)).size, 5);
const semantic = readFileSync(resolve(owner, fixture.semanticOwner), "utf8");
for (const row of fixture.traits as {name: string; owner: string}[]) {
  assert.equal(row.owner.split("/")[0], "🚪️io");
  assert.equal(minimatch(row.owner, "🚪️io/**/*.rs"), true);
  assert.match(readFileSync(resolve(owner, row.owner), "utf8"), new RegExp(`pub trait ${row.name}\\b`));
  assert.doesNotMatch(semantic, new RegExp(`(?:pub trait|pub use[^;]*)\\s${row.name}\\b`));
}
assert.equal(existsSync(resolve(owner, fixture.binaryOwner)), true);
assert.equal(minimatch(fixture.binaryOwner, "🚪️io/💾️binary/**/*.rs"), true);
assert.equal(existsSync(resolve(owner, "🎮️mutation/📦️bytes")), false);
console.log("[DEBUG] replication-io-ownership traits=5 boundary=io oracle=minimatch+Ajv");
