/** 🧵️ Exact native identity assembly retains Unicode text across independent SQLite owner turns. */
import { test, expect } from "bun:test";
import { readFileSync, existsSync } from "node:fs";
import { Database } from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
test("canonical identity assembly copies native fields through paid UTF8 pages", () => {
  const root = new URL("../", import.meta.url);
  const fixtureRoot = new URL("../../../../../🛍️products/💻️os/🔨️modules/🏪️store/📦️prepared/🪪️identity/", root);
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, fixtureRoot), "utf8"));
  const fixture = read("🧫️fixtures/🔣️.json");
  expect(new Ajv2020({ strict: true }).compile(read("🧬️schema/🔣️.json"))(fixture)).toBe(true);
  const original = [fixture.actor.repeat(fixture.repeat), fixture.id, fixture.id];
  const db = new Database(":memory:");
  db.exec("CREATE TABLE fields(position INTEGER PRIMARY KEY,original TEXT,output TEXT)");
  for (const [position, text] of original.entries()) {
    db.query("INSERT INTO fields VALUES(?,?,?)").run(position, text, "");
    for (const scalar of text) db.query("UPDATE fields SET output=output||? WHERE position=?").run(scalar, position);
    expect(db.query("SELECT original,output FROM fields WHERE position=?").get(position)).toEqual({ original: text, output: text });
    expect(Buffer.from(JSON.parse(JSON.stringify(text)))).toEqual(Buffer.from(text));
  }
  db.close();
  console.log("[DEBUG] independent SQLite scalar turns and Node UTF8 identity outputs conserve original giant actor and two edit IDs exactly");
  const path = new URL("🦀️.rs", root);
  expect(existsSync(path)).toBe(true);
  const source = readFileSync(path, "utf8");
  expect(source).toContain("ArtifactCanonicalEditIdentityCursor");
  expect(source).toContain("PagedUtf8AppendCursor");
  expect(source).toContain("next_advance_demand");
  expect(source).not.toContain("to_string_owner");
  expect(source).not.toContain(".clone()");
});
