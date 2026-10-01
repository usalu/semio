/** 📑️ TSV semantic entities mirror the adjacent handcrafted SQLite schema and Rust facet. */
import type { TsvSnapshot } from "../🟦️.ts";
import { artifactSqliteBoolean, artifactSqliteCheckpoint, artifactSqliteDatabase, artifactSqliteDocument, artifactSqliteDocumentReference, artifactSqliteInteger, artifactSqliteOrderedRows, artifactSqliteTables, artifactSqliteText, artifactSqliteTextBytes, artifactSqliteValueBudget, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase, SqliteRow } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Handcrafted SQL kept byte-equal to the adjacent schema asset. */
export const TSV_SQLITE_SCHEMA = "CREATE TABLE tsv_document (\n  id INTEGER PRIMARY KEY CHECK(id = 1),\n  schema TEXT NOT NULL,\n  trailing_newline INTEGER NOT NULL CHECK(trailing_newline IN (0, 1)),\n  line_ending TEXT NOT NULL CHECK(line_ending IN ('lf', 'crlf'))\n);\nCREATE TABLE tsv_record (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES tsv_document(id),\n  ordinal INTEGER NOT NULL CHECK(ordinal >= 0)\n);\nCREATE TABLE tsv_field (\n  id INTEGER PRIMARY KEY,\n  record_id INTEGER NOT NULL REFERENCES tsv_record(id),\n  ordinal INTEGER NOT NULL CHECK(ordinal >= 0),\n  value TEXT NOT NULL\n);\n";

/** 📤️ Project TSV records and ordered fields as directly queryable entities. */
export async function tsvSnapshotToSqliteDatabase(snapshot: TsvSnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  await artifactSqliteCheckpoint(options, "projectSnapshot", 0, snapshot.records.length);
  if (typeof snapshot.schema !== "string" || typeof snapshot.trailingNewline !== "boolean" || (snapshot.lineEnding !== "lf" && snapshot.lineEnding !== "crlf")) throw new Error("TSV snapshot document fields are invalid");
  let total = snapshot.records.length;
  let valueBytes = 24 * total + artifactSqliteTextBytes(snapshot.schema) + 16 + snapshot.lineEnding.length;
  artifactSqliteValueBudget(valueBytes, options);
  let counted = 0;
  for (const record of snapshot.records) {
    total += record.length;
    if (!Number.isSafeInteger(total) || total + 1 > (options.maxRows ?? 1_000_000)) throw new Error("TSV SQLite row limit");
    valueBytes += 24 * record.length;
    artifactSqliteValueBudget(valueBytes, options);
    if (++counted % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, total);
    for (const field of record) {
      if (typeof field !== "string") throw new Error("TSV snapshot field must be a string");
      valueBytes += artifactSqliteTextBytes(field);
      artifactSqliteValueBudget(valueBytes, options);
      if (++counted % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, total);
    }
  }
  if (total + 1 > (options.maxRows ?? 1_000_000)) throw new Error("TSV SQLite row limit");
  const records: SqliteRow[] = [];
  const fields: SqliteRow[] = [];
  let completed = 0;
  for (let ordinal = 0; ordinal < snapshot.records.length; ordinal++) {
    const id = BigInt(ordinal + 1);
    records.push({ rowid: id, values: [id, 1n, BigInt(ordinal)] });
    if (++completed % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", completed, total);
    const record = snapshot.records[ordinal]!;
    for (let fieldOrdinal = 0; fieldOrdinal < record.length; fieldOrdinal++) {
      const fieldId = BigInt(fields.length + 1);
      fields.push({ rowid: fieldId, values: [fieldId, id, BigInt(fieldOrdinal), record[fieldOrdinal]!] });
      if (++completed % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", completed, total);
    }
  }
  const database = artifactSqliteDatabase(TSV_SQLITE_SCHEMA, [[{ rowid: 1n, values: [1n, snapshot.schema, BigInt(Number(snapshot.trailingNewline)), snapshot.lineEnding] }], records, fields], options);
  await artifactSqliteCheckpoint(options, "projectSnapshot", total, total);
  return database;
}

/** 📥️ Reconstruct the ordered TSV grid and document properties from declared entities. */
export async function tsvSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<TsvSnapshot> {
  const total = database.tables.filter(table => ["tsv_record", "tsv_field"].includes(table.name.toLowerCase())).reduce((sum, table) => sum + table.rows.length, 0);
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
  const [documents, recordRows, fieldRows] = await artifactSqliteTables(database, TSV_SQLITE_SCHEMA, options);
  const document = artifactSqliteDocument(documents!);
  const ending = artifactSqliteText(document, 3);
  if (ending !== "lf" && ending !== "crlf") throw new Error("TSV line ending must be lf or crlf");
  const ids = new Set(recordRows!.map(row => artifactSqliteInteger(row, 0)));
  const fields = new Map<bigint, SqliteRow[]>();
  for (let index = 0; index < fieldRows!.length; index++) {
    const row = fieldRows![index]!;
    const parent = artifactSqliteInteger(row, 1);
    if (!ids.has(parent)) throw new Error("TSV field has an unknown record");
    const group = fields.get(parent) ?? [];
    group.push(row);
    fields.set(parent, group);
    if (index % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
  }
  const records: string[][] = [];
  let completed = 0;
  for (const row of artifactSqliteOrderedRows(recordRows!, 2)) {
    artifactSqliteDocumentReference(row, 1);
    const record: string[] = [];
    if (++completed % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", completed, total);
    for (const field of artifactSqliteOrderedRows(fields.get(artifactSqliteInteger(row, 0)) ?? [], 2)) {
      record.push(artifactSqliteText(field, 3));
      if (++completed % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", completed, total);
    }
    records.push(record);
  }
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", total, total);
  return { schema: artifactSqliteText(document, 1), records, trailingNewline: artifactSqliteBoolean(document, 2), lineEnding: ending };
}

