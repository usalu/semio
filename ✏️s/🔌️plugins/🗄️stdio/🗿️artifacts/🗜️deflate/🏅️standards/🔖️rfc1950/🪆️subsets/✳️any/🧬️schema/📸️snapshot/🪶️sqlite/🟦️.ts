/** 🗜️ Individually authored RFC1950 header and decompressed byte relations. */
import type { DeflateSnapshot } from "../🟦️.ts";
import { ArtifactSqliteProjection, artifactSqliteCheckpoint, artifactSqliteDocument, artifactSqliteDocumentReference, artifactSqliteInteger, artifactSqliteOrderedRows, artifactSqliteTables, artifactSqliteText, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase, SqliteRow } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Literal copy of this snapshot's adjacent handcrafted SQL asset. */
export const DEFLATE_SQLITE_SCHEMA = "CREATE TABLE deflate_document (\n  id INTEGER PRIMARY KEY CHECK (id = 1),\n  schema TEXT NOT NULL,\n  compression_method INTEGER NOT NULL CHECK (compression_method BETWEEN 0 AND 15),\n  window_bits INTEGER NOT NULL CHECK (window_bits BETWEEN 0 AND 15),\n  compression_level_hint INTEGER NOT NULL CHECK (compression_level_hint BETWEEN 0 AND 3),\n  dictionary_adler32 INTEGER CHECK (dictionary_adler32 BETWEEN 0 AND 4294967295)\n);\nCREATE TABLE deflate_payload_byte (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES deflate_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)\n);\n";
const hints = ["fastest", "fast", "default", "maximum"] as const;
function integer(value: number, maximum: number): bigint { if (!Number.isSafeInteger(value) || value < 0 || value > maximum) throw new Error("Deflate integer exceeds its declared width"); return BigInt(value); }
function read(row: SqliteRow, index: number, maximum: number): number { const value = artifactSqliteInteger(row, index); if (value < 0n || value > BigInt(maximum)) throw new Error("Deflate integer exceeds its declared width"); return Number(value); }

/** 📤️ Projects only decoded content and actual owned container metadata. */
export async function deflateSnapshotToSqliteDatabase(snapshot: DeflateSnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  const out = await ArtifactSqliteProjection.create(DEFLATE_SQLITE_SCHEMA, options);
  const hint = hints.indexOf(snapshot.compressionLevelHint);
  if (hint < 0) throw new Error("Deflate compression level hint is unknown");
  await out.insert("deflate_document", [snapshot.schema, integer(snapshot.compressionMethod,15), integer(snapshot.windowBits,15), BigInt(hint), snapshot.dictId === undefined ? null : integer(snapshot.dictId,4294967295)]);
  for (let index = 0; index < snapshot.payload.length; index++) await out.insert("deflate_payload_byte", [1n, BigInt(index), integer(snapshot.payload[index]!,255)]);
  return out.finish();
}

/** 📥️ Reconstructs the exact optional dictionary identity and ordered decoded payload. */
export async function deflateSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<DeflateSnapshot> {
  await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);
  const [documents, payloadRows] = await artifactSqliteTables(database,DEFLATE_SQLITE_SCHEMA,options);
  const document = artifactSqliteDocument(documents!);
  const payload: number[] = [];
  for (const row of artifactSqliteOrderedRows(payloadRows!,2)) { artifactSqliteDocumentReference(row,1); payload.push(read(row,3,255)); if (payload.length % 256 === 0) await artifactSqliteCheckpoint(options,"reconstructSnapshot",payload.length,payloadRows!.length); }
  const dictId = document.values[5] === null ? undefined : read(document,5,4294967295);
  const result: DeflateSnapshot = { schema: artifactSqliteText(document,1), compressionMethod: read(document,2,15), windowBits: read(document,3,15), compressionLevelHint: hints[read(document,4,3)]!, dictId, payload };
  await artifactSqliteCheckpoint(options,"reconstructSnapshot",payload.length,payload.length);
  return result;
}
