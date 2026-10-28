/** 👯️ Contract: two agents whose slices are disjoint refresh the graph at the same time (their computations overlap in time) and both edits reach one generation, the later publisher merging onto the earlier one's generation instead of computing again; overlapping slices and structural changes still build one after the other; a lost swap never loses an owner update, a refused merge computes again from the newest generation, no published generation is ever torn, readers stay pinned to whole generations, and only the last holder retires old documents. Every final graph equals the independent Python model in `../🐍️.py`. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdtempSync,readdirSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {canonicalJsonV1,createNodeFilePortV1,type GenerationStoreV1} from "../../🟨️.mjs";
import {loadGenerationGraphV1} from "../../🔌️load/🟨️.mjs";
import {createFileClaimLeasePortV1} from "../../🧩️claims/🟨️.mjs";
import {createSliceCustodyV1,renderCustodyMessageV1,type SliceClaimV1} from "../../🔐️custody/🟨️.mjs";
import {isPseudoOwnerV1} from "../../🔀️merge/🟨️.mjs";
import {applyOps,createMockHost,createWorld,ringFiles,seed} from "../🏃️world/🟦️.ts";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"../🐍️.py");
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-multiwriter-")),clean=(dir:string)=>rmSync(dir,{recursive:true,force:true});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex"),sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
const normalized=(value:unknown)=>JSON.parse(canonicalJsonV1(value));
const shape=(graph:any)=>normalized({projects:graph.projects,dependencies:graph.dependencies,externalNodes:graph.externalNodes,unavailable:graph.unavailable??[]});
const pythonGraph=(dir:string)=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,"graph",dir]).stdout.toString());
const gated=(count:number)=>{let open!:()=>void,arrived=0;const promise=new Promise<void>(resolve=>{open=resolve;});return{arrive:async()=>{if(++arrived===count)open();await promise;}};};
const twoRings=()=>({...ringFiles(4,"a"),...ringFiles(4,"b")});
const edit=(name:string,tag:string)=>({op:"write" as const,path:`pkg/${name}/src/index.ts`,content:`export const ${name}${tag}=1;\n`});

type Span={owners:string[];start:number;end:number};
/** 🧑‍💻️ One agent: a mock repository host whose builder custody is the slice custody of the shared store, with the time span of every computation, the custody notices and the answers of `alone`. */
function agent(dir:string,shared:{spans:Span[];alone:boolean[]},ownerDelayMs=250){
 const mock=createMockHost(dir,{ownerDelayMs}),host=mock.host as any,discover=host.discoverPart;
 const custody=createSliceCustodyV1({root:join(dir,".gen"),fs:createNodeFilePortV1(),lease:createFileClaimLeasePortV1(join(dir,".custody"))});
 host.discoverPart=async(spec:{owners:string[]},signal:AbortSignal,control:unknown)=>{const start=Date.now();try{return await discover(spec,signal,control);}finally{shared.spans.push({owners:[...spec.owners].sort(),start,end:Date.now()});}};
 host.withBuilder=async(body:(store:unknown,control:unknown,claim:SliceClaimV1)=>Promise<unknown>,request?:{dirty:readonly string[];structural:boolean})=>{
  mock.events.push("builder:wait");
  const claim=await custody.hold({owners:request?.dirty??[],structural:request===undefined||request.structural,say:(key,fields)=>mock.said.push({key,text:renderCustodyMessageV1(key,fields,"en")})});
  mock.events.push("builder:held");
  const watched:SliceClaimV1={get token(){return claim.token;},get owners(){return claim.owners;},get structural(){return claim.structural;},extend:options=>claim.extend(options),alone:async options=>{const answer=await claim.alone(options);shared.alone.push(answer);return answer;},release:()=>claim.release()};
  try{return await body({root:join(dir,".gen")},{published:mock.published},watched);}finally{mock.events.push("builder:released");await claim.release();}
 };
 return mock;
}

/** 🧱️ Every problem of every retained generation: a missing fragment or index, owners of the index that differ from the manifest, or edges of another environment. */
async function tornV1(store:GenerationStoreV1,dir:string):Promise<string[]>{
 const problems:string[]=[];
 for(const file of readdirSync(join(dir,".gen","m")).filter(name=>name.endsWith(".json"))){
  const manifest=await store.readManifestAt(file.slice(0,64));if(!manifest){problems.push("manifest "+file);continue;}
  const index=await store.readIndex(manifest.factIndex);if(!index){problems.push("index of "+manifest.generation);continue;}
  const indexed=index.owners.filter(name=>!isPseudoOwnerV1(name)).sort(),named=Object.keys(manifest.owners).sort();
  if(canonicalJsonV1(indexed)!==canonicalJsonV1(named))problems.push("owners of generation "+manifest.generation);
  for(const[name,row]of Object.entries(manifest.owners)){if(row.edgesEnv!==manifest.env)problems.push("environment of "+name);const fragment=await store.readFragment(row);if(!fragment)problems.push("fragment of "+name+" in "+manifest.generation);}
  await store.readGlobal(manifest).catch(()=>problems.push("global of "+manifest.generation));
 }
 return problems;
}

const overlap=(a:Span,b:Span)=>a.start<b.end&&b.start<a.end;

test("disjoint slices refresh at the same time, the later publisher merges onto the earlier generation, and the graph holds both edits",async()=>{
 const dir=scratch();
 try{
  await seed(dir,twoRings());await loadGenerationGraphV1(createMockHost(dir).host);
  await applyOps(dir,[edit("a1","x"),edit("b2","y")]);
  const shared={spans:[] as Span[],alone:[] as boolean[]},a=agent(dir,shared),b=agent(dir,shared);
  const[first,second]=await Promise.all([loadGenerationGraphV1(a.host,{closure:["a0"]}),loadGenerationGraphV1(b.host,{closure:["b0"]})]) as any[];
  expect(first.projects.a1.sources[0].sha256).toBe(sha("export const a1x=1;\n"));expect(second.projects.b2.sources[0].sha256).toBe(sha("export const b2y=1;\n"));
  expect(shared.spans.map(span=>span.owners)).toEqual(expect.arrayContaining([["a1"],["b2"]]));expect(shared.spans).toHaveLength(2);
  expect(overlap(shared.spans[0]!,shared.spans[1]!)).toBe(true);
  const keys=[...a.said,...b.said].map(row=>row.key);
  expect(keys.filter(key=>key==="published")).toHaveLength(1);expect(keys.filter(key=>key==="merged")).toHaveLength(1);expect(keys).not.toContain("blocked");
  expect(shared.alone).toHaveLength(2);expect(shared.alone[0]).toBe(false);
  const final=createMockHost(dir),graph=await loadGenerationGraphV1(final.host);
  expect(final.log).toEqual([]);expect(final.said.some(row=>row.key==="verified"&&row.text.startsWith("graph: generation 2 verified"))).toBe(true);
  if(python)expect(shape(graph)).toEqual(pythonGraph(dir));
  expect(await tornV1(createWorld(dir).store,dir)).toEqual([]);
  await applyOps(dir,[edit("b3","z")]);await loadGenerationGraphV1(agent(dir,shared).host,{closure:["b0"]});expect(shared.alone).toEqual([false,expect.any(Boolean),true]);
 }finally{clean(dir);}
},120000);

test("overlapping slices still build one after the other: the second waits with a notice and finds the work done",async()=>{
 const dir=scratch();
 try{
  await seed(dir,twoRings());await loadGenerationGraphV1(createMockHost(dir).host);
  await applyOps(dir,[edit("a1","x"),edit("a2","y")]);
  const shared={spans:[] as Span[],alone:[] as boolean[]},a=agent(dir,shared),b=agent(dir,shared);
  const graphs=await Promise.all([loadGenerationGraphV1(a.host,{closure:["a0"]}),loadGenerationGraphV1(b.host,{closure:["a3"]})]) as any[];
  for(const graph of graphs){expect(graph.projects.a1.sources[0].sha256).toBe(sha("export const a1x=1;\n"));expect(graph.projects.a2.sources[0].sha256).toBe(sha("export const a2y=1;\n"));}
  expect(shared.spans).toHaveLength(1);expect(shared.spans[0]!.owners).toEqual(["a1","a2"]);
  expect([...a.said,...b.said].filter(row=>row.key==="blocked").map(row=>row.text)).toEqual([expect.stringContaining("shared: a1, a2")]);
  expect([...a.said,...b.said].map(row=>row.key)).not.toContain("merged");
  if(python)expect(shape(await loadGenerationGraphV1(createMockHost(dir).host))).toEqual(pythonGraph(dir));
 }finally{clean(dir);}
},120000);

test("a structural change (a new project) is claimed for the whole workspace and never builds beside another refresh",async()=>{
 const dir=scratch();
 try{
  await seed(dir,twoRings());await loadGenerationGraphV1(createMockHost(dir).host);
  await applyOps(dir,[edit("a1","x"),{op:"write",path:"pkg/c0/project.json",content:JSON.stringify({name:"c0",package:"@syn/c0"})},{op:"write",path:"pkg/c0/src/index.ts",content:'import "@syn/b1";\nexport const c=0;\n'}]);
  const shared={spans:[] as Span[],alone:[] as boolean[]},a=agent(dir,shared),b=agent(dir,shared);
  await Promise.all([loadGenerationGraphV1(a.host,{closure:["a0"]}),loadGenerationGraphV1(b.host)]);
  for(let i=1;i<shared.spans.length;i++)expect(overlap(shared.spans[i-1]!,shared.spans[i]!)).toBe(false);
  const graph:any=await loadGenerationGraphV1(createMockHost(dir).host);expect(graph.projects.c0).toBeDefined();
  if(python)expect(shape(graph)).toEqual(pythonGraph(dir));
  expect(await tornV1(createWorld(dir).store,dir)).toEqual([]);
 }finally{clean(dir);}
},120000);

test("a lost swap of disjoint owners is merged, never recomputed, and readers see only whole generations meanwhile",async()=>{
 const dir=scratch();
 try{
  await seed(dir,twoRings());await createWorld(dir).refresh();
  await applyOps(dir,[edit("a1","x"),edit("b2","y")]);
  const barrier=gated(2),log:any[]=[],racing=()=>createWorld(dir,{sharedLog:log,computeWrap:compute=>async(spec,signal)=>{const result=await compute(spec,signal);await barrier.arrive();return result;}});
  let done=false;const failures:string[]=[],reader=async()=>{const world=createWorld(dir,{lease:false});while(!done){try{const{current,graph}=await world.store.snapshot();if(graph.generation!==current.manifest!.generation)failures.push("pointer");}catch(error){failures.push(String(error));}await sleep(0);}};
  const readers=Array.from({length:8},reader);
  const a=racing(),b=racing(),[first,second]=await Promise.all([a.refresh({bypassBuilder:true,only:["a0","a1","a2","a3"]}),b.refresh({bypassBuilder:true,only:["b0","b1","b2","b3"]})]);
  done=true;await Promise.all(readers);expect(failures).toEqual([]);
  expect(first.published&&second.published).toBe(true);expect(log).toHaveLength(2);
  const stats=[a.store.stats,b.store.stats];
  expect(stats.reduce((sum,row)=>sum+row.casConflicts,0)).toBe(1);expect(stats.reduce((sum,row)=>sum+row.merged,0)).toBe(1);expect(stats.reduce((sum,row)=>sum+row.mergeConflicts,0)).toBe(0);
  expect([first,second].map(row=>row.current.manifest!.generation).sort()).toEqual([1,2]);expect([first,second].filter(row=>row.merged===1)).toHaveLength(1);
  const final=await createWorld(dir).refresh();expect(final.built).toBe(false);expect(final.current.manifest!.generation).toBe(2);
  if(python)expect(normalized(await createWorld(dir).graph(final.current))).toEqual(pythonGraph(dir));
  expect(await tornV1(createWorld(dir).store,dir)).toEqual([]);
 }finally{clean(dir);}
},120000);

test("a lost swap of an owner the winner also wrote refuses the merge and computes again from the newest generation",async()=>{
 const dir=scratch();
 try{
  await seed(dir,twoRings());await createWorld(dir).refresh();
  await applyOps(dir,[edit("a1","x"),edit("b2","y")]);
  const barrier=gated(2),log:any[]=[],racing=()=>createWorld(dir,{sharedLog:log,computeWrap:compute=>async(spec,signal)=>{const result=await compute(spec,signal);if(log.length<=2)await barrier.arrive();return result;}});
  const a=racing(),b=racing(),results=await Promise.all([a.refresh({bypassBuilder:true}),b.refresh({bypassBuilder:true})]);
  const stats=[a.store.stats,b.store.stats];
  expect(stats.reduce((sum,row)=>sum+row.casConflicts,0)).toBe(1);expect(stats.reduce((sum,row)=>sum+row.mergeConflicts,0)).toBe(1);expect(stats.reduce((sum,row)=>sum+row.merged,0)).toBe(0);
  expect(stats.reduce((sum,row)=>sum+row.published,0)).toBe(1);
  for(const result of results)expect(result.current.manifest!.generation).toBe(1);
  if(python)expect(normalized(await createWorld(dir).graph(results[0]!.current))).toEqual(pythonGraph(dir));
 }finally{clean(dir);}
},120000);

test("a lost swap against a peer that added a project refuses the merge, computes again and keeps both changes",async()=>{
 const dir=scratch();
 try{
  await seed(dir,twoRings());await createWorld(dir).refresh();
  await applyOps(dir,[edit("a1","x"),{op:"write",path:"pkg/c0/project.json",content:JSON.stringify({name:"c0",package:"@syn/c0"})},{op:"write",path:"pkg/c0/src/index.ts",content:'export const c=0;\n'}]);
  const barrier=gated(2),log:any[]=[],racing=()=>createWorld(dir,{sharedLog:log,computeWrap:compute=>async(spec,signal)=>{const result=await compute(spec,signal);if(log.length<=2)await barrier.arrive();return result;}});
  const a=racing(),b=racing();
  const results=await Promise.all([a.refresh({bypassBuilder:true,only:["a0","a1","a2","a3"]}),b.refresh({bypassBuilder:true})]);
  expect([a.store.stats,b.store.stats].reduce((sum,row)=>sum+row.mergeConflicts,0)).toBe(1);
  const final=await createWorld(dir).refresh();expect(final.built).toBe(false);
  if(python)expect(normalized(await createWorld(dir).graph(final.current))).toEqual(pythonGraph(dir));
  expect(Object.keys(final.current.manifest!.owners)).toContain("c0");expect(results.every(result=>result.current.manifest!.generation>=1)).toBe(true);
  expect(await tornV1(createWorld(dir).store,dir)).toEqual([]);
 }finally{clean(dir);}
},120000);
