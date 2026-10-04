/** 🪶️ Closed full byte-owner semantic SQLite demand. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import fixture from "../../../🧫️fixtures/🪶️sqlite/🧬️semantic/🔣️.json";
import schema from "../../../🧫️fixtures/🪶️sqlite/🧬️semantic/🧬️schema/🔣️.json";
import corpus from "../../../../../🧫️fixtures/🧬️canonical-byte-authority/🔣️.json";
import { BMP_SQLITE_SCHEMA,bmpSnapshotToSqliteDatabase,bmpSnapshotFromSqliteDatabase } from "../../../🪶️sqlite/🟦️.ts";
import source from "../../../🧫️fixtures/🪶️sqlite/🔣️.json";

test("BMP current byte owner demands complete editable semantic layout and explicit literal octet states", async()=>{
 const validate=new Ajv2020({strict:true,allErrors:true}).compile(schema);
 expect(validate(fixture)).toBe(true);
 expect(validate({...fixture,wholeFileBlob:true})).toBe(false);
 expect(validate({...fixture,tables:fixture.tables.slice(0,-1)})).toBe(false);
 expect(validate({...fixture,literalCases:[...fixture.literalCases,{id:"other",schema:"x",bytes:[256],diagnostic:"x"}]})).toBe(false);
 expect(corpus.accepted.map(value=>value.id)).toEqual(fixture.accepted);
 expect(corpus.rejected.map(value=>value.id)).toEqual(fixture.nativeRejected);
 expect([...new Set(corpus.accepted.map(value=>value.profile))].sort()).toEqual([...fixture.profiles].sort());
 const desired=await Bun.file(new URL("../../../🪶️sqlite/🧬️semantic/🗄️.sql",import.meta.url)).text();
 const independent=new Database(":memory:");
 try{
  independent.exec(desired);
  const tables=independent.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY rowid").all() as {name:string}[];
  expect(tables.map(value=>value.name)).toEqual(fixture.tables);
  for(const table of fixture.tables)expect((independent.query(`PRAGMA table_info("${table}")`).all() as {type:string}[]).every(column=>column.type!=="BLOB")).toBe(true);
  independent.run("INSERT INTO bmp_document VALUES(1,?,'literal_octets',?)",[fixture.literalCases[0]!.schema,fixture.literalCases[0]!.diagnostic]);
  expect(independent.query("SELECT schema,diagnostic FROM bmp_document").get()).toEqual({schema:fixture.literalCases[0]!.schema,diagnostic:fixture.literalCases[0]!.diagnostic});
  expect(()=>independent.run("INSERT INTO bmp_literal_octet VALUES(1,1,0,256)")).toThrow();
  expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
 }finally{independent.close();}
 expect(BMP_SQLITE_SCHEMA).toBe(desired);
});

test("BMP semantic projection captures actual input octets before asynchronous row publication",async()=>{
 expect(fixture.projectionCapture).toEqual({input:"actualSnapshotBytes",buffer:"immutableOctets",before:"asyncSemanticRows",retirementRefund:false});
 const mutable={schema:source.schema,bytes:[...source.bytes]},expected=structuredClone(mutable);let events=0;
 const database=await bmpSnapshotToSqliteDatabase(mutable,{onProgress:event=>{if(event.phase==="projectSnapshot"&&++events===2)mutable.bytes[74]=19;}});
 expect(events).toBeGreaterThanOrEqual(2);expect(mutable.bytes[74]).toBe(19);
 expect(await bmpSnapshotFromSqliteDatabase(database)).toEqual(expected);
});


test("BMP row-tail reconstruction preserves interior cancellation while consuming actual tail occurrences",async()=>{
 const demand=(fixture as unknown as {rowTailProgress:{width:number;height:number;bitsPerPixel:number;cancelAfterOccurrences:number;checkpointCadence:number;preserveCancellationKind:boolean}}).rowTailProgress;
 expect(demand).toEqual({width:1,height:257,bitsPerPixel:1,cancelAfterOccurrences:64,checkpointCadence:256,preserveCancellationKind:true});
 const bytes=new Uint8Array(62+demand.height*4),view=new DataView(bytes.buffer);bytes.set([66,77]);view.setUint32(2,bytes.length,true);view.setUint32(10,62,true);view.setUint32(14,40,true);view.setInt32(18,demand.width,true);view.setInt32(22,-demand.height,true);view.setUint16(26,1,true);view.setUint16(28,demand.bitsPerPixel,true);view.setUint32(34,demand.height*4,true);bytes.set([0,0,0,0,255,255,255,0],54);
 for(let y=0;y<demand.height;y++)bytes[62+y*4]=y%128;
 const database=await bmpSnapshotToSqliteDatabase({schema:"stdio.bmp",bytes:Array.from(bytes)}),tails=database.tables.find(table=>table.name==="bmp_row_tail_bits")!;
 const independent=new Database(":memory:");try{independent.exec(BMP_SQLITE_SCHEMA);independent.run("INSERT INTO bmp_document VALUES(1,'stdio.bmp','valid_layout','')");for(let y=0;y<demand.height;y++)independent.run("INSERT INTO bmp_row_tail_bits VALUES(?,1,?,?)",[y+1,y,y%128]);expect(independent.query("SELECT count(*) AS count,sum(unused_bits) AS sum FROM bmp_row_tail_bits").get()).toEqual({count:demand.height,sum:tails.rows.reduce((sum,row)=>sum+Number(row.values[3]),0)});}finally{independent.close();}
 let pixelsReady=false,tailOccurrences=0;for(const row of tails.rows){const values=row.values;Object.defineProperty(row,"values",{get:()=>new Proxy(values,{get:(target,key,receiver)=>{if(pixelsReady&&key==="3")tailOccurrences++;return Reflect.get(target,key,receiver);}})});}
 const stop=new AbortController();
 await expect(bmpSnapshotFromSqliteDatabase(database,{signal:stop.signal,onProgress:event=>{if(event.phase==="reconstructSnapshot"&&event.completed===demand.checkpointCadence&&event.total===demand.height)pixelsReady=true;if(tailOccurrences>=demand.cancelAfterOccurrences&&tailOccurrences<demand.height)stop.abort();}})).rejects.toMatchObject({kind:"canceled"});
 expect(tailOccurrences).toBeGreaterThanOrEqual(demand.cancelAfterOccurrences);expect(tailOccurrences).toBeLessThan(demand.height);
},30000);
