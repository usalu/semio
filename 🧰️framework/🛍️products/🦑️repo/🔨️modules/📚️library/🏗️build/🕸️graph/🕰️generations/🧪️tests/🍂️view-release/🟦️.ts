/** 🍂️ Contract: a process that opens a generation store (a graph child: the cold build is one store, the delta refresh a fresh one) verifies through a view of the fact index in which the rows its plan read stay materialized; that view is released before the refresh computes, so the inference never carries the rows its plan verified; a refresh whose custody waited and planned again still builds the disk, and the graph equals the independent Python model of the workspace. */
import {expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,realpathSync,rmSync} from "node:fs";
import {writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {GENERATION_POLICY_V1,canonicalJsonV1,createGenerationStoreV1,createNodeFilePortV1,type GenerationComputeSpecV1} from "../../🟨️.mjs";
import {createRowLogV1} from "../../🗃️row-log/🟨️.mjs";
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

for(const mode of ["publish","cancel","cancel during disposal","fail"] as const)test("a cold build releases its row log exactly once on "+mode,async()=>{
 const artifact=process.env.SEMIO_TEST_ARTIFACT_DIR??realpathSync(tmpdir());
 mkdirSync(artifact,{recursive:true});
 const dir=mkdtempSync(join(artifact,"g-")),logPath=join(dir,"r.rows"),log=createRowLogV1({path:logPath});
 try{
  await seed(dir,ringFiles(mode==="publish"?GENERATION_POLICY_V1.ownerWave+1:8));
  const controller=new AbortController(),error=Error(mode),nodeFiles=createNodeFilePortV1(),inner=createCompute(dir,[]);
  let disposed=0,rowReads=0,sealing=false;
  const expectReleased=()=>{
   expect(disposed).toBe(1);
   expect(existsSync(logPath)).toBe(false);
   expect(rowReads).toBeGreaterThan(0);
  };
  const fs={...nodeFiles,async write(...args:Parameters<typeof nodeFiles.write>){
   if(args[0].endsWith(".facts.br")){
    expectReleased();
    expect(mode.startsWith("cancel")).toBe(false);
    if(mode==="fail")throw error;
   }
   return nodeFiles.write(...args);
  }};
  const store=createGenerationStoreV1({root:join(dir,".gen"),fs,holds:createHolds(dir),resources:{facts:"repository-graph-facts:row-release"},say(key){
   if(key==="sealing"){sealing=true;expectReleased();}
  },stepped(step){
   if(mode==="cancel"&&step==="storing"){expect(existsSync(logPath)).toBe(true);controller.abort(error);}
  }});
  const compute=async(spec:GenerationComputeSpecV1,signal?:AbortSignal)=>{
   const computed=await inner(spec,signal);
   return{...computed,owners:computed.owners.map(owner=>({...owner,rows:log.append(owner.rows??[])})),dispose:()=>{
    disposed++;rowReads=log.stats().reads;log.close();
    if(mode==="cancel during disposal")controller.abort(error);
   }};
  };
  const build=store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,membership:createMembership(dir),compute,signal:controller.signal});
  if(mode==="publish"){
   const result=await build;
   expect(result.published).toBe(true);
   expect(sealing).toBe(true);
   const fresh=createGenerationStoreV1({root:join(dir,".gen"),holds:createHolds(dir),resources:{facts:"repository-graph-facts:row-release"}});
   const current=(await fresh.read())!;
   const index=await fresh.readIndexView(current.manifest!.factIndex);
   expect(index?.rows.length).toBe(result.index!.rows);
   const {generation:_generation,membership:_membership,...graph}=await fresh.assemble(current,{});
   expect(normalized(graph)).toEqual(pythonGraph(dir));
  }else{
   await expect(build).rejects.toBe(error);
   expect(await store.read()).toBeUndefined();
  }
  expect(disposed).toBe(1);
  expect(existsSync(logPath)).toBe(false);
  if(mode==="cancel")expect(rowReads).toBe(0);
 }finally{log.close();rmSync(dir,{recursive:true,force:true});}
});
