import { pdfAnnotationFromNativeJson,pdfAnnotationKindFromNativeJson } from "../../../../📝️text/📸️snapshot/🪪️native-json/📌️annotation/🟦️.ts";
/** 📌️ Independent SQL queries verify appearance states, border geometry and ownership. */
import { expect,test } from "bun:test";
import { Database } from "bun:sqlite";
import { parsePdfAnnotation,parsePdfAnnotationKind } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PdfProjection,PdfReader } from "../../🧩️entity/🟦️.ts";
import { PDF17_SQLITE_SCHEMA } from "../../🧬️schema/🟦️.ts";
import { writePdfAppearance,readPdfAppearance,writePdfBorder,readPdfBorder } from "../../📌️annotation/🎭️appearance/🟦️.ts";
import { pdfAnnotationNumberColumns } from "../../📌️annotation/🔢️number/🟦️.ts";
import { writePdfAnnotationKind,readPdfAnnotationKind } from "../../📌️annotation/🏷️kind/🟦️.ts";
import { writePdfAnnotation,readPdfAnnotation } from "../../📌️annotation/🟦️.ts";
import { exportSqliteDatabase,importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("Native annotation JSON admits complete canonical IEEE scalars and bigint indices",async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../../../📝️text/📸️snapshot/🪪️native-json/📌️annotation/🧫️fixtures/🔣️.json",import.meta.url)).text());const value=parsePdfAnnotation(pdfAnnotationFromNativeJson(fixture.annotation));expect(value.rect[2]).toEqual({bits:0x3ff0000000000000n});expect(value.border!.width).toEqual({bits:0x3ff0000000000000n});expect(value.markup!.opacity).toEqual({bits:0x3ff0000000000000n});expect(value.markup!.popup).toBe(4294967295n);expect(value.markup!.inReplyTo).toBe(4294967296n);const popup=parsePdfAnnotationKind(pdfAnnotationKindFromNativeJson(fixture.popup));if(popup.kind!=="popup")throw new Error("Fixture popup");expect(popup.parent).toBe(4294967295n);
});

test("PDF annotation appearances and borders retain optional entity and geometry states",async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../📌️annotation/🎭️appearance/🧫️fixtures/🔣️.json",import.meta.url)).text(),(key,value)=>key==="bits"?BigInt(value):value);
  const out=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{},pdfAnnotationNumberColumns);const appearances:bigint[]=[];const borders:bigint[]=[];for(const value of fixture.appearances)appearances.push(await writePdfAppearance(out,value));for(const value of fixture.borders)borders.push(await writePdfBorder(out,value));
  const bytes=await exportSqliteDatabase(await out.finish());const sql=Database.deserialize(bytes);expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(sql.query("SELECT state,form FROM pdf_appearance_state ORDER BY ordinal").all()).toEqual([{state:"On",form:"A"},{state:"On",form:"B"}]);expect(sql.query("SELECT dash_present FROM pdf_annotation_border ORDER BY id").all()).toEqual([{dash_present:0},{dash_present:1},{dash_present:1}]);
  const reader=await PdfReader.create(await importSqliteDatabase(bytes),PDF17_SQLITE_SCHEMA,{},pdfAnnotationNumberColumns);for(const[index,key]of appearances.entries())expect(await readPdfAppearance(reader,key)).toEqual(fixture.appearances[index]);for(const[index,key]of borders.entries())expect(await readPdfBorder(reader,key)).toEqual(fixture.borders[index]);await reader.finish();sql.close();
},{timeout:30_000});

test("Complete PDF annotations preserve full markup indices and signaling NaN common fields",async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../📌️annotation/🧫️fixtures/🔣️.json",import.meta.url)).text(),(key,value)=>key==="bits"||["parent","popup","inReplyTo"].includes(key)&&typeof value==="string"?BigInt(value):value);const values=fixture.kinds.map((kind:unknown)=>({...fixture.common,kind}));const out=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{},pdfAnnotationNumberColumns);const keys:bigint[]=[];for(const value of values)keys.push(await writePdfAnnotation(out,value));
  const bytes=await exportSqliteDatabase(await out.finish());const sql=Database.deserialize(bytes);expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(sql.query("SELECT popup_high,popup_low,reply_high,reply_low FROM pdf_annotation_markup ORDER BY id LIMIT 1").get()).toEqual({popup_high:4294967295,popup_low:4294967295,reply_high:1,reply_low:0});
  const reader=await PdfReader.create(await importSqliteDatabase(bytes),PDF17_SQLITE_SCHEMA,{},pdfAnnotationNumberColumns);for(const[index,key]of keys.entries())expect(await readPdfAnnotation(reader,key)).toEqual(values[index]);await reader.finish();sql.close();
},{timeout:30_000});

test("PDF annotation import rejects partial wide indices and unrelated NaN geometry",async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../📌️annotation/🧫️fixtures/🔣️.json",import.meta.url)).text(),(key,value)=>key==="bits"||key==="parent"?BigInt(value):value);
  const out=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{},pdfAnnotationNumberColumns);const text=await writePdfAnnotationKind(out,fixture.kinds[0]);const popup=await writePdfAnnotationKind(out,fixture.kinds[15]);const bytes=await exportSqliteDatabase(await out.finish());
  const partial=Database.deserialize(bytes);partial.exec("UPDATE pdf_annotation_detail SET parent_low=NULL WHERE kind='popup'");const partialReader=await PdfReader.create(await importSqliteDatabase(partial.serialize()),PDF17_SQLITE_SCHEMA,{},pdfAnnotationNumberColumns);await expect(readPdfAnnotationKind(partialReader,popup)).rejects.toThrow("absent word");partial.close();
  const foreign=Database.deserialize(bytes);foreign.exec("UPDATE pdf_annotation_detail SET point_0_bits=9221120237041090561,point_0_class='nan' WHERE kind='text'");const foreignReader=await PdfReader.create(await importSqliteDatabase(foreign.serialize()),PDF17_SQLITE_SCHEMA,{},pdfAnnotationNumberColumns);await expect(readPdfAnnotationKind(foreignReader,text)).rejects.toThrow("unrelated scalar or IEEE word");foreign.close();
},{timeout:30_000});

test("Every PDF annotation subtype has interpretable SQL entities and exact owned coordinates",async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../📌️annotation/🧫️fixtures/🔣️.json",import.meta.url)).text(),(key,value)=>key==="bits"||key==="parent"?BigInt(value):value);const out=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{},pdfAnnotationNumberColumns);const keys:bigint[]=[];for(const kind of fixture.kinds)keys.push(await writePdfAnnotationKind(out,kind));
  const bytes=await exportSqliteDatabase(await out.finish());const sql=Database.deserialize(bytes);expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(sql.query("SELECT kind FROM pdf_annotation_detail ORDER BY id").all()).toEqual(fixture.kinds.map((value:{kind:string})=>({kind:value.kind})));expect(sql.query("SELECT parent_high,parent_low FROM pdf_annotation_detail WHERE kind='popup'").get()).toEqual({parent_high:4294967295,parent_low:4294967295});
  const reader=await PdfReader.create(await importSqliteDatabase(bytes),PDF17_SQLITE_SCHEMA,{},pdfAnnotationNumberColumns);for(const[index,key]of keys.entries())expect(await readPdfAnnotationKind(reader,key)).toEqual(fixture.kinds[index]);await reader.finish();sql.close();
},{timeout:30_000});
