/** ⏳️ Contract: a farm job that makes progress is never killed by a wall-clock cap, and one that makes none is: the job timer is a window without processor time (re-armed whenever the child used processor time since the last look), lengthened by the speed factor of the machine, and where the platform cannot say how much processor time a child used the cap stays a plain wall-clock window.
 * @see ../../🏊️pool/🟨️.mjs */
import{expect,test}from"bun:test";
import{dirname,join}from"node:path";
import{fileURLToPath,pathToFileURL}from"node:url";
import{GRAPH_FARM_POLICY_V1,createFarmV1}from"../../🏊️pool/🟨️.mjs";

const here=dirname(fileURLToPath(import.meta.url)),worker=join(here,"..","..","👷️worker","🟨️.mjs"),service=pathToFileURL(join(here,"🛰️service","🟨️.mjs")).href;
const live={livenessMilliseconds:100,livenessCpuMilliseconds:2},policy={...GRAPH_FARM_POLICY_V1,jobTimeoutMilliseconds:700,footprintMilliseconds:100,maximumAttempts:1};
const farm=(options:Record<string,unknown>={})=>createFarmV1({entry:worker,service,session:{identity:"law"},workers:1,megabytes:100000,environment:{...process.env,SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:undefined,SEMIO_HOST_CAPACITY_CORES:undefined},accept:(identity:unknown)=>identity==="law",policy,liveness:live,speed:1,...options} as never);

test("a job that spends processor time outlives the cap, a job that waits without spending any is lost at the cap",async()=>{
 if(process.platform==="win32")return;
 const pool=farm();
 const started=Date.now(),busy=await pool.run({kind:"spin",milliseconds:2600});
 expect((busy.reply as {spun:number}).spun,"3.7 caps of busy work are not a timeout").toBe(2600);expect(Date.now()-started).toBeGreaterThanOrEqual(2500);
 const waited=Date.now(),idle=await pool.run({kind:"idle",milliseconds:5000});
 expect(idle.lost).toBe("job timeout");expect(Date.now()-waited).toBeLessThan(2500);
 await pool.close();
},60000);

test("where the processor time of a child is unknown the cap is a wall-clock window as before",async()=>{
 const pool=farm({cpuOf:()=>undefined}),busy=await pool.run({kind:"spin",milliseconds:2600});
 expect(busy.lost).toBe("job timeout");
 await pool.close();
},60000);

test("the speed factor of the machine lengthens both timers: a job that waits is lost after the cap times the factor",async()=>{
 const pool=farm({speed:2.5,cpuOf:()=>undefined}),started=Date.now(),idle=await pool.run({kind:"idle",milliseconds:6000});
 expect(idle.lost).toBe("job timeout");expect(Date.now()-started).toBeGreaterThanOrEqual(1750-50);
 await pool.close();
},60000);

test("one blocked CPU sample leaves the deadline live and withdraws when that exact child is retired",async()=>{
 const started=Promise.withResolvers<void>(),reply=Promise.withResolvers<number>(),observations:{pid:number;signal?:AbortSignal}[]=[];
 const pool=farm({footprintOf:()=>0,cpuOf:(pid:number,options?:{signal:AbortSignal})=>{observations.push({pid,signal:options?.signal});started.resolve();return reply.promise;}});
 try{
  const running=pool.run({kind:"idle",milliseconds:5000});
  await started.promise;
  await new Promise(resolve=>setTimeout(resolve,250));
  expect(observations).toHaveLength(1);
  expect(observations[0]!.pid).toBeGreaterThan(0);
  expect(observations[0]!.signal?.aborted).toBe(false);
  expect((await running).lost).toBe("job timeout");
  expect(observations[0]!.signal?.aborted).toBe(true);
  reply.resolve(1e6);
 }finally{reply.resolve(1e6);await pool.close();}
},60000);

test("a successful job withdraws its sample before the same child accepts another job",async()=>{
 const started=Promise.withResolvers<AbortSignal>(),reply=Promise.withResolvers<number>();let probes=0;
 const pool=farm({footprintOf:()=>0,cpuOf:(_pid:number,{signal}:{signal:AbortSignal})=>{if(++probes!==1)return undefined;started.resolve(signal);return reply.promise;}});
 try{
  const first=pool.run({kind:"idle",milliseconds:350}),sampling=await started.promise;
  expect((await first).reply).toEqual({idled:350});
  expect(sampling.aborted).toBe(true);
  const second=pool.run({kind:"idle",milliseconds:5000}),before=Date.now();
  reply.resolve(1e6);
  expect((await second).lost).toBe("job timeout");
  expect(Date.now()-before).toBeLessThan(2500);
  expect(probes).toBeGreaterThan(1);
 }finally{reply.resolve(1e6);await pool.close();}
},60000);
