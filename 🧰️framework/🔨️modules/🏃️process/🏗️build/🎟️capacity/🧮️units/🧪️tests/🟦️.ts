import {expect,test}from"bun:test";
import Ajv from"ajv";
import {availableParallelism}from"node:os";
import {mkdirSync,mkdtempSync,writeFileSync}from"node:fs";
import {join}from"node:path";
import {spawnSync}from"node:child_process";
import {fileURLToPath}from"node:url";
import {withBuildTestInvocationV1}from"../../../../🧪️testing/📥️invocation/🟦️.ts";
import {testPythonV1}from"../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import type{ScriptInvocation}from"../../../../🧭️routing/📥️invocation/🟦️.ts";
import {acquireCompilerCapacityLeaseV1,readCompilerCapacityRequestV1,scriptCompilerAdmissionControlV1}from"../../🟦️.ts";
import orderPolicy from"../../../../🔒️leases/🧭️order/🔣️policy.json";
import {fixedCompilerCapacityRequestV1,compileUnitsForHostV1,compileUnitsRequestV1,describeCompileUnitsV1,planCompileUnitsV1,readCompileUnitsHostV1,requireGrantedCompileUnitsV1,type CompileUnitsHostV1}from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json";
import leaseSchema from"../../🧬️schema/🔣️.json";
import fixture from"./🧫️fixtures/🔣️.json";
const output=process.env.SEMIO_TEST_ARTIFACT_DIR!;
mkdirSync(output,{recursive:true});
const distinctClaimants=join(output,"lease-order-policy.json");writeFileSync(distinctClaimants,JSON.stringify({...orderPolicy,fairness:{...orderPolicy.fairness,reentrantShared:false}}));process.env.SEMIO_LEASE_ORDER_POLICY=distinctClaimants;process.env.SEMIO_LEASE_ORDER="off";
const storage=mkdtempSync(join(output,"units-")),oracle=fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url)),schemaPath=fileURLToPath(new URL("../🧬️schema/🔣️.json",import.meta.url));
let invocation!:ScriptInvocation;
const control=(additional?:AbortSignal)=>scriptCompilerAdmissionControlV1(invocation,"test:compile-units",120_000,additional);
const python=(cases:unknown):unknown[]=>{const result=spawnSync(testPythonV1()??"python3",["-I",oracle,schemaPath],{input:JSON.stringify(cases),encoding:"utf8",maxBuffer:1<<26});expect(result.status,result.stderr).toBe(0);return JSON.parse(result.stdout);};
test("compile units values agree with an independent schema oracle",()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(schema,"units");const host=ajv.getSchema("units#/definitions/Host")!,decision=ajv.getSchema("units#/definitions/Decision")!;
 for(const row of fixture.cases){expect(host(row.host)).toBe(true);expect(decision(row.expect)).toBe(true);}
 for(const row of fixture.invalid){expect(host(row.host),row.name).toBe(false);expect(()=>planCompileUnitsV1(row.host as never),row.name).toThrow();}
});
test("the fixture cases are planned exactly as written down",()=>{
 for(const row of fixture.cases)expect(planCompileUnitsV1(row.host as CompileUnitsHostV1,row.intent),row.name).toEqual(row.expect as never);
});
test("every state of a generated grid is planned like the Python model",()=>{
 const grid:{host:CompileUnitsHostV1;intent:{needed:number|null}}[]=[];
 for(const cores of[1,2,3,4,6,8,10,12,16,24,64])for(const available of[null,0,1024,2048,4096,6000,8192,12288,16384,65536])for(const pressure of[null,"normal","warn","critical"] as const)for(const granted of[null,1,2,5,12])for(const needed of[null,1,3,9])grid.push({host:{cores,availableMegabytes:available,pressure,grantedJobs:granted},intent:{needed}});
 const expected=python(grid);expect(grid.length).toBeGreaterThan(5000);
 grid.forEach((row,index)=>expect(planCompileUnitsV1(row.host,row.intent),JSON.stringify(row)).toEqual(expected[index] as never));
});
test("the plan never exceeds half of the host, never starts below one core and keeps minimum within preferred",()=>{
 for(const cores of[1,2,3,4,5,7,10,33]){const plan=planCompileUnitsV1({cores,availableMegabytes:null,pressure:null,grantedJobs:null});expect(plan.preferred).toBeLessThanOrEqual(Math.max(1,Math.floor(cores/2)));expect(plan.minimum).toBeGreaterThanOrEqual(1);expect(plan.minimum).toBeLessThanOrEqual(plan.preferred);if(cores>=2)expect(plan.preferred*2).toBeLessThanOrEqual(cores);}
});
test("compile requests admit minimum within maximum like the schema oracle",()=>{
 const oracleRequest=new Ajv({strict:true}).compile(leaseSchema.definitions.Request);
 for(const row of fixture.requests){expect(oracleRequest(row.value),row.name).toBe(row.schemaAccepts);if(row.accepted)expect(readCompilerCapacityRequestV1(row.value as never),row.name).toMatchObject(row.value);else expect(()=>readCompilerCapacityRequestV1(row.value as never),row.name).toThrow();}
 expect(compileUnitsRequestV1(planCompileUnitsV1({cores:10,availableMegabytes:null,pressure:null,grantedJobs:null}))).toEqual({kind:"available",maximumParallelism:5,minimumParallelism:3});
});
test("the live host is read with free memory, pressure and the cores of the machine",async()=>{
 const live=await readCompileUnitsHostV1({environment:{}}),planned=await compileUnitsForHostV1({environment:{}});
 expect(live.cores).toBe(availableParallelism());expect(live.grantedJobs).toBeNull();expect(planned.request.kind).toBe("available");expect(planned.decision.preferred).toBeLessThanOrEqual(Math.max(1,Math.floor(live.cores/2)));
 const granted=await readCompileUnitsHostV1({environment:{SEMIO_HOST_CAPACITY_PLAN:JSON.stringify({version:1,id:"abcdefgh",jobs:3,cpu:3,memoryMegabytes:8192,fixedMegabytes:1536,floorMegabytes:4096})}});
 expect(granted.grantedJobs).toBe(3);
 const aborted=AbortSignal.abort(Error("cancelled"));await expect(readCompileUnitsHostV1({environment:{},signal:aborted})).rejects.toThrow("cancelled");
});
test("granted units stay within the plan",()=>{
 const plan=planCompileUnitsV1({cores:10,availableMegabytes:null,pressure:null,grantedJobs:null});
 expect(requireGrantedCompileUnitsV1(plan,5)).toBe(5);expect(requireGrantedCompileUnitsV1(plan,3)).toBe(3);expect(()=>requireGrantedCompileUnitsV1(plan,6)).toThrow();expect(()=>requireGrantedCompileUnitsV1(plan,2)).toThrow();
});
test("plans and grants are described in English and German",()=>{
 const values={operation:"cargo",preferred:5,minimum:3,cores:10,reason:"memory",granted:4,seconds:12};
 expect(describeCompileUnitsV1("en","planned",values)).toBe("[compile-units] cargo: 5 of 10 cores planned (at least 3 to start; the free memory carries fewer compile units)");
 expect(describeCompileUnitsV1("de","granted",values)).toContain("4 von 10 Kernen nach 12 s zugeteilt");
 expect(describeCompileUnitsV1("de","waiting",values)).toContain("mindestens 3 freie Kerne von 10");
});
test("real core leases admit the planned compiles side by side in first come first served order like the Python queue model",()=>withBuildTestInvocationV1(async original=>{invocation=original;
 const cores=availableParallelism();if(cores<2)return;
 const held:Awaited<ReturnType<typeof acquireCompilerCapacityLeaseV1>>[]=[],plan=planCompileUnitsV1({cores,availableMegabytes:null,pressure:null,grantedJobs:null}),request=compileUnitsRequestV1(plan);
 try{
  const first=await acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request});held.push(first);
  const second=await acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request});held.push(second);
  const model=python([{cores,requests:[[plan.minimum,plan.preferred],[plan.minimum,plan.preferred]]}])[0] as number[];
  expect([first.parallelism,second.parallelism]).toEqual(model);expect(first.mode).toBe("shared");expect(first.parallelism+second.parallelism).toBeLessThanOrEqual(cores);
  const free=cores-first.parallelism-second.parallelism;
  let entered=false;const waiting=acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"available",minimumParallelism:free+1,maximumParallelism:free+1}}).then(claim=>{entered=true;return claim;});
  await Bun.sleep(150);expect(entered).toBe(false);
  held.shift()!.release();const third=await waiting;held.push(third);expect(third.parallelism).toBe(free+1);
 }finally{for(const claim of held)claim.release();}
}),60_000);
test("a compile waiting for its minimum cores is cancelled and gives nothing back",()=>withBuildTestInvocationV1(async original=>{invocation=original;
 const cores=availableParallelism(),all=await acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"exact",parallelism:cores}});
 try{
  const controller=new AbortController(),waiting=acquireCompilerCapacityLeaseV1({directory:storage,control:control(controller.signal),environment:{},request:{kind:"available",minimumParallelism:1,maximumParallelism:cores}});
  await Bun.sleep(100);controller.abort();await expect(waiting).rejects.toThrow();
 }finally{all.release();}
 const again=await acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request:{kind:"exact",parallelism:cores}});again.release();
}),60_000);
test("a fixed-class tool (the Bun bundler) gets a short counted slot equal to the independent Python derivation, never the whole host",()=>{
 const answers=python(fixture.fixed.map(row=>({fixed:row.fixed,cores:row.cores})));
 fixture.fixed.forEach((row,index)=>{
  const actual=fixedCompilerCapacityRequestV1(row.fixed as "bun-bundle",row.cores);
  expect(actual,String(row.cores)).toEqual(row.expect as never);expect(answers[index]).toEqual(row.expect);expect(actual.kind).toBe("available");
  expect((actual as {maximumParallelism:number}).maximumParallelism).toBeLessThanOrEqual(Math.max(1,Math.floor(row.cores/2)));
 });
 expect(()=>fixedCompilerCapacityRequestV1("nope" as never,4)).toThrow();
});
test("Bun bundlers are admitted side by side exactly as the independent queue model grants them, and the old whole-host request is the control that never overlaps",()=>withBuildTestInvocationV1(async original=>{invocation=original;
 const cores=availableParallelism();if(cores<4)return;
 const slot=fixedCompilerCapacityRequestV1("bun-bundle") as {kind:"available";minimumParallelism:number;maximumParallelism:number},count=40,model=python([{cores,requests:Array.from({length:count},()=>[slot.minimumParallelism,slot.maximumParallelism])}])[0] as number[];
 expect(model.length,"at least two bundlers run at once").toBeGreaterThanOrEqual(2);
 const held:Awaited<ReturnType<typeof acquireCompilerCapacityLeaseV1>>[]=[],ask=(request:Parameters<typeof acquireCompilerCapacityLeaseV1>[0]["request"])=>acquireCompilerCapacityLeaseV1({directory:storage,control:control(),environment:{},request});
 try{
  for(let index=0;index<model.length;index++)held.push(await ask(slot));
  expect(held.map(claim=>claim.parallelism)).toEqual(model);expect(held.reduce((sum,claim)=>sum+claim.parallelism,0)).toBeLessThanOrEqual(cores);
  if(model.length<count){let entered=false;const waiting=ask(slot).then(claim=>{entered=true;return claim;});await Bun.sleep(150);expect(entered,"a bundler beyond the model waits").toBe(false);held.shift()!.release();held.push(await waiting);}
  for(const claim of held.splice(0))claim.release();
  const whole=await ask({kind:"whole-host"});held.push(whole);let beside=false;const blocked=ask(slot).then(claim=>{beside=true;return claim;});await Bun.sleep(150);expect(beside,"a whole-host holder admits no bundler beside it").toBe(false);whole.release();held.length=0;held.push(await blocked);
 }finally{for(const claim of held)claim.release();}
}),60_000);
