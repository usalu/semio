/** 🤫️ Contract: a quiet forecast holds until work really runs. A nested invocation that inherits a quiet grant runs at once beside a full machine with the holder's plan and never grows the grant itself; the holder grows a quiet grant whose tree outgrows the quiet reservation (work no executor announced) in the fair queue while the run goes on, says so in English and German, and the run is learned as executed; a quiet run that stays within its reservation never grows and is learned as quiet.
 * @see ./🥒️.feature
 * @see ../../🎫️gate/🟦️.ts
 * @see ../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/result-quiet16.md */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {newHostCapacityIdV1,readHostCapacityPolicyV1,readHostCapacityStatusV1,requestHostCapacityUpgradeV1,withHostCapacityV1,type HostCapacityHandleV1,type HostCapacityOptionsV1,type HostRequestV1,type HostRunV1,type HostSampleV1} from "../../🟦️.ts";
import decisionFixture from "../🧫️fixtures/🔣️.json";

const policy=readHostCapacityPolicyV1({...decisionFixture.policy,load:{...decisionFixture.policy.load,overrideAfterMilliseconds:600000},wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0,lockStaleMilliseconds:2000,staleMilliseconds:60000}});
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"q16-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const control=(signal=new AbortController().signal)=>({signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const sized=(id:string,fixed:number,over:object={}):HostRequestV1=>({id,operation:"run",priority:"batch",minimumJobs:1,preferredJobs:3,fixedMegabytes:fixed,perJobMegabytes:100,cpuPerJob:1,...over});
const quietOf=(full:HostRequestV1):HostRequestV1=>({...full,fixedMegabytes:500,minimumJobs:1,preferredJobs:1,quiet:true,rampSlot:false,perJobMegabytes:0});
const options=(directory:string,request:HostRequestV1,over:Partial<HostCapacityOptionsV1>={}):HostCapacityOptionsV1=>({directory,policy,request,control:control(),environment:{},write:()=>undefined,sample:async()=>({host:host(),swappedOutMegabytes:0}),...over});
const environmentOf=(directory:string,handle:HostCapacityHandleV1)=>({SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_HOST_CAPACITY_GRANT:handle.id,SEMIO_HOST_CAPACITY_PLAN:JSON.stringify(handle.plan)});
const status=async(directory:string)=>readHostCapacityStatusV1({directory,policy,control:control(),sample:async()=>({host:host(),swappedOutMegabytes:0})});
const holdBig=(directory:string)=>{let release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});const done=withHostCapacityV1(options(directory,sized(newHostCapacityIdV1(),8000,{preferredJobs:1,perJobMegabytes:0})),async()=>hold);return{release,done};};

test("A nested invocation that inherits a quiet grant runs at once beside a full machine with the holder's jobs, leaves the grant quiet and grows it only when it asks for work",async()=>{
 const directory=fresh(),big=holdBig(directory);
 await sleep(40);
 const full=sized(newHostCapacityIdV1(),9000);let nested:HostCapacityHandleV1|undefined,ranAt=0,asked:string|undefined;
 await withHostCapacityV1(options(directory,quietOf(full),{upgrade:full}),async handle=>{
  const started=Date.now();
  await withHostCapacityV1(options(directory,sized(newHostCapacityIdV1(),9000),{environment:environmentOf(directory,handle)}),async inherited=>{
   ranAt=Date.now()-started;nested=inherited;
   expect((await status(directory)).grants.find(row=>row.id===handle.id)!.quiet,"inheriting grows nothing").toBe(true);
   const waiting=requestHostCapacityUpgradeV1(environmentOf(directory,handle),control());
   await sleep(300);
   expect((await status(directory)).grants.find(row=>row.id===handle.id)!.quiet,"an executor's ask waits for the full machine").toBe(true);
   big.release();asked=await waiting;
  });
 });
 await big.done;
 expect(ranAt,"the nested body starts beside the full machine").toBeLessThan(250);
 expect(nested).toMatchObject({inherited:true,jobs:3,cpu:3,reservedMegabytes:500});expect(nested!.plan.jobs).toBe(3);
 expect(asked).toBe("upgraded");
},30000);

test("A quiet grant whose process tree outgrows its quiet reservation grows in the fair queue while the run goes on, says so in English and German, and the run is learned as executed",async()=>{
 for(const [language,waiting,grown] of [["en_US.UTF-8",/^\[capacity\] run: work has to run, waiting \d+s to grow the reservation from 500 MB to 9300 MB: not enough free memory/u,/^\[capacity\] run: reservation grown from 500 MB to 9300 MB after \d+s/u],["de_DE.UTF-8",/^\[capacity\] run: es muss Arbeit laufen, wartet seit \d+ s darauf, die Reservierung von 500 MB auf 9300 MB zu vergrößern: nicht genug freier Speicher/u,/^\[capacity\] run: Reservierung nach \d+ s von 500 MB auf 9300 MB vergrößert/u]] as const){
  const directory=fresh(),big=holdBig(directory),lines:string[]=[],learned:HostRunV1[]=[],ticks:number[]=[];let level=300;
  await sleep(40);
  const full=sized(newHostCapacityIdV1(),9000);
  await withHostCapacityV1(options(directory,quietOf(full),{upgrade:full,environment:{LANG:language},write:line=>lines.push(line),learn:run=>{learned.push(run);},observe:async()=>level}),async handle=>{
   await sleep(80);level=2600;
   for(let tick=0;tick<30;tick++){ticks.push(Date.now());await sleep(20);}
   const row=(await status(directory)).grants.find(item=>item.id===handle.id)!;
   expect(row.quiet,"still quiet while the machine is full").toBe(true);
   expect((await status(directory)).waiters.some(item=>item.request.id===handle.id&&item.upgrade===true),"the growth waits in the fair queue").toBe(true);
   big.release();
   for(let tick=0;tick<100&&(await status(directory)).grants.find(item=>item.id===handle.id)!.quiet===true;tick++)await sleep(20);
   expect((await status(directory)).grants.find(item=>item.id===handle.id)).toMatchObject({reservedMegabytes:9300,jobs:3});
  });
  await big.done;
  expect(Math.max(...ticks.slice(1).map((at,index)=>at-ticks[index]!)),"the run is never paused by the growth").toBeLessThan(400);
  expect(lines.some(line=>waiting.test(line)),language).toBe(true);expect(lines.some(line=>grown.test(line)),language).toBe(true);
  expect(learned.map(run=>[run.executed,run.jobs,run.peakMegabytes])).toEqual([[true,3,2600]]);
  expect(readdirSync(join(directory,"grants"))).toEqual([]);expect(readdirSync(join(directory,"queue"))).toEqual([]);
 }
},30000);

test("A quiet run that ends while its growth still waits leaves no waiter behind and is learned as executed",async()=>{
 const directory=fresh(),big=holdBig(directory),learned:HostRunV1[]=[];let level=300;
 await sleep(40);
 const full=sized(newHostCapacityIdV1(),9000);
 await withHostCapacityV1(options(directory,quietOf(full),{upgrade:full,learn:run=>{learned.push(run);},observe:async()=>level}),async handle=>{
  await sleep(60);level=1800;await sleep(300);
  expect((await status(directory)).waiters.some(item=>item.request.id===handle.id&&item.upgrade===true)).toBe(true);
 });
 await sleep(200);
 expect((await status(directory)).waiters.filter(item=>item.upgrade===true)).toEqual([]);
 expect(learned.map(run=>[run.executed,run.peakMegabytes])).toEqual([[true,1800]]);
 big.release();await big.done;
},30000);

test("A quiet run whose tree stays within its quiet reservation never grows and is learned as quiet with the full jobs",async()=>{
 const directory=fresh(),lines:string[]=[],learned:HostRunV1[]=[];
 const full=sized(newHostCapacityIdV1(),9000);
 await withHostCapacityV1(options(directory,quietOf(full),{upgrade:full,write:line=>lines.push(line),learn:run=>{learned.push(run);},observe:async()=>480}),async handle=>{
  await sleep(250);
  expect((await status(directory)).grants.find(item=>item.id===handle.id)).toMatchObject({quiet:true,reservedMegabytes:500,jobs:1});
 });
 expect(lines.filter(line=>/grow|grown|vergrößer/u.test(line))).toEqual([]);
 expect(learned.map(run=>[run.executed,run.jobs,run.peakMegabytes])).toEqual([[false,3,480]]);
});
