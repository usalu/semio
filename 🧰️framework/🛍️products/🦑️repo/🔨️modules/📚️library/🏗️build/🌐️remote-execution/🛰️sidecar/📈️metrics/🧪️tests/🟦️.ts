import{afterAll,beforeAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{readSeriesV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/📈️metrics/🟦️.ts";
import{createFleetMetricsCollectorV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/📈️metrics/🛰️collector/🟦️.ts";
import{countUnitV1}from"../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📊️counters/🟦️.ts";
import{countEventV1,openCounterLedgerV1}from"../../../../📈️counter-ledger/🟦️.ts";
import{casCounterDirectoryV1}from"../../../../🗄️cas/🗃️store/🟦️.ts";
import{bumpOffloadCountersV1}from"../../../🚀️offload/📊️counters/🟦️.ts";
import{machinePathV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import{NX_CACHE_POLICY_V1}from"../../../../../⚡️caching/🚀️bootstrap/📊️nx-cache/🟦️.ts";
import{cacheStatusMetricsSourcesV1,fleetMetricsSourcesV1,metricsScratchPathV1,schedulerMetricsSourcesV1,startFleetMetricsV1,startFleetScopeMetricsV1}from"../🟦️.ts";
import{readRingFileV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/📈️metrics/🟦️.ts";

/** 📈️ Contract: the product sources read the unit, offload, content store and remote cache counters and the origin files of the fleet store as the collectors expect them, and answer null for what is missing or stale.
 * @see ../🟦️.ts */
const NOW=1_800_000_000_000,roots:string[]=[],saved=process.env.SEMIO_MACHINE_STATE_DIRECTORY;
const scratch=(name:string):string=>{const path=mkdtempSync(join(tmpdir(),`fleet-metrics-${name}-`));roots.push(path);return path;};
beforeAll(()=>{process.env.SEMIO_MACHINE_STATE_DIRECTORY=scratch("state");});
afterAll(()=>{if(saved===undefined)delete process.env.SEMIO_MACHINE_STATE_DIRECTORY;else process.env.SEMIO_MACHINE_STATE_DIRECTORY=saved;for(const path of roots)rmSync(path,{recursive:true,force:true});});
const put=(path:string,value:unknown):void=>{mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,JSON.stringify(value));};

function fixture(origin:string,publisher:string=origin){
 const root=scratch("root"),store=scratch("store"),cas=scratch("cas"),host=scratch("host"),server=scratch("server"),own=join(store,"v1","origins",origin);
 for(let index=0;index<3;index++)countUnitV1(root,"miss");
 for(let index=0;index<2;index++)countUnitV1(root,"localHit");
 countUnitV1(root,"remoteHit");countUnitV1(root,"stored");
 bumpOffloadCountersV1(root,{out:2,refused:1,fellBack:1},"no-idle-machine");
 for(let index=0;index<4;index++)countEventV1(casCounterDirectoryV1(cas),"localHit");countEventV1(casCounterDirectoryV1(cas),"localMiss");
 const ledger=openCounterLedgerV1({path:join(server,"v1","counters.json"),name:"remote-cache",now:()=>NOW});ledger.bind(()=>({ac:{hits:5,misses:2,puts:1,conflicts:0},cas:{hits:9},nx:{misses:3}}));ledger.close();
 put(join(store,"v1","fleet","cache.json"),{publisher:{origin:publisher,directory:server}});
 put(join(own,"pool.json"),{updatedMs:NOW-1000,units:752,claimed:44,published:7,failed:2,rssMegabytes:900});
 put(join(own,"slice-memory.json"),{atMs:NOW-2000,phase:{name:"x",done:300,total:600},sources:{caches:{rowStore:{rows:72628}}}});
 put(join(cas,"v1","census.json"),{version:1,atMs:NOW,objects:3,bytes:12345});
 put(join(own,"ready.json"),{state:"running",stageIndex:3,attempts:2,updatedMs:NOW});
 put(join(own,"head.json"),{events:99,bytes:4096});
 return {root,store,environment:{SEMIO_FLEET_STORE_ROOT:store,SEMIO_CAS_ROOT:cas},host,own};
}

test("the product sources read every counter file and every origin file of the store",()=>{
 const f=fixture("g1-t"),sources=fleetMetricsSourcesV1({root:f.root,environment:f.environment,origin:"g1-t",now:()=>NOW,directory:f.host});
 expect(sources.unitCounters!()).toEqual({localHit:2,remoteHit:1,miss:3,stored:1,skipped:0});

 expect(sources.cacheCounters!()).toEqual({localHit:4,localMiss:1,bytes:12345});
 countEventV1(machinePathV1(f.root,".cache",...NX_CACHE_POLICY_V1.directory),"nxLocalHit");countEventV1(machinePathV1(f.root,".cache",...NX_CACHE_POLICY_V1.directory),"nxExecuted");countEventV1(machinePathV1(f.root,".cache",...NX_CACHE_POLICY_V1.directory),"nxExecuted");
 expect(sources.cacheCounters!()).toEqual({localHit:5,localMiss:3,bytes:12345});
 expect(sources.offload!()).toEqual({out:2,refused:1,fellBack:1});
 expect(sources.facts!()).toBe(72628);
 expect(sources.remoteCache!()!.groups).toMatchObject({ac:{hits:5,misses:2,puts:1,conflicts:0},cas:{hits:9},nx:{misses:3}});
 expect(sources.pool!()).toEqual({units:752,claimed:44,published:7,failed:2,rssMegabytes:900});
 expect(sources.phase!()).toEqual({done:300,total:600});
 expect(sources.ready!()).toEqual({stageIndex:3,state:"running",attempts:2});
 expect(sources.journal!()).toEqual({events:99,bytes:4096});
});

test("a remote cache of another machine, stale pool and slice files and a missing store are gaps, never zeros; a store that has counted nothing yet has zero local hits and misses",()=>{
 const f=fixture("g1-t","g1-server"),stale=fleetMetricsSourcesV1({root:f.root,environment:f.environment,origin:"g1-t",now:()=>NOW+10*60_000,directory:f.host});
 expect(stale.remoteCache!()).toBeNull();expect(stale.pool!()).toBeNull();expect(stale.phase!()).toBeNull();expect(stale.facts!()).toBeNull();

 process.env.SEMIO_MACHINE_STATE_DIRECTORY=scratch("state2");
 const none=fleetMetricsSourcesV1({root:scratch("empty"),environment:{SEMIO_CAS_ROOT:scratch("emptycas")},origin:"g1-t",now:()=>NOW,directory:f.host});
 expect(none.pool!()).toBeNull();expect(none.ready!()).toBeNull();expect(none.journal!()).toBeNull();expect(none.remoteCache!()).toBeNull();expect(none.cacheCounters!(),"no counter file yet means no event yet: zero hits and misses, so the first event of the machine is counted").toEqual({localHit:0,localMiss:0});expect(none.facts!()).toBeNull();
});

test("a collector fed by the product sources writes the deltas of the counters into the rings of the machine",()=>{
 const f=fixture("g1-t"),directory=scratch("rings"),sources=fleetMetricsSourcesV1({root:f.root,environment:f.environment,origin:"g1-t",now:()=>NOW,directory:f.host});
 const collector=createFleetMetricsCollectorV1({directory,sources});
 collector.step(NOW);
 for(let index=0;index<4;index++)countUnitV1(f.root,"miss");countUnitV1(f.root,"localHit");
 const step=collector.step(NOW+10_000);collector.close();
 const series=readSeriesV1({directory,fromMs:NOW-1000,toMs:NOW+20_000,ids:["build.units.compiled","build.units.restored.local","graph.read.owners","ready.state","replica.journal.bytes","exec.offload.out"]}).metrics;
 expect(step.dropped+step.unknown).toBe(0);
 expect(series["build.units.compiled"]!.map(point=>point[6])).toEqual([4]);
 expect(series["build.units.restored.local"]!.map(point=>point[6])).toEqual([1]);
 expect(series["exec.offload.out"]!.map(point=>point[6])).toEqual([0]);
 expect(series["graph.read.owners"]!.map(point=>point[7])).toEqual([600,600]);
 expect(series["ready.state"]!.map(point=>point[7])).toEqual([2,2]);
 expect(series["replica.journal.bytes"]!.map(point=>point[7])).toEqual([4096,4096]);
});

test("the scheduler source counts the executions it holds and the units offloaded in over every machine from its counter ledger, the cache source the entries and bytes of its store",()=>{
 const root=scratch("sched"),environment={},file=join(machinePathV1(root,".cache","f","sidecars"),"build-scheduler.counters.json");
 let counts:{running:number;queued:number}|null={running:2,queued:5};
 const sources=schedulerMetricsSourcesV1({root,environment,counts:()=>counts,now:()=>NOW});
 expect(sources.remoteExecution!()).toEqual({running:2,queued:5,offloadedIn:0});
 const ledger=openCounterLedgerV1({path:file,name:"remote-execution",now:()=>NOW});ledger.bind(()=>({"offload.g1-a":{offloadedIn:6,offloadedOut:1},"offload.g1-b":{offloadedIn:4,offloadedOut:9},scheduler:{completed:50}}));ledger.close();
 expect(sources.remoteExecution!()).toEqual({running:2,queued:5,offloadedIn:10});
 counts=null;expect(sources.remoteExecution!()).toBeNull();
 expect(cacheStatusMetricsSourcesV1({usage:()=>({entries:7,bytes:70})}).remoteCacheStatus!()).toEqual({entries:7,bytes:70});
 expect(cacheStatusMetricsSourcesV1({usage:()=>null}).remoteCacheStatus!()).toBeNull();
});

test("a fleet-scope service starts its collector, the first sample is written and the last window is published below fleet/metrics/<source> when it stops",()=>{
 const store=scratch("fleetstore"),host=scratch("fleethost"),abort=new AbortController(),lines:string[]=[];
 const environment={SEMIO_FLEET_STORE_ROOT:store,SEMIO_HOST_CAPACITY_DIRECTORY:host};
 const stop=startFleetScopeMetricsV1({environment,signal:abort.signal,write:line=>lines.push(line)},{source:"remote-cache",sources:cacheStatusMetricsSourcesV1({usage:()=>({entries:3,bytes:30})})});
 stop();stop();
 const ring=readRingFileV1(join(store,"v1","fleet","metrics","remote-cache","t0.ring"));
 expect(ring?.ids.slice().sort()).toEqual(["cache.remote.bytes","cache.remote.entries"]);
 expect(ring!.rows.length).toBe(1);expect(lines).toEqual([]);
});

test("the agent starts the readiness child, which hosts the metrics collector, on every machine including the group server (which publishes its origin rings and the cache counters of its ledger)",()=>{
 const source=readFileSync(join(import.meta.dir,"..","..","👷️agent","🟦️.ts"),"utf8"),line=source.split("\n").find(row=>/^\s*const ready=/u.test(row));
 expect(line,"the agent has a ready child").toBeDefined();
 expect(line).toMatch(/superviseGraphChildV1\(context,[^;]*"ready"\);\s*$/u);
 expect(line,"no group-server exclusion").not.toContain("-server");
 expect(source).toContain("ready?.stop()");
});

test("a readiness service of a scratch member (inside the temporary directory) starts no collector, a worktree of a machine does",()=>{
 expect(metricsScratchPathV1(scratch("member"))).toBe(true);
 expect(metricsScratchPathV1(realpathSync(scratch("real")))).toBe(true);
 expect(metricsScratchPathV1("/Users/someone/semio")).toBe(false);expect(metricsScratchPathV1(tmpdir())).toBe(false);expect(metricsScratchPathV1(undefined)).toBe(false);expect(metricsScratchPathV1("")).toBe(false);
 const host=scratch("hostdir"),stop=startFleetMetricsV1({root:scratch("member2"),environment:{SEMIO_HOST_CAPACITY_DIRECTORY:host,SEMIO_FLEET_STORE_ROOT:scratch("store3")},signal:new AbortController().signal,write:()=>{}},"g1-m09");
 stop();expect(readdirSync(host),"nothing is written into the ring directory of the machine").toEqual([]);
});

test("the readiness service of the group server idles without spinning: awaited with nothing but unref'd timers it would use 45 % of a core under Bun 1.4.2",async()=>{
 const folder=scratch("idle"),script=join(folder,"idle.ts"),ready=join(import.meta.dir,"..","..","🚦️ready","🟦️.ts");
 writeFileSync(script,`import {runReady} from ${JSON.stringify(ready)};
const stop=new AbortController();process.on("SIGTERM",()=>stop.abort());
let before=process.cpuUsage();
setTimeout(()=>{before=process.cpuUsage();},1500).unref();
setTimeout(()=>{const used=process.cpuUsage(before);console.log("cpu "+(used.user+used.system)/1000);stop.abort();},5500).unref();
await runReady({root:${JSON.stringify(join(folder,"root"))},environment:{SEMIO_FLEET_ORIGIN:"g1-server",SEMIO_FLEET_GROUP:"g1",SEMIO_HOST_CAPACITY_DIRECTORY:${JSON.stringify(join(folder,"host"))},HOME:${JSON.stringify(folder)}},signal:stop.signal,write:()=>{}} as never);`);
 const child=Bun.spawn([process.execPath,script],{stdout:"pipe",stderr:"pipe",env:{...process.env}});
 const text=await new Response(child.stdout).text();await child.exited;
 const spent=Number(/cpu ([0-9.]+)/u.exec(text)?.[1]);
 expect(spent,text).toBeGreaterThanOrEqual(0);expect(spent,`${spent} ms of CPU in 4 s of idling`).toBeLessThan(500);
},60000);
