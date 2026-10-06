import { expect, test } from "bun:test";

import { Database } from "bun:sqlite";
import corpus from "../🧫️fixtures/🔣️.json";

import { ValueError } from "../../../../🌱️value/⚠️refusal/🟦️.ts";
import { exportSqliteDatabase, parseSqliteDatabaseSchema, sqliteValueByteLength, SqliteAllocationControl } from "../../🟦️.ts";
import { ArtifactSqliteProjection } from "../../🧩️artifact/🟦️.ts";
import "../../🧩️artifact/🧪️tests/🧮️allocation/🟦️.ts";

type Row = typeof corpus.cases[number];
const sql = "CREATE TABLE entity (id INTEGER PRIMARY KEY, value TEXT)";
const specimen = { tables: [{ name: "entity", sql, rows: [{ rowid: 1n, values: [1n, "owned"] }] }] };

async function refuse(row: Row): Promise<unknown> {
  switch (row.operation) {
    case "encoding": return sqliteValueByteLength("\ud800");
    case "schema": return parseSqliteDatabaseSchema("CREATE VIEW entity AS SELECT 1");
    case "nan": return sqliteValueByteLength(NaN);
    case "pages": { const owner = new AbortController(); owner.abort(); return exportSqliteDatabase(specimen, { signal: owner.signal }); }
    case "projection": { const owner = new AbortController(); owner.abort(); return ArtifactSqliteProjection.create(sql, { signal: owner.signal }); }
    case "value": return exportSqliteDatabase(specimen, { maxValueBytes: 0 });
    case "schemaBytes": return exportSqliteDatabase(specimen, { maxSchemaBytes: 0 });
    case "file": return exportSqliteDatabase(specimen, { maxFileBytes: 0 });
    case "allocation": return new SqliteAllocationControl({ maxAllocationBytes: 0 }).admit(1);
    case "rows": return exportSqliteDatabase(specimen, { maxRows: 0 });
    case "columns": return exportSqliteDatabase(specimen, { maxColumns: 0 });
    case "tables": return exportSqliteDatabase(specimen, { maxTables: 0 });
    case "pageCount": return exportSqliteDatabase(specimen, { maxPages: 0 });
    default: throw Error("unowned SQLite fixture operation");
  }
}

test("SQLite refusal has one closed language-neutral schema and independent authority table", () => {
  expect(new Set(corpus.cases.map(row => row.id)).size).toBe(13);
  const head = corpus.cases[0];
  if (!head) throw Error("missing SQLite refusal fixture");
  
  const database = new Database(":memory:");
  try {
    database.run("CREATE TABLE authority (id TEXT PRIMARY KEY, producer TEXT NOT NULL, expected TEXT NOT NULL)");
    for (const row of corpus.cases) database.run("INSERT INTO authority VALUES(?,?,?)", [row.id, row.authority, row.expectedKind]);
    expect(database.query("SELECT id, CASE producer WHEN 'syntax' THEN 'invalidValue' WHEN 'callback' THEN 'canceled' WHEN 'ownership' THEN 'ownershipLimit' WHEN 'work' THEN 'workLimit' END AS kind FROM authority ORDER BY id").all()).toEqual(corpus.cases.map(row => ({id: row.id, kind: row.expectedKind})).sort((a,b) => a.id.localeCompare(b.id)));
  } finally { database.close(); }
  console.log("[DEBUG] Independent SQLite admitted thirteen owned refusal authorities");
});

test("SQLite native-equivalent producers retain actual typed refusal identity", async () => {
  for (const row of corpus.cases) {
    let refusal: unknown;
    try { await refuse(row); } catch (error) { refusal = error; }
    expect(refusal, row.id).toBeInstanceOf(ValueError);
    if (!(refusal instanceof ValueError)) throw Error("untyped SQLite refusal: " + row.id);
    expect<string>(refusal.kind, row.id).toBe(row.expectedKind);
    expect(refusal.message.length, row.id).toBeGreaterThan(0);
    expect<string>(refusal.under("document").kind, row.id).toBe(row.expectedKind);
  }
  console.log("[DEBUG] Thirteen actual SQLite producer kinds retained");
});

test("SQLite cumulative allocation admission never refunds retired backing", () => {
  const control = new SqliteAllocationControl({ maxAllocationBytes: 8 });
  control.admit(3); control.admit(5);
  expect(control.remainingBytes()).toBe(0);
  expect(() => control.admit(1)).toThrow(ValueError);
  expect(control.remainingBytes()).toBe(0);
});
