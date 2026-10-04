/** 💰️ Physical scalar work and owned SQLite operation authority. */
import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import fixture from "../../🧫️fixtures/💰️operation/🔣️.json";
import schema from "../../🧬️schema/💰️operation/🔣️.json";
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase,SqliteOperation,SqliteAllocationControl,type SqliteDatabase,type SqliteDatabaseOptions} from "../../🟦️.ts";
import {ArtifactSqliteProjection} from "../../🧩️artifact/🟦️.ts";

async function database(label=fixture.labelUnit):Promise<SqliteDatabase>{const p=await ArtifactSqliteProjection.create(fixture.sql);await p.insert(fixture.table,[label,Uint8Array.from(fixture.payload)],1n);return p.finish();}
test("complete operation corpus has closed neutral schema and independent scalar/debt authority",async()=>{
 const admit=new Ajv({strict:true}).compile(schema);expect(admit(fixture)).toBe(true);expect(validateJsonSchemaSubset(schema,fixture)).toEqual([]);
 for(const hostile of [{...fixture,optionalSecondAuthority:true},{...fixture,tightBytes:1},{...fixture,utf8Bytes:fixture.utf8Bytes-1}]){expect(admit(hostile)).toBe(false);expect(validateJsonSchemaSubset(schema,hostile).length).toBeGreaterThan(0);}
 const label=fixture.labelUnit.repeat(fixture.repeats);expect(label.length).toBe(fixture.codeUnits);const db=Database.deserialize(await exportSqliteDatabase(await database(label)));
 try{expect((await importSqliteDatabase(db.serialize())).tables.find(t=>t.name===fixture.table)!.rows[0]!.values[1]).toBe(label);expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("SELECT length(CAST(label AS BLOB)) AS bytes,hex(payload) AS payload FROM authored_scalar").get()).toEqual({bytes:fixture.utf8Bytes,payload:"00FF4100"});db.run("CREATE TABLE requests(id INTEGER PRIMARY KEY,bytes INTEGER NOT NULL)");for(const[id,bytes]of fixture.allocationRequests.entries())db.run("INSERT INTO requests VALUES(?,?)",[id,bytes]);expect(db.query("SELECT sum(bytes) AS bytes FROM requests").get()).toEqual({bytes:8});}finally{db.close();}
});
test("physical measurement cancels inside one Unicode scalar before any encoding backing",async()=>{
 const value=await database(fixture.labelUnit.repeat(fixture.repeats)),signal=new AbortController();let reached=false,encoded=0;const encode=TextEncoder.prototype.encodeInto;
 TextEncoder.prototype.encodeInto=function(source,destination){encoded++;return encode.call(this,source,destination);};
 try{await expect(exportSqliteDatabase(value,{signal:signal.signal,onProgress:p=>{if(p.phase===fixture.measurePhase&&p.total===fixture.codeUnits&&p.completed>=fixture.measureFrontier&&p.completed<p.total){reached=true;signal.abort();}}})).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true);expect(encoded).toBe(0);}finally{TextEncoder.prototype.encodeInto=encode;}
});
test("physical import decoder cancels inside one UTF-8 scalar without later decoding",async()=>{
 const bytes=await exportSqliteDatabase(await database(fixture.labelUnit.repeat(fixture.repeats))),signal=new AbortController();let reached=false,later=0;const decode=TextDecoder.prototype.decode;
 TextDecoder.prototype.decode=function(input,options){if(reached)later++;return decode.call(this,input,options);};
 try{await expect(importSqliteDatabase(bytes,{signal:signal.signal,onProgress:p=>{if(p.phase===fixture.decodePhase&&p.total===fixture.utf8Bytes&&p.completed>=fixture.decodeFrontier&&p.completed<p.total){reached=true;signal.abort();}}})).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true);expect(later).toBe(0);}finally{TextDecoder.prototype.decode=decode;}
});
async function injected(route:string,value:SqliteDatabase,bytes:Uint8Array,options:SqliteDatabaseOptions,ledger:SqliteAllocationControl):Promise<unknown>{
 if(route==="artifactProjection"){const p=await Reflect.apply(ArtifactSqliteProjection.create,ArtifactSqliteProjection,[fixture.sql,options,ledger]) as ArtifactSqliteProjection;await p.insert(fixture.table,[fixture.labelUnit,Uint8Array.from(fixture.payload)],1n);return p.finish();}
 return Reflect.apply(route==="physicalExport"?exportSqliteDatabase:importSqliteDatabase,null,[route==="physicalExport"?value:bytes,options,ledger]);
}
for(const route of fixture.routes){
 test(`actual ${route} call grant cannot be overridden by an injected permissive ledger`,async()=>{const value=await database(),bytes=await exportSqliteDatabase(value),ledger=new SqliteAllocationControl({maxAllocationBytes:fixture.permissiveBytes});ledger.admit(fixture.retainedDebt);await expect(injected(route,value,bytes,{maxAllocationBytes:fixture.tightBytes},ledger)).rejects.toHaveProperty("kind",fixture.refusal);expect(ledger.ownedBytes).toBe(fixture.retainedDebt);});
 test(`actual ${route} signal belongs to its call rather than an injected stale ledger`,async()=>{const value=await database(),bytes=await exportSqliteDatabase(value),oldSignal=new AbortController(),signal=new AbortController(),ledger=new SqliteAllocationControl({maxAllocationBytes:fixture.permissiveBytes,signal:oldSignal.signal});ledger.admit(fixture.retainedDebt);oldSignal.abort();await expect(injected(route,value,bytes,{signal:signal.signal,maxAllocationBytes:fixture.permissiveBytes},ledger)).resolves.toBeDefined();expect(ledger.ownedBytes).toBe(fixture.retainedDebt);});
}
