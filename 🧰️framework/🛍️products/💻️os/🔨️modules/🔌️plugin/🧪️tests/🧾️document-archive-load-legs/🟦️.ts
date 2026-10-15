import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import schema from "./🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
test("archive status observation preserves retained owner and owns no work authority",()=>{
 const valid=new Ajv({strict:true}).compile(schema);expect(valid(fixture)).toBe(true);expect(validateJsonSchemaSubset(schema,fixture)).toEqual([]);
 const db=new Database(":memory:");db.run("CREATE TABLE archive(operation INTEGER PRIMARY KEY,state TEXT,source BLOB); CREATE TABLE events(sequence INTEGER)");
 try{for(const state of fixture.states){const source=Buffer.from("original archive 源");db.query("INSERT OR REPLACE INTO archive VALUES(?,?,?)").run(fixture.operation,state,source);const first=db.query("SELECT * FROM archive WHERE operation=?").get(fixture.operation);for(let index=0;index<fixture.observations;index++)expect(db.query("SELECT * FROM archive WHERE operation=?").get(fixture.operation)).toEqual(first);expect((db.query("SELECT COUNT(*) AS count FROM events").get()as {count:number}).count).toBe(fixture.eventCount);}}finally{db.close();}
 for(const key of ["wallAuthorityCreated","sourceChanged"]as const)expect(valid({...fixture,[key]:true})).toBe(false);
 const root=resolve(import.meta.dir,"../../../../../../.."),source=["🦀️.rs","🚪️ingress/🦀️.rs","📨️dispatch/🦀️.rs","🖼️render/🦀️.rs","🧷️command-routes/🦀️.rs","🧾️typed-operations/🦀️.rs","🔌️plugin-app-impl/🦀️.rs"].map(s=>readFileSync(resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/"+s),"utf8")).join("\n");
 const poll=source.slice(source.indexOf("    async fn poll_document_archive_load("),source.indexOf("    fn cancel_document_archive_load(",source.indexOf("    async fn poll_document_archive_load(")));
 expect(poll).toContain("ActiveDocumentArchiveLoad::status");for(const forbidden of ["maintenance_step(","runtime_lifecycle_step(","StepContext::","default_now_us()","DOCUMENT_ARCHIVE_POLL_"])expect(poll).not.toContain(forbidden);
 console.log("[DEBUG] archive observer strict schema and independent SQLite preserve pending/running source; actual native observer unqualified until executed");
});
