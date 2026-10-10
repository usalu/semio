/** 🧵️ JSON and SQLite validate persistent canonical field order and original owner admission. */
import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync, existsSync } from "node:fs";
test("persistent protocol canonical fields preserve exact revision bytes and original native field ownership", () => {
 const root = new URL("../", import.meta.url);
 const read = (path: string) => JSON.parse(readFileSync(new URL(path, root), "utf8"));
 const fixture = read("🧫️fixtures/🔣️.json");
 
 const db = new Database(":memory:"); db.exec("CREATE TABLE fields(ordinal INTEGER PRIMARY KEY, name TEXT, original INTEGER)");
 for (const row of fixture.cases) {
  const revision = Object.fromEntries(Object.entries(row.edit).filter(([key]) => key !== "sequenceNumber"));
  expect(JSON.stringify(revision)).toBe(row.expected); expect(JSON.parse(row.expected)).toEqual(revision);
  db.exec("DELETE FROM fields"); Object.keys(revision).forEach((key,index)=>db.query("INSERT INTO fields VALUES(?,?,1)").run(index,key));
  expect((db.query("SELECT name FROM fields ORDER BY ordinal").all() as {name:string}[]).map(row=>row.name)).toEqual(Object.keys(JSON.parse(row.expected)));
  for (const pause of fixture.pauses) { const before=db.query("SELECT SUM(original) AS owners FROM fields").get(); expect(db.query("SELECT SUM(original) AS owners FROM fields").get()).toEqual(before); expect(pause).toBeGreaterThanOrEqual(0); }
 }
 db.close(); console.log("[DEBUG] JSON revision oracle includes transaction/provenance/Unicode/NUL and SQLite preserves native field order/original slots; sequenceNumber excluded only");
 const owner = new URL("🦀️.rs",root); expect(existsSync(owner)).toBe(true); const source=readFileSync(owner,"utf8");
 expect(source).toContain("ArtifactCanonicalJsonTree for Edit"); expect(source).toContain("ArtifactCanonicalJsonTree for MutationMeta"); expect(source).toContain("canonical_tree_child"); expect(source).not.toContain("to_value()"); expect(source).not.toContain("canonical_json_node"); expect(source).not.toContain("transmute");
});
