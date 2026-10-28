/** 🪑️ Contract: a command whose builder is idle builds at once, without the prefetch that would compute every stale owner for nobody; a command whose builder is busy prepares the stale owners first and carries the shared command closures through that preparation, and a host that cannot tell keeps the prefetch. The graph after each route equals the independent Python model of the workspace. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {canonicalJsonV1} from "../../../🟨️.mjs";
import {loadGenerationGraphV1} from "../../🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"../../../🧪️tests/🐍️.py"),python=Bun.which("python3")??Bun.which("python");
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-idle-builder-")),clean=(dir:string)=>rmSync(dir,{recursive:true,force:true});
const normalized=(value:unknown)=>JSON.parse(canonicalJsonV1(value));
const shape=(graph:any)=>normalized({projects:graph.projects,dependencies:graph.dependencies,externalNodes:graph.externalNodes,unavailable:graph.unavailable??[]});
const pythonGraph=(dir:string)=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,"graph",dir]).stdout.toString());
type Request={generation:number;dirty:readonly string[];roots:Readonly<Record<string,string>>;implementation:string};

async function staleWorkspace(dir:string){
 await seed(dir,ringFiles(6));
 const requests:Request[]=[],asked:Request[]=[],mock=createMockHost(dir,{prefetch:async request=>{requests.push(request);}}),host=mock.host as Record<string,unknown>;
 await loadGenerationGraphV1(mock.host);
 await applyOps(dir,[{op:"write",path:"pkg/p2/src/index.ts",content:'import "@syn/p4";\nexport const v=2;\n'}]);
 mock.events.length=0;requests.length=0;
 host.carryClosures=(on:boolean)=>{mock.events.push("carry:"+on);};
 return{mock,host,requests,asked};
}

test("an idle builder builds at once: no prefetch is run, the plan is asked about, and the graph equals the Python model",async()=>{
 const dir=scratch();
 try{
  const{mock,host,requests,asked}=await staleWorkspace(dir);
  host.builderIdle=async(request:Request)=>{asked.push(request);return true;};
  const graph=await loadGenerationGraphV1(mock.host);
  expect(mock.events).toEqual(["shared","builder:wait","builder:held","builder:released"]);
  expect(requests).toEqual([]);
  expect(asked).toHaveLength(1);expect(asked[0]!.dirty).toEqual(["p2"]);expect(asked[0]!.roots.p2).toBe("pkg/p2");expect(asked[0]!.generation).toBe(0);
  expect(shape(graph)).toEqual(pythonGraph(dir));
 }finally{clean(dir);}
},60000);

test("a busy builder prepares the stale owners first and carries the closures exactly through that preparation",async()=>{
 const dir=scratch();
 try{
  const{mock,host,requests,asked}=await staleWorkspace(dir);
  host.builderIdle=async(request:Request)=>{asked.push(request);return false;};
  const graph=await loadGenerationGraphV1(mock.host);
  expect(mock.events).toEqual(["shared","carry:true","prefetch","carry:false","builder:wait","builder:held","builder:released"]);
  expect(requests).toHaveLength(1);expect(requests[0]).toEqual(asked[0]!);
  expect(shape(graph)).toEqual(pythonGraph(dir));
 }finally{clean(dir);}
},60000);

test("a host that cannot tell keeps the prefetch, and a preparation that fails still ends the carrying",async()=>{
 const dir=scratch();
 try{
  const{mock,requests}=await staleWorkspace(dir);
  await loadGenerationGraphV1(mock.host);
  expect(mock.events).toEqual(["shared","carry:true","prefetch","carry:false","builder:wait","builder:held","builder:released"]);expect(requests).toHaveLength(1);
  await applyOps(dir,[{op:"write",path:"pkg/p3/src/index.ts",content:'import "@syn/p5";\nexport const v=3;\n'}]);
  const failing=createMockHost(dir,{prefetch:async()=>{throw new Error("cancelled");}});(failing.host as Record<string,unknown>).carryClosures=(on:boolean)=>{failing.events.push("carry:"+on);};
  await expect(loadGenerationGraphV1(failing.host)).rejects.toThrow("cancelled");
  expect(failing.events).toEqual(["shared","carry:true","prefetch","carry:false"]);
 }finally{clean(dir);}
},60000);

test("nothing is asked and nothing is prepared when the plan names no stale owner",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));
  const requests:Request[]=[],asked:Request[]=[],mock=createMockHost(dir,{prefetch:async request=>{requests.push(request);}});
  (mock.host as Record<string,unknown>).builderIdle=async(request:Request)=>{asked.push(request);return true;};
  await loadGenerationGraphV1(mock.host);
  expect(mock.events).toEqual(["shared","builder:wait","builder:held","builder:released"]);expect(asked).toEqual([]);expect(requests).toEqual([]);
 }finally{clean(dir);}
},60000);
