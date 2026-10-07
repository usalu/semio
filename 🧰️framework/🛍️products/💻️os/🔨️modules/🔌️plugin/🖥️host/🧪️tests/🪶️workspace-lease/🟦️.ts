import {test,expect} from "bun:test";

import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
const root=fileURLToPath(new URL("../../",import.meta.url));
const read=(path:string)=>JSON.parse(readFileSync(join(root,path),"utf8"));
const corpus=()=>read("🧫️fixtures/🪶️workspace-lease/🔣️.json");
test("closed real Count lease contract selects complete native encodings and eight distinct compiled refusal owners",()=>{
 const fixture = corpus();
 expect(fixture.counts).toEqual([-2147483648,-1,0,1,2147483647]);
 expect(fixture.encodings).toEqual(["binary","text"]);
 expect(new Set(fixture.refusals.map((row:{owner:string})=>row.owner)).size).toBe(8);
 expect(fixture.refusals.map((row:{kind:string})=>row.kind)).toEqual(["InvalidValue","Canceled","OwnershipLimit","AllocationFailed","WorkLimit","DepthLimit","UnsupportedOwner","InvariantViolated"]);
 
});
test("independent SQLite validates every Count row for both encodings and exact dialect metadata",()=>{
 const fixture=corpus();
 const sql=readFileSync(join(root,"🧪️testing/🧩️component/🧬️schema/📸️snapshot/🪶️sqlite/🗄️.sql"),"utf8");
 const metadata=readFileSync(join(process.cwd(),"🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧬️schema/🗄️.sql"),"utf8");
 for(const encoding of fixture.encodings)for(const count of fixture.counts){
  const db=new Database(":memory:");try{
   db.exec(sql);db.exec(metadata);
   db.query("INSERT INTO fixture_counter VALUES(1,?)").run(count);
   db.query("INSERT INTO semio_snapshot VALUES(1,?,?,?,1,?)").run("fixture.neutral-host-fixture.counter","1","*",encoding);
   const reopened=Database.deserialize(db.serialize());try{
    expect(reopened.query("SELECT id,count,typeof(count) AS storage FROM fixture_counter").get()).toEqual({id:1,count,storage:"integer"});
    expect(reopened.query("SELECT artifact_kind,standard,subset,schema_version,native_encoding FROM semio_snapshot").get()).toEqual({artifact_kind:"fixture.neutral-host-fixture.counter",standard:"1",subset:"*",schema_version:1,native_encoding:encoding});
    expect(reopened.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all()).toEqual([{name:"fixture_counter"},{name:"semio_snapshot"}]);
    expect(reopened.query("PRAGMA table_info(fixture_counter)").all().map(row=>(row as {name:string}).name)).toEqual(["id","count"]);
    expect(reopened.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
    expect(reopened.query("PRAGMA foreign_key_check").all()).toEqual([]);
   }finally{reopened.close();}
  }finally{db.close();}
 }
});
test("independent UTF8 oracle preserves typed cause and every NUL diagnostic field without prose classification",()=>{
 const fixture=corpus();
 for(const row of fixture.refusals){
  const wire=new TextEncoder().encode(JSON.stringify({kind:row.kind,message:fixture.message,diagnostics:[fixture.diagnostic]}));
  const decoded=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(wire));
  expect(decoded.kind).toBe(row.kind);expect(decoded.message).toBe(fixture.message);
  expect(decoded.diagnostics).toEqual([fixture.diagnostic]);expect(decoded.message.includes("\u0000")).toBe(true);
  expect(decoded.diagnostics[0].message.includes("\u0000")).toBe(true);
  for(const values of Object.values(decoded.diagnostics[0].expected) as string[][])expect(values.every(value=>value.includes("\u0000"))).toBe(true);
 }
});
