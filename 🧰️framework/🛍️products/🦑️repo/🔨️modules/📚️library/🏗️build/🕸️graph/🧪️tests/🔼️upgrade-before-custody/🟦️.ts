/** 🔼️ Contract: a run that was forecast quiet asks for its full capacity before the graph builder custody is taken, never while holding it, from every process of the run (one that only inherits the grant included, since inheriting grows nothing); the phase mark inside the custody is bookkeeping and waits for nothing. */
import{expect,test}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{hostBootAtV1,hostUpgradePathV1,newHostCapacityIdV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import{markBuildCapacityPhaseV1,upgradeBuildCapacityBeforeRebuildV1}from"../../../🌡️host-capacity/🚏️marks/🟦️.ts";

const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8");
const builder=source.indexOf("withBuilder:async<T>");

test("the capacity upgrade of a rebuild is asked before the builder custody is taken",()=>{
 const upgrade=source.indexOf("await upgradeBuildCapacityBeforeRebuildV1();",builder),custody=source.indexOf("withGenerationCustodyV1(root,invocation,phaseControl,request,(store,control,claim)=>{image=store.root;return body(store,control,claim);})",builder);
 expect(builder).toBeGreaterThan(0);expect(upgrade).toBeGreaterThan(builder);expect(custody).toBeGreaterThan(upgrade);
});

test("the phase mark stays inside the custody body and the only wait before the custody is the upgrade",()=>{
 const body=source.slice(builder,source.indexOf("withGenerationCustodyV1(",builder));
 expect(body).toContain('const body=async(store:RepositoryCacheStoreV1,control:ResourceLeaseControl,claim:SliceClaimV1)=>{await markBuildCapacityPhaseV1("rebuild");return build(store,control,claim);};');
 expect(body.match(/upgradeBuildCapacity/gu)?.length).toBe(1);
});

test("a process under no grant asks for nothing, one that only inherits a live grant asks its holder before a rebuild although its phase marks are off",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"q16g-")),id=newHostCapacityIdV1(),now=Date.now();
 try{
  mkdirSync(join(directory,"grants"),{recursive:true});mkdirSync(join(directory,"queue"),{recursive:true});
  writeFileSync(join(directory,"grants",id+".json"),JSON.stringify({id,operation:"run",jobs:8,cpu:8,reservedMegabytes:3000,admittedAtMs:now,observedMegabytes:null,pid:process.pid,bootAtMs:hostBootAtV1(directory,now)}));
  const inheriting={SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_HOST_CAPACITY_GRANT:id,SEMIO_HOST_CAPACITY_PHASE_MARKS:"off"};
  expect(await upgradeBuildCapacityBeforeRebuildV1({})).toBe(false);
  expect(await upgradeBuildCapacityBeforeRebuildV1({SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_HOST_CAPACITY_GRANT:newHostCapacityIdV1(),SEMIO_HOST_CAPACITY_PHASE_MARKS:"off"}),"a grant that is gone").toBe(false);
  expect(await upgradeBuildCapacityBeforeRebuildV1(inheriting)).toBe(true);
  expect(existsSync(hostUpgradePathV1(directory,id)),"the run is recorded as one that executed work").toBe(true);
  expect(await markBuildCapacityPhaseV1("rebuild",inheriting),"no phase mark under an inherited grant").toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
