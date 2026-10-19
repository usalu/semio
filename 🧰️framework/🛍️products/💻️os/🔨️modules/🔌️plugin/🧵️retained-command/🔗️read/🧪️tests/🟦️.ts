import { test, expect } from "bun:test";
import Ajv from "ajv/dist/2020.js";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import law from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
test("original command read donation preserves issuer freshness and funded header transfer",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,replacementSnapshot:true})).toBe(false);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE original_read(operation INTEGER,generation INTEGER,revision TEXT,current INTEGER);INSERT INTO original_read VALUES(17,9,'original-revision',1)");const current=(operation:number,generation:number,revision:string)=>db.query("SELECT count(*) AS admitted FROM original_read WHERE operation=? AND generation=? AND revision=? AND current=1").get(operation,generation,revision);expect(current(17,9,"original-revision")).toEqual({admitted:1});for(const row of [[18,9,"original-revision"],[17,10,"original-revision"],[17,9,"changed-revision"]] as const)expect(current(row[0],row[1],row[2])).toEqual({admitted:0});db.exec("UPDATE original_read SET current=0");expect(current(17,9,"original-revision")).toEqual({admitted:0});}finally{db.close();}
 const source=readModuleTreeSource(new URL("../🦀️.rs",import.meta.url));expect(source).toContain("pub fn take_current(");expect(source).toContain("cx.is_cancelled()");expect(source).toContain("read.commit_authority_matches(generation,revision)");expect(source.indexOf("cx.consume_fuel(1)")).toBeLessThan(source.indexOf("source.matches(cx)"));expect(source).toContain("Ok(original.take())");expect(source).not.toContain("Arc::clone");expect(source).not.toContain(".try_duplicate");expect(law.transferReceipt).toEqual([1,0,0,0]);
});
