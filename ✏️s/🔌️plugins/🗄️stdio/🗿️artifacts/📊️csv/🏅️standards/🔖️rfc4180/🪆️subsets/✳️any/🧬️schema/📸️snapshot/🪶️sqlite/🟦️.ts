/** 📊️ CSV semantic entities mirror the adjacent handcrafted SQLite schema and Rust facet. */
import type { CsvSnapshot, CsvRecord } from "../🟦️.ts";
import { artifactSqliteBoolean, artifactSqliteCheckpoint, artifactSqliteTextBytes, artifactSqliteValueBudget, artifactSqliteDatabase, artifactSqliteDocument, artifactSqliteDocumentReference, artifactSqliteInteger, artifactSqliteOrderedRows, artifactSqliteTables, artifactSqliteText, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase, SqliteRow } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Handcrafted SQL kept byte-equal to the adjacent schema asset. */
export const CSV_SQLITE_SCHEMA = `CREATE TABLE csv_document (
  id INTEGER PRIMARY KEY CHECK(id = 1),
  schema TEXT NOT NULL,
  has_header INTEGER NOT NULL CHECK(has_header IN (0, 1))
);
CREATE TABLE csv_record (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES csv_document(id),
  ordinal INTEGER NOT NULL CHECK(ordinal >= 0)
);
CREATE TABLE csv_field (
  id INTEGER PRIMARY KEY,
  record_id INTEGER NOT NULL REFERENCES csv_record(id),
  ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
  value TEXT NOT NULL,
  quoted INTEGER NOT NULL CHECK(quoted IN (0, 1))
);
`;

/** 📤️ Project CSV records and their ordered fields as independent semantic entities. */
export async function csvSnapshotToSqliteDatabase(snapshot: CsvSnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  await artifactSqliteCheckpoint(options, "projectSnapshot", 0, snapshot.records.length);
  if (typeof snapshot.schema !== "string" || typeof snapshot.hasHeader !== "boolean") throw new Error("CSV snapshot document fields are invalid");
  let total = snapshot.records.length;
  let valueBytes = 24 * total + artifactSqliteTextBytes(snapshot.schema) + 16;
  artifactSqliteValueBudget(valueBytes, options);
  let counted = 0;
  for (const record of snapshot.records) {
    total += record.fields.length;
    if (!Number.isSafeInteger(total) || total + 1 > (options.maxRows ?? 1_000_000)) throw new Error("CSV SQLite row limit");
    valueBytes += 32 * record.fields.length;
    artifactSqliteValueBudget(valueBytes, options);
    if (++counted % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, total);
    for (const field of record.fields) {
      if (typeof field.value !== "string" || typeof field.quoted !== "boolean") throw new Error("CSV snapshot field is invalid");
      valueBytes += artifactSqliteTextBytes(field.value);
      artifactSqliteValueBudget(valueBytes, options);
      if (++counted % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, total);
    }
  }
  if (total + 1 > (options.maxRows ?? 1_000_000)) throw new Error("CSV SQLite row limit");
  const records: SqliteRow[] = [];
  const fields: SqliteRow[] = [];
  let completed = 0;
  for (let ordinal = 0; ordinal < snapshot.records.length; ordinal++) {
    const id = BigInt(ordinal + 1);
    records.push({ rowid: id, values: [id, 1n, BigInt(ordinal)] });
    completed++;
    if (completed % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", completed, total);
    const record = snapshot.records[ordinal]!;
    for (let fieldOrdinal = 0; fieldOrdinal < record.fields.length; fieldOrdinal++) {
      const field = record.fields[fieldOrdinal]!;
      if (typeof field.value !== "string" || typeof field.quoted !== "boolean") throw new Error("CSV snapshot field is invalid");
      const fieldId = BigInt(fields.length + 1);
      fields.push({ rowid: fieldId, values: [fieldId, id, BigInt(fieldOrdinal), field.value, BigInt(Number(field.quoted))] });
      if (++completed % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", completed, total);
    }
  }
  const database = artifactSqliteDatabase(CSV_SQLITE_SCHEMA, [[{ rowid: 1n, values: [1n, snapshot.schema, BigInt(Number(snapshot.hasHeader))] }], records, fields], options);
  await artifactSqliteCheckpoint(options, "projectSnapshot", total, total);
  return database;
}

/** 📥️ Reconstruct CSV semantics from typed relational records, rejecting dangling references and ordinal gaps. */
export async function csvSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<CsvSnapshot> {
  const total = database.tables.filter((table) => table.name.toLowerCase() === "csv_record" || table.name.toLowerCase() === "csv_field").reduce((sum, table) => sum + table.rows.length, 0);
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
  const [documents, recordRows, fieldRows] = await artifactSqliteTables(database, CSV_SQLITE_SCHEMA, options);
  const document = artifactSqliteDocument(documents!);
  const ids = new Set(recordRows!.map((row) => artifactSqliteInteger(row, 0)));
  const fields = new Map<bigint, SqliteRow[]>();
  for (let index = 0; index < fieldRows!.length; index++) {
    const row = fieldRows![index]!;
    const parent = artifactSqliteInteger(row, 1);
    if (!ids.has(parent)) throw new Error("CSV field has an unknown record");
    const group = fields.get(parent) ?? [];
    group.push(row);
    fields.set(parent, group);
    if (index % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
  }
  const records: CsvRecord[] = [];
  let completed = 0;
  for (const row of artifactSqliteOrderedRows(recordRows!, 2)) {
    artifactSqliteDocumentReference(row, 1);
    const record: CsvRecord = { fields: [] };
    if (++completed % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", completed, total);
    for (const field of artifactSqliteOrderedRows(fields.get(artifactSqliteInteger(row, 0)) ?? [], 2)) {
      record.fields.push({ value: artifactSqliteText(field, 3), quoted: artifactSqliteBoolean(field, 4) });
      if (++completed % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", completed, total);
    }
    records.push(record);
  }
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", total, total);
  return { schema: artifactSqliteText(document, 1), hasHeader: artifactSqliteBoolean(document, 2), records };
}
