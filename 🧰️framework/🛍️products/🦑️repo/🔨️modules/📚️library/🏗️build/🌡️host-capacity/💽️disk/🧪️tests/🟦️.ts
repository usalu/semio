/** 🧪️ Contract: a heavy command is admitted to the cache partition before it writes anything. Above the high share a collection starts in the background, at the critical share the command waits for it (with a localized progress line) and is refused with the typed `disk.low-space` refusal when that frees too little, the partition of another workspace is held to the same critical share, and a light or foreign environment is left alone. */
import {afterAll,expect,spyOn,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readHostCapacityPolicyV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {renderRepoErrorV1,isRepoErrorV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {createCasGuardV1,resetCasGuardsV1} from "../../../🗄️cas/🚦️guard/🟦️.ts";
import type {CasCollectionV1} from "../../../🗄️cas/🧹️collector/🟦️.ts";
import type {CasPartitionV1} from "../../../🗄️cas/🌡️partition/🟦️.ts";
import {withBuildCapacityV1} from "../../🟦️.ts";
import {admitBuildDiskV1,expectedDiskGrowthBytesV1} from "../🟦️.ts";
import {Database} from "bun:sqlite";
import {CAS_BUDGET_V1,CAS_RESERVE_FACTOR_V1} from "../../../🗄️cas/🧬️schema/🟦️.ts";
import {DISK_WRITERS_V1} from "../../🧹️disk-recovery/🟦️.ts";
import * as realDiskRecovery from "../../🧹️disk-recovery/🔌️real/🟦️.ts";
import fixture from "../../🧹️disk-recovery/🖼️image-slack/🧪️tests/♻️reclaim/🔣️receipts.json" with {type:"json"};

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"disk-admit-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const MIB=1024*1024;
const stub:CasCollectionV1={status:"collected",partial:false,dryRun:false,scanned:0,evicted:0,freedBytes:0,busy:0,skipped:0,totals:{totalBytes:0,protectedBytes:0,candidateBytes:0,plannedBytes:0,keptBytes:0,shortfallBytes:0,needBytes:0},byReason:{age:0,area:0,host:0}};
const volume=(freeMiB:number,name="/volume"):CasPartitionV1=>({partition:name,totalBytes:1000*MIB,freeBytes:freeMiB*MIB});
const real=(language:"en"|"de")=>{const root=fresh();return{SEMIO_CAS_ROOT:join(root,"cas"),HOME:root,SEMIO_LANGUAGE:language};};

test("a partition at its critical share makes the command wait for the collection and refuses it with the typed refusal, naming the partition, the free bytes and the gc command in both languages",async()=>{
 for(const [language,refusal] of [["en","Partition /volume has only 10 MiB free (99% used, limit 97%)"],["de","Partition /volume hat nur noch 10 MiB frei (99 % belegt, Grenze 97 %)"]] as const){
  resetCasGuardsV1();
  const environment=real(language);let collects=0;
  const guard=createCasGuardV1({root:environment.SEMIO_CAS_ROOT,partition:()=>volume(10),usage:async()=>0,collect:async()=>{collects++;return stub;}});
  const failure=await admitBuildDiskV1(environment,{guard,partition:()=>volume(10),mount:path=>path,workspace:environment.HOME}).then(()=>{throw Error("expected a refusal");},reason=>reason);
  expect(collects).toBe(1);expect(isRepoErrorV1(failure)).toBe(true);expect(failure.key).toBe("process.disk.low-space");expect(failure.code).toBe("refused");
  const text=renderRepoErrorV1(failure,language);expect(text).toContain(refusal);expect(text).toContain("repo cache cas gc");
 }
});

test("the default guard says that it waits for the collection, naming the partition in the language of the environment",async()=>{
 for(const [language,line] of [["en","Partition /volume is 99% used (limit 97%): freeing the least recently used cache objects"],["de","Partition /volume ist zu 99 % belegt (Grenze 97 %): die am längsten ungenutzten Cache-Objekte"]] as const){
  resetCasGuardsV1();
  const environment=real(language),lines:string[]=[];
  const failure=await admitBuildDiskV1(environment,{partition:()=>volume(10),mount:path=>path,write:text=>lines.push(text),workspace:environment.HOME}).then(()=>undefined,reason=>reason);
  expect(lines.some(text=>text.startsWith(line))).toBe(true);expect(failure.key).toBe("process.disk.low-space");
 }
});

test("a partition above its high share starts the collection in the background and the command goes on",async()=>{
 resetCasGuardsV1();
 const environment=real("en");let collects=0;
 const guard=createCasGuardV1({root:environment.SEMIO_CAS_ROOT,partition:()=>volume(80),usage:async()=>0,collect:async()=>{collects++;return stub;}});
 await admitBuildDiskV1(environment,{guard,partition:()=>volume(80),mount:path=>path,workspace:environment.HOME});await guard.settle();expect(collects).toBe(1);
});

test("the partition of another workspace is held to the critical share too, and the same partition is judged once",async()=>{
 resetCasGuardsV1();
 const environment=real("en"),guard=createCasGuardV1({root:environment.SEMIO_CAS_ROOT,partition:()=>volume(500),usage:async()=>0,collect:async()=>stub});
 const other=await admitBuildDiskV1(environment,{guard,partition:path=>path===environment.HOME?volume(10,"/elsewhere"):volume(500),mount:path=>path===environment.HOME?"/elsewhere":"/cache",workspace:environment.HOME}).then(()=>undefined,reason=>reason);
 expect(other.key).toBe("process.disk.low-space");expect(renderRepoErrorV1(other,"en")).toContain("Partition /elsewhere");
 await admitBuildDiskV1(environment,{guard,partition:()=>volume(500),mount:()=>"/same",workspace:environment.HOME});
});

test("an environment that is no real user session leaves the host alone, and the door refuses before it grants capacity",async()=>{
 resetCasGuardsV1();
 await admitBuildDiskV1({},{partition:()=>volume(1),mount:path=>path});
 const directory=fresh(),environment={...real("en"),SEMIO_HOST_CAPACITY_DIRECTORY:directory,LANG:"en_US.UTF-8"};
 const policy=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
 let ran=false,samples=0;
 const sample=async()=>{
  samples++;
  const host:HostSampleV1={version:1,atMs:Date.now(),platform:"darwin",cores:8,totalMegabytes:16384,availableMegabytes:14000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:0};
  return {host,swappedOutMegabytes:0};
 };
 const admission={directory,policy,strip:(args:readonly string[])=>args,tool:"bazel",write:()=>undefined,sample,observe:async()=>500};
 const guard=createCasGuardV1({root:environment.SEMIO_CAS_ROOT,partition:()=>volume(10),usage:async()=>0,collect:async()=>stub});
 const failure=await withBuildCapacityV1(["run","@x/y:build","--parallel=2"],environment,async()=>{ran=true;},{...admission,disk:{guard,partition:()=>volume(10),mount:path=>path}}).then(()=>undefined,reason=>reason);
 expect(ran).toBe(false);expect(failure.key).toBe("process.disk.low-space");expect(samples).toBe(0);
 const roomy=createCasGuardV1({root:environment.SEMIO_CAS_ROOT+"2",partition:()=>volume(500),usage:async()=>0,collect:async()=>stub});
 await withBuildCapacityV1(["run","@x/y:build","--parallel=2"],environment,async()=>{ran=true;},{...admission,disk:{guard:roomy,partition:()=>volume(500),mount:()=>"/same"}});
 expect(ran).toBe(true);expect(samples).toBeGreaterThan(0);
});

test("the expected growth of a command counts before it starts: a partition 96% used admits nothing that would cross 97%, while a no-growth command is admitted",async()=>{
 resetCasGuardsV1();
 const environment=real("en");
 const measured=()=>volume(40);
 await admitBuildDiskV1(environment,{guard:createCasGuardV1({root:environment.SEMIO_CAS_ROOT,partition:measured,usage:async()=>0,collect:async()=>stub}),partition:measured,mount:path=>path,workspace:environment.HOME,expectedBytes:0});
 resetCasGuardsV1();
 const failure=await admitBuildDiskV1(environment,{guard:createCasGuardV1({root:environment.SEMIO_CAS_ROOT,partition:measured,usage:async()=>0,collect:async()=>stub}),partition:measured,mount:path=>path,workspace:environment.HOME,expectedBytes:20*MIB}).then(()=>undefined,reason=>reason);
 expect(failure?.key).toBe("process.disk.low-space");
});

test("expected growth: 4 GiB for a Cargo build, an explicit environment value wins, other classes expect nothing",()=>{
 expect(expectedDiskGrowthBytesV1({},"cargo")).toBe(4*1024**3);
 expect(expectedDiskGrowthBytesV1({},"show")).toBe(0);
 expect(expectedDiskGrowthBytesV1({SEMIO_BUILD_EXPECTED_DISK_BYTES:"123"},"cargo")).toBe(123);
 expect(expectedDiskGrowthBytesV1({SEMIO_BUILD_EXPECTED_DISK_BYTES:"junk"},"cargo")).toBe(4*1024**3);
});


test("automatic build intake carries same-volume in-place reclaim pressure and the existing reserve",async()=>{
 resetCasGuardsV1();
 const environment=real("en"),sample=fixture.buildIngress,GIB=1024**3;
 const needed=sample.needGiB*GIB;
 using oracle=new Database(":memory:");
 const {target}=oracle.query("select ?1+?2*?3 target").get(needed,CAS_BUDGET_V1.largestNeedBytes,CAS_RESERVE_FACTOR_V1) as {target:number};
 const cacheMount=join(environment.HOME,"measured-cache-volume"),workspace=join(environment.HOME,"separate-workspace");
 let free=sample.startGiB*GIB,runs=0,mounted=cacheMount;
 const measure=(path:string):CasPartitionV1=>path===environment.SEMIO_CAS_ROOT?{partition:mounted,totalBytes:sample.totalGiB*GIB,freeBytes:free}:{partition:workspace,totalBytes:sample.totalGiB*GIB,freeBytes:sample.otherGiB*GIB};
 const guard=createCasGuardV1({root:environment.SEMIO_CAS_ROOT,partition:measure,usage:async()=>0,collect:async()=>stub});
 const writers=spyOn(realDiskRecovery,"realDiskWritersV1").mockImplementation(context=>{
  expect(context.reclaimPressure).toMatchObject({path:cacheMount,requiredFreeBytes:target});
  expect(context.pressure).toBeUndefined();
  expect(context.reclaimPressure?.freeBytes()).toBe(sample.startGiB*GIB);
  const row=DISK_WRITERS_V1.find(row=>row.id==="image-slack")!;
  return[{row,root:workspace,measure:()=>0,collect:async({needBytes})=>{
   expect(needBytes).toBe(target-sample.startGiB*GIB);
   mounted=workspace;
   expect(context.reclaimPressure?.freeBytes()).toBeUndefined();
   mounted=cacheMount;
   free=target;
   expect(context.reclaimPressure?.freeBytes()).toBe(target);
   runs++;return needBytes;
  }}];
 });
 try{await admitBuildDiskV1(environment,{guard,partition:measure,mount:path=>measure(path).partition,workspace,expectedBytes:needed,write:()=>{}});}
 finally{writers.mockRestore();}
 expect(runs).toBe(1);
});
