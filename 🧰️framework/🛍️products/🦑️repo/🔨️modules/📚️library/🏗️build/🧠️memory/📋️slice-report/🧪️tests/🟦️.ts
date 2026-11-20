import{afterAll,expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,statSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{basename,dirname,join}from"node:path";
import{pathToFileURL}from"node:url";
import{spawn,spawnSync}from"node:child_process";
import{createHash}from"node:crypto";
import Ajv from"ajv";
import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{SLICE_REPORT_POLICY_V1 as P,measureSliceProcessesV1,parseProcessTreeV1,sharedBytesV1,buildSliceReportV1,registerSliceReportSourceV1,startSliceProgressV1,writeSliceReportV1}from"../🟨️.mjs";
import{READY_POLICY_V1,READY_STAGES_V1,createReadyPortsV1,runReadyStageV1}from"../../../🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts";
import{READY_CONTAINMENT_POLICY_V1,readyOwnerTokenV1}from"../../../🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧹️containment/🟦️.ts";
import{ownedSliceReportSinkV1,ownedSliceReportV1,prepareOwnedSliceReportV1}from"../🏷️owner/🟦️.ts";
import ownedFixture from"./owned.json"with{type:"json"};
import collectionFixture from"./🧺️collections.json"with{type:"json"};
import{captureObservedSourceInputsV1,observationCollectionsV1,recordClosureFactV1}from"../../../👁️observation/🟨️.mjs";

import{collectSliceCollectionsV1,describeCollectionV1,estimateCollectionBytesV1,registerSliceCollectionsV1}from"../🟨️.mjs";
/** 📋️ The slice memory report: a machine nobody has a shell on tells what its slice worker holds, bounded, in the store next to ready.json. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`slice-report-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

test("finished scopes release diagnostic WeakRefs even when no report samples them",()=>{
 const vector=JSON.parse(readFileSync(join(import.meta.dir,"weak-collections.json"),"utf8"));
 const probe=spawnSync(process.execPath,[join(import.meta.dir,"weak-collections.mjs")],{encoding:"utf8"});
 const oracle=spawnSync("python3",["-c","import gc,sys,weakref\nclass Scope: pass\nscopes=weakref.WeakSet()\nfor _ in range(int(sys.argv[1])): scopes.add(Scope())\ngc.collect()\nprint(len(scopes))",String(vector.shortLivedScopes)],{encoding:"utf8"});
 expect(probe.status).toBe(0);expect(oracle.status).toBe(0);
 expect(JSON.parse(probe.stdout).retained).toBeLessThanOrEqual(Number(oracle.stdout)+vector.maximumUnrelatedReferences);
});

test("mixed observation scopes report the largest map and the full entry count",async()=>{
 const scopes=()=>observationCollectionsV1().find(row=>row.name.startsWith("observation:scopes "));
 const previous=scopes()?.name.match(/largest of (\d+) maps, (\d+) entries total/u),database=new Database(":memory:");
 const unregister=registerSliceCollectionsV1("observation-fixture",observationCollectionsV1);
 const spill={builder:()=>({add(){},finish:()=>Object.assign([],{rows:()=>[]})})};
 try{
  const apply=async(index:number):Promise<void>=>{
   if(index<collectionFixture.length){
    const {entries,width,spilled}=collectionFixture[index]!;
    await captureObservedSourceInputsV1(async()=>{
     for(let row=0;row<entries;row++)recordClosureFactV1("fixture/"+index+"/"+row,"a".repeat(width));
     await apply(index+1);
    },undefined,false,spilled?spill:undefined);
    return;
   }
   const row=scopes()!;
   const sample=row.sample?.()??[];
   const collected=collectSliceCollectionsV1({...P,collectionsTop:100}).find(item=>item.name===row.name);
   const oracle=database.query<{entries:number;total:number;maps:number;sampleCount:number;estimatedBytes:number},[string,number,string]>(`
    with sizes as (select max(value->>'entries') entries,sum(value->>'entries') total,count(*) maps from json_each(?1) where value->>'spilled' is null),
    sampled as (select avg(length(value)) width from json_each(?3))
    select entries,total,maps,min(entries,?2) sampleCount,round(entries*width) estimatedBytes from sizes,sampled
   `).get(JSON.stringify(collectionFixture),P.collectionSample,JSON.stringify(sample))!;
   const {entries,total,maps,sampleCount,estimatedBytes}=oracle,scopeCount=Number(previous?.[1]??0)+maps;
   expect(row.entries).toBe(entries);
   expect(row.name).toBe("observation:scopes (largest of "+scopeCount+" maps, "+(total+Number(previous?.[2]??0))+" entries total)");
   expect(sample).toHaveLength(sampleCount);
   expect(sample.every(item=>Array.isArray(item)&&item[1]?.path.startsWith("fixture/1/"))).toBe(true);
   expect(collected).toEqual({name:row.name,entries,estimatedBytes});
  };
  await apply(0);
 }finally{unregister();database.close();}
});

test("a report is closed by its schema, names the phase, memory, heap types and every source, and a source that throws is named and does not stop it",()=>{
 registerSliceReportSourceV1("owners",()=>({jobs:703,ledger:{present:3,fetched:0,here:10}}));
 registerSliceReportSourceV1("broken",()=>{throw new Error("nope");});
 const report=buildSliceReportV1({reason:"interval",heap:()=>({heapSize:104857600,extraMemorySize:52428800,objectCount:12345,objectTypeCounts:{string:100,Object:50,Array:20}})});
 expect(validateJsonSchemaSubset({$ref:"#/definitions/Report"},report,schema)).toEqual([]);
 expect((report.sources as Record<string,unknown>).owners).toEqual({jobs:703,ledger:{present:3,fetched:0,here:10}});
 expect(((report.sources as Record<string,{error:string}>).broken).error).toBe("nope");
 expect((report.heap as {types:{type:string}[]}).types[0]!.type).toBe("string");
});

test("a report never exceeds fileBytes: the largest sources are dropped first and named",()=>{
 registerSliceReportSourceV1("huge",()=>({names:Array.from({length:20000},(_all,index)=>"owner-"+index)}));
 const root=temp("bound");writeSliceReportV1(root,{reason:"plan-stop"});
 const text=readFileSync(join(root,P.directory,P.name),"utf8"),report=JSON.parse(text);
 expect(text.length).toBeLessThanOrEqual(P.fileBytes);
 expect(report.dropped).toContain("huge");
 expect(report.sources.owners).toBeDefined();
});

test("real process: a worker that reads writes the report while it works, the moment its plan stops and at its end",()=>{
 const root=temp("real"),script=join(root,"worker.mjs");
 writeFileSync(script,`import{registerSliceReportSourceV1,startSliceReportV1,noteSliceReportPhaseV1,noteSliceReportStopV1}from${JSON.stringify(join(import.meta.dir,"..","🟨️.mjs"))};
registerSliceReportSourceV1("owners",()=>({jobs:5}));const r=startSliceReportV1({root:process.argv[2],footprint:{self:async()=>200*1048576,tree:async()=>300*1048576},intervalMilliseconds:50});
noteSliceReportPhaseV1({phase:"inference",done:3,total:5});await new Promise(done=>setTimeout(done,200));
noteSliceReportStopV1({stage:"inference",operation:"inference",usedMegabytes:9933,budgetMegabytes:6553});r.now("plan-stop");
import{readFileSync}from"node:fs";console.log(JSON.parse(readFileSync(process.argv[2]+"/${P.directory}/${P.name}","utf8")).reason);r.stop();`);
 const run=spawnSync(process.execPath,[script,root],{encoding:"utf8"});
 expect(run.stdout.trim()).toBe("plan-stop");
 const last=JSON.parse(readFileSync(join(root,P.directory,P.name),"utf8"));
 expect(last.reason).toBe("end");expect(last.phase).toEqual({name:"inference",done:3,total:5});expect(last.stop.usedMegabytes).toBe(9933);
 expect(validateJsonSchemaSubset({$ref:"#/definitions/Report"},last,schema)).toEqual([]);
 expect(last.footprintMegabytes).toBe(200);expect(last.guard.measure).toBe("heapUsed+external");
 expect(Math.abs(last.guard.megabytes-(last.memory.heapUsed+last.memory.external))).toBeLessThan(2);
 if(process.platform!=="win32"){expect(last.processes.self.rssMegabytes).toBeGreaterThan(0);expect(last.processes.count).toBe(0);expect(last.treeFootprintMegabytes).toBeUndefined();}
});

test("readiness mirrors its stage owner while a concurrent side reader retains the generic report",async()=>{
 const root=temp("owners"),store=temp("store"),script=join(root,"reader.mjs"),fixture=ownedFixture;
 const owner=readyOwnerTokenV1(root,fixture.stage,fixture.startMs),other=readyOwnerTokenV1(root,fixture.stage,fixture.startMs+1);
 const policy=schema.definitions.Owned.const,ownerEnv=READY_CONTAINMENT_POLICY_V1.ownerEnvironment;
 const fileOf=(id:string)=>join(root,...policy.directory,createHash("sha256").update(id).digest("hex").slice(0,policy.slotHexCharacters),P.name);
 const ownedFile=fileOf(owner),genericFile=join(root,P.directory,P.name),mirrorFile=join(store,"v1","origins","g1-test",P.name);
 const grant=prepareOwnedSliceReportV1(root,owner)!;
 writeFileSync(script,`import{noteSliceReportPhaseV1,writeSliceReportV1}from${JSON.stringify(pathToFileURL(join(import.meta.dir,"..","🟨️.mjs")).href)};
import{ownedSliceReportSinkV1}from${JSON.stringify(pathToFileURL(join(import.meta.dir,"..","🏷️owner/🟦️.ts")).href)};
noteSliceReportPhaseV1({phase:process.argv[3],done:1,total:3});
const scope=ownedSliceReportSinkV1(process.argv[2]),write=()=>{const text=writeSliceReportV1(process.argv[2],{reason:"interval"});if(text!==undefined)scope?.(text);};
write();process.stdout.write("written");await new Promise(done=>process.stdin.once("data",data=>{if(String(data)==="again")write();done();}));`);
 const start=(phase:string,token:string,key:string)=>{
  const child=spawn(process.execPath,[script,root,phase],{env:{...process.env,[ownerEnv]:token,[policy.grantEnvironment]:key},stdio:["pipe","pipe","inherit"]});
  const written=new Promise<void>((resolve,reject)=>{child.stdout.once("data",()=>resolve());child.once("error",reject);child.once("exit",()=>reject(new Error("reporter exited before writing")));});
  const exited=new Promise<void>(done=>child.once("exit",()=>done()));
  return{child,written,exited};
 };
 const stage=start(fixture.ownedPhase,owner,grant);
 let side:ReturnType<typeof start>|undefined;
 try{
  await stage.written;side=start(fixture.sidePhase,owner,"");await side.written;
  const ports=createReadyPortsV1({root,environment:{SEMIO_FLEET_STORE_ROOT:store},origin:"g1-test",group:undefined,say:()=>undefined});
  ports.mirror!(fixture.stage,["reading"],owner);
  const generic=JSON.parse(readFileSync(genericFile,"utf8")),mirror=JSON.parse(readFileSync(mirrorFile,"utf8"));
  expect(generic.phase.name).toBe(fixture.sidePhase);
  expect(mirror.phase.name).toBe(fixture.ownedPhase);
  expect(mirror.readyOwner).toBe(owner);
  expect(existsSync(fileOf(other))).toBe(false);
  expect(new Ajv({strict:false}).compile({...schema,$ref:"#/definitions/Report"})(mirror)).toBe(true);
  utimesSync(mirrorFile,fixture.startMs/1000,fixture.startMs/1000);
  const mirroredAt=statSync(mirrorFile).mtimeMs;
  ports.mirror!(fixture.stage,["repeat"],owner);
  expect(statSync(mirrorFile).mtimeMs).toBe(mirroredAt);
  expect(prepareOwnedSliceReportV1(root,owner)).toBe(grant);
  expect(JSON.parse(readFileSync(ownedFile,"utf8"))).toEqual(mirror);
  ports.mirror!(fixture.stage+"-failed",["unowned log"]);
  writeFileSync(ownedFile,JSON.stringify({...mirror,readyOwner:other,phase:generic.phase}));
  ports.mirror!(fixture.stage,["reading"],owner);
  expect(JSON.parse(readFileSync(mirrorFile,"utf8"))).toEqual(mirror);
  const nextGrant=prepareOwnedSliceReportV1(root,other)!;
  ports.mirror!(fixture.stage,["starting again"],other);
  expect(existsSync(mirrorFile)).toBe(false);
  stage.child.stdin.end("again");await stage.exited;
  expect(existsSync(ownedFile)).toBe(false);
  side.child.stdin.end("again");await side.exited;
  expect(existsSync(fileOf(other))).toBe(false);
  ownedSliceReportSinkV1(root,{[ownerEnv]:other,[policy.grantEnvironment]:nextGrant})!(JSON.stringify(generic));
  ports.mirror!(fixture.stage,["started"],other);
  expect(JSON.parse(readFileSync(mirrorFile,"utf8")).readyOwner).toBe(other);
  expect(readdirSync(join(root,...policy.directory))).toEqual([basename(dirname(fileOf(other)))]);
 }finally{
  if(!stage.child.stdin.destroyed)stage.child.stdin.end("stop");
  if(side!==undefined&&!side.child.stdin.destroyed)side.child.stdin.end("stop");
  await Promise.all([stage.exited,side?.exited]);
 }
});

test("an adopted stage mirrors the report from the recorded attempt before dropping its running record",async()=>{
 const root=temp("adopted"),store=temp("adopted-store"),stage=READY_STAGES_V1.find(row=>row.name===ownedFixture.stage)!;
 const directory=join(root,...READY_POLICY_V1.directory),startedMs=Date.now()-50,owner=readyOwnerTokenV1(root,stage.name,startedMs);
 const log=join(directory,"workspace.log"),exitFile=join(directory,"workspace.exit");
 mkdirSync(directory,{recursive:true});
 writeFileSync(log,"real owner\n");
 writeFileSync(join(directory,READY_POLICY_V1.inflightName),JSON.stringify({stage:stage.name,pid:process.pid,startedMs,exitFile,log}));
 const old=readyOwnerTokenV1(root,stage.name,startedMs-1);prepareOwnedSliceReportV1(root,old);
 const ports=createReadyPortsV1({root,environment:{SEMIO_FLEET_STORE_ROOT:store},origin:"g1-test",group:undefined,say:()=>undefined});
 const running=runReadyStageV1({root,environment:{},stage,signal:new AbortController().signal,jobs:1,onLine:()=>undefined,mirror:ports.mirror!});
 const file=ownedSliceReportV1(root,owner)!.file;
 for(let count=0;count<100&&!existsSync(dirname(file));count++)await Bun.sleep(10);
 writeFileSync(file,JSON.stringify({...buildSliceReportV1({reason:"end"}),readyOwner:owner}));
 writeFileSync(exitFile,JSON.stringify({exit:0,signal:null,spawnFailed:false}));
 expect(await running).toMatchObject({exit:0,startedMs,detached:false});
 expect(existsSync(dirname(ownedSliceReportV1(root,old)!.file))).toBe(false);
 const mirrored=JSON.parse(readFileSync(join(store,"v1","origins","g1-test",P.name),"utf8"));
 expect(mirrored.readyOwner).toBe(owner);
});

test("an unowned log mirror leaves the generic report local and preserves the final refusal line",()=>{
 const root=temp("root"),store=temp("store");mkdirSync(join(root,P.directory),{recursive:true});
 writeFileSync(join(root,P.directory,P.name),JSON.stringify({version:1,marker:"generic"}));
 const ports=createReadyPortsV1({root,environment:{SEMIO_FLEET_STORE_ROOT:store},origin:"g1-test",group:undefined,say:()=>undefined});
 const long="The project graph could not be read on this machine yet: "+"x".repeat(900)+" END";
 ports.mirror!("workspace-failed",[long]);
 const origin=join(store,"v1","origins","g1-test");
 expect(existsSync(join(origin,P.name))).toBe(false);
 expect(readFileSync(join(origin,"ready-log-workspace-failed.txt"),"utf8")).toContain(long);
});

test("a structure shared by many entries is charged once, not once per referrer",()=>{
 const members=Array.from({length:150},(_all,at)=>Array.from({length:500},(_item,item)=>"m"+at+"-"+item));
 const entries=Array.from({length:120},()=>({sidecar:{facts:members.map((_member,at)=>({at})),members:members.slice()}}));
 const seen=new Set();let total=0;for(const entry of entries)total+=8+sharedBytesV1(entry.sidecar,seen);
 const inner=150*(16+500*8),perEntry=16+(16+(16+150*(8+32)))+(16+(16+150*8));
 expect(total).toBe(120*(8+perEntry)+inner);
 expect(total).toBeLessThan(2*1048576);
 expect(estimateCollectionBytesV1(120,entries.slice(0,16).map(entry=>entry.sidecar))).toBeGreaterThan(total*20);
 expect(sharedBytesV1(new Set([1,2,3]))).toBe(16+3*8);expect(sharedBytesV1(new Map([[1,{a:1}]]))).toBe(16+16+(16+16));
 const cycle:{self?:unknown}={};cycle.self=cycle;expect(sharedBytesV1(cycle)).toBe(16+16);
});

test("a process table names the descendants of a process by role with their resident and virtual sizes and leaves everything else out",()=>{
 const table=[" 100     1   2048   4096 /usr/bin/bun /x/👷️worker.ts --flag","\n 101   100  10240  20480 /usr/bin/bun /x/🏭️walk-farm.mjs 1","\n 102   101   5120   8192 /usr/bin/node child.js","\n 103     1  99999  99999 /usr/bin/other","\n 104   100    900   1000 ps -A -o pid=,ppid=,rss=,vsz=,command=","\n garbage line"].join("");
 const tree=parseProcessTreeV1(table,100);
 expect(tree).toEqual({self:{rssMegabytes:2,vszMegabytes:4},count:2,rssMegabytes:15,children:[{pid:101,role:"🏭️walk-farm.mjs",rssMegabytes:10,vszMegabytes:20},{pid:102,role:"child.js",rssMegabytes:5,vszMegabytes:8}]});
 expect(parseProcessTreeV1(table,100,1).children.length).toBe(1);expect(parseProcessTreeV1(table,555)).toEqual({count:0,rssMegabytes:0,children:[]});
});

test("real processes: the children of this process are found with their roles and the resident size that ps reports for each of them",async()=>{
 if(process.platform==="win32")return;
 const root=temp("census"),script=join(root,"probe-kid.mjs");writeFileSync(script,"const keep=new Uint8Array(24*1048576).fill(1);setTimeout(()=>keep.length,30000);");
 const kids=[spawn(process.execPath,[script],{stdio:"ignore"}),spawn(process.execPath,[script],{stdio:"ignore"})];
 try{
  await new Promise(resolve=>setTimeout(resolve,1500));
  const row=await measureSliceProcessesV1({footprint:{self:async()=>7*1048576}});
  expect(row.footprintMegabytes).toBe(7);expect(row.processes!.count).toBeGreaterThanOrEqual(2);
  const found=row.processes!.children.filter(child=>child.role==="probe-kid.mjs");
  expect(found.map(child=>child.pid).sort()).toEqual(kids.map(kid=>kid.pid!).sort());
  for(const child of found){
   const oracle=Number(spawnSync("ps",["-o","rss=","-p",String(child.pid)],{encoding:"utf8"}).stdout.trim())/1024;
   expect(child.rssMegabytes).toBeGreaterThan(20);expect(Math.abs(child.rssMegabytes-oracle)).toBeLessThan(Math.max(8,oracle*0.25));
  }
  expect(row.treeFootprintMegabytes).toBeUndefined();
  expect((await measureSliceProcessesV1({footprint:{tree:async()=>11*1048576}})).treeFootprintMegabytes).toBe(11);
 }finally{for(const kid of kids)kid.kill();}
},60000);

/** 🧱️ A heap snapshot of a 500 MB heap held 4 GB of resident pages for the rest of the process (measured: 613 MB to 4 780 MB, not returned by a collection) and its summary made the plan guard (heapUsed plus external) see 10 GB. The report must never walk the heap, even where the caller still passes a snapshot function. */
test("real process: a report that runs for 24 seconds beside a 560 MB heap keeps the guard number under 2 GB and the resident set under 3 GB",()=>{
 const root=temp("noheap"),script=join(root,"worker.mjs");
 writeFileSync(script,`import{startSliceReportV1}from${JSON.stringify(join(import.meta.dir,"..","🟨️.mjs"))};
const MB=1048576,keep=[];while(process.memoryUsage().heapUsed/MB<560){for(let at=0;at<200;at++)keep.push({id:at,name:"p"+Math.random(),rows:["a"+at,"b"+at,{x:at}]});}
const report=startSliceReportV1({root:process.argv[2],snapshot:()=>globalThis.Bun.generateHeapSnapshot(),intervalMilliseconds:250});
let guard=0,rss=0;const probe=setInterval(()=>{const usage=process.memoryUsage();guard=Math.max(guard,(usage.heapUsed+usage.external)/MB);rss=Math.max(rss,usage.rss/MB);},100);
await new Promise(done=>setTimeout(done,24000));clearInterval(probe);report.stop();
const usage=process.memoryUsage();console.log(JSON.stringify({guard:Math.max(guard,(usage.heapUsed+usage.external)/MB),rss:Math.max(rss,usage.rss/MB),kept:keep.length}));`);
 const run=spawnSync(process.execPath,[script,root],{encoding:"utf8",timeout:150000}),row=JSON.parse(run.stdout.trim().split("\n").pop()!);
 expect(row.kept).toBeGreaterThan(1000);expect(row.guard).toBeLessThan(2000);expect(row.rss).toBeLessThan(3000);
},170000);

test("collections are estimated by entries times the mean size of 16 sampled entries, sorted by bytes, at most 20 and bounded to 32 KiB, and a broken source does not stop the report",()=>{
 expect(estimateCollectionBytesV1(1000,["abcd","abcd"])).toBe(8000);
 expect(estimateCollectionBytesV1(10,[{a:1}])).toBe(70);
 expect(estimateCollectionBytesV1(5,[])).toBe(0);
 const big=new Map<number,string>(),small=new Map<number,string>();for(let at=0;at<3000;at++)big.set(at,"x".repeat(500));for(let at=0;at<10;at++)small.set(at,"y");
 registerSliceCollectionsV1("test-many",()=>[describeCollectionV1("big",big),describeCollectionV1("small",small),{name:"declared",entries:1,bytes:5}]);
 registerSliceCollectionsV1("test-broken",()=>{throw new Error("nope");});
 registerSliceCollectionsV1("test-flood",()=>Array.from({length:400},(_all,at)=>({name:"flood-"+at+"-".repeat(70),entries:at,bytes:at})));
 const rows=collectSliceCollectionsV1();
 expect(rows.length).toBeLessThanOrEqual(P.collectionsTop);
 expect(JSON.stringify(rows).length).toBeLessThanOrEqual(P.collectionsBytes);
 expect(rows[0]!.name).toBe("big");expect(rows[0]!.entries).toBe(3000);expect(Math.abs(rows[0]!.estimatedBytes/(3000*JSON.stringify([0,"x".repeat(500)]).length)-1)).toBeLessThan(0.01);
 for(let at=1;at<rows.length;at++)expect(rows[at-1]!.estimatedBytes).toBeGreaterThanOrEqual(rows[at]!.estimatedBytes);
 const report=buildSliceReportV1({reason:"interval"});
 expect(validateJsonSchemaSubset({$ref:"#/definitions/Report"},report,schema)).toEqual([]);
 expect((report.collections as {name:string}[])[0]!.name).toBe("big");
});

/** 💓️ A worker that takes owners tells the count that rose, once per count, and nothing while the source is absent or the count stands. */
test("the take counters are told as progress while they rise and not while they stand",async()=>{
 let ledger:{done:number;total:number}|undefined,throwing=false;const told:{phase:string;done:number;total:number}[]=[];
 const stop=startSliceProgressV1({emit:event=>told.push(event),read:()=>{if(throwing)throw new Error("nope");return ledger===undefined?undefined:{ledger};},milliseconds:10});
 try{
  await new Promise(resolve=>setTimeout(resolve,60));expect(told).toEqual([]);
  ledger={done:3,total:478};await new Promise(resolve=>setTimeout(resolve,60));
  ledger={done:9,total:478};await new Promise(resolve=>setTimeout(resolve,60));
  expect(told).toEqual([{phase:"taken",done:3,total:478},{phase:"taken",done:9,total:478}]);
  ledger={done:9,total:0};await new Promise(resolve=>setTimeout(resolve,40));expect(told.length).toBe(2);
 }finally{stop();}
 const after=told.length;ledger={done:11,total:478};await new Promise(resolve=>setTimeout(resolve,40));expect(told.length,"stopped").toBe(after);
 expect(P.progressMilliseconds).toBeGreaterThan(0);expect(P.progressMilliseconds).toBeLessThanOrEqual(10000);
});
