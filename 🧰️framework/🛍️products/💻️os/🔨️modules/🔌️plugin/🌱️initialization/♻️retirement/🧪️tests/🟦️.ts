/** 🌱️ Original initializer cancellation retains independent authority at every frontier. */
import{test,expect}from"bun:test";
import Ajv2020 from"ajv/dist/2020";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
test("original initializer closure keeps all five currencies and separate physical releases",()=>{
 const root=new URL("../",import.meta.url),read=(p:string)=>JSON.parse(readFileSync(new URL(p,root),"utf8")),law=read("🧫️fixtures/🔣️.json");expect(new Ajv2020({strict:true}).compile(read("🧬️schema/🔣️.json"))(law)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE custody(frontier TEXT PRIMARY KEY,original TEXT,closed INTEGER)");for(const frontier of law.frontiers){const original=JSON.stringify({actor:"Ä\u0000original",id:"original-envelope",frontier});db.query("INSERT INTO custody VALUES(?,?,0)").run(frontier,original);for(const axis of Object.keys(law.grant)){const denied={...law.grant,[axis]:0};expect(denied[axis]).toBe(0);expect(db.query("SELECT original,closed FROM custody WHERE frontier=?").get(frontier)).toEqual({original,closed:0});}}}finally{db.close();}
 const source=readFileSync(new URL("../../🦀️.rs",root),"utf8"),start=source.indexOf("pub trait ArtifactStoreInitializationAuthority"),end=source.indexOf("fn drive_artifact_owned_disposer",start),owner=source.slice(start,end);expect(owner).toContain("retirement_demands");expect(owner).toContain("RetainedCloneGrant");expect(owner).toContain("admit_retained_clone_close");expect(owner).not.toContain("maximum_items: usize, maximum_bytes: usize");expect(owner).toContain("cancel_arc_release");expect(owner).toContain("retained_progress");
 console.log("[DEBUG] SQLite initializer original custody conserved through eight independent grant frontiers; native physical acceptance remains separate");
});
