/** 💾️ Binary-byte semantic entities mirror the adjacent handcrafted SQLite schema and Rust facet. */
import type { BinarySnapshot } from "../🟦️.ts";
import { artifactSqliteCheckpoint, artifactSqliteTextBytes, artifactSqliteValueBudget, artifactSqliteDatabase, artifactSqliteDocument, artifactSqliteDocumentReference, artifactSqliteInteger, artifactSqliteOrderedRows, artifactSqliteTables, artifactSqliteText, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase, SqliteRow } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Handcrafted SQL kept byte-equal to the adjacent schema asset. */
export const BINARY_SQLITE_SCHEMA = `CREATE TABLE binary_document (
  id INTEGER PRIMARY KEY CHECK(id = 1),
  schema TEXT NOT NULL
);
CREATE TABLE binary_byte (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES binary_document(id),
  ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
  value INTEGER NOT NULL CHECK(value BETWEEN 0 AND 255)
);
`;

/** 📤️ Project each byte as a constrained integer entity, without a hidden blob payload. */
export async function binarySnapshotToSqliteDatabase(snapshot: BinarySnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  await artifactSqliteCheckpoint(options, "projectSnapshot", 0, snapshot.bytes.length);
  if (snapshot.bytes.length + 1 > (options.maxRows ?? 1_000_000)) throw new Error("Binary SQLite row limit");
  if (typeof snapshot.schema !== "string") throw new Error("Binary snapshot schema must be a string");
  artifactSqliteValueBudget(32 * snapshot.bytes.length + artifactSqliteTextBytes(snapshot.schema) + 8, options);
  const bytes: SqliteRow[] = [];
  for (let ordinal = 0; ordinal < snapshot.bytes.length; ordinal++) {
    const value = snapshot.bytes[ordinal]!;
    if (!Number.isInteger(value) || value < 0 || value > 255) throw new Error("Binary byte must be an integer between 0 and 255");
    const id = BigInt(ordinal + 1);
    bytes.push({ rowid: id, values: [id, 1n, BigInt(ordinal), BigInt(value)] });
    if ((ordinal + 1) % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", ordinal + 1, snapshot.bytes.length);
  }
  const database = artifactSqliteDatabase(BINARY_SQLITE_SCHEMA, [[{ rowid: 1n, values: [1n, snapshot.schema] }], bytes], options);
  await artifactSqliteCheckpoint(options, "projectSnapshot", snapshot.bytes.length, snapshot.bytes.length);
  return database;
}

/** 📥️ Reconstruct binary bytes from ordered constrained integer entities. */
export async function binarySnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<BinarySnapshot> {
  const total = database.tables.find((table) => table.name.toLowerCase() === "binary_byte")?.rows.length ?? 0;
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
  const [documents, byteRows] = await artifactSqliteTables(database, BINARY_SQLITE_SCHEMA, options);
  const document = artifactSqliteDocument(documents!);
  const bytes: number[] = [];
  for (const row of artifactSqliteOrderedRows(byteRows!, 2)) {
    artifactSqliteDocumentReference(row, 1);
    const value = artifactSqliteInteger(row, 3);
    if (value < 0n || value > 255n) throw new Error("Binary byte must be between 0 and 255");
    bytes.push(Number(value));
    if (bytes.length % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", bytes.length, total);
  }
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", total, total);
  return { schema: artifactSqliteText(document, 1), bytes };
}
