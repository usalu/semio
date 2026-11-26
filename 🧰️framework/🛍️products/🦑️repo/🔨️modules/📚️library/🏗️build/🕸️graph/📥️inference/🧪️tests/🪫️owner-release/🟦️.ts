/** 🪫️ Contract: the state the owners of a plan leave in the cache they share is released at the declared interval whatever a footprint reading says, so a worker that runs every owner holds at most one interval of owner state (its managed memory is bounded by the number of owners per release times what one owner leaves, not by the number of owners of the plan), and a footprint reading of a process never stands below the bytes its own runtime holds, so a host that compresses the pages of a busy worker cannot make the gate blind; an independent Python model replays the schedule and the decisions.
 * @see ../../🧭️script-inputs/🟨️.mjs `releaseOwnerStateV1`
 * @see ../../../../🧠️memory/🦶️footprint/🟨️.mjs `footprintAtLeastManagedV1` */
import{expect,test}from"bun:test";
import{join}from"node:path";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{createScriptInputCache,lookReleaseDueV1,releaseAfterTakeLookV1,releaseOwnerStateV1}from"../../🧭️script-inputs/🟨️.mjs";
import{INFERENCE_LIMITS_V1}from"../../🧬️schema/🟨️.mjs";
import{decideFootprintV1,footprintAtLeastManagedV1,type FootprintDecisionV1}from"../../../../🧠️memory/🦶️footprint/🟨️.mjs";
import fixture from"./🔣️.json";

const MiB=1048576;
const model=()=>{
 const run=Bun.spawnSync([testPythonV1()!,"-I",join(import.meta.dir,"🐍️.py")],{stdin:Buffer.from(JSON.stringify({...fixture,interval:INFERENCE_LIMITS_V1.ownersPerRelease,lookReleaseGrowthBytes:INFERENCE_LIMITS_V1.lookReleaseGrowthBytes}))});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 return JSON.parse(run.stdout.toString()) as{held:number[];peak:number;readings:{floored:number;decision:FootprintDecisionV1}[];looks:boolean[]};
};

test("a worker whose footprint reading stands far below what it holds still releases its owner state at the declared interval",()=>{
 const expected=model(),scripts=createScriptInputCache({workspaceRoot:"/w"}) as any;
 const blind={grantBytes:()=>fixture.grantMegabytes*MiB,footprintBytes:()=>fixture.blindFootprintMegabytes*MiB};
 expect(decideFootprintV1({footprintBytes:blind.footprintBytes(),grantBytes:blind.grantBytes()})).toBe("run");
 const held:number[]=[];
 for(let owner=1;owner<=fixture.owners;owner++){
  for(let entry=0;entry<fixture.entriesPerOwner;entry++)scripts.files.set("o"+owner+"/f"+entry,{owner,entry});
  releaseOwnerStateV1(scripts,{gate:blind});
  held.push(scripts.files.size);
 }
 expect(held).toEqual(expected.held);
 expect(Math.max(...held)).toBe(expected.peak);
 expect(expected.peak).toBeLessThanOrEqual(INFERENCE_LIMITS_V1.ownersPerRelease*fixture.entriesPerOwner);
 expect(expected.peak).toBeLessThan(fixture.owners*fixture.entriesPerOwner);
});

test("a footprint reading of the process is raised to the bytes its runtime holds, and the decision follows the raised reading",()=>{
 const expected=model();
 fixture.readings.forEach((row,at)=>{
  const floored=footprintAtLeastManagedV1(row.footprint*MiB,()=>row.managed*MiB);
  expect(floored).toBe(expected.readings[at]!.floored*MiB);
  expect(decideFootprintV1({footprintBytes:floored,grantBytes:fixture.grantMegabytes*MiB})).toBe(expected.readings[at]!.decision);
 });
 expect(footprintAtLeastManagedV1(undefined,()=>fixture.managedMegabytes*MiB)).toBeUndefined();
 expect(footprintAtLeastManagedV1(Number.NaN,()=>fixture.managedMegabytes*MiB)).toBeNaN();
});

test("the release intervals are declared limits with a description in both languages",async()=>{
 const schema=(await import("../../🧬️schema/🔣️.json")).default as{definitions:{Limits:{const:Record<string,number>};LimitsDescriptions:{const:{en:Record<string,string>;de:Record<string,string>}}}};
 for(const key of["ownersPerRelease","ownersPerShrinkRelease","lookReleaseGrowthBytes"]){
  expect(schema.definitions.Limits.const[key]).toBeGreaterThan(0);
  expect(schema.definitions.LimitsDescriptions.const.en[key]!.length).toBeGreaterThan(20);
  expect(schema.definitions.LimitsDescriptions.const.de[key]!.length).toBeGreaterThan(20);
 }
 expect(INFERENCE_LIMITS_V1.ownersPerShrinkRelease).toBeLessThan(INFERENCE_LIMITS_V1.ownersPerRelease);
});

test("a look releases the shared cache by the growth of the managed memory beyond its lowest reading, not by a count of looks",()=>{
 const expected=model();let lowest:number|undefined;
 const due=fixture.lookManagedMegabytes.map((managed,at)=>{
  lowest=lowest===undefined?managed*MiB:Math.min(lowest,managed*MiB);
  const release=lookReleaseDueV1(lowest,managed*MiB,{shrink:fixture.lookShrink[at]});
  if(release)lowest=undefined;
  return release;
 });
 expect(due).toEqual(expected.looks);
 expect(due.filter(Boolean).length).toBeGreaterThan(0);expect(due.filter(Boolean).length).toBeLessThan(due.length);
 const scripts=createScriptInputCache({workspaceRoot:"/w"}) as any;
 for(let look=0;look<INFERENCE_LIMITS_V1.ownersPerRelease*3;look++){scripts.files.set("f"+look,look);releaseAfterTakeLookV1(scripts,{gate:undefined});}
 expect(scripts.files.size).toBe(INFERENCE_LIMITS_V1.ownersPerRelease*3);
 releaseAfterTakeLookV1(scripts,{shrink:true});
 expect(scripts.files.size).toBe(0);
});
