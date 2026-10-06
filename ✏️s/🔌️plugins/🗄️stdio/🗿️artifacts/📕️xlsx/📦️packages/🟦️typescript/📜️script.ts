#!/usr/bin/env bun
/** 📦️ xlsx TypeScript artifact package router. */
import { runArtifactTypeScriptPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
await runArtifactTypeScriptPackageMain(import.meta.dir, "@semio-tech/stdio-xlsx",{suites:["🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});

if ((process.argv[2] ?? "test") === "test") {
  const root = resolve(import.meta.dir, "../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧫️fixtures/🪟️viewer-cell-window");
  const fixture = JSON.parse(readFileSync(resolve(root, "🔣️.json"), "utf8"));
  assert.equal(fixture.expectedKeys.length, fixture.rows);
  assert.equal(fixture.expectedCells.length, fixture.rows);
  assert(fixture.expectedCells.every((row: string[]) => row.length === fixture.columns));
  console.log(`XLSX viewer expected dimensions checked: rows=${fixture.rows} columns=${fixture.columns} locales=${Object.keys(fixture.labels).join(",")}`);
  const draftRoot = resolve(root, "../✍️unchanged-cell-draft");
  const draftFixture = JSON.parse(readFileSync(resolve(draftRoot, "🔣️.json"), "utf8"));
  const snapshotSchema = JSON.parse(readFileSync(resolve(root, "../../🧬️schema/📸️snapshot/🔣️.json"), "utf8"));
  const xmlSnapshotSchema=JSON.parse(readFileSync(resolve(import.meta.dir,"../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json"),"utf8"));
  const cellSchema=JSON.parse(readFileSync(resolve(root,"../../🧬️schema/🧬️mutations/✍️set-cell/🧬️schema/🔣️.json"),"utf8"));
  const addressSchema=JSON.parse(readFileSync(resolve(root,"../../🧬️schema/🧬️mutations/🧭️cell-address/🔣️.json"),"utf8"));
  const validateDrafts = new Ajv({ strict: true, allErrors: true }).addKeyword({ keyword: "x-semio-state", schemaType: "string" }).addKeyword({ keyword: "x-semio-ui", schemaType: "object" }).addSchema(xmlSnapshotSchema).addSchema(snapshotSchema).addSchema(addressSchema).addSchema(cellSchema).compile({$ref: cellSchema.$id + "#/$defs/XlsxCellValue"});
  for (const entry of draftFixture.cases) assert(validateDrafts(entry.value), JSON.stringify(validateDrafts.errors));
  assert.equal(new Set(draftFixture.cases.map((entry: { id: string }) => entry.id)).size, draftFixture.cases.length);
  assert(draftFixture.sharedStringConflict.index < draftFixture.sharedStrings.length);
  console.log(`XLSX unchanged draft values validated with Ajv: cases=${draftFixture.cases.length} sharedStringConflict=true`);
}
