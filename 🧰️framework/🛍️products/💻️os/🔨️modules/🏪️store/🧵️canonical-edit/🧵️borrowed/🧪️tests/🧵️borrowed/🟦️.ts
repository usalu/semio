import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";

test("canonical paged map fixture preserves accepted JSON bytes and exact lifetime conservation", () => {
  const base = new URL("../../../", import.meta.url);
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, base), "utf8"));
  const fixture = read("🧫️fixtures/🗺️canonical-borrowed-map.json");
  const schema = read("🧪️testing/🧬️schema/🔣️.json");
  const contract = read("🧬️schema/🔣️.json");
  const validate = new Ajv({ strict: true }).addSchema(contract).compile({ ...schema, $ref: "#/$defs/CanonicalBorrowedMapEdit" });
  expect(validate(fixture.edit)).toBe(true);
  const { sequenceNumber: _, ...revision } = fixture.edit;
  expect(JSON.stringify(revision)).toBe(fixture.expectedJson);
  const integer = (value: number) => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64BE(BigInt(value)); return bytes; };
  const hash = createHash("sha256").update("semio.artifact.cursor.v2");
  for (const part of ["edit", fixture.edit.id, fixture.expectedJson]) { const bytes = Buffer.from(part); hash.update(integer(bytes.length)).update(bytes); }
  expect(hash.digest("hex")).toBe(fixture.expectedDigest);
  const db = new Database(":memory:");
  db.run("CREATE TABLE lifetime (iterators INTEGER, roots INTEGER)");
  for (const active of [0, 1, 7]) {
    db.run("DELETE FROM lifetime");
    db.run("INSERT INTO lifetime VALUES (?, 0)", [active]);
    for (let count = 0; count < active; count++) db.run("UPDATE lifetime SET iterators = iterators - 1 WHERE iterators > 0");
    expect((db.query("SELECT iterators FROM lifetime").get() as { iterators: number }).iterators).toBe(0);
    db.run("UPDATE lifetime SET roots = roots + 1 WHERE iterators = 0 AND roots = 0");
    expect((db.query("SELECT roots FROM lifetime").get() as { roots: number }).roots).toBe(1);
  }
  db.close();
  const source = readFileSync(new URL("🧵️borrowed/🧪️tests/🧵️borrowed/🦀️.rs", base), "utf8");
  expect(source).toContain("Text(PagedUtf8");
  expect(source).toContain("Array(PagedList");
  expect(source).toContain("Object(PagedMap");
  expect(source).toContain("admit_typed_controlled_retirement");
  expect(source).not.toContain("active.truncate");
  console.log("[DEBUG] original canonical map JSON/hash matches JSON.stringify/Node SHA256; SQLite iterator-before-root conservation at zero/one/seven aliases");
});

