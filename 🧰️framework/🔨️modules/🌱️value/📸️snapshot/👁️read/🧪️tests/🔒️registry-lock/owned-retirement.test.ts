import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import fixture from "../🧫️fixtures/🔣️.json";

test("owned retirement keeps its source until the complete transfer is funded", () => {
  const db = new Database(":memory:");
  try {
    const sqlQuote = db.query("select ? + ? as copy");
    const decision = db.query("select ? >= ? as moved");
    const { copy } = sqlQuote.get(24, 96) as { copy: number };
    for (const row of fixture.ownedRetirementBirth) {
      expect(Boolean((decision.get(copy + row.copyDelta, copy) as { moved: number }).moved)).toBe(row.moved);
    }
  } finally {
    db.close();
  }
  const source = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");
  const quote = source.slice(source.indexOf("pub fn artifact_retirement_owned_birth_demands"), source.indexOf("pub fn artifact_retirement_admit_owned"));
  const admission = source.slice(source.indexOf("pub fn artifact_retirement_admit_owned"), source.indexOf("pub fn artifact_retirement_shared_lease_birth_demands"));
  expect(quote).toContain("controlled_retirement_birth_demand");
  expect(admission).toContain("retained_clone_grant_admits(grant, demand)");
});
