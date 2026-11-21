import{afterAll,expect,test}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{spawnSync}from"node:child_process";
import{startRemoteCacheServerV1}from"../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{createProjectFleetPortV1,type ProjectClaimV1,type ProjectRecordInfoV1}from"../../../../../🪪️graph-state/📦️project-records/🟦️.ts";
import{graphFleetClientV1}from"../../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../../../🪪️graph-state/🧶️nx/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schemaOfPool from"../🧬️schema/🔣️.json"with{type:"json"};
import{createOwnerPoolStatusV1}from"../📊️status/🟦️.ts";
import{acquireOwnerPoolGuardV1}from"../🔒️guard/🟦️.ts";
import{runOwnerPoolV1}from"../🏃️runner/🟨️.mjs";
import{ownerPoolProcessStartV1}from"../🪦️reaper/🟦️.ts";
import{OWNER_POOL_POLICY_V1 as P,ownerPoolSizingV1,ownerPoolChildLimitsV1,ownerPoolChildMegabytesV1,ownerPoolIdV1,ownerPoolOpenWavesV1,ownerPoolLiveChildrenV1,ownerPoolKeepWaveV1,ownerPoolReopenMillisecondsV1,ownerPoolRecycleV1,ownerPoolStagingDirectoryV1,writeOwnerPoolWaveFilesV1,ownerPoolRandomV1,ownerPoolSeedV1,ownerPoolTargetV1,publishOwnerPoolFailureV1,publishOwnerPoolV1,pullOwnerPoolV1,readOwnerPoolFailureV1,readOwnerPoolPointerV1,readOwnerPoolV1,type OwnerPoolHostV1,type OwnerPoolManifestV1,type OwnerPoolStatsV1}from"../🟦️.ts";

/** 🌐️ The owner pool: every machine runs as many child processes as its cores and free memory allow, each child walks the list from a start of its own and claims a window of owners per round trip, and the waves of the pointer only ever accumulate. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`owner-pool-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const fast={...P,claimWindowMaximum:P.claimWindow,doneStoredMinimum:12,sweepPollMilliseconds:5,heldRetryMilliseconds:60,faultRetryMilliseconds:60};
const fixturePath=join(import.meta.dir,"🧫️fixtures","🔣️.json");
type Fixture={policy:Record<string,number>;target:{name:string;host:OwnerPoolHostV1;children:number;childMegabytes:number}[];limits:{total:number}[];childMegabytes:{samples:number[];live?:number[]}[];recycle:{rss:number;readyRss:number;recycleMegabytes:number}[];walk:{puller:string;n:number;draws:number}[];expect:{target:{children:number;reason:string|null}[];limit:Record<string,unknown>[];limits:{recycleMegabytes:number;hardMegabytes:number}[];childMegabytes:number[];recycle:boolean[];walk:number[][]}};
const fixture=JSON.parse(readFileSync(fixturePath,"utf8")) as Fixture;

test("sizing, limits, the child estimate and the walk of a puller equal an independent Python implementation and the authored fixture",()=>{
 for(const [key,value] of Object.entries(fixture.policy))expect((P as Record<string,unknown>)[key]).toBe(value);
 const ours={
  target:fixture.target.map(row=>ownerPoolTargetV1({host:row.host,children:row.children,childMegabytes:row.childMegabytes})),
  limit:fixture.target.map(row=>{const {limit,needMegabytes,freeMegabytes,heldMegabytes,floorMegabytes}=ownerPoolSizingV1({host:row.host,children:row.children,childMegabytes:row.childMegabytes}).limit;return{limit,needMegabytes,freeMegabytes,heldMegabytes,floorMegabytes};}),
  limits:fixture.limits.map(row=>ownerPoolChildLimitsV1(row.total)),
  childMegabytes:fixture.childMegabytes.map(row=>ownerPoolChildMegabytesV1(row.samples,row.live??[])),
  recycle:fixture.recycle.map(row=>ownerPoolRecycleV1(row)),
  walk:fixture.walk.map(row=>{let state=ownerPoolSeedV1(row.puller);return Array.from({length:row.draws},()=>{const next=ownerPoolRandomV1(state);state=next.state;return Math.floor(next.value*row.n);});}),
 };
 expect(ours).toEqual(fixture.expect);
 const python=spawnSync("python3",["-I",join(import.meta.dir,"🧫️fixtures","🐍️oracle.py"),fixturePath],{encoding:"utf8"});
 expect(python.status).toBe(0);expect(JSON.parse(python.stdout)).toEqual(ours);
});

const idle=(cores:number,available:number,extra:Partial<OwnerPoolHostV1>={}):OwnerPoolHostV1=>({cores,totalMegabytes:cores*2048,availableMegabytes:available,pressure:"normal",load:null,swapOutMegabytesPerSecond:null,foreground:{cpu:0,unrealizedMegabytes:0,waiters:0},...extra});
test("admission law: idle headroom is used by one child per core, the machine's own work is respected, and the pool is never starved to zero by it",()=>{
 expect(ownerPoolTargetV1({host:idle(10,24000),children:0,childMegabytes:800}).children).toBe(10);
 for(let booked=0;booked<=10;booked++){
  const target=ownerPoolTargetV1({host:idle(10,24000,{foreground:{cpu:booked,unrealizedMegabytes:0,waiters:0}}),children:0,childMegabytes:800}).children;
  expect(target).toBe(Math.max(Math.ceil(10*P.minimumShareBasisPoints/10000),10-booked));
 }
 const waiting=ownerPoolTargetV1({host:idle(10,24000,{foreground:{cpu:0,unrealizedMegabytes:0,waiters:4}}),children:0,childMegabytes:800}).children;
 expect(waiting).toBe(10-Math.ceil(10*P.foregroundCoreShareBasisPoints/10000));
 expect(ownerPoolTargetV1({host:idle(10,24000,{foreground:{cpu:10,unrealizedMegabytes:0,waiters:9}}),children:0,childMegabytes:800}).children).toBe(1);
 expect(ownerPoolTargetV1({host:idle(10,24000,{pressure:"critical"}),children:5,childMegabytes:800})).toEqual({children:0,reason:"pressure"});
 const memory=[1500,3000,6000,12000].map(available=>ownerPoolTargetV1({host:idle(10,available),children:0,childMegabytes:800}).children);
 expect(memory).toEqual([...memory].sort((a,b)=>a-b));expect(memory[0]).toBe(1);expect(memory[3]).toBeGreaterThanOrEqual(10);
 const loud=ownerPoolTargetV1({host:idle(10,24000,{load:42.7}),children:3,childMegabytes:800});
 expect(loud.children).toBe(Math.ceil(10*P.minimumShareBasisPoints/10000));expect(loud.reason).toBe("yielding");
 const own=ownerPoolTargetV1({host:idle(10,24000,{load:8}),children:8,childMegabytes:800});
 expect(own.children).toBe(10);
});

test("the schema, the pointer and the texts are closed and German twins exist for every English text",()=>{
 const errors=validateJsonSchemaSubset({$ref:"#/definitions/Pointer"},{version:2,waves:[{id:"a".repeat(64),createdMs:1,units:2}]},schemaOfPool);
 expect(errors).toEqual([]);
 expect(validateJsonSchemaSubset({$ref:"#/definitions/Pointer"},{version:1,id:"a".repeat(64),createdMs:1,units:2},schemaOfPool).length).toBeGreaterThan(0);
 const en=Object.keys(schemaOfPool.definitions.Texts.const.en).sort(),de=Object.keys(schemaOfPool.definitions.Texts.const.de).sort();
 expect(de).toEqual(en);
 const dir=temp("status"),status=createOwnerPoolStatusV1({store:dir,origin:"g1-test",role:"puller"});
 status.update({state:"pulling",children:3,target:4,ownersPerMinute:12.5,claimAttempts:7,wave:"w"});status.flush();
 const written=JSON.parse(readFileSync(join(dir,"v1","origins","g1-test","pool.json"),"utf8"));
 expect(validateJsonSchemaSubset({$ref:"#/definitions/Status"},written,schemaOfPool)).toEqual([]);
});

async function world(){
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 return{server,environment};
}
type Unit={name:string;kind:string;expectedMilliseconds:number;json:Record<string,unknown>};
const unitsOf=(count:number):Unit[]=>Array.from({length:count},(_all,index)=>({name:`owner-${String(index).padStart(4,"0")}`,kind:"k",expectedMilliseconds:60000,json:{name:`owner-${index}`}}));
const keyTextOf=(unit:Unit)=>JSON.stringify(["rev",null,[unit.name]]);
const portableOf=(root:string,unit:Unit)=>({digest:sha(unit.name),authority:[],observations:[{kind:"file",path:`${root}/${unit.name}/a.ts`,fact:{sha256:"x",size:1,mtimeMs:1,ctimeMs:1,ino:1}}],image:JSON.stringify({name:unit.name})});
const infoOf=(root:string,unit:Unit,imported?:unknown[]):ProjectRecordInfoV1=>({cachePath:join(root,"np",unit.name+".json"),fleet:`libs/${unit.name}`,keyText:keyTextOf(unit),exportRecord:()=>portableOf(root,unit),async import(portable){imported?.push(portable);return true;}});
const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));

type Table=Map<string,"held"|"stored">;
const memoryClaim=(table:Table,counter:{asked:number;inflight:number;widest:number})=>async(unit:Unit):Promise<ProjectClaimV1>=>{
 counter.asked++;counter.inflight++;counter.widest=Math.max(counter.widest,counter.inflight);await pause(1);counter.inflight--;
 const held=table.get(unit.name);
 if(held==="stored")return{state:"stored"};if(held==="held")return{state:"held",by:"x"};
 table.set(unit.name,"held");
 return{state:"granted",digest:sha(unit.name),why:()=>"",deterministic:()=>false,publish:async()=>{table.set(unit.name,"stored");return true;},abandon:async()=>{table.delete(unit.name);}};
};
const fleetOf=async(pullers:number,units:Unit[],options:{slots?:number;burn?:number}={})=>{
 const table:Table=new Map(),counter={asked:0,inflight:0,widest:0},computed=new Map<string,number>(),started=Date.now();
 const stats=await Promise.all(Array.from({length:pullers},(_all,index)=>pullOwnerPoolV1<Unit>({units:()=>units,puller:`machine-${index}:1`,policy:fast,slots:()=>options.slots??1,claim:memoryClaim(table,counter),run:async unit=>{computed.set(unit.name,(computed.get(unit.name)??0)+1);await pause(options.burn??1);return true;}})));
 return{stats,table,counter,computed,seconds:(Date.now()-started)/1000};
};
const total=(stats:OwnerPoolStatsV1[])=>stats.reduce((sum,row)=>({claimed:sum.claimed+row.claimed,published:sum.published+row.published,stored:sum.stored+row.stored,held:sum.held+row.held,faulted:sum.faulted+row.faulted,failed:sum.failed+row.failed,refused:sum.refused+row.refused,attempts:sum.attempts+row.attempts}),{claimed:0,published:0,stored:0,held:0,faulted:0,failed:0,refused:0,attempts:0});

test("ten pullers on one list never infer an owner twice and spend about one claim request per owner",async()=>{
 const units=unitsOf(1500),fleet=await fleetOf(10,units),sum=total(fleet.stats);
 expect(sum.published).toBe(1500);
 for(const unit of units)expect(fleet.computed.get(unit.name)).toBe(1);
 expect([...fleet.table.values()].every(value=>value==="stored")).toBe(true);
 expect(sum.attempts/sum.published).toBeLessThan(1.25);
 expect(fleet.stats.every(row=>row.published>0)).toBe(true);
});

test("ten pullers on a 4774 owner wave: attempts per success stay near 1 and nobody is left out",async()=>{
 const units=unitsOf(4774),fleet=await fleetOf(10,units),sum=total(fleet.stats);
 expect(sum.published).toBe(4774);
 expect(Math.max(...fleet.computed.values())).toBe(1);
 expect(sum.attempts/sum.published).toBeLessThan(1.08);
 expect(Math.min(...fleet.stats.map(row=>row.published))).toBeGreaterThan(100);
},60000);

test("a puller claims a window of owners in one round trip and its throughput grows linearly with its slots up to the size of the list",async()=>{
 const rows:{slots:number;seconds:number;widest:number}[]=[];
 for(const slots of[1,2,4,8]){
  const fleet=await fleetOf(1,unitsOf(64),{slots,burn:40});
  expect(total(fleet.stats).published).toBe(64);rows.push({slots,seconds:fleet.seconds,widest:fleet.counter.widest});
 }
 const base=rows[0]!.seconds;
 expect(base/rows[1]!.seconds).toBeGreaterThan(1.7);expect(base/rows[2]!.seconds).toBeGreaterThan(3.1);expect(base/rows[3]!.seconds).toBeGreaterThan(5);
 expect(rows[3]!.widest).toBeGreaterThanOrEqual(P.claimWindow);
},30000);

test("ten pullers with one slot each are ten times faster than one, and a puller that is given more slots later uses them at once",async()=>{
 const units=unitsOf(200),one=await fleetOf(1,units,{burn:20}),ten=await fleetOf(10,units,{burn:20});
 expect(one.seconds/ten.seconds).toBeGreaterThan(6);
 const table:Table=new Map(),counter={asked:0,inflight:0,widest:0};let slots=1;const began=Date.now(),seen:number[]=[];
 const run=pullOwnerPoolV1<Unit>({units:()=>units,puller:"grow:1",policy:fast,slots:()=>slots,claim:memoryClaim(table,counter),busy:count=>{seen.push(count);},run:async()=>{await pause(30);return true;}});
 await pause(200);slots=6;const stats=await run;
 expect(stats.published).toBe(200);expect(Math.max(...seen)).toBeGreaterThanOrEqual(5);expect(Date.now()-began).toBeLessThan(200*30/1);
},30000);

test("a stopped puller takes nothing new, gives back what it claimed ahead and reports the owners in flight",async()=>{
 const table:Table=new Map(),counter={asked:0,inflight:0,widest:0};let stop=false;const flights:number[]=[];
 const stats=await pullOwnerPoolV1<Unit>({units:()=>unitsOf(40),puller:"stop:1",policy:fast,slots:()=>2,stop:()=>stop,claim:memoryClaim(table,counter),busy:count=>flights.push(count),run:async()=>{await pause(30);stop=true;return true;}});
 expect(stats.published).toBeGreaterThanOrEqual(1);expect(stats.published).toBeLessThanOrEqual(2);
 expect([...table.values()].filter(value=>value==="held").length).toBe(0);expect(flights.at(-1)).toBe(0);
});

test("a failed inference gives the claim back at once and says whether the cause is deterministic",async()=>{
 const table:Table=new Map(),counter={asked:0,inflight:0,widest:0},failures:string[]=[];let first=true;
 const stats=await pullOwnerPoolV1<Unit>({units:()=>unitsOf(3),puller:"fail:1",policy:fast,slots:()=>1,claim:memoryClaim(table,counter),fail:(name,message,deterministic)=>failures.push(`${name}:${deterministic}:${message.slice(0,12)}`),run:async()=>{if(first){first=false;throw new Error("boom");}return true;}});
 expect(stats.failed).toBe(1);expect(stats.published).toBe(2);expect(failures.length).toBe(1);expect(failures[0]).toContain(":true:boom");
});

test("two machines pulling one pool through the real cache never both finish the same owner, and every owner ends stored",async()=>{
 const {server,environment}=await world();
 try{
  const computed=new Map<string,string[]>(),units=unitsOf(36);
  const machine=(name:string)=>{const root=temp(name),port=createProjectFleetPortV1({root,environment,owner:name+":1"})!;return pullOwnerPoolV1<Unit>({units:()=>units,puller:name,policy:fast,slots:()=>2,claim:unit=>port.claim(infoOf(root,unit)),run:async unit=>{computed.set(unit.name,[...(computed.get(unit.name)??[]),name]);await pause(15);return true;}});};
  const [x,y]=await Promise.all([machine("a"),machine("b")]);
  for(const unit of units)expect(computed.get(unit.name)?.length).toBe(1);
  expect(x.published+y.published).toBe(36);expect(x.published).toBeGreaterThan(0);expect(y.published).toBeGreaterThan(0);
  const reader=createProjectFleetPortV1({root:temp("r"),environment,owner:"r:1"})!;
  for(const unit of units)expect((await reader.claim(infoOf("/r",unit))).state).toBe("stored");
 }finally{await server.stop();}
});

test("the machine that runs the wave takes the record another machine stored and infers nothing for it",async()=>{
 const {server,environment}=await world();
 try{
  const units=unitsOf(6),rootW=temp("w"),port=createProjectFleetPortV1({root:rootW,environment,owner:"w:1"})!;
  await pullOwnerPoolV1<Unit>({units:()=>units,puller:"w",policy:fast,slots:()=>2,claim:unit=>port.claim(infoOf(rootW,unit)),run:async()=>true});
  const rootS=temp("s"),imported:unknown[][]=units.map(()=>[]),consumer=createProjectFleetPortV1({root:rootS,environment,owner:"s:1"})!;
  let inferred=0;
  for(const [index,unit] of units.entries()){const gate=await consumer.gate(infoOf(rootS,unit,imported[index]));if(!gate.seeded)inferred++;}
  expect(inferred).toBe(0);expect(consumer.producers()).toEqual({w:6});expect(consumer.stats().computed).toBe(0);
 }finally{await server.stop();}
});

const manifestOf=(units:Unit[],tag:string,createdMs:number):OwnerPoolManifestV1=>{
 const partial={version:1 as const,identity:"identity-"+tag,globals:{projectAuthority:"a",registryDigest:"r",commandRevision:"c",late:false},publisher:"p:"+tag};
 return{head:{...partial,id:ownerPoolIdV1(partial,units),createdMs},session:Buffer.from(JSON.stringify({tag})),units};
};
test("waves accumulate in the pointer: a newer wave never replaces an open one, equal ids once, old ones and surplus ones leave",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("p"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})}),now=Date.now();
  expect(await readOwnerPoolPointerV1(client)).toBeUndefined();
  const a=manifestOf(unitsOf(5),"a",now-3000),b=manifestOf(unitsOf(9),"b",now-2000),c=manifestOf(unitsOf(7),"c",now-1000);
  for(const manifest of[a,b,c,b])expect(await publishOwnerPoolV1({client,owner:"o",manifest})).toBe(true);
  const pointer=await readOwnerPoolPointerV1(client);
  expect(pointer?.waves.map(wave=>wave.id)).toEqual([a.head.id,b.head.id,c.head.id]);expect(pointer?.waves.map(wave=>wave.units)).toEqual([5,9,7]);
  const read=await readOwnerPoolV1(client,b.head.id);
  expect(read?.units.length).toBe(9);expect(Buffer.from(read!.session).toString()).toContain('"b"');
  const old=manifestOf(unitsOf(2),"old",now-P.manifestMaximumAgeMilliseconds-5000);
  await publishOwnerPoolV1({client,owner:"o",manifest:old});
  expect((await readOwnerPoolPointerV1(client))?.waves.some(wave=>wave.id===old.head.id)).toBe(false);
  const many=Array.from({length:P.wavesKept+3},(_all,index)=>({id:sha("w"+index),createdMs:now-index*10,units:1}));
  const kept=ownerPoolOpenWavesV1(many,now);
  expect(kept.length).toBe(P.wavesKept);expect(kept[kept.length-1]!.createdMs).toBe(now);
 }finally{await server.stop();}
});

test("a pointer read that the cache did not answer is a fault and not 'no wave': the puller keeps its waves; a cache without a pointer is 'no wave'",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("fault"),settings=graphFleetSettingsV1(root,environment)!;
  const good=graphFleetClientV1(settings,{fetch:remoteCacheFetchV1(environment,{root})});
  expect(await readOwnerPoolPointerV1(good)).toBeUndefined();
  let calls=0;
  const flaky=graphFleetClientV1(settings,{fetch:(async()=>{calls++;return new Response("busy",{status:503});}) as never});
  expect(await readOwnerPoolPointerV1(flaky)).toBe("fault");expect(calls).toBeGreaterThan(0);
  const down=graphFleetClientV1(settings,{fetch:(async()=>{throw new Error("socket closed");}) as never});
  expect(await readOwnerPoolPointerV1(down)).toBe("fault");
 }finally{await server.stop();}
});

test("a failure note keeps the other pullers away until it is old or the revision changes",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("f"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})}),digest=sha("x");
  expect(await readOwnerPoolFailureV1(client,digest,"rev1")).toBeUndefined();
  expect(await publishOwnerPoolFailureV1({client,owner:"o",origin:"g1",recordDigest:digest,revision:"rev1",cause:"deterministic"})).toBe(true);
  expect((await readOwnerPoolFailureV1(client,digest,"rev1"))?.cause).toBe("deterministic");
  expect(await readOwnerPoolFailureV1(client,digest,"rev2")).toBeUndefined();
  expect(await readOwnerPoolFailureV1(client,digest,"rev1",()=>Date.now()+P.failureMaximumAgeMilliseconds+1000)).toBeUndefined();
 }finally{await server.stop();}
});

test("one pool process per machine: a live sibling is waited for, a stale pid file is taken over",async()=>{
 const root=temp("guard"),first=await acquireOwnerPoolGuardV1({root,pid:111,alive:()=>false,sleep:async()=>{}});
 expect(first.acquired).toBe(true);
 const second=await acquireOwnerPoolGuardV1({root,pid:222,alive:pid=>pid===111,sleep:async()=>{},waitMilliseconds:100,pollMilliseconds:50});
 expect(second.acquired).toBe(false);expect(second.sibling).toBe(111);
 const third=await acquireOwnerPoolGuardV1({root,pid:333,alive:()=>false,sleep:async()=>{}});
 expect(third.acquired).toBe(true);third.release();
 writeFileSync(join(root,".cache","f",P.guardFile),"garbage");
 expect((await acquireOwnerPoolGuardV1({root,pid:444,alive:()=>true,sleep:async()=>{}})).acquired).toBe(true);
});

const driverPath=join(import.meta.dir,"🧫️fixtures","🧮️driver.mjs");
async function fleetOfProcesses(children:number,count:number,burn:number){
 const {server,environment}=await world();
 try{
  const root=temp("proc"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});
  const units=unitsOf(count).map(unit=>({...unit,expectedMilliseconds:400})),manifest=manifestOf(units,"burn"+children,Date.now());
  const withSession={...manifest,session:Buffer.from(JSON.stringify({burnMilliseconds:burn}))};
  expect(await publishOwnerPoolV1({client,owner:"o",manifest:withSession})).toBe(true);
  const controller=new AbortController(),began=Date.now();let first=0,last=0,done=0;
  await runOwnerPoolV1({root,environment:{...process.env,...environment},signal:controller.signal,capacity:async()=>({children,reason:null}),once:true,driver:driverPath,priority:0,targetPollMilliseconds:100,ramp:false,pollMilliseconds:500,observe:(_id:number,line:{t:string;ok?:boolean},at:number)=>{if(line.t==='done'&&line.ok){if(first===0)first=at;last=at;done++;}}});
  const seconds=(last-first)/1000+(last-first)/Math.max(1,done-1)/1000;
  const reader=createProjectFleetPortV1({root:temp("rr"),environment,owner:"r:1"})!;
  let stored=0;for(const unit of units)if((await reader.claim(infoOf("/r",unit))).state==="stored")stored++;
  return{seconds,stored,perMinute:count/seconds*60};
 }finally{await server.stop();}
}
test("real processes: tiny synthetic owners through the real supervisor, claim and publish scale with the number of children",async()=>{
 const rows:Record<number,{seconds:number;stored:number;perMinute:number}>={};
 for(const children of[1,4,8])rows[children]=await fleetOfProcesses(children,160,150);
 for(const children of[1,4,8])expect(rows[children]!.stored).toBeGreaterThanOrEqual(144);
 if(process.env.OWNER_POOL_REPORT)writeFileSync(process.env.OWNER_POOL_REPORT,JSON.stringify(rows,null,1));
 expect(rows[4]!.perMinute/rows[1]!.perMinute).toBeGreaterThan(1.8);
 expect(rows[8]!.perMinute/rows[1]!.perMinute).toBeGreaterThan(2.4);
},180000);

test("children that finish a wave one after the other are an ordinary end, not a failure of the pool",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("finish"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});
  const units=unitsOf(24).map(unit=>({...unit,expectedMilliseconds:400})),manifest=manifestOf(units,"finish",Date.now());
  expect(await publishOwnerPoolV1({client,owner:"o",manifest:{...manifest,session:Buffer.from(JSON.stringify({burnMilliseconds:20}))}})).toBe(true);
  const store=temp("finish-store"),status=createOwnerPoolStatusV1({store,origin:"g1-finish",role:"puller"}),ended:number[]=[];
  await runOwnerPoolV1({root,environment:{...process.env,...environment},signal:new AbortController().signal,capacity:async()=>({children:5,reason:null}),once:true,driver:driverPath,priority:0,targetPollMilliseconds:100,ramp:false,pollMilliseconds:500,status,observe:(id:number,line:{t:string})=>{if(line.t==="end")ended.push(id);}});
  expect(ended.length).toBeGreaterThanOrEqual(3);
  expect(status.current().lastError).toBeNull();
 }finally{await server.stop();}
},120000);

test("a pointer read the cache does not answer is read again after the back-off before the machine reports that the cache is silent, and a record's failure codes are counted",async()=>{
 const {server,environment}=await world();
 const stamp:string[]=[];let refused=0;
 const proxy=Bun.serve({port:0,hostname:"127.0.0.1",async fetch(request){
  const url=new URL(request.url),target=new URL(url.pathname+url.search,server.url);
  if(request.method==="GET"&&/\/ac\//u.test(url.pathname)&&refused<1){refused++;stamp.push("503");return new Response("busy",{status:503});}
  return fetch(target,{method:request.method,headers:request.headers,body:request.method==="GET"||request.method==="HEAD"?undefined:await request.arrayBuffer()});
 }});
 try{
  const root=temp("pointer"),direct=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});
  const units=unitsOf(6).map(unit=>({...unit,expectedMilliseconds:400})),manifest=manifestOf(units,"pointer",Date.now());
  expect(await publishOwnerPoolV1({client:direct,owner:"o",manifest:{...manifest,session:Buffer.from(JSON.stringify({burnMilliseconds:5}))}})).toBe(true);
  const store=temp("pointer-store"),status=createOwnerPoolStatusV1({store,origin:"g1-pointer",role:"puller"}),via={...environment,SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${proxy.port}`};
  await runOwnerPoolV1({root,environment:{...process.env,...via},signal:new AbortController().signal,capacity:async()=>({children:1,reason:null}),once:true,driver:driverPath,priority:0,targetPollMilliseconds:100,ramp:false,pollMilliseconds:500,status});
  expect(refused).toBe(1);expect(status.current().lastError).toBeNull();expect(status.current().waves).toBeGreaterThanOrEqual(1);
  expect(P.pointerRetryMilliseconds.length).toBeGreaterThanOrEqual(2);
 }finally{proxy.stop(true);await server.stop();}
},120000);

test("a record that could not be stored reports the code of its true refusal, so the pool can count failures by class",async()=>{
 const seen:string[]=[];
 const claim=async():Promise<ProjectClaimV1>=>({state:"granted",digest:"d".repeat(64),why:()=>"the cache says no",code:()=>"capacity",deterministic:()=>false,publish:async()=>false,abandon:async()=>undefined});
 const stats=await pullOwnerPoolV1<Unit>({units:()=>unitsOf(1),puller:"code:1",policy:fast,slots:()=>1,claim,run:async()=>true,fail:(name,message,deterministic,code)=>seen.push(`${name}|${deterministic}|${code}|${message}`)});
 expect(stats.failed).toBe(1);expect(seen).toEqual(["owner-0000|false|capacity|the record was not stored: the cache says no"]);
});

test("the pool yields to the machine's own waiting work: every child ends within seconds of a request that waits in the gate, the children come back when it is served, a child that finishes its owner is never waited for",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("yield"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});
  const units=unitsOf(24).map(unit=>({...unit,expectedMilliseconds:60000})),manifest=manifestOf(units,"yield",Date.now());
  expect(await publishOwnerPoolV1({client,owner:"o",manifest:{...manifest,session:Buffer.from(JSON.stringify({burnMilliseconds:90000}))}})).toBe(true);
  const store=temp("yield-store"),status=createOwnerPoolStatusV1({store,origin:"g1-yield",role:"puller"}),controller=new AbortController();let waiting=false;
  const running=runOwnerPoolV1({root,environment:{...process.env,...environment},signal:controller.signal,capacity:async()=>waiting?{children:0,reason:"waiting"}:{children:3,reason:null},driver:driverPath,priority:0,targetPollMilliseconds:100,ramp:false,pollMilliseconds:500,status});
  const until=async(test:()=>boolean,limit:number)=>{const began=Date.now();while(!test()&&Date.now()-began<limit)await new Promise<void>(done=>setTimeout(done,50));return Date.now()-began;};
  await until(()=>(status.current() as {children:number}).children===3,30000);
  expect((status.current() as {children:number}).children).toBe(3);
  await new Promise<void>(done=>setTimeout(done,1500));waiting=true;
  const took=await until(()=>(status.current() as {children:number}).children===0,20000);
  expect((status.current() as {children:number}).children,"no child is left").toBe(0);expect(took,"within the yield kill limit and two looks").toBeLessThan(P.yieldKillMilliseconds+1500);
  waiting=false;await until(()=>(status.current() as {children:number}).children>0,20000);
  expect((status.current() as {children:number}).children,"the children come back").toBeGreaterThan(0);
  controller.abort();await running;
 }finally{await server.stop();}
},120000);

test("the price of one more child does not jump with the last owners: light children with two giants among them price the next owner as a light one, and a child whose bare start is near the limit is not restarted after every owner",()=>{
 const mixed=[524,530,2100,520,515,540,510,2200,525,530,522,518];
 expect(ownerPoolChildMegabytesV1(mixed)).toBeLessThan(700);
 expect(ownerPoolChildMegabytesV1([...mixed,2100,2150,2200])).toBeGreaterThan(ownerPoolChildMegabytesV1(mixed));
 expect(ownerPoolChildMegabytesV1([...Array(40).fill(2000),...Array(32).fill(500)])).toBe(575);
 expect(ownerPoolRecycleV1({rss:1334,readyRss:900,recycleMegabytes:1200})).toBe(false);
 expect(ownerPoolRecycleV1({rss:1334,readyRss:700,recycleMegabytes:1200})).toBe(true);
 expect(ownerPoolRecycleV1({rss:1100,readyRss:100,recycleMegabytes:1200})).toBe(false);
});

test("the files of a wave go to the next directory when the first one is full or over its quota, and the error of the last one is raised when none takes them",()=>{
 const written:string[]=[],removed:string[]=[],made:string[]=[];
 const full=(bad:string[])=>({mkdir:(path:string)=>{made.push(path);},write:(path:string)=>{if(bad.some(prefix=>path.startsWith(prefix)))throw Object.assign(new Error("EDQUOT: unknown error, write"),{code:"EDQUOT"});written.push(path);},remove:(path:string)=>{removed.push(path);}});
 expect(writeOwnerPoolWaveFilesV1(["/a","/b"],{"head.json":"1","session.bin":new Uint8Array(2)},full(["/a"]))).toBe("/b");
 expect(removed).toEqual(["/a"]);expect(written).toEqual(["/b/head.json","/b/session.bin"]);
 expect(()=>writeOwnerPoolWaveFilesV1(["/a","/b"],{"x":"1"},full(["/a","/b"]))).toThrow("EDQUOT");
 expect(writeOwnerPoolWaveFilesV1(["/c","/d"],{"x":"1"},full([]))).toBe("/c");
});

const aliveOf=(pid:number)=>{try{process.kill(pid,0);return true;}catch{return false;}};
const until=async(test:()=>boolean,limit:number)=>{const began=Date.now();while(!test()&&Date.now()-began<limit)await pause(50);return Date.now()-began;};

test("the id of a wave is the set of its owners: order, re-sorting and the publisher change nothing, one changed owner changes the id, another identity changes it",()=>{
 const head={version:1 as const,identity:"i",globals:{projectAuthority:"a",registryDigest:"r",commandRevision:"c",late:false}},units=unitsOf(30).map(unit=>({...unit,megabytes:100}));
 const id=ownerPoolIdV1(head,units);
 expect(ownerPoolIdV1(head,[...units].reverse())).toBe(id);
 expect(ownerPoolIdV1(head,[...units].sort((a,b)=>(b.megabytes??0)-(a.megabytes??0)||b.name.localeCompare(a.name)).map(unit=>({...unit,megabytes:9000,expectedMilliseconds:1})))).toBe(id);
 expect(ownerPoolIdV1(head,units.map((unit,index)=>index===7?{...unit,json:{...unit.json,changed:true}}:unit))).not.toBe(id);
 expect(ownerPoolIdV1({...head,identity:"j"},units)).not.toBe(id);
 expect(ownerPoolIdV1(head,units.slice(1))).not.toBe(id);
});

test("a child stages its owners in the first directory that takes a probe file: a full or quota-limited temporary directory moves the machine to the fallback instead of failing every owner",()=>{
 const asked:string[]=[],io=(bad:string[])=>({probe:(directory:string,megabytes:number)=>{asked.push(directory+":"+megabytes);if(bad.includes(directory))throw Object.assign(new Error("EDQUOT: unknown error, write"),{code:"EDQUOT"});}});
 expect(ownerPoolStagingDirectoryV1(["/tmp","/work/.cache/f/owner-pool-staging"],io([]))).toBe("/tmp");
 expect(ownerPoolStagingDirectoryV1(["/tmp","/work/.cache/f/owner-pool-staging"],io(["/tmp"]))).toBe("/work/.cache/f/owner-pool-staging");
 expect(ownerPoolStagingDirectoryV1(["/tmp","/w"],io(["/tmp","/w"]))).toBe("/w");expect(asked[0]).toBe("/tmp:"+P.stagingProbeMegabytes);
});

test("the children of a supervisor that went before are counted from the registry: live pids other than its own, dead ones dropped",()=>{
 expect(ownerPoolLiveChildrenV1([{pid:1},{pid:2},{pid:3},{pid:4}],pid=>pid!==3,new Set([4]))).toEqual([1,2]);
 const now=1e9,old=now-P.childLifetimeMilliseconds-P.drainExitMilliseconds-1;
 expect(ownerPoolLiveChildrenV1([{pid:1,startedMs:old},{pid:2,startedMs:now-1000},{pid:3}],()=>true,new Set(),now),"a row older than a child can live is a reused pid, not a child").toEqual([2,3]);
});

test("real processes: a supervisor that ends takes its children with it within seconds, and the next supervisor ends none it did not start, counts none of them as capacity it uses, and starts its whole target",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("handover"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});
  const units=unitsOf(24).map(unit=>({...unit,expectedMilliseconds:400})),manifest=manifestOf(units,"handover",Date.now());
  expect(await publishOwnerPoolV1({client,owner:"o",manifest:{...manifest,session:Buffer.from(JSON.stringify({burnMilliseconds:60000}))}})).toBe(true);
  const registry=join(root,".cache","f",P.childRegistryDirectory),first=new AbortController(),environmentOf={...process.env,...environment};
  const one=runOwnerPoolV1({root,environment:environmentOf,signal:first.signal,capacity:async()=>({children:3,reason:null}),driver:driverPath,priority:0,targetPollMilliseconds:100,ramp:false,pollMilliseconds:500});
  await until(()=>existsSync(registry)&&readdirSync(registry).filter(name=>/^\d+\.json$/u.test(name)).length>=3,30000);
  const pids=readdirSync(registry).filter(name=>/^\d+\.json$/u.test(name)).map(name=>Number(name.replace(".json","")));expect(pids.length).toBe(3);
  const record=JSON.parse(readFileSync(join(registry,pids[0]+".json"),"utf8"));expect(record.startTime,"the record names the start time of the process").toBeTruthy();expect(record.parentPid).toBe(process.pid);
  await pause(1500);first.abort();const began=Date.now();await one;
  expect(Date.now()-began,"the supervisor does not wait for its children").toBeLessThan(2500);
  await until(()=>pids.every(pid=>!aliveOf(pid)),20000);
  expect(pids.some(aliveOf),"its children ended by themselves within seconds").toBe(false);
  expect(existsSync(join(registry,P.leaseFileName)),"the lease is released").toBe(false);
  const second=new AbortController(),counts:number[]=[];
  const two=runOwnerPoolV1({root,environment:environmentOf,signal:second.signal,capacity:async({children})=>{counts.push(children);return{children:3,reason:null};},driver:driverPath,priority:0,targetPollMilliseconds:100,ramp:false,pollMilliseconds:500});
  await until(()=>counts.length>0,15000);
  expect(counts[0],"no child of the pool before is counted").toBe(0);
  await until(()=>existsSync(registry)&&readdirSync(registry).filter(name=>/^\d+\.json$/u.test(name)).length>=3,30000);
  second.abort();await two;
 }finally{await server.stop();}
},180000);

test("real processes: a starting supervisor ends the live children that an earlier supervisor of this checkout left, by the record of their wave and their start time, before it measures the capacity, and its gate counts no orphan",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("reap-start"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});
  const units=unitsOf(24).map(unit=>({...unit,expectedMilliseconds:400})),manifest=manifestOf(units,"reap-start",Date.now());
  expect(await publishOwnerPoolV1({client,owner:"o",manifest:{...manifest,session:Buffer.from(JSON.stringify({burnMilliseconds:200}))}})).toBe(true);
  const waveDir=join(root,".cache","f","owner-pool-waves","old-wave");mkdirSync(waveDir,{recursive:true});
  const orphans=[Bun.spawn(["sleep","600"]),Bun.spawn(["sleep","600"])];
  orphans.forEach((proc,index)=>writeFileSync(join(waveDir,`child-${index+1}.json`),JSON.stringify({pid:proc.pid,startTime:ownerPoolProcessStartV1(proc.pid),parentPid:2**30})));
  const counts:number[]=[],alive:boolean[][]=[],controller=new AbortController();
  const running=runOwnerPoolV1({root,environment:{...process.env,...environment},signal:controller.signal,capacity:async({children})=>{counts.push(children);alive.push(orphans.map(proc=>aliveOf(proc.pid)));return{children:1,reason:null};},driver:driverPath,priority:0,targetPollMilliseconds:100,ramp:false,pollMilliseconds:500});
  await until(()=>counts.length>0,15000);
  expect(alive[0],"both orphans were ended before the first measurement").toEqual([false,false]);expect(counts[0]).toBe(0);
  controller.abort();await running;orphans.forEach(proc=>{try{proc.kill("SIGKILL");}catch{}});
 }finally{await server.stop();}
},180000);

test("real process: a child does not claim an owner that the presence file of this wave names as stored a few minutes ago, and claims one whose line is old",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("presence"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});
  const units=unitsOf(24).map(unit=>({...unit,expectedMilliseconds:400})),manifest=manifestOf(units,"presence",Date.now());
  expect(await publishOwnerPoolV1({client,owner:"o",manifest:{...manifest,session:Buffer.from(JSON.stringify({burnMilliseconds:20}))}})).toBe(true);
  const directory=join(root,".cache","f",P.presenceDirectory);mkdirSync(directory,{recursive:true});
  const lines=units.slice(0,20).map(unit=>`${Date.now()-60000} ${unit.name}`);lines.push(`${Date.now()-2*P.presenceMaximumAgeMilliseconds} ${units[20]!.name}`);
  writeFileSync(join(directory,manifest.head.id.slice(0,16)+".txt"),lines.join("\n")+"\n");
  await runOwnerPoolV1({root,environment:{...process.env,...environment},signal:new AbortController().signal,capacity:async()=>({children:1,reason:null}),once:true,driver:driverPath,priority:0,targetPollMilliseconds:100,ramp:false,pollMilliseconds:500});
  const reader=createProjectFleetPortV1({root:temp("presence-reader"),environment,owner:"r:1"})!,stored:boolean[]=[];
  for(const unit of units)stored.push((await reader.claim(infoOf("/r",unit))).state==="stored");
  expect(stored.slice(0,20).some(Boolean),"the owners the presence file names are never claimed").toBe(false);
  expect(stored.slice(20).every(Boolean),"the old line and the rest are computed").toBe(true);
 }finally{await server.stop();}
},120000);

test("the last known wave outlives the pointer: a wave stays while the pointer lists none, goes only when others are listed, a declined one goes unlisted, a worked one never goes; a stale wave rests ten minutes",()=>{
 const base={listed:false,anyListed:false,children:0,declined:false};
 expect(ownerPoolKeepWaveV1(base),"nothing listed: the last known wave stays").toBe(true);
 expect(ownerPoolKeepWaveV1({...base,anyListed:true}),"others listed, this one not: it goes").toBe(false);
 expect(ownerPoolKeepWaveV1({...base,anyListed:true,children:2}),"a child works on it").toBe(true);
 expect(ownerPoolKeepWaveV1({...base,listed:true,anyListed:true})).toBe(true);
 expect(ownerPoolKeepWaveV1({...base,declined:true}),"declined and unlisted: it goes").toBe(false);
 expect(ownerPoolReopenMillisecondsV1(1000,1000+P.manifestMaximumAgeMilliseconds)).toBe(P.reopenMilliseconds);
 expect(ownerPoolReopenMillisecondsV1(1000,2000+P.manifestMaximumAgeMilliseconds)).toBe(P.staleReopenMilliseconds);
});

test("real process: a pool whose pointer has no wave (expired or never republished) works the wave it kept on disk, and does not delete it when it ends",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("kept"),units=unitsOf(12).map(unit=>({...unit,expectedMilliseconds:400})),manifest=manifestOf(units,"kept",Date.now()-2*P.manifestMaximumAgeMilliseconds);
  const directory=join(root,".cache","f","owner-pool-waves",manifest.head.id.slice(0,16));mkdirSync(directory,{recursive:true});
  writeFileSync(join(directory,"head.json"),JSON.stringify(manifest.head));writeFileSync(join(directory,"session.bin"),Buffer.from(JSON.stringify({burnMilliseconds:20})));writeFileSync(join(directory,"units.json"),JSON.stringify(units));
  const abort=new AbortController(),timer=setTimeout(()=>abort.abort(),20000);
  await runOwnerPoolV1({root,environment:{...process.env,...environment},signal:abort.signal,capacity:async()=>({children:2,reason:null}),once:true,driver:driverPath,priority:0,targetPollMilliseconds:100,ramp:false,pollMilliseconds:500});
  clearTimeout(timer);
  const reader=createProjectFleetPortV1({root:temp("kept-reader"),environment,owner:"r:1"})!,stored:boolean[]=[];
  for(const unit of units)stored.push((await reader.claim(infoOf("/r",unit))).state==="stored");
  expect(stored.every(Boolean),"every owner of the kept wave is stored").toBe(true);
  expect(existsSync(directory),"the kept wave is still on disk").toBe(true);
 }finally{await server.stop();}
},120000);
