/** 📐️ Contract: the width of the project farm is the number of children that fit in the hard share of the tree (parent footprint, then one child at its cap beside the others at their typical footprint); an independent Python model computes the same table from the declared schemas.
 * @see ../../🧬️schema/🔣️.json */
import{expect,test}from"bun:test";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{PLAN_STAGE_POLICY_V1,farmHostMegabytesV1,farmWidthV1,grantedHostMegabytesV1,planFarmWorkersV1}from"../../🟨️.mjs";
import fixture from"./🔣️.json";

const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1(),child=PLAN_STAGE_POLICY_V1.farmChildFootprintMegabytes["project-farm"];

test("free memory above the floor widens the farm beyond a shrunk reservation, never beyond the host, and a missing reading changes nothing",()=>{
 const environment={[PLAN_STAGE_POLICY_V1.grantEnvironment]:"2304"},floor=PLAN_STAGE_POLICY_V1.availableFloorMegabytes;
 expect(farmWidthV1({hostMegabytes:farmHostMegabytesV1(32768,environment),child})).toBe(0);
 expect(farmWidthV1({hostMegabytes:farmHostMegabytesV1(32768,environment,floor),child})).toBe(0);
 expect(farmWidthV1({hostMegabytes:farmHostMegabytesV1(32768,environment,12000),child})).toBeGreaterThanOrEqual(5);
 expect(farmHostMegabytesV1(16384,environment,1e9)).toBe(16384);
 expect(farmHostMegabytesV1(32768,{},12000)).toBe(32768);
 expect(planFarmWorkersV1({workerMegabytes:1536,environment,hostMegabytes:32768,kind:"project-farm",availableMegabytes:12000})).toBeGreaterThanOrEqual(5);
 expect(planFarmWorkersV1({workerMegabytes:1536,environment,hostMegabytes:32768,kind:"project-farm"})).toBe(0);
});

test("the farm width equals the independent model on every host of the table, and the planner uses it for the project farm",()=>{
 expect(python,"python3 is required for the independent model").toBeTruthy();
 const model=JSON.parse(Bun.spawnSync([python!,"-I",join(here,"🐍️.py"),join(here,"🔣️.json"),join(here,"..","..","🧬️schema","🔣️.json"),join(here,"..","..","..","🦶️footprint","🧬️schema","🔣️.json")]).stdout.toString()) as number[];
 expect(model.length).toBe(fixture.cases.length);
 fixture.cases.forEach((row,at)=>{
  const environment={[PLAN_STAGE_POLICY_V1.grantEnvironment]:String((row as {grant?:number}).grant??"")};
  expect(farmWidthV1({hostMegabytes:farmHostMegabytesV1(row.host,environment,(row as {available?:number}).available),reservedMegabytes:row.others,child}),"width on "+row.host+" MB with "+row.others+" held and "+((row as {grant?:number}).grant??"no")+" granted").toBe(model[at]!);
 });
 for(const row of fixture.cases.filter(each=>each.others===0&&!("available" in each)&&grantedHostMegabytesV1(each.host,{[PLAN_STAGE_POLICY_V1.grantEnvironment]:String((each as {grant?:number}).grant??"")})<PLAN_STAGE_POLICY_V1.serveHostMegabytes))expect(planFarmWorkersV1({workerMegabytes:1536,hostMegabytes:row.host,environment:{SEMIO_SLICE_LEAN:"off",...(row as {grant?:number}).grant===undefined?{}:{[PLAN_STAGE_POLICY_V1.grantEnvironment]:String((row as {grant:number}).grant)}},kind:"project-farm"})).toBe(model[fixture.cases.indexOf(row)]!);
 const granted=fixture.cases.filter(each=>"grant" in each).map(each=>model[fixture.cases.indexOf(each)]!);
 expect(granted[0]!,"a rebuild granted only its minimum runs a narrower farm than the machine could").toBeLessThan(granted[2]!);
 expect(granted[3]!,"a grant above the machine changes nothing").toBe(model[3]!);
 expect(model[3],"a 16 GB machine (a 6553 MB plan ceiling) runs four children beside the parent").toBe(4);
 expect(model[1],"an 8 GB machine runs its one child").toBe(1);
 expect(model[0],"a 4 GB machine runs none").toBe(0);
});
