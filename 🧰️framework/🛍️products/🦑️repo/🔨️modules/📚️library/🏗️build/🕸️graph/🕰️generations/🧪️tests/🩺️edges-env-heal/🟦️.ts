/** 🩺️ Contract: an owner whose edges were made under another environment than the published one is derived again, never published stale and never a late internal error: a read whose compute leaves such an owner behind gives it its edges again in a bounded extra pass without inference (an owner is inferred again only when its own facts are stale) and says so in both languages; a moved environment gives every kept owner its edges again without inference, and the published result equals the independent oracle. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {createHash} from "node:crypto";
import {canonicalJsonV1} from "../../🟨️.mjs";
import {GENERATION_MESSAGES_V1} from "../../🔌️load/🟨️.mjs";
import {createFactIndexV1,createFrontierV1,createGenerationStoreV1,type GenerationRowV1} from "../../🟨️.mjs";
import {createGenerationPlanV1} from "../../🧭️plan/🟨️.mjs";
import {IDENTITY,IMPLEMENTATION,createMemoryFilePort,createWorld,ringFiles,seed} from "../🏃️world/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"../🐍️.py"),python=Bun.which("python3")??Bun.which("python");
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-edges-env-heal-"));
const pythonGraph=(dir:string)=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,"graph",dir]).stdout.toString());
const forgetful=(victim:string)=>(compute:any)=>async(spec:any,signal?:AbortSignal)=>{const result=await compute(spec,signal);if(spec.edgeStale?.length)return result;return{...result,owners:result.owners.filter((owner:any)=>!(owner.partial==="edges"&&owner.name===victim)),carried:(result.carried??[]).filter((name:string)=>name!==victim)};};
const grow=(dir:string)=>seed(dir,{"pkg/p5/project.json":JSON.stringify({name:"p5",package:"@syn/p5",uses:[]}),"pkg/p5/src/index.ts":"import \"@syn/p0\";\nexport const v5=5;\n"});

test("a stored owner under environment A and a present environment B that the compute did not refresh gets its edges again without inference and the generation equals the oracle",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(5));
  const world=createWorld(dir,{computeWrap:forgetful("p2")});
  await world.refresh();
  const before=(await world.store.read())!.pointer.generation;
  await grow(dir);
  world.log.length=0;
  const result=await world.refresh();
  expect(result.published).toBe(true);
  expect((await world.store.read())!.pointer.generation).toBe(before+1);
  expect(world.log.some(entry=>entry.full.includes("p2"))).toBe(false);
  const healed=world.log[world.log.length-1]!;
  expect(healed.full).toEqual([]);
  expect(healed.edges).toEqual(["p2"]);
  const{generation:_g,membership:_m,...rest}=await world.store.assemble(result.current,{});
  expect(JSON.parse(canonicalJsonV1(rest))).toEqual(pythonGraph(dir));
  expect(world.log.length).toBeGreaterThan(1);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("the healing line exists in English and German with the count",()=>{
 expect(GENERATION_MESSAGES_V1.en.healing).toContain("{count}");
 expect(GENERATION_MESSAGES_V1.de.healing).toContain("{count}");
 expect(GENERATION_MESSAGES_V1.de.healing).not.toBe(GENERATION_MESSAGES_V1.en.healing);
});

test("an environment that moves between two waves of one read still publishes and equals the oracle",async()=>{
 const dir=scratch();
 try{
  const{GENERATION_POLICY_V1}=await import("../../🟨️.mjs"),count=GENERATION_POLICY_V1.ownerWave*2+7,{writeFile}=await import("node:fs/promises");
  await seed(dir,ringFiles(count+3));
  let calls=0;
  const world=createWorld(dir,{computeWrap:compute=>async(spec:any,signal?:AbortSignal)=>{if(++calls===2)await grow(dir);return compute(spec,signal);}});
  await world.refresh();calls=0;
  await new Promise(resolve=>setTimeout(resolve,20));
  for(let at=1;at<=count;at++)await writeFile(join(dir,`pkg/p${at}/src/index.ts`),`import "@syn/p${at+1}";\nexport const waved=${at};\n`);
  const result=await world.refresh();
  expect(result.published).toBe(true);
  const{generation:_g,membership:_m,...rest}=await world.store.assemble(result.current,{});
  expect(JSON.parse(canonicalJsonV1(rest))).toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a closure repair under a changed environment publishes with the edges of every owner at the current environment",async()=>{
 const sha=(text:string)=>createHash("sha256").update(text).digest("hex"),memory=createMemoryFilePort(),moved=sha("environment-b");
 const store=createGenerationStoreV1({root:"/memory/edges-env-repair",fs:memory.port,resources:{facts:"f"},holds:async()=>true});
 const names=Array.from({length:12},(_,at)=>"p"+at),forced=["p1","p4"],membership=async()=>({digest:sha("membership"),frontier:createFrontierV1({directories:[],members:[],capturedAt:1}),changes:[]});
 let environment=sha("environment-a");const calls:string[][]=[];
 const edgeCalls:string[][]=[];
 const compute=async(spec:{cold:boolean;owners:string[];edgeStale?:readonly string[]})=>{const wanted=spec.cold?names:spec.owners;calls.push(wanted);edgeCalls.push([...spec.edgeStale??[]]);return{owners:[...wanted.map(name=>({name,root:name,kind:"project" as const,node:{name},edges:[],targets:[],rows:[{kind:"file",path:name+"/source",fact:{size:0}} as GenerationRowV1]})),...(spec.edgeStale??[]).map(name=>({name,partial:"edges" as const,edges:[]}))],env:{digest:environment,rows:[]}};};
 const request={identity:IDENTITY,implementation:IMPLEMENTATION,membership,compute};
 const before=await store.refresh(request),parent=before.current.pointer.manifest!;
 await store.invalidate({reason:"closure-missing",expected:before.current.pointer});
 const tombstone=(await store.read())!.pointer,repair={generation:tombstone.generation,parent,owners:forced,receipt:sha("repair")};
 environment=moved;calls.length=0;edgeCalls.length=0;
 const result=await store.refresh({...request,repair,qualify:async()=>{}});
 expect(result.published).toBe(true);
 const manifest=result.current.manifest!;
 expect(manifest.env).toBe(moved);
 for(const name of names)expect(manifest.owners[name]!.edgesEnv).toBe(moved);
 expect(calls.flat().every(name=>forced.includes(name))).toBe(true);
 expect(edgeCalls.flat().sort()).toEqual(names.filter(name=>!forced.includes(name)).sort());
});

test("the plan names untagged owners with stored edges of another environment as edge-stale, not dirty; tagged ones are dirty",async()=>{
 const row=(name:string,edgesEnv:string,extra:Record<string,unknown>={})=>({root:name,kind:"project",fragment:"f-"+name,bytes:1,edgesEnv,targets:[],...extra});
 const manifest={identity:"i",global:"g",factIndex:"x",env:"E",owners:{a:row("a","E"),b:row("b","old"),c:row("c","older"),t:row("t","old",{tag:"tests-v2"})}};
 const plan=createGenerationPlanV1({readIndexView:async()=>createFactIndexV1(new Map() as never) as never,fragmentNames:async()=>["g.json.br",...Object.keys(manifest.owners).map(name=>"f-"+name+".json.br")],now:()=>1,settler:{settle:async()=>({changed:new Map(),held:new Map(),kept:new Set(),failed:0,reused:0}),release:async()=>undefined} as never,derive:(()=>undefined) as never,hold:(async()=>true) as never,workspaceRoot:undefined,say:()=>undefined});
 const stale=await plan({current:{manifest} as never,identity:"i",changes:[],verifyAll:true} as never);
 expect(stale.kind).toBe("delta");
 expect([...stale.dirty]).toEqual(["t"]);
 expect(stale.edgeStale).toEqual(["b","c"]);
 const scoped=await plan({current:{manifest} as never,identity:"i",changes:[],only:new Set(["b"]),verifyAll:true} as never);
 expect(scoped.edgeStale).toEqual(["b"]);
 const fresh=await plan({current:{manifest:{...manifest,owners:{a:row("a","E")}}} as never,identity:"i",changes:[],verifyAll:true} as never);
 expect(fresh.kind).toBe("clean");
 expect(fresh.edgeStale).toEqual([]);
});

const addOwner=(dir:string)=>seed(dir,{"pkg/extra/project.json":JSON.stringify({name:"extra",package:"@syn/extra",uses:[]}),"pkg/extra/src/index.ts":"import \"@syn/p0\";\nexport const extra=1;\n"});

test("a moved environment on a large store gives every kept owner its edges again without any inference and the generation equals the oracle",async()=>{
 const dir=scratch(),count=300;
 try{
  await seed(dir,ringFiles(count));
  const world=createWorld(dir,{computeWrap:compute=>async(spec:any,signal?:AbortSignal)=>compute({...spec,previousParts:undefined},signal)});
  await world.refresh();
  await addOwner(dir);
  world.log.length=0;
  const result=await world.refresh();
  expect(result.published).toBe(true);
  expect(world.log.length).toBe(1);
  expect(world.log[0]!.full).toEqual(["extra"]);
  expect(world.log[0]!.edges.length).toBe(count);
  expect(new Set(world.log[0]!.edges).size).toBe(count);
  const manifest=result.current.manifest!;
  for(const row of Object.values(manifest.owners))expect(row.edgesEnv).toBe(manifest.env);
  const{generation:_g,membership:_m,...rest}=await world.store.assemble(result.current,{});
  expect(JSON.parse(canonicalJsonV1(rest))).toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
},120_000);

test("with the recorded lookups of the previous environment a change elsewhere moves no edges and the generation still equals the oracle",async()=>{
 const dir=scratch(),count=300;
 try{
  await seed(dir,ringFiles(count));
  const world=createWorld(dir);
  await world.refresh();
  await addOwner(dir);
  world.log.length=0;
  const result=await world.refresh();
  expect(result.published).toBe(true);
  expect(world.log[0]!.full).toEqual(["extra"]);
  expect(world.log[0]!.edges).toEqual([]);
  const manifest=result.current.manifest!;
  for(const row of Object.values(manifest.owners))expect(row.edgesEnv).toBe(manifest.env);
  const{generation:_g,membership:_m,...rest}=await world.store.assemble(result.current,{});
  expect(JSON.parse(canonicalJsonV1(rest))).toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
},120_000);
