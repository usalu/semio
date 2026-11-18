/** 🎯️ Contract: the roster request of an invocation (`*:*`) names the tasks the invocation selected, not every target of every project of the slice: a `run-many` of literal projects and targets plans exactly those tasks and what they need, so its scoped plan stays far below the closed limit of the scope (g1-m08, 2026-11-17: 6276 tasks of 259 projects, 4443 scope targets, refused as "Closed scoped plan required" after 25 minutes of reading).
 * @see ../../🔌️ports/🧭️plan/🟨️.mjs
 * @see ../../../🧭️invocation/🛰️plan/🟦️.ts */
import{expect,test}from"bun:test";
import{createGraphPlannerPortV1}from"../../🔌️ports/🧭️plan/🟨️.mjs";
import{scopedPlanOfReplyV1}from"../../../🧭️invocation/🛰️plan/🟦️.ts";

const project=(name:string,targets:Record<string,unknown>)=>({name,root:name,targets,metadata:{}});
const slice=(selection:unknown,extra=0)=>{
 const projects:Record<string,unknown>={a:project("a",{build:{dependsOn:["^build"]},test:{dependsOn:["build"]},lint:{}}),b:project("b",{build:{},test:{},docs:{}}),c:project("c",{build:{},test:{}})};
 for(let at=0;at<extra;at++)projects["p"+at]=project("p"+at,{build:{},test:{},lint:{}});
 return{version:1,projects,dependencies:[{source:"a",target:"b"}],admission:{selection,registry:"r",closure:"c"}};
};
const roster=async(selection:unknown,extra=0)=>{
 const graph=slice(selection,extra),planner=createGraphPlannerPortV1({identify:async()=>"k".repeat(64),loadSlice:async()=>({graph})});
 const index=await planner.load({root:"/w",workspace:"w"});
 return planner.plan({index:index as never,targets:["*:*"],root:"/w",workspace:"w",agent:"a",arguments:["run-many"],cursor:"c"} as never);
};

test("a run-many of literal projects and targets plans those tasks and what they need, and a project without the target contributes nothing",async()=>{
 expect((await roster({kind:"owners",projects:["a"],exclude:[],targets:["build"]})).tasks.map(task=>task.id)).toEqual(["b:build","a:build"]);
 expect((await roster({kind:"owners",projects:["a","c"],exclude:["c"],targets:["build","docs"]})).tasks.map(task=>task.id)).toEqual(["b:build","a:build"]);
 expect((await roster({kind:"owners",projects:["a","c"],exclude:[],targets:["docs"]})).tasks).toEqual([]);
});

test("the whole selection and an owner selection without targets keep the roster of every task of the slice",async()=>{
 const every=["a:build","a:lint","a:test","b:build","b:docs","b:test","c:build","c:test"];
 expect((await roster({kind:"all"})).tasks.map(task=>task.id).sort()).toEqual(every);
 expect((await roster({kind:"owners",projects:["a"],exclude:[]})).tasks.map(task=>task.id).sort()).toEqual(every);
});

test("three selected projects of a slice of thousands of tasks give a scoped plan of their own tasks, inside the closed limit of the scope",async()=>{
 const reply=await roster({kind:"owners",projects:["a","c","p7"],exclude:[],targets:["build"]},2500);
 expect(reply.tasks.map(task=>task.id).sort()).toEqual(["a:build","b:build","c:build","p7:build"]);
 const scoped=scopedPlanOfReplyV1({arguments:["run-many"],tool:"nx",targets:["*:*"]} as never,reply.tasks as never);
 expect(scoped.scope.targets.length).toBeLessThanOrEqual(4);
});
