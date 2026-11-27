/** 🪜️ Contract: a command of the Nx path for a project in a dependency scope that is not installed installs that scope itself, before Nx starts, with no `repo dependencies sync --scope` by a human. The scopes are the owners of the project and of every project it transitively depends on; the task graph is the one of the caller. Cross-checked against an independent breadth-first closure in Python.
 * @see ../../🔌️backend/🟦️.ts */
import{expect,mock,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{join}from"node:path";
import{readScriptPolicy}from"../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts";

process.env.SEMIO_BUILD_MODE="prototype";
const calls:{owners:string[];order:string}[]=[],order:string[]=[];
mock.module(join(import.meta.dir,"../../../../🚀️bootstrap/🟦️.ts"),()=>({
 ensureBuildScopesV1:async(_root:string,_invocation:unknown,_phase:unknown,projectRoots:()=>readonly string[])=>{order.push("ensure");calls.push({owners:[...projectRoots()],order:order.join(">")});return [];},
}));
mock.module(join(import.meta.dir,"../../../../../⚡️caching/🚀️bootstrap/📜️script.ts"),()=>({publishBootstrapSources:()=>undefined,runOwnedNxInvocationV1:async()=>{order.push("nx");}}));
mock.module(join(import.meta.dir,"../../../../🧾️store-receipt/🟦️.ts"),()=>({storeReceiptTasksV1:async()=>[],withStoreReceiptsV1:async(input:{run:()=>Promise<void>})=>input.run()}));
const{backend}=await import("../../🔌️backend/🟦️.ts");

const target={execution:{kind:"command" as const,command:"true"}};
const graph=(()=>{const make=(name:string,root:string)=>({name,root,targets:{build:target}});return{version:1 as const,projects:{app:make("app","✏️s/app"),lib:make("lib","✏️s/lib"),base:make("base","🧰️x/base"),other:make("other","🌙️y/other")},dependencies:[{source:"app",target:"lib"},{source:"lib",target:"base"}]};})();
const invocation=Object.freeze({policy:readScriptPolicy({version:1,owner:"test",maximumElapsedMilliseconds:0}),control:Object.freeze({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined}),capabilities:Object.freeze({})}) as never;

test("a run of a project in an uninstalled scope installs the scopes of its closure before nx starts",async()=>{
 calls.length=0;order.length=0;
 await backend.execute({root:"/w",invocation,phase:{} as never,graph:graph as never,args:["run","app:build"],tool:"nx",release:async()=>undefined} as never);
 expect(calls.length).toBe(1);
 expect(calls[0]!.owners).toEqual(["✏️s/app","✏️s/lib","🧰️x/base"]);
 expect(order).toEqual(["ensure","nx"]);
});

test("the closure equals an independent breadth-first walk on a grid of start sets",()=>{
 const edges=graph.dependencies,starts=[["app"],["lib"],["base"],["other"],["app","other"]];
 const script=`
import json,sys
edges=json.loads(sys.argv[1]);starts=json.loads(sys.argv[2]);out=[]
for s in starts:
    seen=set();queue=list(s)
    while queue:
        n=queue.pop(0)
        if n in seen: continue
        seen.add(n);queue+= [e["target"] for e in edges if e["source"]==n]
    out.append(sorted(seen))
print(json.dumps(out))`;
 const run=spawnSync("python3",["-I","-c",script,JSON.stringify(edges),JSON.stringify(starts)],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const names=JSON.parse(run.stdout) as string[][];
 return import("../../../../🧭️selection/🟦️.ts").then(({buildTaskClosureRootsV1})=>{
  starts.forEach((start,index)=>expect(buildTaskClosureRootsV1(graph as never,start.map(name=>name+":build"))).toEqual(names[index]!.map(name=>(graph.projects as Record<string,{root:string}>)[name]!.root).sort()));
 });
});
