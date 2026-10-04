/** 📄️ Text-document semantic entities mirror the adjacent handcrafted SQLite schema and Rust facet. */
import type { TxtSnapshot } from "../🟦️.ts";
import { ArtifactSqliteProjection, artifactSqliteBoolean, artifactSqliteCheckpoint, artifactSqliteInteger, artifactSqliteValueBudget, artifactSqliteValueByteLengthControlled, artifactSqliteOrderedRowsControlled, artifactSqliteTables, artifactSqliteText, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Handcrafted SQL kept byte-equal to the adjacent schema asset. */
export const TXT_SQLITE_SCHEMA = `CREATE TABLE text_document (
  id INTEGER PRIMARY KEY CHECK(id > 0),
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
  const ending = snapshot.lineEnding === "lf" ? "lf" : "crlf";
  let valueBytes = 16 + ending.length;
  valueBytes += await artifactSqliteValueByteLengthControlled(snapshot.schema, options, "projectSnapshot", valueBytes);
  artifactSqliteValueBudget(valueBytes, options);
  for (let ordinal = 0; ordinal < snapshot.lines.length; ordinal++) {
    const line = snapshot.lines[ordinal]!;
    if (typeof line !== "string") throw new Error("Text snapshot line must be a string");
    valueBytes += 24;
    valueBytes += await artifactSqliteValueByteLengthControlled(line, options, "projectSnapshot", valueBytes);
    artifactSqliteValueBudget(valueBytes, options);
    if ((ordinal + 1) % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, snapshot.lines.length);
  }
  const projection = await ArtifactSqliteProjection.create(TXT_SQLITE_SCHEMA, options);
  projection.checkRowsAdditional(snapshot.lines.length + 1);
  const document = await projection.insert("text_document", [snapshot.schema, BigInt(Number(snapshot.trailingNewline)), ending]);
  for (let ordinal = 0; ordinal < snapshot.lines.length; ordinal++) {
    const content = snapshot.lines[ordinal]!;
    if (typeof content !== "string") throw new Error("Text snapshot line must be a string");
    await projection.insert("text_line", [document, BigInt(ordinal), content]);
    if ((ordinal + 1) % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", ordinal + 1, snapshot.lines.length);
  }
  const database = await projection.finish();
  await artifactSqliteCheckpoint(options, "projectSnapshot", snapshot.lines.length, snapshot.lines.length);
  return database;
}

/** 📥️ Reconstruct text-document properties and lines from their declared entities. */
export async function txtSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<TxtSnapshot> {
  const total = database.tables.find((table) => table.name.toLowerCase() === "text_line")?.rows.length ?? 0;
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, total);
  const [documents, lineRows] = await artifactSqliteTables(database, TXT_SQLITE_SCHEMA, options);
  if (documents!.length !== 1) throw new Error("Text SQLite requires one document");
  const document = documents![0]!;
  const key = artifactSqliteInteger(document, 0);
  if (key <= 0n || key !== document.rowid || document.values.length !== 4) throw new Error("Text document identity or columns are invalid");
  const ending = artifactSqliteText(document, 3);
  if (ending !== "lf" && ending !== "crlf") throw new Error("Text line ending must be lf or crlf");
  const lines: string[] = [];
  for (const row of await artifactSqliteOrderedRowsControlled(lineRows!, 2, options)) {
    if (row.values.length !== 4 || row.rowid <= 0n || artifactSqliteInteger(row, 0) !== row.rowid) throw new Error("Text line identity or columns are invalid");
    if (artifactSqliteInteger(row, 1) !== key) throw new Error("Text line has an unknown document");
    lines.push(artifactSqliteText(row, 3));
    if (lines.length % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", lines.length, total);
  }
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", total, total);
  return { schema: artifactSqliteText(document, 1), lines, trailingNewline: artifactSqliteBoolean(document, 2), lineEnding: ending === "lf" ? "lf" : "crLf" };
}
