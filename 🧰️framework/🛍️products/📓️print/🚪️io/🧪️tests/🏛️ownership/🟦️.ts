/** 📓️ Checks the chart codec boundary independently of its event-sourced semantics. */
import assert from "node:assert/strict";
import { readFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";
import { minimatch } from "minimatch";

const owner = resolve(import.meta.dir, "../../..");
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🏛️ownership/🔣️.json"), "utf8")) as {semanticOwners: string[]; ioOwners: string[]};
for (const path of fixture.semanticOwners) {
  assert.doesNotMatch(readFileSync(resolve(owner, path), "utf8"), /(?:mod (?:codec|sqlite|sqlite_native)|export .*Sqlite|impl .*Artifact(?:Dsl|Pack))/);
}
for (const path of fixture.ioOwners) {
  assert.equal(path.split("/")[0], "🚪️io");
  assert.equal(minimatch(path, "🚪️io/**/*.{rs,ts}"), true);
  assert.equal(existsSync(resolve(owner, path)), true);
}
console.log("print-io-ownership representations=text,binary,sqlite oracle=minimatch");
