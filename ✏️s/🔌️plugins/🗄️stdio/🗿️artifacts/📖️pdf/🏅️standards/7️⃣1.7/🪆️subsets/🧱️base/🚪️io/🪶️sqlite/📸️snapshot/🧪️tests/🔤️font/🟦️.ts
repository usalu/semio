/** 🔤️ Neutral font entities and independent SQLite Unicode, byte and numeric interpretation. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { PDF17_SQLITE_SCHEMA as publicSchema } from "@semio-tech/stdio-pdf";
import type { PdfFont, PdfFontDescriptor, PdfFontProgram, PdfSimpleEncoding, PdfToUnicode, PdfCMap, PdfCidFont, PdfCidToGid, Binary64 } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PdfProjection, PdfReader, type SqliteDatabase } from "../../🧩️entity/🟦️.ts";
import { PDF17_SQLITE_SCHEMA } from "../../🧬️schema/🟦️.ts";
import { pdfFontNumberColumns, writePdfFontDescriptor, readPdfFontDescriptor, writePdfFontProgram, readPdfFontProgram, writePdfEncoding, readPdfEncoding, writePdfToUnicode, readPdfToUnicode, writePdfCMap, readPdfCMap, writePdfCidFont, readPdfCidFont, writePdfFont, readPdfFont } from "../../🔤️font/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { parsePdfFont } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";

async function source(): Promise<{ descriptor: PdfFontDescriptor; programs: PdfFontProgram[]; encodings: PdfSimpleEncoding[]; unicode: PdfToUnicode; cmaps: PdfCMap[] }> {
  const input = await Bun.file(new URL("../../🔤️font/🧫️fixtures/🔣️.json", import.meta.url)).json();
  const values: Binary64[] = input.binary64.map((bits: string) => ({ bits: BigInt(`0x${bits}`) }));
  const descriptor: PdfFontDescriptor = { fontName: input.fontName, flags: 4294967295, fontBbox: [values[0]!, values[1]!, values[2]!, values[3]!], italicAngle: values[4]!, ascent: values[5]!, descent: values[6]!, capHeight: values[0]!, stemV: values[1]!, stemH: null, xHeight: values[2]!, leading: values[3]!, avgWidth: values[4]!, maxWidth: values[5]!, missingWidth: values[6]!, fontFamily: "Ω Family", fontStretch: null, fontWeight: values[5]!, charSet: "A/B Ω", extra: [{ key: "Max", value: { kind: "int", value: 9223372036854775807n } }] };
  return { descriptor, programs: input.programs, encodings: input.encodings.map((base: PdfSimpleEncoding["base"]) => ({ base, differences: input.differences })), unicode: input.unicode, cmaps: input.cmaps };
}

async function project(write: (out: PdfProjection) => Promise<bigint>): Promise<{ database: SqliteDatabase; root: bigint }> { const out = await PdfProjection.create(PDF17_SQLITE_SCHEMA, {}, pdfFontNumberColumns); const root = await write(out); return { database: await out.finish(), root }; }

test("PDF font descriptors preserve all fields and exact numeric companions", async () => {
  expect(publicSchema).toBe(PDF17_SQLITE_SCHEMA);
  const input = await source(); const encoded = await project(out => writePdfFontDescriptor(out, input.descriptor));
  const bytes = await exportSqliteDatabase(encoded.database); const sql = Database.deserialize(bytes);
  expect(sql.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" }); expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(sql.query("SELECT font_name,CAST(flags AS TEXT) AS flags,CAST(ascent_bits AS TEXT) AS bits,ascent_class FROM pdf_font_descriptor").get()).toEqual({ font_name: input.descriptor.fontName, flags: "4294967295", bits: BigInt("0x7ff0000000000042").toString(), ascent_class: "nan" });
  const reader = await PdfReader.create(await importSqliteDatabase(bytes), PDF17_SQLITE_SCHEMA, {}, pdfFontNumberColumns); expect(await readPdfFontDescriptor(reader, encoded.root)).toEqual(input.descriptor); await reader.finish(); sql.close();
});

test("PDF intrinsic font programs and ordered encoding differences are independently readable", async () => {
  const input = await source();
  for (const value of input.programs) {
    const encoded = await project(out => writePdfFontProgram(out, value)); const sql = Database.deserialize(await exportSqliteDatabase(encoded.database));
    expect(sql.query("SELECT kind,hex(data) AS octets FROM pdf_font_program").get()).toEqual({ kind: value.kind, octets: value.data.map(byte => byte.toString(16).padStart(2,"0")).join("").toUpperCase() });
    const reader = await PdfReader.create(await importSqliteDatabase(sql.serialize()), PDF17_SQLITE_SCHEMA, {}, pdfFontNumberColumns); expect(await readPdfFontProgram(reader, encoded.root)).toEqual(value); await reader.finish(); sql.close();
  }
  for (const value of input.encodings) { const encoded = await project(out => writePdfEncoding(out, value)); const reader = await PdfReader.create(encoded.database, PDF17_SQLITE_SCHEMA, {}, pdfFontNumberColumns); expect(await readPdfEncoding(reader, encoded.root)).toEqual(value); await reader.finish(); }
}, { timeout: 30_000 });

test("PDF every font variant preserves typed ownership and Type3 content without lowering", async () => {
  const input = await source(); const zero: Binary64={bits:0n}; const unusual: Binary64={bits:0x7ff0000000000042n};
  const descendant: PdfCidFont={trueType:true,baseFont:"CID",systemInfo:{registry:"Adobe",ordering:"Identity",supplement:0},descriptor:input.descriptor,defaultWidth:unusual,widths:[],defaultVertical:null,verticalMetrics:[],cidToGid:null,program:null,extra:[]};
  const fonts: PdfFont[]=[
    {id:"Type1",kind:{kind:"type1",baseFont:"One",encoding:input.encodings[0]!,firstChar:255,widths:[zero,unusual],descriptor:input.descriptor,program:input.programs[0]!},toUnicode:input.unicode,extra:[]},
    {id:"TrueType",kind:{kind:"trueType",baseFont:"True",encoding:input.encodings[1]!,firstChar:0,widths:[],descriptor:null,program:null},toUnicode:null,extra:[]},
    {id:"Type3",kind:{kind:"type3",fontMatrix:[zero,unusual,zero,unusual,zero,unusual],fontBbox:[zero,unusual,zero,unusual],encoding:input.encodings[2]!,firstChar:1,widths:[unusual],charProcs:[{name:"Ω Glyph",content:[{op:"setLineWidth",width:unusual},{op:"showText",text:{kind:"codes",bytes:[0,255]}}]}],descriptor:null},toUnicode:null,extra:[]},
    {id:"Type0",kind:{kind:"type0",baseFont:"Composite",cmap:input.cmaps[1]!,descendant},toUnicode:input.unicode,extra:[]}
  ];
  for (const value of fonts) {
    const encoded=await project(out=>writePdfFont(out,value));const bytes=await exportSqliteDatabase(encoded.database);const sql=Database.deserialize(bytes);
    expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(sql.query("SELECT resource_name,kind FROM pdf_font").get()).toEqual({resource_name:value.id,kind:value.kind.kind});
    const reader=await PdfReader.create(await importSqliteDatabase(bytes),PDF17_SQLITE_SCHEMA,{},pdfFontNumberColumns);expect(await readPdfFont(reader,encoded.root)).toEqual(value);await reader.finish();sql.close();
  }
}, { timeout:30_000 });

test("PDF native font JSON admission constructs the same owned Binary64 and content model", async () => {
  const value=parsePdfFont(await Bun.file(new URL("../../../../../🧬️schema/📸️snapshot/🪪️native-json/🔤️font/🧫️fixtures/🔣️.json",import.meta.url)).json());
  expect(value.kind.kind).toBe("type3");if(value.kind.kind!=="type3")throw new Error("Incorrect owned font");
  expect(value.kind.fontMatrix[0]).toEqual({bits:0x3ff0000000000000n});expect(value.kind.widths[0]).toEqual({bits:0x3ff0000000000000n});expect(value.kind.charProcs[0]!.content).toEqual([{op:"setLineWidth",width:{bits:0x3ff0000000000000n}}]);
});

test("PDF Unicode and CID mappings preserve exact u32 coordinates and ordered duplicate semantics", async () => {
  const input = await source(); const encoded = await project(out => writePdfToUnicode(out, input.unicode)); const sql = Database.deserialize(await exportSqliteDatabase(encoded.database));
  expect(sql.query("SELECT kind,CAST(code AS TEXT) AS code,CAST(high AS TEXT) AS high,text FROM pdf_unicode_mapping ORDER BY ordinal").all()).toEqual([{ kind: "char", code: "4294967295", high: null, text: "Ω" }, { kind: "range", code: null, high: "4294967295", text: "A 🧬" }]);
  const reader = await PdfReader.create(await importSqliteDatabase(sql.serialize()), PDF17_SQLITE_SCHEMA, {}, pdfFontNumberColumns); expect(await readPdfToUnicode(reader, encoded.root)).toEqual(input.unicode); await reader.finish(); sql.close();
  for (const value of input.cmaps) { const encoded = await project(out => writePdfCMap(out, value)); const reader = await PdfReader.create(encoded.database, PDF17_SQLITE_SCHEMA, {}, pdfFontNumberColumns); expect(await readPdfCMap(reader, encoded.root)).toEqual(value); await reader.finish(); }
});

test("PDF CID font ownership preserves every metric, optional paired words and all glyph mappings", async () => {
  const input = await source();
  const mappings: (PdfCidToGid | null)[] = [null, { kind: "identity" }, { kind: "map", data: [0,255,128] }];
  for (const [index, cidToGid] of mappings.entries()) {
    const value: PdfCidFont = { trueType: index % 2 === 0, baseFont: "CID Ω", systemInfo: { registry: "Authored", ordering: "Ω", supplement: 4294967295 }, descriptor: input.descriptor, defaultWidth: { bits: 0x7ff0000000000042n }, widths: [{ startCid: 4294967295, widths: [{ bits: 0x8000000000000000n }, { bits: 1n }] }, { startCid: 0, widths: [] }], defaultVertical: index === 0 ? null : [{ bits: 0xfff0000000000000n }, { bits: 0x7ff0000000000042n }], verticalMetrics: [{ startCid: 2, metrics: [[{ bits: 1n }, { bits: 0x8000000000000000n }, { bits: 0xfff8000000000055n }]] }], cidToGid, program: index === 0 ? null : input.programs[index]!, extra: [] };
    const encoded = await project(out => writePdfCidFont(out, value)); const bytes = await exportSqliteDatabase(encoded.database); const sql = Database.deserialize(bytes);
    expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(sql.query("SELECT CAST(start_cid AS TEXT) AS cid FROM pdf_cid_width_run ORDER BY ordinal").all()).toEqual([{cid:"4294967295"},{cid:"0"}]);
    const reader = await PdfReader.create(await importSqliteDatabase(bytes), PDF17_SQLITE_SCHEMA, {}, pdfFontNumberColumns); expect(await readPdfCidFont(reader, encoded.root)).toEqual(value); await reader.finish();
    sql.close();
  }
}, { timeout: 30_000 });
