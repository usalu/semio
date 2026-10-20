/** 📨️ Original OS payload source laws retain the declared Toy caller authority. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync,existsSync} from "node:fs";

test("original tick payload uses the same declared Toy caller policy and paid borrowed context",()=>{
 const policy=JSON.parse(readFileSync(new URL("../../../../🧪️tests/🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🧾️fixture-caller/🔣️.json",import.meta.url),"utf8"));
 expect(policy.native.grant).toEqual([1,16384,65536,1048576,4096]);
 const db=new Database(":memory:");try{
  db.exec("CREATE TABLE policy(position INTEGER PRIMARY KEY,value INTEGER)");
  for(const[i,value]of policy.native.grant.entries())db.query("INSERT INTO policy VALUES(?,?)").run(i,value);
  expect(db.query("SELECT value FROM policy ORDER BY position").all().map(row=>(row as {value:number}).value)).toEqual(policy.native.grant);
 }finally{db.close();}
 const path=new URL("../🦀️.rs",import.meta.url);expect(existsSync(path)).toBe(true);const source=readFileSync(path,"utf8");
 for(const token of["advance_original_tick_payload","append_original","consume_retained","acknowledge_chunk","retained_grant"])expect(source).toContain(token);
 for(const token of["payload_from_bytes","Arc::new","StepContext::new",".finish()",".encode()","to_vec()"])expect(source).not.toContain(token);
});

import Ajv from "ajv";
const read=(path:string)=>JSON.parse(readFileSync(new URL(path,import.meta.url),"utf8"));

test("writer and native tick sources preserve original bytes across bounded range relay",()=>{

 const law=read("./🧫️fixtures/🔣️.json");expect(new Ajv({strict:true}).compile(read("../🧬️schema/🔣️.json"))(law)).toBe(true);

 const db=new Database(":memory:");try{db.exec("CREATE TABLE original(mode TEXT PRIMARY KEY,payload BLOB)");for(const mode of law.sources){const original=Uint8Array.from({length:law.payloadBytes},()=>law.byte);db.query("INSERT INTO original VALUES(?,?)").run(mode,original);for(const cut of law.cancelCuts){const maximum=cut==="complete"?Infinity:cut,pages:Uint8Array[]=[];for(let offset=0;offset<original.length&&pages.length<maximum;offset+=64){const actual=original.slice(offset,Math.min(offset+64,original.length)),oracle=(db.query("SELECT substr(payload,?,64) AS bytes FROM original WHERE mode=?").get(offset+1,mode)as{bytes:Uint8Array}).bytes;expect(Buffer.from(actual)).toEqual(Buffer.from(oracle));pages.push(actual);}expect((db.query("SELECT payload FROM original WHERE mode=?").get(mode)as{payload:Uint8Array}).payload).toEqual(original);if(cut==="complete")expect(Buffer.concat(pages)).toEqual(Buffer.from(original));}}

 }finally{db.close();}

});
