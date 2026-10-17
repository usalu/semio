/** 🧪️ The mutable copy is deep and independent, and a forged copy is still a `SqliteDatabase`.
 * @see 🟦️.ts
 */
import { expect, test } from "bun:test";
import type { SqliteDatabase } from "../../🟦️.ts";
import { mutableSqliteDatabase } from "./🟦️.ts";

const database: SqliteDatabase = { tables: [{ name: "t", sql: "CREATE TABLE t(a)", rows: [{ rowid: 1n, values: [1n, new Uint8Array([1, 2])] }, { rowid: 2n, values: [2n, null] }] }] };

test("a forged copy reorders, overwrites and drops rows without touching the original", () => {
  const forged = mutableSqliteDatabase(database);
  forged.tables[0]!.rows.reverse();
  forged.tables[0]!.rows[0]!.rowid = 9n;
  forged.tables[0]!.rows[1]!.values[0] = 99n;
  (forged.tables[0]!.rows[1]!.values[1] as Uint8Array)[0] = 255;
  expect(database.tables[0]!.rows.map((row) => row.rowid)).toEqual([1n, 2n]);
  expect(database.tables[0]!.rows[0]!.values[0]).toBe(1n);
  expect([...(database.tables[0]!.rows[0]!.values[1] as Uint8Array)]).toEqual([1, 2]);
  const accepted: SqliteDatabase = forged;
  expect(accepted.tables[0]!.rows.map((row) => row.rowid)).toEqual([9n, 1n]);
});

test("the copy equals the original until it is forged", () => {
  const copy: SqliteDatabase = mutableSqliteDatabase(database);
  expect(copy).toEqual(database);
});
