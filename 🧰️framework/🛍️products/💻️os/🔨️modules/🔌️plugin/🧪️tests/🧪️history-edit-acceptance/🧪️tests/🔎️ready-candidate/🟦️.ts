import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
test("history acceptance search discards blocked trials before selecting a ready draft",()=>{
 const read=(path:string)=>readFileSync(resolve(import.meta.dir,path),"utf8");const fixture=JSON.parse(read("🧫️fixtures/🔣️.json"));expect(new Ajv({strict:false}).compile(JSON.parse(read("🧬️schema/🔣️.json")))(fixture)).toBe(true);
 for(const row of fixture.cases){
  const db=new Database(":memory:");db.exec("CREATE TABLE committed (trial INTEGER)");let selected:number|null=null;
  for(let index=0;index<row.attempts.length;index++){
   db.exec("BEGIN");db.query("INSERT INTO committed VALUES (?)").run(index);
   if(row.attempts[index]==="blocked"){db.exec("ROLLBACK");expect(db.query("SELECT count(*) AS n FROM committed").get()).toEqual({n:0});if(row.mode==="fixed")break;continue;}
   db.exec("COMMIT");selected=index;break;
  }
  expect(selected).toBe(row.expected);expect(db.query("SELECT trial FROM committed").all()).toEqual(row.expected===null?[]:[{trial:row.expected}]);db.close();
 }
 const source=read("../../🦀️.rs");const start=source.indexOf("for candidate in changes {");const end=source.indexOf("let Some(drafted) = drafted",start);const search=source.slice(start,end);
 expect(search).toContain("change.is_none() && app.time_travel.status().is_some_and(|status| status.review == Some(HistoryTimeTravelReview::Blocked))");
 expect(search).toContain("HISTORY_EDIT_EXIT_ACTION_ID");expect(search).toContain("discarding a blocked trial left a supersession of the mutation");
 console.log("[DEBUG] independent SQLite rollback and schema preserve blocked trials without durable edits");
});
