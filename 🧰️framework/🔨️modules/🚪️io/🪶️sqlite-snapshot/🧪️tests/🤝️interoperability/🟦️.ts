import { describe, expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { exportSqliteDatabase, importSqliteDatabase, type SqliteDatabase, type SqliteValue } from "../../🟦️.ts";

type FixtureValue = null | number | string | { integer: string } | { hex: string };
type Fixture = { tables: { name: string; sql: string; rows: { rowid: string; values: FixtureValue[] }[] }[]; query: string; queryResult: unknown[] };
type ExactIntegerStatement = { safeIntegers(value: boolean): ExactIntegerStatement; values(): SqliteValue[][] };
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🏛️relational/🔣️.json", import.meta.url), "utf8")) as Fixture;
const target = process.env.CARGO_TARGET_DIR ?? ".🧬semio/🦑️repo/⚡️cache/cargo/target";
const oracle = process.env.SEMIO_SQLITE_SNAPSHOT_ORACLE ?? resolve(target, "debug", `semio-io-sqlite-snapshot-oracle${process.platform === "win32" ? ".exe" : ""}`);

function value(cell: FixtureValue): SqliteValue {
  if (cell === null || typeof cell !== "object") return cell;
  return "integer" in cell ? BigInt(cell.integer) : new Uint8Array(Buffer.from(cell.hex, "hex"));
}

const specimen: SqliteDatabase = { tables: fixture.tables.map((table) => ({ ...table, rows: table.rows.map((row) => ({ rowid: BigInt(row.rowid), values: row.values.map(value) })) })) };

function field(cell: SqliteValue): string {
  if (cell === null) return "n";
  if (typeof cell === "bigint") return `i:${cell}`;
  if (typeof cell === "number") return `r:${Object.is(cell, -0) ? "-0" : cell}`;
  if (typeof cell === "string") return `t:${Buffer.from(cell).toString("hex")}`;
  return `b:${Buffer.from(cell).toString("hex")}`;
}

function lines(database: SqliteDatabase): string {
  return database.tables.flatMap((table) => table.rows.map((row) => [Buffer.from(table.name).toString("hex"), row.rowid.toString(), ...row.values.map(field)].join("\t"))).sort().join("\n");
}

function runOracle(args: string[]): void {
  const result = Bun.spawnSync([oracle, ...args], { stdout: "pipe", stderr: "pipe" });
  if (result.exitCode !== 0) throw new Error(`native relational oracle failed: ${result.stderr.toString()}`);
}

function inspect(bytes: Uint8Array, expected: SqliteDatabase): void {
  const database = Database.deserialize(bytes);
  try {
    expect(database.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(database.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(database.query("PRAGMA application_id").get()).toEqual({ application_id: 1397576526 });
    expect(database.query("PRAGMA user_version").get()).toEqual({ user_version: 1 });
    for (const table of expected.tables) {
      const name = table.name.replaceAll('"', '""');
      const statement = database.query(`SELECT rowid,* FROM "${name}" ORDER BY rowid`) as unknown as ExactIntegerStatement;
      const rows = statement.safeIntegers(true).values();
      const sorted = [...table.rows].sort((a, b) => a.rowid < b.rowid ? -1 : a.rowid > b.rowid ? 1 : 0);
      expect(rows.map((row) => row.map(field))).toEqual(sorted.map((row) => [row.rowid, ...row.values].map(field)));
    }
  } finally { database.close(); }
}

function independent(database: SqliteDatabase, pageSize: number): Uint8Array {
  const engine = new Database(":memory:");
  try {
    engine.exec(`PRAGMA page_size=${pageSize}; PRAGMA application_id=1397576526; PRAGMA user_version=1; PRAGMA foreign_keys=ON; CREATE TABLE discarded(value TEXT);`);
    for (const table of database.tables) {
      engine.exec(table.sql);
      for (const row of table.rows) engine.query(`INSERT INTO "${table.name.replaceAll('"', '""')}" VALUES (${row.values.map(() => "?").join(",")})`).run(...row.values);
    }
    engine.exec("DROP TABLE discarded");
    return new Uint8Array(engine.serialize());
  } finally { engine.close(); }
}

async function verify(database: SqliteDatabase, inspectJoin = false): Promise<void> {
  const folder = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "relational-sqlite-"));
  const nativeFile = join(folder, "native.sqlite");
  const typescriptFile = join(folder, "typescript.sqlite");
  const schemaFile = join(folder, "schema.sql");
  const rowsFile = join(folder, "rows.tsv");
  const restoredFile = join(folder, "restored.tsv");
  try {
    writeFileSync(schemaFile, database.tables.map((table) => table.sql.replace(/;\s*$/, "") + ";").join("\n"));
    writeFileSync(rowsFile, lines(database) + "\n");
    runOracle(["export", nativeFile, schemaFile, rowsFile]);
    const native = new Uint8Array(readFileSync(nativeFile));
    inspect(native, database);
    expect(lines(await importSqliteDatabase(native))).toBe(lines(database));
    const typescript = await exportSqliteDatabase(database);
    writeFileSync(typescriptFile, typescript);
    inspect(typescript, database);
    runOracle(["import", typescriptFile, restoredFile]);
    expect(readFileSync(restoredFile, "utf8").trimEnd().split("\n").sort().join("\n")).toBe(lines(database));
    if (inspectJoin) {
      for (const bytes of [native, typescript]) {
        const engine = Database.deserialize(bytes);
        try { expect(engine.query(fixture.query).all()).toEqual(fixture.queryResult); } finally { engine.close(); }
      }
    }
    for (const pageSize of [512, 1024, 4096, 65536]) {
      const bytes = independent(database, pageSize);
      writeFileSync(typescriptFile, bytes);
      expect(lines(await importSqliteDatabase(bytes))).toBe(lines(database));
      runOracle(["import", typescriptFile, restoredFile]);
      expect(readFileSync(restoredFile, "utf8").trimEnd().split("\n").sort().join("\n")).toBe(lines(database));
    }
  } finally { rmSync(folder, { recursive: true, force: true }); }
}

describe("Semantic SQLite Rust, TypeScript and Independent Engine Interoperability", () => {
  test("shared language-neutral entities, relationships and scalar types", async () => verify(specimen, true));
  test("many entity rows, schema pages and overflow cells", async () => {
    const database: SqliteDatabase = { tables: [
      ...Array.from({ length: 80 }, (_, index) => ({ name: `entity_${index}`, sql: `CREATE TABLE entity_${index} (id INTEGER PRIMARY KEY, name TEXT NOT NULL)`, rows: [] })),
      { name: "entity", sql: "CREATE TABLE entity (id INTEGER PRIMARY KEY, ordinal INTEGER NOT NULL, title TEXT NOT NULL)", rows: Array.from({ length: 1500 }, (_, index) => ({ rowid: BigInt(index + 1), values: [BigInt(index + 1), BigInt(index), "Grüße 🌠 ".repeat(index === 700 ? 15000 : 10)] })) },
    ] };
    await verify(database);
  });
  test("SQL edits are imported as semantic entities without a native codec", async () => {
    const bytes = await exportSqliteDatabase(specimen);
    const engine = Database.deserialize(bytes);
    try {
      engine.exec("UPDATE room SET name='Forschungsraum' WHERE ordinal=1; UPDATE building SET height=20.75 WHERE id=1");
      const edited = await importSqliteDatabase(new Uint8Array(engine.serialize()));
      expect(edited.tables.find((table) => table.name === "room")!.rows[1]!.values[3]).toBe("Forschungsraum");
      expect(edited.tables.find((table) => table.name === "building")!.rows[0]!.values[2]).toBe(20.75);
      await verify(edited);
    } finally { engine.close(); }
  });
});
