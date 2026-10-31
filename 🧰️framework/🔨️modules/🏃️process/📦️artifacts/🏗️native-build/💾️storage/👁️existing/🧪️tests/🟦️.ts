import{test,expect}from"bun:test";
import{mkdtempSync,writeFileSync,readFileSync,lstatSync}from"node:fs";
import{join,resolve}from"node:path";
import{createHash}from"node:crypto";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{replayExistingNativeStorageJournalV1}from"../🟦️.ts";
import{withGraphTestInvocationV1}from"../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧰️support/🟦️.ts";
import{createIssuedResourceControlV1}from"@semio-tech/process/routing";
import nativeSchema from"../../🧬️schema/🔣️.json";
import schema from"../🔣️schema.json";
import fixture from"./🔣️.json";

const wire=fixture.events.map(event=>JSON.stringify(event)).join("\n")+"\n";
test("fresh existing journal replay preserves all active phases and refuses incomplete input",()=>withGraphTestInvocationV1(async original=>{
 const control=createIssuedResourceControlV1(original,"Native:shared-replay-law",0),result=await replayExistingNativeStorageJournalV1(wire,fixture.capacityBytes,control),db=new Database(":memory:");try{db.exec("create table charges(id text primary key,bytes integer,phase text)");for(const event of fixture.events){if(event.type==="reserved")db.query("insert into charges values(?,?,?)").run(event.allocation!.id,event.allocation!.bytes,"reserved");else db.query("update charges set phase=? where id=?").run(event.type,event.id!);}expect(result.state.allocations.reduce((sum,row)=>sum+row.bytes,0)).toBe((db.query("select sum(bytes) used from charges").get()as any).used);expect(result.events).toBe(fixture.events.length);expect(result.state.allocations.map(row=>row.phase)).toEqual(["reserved","committed","retiring"]);for(const value of["",wire.slice(0,-1),'{"type":"unknown"}\n'])await expect(replayExistingNativeStorageJournalV1(value,fixture.capacityBytes,control)).rejects.toThrow();}finally{db.close();}
}));
