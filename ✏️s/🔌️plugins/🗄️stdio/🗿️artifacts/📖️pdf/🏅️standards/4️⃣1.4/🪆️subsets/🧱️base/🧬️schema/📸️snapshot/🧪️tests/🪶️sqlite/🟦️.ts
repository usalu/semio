/** 📖️ PDF1.4 owned dimensions, ordered page text and independent SQL laws. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { parsePdfSnapshot } from "../../🟦️.ts";
import { PDF14_SQLITE_SCHEMA, pdf14SnapshotToSqliteDatabase, pdf14SnapshotFromSqliteDatabase } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("PDF1.4 native-neutral fixture is complete and independently editable", async () => {
  expect(PDF14_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  const snapshot = parsePdfSnapshot(await Bun.file(new URL("../../🪶️sqlite/🧫️fixtures/🔣️.json", import.meta.url)).json());
  const bytes = await exportSqliteDatabase(await pdf14SnapshotToSqliteDatabase(snapshot));
  const sql = Database.deserialize(bytes);
  expect(sql.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
  expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(sql.query("SELECT ordinal,text FROM pdf14_page ORDER BY ordinal").all()).toEqual(snapshot.pages.map((page, ordinal) => ({ ordinal, text: page.text })));
  expect(await pdf14SnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(snapshot);
  sql.run("UPDATE pdf14_page SET text='Independent Ω edit' WHERE ordinal=1");
  const restored = await pdf14SnapshotFromSqliteDatabase(await importSqliteDatabase(sql.serialize()));
  expect(restored.schema).toBe(snapshot.schema);
  expect(restored.pages[1]!.text).toBe("Independent Ω edit");
  sql.close();
});

test("PDF1.4 preserves every neutral IEEE word without numeric canonicalization", async () => {
  const cases = await Bun.file(new URL("../../🪶️sqlite/🧫️fixtures/🔢ieee.json", import.meta.url)).json();
  for (const item of cases.binary64) {
    const scalar = { bits: BigInt(`0x${item.bits}`) };
    const snapshot = { schema: "pdf14.custom-schema", pages: [{ width: scalar, height: scalar, text: item.bits }] };
    const bytes = await exportSqliteDatabase(await pdf14SnapshotToSqliteDatabase(snapshot));
    expect(await pdf14SnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(snapshot);
    const sql = Database.deserialize(bytes);
    expect(sql.query("SELECT CAST(width_bits AS TEXT) AS bits,width_class AS class,width IS NULL AS nullQuery FROM pdf14_page").get()).toEqual({ bits: BigInt.asIntN(64,scalar.bits).toString(), class: item.class, nullQuery: item.class === "nan" ? 1 : 0 });
    sql.close();
  }
});

test("PDF1.4 rejects noncontiguous pages, weakened schema and bounded cancellation", async () => {
  const snapshot = parsePdfSnapshot(await Bun.file(new URL("../../🪶️sqlite/🧫️fixtures/🔣️.json", import.meta.url)).json());
  await expect(pdf14SnapshotToSqliteDatabase(snapshot, { maxRows: 1 })).rejects.toThrow();
  await expect(pdf14SnapshotToSqliteDatabase(snapshot, { maxValueBytes: 1 })).rejects.toThrow();
  const controller = new AbortController(); controller.abort();
  await expect(pdf14SnapshotToSqliteDatabase(snapshot, { signal: controller.signal })).rejects.toThrow();
  const valid = await pdf14SnapshotToSqliteDatabase(snapshot);
  const changed = { tables: valid.tables.map(table => table.name === "pdf14_page" ? { ...table, rows: table.rows.map((row, index) => index === 0 ? { ...row, values: row.values.map((value, index) => index === 2 ? 999n : value) } : row) } : table) };
  await expect(pdf14SnapshotFromSqliteDatabase(changed)).rejects.toThrow();
  const weakened = { tables: valid.tables.map(table => ({ ...table, sql: table.sql.replace("schema TEXT NOT NULL", "schema TEXT \"NOT NULL\"") })) };
  await expect(pdf14SnapshotFromSqliteDatabase(weakened)).rejects.toThrow();
});
