/** 🔓️ Pool lock-order laws over `./🧫️fixtures/🔣️.json`: no role of the new design blocks on a lease while it holds the exclusive `nx:graph-pool` and the wait-for graph is acyclic (independent Python graphlib model `./🐍️.py`; the old design must fail the same check, so it is not vacuous); the source keeps the order the model assumes: the namespace lease is taken before the pool pass, attachment and eviction never wrap retirement in the pool, victims are only try-locked. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";

const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"./🧫️fixtures/🔣️.json"),"utf8")) as any;
const source=readFileSync(resolve(import.meta.dir,"../../🟦️.ts"),"utf8");

type Step=readonly string[];
function model(design:Record<string,readonly Step[]>):{cycle:boolean;poolHoldersWaiting:string[][]}{
 const edges=new Map<string,Set<string>>(),waiting:string[][]=[];
 for(const [role,steps] of Object.entries(design)){
  const held:string[]=[];
  for(const step of steps){
   if(step[0]==="acquire"){
    if(step[2]==="wait"){for(const resource of held){if(!edges.has(step[1]!))edges.set(step[1]!,new Set());edges.get(step[1]!)!.add(resource);}if(held.includes("pool"))waiting.push([role,step[1]!]);}
    held.push(step[1]!);
   }else held.splice(held.indexOf(step[1]!),1);
  }
 }
 const state=new Map<string,number>();let cycle=false;
 const visit=(node:string):void=>{if(state.get(node)===1){cycle=true;return;}if(state.get(node)===2)return;state.set(node,1);for(const next of edges.get(node)??[])visit(next);state.set(node,2);};
 for(const node of edges.keys())visit(node);
 return {cycle,poolHoldersWaiting:waiting};
}

for(const name of ["before","after"]){
 test("lock-order model agrees with the independent Python model: "+name,()=>{
  const ours=model(fixture.designs[name]),python=spawnSync(testPythonV1()??"python3",["-I",resolve(import.meta.dir,"./🐍️.py"),resolve(import.meta.dir,"./🧫️fixtures/🔣️.json"),name],{encoding:"utf8"});
  expect(python.error).toBeUndefined();expect(python.status).toBe(0);
  expect(JSON.parse(python.stdout)).toEqual(ours);
 });
}
test("the old design blocks while holding the pool, the new design never does and has no wait cycle",()=>{
 expect(model(fixture.designs.before).poolHoldersWaiting.length).toBeGreaterThan(0);
 const after=model(fixture.designs.after);expect(after.poolHoldersWaiting).toEqual([]);expect(after.cycle).toBe(false);
});

function body(start:string):string{
 const first=source.indexOf(start);expect(first).toBeGreaterThan(-1);
 const next=source.indexOf("\n/** ",first+start.length),line=source.indexOf("\n",first);
 return source.slice(first,Math.min(...[next,source.length].filter(value=>value>first)));
}
test("source: the namespace lease precedes the pool pass of an acquisition and the old room walk is gone",()=>{
 const text=body(fixture.sourceLaws.acquireFunction);
 expect(text.indexOf("acquireResourceLease(")).toBeGreaterThan(-1);
 expect(text.indexOf("acquireResourceLease(")).toBeLessThan(text.indexOf("poolWithRoom("));
 expect(text.indexOf("attachExistingNamespaceV1(")).toBeLessThan(text.indexOf("poolWithRoom("));
 expect(source.includes(fixture.sourceLaws.removed)).toBe(false);expect(source.includes("await room(")).toBe(false);
});
test("source: inside a pool pass the namespace lease is only try-locked",()=>{
 const pass=body("async function tryNamespaceLeaseV1");expect(pass).toContain("timeoutMs:0");
 const text=body(fixture.sourceLaws.acquireFunction);expect(text).toContain("tryNamespaceLeaseV1(store,directory)");
 expect(text.indexOf("poolWithRoom(")).toBeLessThan(text.indexOf("tryNamespaceLeaseV1(store,directory)"));
});
test("source: attachment takes no pool and eviction only reads the roster under it and only try-locks victims",()=>{
 const attach=body(fixture.sourceLaws.attachFunction);expect(attach).not.toContain("pool(");expect(attach).toContain("observeNativeStorageStateV1(");expect(attach).toContain('phase!=="committed"');
 const evict=body(fixture.sourceLaws.evictFunction);
 expect(evict.split("pool(store").length-1).toBe(fixture.sourceLaws.evictPoolCalls);
 expect(evict.split("timeoutMs:0").length-1).toBe(fixture.sourceLaws.evictTryLocks);
 expect(evict.indexOf("rmSync(")).toBeGreaterThan(evict.indexOf("timeoutMs:0"));
 expect(evict).toContain("acquireQueuedResourceLease(");
});

test("source: every user of the fixed cache image holds it shared",()=>{
 const nx=readFileSync(resolve(import.meta.dir,"../../../🟦️.ts"),"utf8");
 expect(nx).toContain('resources:[{resource:"nx:fixed-cache:"+image.root,mode:"shared"}]');expect(nx).not.toContain('resource:"nx:fixed-cache:"+image.root,mode}');
});
