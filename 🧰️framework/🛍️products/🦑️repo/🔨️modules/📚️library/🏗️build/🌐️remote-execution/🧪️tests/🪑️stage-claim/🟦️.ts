import {afterEach,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {testScratchRootV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {admitRemoteExecutionActionV1,readRemoteExecutionHostViewV1,type RemoteExecutionHostViewV1} from "../../🌡️load/🟦️.ts";
import {READY_STAGE_CLAIM_POLICY_V1 as P,clearReadyStageClaimV1,createReadyStageClaimPublisherV1,readReadyStageClaimV1,readyStageEntitlementV1,readyStageHoldV1,withReadyStageV1,writeReadyStageClaimV1,type ReadyStageClaimV1,type ReadyStageRunV1} from "../../🌡️load/🪑️stage-claim/🟦️.ts";
import {ownerPoolHostOfV1} from "../../🛰️sidecar/🏊️pool/🟦️.ts";
import {ownerPoolSizingV1,OWNER_POOL_POLICY_V1} from "../../../🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🟦️.ts";
import fixture from "./🔣️.json";

const scratch:string[]=[];
afterEach(()=>{for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const temporary=():string=>{const path=mkdtempSync(join(testScratchRootV1("rexec-stage-claim"),"scratch-"));scratch.push(path);return path;};
const NOW=1_800_000_000_000;
const runsByCase:Readonly<Record<string,readonly ReadyStageRunV1[]>>=fixture.runs;
const host=(total:number,available:number,pressure:"normal"|"warn"|"critical"="normal")=>({version:1,atMs:NOW,platform:"darwin",cores:10,totalMegabytes:total,availableMegabytes:available,pressure,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:null,openFiles:1000,openFilesMaximum:200000,attachedImages:0});
const view=(total:number,available:number,pressure:"normal"|"warn"|"critical"="normal"):RemoteExecutionHostViewV1=>({host:host(total,available,pressure),grants:[],waiters:[]} as never);
const claim=(entitlementMegabytes:number,observedMegabytes:number,atMs:number=NOW,waiting:boolean=false):ReadyStageClaimV1=>({version:1,stage:"workspace",pid:4242,entitlementMegabytes,observedMegabytes,waiting,atMs});

test("The entitlement of every host size and run history equals the Python oracle",()=>{
 for(const row of fixture.entitlements){
  const runs=runsByCase[row.case]!;
  expect(readyStageEntitlementV1({totalMegabytes:row.totalMegabytes,stage:"workspace",runs}),`${row.totalMegabytes} ${row.case}`).toBe(row.entitlementMegabytes);
 }
 const sixteen=(name:string)=>fixture.entitlements.find(row=>row.totalMegabytes===16384&&row.case===name)!.entitlementMegabytes;
 expect(sixteen("none")).toBe(Math.floor(16384*P.shareBasisPoints/10000));
 expect(sixteen("failed-only")).toBe(sixteen("none"));
 expect(sixteen("one-ok")).toBeGreaterThan(sixteen("none"));
 expect(sixteen("huge")).toBe(16384*P.maximumShareBasisPoints/10000);
});

test("The hold is the measured use plus the margin less what is used (never above the learned peak), nothing while the stage waits, and a stale claim holds nothing",()=>{
 for(const row of fixture.holds)expect(readyStageHoldV1(claim(row.entitlementMegabytes,row.observedMegabytes,NOW,row.waiting),NOW+row.ageMilliseconds),`${row.observedMegabytes} ${row.waiting} ${row.ageMilliseconds}`).toBe(row.holdMegabytes);
 expect(readyStageHoldV1(undefined,NOW)).toBe(0);
 expect(readyStageHoldV1(claim(4915,1200),NOW),"running: 1200 x 1.25 less 1200").toBe(300);
 expect(readyStageHoldV1(claim(4915,1200,NOW,true),NOW),"waiting").toBe(0);
 expect(readyStageHoldV1(claim(4915,0),NOW),"a stage that just started keeps the floor").toBe(P.floorMegabytes);
});

test("The claim file round-trips, a stale, malformed or foreign file reads as no claim and only the owner clears it",()=>{
 const directory=temporary(),file=join(directory,P.fileName);
 expect(readReadyStageClaimV1(directory,NOW)).toBeUndefined();
 writeReadyStageClaimV1(directory,claim(4915,1200,NOW,true));
 expect(readReadyStageClaimV1(directory,NOW)).toEqual(claim(4915,1200,NOW,true));
 expect(readReadyStageClaimV1(directory,NOW+P.freshMilliseconds+1)).toBeUndefined();
 clearReadyStageClaimV1(directory,1);expect(existsSync(file)).toBe(true);
 clearReadyStageClaimV1(directory,4242);expect(existsSync(file)).toBe(false);
 writeFileSync(file,JSON.stringify({version:1,stage:"workspace",pid:4242,entitlementMegabytes:4915,observedMegabytes:1200,atMs:NOW}));expect(readReadyStageClaimV1(directory,NOW),"a claim without the waiting flag is not a claim").toBeUndefined();
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
 clock+=P.writeMilliseconds;publisher.observe(1101);expect(at().observedMegabytes).toBe(1101);expect(at().waiting).toBe(false);
 clock+=1000;publisher.observe(1101,true);expect(at().waiting,"a stage that starts waiting is written at once").toBe(true);
 clock+=1000;publisher.observe(1102,true);expect(at().observedMegabytes).toBe(1101);
 clock+=1000;publisher.observe(1102,false);expect(at().waiting,"and when it runs again").toBe(false);
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
 expect(seen.host.availableMegabytes,"1200 x 1.25 less 1200 is held, not the 3715 of the entitlement").toBe(9000-300);
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

test("The agent of a 16 GB worker keeps back only the measured growth of its stage, nothing while the stage waits, and judges raised pressure by the size of the unit",()=>{
 const now=Date.now(),plain=view(16384,7000),ask=(one:RemoteExecutionHostViewV1,megabytes?:number)=>admitRemoteExecutionActionV1({view:one,running:0,concurrency:10,nowMs:now,...megabytes===undefined?{}:{megabytes}});
 expect(ask(plain)).toEqual({admit:true});
 expect(ask(withReadyStageV1(plain,claim(4915,1200,now),now)),"the stage holds 300 more, not the 3715 of its entitlement").toEqual({admit:true});
 const tight=view(16384,4300),running=ask(withReadyStageV1(tight,claim(4915,1200,now),now));
 expect(running.admit).toBe(false);
 expect(running.admit?"":running.reason).toBe("memory");
 expect(ask(withReadyStageV1(tight,claim(4915,1200,now,true),now)),"the same stage waiting holds nothing back").toEqual({admit:true});
 expect(ask(withReadyStageV1(tight,claim(4915,1200,now),now),1024),"a small unit fits beside the growing stage").toEqual({admit:true});
 const warn=withReadyStageV1(view(16384,12000,"warn"),claim(4915,4915,now),now),refused=ask(warn);
 expect(refused.admit?"":refused.reason).toBe("pressure-warn");
 expect(ask(warn,512),"raised pressure alone does not refuse a small unit").toEqual({admit:true});
 const critical=withReadyStageV1(view(16384,12000,"critical"),claim(4915,4915,now),now);
 expect((ask(critical,512) as {reason?:string}).reason).toBe("pressure-critical");
});

test("The oracle vectors are current",()=>{
 const run=Bun.spawnSync(["python3",join(import.meta.dir,"🐍️.py"),"--check"]);
 expect(run.exitCode,String(run.stderr)).toBe(0);
});
