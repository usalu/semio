import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {readModuleTreeSource} from "@semio-tech/framework-compiler-ts/syntax/module-tree";
test("history acceptance search discards blocked trials before selecting a ready draft",()=>{
 const read=(path:string)=>readFileSync(resolve(import.meta.dir,path),"utf8");const fixture=JSON.parse(read("🧫️fixtures/🔣️.json"));
 for(const row of fixture.cases){
  const db=new Database(":memory:");db.exec("CREATE TABLE committed (trial INTEGER)");let selected:number|null=null;
  for(let index=0;index<row.attempts.length;index++){
   db.exec("BEGIN");db.query("INSERT INTO committed VALUES (?)").run(index);
   if(row.attempts[index]==="blocked"){db.exec("ROLLBACK");expect(db.query("SELECT count(*) AS n FROM committed").get()).toEqual({n:0});if(row.mode==="fixed")break;continue;}
   db.exec("COMMIT");selected=index;break;
  }
  expect(selected).toBe(row.expected);expect(db.query("SELECT trial FROM committed").all()).toEqual(row.expected===null?[]:[{trial:row.expected}]);db.close();
 }
 const source=readModuleTreeSource(resolve(import.meta.dir,"../../🦀️.rs"));const start=source.indexOf("for candidate in changes {");const end=source.indexOf("let Some(drafted) = drafted",start);const search=source.slice(start,end);
 expect(search).toContain("change.is_none() && app.time_travel.status().is_some_and(|status| status.review == Some(HistoryTimeTravelReview::Blocked))");
 expect(search).toContain("HISTORY_EDIT_EXIT_ACTION_ID");expect(search).toContain("discarding a blocked trial left a supersession of the mutation");
});

test("a discarded blocked trial leaves no preview owner the pump stops on while the next selected input would be refused",()=>{
 const tree=resolve(import.meta.dir,"../../../../⏪️time-travel");
 const state=readFileSync(resolve(tree,"🗄️store-state/🦀️.rs"),"utf8");const visitors=readFileSync(resolve(tree,"🧱️store-owners-and-member-visitors/🦀️.rs"),"utf8");
 const live=state.slice(state.indexOf("fn preview_owners_live("),state.indexOf("fn cancel_preview("));
 for(const owner of ["preview_job","preview_derived","preview_original","preview_source","preview_native_input","preview_input_close","preview_original_close"])expect(live).toContain(`self.${owner}.is_some()`);
 const preview=state.slice(state.indexOf("TimeTravelStoreCommand::Preview {"),state.indexOf("TimeTravelStoreCommand::StepPreview"));
 expect(preview).toContain("if self.preview_owners_live(){self.cancel_preview(store)?;return Err(");expect(preview).toContain("original preview cancellation must settle before another selected input");
 const pending=visitors.slice(visitors.indexOf("fn has_pending_work("),visitors.indexOf("fn terminal_is_empty("));
 expect(pending).toContain("self.preview_cancelled&&(self.preview_owners_live()");
});
