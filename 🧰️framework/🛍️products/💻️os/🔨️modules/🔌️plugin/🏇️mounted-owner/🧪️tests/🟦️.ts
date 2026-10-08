import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020.js";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
type Observation = typeof fixture.vectors[number]["observation"];
interface MountedOwnerOracleV1 { valid(value:unknown):boolean; verdict(value:Observation):string }
/** 🔮️ Strict independent schema bounds reproduce admitted ownership verdicts behind a first-party oracle. */
function createMountedOwnerOracleV1():MountedOwnerOracleV1 {
 const ajv=new Ajv({strict:true,allErrors:true}),validate=ajv.compile(schema);
 const bounded=(properties:Record<string,unknown>,value:unknown)=>ajv.compile({type:"object",properties,required:Object.keys(properties)})(value)===true;
 const zero={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};
 return {valid:value=>validate(value)===true,verdict:value=>{
  if(!validate(value))return "refused";
  if(!bounded(Object.fromEntries(Object.entries(value.expectedIdentity).map(([key,identity])=>[key,{const:identity}])),value.identity))return "refused";
  if(value.schedule.cancelled)return "cancelled";
  if(!bounded({clockAvailable:{const:true},fuel:{type:"integer",minimum:1}},value.schedule)||BigInt(value.schedule.nowUs)>=BigInt(value.schedule.deadlineUs))return "held";
  const grant=value.grant,demand=value.demand;
  if(!bounded({maximumItems:{type:"integer",minimum:1},maximumCopyBytes:{type:"integer",minimum:demand.copyBytes},maximumCapacityBytes:{type:"integer",minimum:demand.capacityBytes},maximumReleaseBytes:{type:"integer",minimum:demand.releaseBytes},maximumDepth:{type:"integer",minimum:demand.depth}},grant))return "held";
  if(!bounded({copiedItems:{type:"integer",maximum:grant.maximumItems},copiedBytes:{type:"integer",maximum:grant.maximumCopyBytes},retainedCapacityBytes:{type:"integer",maximum:grant.maximumCapacityBytes},releasedBytes:{type:"integer",maximum:grant.maximumReleaseBytes}},value.receipt))return "refused";
  if(value.result==="complete")return value.terminalIsEmpty?"complete":"refused";
  if(value.result==="blocked"||value.result==="awaiting-input")return value.wakeOwned&&JSON.stringify(value.receipt)===JSON.stringify(zero)?value.result:"refused";
  return value.result;
 }};
}
test("portable mounted ownership schema agrees with strict independent Ajv",()=>{
 const oracle=createMountedOwnerOracleV1();expect(fixture.vectors).toHaveLength(27);
 for(const vector of fixture.vectors){expect(validateJsonSchemaSubset(schema,vector.observation)).toEqual([]);expect(oracle.valid(vector.observation)).toBe(true);}
 for(const vector of fixture.invalid){expect(validateJsonSchemaSubset(schema,vector.observation).length).toBeGreaterThan(0);expect(oracle.valid(vector.observation)).toBe(false);}
 console.log("[DEBUG] Mounted turn schema: 27 complete observations and four malformed grants agreed with strict Ajv");
});
test("all independent physical currencies and original scheduling ownership have portable oracle verdicts",()=>{
 const oracle=createMountedOwnerOracleV1();for(const vector of fixture.vectors)expect(oracle.verdict(vector.observation)).toBe(vector.expected);
 console.log("[DEBUG] Mounted ownership verdicts retained independent copy/capacity/release/depth, identity, cancellation, deadline, wake and terminal rules");
});
test("actual runtime mounted receivers require caller policy and typed scheduling turn",()=>{
 const root=resolve(import.meta.dir,"../../../../../../.."),source=readFileSync(resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"),"utf8");
 expect(source).toContain("MountedOwnerTurnV1");
 expect(source).not.toMatch(/fn mounted_job_(?:maintenance|close)_step\([^\n]*maximum_(?:items|bytes)/);
 expect(source).toMatch(/pub fn new\(mounted_policy:\s*MountedOwnerPolicyV1\)/);
 expect(source).not.toContain("PluginRuntime::new();");
 expect(source).toMatch(/mounted_job_maintenance_step\([^;\n]*turn/);
 expect(source).toMatch(/mounted_job_close_step\([^;\n]*turn/);
 expect(source).toContain("cx.is_cancelled()");expect(source).toContain("cx.deadline_exceeded()");
 console.log("[DEBUG] Actual General mounted receiver source binding is explicit; native host/session runtime remains a separate required gate");
});
