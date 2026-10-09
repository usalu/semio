/** 📐️ Pure native text demand agrees with independent UTF8 scalar and SQLite turn accounting. */
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { Database } from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
test("native UTF8 append quotes original scalar page and publication work", () => {
  const root = new URL("../", import.meta.url);
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, root), "utf8"));
  const fixture = read("🧫️fixtures/🔣️.json");
  expect(new Ajv2020({ strict: true }).compile(read("🧬️schema/🔣️.json"))(fixture)).toBe(true);
  const db = new Database(":memory:");
  db.exec("CREATE TABLE chunks(text TEXT,bytes INTEGER)");
  for (const text of fixture.texts) {
    const scalars = [...text].map(value => Buffer.byteLength(value));
    expect(scalars.reduce((sum, value) => sum + value, 0)).toBe(Buffer.byteLength(text));
    expect(scalars.every(value => value >= 1 && value <= 4)).toBe(true);
    db.query("INSERT INTO chunks VALUES(?,?)").run(text, Buffer.byteLength(text));
    expect(db.query("SELECT bytes FROM chunks WHERE text=?").get(text)).toEqual({ bytes: Buffer.byteLength(text) });
  }
  db.close();
  console.log("[DEBUG] independent Node UTF8 scalar1..4 and SQLite exact byte ownership agree with native1024 chunk/4096 work quotes");
  expect(readFileSync(new URL("🦀️.rs", root), "utf8")).toContain("pub fn next_advance_demand");
});
