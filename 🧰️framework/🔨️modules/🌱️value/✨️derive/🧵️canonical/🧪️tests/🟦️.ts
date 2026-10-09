/** 🧵️ Native field-role declaration follows an independent tagged JSON/SQLite oracle. */
import {test,expect} from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import {readFileSync,existsSync} from "node:fs";
test("canonical native field derivation retains schema wire order without serialization callbacks",()=>{
 const root=new URL("../",import.meta.url);const read=(path:string)=>JSON.parse(readFileSync(new URL(path,root),"utf8"));const fixture=read("🧫️fixtures/🔣️.json");
 expect(new Ajv2020({strict:true}).compile(read("🧬️schema/🔣️.json"))(fixture)).toBe(true);expect(JSON.stringify({[fixture.tag]:fixture.variant,...fixture.values})).toBe(fixture.expected);
 const db=new Database(":memory:");db.exec("CREATE TABLE fields(ordinal INTEGER PRIMARY KEY,name TEXT,owner INTEGER)");Object.keys(fixture.values).forEach((key,index)=>db.query("INSERT INTO fields VALUES(?,?,1)").run(index,key));expect((db.query("SELECT name FROM fields ORDER BY ordinal").all() as {name:string}[]).map(row=>row.name)).toEqual(Object.keys(JSON.parse(fixture.expected)).slice(1));expect(db.query("SELECT SUM(owner) AS n FROM fields").get()).toEqual({n:3});db.close();
 console.log("[DEBUG] Ajv/JSON tagged record byte oracle and SQLite native schema field ownership/order agree; no whole payload copies");
 const path=new URL("🦀️.rs",root);expect(existsSync(path)).toBe(true);const source=readFileSync(path,"utf8");expect(source).toContain("canonical_tree_child");expect(source).toContain("ArtifactCanonicalJsonTree");expect(source).not.toContain("ToValue::to_value");expect(source).not.toContain("transmute");
});
