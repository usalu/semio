/** 📏️ Contract: the closed scoped plan holds a plan of the whole workspace and refuses any other violation of its schema as a typed bilingual refusal, built where the plan is built (g1-m07, 2026-11-17 23:11 UTC: `/scope/targets must contain at most 4096 item(s)` surfaced as an internal error after 800 s). The fixed bound of a collection that grows with the workspace is declared in the workspace scale family; Ajv (third party) validates the same plans without `uniqueItems`, whose deep comparison is quadratic in Ajv (uniqueness is checked by our own linear validator).
 * @see ../../🛰️plan/🟦️.ts
 * @see ../../../📏️workspace-scale/🧬️schema/🔣️.json */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import scopeSchema from "../../../🧭️selection/🎯️scope/🧬️schema/🔣️.json" with {type:"json"};
import scaleDeclaration from "../../../📏️workspace-scale/🧬️schema/🔣️.json" with {type:"json"};
import {isRepoErrorV1,renderRepoErrorV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import {scopedPlanOfReplyV1} from "../../🛰️plan/🟦️.ts";

const family=scaleDeclaration.definitions.Family.const,owners=family.workspace.owners,perOwner=family.workspace.tasksPerOwner;
const wholeWorkspace=(count:number)=>Array.from({length:count},(_,index)=>({id:"p"+Math.floor(index/perOwner)+":t"+(index%perOwner),dependsOn:index%perOwner===0?[]:["p"+Math.floor(index/perOwner)+":t"+(index%perOwner-1)],stale:true}));

test("a plan of every task of the whole workspace is a closed scoped plan, every task a scope target of its own when nothing depends on it",()=>{
 const tasks=Array.from({length:owners*perOwner},(_,index)=>({id:"p"+Math.floor(index/perOwner)+":t"+(index%perOwner),dependsOn:[] as string[],stale:true})),request={arguments:["run-many"],tool:"nx",targets:["*:*"]};
 const began=Date.now(),plan=scopedPlanOfReplyV1(request,tasks);
 expect(plan.scope.targets.length).toBe(tasks.length);
 expect(plan.scope.targets.length).toBeGreaterThan(4096);
 expect(Date.now()-began,"a whole-workspace plan is built in seconds").toBeLessThan(20000);
 const withoutUnique=(node:any):any=>Array.isArray(node)?node.map(withoutUnique):node&&typeof node==="object"?Object.fromEntries(Object.entries(node).filter(([key])=>key!=="uniqueItems").map(([key,value])=>[key,withoutUnique(value)])):node;
 const ajv=new Ajv({strict:false}),validate=ajv.compile({...withoutUnique(scopeSchema),$ref:"#/definitions/ScopedPlan"});
 expect(validate(plan),JSON.stringify(validate.errors?.slice(0,2))).toBe(true);
});

test("a chained plan of the whole workspace keeps exactly its tasks and edges",()=>{
 const tasks=wholeWorkspace(owners*perOwner),plan=scopedPlanOfReplyV1({arguments:["run-many"],tool:"nx",targets:["*:*"]},tasks);
 expect(plan.tasks.length).toBe(tasks.length);
 expect(plan.edges.length).toBe(tasks.length-owners);
 expect(plan.projects.length).toBe(owners);
 expect(plan.scope.targets.length).toBe(owners);
});

test("a plan that breaks the closed schema is a typed refusal in both languages with the rule, the path and the counts",()=>{
 const tasks=[{id:"p".repeat(300)+":build",dependsOn:[] as string[],stale:true}];
 let caught:unknown;
 try{scopedPlanOfReplyV1({arguments:["run"],tool:"nx",targets:[tasks[0]!.id]},tasks);}catch(error){caught=error;}
 expect(isRepoErrorV1(caught),String(caught)).toBe(true);
 const english=renderRepoErrorV1(caught as never,"en"),german=renderRepoErrorV1(caught as never,"de");
 expect(english).toContain("at most 256 character(s)");
 expect(english).toContain("1 tasks");
 expect(german).toContain("1 Aufgaben");
 expect(german).not.toBe(english);
});
