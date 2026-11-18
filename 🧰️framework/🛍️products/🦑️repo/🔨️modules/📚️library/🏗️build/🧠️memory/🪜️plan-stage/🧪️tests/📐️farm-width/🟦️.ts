/** 📐️ Contract: the width of the project farm is the number of children that fit in the hard share of the tree (parent footprint, then one child at its cap beside the others at their typical footprint); an independent Python model computes the same table from the declared schemas.
 * @see ../../🧬️schema/🔣️.json */
import{expect,test}from"bun:test";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{PLAN_STAGE_POLICY_V1,farmWidthV1,planFarmWorkersV1}from"../../🟨️.mjs";
import fixture from"./🔣️.json";

const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1(),child=PLAN_STAGE_POLICY_V1.farmChildFootprintMegabytes["project-farm"];

test("the farm width equals the independent model on every host of the table, and the planner uses it for the project farm",()=>{
 expect(python,"python3 is required for the independent model").toBeTruthy();
 const model=JSON.parse(Bun.spawnSync([python!,"-I",join(here,"🐍️.py"),join(here,"🔣️.json"),join(here,"..","..","🧬️schema","🔣️.json"),join(here,"..","..","..","🦶️footprint","🧬️schema","🔣️.json")]).stdout.toString()) as number[];
 expect(model.length).toBe(fixture.cases.length);
 fixture.cases.forEach((row,at)=>{
  expect(farmWidthV1({hostMegabytes:row.host,reservedMegabytes:row.others,child}),"width on "+row.host+" MB with "+row.others+" held").toBe(model[at]!);
 });
 for(const row of fixture.cases.filter(each=>each.others===0))expect(planFarmWorkersV1({workerMegabytes:1536,hostMegabytes:row.host,environment:{SEMIO_SLICE_LEAN:"off"},kind:"project-farm"})).toBe(model[fixture.cases.indexOf(row)]!);
 expect(model[3],"a 16 GB machine (a 6553 MB plan ceiling) runs four children beside the parent").toBe(4);
 expect(model[1],"an 8 GB machine runs its one child").toBe(1);
 expect(model[0],"a 4 GB machine runs none").toBe(0);
});
