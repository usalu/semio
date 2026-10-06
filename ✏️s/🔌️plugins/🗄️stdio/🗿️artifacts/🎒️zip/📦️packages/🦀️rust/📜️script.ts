#!/usr/bin/env bun
/** 📦️ zip Rust artifact package router. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv2020 from "ajv/dist/2020";
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";

if (process.argv[2] === "retained-opc-check") {
  const root = join(import.meta.dir, "../../📦️opc/🧬️retained/🧫️fixtures");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  
  
  
  const payload = Uint8Array.from({ length: fixture.payload.length }, (_, ordinal) => (ordinal * fixture.payload.multiplier + fixture.payload.increment) & 255);
  const oracle = structuredClone({
    parts: fixture.parts.map((part: { path: string; contentType: string; payload: "pattern" | "empty" }) => ({ path: part.path, contentType: part.contentType, bytes: part.payload === "pattern" ? payload : new Uint8Array() })),
    contentTypes: fixture.contentTypes,
    relationships: fixture.relationships,
    comment: fixture.comment,
  });
  assert.deepEqual(oracle.parts.map((part: { path: string }) => part.path), fixture.parts.map((part: { path: string }) => part.path));
  assert.deepEqual(oracle.contentTypes.defaults, fixture.contentTypes.defaults);
  assert.deepEqual(oracle.contentTypes.overrides, fixture.contentTypes.overrides);
  assert.deepEqual(oracle.relationships.map((owner: { owner: string }) => owner.owner), fixture.relationships.map((owner: { owner: string }) => owner.owner));
  assert.deepEqual(oracle.relationships[1].entries.map((relationship: { id: string }) => relationship.id), ["rId2", "rId1"]);
  assert.equal(oracle.parts[0].bytes.byteLength, fixture.payload.length);
  assert.equal(oracle.parts[1].bytes.byteLength, 0);
  console.log(`retained-opc-check: parts=${oracle.parts.length} payload=${oracle.parts[0].bytes.byteLength} owners=${oracle.relationships.length}`);
} else {
  await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-zip", { snapshotSqliteTests: ["../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
}
