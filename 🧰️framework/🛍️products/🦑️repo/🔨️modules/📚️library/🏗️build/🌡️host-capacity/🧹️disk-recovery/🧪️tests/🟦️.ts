/** 🧪️ Contract: the machine has one disk budget derived from its volume, every writer of the build system is registered with a collector and a priority, the gate frees derivable artefacts first and the least recently used cache objects last, before it refuses and until the command's declared need fits, and it refuses with the largest remaining consumers and the exact command to free each; the writers that had no bound have one. */
import {afterAll,expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import verdictCases from "./🧬️verdict-bounds.json" with {type:"json"};
import collectorStarts from "./🧰️collector-starts.json" with {type:"json"};
import consumerCensus from "./📊️consumer-census.json" with {type:"json"};
import {mkdirSync,mkdtempSync,existsSync,rmSync,symlinkSync,utimesSync,writeFileSync,readdirSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {renderRepoErrorV1,isRepoErrorV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import readySchema from "../../../🌐️remote-execution/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json" with {type:"json"};
import family from "../../../📏️workspace-scale/🧬️schema/🔣️.json" with {type:"json"};
import ledgerPolicy from "../../../../⚡️caching/💾️storage/🧾️ledger/🔣️policy.json" with {type:"json"};
import {createCasGuardV1,resetCasGuardsV1} from "../../../🗄️cas/🚦️guard/🟦️.ts";
import type {CasCollectionV1} from "../../../🗄️cas/🧹️collector/🟦️.ts";
import {casPartitionBudgetV1,casPartitionPolicyForV1,decideCasEnforcementV1} from "../../../🗄️cas/🌡️partition/🟦️.ts";
import {CAS_BUDGET_V1,CAS_POLICY_V1,CAS_RESERVE_FACTOR_V1} from "../../../🗄️cas/🧬️schema/🟦️.ts";
import {admitBuildDiskV1,expectedDiskGrowthBytesV1} from "../../💽️disk/🟦️.ts";
import {DISK_WRITERS_V1,consumersOfV1,judgeDiskV1,machineDiskBudgetV1,recoverDiskV1,recoverNeedV1,refuseDiskRecoveryV1,requireDiskWritersV1,type DiskFactsV1,type DiskWriterRowV1,type DiskWriterV1} from "../🟦️.ts";
import {applyBoundV1,DISK_BOUNDS_V1,finishedReadVerdictV1,selectExpiredV1,selectRotatedV1,selectTemporaryV1} from "../🧺️bounds/🟦️.ts";
import {collectMachineBoundsV1,realDiskWritersV1} from "../🔌️real/🟦️.ts";
import {CACHE_SIZE_FILE_V1,readCacheLedgerV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {readMachinePolicyV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts";
import {cacheRunnerPortsV1} from "../../../🧹️cache-gc/🏃️runner/🟦️.ts";
import {runCacheLedgerCommandV1} from "../../../🧹️cache-gc/⌨️command/🟦️.ts";

const GIB=1024**3,roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"disk-recovery-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const stub:CasCollectionV1={status:"collected",partial:false,dryRun:false,scanned:0,evicted:0,freedBytes:0,busy:0,skipped:0,totals:{totalBytes:0,protectedBytes:0,candidateBytes:0,plannedBytes:0,keptBytes:0,shortfallBytes:0,needBytes:0},byReason:{age:0,area:0,host:0}};

class Volume{free:number;constructor(readonly total:number,free:number,readonly partition="/"){this.free=free;}facts=():DiskFactsV1=>({partition:this.partition,totalBytes:this.total,freeBytes:this.free});}
const row=(id:string,priority:number,patch:Partial<DiskWriterRowV1>={}):DiskWriterRowV1=>({id,priority,class:"derivable",relief:"host",root:"/"+id,collector:"age-prune",command:"repo cache gc --area="+id,bound:{en:"b",de:"b"},...patch});
const fake=(volume:Volume,id:string,priority:number,holds:number,log:string[],patch:Partial<DiskWriterRowV1>={},fail=false):DiskWriterV1=>{
 let left=holds;
 return{row:row(id,priority,patch),root:"/"+id,measure:()=>left,collect:async()=>{log.push(id);if(fail)throw Error("boom "+id);const freed=left;left=0;volume.free+=freed;return freed;}};
};

test("the registry is closed: unique ids and priorities, a command and a bound in both languages for every writer, manual writers last, derivable artefacts before the least recently used cache objects before the user's overwritten files",()=>{
 const rows=requireDiskWritersV1();
 expect(rows.length).toBeGreaterThanOrEqual(12);
 const priority=(id:string)=>rows.find(item=>item.id===id)!.priority;
 expect(priority("cargo-incremental")).toBeLessThan(priority("leaked-staging"));
 expect(priority("leaked-images")).toBeLessThan(priority("cas-lru"));
 expect(priority("cas-lru")).toBeLessThan(priority("replica-lost"));
 expect(priority("replica-lost")).toBeLessThan(priority("image-slack"));
 for(const item of rows)expect(item.command.startsWith("repo ")||item.command.startsWith("hdiutil ")||item.command.startsWith("cd ")).toBe(true);
 expect(()=>requireDiskWritersV1([row("a",1),row("a",2)])).toThrow();
 expect(()=>requireDiskWritersV1([row("a",1,{collector:"manual"}),row("b",2)])).toThrow();
});

test("every command the registry names is a command that exists: a cache area that is in the ledger, `repo cache gc`, `repo cache cas gc`, the doctor repair or the manual image removal",()=>{
 const areas=new Set((ledgerPolicy as {areas:{id:string}[]}).areas.map(area=>area.id));
 for(const item of DISK_WRITERS_V1){
  const named=/--area=([a-z0-9,-]+)/u.exec(item.command)?.[1];
  if(named)for(const id of named.split(","))expect(areas.has(id)).toBe(true);
  expect(["repo cache gc","repo cache cas gc","repo doctor --fix --only=host"].some(command=>item.command.startsWith(command))||["manual","image-reset","store-compaction"].includes(item.collector)).toBe(true);
 }
});

test("the budget is derived from the scale family and the declared stage needs: the largest need covers every stage of the ready table and a Cargo build, the reserve is the largest need times the safety factor of the family",()=>{
 const needs=Object.values((readySchema as unknown as {definitions:{Policy:{const:{diskRequirements:Record<string,{megabytes:number}>}}}}).definitions.Policy.const.diskRequirements).map(need=>need.megabytes*1024*1024);
 for(const need of needs)expect(CAS_BUDGET_V1.largestNeedBytes).toBeGreaterThanOrEqual(need);
 expect(CAS_BUDGET_V1.largestNeedBytes).toBeGreaterThanOrEqual(expectedDiskGrowthBytesV1({},"cargo"));
 expect(CAS_RESERVE_FACTOR_V1).toBe(family.definitions.Family.const.safetyFactor);
 const budget=casPartitionBudgetV1(256*GIB);
 expect(budget.reserveBytes).toBe(CAS_BUDGET_V1.largestNeedBytes*CAS_RESERVE_FACTOR_V1);
 expect(budget.cushionBytes).toBe(Math.ceil(CAS_BUDGET_V1.largestNeedBytes/CAS_BUDGET_V1.cushionDivisor));
});

test("a disk of 256 GB and a disk of 4 TB keep the same bytes free for a stage, no share of the volume strands the small one, and a volume too small to derive keeps the fixed shares",()=>{
 const small=casPartitionBudgetV1(238*GIB),large=casPartitionBudgetV1(3725*GIB);
 expect(small.reserveBytes).toBe(large.reserveBytes);expect(small.cushionBytes).toBe(large.cushionBytes);
 const fixedSmall=1-CAS_POLICY_V1.partition.criticalUsedFraction;
 expect(small.cushionBytes/(238*GIB)).toBeLessThan(fixedSmall);
 expect(casPartitionBudgetV1(10*GIB).derivable).toBe(false);
 expect(casPartitionPolicyForV1(CAS_POLICY_V1,{totalBytes:10*GIB}).partition).toBe(CAS_POLICY_V1.partition);
 expect(casPartitionPolicyForV1(CAS_POLICY_V1,undefined)).toBe(CAS_POLICY_V1);
 const derived=casPartitionPolicyForV1(CAS_POLICY_V1,{totalBytes:238*GIB}).partition;
 expect(derived.lowUsedFraction).toBeLessThan(derived.highUsedFraction);expect(derived.highUsedFraction).toBeLessThan(derived.criticalUsedFraction);
});

test("the wall of g1-m01: at 97.1% used with 6.77 GiB free the fixed shares refused the command, the derived budget admits it, and a command that declares more than fits with its cushion still waits",()=>{
 const partition={totalBytes:Math.round(6.77*GIB/0.029),freeBytes:Math.round(6.77*GIB)};
 const fixed=decideCasEnforcementV1({policy:CAS_POLICY_V1,storeBytes:0,incomingBytes:0,partition,collected:true});
 expect(fixed.action).toBe("refuse");
 const derived=decideCasEnforcementV1({policy:casPartitionPolicyForV1(CAS_POLICY_V1,partition),storeBytes:0,incomingBytes:0,partition,collected:true});
 expect(derived.action).not.toBe("refuse");
 const cargo=decideCasEnforcementV1({policy:casPartitionPolicyForV1(CAS_POLICY_V1,partition),storeBytes:0,incomingBytes:6*GIB,partition,collected:true});
 expect(cargo.action).toBe("refuse");
});

test("the guard of the store judges by the derived budget: at 97.1% with 6.77 GiB free it admits, below the cushion it collects once and then refuses",async()=>{
 resetCasGuardsV1();
 const root=fresh();let collects=0;
 const total=Math.round(6.77*GIB/0.029),open=createCasGuardV1({root,partition:()=>({partition:"/",totalBytes:total,freeBytes:Math.round(6.77*GIB)}),usage:async()=>0,collect:async()=>{collects++;return stub;}});
 await open.admit(0);await open.settle();expect(collects).toBeLessThanOrEqual(1);
 resetCasGuardsV1();collects=0;
 const tight=createCasGuardV1({root,partition:()=>({partition:"/",totalBytes:total,freeBytes:Math.round(1*GIB)}),usage:async()=>0,collect:async()=>{collects++;return stub;}});
 const failure=await tight.admit(0).then(()=>undefined,reason=>reason);
 expect(collects).toBe(1);expect(failure.key).toBe("process.disk.low-space");
});

test("law: a machine at 97.1% used with collectable artefacts admits the command after the collection, the derivable writers run first by priority, the least recently used cache objects only when they are needed, and the collection stops once the need fits with its reserve",async()=>{
 const volume=new Volume(Math.round(6.77*GIB/0.029),Math.round(6.77*GIB)),log:string[]=[],lines:string[]=[];
 const writers=[fake(volume,"cas-lru",70,10*GIB,log,{class:"content",collector:"cas-lru"}),fake(volume,"incremental",10,1*GIB,log,{relief:"host"}),fake(volume,"waves",40,2*GIB,log),fake(volume,"staging",60,3*GIB,log),fake(volume,"inside-image",20,9*GIB,log,{relief:"image"}),fake(volume,"slack",90,40*GIB,log,{collector:"manual"})];
 const outcome=await recoverDiskV1({facts:volume.facts,needBytes:0,writers,language:"en",report:line=>lines.push(line)});
 expect(outcome.verdict).toBe("admit");
 expect(log).toEqual(["incremental","waves","staging"]);
 expect(volume.free).toBeGreaterThanOrEqual(outcome.targetFreeBytes);
 expect(outcome.steps.map(step=>step.id)).toEqual(log);
 expect(lines[0]).toContain("Partition / has");expect(lines[lines.length-1]).toContain("the command starts");
});

test("disk recovery names the current writer before awaiting it and stops reporting at measured admission",async()=>{
 const MIB=1024**2,{initialMiB,requiredMiB,writers,starts}=collectorStarts;
 const oracle=new Database(":memory:");
 const expected=oracle.query(`WITH ordered AS (
  SELECT json_extract(value,'$.id') AS id,json_extract(value,'$.priority') AS priority,
   coalesce(sum(json_extract(value,'$.measuredMiB')) OVER (ORDER BY json_extract(value,'$.priority')
    ROWS BETWEEN UNBOUNDED PRECEDING AND 1 PRECEDING),0) AS prior
  FROM json_each(?1) WHERE json_extract(value,'$.relief')='host'
 ) SELECT id FROM ordered WHERE (?2+prior)<?3 ORDER BY priority`)
  .all(JSON.stringify(writers),initialMiB,requiredMiB).map(value=>(value as {id:string}).id);
 oracle.close();
 expect(expected).toEqual(["leaked-images","compile-caches","cas-lru"]);
 for(const language of ["en","de"] as const){
  const volume=new Volume(238*GIB,initialMiB*MIB),lines:string[]=[],called:string[]=[],waiting=Promise.withResolvers<void>(),release=Promise.withResolvers<void>();
  const message=(id:string)=>starts[language].replace("{id}",id);
  const ports:DiskWriterV1[]=writers.map(writer=>({...fake(volume,writer.id,writer.priority,0,called,{relief:writer.relief as DiskWriterRowV1["relief"]}),collect:async()=>{
   called.push(writer.id);
   if(writer.id==="compile-caches"){waiting.resolve();await release.promise;}
   volume.free+=writer.measuredMiB*MIB;return 0;
  }}));
  const recover=()=>recoverNeedV1({facts:volume.facts,needBytes:requiredMiB*MIB,writers:ports,language,report:line=>lines.push(line)});
  const pending=recover();
  await waiting.promise;
  const beforeCompletion=lines.at(-1);
  release.resolve();
  const outcome=await pending;
  expect(beforeCompletion).toBe(message("compile-caches"));
  expect(outcome.verdict).toBe("admit");expect(volume.free).toBe((initialMiB+1000)*MIB);
  expect(called).toEqual(expected);
  expect(lines.filter(line=>writers.some(writer=>line===message(writer.id)))).toEqual(expected.map(message));
  const cancelled=new AbortController();volume.free=initialMiB*MIB;called.length=0;
  const stopAtStart=recoverNeedV1({facts:volume.facts,needBytes:requiredMiB*MIB,writers:ports,language,report:line=>{if(line===message("compile-caches"))cancelled.abort();},signal:cancelled.signal});
  await expect(stopAtStart).rejects.toBeDefined();
  expect(called).toEqual(["leaked-images"]);
 }
});

test("a Ready disk refusal bounds the post-collection census using the last physical measure",async()=>{
 const MIB=1024**2,{freeMiB,requiredMiB,budgetMs,writers}=consumerCensus;
 const db=new Database(":memory:");
 const expected=db.query(`with steps as (
  select json_extract(value,'$.id') as id,json_extract(value,'$.elapsedMs') as elapsed,
   coalesce(sum(json_extract(value,'$.elapsedMs')) over (order by key rows between unbounded preceding and 1 preceding),0) as prior
  from json_each(?1)
 ) select id, (prior+elapsed<=?2) as complete from steps where prior<?2`)
  .all(JSON.stringify(writers),budgetMs) as {id:string;complete:number}[];
 db.close();
 const volume=new Volume(238*GIB,freeMiB*MIB),progress:string[]=[],visited:string[]=[],control=new AbortController();
 const ports=(cancel:boolean):DiskWriterV1[]=>writers.map((item,index)=>({
  row:row(item.id,index),root:"/"+item.id,
  async measure(signal){
   visited.push(item.id);
   if(item.elapsedMs>budgetMs){
    if(cancel){control.abort();signal?.throwIfAborted();}
    if(!signal?.aborted)await new Promise<void>(resolve=>signal?.addEventListener("abort",()=>resolve(),{once:true}));
   }
   return item.bytes;
  }
 }));
 const options={facts:volume.facts,needBytes:requiredMiB*MIB,language:"en" as const,consumerBudgetMilliseconds:budgetMs};
 const outcome=await recoverNeedV1({...options,writers:ports(false),signal:AbortSignal.timeout(1_000),report:line=>progress.push(line)});
 expect(outcome.verdict).toBe("refuse");
 expect(outcome.facts?.freeBytes).toBe(freeMiB*MIB);
 expect(visited).toEqual(expected.map(item=>item.id));
 expect(outcome.consumers.map(row=>row.id)).toEqual(expected.filter(item=>item.complete).map(item=>item.id));
 expect(progress.at(-2)).toBe("Measuring disk consumer slow.");
 expect(progress.at(-1)).toContain("Stopped measuring disk consumers");
 visited.length=0;
 await expect(recoverNeedV1({...options,writers:ports(true),signal:control.signal})).rejects.toBeDefined();
 expect(visited).toEqual(["small","slow"]);
});

test("law: the least recently used cache objects go last and only when the derivable writers did not free enough, image-internal writers never run for the host and manual ones are never run",async()=>{
 const volume=new Volume(Math.round(6.77*GIB/0.029),Math.round(6.77*GIB)),log:string[]=[];
 const writers=[fake(volume,"incremental",10,1*GIB,log),fake(volume,"cas-lru",70,10*GIB,log,{class:"content"}),fake(volume,"inside-image",20,9*GIB,log,{relief:"image"}),fake(volume,"slack",90,40*GIB,log,{collector:"manual"})];
 const outcome=await recoverDiskV1({facts:volume.facts,needBytes:0,writers,language:"de"});
 expect(outcome.verdict).toBe("admit");expect(log).toEqual(["incremental","cas-lru"]);
});

test("law: a command that declares its need is admitted only when need plus cushion is free, and the collection continues until that fits",async()=>{
 const volume=new Volume(Math.round(238*GIB),Math.round(5*GIB)),log:string[]=[];
 const writers=[fake(volume,"a",10,1*GIB,log),fake(volume,"b",11,0.4*GIB,log),fake(volume,"c",12,3*GIB,log),fake(volume,"d",13,3*GIB,log)];
 const outcome=await recoverDiskV1({facts:volume.facts,needBytes:4*GIB,writers,language:"en"});
 expect(outcome.verdict).toBe("admit");expect(volume.free).toBeGreaterThanOrEqual(4*GIB+casPartitionBudgetV1(volume.total).cushionBytes);
 expect(log).toEqual(["a","b","c","d"].slice(0,log.length));expect(log.length).toBeLessThan(5);
});

test("law: a machine with nothing collectable refuses with the largest remaining consumers by root with their sizes and the exact command to free each, in both languages",async()=>{
 const volume=new Volume(Math.round(238*GIB),Math.round(0.8*GIB)),log:string[]=[];
 const writers=[fake(volume,"cas-lru",70,0,log,{class:"content",command:"repo cache cas gc"}),{row:row("compile-caches",11,{command:"repo cache gc"}),root:"/cache",measure:()=>3*GIB,collect:async()=>0},{row:row("image-slack",90,{collector:"manual",class:"image",command:"hdiutil detach /w/.cache/b/r/m && rm -r /w/.cache/b/r"}),root:"/w/.cache/b",measure:()=>36*GIB},{row:row("stale-waves",40),root:"/waves",measure:()=>0.1*GIB,collect:async()=>0},{row:row("zero",41),root:"/zero",measure:()=>0,collect:async()=>0},{row:row("dead",42),root:"/dead",measure:()=>{throw Error("gone");},collect:async()=>0}];
 const outcome=await recoverDiskV1({facts:volume.facts,needBytes:0,writers,language:"en"});
 expect(outcome.verdict).toBe("refuse");
 expect(outcome.consumers.map(item=>item.id)).toEqual(["image-slack","compile-caches","stale-waves"]);
 expect(outcome.consumers[0]!.bytes).toBe(36*GIB);
 for(const [language,text] of [["en","Largest consumers: /w/.cache/b (36 GiB): `hdiutil detach"],["de","Größte Verbraucher: /w/.cache/b (36 GiB): `hdiutil detach"]] as const){
  const failure=refuseDiskRecoveryV1(outcome,volume.facts());
  expect(isRepoErrorV1(failure)).toBe(true);expect(failure.code).toBe("refused");expect(failure.key).toBe("disk-recovery.low-space-consumers");
  const rendered=renderRepoErrorV1(failure,language);expect(rendered).toContain(text);expect(rendered).toContain("repo cache gc");expect(rendered).toContain("Partition /");
 }
});

test("a failing collector never stops the next one, a cancellation stops between collectors, and a volume that cannot be measured or derived is left to the fixed shares",async()=>{
 const volume=new Volume(Math.round(238*GIB),Math.round(5*GIB)),log:string[]=[],lines:string[]=[];
 const writers=[fake(volume,"broken",10,0,log,{},true),fake(volume,"good",11,10*GIB,log)];
 const outcome=await recoverDiskV1({facts:volume.facts,needBytes:0,writers,language:"en",report:line=>lines.push(line)});
 expect(outcome.verdict).toBe("admit");expect(log).toEqual(["broken","good"]);expect(outcome.steps[0]!.failure).toContain("boom broken");expect(lines.some(line=>line.includes("Collector broken could not free space"))).toBe(true);
 const control=new AbortController(),cancelled=new Volume(Math.round(238*GIB),Math.round(1*GIB)),seen:string[]=[];
 const stopper:DiskWriterV1={row:row("stopper",10),root:"/s",measure:()=>0,collect:async()=>{seen.push("stopper");control.abort();return 0;}};
 await expect(recoverDiskV1({facts:cancelled.facts,needBytes:0,writers:[stopper,fake(cancelled,"after",11,5*GIB,seen)],language:"en",signal:control.signal})).rejects.toBeDefined();
 expect(seen).toEqual(["stopper"]);
 expect((await recoverDiskV1({facts:()=>undefined,needBytes:0,writers:[],language:"en"})).verdict).toBe("fixed-shares");
 expect((await recoverDiskV1({facts:()=>({partition:"/",totalBytes:1000*1024*1024,freeBytes:10*1024*1024}),needBytes:0,writers:[],language:"en"})).verdict).toBe("fixed-shares");
 const slow=new Volume(Math.round(238*GIB),Math.round(1*GIB));
 const sleeper:DiskWriterV1={row:row("sleeper",10),root:"/z",measure:()=>0,collect:({signal})=>new Promise((_accept,reject)=>{signal?.addEventListener("abort",()=>reject(Error("timed out")));})};
 const timed=await recoverDiskV1({facts:slow.facts,needBytes:0,writers:[sleeper],language:"en",budgetMilliseconds:20});
 expect(timed.steps[0]!.failure).toContain("timed out");
});

test("the gate: a machine at 97.1% with collectable writers starts the command, and a machine with nothing collectable refuses it with the typed refusal that lists the consumers",async()=>{
 resetCasGuardsV1();
 const root=fresh(),environment={SEMIO_CAS_ROOT:join(root,"cas"),HOME:root,SEMIO_LANGUAGE:"en"};
 const volume=new Volume(Math.round(238*GIB),Math.round(0.9*GIB)),log:string[]=[];
 const guard=createCasGuardV1({root:environment.SEMIO_CAS_ROOT,partition:volume.facts,usage:async()=>0,collect:async()=>stub});
 await admitBuildDiskV1(environment,{guard,partition:volume.facts,mount:()=>"/same",workspace:root,writers:[fake(volume,"incremental",10,8*GIB,log)],write:()=>undefined});
 expect(log).toEqual(["incremental"]);
 resetCasGuardsV1();
 const empty=new Volume(Math.round(238*GIB),Math.round(0.9*GIB)),guard2=createCasGuardV1({root:environment.SEMIO_CAS_ROOT,partition:empty.facts,usage:async()=>0,collect:async()=>stub});
 const failure=await admitBuildDiskV1(environment,{guard:guard2,partition:empty.facts,mount:()=>"/same",workspace:root,writers:[{row:row("big",11,{command:"repo cache gc"}),root:"/big",measure:()=>5*GIB,collect:async()=>0}],write:()=>undefined}).then(()=>undefined,reason=>reason);
 expect(failure.key).toBe("disk-recovery.low-space-consumers");expect(renderRepoErrorV1(failure,"de")).toContain("Größte Verbraucher: /big (5 GiB)");
 resetCasGuardsV1();
 const between=new Volume(Math.round(238*GIB),Math.round(8*GIB)),log2:string[]=[],guard3=createCasGuardV1({root:environment.SEMIO_CAS_ROOT,partition:between.facts,usage:async()=>0,collect:async()=>stub});
 await admitBuildDiskV1(environment,{guard:guard3,partition:between.facts,mount:()=>"/same",workspace:root,writers:[fake(between,"background",10,10*GIB,log2)],write:()=>undefined});
 await Bun.sleep(30);expect(log2).toEqual(["background"]);
});

test("the machine budget follows the volume and the footprint: free space plus what is held, less the reserve, never above the volume",()=>{
 const oracle=(cases:unknown)=>JSON.parse(spawnSync("python3",["-I",join(import.meta.dirname,"oracle.py")],{input:JSON.stringify(cases),encoding:"utf8"}).stdout);
 const cases=[[238,5,0,60],[238,47,4,60],[238,1,0,0],[926,47,0,90],[3725,2000,6,10],[20,5,0,0],[40,39,3,1],[256,200,10,20]].map(([total,free,need,footprint])=>({total:total!*GIB,free:free!*GIB,need:need!*GIB,footprint:footprint!*GIB}));
 const expected=oracle({budget:{largest:CAS_BUDGET_V1.largestNeedBytes,divisor:CAS_BUDGET_V1.cushionDivisor,factor:CAS_RESERVE_FACTOR_V1,minimum:CAS_BUDGET_V1.minimumTotalFactor},cases});
 cases.forEach((input,index)=>{
  const judged=judgeDiskV1({partition:"/",totalBytes:input.total,freeBytes:input.free},input.need),want=expected[index];
  expect(judged.action).toBe(want.action);
  if(want.action!=="fixed-shares"){expect(judged.mustFreeBytes).toBe(want.must);expect(judged.targetFreeBytes).toBe(want.target);expect(machineDiskBudgetV1({totalBytes:input.total,freeBytes:input.free,footprintBytes:input.footprint})).toBe(want.budget);}
  else expect(machineDiskBudgetV1({totalBytes:input.total,freeBytes:input.free,footprintBytes:input.footprint})).toBe(0);
 });
});

const entry=(name:string,ageHours:number,now:number,directory=true)=>({name,newestMs:now-ageHours*3600000,bytes:1024,directory});

test("bounds: waves go by age and keep the newest few, footprint logs are bounded in count and age, read verdicts by age, the files the replica overwrote by age and count",()=>{
 const now=Date.UTC(2026,10,18);
 const waves=Array.from({length:12},(_,index)=>entry("w"+index,index*5,now));
 const expired=selectExpiredV1(waves,now,DISK_BOUNDS_V1.waves);
 expect(expired.length).toBeGreaterThan(0);expect(expired).not.toContain("w0");expect(expired).not.toContain("w7");expect(expired.every(name=>Number(name.slice(1))>=DISK_BOUNDS_V1.waves.keepNewest)).toBe(true);
 expect(selectExpiredV1(waves.map(item=>({...item,newestMs:now})),now,DISK_BOUNDS_V1.waves)).toEqual([]);
 const logs=Array.from({length:300},(_,index)=>entry("l"+index,index/10,now,false));
 const rotated=selectRotatedV1(logs,now,DISK_BOUNDS_V1.footprintLogs);
 expect(logs.length-rotated.length).toBeLessThanOrEqual(DISK_BOUNDS_V1.footprintLogs.keepNewest);expect(rotated).not.toContain("l0");
 expect([entry("1-fresh-1",1,now),entry("1-old-2",3,now)].filter(finishedReadVerdictV1(now,DISK_BOUNDS_V1.readVerdicts,()=>false)).map(row=>row.name)).toEqual(["1-old-2"]);
 const lost=Array.from({length:40},(_,index)=>entry("x"+index,index*24,now));
 const gone=selectExpiredV1(lost,now,DISK_BOUNDS_V1.replicaLost);
 expect(gone).not.toContain("x0");expect(gone).not.toContain("x15");expect(gone).toContain("x39");
});

test("bounds: a temporary directory of the build system goes once its process is gone, one without a pid after a week, a live one, a file, a socket, a foreign name and the long-lived runtime roots never",()=>{
 const now=Date.UTC(2026,10,18),bound=DISK_BOUNDS_V1.temporaryDirectories,alive=(pid:number)=>pid===4242;
 const names=[entry("semio-farm-session-1111-X6WOW1",2,now),entry("semio-farm-session-4242-X6WOW1",200,now),entry("semio-graph-farm-1111-7ZBcTr",0.5,now),entry("semio-owner-pool-1xSrgi",30,now),entry("semio-owner-pool-7iJNoL",300,now),entry("semio-broker-47730-3701046317fc.sock",900,now,false),entry("other-1111-abcdef",900,now),entry("semio-repo",900,now),entry("semio-footprint",900,now),entry("semio-read-verdicts-501",900,now)];
 expect(selectTemporaryV1(names,now,bound,alive)).toEqual(["semio-farm-session-1111-X6WOW1","semio-owner-pool-7iJNoL"]);
});

test("bounds on a real directory: only expired entries inside the root go, a link out of the root is never followed, and a dry run removes nothing",async()=>{
 const root=fresh(),outside=fresh(),now=Date.now();
 writeFileSync(join(outside,"precious"),"keep");
 for(const [name,hours] of [["old-a",100],["old-b",90],["new-a",1],["new-b",2]] as const){mkdirSync(join(root,name));writeFileSync(join(root,name,"f"),"x".repeat(5000));const time=(now-hours*3600000)/1000;utimesSync(join(root,name,"f"),time,time);utimesSync(join(root,name),time,time);}
 symlinkSync(outside,join(root,"link"));const old=(now-500*3600000)/1000;
 const select=(entries:readonly {name:string;newestMs:number}[])=>entries.filter(entry=>now-entry.newestMs>24*3600000).map(entry=>entry.name);
 const dry=await applyBoundV1({directory:root,limit:1000,dryRun:true,select});
 expect(dry.removed.sort()).toEqual(["old-a","old-b"]);expect(readdirSync(root).length).toBe(5);
 const real=await applyBoundV1({directory:root,limit:1000,dryRun:false,select});
 expect(real.removed.sort()).toEqual(["old-a","old-b"]);expect(real.freedBytes).toBeGreaterThan(0);
 expect(existsSync(join(root,"old-a"))).toBe(false);expect(existsSync(join(root,"new-a"))).toBe(true);expect(existsSync(join(outside,"precious"))).toBe(true);
 utimesSync(root,old,old);
 const escape=await applyBoundV1({directory:root,limit:1000,dryRun:false,select:()=>["link","../x","",".."]});
 expect(existsSync(join(outside,"precious"))).toBe(true);expect(escape.freedBytes).toBe(0);
});

test("the real writers bind the registry: every writer that gives bytes to the host and is not manual has a collector, the manual ones have none, and the image slack is measured as allocated minus used",async()=>{
 const root=fresh(),writers=realDiskWritersV1({workspace:root,home:root,tmp:root,uid:501,environment:{},dryRun:true});
 expect(writers.length).toBe(DISK_WRITERS_V1.length);
 for(const writer of writers){
  if(writer.row.collector==="manual")expect(writer.collect).toBeUndefined();
  else if(writer.row.relief==="host")expect(typeof writer.collect).toBe("function");
 }
 expect((await consumersOfV1(writers)).length).toBeGreaterThanOrEqual(0);
 let spawned=0;
 const counting=realDiskWritersV1({workspace:root,home:root,tmp:root,uid:501,environment:{},ledger:async()=>{spawned++;}});
 for(const writer of counting.filter(item=>item.row.collector==="ledger"&&item.row.relief==="host"))await writer.collect!({needBytes:1});
 expect(spawned).toBe(1);
});

test("the writers that had no bound have one in the registry and in the real writers: waves, footprint logs, read verdicts, staging, the replica's overwritten files, each with a collector that the command `repo cache gc` runs",async()=>{
 const rows=new Map(DISK_WRITERS_V1.map(item=>[item.id,item]));
 for(const id of ["stale-waves","footprint-logs","read-verdicts","leaked-staging","replica-lost"]){expect(rows.get(id)?.relief).toBe("host");expect(rows.get(id)?.collector).not.toBe("manual");expect(rows.get(id)?.command).toBe("repo cache gc");}
 const root=fresh(),tmp=join(root,"tmp"),now=Date.now(),old=(now-100*86400000)/1000;
 mkdirSync(join(root,".cache","f","owner-pool-waves"),{recursive:true});mkdirSync(join(root,".cache","f","lost"),{recursive:true});mkdirSync(join(tmp,"semio-footprint"),{recursive:true});
 for(let index=0;index<12;index++){const dir=join(root,".cache","f","owner-pool-waves","wave"+index);mkdirSync(dir);writeFileSync(join(dir,"session.bin"),"x".repeat(9000));const time=old+index*86400;utimesSync(join(dir,"session.bin"),time,time);utimesSync(dir,time,time);}
 for(let index=0;index<300;index++){const file=join(tmp,"semio-footprint",index+".log");writeFileSync(file,"x");}
 const done=await collectMachineBoundsV1({workspace:root,home:root,tmp,uid:501,environment:{},now:()=>now},"en");
 expect(done.freed["stale-waves"]).toBeGreaterThan(0);
 expect(readdirSync(join(root,".cache","f","owner-pool-waves")).length).toBe(DISK_BOUNDS_V1.waves.keepNewest);
 expect(readdirSync(join(tmp,"semio-footprint")).length).toBeLessThanOrEqual(DISK_BOUNDS_V1.footprintLogs.keepNewest);
 expect(done.lines.some(line=>line.startsWith("Machine bound stale-waves: "))).toBe(true);
 const german=await collectMachineBoundsV1({workspace:root,home:root,tmp,uid:501,environment:{},now:()=>now,dryRun:true},"de");
 expect(german.lines[0]).toContain("würden freigegeben");
});

test("`repo cache gc` applies the machine bounds after the ledger, in the language asked for, a dry run removes nothing and the json form names what was freed",async()=>{
 const DAY=86400000,HOUR=3600000,reason={en:"x",de:"y"},purpose={en:"p",de:"q"},partitions={alpha:{directory:".cache/b/a",capacityBytes:1<<30,maximumLogicalBytes:1<<30}};
 const ledger=readCacheLedgerV1({version:1,roots:{repository:{directory:"r/cache",maximumBytes:1<<30,undeclaredUnusedAgeMs:14*DAY,undeclaredQuietMs:HOUR,ignored:[CACHE_SIZE_FILE_V1]},native:{directory:".cache",maximumBytes:1<<20,undeclaredUnusedAgeMs:7*DAY,undeclaredQuietMs:HOUR,ignored:[]},rust:{directory:".r",maximumBytes:0,undeclaredUnusedAgeMs:1000,undeclaredQuietMs:HOUR,ignored:[]}},areas:[{id:"alpha",root:"native",kind:"partition",directory:"b/a",partition:"alpha",managed:{owner:"image"},writers:["w"],purpose},{id:"loose",root:"repository",kind:"tree",directory:"loose",budget:{bytes:1<<20},collect:{unusedAgeMs:100*DAY,quietMs:HOUR,custody:"quiet"},writers:["w"],purpose}],retirements:[{root:"repository",directory:"old",quietMs:HOUR,reason}]},partitions);
 const machinePolicy=readMachinePolicyV1({version:1,defaultTotalBytes:1<<20,environment:"TEST_BUDGET",file:"m.json",hostFreeFloorBytes:0,alignBytes:1048576,toleranceBytes:0,dedupe:[],stores:[{id:"alpha",partition:"alpha",weightBasisPoints:1000,floorBytes:0,headroomBasisPoints:0,purpose},{id:"loose",loose:true,weightBasisPoints:9000,floorBytes:0,headroomBasisPoints:0,purpose}]},partitions);
 const workspace=fresh(),tmp=join(workspace,"tmp"),old=(Date.now()-100*86400000)/1000;
 mkdirSync(tmp,{recursive:true});mkdirSync(join(workspace,".cache","f","owner-pool-waves"),{recursive:true});writeFileSync(join(workspace,".cache","f","machine.json"),"{}");
 for(let index=0;index<12;index++){const dir=join(workspace,".cache","f","owner-pool-waves","w"+index);mkdirSync(dir);writeFileSync(join(dir,"s"),"x".repeat(5000));const time=old+index*86400;utimesSync(join(dir,"s"),time,time);utimesSync(dir,time,time);}
 const ports=cacheRunnerPortsV1({signal:new AbortController().signal,progress:async()=>{},exclusive:async()=>({release:()=>{}}),atimeReliable:async()=>true});
 const run=async(args:string[],language:string)=>{const lines:string[]=[];const code=await runCacheLedgerCommandV1(args,{workspace,environment:{HOME:workspace,TMPDIR:tmp,SEMIO_LANGUAGE:language},write:line=>lines.push(line),ledger,partitions,machinePolicy},ports);return{code,text:lines.join("\n")};};
 const dry=await run(["gc","--dry-run"],"de");
 expect(dry.text).toContain("Maschinengrenze stale-waves: ");expect(dry.text).toContain("würden freigegeben");expect(readdirSync(join(workspace,".cache","f","owner-pool-waves")).length).toBe(12);
 const real=await run(["gc"],"en");
 expect(real.text).toContain("Machine bound stale-waves: ");expect(readdirSync(join(workspace,".cache","f","owner-pool-waves")).length).toBe(DISK_BOUNDS_V1.waves.keepNewest);
 const json=await run(["gc","--json"],"en");expect(JSON.parse(json.text).machineBounds).toBeDefined();
});

test("verdict collection measures and removes only expired directories of dead readers",async()=>{
 const root=fresh(),now=Date.now(),bounds=DISK_BOUNDS_V1.readVerdicts,dir=join(root,"semio-read-verdicts-501");
 mkdirSync(dir);
 const alive=new Set(verdictCases.entries.filter(entry=>entry.alive).map(entry=>entry.pid));
 for(const entry of verdictCases.entries){
  const child=join(dir,entry.name),file=join(child,"one-verdict");
  mkdirSync(child);writeFileSync(file,"1");
  const at=new Date(now-entry.age);
  utimesSync(file,at,at);utimesSync(child,at,at);
 }
 const db=new Database(":memory:");
 try{
  const expectKept=(db.query(`select json_extract(value,'$.name') name from json_each(?1)
     where json_extract(value,'$.pid') is null or json_extract(value,'$.alive') or json_extract(value,'$.age')<=?2
     order by name`).all(JSON.stringify(verdictCases.entries),bounds.maximumAgeMilliseconds) as {name:string}[]).map(row=>row.name);
  const probes:number[]=[];
  const writers=realDiskWritersV1({workspace:root,home:root,tmp:root,uid:501,environment:{},now:()=>now,alive:pid=>{probes.push(pid);return alive.has(pid);}});
  const result=await writers.find(writer=>writer.row.id==="read-verdicts")!.collect!({needBytes:1});
  expect(result).toBeGreaterThan(0);
  expect(readdirSync(dir).sort()).toEqual(expectKept);
  expect([...new Set(probes)].sort()).toEqual([18021,18022]);
 }finally{db.close();}
});
