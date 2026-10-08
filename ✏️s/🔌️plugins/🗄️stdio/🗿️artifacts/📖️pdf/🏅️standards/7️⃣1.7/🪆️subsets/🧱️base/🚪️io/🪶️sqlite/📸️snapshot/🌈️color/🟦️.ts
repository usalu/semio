import { writePdfArtifactReference,readPdfArtifactReference,PDF_ARTIFACT_REFERENCE_SQLITE_SCHEMA } from "../📦️artifact-reference/🟦️.ts";
/** 🌈️ Handwritten PDF functions and twelve color spaces with exact owned numeric words. */
import type { PdfFunction, PdfColorSpace, Binary64 } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PdfProjection, PdfReader, pdfInteger, pdfNumber, pdfBoolean, type PdfCell, type ArtifactSqliteOptions, type SqliteDatabase, type SqliteRow } from "../🧩️entity/🟦️.ts";
import { PDF_COS_SQLITE_SCHEMA, writePdfDictionary, readPdfDictionary } from "../🧩️cos/🟦️.ts";
import { artifactSqliteInteger, artifactSqliteText, artifactSqliteCheckpoint } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { Ieee754Column } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";

/** 🏛️ Handwritten function domains, ordered samples and color alternate relationships. */
export const PDF_COLOR_SQLITE_SCHEMA = PDF_COS_SQLITE_SCHEMA + PDF_ARTIFACT_REFERENCE_SQLITE_SCHEMA + `
CREATE TABLE pdf_function (id INTEGER PRIMARY KEY, kind TEXT NOT NULL CHECK (kind IN ('sampled','exponential','stitching','postScript','array')), range_present INTEGER CHECK (range_present IN (0,1)), encode_present INTEGER CHECK (encode_present IN (0,1)), decode_present INTEGER CHECK (decode_present IN (0,1)), bits_per_sample INTEGER, interpolation_order INTEGER, exponent REAL, sample_count INTEGER, postscript_source TEXT, exponent_bits INTEGER, exponent_class TEXT CHECK (exponent_class IN ('finite','positiveInfinity','negativeInfinity','nan')));
CREATE TABLE pdf_function_real (id INTEGER PRIMARY KEY, function_id INTEGER NOT NULL REFERENCES pdf_function(id), role TEXT NOT NULL CHECK (role IN ('domain','range','c0','c1','bounds','encode','decode')), ordinal INTEGER NOT NULL, value REAL, value_bits INTEGER, value_class TEXT CHECK (value_class IN ('finite','positiveInfinity','negativeInfinity','nan')));
CREATE TABLE pdf_function_sample (id INTEGER PRIMARY KEY, function_id INTEGER NOT NULL REFERENCES pdf_function(id), ordinal INTEGER NOT NULL, value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 4294967295));
CREATE TABLE pdf_function_size (id INTEGER PRIMARY KEY, function_id INTEGER NOT NULL REFERENCES pdf_function(id), ordinal INTEGER NOT NULL, value INTEGER NOT NULL);
CREATE TABLE pdf_function_child (id INTEGER PRIMARY KEY, function_id INTEGER NOT NULL REFERENCES pdf_function(id), ordinal INTEGER NOT NULL, child_id INTEGER NOT NULL REFERENCES pdf_function(id));
CREATE TABLE pdf_color_space (id INTEGER PRIMARY KEY, kind TEXT NOT NULL CHECK (kind IN ('deviceGray','deviceRgb','deviceCmyk','calGray','calRgb','lab','iccBased','indexed','separation','deviceN','pattern','named')), components INTEGER, profile_id INTEGER REFERENCES pdf_artifact_reference(id), hival INTEGER, palette BLOB, name TEXT, alternate_id INTEGER REFERENCES pdf_color_space(id), tint_transform_id INTEGER REFERENCES pdf_function(id), attributes_id INTEGER REFERENCES pdf_cos_value(id), range_present INTEGER CHECK (range_present IN (0,1)), black_present INTEGER CHECK (black_present IN (0,1)), gamma_present INTEGER CHECK (gamma_present IN (0,1)), matrix_present INTEGER CHECK (matrix_present IN (0,1)));
CREATE TABLE pdf_color_real (id INTEGER PRIMARY KEY, color_space_id INTEGER NOT NULL REFERENCES pdf_color_space(id), role TEXT NOT NULL CHECK (role IN ('whitePoint','blackPoint','gamma','matrix','range')), ordinal INTEGER NOT NULL, value REAL, value_bits INTEGER, value_class TEXT CHECK (value_class IN ('finite','positiveInfinity','negativeInfinity','nan')));
CREATE TABLE pdf_color_name (id INTEGER PRIMARY KEY, color_space_id INTEGER NOT NULL REFERENCES pdf_color_space(id), ordinal INTEGER NOT NULL, name TEXT NOT NULL);
`;
/** 🔢️ Exact scalar companion locations authored for function and color entities. */
export function pdfColorNumberColumns(table: string): readonly Ieee754Column[] { switch (table) { case "pdf_function": return [{ index: 7, width: 64 }]; case "pdf_function_real": case "pdf_color_real": return [{ index: 4, width: 64 }]; default: return []; } }

/** 🎚️ Store ordered numeric components under an explicit semantic role. */
export async function writePdfReals(out: PdfProjection, table: string, owner: bigint, role: string, values: readonly Binary64[]): Promise<void> { out.checkRowsAdditional(values.length); for (const [ordinal, value] of values.entries()) await out.insert(table, [owner, role, BigInt(ordinal), value]); }
/** 📐️ Restore exact ordered numeric words without number canonicalization. */
export async function readPdfReals(reader: PdfReader, table: string, owner: bigint, role: string): Promise<Binary64[]> { const result: Binary64[] = []; for (const member of await reader.children(table, 1, 3, owner, 2, role)) { const row = await reader.take(table, member.rowid, 5); result.push(reader.real(table, row, 4)); } return result; }
function optional(row: SqliteRow, column: number): bigint | null { return row.values[column] === null ? null : artifactSqliteInteger(row, column); }
function present(value: unknown): bigint { return value == null ? 0n : 1n; }
function children(value: PdfFunction): readonly PdfFunction[] { return value.kind === "array" || value.kind === "stitching" ? value.functions : []; }

async function writeFunctionShallow(out: PdfProjection, value: PdfFunction): Promise<bigint> {
  const fields: PdfCell[] = new Array(8).fill(null);
  switch (value.kind) {
    case "sampled": fields[1] = present(value.encode); fields[2] = present(value.decode); fields[3] = pdfInteger(value.bitsPerSample); fields[4] = value.order == null ? null : pdfInteger(value.order); fields[6] = pdfInteger(value.samples.length); break;
    case "exponential": fields[0] = present(value.range); fields[5] = value.n; break;
    case "stitching": fields[0] = present(value.range); break;
    case "postScript": fields[7] = value.code; break;
    case "array": break;
    default: throw new Error("Unknown PDF function variant");
  }
  const key = await out.insert("pdf_function", [value.kind, ...fields]);
  if (value.kind !== "array") await writePdfReals(out, "pdf_function_real", key, "domain", value.domain);
  switch (value.kind) {
    case "sampled":
      await writePdfReals(out, "pdf_function_real", key, "range", value.range);
      out.checkRowsAdditional(value.size.length);
      out.checkRowsAdditional(value.samples.length); for (const [ordinal, sample] of value.samples.entries()) { if (!Number.isInteger(sample) || sample < 0 || sample > 4294967295) throw new Error("PDF function sample exceeds logical word width"); await out.insert("pdf_function_sample", [key, BigInt(ordinal), pdfInteger(sample)]); }
      for (const [ordinal, size] of value.size.entries()) await out.insert("pdf_function_size", [key, BigInt(ordinal), pdfInteger(size)]);
      if (value.encode != null) await writePdfReals(out, "pdf_function_real", key, "encode", value.encode);
      if (value.decode != null) await writePdfReals(out, "pdf_function_real", key, "decode", value.decode); break;
    case "exponential":
      if (value.range != null) await writePdfReals(out, "pdf_function_real", key, "range", value.range);
      await writePdfReals(out, "pdf_function_real", key, "c0", value.c0); await writePdfReals(out, "pdf_function_real", key, "c1", value.c1); break;
    case "stitching":
      if (value.range != null) await writePdfReals(out, "pdf_function_real", key, "range", value.range);
      await writePdfReals(out, "pdf_function_real", key, "bounds", value.bounds); await writePdfReals(out, "pdf_function_real", key, "encode", value.encode); break;
    case "postScript": await writePdfReals(out, "pdf_function_real", key, "range", value.range); break;
    case "array": break;
  }
  return key;
}

/** 🌳️ Project nested function arrays and stitching nodes with bounded iterative traversal. */
export async function writePdfFunction(out: PdfProjection, root: PdfFunction): Promise<bigint> {
  const key = await writeFunctionShallow(out, root);
  const active = new WeakSet<object>(); active.add(root);
  const pending = [{ value: root, key, index: 0 }];
  while (pending.length) {
    const frame = pending[pending.length - 1]!; const values = children(frame.value);
    if (frame.index === values.length) { active.delete(frame.value); pending.pop(); continue; }
    out.checkRowsAdditional(2);
    const ordinal = frame.index++; const child = values[ordinal]!;
    if (active.has(child)) throw new Error("PDF function cannot contain an ownership cycle");
    const childKey = await writeFunctionShallow(out, child);
    await out.insert("pdf_function_child", [frame.key, BigInt(ordinal), childKey]);
    active.add(child); pending.push({ value: child, key: childKey, index: 0 });
  }
  return key;
}

async function readFunctionShallow(reader: PdfReader, key: bigint): Promise<PdfFunction> {
  const row = await reader.take("pdf_function", key, 10);
  const kind = artifactSqliteText(row, 1);
  reader.nullExcept("pdf_function", row, 2, 10, kind === "sampled" ? [3, 4, 5, 6, 8] : kind === "exponential" ? [2, 7] : kind === "stitching" ? [2] : kind === "postScript" ? [9] : []);
  const reals = (role: string): Promise<Binary64[]> => readPdfReals(reader, "pdf_function_real", key, role);
  switch (kind) {
    case "sampled": { const samples: number[] = []; for (const member of await reader.children("pdf_function_sample", 1, 2, key)) { const value = await reader.take("pdf_function_sample", member.rowid, 4); const word = pdfNumber(value, 3); if (!Number.isInteger(word) || word < 0 || word > 4294967295) throw new Error("PDF function sample exceeds logical word width"); samples.push(word); } if (samples.length !== pdfNumber(row, 8)) throw new Error("PDF function sample count differs"); const size: number[] = []; for (const member of await reader.children("pdf_function_size", 1, 2, key)) { const value = await reader.take("pdf_function_size", member.rowid, 4); size.push(pdfNumber(value, 3)); } return { kind, domain: await reals("domain"), range: await reals("range"), size, bitsPerSample: pdfNumber(row, 5), order: row.values[6] === null ? null : pdfNumber(row, 6), encode: pdfBoolean(row, 3) ? await reals("encode") : null, decode: pdfBoolean(row, 4) ? await reals("decode") : null, samples }; }
    case "exponential": return { kind, domain: await reals("domain"), range: pdfBoolean(row, 2) ? await reals("range") : null, c0: await reals("c0"), c1: await reals("c1"), n: reader.real("pdf_function", row, 7) };
    case "stitching": return { kind, domain: await reals("domain"), range: pdfBoolean(row, 2) ? await reals("range") : null, functions: [], bounds: await reals("bounds"), encode: await reals("encode") };
    case "postScript": return { kind, domain: await reals("domain"), range: await reals("range"), code: await reader.text(row, 9) };
    case "array": return { kind, functions: [] };
    default: throw new Error("Unknown PDF stored function variant");
  }
}

/** 🪜️ Reconstruct function trees while rejecting cycles, repeated ownership and scalar children. */
export async function readPdfFunction(reader: PdfReader, key: bigint): Promise<PdfFunction> {
  const pending = [{ value: await readFunctionShallow(reader, key), children: await reader.children("pdf_function_child", 1, 2, key), index: 0 }];
  while (pending.length) {
    const frame = pending[pending.length - 1]!;
    if (frame.index < frame.children.length) {
      if (frame.value.kind !== "array" && frame.value.kind !== "stitching") throw new Error("PDF scalar function cannot own children");
      const row = await reader.take("pdf_function_child", frame.children[frame.index++]!.rowid, 4); const child = artifactSqliteInteger(row, 3);
      pending.push({ value: await readFunctionShallow(reader, child), children: await reader.children("pdf_function_child", 1, 2, child), index: 0 });
    } else {
      pending.pop(); const parent = pending[pending.length - 1]; if (!parent) return frame.value;
      if (parent.value.kind !== "array" && parent.value.kind !== "stitching") throw new Error("PDF scalar function cannot own children"); parent.value.functions.push(frame.value);
    }
  }
  throw new Error("PDF function reconstruction stack is empty");
}

function alternate(value: PdfColorSpace): PdfColorSpace | null { switch (value.kind) { case "iccBased": case "separation": case "deviceN": return value.alternate ?? null; case "indexed": return value.base; case "pattern": return value.base ?? null; default: return null; } }
async function writeColorShallow(out: PdfProjection, value: PdfColorSpace, child: bigint | null): Promise<bigint> {
  const fields: PdfCell[] = new Array(12).fill(null);
  switch (value.kind) {
    case "calGray": fields[9] = present(value.blackPoint); fields[10] = present(value.gamma); break;
    case "calRgb": fields[9] = present(value.blackPoint); fields[10] = present(value.gamma); fields[11] = present(value.matrix); break;
    case "lab": fields[9] = present(value.blackPoint); fields[8] = present(value.range); break;
    case "iccBased": fields[0] = pdfInteger(value.components); fields[1] = await writePdfArtifactReference(out,value.profile); fields[5] = child; fields[8] = present(value.range); break;
    case "indexed": if (child === null) throw new Error("PDF indexed color has no base"); fields[5] = child; fields[2] = pdfInteger(value.hival); fields[3] = await out.bytes(value.palette); break;
    case "separation": if (child === null) throw new Error("PDF color has no alternate"); fields[4] = value.name; fields[5] = child; fields[6] = await writePdfFunction(out, value.tintTransform); break;
    case "deviceN": if (child === null) throw new Error("PDF color has no alternate"); fields[5] = child; fields[6] = await writePdfFunction(out, value.tintTransform); fields[7] = value.attributes == null ? null : await writePdfDictionary(out, value.attributes); break;
    case "pattern": fields[5] = child; break;
    case "named": fields[4] = value.name; break;
    case "deviceGray": case "deviceRgb": case "deviceCmyk": break;
    default: throw new Error("Unknown PDF color variant");
  }
  const key = await out.insert("pdf_color_space", [value.kind, ...fields]);
  switch (value.kind) {
    case "calGray": case "calRgb": case "lab":
      await writePdfReals(out, "pdf_color_real", key, "whitePoint", value.whitePoint);
      if (value.blackPoint != null) await writePdfReals(out, "pdf_color_real", key, "blackPoint", value.blackPoint);
      if (value.kind === "calGray") { if (value.gamma != null) await writePdfReals(out, "pdf_color_real", key, "gamma", [value.gamma]); }
      if (value.kind === "calRgb") { if (value.gamma != null) await writePdfReals(out, "pdf_color_real", key, "gamma", value.gamma); if (value.matrix != null) await writePdfReals(out, "pdf_color_real", key, "matrix", value.matrix); }
      if (value.kind === "lab" && value.range != null) await writePdfReals(out, "pdf_color_real", key, "range", value.range); break;
    case "iccBased": if (value.range != null) await writePdfReals(out, "pdf_color_real", key, "range", value.range); break;
    case "deviceN": out.checkRowsAdditional(value.names.length); for (const [ordinal, name] of value.names.entries()) await out.insert("pdf_color_name", [key, BigInt(ordinal), name]); break;
    default: break;
  }
  return key;
}

/** 🪜️ Project explicit alternate/base chains iteratively with cycle and preallocation checks. */
export async function writePdfColorSpace(out: PdfProjection, root: PdfColorSpace): Promise<bigint> {
  const pending: PdfColorSpace[] = []; const active = new WeakSet<object>();
  let value: PdfColorSpace | null = root;
  while (value) { if (active.has(value)) throw new Error("PDF color cannot contain an alternate cycle"); out.checkRowsAdditional(pending.length + 1); active.add(value); pending.push(value); if (pending.length % 256 === 0) await artifactSqliteCheckpoint(out.options, "projectSnapshot", pending.length, 0); value = alternate(value); }
  let child: bigint | null = null; while (pending.length) child = await writeColorShallow(out, pending.pop()!, child);
  if (child === null) throw new Error("PDF color projection stack is empty"); return child;
}

/** 🎨️ Require fixed color component counts under the authored PDF field contract. */
export function pdfThree(values: Binary64[]): [Binary64, Binary64, Binary64] { if (values.length !== 3) throw new Error("PDF color requires three components"); return [values[0]!, values[1]!, values[2]!]; }
export function pdfFour(values: Binary64[]): [Binary64, Binary64, Binary64, Binary64] { if (values.length !== 4) throw new Error("PDF color requires four components"); return [values[0]!, values[1]!, values[2]!, values[3]!]; }
export function pdfNine(values: Binary64[]): [Binary64, Binary64, Binary64, Binary64, Binary64, Binary64, Binary64, Binary64, Binary64] { if (values.length !== 9) throw new Error("PDF color requires nine components"); return [values[0]!, values[1]!, values[2]!, values[3]!, values[4]!, values[5]!, values[6]!, values[7]!, values[8]!]; }

async function readColorShallow(reader: PdfReader, row: SqliteRow, child: PdfColorSpace | null): Promise<PdfColorSpace> {
  const key = row.rowid; const kind = artifactSqliteText(row, 1);
  reader.nullExcept("pdf_color_space", row, 2, 14, kind === "calGray" ? [11, 12] : kind === "calRgb" ? [11, 12, 13] : kind === "lab" ? [10, 11] : kind === "iccBased" ? [2, 3, 7, 10] : kind === "indexed" ? [4, 5, 7] : kind === "separation" ? [6, 7, 8] : kind === "deviceN" ? [7, 8, 9] : kind === "pattern" ? [7] : kind === "named" ? [6] : []);
  const reals = (role: string): Promise<Binary64[]> => readPdfReals(reader, "pdf_color_real", key, role);
  switch (kind) {
    case "deviceGray": case "deviceRgb": case "deviceCmyk": return { kind };
    case "calGray": { const gamma = pdfBoolean(row, 12) ? await reals("gamma") : null; if (gamma && gamma.length !== 1) throw new Error("PDF gray gamma requires one component"); return { kind, whitePoint: pdfThree(await reals("whitePoint")), blackPoint: pdfBoolean(row, 11) ? pdfThree(await reals("blackPoint")) : null, gamma: gamma ? gamma[0]! : null }; }
    case "calRgb": return { kind, whitePoint: pdfThree(await reals("whitePoint")), blackPoint: pdfBoolean(row, 11) ? pdfThree(await reals("blackPoint")) : null, gamma: pdfBoolean(row, 12) ? pdfThree(await reals("gamma")) : null, matrix: pdfBoolean(row, 13) ? pdfNine(await reals("matrix")) : null };
    case "lab": return { kind, whitePoint: pdfThree(await reals("whitePoint")), blackPoint: pdfBoolean(row, 11) ? pdfThree(await reals("blackPoint")) : null, range: pdfBoolean(row, 10) ? pdfFour(await reals("range")) : null };
    case "iccBased": return { kind, components: pdfNumber(row, 2), profile: await readPdfArtifactReference(reader,artifactSqliteInteger(row,3)), alternate: child, range: pdfBoolean(row, 10) ? await reals("range") : null };
    case "indexed": if (!child) throw new Error("PDF indexed color has no base"); return { kind, base: child, hival: pdfNumber(row, 4), palette: await reader.bytes(row, 5) };
    case "separation": if (!child) throw new Error("PDF color has no alternate"); return { kind, name: await reader.text(row, 6), alternate: child, tintTransform: await readPdfFunction(reader, artifactSqliteInteger(row, 8)) };
    case "deviceN": { if (!child) throw new Error("PDF color has no alternate"); const names: string[] = []; for (const member of await reader.children("pdf_color_name", 1, 2, key)) { const name = await reader.take("pdf_color_name", member.rowid, 4); names.push(await reader.text(name, 3)); } return { kind, names, alternate: child, tintTransform: await readPdfFunction(reader, artifactSqliteInteger(row, 8)), attributes: row.values[9] === null ? null : await readPdfDictionary(reader, artifactSqliteInteger(row, 9)) }; }
    case "pattern": return { kind, base: child };
    case "named": return { kind, name: await reader.text(row, 6) };
    default: throw new Error("Unknown PDF stored color variant");
  }
}

/** 🌈️ Restore alternate/base chains with exact native ownership and component constraints. */
export async function readPdfColorSpace(reader: PdfReader, root: bigint): Promise<PdfColorSpace> {
  const pending: SqliteRow[] = []; let key: bigint | null = root;
  while (key !== null) { const row = await reader.take("pdf_color_space", key, 14); pending.push(row); key = optional(row, 7); }
  let child: PdfColorSpace | null = null; while (pending.length) child = await readColorShallow(reader, pending.pop()!, child);
  if (!child) throw new Error("PDF color reconstruction stack is empty"); return child;
}

/** 📤️ Export the canonical function ownership root for independent domain verification. */
export async function pdfFunctionToSqliteDatabase(value: PdfFunction, options: ArtifactSqliteOptions = {}): Promise<{ database: SqliteDatabase; root: bigint }> { const out = await PdfProjection.create(PDF_COLOR_SQLITE_SCHEMA, options, pdfColorNumberColumns); const root = await writePdfFunction(out, value); return { database: await out.finish(), root }; }
/** 📥️ Restore one function domain and reject unused rows. */
export async function pdfFunctionFromSqliteDatabase(database: SqliteDatabase, root: bigint, options: ArtifactSqliteOptions = {}): Promise<PdfFunction> { const reader = await PdfReader.create(database, PDF_COLOR_SQLITE_SCHEMA, options, pdfColorNumberColumns); const value = await readPdfFunction(reader, root); await reader.finish(); return value; }
/** 📤️ Export the canonical color ownership root without embedding its native serialization. */
export async function pdfColorSpaceToSqliteDatabase(value: PdfColorSpace, options: ArtifactSqliteOptions = {}): Promise<{ database: SqliteDatabase; root: bigint }> { const out = await PdfProjection.create(PDF_COLOR_SQLITE_SCHEMA, options, pdfColorNumberColumns); const root = await writePdfColorSpace(out, value); return { database: await out.finish(), root }; }
/** 📥️ Restore one full color graph with exact words and no orphan acceptance. */
export async function pdfColorSpaceFromSqliteDatabase(database: SqliteDatabase, root: bigint, options: ArtifactSqliteOptions = {}): Promise<PdfColorSpace> { const reader = await PdfReader.create(database, PDF_COLOR_SQLITE_SCHEMA, options, pdfColorNumberColumns); const value = await readPdfColorSpace(reader, root); await reader.finish(); return value; }
