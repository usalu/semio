/** 🍂️ Contract: a process that opens a generation store (a graph child: the cold build is one store, the delta refresh a fresh one) verifies through a view of the fact index in which the rows its plan read stay materialized; that view is released before the refresh computes, so the inference never carries the rows its plan verified; a refresh whose custody waited and planned again still builds the disk, and the graph equals the independent Python model of the workspace. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {GENERATION_POLICY_V1,canonicalJsonV1,createGenerationStoreV1} from "../../🟨️.mjs";
import {IDENTITY,IMPLEMENTATION,createCompute,createHolds,createMembership,ringFiles,seed} from "../🏃️world/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"../🐍️.py"),python=Bun.which("python3")??Bun.which("python");
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-view-release-")),sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
const pythonGraph=(dir:string)=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,"graph",dir]).stdout.toString());
const normalized=(value:unknown)=>JSON.parse(canonicalJsonV1(value));

function setup(dir:string,now?:()=>number){
 const holds=createHolds(dir),membership=createMembership(dir),inner=createCompute(dir,[]),seen:{views:number;opened:number}[]=[];
 let store:ReturnType<typeof createGenerationStoreV1>;
 const compute=async(spec:unknown,signal?:AbortSignal)=>{const current=await store.read(signal);if(current?.manifest){const before=store.stats.indexViews;await store.readIndexView(current.manifest.factIndex);seen.push({views:before,opened:store.stats.indexViews-before});}return(inner as (spec:unknown,signal?:AbortSignal)=>Promise<unknown>)(spec,signal);};
 store=createGenerationStoreV1({root:join(dir,".gen"),holds,resources:{facts:"repository-graph-facts:view-release",builder:"repository-graph-build:view-release"},...now?{now}:{}});
 return{store,seen,base:{identity:IDENTITY,implementation:IMPLEMENTATION,membership,compute}};
}

for(const scope of["closure","whole-workspace"] as const)test("a "+scope+" delta refresh computes without the verification view its plan opened, and builds the disk",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(8));
  const refresh=({store,base}:ReturnType<typeof setup>)=>scope==="closure"?store.refreshClosure({...base,roots:["p0"],requestedAt:Date.now()} as never):store.refresh({...base,requestedAt:Date.now()} as never);
  expect((await refresh(setup(dir))).published).toBe(true);
  await sleep(20);
  await writeFile(join(dir,"pkg/p3/src/index.ts"),"import \"@syn/p4\";\nexport const edited=1;\n");
  const child=setup(dir),{store,seen}=child,result=await refresh(child);
  expect(store.stats.indexViews).toBeGreaterThanOrEqual(1);
  expect(result.built).toBe(true);expect(result.published).toBe(true);
  expect(seen).toEqual([{views:seen[0]!.views,opened:1}]);
  const graph=await store.assemble(result.current,{});const{generation:_g,membership:_m,...rest}=graph;
  expect(normalized(rest)).toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("[adversarial] a custody that waited beyond the bound plans again, still computes without a cached view and builds the edit made during the wait",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(8));
  const clock={at:Date.now()},first=setup(dir,()=>clock.at);
  await first.store.refreshClosure({...first.base,roots:["p0"],requestedAt:clock.at} as never);clock.at+=10;
  await sleep(20);
  await writeFile(join(dir,"pkg/p3/src/index.ts"),"import \"@syn/p4\";\nexport const edited=1;\n");
  const{store,seen,base}=setup(dir,()=>clock.at);
  const builder=async(body:()=>Promise<unknown>)=>{clock.at+=GENERATION_POLICY_V1.hintMaximumAgeMilliseconds+1;await sleep(20);await writeFile(join(dir,"pkg/p5/src/index.ts"),"import \"@syn/p6\";\nexport const edited=2;\n");return body();};
  const result=await store.refreshClosure({...base,roots:["p0"],requestedAt:clock.at,builder} as never);
  expect(result.built).toBe(true);expect(seen.map(row=>row.opened)).toEqual([1]);
  const graph=await store.assemble(result.current,{});const{generation:_g,membership:_m,...rest}=graph;
  expect(normalized(rest),"the generation equals the disk after the wait").toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
});
