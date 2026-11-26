import {expect,test}from"bun:test";
import Ajv from"ajv";
import {spawnSync}from"node:child_process";
import {fileURLToPath}from"node:url";
import {testPythonV1}from"../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {describeCompileUnitsV1,planFleetJobsV1,type FleetJobsHostV1}from"../../🟦️.ts";
import schema from"../../🧬️schema/🔣️.json";
import fixture from"./🧫️fixtures/🔣️.json";
const oracle=fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url)),schemaPath=fileURLToPath(new URL("../../🧬️schema/🔣️.json",import.meta.url));
const python=(cases:unknown):unknown[]=>{const result=spawnSync(testPythonV1()??"python3",["-I",oracle,schemaPath],{input:JSON.stringify(cases),encoding:"utf8",maxBuffer:1<<26});expect(result.status,result.stderr).toBe(0);return JSON.parse(result.stdout);};
test("fleet job fixtures are valid for the schema oracle and planned exactly as written down",()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(schema,"units");const host=ajv.getSchema("units#/definitions/FleetHost")!,decision=ajv.getSchema("units#/definitions/FleetDecision")!;
 for(const row of fixture.cases){expect(host(row.host),row.name).toBe(true);expect(decision(row.expect),row.name).toBe(true);expect(planFleetJobsV1(row.host as FleetJobsHostV1),row.name).toEqual(row.expect as never);}
 for(const row of fixture.invalid){expect(host(row.host),row.name).toBe(false);expect(()=>planFleetJobsV1(row.host as never),row.name).toThrow();}
});
test("every state of a generated grid is planned like the Python model",()=>{
 const grid:{host:FleetJobsHostV1}[]=[];
 for(const local of[1,2,4,10,24,47,48,64])for(const fleetIdle of[null,0,1,3,7,40,100,1000])for(const availableMegabytes of[null,0,2048,4096,6000,16000,65536])for(const pressure of[null,"normal","warn","critical"] as const)for(const unitMegabytes of[256,512,2048])for(const grantedMegabytes of[null,704,3072])grid.push({host:{local,fleetIdle,availableMegabytes,pressure,unitMegabytes,grantedMegabytes}});
 const expected=python(grid);expect(grid.length).toBeGreaterThan(5000);
 grid.forEach((row,index)=>expect(planFleetJobsV1(row.host),JSON.stringify(row)).toEqual(expected[index] as never));
});
test("the jobs never fall below the local grant, never exceed the cap above it and never use more than the idle slots",()=>{
 const cap=schema.definitions.Policy.const.fleetJobsMaximum;
 for(const local of[1,4,10,47,48,64])for(const fleetIdle of[null,0,1,9,40,500]){
  const plan=planFleetJobsV1({local,fleetIdle,grantedMegabytes:null,availableMegabytes:null,pressure:null,unitMegabytes:512});
  expect(plan.local).toBe(local);expect(plan.jobs).toBe(local+plan.remote);expect(plan.remote).toBeLessThanOrEqual(fleetIdle??0);expect(plan.jobs).toBeLessThanOrEqual(Math.max(local,cap));
 }
});
test("the plan is described in English and German with the local and remote share",()=>{
 const values={operation:"component build",jobs:44,local:4,remote:40,idle:40,reason:"fleet-all"};
 expect(describeCompileUnitsV1("en","fleet",values)).toBe("[compile-units] component build: fleet-sized 44 jobs = 4 local + 40 remote (40 idle on the fleet; every idle fleet slot of the same platform is used)");
 expect(describeCompileUnitsV1("de","fleet",values)).toContain("44 Jobs = 4 lokal + 40 entfernt");
});
