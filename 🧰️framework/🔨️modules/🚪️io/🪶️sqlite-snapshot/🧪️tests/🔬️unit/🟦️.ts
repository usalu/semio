import Ajv from "ajv/dist/2020";
import { readFileSync } from "node:fs";
import ownership from "../../🧫️fixtures/🫳️import-ownership/🔣️.json";
import ownershipSchema from "../../🧬️schema/🫳️import-ownership/🔣️.json";
/** 🧫️ Semantic relational SQLite interoperability against Bun's independent engine. */
import { Database } from "bun:sqlite";
import { describe, expect, test } from "bun:test";
import { exportSqliteDatabase, importSqliteDatabase, parseSqliteDatabaseSchema, validateSqliteDatabaseSchema, type SqliteDatabase, type SqliteValue } from "../../🟦️.ts";
import corpus from "../../🧫️fixtures/🏛️relational/🔣️.json";
import projectionFixture from "../../🧫️fixtures/🏗️projection/🔣️.json";
import { ArtifactSqliteProjection } from "../../🧩️artifact/🟦️.ts";

test("explicit owned row projection bounds copies and yields to cancellation", async () => {
  const out = await ArtifactSqliteProjection.create(projectionFixture.schemaSql, { maxValueBytes: projectionFixture.maxValueBytes });
  await expect(out.insert(projectionFixture.table, [projectionFixture.tooLong])).rejects.toThrow("value limit");
  await out.insert(projectionFixture.table, [projectionFixture.label]);
  const database = await out.finish();
  const bytes = await exportSqliteDatabase(database);
  const db = integrity(bytes);
  try { expect(db.query("SELECT label FROM explicit_entity").get()).toEqual({ label: projectionFixture.label }); } finally { db.close(); }
  const octets = Uint8Array.from(projectionFixture.octets);
  const owned = await ArtifactSqliteProjection.create(projectionFixture.octetsSql);
  await owned.insert("owned_octets", [octets]);
  octets.fill(42);
  const ownedDatabase = await owned.finish();
  await expect(owned.insert("owned_octets", [Uint8Array.of(42)])).rejects.toThrow("completed");
  await expect(owned.finish()).rejects.toThrow("completed");
  const octetsDb = integrity(await exportSqliteDatabase(ownedDatabase));
  try { expect(Array.from((octetsDb.query("SELECT bytes FROM owned_octets").get() as { bytes: Uint8Array }).bytes)).toEqual(projectionFixture.octets); } finally { octetsDb.close(); }
  const controller = new AbortController();
  const cancelled = await ArtifactSqliteProjection.create(projectionFixture.schemaSql, { signal: controller.signal, onProgress: progress => { if (progress.completed >= projectionFixture.cancelAfterRows) controller.abort(); } });
  await expect((async () => { for (let index = 0; index < projectionFixture.inputRows; index++) await cancelled.insert(projectionFixture.table, [projectionFixture.label]); })()).rejects.toHaveProperty("name", "AbortError");
  const beforeCopy = new AbortController();
  const large = await ArtifactSqliteProjection.create(projectionFixture.octetsSql, { signal: beforeCopy.signal, onProgress: progress => { if (progress.completed > 0) beforeCopy.abort(); } });
  const largeOctets = new Uint8Array(100_000);
  let copied = false;
  largeOctets.slice = () => { copied = true; throw new Error("large payload copied before cancellation"); };
  await expect(large.insert("owned_octets", [largeOctets])).rejects.toHaveProperty("name", "AbortError");
  expect(copied).toBe(false);
});

const sql = "CREATE TABLE scalar (id INTEGER PRIMARY KEY, text TEXT, integer_value INTEGER, real_value REAL, bytes BLOB, absent TEXT)";
const sample: SqliteDatabase = { tables: [{ name: "scalar", sql, rows: [{ rowid: 1n, values: [1n, "Grüße 🌠\0end", -9223372036854775808n, 1.25, Uint8Array.of(0, 127, 255), null] }] }] };

function oracle(value: SqliteDatabase, pageSize = 4096): Uint8Array {
  const db = new Database(":memory:");
  try {
    db.run(`PRAGMA page_size=${pageSize}`);
    db.run("PRAGMA application_id=1397576526");
    db.run("PRAGMA user_version=1");
    for (const table of value.tables) {
      db.run(table.sql);
      for (const row of table.rows) db.query(`INSERT INTO "${table.name}" VALUES (${row.values.map(() => "?").join(",")})`).run(...row.values);
    }
    return new Uint8Array(db.serialize());
  } finally { db.close(); }
}

function integrity(bytes: Uint8Array): Database {
  const db = Database.deserialize(bytes);
  expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
  return db;
}

function mutate(bytes: Uint8Array, offset: number, value: number, width = 4): Uint8Array {
  const result = bytes.slice();
  const view = new DataView(result.buffer);
  if (width === 4) view.setUint32(offset, value);
  else if (width === 2) view.setUint16(offset, value);
  else result[offset] = value;
  return result;
}

describe("relational SQLite physical engine", () => {
  test("schema validation preserves quoted keyword semantics from independent SQLite", async () => {
    const expected = "CREATE TABLE t (id INTEGER PRIMARY KEY, value TEXT NOT NULL)";
    const weakened: SqliteDatabase = { tables: [{ name: "t", sql: 'CREATE TABLE t (id INTEGER PRIMARY KEY, value TEXT "NOT" "NULL")', rows: [{ rowid: 1n, values: [1n, null] }] }] };
    const db = integrity(oracle(weakened));
    try { expect(db.query("PRAGMA table_info(t)").all()).toMatchObject([{ name: "id" }, { name: "value", notnull: 0, type: 'TEXT "NOT" "NULL"' }]); } finally { db.close(); }
    expect(() => validateSqliteDatabaseSchema(weakened, expected)).toThrow("schema mismatch");
    const quoted = { tables: [{ name: "T", sql: 'create table "T" ("ID" integer primary key, [value] text not null)', rows: [{ rowid: 1n, values: [1n, "independent"] }] }] };
    const valid = integrity(oracle(quoted));
    valid.close();
    expect(() => validateSqliteDatabaseSchema(quoted, expected)).not.toThrow();
  });

  test("export checks aggregate text and schema budgets before encoding copies", async () => {
    const encode = TextEncoder.prototype.encode;
    const copied: string[] = [];
    const content = "long Unicode 🌠 cell".repeat(10);
    TextEncoder.prototype.encode = function(value?: string) { copied.push(value ?? ""); return encode.call(this, value); };
    try {
      await expect(exportSqliteDatabase({ tables: [{ name: "t", sql: "CREATE TABLE t (id INTEGER PRIMARY KEY, value TEXT)", rows: [{ rowid: 1n, values: [1n, content] }] }] }, { maxValueBytes: 8 })).rejects.toThrow("value limit");
      expect(copied).not.toContain(content);
      copied.length = 0;
      const oversizedSql = `CREATE TABLE t (id INTEGER PRIMARY KEY, value TEXT DEFAULT '${content}')`;
      await expect(exportSqliteDatabase({ tables: [{ name: "t", sql: oversizedSql, rows: [] }] }, { maxSchemaBytes: 1 })).rejects.toThrow("schema limit");
      expect(copied).not.toContain(oversizedSql);
    } finally { TextEncoder.prototype.encode = encode; }
  });

  test("Unicode scalar strings roundtrip through independent SQLite while unpaired UTF-16 rejects", async () => {
    const unicode = "a\u0000é漢𐐀🌠\uFEFF";
    const database: SqliteDatabase = { tables: [{ name: "unicode", sql: "CREATE TABLE unicode (id INTEGER PRIMARY KEY, content TEXT)", rows: [{ rowid: 1n, values: [1n, unicode] }] }] };
    const db = integrity(await exportSqliteDatabase(database));
    try { expect(db.query("SELECT content FROM unicode").get()).toEqual({ content: unicode }); } finally { db.close(); }
    expect((await importSqliteDatabase(oracle(database))).tables[0]!.rows[0]!.values[1]).toBe(unicode);
    for (const malformed of ["\uD800", "\uDC00", "a\uD800x", "\uDC00\uD800"]) {
      await expect(exportSqliteDatabase({ tables: [{ ...database.tables[0]!, rows: [{ rowid: 1n, values: [1n, malformed] }] }] })).rejects.toThrow("UTF-8");
      await expect(exportSqliteDatabase({ tables: [{ name: malformed, sql: `CREATE TABLE "${malformed}" (id INTEGER PRIMARY KEY)`, rows: [] }] })).rejects.toThrow("UTF-8");
      await expect(exportSqliteDatabase({ tables: [{ name: "unicode", sql: `CREATE TABLE unicode (id INTEGER PRIMARY KEY, content TEXT DEFAULT '${malformed}')`, rows: [] }] })).rejects.toThrow("UTF-8");
    }
  });

  test("schema-first SQL scripts split statements while preserving quoted semicolons", async () => {
    const source = "-- leading comment;\nCREATE TABLE first (id INTEGER PRIMARY KEY, content TEXT CHECK(content != ';'));\n/* between; */ CREATE TABLE \"second\" (id INTEGER PRIMARY KEY, target INTEGER REFERENCES first(id));";
    const database = parseSqliteDatabaseSchema(source);
    expect(database.tables.map((table) => table.name)).toEqual(["first", "second"]);
    const db = integrity(await exportSqliteDatabase(database));
    db.close();
    expect(await importSqliteDatabase(oracle(database))).toEqual(database);
    expect(() => parseSqliteDatabaseSchema(source, { maxTables: 1 })).toThrow("table limit");
  });

  test("nested CHECK constraints do not impose column-level NOT NULL", async () => {
    const database = parseSqliteDatabaseSchema("CREATE TABLE entity (id INTEGER PRIMARY KEY, present TEXT NOT NULL, absent TEXT CHECK(present IS NOT NULL));");
    const value: SqliteDatabase = { tables: [{ ...database.tables[0]!, rows: [{ rowid: 1n, values: [1n, "present", null] }] }] };
    const db = integrity(await exportSqliteDatabase(value));
    db.close();
    expect(await importSqliteDatabase(oracle(value))).toEqual(value);
  });
  test("shared language-neutral relational corpus exposes semantic joins", async () => {
    const hydrate = (cell: unknown): SqliteValue => {
      if (cell === null || typeof cell === "number" || typeof cell === "string") return cell;
      if (typeof cell === "object" && "integer" in cell && typeof cell.integer === "string") return BigInt(cell.integer);
      if (typeof cell === "object" && "hex" in cell && typeof cell.hex === "string") return Uint8Array.from(cell.hex.match(/../g) ?? [], (hex) => Number.parseInt(hex, 16));
      throw new Error("invalid relational fixture cell");
    };
    const value: SqliteDatabase = { tables: corpus.tables.map((table) => ({ name: table.name, sql: table.sql, rows: table.rows.map((row) => ({ rowid: BigInt(row.rowid), values: row.values.map(hydrate) })) })) };
    expect(corpus.schemaVersion).toBe(1);
    const bytes = await exportSqliteDatabase(value);
    const db = integrity(bytes);
    try {
      expect(db.query(corpus.query).all()).toEqual(corpus.queryResult);
      expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    } finally { db.close(); }
    expect(await importSqliteDatabase(bytes)).toEqual(value);
    expect(await importSqliteDatabase(oracle(value))).toEqual(value);
  });
  test("semantic scalars, native signed integers and IPK alias in both directions", async () => {
    const bytes = await exportSqliteDatabase(sample);
    const db = integrity(bytes);
    try {
      const row = db.query("SELECT id,text,CAST(integer_value AS TEXT) AS integer_value,real_value,bytes,absent,typeof(real_value) AS kind FROM scalar").get() as Record<string, unknown>;
      expect(row.id).toBe(1);
      expect(row.integer_value).toBe("-9223372036854775808");
      expect(row.real_value).toBe(1.25);
      expect(row.kind).toBe("real");
      expect(row.text).toBe(sample.tables[0]!.rows[0]!.values[1]);
      expect(Array.from(row.bytes as Uint8Array)).toEqual([0, 127, 255]);
      expect(row.absent).toBeNull();
    } finally { db.close(); }
    expect(await importSqliteDatabase(bytes)).toEqual(sample);
    expect(await importSqliteDatabase(oracle(sample))).toEqual(sample);
  });

  test("entity joins, foreign keys and edited relational files require no native artifact codec", async () => {
    const value: SqliteDatabase = { tables: [
      { name: "entity", sql: "CREATE TABLE entity (id INTEGER PRIMARY KEY, name TEXT NOT NULL)", rows: [{ rowid: 1n, values: [1n, "first"] }, { rowid: 2n, values: [2n, "second"] }] },
      { name: "relationship", sql: "CREATE TABLE relationship (id INTEGER PRIMARY KEY, source INTEGER NOT NULL REFERENCES entity(id), target INTEGER NOT NULL REFERENCES entity(id), ordinal INTEGER NOT NULL)", rows: [{ rowid: 1n, values: [1n, 1n, 2n, 0n] }] },
    ] };
    const db = integrity(await exportSqliteDatabase(value));
    try {
      expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
      expect(db.query("SELECT source.name AS source,target.name AS target FROM relationship JOIN entity AS source ON source.id=relationship.source JOIN entity AS target ON target.id=relationship.target").get()).toEqual({ source: "first", target: "second" });
      db.run("UPDATE entity SET name='edited' WHERE id=2");
      const imported = await importSqliteDatabase(new Uint8Array(db.serialize()));
      expect(imported.tables[0]!.rows[1]!.values).toEqual([2n, "edited"]);
      const edited = integrity(await exportSqliteDatabase(imported));
      edited.close();
    } finally { db.close(); }
    expect(await importSqliteDatabase(oracle(value))).toEqual(value);
  });

  test("multiple row and schema interior levels with signed64 keys", async () => {
    const rows = Array.from({ length: 10000 }, (_, i) => ({ rowid: BigInt(i - 5000), values: [BigInt(i - 5000), `entity ${i} `.repeat(25), BigInt(i)] }));
    const value: SqliteDatabase = { tables: Array.from({ length: 100 }, (_, i) => ({ name: `entity_${i}`, sql: `CREATE TABLE entity_${i} (id INTEGER PRIMARY KEY, label TEXT, ordinal INTEGER)`, rows: i === 0 ? rows : [] })) };
    const bytes = await exportSqliteDatabase(value);
    const db = integrity(bytes);
    expect(db.query("SELECT COUNT(*) AS count FROM entity_0").get()).toEqual({ count: 10000 });
    db.close();
    expect(await importSqliteDatabase(bytes)).toEqual(value);
  });

  for (const size of [512, 1024, 2048, 4096, 8192, 16384, 32768, 65536]) test(`independent page size ${size}, schema roots and overflow`, async () => {
    const value: SqliteDatabase = { tables: [...Array.from({ length: 90 }, (_, i) => ({ name: `empty_${i}`, sql: `CREATE TABLE empty_${i} (id INTEGER PRIMARY KEY)`, rows: [] })), { name: "scalar", sql, rows: [{ rowid: -1n, values: [-1n, "x".repeat(100000), 9223372036854775807n, 42, new Uint8Array(180000).fill(165), null] }] }] };
    expect(await importSqliteDatabase(oracle(value, size))).toEqual(value);
  });

  test("quoted ASCII identifiers, signed rowid boundaries, empty databases and tables", async () => {
    const value: SqliteDatabase = { tables: [{ name: "ENTITY", sql: 'CREATE TABLE "ENTITY" ("ID" INTEGER PRIMARY KEY, [name] TEXT, `value` REAL)', rows: [-9223372036854775808n, -1n, 9223372036854775806n, 9223372036854775807n].map((rowid) => ({ rowid, values: [rowid, null, 42] })) }] };
    for (const database of [value, { tables: [] }, { tables: [{ name: "empty", sql: "CREATE TABLE empty (id INTEGER PRIMARY KEY)", rows: [] }] }]) {
      const bytes = await exportSqliteDatabase(database);
      const db = integrity(bytes);
      db.close();
      expect(await importSqliteDatabase(bytes)).toEqual(database);
      if (database.tables.length) expect(await importSqliteDatabase(oracle(database))).toEqual(database);
    }
  });

  test("unsupported indexed schemas, aliases, row widths and resource limits", async () => {
    for (const sql of ["CREATE TABLE bad (id TEXT PRIMARY KEY)", "CREATE TABLE bad (id INTEGER PRIMARY KEY, name TEXT UNIQUE)", "CREATE TABLE bad (id INTEGER PRIMARY KEY AUTOINCREMENT)", "CREATE TABLE bad (id INTEGER PRIMARY KEY) WITHOUT ROWID"]) {
      const value: SqliteDatabase = { tables: [{ name: "bad", sql, rows: [] }] };
      await expect(exportSqliteDatabase(value)).rejects.toThrow("unsupported");
      await expect(importSqliteDatabase(oracle(value))).rejects.toThrow("unsupported");
    }
    const table = sample.tables[0]!;
    await expect(exportSqliteDatabase({ tables: [{ ...table, rows: [{ rowid: 2n, values: table.rows[0]!.values }] }] })).rejects.toThrow("primary key");
    await expect(exportSqliteDatabase({ tables: [{ ...table, rows: [{ rowid: 1n, values: [1n] }] }] })).rejects.toThrow("column");
    await expect(exportSqliteDatabase({ tables: [{ ...table, rows: [...table.rows, ...table.rows] }] })).rejects.toThrow("rowid");
    const bytes = await exportSqliteDatabase(sample);
    for (const options of [{ maxFileBytes: 100 }, { maxValueBytes: 1 }, { maxSchemaBytes: 1 }, { maxRows: 0 }, { maxColumns: 1 }, { maxTables: 0 }, { maxPages: 0 }]) {
      await expect(exportSqliteDatabase(sample, options)).rejects.toThrow("limit");
      await expect(importSqliteDatabase(bytes, options)).rejects.toThrow("limit");
    }
  });

  test("table-level INTEGER primary keys and a schema root with one leaf child", async () => {
    const base = "CREATE TABLE entity (id INTEGER, name TEXT, PRIMARY KEY(id))";
    const sql = base.slice(0, -1) + "/*" + "x".repeat(3980 - base.length - 4) + "*/)";
    const value: SqliteDatabase = { tables: [{ name: "entity", sql, rows: [{ rowid: 1n, values: [1n, "semantic"] }] }] };
    const bytes = await exportSqliteDatabase(value);
    expect(bytes[100]).toBe(5);
    const db = integrity(bytes);
    db.close();
    expect(await importSqliteDatabase(bytes)).toEqual(value);
    expect(await importSqliteDatabase(oracle(value))).toEqual(value);
  });

  test("wide records exercise serial-header varints and declared column limits", async () => {
    const value: SqliteDatabase = { tables: [{ name: "wide", sql: "CREATE TABLE wide (id INTEGER PRIMARY KEY," + Array.from({ length: 599 }, (_, i) => `field_${i} INTEGER`).join(",") + ")", rows: [{ rowid: 1n, values: Array.from({ length: 600 }, (_, i) => BigInt(i === 0 ? 1 : i - 1)) }] }] };
    const bytes = await exportSqliteDatabase(value);
    const db = integrity(bytes);
    expect(db.query("SELECT field_598 FROM wide").get()).toEqual({ field_598: 598 });
    db.close();
    expect(await importSqliteDatabase(bytes)).toEqual(value);
    expect(await importSqliteDatabase(oracle(value))).toEqual(value);
    await expect(importSqliteDatabase(bytes, { maxColumns: 599 })).rejects.toThrow("column limit");
  });

  test("schema separator corruption and quoted SQL keywords fail the independent oracle", async () => {
    const value: SqliteDatabase = { tables: Array.from({ length: 90 }, (_, i) => ({ name: `entity_${i}`, sql: `CREATE TABLE entity_${i} (id INTEGER PRIMARY KEY)`, rows: [] })) };
    const bytes = oracle(value, 512);
    const view = new DataView(bytes.buffer);
    expect(bytes[100]).toBe(5);
    const first = view.getUint16(112);
    const corrupted = bytes.slice();
    corrupted[first + 4] = 0;
    const db = Database.deserialize(corrupted);
    try {
      expect((db.query("PRAGMA integrity_check").all() as { integrity_check: string }[]).some((row) => row.integrity_check.includes("out of order"))).toBe(true);
    } finally { db.close(); }
    await expect(importSqliteDatabase(corrupted)).rejects.toThrow("rowid");
    const invalidSql = await exportSqliteDatabase(sample);
    const original = new TextEncoder().encode(sql);
    const replacement = new TextEncoder().encode(sql.replace("CREATE", '"CREATE"').replace(", ", ",").replace(", ", ","));
    const offset = invalidSql.findIndex((_, index) => original.every((byte, i) => invalidSql[index + i] === byte));
    invalidSql.set(replacement, offset);
    const invalidDb = Database.deserialize(invalidSql);
    try { expect(() => invalidDb.query("PRAGMA integrity_check").all()).toThrow("malformed"); } finally { invalidDb.close(); }
    await expect(importSqliteDatabase(invalidSql)).rejects.toThrow("schema");
  });

  test("corruption, quoted keywords, overflow chains, sliced buffers and cancellation", async () => {
    const value: SqliteDatabase = { tables: [{ name: "bytes", sql: "CREATE TABLE bytes (id INTEGER PRIMARY KEY, value BLOB)", rows: [{ rowid: 1n, values: [1n, new Uint8Array(400000)] }] }] };
    const events: { completed: number; total: number }[] = [];
    const bytes = await exportSqliteDatabase(value, { onProgress: (event) => events.push(event) });
    expect(events.at(-1)?.completed).toBe(events.at(-1)?.total);
    const padded = new Uint8Array(bytes.length + 9);
    padded.set(bytes, 5);
    expect(await importSqliteDatabase(padded.subarray(5, bytes.length + 5))).toEqual(value);
    for (const corrupted of [mutate(bytes, 68, 0), mutate(bytes, 60, 2), mutate(bytes, 16, 513, 2), mutate(bytes, 4103, 1, 1), bytes.subarray(0, bytes.length - 1), mutate(bytes, 8192, 3), mutate(bytes, 8192, 0), mutate(bytes, 8192, 1), mutate(bytes, 8192, 999999)]) await expect(importSqliteDatabase(corrupted)).rejects.toThrow();
    const controller = new AbortController();
    await expect(importSqliteDatabase(bytes, { signal: controller.signal, onProgress: () => controller.abort() })).rejects.toMatchObject({ name: "AbortError" });
    const exporting = new AbortController();
    await expect(exportSqliteDatabase(value, { signal: exporting.signal, onProgress: () => exporting.abort() })).rejects.toMatchObject({ name: "AbortError" });
    const timerController = new AbortController();
    const timer = setTimeout(() => timerController.abort(), 0);
    await expect(exportSqliteDatabase(value, { signal: timerController.signal })).rejects.toMatchObject({ name: "AbortError" });
    clearTimeout(timer);
  });
});


test("physical SQLite transfer produces validated relational values and cancellation", async () => {
  const validate = new Ajv({ strict: true }).compile(ownershipSchema);
  expect(validate(ownership), JSON.stringify(validate.errors)).toBe(true);
  const engine = new Database(":memory:");
  try {
    engine.exec(ownership.schema);
    for (const value of ownership.values) {
      engine.query(`INSERT INTO ${ownership.table} VALUES (?, ?)`).run(1, value);
      const database: SqliteDatabase = { tables: [{ name: ownership.table, sql: ownership.schema, rows: [{ rowid: 1n, values: [1n, value === null ? null : BigInt(value)] }] }] };
      const bytes = await exportSqliteDatabase(database);
      for (let count = 0; count < 2; count++) {
        const restored = await importSqliteDatabase(bytes);
        expect(restored.tables[0]!.rows[0]!.values[1]).toEqual(value === null ? null : BigInt(value));
        expect(engine.query(`SELECT n FROM ${ownership.table}`).get()).toEqual({ n: value });
      }
      const abort = new AbortController();
      await expect(exportSqliteDatabase(database, { signal: abort.signal, onProgress: () => abort.abort() })).rejects.toThrow();
      engine.query(`DELETE FROM ${ownership.table}`).run();
    }
    for (const value of ownership.invalidValues) expect(() => engine.query(`INSERT INTO ${ownership.table} VALUES (?, ?)`).run(1, value)).toThrow();
  } finally { engine.close(); }
});


test("exact guest snapshot transport matches its language-neutral schema", () => {
  const base = new URL("../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/", import.meta.url);
  const schema = JSON.parse(readFileSync(new URL("🔣️.json", base), "utf8"));
  const fixture = JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json", base), "utf8"));
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...fixture, encoding: "opaque" })).toBe(false);
  expect(validate({ ...fixture, limits: { ...fixture.limits, max_rows: -1 } })).toBe(false);
  expect(validate({ ...fixture, payload: [256] })).toBe(false);
});
