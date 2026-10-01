/** 📖️ PDF1.4's authored page dimensions and ordered text entities. */
import type { PdfSnapshot } from "../🟦️.ts";
import { ArtifactSqliteProjection, artifactSqliteCheckpoint, artifactSqliteTables, artifactSqliteDocument, artifactSqliteInteger, artifactSqliteText, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import { encodeIeee754Cells, readBinary64, type Ieee754Column } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import type { SqliteDatabase } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Language-neutral handwritten PDF1.4 entity schema. */
export const PDF14_SQLITE_SCHEMA = `CREATE TABLE pdf14_document (id INTEGER PRIMARY KEY CHECK (id = 1), schema TEXT NOT NULL);
CREATE TABLE pdf14_page (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES pdf14_document(id), ordinal INTEGER NOT NULL, width REAL, height REAL, text TEXT NOT NULL, width_bits INTEGER NOT NULL, width_class TEXT NOT NULL CHECK (width_class IN ('finite','positiveInfinity','negativeInfinity','nan')), height_bits INTEGER NOT NULL, height_class TEXT NOT NULL CHECK (height_class IN ('finite','positiveInfinity','negativeInfinity','nan')));
`;
const dimensions: readonly Ieee754Column[] = [{ index: 3, width: 64 }, { index: 4, width: 64 }];

/** 📤️ Project the complete canonical owner without encoding its native PDF wire. */
export async function pdf14SnapshotToSqliteDatabase(snapshot: PdfSnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  await artifactSqliteCheckpoint(options, "projectSnapshot", 0, snapshot.pages.length + 1);
  if (snapshot.pages.length + 2 > (options.maxRows ?? 1_000_000)) throw new Error("PDF1.4 SQLite row limit");
  const out = await ArtifactSqliteProjection.create(PDF14_SQLITE_SCHEMA, options);
  await out.insert("pdf14_document", [snapshot.schema], 1n);
  for (const [ordinal, page] of snapshot.pages.entries()) {
    const id = BigInt(ordinal + 1);
    const values = encodeIeee754Cells([id, 1n, BigInt(ordinal), page.width, page.height, page.text], dimensions, options.maxColumns);
    await out.insert("pdf14_page", values.slice(1), id);
  }
  return out.finish();
}

/** 📥️ Restore exact owned numeric words and contiguous page semantics. */
export async function pdf14SnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<PdfSnapshot> {
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, 0);
  const [documents, pages] = await artifactSqliteTables(database, PDF14_SQLITE_SCHEMA, options);
  const document = artifactSqliteDocument(documents!);
  if (document.values.length !== 2 || pages!.length + 2 > (options.maxRows ?? 1_000_000)) throw new Error("PDF1.4 SQLite row width or row limit");
  const result: PdfSnapshot = { schema: artifactSqliteText(document, 1), pages: new Array(pages!.length) };
  const seen = new Set<bigint>();
  for (const [index, row] of pages!.entries()) {
    if (row.rowid <= 0n || artifactSqliteInteger(row, 0) !== row.rowid || seen.has(row.rowid) || artifactSqliteInteger(row, 1) !== 1n || row.values.length !== 10) throw new Error("PDF1.4 page identity or owner differs");
    seen.add(row.rowid);
    const ordinal = artifactSqliteInteger(row, 2);
    if (ordinal < 0n || ordinal >= BigInt(pages!.length) || result.pages[Number(ordinal)]) throw new Error("PDF1.4 page ordinals must be contiguous and unique");
    result.pages[Number(ordinal)] = { width: readBinary64(row, 3, dimensions), height: readBinary64(row, 4, dimensions), text: artifactSqliteText(row, 5) };
    if (index % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", index + 1, pages!.length);
  }
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", pages!.length, pages!.length);
  return result;
}
