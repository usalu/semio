/** 🧪️ Scoped planning: the closure of a requested target equals the restriction of the whole-workspace plan, matches an independent Python oracle and actual Nx, and reads no project outside the closure. */
import {test,expect} from "bun:test";
import {join} from "node:path";
import Ajv from "ajv";
import {buildPatternV1} from "../../🟦️.ts";
import {planScopedTargetsV1,type ScopeNodeV1,type ScopeSourceV1,type TargetScopeV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
type Case=typeof fixture.cases[number]&{error?:string;nx?:boolean;tasks?:string[];projects?:string[];readsAtMost?:number};
const projects=fixture.projects as unknown as Record<string,ScopeNodeV1>,cases=fixture.cases as Case[],ids=(plan:{tasks:readonly {project:string;target:string}[]})=>plan.tasks.map(row=>row.project+":"+row.target);
const counting=(reads:string[]):ScopeSourceV1=>({select:patterns=>Object.keys(projects).filter(name=>patterns.some(pattern=>buildPatternV1(pattern,name))),node:async name=>{reads.push(name);return projects[name];}});

test("fixture and every plan validate against the schema",async()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(schema,"scope");
 for(const row of cases.filter(row=>!row.error))expect(ajv.validate("scope#/definitions/TargetScope",row.scope),row.name).toBe(true);
 for(const row of cases.filter(row=>!row.error))expect(ajv.validate("scope#/definitions/ScopedPlan",await planScopedTargetsV1(counting([]),row.scope)),row.name).toBe(true);
});

test("a scoped plan has the expected closure and reads only that closure",async()=>{
 for(const row of cases.filter(row=>!row.error)){
  const reads:string[]=[],plan=await planScopedTargetsV1(counting(reads),row.scope);
  expect(ids(plan),row.name).toEqual(row.tasks!);expect([...plan.projects],row.name).toEqual(row.projects!);
  expect(plan.projectsRead,row.name).toBeLessThanOrEqual(row.readsAtMost!);expect(new Set(reads).size,row.name).toBe(plan.projectsRead);
  expect(reads.filter(name=>!row.projects!.includes(name)),row.name).toEqual([]);
 }
});

test("refusals are typed and bilingual",async()=>{
 for(const row of cases.filter(row=>row.error))await expect(planScopedTargetsV1(counting([]),row.scope),row.name).rejects.toThrow(row.error!);
 await expect(planScopedTargetsV1(counting([]),cases[0]!.scope,{maximumTasks:1})).rejects.toThrow("Kapazität");
 const controller=new AbortController(),reason=new Error("cancelled");controller.abort(reason);
 await expect(planScopedTargetsV1(counting([]),cases[0]!.scope,{signal:controller.signal})).rejects.toThrow(reason);
 const seen:number[]=[];await planScopedTargetsV1(counting([]),cases[0]!.scope,{progress:update=>seen.push(update.projectsRead)});expect(seen.length).toBeGreaterThan(0);expect(seen).toEqual([...seen].sort((a,b)=>a-b));
});

test("the scoped plan equals the restriction of the whole-workspace plan",async()=>{
 const every:TargetScopeV1={version:1,includeToolClosure:false,targets:Object.entries(projects).flatMap(([project,node])=>Object.keys(node.targets).filter(target=>!(project==="bridge")).map(target=>({project,target})))};
 const whole=await planScopedTargetsV1(counting([]),every),next=new Map<string,string[]>();
 for(const edge of whole.edges){const from=edge.from.project+":"+edge.from.target,rows=next.get(from)??[];rows.push(edge.to.project+":"+edge.to.target);next.set(from,rows);}
 for(const row of cases.filter(row=>!row.error)){
  const scoped=await planScopedTargetsV1(counting([]),row.scope),reach=new Set(ids({tasks:row.scope.targets}));
  const work=[...reach];for(let index=0;index<work.length;index++)for(const to of next.get(work[index]!)??[])if(!reach.has(to)){reach.add(to);work.push(to);}
  const restricted=[...reach].sort(),edges=whole.edges.filter(edge=>reach.has(edge.from.project+":"+edge.from.target));
  expect(ids(scoped),row.name).toEqual(restricted);expect(scoped.edges,row.name).toEqual(edges);
 }
});

test("the scoped plan agrees with the independent Python oracle",async()=>{
 const run=Bun.spawnSync(["/home/deck/semio/.venv/bin/python","-I",join(import.meta.dir,"🐍️oracle.py"),join(import.meta.dir,"🔣️.json")]),oracle=JSON.parse(run.stdout.toString());
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 for(const row of cases.filter(row=>!row.error)){const plan=await planScopedTargetsV1(counting([]),row.scope);expect({tasks:ids(plan),projects:[...plan.projects]},row.name).toEqual(oracle[row.name]);}
});
