/** 🕰️ Contract: many agents edit different projects of one folder while the graph is read, verified and published without any of them waiting for a whole-workspace rebuild. An edit in one project refreshes exactly its owners, never yields a stale graph for that project, a reader never sees a torn generation, concurrent writers coalesce under one builder, the exclusive hold of the facts resource is bounded to the pointer swap, and a graph policy decides who may wait. Every expectation is produced by the independent Python model in `🐍️.py`. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {createHash} from "node:crypto";
import {chmodSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {lstat,readFile,readdir,writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import Ajv2020 from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {GENERATION_POLICY_V1,canonicalJsonV1,createFactIndexV1,createFileLeasePortV1,createFrontierV1,createGenerationStoreV1,diffFrontiersV1,factConsumersV1,frontierHoldsV1,patchFactIndexV1,readGenerationFragmentV1,readGenerationManifestV1,readGenerationPointerV1,refineFactIndexV1,rowKeyV1,selectFactClassesV1,staleFactRowsV1,type GenerationRowV1} from "../🟨️.mjs";
import {IDENTITY,IMPLEMENTATION,applyOps,createMemoryFilePort,createWorld,ringFiles,seed,type Op} from "./🏃️world/🟦️.ts";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️.py"),windows=process.platform==="win32";
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-generations-"));
const clean=(dir:string)=>rmSync(dir,{recursive:true,force:true});
const sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
const pythonGraph=(dir:string)=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,"graph",dir]).stdout.toString());
const normalized=(value:unknown)=>JSON.parse(canonicalJsonV1(value));
const gated=()=>{let open!:()=>void;const promise=new Promise<void>(resolve=>{open=resolve;});return{promise,open};};

test("the fixture is closed under its schema and the independent model reproduces every expectation",()=>{
 expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);
 if(python){const result=Bun.spawnSync([python,"-I",oracle,"check",join(here,"🔣️.json")]);expect(result.stdout.toString()).toContain("differences: 0");expect(result.exitCode).toBe(0);}
});

test("the declared policy and the bilingual texts are the ones the implementation uses",()=>{
 expect(schema.definitions.Policy.const).toEqual(JSON.parse(JSON.stringify(GENERATION_POLICY_V1)));
 const placeholders=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
 expect(Object.keys(schema.definitions.Messages.const.en).sort()).toEqual(Object.keys(schema.definitions.Messages.const.de).sort());
 for(const key of Object.keys(schema.definitions.Messages.const.en))expect(placeholders(schema.definitions.Messages.const.de[key as keyof typeof schema.definitions.Messages.const.de])).toEqual(placeholders(schema.definitions.Messages.const.en[key as keyof typeof schema.definitions.Messages.const.en]));
});

for(const scenario of fixture.scenarios)test("scenario "+scenario.name+": every edit refreshes exactly the owners the model names and the graph equals the model graph",async()=>{
 const dir=scratch();
 try{
  await seed(dir,scenario.initial as unknown as Record<string,string>);const world=createWorld(dir);let last;
  for(const step of scenario.steps){
   await applyOps(dir,step.ops as Op[]);
   const before=world.log.length,result=await world.refresh();last=result;
   const where=scenario.name+" / "+step.label;
   expect(result.current.manifest!.generation,where).toBe(step.expect.generation);
   if(step.expect.built){
    expect(result.built&&result.published,where).toBe(true);expect(world.log.length-before,where).toBe(1);
    expect(world.log.at(-1)!.full,where).toEqual(step.expect.full);expect(world.log.at(-1)!.edges,where).toEqual(step.expect.edges);
   }else{expect(result.built,where).toBe(false);expect(world.log.length,where).toBe(before);}
   expect(await world.graph(result.current),where).toEqual(step.expect.graph);
  }
  if(python)expect(normalized(await world.graph(last!.current))).toEqual(pythonGraph(dir));
  for(const closure of scenario.closures){
   const reads=world.store.stats.fragmentReads,snapshot=await world.store.snapshot({roots:closure.roots});
   expect([...await world.store.closureOf(snapshot.current,closure.roots)].sort()).toEqual(closure.names);
   const{generation,membership,...rest}=snapshot.graph;expect(normalized(rest)).toEqual(closure.graph);
   expect(world.store.stats.fragmentReads-reads).toBeLessThanOrEqual(3*closure.names.length);
  }
 }finally{clean(dir);}
},60000);

test("an edit outside a closure neither blocks nor is refreshed for it; an edit inside is refreshed before the closure is served",async()=>{
 const dir=scratch();
 try{
  const files:Record<string,string>={};
  for(const[name,imports]of[["a",["@syn/b"]],["b",["@syn/c"]],["c",[]],["d",["@syn/e"]],["e",[]],["f",[]]] as [string,string[]][])files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:[]}),files[`pkg/${name}/src/index.ts`]=imports.map(spec=>`import "${spec}";`).join("\n")+"\nexport const v=1;\n";
  await seed(dir,files);
  let rows:string[]=[];const world=createWorld(dir,{holdsWrap:holds=>async(list,signal)=>{rows.push(...list.map(row=>row.path));return holds(list,signal);}});
  await world.refresh();
  await applyOps(dir,[{op:"write",path:"pkg/e/src/index.ts",content:"export const v=2;\n"}]);
  rows=[];const clean1=await world.refreshClosure(["a"]);
  expect(clean1.served).toBe("verified");expect(clean1.built).toBe(false);expect([...clean1.names].sort()).toEqual(["a","b","c"]);
  expect(rows.some(path=>/^pkg\/[def]\/src/u.test(path))).toBe(false);expect(rows.some(path=>/^pkg\/[abc]\/src/u.test(path))).toBe(true);expect(world.log.length).toBe(1);
  await applyOps(dir,[{op:"write",path:"pkg/b/src/index.ts",content:'import "@syn/c";\nimport "@syn/f";\nexport const v=3;\n'}]);
  const dirty=await world.refreshClosure(["a"]);
  expect(dirty.built).toBe(true);expect(world.log.at(-1)!.full).toEqual(["b"]);expect([...dirty.names].sort()).toEqual(["a","b","c","f"]);
  const graph:any=await world.graph(dirty.current,[...dirty.names]);expect(graph.dependencies.map((edge:any)=>edge.source+">"+edge.target).sort()).toEqual(["a>b","b>c","b>f"]);
 }finally{clean(dir);}
},30000);

test("a missing fragment is rebuilt, a missing index or manifest starts over, and a tampered document is refused",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(5));const world=createWorld(dir),first=await world.refresh(),store=join(dir,".gen");
  const victim=Object.entries(first.current.manifest!.owners)[2]!;
  rmSync(join(store,"o",victim[1].fragment+".json.br"));
  const healed=await world.refresh();expect(healed.built).toBe(true);expect(world.log.at(-1)!.full).toEqual([victim[0]]);
  expect(await world.graph(healed.current)).toEqual(normalized(pythonGraph(dir)));
  rmSync(join(store,"x",healed.current.manifest!.factIndex+".facts.br"));
  const again=await createWorld(dir).refresh();expect(again.current.manifest!.built.kind).toBe("cold");
  rmSync(join(store,"m",again.current.pointer.manifest+".json"));
  const third=createWorld(dir),rebuilt=await third.refresh();expect(rebuilt.built).toBe(true);expect(rebuilt.current.manifest!.built.kind).toBe("cold");
  const bytes=Buffer.from(JSON.stringify({...rebuilt.current.pointer,extra:1}));
  expect(()=>readGenerationPointerV1(bytes)).toThrow();
  writeFileSync(join(store,"m",rebuilt.current.pointer.manifest+".json"),"{}");
  await expect(createWorld(dir).store.read()).rejects.toThrow(/differs from its name/u);
 }finally{clean(dir);}
},60000);

test("a torn read is impossible: readers of every kind see exactly the graph of one published generation while a writer publishes",async()=>{
 const dir=scratch(),reference=scratch();
 try{
  const steps:Op[][]=Array.from({length:14},(_,i)=>[{op:"write",path:`pkg/p${i%6}/src/index.ts`,content:`import "@syn/p${(i+3)%6}";\nexport const edit=${i};\n`}]);
  await seed(reference,ringFiles(6));const ref=createWorld(reference),expected=new Map<number,unknown>(),first=await ref.refresh();expected.set(0,await ref.graph(first.current));
  for(const ops of steps){await applyOps(reference,ops);const result=await ref.refresh();expected.set(result.current.manifest!.generation,await ref.graph(result.current));}
  await seed(dir,ringFiles(6));const writer=createWorld(dir);await writer.refresh();
  let done=false;const seen:number[][]=[],failures:string[]=[];
  const reader=async(index:number)=>{
   const world=createWorld(dir,{lease:false}),mine:number[]=[];
   while(!done){
    const{current,graph}=await world.store.snapshot();
    const{generation,membership,...rest}=graph;mine.push(generation);
    const want=expected.get(generation);if(want===undefined||canonicalJsonV1(rest)!==canonicalJsonV1(want))failures.push(`reader ${index} generation ${generation}`);
    if(current.manifest!.generation!==generation)failures.push("snapshot generation differs from its pointer");
    await sleep(0);
   }
   seen.push(mine);
  };
  const readers=Array.from({length:24},(_,i)=>reader(i));
  for(const ops of steps){await applyOps(dir,ops);await writer.refresh();await sleep(2);}
  done=true;await Promise.all(readers);
  expect(failures).toEqual([]);
  for(const mine of seen){for(let i=1;i<mine.length;i++)expect(mine[i]!).toBeGreaterThanOrEqual(mine[i-1]!);expect(mine.length).toBeGreaterThan(0);}
  expect(Math.max(...seen.flat())).toBe(14);
 }finally{clean(dir);clean(reference);}
},120000);

test("a reader survives the retirement of old documents by restarting from the newest pointer",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const writer=createWorld(dir,{now:()=>Date.now()+10*GENERATION_POLICY_V1.graceMilliseconds});await writer.refresh();
  let done=false;const failures:string[]=[],reader=async()=>{const world=createWorld(dir,{lease:false});while(!done){try{const{graph}=await world.store.snapshot();if(Object.keys(graph.projects).length!==6)failures.push("projects");}catch(error){failures.push(String((error as Error).message));}await sleep(0);}};
  const readers=Array.from({length:8},reader);
  for(let i=0;i<30;i++){await applyOps(dir,[{op:"write",path:`pkg/p${i%6}/src/index.ts`,content:`import "@syn/p${(i+1)%6}";\nexport const e=${i};\n`}]);await writer.refresh();}
  done=true;await Promise.all(readers);expect(failures).toEqual([]);
  expect(readdirSync(join(dir,".gen","m")).length).toBeLessThanOrEqual(GENERATION_POLICY_V1.retainGenerations+1);
 }finally{clean(dir);}
},120000);

test("concurrent writers coalesce under one builder, never lose an edit and never overlap their builds",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(8));const sharedHolds:ReturnType<typeof createWorld>["holdsLog"]=[],sharedLog:ReturnType<typeof createWorld>["log"]=[];
  await createWorld(dir).refresh();
  const agents=Array.from({length:8},()=>createWorld(dir,{sharedHoldsLog:sharedHolds,sharedLog,computeWrap:compute=>async(spec,signal)=>{await sleep(40);return compute(spec,signal);}}));
  const results=await Promise.all(agents.map(async(agent,index)=>{
   const content=`import "@syn/p${(index+2)%8}";\nexport const agent${index}=${index};\n`;
   await applyOps(dir,[{op:"write",path:`pkg/p${index}/src/index.ts`,content}]);
   const result=await agent.refresh(),graph:any=await agent.graph(result.current);
   return{index,content,graph,generation:result.current.manifest!.generation};
  }));
  for(const row of results){const node=results.length&&row.graph.projects[`p${row.index}`];expect(node.sources[0].sha256).toBe(sha(row.content));}
  expect(sharedLog.length).toBeLessThanOrEqual(8);expect(sharedLog.length).toBeGreaterThan(0);
  const builds=sharedHolds.filter(row=>row.resource.startsWith("repository-graph-build")).map(row=>({start:row.endedAt-row.heldMs,end:row.endedAt})).sort((a,b)=>a.start-b.start);
  for(let i=1;i<builds.length;i++)expect(builds[i]!.start).toBeGreaterThanOrEqual(builds[i-1]!.end-2);
  const final=await createWorld(dir).refresh();expect(final.built).toBe(false);expect(final.served).toBe("verified");
  if(python)expect(normalized(await createWorld(dir).graph(final.current))).toEqual(pythonGraph(dir));
 }finally{clean(dir);}
},120000);

test("two builders that race for the same base publish exactly one generation by compare-and-swap and the loser adopts it",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));await createWorld(dir).refresh();
  await applyOps(dir,[{op:"write",path:"pkg/p1/src/index.ts",content:"export const a=1;\n"},{op:"write",path:"pkg/p4/src/index.ts",content:"export const b=2;\n"}]);
  const barrier=gated();let arrived=0;
  const racing=()=>createWorld(dir,{computeWrap:compute=>async(spec,signal)=>{const result=await compute(spec,signal);if(++arrived===2)barrier.open();await barrier.promise;return result;}});
  const[a,b]=[racing(),racing()],[first,second]=await Promise.all([a.refresh({bypassBuilder:true}),b.refresh({bypassBuilder:true})]);
  expect(first.current.manifest!.generation).toBe(1);expect(second.current.manifest!.generation).toBe(1);
  expect(a.store.stats.casConflicts+b.store.stats.casConflicts).toBe(1);expect(a.store.stats.published+b.store.stats.published).toBe(1);
  expect(await a.graph(first.current)).toEqual(await b.graph(second.current));
 }finally{clean(dir);}
},60000);

test("the exclusive hold of the facts resource is the pointer swap and nothing else, however long the build takes",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(10));const holds:ReturnType<typeof createWorld>["holdsLog"]=[];
  await createWorld(dir).refresh();
  const slow=createWorld(dir,{sharedHoldsLog:holds,computeWrap:compute=>async(spec,signal)=>{await sleep(300);return compute(spec,signal);}});
  await applyOps(dir,[{op:"write",path:"pkg/p3/src/index.ts",content:"export const slow=1;\n"}]);
  const result=await slow.refresh();expect(result.built).toBe(true);
  const facts=holds.filter(row=>row.resource.startsWith("repository-graph-facts")),build=holds.filter(row=>row.resource.startsWith("repository-graph-build"));
  expect(facts.length).toBe(1);expect(build.length).toBe(1);
  expect(build[0]!.heldMs).toBeGreaterThanOrEqual(290);expect(facts[0]!.heldMs).toBeLessThan(100);expect(Math.max(...slow.store.stats.publishHeldMs)).toBeLessThan(100);
 }finally{clean(dir);}
},60000);

test("graph policies: current never waits for a builder, a verified closure never waits, only a dirty closure waits for the build",async()=>{
 const dir=scratch();
 try{
  const files:Record<string,string>={};
  for(const name of["a","b","z"])files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:[]}),files[`pkg/${name}/src/index.ts`]=name==="a"?'import "@syn/b";\n':"export const v=1;\n";
  await seed(dir,files);await createWorld(dir).refresh();
  await applyOps(dir,[{op:"write",path:"pkg/z/src/index.ts",content:"export const v=2;\n"}]);
  const gate=gated();let building=false;
  const builder=createWorld(dir,{computeWrap:compute=>async(spec,signal)=>{building=true;await gate.promise;return compute(spec,signal);}}),building1=builder.refresh();
  while(!building)await sleep(5);
  const within=async<T>(promise:Promise<T>,ms:number)=>Promise.race([promise,sleep(ms).then(()=>"timeout" as const)]);
  const peer=createWorld(dir);
  const current=await within(peer.refresh({policy:"current"}),500);
  expect(current).not.toBe("timeout");expect((current as any).served).toBe("current");expect((current as any).current.manifest.generation).toBe(0);
  const closure=await within(peer.refreshClosure(["a"]),500);
  expect(closure).not.toBe("timeout");expect((closure as any).served).toBe("verified");expect((closure as any).built).toBe(false);
  const blocked=peer.refreshClosure(["z"]);expect(await within(blocked,150)).toBe("timeout");
  gate.open();const built=await building1,waited=await blocked;
  expect(built.published).toBe(true);expect(waited.current.manifest!.generation).toBe(1);expect(waited.built).toBe(false);
  const empty=scratch();try{await seed(empty,files);const fresh=createWorld(empty),first=await fresh.refresh({policy:"current"});expect(first.served).toBe("built");}finally{clean(empty);}
  const other=createWorld(dir,{identity:sha("another-identity")}),moved=await other.refresh({policy:"current"});expect(moved.served).toBe("built");expect(moved.current.manifest!.built.kind).toBe("cold");
 }finally{clean(dir);}
},60000);

test("cancellation leaves the published generation untouched and the next run refreshes only what is left; progress is monotonic and complete",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const base=createWorld(dir),first=await base.refresh();
  await applyOps(dir,[{op:"write",path:"pkg/p2/src/index.ts",content:"export const x=1;\n"}]);
  const controller=new AbortController();let calls=0;
  const aborting=createWorld(dir,{holdsWrap:holds=>async(list,signal)=>{if(++calls===2)controller.abort(new Error("cancelled"));return holds(list,signal);}});
  await expect(aborting.refresh({signal:controller.signal})).rejects.toThrow("cancelled");
  expect((await base.store.read())!.manifest!.generation).toBe(first.current.manifest!.generation);
  const second=new AbortController(),computing=createWorld(dir,{computeWrap:compute=>async(spec,signal)=>{second.abort(new Error("stopped"));signal?.throwIfAborted();return compute(spec,signal);}});
  await expect(computing.refresh({signal:second.signal})).rejects.toThrow("stopped");
  expect((await base.store.read())!.manifest!.generation).toBe(first.current.manifest!.generation);
  const events:{phase:string;done:number;total:number}[]=[],finished=await createWorld(dir).refresh({progress:event=>{events.push(event as never);}});
  expect(finished.built&&finished.published).toBe(true);expect(finished.current.manifest!.generation).toBe(first.current.manifest!.generation+1);
  expect(events.length).toBeGreaterThan(1);expect(events.every(event=>event.done<=event.total)).toBe(true);
  const runs:typeof events[]=[];for(const event of events){if(!runs.length||event.done<runs.at(-1)!.at(-1)!.done)runs.push([]);runs.at(-1)!.push(event);}
  for(const run of runs){expect(run[0]!.done).toBe(0);expect(run.at(-1)!.done).toBe(run.at(-1)!.total);for(let i=1;i<run.length;i++)expect(run[i]!.done).toBeGreaterThanOrEqual(run[i-1]!.done);}
 }finally{clean(dir);}
},60000);

test("invalidation withdraws the generation without deleting anything and the next fresh reader rebuilds",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(4));const world=createWorld(dir),first=await world.refresh();
  expect(await world.store.invalidate({reason:"reset"})).toBe(true);
  const sealed=await world.store.read();expect(sealed!.manifest).toBeUndefined();expect(sealed!.pointer.generation).toBe(first.current.manifest!.generation+1);
  const rebuilt=await world.refresh({policy:"current"});expect(rebuilt.built).toBe(true);expect(rebuilt.current.manifest!.generation).toBe(first.current.manifest!.generation+2);
  expect(readdirSync(join(dir,".gen","m")).length).toBeGreaterThanOrEqual(2);
 }finally{clean(dir);}
},30000);

test("the membership frontier proves an unchanged tree by stat calls and rejects every change, including a racy directory",async()=>{
 const dir=scratch();
 try{
  mkdirSync(join(dir,"a/b"),{recursive:true});writeFileSync(join(dir,"a/one.ts"),"1");writeFileSync(join(dir,"a/b/two.ts"),"2");
  const old=new Date(Date.now()-60000);for(const path of["a","a/b","a/one.ts","a/b/two.ts"])utimesSync(join(dir,path),old,old);
  const capture=async()=>{
   const directories:[string,number,number][]=[],members:[string,number][]=[];
   for(const path of["a","a/b"]){const status=await lstat(join(dir,path));directories.push([path,status.mtimeMs,status.ino]);}
   for(const path of["a/one.ts","a/b/two.ts"]){const status=await lstat(join(dir,path));members.push([path,status.mode]);}
   return createFrontierV1({directories,members,capturedAt:Date.now()});
  };
  const stat=async(path:string)=>{try{const status=await lstat(join(dir,path));return{directory:status.isDirectory(),file:status.isFile(),mtimeMs:status.mtimeMs,ino:status.ino,mode:status.mode};}catch{return undefined;}};
  const frontier=await capture();
  expect(await frontierHoldsV1(frontier,stat)).toBe(true);
  expect(await frontierHoldsV1({...frontier,capturedAt:frontier.capturedAt+1e9},stat)).toBe(true);
  expect(await frontierHoldsV1({...frontier,capturedAt:Date.parse("2000-01-01")},stat)).toBe(false);
  writeFileSync(join(dir,"a/b/three.ts"),"3");expect(await frontierHoldsV1(frontier,stat)).toBe(false);
  expect(diffFrontiersV1(frontier,await capture()).filter(change=>!change.directory)).toEqual([]);
  rmSync(join(dir,"a/b/three.ts"));utimesSync(join(dir,"a/b"),old,old);utimesSync(join(dir,"a"),old,old);
  const again=await capture();expect(await frontierHoldsV1(again,stat)).toBe(true);
  rmSync(join(dir,"a/one.ts"));expect(await frontierHoldsV1(again,stat)).toBe(false);
  writeFileSync(join(dir,"a/one.ts"),"1");utimesSync(join(dir,"a"),old,old);utimesSync(join(dir,"a/one.ts"),old,old);
  const third=await capture();if(!windows){chmodSync(join(dir,"a/one.ts"),0o755);expect(await frontierHoldsV1(third,stat)).toBe(false);expect(diffFrontiersV1(third,await capture()).some(change=>change.path==="a/one.ts"&&change.kind==="changed")).toBe(true);}
  const before=createFrontierV1({directories:[["p",1,1]],members:[["p/x.ts",1],["p/y.ts",2]],capturedAt:9}),after=createFrontierV1({directories:[["p",2,1],["q",1,1]],members:[["p/y.ts",3],["p/z.ts",1]],capturedAt:10});
  expect(diffFrontiersV1(before,after)).toEqual([{path:"p",kind:"changed",directory:true},{path:"p/x.ts",kind:"removed",directory:false},{path:"p/y.ts",kind:"changed",directory:false},{path:"p/z.ts",kind:"added",directory:false},{path:"q",kind:"added",directory:true}]);
 }finally{clean(dir);}
},30000);

const lcg=(seed:number)=>()=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed/4294967296;};
const row=(path:string,fact:number):GenerationRowV1=>({kind:"file",path,fact:{size:fact}});

test("the fact index names the consumers of every row exactly like a naive map, through random patches, drops and refinements (python model agrees)",()=>{
 const random=lcg(7);
 for(let world=0;world<40;world++){
  const owners=1+Math.floor(random()*10),pool=Array.from({length:25},(_,i)=>row("f"+i,0)),table=new Map<string,GenerationRowV1[]>();
  const pick=()=>pool.filter(()=>random()<0.3);
  for(let owner=0;owner<owners;owner++)table.set("o"+owner,pick());
  let index=createFactIndexV1(table);
  const naive=()=>{const map=new Map<string,string[]>();for(const[name,rows]of table)for(const item of rows){const key=rowKeyV1(item);map.set(key,[...(map.get(key)??[]),name]);}return[...map.entries()].map(([key,names])=>[key,[...new Set(names)].sort()] as [string,string[]]).sort((a,b)=>a[0]<b[0]?-1:1);};
  const view=()=>factConsumersV1(index).filter(([,names])=>names.length).sort((a,b)=>a[0]<b[0]?-1:1);
  expect(view()).toEqual(naive());
  for(let step=0;step<8;step++){
   const kind=random(),name="o"+Math.floor(random()*(owners+2));
   if(kind<0.45){const rows=pick();table.set(name,rows);index=patchFactIndexV1(index,{upserts:new Map([[name,rows]])});}
   else if(kind<0.7){table.delete(name);index=patchFactIndexV1(index,{drop:[name]});}
   else if(kind<0.9&&table.has(name)){const extra=pick();table.set(name,[...table.get(name)!,...extra]);index=patchFactIndexV1(index,{append:new Map([[name,extra]])});}
   else index=refineFactIndexV1(index);
   expect(view()).toEqual(naive());
   expect(index.rows.length).toBeLessThanOrEqual(new Set([...table.values()].flat().map(rowKeyV1)).size);
  }
 }
 if(python){
  const owners=Object.fromEntries(Array.from({length:6},(_,o)=>["o"+o,Array.from({length:8},(_,i)=>i).filter(i=>(i*7+o*3)%4<2).map(i=>"f"+i)])),index=createFactIndexV1(new Map(Object.entries(owners).map(([name,rows])=>[name,rows.map(path=>row(path,0))])));
  const script="import json,sys\nowners=json.load(sys.stdin)\nout={}\nfor name,rows in owners.items():\n    for r in rows:\n        out.setdefault(r,set()).add(name)\nprint(json.dumps({k:sorted(v) for k,v in out.items()},sort_keys=True))";
  const result=Bun.spawnSync([python,"-I","-c",script],{stdin:Buffer.from(JSON.stringify(owners))}),model=JSON.parse(result.stdout.toString());
  expect(Object.fromEntries(factConsumersV1(index).map(([key,names])=>[JSON.parse(key)[1],names]))).toEqual(model);
 }
});

test("stale rows are found exactly, with calls proportional to the stale rows and bounded when everything is stale",async()=>{
 const rows=Array.from({length:9000},(_,i)=>row("r"+i,i)),index=createFactIndexV1(new Map([["o",rows]])),ids=rows.map((_,i)=>i);
 const make=(stale:Set<number>)=>{const lookup=new Map(index.rows.map((item,id)=>[item.path,id]));return async(list:GenerationRowV1[])=>list.every(item=>!stale.has(lookup.get(item.path)!));};
 const none=await staleFactRowsV1(index,ids,make(new Set()));expect(none.stale.size).toBe(0);expect(none.calls).toBe(Math.ceil(9000/GENERATION_POLICY_V1.verifyChunkRows));
 const random=lcg(11);
 for(const count of[1,3,40,500]){
  const stale=new Set<number>();while(stale.size<count)stale.add(Math.floor(random()*9000));
  const found=await staleFactRowsV1(index,ids,make(stale));expect([...found.stale].sort((a,b)=>a-b)).toEqual([...stale].sort((a,b)=>a-b));
  expect(found.calls).toBeLessThanOrEqual(3+count*(2*13+GENERATION_POLICY_V1.bisectBudgetCalls)+GENERATION_POLICY_V1.verifyChunkRows*Math.min(count,3));
  if(count===1)expect(found.calls).toBeLessThanOrEqual(3+2*13);
 }
 const all=new Set(ids),found=await staleFactRowsV1(index,ids,make(all));expect(found.stale.size).toBe(9000);
 expect(found.calls).toBeLessThanOrEqual(3*9000+Math.ceil(9000/GENERATION_POLICY_V1.verifyChunkRows)*GENERATION_POLICY_V1.bisectBudgetCalls);
 expect(createFactIndexV1(new Map([["o",[row("x",1),row("x",2),row("y",3)]]])).rows.length).toBe(3);
 const trace:string[]=[];await staleFactRowsV1(index,[0,1],async list=>{trace.push("hold"+list.length);return true;},{progress:event=>{trace.push(event.phase+event.done);}});expect(trace).toEqual(["facts0","hold2","facts2"]);
});

test("a closed pointer, manifest, fragment and index are admitted exactly when the declared schema admits them",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(3));const world=createWorld(dir),result=await world.refresh(),manifest=result.current.manifest!,pointer=result.current.pointer;
  const ajv=new Ajv2020({strict:false,allErrors:false});ajv.addSchema(schema,"s");
  const check=(name:string,value:unknown)=>ajv.validate({$ref:"s#/definitions/"+name},value) as boolean;
  const engine=(read:()=>unknown)=>{try{read();return true;}catch{return false;}};
  const mutations:[string,(value:any)=>any][]=[["valid",v=>v],["extra key",v=>({...v,extra:1})],["missing key",v=>{const{version,...rest}=v;return rest;}],["bad version",v=>({...v,version:2})],["bad digest",v=>({...v,generation:-1})]];
  for(const[name,mutate]of mutations){
   const pointerValue=mutate({...pointer}),manifestValue=mutate(JSON.parse(JSON.stringify(manifest)));
   expect(engine(()=>readGenerationPointerV1(Buffer.from(JSON.stringify(pointerValue)))),"pointer "+name).toBe(check("Pointer",pointerValue));
   expect(engine(()=>readGenerationManifestV1(Buffer.from(JSON.stringify(manifestValue)))),"manifest "+name).toBe(check("Manifest",manifestValue));
  }
  const bad=JSON.parse(JSON.stringify(manifest));bad.owners.p0.fragment="zz";expect(check("Manifest",bad)).toBe(false);expect(engine(()=>readGenerationManifestV1(Buffer.from(JSON.stringify(bad))))).toBe(false);
  const bad2=JSON.parse(JSON.stringify(manifest));bad2.owners.p0.kind="other";expect(check("Manifest",bad2)).toBe(false);expect(engine(()=>readGenerationManifestV1(Buffer.from(JSON.stringify(bad2))))).toBe(false);
  const fragment=await world.store.readFragment(manifest.owners.p0!);expect(check("Fragment",fragment)).toBe(true);expect(engine(()=>readGenerationFragmentV1(fragment))).toBe(true);
  expect(check("Fragment",{...fragment,extra:1})).toBe(false);expect(engine(()=>readGenerationFragmentV1({...fragment,extra:1}))).toBe(false);
  const index=await world.store.readIndex(manifest.factIndex);expect(check("FactIndex",index)).toBe(true);
  const frontier=await world.store.readFrontier(manifest.frontier);expect(check("Frontier",frontier)).toBe(true);
 }finally{clean(dir);}
},30000);

test("scale: thousands of owners verify by distinct rows, refresh only the dirty one and keep the index small",async()=>{
 const owners=3656,shared=150,priv=50,memory=createMemoryFilePort(),state={edit:0,hold:0,rows:0,full:[] as string[],computed:0};
 const sharedRows=Array.from({length:shared},(_,i)=>row("shared/"+i,0)),privateRows=(name:string)=>Array.from({length:priv},(_,i)=>row(name+"/f"+i,name==="o17"&&i===3?state.edit:0));
 const names=Array.from({length:owners},(_,i)=>"o"+i);
 const stale=(item:GenerationRowV1)=>item.path==="o17/f3"&&(item.fact as {size:number}).size!==state.edit;
 const store=createGenerationStoreV1({root:"/memory/gen",fs:memory.port,resources:{facts:"f",builder:"b"},holds:async list=>{state.hold++;state.rows+=list.length;return!list.some(stale);}});
 const membership=async()=>({digest:sha("members"),frontier:createFrontierV1({directories:[["pkg",1,1]],members:names.map(name=>[name+"/project.json",1] as [string,number]),capturedAt:5}),changes:[] as never[]});
 const compute=async(spec:{cold:boolean;owners:string[]})=>{
  const todo=spec.cold?names:spec.owners;state.computed+=todo.length;
  return{owners:todo.map(name=>({name,root:name,kind:"project" as const,node:{name},edges:[],targets:["build"],config:name+"/project.json",rows:[...sharedRows,...privateRows(name)]})),env:{digest:sha("env"),rows:[row("env/file",0)]},global:{}};
 };
 const started=Date.now(),cold=await store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,membership,compute});
 expect(cold.built).toBe(true);expect(Object.keys(cold.current.manifest!.owners).length).toBe(owners);expect(Date.now()-started).toBeLessThan(60000);
 const index=await store.readIndex(cold.current.manifest!.factIndex);
 expect(index!.rows.length).toBe(shared+owners*priv+1);expect(index!.classes.length).toBeLessThanOrEqual(owners+3);
 state.hold=0;state.rows=0;const warm=await store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,membership,compute});
 expect(warm.served).toBe("verified");expect(state.rows).toBe(index!.rows.length);expect(state.hold).toBe(Math.ceil(index!.rows.length/GENERATION_POLICY_V1.verifyChunkRows));
 state.edit=1;state.hold=0;state.computed=0;const edited=await store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,membership,compute});
 expect(edited.built&&edited.published).toBe(true);expect(state.computed).toBe(1);expect(state.hold).toBeLessThanOrEqual(Math.ceil(index!.rows.length/GENERATION_POLICY_V1.verifyChunkRows)+2*13+8);
 const patched=await store.readIndex(edited.current.manifest!.factIndex);expect(patched!.classes.length).toBeLessThanOrEqual(GENERATION_POLICY_V1.compactionFactor*patched!.owners.length+GENERATION_POLICY_V1.compactionFloor);
 expect(selectFactClassesV1(patched!,new Set(["o17"])).ids.length).toBeLessThan(shared+priv+3);
},120000);

test("operating system processes share one store: writers and readers never see a torn or stale generation and every lock is released",async()=>{
 if(windows||!process.execPath)return;
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));await createWorld(dir).refresh();
  const entry=join(here,"🏃️world","🟦️.ts"),store=join(dir,".gen"),run=(role:string,tag:string,count:number)=>Bun.spawn([process.execPath,entry,"child",dir,store,role,String(count),tag],{stdout:"pipe",stderr:"pipe"});
  const children=[run("writer","p1",5),run("writer","p4",5),run("reader","none",8),run("reader-current","none",8)];
  const codes=await Promise.all(children.map(async child=>{const code=await child.exited;if(code!==0)console.error(await new Response(child.stderr).text());return code;}));
  expect(codes).toEqual([0,0,0,0]);
  for(const[role,tag]of[["writer","p1"],["writer","p4"],["reader","none"],["reader-current","none"]]){
   const lines=JSON.parse(await Bun.file(join(dir,`.child-${role}-${tag}.json`)).text()) as {generation:number;graph:any}[];
   for(let i=1;i<lines.length;i++)expect(lines[i]!.generation).toBeGreaterThanOrEqual(lines[i-1]!.generation);
   if(role==="writer")lines.forEach((line,step)=>{expect(line.graph.projects[tag].sources[0].sha256).toBe(sha(`import "@syn/p1";\nexport const edit${tag}${step}=${step};\n`));});
  }
  const final=await createWorld(dir).refresh();expect(final.current.manifest!.generation).toBeGreaterThan(0);
  if(python)expect(normalized(await createWorld(dir).graph(final.current))).toEqual(pythonGraph(dir));
  expect((await readdir(join(dir,".leases"))).filter(name=>name.endsWith(".lock"))).toEqual([]);
 }finally{clean(dir);}
},180000);

test("the reference lease is exclusive, honours its timeout and cancellation, and replaces the lock of a dead process",async()=>{
 const dir=scratch();
 try{
  const lease=createFileLeasePortV1(dir);let inside=0,max=0;
  await Promise.all(Array.from({length:12},()=>lease.exclusive("r",async()=>{inside++;max=Math.max(max,inside);await sleep(5);inside--;})));expect(max).toBe(1);
  const gate=gated(),holder=lease.exclusive("t",()=>gate.promise);await sleep(20);
  await expect(lease.exclusive("t",async()=>1,{timeoutMs:60})).rejects.toThrow(/timed out/u);
  const controller=new AbortController(),waiting=lease.exclusive("t",async()=>1,{signal:controller.signal});await sleep(20);controller.abort(new Error("stop"));await expect(waiting).rejects.toThrow("stop");
  gate.open();await holder;
  const lockName=sha("dead")+".lock";writeFileSync(join(dir,lockName),JSON.stringify({pid:2147483646,token:"x",since:Date.now()}));
  expect(await lease.exclusive("dead",async()=>"ok")).toBe("ok");
 }finally{clean(dir);}
},30000);

test("virtual groups are replaced wholesale, an environment move refreshes only the edges of the other owners, and removed groups leave no ghost",async()=>{
 const memory=createMemoryFilePort(),state={env:sha("env-1"),group:["g1","g2"],dirty:[] as string[],log:[] as string[]};
 const node=(name:string)=>({name,root:name});
 const store=createGenerationStoreV1({root:"/memory/gen",fs:memory.port,resources:{facts:"f"},holds:async list=>!list.some(item=>(item.fact as {stale?:boolean}).stale===true&&state.dirty.includes(item.path))});
 const membership=async()=>({digest:sha("m"),frontier:createFrontierV1({directories:[],members:[],capturedAt:1}),changes:[] as never[]});
 const compute=async(spec:{cold:boolean;owners:string[]})=>{
  state.log.push((spec.cold?"cold:":"delta:")+spec.owners.join("+"));
  const real=spec.cold?["a","b"]:spec.owners.filter(name=>name==="a"||name==="b"),edgeOnly=spec.cold?[]:["a","b"].filter(name=>!real.includes(name));
  return{
   owners:[...real.map(name=>({name,root:name,kind:"project" as const,node:node(name),edges:[{source:name,target:"x"+state.env.slice(0,2)}],targets:["build"],rows:[{kind:"file",path:name,fact:{stale:true}}]})),
    ...edgeOnly.map(name=>({name,partial:"edges" as const,edges:[{source:name,target:"x"+state.env.slice(0,2)}],rows:[]})),
    ...state.group.map(name=>({name,root:name,kind:"project" as const,node:node(name),edges:[],targets:[],tag:"g",rows:[]}))],
   env:{digest:state.env,rows:[{kind:"file",path:"@env",fact:{stale:true}}]},global:{},replaceTags:["g"],
  };
 };
 const options={identity:IDENTITY,implementation:IMPLEMENTATION,membership,compute:compute as never},first=await store.refresh(options);
 expect(Object.keys(first.current.manifest!.owners).sort()).toEqual(["a","b","g1","g2"]);
 state.group=["g1"];state.dirty=["a"];
 const second=await store.refresh(options);
 expect(Object.keys(second.current.manifest!.owners).sort()).toEqual(["a","b","g1"]);expect(state.log).toEqual(["cold:","delta:a"]);
 state.env=sha("env-2");state.dirty=["@env"];state.group=[];
 const third=await store.refresh(options);
 expect(state.log.at(-1)).toBe("delta:");expect(Object.keys(third.current.manifest!.owners).sort()).toEqual(["a","b"]);
 const graph=await store.assemble(third.current);expect(graph.dependencies.map(edge=>(edge as {source:string;target:string}).target).sort()).toEqual(["x"+state.env.slice(0,2),"x"+state.env.slice(0,2)]);
 expect(Object.values(third.current.manifest!.owners).every(row=>row.edgesEnv===third.current.manifest!.env)).toBe(true);
 const index=await store.readIndex(third.current.manifest!.factIndex);expect(index!.owners.includes("g2")).toBe(false);
});

test("a shared file edited every few hundred milliseconds never restarts a 300 project build: it publishes a snapshot consistent generation within 1.2 times its quiet time and the next generation refreshes exactly the owners that read an older file",async()=>{
 const count=300,files:Record<string,string>={"shared/common.json":"0"};
 for(let i=0;i<count;i++){files[`pkg/q${i}/project.json`]=JSON.stringify({name:`q${i}`,package:`@syn/q${i}`,uses:["shared/common.json"]});files[`pkg/q${i}/src/index.ts`]=`import "@syn/q${(i+1)%count}";\nexport const v=${i};\n`;}
 const quiet=scratch(),busy=scratch();
 try{
  await seed(quiet,files);let quietMs=0;for(const attempt of[0,1]){const directory=attempt?scratch():quiet;if(attempt)await seed(directory,files);const started=Date.now();await createWorld(directory,{ownerDelayMs:8}).refresh();quietMs=Math.max(quietMs,Date.now()-started);if(attempt)clean(directory);}
  await seed(busy,files);const world=createWorld(busy,{ownerDelayMs:8});
  let editing=true,edits=0;const editor=(async()=>{while(editing){await sleep(250);edits++;await writeFile(join(busy,"shared/common.json"),String(edits));}})();
  const begin=Date.now(),result=await world.refresh(),busyMs=Date.now()-begin;editing=false;await editor;
  expect(edits).toBeGreaterThanOrEqual(3);expect(world.log.length).toBe(1);expect(result.published).toBe(true);expect(result.current.manifest!.generation).toBe(0);
  expect(busyMs).toBeLessThanOrEqual(1.2*quietMs+500);
  const current=sha(await readFile(join(busy,"shared/common.json"),"utf8")),graph:any=await world.graph(result.current),older=Object.keys(graph.projects).filter(name=>graph.projects[name].uses[0].sha256!==current).sort();
  expect(older.length).toBeGreaterThan(0);expect(new Set(Object.values(graph.projects).map((project:any)=>project.uses[0].sha256)).size).toBeGreaterThanOrEqual(3);
  for(const name of Object.keys(graph.projects))expect(Array.from({length:edits+2},(_,i)=>sha(String(i)))).toContain(graph.projects[name].uses[0].sha256);
  const next=await world.refresh();
  expect(next.built).toBe(true);expect(world.log.at(-1)!.full).toEqual(older);expect(next.current.manifest!.generation).toBe(1);
  const after=await world.refresh();expect(after.built).toBe(false);expect(after.served).toBe("verified");
  if(python)expect(normalized(await world.graph(after.current))).toEqual(pythonGraph(busy));
 }finally{clean(quiet);clean(busy);}
},180000);

test("the manifest stamps the implementation of its builder, a generation of another implementation is served as it is, and a manifest of another format is no generation",async()=>{
 const memory=createMemoryFilePort(),state={log:[] as string[]};
 const membership=async()=>({digest:sha("m"),frontier:createFrontierV1({directories:[],members:[],capturedAt:1}),changes:[] as never[]});
 const compute=async(spec:{cold:boolean;owners:string[]})=>{state.log.push(spec.cold?"cold":"delta:"+spec.owners.join("+"));return{owners:[{name:"a",root:"a",kind:"project" as const,node:{name:"a",root:"a"},edges:[],targets:["build"],rows:[{kind:"file",path:"a",fact:{}}]}],env:{digest:sha("env"),rows:[]},global:{},replaceTags:[]};};
 const store=createGenerationStoreV1({root:"/memory/gen",fs:memory.port,resources:{facts:"f"},holds:async()=>true}),options=(implementation:string,policy?:"fresh"|"current")=>({identity:IDENTITY,implementation,...(policy?{policy}:{}),membership,compute:compute as never});
 const first=await store.refresh(options(sha("impl-a")));
 expect(first.current.manifest!.implementation).toBe(sha("impl-a"));
 const verified=await store.refresh(options(sha("impl-b")));
 expect(verified.built).toBe(false);expect(verified.current.manifest!.implementation).toBe(sha("impl-a"));expect(state.log).toEqual(["cold"]);
 expect((await store.refresh(options(sha("impl-b"),"current"))).served).toBe("current");
 const manifestPath=[...memory.files.keys()].find(path=>path.startsWith("/memory/gen/m/"))!,old=JSON.parse(Buffer.from(memory.files.get(manifestPath)!.bytes).toString("utf8"));delete old.implementation;
 const bytes=Buffer.from(canonicalJsonV1(old)),digest=createHash("sha256").update(bytes).digest("hex");
 memory.files.set("/memory/gen/m/"+digest+".json",{bytes,modifiedMs:Date.now()});memory.files.set("/memory/gen/current.json",{bytes:Buffer.from(canonicalJsonV1({version:1,generation:0,manifest:digest,parent:null})),modifiedMs:Date.now()});
 const reborn=await store.refresh(options(sha("impl-b")));
 expect(reborn.built).toBe(true);expect(state.log).toEqual(["cold","cold"]);expect(reborn.current.manifest!.implementation).toBe(sha("impl-b"));
 await expect(createGenerationStoreV1({root:"/memory/other",fs:createMemoryFilePort().port,resources:{facts:"f"},holds:async()=>true}).refresh(options("not-a-digest"))).rejects.toThrow();
});

test("a star neighbor admits every owner of the generation and an unknown root is refused",async()=>{
 const memory=createMemoryFilePort(),membership=async()=>({digest:sha("m"),frontier:createFrontierV1({directories:[],members:[],capturedAt:1}),changes:[] as never[]});
 const compute=async()=>({owners:["a","b","c","d"].map(name=>({name,root:name,kind:"project" as const,node:{name,root:name,targets:{}},edges:name==="d"?[{source:"d",target:"a"}]:[],targets:[],rows:[]})),env:{digest:sha("env"),rows:[]},global:{},replaceTags:[]});
 const store=createGenerationStoreV1({root:"/memory/gen",fs:memory.port,resources:{facts:"f"},holds:async()=>true}),built=await store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,membership,compute:compute as never});
 const neighbors=(fragment:{name?:string})=>fragment.name==="b"?["*"]:[];
 expect([...await store.closureOf(built.current,["b"],{neighbors:(fragment:any)=>neighbors({name:fragment.name})})].sort()).toEqual(["a","b","c","d"]);
 expect([...await store.closureOf(built.current,["a"],{neighbors:(fragment:any)=>neighbors({name:fragment.name})})].sort()).toEqual(["a"]);
 expect([...await store.closureOf(built.current,["d"])].sort()).toEqual(["a","d"]);
 await expect(store.closureOf(built.current,["zz"])).rejects.toMatchObject({code:"GENERATION_UNKNOWN_OWNER"});
});

test("a project that enters the closure through a build is verified in the same call",async()=>{
 const dir=scratch();
 try{
  const files:Record<string,string>={};
  for(const[name,imports]of[["a",["@syn/b"]],["b",[]],["f",[]],["g",[]]] as [string,string[]][])files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:[]}),files[`pkg/${name}/src/index.ts`]=imports.map(spec=>`import "${spec}";`).join("\n")+"\nexport const v=1;\n";
  await seed(dir,files);const world=createWorld(dir);await world.refresh();
  await applyOps(dir,[{op:"write",path:"pkg/b/src/index.ts",content:'import "@syn/f";\nexport const v=2;\n'},{op:"write",path:"pkg/f/src/index.ts",content:"export const v=9;\n"},{op:"write",path:"pkg/g/src/index.ts",content:"export const v=7;\n"}]);
  const result=await world.refreshClosure(["a"]);
  expect(result.built).toBe(true);expect([...result.names].sort()).toEqual(["a","b","f"]);expect(world.log.map(row=>row.full)).toEqual([["a","b","f","g"],["b"],["f"]]);
  const graph:any=await world.graph(result.current,[...result.names]);expect(graph.projects.f.sources[0].sha256).toBe(sha("export const v=9;\n"));
 }finally{clean(dir);}
},30000);

test("verification opens the fact index as a view that reads only the rows it checks; a build loads it whole to patch it",async()=>{
 const dir=scratch();
 try{
  const files:Record<string,string>={};
  for(const[name,imports]of[["a",["@syn/b"]],["b",[]],["c",[]]] as [string,string[]][])files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:[]}),files[`pkg/${name}/src/index.ts`]=imports.map(spec=>`import "${spec}";`).join("\n")+"\nexport const v=1;\n";
  await seed(dir,files);await createWorld(dir).refresh();
  const world=createWorld(dir),stats=world.store.stats,loads=stats.indexLoads;
  const clean1=await world.refreshClosure(["a"]);
  expect(clean1.built).toBe(false);expect(stats.indexViews).toBeGreaterThanOrEqual(1);expect(stats.indexLoads).toBe(loads);
  await applyOps(dir,[{op:"write",path:"pkg/b/src/index.ts",content:"export const v=2;\n"}]);
  const dirty=await world.refreshClosure(["a"]);
  expect(dirty.built).toBe(true);expect(stats.indexLoads).toBeGreaterThan(loads);
  expect([...dirty.names].sort()).toEqual(["a","b"]);
 }finally{clean(dir);}
},30000);
