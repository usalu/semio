/** 🐢️ Contract: the speed factor of a machine is 1 at or above the reference (8 cores, 16 GB), grows with the missing cores and the square root of the missing memory, and never exceeds four; the processor time of a live process is read on Linux and macOS and grows while the process works. An independent Python model computes the factors from the declared schema.
 * @see ../🧬️schema/🔣️.json */
import{expect,test}from"bun:test";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{MACHINE_SPEED_POLICY_V1,cpuTimeTextMillisecondsV1,localMachineSpeedFactorV1,machineSpeedFactorV1,processCpuMillisecondsV1,scaledMillisecondsV1}from"../🟨️.mjs";
import fixture from"./🔣️.json";

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
