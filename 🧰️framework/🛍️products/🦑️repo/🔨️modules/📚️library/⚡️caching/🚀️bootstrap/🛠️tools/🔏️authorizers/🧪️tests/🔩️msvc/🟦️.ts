import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { readdirSync } from "node:fs";
import { join } from "node:path";
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import { canonicalGoPlan } from "../../../../../../🧪️testing/🐹️go-plan/🟦️.ts";

test("the native extractor's exact production inputs enter its runtime owner's Source closure", () => {
  const row = fixture.runtimeCompanions.cases.find(row => row.path.endsWith("/acquire.go"));
  expect(row).toBeDefined();
  expect(new Ajv({ strict: true }).validate(schema.$defs.RuntimeCompanionCase, row)).toBe(true);
  const module = row!.path.slice(0, -"/acquire.go".length);
  const expected = readdirSync(join(process.cwd(), module)).filter(name => name === "go.mod" || name.endsWith(".go") && !name.endsWith("_test.go")).map(name => module + "/" + name).sort();
  const declared = schema.$defs.RuntimeDataCompanions.const.filter(entry => entry.owner === row!.owner).flatMap(entry => entry.paths).sort();
  expect(declared).toEqual(expected);
  expect(canonicalGoPlan(join(process.cwd(), module)).packages).toEqual(["."]);
  const database = new Database(":memory:");
  try {
    database.run("CREATE TABLE declared(owner TEXT,path TEXT,PRIMARY KEY(owner,path))");
    for (const entry of schema.$defs.RuntimeDataCompanions.const) for (const path of entry.paths) database.run("INSERT INTO declared VALUES (?,?)", [entry.owner, path]);
    expect(database.query("SELECT path FROM declared WHERE owner=? ORDER BY path").all(row!.owner).map(entry => (entry as { path: string }).path).sort()).toEqual(expected);
    expect(database.query("SELECT path FROM declared WHERE path=?").get(row!.excluded)).toBeNull();
    expect(database.query("SELECT count(DISTINCT path) AS value FROM declared").get()).toEqual({ value: fixture.runtimeCompanions.uniquePaths });
  } finally { database.close(); }
});
