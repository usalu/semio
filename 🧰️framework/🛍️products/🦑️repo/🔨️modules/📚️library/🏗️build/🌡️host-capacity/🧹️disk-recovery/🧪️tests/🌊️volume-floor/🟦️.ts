/** 🧪️ Contract: the volume floor frees the derivable classes no cache store accounts for (the compiled Wasmtime cache and the staging directories of the shared store), oldest first, before any content, store or image collector; their limits are the ones the plugin host applies at engine start, and the TypeScript selector judges the plugin host's own fixture. */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import engineFixture from "../../../../../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔧️engine/🧹️cache-retirement/🧪️tests/🧫️fixtures/🔣️.json" with {type:"json"};
import engineConfiguration from "../../../../../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔧️engine/🧹️cache-retirement/⚙️configuration/🔣️.json" with {type:"json"};
import {ownProcessIdentityV1} from "../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";
import {DISK_WRITERS_V1,relieveVolumeFloorV1,requireDiskWritersV1,type DiskFactsV1,type DiskWriterRowV1,type DiskWriterV1} from "../../🟦️.ts";
import {DISK_BOUNDS_V1} from "../../🧺️bounds/🟦️.ts";
import {retireCompiledCacheV1,selectCompiledCacheV1} from "../../🧺️bounds/📦️compiled-cache/🟦️.ts";
import {realDiskWritersV1,relieveMachineVolumeV1} from "../../🔌️real/🟦️.ts";

const roots:string[]=[],hour=3_600_000,day=24*hour,fresh=():string=>{const root=mkdtempSync(join(tmpdir(),"volume-floor-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const aged=(path:string,bytes:number,idleMs:number,now:number):void=>{mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,Buffer.alloc(bytes,7));utimesSync(path,(now-idleMs)/1000,(now-idleMs)/1000);};

test("the floor rows are derivable host classes that come before every content, store and image collector, and the registry stays closed",()=>{
 requireDiskWritersV1();
 const floor=DISK_WRITERS_V1.filter(row=>row.floor===true),ids=floor.map(row=>row.id);
 expect(ids).toEqual(expect.arrayContaining(["wasmtime-compiled-cache","fleet-store-staging","leaked-staging"]));
 for(const row of floor){expect(row.class).toBe("derivable");expect(row.relief).toBe("host");}
 const firstContent=Math.min(...DISK_WRITERS_V1.filter(row=>row.class!=="derivable").map(row=>row.priority));
 for(const row of floor)expect(row.priority).toBeLessThan(firstContent);
});

test("the limits of the disk bound are the ones the plugin host declares for its engine start",()=>{
 const bound=DISK_BOUNDS_V1.compiledCache;
 expect(bound.maximumBytes).toBe(engineConfiguration.maximumBytes);
 expect(bound.maximumAgeMilliseconds).toBe(engineConfiguration.maximumAgeSeconds*1000);
 expect(bound.minimumIdleMilliseconds).toBe(engineConfiguration.minimumIdleSeconds*1000);
 expect(bound.scratchGraceMilliseconds).toBe(engineConfiguration.scratchGraceSeconds*1000);
 expect(bound.maximumBytes).toBe(8*1024**3);expect(bound.maximumAgeMilliseconds).toBe(14*day);
});

test("the TypeScript selector retires exactly what the plugin host's shared fixture declares",()=>{
 const bound={maximumBytes:engineFixture.policy.maximumBytes,maximumAgeMilliseconds:engineFixture.policy.maximumAgeSeconds*1000,minimumIdleMilliseconds:engineFixture.policy.minimumIdleSeconds*1000,scratchGraceMilliseconds:engineFixture.policy.scratchGraceSeconds*1000},now=1_000_000_000;
 for(const row of engineFixture.cases){
  const entries=(row.entries as {name:string;bytes:number;idleSeconds:number;scratch?:boolean}[]).map(entry=>({path:entry.name,bytes:entry.bytes,usedMs:now-entry.idleSeconds*1000,scratch:entry.scratch===true}));
  expect(selectCompiledCacheV1(entries,now,bound),row.name).toEqual([...row.retired].sort());
 }
});

test("the real compiled-cache writer retires the expired and the least recently used over the cap, keeps in-use files, other files and a missing root, and a dry run removes nothing",async()=>{
 const home=fresh(),now=Date.now(),cache=join(home,".semio","cache","wasmtime"),bound={maximumBytes:250,maximumAgeMilliseconds:10*day,minimumIdleMilliseconds:60_000,scratchGraceMilliseconds:hour};
 aged(join(cache,"aa","expired.cwasm"),10,11*day,now);aged(join(cache,"aa","oldest.cwasm"),100,5*day,now);aged(join(cache,"bb","newer.cwasm"),100,day,now);aged(join(cache,"bb","open.cwasm"),100,1000,now);
 aged(join(cache,"bb","w.cwasm.1.scratch"),500,2*hour,now);aged(join(cache,"bb","live.cwasm.2.scratch"),500,1000,now);aged(join(cache,"cold-compile.lock"),0,40*day,now);
 expect(await retireCompiledCacheV1(cache,{now,dryRun:true,limit:1000,bound})).toEqual({files:3,bytes:610});
 expect(existsSync(join(cache,"aa","expired.cwasm"))).toBe(true);
 expect(await retireCompiledCacheV1(cache,{now,limit:1000,bound})).toEqual({files:3,bytes:610});
 for(const gone of ["aa/expired.cwasm","aa/oldest.cwasm","bb/w.cwasm.1.scratch"])expect(existsSync(join(cache,gone))).toBe(false);
 for(const kept of ["bb/newer.cwasm","bb/open.cwasm","bb/live.cwasm.2.scratch","cold-compile.lock"])expect(existsSync(join(cache,kept))).toBe(true);
 expect(await retireCompiledCacheV1(join(home,"absent"),{now,limit:10})).toEqual({files:0,bytes:0});expect(existsSync(join(home,"absent"))).toBe(false);
 const writers=realDiskWritersV1({workspace:home,home,tmp:home,uid:501,environment:{},now:()=>now,dryRun:false}),writer=writers.find(item=>item.row.id==="wasmtime-compiled-cache")!;
 expect(writer.root).toBe(cache);expect(await writer.measure()).toBeGreaterThan(0);
});

test("the real staging writer retires the dead and unowned staging directories of the shared store and never a live owner's, a young one or a loose file in use",async()=>{
 const workspace=fresh(),store=fresh(),now=Date.now(),tmp=join(store,"v1","tmp"),mine=ownProcessIdentityV1();
 mkdirSync(join(workspace,".cache","f"),{recursive:true});mkdirSync(tmp,{recursive:true});
 writeFileSync(join(workspace,".cache","f","machine.json"),JSON.stringify({originId:"g1-m01",groupId:"g1",storeRoot:store}));
 const make=(name:string,idleMs:number)=>{aged(join(tmp,name,"units.bin"),4096,idleMs,now);utimesSync(join(tmp,name),(now-idleMs)/1000,(now-idleMs)/1000);};
 const live=`g1-m01.cold-units.${mine.pid}.${mine.startMs}.0123456789abcdef`,dead=`g1-m01.cold-units.999999001.1700000000000.0123456789abcdef`,legacy="procedural3d-cold-units-am00uln3",young="draw-guest-fcc8.21L2Yt";
 make(live,40*hour);make(dead,2*hour);make(legacy,40*hour);make(young,hour/4);
 const writers=realDiskWritersV1({workspace,home:workspace,tmp:workspace,uid:501,environment:{},now:()=>now}),writer=writers.find(item=>item.row.id==="fleet-store-staging")!;
 expect(writer.root).toBe(tmp);expect(await writer.measure()).toBeGreaterThan(0);
 expect(await realDiskWritersV1({workspace,home:workspace,tmp:workspace,uid:501,environment:{},now:()=>now,dryRun:true}).find(item=>item.row.id==="fleet-store-staging")!.collect!({needBytes:0})).toBeGreaterThan(0);
 expect(existsSync(join(tmp,dead))).toBe(true);
 expect(await writer.collect!({needBytes:0})).toBeGreaterThan(0);
 expect(existsSync(join(tmp,dead))).toBe(false);expect(existsSync(join(tmp,legacy))).toBe(false);
 expect(existsSync(join(tmp,live))).toBe(true);expect(existsSync(join(tmp,young))).toBe(true);
 const bare=fresh();
 expect(await realDiskWritersV1({workspace:bare,home:bare,tmp:bare,uid:501,environment:{}}).find(item=>item.row.id==="fleet-store-staging")!.collect!({needBytes:0})).toBe(0);
});

class Volume{free:number;constructor(readonly total:number,free:number){this.free=free;}facts=():DiskFactsV1=>({partition:"/",totalBytes:this.total,freeBytes:this.free});}
const stub=(volume:Volume,id:string,priority:number,holds:number,log:string[],patch:Partial<DiskWriterRowV1>={}):DiskWriterV1=>{
 let left=holds;
 const row:DiskWriterRowV1={id,priority,class:"derivable",relief:"host",root:"/"+id,collector:"age-prune",command:"repo cache gc",bound:{en:"b",de:"b"},...patch};
 return{row,root:row.root,measure:()=>left,collect:async()=>{log.push(id);const freed=left;left=0;volume.free+=freed;return freed;}};
};

test("the floor runs only the floor rows, by priority, and stops once the target is free; a volume already above it is left alone",async()=>{
 const volume=new Volume(1000,100),log:string[]=[],writers=[stub(volume,"cas",70,500,log,{class:"content"}),stub(volume,"b",64,300,log,{floor:true}),stub(volume,"a",63,200,log,{floor:true}),stub(volume,"c",65,900,log,{floor:true}),stub(volume,"image",66,900,log,{relief:"image",floor:true})];
 const outcome=await relieveVolumeFloorV1({facts:volume.facts,targetFreeBytes:550,writers,language:"en"});
 expect(log).toEqual(["a","b"]);
 expect(outcome.reached).toBe(true);expect(outcome.freedBytes).toBe(500);
 const second=await relieveVolumeFloorV1({facts:volume.facts,targetFreeBytes:2000,writers,language:"en"});
 expect(log).toEqual(["a","b","a","b","c"]);expect(second.reached).toBe(false);expect(second.freedBytes).toBe(900);
 const quiet=await relieveVolumeFloorV1({facts:volume.facts,targetFreeBytes:10,writers,language:"en"});
 expect(quiet).toEqual({reached:true,freedBytes:0,steps:[]});expect(log).not.toContain("cas");
 expect(await relieveVolumeFloorV1({facts:()=>undefined,targetFreeBytes:10,writers,language:"en"})).toEqual({reached:false,freedBytes:0,steps:[]});
});

test("a failing floor collector never stops the next one",async()=>{
 const volume=new Volume(1000,100),log:string[]=[],broken=stub(volume,"a",63,0,log,{floor:true});
 const writers=[{...broken,collect:async()=>{log.push("a");throw Error("boom");}},stub(volume,"b",64,400,log,{floor:true})];
 const outcome=await relieveVolumeFloorV1({facts:volume.facts,targetFreeBytes:400,writers,language:"en"});
 expect(log).toEqual(["a","b"]);expect(outcome.reached).toBe(true);expect(outcome.steps[0]?.failure).toContain("boom");
});

test("the machine relief reads the volume of the store root and runs the floor collectors until the target; an unreachable target runs them all and still ends",async()=>{
 const home=fresh(),now=Date.now(),cache=join(home,".semio","cache","wasmtime"),lines:string[]=[];
 aged(join(cache,"aa","expired.cwasm"),10,30*day,now);aged(join(cache,"aa","fresh.cwasm"),10,1000,now);
 await relieveMachineVolumeV1({workspace:home,home,tmp:home,uid:501,environment:{},now:()=>now,report:line=>lines.push(line)},{root:home,targetFreeBytes:Number.MAX_SAFE_INTEGER});
 expect(existsSync(join(cache,"aa","expired.cwasm"))).toBe(false);expect(existsSync(join(cache,"aa","fresh.cwasm"))).toBe(true);
 expect(lines.some(line=>line.includes("wasmtime-compiled-cache"))).toBe(true);
 aged(join(cache,"aa","expired-again.cwasm"),10,30*day,now);
 await relieveMachineVolumeV1({workspace:home,home,tmp:home,uid:501,environment:{},now:()=>now},{root:home,targetFreeBytes:1});
 expect(existsSync(join(cache,"aa","expired-again.cwasm"))).toBe(true);
});
