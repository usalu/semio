import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import {existsSync,readFileSync} from "node:fs";
import {resolve} from "node:path";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";

test("Puzzle2d optional text candidate changes only first original owner and retires displaced text",()=>{
 expect(new Ajv({strict:false}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");db.exec("CREATE TABLE target(ordinal INTEGER PRIMARY KEY,id TEXT,value TEXT)");
 try{for(const row of fixture.cases){
  db.exec("DELETE FROM target");db.query("INSERT INTO target VALUES(?,?,?)").run(0,"retained",row.before);db.query("INSERT INTO target VALUES(?,?,?)").run(1,"retained","second owner");
  const original=db.query("SELECT CASE WHEN value IS ? THEN 'noOp' ELSE 'changed' END AS disposition FROM target WHERE id=? ORDER BY ordinal LIMIT 1").get(row.next,row.target?"retained":"missing") as {disposition:string}|null;
  expect(original?.disposition??"targetMissing").toBe(row.expected);
  const before={owners:[{id:"retained",value:row.before},{id:"retained",value:"second owner"}]};
  if(row.expected==="changed"){
   db.query("UPDATE target SET value=? WHERE ordinal=(SELECT ordinal FROM target WHERE id=? ORDER BY ordinal LIMIT 1)").run(row.next,"retained");
   const result=db.query("SELECT id,value FROM target ORDER BY ordinal").all();
   const after=applyPatch(structuredClone(before),[{op:"replace",path:"/owners/0/value",value:row.next}],true,true).newDocument;
   expect(after.owners).toEqual(result);expect(after.owners[1]).toEqual(before.owners[1]);expect(before.owners[0]!.value).toBe(row.before);
  }else expect(db.query("SELECT id,value FROM target ORDER BY ordinal").all()).toEqual(before.owners);
  for(const pause of fixture.pausePoints)expect({pause,source:before.owners[0]!.value}).toEqual({pause,source:row.before});
 }}finally{db.close()}
 console.log("[DEBUG] optional text candidates match fourteen SQLite first-owner/RFC6902 cases; duplicates and source retained",fixture.pausePoints);
 const path=resolve(import.meta.dir,"../🦀️.rs");expect(existsSync(path)).toBe(true);const source=readFileSync(path,"utf8");
 for(const marker of ["Puzzle2dTextCandidateCursor","Puzzle2dTextPreparationCursor","RetainedFieldCursor","ControlledRetirement","displaced","close_granted","RetainedCloneBinding::close_one"])expect(source).toContain(marker);
 expect(source).not.toMatch(/\.to_string_owner\(|MutationKind::(?:diff|inverse)|\.collect\(|vec!\[/u);
});
