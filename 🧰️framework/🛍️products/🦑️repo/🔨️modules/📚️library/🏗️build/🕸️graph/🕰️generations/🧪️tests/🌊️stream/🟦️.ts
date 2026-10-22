/** 🌊️ Contract: the graph of a generation is assembled fragment by fragment. The collected assembly equals the independent model for every selection, the node stream is ordered, single use and cancellable, and the resident state of a consumer that drops what it was given does not depend on the number of owners (a synthetic corpus of 5,000 projects). */
import{expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{heapStats}from"bun:jsc";
import Ajv from"ajv";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";
import portable from"../../../🧪️tests/📤️portable/🔣️.json";
import{canonicalJsonV1,createFrontierV1,createGenerationStoreV1}from"../../🟨️.mjs";
import{expandInChunksV1,expandStreamV1}from"../../🔌️load/🟨️.mjs";
import{createMemoryFilePort}from"../🏃️world/🟦️.ts";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️oracle.py"),sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const IDENTITY=sha("assembly-identity"),IMPLEMENTATION=sha("assembly-implementation");
type Owner={name:string;root:string;kind:"project"|"unavailable";node:unknown;edges:unknown[]};
type Corpus={owners:Owner[];global:{externalNodes:Record<string,unknown>;edges:unknown[];unavailable:unknown[]}};

/** 🏗️ Publishes one generation of the corpus into an in-memory store and returns the store with its sealed generation. */
async function publish(corpus:Corpus){
 const memory=createMemoryFilePort(),store=createGenerationStoreV1({root:"/store",fs:memory.port as never,holds:async()=>true,resources:{facts:"facts"}});
 const result=await store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,membership:async()=>({digest:sha("membership"),frontier:createFrontierV1({directories:[],members:[],capturedAt:0}),changes:[]}),compute:async()=>({
  owners:corpus.owners.map(owner=>({name:owner.name,root:owner.root,kind:owner.kind,node:owner.node,edges:owner.edges,targets:Object.keys((owner.node as {targets?:object}).targets??{}),rows:[]})),
  env:{digest:sha("environment"),rows:[]},global:{version:1,...corpus.global},
 })});
 return{store,current:result.current};
}
const normalized=(value:unknown)=>JSON.parse(canonicalJsonV1(value));
const strip=({generation,membership,...rest}:{generation:number;membership:string;[key:string]:unknown})=>normalized(rest);

test("the corpus is closed under its schema and the independent model reproduces every expectation",()=>{
 expect(new Ajv({strict:false}).validate(fixtureSchema,fixture)).toBe(true);
 if(python){const done=Bun.spawnSync([python,"-I",oracle,"expect",join(here,"🔣️.json")]);expect(done.exitCode).toBe(0);expect(JSON.parse(done.stdout.toString())).toEqual(fixture.expect);}
});

test("the collected assembly and the streamed assembly equal the model for every selection",async()=>{
 const{store,current}=await publish(fixture as unknown as Corpus);
 for(const[index,row]of fixture.cases.entries()){
  const options=row.names===null?{}:{names:row.names},whole=await store.assemble(current,options);
  expect(strip(whole as never),row.name).toEqual(fixture.expect[index]);
  const stream=store.assembleStream(current,options),seen:string[]=[],projects:Record<string,unknown>={};
  for await(const[name,node]of stream.nodes()){seen.push(name);projects[name]=node;}
  const rest=await stream.finish();
  expect(seen,row.name).toEqual([...seen].sort());expect(strip({...rest,projects} as never),row.name).toEqual(fixture.expect[index]);
  expect(Object.keys(whole.projects),row.name).toEqual(seen);
 }
});

test("a node stream is read once, is finished only after it was consumed, reports progress and stops at an abort",async()=>{
 const{store,current}=await publish(fixture as unknown as Corpus),events:{done:number;total:number}[]=[];
 const stream=store.assembleStream(current,{progress:event=>{events.push({done:event.done,total:event.total});}});
 await expect(stream.finish()).rejects.toMatchObject({code:"GENERATION_STREAM_OPEN"});
 const reader=stream.nodes();await reader.next();
 await expect(stream.finish()).rejects.toMatchObject({code:"GENERATION_STREAM_OPEN"});
 for await(const _ of reader);
 await stream.finish();
 await expect(stream.nodes().next()).rejects.toMatchObject({code:"GENERATION_STREAM_USED"});
 const total=fixture.owners.length;expect(events[0]).toEqual({done:0,total});expect(events.at(-1)).toEqual({done:total,total});expect(events.map(row=>row.done)).toEqual(Array.from({length:total+1},(_,i)=>i));
 const controller=new AbortController(),cancelled=store.assembleStream(current,{signal:controller.signal}),pulled:string[]=[];
 await expect((async()=>{for await(const[name]of cancelled.nodes()){pulled.push(name);if(pulled.length===2)controller.abort(Error("stopped"));}})()).rejects.toThrow("stopped");
 expect(pulled.length).toBe(2);
 expect(()=>store.assembleStream(current,{names:["nobody"]})).toThrow("Unknown project in the closure: nobody");
});

test("the streaming expansion forms the same project and byte groups as the fixed batching of the portable corpus",async()=>{
 for(const row of portable.batches){
  const projects:Record<string,unknown>={};for(const[index,units]of row.lengths.entries())projects["p"+index]={payload:row.glyph.repeat((units-14)/Buffer.byteLength(row.glyph))};
  const limits={maximumProjects:portable.policy.maximumProjects,maximumBytes:portable.policy.maximumBytes},sequential:string[][]=[],streamed:string[][]=[];
  const a=await expandInChunksV1(projects,async chunk=>{sequential.push(Object.keys(chunk));return chunk;},limits);
  async function*source(){for(const entry of Object.entries(projects))yield entry as [string,unknown];}
  const b=await expandStreamV1(source(),async chunk=>{streamed.push(Object.keys(chunk));return chunk;},limits);
  expect(streamed,row.id).toEqual(sequential);expect(streamed.map(group=>group.length),row.id).toEqual(row.groups);expect(Object.keys(b),row.id).toEqual(Object.keys(a));
 }
});

/** 🏭️ A synthetic owner of about 17 KB of node JSON: its own command lines, input lists (most paths shared between owners like the closure paths of a real workspace) and one import edge. */
function syntheticOwner(index:number,count:number):Owner{
 const name="s"+index,targets:Record<string,unknown>={};
 for(let target=0;target<8;target++)targets["t"+target]={execution:{kind:"command",command:`run ${name} t${target} --flag ${index*31+target}`},inputs:[...Array.from({length:10},(_,item)=>`{workspaceRoot}/shared/library/part${target}/file${item}.ts`),`{projectRoot}/src/${name}/part${target}.ts`,`{projectRoot}/${name}/package.json`],weights:Array.from({length:300},(_,item)=>index*100000+target*300+item)};
 return{name,root:"pkg/"+name,kind:"project",node:{name,root:"pkg/"+name,targets},edges:[{source:name,target:"s"+((index+1)%count),type:"static"}]};
}

/** 📏️ The live managed bytes after a full collection. */
function live():number{Bun.gc(true);const statistics=heapStats();return statistics.heapSize+statistics.extraMemorySize;}

async function streamedGrowth(count:number){
 const{store,current}=await publish({owners:Array.from({length:count},(_,i)=>syntheticOwner(i,count)),global:{externalNodes:{},edges:[],unavailable:[]}});
 const base=live();let peak=0,seen=0,digest=createHash("sha256");
 const stream=store.assembleStream(current);
 for await(const[name,node]of stream.nodes()){digest.update(name+JSON.stringify(node).length);if(++seen%250===0)peak=Math.max(peak,live()-base);}
 const rest=await stream.finish();expect(seen).toBe(count);expect(rest.dependencies.length).toBe(count);
 const kept=live();const collected=await store.assemble(current);const retained=live()-kept;expect(Object.keys(collected.projects).length).toBe(count);
 return{peak,retained,digest:digest.digest("hex")};
}

test("the resident state of a streaming consumer does not grow with the number of projects while collecting does (5,000 synthetic projects)",async()=>{
 const small=await streamedGrowth(1000),large=await streamedGrowth(5000);
 expect(large.peak).toBeLessThan(small.peak*1.5+16*1048576);
 expect(large.retained).toBeGreaterThan(large.peak*4);
 expect(large.retained).toBeGreaterThan(small.retained*3);
},300000);
