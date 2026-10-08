/** 🎟️ SQLite transactions independently preserve original custody before complete birth admission. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));

test("snapshot clone source admission retains every owner until its exact granted birth", () => {
  const law = read("../../🧫️fixtures/🔣️.json");
  const db = new Database(":memory:");
  try {
    db.exec("CREATE TABLE custody (ordinal INTEGER PRIMARY KEY, owner TEXT NOT NULL); CREATE TABLE birth (ordinal INTEGER PRIMARY KEY, bytes INTEGER NOT NULL)");
    const capacity = law.capacity[0];
    law.capacity.forEach((bytes: number) => expect(bytes).toBeLessThanOrEqual(law.maximum));
    for (const grant of law.grants) {
      db.exec("DELETE FROM custody; DELETE FROM birth");
      law.closure.forEach((owner: string, ordinal: number) => db.query("INSERT INTO custody VALUES (?, ?)").run(ordinal, owner));
      const accepted = grant.items > 0 && grant.depth >= law.depth && grant.capacity >= capacity;
      db.exec("BEGIN");
      db.query("INSERT INTO birth VALUES (0, ?)").run(capacity);
      db.exec(accepted ? "COMMIT" : "ROLLBACK");
      expect(accepted).toBe(grant.accepted);
      expect(db.query("SELECT COALESCE(SUM(bytes),0) AS bytes FROM birth").get()).toEqual({ bytes: accepted ? capacity : 0 });
      expect(db.query("SELECT owner FROM custody ORDER BY ordinal").all().map((row: any) => row.owner)).toEqual(law.closure);
    }
    law.capacity.slice(1).forEach((bytes: number, index: number) => {
      db.exec("BEGIN");
      db.query("INSERT INTO birth VALUES (?, ?)").run(index + 1, bytes);
      db.exec("COMMIT");
      expect(db.query("SELECT bytes FROM birth WHERE ordinal = ?").get(index + 1)).toEqual({ bytes });
    });
    while (db.query("SELECT ordinal FROM custody LIMIT 1").get()) {
      const first = db.query("DELETE FROM custody WHERE ordinal = (SELECT MIN(ordinal) FROM custody) RETURNING owner").get() as { owner: string };
      expect(law.closure.includes(first.owner)).toBe(true);
    }
    expect(db.query("SELECT COUNT(*) AS count FROM custody").get()).toEqual({ count: 0 });
    console.log("[DEBUG] snapshot clone SQLite exact birth and ordered original owner conservation");
  } finally { db.close(); }
  const source = readFileSync(new URL("../../../🦀️.rs", import.meta.url), "utf8");
  expect(source.includes("fn begin_demand(")).toBe(true);
  expect(source.includes("RetainedCloneSource::admit_owned")).toBe(true);
  expect(source.includes("RetainedCloneSource::from_authority")).toBe(false);
});
