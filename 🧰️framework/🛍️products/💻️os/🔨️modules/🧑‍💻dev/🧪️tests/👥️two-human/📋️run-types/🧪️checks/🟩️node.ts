import {test} from "node:test";
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {DatabaseSync} from "node:sqlite";
import Ajv from "ajv";
import {requiredJourneyChecksPass} from "../🟦️.ts";
const read=(name:string)=>JSON.parse(readFileSync(new URL(name,import.meta.url),"utf8"));
test("neutral reconnect check accounting agrees with independent SQLite",()=>{const policy=read("../../🧭️journeys/🧬️reconnect.json"),policySchema=read("../../🧭️journeys/🧬️reconnect.schema.json"),rows=read("./🔣️.json"),schema=read("./🧬️.json"),ajv=new Ajv({strict:true}),db=new DatabaseSync(":memory:");try{assert.equal(ajv.compile(policySchema)(policy),true);assert.equal(ajv.compile(schema)(rows),true);for(const row of rows){const checks=Object.fromEntries(row.names.map((name:string)=>[name,{pass:!row.failed.includes(name)}])),oracle=db.prepare("select count(*)=0 ok from json_each(?) r where not exists(select 1 from json_each(?) n where n.value=r.value) or exists(select 1 from json_each(?) f where f.value=r.value) or exists(select 1 from json_each(?) d where d.value=r.value)").get(JSON.stringify(policy.requiredChecks),JSON.stringify(row.names),JSON.stringify(row.failed),JSON.stringify(row.duplicates)) as {ok:number};assert.equal(Boolean(oracle.ok),row.pass);assert.equal(requiredJourneyChecksPass(policy.requiredChecks,checks,new Set(row.duplicates)),row.pass)}}finally{db.close()}});
