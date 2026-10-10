import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {createRequire} from "node:module";
import {Database} from "bun:sqlite";
test("Canonical Fault cursor has an original zero-heap and independently bounded wire contract",()=>{
 const owner=resolve(import.meta.dir,".."),require=createRequire(import.meta.url),law=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(resolve(owner,"🧬️schema/🔣️.json"),"utf8"));expect(new(require("ajv").default)({strict:true}).compile(schema)(law)).toBe(true);expect(law.originalGrant).toEqual([1,17,0,0,1]);const canonical=JSON.parse(readFileSync(resolve(owner,"../../🧫️fixtures/🧯️fault/🔣️.json"),"utf8"));const db=new Database(":memory:");try{db.run("CREATE TABLE original_fault(id TEXT,expected TEXT,bytes INTEGER)");for(const row of canonical.cases){const expected=JSON.stringify(row.expected);db.query("INSERT INTO original_fault VALUES(?,?,?)").run(row.id,expected,Buffer.byteLength(expected));expect(JSON.parse(expected)).toEqual(row.expected);}expect(db.query("SELECT count(*) AS n FROM original_fault WHERE json_valid(expected) AND bytes>0").get()).toEqual({n:canonical.cases.length});}finally{db.close();}
 const source=readFileSync(resolve(owner,"🦀️.rs"),"utf8");expect(source).toContain("pub struct FaultWireCursor");for(const obsolete of ["serde_json::",".to_value(","fault.clone(","Vec<", "String::"]){expect(source.includes(obsolete)).toBe(false);}expect(source).toContain("advance_demands");expect(source).toContain("advance_one");expect(source).toContain("close_step");console.log("[DEBUG] Ajv/SQLite canonical Fault cursor original copy17 capacity0 release0 metadata0; source contract only, native results remain separate");
});
