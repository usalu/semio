import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {existsSync,readFileSync} from "node:fs";
import {resolve} from "node:path";
import fixture from "../🧫️fixtures/🔣️.json";

test("Puzzle2d optional text diagnostics expose borrowed raw ID fragments and exact severity",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE target(ordinal INTEGER PRIMARY KEY,id TEXT,value TEXT)");
 try{for(const row of fixture.cases){
  db.exec("DELETE FROM target");
  if(row.target){db.query("INSERT INTO target VALUES(0,?,?)").run(row.id,row.before);db.query("INSERT INTO target VALUES(1,?,?)").run(row.id,"untouched duplicate");}
  const result=db.query("SELECT CASE WHEN value IS ? THEN 'noOp' ELSE 'changed' END AS disposition FROM target WHERE id=? ORDER BY ordinal LIMIT 1").get(row.next,row.id)as {disposition:string}|null;
  const disposition=result?.disposition??"targetMissing",prefix=row.role==="nodeIcon"?"node \"":"target region \"";
  const fragments=disposition==="noOp"?[{kind:"static",text:"no changes to apply"}]:[{kind:"static",text:prefix},{kind:"text",text:row.id},{kind:"static",text:"\" not found"}];
  const actual=disposition==="changed"?null:{level:disposition==="noOp"?"warning":"error",code:disposition==="noOp"?"mutation.no-op":"mutation.target-missing",body:fragments.map(value=>value.text).join(""),targets:[row.id],fragmentKinds:fragments.map(value=>value.kind)};
  expect(actual).toEqual(row.expected);
  if(actual){const decoded=new TextDecoder().decode(new TextEncoder().encode(actual.body));expect(decoded).toBe(row.expected!.body);expect(JSON.parse(JSON.stringify(actual))).toEqual(row.expected);}
  if(row.target)expect((db.query("SELECT value FROM target WHERE ordinal=1").get()as {value:string}).value).toBe("untouched duplicate");
 }}finally{db.close()}
 console.log("[DEBUG] six SQLite first-owner diagnostics preserve raw quotes/NUL/Unicode, exact severity and untouched duplicate");
 const path=resolve(import.meta.dir,"../🦀️.rs");expect(existsSync(path)).toBe(true);
 const source=readFileSync(path,"utf8");
 for(const marker of["Puzzle2dTextDiagnosticSource","ArtifactMessageSource","ArtifactMessageFragment::Text","Puzzle2dTextDisposition::TargetMissing","operation_index"])expect(source).toContain(marker);
 expect(source).not.toMatch(/format!|\.to_string_owner\(|ArtifactMessageFragment::JsonQuoted|\.clone\(/u);
});
