/** 🖋️ Every neutral operator is independently interpreted by SQLite with exact named operands. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import type { PdfOp } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PdfProjection, PdfReader } from "../../🧩️entity/🟦️.ts";
import { PDF17_SQLITE_SCHEMA } from "../../🧬️schema/🟦️.ts";
import { writePdfOperations, readPdfOperations, pdfContentNumberColumns } from "../../🖋️content/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("PDF content operators retain every named scalar, intrinsic image and ordered relationship", async () => {
  const fixture = JSON.parse(await Bun.file(new URL("../../🖋️content/🧫️fixtures/🔣️.json", import.meta.url)).text(), (key, value) => key === "bits" ? BigInt(value) : value);
  const input: PdfOp[] = fixture.operations;
  const out = await PdfProjection.create(PDF17_SQLITE_SCHEMA, {}, pdfContentNumberColumns); const root = await writePdfOperations(out, input);
  const bytes = await exportSqliteDatabase(await out.finish()); const sql = Database.deserialize(bytes);
  expect(sql.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"}); expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(sql.query("SELECT kind FROM pdf_operation ORDER BY ordinal").all()).toEqual(input.map(value => ({kind:value.op})));
  expect(sql.query("SELECT CAST(line_width_bits AS TEXT) AS bits,line_width_class FROM pdf_operation WHERE kind='setLineWidth'").get()).toEqual({bits:"-9223372036854775808",line_width_class:"finite"});
  expect(sql.query("SELECT hex(data) AS bytes,CAST(width AS TEXT) AS width FROM pdf_inline_image").get()).toEqual({bytes:"00FF80",width:"4294967295"});
  const reader = await PdfReader.create(await importSqliteDatabase(bytes), PDF17_SQLITE_SCHEMA, {}, pdfContentNumberColumns); expect(await readPdfOperations(reader, root)).toEqual(input); await reader.finish(); sql.close();
}, { timeout:30_000 });

test("PDF content rejects foreign IEEE operands and bounds intrinsic ownership copies", async () => {
  const out=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{},pdfContentNumberColumns);const root=await writePdfOperations(out,[{op:"save"}]);const sql=Database.deserialize(await exportSqliteDatabase(await out.finish()));
  sql.run("UPDATE pdf_operation SET line_width_bits=9221120237041090626,line_width_class='nan'");
  const reader=await PdfReader.create(await importSqliteDatabase(sql.serialize()),PDF17_SQLITE_SCHEMA,{},pdfContentNumberColumns);await expect(readPdfOperations(reader,root)).rejects.toThrow("unrelated scalar or IEEE");sql.close();
  const controller=new AbortController();const canceled=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{signal:controller.signal,onProgress:progress=>{if(progress.phase==="projectSnapshot"&&progress.completed>0)controller.abort();}},pdfContentNumberColumns);
  const many: PdfOp[]=Array.from({length:300},()=>({op:"save"}));await expect(writePdfOperations(canceled,many)).rejects.toThrow("cancel");
  const bounded=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{maxValueBytes:128},pdfContentNumberColumns);await expect(writePdfOperations(bounded,[{op:"showText",text:{kind:"codes",codes:new Array(129).fill(255)}}])).rejects.toThrow("value");
}, { timeout:30_000 });

test("PDF logical code units retain u32 precision in independent integer rows", async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../../../🧫️fixtures/🔢️logical-character-codes/🔣️.json",import.meta.url)).text());
  for(const sample of fixture.cases){
    const input:PdfOp[]=[{op:"showText",text:{kind:"codes",codes:sample.codes}},{op:"showTextArray",items:[{kind:"codes",codes:sample.codes}]}];
    const out=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{},pdfContentNumberColumns);const root=await writePdfOperations(out,input);
    const bytes=await exportSqliteDatabase(await out.finish());const sql=Database.deserialize(bytes);
    for(const table of ["pdf_operation_code","pdf_array_item_code"]) expect(sql.query(`SELECT code FROM ${table} ORDER BY ordinal`).all()).toEqual(sample.codes.map((code:number)=>({code})));
    expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
    const reader=await PdfReader.create(await importSqliteDatabase(bytes),PDF17_SQLITE_SCHEMA,{},pdfContentNumberColumns);expect(await readPdfOperations(reader,root)).toEqual(input);await reader.finish();
    sql.run("UPDATE pdf_operation_code SET code=4294967295 WHERE ordinal=0");const edited=await PdfReader.create(await importSqliteDatabase(sql.serialize()),PDF17_SQLITE_SCHEMA,{},pdfContentNumberColumns);const current=await readPdfOperations(edited,root);expect(current[0]).toEqual({op:"showText",text:{kind:"codes",codes:[0xffffffff,...sample.codes.slice(1)]}});await edited.finish();sql.close();
    console.log(`[DEBUG] PDF logical code case=${sample.id} native width=${sample.width} independent integer rows exact`);
  }
}, {timeout:30000});
