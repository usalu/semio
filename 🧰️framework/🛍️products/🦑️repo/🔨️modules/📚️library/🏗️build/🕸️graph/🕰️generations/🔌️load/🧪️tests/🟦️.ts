/** 🔌️ Contract: the repository protocol of the graph generation. A shared reader serves a verified or current generation without ever entering the builder custody; only a stale generation under a fresh policy enters it, the facts resource is held only for the pointer swap, closures read only their own projects, texts come in English and German, and partial inference results are shaped into exactly the facts and edges each owner consumed. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import closureCustody from "./🚧️closure-custody/🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import {GENERATION_MESSAGES_V1,extractGraphPolicyV1,implementationDigestV1,loadGenerationGraphV1,renderGenerationMessageV1,scopeImplementationRowsV1,sliceClosureV1,sliceViewV1,expandInChunksV1,shapeComputeResultV1,survivingStubsV1} from "../🟨️.mjs";
const graphOfV1=async(...input:Parameters<typeof loadGenerationGraphV1>)=>(await loadGenerationGraphV1(...input)).graph;
import {GENERATION_POLICY_V1,canonicalJsonV1,createNodeFilePortV1,createGenerationStoreV1,type GenerationRowV1} from "../../🟨️.mjs";
import {createFileClaimLeasePortV1} from "../../🧩️claims/🟨️.mjs";
import {createSliceCustodyV1} from "../../🔐️custody/🟨️.mjs";
import {encodeGenerationPinV1,generationPinOpenV1,readGenerationPinV1} from "../../🧷️pin/🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../🧪️tests/🏃️world/🟦️.ts";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"..","..","🧪️tests","🐍️.py");
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-load-"));
const clean=(dir:string)=>rmSync(dir,{recursive:true,force:true});
const sleep=(ms:number)=>new Promise<void>(resolve=>setTimeout(resolve,ms));
const pythonGraph=(dir:string,names?:string[])=>{const graph=JSON.parse(Bun.spawnSync([python!,"-I",oracle,"graph",dir]).stdout.toString());if(!names)return graph;return{...graph,projects:Object.fromEntries(Object.entries(graph.projects).filter(([name])=>names.includes(name))),dependencies:graph.dependencies.filter((edge:any)=>names.includes(edge.source)&&names.includes(edge.target))};};
const normalized=(value:unknown)=>JSON.parse(canonicalJsonV1(value));
const shape=(graph:any)=>normalized({projects:graph.projects,dependencies:graph.dependencies,externalNodes:graph.externalNodes,unavailable:graph.unavailable??[]});

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

test("the bilingual texts are the declared ones, equal in both languages by placeholder, and rendering fills them",()=>{
 expect(JSON.parse(JSON.stringify(GENERATION_MESSAGES_V1))).toEqual(schema.definitions.Messages.const);
 expect(renderGenerationMessageV1("published",{generation:7,computed:2,carried:5,seconds:3},"en")).toBe("graph: generation 7 published, 2 projects refreshed in 3 s (5 kept their edges)");
 expect(renderGenerationMessageV1("published",{generation:7,computed:2,carried:5,seconds:3},"de")).toBe("Graph: Generation 7 veröffentlicht, 2 Projekte in 3 s erneuert (5 behielten ihre Kanten)");
 expect(()=>renderGenerationMessageV1("other" as never,{},"en")).toThrow();
});


for(const row of fixture.policy)test("policy flag "+JSON.stringify([row.args,row.env]),()=>{
 if((row.expected as {error?:boolean}).error)expect(()=>extractGraphPolicyV1(row.args,row.env)).toThrow();
 else expect(extractGraphPolicyV1(row.args,row.env)).toEqual(row.expected as never);
});

test("shaping routes facts, edges and unclaimed facts exactly like the independent model",()=>{
 const{workspaceRoot,part,stubs,expected}=fixture.shape;
 expect(shapeComputeResultV1({workspaceRoot,part:part as never,stubs:stubs as never})).toEqual(expected as never);
 const owners=shapeComputeResultV1({workspaceRoot,part:part as never,stubs:stubs as never}).owners;
 expect(owners.find(owner=>owner.name==="root")!.rows!.length).toBe(1);
 expect(owners.find(owner=>owner.name==="b")).toMatchObject({partial:"edges"});
});

test("a shared reader never enters the builder; a stale generation does and holds the facts resource only for the swap",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const mock=createMockHost(dir);
  const cold=(await loadGenerationGraphV1(mock.host)).graph as any;
  expect(mock.events).toEqual(["shared","builder:wait","builder:held","builder:released"]);expect(shape(cold)).toEqual(pythonGraph(dir));expect(mock.said.at(-2)!.text).toContain("published");expect(mock.said.at(-1)!.text).toContain("fact index of generation 0");
  mock.events.length=0;mock.holdsLog.length=0;const warm=(await loadGenerationGraphV1(mock.host)).graph as any;
  expect(mock.events).toEqual(["shared"]);expect(mock.holdsLog).toEqual([]);expect(shape(warm)).toEqual(shape(cold));
  await applyOps(dir,[{op:"write",path:"pkg/p2/src/index.ts",content:'import "@syn/p4";\nexport const v=2;\n'}]);
  mock.events.length=0;mock.holdsLog.length=0;mock.log.length=0;const edited=(await loadGenerationGraphV1(mock.host)).graph as any;
  expect(mock.events).toEqual(["shared","builder:wait","builder:held","builder:released"]);expect(mock.log).toEqual([{full:["p2"],edges:[],cold:false}]);
  expect(mock.holdsLog.map(row=>row.resource)).toEqual(["repository-graph-facts:mock"]);expect(mock.holdsLog[0]!.heldMs).toBeLessThan(100);
  expect(shape(edited)).toEqual(pythonGraph(dir));
 }finally{clean(dir);}
},60000);

test("a current policy serves the newest generation without verification or a builder; without one it refuses and builds nothing",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(5));const mock=createMockHost(dir),refused=await loadGenerationGraphV1(mock.host,{policy:"current"}).then(()=>undefined,error=>error as Error&{code?:string});
  expect(refused?.code).toBe("GENERATION_NONE_SEALED");expect(mock.events).toEqual(["shared"]);
  expect(shape((await loadGenerationGraphV1(mock.host)).graph)).toEqual(pythonGraph(dir));
  await applyOps(dir,[{op:"write",path:"pkg/p1/src/index.ts",content:"export const stale=1;\n"}]);
  mock.events.length=0;mock.said.length=0;const served=(await loadGenerationGraphV1(mock.host,{policy:"current"})).graph as any;
  expect(mock.events).toEqual(["shared"]);expect(mock.said.filter(row=>row.key!=="sealedAge").map(row=>row.text)).toEqual(["graph: serving generation 0 without verification (graph policy current)"]);
  expect(served.projects.p1.sources.length).toBe(1);expect(served.projects.p1.sources[0].path).toBe("pkg/p1/src/index.ts");
  expect(shape(served)).not.toEqual(pythonGraph(dir));
  mock.events.length=0;const fresh=(await loadGenerationGraphV1(mock.host)).graph as any;expect(mock.events).toContain("builder:held");expect(shape(fresh)).toEqual(pythonGraph(dir));
  const german=createMockHost(dir,{language:"de"});(await loadGenerationGraphV1(german.host,{policy:"current"})).graph;
  expect(german.said.filter(row=>row.key!=="sealedAge").map(row=>row.text)).toEqual(["Graph: Generation 1 wird ohne Prüfung geliefert (Graph-Richtlinie current)"]);
 }finally{clean(dir);}
},60000);

test("a builder asks the remote exchange to restore first and publishes what it built; a served load only offers, and a read-only machine never writes",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(3));const mock=createMockHost(dir),calls:string[]=[],outcome={status:"uploaded",generation:0,owners:3,documents:4,bytes:2097152};
  (mock.host as any).exchange={upload:true,restore:async()=>{calls.push("restore");return{status:"absent"};},publish:async(options:{ifAbsent:boolean})=>{calls.push(options.ifAbsent?"offer":"publish");return outcome;}};
  await loadGenerationGraphV1(mock.host);expect(calls).toEqual(["restore","publish"]);
  expect(mock.said.map(row=>row.text)).toContain("graph: generation 0 (3 projects) uploaded to the remote cache, 4 documents (2 MB) in 0 s");
  calls.length=0;await loadGenerationGraphV1(mock.host);expect(calls).toEqual(["offer"]);
  const readOnly=createMockHost(dir);(readOnly.host as any).exchange={upload:false,restore:async()=>({status:"absent"}),publish:async()=>{throw Error("read-only machines never write");}};
  await loadGenerationGraphV1(readOnly.host);
 }finally{clean(dir);}
},60000);

test("a closure reads and expands only its own projects and never enters the builder while its facts hold",async()=>{
 const dir=scratch();
 try{
  const files:Record<string,string>={};
  for(const[name,imports]of[["a",["@syn/b"]],["b",["@syn/c"]],["c",[]],["d",["@syn/e"]],["e",[]]] as [string,string[]][])files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:[]}),files[`pkg/${name}/src/index.ts`]=imports.map(spec=>`import "${spec}";`).join("\n")+"\nexport const v=1;\n";
  await seed(dir,files);const mock=createMockHost(dir);(await loadGenerationGraphV1(mock.host)).graph;
  await applyOps(dir,[{op:"write",path:"pkg/e/src/index.ts",content:"export const v=2;\n"}]);
  mock.events.length=0;mock.expanded.length=0;const graph=(await loadGenerationGraphV1(mock.host,{closure:["a"]})).graph as any;
  expect(mock.events).toEqual(["shared"]);expect(mock.expanded).toEqual([["a","b","c"]]);expect(shape(graph)).toEqual(pythonGraph(dir,["a","b","c"]));
  expect(mock.said.at(-1)!.text).toBe("graph: 3 projects of the closure verified, no other project read");
  await applyOps(dir,[{op:"write",path:"pkg/c/src/index.ts",content:"export const v=3;\n"}]);
  mock.events.length=0;mock.log.length=0;const rebuilt=(await loadGenerationGraphV1(mock.host,{closure:["a"]})).graph as any;
  expect(mock.events).toContain("builder:held");expect(mock.log[0]!.full).toEqual(["c"]);expect(shape(rebuilt)).toEqual(pythonGraph(dir,["a","b","c"]));
 }finally{clean(dir);}
},60000);

test("readers are served while a builder holds its custody; a dirty reader queues behind it and then adopts its generation",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(5));const first=createMockHost(dir);(await loadGenerationGraphV1(first.host)).graph;
  await applyOps(dir,[{op:"write",path:"pkg/p0/src/index.ts",content:"export const v=9;\n"}]);
  let open!:()=>void;const gate=new Promise<void>(resolve=>{open=resolve;}),slow=createMockHost(dir,{gate:()=>gate});
  const builder=graphOfV1(slow.host) as Promise<any>;
  while(!slow.events.includes("builder:held"))await new Promise(resolve=>setTimeout(resolve,5));
  const peer=createMockHost(dir),race=await Promise.race([graphOfV1(peer.host,{policy:"current"}),new Promise(resolve=>setTimeout(()=>resolve("timeout"),400))]);
  expect(race).not.toBe("timeout");
  const waiting=graphOfV1(peer.host,{policy:"fresh"}) as Promise<any>,state={done:false};void waiting.then(()=>{state.done=true;});
  await new Promise(resolve=>setTimeout(resolve,150));
  open();const a=await builder,b=await waiting;
  expect(shape(a)).toEqual(pythonGraph(dir));expect(shape(b)).toEqual(shape(a));
 }finally{clean(dir);}
},60000);

test("progress counters are the shared graph phases and a cancelled load leaves the pointer alone",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(4));const mock=createMockHost(dir);(await loadGenerationGraphV1(mock.host)).graph;
  expect(mock.published.every(resource=>/^build:graph-phase:(facts|fragments|assembly|patching|packing|publishing):\d+:\d+(?::.*)?$/u.test(resource))).toBe(true);
  expect(mock.published).toContain("build:graph-phase:fragments:4:4");
  expect(mock.published.some(resource=>resource.startsWith("build:graph-phase:assembly:"))).toBe(true);
  await applyOps(dir,[{op:"write",path:"pkg/p1/src/index.ts",content:"export const v=5;\n"}]);
  const controller=new AbortController(),cancelled=createMockHost(dir,{signal:controller.signal});controller.abort(new Error("cancelled"));
  await expect(graphOfV1(cancelled.host)).rejects.toThrow("cancelled");
  const after=createMockHost(dir),graph=(await loadGenerationGraphV1(after.host)).graph as any;expect(shape(graph)).toEqual(pythonGraph(dir));expect(after.log.length).toBe(1);
 }finally{clean(dir);}
},60000);

for(const[index,row]of fixture.implementation.entries())test("implementation facts are scoped and stamped like the independent model "+index,()=>{
 const scoped=scopeImplementationRowsV1(row.rows as never),expected=row.expected as {all:unknown[];environment:unknown[];digest:string};
 expect({all:scoped.all,environment:scoped.environment}).toEqual({all:expected.all,environment:expected.environment} as never);
 expect(implementationDigestV1(row.rows as never)).toBe(expected.digest);
});

test("the implementation digest is the same on every machine of a fleet: it names relative paths, content digests and stamps, never a stat fact, a root or an order",()=>{
 const row=(path:string,sha256:string,stat:Readonly<{size:number;mtimeMs:number}>)=>({kind:"digest",path,fact:{sha256,...stat}}),stamp={kind:"stamp",path:"inferrer:rust",fact:"semio.graph-inferrer.rust/3+0123"};
 const here=[row("libs/a/data.json","aa",{size:1,mtimeMs:10}),row("libs/b/data.json","bb",{size:2,mtimeMs:20}),stamp],there=[stamp,row("libs/b/data.json","bb",{size:2,mtimeMs:99}),row("libs/a/data.json","aa",{size:7,mtimeMs:77})];
 expect(implementationDigestV1(here as never)).toBe(implementationDigestV1(there as never));
 expect(implementationDigestV1(here as never)).not.toBe(implementationDigestV1([row("libs/a/data.json","ac",{size:1,mtimeMs:10}),row("libs/b/data.json","bb",{size:2,mtimeMs:20}),stamp] as never));
 expect(JSON.stringify(here)).not.toMatch(/\/(?:Users|home|private)\/|[A-Za-z]:\\/u);
});

test("shaping adds the implementation facts of the consumers: every project the data files, the environment also the stamps of the inferrers that shape edges",()=>{
 const{workspaceRoot,part,stubs,implementation,expected}=fixture.shapeImplementation;
 expect(shapeComputeResultV1({workspaceRoot,part:part as never,stubs:stubs as never,implementation:implementation as never})).toEqual(expected as never);
});

test("the stale generation text is declared in both languages",()=>{
 expect(renderGenerationMessageV1("stale",{generation:2},"en")).toBe("graph: serving generation 2 built by another implementation (graph policy current)");
 expect(renderGenerationMessageV1("stale",{generation:2},"de")).toBe("Graph: Generation 2 stammt von einer anderen Implementierung und wird geliefert (Graph-Richtlinie current)");
});

const stampTable=(nodes="n/1",core="c/1",edges="e/1")=>({"inferrer:nodes":{stamp:"semio.graph-inferrer.nodes/"+nodes.split("/")[1],scope:"project" as const},"inferrer:core":{stamp:"semio.graph-inferrer.core/"+core.split("/")[1],scope:"all" as const},"inferrer:edges":{stamp:"semio.graph-inferrer.edges/"+edges.split("/")[1],scope:"environment" as const}});
const stampOf=(dir:string):string=>{const pointer=JSON.parse(readFileSync(join(dir,".gen","current.json"),"utf8"));return JSON.parse(readFileSync(join(dir,".gen","m",pointer.manifest+".json"),"utf8")).implementation;};
const brokenRing=(count:number,broken:number)=>{const files=ringFiles(count);for(let i=count-broken;i<count;i++)files[`pkg/p${i}/project.json`]=JSON.stringify({name:`p${i}`,broken:true});return files;};
const names=(count:number)=>Array.from({length:count},(_,i)=>"p"+i).sort();

test("a generation names the stamps of the inferrers, never the code: tool source edits refresh nothing, one project edit one project, a bumped stamp exactly its consumers, and each reason names its input in English and German",async()=>{
 const dir=scratch(),every=names(12),valid=names(10),files={...brokenRing(12,2),"tool/graph.mjs":"1","data/policy.json":"{}"};
 try{
  await seed(dir,files);const options={implementationFiles:["data/policy.json"],stamps:stampTable()};
  const loader=createMockHost(dir,options);(await loadGenerationGraphV1(loader.host)).graph;
  const stampA=implementationDigestV1(await loader.host.implementationRows());expect(stampOf(dir)).toBe(stampA);expect(loader.log).toEqual([{full:every,edges:[],cold:true}]);expect(loader.said.map(row=>row.key)).toContain("cold");
  await applyOps(dir,[{op:"write",path:"tool/graph.mjs",content:"2"}]);
  const tool=createMockHost(dir,options);(await loadGenerationGraphV1(tool.host)).graph;
  expect(tool.events).toEqual(["shared"]);expect(tool.log).toEqual([]);expect(stampOf(dir)).toBe(stampA);
  await applyOps(dir,[{op:"write",path:"pkg/p3/project.json",content:JSON.stringify({name:"p3",package:"@syn/p3",uses:[],edit:1})}]);
  const manifest=createMockHost(dir,options);(await loadGenerationGraphV1(manifest.host)).graph;
  expect(manifest.log).toEqual([{full:["p3"],edges:[],cold:false}]);
  const reason=manifest.said.find(row=>row.key==="reason");expect(reason!.text).toMatch(/^graph: p3 is stale: .*pkg\/p3\/project\.json/u);
  const german=createMockHost(dir,{...options,language:"de"});await applyOps(dir,[{op:"write",path:"pkg/p3/project.json",content:JSON.stringify({name:"p3",package:"@syn/p3",uses:[],edit:2})}]);(await loadGenerationGraphV1(german.host)).graph;
  expect(german.said.find(row=>row.key==="reason")!.text).toMatch(/^Graph: p3 ist veraltet: .*pkg\/p3\/project\.json/u);
  const nodes=createMockHost(dir,{...options,stamps:stampTable("n/2")});(await loadGenerationGraphV1(nodes.host)).graph;
  expect(nodes.log).toEqual([{full:every,edges:[],cold:false}]);expect(nodes.said.find(row=>row.key==="reason")!.text).toContain("the inference protocol of nodes changed from semio.graph-inferrer.nodes/1 to semio.graph-inferrer.nodes/2");
  const stampB=implementationDigestV1(await nodes.host.implementationRows());expect(stampB).not.toBe(stampA);expect(stampOf(dir)).toBe(stampB);
  const edges=createMockHost(dir,{...options,stamps:stampTable("n/2","c/1","e/2")});(await loadGenerationGraphV1(edges.host)).graph;
  expect(edges.log).toEqual([{full:[],edges:[],cold:false}]);expect(edges.said.find(row=>row.key==="reason")!.text).toContain("@env is stale");expect(edges.said.some(row=>row.text.includes("edges changed from"))).toBe(true);
  const core=createMockHost(dir,{...options,stamps:stampTable("n/2","c/2","e/2")});(await loadGenerationGraphV1(core.host)).graph;
  expect(core.log).toEqual([{full:every,edges:[],cold:false}]);
  await applyOps(dir,[{op:"write",path:"data/policy.json",content:'{"v":2}'}]);
  const data=createMockHost(dir,{...options,stamps:stampTable("n/2","c/2","e/2")});(await loadGenerationGraphV1(data.host)).graph;
  expect(data.log).toEqual([{full:every,edges:[],cold:false}]);
  loader.events.length=0;loader.said.length=0;const stale=(await loadGenerationGraphV1(loader.host,{policy:"current"})).graph as any;
  expect(loader.events).toEqual(["shared"]);expect(loader.said.filter(row=>row.key!=="sealedAge").map(row=>row.text)).toEqual(["graph: serving generation "+(JSON.parse(readFileSync(join(dir,".gen","current.json"),"utf8")).generation)+" built by another implementation (graph policy current)"]);expect(Object.keys(stale.projects).length).toBe(10);
  loader.events.length=0;(await loadGenerationGraphV1(loader.host)).graph;expect(loader.events).toEqual(["shared","builder:wait","builder:held","builder:released"]);expect(valid.length).toBe(10);
 }finally{clean(dir);}
},120000);

test("a tool source edited every few hundred milliseconds never restarts a 300 project build and never refreshes a project afterwards",async()=>{
 const count=300,broken=30,files={...brokenRing(count,broken),"tool/graph.mjs":"0"};
 const quiet=scratch(),busy=scratch(),options={stamps:stampTable(),ownerDelayMs:8};
 try{
  await seed(quiet,files);const timed=createMockHost(quiet,options),started=process.cpuUsage();await loadGenerationGraphV1(timed.host);
  const baseline=process.cpuUsage(started);
  await seed(busy,files);const loaded=createMockHost(busy,options),loadedStamp=implementationDigestV1(await loaded.host.implementationRows());
  let editing=true,edits=0;
  const editor=(async()=>{while(editing){await sleep(250);if(!editing)break;await writeFile(join(busy,"tool/graph.mjs"),String(++edits));}})();
  const begin=process.cpuUsage();
  const graph=(await loadGenerationGraphV1(loaded.host).finally(async()=>{editing=false;await editor;})).graph as any;
  const work=process.cpuUsage(begin);
  expect(edits).toBeGreaterThanOrEqual(3);expect(loaded.log.length).toBe(1);expect(loaded.log).toEqual(timed.log);
  expect(loaded.said.at(-2)!.text).toContain("published");expect(loaded.said.at(-1)!.text).toContain("fact index of generation");
  expect(work.user+work.system).toBeLessThanOrEqual(1.2*(baseline.user+baseline.system)+500_000);
  expect(stampOf(busy)).toBe(loadedStamp);expect(Object.keys(graph.projects).length).toBe(count-broken);
  const next=createMockHost(busy,options);(await loadGenerationGraphV1(next.host)).graph;
  expect(next.log).toEqual([]);expect(next.events).toEqual(["shared"]);expect(stampOf(busy)).toBe(implementationDigestV1(await next.host.implementationRows()));
 }finally{clean(quiet);clean(busy);}
},240000);


const sliceTree=()=>{
 const files:Record<string,string>={};
 for(const[name,imports]of[["a",["@syn/b"]],["b",["@syn/c"]],["c",[]],["d",["@syn/e"]],["e",[]],["f",[]]] as [string,string[]][])files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:[]}),files[`pkg/${name}/src/index.ts`]=imports.map(spec=>`import "${spec}";`).join("\n")+"\nexport const v=1;\n";
 return files;
};


for(const[index,row]of(fixture as any).closures.entries())test("selected closure is the independent model "+index,async()=>{
 const owners=Object.fromEntries(Object.entries(row.owners).map(([name,value]:[string,any])=>[name,value.root])),fragments=row.owners as Record<string,any>;
 const run=()=>sliceClosureV1({owners,fragment:async(name:string)=>({edges:fragments[name].edges.map((target:string)=>({source:name,target})),node:{targets:fragments[name].targets,namedInputs:fragments[name].namedInputs}}),selection:row.selection as never,roots:row.roots,external:row.external??[],providers:row.providers??[]});
 if(row.expected.error)await expect(run()).rejects.toMatchObject({code:"GENERATION_UNKNOWN_OWNER"});else expect([...await run()].sort()).toEqual(row.expected.admitted);
});


for(const[index,row]of(fixture as any).closures.entries())test("selected closure over the slice view of the fragments is the independent model "+index,async()=>{
 const owners=Object.fromEntries(Object.entries(row.owners).map(([name,value]:[string,any])=>[name,value.root])),fragments=row.owners as Record<string,any>;
 const run=()=>sliceClosureV1({owners,fragment:async(name:string)=>sliceViewV1({edges:fragments[name].edges.map((target:string)=>({source:name,target,type:"static"})),node:{targets:Object.fromEntries(Object.entries(fragments[name].targets??{}).map(([target,operation]:[string,any])=>[target,{...operation,executor:"repo:noop",options:{big:"x".repeat(64)},outputs:["o"]}])),namedInputs:fragments[name].namedInputs}}),selection:row.selection as never,roots:row.roots,external:row.external??[],providers:row.providers??[]});
 if(row.expected.error)await expect(run()).rejects.toMatchObject({code:"GENERATION_UNKNOWN_OWNER"});else expect([...await run()].sort()).toEqual(row.expected.admitted);
});

test("the slice view keeps only what the closure reads",()=>{
 const view=sliceViewV1({edges:[{target:"b",source:"a",sourceFile:"x"} as never],node:{targets:{build:{inputs:["{workspaceRoot}/x",{projects:["b"],input:"nativeSources"},{env:"E"}],dependsOn:["^build"],executor:"repo:script",options:{command:"x"}} as never}}});
 expect(view).toEqual({edges:[{target:"b"}],node:{targets:{build:{inputs:[{input:"nativeSources",projects:["b"]}],dependsOn:["^build"]}}}});
 const grouped=sliceViewV1({edges:[],node:{targets:{test:{inputs:["default","^shared","!{projectRoot}/dist/**",{env:"E"}]}},namedInputs:{default:["{projectRoot}/**/*",{input:"cargoToolchain",projects:["proctor"],extra:1} as never],shared:[{input:"own",dependencies:true}]}}});
 expect(grouped).toEqual({edges:[],node:{targets:{test:{inputs:["default","^shared"],dependsOn:undefined}},namedInputs:{default:[{input:"cargoToolchain",projects:["proctor"]}],shared:[{input:"own",dependencies:true}]}}});
});

test("an orchestrating build pins a generation that covers the nested run it launches: the outer slice admits the declared children, the nested slice is inside it",async()=>{
 const fragments:Record<string,{root:string;edges:string[];targets:Record<string,unknown>}>={
  dash:{root:"pkg/dash",edges:["lib"],targets:{build:{executor:"repo:script",metadata:{semio:{execution:{version:1,kind:"orchestration",children:[{project:"dash",target:"compile"}]},cache:{class:"driver-owned"}}}},compile:{inputs:["default",{input:"scriptProvider:leases:native",projects:["providers"]}]}}},
  providers:{root:"lib/providers",edges:["process"],targets:{}},process:{root:"process",edges:["framework"],targets:{}},framework:{root:"framework",edges:["value"],targets:{}},value:{root:"value",edges:[],targets:{}},lib:{root:"lib",edges:[],targets:{}},course:{root:"course",edges:["framework"],targets:{build:{}}},
 };
 const owners=Object.fromEntries(Object.entries(fragments).map(([name,row])=>[name,row.root])),fragment=async(name:string)=>sliceViewV1({edges:fragments[name]!.edges.map(target=>({target})),node:{targets:fragments[name]!.targets as never}});
 const closure=(target:string)=>sliceClosureV1({owners,fragment,selection:{kind:"tasks",tasks:[{project:"dash",target}]}});
 const outer=await closure("build"),nested=await closure("compile");
 expect([...outer].sort()).toEqual(["dash","framework","lib","process","providers","value"]);
 expect([...nested].every(name=>outer.has(name))).toBe(true);
 const names=Object.keys(owners).sort(),pin=readGenerationPinV1(encodeGenerationPinV1({owner:1,generation:7,manifest:"a".repeat(64),owners:names,covered:outer,atMs:Date.now()}));
 expect(generationPinOpenV1({pin,owners:names,needed:nested})).toEqual([]);
 expect((await fragment("dash")).node.targets.build).toEqual({inputs:[],dependsOn:undefined,metadata:{semio:{execution:{kind:"orchestration",children:[{project:"dash",target:"compile"}]}}}});
 fragments.dash!.targets.build={metadata:{semio:{execution:{version:1,kind:"orchestration",children:[{project:"ghost",target:"compile"}]}}}};
 await expect(closure("build")).rejects.toMatchObject({code:"GENERATION_UNKNOWN_OWNER"});
});

test("the slice scenario corpus agrees with the python oracle",()=>{
 if(!python)throw Error("Independent Python oracle required");
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});

test("a selected scope is the closure of its projects read from the generation: it builds the generation when none is usable, current serves it, fresh refreshes only the dirty closure, the admission names the selection",async()=>{
 const dir=scratch(),selection={kind:"tasks",tasks:[{project:"a",target:"build"}]},options={selection,slice:{selection,roots:["a"]}};
 try{
  await seed(dir,sliceTree());const mock=createMockHost(dir);
  const cold=(await loadGenerationGraphV1(mock.host,options)).graph as any;expect(mock.events).toContain("builder:held");expect(mock.log.length).toBe(1);expect(Object.keys(cold.projects).sort()).toEqual(["a","b","c"]);
  mock.events.length=0;mock.expanded.length=0;const graph=(await loadGenerationGraphV1(mock.host,{...options,policy:"current"})).graph as any;
  expect(mock.events).toEqual(["shared"]);expect(Object.keys(graph.projects).sort()).toEqual(["a","b","c"]);expect(graph.admission.selection).toEqual(selection);expect(graph.admission.registry).toBe(mock.host.registryDigest);expect(graph.admission.closure).toMatch(/^[a-f0-9]{64}$/u);expect(mock.expanded).toEqual([["a","b","c"]]);
  await applyOps(dir,[{op:"write",path:"pkg/f/src/index.ts",content:"export const other=1;\n"},{op:"write",path:"pkg/c/src/index.ts",content:"export const v=2;\n"}]);
  mock.events.length=0;mock.log.length=0;const served=(await loadGenerationGraphV1(mock.host,{...options,policy:"current"})).graph as any;
  expect(mock.events).toEqual(["shared"]);expect(served.projects.c.sources.length).toBe(1);
  const fresh=(await loadGenerationGraphV1(mock.host,options)).graph as any;
  expect(mock.log).toEqual([{full:["c"],edges:[],cold:false}]);expect(Object.keys(fresh.projects).sort()).toEqual(["a","b","c"]);
  const unknown={kind:"tasks",tasks:[{project:"zz",target:"build"}]};
  await expect(graphOfV1(mock.host,{selection:unknown,slice:{selection:unknown,roots:["zz"]}})).rejects.toMatchObject({code:"GENERATION_UNKNOWN_OWNER"});
 }finally{clean(dir);}
},60000);

test("a lean load names projects, targets and unavailable projects from the manifest alone and expands nothing",async()=>{
 const dir=scratch();
 try{
  const files=ringFiles(12);for(const index of[10,11])files[`pkg/p${index}/project.json`]=JSON.stringify({name:`p${index}`,broken:true});
  await seed(dir,files);const mock=createMockHost(dir);(await loadGenerationGraphV1(mock.host)).graph;
  mock.events.length=0;mock.expanded.length=0;const lean=(await loadGenerationGraphV1(mock.host,{policy:"current",lean:true})).graph as any;
  expect(mock.events).toEqual(["shared"]);expect(mock.expanded).toEqual([]);expect(lean.lean).toBe(true);expect(Object.keys(lean.projects).sort()).toEqual(Array.from({length:10},(_,i)=>"p"+i).sort());
  expect(lean.unavailable.map((row:any)=>row.project).sort()).toEqual(["p10","p11"]);
 }finally{clean(dir);}
},60000);

test("compact inputs are expanded within the capacity of one capture: text bytes, not UTF-16 units, bound every call and an oversized project is refused",async()=>{
 const reuse=await import("../../../../👁️observation/📥️reuse/🟨️.mjs"),policy=(await import("../../../../👁️observation/📥️reuse/🧬️schema/🔣️.json")).default.definitions.Policy.const;
 const limit=policy.maximumTextBytes,text=(bytes:number)=>"é".repeat(Math.floor(bytes/2));
 const wide=text(limit+2);expect(wide.length).toBeLessThan(limit);expect(Buffer.byteLength(wide)).toBeGreaterThan(limit);
 expect(()=>reuse.captureInferenceInventoryReferencesV1({p:wide})).toThrow("text capacity exceeded");
 const projects:Record<string,unknown>={};for(let index=0;index<5;index++)projects["p"+index]={sources:text(Math.floor(limit/3))};
 const calls:number[]=[],expand=async(chunk:Record<string,unknown>)=>{reuse.captureInferenceInventoryReferencesV1(chunk);calls.push(Object.keys(chunk).length);return chunk;};
 await expect(expand(projects)).rejects.toThrow("text capacity exceeded");calls.length=0;
 const bounded=await expandInChunksV1(projects,expand,{maximumProjects:32,maximumBytes:Math.floor(limit/4)});
 expect(Object.keys(bounded).sort()).toEqual(Object.keys(projects).sort());expect(calls.every(count=>count===1)).toBe(true);
 calls.length=0;const generous=await expandInChunksV1(projects,expand,{maximumProjects:32,maximumBytes:limit});expect(Object.keys(generous).length).toBe(5);expect(calls.reduce((sum,count)=>sum+count,0)).toBe(5);expect(Math.max(...calls)).toBeLessThan(5);
 await expect(expandInChunksV1({big:{sources:wide}},expand,{maximumProjects:32,maximumBytes:limit})).rejects.toThrow("text capacity exceeded");
},120000);

test("a removed owner is no stub of the next inference, so no edge refresh can ask for its published fragment",()=>{
 const stubs={a:{root:"pkg/a"},b:{root:"pkg/b"},c:{root:"pkg/c"}};
 expect(Object.keys(survivingStubsV1(stubs,new Set(["b"])))).toEqual(["a","c"]);
 expect(survivingStubsV1(stubs,[])).toEqual(stubs);
 expect(survivingStubsV1(stubs,["a","b","c","unknown"])).toEqual({});
 const python3=Bun.spawnSync([python!,"-I","-c","import json,sys;s=json.load(sys.stdin);print(json.dumps({k:v for k,v in s['stubs'].items() if k not in s['removed']}))"],{stdin:new TextEncoder().encode(JSON.stringify({stubs,removed:["b"]}))});
 expect(JSON.parse(python3.stdout.toString())).toEqual(survivingStubsV1(stubs,["b"]));
});

test("the stale projects of a pending refresh are prepared once, after the shared attempt and before the builder, for the owners the plan named",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const requests:{generation:number;dirty:readonly string[];roots:Readonly<Record<string,string>>;implementation:string}[]=[];
  const mock=createMockHost(dir,{prefetch:async request=>{requests.push(request);}});
  (await loadGenerationGraphV1(mock.host)).graph;
  expect(requests).toEqual([]);expect(mock.events).toEqual(["shared","builder:wait","builder:held","builder:released"]);
  mock.events.length=0;(await loadGenerationGraphV1(mock.host)).graph;expect(requests).toEqual([]);expect(mock.events).toEqual(["shared"]);
  await applyOps(dir,[{op:"write",path:"pkg/p2/src/index.ts",content:'import "@syn/p4";\nexport const v=2;\n'}]);
  mock.events.length=0;(await loadGenerationGraphV1(mock.host)).graph;
  expect(mock.events).toEqual(["shared","prefetch","builder:wait","builder:held","builder:released"]);
  expect(requests).toHaveLength(1);expect(requests[0]!.dirty).toEqual(["p2"]);expect(requests[0]!.generation).toBe(0);expect(Object.keys(requests[0]!.roots)).toEqual(["p2"]);expect(requests[0]!.roots.p2).toBe("pkg/p2");expect(requests[0]!.implementation).toMatch(/^[a-f0-9]{64}$/u);
 }finally{clean(dir);}
},60000);

test("a refresh restricted to a closure prepares only the stale owners of that closure",async()=>{
 const dir=scratch();
 try{
  const files:Record<string,string>={};
  for(const[name,imports]of[["a",["@syn/b"]],["b",["@syn/c"]],["c",[]],["d",["@syn/e"]],["e",[]]] as [string,string[]][])files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:[]}),files[`pkg/${name}/src/index.ts`]=imports.map(spec=>`import "${spec}";`).join("\n")+"\nexport const v=1;\n";
  await seed(dir,files);const requests:string[][]=[];const mock=createMockHost(dir,{prefetch:async request=>{requests.push([...request.dirty]);}});
  (await loadGenerationGraphV1(mock.host)).graph;
  await applyOps(dir,[{op:"write",path:"pkg/c/src/index.ts",content:"export const v=3;\n"},{op:"write",path:"pkg/e/src/index.ts",content:"export const v=4;\n"}]);
  (await loadGenerationGraphV1(mock.host,{closure:["a"]})).graph;
  expect(requests).toEqual([["c"]]);
 }finally{clean(dir);}
},60000);

test("a prefetch that settles late delays the builder, and one that rejects fails the load before any builder is entered",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const mock=createMockHost(dir);(await loadGenerationGraphV1(mock.host)).graph;
  await applyOps(dir,[{op:"write",path:"pkg/p2/src/index.ts",content:'import "@syn/p4";\nexport const v=5;\n'}]);
  const entered=Promise.withResolvers<void>(),release=Promise.withResolvers<void>();
  const slow=createMockHost(dir,{prefetch:()=>{entered.resolve();return release.promise;}}),loading=graphOfV1(slow.host);
  try{
   await Promise.race([entered.promise,loading.then(()=>{throw Error("Graph returned before prefetch entered");})]);
   expect(slow.events).toEqual(["shared","prefetch"]);
  }finally{release.resolve();await loading;}
  expect(slow.events).toEqual(["shared","prefetch","builder:wait","builder:held","builder:released"]);
  await applyOps(dir,[{op:"write",path:"pkg/p3/src/index.ts",content:'import "@syn/p5";\nexport const v=6;\n'}]);
  const failing=createMockHost(dir,{prefetch:async()=>{throw new Error("cancelled");}});
  await expect(graphOfV1(failing.host)).rejects.toThrow("cancelled");expect(failing.events).toEqual(["shared","prefetch"]);
 }finally{clean(dir);}
},60000);

for(const row of fixture.attempts)test("shared to builder answers: "+row.id,async()=>{
 const dir=scratch(),controller=new AbortController();let answer="equal",queries=0,active=false,inBuilder=false,failed=false;const proof:string[]=[];
 const support=():GenerationRowV1=>({kind:"file",path:"test-support",fact:Number(readFileSync(join(dir,"test-support"),"utf8"))});
 const head=():GenerationRowV1=>({kind:"derived",path:"test-question",fact:answer});
 const attach=(mock:ReturnType<typeof createMockHost>)=>{
  const storeOptions=mock.host.storeOptions,discover=mock.host.discoverPart;
  mock.host.storeOptions=((store:any)=>{
   const options=storeOptions(store);
   return{...options,holds:async(rows:readonly GenerationRowV1[],signal?:AbortSignal)=>{
    if(active&&rows.some(read=>read.path==="test-support"))proof.push((inBuilder?"builder":"shared")+":hold:"+rows.filter(read=>read.path==="test-support").map(read=>read.fact).join(","));
    if(active&&inBuilder&&row.transition==="cancel"&&rows.some(read=>read.path==="test-support"&&read.fact===2))controller.abort(new Error("support cancelled"));
    signal?.throwIfAborted();
    const raw=rows.filter(read=>read.path!=="test-support");
    return rows.every(read=>read.path!=="test-support"||read.fact===support().fact)&&await options.holds(raw,signal);
   },derive:async(_read:GenerationRowV1,signal?:AbortSignal)=>{
    signal?.throwIfAborted();queries++;proof.push((inBuilder?"builder":"shared")+":derive");
    if(row.transition==="failure"&&!failed){failed=true;throw Error("derive failed");}
    return{fact:answer,rows:[support()]};
   }};
  }) as typeof mock.host.storeOptions;
  mock.host.discoverPart=(async(spec:any,signal?:AbortSignal)=>{
   const part=await discover(spec,signal),owner=part.owners.find(owner=>owner.name==="p0");
   if(!owner)return part;
   owner.nodeRows.push(head());
   return{...part,derived:[{row:head(),support:[support()]}]};
  }) as typeof mock.host.discoverPart;
  return mock;
 };
 try{
  await seed(dir,{...ringFiles(2),"test-support":"1"});const initial=attach(createMockHost(dir));(await loadGenerationGraphV1(initial.host)).graph;
  const alternateRoot=join(dir,".other");
  if(row.transition==="root"){const alternate=attach(createMockHost(dir)),options=alternate.host.storeOptions;alternate.host.storeOptions=store=>options({root:alternateRoot});(await loadGenerationGraphV1(alternate.host)).graph;}
  await applyOps(dir,[{op:"write",path:"test-support",content:"2"}]);
  await applyOps(dir,[{op:"write",path:"pkg/p1/src/index.ts",content:"export const changed=2;\n"}]);
  const before=readFileSync(join(dir,".gen","current.json"),"utf8");
  if(row.priorLoad){const abandoned=attach(createMockHost(dir,{prefetch:async()=>{throw Error("abandoned load");}}));await expect(graphOfV1(abandoned.host)).rejects.toThrow("abandoned load");expect(queries).toBe(1);queries=0;proof.length=0;}
  const mock=attach(createMockHost(dir,{signal:controller.signal,gate:async()=>{
   inBuilder=true;
   if(row.transition==="support"||row.transition==="answer")await applyOps(dir,[{op:"write",path:"test-support",content:"3"}]);
   if(row.transition==="answer")answer="changed";
   if(row.transition==="generation"){
    const peer=attach(createMockHost(dir));(await loadGenerationGraphV1(peer.host)).graph;
   }
   if(row.transition==="membership")await applyOps(dir,[{op:"write",path:"pkg/new/project.json",content:JSON.stringify({name:"new",uses:[]})},{op:"write",path:"pkg/new/src/index.ts",content:"export const fresh=1;\n"}]);
  }}));
  if(row.transition==="root"){const options=mock.host.storeOptions;mock.host.storeOptions=store=>options(inBuilder?{root:alternateRoot}:store);}
  active=true;
  if(row.cancelled){
   await expect(graphOfV1(mock.host)).rejects.toThrow("support cancelled");
   expect(readFileSync(join(dir,".gen","current.json"),"utf8")).toBe(before);
  }else{
   const graph=(await loadGenerationGraphV1(mock.host)).graph;
   expect(shape(graph)).toEqual(pythonGraph(dir));
   const options=mock.host.storeOptions({root:join(dir,".gen")}),store=createGenerationStoreV1(options);
   const settled=await store.plan({current:await store.read(),identity:await mock.host.identity(),changes:[]});
   expect(settled.kind).toBe("clean");expect(settled.cutoff?.questions??0).toBe(0);
   expect(mock.log.flatMap(row=>row.full).includes("p0")).toBe(row.transition==="answer");
  }
  expect(queries).toBe(row.queries);
  if(row.transition==="none"){expect(proof.indexOf("shared:derive")).toBeGreaterThanOrEqual(0);expect(proof.indexOf("builder:hold:2")).toBeGreaterThan(proof.indexOf("shared:derive"));expect(proof).not.toContain("builder:derive");}
 }finally{clean(dir);}
},60000);

test("a build input closure that no longer exists is a miss: the generation is invalidated, rebuilt cold and the load still answers the graph, and a closure that stays missing fails",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const mock=createMockHost(dir);
  const cold=(await loadGenerationGraphV1(mock.host)).graph as any,expand=mock.host.expand.bind(mock.host);let missing=2;
  mock.host.expand=async(projects:Record<string,unknown>)=>{if(missing-->0)throw Object.assign(Error("ENOENT: no such file or directory, lstat 'build-input-closures/k.json'"),{code:"ENOENT",missingClosure:"k"});return expand(projects);};
  mock.events.length=0;const healed=await loadGenerationGraphV1(mock.host);
  expect(mock.events).toEqual(["shared","builder:wait","builder:held","builder:released","builder:wait","builder:held","builder:released"]);
  expect(healed.stats.ownersReinferred).toBeGreaterThan(0);expect(shape(healed.graph)).toEqual(shape(cold));expect(shape(healed.graph)).toEqual(pythonGraph(dir));
  mock.host.expand=async()=>{throw Object.assign(Error("ENOENT"),{code:"ENOENT",missingClosure:"k"});};
  await expect(loadGenerationGraphV1(mock.host)).rejects.toThrow("closure-missing");
  mock.host.expand=async()=>{throw Error("other failure");};
  await expect(loadGenerationGraphV1(mock.host)).rejects.toThrow("other failure");
 }finally{clean(dir);}
},120000);

const KEY_V1="d".repeat(64);
const typed=(projects:string[])=>Object.assign(Error("closure-missing:"+KEY_V1+" build-input-closures leaf is not on this machine"),{code:"closure-missing",missingClosure:KEY_V1,missingNamespace:"build-input-closures",missingProjects:projects});

for(const origin of ["live","archived","still-missing","refetched"] as const)test("closure recovery uses exact archived ownership from "+origin,async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(12));
  const mock=createMockHost(dir),discover=mock.host.discoverPart,expand=mock.host.expand,marker="env:SEMIO_SOURCE_CLOSURE_"+KEY_V1;
  using oracle=new Database(":memory:");
  const expected=(oracle.query("SELECT value FROM json_each(?) ORDER BY value").all(JSON.stringify(["p2","p9"])) as {value:string}[]).map(row=>row.value);
  let nameLeaf=true,available=true,unavailableProbes=0,refetches=0;
  mock.host.discoverPart=async(...args)=>{
   const part=await discover(...args);
   for(const owner of part.owners)if(owner.node&&expected.includes(owner.name)&&nameLeaf)owner.node={...owner.node,closureProof:marker};
   return part;
  };
  mock.host.expand=async projects=>{
   const copy=Object.fromEntries(Object.entries(projects).map(([name,project])=>{const{closureProof,...rest}=project as Record<string,unknown>;if(closureProof===marker&&!available)throw typed([name]);return[name,rest];}));
   return expand(copy);
  };
  const host={...mock.host,
   settleClosure:async()=>"re-derived",
   refetchClosure:async({key}:{key:string})=>{
    expect(key).toBe(KEY_V1);refetches++;
    if(origin!=="refetched")return false;
    available=true;return true;
   },
   verifyClosure:async({key}:{key:string})=>{
    if(key!==KEY_V1)throw Error("Unexpected test closure");
    if(!available)unavailableProbes++;
    return available;
   }
  };
  const complete=await loadGenerationGraphV1(host);const before=readWorldPointer(dir);
  if(origin==="archived"){
   const engine=createGenerationStoreV1(host.storeOptions({root:join(dir,".gen")}));
   await engine.invalidate({reason:"closure-missing",expected:(await engine.read())!.pointer});
  }
  available=false;nameLeaf=origin==="still-missing";mock.log.length=0;mock.events.length=0;
  if(origin==="refetched"){
   const repaired=await loadGenerationGraphV1(host);
   expect(mock.events).toEqual(["shared","shared"]);
   expect(mock.log).toEqual([]);
   expect(refetches).toBe(1);
   expect(readWorldPointer(dir)).toEqual(before);
   expect(shape(repaired.graph)).toEqual(pythonGraph(dir));
  }else if(origin==="still-missing"){
   await expect(loadGenerationGraphV1(host)).rejects.toMatchObject({code:"GENERATION_CLOSURE_REPAIR_UNQUALIFIED"});
   expect(readWorldPointer(dir).manifest).toBeNull();
   expect(mock.log.flatMap(row=>row.full).length).toBeGreaterThan(expected.length);
  }else{
   const repaired=await loadGenerationGraphV1(host);
   expect(mock.log.flatMap(row=>row.full).sort()).toEqual(expected);
   expect(repaired.stats.ownersReinferred).toBe(expected.length);
   expect(readWorldPointer(dir).generation).toBe(before.generation+2);
   expect(shape(repaired.graph)).toEqual(shape(complete.graph));
   expect(shape(repaired.graph)).toEqual(pythonGraph(dir));
  }
  if(origin!=="refetched")expect(unavailableProbes).toBeGreaterThan(0);
 }finally{clean(dir);}
},120000);

test("a leaf that is absent in the request is settled by the host: the owners that name it are asked of the fleet first and derived again once, the graph completes and no failure is an ENOENT",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const mock=createMockHost(dir);
  const cold=(await loadGenerationGraphV1(mock.host)).graph as any,expand=mock.host.expand.bind(mock.host),settled:unknown[]=[],asked:any[]=[];let missing=2;
  mock.host.expand=async(projects:Record<string,unknown>)=>{if(missing-->0)throw typed(["p1","p2","ghost"]);return expand(projects);};
  (mock.host as any).settleClosure=async(input:unknown)=>{settled.push(input);return"re-derived";};
  (mock.host as any).fleetRead=async(request:unknown)=>{asked.push(request);};
  mock.events.length=0;const healed=await loadGenerationGraphV1(mock.host);
  expect(settled).toEqual([{key:KEY_V1,namespace:"build-input-closures",projects:["p1","p2"]}]);
  expect(asked.map(request=>[request.dirty,Object.keys(request.roots)])).toEqual([[["p1","p2"],["p1","p2"]],[["p1","p2"],["p1","p2"]]]);
  expect(healed.stats.ownersReinferred).toBeGreaterThan(0);expect(shape(healed.graph)).toEqual(shape(cold));expect(shape(healed.graph)).toEqual(pythonGraph(dir));
 }finally{clean(dir);}
},120000);

test("a leaf the host refetched needs no invalidation and no re-derivation: the graph is served again from the same generation",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const mock=createMockHost(dir);
  const cold=(await loadGenerationGraphV1(mock.host)).graph as any,expand=mock.host.expand.bind(mock.host);let missing=1;
  mock.host.expand=async(projects:Record<string,unknown>)=>{if(missing-->0)throw typed(["p1"]);return expand(projects);};
  (mock.host as any).settleClosure=async()=>"refetched";
  const healed=await loadGenerationGraphV1(mock.host);
  expect(healed.stats.ownersReinferred).toBe(0);expect(shape(healed.graph)).toEqual(shape(cold));
 }finally{clean(dir);}
},120000);

test("a leaf that cannot be obtained or re-derived ends after the bounded rounds with the typed closure-missing failure, never an ENOENT",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const mock=createMockHost(dir);
  await loadGenerationGraphV1(mock.host);
  mock.host.expand=async()=>{throw typed(["p1"]);};(mock.host as any).settleClosure=async()=>"re-derived";
  const failure=await loadGenerationGraphV1(mock.host).catch(error=>error);
  expect(failure.code).toBe("closure-missing");expect(failure.message).not.toContain("ENOENT");expect(failure.missingClosure).toBe(KEY_V1);
  mock.host.expand=async()=>{throw Object.assign(Error("ENOENT: lstat 'build-input-closures/"+KEY_V1+".json'"),{code:"ENOENT"});};
  const raw=await loadGenerationGraphV1(mock.host).catch(error=>error);
  expect(raw.code).toBe("closure-missing");expect(raw.message).not.toContain("ENOENT:");
 }finally{clean(dir);}
},120000);

test("a restored generation that names a leaf nobody holds is completed in the same request: the host settles each such leaf once and the generation is invalidated so exactly the records that name it are derived again",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const mock=createMockHost(dir),settled:unknown[]=[];
  await loadGenerationGraphV1(mock.host);const restored=readWorldPointer(dir);let ahead=true;
  (mock.host as any).exchange={upload:false,restore:async()=>({status:"restored",generation:restored.generation,localManifest:restored.manifest,owners:6,documents:3,bytes:10,ageSeconds:5,stale:1,closures:{named:3,sent:0,fetched:2,missing:[{key:KEY_V1,namespace:"build-input-closures",projects:["p1","p4"]}]}}),ahead:async()=>{const pending=ahead;ahead=false;return pending;},publish:async()=>({status:"current"}),leaf:async()=>undefined};
  (mock.host as any).settleClosure=async(input:unknown)=>{settled.push(input);return"re-derived";};
  const result=await loadGenerationGraphV1(mock.host);
  expect(settled).toEqual([{key:KEY_V1,namespace:"build-input-closures",projects:["p1","p4"]}]);
  expect(shape(result.graph)).toEqual(pythonGraph(dir));
 }finally{clean(dir);}
},120000);

test("a missing closure leaf is recognised by its tag, by its message after a boundary dropped the properties, and through causes; other failures are not",async()=>{
 const{missingClosureOfV1}=await import("../🟨️.mjs"),key="a".repeat(64);
 expect(missingClosureOfV1(Object.assign(Error("x"),{missingClosure:key}))).toBe(key);
 expect(missingClosureOfV1(Error("ENOENT: no such file or directory, lstat '/w/.cache/b/g/m/build-input-closures/"+key+".json'"))).toBe(key);
 expect(missingClosureOfV1(Error("ENOENT: closure-missing:"+key+" build-lock-closures leaf is not on this machine"))).toBe(key);
 expect(missingClosureOfV1(Object.assign(Error("wrapped"),{cause:Error("ENOENT: lstat 'build-lock-closures/"+key+".json'")}))).toBe(key);
 expect(missingClosureOfV1(Error("EACCES: lstat 'build-input-closures/"+key+".json'"))).toBeUndefined();
 expect(missingClosureOfV1(Error("Error"))).toBeUndefined();
});

test("a failure that lost its properties on the way (message only) still heals through the repair rounds",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const mock=createMockHost(dir);
  const cold=(await loadGenerationGraphV1(mock.host)).graph as any,expand=mock.host.expand.bind(mock.host);let stale=true;const repaired:string[]=[];
  mock.host.expand=async(projects:Record<string,unknown>)=>{if(stale)throw Error("ENOENT: no such file or directory, lstat '/w/.cache/b/g/m/build-input-closures/"+"d".repeat(64)+".json'");return expand(projects);};
  (mock.host as any).repairClosure=async(key:string)=>{repaired.push(key);stale=false;return 1;};
  const healed=await loadGenerationGraphV1(mock.host);
  expect(repaired).toEqual(["d".repeat(64)]);expect(shape(healed.graph)).toEqual(shape(cold));
 }finally{clean(dir);}
},120000);

test("a closure leaf that stays missing after the cold rebuild is repaired at its source: the records that still name it are removed through the host, then the load answers; without a repair it fails after a bounded number of rebuilds",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const mock=createMockHost(dir);
  const cold=(await loadGenerationGraphV1(mock.host)).graph as any,expand=mock.host.expand.bind(mock.host);let stale=true;const repaired:string[]=[];
  mock.host.expand=async(projects:Record<string,unknown>)=>{if(stale)throw Object.assign(Error("ENOENT: no such file or directory, lstat 'build-input-closures/"+"b".repeat(64)+".json'"),{code:"ENOENT",missingClosure:"b".repeat(64)});return expand(projects);};
  (mock.host as any).repairClosure=async(key:string)=>{repaired.push(key);stale=false;return 1;};
  mock.events.length=0;const healed=await loadGenerationGraphV1(mock.host);
  expect(repaired).toEqual(["b".repeat(64)]);expect(shape(healed.graph)).toEqual(shape(cold));
  stale=true;let calls=0;(mock.host as any).repairClosure=async()=>{calls++;return 0;};
  await expect(loadGenerationGraphV1(mock.host)).rejects.toThrow("closure-missing");
  expect(calls).toBe(2);
 }finally{clean(dir);}
},120000);

test("a load returns its graph with the stats of what it verified and reinferred: a cold build computes owners, a warm load verifies facts and computes none",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const mock=createMockHost(dir);
  const cold=await loadGenerationGraphV1(mock.host);
  expect(Object.keys(cold.stats).sort()).toEqual(["closuresVerified","doubtfulFacts","ownersCutOff","ownersReinferred","staleFacts","verifiedMilliseconds"]);
  expect(cold.stats.ownersReinferred).toBeGreaterThan(0);
  const warm=await loadGenerationGraphV1(mock.host);
  expect(warm.stats.ownersReinferred).toBe(0);expect(warm.stats.staleFacts).toBe(0);expect(warm.stats.verifiedMilliseconds).toBeGreaterThanOrEqual(0);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("the progress of concurrent workers in one phase goes through the real phase clock in order: the phase is opened once, advanced to its total and closed, however the events interleave",async()=>{
 const{createGraphPhaseClockV1}=await import("../../../⏱️phases/🟨️.mjs"),{createClosureSettlerV1}=await import("../../../🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs"),{createGenerationProgressV1}=await import("../🟨️.mjs");
 const rows:any[]=[],clock=createGraphPhaseClockV1({publish:async()=>{await new Promise(resolve=>setTimeout(resolve,1));},write:row=>rows.push(row),minimumMilliseconds:0}),progress=createGenerationProgressV1(clock);
 const names=Array.from({length:60},(_,at)=>"q"+at),pseudoRows=new Map(names.map(name=>[name,{kind:"derived",path:JSON.stringify({walk:1,name}),fact:"old"}]));
 const settled=await createClosureSettlerV1({}).settle({pseudoRows:pseudoRows as never,derive:async()=>({fact:"old",rows:[{kind:"file",path:"/x",fact:"h"}]}),hold:async()=>true,progress});
 expect(settled.failed).toBe(0);
 const verification=rows.filter(row=>row.phase==="verification");expect(verification.length).toBe(1);expect(verification[0]).toMatchObject({done:60,total:60});
 await Promise.all([progress({phase:"files",done:0,total:4}),progress({phase:"files",done:1,total:4}),progress({phase:"files",done:4,total:4}),progress({phase:"files",done:0,total:2}),progress({phase:"files",done:2,total:2})]);
 expect(rows.filter(row=>row.phase==="files").map(row=>[row.done,row.total])).toEqual([[4,4],[2,2]]);
});

test("running owners and the admission decision pass through the generation progress to the clock and the sinks without restarting the phase",async()=>{
 const{createGenerationProgressV1,onGenerationProgressV1}=await import("../🟨️.mjs"),{createGraphPhaseClockV1}=await import("../../../⏱️phases/🟨️.mjs");
 const published:string[]=[],events:any[]=[],clock=createGraphPhaseClockV1({publish:resource=>{published.push(resource);},minimumMilliseconds:0}),stop=onGenerationProgressV1(event=>events.push(event)),progress=createGenerationProgressV1(clock);
 await progress({phase:"owners",done:0,total:4});
 await new Promise(resolve=>setTimeout(resolve,1100));
 await progress({phase:"owners",done:0,total:4,running:[{name:"@semio-tech/framework-rs",seconds:12,kind:"language:cargo",rows:34}],decision:{key:"learning",used:256,budget:1386,kind:"language:cargo"}});
 await progress({phase:"owners",done:1,total:4});
 await progress({phase:"owners",done:4,total:4});
 await progress({phase:"owners",done:4,total:4,running:[{name:"late",seconds:1,kind:"project",rows:1}]});
 stop();
 expect(published.filter(resource=>resource==="build:graph-phase:owners:0:4").length,"the phase begins once").toBe(1);
 expect(published.some(resource=>resource.includes("%40semio-tech%2Fframework-rs|language%3Acargo|10|34:learning|256|1386|language%3Acargo"))).toBe(true);
 expect(published.some(resource=>resource.includes("late"))).toBe(false);
 const withRunning=events.find(event=>event.running!==undefined);
 expect(withRunning).toEqual({phase:"owners",done:0,total:4,running:[{name:"@semio-tech/framework-rs",seconds:12,kind:"language:cargo",rows:34}],decision:{key:"learning",used:256,budget:1386,kind:"language:cargo"}});
});

test("a phase report that falls behind an interleaved one is dropped and never ends the load",async()=>{
 const{createGenerationProgressV1}=await import("../🟨️.mjs"),{createGraphPhaseClockV1}=await import("../../../⏱️phases/🟨️.mjs");
 const published:string[]=[],progress=createGenerationProgressV1(createGraphPhaseClockV1({publish:resource=>{published.push(resource);},minimumMilliseconds:0}) as never);
 await progress({phase:"decisions",done:5,total:10});
 await progress({phase:"decisions",done:3,total:12});
 await progress({phase:"decisions",done:12,total:12});
 await progress({phase:"decisions",done:2,total:4});
 await progress({phase:"decisions",done:4,total:4});
 expect(published.some(resource=>resource.includes("decisions"))).toBe(true);
});

test("no ordering, shape or failure of progress reports can reject a report and end the load",async()=>{
 const{createGenerationProgressV1}=await import("../🟨️.mjs"),{createGraphPhaseClockV1}=await import("../../../⏱️phases/🟨️.mjs");
 const names=["decisions","owners","take","Bad Phase",""],values=[0,1,2,3,5,12,-1,2.5,Number.NaN,Number.POSITIVE_INFINITY,Number.MAX_SAFE_INTEGER+2];
 let seed=20261116;const next=()=>{seed=(seed*1664525+1013904223)%4294967296;return seed/4294967296;};
 const pick=<T,>(list:T[])=>list[Math.floor(next()*list.length)];
 for(let round=0;round<300;round++){
  const failing=round%7===0,progress=createGenerationProgressV1(createGraphPhaseClockV1({publish:()=>{if(failing&&next()<0.3)throw Error("publish failed");},minimumMilliseconds:0}) as never);
  for(let step=0;step<40;step++)await progress({phase:pick(names),done:pick(values),total:pick(values)} as never);
 }
 const progress=createGenerationProgressV1(createGraphPhaseClockV1({minimumMilliseconds:0}) as never);
 await progress({phase:"decisions",done:Number.NaN,total:4} as never);
 await progress({phase:"decisions",done:-1,total:4});
 await progress({phase:"decisions",done:1,total:4});
 await progress({phase:"decisions",done:4,total:4});
});

const readWorldPointer=(dir:string)=>JSON.parse(readFileSync(join(dir,".gen",GENERATION_POLICY_V1.pointer),"utf8")) as {generation:number;manifest:string|null};

for(const fixture of closureCustody)test("closure repair keeps the healthy head published during "+fixture.origin+" "+fixture.wait,async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const mock=createMockHost(dir),peer=createMockHost(dir);
  await loadGenerationGraphV1(peer.host);
  const before=readWorldPointer(dir);
  let healthy:ReturnType<typeof readWorldPointer>|undefined,settled=0,published=0;
  const advance=async()=>{
   published++;
   await applyOps(dir,[{op:"write",path:"pkg/p2/src/index.ts",content:`import "@syn/p4";\nexport const peer = ${published};\n`}]);
   await loadGenerationGraphV1(peer.host);
   healthy=readWorldPointer(dir);
  };
  if(fixture.origin==="local"){
   const expand=mock.host.expand;let remaining=2;
   mock.host.expand=async projects=>{if(remaining-->0)throw typed(["p1"]);return expand(projects);};
  }else{
   let firstShared=true,firstRestore=true;
   (mock.host as any).exchange={upload:false,ahead:async()=>firstShared&&(firstShared=false,true),restore:async()=>{
    if(!firstRestore)return{status:"current"};
    firstRestore=false;
    await advance();const restored=healthy!;
    if(fixture.wait==="restore")await advance();
    return{status:"restored",generation:restored.generation,localManifest:restored.manifest,closures:{named:1,sent:0,fetched:0,missing:[{key:KEY_V1,namespace:"build-input-closures",projects:["p1"]}]}};
   }};
  }
  (mock.host as any).settleClosure=async()=>{
   settled++;
   if(fixture.wait==="settle")await advance();
   return fixture.wait==="refetch"?"refetched":"re-derived";
  };
  const result=await loadGenerationGraphV1(mock.host);
  expect(healthy?.generation).toBeGreaterThan(before.generation);
  expect(settled).toBe(fixture.settled);
  expect(readWorldPointer(dir)).toEqual(healthy!);
  expect(result.stats.ownersReinferred).toBe(0);
  expect(shape(result.graph)).toEqual(pythonGraph(dir));
 }finally{clean(dir);}
},120000);


test("a local missing closure reacquires live structural custody before it can invalidate the observed head",async()=>{
 using model=new Database(":memory:");
 const expected=model.query(`SELECT CASE WHEN $mode='cancel at swap' THEN 'ABORT_ERR' ELSE 'served' END AS status,
  CASE WHEN $mode='peer wins' THEN 0 ELSE 1 END AS settled,
  CASE WHEN $mode='cancel at swap' THEN 1 ELSE 0 END AS kept`);
 for(const mode of ["repair","peer wins","cancel at swap"]){
  const dir=scratch(),root=join(dir,".gen"),controller=new AbortController(),mock=createMockHost(dir,{signal:controller.signal}),peer=createMockHost(dir);
  const files=createNodeFilePortV1(),custody=createSliceCustodyV1({root,fs:files,lease:createFileClaimLeasePortV1(join(dir,"claims"))});
  let entries=0,settled=0,failures=2,armed=false;
  const tokens=new Set<string>(),expand=mock.host.expand,options=mock.host.storeOptions;
  const withClaim=async(body:Function,request?:{dirty:string[];structural:boolean})=>{
   const claim=await custody.hold({owners:request?.dirty,structural:request?.structural??true,signal:controller.signal});
   try{return await body({root},{published:mock.published},claim);}finally{await claim.release();}
  };
  try{
   await seed(dir,ringFiles(6));await loadGenerationGraphV1(mock.host);
   const observed=readWorldPointer(dir);
   let healthy:ReturnType<typeof readWorldPointer>|undefined;
   mock.host.expand=async projects=>{if(failures-->0)throw typed(["p1"]);return expand(projects);};
   Object.assign(mock.host,{
    async withBuilder(body:Function,request?:{dirty:string[];structural:boolean}){
     expect(await custody.list()).toEqual([]);
     entries++;
     if(entries===2&&mode==="peer wins"){
      await applyOps(dir,[{op:"write",path:"pkg/p2/src/index.ts",content:"export const peer = 1;"}]);
      await loadGenerationGraphV1({...peer.host,withBuilder:withClaim} as never);
      healthy=readWorldPointer(dir);
     }
     return withClaim(async (store:unknown,control:unknown,claim:{token:string})=>{tokens.add(claim.token);return body(store,control,claim);},request);
    },
    async settleClosure(){
     settled++;
     expect((await custody.list()).filter(row=>row.state==="held").map(row=>row.structural)).toEqual([true]);
     armed=mode==="cancel at swap";
     return"re-derived";
    },
    storeOptions:(store:{root:string})=>({...options(store),fs:{...files,async read(path:string){
     const data=await files.read(path);
     if(armed&&path===join(root,GENERATION_POLICY_V1.pointer)){
      armed=false;controller.abort(Object.assign(Error("cancel at pointer exchange"),{code:"ABORT_ERR"}));
     }
     return data;
    }}}),
   });
   const status=await loadGenerationGraphV1(mock.host).then(()=>"served",error=>error.code);
   const after=readWorldPointer(dir);
   expect({status,settled,kept:Number(after.manifest===observed.manifest)},mode).toEqual(expected.get({$mode:mode}));
   expect(entries,mode).toBe(2);
   expect(tokens.size,mode).toBe(2);
   if(healthy)expect(after,mode).toEqual(healthy);
   expect(await custody.list()).toEqual([]);
  }finally{clean(dir);}
 }
},120000);

test("cold fragment payloads leave memory before the source rows are consumed",async()=>{
 const dir=scratch();
 try{
  const count=2*GENERATION_POLICY_V1.readConcurrency+16;await seed(dir,ringFiles(count));
  const mock=createMockHost(dir),discover=mock.host.discoverPart,options=mock.host.storeOptions,clock=mock.host.clock,events:string[]=[];
  const payloads:WeakRef<object>[]=[];
  const released=async(minimum:number)=>{
   let collected=0;
   for(let turn=0;turn<8&&collected<minimum;turn++){
    await sleep(5);Bun.gc(true);
    collected=payloads.filter(payload=>payload.deref()===undefined).length;
   }
   return collected;
  };
  let releasedMidway=0,releasedBeforeRows=0;
  mock.host.discoverPart=async(...args)=>{
   const part=await discover(...args);
   for(const {node} of part.owners)if(node!==undefined)payloads.push(new WeakRef(node));
   events.push("discovered");
   return{...part,dispose:()=>{events.push("disposed");}};
  };
  mock.host.clock=control=>{
   const phases=clock(control);
   return{...phases,async advance(phase:string,done:number){
    if(phase==="fragments"&&done===count/2){
     expect(events).toEqual(["discovered"]);
     releasedMidway=await released(count/2);
    }
    await phases.advance(phase,done);
   }};
  };
  mock.host.storeOptions=store=>({...options(store),stepped:async(step:string)=>{
   events.push(step);
   if(step!=="storing")return;
   expect(events).not.toContain("disposed");
   releasedBeforeRows=await released(count);
  }});
  const built=await loadGenerationGraphV1(mock.host);
  expect(events).toEqual(["discovered","storing","disposed","indexing","sealing"]);
  expect(payloads.length).toBe(count);
  expect(built.stats.ownersReinferred).toBe(count);
  expect(shape(built.graph)).toEqual(pythonGraph(dir));
  expect(releasedMidway).toBeGreaterThanOrEqual(count/2);
  expect(releasedBeforeRows).toBe(count);
 }finally{clean(dir);}
},120000);

for(const mode of ["cancel","write error"])test("cold fragment "+mode+" leaves the exact graph available for a fresh retry",async()=>{
 const dir=scratch();
 try{
  const count=48;await seed(dir,ringFiles(count));
  const controller=new AbortController(),mock=createMockHost(dir,{signal:controller.signal}),discover=mock.host.discoverPart,clock=mock.host.clock,options=mock.host.storeOptions,fs=createNodeFilePortV1();
  const store=createGenerationStoreV1(options({root:join(dir,".gen")}));
  await loadGenerationGraphV1(createMockHost(dir).host);
  const baseline=(await store.read())!;
  expect(await store.invalidate({reason:"closure-missing",expected:baseline.pointer})).toBe(true);
  const quarantined=readWorldPointer(dir);
  const stopped=Error(mode);let writes=0,disposed=0;
  mock.host.discoverPart=async(...args)=>{
   const part=await discover(...args);
   return{...part,dispose:()=>{disposed++;part.dispose?.();}};
  };
  mock.host.clock=control=>{
   const phases=clock(control);
   return{...phases,async advance(phase:string,done:number){
    if(mode==="cancel"&&phase==="fragments"&&done===count/2)controller.abort(stopped);
    await phases.advance(phase,done);
   }};
  };
  mock.host.storeOptions=store=>({...options(store),fs:{...fs,async write(path,...args){
   if(dirname(path)===join(store.root,GENERATION_POLICY_V1.fragments)&&++writes===count&&mode==="write error")throw stopped;
   await fs.write(path,...args);
  }}});
  await expect(loadGenerationGraphV1(mock.host)).rejects.toThrow(mode);
  expect(disposed).toBe(1);
  expect(readWorldPointer(dir)).toEqual(quarantined);
  expect(quarantined.manifest).toBeNull();
  const retried=await loadGenerationGraphV1(createMockHost(dir).host),sealed=(await store.read())!;
  expect(retried.stats.ownersReinferred).toBe(count);
  expect(shape(retried.graph)).toEqual(pythonGraph(dir));
  expect(sealed.pointer.generation).toBe(quarantined.generation+1);
  expect(sealed.manifest?.owners).toEqual(baseline.manifest?.owners);
  expect(sealed.manifest?.global).toBe(baseline.manifest?.global);
  expect(sealed.manifest?.factIndex).toBe(baseline.manifest?.factIndex);
 }finally{clean(dir);}
},120000);
