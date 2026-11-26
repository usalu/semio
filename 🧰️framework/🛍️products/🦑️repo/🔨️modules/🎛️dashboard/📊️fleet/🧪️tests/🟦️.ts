import {expect,test} from "bun:test";
import {copyFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import {closeMetricsRingsV1,metricSupportedOnV1,openMetricsRingsV1,readSeriesV1,recordSampleV1,type MetricsTierV1} from "../../../../../../🔨️modules/🏃️process/🏗️build/📈️metrics/🟦️.ts";

/** 📊️ Contract: the synthetic fleet store of the fixtures is exactly what the TypeScript ring writer produces from the samples below. The Rust reader, the dashboard and the language-agnostic goldens read these bytes. */
const here=import.meta.dir,store=join(here,"🧫️fixtures","🏪️store","origins");
export const T0=1_800_000_000_000;
const TIERS:readonly MetricsTierV1[]=[{id:"t0",bucketMs:1000,buckets:30},{id:"t1",bucketMs:5000,buckets:24},{id:"t2",bucketMs:30000,buckets:20}];
const IDS=["host.cpu.busy","host.cpu.load","host.memory.total","host.memory.available","host.disk.total","host.disk.free","host.swap.used","process.repo.count","build.tasks.running","build.units.compiled","build.units.restored.local","cache.local.hit","cache.local.miss","cache.remote.ac.hit","cache.remote.ac.miss","exec.offload.out","exec.refused","ready.state","ready.stage"];
const GIB=1024**3;

export function samplesOf(variant:"busy"|"short",tick:number):Record<string,number>{
 const scale=variant==="busy"?1:0.5;
 const cpu=Math.round((40+30*Math.sin(tick/20))*scale*4)/4;
 return {
  "host.cpu.busy":cpu,"host.cpu.load":Math.round(cpu/50*100)/100,
  "host.memory.total":16*GIB,"host.memory.available":Math.round((6+2*Math.cos(tick/30))*GIB),
  "host.disk.total":1e12,"host.disk.free":4e11-tick*1e6,"host.swap.used":2*GIB,"process.repo.count":10+tick%5,
  "build.tasks.running":Math.floor(tick/10)%5,
  "build.units.compiled":tick%7===0?3:tick%3===0?1:0,"build.units.restored.local":tick%4===0?2:0,
  "cache.local.hit":tick%2?5:3,"cache.local.miss":1,
  "cache.remote.ac.hit":2,"cache.remote.ac.miss":tick%5===0?1:0,
  "exec.offload.out":tick%40===0?2:0,"exec.refused":tick%97===0?1:0,
  "ready.state":3,"ready.stage":0,
 };
}

const FLEET_SOURCES:Record<string,readonly string[]>={"remote-execution":["exec.queue.running","exec.queue.queued","exec.offload.in"],"remote-cache":["cache.remote.entries","cache.remote.bytes"]};
export function fleetSamplesOf(tick:number):Record<string,number>{
 return {"exec.queue.running":tick%6,"exec.queue.queued":tick%11===0?4:0,"exec.offload.in":tick%50===0?1:0,"cache.remote.entries":1000+tick,"cache.remote.bytes":5e6+tick*1000};
}
const fleetStore=join(here,"🧫️fixtures","🏪️store","fleet","metrics");

function write(directory:string,variant:"busy"|"short",ticks:number):void{
 mkdirSync(directory,{recursive:true});
 const handle=openMetricsRingsV1({directory,ids:IDS,tiers:TIERS,createdMs:T0});
 for(let tick=0;tick<ticks;tick++)recordSampleV1(handle,T0+tick*1000,samplesOf(variant,tick));
 closeMetricsRingsV1(handle);
}

test("fixtures: the checked-in rings are byte for byte what the TypeScript writer produces", () => {
 const scratch=mkdtempSync(join(tmpdir(),"fleet-fixture-"));
 try{
  for(const [origin,variant,ticks] of [["g1-m01","busy",300],["g1-m02","short",60]] as const){
   const directory=join(scratch,origin);write(directory,variant,ticks);
   const target=join(store,origin,"metrics");
   if(process.env["FLEET_FIXTURE_WRITE"]==="1"){mkdirSync(target,{recursive:true});for(const file of readdirSync(directory))copyFileSync(join(directory,file),join(target,file));}
   for(const tier of TIERS){
    const file=`${tier.id}.ring`;
    expect(existsSync(join(target,file)),`${origin}/${file} missing; rerun with FLEET_FIXTURE_WRITE=1`).toBe(true);
    expect(readFileSync(join(directory,file)).equals(readFileSync(join(target,file))),`${origin}/${file} drifted`).toBe(true);
   }
  }
 for(const [source,ids] of Object.entries(FLEET_SOURCES)){
  const directory=join(scratch,"fleet-"+source);mkdirSync(directory,{recursive:true});
  const handle=openMetricsRingsV1({directory,ids,tiers:TIERS,createdMs:T0});
  for(let tick=0;tick<300;tick++){const all=fleetSamplesOf(tick),values:Record<string,number>={};for(const id of ids)values[id]=all[id]!;recordSampleV1(handle,T0+tick*1000,values);}
  closeMetricsRingsV1(handle);
  const target=join(fleetStore,source);
  if(process.env["FLEET_FIXTURE_WRITE"]==="1"){mkdirSync(target,{recursive:true});for(const file of readdirSync(directory))copyFileSync(join(directory,file),join(target,file));}
  for(const tier of TIERS)expect(readFileSync(join(directory,`${tier.id}.ring`)).equals(readFileSync(join(target,`${tier.id}.ring`))),`fleet ${source}/${tier.id}.ring drifted`).toBe(true);
 }
 }finally{rmSync(scratch,{recursive:true,force:true});}
});

const schema=JSON.parse(readFileSync(join(here,"..","..","🧬️schema","📊️fleet","🔣️.json"),"utf8")) as {definitions:Record<string,{const?:unknown}>};
const golden=(name:string)=>JSON.parse(readFileSync(join(here,"🧫️fixtures","🏅️golden",name),"utf8")) as Snapshot;
type Snapshot={atMs:number;range:string;columns:number;fromMs:number;toMs:number;store:string;rows:Row[];detail?:{origin:string;groups:{id:string;series:{id:string;kind:string;unit:string;points:number[][]}[]}[]}};
type Row={origin:string;freshness:string;hostName:string;duplicates:number;metricsMs?:number;cpu?:number;memory?:number;disk?:number;unitsPerMinute?:number;localHit?:number;localMiss?:number;remoteHit?:number;remoteMiss?:number;offloadIn?:number;offloadOut?:number;cpuSeries?:number[][];unitsSeries?:number[][]};
const round3=(value:number):number=>Math.round(value*1000)/1000;
type Totals={offloadIn?:number;queueRunning?:number;queueQueued?:number;remoteEntries?:number;remoteBytes?:number};
const GOLDENS=["overview-5m.json","overview-1h.json","overview-90d.json","detail-g1-m01-1h.json"];

test("goldens: Ajv independently accepts every snapshot the Rust reader wrote and refuses broken ones", () => {
 const ajv=new Ajv({strict:false}),wrap=(name:string)=>ajv.compile({definitions:schema.definitions,$ref:`#/definitions/${name}`});
 const snapshot=wrap("Snapshot"),request=wrap("Request");
 for(const name of GOLDENS)expect(snapshot(golden(name)),`${name}: ${JSON.stringify(snapshot.errors)}`).toBe(true);
 for(const valid of [{range:"5m",columns:16},{range:"90d",columns:240,origin:"g1-m01"}])expect(request(valid),JSON.stringify(valid)).toBe(true);
 for(const invalid of [{range:"2h",columns:16},{range:"5m",columns:7},{range:"5m",columns:16,extra:1},{range:"5m",columns:16,origin:""}])expect(request(invalid),JSON.stringify(invalid)).toBe(false);
 const broken={...golden("overview-5m.json"),rows:[{origin:"g1-m01",freshness:"asleep",hostName:"ok",duplicates:0}]};
 expect(snapshot(broken)).toBe(false);
 const leaked=JSON.stringify(GOLDENS.map(golden));
 for(const word of ["address","hostLabel","192.168",".lan"])expect(leaked.includes(word),word).toBe(false);
});

test("n/a: a metric the platform cannot measure travels as an unsupported series without points, and the TypeScript catalogue helper agrees with the Windows set of the drill-down", () => {
 const ajv=new Ajv({strict:false}),wrap=(name:string)=>ajv.compile({definitions:schema.definitions,$ref:`#/definitions/${name}`});
 const series=wrap("Series"),row=wrap("Row");
 const base={id:"host.cpu.load",kind:"gauge",unit:"count",scope:"origin",points:[]};
 expect(series({...base,unsupported:true}),JSON.stringify(series.errors)).toBe(true);
 expect(series(base),"a series that is merely empty is not marked").toBe(true);
 expect(series({...base,unsupported:false}),"the flag is only ever set").toBe(false);
 expect(row({origin:"g1-m05",freshness:"live",hostName:"ok",duplicates:0,unsupported:["host.cpu.busy"]})).toBe(true);
 expect(row({origin:"g1-m05",freshness:"live",hostName:"ok",duplicates:0,unsupported:[1]})).toBe(false);
 const hostGroup=(schema.definitions["Groups"]!.const as {id:string;metrics:string[]}[]).find(group=>group.id==="host")!;
 expect(hostGroup.metrics.filter(id=>!metricSupportedOnV1(id,"win32"))).toEqual(["host.cpu.load","host.swap.used"]);
 expect(hostGroup.metrics.filter(id=>!metricSupportedOnV1(id,"darwin"))).toEqual([]);
});

test("goldens: the TypeScript reader and independent arithmetic reproduce every overview number of the live origin", () => {
 const policy=schema.definitions["Policy"]!.const as {staleBeats:number;defaultBeatMs:number};
 for(const name of ["overview-5m.json","overview-1h.json","overview-90d.json"]){
  const snapshot=golden(name),row=snapshot.rows.find(candidate=>candidate.origin==="g1-m01")!;
  const series=readSeriesV1({directory:join(store,"g1-m01","metrics"),fromMs:snapshot.fromMs,toMs:snapshot.toMs});
  const last=(id:string):number|undefined=>[...(series.metrics[id]??[])].reverse().find(point=>point[3]>0)?.[7];
  const total=(id:string):number|undefined=>{const points=series.metrics[id]??[];return points.some(point=>point[3]>0)?points.reduce((sum,point)=>sum+point[6],0):undefined;};
  const used=(free:string,all:string):number|undefined=>{const a=last(free),b=last(all);return a===undefined||b===undefined||b<=0?undefined:round3(Math.min(100,Math.max(0,(b-a)/b*100)));};
  expect(row.cpu).toBe(last("host.cpu.busy")===undefined?undefined:round3(last("host.cpu.busy")!));
  expect(row.memory).toBe(used("host.memory.available","host.memory.total"));
  expect(row.disk).toBe(used("host.disk.free","host.disk.total"));
  const units=(series.metrics["build.units.compiled"]??[]).filter(point=>point[3]>0);
  const bucket=(point:readonly number[]):number=>TIERS[point[2]!]!.bucketMs;
  const weighted=units.reduce((sum,point)=>sum+point[6]!/bucket(point)*60000*point[1]!,0)/units.reduce((sum,point)=>sum+point[1]!,0);
  expect(Math.abs(row.unitsPerMinute!-weighted),`${name} units/min ${row.unitsPerMinute} vs ${weighted}`).toBeLessThan(0.002);
  for(const [key,id] of [["localHit","cache.local.hit"],["localMiss","cache.local.miss"],["remoteHit","cache.remote.ac.hit"],["remoteMiss","cache.remote.ac.miss"],["offloadOut","exec.offload.out"]] as const){
   const expected=total(id);expect(row[key],`${name} ${key}`).toBe(expected===undefined?undefined:round3(expected));
  }
  expect(row.cpuSeries!.every(point=>point[2]!<=point[4]!+1e-9&&point[4]!<=point[3]!+1e-9)).toBe(true);
  const ready=JSON.parse(readFileSync(join(store,"g1-m01","ready.json"),"utf8")) as {updatedMs:number;beatMilliseconds?:number};
  expect(row.freshness).toBe(snapshot.atMs-ready.updatedMs<=policy.staleBeats*(ready.beatMilliseconds??policy.defaultBeatMs)?"live":"stale");
 }
 const stale=golden("overview-5m.json").rows.find(candidate=>candidate.origin==="g1-m02")!,missing=golden("overview-5m.json").rows.find(candidate=>candidate.origin==="g1-m03")!;
 expect([stale.freshness,stale.hostName,stale.duplicates,missing.freshness,missing.cpu,missing.cpuSeries]).toEqual(["stale","warning",1,"missing",undefined,undefined]);
});

test("goldens: a counter is a rate per minute and a drill-down thins every tier run without moving its boundaries", () => {
 const detail=golden("detail-g1-m01-1h.json"),groups=detail.detail!.groups;
 expect(groups.map(group=>group.id)).toEqual(["host","build","cache","exec","graph","ready"]);
 const units=groups[1]!.series.find(series=>series.id==="build.units.compiled")!;
 expect(units.kind).toBe("counter");
 expect(Math.max(...units.points.map(point=>point[3]!))).toBeLessThanOrEqual(3*60+1e-9);
 for(const group of groups)for(const series of group.series){
  expect(series.points.length,series.id).toBeGreaterThan(0);
  for(let index=1;index<series.points.length;index++)expect(series.points[index]![0]!,`${series.id} ascending`).toBeGreaterThanOrEqual(series.points[index-1]![0]!+series.points[index-1]![1]!-1);
 }
 const widths=groups[0]!.series[0]!.points.map(point=>point[1]!),changes=widths.filter((width,index)=>index>0&&width!==widths[index-1]).length;
 expect(changes,"two tier boundaries plus the partial bucket that straddles the finer tier").toBeLessThanOrEqual(3);
});

test("goldens: the fleet-scope facts come from the fleet rings of the store, never from an origin", () => {
 const fleet=join(store,"..","fleet","metrics"),snapshot=golden("overview-5m.json") as Snapshot&{fleet?:Totals};
 const read=(source:string)=>readSeriesV1({directory:join(fleet,source),fromMs:snapshot.fromMs,toMs:snapshot.toMs}).metrics;
 const execution=read("remote-execution"),cache=read("remote-cache");
 const last=(points:readonly (readonly number[])[]|undefined)=>[...(points??[])].reverse().find(point=>point[3]!>0)?.[7];
 const sum=(points:readonly (readonly number[])[]|undefined)=>(points??[]).reduce((total,point)=>total+point[6]!,0);
 expect(snapshot.fleet).toEqual({offloadIn:round3(sum(execution["exec.offload.in"])),queueRunning:round3(last(execution["exec.queue.running"])!),queueQueued:round3(last(execution["exec.queue.queued"])!),remoteEntries:round3(last(cache["cache.remote.entries"])!),remoteBytes:round3(last(cache["cache.remote.bytes"])!)});
 for(const row of snapshot.rows)expect(row.offloadIn,row.origin).toBeUndefined();
 const exec=golden("detail-g1-m01-1h.json").detail!.groups.find(group=>group.id==="exec")!;
 expect(exec.series.find(series=>series.id==="exec.queue.running")).toMatchObject({scope:"fleet"});
 expect(exec.series.find(series=>series.id==="exec.offload.out")).toMatchObject({scope:"origin"});
});
