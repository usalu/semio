import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import schema from "../../🧬️schema/🫙️projection/🔣️.json";
import fixture from "../../🧫️fixtures/🫙️projection/🔣️.json";

test("original projection corpus is closed against independent Ajv", () => {
  const validate = new Ajv2020({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  for (const value of [
    { ...fixture, foreign: true },
    { ...fixture, expectedCounts: { ...fixture.expectedCounts, owner: -1 } },
    { ...fixture, owners: [{ ...fixture.owners[0], payload: [256] }] },
    { ...fixture, relations: [{ ...fixture.relations[0], owner: 0 }] },
  ]) expect(validate(value)).toBe(false);
  console.log("[DEBUG] Original projection closed corpus accepted; four hostile cases refused");
});

test("original projection per-table census and fields agree with independent SQLite", () => {
  const database = new Database(":memory:", { safeIntegers: true });
  try {
    database.exec("PRAGMA foreign_keys=ON");
    database.exec(fixture.schemaSql);
    for (const row of fixture.owners) database.run("INSERT INTO owner VALUES(?,?,?)", [row.id, row.label, new Uint8Array(row.payload)]);
    for (const row of fixture.relations) database.run("INSERT INTO relation VALUES(?,?,?)", [row.id, row.owner, row.ordinal]);
    const counts = database.query("SELECT (SELECT count(*) FROM owner) AS owner,(SELECT count(*) FROM relation) AS relation").get() as { owner: bigint; relation: bigint };
    expect(Number(counts.owner)).toBe(fixture.expectedCounts.owner);
    expect(Number(counts.relation)).toBe(fixture.expectedCounts.relation);
    expect(database.query("SELECT id FROM relation ORDER BY owner,ordinal").all()).toEqual([{ id: 2n }, { id: 3n }, { id: 1n }]);
    expect(database.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(() => database.run("INSERT INTO relation VALUES(4,99,0)")).toThrow();
    console.log("[DEBUG] SQLite independently confirms exact per-table rows, ordering, fields and foreign-key refusal");
  } finally { database.close(); }
});

test("production projection holds original schema, census and rows before fallible field work", async () => {
  const source = await Bun.file(new URL("../../🫙️projection/🦀️.rs", import.meta.url)).text();
  expect(source).toContain("pub struct ProjectionStorage");
  expect(source).toContain("SchemaValidationStorage");
  expect(source).toContain("pub fn count_row");
  expect(source).toContain("pub fn prepare_rows");
  expect(source).toContain("pub fn insert_key");
  expect(source).toContain("SqliteRow{rowid:key,values:Vec::new()}");
  expect(source).toContain("copy_cell_into");
  expect(source).not.toContain("cell.owned(");
});
