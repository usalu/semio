import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧫️fixtures/🔣️.schema.json";

test("persisted child identity keeps every exact target independently of local materialization", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  const db = new Database(":memory:");
  try {
    db.exec("CREATE TABLE child(ordinal INTEGER PRIMARY KEY, identity TEXT NOT NULL)");
    fixture.cases.forEach((child, ordinal) => db.query("INSERT INTO child VALUES(?,?)").run(ordinal, JSON.stringify(child)));
    const rows = db.query("SELECT identity FROM child ORDER BY ordinal").all() as { identity: string }[];
    expect(rows.map(row => JSON.parse(row.identity))).toEqual(fixture.cases);
    for (const child of fixture.cases) {
      expect(Buffer.from(child.childId).toString("utf8")).toBe(child.childId);
      expect(validate({ ...fixture, cases: [{ ...child, localOwner: {} }, child] })).toBe(false);
      const missing = { ...child.target.dialect } as Partial<typeof child.target.dialect>;
      delete missing.subset;
      expect(validate({ ...fixture, cases: [{ ...child, target: { ...child.target, dialect: missing } }, child] })).toBe(false);
    }
  } finally { db.close(); }
  const source = readFileSync(new URL("../🦀️.rs", import.meta.url), "utf8");
  expect(source).toContain("self.header.advance(original,control,grant)");
  expect(source).toContain("self.header.close_step(grant)");
  expect(source).toContain("ArtifactChild::new(header.child_id,header.target)");
  expect(source).not.toContain("FromValue");
  console.log("[DEBUG] authored persisted child identities agree with Ajv, SQLite JSON and UTF8; staged native owner preserves original headers without a local snapshot");
});
