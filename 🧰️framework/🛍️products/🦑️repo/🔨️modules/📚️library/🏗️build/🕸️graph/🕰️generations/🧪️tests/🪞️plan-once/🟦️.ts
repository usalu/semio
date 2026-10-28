/** 🪞️ Contract: a builder that took over a delta refresh at once plans it once; a builder that had to wait for its custody longer than the declared bound plans again, so an edit made during the wait reaches the generation. The plan (membership frontier walk and the verification of every fact in scope) that decided the refresh is the plan the build executes, also when the refresh is scoped to a closure or to the owners a coverage stamp left open; only a generation that moved since forces a new plan. The graph after the refresh equals the independent Python model of the workspace. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {GENERATION_POLICY_V1,canonicalJsonV1,createGenerationStoreV1} from "../../🟨️.mjs";
import {IDENTITY,IMPLEMENTATION,createCompute,createHolds,createMembership,ringFiles,seed} from "../🏃️world/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"../🐍️.py"),python=Bun.which("python3")??Bun.which("python");
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-plan-once-")),sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
const pythonGraph=(dir:string)=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,"graph",dir]).stdout.toString());
const normalized=(value:unknown)=>JSON.parse(canonicalJsonV1(value));

function setup(dir:string,now?:()=>number){
 const counters={walks:0,rows:0,passes:0},holds=createHolds(dir),membership=createMembership(dir,counters),compute=createCompute(dir,[]);
 const counted=async(rows:readonly {kind:string;path:string;fact:unknown}[],signal?:AbortSignal)=>{counters.rows+=rows.length;return holds(rows as never,signal);};
 const store=createGenerationStoreV1({root:join(dir,".gen"),holds:counted,resources:{facts:"repository-graph-facts:plan-once",builder:"repository-graph-build:plan-once"},...now?{now}:{}});
 const progress=(event:{phase?:string;done:number})=>{if(event.phase==='facts'&&event.done===0)counters.passes++;};
 return{counters,store,base:{identity:IDENTITY,implementation:IMPLEMENTATION,membership,compute,progress}};
}

for(const scope of["closure","whole-workspace"] as const)test("a "+scope+" refresh with a coverage stamp walks the membership once and verifies the facts of its scope in one pass",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(8));
  const{counters,store,base}=setup(dir);
  const cold=await store.refreshClosure({...base,roots:["p0"],requestedAt:Date.now()});
  expect(cold.published).toBe(true);expect(cold.current.pointer.verified).toBeDefined();
  const quiet=await store.refreshClosure({...base,roots:["p0"],requestedAt:Date.now()+1});
  expect(quiet.built).toBe(false);
  await sleep(20);
  await writeFile(join(dir,"pkg/p3/src/index.ts"),"import \"@syn/p4\";\nexport const edited=1;\n");
  counters.walks=0;counters.passes=0;
  const refreshed=scope==="closure"?await store.refreshClosure({...base,roots:["p0"],requestedAt:Date.now()}):await store.refresh({...base,requestedAt:Date.now()});
  expect(refreshed.built).toBe(true);expect(refreshed.published).toBe(true);expect(refreshed.computed).toBe(1);
  expect(counters.walks).toBe(1);
  expect(counters.passes).toBe(1);
  const graph=await store.assemble(refreshed.current,{});const{generation:_g,membership:_m,...rest}=graph;
  expect(normalized(rest)).toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a generation that moved after the plan is planned again before it is built on",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(8));
  const{counters,store,base}=setup(dir),peer=setup(dir);
  await store.refreshClosure({...base,roots:["p0"],requestedAt:Date.now()});
  await sleep(20);
  await writeFile(join(dir,"pkg/p3/src/index.ts"),"import \"@syn/p4\";\nexport const edited=1;\n");
  counters.walks=0;
  let moved=false;
  const builder=async(body:()=>Promise<unknown>)=>{
   if(!moved){moved=true;await writeFile(join(dir,"pkg/p5/src/index.ts"),"import \"@syn/p6\";\nexport const edited=2;\n");await peer.store.refresh({...peer.base,policy:"fresh"});}
   return body();
  };
  const result=await store.refreshClosure({...base,roots:["p0"],requestedAt:Date.now(),builder} as never);
  expect(result.current.manifest.generation).toBe(1);
  expect(counters.walks).toBe(2);
  const graph=await store.assemble(result.current,{});const{generation:_g,membership:_m,...rest}=graph;
  expect(normalized(rest)).toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
});

for(const scope of["closure","whole-workspace"] as const)test("[adversarial] a file edited while the "+scope+" refresh waits for the builder custody is built into the generation",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(8));
  const clock={at:Date.now()},{counters,store,base}=setup(dir,()=>clock.at);
  const refresh=(extra:object={})=>scope==="closure"?store.refreshClosure({...base,roots:["p0"],requestedAt:clock.at,...extra} as never):store.refresh({...base,requestedAt:clock.at,...extra} as never);
  await refresh();clock.at+=10;
  await sleep(20);
  await writeFile(join(dir,"pkg/p3/src/index.ts"),"import \"@syn/p4\";\nexport const edited=1;\n");
  counters.walks=0;
  const builder=async(body:()=>Promise<unknown>)=>{
   clock.at+=GENERATION_POLICY_V1.hintMaximumAgeMilliseconds+1;
   await sleep(20);await writeFile(join(dir,"pkg/p5/src/index.ts"),"import \"@syn/p6\";\nexport const edited=2;\n");
   return body();
  };
  const result=await refresh({builder}) as any;
  expect(result.built).toBe(true);expect(result.computed).toBe(2);expect(counters.walks).toBe(2);
  const graph=await store.assemble(result.current,{});const{generation:_g,membership:_m,...rest}=graph;
  expect(normalized(rest),"the generation equals the disk after the wait").toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a custody taken within the declared bound reuses the plan, one beyond it does not",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(8));
  const clock={at:Date.now()},{counters,store,base}=setup(dir,()=>clock.at);
  await store.refreshClosure({...base,roots:["p0"],requestedAt:clock.at});
  for(const[late,walks]of[[GENERATION_POLICY_V1.hintMaximumAgeMilliseconds,1],[GENERATION_POLICY_V1.hintMaximumAgeMilliseconds+1,2]] as const){
   clock.at+=10;await sleep(20);
   await writeFile(join(dir,"pkg/p3/src/index.ts"),"import \"@syn/p4\";\nexport const edited="+late+";\n");
   counters.walks=0;
   const result=await store.refreshClosure({...base,roots:["p0"],requestedAt:clock.at,builder:async(body:()=>Promise<unknown>)=>{clock.at+=late;return body();}} as never);
   expect(result.built).toBe(true);expect(counters.walks).toBe(walks);
  }
 }finally{rmSync(dir,{recursive:true,force:true});}
});
