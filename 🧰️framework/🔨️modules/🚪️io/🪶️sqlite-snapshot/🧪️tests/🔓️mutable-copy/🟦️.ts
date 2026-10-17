/** 🔓️ A deep, independent, mutable copy of a relational database for tests that forge rows (reorder, overwrite a cell, duplicate an identity) to prove the reader refuses them.
 * @see ../../🟦️.ts
 */
import type { SqliteDatabase, SqliteRow, SqliteTable, SqliteValue } from "../../🟦️.ts";

/** 🧾️ A row whose identity and cells a test may overwrite. */
export interface MutableSqliteRow { rowid: bigint; values: SqliteValue[] }
/** 🗃️ A table whose rows a test may reorder, replace or drop. */
export interface MutableSqliteTable { name: string; sql: string; rows: MutableSqliteRow[] }
/** 🗄️ A database a test may rewrite freely; it is still accepted wherever a `SqliteDatabase` is. */
export interface MutableSqliteDatabase { tables: MutableSqliteTable[] }

const copyValue = (value: SqliteValue): SqliteValue => (value instanceof Uint8Array ? value.slice() : value);
const copyRow = (row: SqliteRow): MutableSqliteRow => ({ rowid: row.rowid, values: row.values.map(copyValue) });
const copyTable = (table: SqliteTable): MutableSqliteTable => ({ name: table.name, sql: table.sql, rows: table.rows.map(copyRow) });

/** 🔓️ Copies every table, row and byte cell, so a forged copy never aliases the database it came from. */
export function mutableSqliteDatabase(database: SqliteDatabase): MutableSqliteDatabase {
  return { tables: database.tables.map(copyTable) };
}
