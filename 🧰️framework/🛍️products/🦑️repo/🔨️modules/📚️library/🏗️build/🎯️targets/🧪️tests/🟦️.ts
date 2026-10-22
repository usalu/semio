import { expect, test } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import { canonicalBuildTargetV1 } from "../🟦️.ts";

test("canonical build leaves match the closed schema and independent SQLite relation", () => {
  const validate = new Ajv().compile(schema), db = new Database(":memory:");
  try {
    db.run("CREATE TABLE targets (slice TEXT PRIMARY KEY, target TEXT NOT NULL)");
    for (const [slice, definition] of Object.entries(schema.$defs.Targets.properties)) db.run("INSERT INTO targets VALUES (?, ?)", [slice, definition.const]);
    for (const row of fixture.cases) {
      const reference = db.query("SELECT target FROM targets WHERE slice = ?").get(row.slice) as { target: string } | null;
      expect(reference?.target ?? null, row.slice).toBe(row.target);
      expect(validate(row.slice), row.slice).toBe(row.target !== null);
      if (row.target === null) expect(() => canonicalBuildTargetV1(row.slice)).toThrow();
      else expect(canonicalBuildTargetV1(row.slice)).toBe(row.target);
    }
  } finally { db.close(); }
});
