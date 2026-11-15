/** 🏊️ Contract: what a read infers while it waits for the fleet is ONE continuous run (one prefetch with one plan stage entry and one farm open for the whole wait, projects in the given order, arrivals dropped between projects), never holds more worker processes than the memory of the plan allows (`planFarmWorkersV1`), ends them when the run ends (also when it fails) and runs as the named plan stage `inference` of the plan guard.
 * @see ../../🤝️prefetch/🟦️.ts */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{PLAN_STAGE_POLICY_V1,planFarmWorkersV1}from"../../../🧠️memory/🪜️plan-stage/🟨️.mjs";
import{GRAPH_FARM_POLICY_V1,planGraphFarmV1}from"../../📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.mjs";
import{inferRunV1,type OwnerHooksV1}from"../../🤝️prefetch/🟦️.ts";

const text=(path:string)=>readFileSync(new URL(path,import.meta.url),"utf8"),MAC={hostMegabytes:16384,environment:{}},WORKER=GRAPH_FARM_POLICY_V1.workerMegabytes;
const request=(count:number)=>({generation:1,dirty:Array.from({length:count},(_,at)=>`p${at}`),roots:Object.fromEntries(Array.from({length:count},(_,at)=>[`p${at}`,`r${at}`])),structural:false,implementation:"a".repeat(64)});

test("the farm of a plan is bounded by the share of the host its process tree may hold: a 16 GB Mac gets 4 workers beside a 1.3 GB plan, a serving machine and a disabled limit have none, a full process none",()=>{
 expect(planFarmWorkersV1({...MAC,workerMegabytes:WORKER,residentMegabytes:1300})).toBe(4);
 expect(planFarmWorkersV1({...MAC,workerMegabytes:WORKER,residentMegabytes:9000})).toBe(0);
 expect(planFarmWorkersV1({hostMegabytes:32768,environment:{},workerMegabytes:WORKER,residentMegabytes:1300})).toBe(Infinity);
 expect(planFarmWorkersV1({hostMegabytes:16384,environment:{[PLAN_STAGE_POLICY_V1.limitEnvironment]:PLAN_STAGE_POLICY_V1.off},workerMegabytes:WORKER,residentMegabytes:1300})).toBe(Infinity);
 const unbounded=planGraphFarmV1({projects:128,environment:{},cores:16}),bounded=planGraphFarmV1({projects:128,environment:{},cores:16,maximumWorkers:4});
 expect(unbounded.workers).toBeGreaterThan(4);expect(bounded).toEqual({workers:4,reason:"memory"});
 expect(planGraphFarmV1({projects:128,environment:{},cores:16,maximumWorkers:1})).toEqual({workers:0,reason:"memory"});
});

test("the whole cooperative wait is one prefetch: one plan stage entry, one farm open, never more workers than the farm allows, the order given, arrivals skipped between projects, workers ended once at the end, also when it fails",async()=>{
 const allowed=planFarmWorkersV1({...MAC,workerMegabytes:WORKER,residentMegabytes:1300}),source=request(300),ranked=[...source.dirty].reverse();
 for(const failing of[false,true]){
  let live=0,peak=0,opens=0,stages=0,prefetches=0,released=0;const done:string[]=[],skipped:string[]=[],arrived=new Set<string>();
  const prefetch=async(next:{dirty:readonly string[]},hooks:OwnerHooksV1)=>{
   prefetches++;stages++;opens++;
   const workers=planGraphFarmV1({projects:next.dirty.length,environment:{},cores:16,maximumWorkers:allowed}).workers;live+=workers;peak=Math.max(peak,live);
   const jobs=[...next.dirty].sort((a,b)=>hooks.rank!(a)-hooks.rank!(b));
   for(const[at,name]of jobs.entries()){
    if(at===100)for(const late of jobs.slice(150,200))arrived.add(late);
    if(hooks.skip!(name)){skipped.push(name);continue;}
    await Promise.resolve();hooks.done!(name);
    if(failing&&at===250)throw new Error("boom");
   }
  };
  const run=inferRunV1({request:source,names:ranked,prefetch,skip:name=>arrived.has(name),done:name=>done.push(name),release:async()=>{released++;live=0;}});
  if(failing)await expect(run).rejects.toThrow("boom");else await run;
  expect(prefetches).toBe(1);expect(stages).toBe(1);expect(opens).toBe(1);expect(released).toBe(1);expect(live).toBe(0);
  expect(peak).toBeGreaterThan(0);expect(peak).toBeLessThanOrEqual(allowed);expect(peak*WORKER+1300).toBeLessThan(16384*0.6);
  expect(done.slice(0,5)).toEqual(ranked.slice(0,5));expect(skipped.length).toBe(50);expect(new Set(done).size).toBe(done.length);
  for(const name of skipped)expect(done).not.toContain(name);
  expect(done.length+skipped.length).toBe(failing?251:300);
 }
});

test("the batches run as the plan stage inference, the farm is sized by the plan memory and the host ends the workers after every batch",()=>{
 const prefetch=text("../../🤝️prefetch/🟦️.ts"),part=prefetch.slice(prefetch.indexOf("export async function prefetchBuildGraphProjectsV1"),prefetch.indexOf("export type OfferedStateV1"));
 expect(part).toContain('createPlanStageGuardV1({refuse:detail=>refuseBuildV1("unserved","graph.plan-too-heavy",detail)})');
 expect(part).toContain("await stages.run(()=>observationV1.captureObservedSourceInputsV1(async()=>{\n    stages.mark(\"inference\");");
 expect(PLAN_STAGE_POLICY_V1.stages[0]).toBe("inference");
 const nodes=text("../../📥️inference/📦️project-nodes/🟨️.mjs");
 expect(nodes).toContain("maximumWorkers:planFarmWorkersV1({workerMegabytes:GRAPH_FARM_POLICY_V1.workerMegabytes})");
 expect(nodes).toContain("if(options?.skipOwner?.(job.name)===true){finished++;continue;}");expect(nodes).toContain("(rank===undefined?0:rank(a.name)-rank(b.name))");
 expect(part).toContain("hooks.done?.(name)");expect(part).toContain("skipOwner:hooks.skip");expect(part).toContain("ownerRank:hooks.rank");
 const host=text("../../🕰️host/🟦️.ts");
 expect(host).toContain("infer:({names,skip,done})=>inferRunV1({request,names,skip,done,prefetch:(next,hooks)=>this.prefetch(next,{hooks}),release:closeProjectFarmsV1})");
 expect(host).toContain("hooks:continuous.hooks,chunk:Number.MAX_SAFE_INTEGER");
 expect(text("../../🕰️generations/🧩️claims/🟨️.mjs")).toContain("chunk}");
});
