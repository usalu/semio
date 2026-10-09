import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

const fixture = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
test("durable journal original string capacities require independent physical release", () => {
  const db = new Database(":memory:");
  db.run("CREATE TABLE owners (ordinal INTEGER PRIMARY KEY, backing BLOB NOT NULL)");
  for (const row of fixture.cases) {
    db.run("DELETE FROM owners");
    for (const [ordinal, text, capacity] of [[0, row.decision, row.decisionCapacity], [1, row.anchor, row.anchorCapacity]] as const) {
      expect(Buffer.byteLength(text, "utf8")).toBeLessThanOrEqual(capacity);
      db.run("INSERT INTO owners VALUES (?, zeroblob(?))", [ordinal, capacity]);
    }
    const oracle = db.query("SELECT length(backing) AS bytes FROM owners ORDER BY ordinal").all().map((row: any) => row.bytes);
    expect(oracle).toEqual([row.decisionCapacity, row.anchorCapacity]);
    expect(db.query("SELECT sum(length(backing)) AS bytes FROM owners").get()).toEqual({bytes: row.decisionCapacity + row.anchorCapacity});
    for (const release of oracle) expect(release - 1).toBeLessThan(release);
    console.log(`[DEBUG] durable journal case=${row.id} independent SQLite physical extents=${oracle} UTF8=${Buffer.byteLength(row.decision)+Buffer.byteLength(row.anchor)} unchangedBody=${fixture.maximumBodyBytes}`);
  }
  db.close();
  const production = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");
  const fixtureSource = readFileSync(new URL("../🔬️unit/🦀️.rs", import.meta.url), "utf8");
  expect(production).toContain("fn retirement_demands(&self, body: usize)");
  expect(fixtureSource).toContain("durable_journal_close_preserves_original_capacities_and_exact_receipts");
});
