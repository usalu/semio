/** ⏭️ Borrowed SQLite and native census yield through the actual host continuation port. */
import {expect,test,spyOn} from "bun:test";
import {Database} from "bun:sqlite";

import fixture from "./🧫️fixtures/🔣️.json";
import {hostContinuations} from "../../../../⏳️async/🪃️continuation/🟦️.ts";
import {NativeDecodeControl} from "../../../../🌱️value/🛬️decode/🟦️.ts";
import {parseSqliteDatabaseSchemaControlled} from "../../🟦️.ts";
import {artifactSqliteCheckpoint} from "../../🧩️artifact/🟦️.ts";

test("bounded borrowed census keeps real host continuation turns and timer cancellation",async()=>{
 const law=fixture as {authority:string;unit:string;repeat:number;utf8Bytes:number;checkpointSources:string[]};
 const literal=law.unit.repeat(law.repeat);expect(Buffer.byteLength(literal,"utf8")).toBe(law.utf8Bytes);
 const sql="CREATE TABLE continuation (id INTEGER PRIMARY KEY,label TEXT DEFAULT '"+literal+"')",oracle=new Database(":memory:");
 try{oracle.exec(sql);oracle.exec("INSERT INTO continuation(id) VALUES(1)");expect(oracle.query("SELECT length(CAST(label AS BLOB)) AS bytes FROM continuation").get()).toEqual({bytes:law.utf8Bytes});}finally{oracle.close()}
 const original=hostContinuations.yieldContinuation,turns=spyOn(hostContinuations,"yieldContinuation").mockImplementation(()=>original());
 try{
  let callbacks=0;await parseSqliteDatabaseSchemaControlled(sql,{onProgress:()=>callbacks++});expect(callbacks).toBeGreaterThan(0);expect(turns.mock.calls.length).toBe(callbacks);
  turns.mockClear();callbacks=0;const native=new NativeDecodeControl(0,()=>{callbacks++;return true});await native.beginStage(1);await native.step();expect(turns.mock.calls.length).toBe(callbacks);expect(native.ownedBytes).toBe(0);
  turns.mockClear();await artifactSqliteCheckpoint({},"projectSnapshot",0,1);expect(turns.mock.calls.length).toBe(1);
  const abort=new AbortController(),timer=setTimeout(()=>abort.abort(),0);let refused=false;
  try{for(let count=0;count<10000&&!refused;count++){try{await artifactSqliteCheckpoint({signal:abort.signal},"projectSnapshot",0,1);}catch(error){expect(error).toMatchObject({kind:"canceled"});refused=true}}expect(refused).toBe(true);}finally{clearTimeout(timer)}
 }finally{turns.mockRestore()}
});
