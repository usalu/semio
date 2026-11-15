/** 🌊️ Contract: a build re-infers at most `ownerWave` stale owners before it publishes a generation, so a build that is cancelled (memory pressure, an interrupt) keeps every finished wave and the next build starts with the owners that are still stale instead of from zero; the final generation equals the graph an uninterrupted build produces and the independent Python oracle. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {GENERATION_POLICY_V1,canonicalJsonV1} from "../../🟨️.mjs";
import {loadGenerationGraphV1} from "../../🔌️load/🟨️.mjs";
import {createCompute,createMembership,createMockHost,createWorld,IDENTITY,IMPLEMENTATION,ringFiles,seed} from "../🏃️world/🟦️.ts";
import {createHash} from "node:crypto";

const here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"../🐍️.py"),python=Bun.which("python3")??Bun.which("python");
const COUNT=GENERATION_POLICY_V1.ownerWave*2+7,scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-waves-")),sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
const pythonGraph=(dir:string)=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,"graph",dir]).stdout.toString());
const edit=async(dir:string,count:number)=>{await sleep(20);for(let at=1;at<=count;at++)await writeFile(join(dir,`pkg/p${at}/src/index.ts`),`import "@syn/p${at+1}";\nexport const waved=${at};\n`);};

test("the wave is a finite positive policy",()=>{expect(Number.isSafeInteger(GENERATION_POLICY_V1.ownerWave)).toBe(true);expect(GENERATION_POLICY_V1.ownerWave).toBeGreaterThan(0);});

test("an uninterrupted refresh publishes one generation per wave and equals the oracle",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT+3));
  const world=createWorld(dir);
  await world.refresh();
  const before=(await world.store.read())!.pointer.generation;
  await edit(dir,COUNT);
  world.log.length=0;
  const result=await world.refresh();
  expect(result.published).toBe(true);expect(result.computed).toBe(COUNT);
  expect((await world.store.read())!.pointer.generation-before).toBe(3);
  expect(world.log.map(entry=>entry.full.length)).toEqual([GENERATION_POLICY_V1.ownerWave,GENERATION_POLICY_V1.ownerWave,7]);
  expect(new Set(world.log.flatMap(entry=>entry.full)).size).toBe(COUNT);
  const{generation:_g,membership:_m,...rest}=await world.store.assemble(result.current,{});
  expect(JSON.parse(canonicalJsonV1(rest))).toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("the build tells the host the generation of every wave it published, in order, and a host that throws does not fail the build",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT+3));
  const world=createWorld(dir);
  await world.refresh();
  const before=(await world.store.read())!.pointer.generation,told:number[]=[];
  await edit(dir,COUNT);
  const result=await world.refresh({published:generation=>{told.push(generation);if(told.length===2)throw new Error("host failed");}});
  expect(result.published).toBe(true);
  expect(told).toEqual([before+1,before+2,before+3]);
  expect((await world.store.read())!.pointer.generation).toBe(before+3);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a build cancelled after a wave keeps the finished wave and the next build derives only the rest",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT+3));
  const controller=new AbortController();let calls=0;
  const world=createWorld(dir,{computeWrap:compute=>async(spec,signal)=>{if(++calls===2)controller.abort(new Error("memory pressure"));return compute(spec,signal);}});
  await world.refresh();calls=0;
  const before=(await world.store.read())!.pointer.generation;
  await edit(dir,COUNT);
  world.log.length=0;
  await expect(world.refresh({signal:controller.signal})).rejects.toThrow();
  const kept=await world.store.read();
  expect(kept!.pointer.generation-before).toBe(1);
  expect(kept!.manifest!.built.dirty.length).toBe(GENERATION_POLICY_V1.ownerWave);
  const finished=new Set(kept!.manifest!.built.dirty);
  world.log.length=0;
  const resumed=await world.refresh();
  expect(resumed.published).toBe(true);expect(resumed.computed).toBe(COUNT-GENERATION_POLICY_V1.ownerWave);
  expect(world.log.flatMap(entry=>entry.full).some(name=>finished.has(name))).toBe(false);
  const{generation:_g,membership:_m,...rest}=await world.store.assemble(resumed.current,{});
  expect(JSON.parse(canonicalJsonV1(rest))).toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a closure refresh does not claim the owners of the waves still to come as verified",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT+3));
  const world=createWorld(dir);
  await world.refreshClosure(["p0"]);
  await edit(dir,COUNT);
  const result=await world.refreshClosure(["p0"]);
  expect(result.published).toBe(true);
  const quiet=await world.refreshClosure(["p0"]);
  expect(quiet.built).toBe(false);
  const{generation:_g,membership:_m,...rest}=await world.store.assemble(result.current,{});
  expect(JSON.parse(canonicalJsonV1(rest))).toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a load across waves with a new project reports one monotonic progress and says the build once",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT+3));
  await loadGenerationGraphV1(createMockHost(dir).host as never);
  await edit(dir,COUNT);
  await seed(dir,{"pkg/fresh/project.json":JSON.stringify({name:"fresh",package:"@syn/fresh",uses:[]}),"pkg/fresh/src/index.ts":"export const fresh=1;\n"});
  const mock=createMockHost(dir),loaded=await loadGenerationGraphV1(mock.host as never) as {graph:{projects:Record<string,unknown>}};
  expect(Object.keys(loaded.graph.projects)).toContain("fresh");
  expect(mock.said.filter(row=>row.key==="building").length).toBe(1);
  expect(mock.log.length).toBeGreaterThan(1);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("an implementation that changed while a read was cancelled after a wave resumes with the finished owners reused, and every kind of change of the implementation digest does the same",async()=>{
 const other=(name:string)=>createHash("sha256").update(name).digest("hex");
 for(const implementation of[other("code-body-edit"),other("data-file-row-changed"),IMPLEMENTATION]){
  const dir=scratch();
  try{
   await seed(dir,ringFiles(COUNT+3));
   const controller=new AbortController();let calls=0;
   const world=createWorld(dir,{computeWrap:compute=>async(spec,signal)=>{if(++calls===2)controller.abort(new Error("the code changed under the worker"));return compute(spec,signal);}});
   await world.refresh();calls=0;
   const before=(await world.store.read())!.pointer.generation;
   await edit(dir,COUNT);
   await expect(world.refresh({signal:controller.signal})).rejects.toThrow();
   const kept=await world.store.read();
   expect(kept!.pointer.generation-before).toBe(1);
   const finished=new Set(kept!.manifest!.built.dirty),log:Parameters<typeof createCompute>[1]=[];
   const resumed=await world.store.refresh({identity:IDENTITY,implementation,membership:createMembership(dir),compute:createCompute(dir,log)});
   expect(resumed.published).toBe(true);expect(resumed.computed).toBe(COUNT-GENERATION_POLICY_V1.ownerWave);
   expect(log.flatMap(entry=>entry.full).some(name=>finished.has(name))).toBe(false);
   expect(resumed.current.manifest.implementation).toBe(implementation);
   const{generation:_g,membership:_m,...rest}=await world.store.assemble(resumed.current,{});
   expect(JSON.parse(canonicalJsonV1(rest))).toEqual(pythonGraph(dir));
  }finally{rmSync(dir,{recursive:true,force:true});}
 }
});

test("a graph read served by the graph server takes the asked-for owners in its first wave and tells which of them are still pending after every plan",async()=>{
 const dir=scratch(),steer=scratch(),saved=process.env.SEMIO_GRAPH_STEER_DIRECTORY;
 try{
  const {writePriorityV1,readPendingV1}=await import("../../🧭️steering/🟨️.mjs");
  await seed(dir,ringFiles(COUNT+3));
  const world=createWorld(dir);
  await world.refresh();
  const names=Object.keys((await world.store.read())!.manifest.owners).sort(),asked=names.slice(-3).concat(names.slice(-60,-58));
  await edit(dir,COUNT);
  process.env.SEMIO_GRAPH_STEER_DIRECTORY=steer;writePriorityV1(steer,asked,7);
  world.log.length=0;
  const seen:string[][]=[];
  const result=await world.refresh({published:()=>{seen.push([...readPendingV1(steer)!.projects]);}});
  expect(result.published).toBe(true);
  const first=world.log[0]!.full;
  for(const name of asked)expect(first,name).toContain(name);
  expect(seen[0]!.every(name=>!first.includes(name))).toBe(true);expect(seen.at(-1)).toEqual([]);
  expect(readPendingV1(steer)).toMatchObject({priorityAtMs:7});
  expect(new Set(world.log.flatMap(entry=>entry.full)).size).toBe(COUNT);
 }finally{
  if(saved===undefined)delete process.env.SEMIO_GRAPH_STEER_DIRECTORY;else process.env.SEMIO_GRAPH_STEER_DIRECTORY=saved;
  rmSync(dir,{recursive:true,force:true});rmSync(steer,{recursive:true,force:true});
 }
});
