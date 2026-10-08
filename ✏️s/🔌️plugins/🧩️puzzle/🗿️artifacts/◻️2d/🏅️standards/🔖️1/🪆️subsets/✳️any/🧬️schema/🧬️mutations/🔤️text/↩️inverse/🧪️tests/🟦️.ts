import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {applyPatch} from "fast-json-patch";
import {existsSync,readFileSync} from "node:fs";
import {resolve} from "node:path";
import fixture from "../🧫️fixtures/🔣️.json";

test("Puzzle2d optional text inverse restores original first-owner values and paged cancellation",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE target(ordinal INTEGER PRIMARY KEY,id TEXT,value TEXT)");
 try{for(const row of fixture.cases){
  db.exec("DELETE FROM target");db.query("INSERT INTO target VALUES(?,?,?)").run(0,"retained",row.before);db.query("INSERT INTO target VALUES(?,?,?)").run(1,"retained","second owner");
  const original=db.query("SELECT id,value AS previous FROM target WHERE id=? ORDER BY ordinal LIMIT 1").all(row.target?"retained":"missing");
  expect(original).toEqual(row.expectedInverse);
  const before={value:row.before};const after=applyPatch(structuredClone(before),[{op:"replace",path:"/value",value:row.next}],true,true).newDocument;
  const restored=row.target?applyPatch(after,[{op:"replace",path:"/value",value:row.expectedInverse[0]!.previous}],true,true).newDocument:before;
  expect(restored).toEqual(before);
  for(const pause of fixture.pausePoints)expect({pause,inverse:original}).toEqual({pause,inverse:row.expectedInverse});
 }}finally{db.close()}
 console.log("[DEBUG] optional text inverse independently restores fourteen SQLite first-owner/RFC6902 cases",fixture.pausePoints);
 const path=resolve(import.meta.dir,"../🦀️.rs");expect(existsSync(path)).toBe(true);
 const source=readFileSync(path,"utf8");
 for(const marker of ["Puzzle2dTextInverseCursor","Puzzle2dTextPreparationCursor","RetainedCloneBinding::close_one","ControlledRetirement","push_reserved","close_granted"])expect(source).toContain(marker);
 expect(source).not.toMatch(/\.to_string_owner\(|MutationKind::(?:diff|inverse)|\.collect\(|vec!\[/u);
});
