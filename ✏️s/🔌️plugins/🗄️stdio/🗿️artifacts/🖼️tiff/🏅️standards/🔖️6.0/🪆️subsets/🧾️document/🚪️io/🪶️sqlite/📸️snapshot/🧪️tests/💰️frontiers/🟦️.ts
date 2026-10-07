/** 🖼️ Complete TIFF frontiers laws retain their original operation limits. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import fixture from "../../../../../🧬️schema/📸️snapshot/🧫️fixtures/💰️frontiers/🔣️.json";
import {exportSqliteDatabase,SqliteOperation} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { tiffSnapshotToSqliteDatabase, tiffSnapshotToSqliteFile, tiffSnapshotFromSqliteFile } from "../../🟦️.ts";
const small={schema:"stdio.tiff" as const,ifds:[]};
const corpus={...small,ifds:[{entries:[{tag:65535,values:{kind:"long" as const,value:Array.from({length:fixture.domainValueCount},(_,i)=>i)}}],blocks:[]}]};
let corpusFile:Promise<Uint8Array>|undefined;
function bytes(){return corpusFile??=tiffSnapshotToSqliteFile(corpus);}
test("cooperative frontier example independently preserves TIFF relational scalars",async()=>{
 const db=Database.deserialize(await bytes());try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("SELECT count(*) AS n,min(value) AS first,max(value) AS last FROM tiff_long_value").get()).toEqual({n:fixture.domainValueCount,first:0,last:fixture.domainValueCount-1});const tables=db.query("SELECT name FROM sqlite_schema WHERE type='table'").all()as{name:string}[];let rows=0;for(const t of tables)rows+=(db.query(`SELECT count(*) AS n FROM "${t.name}"`).get()as{n:number}).n;expect(rows).toBe(fixture.domainRows);}finally{db.close();}
});
test("complete TIFF owned text materialization cancels within its first byte sequence",async()=>{
 const text=fixture.asciiUnit.repeat(fixture.asciiLength),input={...small,ifds:[{entries:[{tag:315,values:{kind:"ascii" as const,value:[text]}}],blocks:[]}]},abort=new AbortController();let reached=false;
 await expect(tiffSnapshotToSqliteFile(input,{signal:abort.signal,onProgress:progress=>{if(progress.phase==="projectSnapshot"&&progress.total===fixture.asciiLength&&progress.completed>=fixture.asciiFrontier&&progress.completed<progress.total){reached=true;abort.abort();}}})).rejects.toHaveProperty("kind",fixture.refusal);expect(reached).toBe(true);
});
test("complete TIFF physical import cancels inside its actual schema token scan",async()=>{
 const projected=await tiffSnapshotToSqliteDatabase(small);const sql=projected.tables[0]!.sql.replace("tiff_document (",`tiff_document /*${" ".repeat(fixture.schemaCommentCharacters)}*/ (`),value={tables:projected.tables.map((t,i)=>i===0?{...t,sql}:t)},file=await exportSqliteDatabase(value),db=Database.deserialize(file);
 try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect((db.query("SELECT sql FROM sqlite_schema WHERE name='tiff_document'").get()as{sql:string}).sql).toBe(sql);}finally{db.close();}
 const abort=new AbortController();let reached=false;await expect(tiffSnapshotFromSqliteFile(file,{signal:abort.signal,onProgress:p=>{if(p.phase===fixture.phases[0]&&p.total===sql.length&&p.completed>=fixture.schemaFrontier&&p.completed<p.total){reached=true;abort.abort();}}})).rejects.toHaveProperty("kind",fixture.refusal);expect(reached).toBe(true);
});
test("complete TIFF import cancels within shared relational row validation",async()=>{
 const abort=new AbortController();let reached=false;await expect(tiffSnapshotFromSqliteFile(await bytes(),{signal:abort.signal,onProgress:p=>{if(p.phase===fixture.phases[1]&&p.total===fixture.domainRows&&p.completed>=fixture.rowFrontier&&p.completed<p.total){reached=true;abort.abort();}}})).rejects.toHaveProperty("kind",fixture.refusal);expect(reached).toBe(true);
});
test("complete TIFF export cancels within physical corpus tree grouping",async()=>{
 const abort=new AbortController();let reached=false;await expect(tiffSnapshotToSqliteFile(corpus,{signal:abort.signal,onProgress:p=>{if(p.phase===fixture.phases[2]&&p.total===fixture.domainValueCount&&p.completed>=fixture.treeFrontier&&p.completed<p.total){reached=true;abort.abort();}}})).rejects.toHaveProperty("kind",fixture.refusal);expect(reached).toBe(true);
});
test("complete TIFF import retains prior debt and honors exact and minus-one cumulative grants",async()=>{
 const file=await tiffSnapshotToSqliteFile(small),measured=new SqliteOperation();expect(await tiffSnapshotFromSqliteFile(file,measured)).toEqual(small);expect(measured.ownedBytes).toBeGreaterThan(0);
 const exact=new SqliteOperation({maxAllocationBytes:measured.ownedBytes+fixture.retainedDebt});exact.allocateBytes(fixture.retainedDebt);expect(await tiffSnapshotFromSqliteFile(file,exact)).toEqual(small);expect(exact.remainingBytes()).toBe(0);
 const short=new SqliteOperation({maxAllocationBytes:measured.ownedBytes+fixture.retainedDebt-1});short.allocateBytes(fixture.retainedDebt);await expect(tiffSnapshotFromSqliteFile(file,short)).rejects.toHaveProperty("kind","ownershipLimit");expect(short.ownedBytes).toBeGreaterThanOrEqual(fixture.retainedDebt);
});
test("complete TIFF export retains prior debt and captures declarative options before mutation",async()=>{
 const measured=new SqliteOperation();await tiffSnapshotToSqliteFile(small,measured);const valid=new AbortController(),stale=new AbortController();stale.abort();let calls=0;const options={maxAllocationBytes:measured.ownedBytes+fixture.retainedDebt,signal:valid.signal,onProgress:()=>{calls++;}},operation=new SqliteOperation(options);operation.allocateBytes(fixture.retainedDebt);options.maxAllocationBytes=0;options.signal=stale.signal;options.onProgress=()=>{throw new Error("mutated callback escaped canonical operation");};expect(await tiffSnapshotFromSqliteFile(await tiffSnapshotToSqliteFile(small,operation))).toEqual(small);expect(operation.remainingBytes()).toBe(0);expect(calls).toBeGreaterThan(0);
 const short=new SqliteOperation({maxAllocationBytes:measured.ownedBytes+fixture.retainedDebt-1});short.allocateBytes(fixture.retainedDebt);await expect(tiffSnapshotToSqliteFile(small,short)).rejects.toHaveProperty("kind","ownershipLimit");expect(short.ownedBytes).toBeGreaterThanOrEqual(fixture.retainedDebt);
});
test("complete TIFF export cancels inside the second physical tree grouping pass",async()=>{
 const abort=new AbortController();let passes=0;await expect(tiffSnapshotToSqliteFile(corpus,{signal:abort.signal,onProgress:p=>{if(p.phase===fixture.phases[2]&&p.total===fixture.domainValueCount&&p.completed===fixture.treeFrontier&&++passes===fixture.groupingPass)abort.abort();}})).rejects.toHaveProperty("kind",fixture.refusal);expect(passes).toBe(fixture.groupingPass);
});
