/** 📄️ Text-document semantic entities mirror the adjacent handcrafted SQLite schema and Rust facet. */
import type { TxtSnapshot } from "../🟦️.ts";
import { artifactSqliteBoolean, artifactSqliteCheckpoint, artifactSqliteTextBytes, artifactSqliteValueBudget, artifactSqliteDatabase, artifactSqliteDocument, artifactSqliteDocumentReference, artifactSqliteOrderedRows, artifactSqliteTables, artifactSqliteText, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase, SqliteRow } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Handcrafted SQL kept byte-equal to the adjacent schema asset. */
export const TXT_SQLITE_SCHEMA = `CREATE TABLE text_document (
  id INTEGER PRIMARY KEY CHECK(id = 1),
  schema TEXT NOT NULL,
  trailing_newline INTEGER NOT NULL CHECK(trailing_newline IN (0, 1)),
  line_ending TEXT NOT NULL CHECK(line_ending IN ('lf', 'crlf'))
);
CREATE TABLE text_line (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES text_document(id),
  ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
  content TEXT NOT NULL
);
`;

/** 📤️ Project text lines as directly queryable ordered entities. */
export async function txtSnapshotToSqliteDatabase(snapshot: TxtSnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  await artifactSqliteCheckpoint(options, "projectSnapshot", 0, snapshot.lines.length);
  if (snapshot.lines.length + 1 > (options.maxRows ?? 1_000_000)) throw new Error("Text SQLite row limit");
  if (typeof snapshot.schema !== "string" || typeof snapshot.trailingNewline !== "boolean" || (snapshot.lineEnding !== "lf" && snapshot.lineEnding !== "crLf")) throw new Error("Text snapshot document fields are invalid");
  let valueBytes = 24 * snapshot.lines.length + artifactSqliteTextBytes(snapshot.schema) + 16 + (snapshot.lineEnding === "lf" ? 2 : 4);
  artifactSqliteValueBudget(valueBytes, options);
  for (let ordinal = 0; ordinal < snapshot.lines.length; ordinal++) {
    const line = snapshot.lines[ordinal]!;
    if (typeof line !== "string") throw new Error("Text snapshot line must be a string");
    valueBytes += artifactSqliteTextBytes(line);
    artifactSqliteValueBudget(valueBytes, options);
    if ((ordinal + 1) % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, snapshot.lines.length);
  }
  const lines: SqliteRow[] = [];
  for (let ordinal = 0; ordinal < snapshot.lines.length; ordinal++) {
    const content = snapshot.lines[ordinal]!;
    if (typeof content !== "string") throw new Error("Text snapshot line must be a string");
    const id = BigInt(ordinal + 1);
    lines.push({ rowid: id, values: [id, 1n, BigInt(ordinal), content] });
    if ((ordinal + 1) % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", ordinal + 1, snapshot.lines.length);
  }
  const database = artifactSqliteDatabase(TXT_SQLITE_SCHEMA, [[{ rowid: 1n, values: [1n, snapshot.schema, BigInt(Number(snapshot.trailingNewline)), snapshot.lineEnding === "lf" ? "lf" : "crlf"] }], lines], options);
  await artifactSqliteCheckpoint(options, "projectSnapshot", snapshot.lines.length, snapshot.lines.length);
  return database;
}

/** 📥️ Reconstruct text-document properties and lines from their declared entities. */
export async function txtSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<TxtSnapshot> {
  const total = database.tables.find((table) => table.name.toLowerCase() === "text_line")?.rows.length ?? 0;
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
  const [documents, lineRows] = await artifactSqliteTables(database, TXT_SQLITE_SCHEMA, options);
  const document = artifactSqliteDocument(documents!);
  const ending = artifactSqliteText(document, 3);
  if (ending !== "lf" && ending !== "crlf") throw new Error("Text line ending must be lf or crlf");
  const lines: string[] = [];
  for (const row of artifactSqliteOrderedRows(lineRows!, 2)) {
    artifactSqliteDocumentReference(row, 1);
    lines.push(artifactSqliteText(row, 3));
    if (lines.length % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", lines.length, total);
  }
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", total, total);
  return { schema: artifactSqliteText(document, 1), lines, trailingNewline: artifactSqliteBoolean(document, 2), lineEnding: ending === "lf" ? "lf" : "crLf" };
}
