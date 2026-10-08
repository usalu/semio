import { writePdfArtifactReference,readPdfArtifactReference } from "../📦️artifact-reference/🟦️.ts";
/** 🪪️ Normalized SQL ownership for explicit semantic stream roles and their graph identities. */
import type { PdfAdmittedStreamRole, PdfGraphIdentity, PdfGraphPath, PdfStreamRoleValue } from "../../../../🧬️schema/🪪️stream-roles/🟦️.ts";
import { PdfProjection, PdfReader, pdfInteger, pdfNumber, type PdfCell } from "../🧩️entity/🟦️.ts";
import { writePdfOperations, readPdfOperations } from "../🖋️content/🟦️.ts";
import { writePdfToUnicode, readPdfToUnicode, writePdfCMap, readPdfCMap, writePdfFontProgram, readPdfFontProgram } from "../🔤️font/🟦️.ts";
import { writePdfImage, readPdfImage } from "../🖼️resource/🟦️.ts";
import { artifactSqliteInteger } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";

async function writeIdentity(out: PdfProjection, value: PdfGraphIdentity): Promise<bigint> {
  const key = await out.insert("pdf_graph_identity", [pdfInteger(value.owner.num), pdfInteger(value.owner.gen, 16)]);
  for (const [ordinal, part] of value.path.entries()) await out.insert("pdf_graph_path", [key, BigInt(ordinal), part.kind, part.kind === "entry" ? part.key : null, part.kind === "item" ? BigInt(part.index) : null]);
  return key;
}
async function readIdentity(reader: PdfReader, key: bigint): Promise<PdfGraphIdentity> {
  const row = await reader.take("pdf_graph_identity", key, 3), path: PdfGraphPath[] = [];
  for (const child of await reader.children("pdf_graph_path", 1, 2, key)) {
    const row = await reader.take("pdf_graph_path", child.rowid, 6), kind = await reader.text(row, 3);
    if (kind === "entry" && row.values[5] === null) path.push({ kind, key: await reader.text(row, 4) });
    else if (kind === "item" && row.values[4] === null) { const index = artifactSqliteInteger(row, 5); if (index < 0 || index > Number.MAX_SAFE_INTEGER) throw new Error("PDF stream identity ordinal is invalid"); path.push({ kind, index: Number(index) }); }
    else throw new Error("PDF stream identity path variant differs");
  }
  return { owner: { num: pdfNumber(row, 1), gen: pdfNumber(row, 2, 16) }, path };
}
/** 🛫️ Projects role payloads into specific native semantic tables. */
export async function writePdfStreamRole(out: PdfProjection, role: PdfAdmittedStreamRole): Promise<bigint> {
  const cells: PdfCell[] = [await writeIdentity(out, role.identity), role.value.kind, null, null, null, null, null, null, null, null, null, null, null, null];
  const value = role.value;
  if (value.kind === "operators") cells[2] = await writePdfOperations(out, value.content);
  else if (value.kind === "sampledWords") cells[3] = BigInt(value.samples.length);
  else if (value.kind === "calculatorProgram") cells[4] = value.code;
  else if (value.kind === "unicodeMap") cells[5] = await writePdfToUnicode(out, value.mapping);
  else if (value.kind === "characterMap") cells[6] = await writePdfCMap(out, { kind: "embedded", cmap: value.cmap });
  else if (value.kind === "fontProgram") cells[7] = await writePdfFontProgram(out, value.program);
  else if (value.kind === "image") cells[8] = await writePdfImage(out, value.image);
  else if (value.kind === "metadataText") cells[9] = value.text;
  else if (value.kind === "attachmentBytes") cells[10] = await out.bytes(value.bytes);
  else if (value.kind === "paletteComponents") cells[11] = await out.bytes(value.components);
  else if (value.kind === "glyphIds") cells[12] = BigInt(value.glyphs.length);
  else cells[13] = await writePdfArtifactReference(out,value.reference);
  const key = await out.insert("pdf_stream_role", cells);
  if (value.kind === "sampledWords") for (const [ordinal, word] of value.samples.entries()) await out.insert("pdf_stream_role_sample", [key, BigInt(ordinal), pdfInteger(word)]);
  if(value.kind === "glyphIds") for(const [ordinal,glyph] of value.glyphs.entries()) await out.insert("pdf_stream_role_glyph",[key,BigInt(ordinal),pdfInteger(glyph,16)]);
  for (const [ordinal, dependency] of role.dependencies.entries()) await out.insert("pdf_stream_role_dependency", [key, BigInt(ordinal), await writeIdentity(out, dependency)]);
  return key;
}
/** 📥️ Reconstructs every role with explicit variant shape and ordered dependencies. */
export async function readPdfStreamRole(reader: PdfReader, key: bigint): Promise<PdfAdmittedStreamRole> {
  const row = await reader.take("pdf_stream_role", key, 15), kind = await reader.text(row, 2);
  const kinds = ["operators", "sampledWords", "calculatorProgram", "unicodeMap", "characterMap", "fontProgram", "image", "metadataText", "attachmentBytes", "paletteComponents", "glyphIds", "referenceBody"], column = kinds.indexOf(kind) + 3;
  if (column < 3 || row.values[column] === null || row.values.slice(3).some((value, index) => index + 3 !== column && value !== null)) throw new Error("PDF stream role payload variant differs");
  let value: PdfStreamRoleValue;
  if (kind === "operators") value = { kind, content: await readPdfOperations(reader, artifactSqliteInteger(row, 3)) };
  else if (kind === "sampledWords") {
    const children = await reader.children("pdf_stream_role_sample", 1, 2, key), count = artifactSqliteInteger(row, 4), samples: number[] = [];
    if (BigInt(children.length) !== count) throw new Error("PDF stream role sample count differs");
    for (const child of children) { const row = await reader.take("pdf_stream_role_sample", child.rowid, 4); samples.push(pdfNumber(row, 3)); }
    value = { kind, samples };
  }
  else if (kind === "calculatorProgram") value = { kind, code: await reader.text(row, 5) };
  else if (kind === "unicodeMap") value = { kind, mapping: await readPdfToUnicode(reader, artifactSqliteInteger(row, 6)) };
  else if (kind === "characterMap") { const cmap = await readPdfCMap(reader, artifactSqliteInteger(row, 7)); if (cmap.kind !== "embedded") throw new Error("PDF stream role CMap is not embedded"); value = { kind, cmap: cmap.cmap }; }
  else if (kind === "fontProgram") value = { kind, program: await readPdfFontProgram(reader, artifactSqliteInteger(row, 8)) };
  else if (kind === "image") value = { kind, image: await readPdfImage(reader, artifactSqliteInteger(row, 9)) };
  else if (kind === "metadataText") value = { kind, text: await reader.text(row, 10) };
  else if (kind === "attachmentBytes") value = { kind, bytes: await reader.bytes(row,11) };
  else if (kind === "paletteComponents") value = { kind, components: await reader.bytes(row,12) };
  else if (kind === "referenceBody") value = { kind, reference: await readPdfArtifactReference(reader,artifactSqliteInteger(row,14)) };
  else {const children=await reader.children("pdf_stream_role_glyph",1,2,key),glyphs:number[]=[];if(BigInt(children.length)!==artifactSqliteInteger(row,13))throw new Error("PDF glyph role count differs");for(const child of children){const row=await reader.take("pdf_stream_role_glyph",child.rowid,4);glyphs.push(pdfNumber(row,3,16));}value={kind:"glyphIds",glyphs};}
  const dependencies: PdfGraphIdentity[] = [];
  for (const child of await reader.children("pdf_stream_role_dependency", 1, 2, key)) { const row = await reader.take("pdf_stream_role_dependency", child.rowid, 4); dependencies.push(await readIdentity(reader, artifactSqliteInteger(row, 3))); }
  return { identity: await readIdentity(reader, artifactSqliteInteger(row, 1)), dependencies, value };
}
