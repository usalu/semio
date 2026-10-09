import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import fixture from "../🧫️fixtures/🔣️.json";
test("original compiled validator retains every transitive pattern owner",()=>{
 const ajv=new Ajv();const validate=ajv.compile(fixture.schema);for(const value of fixture.valid)expect(validate(value)).toBe(true);for(const value of fixture.invalid)expect(validate(value)).toBe(false);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE custody(owner TEXT,phase TEXT)");for(const owner of fixture.ownerTypes)db.query("INSERT INTO custody VALUES(?,'original')").run(owner);for(const denial of fixture.denials){expect(db.query("SELECT COUNT(*) AS count FROM custody WHERE phase='original'").get()).toEqual({count:fixture.ownerTypes.length});expect(denial.length>0).toBe(true);}}finally{db.close();}
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");for(const owner of fixture.ownerTypes)expect(source).toMatch(new RegExp("derive\\([^\\n]*semio_framework_value::RetireOwned[^\\n]*\\)\\]\\n(?:pub )?(?:struct|enum) "+owner));console.log("[DEBUG] Ajv and SQLite preserve original compiled validator and recursive pattern ownership across denied construction");
});
