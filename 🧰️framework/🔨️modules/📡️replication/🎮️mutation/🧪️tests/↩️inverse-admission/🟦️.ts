import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv/dist/2020";
import { Database } from "bun:sqlite";

/** 🛂️ Independent relational admission oracle for the shared inverse contract. */
export function testInverseAdmission(): void {
  const owner = join(import.meta.dir, "../..");
  const corpus = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/↩️inverse-admission/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/↩️inverse-admission/🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true }).compile(schema);
  assert(Boolean(validate(corpus)), JSON.stringify(validate.errors));
  const database = new Database(":memory:");
  try {
    database.exec("CREATE TABLE outcomes (id TEXT PRIMARY KEY, level TEXT, empty INTEGER NOT NULL, foreign_steps INTEGER NOT NULL)");
    for (const row of corpus.cases) {
      database.query("INSERT INTO outcomes VALUES (?, ?, ?, ?)").run(row.id, row.level, Number(row.empty), Number(row.foreignSteps));
      const result = database.query("SELECT ((level IS NULL OR level IN ('info', 'warning')) AND (empty = 0 OR foreign_steps = 1)) AS admitted FROM outcomes WHERE id = ?").get(row.id) as { admitted: number };
      assert.equal(Boolean(result.admitted), row.inverseAdmitted, row.id);
    }
  } finally { database.close(); }
  console.log(`[DEBUG] Inverse admission: ${corpus.cases.length} neutral outcomes agree with strict Ajv and independent SQLite admission`);
}
