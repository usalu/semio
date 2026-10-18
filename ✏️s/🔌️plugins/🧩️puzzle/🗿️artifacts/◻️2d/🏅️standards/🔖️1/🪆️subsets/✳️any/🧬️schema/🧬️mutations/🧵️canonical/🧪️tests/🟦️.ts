/** 🧩️ Actual family payloads preserve schema wire identity and independent owner ordinals. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {existsSync,readdirSync,readFileSync} from "node:fs";
test("Puzzle canonical native fields cover all authored mutation payload owners",()=>{
 const root=new URL("../",import.meta.url);const family=new URL("../../../",root);const read=(url:URL)=>JSON.parse(readFileSync(url,"utf8"));const fixture=read(new URL("../../../🧪️tests/🧫️fixtures/🧬️mutations/🧵️canonical/🔣️.json",root));
 expect(JSON.stringify(fixture.scalar)).toBe(fixture.scalarExpected);
 const db=new Database(":memory:");db.exec("CREATE TABLE payloads(kind TEXT PRIMARY KEY,tag TEXT UNIQUE,bytes TEXT)");
 for(const row of fixture.cases){const original=read(new URL(row.path,family));expect(original.mutation).toBe(row.tag);db.query("INSERT INTO payloads VALUES(?,?,?)").run(row.kind,row.tag,JSON.stringify(original));expect(JSON.parse((db.query("SELECT bytes FROM payloads WHERE kind=?").get(row.kind)as{bytes:string}).bytes)).toEqual(original);}
 expect(db.query("SELECT COUNT(*) AS n FROM payloads").get()).toEqual({n:36});db.close();
 const dispatch=readFileSync(new URL("🧬️schema/🧬️mutations/🦀️.rs",family),"utf8");expect(dispatch).toContain("CanonicalJsonTree");expect(dispatch).toContain("canonical_json(owner=semio_framework_pack_json)");
 const mutations=new URL("🧬️schema/🧬️mutations/",family),leaf=(kind:string)=>new URL(readdirSync(mutations,{withFileTypes:true}).filter(entry=>entry.isDirectory()).map(entry=>`${entry.name}/${kind}/🦀️.rs`).find(path=>existsSync(new URL(path,mutations)))!,mutations);
 for(const row of fixture.cases){const source=readFileSync(leaf(row.kind),"utf8");expect(source).toContain("CanonicalJsonTree");expect(source).toContain("canonical_json(owner=semio_framework_pack_json)");}
});
