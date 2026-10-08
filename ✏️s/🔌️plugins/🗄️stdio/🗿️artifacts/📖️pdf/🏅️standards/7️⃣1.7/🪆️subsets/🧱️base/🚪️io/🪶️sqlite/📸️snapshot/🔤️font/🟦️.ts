/** 🔤️ Handwritten font descriptors, intrinsic programs, Unicode and CID encodings. */
import type { PdfFont, PdfFontKind, PdfCharProc, PdfFontDescriptor, PdfFontProgram, PdfSimpleEncoding, PdfBaseEncoding, PdfToUnicode, PdfToUnicodeMapping, PdfCMap, PdfCodespaceRange, PdfCidMapping, PdfCidFont, PdfCidToGid, PdfCidWidthRun, PdfCidVerticalRun, Binary64 } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PdfProjection, PdfReader, pdfInteger, pdfNumber, pdfBoolean, type SqliteRow, type PdfCell } from "../🧩️entity/🟦️.ts";
import {writePdfArtifactReference,readPdfArtifactReference} from "../📦️artifact-reference/🟦️.ts";
import { writePdfDictionary, readPdfDictionary } from "../🧩️cos/🟦️.ts";
import { writePdfOperations, readPdfOperations, pdfContentNumberColumns } from "../🖋️content/🟦️.ts";
import { artifactSqliteInteger, artifactSqliteText } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { Ieee754Column } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";

/** 🔢️ Each native font binary64 field has its explicitly authored companion location. */
export function pdfFontNumberColumns(table: string): readonly Ieee754Column[] {
  switch (table) {
    case "pdf_font_descriptor": return [{ index: 3, width: 64 }, { index: 4, width: 64 }, { index: 5, width: 64 }, { index: 6, width: 64 }, { index: 7, width: 64 }, { index: 8, width: 64 }, { index: 9, width: 64 }, { index: 10, width: 64 }, { index: 11, width: 64 }, { index: 12, width: 64 }, { index: 13, width: 64 }, { index: 14, width: 64 }, { index: 15, width: 64 }, { index: 16, width: 64 }, { index: 17, width: 64 }, { index: 20, width: 64 }];
    case "pdf_cid_font": return [{ index: 7, width: 64 }, { index: 8, width: 64 }, { index: 9, width: 64 }];
    case "pdf_cid_width": case "pdf_font_width": return [{ index: 3, width: 64 }];
    case "pdf_cid_vertical_metric": return [{ index: 3, width: 64 }, { index: 4, width: 64 }, { index: 5, width: 64 }];
    case "pdf_font": return [{ index: 8, width: 64 }, { index: 9, width: 64 }, { index: 10, width: 64 }, { index: 11, width: 64 }, { index: 12, width: 64 }, { index: 13, width: 64 }, { index: 14, width: 64 }, { index: 15, width: 64 }, { index: 16, width: 64 }, { index: 17, width: 64 }];
    default: return pdfContentNumberColumns(table);
  }
}
const zero: Binary64 = { bits: 0n };
function optionalReal(reader: PdfReader, table: string, row: SqliteRow, column: number): Binary64 | null { return reader.isNull(table, row, column) ? null : reader.real(table, row, column); }

/** 📏️ Project every font descriptor field, including optional exact IEEE words. */
export async function writePdfFontDescriptor(out: PdfProjection, value: PdfFontDescriptor): Promise<bigint> {
  const extra = await writePdfDictionary(out, value.extra ?? []);
  const bbox = value.fontBbox ?? [zero, zero, zero, zero];
  if (bbox.length !== 4) throw new Error("PDF descriptor bounds require four components");
  return out.insert("pdf_font_descriptor", [value.fontName, pdfInteger(value.flags ?? 0), ...bbox, value.italicAngle ?? zero, value.ascent ?? zero, value.descent ?? zero, value.capHeight ?? zero, value.stemV ?? zero, value.stemH ?? null, value.xHeight ?? null, value.leading ?? null, value.avgWidth ?? null, value.maxWidth ?? null, value.missingWidth ?? null, value.fontFamily ?? null, value.fontStretch ?? null, value.fontWeight ?? null, value.charSet ?? null, extra]);
}
/** 🔤️ Reconstruct full font descriptor ownership without decoding native PDF bytes. */
export async function readPdfFontDescriptor(reader: PdfReader, key: bigint): Promise<PdfFontDescriptor> {
  const row = await reader.take("pdf_font_descriptor", key, 23);
  const number = (column: number): Binary64 => reader.real("pdf_font_descriptor", row, column);
  const optional = (column: number): Binary64 | null => optionalReal(reader, "pdf_font_descriptor", row, column);
  return { fontName: await reader.text(row, 1), flags: pdfNumber(row, 2), fontBbox: [number(3), number(4), number(5), number(6)], italicAngle: number(7), ascent: number(8), descent: number(9), capHeight: number(10), stemV: number(11), stemH: optional(12), xHeight: optional(13), leading: optional(14), avgWidth: optional(15), maxWidth: optional(16), missingWidth: optional(17), fontFamily: await reader.optionalText(row, 18), fontStretch: await reader.optionalText(row, 19), fontWeight: optional(20), charSet: await reader.optionalText(row, 21), extra: await readPdfDictionary(reader, artifactSqliteInteger(row, 22)) };
}

/** 🅰️ Preserve the semantic font kind and independently admitted native artifact reference. */
export async function writePdfFontProgram(out:PdfProjection,value:PdfFontProgram):Promise<bigint>{if(!["type1","trueType","cff","cidCff","openType"].includes(value.kind))throw new Error("Unknown PDF font program");return out.insert("pdf_font_program",[value.kind,await writePdfArtifactReference(out,value.reference)]);}
/** 📥️ Restore explicit font artifact custody without encoded program bodies. */
export async function readPdfFontProgram(reader:PdfReader,key:bigint):Promise<PdfFontProgram>{const row=await reader.take("pdf_font_program",key,3);const kind=artifactSqliteText(row,1);const reference=await readPdfArtifactReference(reader,artifactSqliteInteger(row,2));switch(kind){case "type1":case "trueType":case "cff":case "cidCff":case "openType":return {kind,reference};default:throw new Error("Unknown PDF font program");}}

function encodingBase(value: string | null): PdfBaseEncoding | null { switch (value) { case null: case "standard": case "winAnsi": case "macRoman": case "macExpert": return value; default: throw new Error("Unknown PDF base encoding"); } }
/** 🔡️ Preserve the optional base and every ordered difference, including duplicate codes. */
export async function writePdfEncoding(out: PdfProjection, value: PdfSimpleEncoding): Promise<bigint> {
  const key = await out.insert("pdf_font_encoding", [encodingBase(value.base ?? null)]);
  out.checkRowsAdditional(value.differences?.length ?? 0);
  for (const [ordinal, difference] of (value.differences ?? []).entries()) await out.insert("pdf_encoding_difference", [key, BigInt(ordinal), pdfInteger(difference.code), difference.glyph]);
  return key;
}
/** 📖️ Restore explicit encoding order and exact glyph names. */
export async function readPdfEncoding(reader: PdfReader, key: bigint): Promise<PdfSimpleEncoding> {
  const row = await reader.take("pdf_font_encoding", key, 2); const differences = [];
  for (const member of await reader.children("pdf_encoding_difference", 1, 2, key)) { const value = await reader.take("pdf_encoding_difference", member.rowid, 5); differences.push({ code: pdfNumber(value, 3), glyph: await reader.text(value, 4) }); }
  return { base: encodingBase(await reader.optionalText(row, 1)), differences };
}

/** 🈴️ Project typed Unicode mappings without encoding a CMap wire stream. */
export async function writePdfToUnicode(out: PdfProjection, value: PdfToUnicode): Promise<bigint> {
  const key = await out.insert("pdf_to_unicode", [pdfInteger(value.byteWidth)]);
  out.checkRowsAdditional(value.mappings?.length ?? 0);
  for (const [ordinal, mapping] of (value.mappings ?? []).entries()) {
    switch (mapping.kind) { case "char": await out.insert("pdf_unicode_mapping", [key, BigInt(ordinal), mapping.kind, pdfInteger(mapping.code), null, null, mapping.text]); break; case "range": await out.insert("pdf_unicode_mapping", [key, BigInt(ordinal), mapping.kind, null, pdfInteger(mapping.low), pdfInteger(mapping.high), mapping.text]); break; default: throw new Error("Unknown PDF Unicode mapping"); }
  }
  return key;
}
/** 📥️ Restore ordered Unicode mappings with exact native coordinate widths. */
export async function readPdfToUnicode(reader: PdfReader, key: bigint): Promise<PdfToUnicode> {
  const row = await reader.take("pdf_to_unicode", key, 2); const mappings: PdfToUnicodeMapping[] = [];
  for (const member of await reader.children("pdf_unicode_mapping", 1, 2, key)) {
    const value = await reader.take("pdf_unicode_mapping", member.rowid, 8); const kind = artifactSqliteText(value, 3); const text = await reader.text(value, 7);
    switch (kind) { case "char": reader.nullExcept("pdf_unicode_mapping", value, 4, 7, [4]); mappings.push({ kind, code: pdfNumber(value, 4), text }); break; case "range": reader.nullExcept("pdf_unicode_mapping", value, 4, 7, [5, 6]); mappings.push({ kind, low: pdfNumber(value, 5), high: pdfNumber(value, 6), text }); break; default: throw new Error("Unknown PDF Unicode mapping kind"); }
  }
  return { byteWidth: pdfNumber(row, 1), mappings };
}

/** 🗺️ Project explicit predefined or embedded CMap entities and typed CID relationships. */
export async function writePdfCMap(out: PdfProjection, value: PdfCMap): Promise<bigint> {
  if (value.kind === "predefined") return out.insert("pdf_cmap", [value.kind, value.name, null, null]);
  if (value.kind !== "embedded") throw new Error("Unknown PDF CMap variant");
  const cmap = value.cmap;
  const key = await out.insert("pdf_cmap", [value.kind, cmap.name, (cmap.vertical ?? false) ? 1n : 0n, cmap.useCmap ?? null]);
  out.checkRowsAdditional((cmap.codespace?.length ?? 0) + (cmap.mappings?.length ?? 0));
  for (const [ordinal, range] of (cmap.codespace ?? []).entries()) await out.insert("pdf_codespace_range", [key, BigInt(ordinal), pdfInteger(range.byteWidth), pdfInteger(range.low), pdfInteger(range.high)]);
  for (const [ordinal, mapping] of (cmap.mappings ?? []).entries()) { switch (mapping.kind) { case "char": await out.insert("pdf_cid_mapping", [key, BigInt(ordinal), mapping.kind, pdfInteger(mapping.code), null, null, pdfInteger(mapping.cid)]); break; case "range": await out.insert("pdf_cid_mapping", [key, BigInt(ordinal), mapping.kind, null, pdfInteger(mapping.low), pdfInteger(mapping.high), pdfInteger(mapping.cid)]); break; default: throw new Error("Unknown PDF CID mapping variant"); } }
  return key;
}
/** 📥️ Restore CMap ownership and reject predefined maps with embedded payloads. */
export async function readPdfCMap(reader: PdfReader, key: bigint): Promise<PdfCMap> {
  const row = await reader.take("pdf_cmap", key, 5); const kind = artifactSqliteText(row, 1); const name = await reader.text(row, 2);
  if (kind === "predefined") { reader.nullExcept("pdf_cmap", row, 3, 5, []); return { kind, name }; }
  if (kind !== "embedded") throw new Error("Unknown PDF CMap kind");
  const codespace: PdfCodespaceRange[] = []; const mappings: PdfCidMapping[] = [];
  for (const member of await reader.children("pdf_codespace_range", 1, 2, key)) { const range = await reader.take("pdf_codespace_range", member.rowid, 6); codespace.push({ byteWidth: pdfNumber(range, 3), low: pdfNumber(range, 4), high: pdfNumber(range, 5) }); }
  for (const member of await reader.children("pdf_cid_mapping", 1, 2, key)) { const mapping = await reader.take("pdf_cid_mapping", member.rowid, 8); const kind = artifactSqliteText(mapping, 3); const cid = pdfNumber(mapping, 7); switch (kind) { case "char": reader.nullExcept("pdf_cid_mapping", mapping, 4, 7, [4]); mappings.push({ kind, code: pdfNumber(mapping, 4), cid }); break; case "range": reader.nullExcept("pdf_cid_mapping", mapping, 4, 7, [5, 6]); mappings.push({ kind, low: pdfNumber(mapping, 5), high: pdfNumber(mapping, 6), cid }); break; default: throw new Error("Unknown PDF CID mapping kind"); } }
  return { kind, cmap: { name, vertical: pdfBoolean(row, 3), codespace, mappings, useCmap: await reader.optionalText(row, 4) } };
}

/** 🈶️ Preserve CID font entities, exact widths, paired vertical defaults and intrinsic glyph mappings. */
export async function writePdfCidFont(out: PdfProjection, font: PdfCidFont): Promise<bigint> {
  const descriptor = await writePdfFontDescriptor(out, font.descriptor);
  const program = font.program == null ? null : await writePdfFontProgram(out, font.program);
  const extra = await writePdfDictionary(out, font.extra ?? []);
  const system = font.systemInfo ?? { registry: "Adobe", ordering: "Identity", supplement: 0 };
  const vertical = font.defaultVertical ?? null;
  if (vertical !== null && vertical.length !== 2) throw new Error("PDF vertical defaults require both metric components");
  let gidKind: string | null = null; let gidData: bigint | null = null;
  if (font.cidToGid != null) { switch (font.cidToGid.kind) { case "identity": gidKind = "identity"; break; case "map": gidKind = "map"; gidData = pdfInteger(font.cidToGid.glyphs.length); break; default: throw new Error("Unknown PDF CID to glyph mapping"); } }
  const key = await out.insert("pdf_cid_font", [font.trueType ? 1n : 0n, font.baseFont, system.registry, system.ordering, pdfInteger(system.supplement), descriptor, font.defaultWidth ?? { bits: 0x408f400000000000n }, vertical?.[0] ?? null, vertical?.[1] ?? null, gidKind, gidData, program, extra]);
  out.checkRowsAdditional((font.widths?.length ?? 0) + (font.verticalMetrics?.length ?? 0));
  if(font.cidToGid?.kind==="map"){out.checkRowsAdditional(font.cidToGid.glyphs.length);for(const[ordinal,glyph]of font.cidToGid.glyphs.entries())await out.insert("pdf_cid_glyph",[key,BigInt(ordinal),pdfInteger(glyph,16)]);}
  for (const [ordinal, run] of (font.widths ?? []).entries()) {
    out.checkRowsAdditional(run.widths.length + 1);
    const runKey = await out.insert("pdf_cid_width_run", [key, BigInt(ordinal), pdfInteger(run.startCid)]);
    for (const [ordinal, width] of run.widths.entries()) await out.insert("pdf_cid_width", [runKey, BigInt(ordinal), width]);
  }
  for (const [ordinal, run] of (font.verticalMetrics ?? []).entries()) {
    out.checkRowsAdditional(run.metrics.length + 1);
    const runKey = await out.insert("pdf_cid_vertical_run", [key, BigInt(ordinal), pdfInteger(run.startCid)]);
    for (const [ordinal, metric] of run.metrics.entries()) { if (metric.length !== 3) throw new Error("PDF vertical metrics require three components"); await out.insert("pdf_cid_vertical_metric", [runKey, BigInt(ordinal), metric[0], metric[1], metric[2]]); }
  }
  return key;
}

/** 📥️ Reconstruct all ordered CID font relationships and reject mismatched optional payloads. */
export async function readPdfCidFont(reader: PdfReader, key: bigint): Promise<PdfCidFont> {
  const row = await reader.take("pdf_cid_font", key, 14);
  const y = optionalReal(reader, "pdf_cid_font", row, 8); const width = optionalReal(reader, "pdf_cid_font", row, 9);
  if ((y === null) !== (width === null)) throw new Error("PDF vertical defaults require both metric components");
  const gidKind = await reader.optionalText(row, 10); let cidToGid: PdfCidToGid | null = null;
  switch (gidKind) { case null: case "identity": reader.nullExcept("pdf_cid_font", row, 11, 12, []); cidToGid = gidKind === null ? null : { kind: "identity" }; break; case "map": {const glyphs:number[]=[];for(const child of await reader.children("pdf_cid_glyph",1,2,key)){const value=await reader.take("pdf_cid_glyph",child.rowid,4);glyphs.push(pdfNumber(value,3,16));}if(glyphs.length!==pdfNumber(row,11))throw new Error("PDF CID glyph count mismatch");cidToGid={kind:"map",glyphs};} break; default: throw new Error("Unknown PDF CID to glyph mapping"); }
  const widths: PdfCidWidthRun[] = []; const verticalMetrics: PdfCidVerticalRun[] = [];
  for (const child of await reader.children("pdf_cid_width_run", 1, 2, key)) {
    const run = await reader.take("pdf_cid_width_run", child.rowid, 4); const values: Binary64[] = [];
    for (const child of await reader.children("pdf_cid_width", 1, 2, run.rowid)) { const row = await reader.take("pdf_cid_width", child.rowid, 4); values.push(reader.real("pdf_cid_width", row, 3)); }
    widths.push({ startCid: pdfNumber(run, 3), widths: values });
  }
  for (const child of await reader.children("pdf_cid_vertical_run", 1, 2, key)) {
    const run = await reader.take("pdf_cid_vertical_run", child.rowid, 4); const metrics: [Binary64, Binary64, Binary64][] = [];
    for (const child of await reader.children("pdf_cid_vertical_metric", 1, 2, run.rowid)) { const row = await reader.take("pdf_cid_vertical_metric", child.rowid, 6); metrics.push([reader.real("pdf_cid_vertical_metric", row, 3), reader.real("pdf_cid_vertical_metric", row, 4), reader.real("pdf_cid_vertical_metric", row, 5)]); }
    verticalMetrics.push({ startCid: pdfNumber(run, 3), metrics });
  }
  const program = row.values[12] === null ? null : await readPdfFontProgram(reader, artifactSqliteInteger(row, 12));
  return { trueType: pdfBoolean(row, 1), baseFont: await reader.text(row, 2), systemInfo: { registry: await reader.text(row, 3), ordering: await reader.text(row, 4), supplement: pdfNumber(row, 5) }, descriptor: await readPdfFontDescriptor(reader, artifactSqliteInteger(row, 6)), defaultWidth: reader.real("pdf_cid_font", row, 7), widths, defaultVertical: y === null ? null : [y, width!], verticalMetrics, cidToGid, program, extra: await readPdfDictionary(reader, artifactSqliteInteger(row, 13)) };
}

/** 🔤️ Preserve all four native font variants and their actual typed relationships. */
export async function writePdfFont(out: PdfProjection,font: PdfFont): Promise<bigint> {
  const fields: PdfCell[]=new Array(17).fill(null); const value=font.kind;
  switch (value.kind) {
    case "type1": case "trueType": {
      fields[0]=value.baseFont; fields[1]=await writePdfEncoding(out,value.encoding); fields[2]=pdfInteger(value.firstChar);
      fields[3]=value.descriptor == null ? null : await writePdfFontDescriptor(out,value.descriptor);
      fields[4]=value.program == null ? null : await writePdfFontProgram(out,value.program); break;
    }
    case "type3": {
      if (value.fontMatrix.length!==6 || value.fontBbox.length!==4) throw new Error("PDF Type3 matrix and bounds require six and four components");
      fields[1]=await writePdfEncoding(out,value.encoding); fields[2]=pdfInteger(value.firstChar);
      fields[3]=value.descriptor == null ? null : await writePdfFontDescriptor(out,value.descriptor);
      fields[5]=value.fontMatrix[0];fields[6]=value.fontMatrix[1];fields[7]=value.fontMatrix[2];fields[8]=value.fontMatrix[3];fields[9]=value.fontMatrix[4];fields[10]=value.fontMatrix[5];
      fields[11]=value.fontBbox[0];fields[12]=value.fontBbox[1];fields[13]=value.fontBbox[2];fields[14]=value.fontBbox[3]; break;
    }
    case "type0": fields[0]=value.baseFont;fields[15]=await writePdfCMap(out,value.cmap);fields[16]=await writePdfCidFont(out,value.descendant);break;
    default: throw new Error("Unknown PDF font variant");
  }
  const unicode=font.toUnicode == null ? null : await writePdfToUnicode(out,font.toUnicode);const extra=await writePdfDictionary(out,font.extra ?? []);
  const key=await out.insert("pdf_font",[font.id,value.kind,...fields,unicode,extra]);
  if (value.kind!=="type0") { out.checkRowsAdditional(value.widths.length); for (const [ordinal,width] of value.widths.entries()) await out.insert("pdf_font_width",[key,BigInt(ordinal),width]); }
  if (value.kind==="type3") { out.checkRowsAdditional(value.charProcs.length); for (const [ordinal,procedure] of value.charProcs.entries()) await out.insert("pdf_char_proc",[key,BigInt(ordinal),procedure.name,await writePdfOperations(out,procedure.content)]); }
  return key;
}

/** 📥️ Reconstruct font ownership while rejecting unrelated variant scalars and IEEE companions. */
export async function readPdfFont(reader: PdfReader,key: bigint): Promise<PdfFont> {
  const row=await reader.take("pdf_font",key,22);const kind=artifactSqliteText(row,2);let present: readonly number[];
  switch (kind) { case "type1": case "trueType": present=[3,4,5,6,7];break;case "type3":present=[4,5,6,8,9,10,11,12,13,14,15,16,17];break;case "type0":present=[3,18,19];break;default:throw new Error("Unknown PDF font variant"); }
  reader.nullExcept("pdf_font",row,3,20,present);const widths: Binary64[]=[];
  if (kind!=="type0") for (const child of await reader.children("pdf_font_width",1,2,key)) { const row=await reader.take("pdf_font_width",child.rowid,4);widths.push(reader.real("pdf_font_width",row,3)); }
  const descriptor=async (): Promise<PdfFontDescriptor|null> => row.values[6]===null ? null : readPdfFontDescriptor(reader,artifactSqliteInteger(row,6));
  let value: PdfFontKind;
  switch (kind) {
    case "type1":case "trueType": value={kind,baseFont:await reader.text(row,3),encoding:await readPdfEncoding(reader,artifactSqliteInteger(row,4)),firstChar:pdfNumber(row,5),widths,descriptor:await descriptor(),program:row.values[7]===null ? null : await readPdfFontProgram(reader,artifactSqliteInteger(row,7))};break;
    case "type3": {
      const charProcs: PdfCharProc[]=[];
      for (const child of await reader.children("pdf_char_proc",1,2,key)) { const row=await reader.take("pdf_char_proc",child.rowid,5);charProcs.push({name:await reader.text(row,3),content:await readPdfOperations(reader,artifactSqliteInteger(row,4))}); }
      value={kind,fontMatrix:[reader.real("pdf_font",row,8),reader.real("pdf_font",row,9),reader.real("pdf_font",row,10),reader.real("pdf_font",row,11),reader.real("pdf_font",row,12),reader.real("pdf_font",row,13)],fontBbox:[reader.real("pdf_font",row,14),reader.real("pdf_font",row,15),reader.real("pdf_font",row,16),reader.real("pdf_font",row,17)],encoding:await readPdfEncoding(reader,artifactSqliteInteger(row,4)),firstChar:pdfNumber(row,5),widths,charProcs,descriptor:await descriptor()};break;
    }
    case "type0":value={kind,baseFont:await reader.text(row,3),cmap:await readPdfCMap(reader,artifactSqliteInteger(row,18)),descendant:await readPdfCidFont(reader,artifactSqliteInteger(row,19))};break;
    default:throw new Error("Unknown PDF font variant");
  }
  return {id:await reader.text(row,1),kind:value,toUnicode:row.values[20]===null ? null : await readPdfToUnicode(reader,artifactSqliteInteger(row,20)),extra:await readPdfDictionary(reader,artifactSqliteInteger(row,21))};
}
