import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/🔣️.schema.json";

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
      const wrongType = { ...child.target.dialect, subset: 0 };
      expect(validate({ ...fixture, cases: [{ ...child, target: { ...child.target, dialect: wrongType } }, child] })).toBe(false);
      expect(validate({ ...fixture, cases: [{ ...child, target: { ...child.target, dialect: missing } }, child] })).toBe(false);
    }
  } finally { db.close(); }
});
