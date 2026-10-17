/** 🌱️ Original initializer cancellation retains independent authority at every frontier. */
import{test,expect}from"bun:test";
import Ajv2020 from"ajv/dist/2020";
import {moduleFamilyText, pluginSubtreeText} from "../../../🧷️companions/🧯️test-support/🔌️plugin-family/🟦️.ts";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
test("original initializer closure keeps all five currencies and separate physical releases",()=>{
 const root=new URL("../",import.meta.url),read=(p:string)=>JSON.parse(readFileSync(new URL(p,root),"utf8")),law=read("🧪️tests/🧫️fixtures/🔣️.json");expect(new Ajv2020({strict:true}).compile(read("🧬️schema/🔣️.json"))(law)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE custody(frontier TEXT PRIMARY KEY,original TEXT,closed INTEGER)");for(const frontier of law.frontiers){const original=JSON.stringify({actor:"Ä\u0000original",id:"original-envelope",frontier});db.query("INSERT INTO custody VALUES(?,?,0)").run(frontier,original);for(const axis of Object.keys(law.grant)){const denied={...law.grant,[axis]:0};expect(denied[axis]).toBe(0);expect(db.query("SELECT original,closed FROM custody WHERE frontier=?").get(frontier)).toEqual({original,closed:0});}}}finally{db.close();}
 const authority=moduleFamilyText("🏪️store"),start=authority.indexOf("pub trait ArtifactStoreInitializationAuthority"),job=pluginSubtreeText("♻️lifecycle"),owner=authority.slice(start,authority.indexOf("fn terminal_is_empty(&self) -> bool;",start))+job.slice(job.indexOf("pub struct ArtifactStoreInitializationJob"),job.indexOf("impl<P, Mutation> Drop for ArtifactStoreInitializationJob"));expect(start).toBeGreaterThan(0);expect(owner).toContain("retirement_demands");expect(owner).toContain("RetainedCloneGrant");expect(owner).toContain("close_original_retained_step(cx,demand,|grant|");expect(owner).not.toContain("maximum_items: usize, maximum_bytes: usize");expect(owner).toContain("cancel_arc_release");expect(owner).toContain("RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes");
});
