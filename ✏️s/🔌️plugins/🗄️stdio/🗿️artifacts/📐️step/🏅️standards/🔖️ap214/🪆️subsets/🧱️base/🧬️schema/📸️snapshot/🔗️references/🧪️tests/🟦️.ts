/** 🔗️ Internal STEP references agree with independent SQLite identity and deferred foreign-key constraints. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync,existsSync} from "node:fs";
import corpus from "../🧫️fixtures/🔣️.json";

test("STEP closed internal references preserve forward and cyclic edges while refusing missing or duplicated identities",async()=>{
 for(const row of corpus.cases){
  const db=new Database(":memory:");db.exec("PRAGMA foreign_keys=ON; CREATE TABLE entity(id INTEGER PRIMARY KEY); CREATE TABLE edge(target INTEGER NOT NULL REFERENCES entity(id) DEFERRABLE INITIALLY DEFERRED);");let refusal=false;
  const references=(value:any):number[]=>typeof value!=="object"||value===null?[]:"reference" in value?[value.reference]:"aggregate" in value?value.aggregate.flatMap(references):"typedValue" in value?references(value.typedValue.value):[];
  try{db.exec("BEGIN");for(const entity of row.entities)db.query("INSERT INTO entity VALUES(?)").run(entity.id);for(const entity of row.entities)for(const value of [...entity.args,...entity.complex.flatMap((x:any)=>x.args)])for(const target of references(value))db.query("INSERT INTO edge VALUES(?)").run(target);db.exec("COMMIT");}catch{refusal=true;db.exec("ROLLBACK");}finally{db.close();}
  expect(refusal,row.id).toBe(row.error!==null);
 }
 const owner=new URL("../🦀️.rs",import.meta.url);expect(existsSync(owner)).toBe(true);
 const native=readFileSync(owner,"utf8");expect(native).toContain("validate_step_references");
 const {validateStepReferences}=await import("../🟦️.ts");
 for(const row of corpus.cases){const entities=row.entities.map((entity:any)=>({...entity,id:BigInt(entity.id),args:entity.args.map(convert),complex:entity.complex.map((part:any)=>({...part,args:part.args.map(convert)}))}));expect(validateStepReferences({entities} as any),row.id).toEqual(row.error);}
 expect(readFileSync(new URL("../../../🔺️diff/🦀️.rs",import.meta.url),"utf8")).toContain("validate_step_references(&next)?");
 console.log("[DEBUG] STEP eight native reference-closure vectors agree with independent SQLite identity/FK constraints");
});
function convert(value:any):any{return typeof value!=="object"||value===null?value:"reference" in value?{reference:BigInt(value.reference)}:"aggregate" in value?{aggregate:value.aggregate.map(convert)}:"typedValue" in value?{typedValue:{...value.typedValue,value:convert(value.typedValue.value)}}:value;}
