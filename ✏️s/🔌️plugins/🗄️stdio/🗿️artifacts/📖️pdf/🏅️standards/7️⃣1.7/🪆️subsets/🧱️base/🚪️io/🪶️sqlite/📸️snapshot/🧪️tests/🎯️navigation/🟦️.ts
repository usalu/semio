import { pdfDestinationFromNativeJson,pdfActionFromNativeJson,pdfOutlineFromNativeJson } from "../../../../📝️text/📸️snapshot/🪪️native-json/🎯️navigation/🟦️.ts";
/** 🎯️ Independent SQLite queries validate every destination/action and containment order. */
import { expect,test } from "bun:test";
import { Database } from "bun:sqlite";
import type { PdfAction } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { parsePdfDestination,parsePdfAction,parsePdfOutlineItem } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PdfProjection,PdfReader } from "../../🧩️entity/🟦️.ts";
import { PDF17_SQLITE_SCHEMA } from "../../🧬️schema/🟦️.ts";
import { writePdfDestination,readPdfDestination,writePdfAction,readPdfAction,pdfNavigationNumberColumns } from "../../🎯️navigation/🟦️.ts";
import { writePdfOutline,readPdfOutline,writePdfNamedDestination,readPdfNamedDestination,writePdfLabel,readPdfLabel,writePdfOpenAction,readPdfOpenAction } from "../../🎯️navigation/🔖️bookmark/🟦️.ts";
import { exportSqliteDatabase,importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("Native JSON navigation enters the canonical owned floating point domain",async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../../../📝️text/📸️snapshot/🪪️native-json/🎯️navigation/🧫️fixtures/🔣️.json",import.meta.url)).text());const destination=parsePdfDestination(pdfDestinationFromNativeJson(fixture.destination));if(destination.kind!=="page"||destination.fit.kind!=="xyz")throw new Error("Fixture destination");expect(destination.fit.left).toEqual({bits:0x3ff0000000000000n});
  const action=parsePdfAction(pdfActionFromNativeJson(fixture.action));if(action.kind.kind!=="sound")throw new Error("Fixture action");expect(action.kind.volume).toEqual({bits:0x3ff0000000000000n});expect(action.next!.length).toBe(1);expect(parsePdfOutlineItem(pdfOutlineFromNativeJson(fixture.outline)).color).toEqual([{bits:0n},{bits:0x3ff0000000000000n},{bits:0n}]);
});

test("All PDF destinations and action variants roundtrip as explicit semantic SQL",async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../🎯️navigation/🧫️fixtures/🔣️.json",import.meta.url)).text(),(key,value)=>key==="bits"?BigInt(value):value);
  const out=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{},pdfNavigationNumberColumns);const destinations:bigint[]=[];const actions:bigint[]=[];
  for(const value of fixture.destinations)destinations.push(await writePdfDestination(out,value));for(const value of fixture.actions)actions.push(await writePdfAction(out,value));
  const bytes=await exportSqliteDatabase(await out.finish());const sql=Database.deserialize(bytes);
  expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(sql.query("SELECT name FROM pdf_action_name ORDER BY id").all()).toEqual([{name:"A"},{name:"A"},{name:"F"},{name:"F"}]);
  expect(sql.query("SELECT COUNT(*) AS actions FROM pdf_action").get()).toEqual({actions:20});
  const reader=await PdfReader.create(await importSqliteDatabase(bytes),PDF17_SQLITE_SCHEMA,{},pdfNavigationNumberColumns);
  for(const[index,key]of destinations.entries())expect(await readPdfDestination(reader,key)).toEqual(fixture.destinations[index]);for(const[index,key]of actions.entries())expect(await readPdfAction(reader,key)).toEqual(fixture.actions[index]);await reader.finish();sql.close();
},{timeout:30_000});

test("Deep PDF action containment remains iterable and cancellable with enforced row budgets",async()=>{
  const plan=JSON.parse(await Bun.file(new URL("../../🧫️fixtures/🌲️deep.json",import.meta.url)).text());let action:PdfAction={kind:{kind:"named",name:plan.leafName},next:[]};for(let index=0;index<plan.depth;index++)action={kind:{kind:"named",name:`Level-${index}`},next:[action]};
  const out=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{},pdfNavigationNumberColumns);const key=await writePdfAction(out,action);const bytes=await exportSqliteDatabase(await out.finish());const sql=Database.deserialize(bytes);expect(sql.query("SELECT COUNT(*) AS actions FROM pdf_action").get()).toEqual({actions:plan.expectedValues});
  const reader=await PdfReader.create(await importSqliteDatabase(bytes),PDF17_SQLITE_SCHEMA,{},pdfNavigationNumberColumns);let value=await readPdfAction(reader,key);let depth=0;while(value.next!.length){value=value.next![0]!;depth++;}expect(depth).toBe(plan.depth);expect(value.kind).toEqual({kind:"named",name:plan.leafName});await reader.finish();sql.close();
  const abort=new AbortController();let progress=false;const canceled=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{signal:abort.signal,onProgress:event=>{if(event.phase==="projectSnapshot"&&event.completed>0){progress=true;abort.abort();}}},pdfNavigationNumberColumns);await expect(writePdfAction(canceled,action)).rejects.toThrow(/cancel/i);expect(progress).toBe(true);
  const bounded=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{maxRows:8},pdfNavigationNumberColumns);await expect(writePdfAction(bounded,action)).rejects.toThrow(/row limit/i);
},{timeout:30_000});

test("PDF bookmarks preserve containment, repeated titles and full label and open action fields",async()=>{
  const fixture=JSON.parse(await Bun.file(new URL("../../🎯️navigation/🔖️bookmark/🧫️fixtures/🔣️.json",import.meta.url)).text(),(key,value)=>key==="bits"?BigInt(value):value);
  const out=await PdfProjection.create(PDF17_SQLITE_SCHEMA,{},pdfNavigationNumberColumns);const outline=await writePdfOutline(out,fixture.outline);const named=await writePdfNamedDestination(out,fixture.named);const labels:bigint[]=[];const open:bigint[]=[];
  for(const value of fixture.labels)labels.push(await writePdfLabel(out,value));for(const value of fixture.openActions)open.push(await writePdfOpenAction(out,value));
  const bytes=await exportSqliteDatabase(await out.finish());const sql=Database.deserialize(bytes);expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(sql.query("SELECT ordinal,title FROM pdf_outline_child c JOIN pdf_outline o ON o.id=c.child_id ORDER BY ordinal").all()).toEqual([{ordinal:0,title:"Child"},{ordinal:1,title:"Child"}]);
  const reader=await PdfReader.create(await importSqliteDatabase(bytes),PDF17_SQLITE_SCHEMA,{},pdfNavigationNumberColumns);expect(await readPdfOutline(reader,outline)).toEqual(fixture.outline);expect(await readPdfNamedDestination(reader,named)).toEqual(fixture.named);for(const[index,key]of labels.entries())expect(await readPdfLabel(reader,key)).toEqual(fixture.labels[index]);for(const[index,key]of open.entries())expect(await readPdfOpenAction(reader,key)).toEqual(fixture.openActions[index]);await reader.finish();sql.close();
},{timeout:30_000});
