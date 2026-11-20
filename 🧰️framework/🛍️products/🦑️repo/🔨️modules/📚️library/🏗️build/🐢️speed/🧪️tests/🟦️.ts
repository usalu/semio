/** 🐢️ Contract: the speed factor of a machine is 1 at or above the reference (8 cores, 16 GB), grows with the missing cores and the square root of the missing memory, and never exceeds four; the processor time of a live process is read on Linux and macOS and grows while the process works. An independent Python model computes the factors from the declared schema.
 * @see ../🧬️schema/🔣️.json */
import{expect,test}from"bun:test";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import Ajv from "ajv";
import{testPythonV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{MACHINE_SPEED_POLICY_V1,cpuTimeTextMillisecondsV1,localMachineSpeedFactorV1,machineSpeedFactorV1,processCpuMillisecondsV1,readProcessCpuMillisecondsV1,scaledMillisecondsV1}from"../🟨️.mjs";
import fixture from"./🔣️.json";
import samples from"./📡️cpu/🔣️.json";
import sampleSchema from"./📡️cpu/🧬️schema/🔣️.json";

const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();

test("the factor equals the independent model on every machine of the table, and the anchors are exact",()=>{
 expect(python,"python3 is required for the independent model").toBeTruthy();
 const model=JSON.parse(Bun.spawnSync([python!,"-I",join(here,"🐍️.py"),join(here,"🔣️.json"),join(here,"..","🧬️schema","🔣️.json")]).stdout.toString()) as number[];
 expect(model.length).toBe(fixture.machines.length);
 fixture.machines.forEach((machine,at)=>expect(machineSpeedFactorV1(machine),JSON.stringify(machine)).toBe(model[at]!));
 expect(machineSpeedFactorV1({cores:4,totalMegabytes:8192})).toBe(2.83);
 expect(machineSpeedFactorV1({cores:8,totalMegabytes:16384})).toBe(1);
 expect(machineSpeedFactorV1({cores:10,totalMegabytes:32768})).toBe(1);
 expect(machineSpeedFactorV1({cores:1,totalMegabytes:512})).toBe(MACHINE_SPEED_POLICY_V1.maximumFactor);
});

test("a limit is lengthened by the factor and never shortened; a declared smaller machine counts as declared",()=>{
 expect(scaledMillisecondsV1(180000,2.83)).toBe(509400);
 expect(scaledMillisecondsV1(180000,1)).toBe(180000);
 expect(scaledMillisecondsV1(180000,0.5)).toBe(180000);
 expect(localMachineSpeedFactorV1({SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"8192",SEMIO_HOST_CAPACITY_CORES:"4"})).toBeGreaterThanOrEqual(2.83);
 expect(localMachineSpeedFactorV1({})).toBeGreaterThanOrEqual(1);
});

test("the time text of ps reads as milliseconds, and the processor time of a live process grows while it works",()=>{
 for(const row of fixture.times)expect(cpuTimeTextMillisecondsV1(row.text),row.text).toBe(row.milliseconds ?? undefined);
 const first=processCpuMillisecondsV1(process.pid);
 if(process.platform==="linux"||process.platform==="darwin"){
  expect(first).toBeGreaterThan(0);
  const own=process.cpuUsage(),wall=Date.now()+60000;let sink=0;while(Date.now()<wall&&(process.cpuUsage(own).user+process.cpuUsage(own).system)/1000<700)sink+=Math.sqrt(sink+1);
  expect(processCpuMillisecondsV1(process.pid)!,"the busy loop used processor time").toBeGreaterThan(first!+400);
 }else expect(first).toBeUndefined();
 expect(processCpuMillisecondsV1(0)).toBeUndefined();
});

test("the asynchronous processor-time probe reads only the requested pid and equals the independent Python model",async()=>{
 expect(new Ajv({strict:true}).validate(sampleSchema,samples)).toBe(true);
 const model=Bun.spawnSync([python!,"-I",join(here,"📡️cpu","🐍️.py"),join(here,"📡️cpu","🔣️.json")]);
 expect(model.exitCode).toBe(0);
 const expected=JSON.parse(model.stdout.toString());
 for(const [at,row] of samples.entries()){
  const calls:string[][]=[];
  const measured=await readProcessCpuMillisecondsV1(row.pid,{platform:row.platform,run:async(command,args)=>{calls.push([command,...args]);return row.text;},read:async path=>{calls.push([path]);return row.text;}});
  expect(measured).toBe(expected[at]??undefined);
  expect(calls).toEqual(row.pid===0||row.platform==="win32"?[]:row.platform==="darwin"?[["ps","-o","time=","-p",String(row.pid)]]:[[`/proc/${row.pid}/stat`]]);
 }
});

test("a pending host sample leaves the next event-loop turn available and a withdrawn result is dropped",async()=>{
 for(const platform of ["darwin","linux"]){
  const issued=new AbortController(),reply=Promise.withResolvers<string>();let calls=0,commandSignal:AbortSignal|undefined;
  const pending=readProcessCpuMillisecondsV1(42,{platform,signal:issued.signal,run:(_command,_args,signal)=>{calls++;commandSignal=signal;return reply.promise;},read:()=>{calls++;return reply.promise;}});
  await new Promise(resolve=>setTimeout(resolve,0));
  expect(calls).toBe(1);
  issued.abort();
  if(platform==="darwin")expect(commandSignal?.aborted).toBe(true);
  reply.resolve(platform==="darwin"?samples[0]!.text:samples[1]!.text);
  expect(await pending).toBeUndefined();
  expect(await readProcessCpuMillisecondsV1(42,{platform,signal:issued.signal,run:()=>{calls++;return reply.promise;},read:()=>{calls++;return reply.promise;}})).toBeUndefined();
  expect(calls).toBe(1);
 }
});
