import {expect,test}from"bun:test";
import Ajv from"ajv";
import {spawnSync}from"node:child_process";
import {fileURLToPath}from"node:url";
import {testPythonV1}from"../../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {admitLocalComponentJobV1,componentPassPeakMegabytesV1,serialFinalHistoryV1,describeLocalComponentJobsV1,planComponentJobSlotsV1,planLocalComponentJobsV1,runPassesInJobSlotsV1,type LocalJobAdmitHostV1,type LocalJobPressureV1,type LocalJobSampleV1,type LocalJobsHostV1,type PassJobSlotV1,type ComponentJobSlotsHostV1}from"../../🟦️.ts";
import schema from"../../🧬️schema/🔣️.json";
import fixture from"./🧫️fixtures/🔣️.json";
const oracle=fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url)),schemaPath=fileURLToPath(new URL("../../🧬️schema/🔣️.json",import.meta.url));
const python=(cases:unknown):unknown[]=>{const result=spawnSync(testPythonV1()??"python3",["-I",oracle,schemaPath],{input:JSON.stringify(cases),encoding:"utf8",maxBuffer:1<<26});expect(result.status,result.stderr).toBe(0);return JSON.parse(result.stdout);};
const policy=schema.definitions.Policy.const;
test("local job fixtures are valid for the schema oracle and decided exactly as written down",()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(schema,"jobs");const get=(name:string)=>ajv.getSchema("jobs#/definitions/"+name)!;
 for(const row of fixture.plan){expect(get("LocalJobsHost")(row.host),row.name).toBe(true);expect(get("LocalJobsDecision")(row.expect),row.name).toBe(true);expect(planLocalComponentJobsV1(row.host as LocalJobsHostV1),row.name).toEqual(row.expect as never);}
 for(const row of fixture.admit){expect(get("LocalJobAdmitHost")(row.host),row.name).toBe(true);expect(get("LocalJobAdmitDecision")(row.expect),row.name).toBe(true);expect(admitLocalComponentJobV1(row.host as LocalJobAdmitHostV1),row.name).toEqual(row.expect as never);}
 for(const row of fixture.slots){expect(get("ComponentJobSlotsHost")(row.host),row.name).toBe(true);expect(get("ComponentJobSlotsDecision")(row.expect),row.name).toBe(true);expect(planComponentJobSlotsV1(row.host),row.name).toEqual(row.expect as never);}
 const checks={plan:[planLocalComponentJobsV1,"LocalJobsHost"],admit:[admitLocalComponentJobV1,"LocalJobAdmitHost"],slots:[planComponentJobSlotsV1,"ComponentJobSlotsHost"]} as const;
 for(const row of fixture.invalid){const[run,definition]=checks[row.kind as keyof typeof checks];expect(get(definition)(row.host),row.name).toBe(false);expect(()=>(run as(host:unknown)=>unknown)(row.host),row.name).toThrow();}
});
test("the vectors are the ones of the Python model, also for every state of a generated grid",()=>{
 const cases:{kind:"plan"|"admit"|"slots";host:unknown}[]=[...fixture.plan.map(row=>({kind:"plan" as const,host:row.host})),...fixture.admit.map(row=>({kind:"admit" as const,host:row.host})),...fixture.slots.map(row=>({kind:"slots" as const,host:row.host}))];
 const lists=[[1000],[6500,3300,1200],[3300,1200,6500],[1200,3300,6500],[2000,2000,2000,2000],[500,500,500],[9000,100]];
 for(const tokens of[1,2,3,4,8])for(const availableMegabytes of[null,0,3000,5500,9500,12000,40000])for(const floorMegabytes of[0,2048,4000])for(const pressure of[null,"normal","warn","critical"] as const)for(const peaksMegabytes of lists)cases.push({kind:"plan",host:{tokens,availableMegabytes,floorMegabytes,pressure,peaksMegabytes}});
 for(const running of[0,1,2,3])for(const maximum of[1,2,3])for(const availableMegabytes of[null,0,3000,9500])for(const ownedMegabytes of[0,2000,9000])for(const reservedMegabytes of[0,3300,6500])for(const floorMegabytes of[2048,4000])for(const pressure of[null,"normal","warn","critical"] as const)for(const peakMegabytes of[900,3300])cases.push({kind:"admit",host:{running,maximum,availableMegabytes,ownedMegabytes,reservedMegabytes,floorMegabytes,pressure,peakMegabytes}});
 for(const fleetJobs of[1,2,6,24,40,100])for(const tokens of[1,2,3,4,64])for(const wanted of[1,2,3,1024])cases.push({kind:"slots",host:{fleetJobs,tokens,wanted}});
 expect(cases.length).toBeGreaterThan(6000);
 const expected=python(cases),run={plan:planLocalComponentJobsV1,admit:admitLocalComponentJobV1,slots:planComponentJobSlotsV1} as Record<string,(host:never)=>unknown>;
 cases.forEach((row,index)=>expect(run[row.kind]!(row.host as never),JSON.stringify(row)).toEqual(expected[index] as never));
});
test("the plan of local jobs is the count of starts the live decision grants one after the other, so plan and run agree",()=>{
 const lists=[[1000],[6500,3300,1200],[3300,1200,6500],[1200,3300,6500],[2000,2000,2000,2000],[500,500,500],[9000,100]];
 for(const tokens of[1,2,4,8])for(const availableMegabytes of[null,0,5500,9500,40000])for(const floorMegabytes of[0,4000])for(const pressure of[null,"normal","warn"] as LocalJobPressureV1[])for(const peaksMegabytes of lists){
  const planned=planLocalComponentJobsV1({tokens,availableMegabytes,floorMegabytes,pressure,peaksMegabytes}).jobs,maximum=Math.max(1,Math.min(policy.localJobsMaximum,tokens));
  let started=0,reserved=0;
  for(const peak of peaksMegabytes){if(!admitLocalComponentJobV1({running:started,maximum,availableMegabytes,ownedMegabytes:0,reservedMegabytes:reserved,floorMegabytes,pressure,peakMegabytes:peak}).admit)break;started++;reserved+=peak;}
  expect(planned,JSON.stringify({tokens,availableMegabytes,floorMegabytes,pressure,peaksMegabytes})).toBe(started);
 }
});
test("without a fleet the slots of a lane come from the local grant and memory only, with a fleet the light jobs stay and the cap holds",()=>{
 for(const tokens of[1,2,3,4,64]){const slots=planComponentJobSlotsV1({fleetJobs:1,tokens,wanted:1024});expect(slots.light).toBe(0);expect(slots.jobs).toBe(slots.local);expect(slots.local).toBe(Math.min(policy.localJobsMaximum,tokens));}
 for(const fleetJobs of[2,6,24,100]){const slots=planComponentJobSlotsV1({fleetJobs,tokens:4,wanted:1024});expect(slots.jobs).toBeLessThanOrEqual(policy.componentJobsMaximum);expect(slots.light).toBe(Math.min(fleetJobs-1,policy.componentJobsMaximum-1));expect(slots.light+slots.local).toBeGreaterThanOrEqual(slots.jobs);}
});
test("component finals are reserved at the recorded peak of their crate, an unseen final at the high quantile of the recorded finals",()=>{
 const row=(key:string,mb:number)=>[key,[{v:1 as const,key,peakRssBytes:mb*1048576,outputBytes:0,wallMs:1,cpuMs:1,remainingMs:0,atMs:1}]] as const;
 const key=(crate:string)=>"semio_"+crate+"@"+crate+"|wasm32-wasip2|release|cdylib";
 const history=new Map([row(key("a_component"),1000),row(key("b_component"),2000),row(key("c_component"),4000),row("x@lib|host|release|lib",50000),row(key("a_component").replace("release","prototype"),60000)]);
 const pass=(...names:string[])=>({profile:"release",packages:names.map(name=>({name}))});
 expect(componentPassPeakMegabytesV1(history,pass("a-component"),4)).toBeGreaterThanOrEqual(1000);expect(componentPassPeakMegabytesV1(history,pass("a-component"),4)).toBeLessThan(1500);
 expect(componentPassPeakMegabytesV1(history,pass("c-component"),4)).toBeGreaterThanOrEqual(4000);
 const unseen=componentPassPeakMegabytesV1(history,pass("z-component"),4);expect(unseen).toBeGreaterThan(componentPassPeakMegabytesV1(history,pass("b-component"),4));
 expect(componentPassPeakMegabytesV1(new Map(),pass("z-component"),4)).toBe(6144);
 expect(componentPassPeakMegabytesV1(history,pass("a-component","c-component"),4)).toBeGreaterThan(componentPassPeakMegabytesV1(history,pass("c-component"),4));
 expect(componentPassPeakMegabytesV1(history,pass("a-component","c-component"),1)).toBe(componentPassPeakMegabytesV1(history,pass("c-component"),1));
});
test("a final is forecast from the serial runs the scheduler now makes: threaded-linker observations are dropped as the Python oracle drops them, and a crate with none keeps all of its rows",()=>{
 const key="semio_hub_norm_component@semio_hub_norm_component|wasm32-wasip2|prototype|cdylib",other="semio_hub_norm_note_component@semio_hub_norm_note_component|wasm32-wasip2|prototype|cdylib";
 const row=(mb:number,wallMs:number,cpuMs:number,at:number)=>({v:1 as const,key,peakRssBytes:mb*1048576,outputBytes:0,wallMs,cpuMs,remainingMs:0,atMs:at});
 const rows=[row(2973,385000,285000,1),row(2141,1624000,1494000,2),row(2966,468000,2091000,3),row(2936,1169000,1115000,4),row(4774,295000,1947000,5),row(3000,1000,1500,6),row(3100,1000,1501,7)];
 const history=new Map([[key,rows],[other,rows.slice(2,3).concat(rows.slice(4,5))]]);
 const expected=(python([{kind:"serial",host:{rows}}]) as number[][])[0]!;
 expect(expected).toEqual([0,1,3,5]);
 expect(serialFinalHistoryV1(history,"prototype").get(key)).toEqual(expected.map(at=>rows[at]));
 expect(serialFinalHistoryV1(history,"prototype").get(other)).toEqual(history.get(other));
 expect(serialFinalHistoryV1(new Map([["x@lib|host|prototype|lib",rows]]),"prototype").get("x@lib|host|prototype|lib")).toEqual(rows);
 const pass={profile:"prototype",packages:[{name:"semio-hub-norm-component"}]};
 const forecast=componentPassPeakMegabytesV1(history,pass,1),kept=componentPassPeakMegabytesV1(new Map([[key,expected.map(at=>rows[at]!)]]),pass,1);
 expect(forecast,"the threaded 4,774 MiB observation does not inflate the forecast").toBe(kept);expect(forecast).toBeLessThan(4000);
});
test("the events are described in English and German on one line",()=>{
 const line=describeLocalComponentJobsV1("waiting",{pass:"release-w2",running:2,peak:3300,free:1200,floor:4000,reason:"local-memory"});
 expect(line).toContain("[component-jobs] release-w2 waits for memory beside 2 running final(s): it needs 3300 MB, 1200 MB are free above the 4000 MB floor (the free memory above the floor does not carry another final)");
 expect(line).toContain(" / [component-jobs] release-w2 wartet auf Speicher neben 2 laufenden Endstück(en)");
});
type Row={id:string;target:string};
const wasm=(count:number):Row[]=>Array.from({length:count},(_,index)=>({id:"p"+index,target:"wasm32-wasip2"}));
const pause=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));
async function until(condition:()=>boolean,milliseconds=2000){const end=Date.now()+milliseconds;while(!condition()&&Date.now()<end)await pause(2);expect(condition()).toBe(true);}
function lane(peaks:Record<string,number>,sample:{current:LocalJobSampleV1},jobs=3,maximum=3){
 const started:string[]=[],events:string[]=[],gates=new Map<string,()=>void>(),slots:Record<string,PassJobSlotV1>={};let running=0,maximumRunning=0,reserved=0,maximumReserved=0;const reservations=new Map<string,number>();
 const done=runPassesInJobSlotsV1(wasm(Object.keys(peaks).length),{jobs,check:()=>undefined,run:(pass,slot)=>new Promise<void>(resolve=>{started.push(pass.id);slots[pass.id]=slot;running++;maximumRunning=Math.max(maximumRunning,running);if(slot==="local"){reservations.set(pass.id,peaks[pass.id]!);reserved+=peaks[pass.id]!;}if(running>=2)maximumReserved=Math.max(maximumReserved,reserved);gates.set(pass.id,()=>{running--;if(reservations.delete(pass.id))reserved-=peaks[pass.id]!;resolve();});}),local:{maximum,floorMegabytes:4000,peak:pass=>peaks[pass.id]!,sample:async()=>sample.current,pollMilliseconds:5,progressMilliseconds:20,report:(pass,event)=>{events.push(event.kind+":"+pass.id+":"+event.reason);}}});
 return{done,started,events,slots,finish:(id:string)=>gates.get(id)!(),running:()=>running,maximumRunning:()=>maximumRunning,maximumReserved:()=>maximumReserved};
}
test("two local jobs overlap with their reservations and a third waits for memory, offload never means no light slot at all",async()=>{
 const sample={current:{availableMegabytes:9500,ownedMegabytes:0,pressure:"normal" as LocalJobPressureV1}},run=lane({p0:3300,p1:1200,p2:6500},sample);
 await until(()=>run.started.length===2);await pause(60);
 expect(run.started).toEqual(["p0","p1"]);expect(run.slots).toEqual({p0:"local",p1:"local"});expect(run.running()).toBe(2);
 expect(run.events.some(event=>event==="started:p1:local-fits")).toBe(true);expect(run.events.some(event=>event==="waiting:p2:local-memory")).toBe(true);
 run.finish("p0");await pause(60);expect(run.started).toEqual(["p0","p1"]);
 run.finish("p1");await until(()=>run.started.length===3);expect(run.slots.p2).toBe("local");run.finish("p2");await run.done;
 expect(run.maximumRunning()).toBe(2);expect(run.maximumReserved()).toBe(4500);
});
test("the memory is read again before each start: freed memory starts the waiting job without a job settling, and a drop makes the next one wait",async()=>{
 const sample={current:{availableMegabytes:5000,ownedMegabytes:0,pressure:"normal" as LocalJobPressureV1}},run=lane({p0:3300,p1:1200},sample);
 await until(()=>run.started.length===1);await pause(60);expect(run.started).toEqual(["p0"]);expect(run.events.some(event=>event==="waiting:p1:local-memory")).toBe(true);
 sample.current={availableMegabytes:9500,ownedMegabytes:0,pressure:"normal"};await until(()=>run.started.length===2);
 run.finish("p0");run.finish("p1");await run.done;
 const dropped={current:{availableMegabytes:9500,ownedMegabytes:0,pressure:"normal" as LocalJobPressureV1}},next=lane({p0:3300,p1:1200,p2:1200},dropped);
 await until(()=>next.started.length===2);dropped.current={availableMegabytes:7000,ownedMegabytes:0,pressure:"normal"};await pause(60);
 expect(next.started).toEqual(["p0","p1"]);next.finish("p0");next.finish("p1");await until(()=>next.started.length===3);next.finish("p2");await next.done;
});
test("raised pressure, unreadable memory and a failing check keep every further local job away; cancellation ends the wait at once",async()=>{
 for(const sample of[{availableMegabytes:60000,ownedMegabytes:0,pressure:"warn" as LocalJobPressureV1},{availableMegabytes:null,ownedMegabytes:0,pressure:null}]){
  const run=lane({p0:100,p1:100},{current:sample});await until(()=>run.started.length===1);await pause(40);expect(run.started).toEqual(["p0"]);run.finish("p0");await until(()=>run.started.length===2);run.finish("p1");await run.done;
 }
 const controller=new AbortController(),gates:(()=>void)[]=[];let cancelled=false;const started:string[]=[];
 const done=runPassesInJobSlotsV1(wasm(2),{jobs:2,check:()=>{if(cancelled)throw Error("cancelled");},signal:controller.signal,fail:()=>gates.forEach(gate=>gate()),run:pass=>new Promise<void>(resolve=>{started.push(pass.id);gates.push(resolve);}),local:{maximum:2,floorMegabytes:4000,peak:()=>100,sample:async()=>({availableMegabytes:0,ownedMegabytes:0,pressure:"normal"}),pollMilliseconds:60000}});
 await until(()=>started.length===1);await pause(20);cancelled=true;controller.abort();
 await expect(done).rejects.toThrow("cancelled");expect(started).toEqual(["p0"]);
});
test("light jobs of the fleet fill their slots first, local jobs beyond the first take the rest only while memory carries them",async()=>{
 const sample={current:{availableMegabytes:30000,ownedMegabytes:0,pressure:"normal" as LocalJobPressureV1}},run=lane({p0:500,p1:500,p2:500,p3:500,p4:500},sample,4,2);
 await until(()=>run.started.length===4);expect(run.started).toEqual(["p0","p1","p2","p3"]);expect(run.slots).toEqual({p0:"local",p1:"fleet",p2:"fleet",p3:"local"});
 for(const id of["p0","p1","p2","p3"])run.finish(id);await until(()=>run.started.length===5);run.finish("p4");await run.done;
});
