/** 🔼️ Contract: a run that was forecast quiet asks for its full capacity before the graph builder custody is taken, never while holding it, from every process of the run (one that only inherits the grant included, since inheriting grows nothing); the phase mark inside the custody is bookkeeping and waits for nothing. */
import {graphSourceV1} from "../📖️source/🟦️.ts";
import{expect,test}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{hostBootAtV1,hostUpgradePathV1,newHostCapacityIdV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import{markBuildCapacityPhaseV1,upgradeBuildCapacityBeforeRebuildV1}from"../../../🌡️host-capacity/🚏️marks/🟦️.ts";

const source=graphSourceV1();
const builder=source.indexOf("withBuilder:async<T>");

test("the capacity upgrade of a rebuild is asked before the builder custody is taken",()=>{
 const upgrade=source.indexOf("()=>prepareRebuild(published)",builder),custody=source.indexOf("withGenerationCustodyV1(root,invocation,phaseControl,request,",builder);
 expect(builder).toBeGreaterThan(0);expect(upgrade).toBeGreaterThan(builder);expect(custody).toBeGreaterThan(upgrade);
 expect(source).toContain("await upgradeBuildCapacityBeforeRebuildV1(process.env,phaseControl,");
});

test("the phase mark stays inside the custody body and the only wait before the custody is the upgrade",()=>{
 const body=source.slice(builder,source.indexOf("withGenerationCustodyV1(",builder));
 expect(body).toContain('const body=async(store:RepositoryCacheStoreV1,control:ResourceLeaseControl,claim:SliceClaimV1)=>{if(rebuilding)await markBuildCapacityPhaseV1("rebuild");return build(store,control,claim);};');expect(source.slice(source.indexOf("withGenerationCustodyV1(",builder),source.indexOf("return result;",builder))).not.toContain("upgradeBuildCapacity");
 expect(body.match(/prepareRebuild/gu)?.length).toBe(1);
});

test("a process under no grant asks for nothing, one that only inherits a live grant asks its holder before a rebuild although its phase marks are off",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"q16g-")),id=newHostCapacityIdV1(),now=Date.now();
 try{
  mkdirSync(join(directory,"grants"),{recursive:true});mkdirSync(join(directory,"queue"),{recursive:true});
  writeFileSync(join(directory,"grants",id+".json"),JSON.stringify({id,operation:"run",jobs:8,cpu:8,reservedMegabytes:3000,admittedAtMs:now,observedMegabytes:null,pid:process.pid,bootAtMs:hostBootAtV1(directory,now)}));
  const inheriting={SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_HOST_CAPACITY_GRANT:id,SEMIO_HOST_CAPACITY_PHASE_MARKS:"off"};
  expect(await upgradeBuildCapacityBeforeRebuildV1({})).toBeUndefined();
  expect(await upgradeBuildCapacityBeforeRebuildV1({SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_HOST_CAPACITY_GRANT:newHostCapacityIdV1(),SEMIO_HOST_CAPACITY_PHASE_MARKS:"off"}),"a grant that is gone").toBeUndefined();
  expect(await upgradeBuildCapacityBeforeRebuildV1(inheriting)).toBeUndefined();
  expect(existsSync(hostUpgradePathV1(directory,id)),"the run is recorded as one that executed work").toBe(true);
  expect(await markBuildCapacityPhaseV1("rebuild",inheriting),"no phase mark under an inherited grant").toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the rebuild gate forwards a waiting quiet grant's peer reuse before taking custody",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"q16g-")),id=newHostCapacityIdV1(),now=Date.now(),marker=hostUpgradePathV1(directory,id);
 try{
  mkdirSync(join(directory,"grants"),{recursive:true});
  const grant=join(directory,"grants",id+".json");
  writeFileSync(grant,JSON.stringify({id,operation:"run",jobs:1,cpu:1,reservedMegabytes:500,admittedAtMs:now,observedMegabytes:null,pid:process.pid,bootAtMs:hostBootAtV1(directory,now),quiet:true}));
  const environment={SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_HOST_CAPACITY_GRANT:id};
  const control={signal:AbortSignal.timeout(2500),remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined};
  const reuse=async()=>existsSync(marker);
  expect(await upgradeBuildCapacityBeforeRebuildV1(environment,control,reuse)).toBeUndefined();
  expect(existsSync(marker)).toBe(false);
  expect(JSON.parse(readFileSync(grant,"utf8"))).toMatchObject({quiet:true,reservedMegabytes:500});
 }finally{rmSync(directory,{recursive:true,force:true});}
});
