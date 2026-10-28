import {afterAll,beforeAll,expect,test} from "bun:test";
import {runToolSyncV1,startToolV1} from "../../../../../../../../🔨️modules/🏃️process/🧰️tools/🟦️.ts";
import {mkdirSync,mkdtempSync,readdirSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {CACHE_SIZE_FILE_V1,cacheLedgerV1,readCacheLedgerV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {cachePidHolderV1,cacheRunnerPortsV1,collectCacheV1,measureCacheTreeV1,reportCacheLedgerV1} from "../../🏃️runner/🟦️.ts";

const DAY=86400000,HOUR=3600000,MINUTE=60000,purpose={en:"p",de:"q"},oracle=join(import.meta.dir,"🐍️.py"),units="^tmp-(?<pid>[0-9]+)-[A-Za-z0-9]+$";
const partitions={alpha:{directory:".cache/b/a",capacityBytes:1<<30,maximumLogicalBytes:1<<30}};
const document=():any=>({
 version:1,
 roots:{repository:{directory:"r/cache",maximumBytes:1<<30,undeclaredUnusedAgeMs:14*DAY,undeclaredQuietMs:HOUR,ignored:[CACHE_SIZE_FILE_V1]},native:{directory:".cache",maximumBytes:1<<20,undeclaredUnusedAgeMs:7*DAY,undeclaredQuietMs:HOUR,ignored:[]}},
 areas:[
  {id:"alpha",root:"native",kind:"partition",directory:"b/a",partition:"alpha",managed:{owner:"image"},recreate:"derived",writers:["w"],purpose},
  {id:"tmps",root:"native",kind:"nested",directory:"nx/tmp",partition:"alpha",budget:{bytes:100000},collect:{unusedAgeMs:HOUR,quietMs:10*MINUTE,custody:"pid",units},writers:["w"],purpose},
  {id:"cc",root:"native",kind:"nested",directory:"nx/tmp/node-compile-cache",partition:"alpha",budget:{bytes:1<<20},collect:{unusedAgeMs:DAY,quietMs:0,custody:"quiet",depth:2},writers:["w"],purpose}
 ],
 retirements:[]
});
const ledger=readCacheLedgerV1(document(),partitions);
let workspace="";
const now=Date.now();
const base=():string=>join(workspace,".cache/b/a/m/nx/tmp");
function tree(name:string,size:number,ageMs:number):void{const directory=join(base(),name);mkdirSync(directory,{recursive:true});const path=join(directory,"payload.bin");writeFileSync(path,"x".repeat(size));const time=(now-ageMs)/1000;utimesSync(path,time,time);}
const reset=():void=>{rmSync(workspace,{recursive:true,force:true});mkdirSync(base(),{recursive:true});};
const output=()=>({write:(_line:string)=>{},language:"en" as const});
const portsFor=(alive:(pid:number)=>boolean=()=>false)=>cacheRunnerPortsV1({signal:new AbortController().signal,now:()=>now,progress:async()=>{},exclusive:async()=>({release:()=>{}}),alive});
function generator(seed:number):()=>number{let state=seed>>>0;return()=>{state=(state+0x6d2b79f5)>>>0;let value=state;value=Math.imul(value^value>>>15,value|1);value^=value+Math.imul(value^value>>>7,value|61);return((value^value>>>14)>>>0)/4294967296;};}

beforeAll(()=>{workspace=mkdtempSync(join(tmpdir(),"cgc-units-"));});
afterAll(()=>{rmSync(workspace,{recursive:true,force:true});});

test("the declared ledger shares nx/tmp between a pid-collected area and the compile cache, and refuses every forged variant",()=>{
 const declared=cacheLedgerV1();
 for(const id of ["nx-tooling","nx-tmp"])expect(declared.areas.some(area=>area.id===id),id).toBe(true);
 const tmp=declared.areas.find(area=>area.id==="nx-tmp")!,cache=declared.areas.find(area=>area.id==="compile-cache-rust")!;
 expect(tmp.collect).toMatchObject({custody:"pid"});expect(tmp.directory).toBe("nx/tmp");expect(cache.directory).toBe("nx/tmp/node-compile-cache");expect(new RegExp(tmp.collect!.units!,"u").test("node-compile-cache")).toBe(false);
 expect(declared.areas.find(area=>area.id==="nx-tooling")).toMatchObject({kind:"nested",partition:"tools",managed:{owner:"nx-tooling"}});
 const refused:Array<[string,(value:any)=>void,string]>=[
  ["pid custody without a pattern",value=>{delete value.areas[1].collect.units;},"pid custody needs"],
  ["pid custody with a pattern without the group",value=>{value.areas[1].collect.units="^tmp-[0-9]+-x$";},"pid custody needs"],
  ["a pid group without pid custody",value=>{value.areas[1].collect.custody="quiet";},"pid custody needs"],
  ["a pattern on two levels",value=>{value.areas[1].collect.depth=2;},"unit patterns need a depth-1"],
  ["a pattern on a tree",value=>{value.areas.push({id:"tre",root:"repository",kind:"tree",directory:"tre",budget:{bytes:1},collect:{unusedAgeMs:1,quietMs:0,custody:"quiet",units:"^a$"},writers:["w"],purpose});},"unit patterns need a depth-1"],
  ["an invalid pattern",value=>{value.areas[1].collect.units="^tmp-(?<pid>[0-9]+";},"valid unit pattern"],
  ["a sibling the pattern matches",value=>{value.areas[2].directory="nx/tmp/tmp-5-abc";},"overlapping nested areas"],
  ["a parent without a pattern",value=>{delete value.areas[1].collect.units;value.areas[1].collect.custody="quiet";},"overlapping nested areas"],
  ["a sibling below an area without a pattern",value=>{value.areas[1].collect={unusedAgeMs:HOUR,quietMs:0,custody:"quiet"};},"overlapping nested areas"],
  ["the same directory twice",value=>{value.areas[2].directory="nx/tmp";},"overlapping nested areas"]
 ];
 for(const[name,mutate,message]of refused){const value=document();mutate(value);expect(()=>readCacheLedgerV1(value,partitions),name).toThrow(message);}
 const quietPattern=document();quietPattern.areas[1].collect.custody="quiet";quietPattern.areas[1].collect.units="^tmp-[0-9]+-[a-z]+$";expect(()=>readCacheLedgerV1(quietPattern,partitions)).not.toThrow();
});

test("a pid holder is a live process named in the unit, and nothing else",()=>{
 const pattern=new RegExp(units,"u"),alive=(pid:number)=>pid===4242;
 expect(cachePidHolderV1("tmp-4242-abc",pattern,{alive})).toBe("pid:4242");
 expect(cachePidHolderV1("tmp-4243-abc",pattern,{alive})).toBeUndefined();
 expect(cachePidHolderV1("node-compile-cache",pattern,{alive})).toBeUndefined();
 expect(cachePidHolderV1("tmp-0-abc",pattern,{alive:()=>true})).toBeUndefined();
 expect(cachePidHolderV1("tmp-99999999999999999999-abc",pattern,{alive:()=>true})).toBeUndefined();
 expect(cachePidHolderV1("tmp-4242-abc",undefined,{alive})).toBeUndefined();
});

test("collection equals the independent Python oracle on 150 seeded directories and never touches a stranger",async()=>{
 const random=generator(20261026),cases:any[]=[],observed:string[][]=[];
 const scenarios=Array.from({length:150},(_,index)=>{
  const count=1+Math.floor(random()*12),livePids=new Set<number>(),entries=Array.from({length:count},(_,row)=>{
   const pid=100+Math.floor(random()*40),alive=random()<0.3;if(alive)livePids.add(pid);
   return {name:`tmp-${pid}-${index}x${row}`,bytes:1+Math.floor(random()*60000),ageMs:[MINUTE,5*MINUTE,30*MINUTE,2*HOUR,5*HOUR][Math.floor(random()*5)]!,pid};
  });
  return {entries,livePids};
 });
 for(const[index,scenario]of scenarios.entries()){
  reset();
  for(const entry of scenario.entries)tree(entry.name,entry.bytes,entry.ageMs);
  tree("semio-farm-session-abc",9000,5*DAY);mkdirSync(join(base(),"node-compile-cache","h"),{recursive:true});writeFileSync(join(base(),"node-compile-cache","h","c.bin"),"c".repeat(9000));
  const sizes=new Map(await Promise.all(scenario.entries.map(async entry=>[entry.name,(await measureCacheTreeV1(join(base(),entry.name),portsFor(),1000)).bytes] as const))),recency=new Map(await Promise.all(scenario.entries.map(async entry=>[entry.name,(await measureCacheTreeV1(join(base(),entry.name),portsFor(),1000)).recencyMs] as const)));
  cases.push({now,units,unusedAgeMs:HOUR,quietMs:10*MINUTE,budgetBytes:100000,entries:scenario.entries.map(entry=>({name:entry.name,bytes:sizes.get(entry.name)!,recencyMs:recency.get(entry.name)!,alive:scenario.livePids.has(entry.pid)}))});
  const before=readdirSync(base()).sort(),result=await collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["tmps"])},portsFor(pid=>scenario.livePids.has(pid)),output(),partitions),after=readdirSync(base()).sort();
  observed.push(before.filter(name=>!after.includes(name)));
  expect(after).toContain("semio-farm-session-abc");expect(after).toContain("node-compile-cache");
  expect(result.failed).toEqual([]);
  const again=await collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["tmps"])},portsFor(pid=>scenario.livePids.has(pid)),output(),partitions);
  expect(again.removed.filter(row=>row.reason==="age")).toEqual([]);
  for(const kept of result.plans[0]!.kept)if(kept.reason==="held")expect(scenario.livePids.has(Number(kept.name.split("-")[1]))).toBe(true);
 }
 reset();const file=join(workspace,"cases.json");writeFileSync(file,JSON.stringify({cases}));
 const run=runToolSyncV1("python",["-I",oracle,file],{encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);
 const expected=JSON.parse(run.stdout) as string[][];
 observed.forEach((names,index)=>expect(names.sort(),"scenario "+index).toEqual(expected[index]));
 expect(expected.flat().length).toBeGreaterThan(40);expect(expected.some(names=>names.length===0)).toBe(true);
});

test("a tree whose process is really alive survives until that process exits, a dry run removes nothing",async()=>{
 reset();
 const child=startToolV1("bun",["-e","setTimeout(()=>{},60000)"],{stdio:"ignore"});
 try{
  const pid=child.pid!,live=`tmp-${pid}-live`,dead=`tmp-999999-gone`;
  tree(live,20000,5*HOUR);tree(dead,20000,5*HOUR);
  const real=cacheRunnerPortsV1({signal:new AbortController().signal,now:()=>now,progress:async()=>{},exclusive:async()=>({release:()=>{}})});
  const plan=await collectCacheV1(workspace,ledger,{dryRun:true,only:new Set(["tmps"])},real,output(),partitions);
  expect(plan.removed.map(row=>row.name)).toEqual([dead]);expect(readdirSync(base()).sort()).toEqual([dead,live].sort());
  const first=await collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["tmps"])},real,output(),partitions);
  expect(first.removed.map(row=>row.name)).toEqual([dead]);expect(readdirSync(base())).toEqual([live]);
  const exited=new Promise<void>(done=>child.on("exit",()=>done()));child.kill();await exited;
  const second=await collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["tmps"])},real,output(),partitions);
  expect(second.removed.map(row=>row.name)).toEqual([live]);expect(readdirSync(base())).toEqual([]);
 }finally{try{child.kill();}catch{}}
},60000);

test("status counts only the unit directories of the area, not the compile cache beside them",async()=>{
 reset();
 tree("tmp-77-aaa",30000,2*HOUR);tree("tmp-78-bbb",10000,2*HOUR);mkdirSync(join(base(),"node-compile-cache"),{recursive:true});writeFileSync(join(base(),"node-compile-cache","big.bin"),"y".repeat(500000));tree("stranger",40000,HOUR);
 const report=await reportCacheLedgerV1(workspace,ledger,portsFor(),partitions,new Set(["tmps"]),{deep:true}),row=report.areas.find(area=>area.id==="tmps")!;
 const expected=(await measureCacheTreeV1(join(base(),"tmp-77-aaa"),portsFor(),1000)).bytes+(await measureCacheTreeV1(join(base(),"tmp-78-bbb"),portsFor(),1000)).bytes;
 expect(row.usedBytes).toBe(expected);expect(statSync(join(base(),"node-compile-cache","big.bin")).size).toBe(500000);
});

test("a managed area's delegate runs after the machine plan, gets it and whether its area was named, and its note keeps its place",async()=>{
 reset();
 const value=document();value.areas.push({id:"owned",root:"native",kind:"nested",directory:"owned",partition:"alpha",budget:{bytes:1<<20},managed:{owner:"nx-tooling"},writers:["w"],purpose});
 const owned=readCacheLedgerV1(value,partitions),calls:Array<{owner:string;id:string;dryRun:boolean;context:any}>=[];
 const run=async(only?:ReadonlySet<string>)=>{
  const lines=output(),out:string[]=[];
  const result=await collectCacheV1(workspace,owned,{dryRun:true,...only?{only}:{},machine:{totalBytes:40*1024**3,source:"default"}},{...portsFor(),delegate:async(owner,id,dryRun,_language,context)=>{calls.push({owner,id,dryRun,context});return "delegated "+id;}},{write:line=>{out.push(line);},language:lines.language},partitions);
  return {result,out};
 };
 const all=await run();
 expect(calls.length).toBe(1);expect(calls[0]).toMatchObject({owner:"nx-tooling",id:"owned",dryRun:true});expect(calls[0]!.context.selected).toBe(false);expect(calls[0]!.context.machine.stores.length).toBeGreaterThan(0);expect(calls[0]!.context.machine.totalBytes).toBe(40*1024**3);
 expect(all.result.delegated).toEqual([{owner:"nx-tooling",text:"delegated owned"}]);
 const noteAt=all.out.findIndex(line=>line.includes("nx-tooling reports: delegated owned")),startAt=all.out.findIndex(line=>line.startsWith("Collecting")),machineAt=all.out.findIndex(line=>line.startsWith("Machine budget"));
 expect(startAt).toBeGreaterThanOrEqual(0);expect(noteAt).toBeGreaterThan(startAt);expect(machineAt).toBeGreaterThan(noteAt);
 const named=await run(new Set(["owned"]));expect(calls.length).toBe(2);expect(calls[1]!.context.selected).toBe(true);expect(named.result.delegated.length).toBe(1);
 const plain=await collectCacheV1(workspace,owned,{dryRun:true},portsFor(),output(),partitions);expect(plain.delegated).toEqual([]);expect(plain.machine).toBeNull();
});
