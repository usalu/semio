import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,statSync,statfsSync,writeFileSync} from "node:fs";
import {freemem,tmpdir,totalmem} from "node:os";
import {join} from "node:path";
import {METRICS_CATALOGUE_V1,METRICS_FILES_V1 as schemaFiles,METRICS_TIERS_V1,closeMetricsRingsV1,openMetricsRingsV1,readRingFileV1,readSeriesV1,recordSampleV1} from "../../🟦️.ts";
import {METRICS_COLLECTORS_V1,METRICS_UNSOURCED_V1,acquireMetricsLockV1,createFleetMetricsCollectorV1,createMetricsCollectorStateV1,metricsLockRefusalV1,releaseMetricsLockV1,runMetricsCollectorV1,type MetricsSourcesV1} from "../🟦️.ts";
import {createMetricsPublisherV1} from "../📤️publication/🟦️.ts";
import {hostMetricsSourcesV1,metricsVolumePathsV1} from "../🔌️host/🟦️.ts";
import Ajv from "ajv";
import {processIdentityOfV1} from "../../../../🔖️identity/🟦️.ts";
import {requireTestToolV1} from "../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import metricsSchema from "../../🧬️schema/🔣️.json" with {type:"json"};

/** 📡️ Contract: every metric of the catalogue has exactly one feeding collector, each collector turns injected probes into the documented numbers, counters become exact deltas across resets and restarts (checked against the independent Python oracle of the ring tests), one process writes a ring directory, and the publication into the store is atomic and bounded.
 * @see ../🟦️.ts */
const here=import.meta.dir,MS=metricsSchema.definitions,oracle=join(here,"..","..","🧪️tests","🐍️.py"),S=schema.definitions,scratchRoots:string[]=[];
const scratch=(name:string):string=>{const path=mkdtempSync(join(tmpdir(),`collector-${name}-`));scratchRoots.push(path);return path;};
afterAll(()=>{for(const path of scratchRoots)rmSync(path,{recursive:true,force:true});});
const python=(args:string[],input?:string):string=>{const run=spawnSync(requireTestToolV1("python"),[oracle,...args],{input,encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);return run.stdout;};
const pythonCode=(code:string,...args:string[]):string=>{const run=spawnSync(requireTestToolV1("python"),["-I","-c",code,...args],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return run.stdout;};
const BASE=1_800_000_000_000,MEGABYTE=1048576,ALL=METRICS_CATALOGUE_V1.map(metric=>metric.id);

const full:MetricsSourcesV1={
 now:()=>BASE,
 cpuTimes:()=>({busy:50,total:100}),cores:()=>8,load:()=>2.5,memory:()=>({totalBytes:16*1024*MEGABYTE,freeBytes:4096*MEGABYTE}),
 warden:()=>({atMs:BASE,totalMegabytes:16384,availableMegabytes:5000,swapUsedMegabytes:1000,swapOutMegabytesPerSecond:2.5,compressedMegabytes:3000,pressure:"warn",level:"guard",subjects:{count:10,megabytes:4000,cpuSeconds:10,byKind:{cargo:{count:3,megabytes:1800},"local-unit":{count:6,megabytes:2000},"remote-unit":{count:1,megabytes:100},"pool-child":{count:2,megabytes:70}}},headroomMegabytes:9000,cancelled:4}),
 gate:()=>({totalMegabytes:16384,availableMegabytes:4000,swapUsedMegabytes:null,swapOutMegabytesPerSecond:null,compressedMegabytes:null,pressure:"normal",openFiles:6000,attachedImages:19}),
 volumes:()=>[{freeBytes:900,totalBytes:1000},{freeBytes:40,totalBytes:500}],queueDepth:()=>3,
 unitCounters:()=>({localHit:11,remoteHit:12,miss:13,stored:14,skipped:15}),cacheCounters:()=>({localHit:21,localMiss:22,bytes:23}),
 remoteCache:()=>({groups:{ac:{hits:1,misses:2,puts:3,conflicts:4},cas:{hits:5,misses:6,puts:7},nx:{hits:8,misses:9,puts:10}}}),remoteCacheStatus:()=>({entries:11,bytes:12}),
 offload:()=>({out:31,refused:32,fellBack:33}),remoteExecution:()=>({running:2,queued:1,offloadedIn:34}),
 pool:()=>({units:752,claimed:44,published:7,failed:2,rssMegabytes:900}),phase:()=>({done:300,total:600}),facts:()=>1234,
 ready:()=>({stageIndex:3,state:"running",attempts:2}),journal:()=>({events:99,bytes:4096}),
 self:()=>({cpuMicroseconds:5000,rssBytes:30*MEGABYTE}),microseconds:()=>1_000_000,
};
const run=(name:string,sources:MetricsSourcesV1,state=createMetricsCollectorStateV1()):Record<string,number>=>runMetricsCollectorV1(name,sources,state);

test("law: every catalogue metric belongs to exactly one collector of its own source, and the unsourced list is pinned", () => {
 const seen=new Map<string,string>();
 for(const collector of METRICS_COLLECTORS_V1){
  expect(["tick","slow"]).toContain(collector.cadence);
  expect(Object.keys(MS.Scopes.const)).toContain(collector.scope);
  for(const id of collector.ids){
   expect(seen.has(id),`${id} twice`).toBe(false);seen.set(id,collector.name);
   const metric=METRICS_CATALOGUE_V1.find(row=>row.id===id)!;
   expect(metric.source,id).toBe(collector.source);
   expect((metric as unknown as {scope:string}).scope,`${id} is fed by the ${collector.scope} collector ${collector.name}`).toBe(collector.scope);
  }
 }
 expect([...seen.keys()].sort()).toEqual([...ALL].sort());
 expect(METRICS_UNSOURCED_V1,"every metric has a source").toEqual([]);
 for(const source of ["remote-execution","remote-cache"])expect(METRICS_COLLECTORS_V1.filter(collector=>collector.scope==="fleet"&&collector.source===source).length,`${source} has a fleet collector`).toBe(1);
 for(const language of ["en","de"] as const)expect(Object.keys(S.Texts.const[language]).sort()).toEqual(Object.keys(S.Texts.const.en).sort());
 for(const collector of S.Collectors.const)expect(Object.keys(collector.label).sort()).toEqual(["de","en"]);
});

test("law: fully populated sources feed every id, and each collector feeds only the ids it declares", () => {
 const state=createMetricsCollectorStateV1(),fed=new Set<string>();
 for(const second of [0,1])for(const collector of METRICS_COLLECTORS_V1){
  const values=run(collector.name,{...full,warden:()=>({...full.warden!()!,atMs:BASE+second*2000,subjects:{...full.warden!()!.subjects,cpuSeconds:10+second*3}}),self:()=>({cpuMicroseconds:5000+second*10000,rssBytes:30*MEGABYTE}),microseconds:()=>1_000_000+second*1_000_000,cpuTimes:()=>({busy:50+second*30,total:100+second*100})},state);
  for(const id of Object.keys(values)){expect(collector.ids,`${collector.name} feeds ${id}`).toContain(id);fed.add(id);expect(Number.isFinite(values[id]!)).toBe(true);}
 }
 expect([...fed].sort()).toEqual([...ALL].sort());
});

test("cpu: busy share of two processor time readings, clamped, absent on the first reading", () => {
 const state=createMetricsCollectorStateV1(),reads=[{busy:10,total:100},{busy:40,total:200},{busy:40,total:200},{busy:400,total:210}];
 let at=0;const sources:MetricsSourcesV1={cpuTimes:()=>reads[at++]!,cores:()=>4,load:()=>1.5};
 expect(run("cpu",sources,state)).toEqual({"host.cpu.load":1.5,"host.cpu.cores":4});
 expect(run("cpu",sources,state)["host.cpu.busy"]).toBeCloseTo(30,9);
 expect(run("cpu",sources,state)["host.cpu.busy"]).toBeUndefined();
 expect(run("cpu",sources,state)["host.cpu.busy"]).toBe(100);
});

test("memory: the warden state first, then the gate sample, then the operating system; state numbers and bytes", () => {
 expect(run("memory",full)).toEqual({"host.memory.total":16384*MEGABYTE,"host.memory.available":5000*MEGABYTE,"host.memory.compressed":3000*MEGABYTE,"host.memory.pressure":1,"host.swap.used":1000*MEGABYTE,"host.swap.rate":2.5*MEGABYTE});
 expect(run("memory",{...full,warden:()=>null})).toEqual({"host.memory.total":16384*MEGABYTE,"host.memory.available":4000*MEGABYTE,"host.memory.pressure":0});
 expect(run("memory",{...full,warden:()=>null,gate:()=>null})).toEqual({"host.memory.total":16*1024*MEGABYTE,"host.memory.available":4096*MEGABYTE});
 expect(run("memory",{})).toEqual({});
});

test("warden and process tree: level number, guarded processes, compilation tasks are the unit, cargo and stage kinds", () => {
 expect(run("warden",full)).toEqual({"warden.level":1,"warden.subjects.count":10,"warden.subjects.memory":4000*MEGABYTE,"warden.headroom":9000*MEGABYTE,"warden.cancelled":4});
 expect(run("process-tree",full)).toEqual({"process.repo.count":10,"process.repo.memory":4000*MEGABYTE,"build.tasks.running":10});
 const pools={...full.warden!()!,subjects:{count:2,megabytes:70,byKind:{"pool-child":{count:2,megabytes:70}}}};
 expect(run("process-tree",{warden:()=>pools})["build.tasks.running"]).toBe(0);
 expect(run("warden",{warden:()=>null})).toEqual({});
});

test("process tree: the repo CPU share is the rate of the warden's cumulative CPU seconds, a gap for the first reading, a restart, a long silence and a warden without the counter", () => {
 const state=createMetricsCollectorStateV1(),readings=[[0,100],[2000,103],[2000,103],[4000,104],[6000,1],[70000,5],[72000,6]] as const,rows=readings.map(([at,cpu])=>({...full.warden!()!,atMs:BASE+at,subjects:{...full.warden!()!.subjects,cpuSeconds:cpu}}));
 let index=0;const sources:MetricsSourcesV1={warden:()=>rows[index]!};
 const out:(number|undefined)[]=[];for(;index<rows.length;index++)out.push(run("process-tree",sources,state)["process.repo.cpu"]);
 const expected=JSON.parse(pythonCode(`import json,sys
r=json.loads(sys.argv[1]);o=[None];p=r[0]
for a,c in r[1:]:
    if a==p[0]:
        o.append(o[-1]);continue
    s=a-p[0]
    o.append(None if s>60000 or c<p[1] else 100.0*(c-p[1])*1000/s);p=[a,c]
print(json.dumps(o))`,JSON.stringify(readings))) as (number|null)[];
 expect(expected.map(value=>value===null?undefined:value)).toEqual(out.map(value=>value===undefined?undefined:value));
 expect(out[1]).toBeCloseTo(150,9);expect(out[3]).toBeCloseTo(50,9);expect(out[4]).toBeUndefined();expect(out[5]).toBeUndefined();
 expect(run("process-tree",{warden:()=>({...full.warden!()!,subjects:{count:1,megabytes:1,byKind:{}}})})["process.repo.cpu"]).toBeUndefined();
});

test("volumes: the least free volume with its size, open files and images from the gate sample; queue depth", () => {
 expect(run("volumes",full)).toEqual({"host.disk.free":40,"host.disk.total":500,"host.files.open":6000,"host.images.attached":19});
 expect(run("volumes",{volumes:()=>[]})).toEqual({});
 expect(run("queue",full)).toEqual({"build.tasks.queued":3});
});

test("counter files: unit, local cache, remote cache, offload, pool, graph phase, ready and journal map to their ids", () => {
 expect(run("unit-counters",full)).toEqual({"build.units.compiled":13,"build.units.restored.local":11,"build.units.restored.fleet":12,"build.units.stored":14,"build.units.skipped":15});
 expect(run("local-cache",full)).toEqual({"cache.local.hit":21,"cache.local.miss":22,"cache.local.bytes":23});
 expect(run("remote-cache",full)).toMatchObject({"cache.remote.ac.hit":1,"cache.remote.ac.miss":2,"cache.remote.ac.put":3,"cache.remote.ac.conflict":4,"cache.remote.cas.hit":5,"cache.remote.nx.put":10});
 expect(run("remote-cache-status",full)).toEqual({"cache.remote.entries":11,"cache.remote.bytes":12});
 expect(run("remote-cache",full)).not.toHaveProperty("cache.remote.entries");
 expect(run("remote-cache",{remoteCache:()=>({groups:{ac:{hits:1}}})})).toEqual({"cache.remote.ac.hit":1});
 expect(run("offload-counters",full)).toEqual({"exec.offload.out":31,"exec.offload.fellback":33,"exec.refused":32});
 expect(run("remote-execution",full)).toEqual({"exec.queue.running":2,"exec.queue.queued":1,"exec.offload.in":34});
 expect(run("owner-pool",full)).toEqual({"graph.read.owners":600,"graph.read.owners.done":300,"graph.pool.claimed":44,"graph.pool.published":7,"graph.pool.failed":2,"graph.pool.memory":900*MEGABYTE});
 expect(run("owner-pool",{...full,phase:()=>null})).toMatchObject({"graph.read.owners":752,"graph.read.owners.done":7});
 expect(run("graph-service",full)).toEqual({"graph.read.facts":1234});
 expect(run("ready",full)).toEqual({"ready.stage":3,"ready.state":2,"ready.attempts":2});
 expect(run("ready",{ready:()=>null})).toEqual({"ready.state":0});
 expect(run("ready",{ready:()=>({stageIndex:0,state:"backoff",attempts:5})})["ready.state"]).toBe(1);
 expect(run("journal",full)).toEqual({"replica.journal.events":99,"replica.journal.bytes":4096});
 for(const name of ["unit-counters","local-cache","remote-cache","remote-cache-status","offload-counters","remote-execution","owner-pool","graph-service","journal","warden","queue","volumes"])expect(run(name,{}),name).toEqual({});
});

test("self: the collector's CPU share is the CPU spent inside its own steps, the readiness process is reported separately, memory is what the collector retains", () => {
 const state=createMetricsCollectorStateV1(70);state.self.tickMicroseconds=420;state.self.writeMicroseconds=17;state.self.publishMilliseconds=3;state.self.writeBytes=5000;state.self.samples=2;state.self.dropped=1;
 let cpu=1000,clock=1_000_000;const sources:MetricsSourcesV1={self:()=>({cpuMicroseconds:cpu,rssBytes:25*MEGABYTE}),microseconds:()=>clock};
 const first=run("self",sources,state);expect(first["metrics.self.cpu"]).toBeUndefined();expect(first["process.ready.cpu"]).toBeUndefined();
 cpu=1000+5000;clock=1_000_000+1_000_000;state.self.stepCpuMicroseconds=2500;
 const second=run("self",sources,state);
 expect(second["metrics.self.cpu"]).toBeCloseTo(0.25,9);
 expect(second["process.ready.cpu"]).toBeCloseTo(0.5,9);
 expect(second["process.ready.rss"]).toBe(25*MEGABYTE);
 expect(second["metrics.self.memory"]).toBe(state.self.retainedBytes);expect(state.self.retainedBytes).toBeGreaterThan(10_000);expect(state.self.retainedBytes).toBeLessThan(MEGABYTE);
 expect(second).toMatchObject({"metrics.self.tick.duration":420,"metrics.self.write.duration":17,"metrics.self.publish.duration":3,"metrics.self.write.bytes":5000,"metrics.self.samples":2,"metrics.self.dropped":1});
 expect(state.self.stepCpuMicroseconds,"the accumulated step CPU starts again after a report").toBe(0);
 expect(state.rowBytes).toBe(2*8+70*36+8);
});

test("self: the loop measures the CPU time inside its steps (here 100 us per step in 5 ms of clock), not the process around it", () => {
 const directory=scratch("selfcpu");let used=0,clock=0;
 const collector=createFleetMetricsCollectorV1({directory,sources:{queueDepth:()=>1,cpuMicroseconds:()=>(used+=100),self:()=>({cpuMicroseconds:used*1000,rssBytes:40*MEGABYTE}),microseconds:()=>(clock+=1000)}});
 for(let index=0;index<8;index++)collector.step(BASE+index*1000);
 collector.close();
 const series=readSeriesV1({directory,fromMs:BASE-1000,toMs:BASE+9000,ids:["metrics.self.cpu","process.ready.cpu","process.ready.rss","metrics.self.memory"]});
 const own=series.metrics["metrics.self.cpu"]!;
 expect(own.length,"the first step has no span yet").toBe(7);
 for(const point of own)expect(point[6]).toBeCloseTo(2,6);
 for(const point of series.metrics["process.ready.cpu"]!)expect(point[6],"the whole process is far above the collector").toBeGreaterThan(100);
 expect(series.metrics["process.ready.rss"]!.every(point=>point[6]===40*MEGABYTE)).toBe(true);
 expect(series.metrics["metrics.self.memory"]!.every(point=>point[6]>10_000&&point[6]<MEGABYTE)).toBe(true);
});

test("opening: rings that cannot be opened are reported once with the reason in both languages and retried after the retry interval, never every second", () => {
 const base=scratch("blocked"),file=join(base,"file");writeFileSync(file,"x");
 const lines:string[]=[];let reads=0;
 const collector=createFleetMetricsCollectorV1({directory:join(file,"rings"),report:line=>lines.push(line),sources:{queueDepth:()=>{reads++;return 1;}}});
 for(const offset of [0,1000,2000,29_000])expect(collector.step(BASE+offset).owner).toBe(false);
 expect(lines.length).toBe(1);expect(lines[0]).toContain(" / ");
 expect(collector.step(BASE+31_000).owner).toBe(false);expect(lines.length,"the same reason is not repeated").toBe(1);
 expect(reads).toBe(0);collector.close();
});

const unit=(local:number,remote:number,miss:number)=>({localHit:local,remoteHit:remote,miss,stored:0,skipped:0});
const collectorWith=(directory:string,counts:()=>ReturnType<typeof unit>|null,extra:Partial<MetricsSourcesV1>={}):ReturnType<typeof createFleetMetricsCollectorV1>=>createFleetMetricsCollectorV1({directory,sources:{unitCounters:counts,...extra}});
const lastValues=(directory:string,id:string,toMs:number):readonly number[]=>(readSeriesV1({directory,fromMs:BASE-1000,toMs,ids:[id]}).metrics[id]??[]).map(point=>point[6]);

test("deltas: counter readings become per-step deltas, a reading below the one before is a restart, checked against the Python oracle", () => {
 const readings=[10,15,15,40,3,3,9],directory=scratch("delta");let at=0;
 const collector=collectorWith(directory,()=>unit(0,0,readings[at++]!));
 readings.forEach((_,index)=>collector.step(BASE+index*10_000));collector.close();
 const expected=(JSON.parse(python(["delta"],JSON.stringify({readings:readings.map(value=>({id:"build.units.compiled",value}))}))) as (number|null)[]).filter((value):value is number=>value!==null);
 expect(expected).toEqual([5,0,25,3,0,6]);
 expect(lastValues(directory,"build.units.compiled",BASE+readings.length*10_000)).toEqual(expected);
});

test("restart: the baseline of the last readings survives a new collector within its age and is only a baseline after it", () => {
 const directory=scratch("baseline");
 const first=collectorWith(directory,()=>unit(0,0,100));first.step(BASE);first.step(BASE+10_000);first.close();
 const second=collectorWith(directory,()=>unit(0,0,130));second.step(BASE+20_000);second.close();
 expect(lastValues(directory,"build.units.compiled",BASE+30_000)).toEqual([0,30]);
 expect(JSON.parse(readFileSync(join(directory,S.Policy.const.baselineFile),"utf8"))).toMatchObject({version:1,readings:{"build.units.compiled":130}});
 const late=collectorWith(directory,()=>unit(0,0,500));late.step(BASE+20_000+S.Policy.const.baselineMaximumAgeMilliseconds+60_000);late.close();
 expect(lastValues(directory,"build.units.compiled",BASE+10_000_000)).toEqual([0,30]);
});

const ORIGIN={role:"origin",checkout:"/Users/someone/semio"};
const lockOf=(directory:string):Record<string,unknown>=>JSON.parse(readFileSync(join(directory,S.Policy.const.lockFile),"utf8")) as Record<string,unknown>;

test("one writer: a live holder of the lock keeps a second collector idle, a dead holder is taken over, the holder gives the lock back", () => {
 const directory=scratch("lock");mkdirSync(directory,{recursive:true});
 const parent=processIdentityOfV1(process.ppid);writeFileSync(join(directory,S.Policy.const.lockFile),JSON.stringify(parent??{pid:process.ppid,startMs:1}));
 expect(existsSync(join(directory,S.Policy.const.lockFile))).toBe(true);
 const lines:string[]=[],idle=createFleetMetricsCollectorV1({directory,report:line=>lines.push(line),sources:{unitCounters:()=>unit(0,0,1)}});
 const live=idle.step(BASE);expect(live.owner||live.accepted>0).toBe(false);
 idle.step(BASE+1000);expect(lines.length,"the foreign holder is reported once, in both languages").toBe(1);expect(lines[0]).toContain(" / ");expect(lines[0]).toContain(`pid ${process.ppid}`);
 writeFileSync(join(directory,S.Policy.const.lockFile),JSON.stringify({pid:2147483000,startMs:1}));
 const taken=idle.step(BASE+2000);expect(taken.owner).toBe(true);expect(taken.accepted).toBeGreaterThan(0);
 idle.close();expect(existsSync(join(directory,S.Policy.const.lockFile))).toBe(false);
 expect(acquireMetricsLockV1(directory,ORIGIN).held).toBe(true);expect(acquireMetricsLockV1(directory,ORIGIN).held).toBe(true);releaseMetricsLockV1(directory,ORIGIN);expect(existsSync(join(directory,S.Policy.const.lockFile))).toBe(false);
});

test("the lock is owned by identity and role: it names pid, start time, role and checkout, validates against the schema and is released only by its owner and role", () => {
 const directory=scratch("identity");
 expect(acquireMetricsLockV1(directory,ORIGIN)).toEqual({held:true});
 const record=lockOf(directory),mine=processIdentityOfV1(process.pid)!;
 expect(record).toEqual({version:1,pid:mine.pid,startMs:mine.startMs,role:"origin",checkout:ORIGIN.checkout});
 const validate=new Ajv({strict:false}).compile({...S.Lock,definitions:S} as object);expect(validate(record),JSON.stringify(validate.errors)).toBe(true);
 expect(validate({pid:1,startMs:1}),"a record without role is no lock of this version").toBe(false);
 const other=acquireMetricsLockV1(directory,{role:"fleet:remote-cache"});
 expect(other.held).toBe(false);expect(other.refusal).toBe("foreign");expect(other.detail).toContain("origin");
 releaseMetricsLockV1(directory,{role:"fleet:remote-cache"});expect(existsSync(join(directory,S.Policy.const.lockFile)),"another role cannot release it").toBe(true);
 releaseMetricsLockV1(directory,ORIGIN);expect(existsSync(join(directory,S.Policy.const.lockFile))).toBe(false);
 expect(readdirSync(directory).filter(name=>name.endsWith(".tmp")),"no temporary record is left behind").toEqual([]);
});

test("a live holder of an older code (a record without role) is never robbed: two writers would corrupt the rings; a reused pid is no holder", () => {
 const directory=scratch("legacy");mkdirSync(directory,{recursive:true});
 const parent=processIdentityOfV1(process.ppid)!;writeFileSync(join(directory,S.Policy.const.lockFile),JSON.stringify(parent));
 const verdict=acquireMetricsLockV1(directory,ORIGIN);expect(verdict.held).toBe(false);expect(verdict.refusal).toBe("foreign");expect(verdict.detail).toContain("unnamed collector");
 writeFileSync(join(directory,S.Policy.const.lockFile),JSON.stringify({pid:parent.pid,startMs:parent.startMs+86_400_000}));
 expect(acquireMetricsLockV1(directory,ORIGIN).held,"same pid, other start time: the holder is gone").toBe(true);
 releaseMetricsLockV1(directory,ORIGIN);
 writeFileSync(join(directory,S.Policy.const.lockFile),"{");
 expect(acquireMetricsLockV1(directory,ORIGIN).refusal,"a record that was just created may still be written").toBe("foreign");
});

test("only a real collector holds the lock of the machine-wide ring directory: a scratch checkout, a test runner, an unknown role and a missing checkout are refused, a private directory is free", () => {
 const machine=scratch("machine"),metrics=join(machine,schemaFiles.directory),production={NODE_ENV:"production"},previous=process.env["SEMIO_HOST_CAPACITY_DIRECTORY"];
 process.env["SEMIO_HOST_CAPACITY_DIRECTORY"]=machine;
 try{
  const scratchCheckout=scratch("checkout");
  expect(metricsLockRefusalV1(metrics,{role:"origin",checkout:scratchCheckout},production)).toContain("temporary directory");
  expect(metricsLockRefusalV1(join(metrics,"fleet","remote-cache"),{role:"fleet:remote-cache",checkout:scratchCheckout},production)).toContain("temporary directory");
  expect(metricsLockRefusalV1(metrics,ORIGIN,{NODE_ENV:"test"})).toContain("test runner");
  expect(metricsLockRefusalV1(metrics,{role:"origin"},production)).toContain("no checkout");
  expect(metricsLockRefusalV1(metrics,{role:"gardener",checkout:ORIGIN.checkout},production)).toContain("unknown role");
  expect(metricsLockRefusalV1(metrics,ORIGIN,production),"a real collector of a checkout may").toBeUndefined();
  expect(metricsLockRefusalV1(join(metrics,"fleet","remote-cache"),{role:"fleet:remote-cache"},production),"a fleet-scope source needs no checkout").toBeUndefined();
  expect(metricsLockRefusalV1(scratch("private"),{role:"origin",checkout:scratchCheckout},{NODE_ENV:"test"}),"a private directory is free for a test").toBeUndefined();
  const refused=acquireMetricsLockV1(metrics,{role:"origin",checkout:scratchCheckout});
  expect(refused.held,"the real acquire refuses it too").toBe(false);expect(refused.refusal).toBe("refused");expect(existsSync(join(metrics,S.Policy.const.lockFile))).toBe(false);
  const lines:string[]=[],collector=createFleetMetricsCollectorV1({directory:metrics,checkout:scratchCheckout,report:line=>lines.push(line),sources:{unitCounters:()=>unit(0,0,1)}});
  expect(collector.step(BASE).owner).toBe(false);collector.step(BASE+1000);collector.close();
  expect(lines.length,"the refusal is reported once, in both languages").toBe(1);expect(lines[0]).toContain(" / ");expect(existsSync(metrics)&&readdirSync(metrics),"nothing is written into the machine directory").toEqual(false);
 }finally{if(previous===undefined)delete process.env["SEMIO_HOST_CAPACITY_DIRECTORY"];else process.env["SEMIO_HOST_CAPACITY_DIRECTORY"]=previous;}
});

test("a collector that finds its lock taken by another process stops writing instead of writing next to it", () => {
 const directory=scratch("lost");
 const collector=createFleetMetricsCollectorV1({directory,sources:{unitCounters:()=>unit(0,0,1)}});
 expect(collector.step(BASE).owner).toBe(true);
 const parent=processIdentityOfV1(process.ppid)!;writeFileSync(join(directory,S.Policy.const.lockFile),JSON.stringify({...parent,version:1,role:"origin"}));
 const later=collector.step(BASE+11_000);expect(later.owner,"the slow step notices it").toBe(false);expect(later.accepted).toBe(0);
 rmSync(join(directory,S.Policy.const.lockFile));
 expect(collector.step(BASE+12_000).owner,"it takes the lock again when the other one is gone").toBe(false);
 expect(collector.step(BASE+42_000).owner).toBe(true);
 collector.close();
});

test("a failing collector costs only its own metrics and is reported once", () => {
 const directory=scratch("fail"),lines:string[]=[];
 const collector=createFleetMetricsCollectorV1({directory,report:line=>lines.push(line),sources:{unitCounters:()=>{throw new Error("boom");},queueDepth:()=>7}});
 const first=collector.step(BASE),second=collector.step(BASE+10_000);collector.close();
 expect(first.accepted).toBeGreaterThan(0);expect(second.accepted).toBeGreaterThan(0);
 expect(lines.length).toBe(1);expect(lines[0]).toContain("unit-counters");expect(lines[0]).toContain(" / ");
 expect(lastValues(directory,"build.tasks.queued",BASE+20_000)).toEqual([7,7]);
});

const DAY=86_400_000;
const ring=(directory:string,rows:number,spacing:number=1000):void=>{
 const collector=createFleetMetricsCollectorV1({directory,sources:{queueDepth:()=>1}});
 for(let index=0;index<rows;index++)collector.step(BASE+index*spacing);
 collector.close();
};

test("publication: the first look copies a file whole and atomically, every later look appends only the closed rows in place, the published t0 is the whole ring", () => {
 const from=scratch("from"),to=scratch("to"),events:string[]=[];
 const collector=createFleetMetricsCollectorV1({directory:from,store:{root:to,origin:"g1-x"},sources:{queueDepth:()=>1},report:line=>events.push(line)});
 for(let index=0;index<25;index++)collector.step(BASE+index*1000);
 const published=join(to,"v1","origins","g1-x","metrics");
 expect(readdirSync(published).sort(),"only the tiers that have a row are published").toEqual(["t0.ring","t1.ring"]);
 expect(readRingFileV1(join(published,"t0.ring"))!.bucketCount).toBe(METRICS_TIERS_V1[0]!.buckets);
 expect(readdirSync(published).filter(name=>name.endsWith(".tmp"))).toEqual([]);
 expect(readRingFileV1(join(published,"t0.ring"))!.rows.length).toBeGreaterThan(15);
 collector.close();
 for(const name of readdirSync(published))expect(readFileSync(join(published,name)),name).toEqual(readFileSync(join(from,name)));
 expect(readRingFileV1(join(published,"t0.ring"))!.rows.length).toBe(25);
 expect(events).toEqual([]);
});

test("publication: a look appends exactly the rows closed since the last one (a few KB), an unchanged file costs nothing, a restarted publisher continues from the published header, another layout is copied whole", () => {
 const from=scratch("append"),to=scratch("appendto"),ids=ALL,rowBytes=2*8+ids.length*36;
 const writer=createFleetMetricsCollectorV1({directory:from,sources:{queueDepth:()=>1}});
 for(let index=0;index<22;index++)writer.step(BASE+index*1000);
 const publisher=createMetricsPublisherV1({from,to});
 const first=publisher.publish(BASE);expect(first.failed).toBe(0);expect(first.copied,"t0 and t1 have rows, the older tiers none yet").toBe(2);expect(first.bytes).toBeGreaterThan(rowBytes*METRICS_TIERS_V1[0]!.buckets);
 expect(publisher.publish(BASE+1).copied,"nothing changed").toBe(0);
 for(let index=22;index<27;index++)writer.step(BASE+index*1000);
 const look=publisher.publish(BASE+5000);
 expect(look.copied).toBe(1);expect(look.rows).toBe(5);expect(look.bytes).toBe(5*rowBytes+8);expect(look.bytes).toBeLessThan(20_000);
 expect(readRingFileV1(join(to,"t0.ring"))!.rows.length).toBe(26);
 for(let index=27;index<30;index++)writer.step(BASE+index*1000);
 const restarted=createMetricsPublisherV1({from,to}).publish(BASE+9000);
 expect(restarted.copied).toBe(1);expect(restarted.rows).toBe(3);expect(restarted.bytes).toBe(3*rowBytes+8);expect(restarted.bytes).toBeLessThan(20_000);
 writer.close();
 createMetricsPublisherV1({from,to}).publish(BASE+10_000);
 for(const name of readdirSync(to))expect(readFileSync(join(to,name)),name).toEqual(readFileSync(join(from,name)));
 const viaPython=JSON.parse(python(["rows",join(to,"t0.ring")])) as {start:number;slots:number[][]}[];
 expect(viaPython.map(row=>row.start),"the independent Python reader sees the appended rows").toEqual(readRingFileV1(join(to,"t0.ring"))!.rows.map(row=>row.start));
 expect(viaPython.map(row=>row.slots)).toEqual(readRingFileV1(join(to,"t0.ring"))!.rows.map(row=>row.slots.map(slot=>[...slot])));
 const other=openMetricsRingsV1({directory:from,ids:["host.cpu.busy","host.cpu.load"]});
 recordSampleV1(other,BASE+60_000,{"host.cpu.busy":5});recordSampleV1(other,BASE+61_000,{"host.cpu.busy":6});closeMetricsRingsV1(other);
 const again=createMetricsPublisherV1({from,to}).publish(BASE+70_000);
 expect(again.failed).toBe(0);expect(readRingFileV1(join(to,"t0.ring"))!.ids).toEqual(["host.cpu.busy","host.cpu.load"]);
 expect(readFileSync(join(to,"t0.ring"))).toEqual(readFileSync(join(from,"t0.ring")));
});

test("publication volume: 300 s of one machine publish about 3 KB per second once the rings exist, far below the 75 KB/s budget", () => {
 const from=scratch("volume"),to=scratch("volumeto"),publisher=createMetricsPublisherV1({from,to}),writer=createFleetMetricsCollectorV1({directory:from,sources:{queueDepth:()=>1,unitCounters:()=>unit(0,0,1)}});
 let bytes=0,steady=0;
 for(let second=0;second<300;second++){
  writer.step(BASE+second*1000);
  if(second%5===0){const outcome=publisher.publish(BASE+second*1000);bytes+=outcome.bytes;if(second>=100)steady+=outcome.bytes;expect(outcome.failed).toBe(0);}
 }
 writer.close();
 expect(bytes/300,"bytes per second including the first whole copies of the tiers").toBeLessThan(75_000);
 expect(steady/200,"steady state bytes per second").toBeLessThan(4000);
});

test("publication: no AppleDouble stub is ever written next to a ring, existing ones are swept, a copy carries no extended attribute of its source", () => {
 const from=scratch("adfrom"),to=scratch("adto");
 ring(from,5,2*DAY);
 writeFileSync(join(to,"._t0.ring"),"stub");writeFileSync(join(to,"._t4.ring"),"stub");writeFileSync(join(to,"keep.txt"),"keep");
 if(process.platform==="darwin"){const set=spawnSync("/usr/bin/xattr",["-w","com.semio.m7","1",join(from,"t0.ring")],{encoding:"utf8"});expect(set.status,set.stderr).toBe(0);}
 const publisher=createMetricsPublisherV1({from,to});
 expect(publisher.publish(BASE,true).failed).toBe(0);
 expect(readdirSync(to).sort()).toEqual(["keep.txt","t0.ring","t1.ring","t2.ring","t3.ring","t4.ring"]);
 if(process.platform==="darwin"){const listed=spawnSync("/usr/bin/xattr",["-l",join(to,"t0.ring")],{encoding:"utf8"});expect(listed.status,listed.stderr).toBe(0);expect(listed.stdout,"the extended attributes of the source are not copied into the store (the kernel's own provenance mark is not ours to avoid)").not.toContain("com.semio.m7");}
 expect(readSeriesV1({directory:to,fromMs:BASE-1000,toMs:BASE+10*DAY,ids:["build.tasks.queued"]}).metrics["build.tasks.queued"]!.length,"the readers ignore every ._ file").toBeGreaterThan(0);
});

test("publication: stubs that appear later (a peer process of an older version copying whole) are swept at the next sweep interval", () => {
 const from=scratch("lfrom"),to=scratch("lto");
 ring(from,5,2*DAY);
 const publisher=createMetricsPublisherV1({from,to});
 publisher.publish(BASE,true);
 writeFileSync(join(to,"._t2.ring"),"late");
 publisher.publish(BASE+1000,true);expect(readdirSync(to)).toContain("._t2.ring");
 publisher.publish(BASE+11*60_000,true);expect(readdirSync(to).filter(name=>name.startsWith("._"))).toEqual([]);
});

test("publication: a failed rename removes its temporary file, a refused file is reported once, a dead writer's temporary file is swept, bounds hold", () => {
 const from=scratch("pubfrom"),to=scratch("pubto"),lines:string[]=[];
 ring(from,5,2*DAY);
 const publisher=createMetricsPublisherV1({from,to,report:line=>lines.push(line)});
 mkdirSync(join(to,"t1.ring"));
 writeFileSync(join(to,`t3.ring.2147483000.tmp`),"dead");writeFileSync(join(to,`t0.ring.${process.ppid}.tmp`),"live");writeFileSync(join(to,"notes.txt"),"keep");
 const outcome=publisher.publish(BASE,true);
 expect(outcome.failed).toBe(1);expect(outcome.copied).toBe(4);
 expect(readdirSync(to).sort()).toEqual(["notes.txt","t0.ring","t1.ring","t2.ring","t3.ring","t4.ring",`t0.ring.${process.ppid}.tmp`].sort());
 expect(readdirSync(join(to,"t1.ring"))).toEqual([]);
 expect(lines.length).toBe(1);publisher.publish(BASE+1,true);expect(lines.length).toBe(1);
 writeFileSync(join(from,"t4.ring"),"not a ring at all but long enough to pass the header size check of sixty four bytes....");
 const refused=createMetricsPublisherV1({from,to:scratch("pubto2"),report:line=>lines.push(line)}).publish(BASE,true);
 expect(refused.failed).toBeGreaterThanOrEqual(1);expect(lines.some(line=>line.includes("t4.ring"))).toBe(true);
});

test("publication: every published file is a valid ring at every look while the writer keeps writing (no torn file is ever visible)", () => {
 const from=scratch("tornfrom"),to=scratch("tornto"),publisher=createMetricsPublisherV1({from,to});
 const collector=createFleetMetricsCollectorV1({directory:from,sources:{queueDepth:()=>2}});
 for(let index=0;index<40;index++){
  collector.step(BASE+index*1000);publisher.publish(BASE+index*60_000,true);
  for(const name of readdirSync(to)){expect(name.endsWith(".tmp")).toBe(false);const parsed=readRingFileV1(join(to,name));expect(parsed,name).not.toBeNull();}
 }
 collector.close();
});

test("fleet scope: a fleet collector feeds only the fleet metrics of its source, keeps rings of only those ids and publishes below fleet/metrics/<source>", () => {
 const directory=scratch("fleet"),root=scratch("fleetstore"),readings=[100,130,130,131];let at=0;
 const collector=createFleetMetricsCollectorV1({directory,fleet:{source:"remote-execution",root},sources:{remoteExecution:()=>({running:2,queued:5,offloadedIn:readings[at++]!}),queueDepth:()=>9,unitCounters:()=>unit(0,0,1)}});
 for(let index=0;index<readings.length;index++)collector.step(BASE+index*10_000);
 collector.close();
 const published=join(root,"v1","fleet","metrics","remote-execution");
 expect(readdirSync(published).sort(),"only the tiers that have a row are published").toEqual(["t0.ring","t1.ring"]);
 expect(readRingFileV1(join(published,"t0.ring"))!.ids.slice().sort()).toEqual(["exec.offload.in","exec.queue.queued","exec.queue.running"]);
 const series=readSeriesV1({directory:published,fromMs:BASE-1000,toMs:BASE+100_000});
 expect(series.metrics["exec.offload.in"]!.map(point=>point[6])).toEqual([30,0,1]);
 expect(series.metrics["exec.queue.running"]!.map(point=>point[6])).toEqual([2,2,2,2]);
 expect(readFileSync(join(published,"t0.ring"))).toEqual(readFileSync(join(directory,"t0.ring")));
 const other=scratch("fleetother");
 const status=createFleetMetricsCollectorV1({directory:other,fleet:{source:"remote-cache",root},sources:{remoteCacheStatus:()=>({entries:7,bytes:70})}});status.step(BASE);status.close();
 expect(readRingFileV1(join(root,"v1","fleet","metrics","remote-cache","t0.ring"))!.ids.slice().sort()).toEqual(["cache.remote.bytes","cache.remote.entries"]);
 const origin=scratch("fleetorigin"),mine=createFleetMetricsCollectorV1({directory:origin,sources:{remoteExecution:()=>({running:2,queued:5,offloadedIn:1}),remoteCacheStatus:()=>({entries:7,bytes:70}),queueDepth:()=>3}});mine.step(BASE);mine.close();
 expect(readSeriesV1({directory:origin,fromMs:BASE-1000,toMs:BASE+10_000,ids:["exec.queue.running","cache.remote.entries","build.tasks.queued"]}).metrics).toEqual({"exec.queue.running":[],"cache.remote.entries":[],"build.tasks.queued":[expect.any(Array)]});
});

test("host sources against independent measurements: free space and size of a volume (statfs and Python), memory total (Python sysconf)", () => {
 const sources=hostMetricsSourcesV1({volumes:()=>[tmpdir()],directory:scratch("hostdir")});
 const [volume]=sources.volumes!()!,fact=statfsSync(tmpdir());
 expect(volume!.totalBytes).toBe(Number(fact.blocks)*Number(fact.bsize));
 const probe=JSON.parse(pythonCode("import json,os,shutil,sys;u=shutil.disk_usage(sys.argv[1]);print(json.dumps({'total':u.total,'free':u.free,'mem':(os.sysconf('SC_PHYS_PAGES')*os.sysconf('SC_PAGE_SIZE')) if hasattr(os,'sysconf') and 'SC_PHYS_PAGES' in os.sysconf_names else None}))",tmpdir())) as {total:number;free:number;mem:number|null};
 expect(volume!.totalBytes).toBe(probe.total);
 expect(Math.abs(volume!.freeBytes-probe.free)).toBeLessThan(2*1024*MEGABYTE);
 if(probe.mem!==null&&process.platform!=="win32")expect(sources.memory!()!.totalBytes).toBe(probe.mem);
 expect(sources.memory!()!.totalBytes).toBe(totalmem());expect(sources.memory!()!.freeBytes).toBeLessThanOrEqual(totalmem());expect(freemem()).toBeGreaterThan(0);
});

test("host sources on this machine: processor, cores, self and volumes answer sane numbers without a process spawn", () => {
 const directory=scratch("real"),sources=hostMetricsSourcesV1({volumes:()=>metricsVolumePathsV1(directory,join(directory,".cache"),tmpdir()),directory});
 const first=sources.cpuTimes!()!;
 let burn=0;const stop=Date.now()+30;while(Date.now()<stop)burn+=Math.sqrt(burn+1);
 const second=sources.cpuTimes!()!;
 expect(second.total).toBeGreaterThan(first.total);expect(second.busy).toBeGreaterThanOrEqual(first.busy);
 expect(sources.cores!()).toBeGreaterThan(0);expect(sources.self!().rssBytes).toBeGreaterThan(1_000_000);
 expect(sources.volumes!()!.length).toBeGreaterThan(0);expect(sources.warden!()).toBeNull();expect(sources.gate!()).toBeNull();expect(sources.queueDepth!()).toBeNull();
 const collector=createFleetMetricsCollectorV1({directory:join(directory,"rings"),sources});
 collector.step(BASE);const step=collector.step(BASE+1000);collector.close();
 expect(step.accepted).toBeGreaterThan(8);expect(step.dropped+step.unknown).toBe(0);
});
