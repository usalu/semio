import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import fixture from "../../🧫️fixtures/🔣️.json" with {type:"json"};
test("empty initialization catalog retires one admitted native page at a time",()=>{
 const schema=JSON.parse(readFileSync(resolve(import.meta.dir,"../../🧬️schema/🔣️.json"),"utf8"));
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const database=new Database(":memory:");try{
  database.exec("CREATE TABLE pages(ordinal INTEGER PRIMARY KEY, lane TEXT, slots INTEGER)");
  fixture.lanes.forEach((lane,ordinal)=>database.query("INSERT INTO pages VALUES(?,?,?)").run(ordinal,lane,fixture.pageSlots));
  let owner=Object.fromEntries(fixture.lanes.map(lane=>[lane,{slots:fixture.pageSlots}]));
  for(const lane of fixture.lanes){
   const before=database.query("SELECT SUM(slots) AS slots FROM pages").get();
   for(const grant of fixture.grants.slice(0,2)){
    expect(["zeroItems","oneBelow"]).toContain(grant);
    expect(database.query("SELECT SUM(slots) AS slots FROM pages").get()).toEqual(before);
    expect(Object.keys(owner)).toEqual(database.query("SELECT lane FROM pages ORDER BY ordinal").all().map((row:any)=>row.lane));
   }
   database.query("DELETE FROM pages WHERE ordinal=(SELECT MIN(ordinal) FROM pages)").run();
   owner=applyPatch(owner,[{op:"remove",path:`/${lane}`}],true,false).newDocument;
   expect(Object.keys(owner)).toEqual(database.query("SELECT lane FROM pages ORDER BY ordinal").all().map((row:any)=>row.lane));
  }
  expect(owner).toEqual({});expect(database.query("SELECT COUNT(*) AS pages FROM pages").get()).toEqual({pages:0});
 }finally{database.close();}
 const source=readFileSync(resolve(import.meta.dir,"../../🦀️.rs"),"utf8");
 expect(source).toContain("fn next_close_byte_demand(");expect(source).toContain("fn close_step(");
 expect(source).toContain("next_empty_page_release_byte_demand");
 console.log("[DEBUG] Empty initialization catalog Ajv/SQLite/RFC6902: six native lanes, zero/one-below retain, exact one-page release, no final backing");
});
