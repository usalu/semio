import Ajv from "ajv";
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {GRAPH_REFUSAL_POLICY_V1,graphRefusalOfV1,type GraphAnswerV1} from "../🟦️.ts";

/** 🧯️ Contract: every answer of the cache, and every local reason not to ask, maps to one refusal with the flags the caller acts on (keep the work and try again, write a failure note so others stop, or count the key as stored). The fixture is language-agnostic; a Python oracle matches the rows again. */
const ajv=new Ajv({strict:false}),validate=ajv.compile({$ref:"#/definitions/Fixture",definitions:schema.definitions}),codes=new Set(schema.definitions.Code.enum);

test("the fixture and the policy are valid against the schema",()=>{
 expect(validate(fixture)).toBe(true);
 expect(ajv.validate({$ref:"#/definitions/Row",definitions:schema.definitions},GRAPH_REFUSAL_POLICY_V1.rows[0])).toBe(true);
 for(const row of GRAPH_REFUSAL_POLICY_V1.rows)expect(ajv.validate({$ref:"#/definitions/Row",definitions:schema.definitions},row),row.code).toBe(true);
});

test("every case gets the code and the flags of the fixture",()=>{
 for(const row of fixture.cases){
  const found=graphRefusalOfV1(row.answer as GraphAnswerV1,(row as {body?:string}).body??"");
  expect([found.code,found.transient,found.deterministic,found.stored],row.name).toEqual([row.expect.code,row.expect.transient,row.expect.deterministic,row.expect.stored]);
 }
});

test("every refusal says its reason in English and German, and a status keeps the first words of the body",()=>{
 for(const row of GRAPH_REFUSAL_POLICY_V1.rows){expect(row.en.length).toBeGreaterThan(10);expect(row.de.length).toBeGreaterThan(10);expect(codes.has(row.code)).toBe(true);}
 const found=graphRefusalOfV1(413,"Entry exceeds the maximum entry size "+"x".repeat(1000));
 expect(found.status).toBe(413);expect(found.detail.length).toBe(GRAPH_REFUSAL_POLICY_V1.bodyCharacters);expect(found.text).toContain(" / ");
});

test("no transient refusal is deterministic and no stored key is a failure",()=>{
 for(const row of GRAPH_REFUSAL_POLICY_V1.rows){if(row.transient)expect(row.deterministic,row.code).toBe(false);if(row.stored){expect(row.transient,row.code).toBe(false);expect(row.deterministic,row.code).toBe(false);}}
 expect(GRAPH_REFUSAL_POLICY_V1.rows.at(-1)!.when).toEqual({});
});

test("the Python oracle matches the rows again and agrees on every case",()=>{
 const here=(name:string)=>fileURLToPath(new URL(name,import.meta.url)),run=spawnSync("python3",["-I",here("./🐍️.py"),here("../🧬️schema/🔣️.json"),here("./🔣️.json")],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const oracle=JSON.parse(run.stdout) as {code:string;transient:boolean;deterministic:boolean;stored:boolean}[];
 expect(oracle).toEqual(fixture.cases.map(row=>{const found=graphRefusalOfV1(row.answer as GraphAnswerV1,(row as {body?:string}).body??"");return {code:found.code,deterministic:found.deterministic,stored:found.stored,transient:found.transient};}));
});
