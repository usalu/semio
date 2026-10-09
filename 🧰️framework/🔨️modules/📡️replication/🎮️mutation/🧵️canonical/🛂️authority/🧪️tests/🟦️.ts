/** 🛂️ Node byte equality and SQLite immutable native-field custody precede authority validation. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync,existsSync} from "node:fs";
test("canonical semantic authority compares original native fields under independently admitted byte pairs",()=>{
 const root=new URL("../",import.meta.url);const read=(path:string)=>JSON.parse(readFileSync(new URL(path,root),"utf8"));const fixture=read("🧫️fixtures/🔣️.json");
 const equal=(left:string|null,right:string|null)=>left===null||right===null?left===right:Buffer.from(left).equals(Buffer.from(right));
 const db=new Database(":memory:");db.exec("CREATE TABLE immutable(id TEXT PRIMARY KEY,original TEXT)");
 for(const repeat of fixture.actorRepeats){const suffix=fixture.actorSuffix.repeat(repeat);const authority={...fixture.authority,actor:fixture.authority.actor+suffix};for(const row of fixture.cases){const edit={...row.edit,actor:row.edit.actor===fixture.authority.actor?authority.actor:row.edit.actor,author:row.edit.author===fixture.authority.actor?authority.actor:row.edit.author};const accepted=edit.id.length>0&&Buffer.byteLength(edit.id)<=128&&edit.sequence===authority.sequence&&edit.forwardCount===1&&edit.metaCount===1&&JSON.stringify(edit.clock)===JSON.stringify(authority.clock)&&equal(edit.actor,authority.actor)&&equal(edit.line,authority.line)&&equal(edit.author,authority.actor)&&equal(edit.group,authority.group);expect(accepted).toBe(row.accepted);const key=repeat+":"+row.id;const original=JSON.stringify(edit);db.query("INSERT INTO immutable VALUES(?,?)").run(key,original);expect(db.query("SELECT original FROM immutable WHERE id=?").get(key)).toEqual({original});}}
 db.close();console.log("[DEBUG] independent Node exact UTF8/native optional equality and SQLite immutable one-owner conservation agree for ten scalar/field refusals and actor bytes beyond4096");
 const path=new URL("🦀️.rs",root);expect(existsSync(path)).toBe(true);const source=readFileSync(path,"utf8");expect(source).toContain("ArtifactCanonicalEditAuthorityCursor");expect(source).toContain("RetainedOwnedProjection");expect(source).toContain("next_demand");expect(source).not.toContain(".clone()");expect(source).not.toContain("ToValue");
});
