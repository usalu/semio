import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "../🏪️store/🧬️schema/🔣️.json";
import corpus from "./🏪️scope.json";
import {validateJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

test("closed Original callback lifetime agrees with independent schema and SQLite",()=>{const validate=new Ajv({strict:true}).compile(schema.definitions.CallbackScope),oracle=new Database(":memory:");try{for(const row of corpus.cases){const value={version:row.version,live:row.live},accepted=oracle.query("SELECT ?1=1 AND ?2=1 AND ?3='boolean' AS accepted").get(row.version,Number(row.live),typeof row.live) as {accepted:number};expect(Boolean(accepted.accepted)).toBe(row.accepted);expect(validate(value)).toBe(row.accepted);expect(validateJsonSchemaSubset(schema.definitions.CallbackScope,value).length===0).toBe(row.accepted);}}finally{oracle.close();}});
