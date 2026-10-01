/** 🧩️ Handwritten COS values, ordered dictionary members and ten intrinsic stream filters. */
import type { PdfObject, PdfDictEntry, PdfStreamFilter, PdfPredictor } from "../../🟦️.ts";
import { PdfProjection, PdfReader, pdfInteger, pdfNumber, pdfBoolean, pdfNullExcept, type ArtifactSqliteOptions, type SqliteDatabase, type SqliteRow, type SqliteValue } from "../🧩️entity/🟦️.ts";
import { artifactSqliteInteger, artifactSqliteText } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";

/** 🏛️ Independently interpretable PDF COS entities, matching the native owner's schema. */
export const PDF_COS_SQLITE_SCHEMA = `CREATE TABLE pdf_filter_chain (id INTEGER PRIMARY KEY);
CREATE TABLE pdf_stream_filter (id INTEGER PRIMARY KEY, chain_id INTEGER NOT NULL REFERENCES pdf_filter_chain(id), ordinal INTEGER NOT NULL, kind TEXT NOT NULL CHECK (kind IN ('flate','lzw','asciiHex','ascii85','runLength','dct','jpx','ccitt','jbig2','crypt')), early_change INTEGER CHECK (early_change IN (0,1)), color_transform INTEGER, globals BLOB, crypt_name TEXT);
CREATE TABLE pdf_filter_predictor (id INTEGER PRIMARY KEY REFERENCES pdf_stream_filter(id), predictor INTEGER NOT NULL, colors INTEGER NOT NULL, bits_per_component INTEGER NOT NULL, columns INTEGER NOT NULL);
CREATE TABLE pdf_filter_ccitt (id INTEGER PRIMARY KEY REFERENCES pdf_stream_filter(id), k INTEGER NOT NULL, columns INTEGER NOT NULL, rows INTEGER NOT NULL, black_is_1 INTEGER NOT NULL CHECK (black_is_1 IN (0,1)), encoded_byte_align INTEGER NOT NULL CHECK (encoded_byte_align IN (0,1)), end_of_line INTEGER NOT NULL CHECK (end_of_line IN (0,1)), end_of_block INTEGER NOT NULL CHECK (end_of_block IN (0,1)), damaged_rows_before_error INTEGER NOT NULL);
CREATE TABLE pdf_cos_value (id INTEGER PRIMARY KEY, kind TEXT NOT NULL CHECK (kind IN ('null','boolean','integer','decimal','string','name','array','dictionary','reference','stream')), boolean_value INTEGER CHECK (boolean_value IN (0,1)), integer_value INTEGER, negative INTEGER CHECK (negative IN (0,1)), coefficient TEXT, scale INTEGER, byte_string BLOB, name TEXT, reference_number INTEGER, reference_generation INTEGER, stream_data BLOB, filter_chain_id INTEGER REFERENCES pdf_filter_chain(id));
CREATE TABLE pdf_cos_array_element (id INTEGER PRIMARY KEY, array_id INTEGER NOT NULL REFERENCES pdf_cos_value(id), ordinal INTEGER NOT NULL, value_id INTEGER NOT NULL REFERENCES pdf_cos_value(id));
CREATE TABLE pdf_cos_dictionary_entry (id INTEGER PRIMARY KEY, dictionary_id INTEGER NOT NULL REFERENCES pdf_cos_value(id), ordinal INTEGER NOT NULL, key TEXT NOT NULL, value_id INTEGER NOT NULL REFERENCES pdf_cos_value(id));
`;

/** 🗜️ Project explicit stream filter parameters, including optional predictors and global bytes. */
export async function writePdfFilters(out: PdfProjection, filters: readonly PdfStreamFilter[]): Promise<bigint> {
  const chain = await out.insert("pdf_filter_chain", []);
  out.checkRowsAdditional(filters.length);
  for (const [ordinal, filter] of filters.entries()) {
    const fields: SqliteValue[] = [null, null, null, null];
    let predictor: PdfPredictor | null = null;
    switch (filter.kind) {
      case "flate": predictor = filter.predictor ?? null; break;
      case "lzw": fields[0] = boolCell(filter.earlyChange); predictor = filter.predictor ?? null; break;
      case "dct": fields[1] = filter.colorTransform == null ? null : pdfInteger(filter.colorTransform); break;
      case "jbig2": fields[2] = filter.globals == null ? null : await out.bytes(filter.globals); break;
      case "crypt": fields[3] = filter.name ?? null; break;
      case "asciiHex": case "ascii85": case "runLength": case "jpx": case "ccitt": break;
      default: throw new Error("Unknown PDF stream filter");
    }
    const key = await out.insert("pdf_stream_filter", [chain, pdfInteger(ordinal, 64), filter.kind, ...fields]);
    if (predictor) await out.insert("pdf_filter_predictor", [pdfInteger(predictor.predictor), pdfInteger(predictor.colors), pdfInteger(predictor.bitsPerComponent), pdfInteger(predictor.columns)], key);
    if (filter.kind === "ccitt") {
      const value = filter.parameters;
      await out.insert("pdf_filter_ccitt", [pdfInteger(value.k ?? 0, 32, true), pdfInteger(value.columns ?? 1728), pdfInteger(value.rows ?? 0), boolCell(value.blackIs1 ?? false), boolCell(value.encodedByteAlign ?? false), boolCell(value.endOfLine ?? false), boolCell(value.endOfBlock ?? true), pdfInteger(value.damagedRowsBeforeError ?? 0)], key);
    }
  }
  return chain;
}

/** 📥️ Reconstruct one filter chain and reject parameters owned by a different variant. */
export async function readPdfFilters(reader: PdfReader, chain: bigint): Promise<PdfStreamFilter[]> {
  await reader.take("pdf_filter_chain", chain, 1);
  const filters: PdfStreamFilter[] = [];
  for (const member of await reader.children("pdf_stream_filter", 1, 2, chain)) {
    const row = await reader.take("pdf_stream_filter", member.rowid, 8);
    const kind = artifactSqliteText(row, 3);
    let predictor: PdfPredictor | null = null;
    if (reader.has("pdf_filter_predictor", row.rowid)) { const value = await reader.take("pdf_filter_predictor", row.rowid, 5); predictor = { predictor: pdfNumber(value, 1), colors: pdfNumber(value, 2), bitsPerComponent: pdfNumber(value, 3), columns: pdfNumber(value, 4) }; }
    if (predictor && kind !== "flate" && kind !== "lzw") throw new Error("PDF predictor belongs only to Flate or LZW");
    pdfNullExcept(row, 4, 8, kind === "lzw" ? [4] : kind === "dct" ? [5] : kind === "jbig2" ? [6] : kind === "crypt" ? [7] : []);
    switch (kind) {
      case "flate": filters.push({ kind, predictor }); break;
      case "lzw": filters.push({ kind, predictor, earlyChange: pdfBoolean(row, 4) }); break;
      case "asciiHex": case "ascii85": case "runLength": case "jpx": filters.push({ kind }); break;
      case "dct": filters.push({ kind, colorTransform: row.values[5] === null ? null : pdfNumber(row, 5) }); break;
      case "jbig2": filters.push({ kind, globals: row.values[6] === null ? null : await reader.bytes(row, 6) }); break;
      case "crypt": filters.push({ kind, name: await reader.optionalText(row, 7) }); break;
      case "ccitt": {
        const value = await reader.take("pdf_filter_ccitt", row.rowid, 9);
        filters.push({ kind, parameters: { k: pdfNumber(value, 1, 32, true), columns: pdfNumber(value, 2), rows: pdfNumber(value, 3), blackIs1: pdfBoolean(value, 4), encodedByteAlign: pdfBoolean(value, 5), endOfLine: pdfBoolean(value, 6), endOfBlock: pdfBoolean(value, 7), damagedRowsBeforeError: pdfNumber(value, 8) } }); break;
      }
      default: throw new Error("Unknown PDF stream filter kind");
    }
  }
  return filters;
}

function boolCell(value: boolean): bigint { if (typeof value !== "boolean") throw new Error("PDF boolean must be an owned boolean"); return value ? 1n : 0n; }
interface WriteFrame { readonly owner: PdfObject; readonly key: bigint; readonly entries: readonly PdfDictEntry[] | null; readonly values: readonly PdfObject[] | null; index: number }
function writeFrame(owner: PdfObject, key: bigint): WriteFrame | null { switch (owner.kind) { case "array": return { owner, key, values: owner.value, entries: null, index: 0 }; case "dict": return { owner, key, values: null, entries: owner.value, index: 0 }; case "stream": return { owner, key, values: null, entries: owner.dict, index: 0 }; default: return null; } }

async function writeShallow(out: PdfProjection, value: PdfObject): Promise<bigint> {
  const fields: SqliteValue[] = new Array(11).fill(null);
  let kind: string;
  switch (value.kind) {
    case "null": kind = "null"; break;
    case "bool": kind = "boolean"; fields[0] = boolCell(value.value); break;
    case "int": if (typeof value.value !== "bigint") throw new Error("PDF COS INTEGER requires owned bigint"); kind = "integer"; fields[1] = pdfInteger(value.value, 64, true); break;
    case "real": kind = "decimal"; fields[2] = boolCell(value.negative); fields[3] = value.coefficient; fields[4] = pdfInteger(value.scale); break;
    case "str": kind = "string"; fields[5] = await out.bytes(value.value); break;
    case "name": kind = "name"; fields[6] = value.value; break;
    case "ref": kind = "reference"; fields[7] = pdfInteger(value.num); fields[8] = pdfInteger(value.gen, 16); break;
    case "array": kind = "array"; out.checkRowsAdditional(value.value.length * 2); break;
    case "dict": kind = "dictionary"; out.checkRowsAdditional(value.value.length * 2); break;
    case "stream": kind = "stream"; out.checkRowsAdditional(value.dict.length * 2); fields[10] = await writePdfFilters(out, value.filters); fields[9] = await out.bytes(value.data); break;
    default: throw new Error("Unknown PDF COS variant");
  }
  return out.insert("pdf_cos_value", [kind, ...fields]);
}

/** 🌳️ Project a full COS ownership tree with an iterative borrowed traversal. */
export async function writePdfObject(out: PdfProjection, root: PdfObject): Promise<bigint> {
  const key = await writeShallow(out, root);
  const first = writeFrame(root, key);
  if (!first) return key;
  const active = new WeakSet<object>(); active.add(root);
  const pending: WriteFrame[] = [first];
  while (pending.length) {
    const frame = pending[pending.length - 1]!;
    if (frame.index >= (frame.entries?.length ?? frame.values!.length)) { active.delete(frame.owner); pending.pop(); continue; }
    const ordinal = frame.index++;
    const entry = frame.entries?.[ordinal];
    const value = entry?.value ?? frame.values![ordinal]!;
    if (active.has(value)) throw new Error("PDF COS cannot contain an ownership cycle");
    const child = await writeShallow(out, value);
    if (entry) await out.insert("pdf_cos_dictionary_entry", [frame.key, BigInt(ordinal), entry.key, child]); else await out.insert("pdf_cos_array_element", [frame.key, BigInt(ordinal), child]);
    const children = writeFrame(value, child);
    if (children) { active.add(value); pending.push(children); }
  }
  return key;
}
/** 📚️ Project ordered dictionary entries without synthesizing a serialization carrier. */
export async function writePdfDictionary(out: PdfProjection, entries: PdfDictEntry[]): Promise<bigint> { return writePdfObject(out, { kind: "dict", value: entries }); }

async function readShallow(reader: PdfReader, key: bigint): Promise<PdfObject> {
  const row = await reader.take("pdf_cos_value", key, 13);
  const kind = artifactSqliteText(row, 1);
  const allowed = kind === "boolean" ? [2] : kind === "integer" ? [3] : kind === "decimal" ? [4, 5, 6] : kind === "string" ? [7] : kind === "name" ? [8] : kind === "reference" ? [9, 10] : kind === "stream" ? [11, 12] : [];
  pdfNullExcept(row, 2, 13, allowed);
  switch (kind) {
    case "null": return { kind: "null" };
    case "boolean": return { kind: "bool", value: pdfBoolean(row, 2) };
    case "integer": return { kind: "int", value: pdfInteger(artifactSqliteInteger(row, 3), 64, true) };
    case "decimal": return { kind: "real", negative: pdfBoolean(row, 4), coefficient: await reader.text(row, 5), scale: pdfNumber(row, 6) };
    case "string": return { kind: "str", value: await reader.bytes(row, 7) };
    case "name": return { kind: "name", value: await reader.text(row, 8) };
    case "reference": return { kind: "ref", num: pdfNumber(row, 9), gen: pdfNumber(row, 10, 16) };
    case "array": return { kind: "array", value: [] };
    case "dictionary": return { kind: "dict", value: [] };
    case "stream": return { kind: "stream", dict: [], data: await reader.bytes(row, 11), filters: await readPdfFilters(reader, artifactSqliteInteger(row, 12)) };
    default: throw new Error("Unknown PDF COS stored variant");
  }
}
interface ReadFrame { readonly value: PdfObject; readonly members: readonly SqliteRow[]; readonly name: string | null; index: number }
async function readFrame(reader: PdfReader, value: PdfObject, key: bigint, name: string | null): Promise<ReadFrame> { const members = value.kind === "array" ? await reader.children("pdf_cos_array_element", 1, 2, key) : value.kind === "dict" || value.kind === "stream" ? await reader.children("pdf_cos_dictionary_entry", 1, 2, key) : []; return { value, members, name, index: 0 }; }

/** 🪜️ Restore a COS tree iteratively while detecting cycles and multiply owned entities. */
export async function readPdfObject(reader: PdfReader, key: bigint): Promise<PdfObject> {
  const pending: ReadFrame[] = [await readFrame(reader, await readShallow(reader, key), key, null)];
  while (pending.length) {
    const frame = pending[pending.length - 1]!;
    if (frame.index < frame.members.length) {
      const member = frame.members[frame.index++]!;
      const array = frame.value.kind === "array";
      const row = await reader.take(array ? "pdf_cos_array_element" : "pdf_cos_dictionary_entry", member.rowid, array ? 4 : 5);
      const name = array ? null : await reader.text(row, 3);
      const child = artifactSqliteInteger(row, array ? 3 : 4);
      pending.push(await readFrame(reader, await readShallow(reader, child), child, name));
    } else {
      pending.pop();
      const parent = pending[pending.length - 1];
      if (!parent) return frame.value;
      switch (parent.value.kind) {
        case "array": if (frame.name !== null) throw new Error("PDF array member cannot have a dictionary key"); parent.value.value.push(frame.value); break;
        case "dict": case "stream": { if (frame.name === null) throw new Error("PDF dictionary member requires a key"); const entries = parent.value.kind === "dict" ? parent.value.value : parent.value.dict; entries.push({ key: frame.name, value: frame.value }); break; }
        default: throw new Error("PDF scalar cannot own children");
      }
    }
  }
  throw new Error("PDF COS reconstruction stack is empty");
}
/** 📖️ Require a dictionary ownership relationship at the exact target entity. */
export async function readPdfDictionary(reader: PdfReader, key: bigint): Promise<PdfDictEntry[]> { const value = await readPdfObject(reader, key); if (value.kind !== "dict") throw new Error("PDF relationship requires a dictionary"); return value.value; }

/** 📤️ Export one canonical COS value under its owner's independently usable schema. */
export async function pdfObjectToSqliteDatabase(value: PdfObject, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> { const out = await PdfProjection.create(PDF_COS_SQLITE_SCHEMA, options); await writePdfObject(out, value); return out.finish(); }
/** 📥️ Import one COS root while rejecting hidden orphan entities. */
export async function pdfObjectFromSqliteDatabase(database: SqliteDatabase, root: bigint, options: ArtifactSqliteOptions = {}): Promise<PdfObject> { const reader = await PdfReader.create(database, PDF_COS_SQLITE_SCHEMA, options); const result = await readPdfObject(reader, root); await reader.finish(); return result; }
