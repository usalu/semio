/** 🪑️ Contract: a command whose builder is idle builds at once, without the prefetch that would compute every stale owner for nobody; a command whose builder is busy and that cannot attach prepares the stale owners first and carries the shared command closures through that preparation, a host that cannot tell keeps the prefetch, and a command that can attach (a live builder of the same machine holds the custody against it) does not prepare anything beside that builder: it tells the waves, takes the records as they publish and builds only to find the work done. The graph after each route equals the independent Python model of the workspace. */
import {expect,test} from "bun:test";
import {mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {canonicalJsonV1} from "../../../🟨️.mjs";
import {loadGenerationGraphV1} from "../../🟨️.mjs";
import {readGenerationPointerV1} from "../../../🟨️.mjs";
import {CUSTODY_POLICY_V1,attachToBuilderV1,renderCustodyMessageV1} from "../../../🔐️custody/🟨️.mjs";
const graphOfV1=async(...input:Parameters<typeof loadGenerationGraphV1>)=>(await loadGenerationGraphV1(...input)).graph;
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
 (await loadGenerationGraphV1(mock.host)).graph;
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
  const graph=(await loadGenerationGraphV1(mock.host)).graph;
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
  const graph=(await loadGenerationGraphV1(mock.host)).graph;
  expect(mock.events).toEqual(["shared","carry:true","prefetch","carry:false","builder:wait","builder:held","builder:released"]);
  expect(requests).toHaveLength(1);expect(requests[0]).toEqual(asked[0]!);
  expect(shape(graph)).toEqual(pythonGraph(dir));
 }finally{clean(dir);}
},60000);

test("a host that cannot tell keeps the prefetch, and a preparation that fails still ends the carrying",async()=>{
 const dir=scratch();
 try{
  const{mock,requests}=await staleWorkspace(dir);
  (await loadGenerationGraphV1(mock.host)).graph;
  expect(mock.events).toEqual(["shared","carry:true","prefetch","carry:false","builder:wait","builder:held","builder:released"]);expect(requests).toHaveLength(1);
  await applyOps(dir,[{op:"write",path:"pkg/p3/src/index.ts",content:'import "@syn/p5";\nexport const v=3;\n'}]);
  const failing=createMockHost(dir,{prefetch:async()=>{throw new Error("cancelled");}});(failing.host as Record<string,unknown>).carryClosures=(on:boolean)=>{failing.events.push("carry:"+on);};
  await expect(graphOfV1(failing.host)).rejects.toThrow("cancelled");
  expect(failing.events).toEqual(["shared","carry:true","prefetch","carry:false"]);
 }finally{clean(dir);}
},60000);

test("nothing is asked and nothing is prepared when the plan names no stale owner",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));
  const requests:Request[]=[],asked:Request[]=[],mock=createMockHost(dir,{prefetch:async request=>{requests.push(request);}});
  (mock.host as Record<string,unknown>).builderIdle=async(request:Request)=>{asked.push(request);return true;};
  (await loadGenerationGraphV1(mock.host)).graph;
  expect(mock.events).toEqual(["shared","builder:wait","builder:held","builder:released"]);expect(asked).toEqual([]);expect(requests).toEqual([]);
 }finally{clean(dir);}
},60000);

async function twoReaders(dir:string,reader:{recheckMilliseconds:number;expandDelayMs:number}){
 await seed(dir,ringFiles(6));
 (await loadGenerationGraphV1(createMockHost(dir).host)).graph;
 const files=ringFiles(6);
 await applyOps(dir,[1,2,3,4].map(index=>({op:"write" as const,path:`pkg/p${index}/src/index.ts`,content:files[`pkg/p${index}/src/index.ts`]!.replace(/=\d+;\n$/u,"=77;\n")})));
 const custody={tail:Promise.resolve()},order:string[]=[],lines:string[]=[];let holding=0,peak=0;
 const a=createMockHost(dir,{custody,ownerDelayMs:100}),b=createMockHost(dir,{custody});
 const track=(name:string,mock:ReturnType<typeof createMockHost>)=>{
  const host=mock.host as Record<string,any>,withBuilder=host.withBuilder.bind(host),expand=host.expand.bind(host);
  host.withBuilder=(body:(...input:unknown[])=>Promise<unknown>,request:unknown)=>withBuilder(async(...input:unknown[])=>{holding++;peak=Math.max(peak,holding);order.push(name+":held");try{return await body(...input);}finally{holding--;order.push(name+":done");}},request);
  if(name==="A")host.expand=async(projects:Record<string,unknown>)=>{await new Promise(resolve=>setTimeout(resolve,reader.expandDelayMs));return expand(projects);};
 };
 track("A",a);track("B",b);
 const pointer=()=>readGenerationPointerV1(readFileSync(join(dir,".gen","current.json"))).generation;
 const bHost=b.host as Record<string,any>;
 bHost.attach=(request:Request,recheck:()=>Promise<unknown>)=>attachToBuilderV1({look:async()=>({held:holding>0,generation:pointer()}),recheck,say:(key,fields)=>lines.push(key+":"+renderCustodyMessageV1(key,fields,"en")),policy:{...CUSTODY_POLICY_V1,attachPollMilliseconds:15,attachRecheckMilliseconds:reader.recheckMilliseconds,noticeMilliseconds:50}});
 const building=loadGenerationGraphV1(a.host);
 for(let waited=0;holding===0&&waited<400;waited++)await new Promise(resolve=>setTimeout(resolve,10));
 expect(holding,"the first reader holds the custody").toBe(1);
 const serving=await loadGenerationGraphV1(b.host);
 return{a,b,order,lines,peak:()=>peak,first:(await building).graph,second:serving.graph};
}

test("two readers on one state root: the second attaches to the builder, infers nothing itself, builds after it only to find the work done, and both graphs equal the Python model",async()=>{
 const dir=scratch(),previous=process.env.SEMIO_SLICE_WAVE_BASIS_POINTS;process.env.SEMIO_SLICE_WAVE_BASIS_POINTS="200";
 try{
  const world=await twoReaders(dir,{recheckMilliseconds:3_600_000,expandDelayMs:0});
  expect(world.peak(),"never two builders at once").toBe(1);
  expect(world.order).toEqual(["A:held","A:done","B:held","B:done"]);
  expect(world.a.log.flatMap(row=>row.full).sort()).toEqual(["p1","p2","p3","p4"]);
  expect(world.b.log.flatMap(row=>row.full),"the attached reader infers nothing").toEqual([]);
  expect(world.b.events).not.toContain("prefetch");
  const keys=world.lines.map(line=>line.split(":")[0]);expect(keys[0]).toBe("attached");expect(keys).toContain("attachedWave");expect(keys).not.toContain("attachedServed");
  expect(world.lines.filter(line=>line.startsWith("attachedWave")).length).toBeGreaterThanOrEqual(2);
  expect(shape(world.first)).toEqual(pythonGraph(dir));expect(shape(world.second)).toEqual(pythonGraph(dir));
 }finally{if(previous===undefined)delete process.env.SEMIO_SLICE_WAVE_BASIS_POINTS;else process.env.SEMIO_SLICE_WAVE_BASIS_POINTS=previous;clean(dir);}
},120000);

test("an attached reader takes the records as the waves publish: once the wave that covers it is published it is served from it before the builder has let go, and never builds",async()=>{
 const dir=scratch(),previous=process.env.SEMIO_SLICE_WAVE_BASIS_POINTS;process.env.SEMIO_SLICE_WAVE_BASIS_POINTS="200";
 try{
  const world=await twoReaders(dir,{recheckMilliseconds:40,expandDelayMs:1500});
  expect(world.lines.map(line=>line.split(":")[0])).toContain("attachedServed");
  expect(world.order,"the second reader was served while the first still held the custody").toEqual(["A:held","A:done"]);
  expect(world.b.log.flatMap(row=>row.full)).toEqual([]);
  expect(shape(world.second)).toEqual(pythonGraph(dir));
 }finally{if(previous===undefined)delete process.env.SEMIO_SLICE_WAVE_BASIS_POINTS;else process.env.SEMIO_SLICE_WAVE_BASIS_POINTS=previous;clean(dir);}
},120000);

test("a refresh that is cut off keeps every published wave: the next read starts from the newest generation and infers only what no published wave holds",async()=>{
 const dir=scratch(),previous=process.env.SEMIO_SLICE_WAVE_BASIS_POINTS;process.env.SEMIO_SLICE_WAVE_BASIS_POINTS="200";
 try{
  await seed(dir,ringFiles(6));
  (await loadGenerationGraphV1(createMockHost(dir).host)).graph;
  const files=ringFiles(6);
  await applyOps(dir,[1,2,3,4].map(index=>({op:"write" as const,path:`pkg/p${index}/src/index.ts`,content:files[`pkg/p${index}/src/index.ts`]!.replace(/=\d+;\n$/u,"=78;\n")})));
  const controller=new AbortController(),cut=createMockHost(dir,{ownerDelayMs:120,signal:controller.signal}),pointer=()=>readGenerationPointerV1(readFileSync(join(dir,".gen","current.json"))).generation;
  const base=pointer(),running=loadGenerationGraphV1(cut.host).catch(error=>error);
  for(let waited=0;pointer()<base+2&&waited<600;waited++)await new Promise(resolve=>setTimeout(resolve,10));
  controller.abort(new Error("killed by the server"));
  expect(String(await running)).toContain("killed by the server");
  const kept=pointer();expect(kept,"two waves were published before the kill").toBeGreaterThanOrEqual(base+2);
  const again=createMockHost(dir),graph=(await loadGenerationGraphV1(again.host)).graph;
  const published=cut.log.flatMap(row=>row.full).slice(0,kept-base);
  expect(again.log.flatMap(row=>row.full).sort(),"only the owners no published wave holds are inferred again").toEqual(["p1","p2","p3","p4"].filter(name=>!published.includes(name)));
  expect(again.log.flatMap(row=>row.full).length).toBeLessThanOrEqual(4-(kept-base));
  expect(shape(graph)).toEqual(pythonGraph(dir));
 }finally{if(previous===undefined)delete process.env.SEMIO_SLICE_WAVE_BASIS_POINTS;else process.env.SEMIO_SLICE_WAVE_BASIS_POINTS=previous;clean(dir);}
},120000);
