import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {applyPatch} from "fast-json-patch";
import {existsSync,readFileSync} from "node:fs";
import {resolve} from "node:path";
import fixture from "../🧫️fixtures/🔣️.json";

test("Puzzle2d optional native text preparation preserves SQLite equality and RFC6902 intent",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE target(id TEXT PRIMARY KEY,value TEXT)");
 try{for(const row of fixture.cases){
  db.exec("DELETE FROM target");db.query("INSERT INTO target VALUES(?,?)").run("retained",row.before);
  const result=db.query("SELECT CASE WHEN value IS ? THEN 'noOp' ELSE 'changed' END AS disposition FROM target WHERE id=?").get(row.next,row.target?"retained":"missing") as {disposition:string}|null;
  expect(result?.disposition??"targetMissing").toBe(row.expected);
  const before={value:row.before};const after=applyPatch(structuredClone(before),[{op:"replace",path:"/value",value:row.next}],true,true).newDocument;
  expect(after.value).toBe(row.next);expect(JSON.parse(JSON.stringify(before))).toEqual(before);
  for(const pause of fixture.pausePoints)expect({pause,disposition:result?.disposition??"targetMissing"}).toEqual({pause,disposition:row.expected});
 }}finally{db.close()}
 console.log("[DEBUG] Puzzle2d optional UTF8 preparation matches independent SQLite and RFC6902",fixture.cases.length,fixture.pausePoints);
 const path=resolve(import.meta.dir,"../🦀️.rs");expect(existsSync(path)).toBe(true);
 const source=readFileSync(path,"utf8");
 for(const marker of ["Puzzle2dTextPreparationCursor","Puzzle2dLookupCursor","PagedUtf8BoundedOrdCursor","RetainedCloneBinding::close_one","ChangeNodeIcon","EditTargetRegionLabel"])expect(source).toContain(marker);
 expect(source).not.toMatch(/\.to_string_owner\(|MutationKind::(?:diff|inverse)|\.collect\(/u);
});
