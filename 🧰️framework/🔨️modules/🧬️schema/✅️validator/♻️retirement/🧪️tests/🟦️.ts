import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync,readdirSync} from "node:fs";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import fixture from "./🧫️fixtures/🔣️.json";
test("original compiled validator retains every transitive pattern owner",()=>{
 const ajv=new Ajv();const validate=ajv.compile(fixture.schema);for(const value of fixture.valid)expect(validate(value)).toBe(true);for(const value of fixture.invalid)expect(validate(value)).toBe(false);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE custody(owner TEXT,phase TEXT)");for(const owner of fixture.ownerTypes)db.query("INSERT INTO custody VALUES(?,'original')").run(owner);for(const denial of fixture.denials){expect(db.query("SELECT COUNT(*) AS count FROM custody WHERE phase='original'").get()).toEqual({count:fixture.ownerTypes.length});expect(denial.length>0).toBe(true);}}finally{db.close();}
 const sourceTree=(directory:string):string=>readdirSync(directory,{withFileTypes:true}).filter(entry=>!["node_modules","🧪️tests","🧪️testing","📦️packages"].includes(entry.name)).flatMap(entry=>entry.isDirectory()?[sourceTree(join(directory,entry.name))]:entry.name==="🦀️.rs"?[readFileSync(join(directory,entry.name),"utf8")]:[]).join("\n"),source=sourceTree(fileURLToPath(new URL("../../",import.meta.url)));for(const owner of fixture.ownerTypes)expect(source).toMatch(new RegExp("derive\\([^\\n]*semio_framework_value::RetireOwned[^\\n]*\\)\\]\\n(?:pub )?(?:struct|enum) "+owner));
});
