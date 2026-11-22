import {afterEach,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {testScratchRootV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {admitRemoteExecutionActionV1,readRemoteExecutionHostViewV1,type RemoteExecutionHostViewV1} from "../../🌡️load/🟦️.ts";
import {READY_STAGE_CLAIM_POLICY_V1 as P,clearReadyStageClaimV1,createReadyStageClaimPublisherV1,readReadyStageClaimV1,readyStageEntitlementV1,readyStageHoldV1,withReadyStageV1,writeReadyStageClaimV1,type ReadyStageClaimV1} from "../../🌡️load/🪑️stage-claim/🟦️.ts";
import {ownerPoolHostOfV1} from "../../🛰️sidecar/🏊️pool/🟦️.ts";
import {ownerPoolSizingV1,OWNER_POOL_POLICY_V1} from "../../../🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🟦️.ts";
import fixture from "./🔣️.json";

const scratch:string[]=[];
afterEach(()=>{for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const temporary=():string=>{const path=mkdtempSync(join(testScratchRootV1("rexec-stage-claim"),"scratch-"));scratch.push(path);return path;};
const NOW=1_800_000_000_000;
const host=(total:number,available:number,pressure:"normal"|"warn"|"critical"="normal")=>({version:1,atMs:NOW,platform:"darwin",cores:10,totalMegabytes:total,availableMegabytes:available,pressure,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:null,openFiles:1000,openFilesMaximum:200000,attachedImages:0});
const view=(total:number,available:number,pressure:"normal"|"warn"|"critical"="normal"):RemoteExecutionHostViewV1=>({host:host(total,available,pressure),grants:[],waiters:[]} as never);
const claim=(entitlementMegabytes:number,observedMegabytes:number,atMs:number=NOW):ReadyStageClaimV1=>({version:1,stage:"workspace",pid:4242,entitlementMegabytes,observedMegabytes,atMs});

test("The entitlement of every host size and run history equals the Python oracle",()=>{
 for(const row of fixture.entitlements){
  const runs=(fixture.runs as Record<string,never[]>)[row.case]!;
  expect(readyStageEntitlementV1({totalMegabytes:row.totalMegabytes,stage:"workspace",runs}),`${row.totalMegabytes} ${row.case}`).toBe(row.entitlementMegabytes);
 }
 const sixteen=(name:string)=>fixture.entitlements.find(row=>row.totalMegabytes===16384&&row.case===name)!.entitlementMegabytes;
 expect(sixteen("none")).toBe(Math.floor(16384*P.shareBasisPoints/10000));
 expect(sixteen("failed-only")).toBe(sixteen("none"));
 expect(sixteen("one-ok")).toBeGreaterThan(sixteen("none"));
 expect(sixteen("huge")).toBe(16384*P.maximumShareBasisPoints/10000);
});

test("The hold is what the stage may still grow into and a stale claim holds nothing",()=>{
 for(const row of fixture.holds)expect(readyStageHoldV1(claim(row.entitlementMegabytes,row.observedMegabytes,NOW),NOW+row.ageMilliseconds),`${row.observedMegabytes} ${row.ageMilliseconds}`).toBe(row.holdMegabytes);
 expect(readyStageHoldV1(undefined,NOW)).toBe(0);
});

test("The claim file round-trips, a stale, malformed or foreign file reads as no claim and only the owner clears it",()=>{
 const directory=temporary(),file=join(directory,P.fileName);
 expect(readReadyStageClaimV1(directory,NOW)).toBeUndefined();
 writeReadyStageClaimV1(directory,claim(4915,1200));
 expect(readReadyStageClaimV1(directory,NOW)).toEqual(claim(4915,1200));
 expect(readReadyStageClaimV1(directory,NOW+P.freshMilliseconds+1)).toBeUndefined();
 clearReadyStageClaimV1(directory,1);expect(existsSync(file)).toBe(true);
 clearReadyStageClaimV1(directory,4242);expect(existsSync(file)).toBe(false);
 writeFileSync(file,"{\"version\":1,\"stage\":\"workspace\"}");expect(readReadyStageClaimV1(directory,NOW)).toBeUndefined();
 writeFileSync(file,"not json");expect(readReadyStageClaimV1(directory,NOW)).toBeUndefined();
});

test("The publisher learns the entitlement once, writes on a size change or after the interval and clears its own file",()=>{
 const directory=temporary();let clock=NOW,reads=0;
 const publisher=createReadyStageClaimPublisherV1({directory,stage:"workspace",pid:4242,totalMegabytes:16384,runs:()=>{reads++;return [{stage:"workspace",outcome:"ok",peakMegabytes:4514}];},now:()=>clock});
 const at=()=>JSON.parse(readFileSync(join(directory,P.fileName),"utf8")) as ReadyStageClaimV1;
 publisher.observe(1000);expect(at().observedMegabytes).toBe(1000);expect(at().entitlementMegabytes).toBe(5643);
 clock+=1000;publisher.observe(1030);expect(at().observedMegabytes).toBe(1000);
 clock+=1000;publisher.observe(1100);expect(at().observedMegabytes).toBe(1100);
 clock+=P.writeMilliseconds;publisher.observe(1101);expect(at().observedMegabytes).toBe(1101);
 expect(reads).toBe(1);
 publisher.clear();expect(existsSync(join(directory,P.fileName))).toBe(false);
});

test("The reader adds the stage of this machine to the view and takes its growth room from the available memory",async()=>{
 const directory=temporary(),simulated=join(directory,"view.json");
 writeFileSync(simulated,JSON.stringify({host:{...host(16384,9000),atMs:Date.now()+86_400_000},grants:[],waiters:[]}));
 const environment={SEMIO_REMOTE_EXECUTION_HOST_VIEW:simulated,SEMIO_HOST_CAPACITY_DIRECTORY:directory};
 expect((await readRemoteExecutionHostViewV1(environment)).stage).toBeUndefined();
 writeReadyStageClaimV1(directory,claim(4915,1200,Date.now()));
 const seen=await readRemoteExecutionHostViewV1(environment);
 expect(seen.stage?.entitlementMegabytes).toBe(4915);
 expect(seen.host.availableMegabytes).toBe(9000-3715);
 expect(seen.grants.map(row=>row.id)).toEqual(["ready-stage"]);
 expect(seen.grants[0]!.observedMegabytes).toBe(1200);
});

test("A 16 GB worker's pool runs fewer children while its stage still grows, as many as the oracle says, and the same once it has grown",()=>{
 for(const row of fixture.pools){
  const grown=withReadyStageV1(view(row.totalMegabytes,row.availableMegabytes),claim(row.entitlementMegabytes,row.observedMegabytes),NOW);
  const size=(one:RemoteExecutionHostViewV1)=>ownerPoolSizingV1({host:ownerPoolHostOfV1(one,NOW),children:0,childMegabytes:row.childMegabytes},OWNER_POOL_POLICY_V1);
  expect(grown.host.availableMegabytes,JSON.stringify(row)).toBe(row.leftMegabytes);
  expect(size(grown).children,JSON.stringify(row)).toBe(row.children);
  const full=withReadyStageV1(view(row.totalMegabytes,row.availableMegabytes),claim(row.entitlementMegabytes,row.entitlementMegabytes),NOW);
  expect(size(full).children,JSON.stringify(row)).toBe(row.childrenGrown);
  expect(row.children).toBeLessThanOrEqual(row.childrenGrown);
 }
});

test("The agent of a 16 GB worker refuses an offload while its stage still has to grow, accepts it without a stage and refuses under pressure with a stage",()=>{
 const now=Date.now(),plain=view(16384,7000);
 expect(admitRemoteExecutionActionV1({view:plain,running:0,concurrency:10,nowMs:now})).toEqual({admit:true});
 const staged=withReadyStageV1(plain,claim(4915,1200,now),now);
 const verdict=admitRemoteExecutionActionV1({view:staged,running:0,concurrency:10,nowMs:now});
 expect(verdict.admit).toBe(false);
 expect(verdict.admit?"":verdict.reason).toBe("memory");
 const grown=withReadyStageV1(plain,claim(4915,4915,now),now);
 expect(admitRemoteExecutionActionV1({view:grown,running:0,concurrency:10,nowMs:now})).toEqual({admit:true});
 const warn=withReadyStageV1(view(16384,12000,"warn"),claim(4915,4915,now),now);
 const refused=admitRemoteExecutionActionV1({view:warn,running:0,concurrency:10,nowMs:now});
 expect(refused.admit?"":refused.reason).toBe("pressure-warn");
 const critical=withReadyStageV1(view(16384,12000,"critical"),claim(4915,4915,now),now);
 expect((admitRemoteExecutionActionV1({view:critical,running:0,concurrency:10,nowMs:now}) as {reason?:string}).reason).toBe("pressure-critical");
});

test("The oracle vectors are current",()=>{
 const run=Bun.spawnSync(["python3",join(import.meta.dir,"🐍️.py"),"--check"]);
 expect(run.exitCode,String(run.stderr)).toBe(0);
});
