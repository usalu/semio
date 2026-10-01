/** 🖋️ Every neutral operator is independently interpreted by SQLite with exact named operands. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import type { PdfOp } from "../../../🟦️.ts";
import { PdfProjection, PdfReader } from "../../../🪶️sqlite/🧩️entity/🟦️.ts";
import { PDF17_SQLITE_SCHEMA } from "../../../🪶️sqlite/🧬️schema/🟦️.ts";
import { writePdfOperations, readPdfOperations, pdfContentNumberColumns } from "../../../🪶️sqlite/🖋️content/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("PDF content operators retain every named scalar, intrinsic image and ordered relationship", async () => {
  const fixture = JSON.parse(await Bun.file(new URL("../../../🪶️sqlite/🖋️content/🧫️fixtures/🔣️.json", import.meta.url)).text(), (key, value) => key === "bits" ? BigInt(value) : value);
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
  const bounded=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{maxValueBytes:128},pdfContentNumberColumns);await expect(writePdfOperations(bounded,[{op:"showText",text:{kind:"codes",bytes:new Array(129).fill(255)}}])).rejects.toThrow("value");
}, { timeout:30_000 });
