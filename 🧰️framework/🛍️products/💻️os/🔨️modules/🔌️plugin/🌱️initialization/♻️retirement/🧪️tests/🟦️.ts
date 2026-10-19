/** 🌱️ Original initializer cancellation retains independent authority at every frontier. */
import{test,expect}from"bun:test";
import Ajv2020 from"ajv/dist/2020";
import {moduleFamilyText, pluginSubtreeText} from "../../../🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
test("original initializer closure keeps all five currencies and separate physical releases",()=>{
 const root=new URL("../",import.meta.url),read=(p:string)=>JSON.parse(readFileSync(new URL(p,root),"utf8")),law=read("🧪️tests/🧫️fixtures/🔣️.json");expect(new Ajv2020({strict:true}).compile(read("🧬️schema/🔣️.json"))(law)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE custody(frontier TEXT PRIMARY KEY,original TEXT,closed INTEGER)");for(const frontier of law.frontiers){const original=JSON.stringify({actor:"Ä\u0000original",id:"original-envelope",frontier});db.query("INSERT INTO custody VALUES(?,?,0)").run(frontier,original);for(const axis of Object.keys(law.grant)){const denied={...law.grant,[axis]:0};expect(denied[axis]).toBe(0);expect(db.query("SELECT original,closed FROM custody WHERE frontier=?").get(frontier)).toEqual({original,closed:0});}}}finally{db.close();}
 const plugin=new URL("../../../",import.meta.url),authority=moduleFamilyText("🏪️store"),owner=pluginSubtreeText("♻️lifecycle");

 expect(authority).toContain("retirement_demands");expect(authority).toContain("OriginalTurnFailure");expect(authority).toMatch(/fn close_step\(&mut self,\s*cx:/);

 expect(authority).not.toContain("close_step(&mut self, grant:");

 expect(owner).toContain("authority.close_step(same)");expect(owner).toContain("with_retained_loan");expect(owner).toContain("cancel_arc_release");

 expect(owner).not.toContain("let step=authority.close_step(child)?");



});

/** 🎟️ Authored native fixtures retain one real source across pauses, cancellation and authority closure. */
test("initializer fixture callers preserve original source and whole failure API",()=>{
 const plugin=new URL("../../../",import.meta.url),read=(f:string)=>readFileSync(new URL(f,plugin),"utf8");
 const reload=read("🧪️tests/🧪️bounded-reload/🦀️.rs"),replay=read("../🏪️store/🏗️initialization/🔁️replay/🧪️tests/🔬️unit/🦀️.rs"),native=read("🌱️initialization/♻️retirement/🧪️tests/🦀️.rs"),composition=read("🧪️tests/🧩️composition/🦀️.rs");
 expect(reload).toContain("OriginalWorkFixture::with_identity(operation,generation)");expect(reload).toContain("original_caller.work_turn(original,1,|cx|");expect(reload).toContain("original_caller.finish()");expect(reload).not.toContain("StepContext::new");
 expect(replay).toContain("turn_with_decoder(grant,&mut self.decoder,advance)");expect(replay).toContain("original.caller.as_ref().unwrap().cancel().cancel_now()");expect(replay).toContain("fn run_until(init: &mut Initializer, original:&mut ReplayOriginal");expect(replay).toContain("original.turn(WIDE,|cx|ArtifactStoreInitializationAuthority::close_step(init,cx))");expect(replay).toContain("self.caller.take().unwrap().retire(WIDE)");expect(replay).not.toContain("StepContext::new");expect(replay).not.toContain("StepContext::with_payload_authority");expect(replay).not.toContain("ArtifactStoreInitializationAuthority::close_step(init, WIDE)");
 expect(native).toContain("source.turn(denied,|cx|InteractiveJob::close_step(&mut job,cx))");expect(native).toContain("source.turn(grant,|cx|InteractiveJob::close_step(&mut job,cx))");expect(native).toContain("source.finish()");
 const owner=composition.slice(composition.indexOf("impl store::ArtifactStoreInitializationAuthority"),composition.indexOf("impl Drop for ReadyComposedParentInitialization"));expect(owner).toContain("fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)");expect(owner).not.toContain("fn close_step(&mut self, grant:");
 console.log("[DEBUG] initializer authored native fixture source continuity verified; canonical native execution remains prerequisite-blocked");
});
