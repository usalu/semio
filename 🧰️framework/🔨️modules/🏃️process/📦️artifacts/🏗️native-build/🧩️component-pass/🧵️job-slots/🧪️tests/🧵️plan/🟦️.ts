import {expect,test}from"bun:test";
import Ajv from"ajv";
import {spawnSync}from"node:child_process";
import {fileURLToPath}from"node:url";
import {testPythonV1}from"../../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {describeComponentJobsV1,planComponentJobsV1,type ComponentJobsHostV1}from"../../🟦️.ts";
import schema from"../../🧬️schema/🔣️.json";
import fixture from"./🧫️fixtures/🔣️.json";
const oracle=fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url)),schemaPath=fileURLToPath(new URL("../../🧬️schema/🔣️.json",import.meta.url));
const python=(cases:unknown):unknown[]=>{const result=spawnSync(testPythonV1()??"python3",["-I",oracle,schemaPath],{input:JSON.stringify(cases),encoding:"utf8",maxBuffer:1<<26});expect(result.status,result.stderr).toBe(0);return JSON.parse(result.stdout);};
test("component job fixtures are valid for the schema oracle and planned exactly as written down",()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(schema,"units");const host=ajv.getSchema("units#/definitions/ComponentJobsHost")!,decision=ajv.getSchema("units#/definitions/ComponentJobsDecision")!;
 for(const row of fixture.cases){expect(host(row.host),row.name).toBe(true);expect(decision(row.expect),row.name).toBe(true);expect(planComponentJobsV1(row.host as ComponentJobsHostV1),row.name).toEqual(row.expect as never);}
 for(const row of fixture.invalid){expect(host(row.host),row.name).toBe(false);expect(()=>planComponentJobsV1(row.host as never),row.name).toThrow();}
});
test("every state of a generated grid is planned like the Python model",()=>{
 const grid:{host:ComponentJobsHostV1}[]=[];
 for(const local of[1,2,4,23,24,64])for(const fleetSlots of[null,0,1,2,3,11,40,1000])for(const availableMegabytes of[null,0,2048,2600,3072,4096,16000,65536])for(const pressure of[null,"normal","warn","critical"] as const)for(const unitMegabytes of[256,512,2048])grid.push({host:{local,fleetSlots,availableMegabytes,pressure,unitMegabytes}});
 const expected=python(grid);expect(grid.length).toBeGreaterThan(4000);
 grid.forEach((row,index)=>expect(planComponentJobsV1(row.host),JSON.stringify(row)).toEqual(expected[index] as never));
});
test("local grant 1 plus N fitting fleet slots is N+1 jobs, fleet down is 1 and the memory of the coordinator bounds the remote jobs",()=>{
 const cap=schema.definitions.Policy.const.componentJobsMaximum,residue=schema.definitions.Policy.const.componentJobResidueMegabytes,reserve=schema.definitions.Policy.const.reserveMegabytes;
 for(const slots of[1,2,5,11,22])expect(planComponentJobsV1({local:1,fleetSlots:slots,availableMegabytes:null,pressure:null,unitMegabytes:512}).jobs,String(slots)).toBe(slots+1);
 for(const down of[null,0])expect(planComponentJobsV1({local:1,fleetSlots:down,availableMegabytes:64000,pressure:"normal",unitMegabytes:512}).jobs).toBe(1);
 for(const local of[1,4,23,24,64])for(const fleetSlots of[null,0,1,9,40,500]){
  const plan=planComponentJobsV1({local,fleetSlots,availableMegabytes:null,pressure:null,unitMegabytes:512});
  expect(plan.local).toBe(local);expect(plan.jobs).toBe(local+plan.remote);expect(plan.remote).toBeLessThanOrEqual(fleetSlots??0);expect(plan.jobs).toBeLessThanOrEqual(Math.max(local,cap));
 }
 for(const available of[0,2048,2560,3000,4096,9000,30000]){
  const plan=planComponentJobsV1({local:1,fleetSlots:20,availableMegabytes:available,pressure:"normal",unitMegabytes:512});
  expect(plan.remote*residue,String(available)).toBeLessThanOrEqual(Math.max(0,available-reserve-512));
 }
});
test("the plan is described in English and German with the local and remote share and the peak a final is counted at",()=>{
 const values={operation:"component build",jobs:12,local:1,remote:11,slots:11,peak:6144,reason:"jobs-all"};
 expect(describeComponentJobsV1("en",values)).toBe("[component-jobs] component build: 12 component jobs at once = 1 local + 11 on the fleet (11 worker slots fit one 6144 MB final; every worker slot that fits a wasm final takes one job)");
 expect(describeComponentJobsV1("de",values)).toContain("12 Komponenten-Jobs gleichzeitig = 1 lokal + 11 auf der Flotte");
});
