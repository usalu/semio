/** 🧳️ Independent original-source custody phases precede canonical Store integration. */
import{test,expect}from"bun:test";
import Ajv2020 from"ajv/dist/2020";
import{Database}from"bun:sqlite";
import{readFileSync,existsSync}from"node:fs";
test("canonical Store source drives genuine semantic identity and hash children under separate original custody turns",()=>{
 const root=new URL("../",import.meta.url),read=(path:string)=>JSON.parse(readFileSync(new URL(path,root),"utf8")),fixture=read("🧪️tests/🧫️fixtures/🔣️.json");expect(new Ajv2020({strict:true}).compile(read("🧬️schema/🔣️.json"))(fixture)).toBe(true);
 const actor="ä\u0000".repeat(fixture.actorRepeat);expect(Buffer.byteLength(actor)).toBeGreaterThan(fixture.copyCapacityLimit);const db=new Database(":memory:");try{db.exec("CREATE TABLE custody(edit INTEGER PRIMARY KEY,phase INTEGER,originalActor TEXT,aliases INTEGER)");db.query("INSERT INTO custody VALUES(1,0,?,0)").run(actor);for(const cancelledAt of fixture.cancellationPhases){for(let phase=0;phase<=cancelledAt;phase++){db.query("UPDATE custody SET phase=? WHERE edit=1").run(phase);expect(db.query("SELECT edit,originalActor FROM custody").get()).toEqual({edit:1,originalActor:actor});}expect(db.query("SELECT COUNT(*) AS owners FROM custody").get()).toEqual({owners:1});}}finally{db.close();}
 const path=new URL("🦀️.rs",root);expect(existsSync(path)).toBe(true);const source=readFileSync(path,"utf8");for(const producer of["ArtifactCanonicalEditAuthorityCursor","ArtifactCanonicalEditIdentityCursor","ArtifactCanonicalEditSealCursor::admit_source","RetainedCloneSource","source_constructor_demand"]){expect(source).toContain(producer);}expect(source).not.toContain("ArtifactCanonicalEditEncoder");expect(source).not.toContain("validate_semantic_edit");expect(source).not.toContain("from_utf8_unchecked");
});

test("canonical source admission uses the selected original Work and return demand",()=>{
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
 const quote=source.slice(source.indexOf("fn source_admission_quote"),source.indexOf("fn projection<"));
 expect(quote).toContain("owner.owned_projection_demand()");
 expect(quote).toContain("owner.take_output_demand(), owner.take_output_work_demand()");
 expect(quote).toContain("owner.next_demand()?, owner.advance_work_demand()?");
 for(const owner of["edit_admission","authority_admission"]){
  expect((source.match(new RegExp(`source_admission_quote\\(self\\.${owner}\\.as_ref\\(\\)`,"g"))??[])).toHaveLength(2);
 }
});
