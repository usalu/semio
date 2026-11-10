/** ✂️ Contract: a derived fact is the answer of a question put to a pure computation; the projects that consumed it hold one row and the raw rows it read are the support of a pseudo owner. A stale support row asks the question again: an equal answer keeps every holder valid, a different or failing answer stales exactly the holders, and a stale raw row a project reads itself stales it as before. The decisions equal the independent Python model on hand written and seeded random worlds, and a cutoff never leaves a stale row in a valid generation. */
import{expect,test}from"bun:test";
import Ajv2020 from"ajv";
import{dirname,join}from"node:path";
import{mkdirSync,mkdtempSync,readdirSync,rmSync,statSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{canonicalJsonV1,createFactIndexV1,factConsumersV1,patchFactIndexV1,selectFactClassesV1,staleFactRowsV1,type GenerationRowV1}from"../../🟨️.mjs";
import{CUTOFF_POLICY_V1,derivedOwnerNameV1,derivedRowIdsV1,derivedUpsertsV1,holdersOfDerivedV1,isDerivedOwnerV1,orphanedDerivedOwnersV1,scopeWithDerivedV1,splitStaleConsumersV1,suspectRowsV1,suspectsWithinV1,unsupportedDerivedV1,verifiableRowIdsV1}from"../🟨️.mjs";
import{CLOSURE_KINDS_V1,CLOSURE_SETTLE_V1,closureKindOfV1,createClosureSettlerV1,sharedClosureSettlerV1}from"../../../🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs";
import{createNodeStoreV1}from"../../../🧮️derived-nodes/💾️store/🟨️.mjs";
import{dirtySummaryV1,rowPathsV1,rowTouchedV1}from"../../../🧮️derived-nodes/🧩️closure-kinds/🧭️row-paths/🟨️.mjs";
import{announceGenerationBuildV1,describeGenerationReasonV1,renderGenerationMessageV1,shapeComputeResultV1}from"../../🔌️load/🟨️.mjs";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";

const settleSuspectsV1=(options:Record<string,any>)=>createClosureSettlerV1({store:options.store}).settle({pseudoRows:suspectRowsV1(options.index,options.suspects),derive:options.derive,hold:options.hold,signal:options.signal,progress:options.progress});
type World={readonly id:string;readonly owners:Readonly<Record<string,{readonly raw:readonly string[];readonly derived:readonly string[]}>>;readonly derived:Readonly<Record<string,{readonly answer:string;readonly support:readonly string[]}>>;readonly stale:readonly string[];readonly answers:Readonly<Record<string,string>>;readonly expect:{dirty:string[];held:string[];changed:string[]};readonly scope?:readonly string[]};
const isFixture=(value:unknown):value is {readonly worlds:readonly World[]}=>new Ajv2020({strict:false}).validate({$ref:"#/definitions/Fixture",definitions:schema.definitions},value) as boolean;
const worlds=((): readonly World[]=>{if(!isFixture(fixture))throw Error("Closed cutoff fixture required");return fixture.worlds;})();
const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️oracle/🐍️.py");
const model=(world:unknown)=>{const run=Bun.spawnSync([python!,"-I",oracle,"plan"],{stdin:Buffer.from(JSON.stringify(world))});expect(run.exitCode).toBe(0);return JSON.parse(run.stdout.toString());};
const raw=(path:string):GenerationRowV1=>({kind:"file",path,fact:"h"});
const derivedRow=(world:World,question:string):GenerationRowV1=>({kind:"derived",path:question,fact:(world.derived as Record<string,{answer:string}>)[question]!.answer});

function indexOf(world:World){
 const owners=new Map<string,GenerationRowV1[]>(),questionOf=new Map<string,string>();
 for(const[name,row]of Object.entries(world.owners))owners.set(name,[...row.raw.map(raw),...row.derived.map(question=>derivedRow(world,question))]);
 for(const[question,row]of Object.entries(world.derived)){
  if(!Object.values(world.owners).some(owner=>owner.derived.includes(question)))continue;
  const head=derivedRow(world,question),name=derivedOwnerNameV1(head);questionOf.set(name,question);owners.set(name,[head,...row.support.map(raw)]);
 }
 return{index:createFactIndexV1(owners),questionOf};
}

async function decide(world:World){
 const{index,questionOf}=indexOf(world),scope=world.scope?new Set(world.scope):undefined,selection=selectFactClassesV1(index,scope?scopeWithDerivedV1(index,scope):undefined),stale=new Set(world.stale),inScope=(name:string)=>!scope||scope.has(name);
 const checked=await staleFactRowsV1(index,verifiableRowIdsV1(index,selection.ids),async rows=>rows.every(row=>!stale.has(row.path)));
 const{direct,suspects:reached}=splitStaleConsumersV1(index,selection.classes,checked.stale),suspects=scope?suspectsWithinV1(index,reached,scope):reached;
 const answers=world.answers as Record<string,string>;
 const settled=await settleSuspectsV1({index,suspects,derive:async row=>{const answer=answers[row.path];if(answer===undefined||answer==="!error")throw Error("computation failed");return{fact:answer,rows:[raw("fresh-"+row.path)]};}});
 const holders=holdersOfDerivedV1(index,settled.changed.keys()),dirty=new Set([...direct].filter(name=>!isDerivedOwnerV1(name)&&inScope(name)));
 for(const set of holders.values())for(const name of set)if(inScope(name))dirty.add(name);
 return{index,settled,suspects,dirty:[...dirty].sort(),held:[...settled.held.keys()].map(name=>questionOf.get(name)!).sort(),changed:[...settled.changed.keys()].map(name=>questionOf.get(name)!).sort()};
}

test("the fixture and the policy follow the schema file",()=>{
 expect(isFixture(fixture)).toBe(true);
 expect(CUTOFF_POLICY_V1).toEqual(schema.definitions.Policy.const as typeof CUTOFF_POLICY_V1);
});

for(const world of worlds)test("decisions equal the recorded expectation and the Python model: "+world.id,async()=>{
 const outcome=await decide(world);
 expect({dirty:outcome.dirty,held:outcome.held,changed:outcome.changed}).toEqual(world.expect);
 expect(model(world)).toEqual(world.expect);
});

test("a scoped verification asks only the questions a project of its scope holds, for every world under a seeded half scope, equal to the Python model",async()=>{
 let deferred=0;
 for(const world of worlds){
  const names=Object.keys(world.owners).sort(),scope=names.filter((_name,at)=>at%2===0),scoped={...world,scope},outcome=await decide(scoped),{questionOf}=indexOf(world);
  expect({dirty:outcome.dirty,held:outcome.held,changed:outcome.changed},world.id).toEqual(model(scoped));
  for(const name of outcome.suspects)expect(Object.entries(world.owners).some(([owner,row])=>scope.includes(owner)&&row.derived.includes(questionOf.get(name)!)),world.id).toBe(true);
  deferred+=(await decide(world)).suspects.size-outcome.suspects.size;
 }
 expect(deferred,"questions only projects outside the scope hold").toBeGreaterThan(0);
},300000);

test("a derived row is never checked as a raw fact and the pseudo owner of a derived fact is named by the whole row",()=>{
 const world=worlds.find(row=>row.id==="support stale, same answer: cutoff")!,{index}=indexOf(world);
 const ids=verifiableRowIdsV1(index,index.rows.map((_row,id)=>id));
 expect(ids.every(id=>index.rows[id]!.kind!=="derived")).toBe(true);
 expect(derivedRowIdsV1(index).size).toBe(1);
 const row=derivedRow(world,"q1");
 expect(derivedOwnerNameV1(row)).toBe(derivedOwnerNameV1({...row}));
 expect(derivedOwnerNameV1({...row,fact:"other"})).not.toBe(derivedOwnerNameV1(row));
 expect(derivedOwnerNameV1(row).startsWith(CUTOFF_POLICY_V1.ownerPrefix)).toBe(true);
 expect(derivedOwnerNameV1(row)).toHaveLength(CUTOFF_POLICY_V1.ownerPrefix.length+CUTOFF_POLICY_V1.nameDigestCharacters);
});

test("a scoped verification includes the supports of the facts its projects hold and nothing else",()=>{
 const world=worlds.find(row=>row.id==="shared support row staled two questions, one changes")!,{index,questionOf}=indexOf(world);
 const scoped=scopeWithDerivedV1(index,new Set(["a"]));
 expect([...scoped].filter(isDerivedOwnerV1).map(name=>questionOf.get(name)).sort()).toEqual(["q1"]);
 expect(scoped.has("a")).toBe(true);expect(scoped.has("b")).toBe(false);
 expect([...scopeWithDerivedV1(index,new Set(["c"]))].filter(isDerivedOwnerV1).map(name=>questionOf.get(name)).sort()).toEqual(["q1","q2"]);
});

test("without a computation every suspect changed and a failing computation changes its fact",async()=>{
 const world=worlds.find(row=>row.id==="support stale, same answer: cutoff")!,{index,questionOf}=indexOf(world),selection=selectFactClassesV1(index);
 const{stale}=await staleFactRowsV1(index,verifiableRowIdsV1(index,selection.ids),async rows=>rows.every(row=>row.path!=="s1")),{suspects}=splitStaleConsumersV1(index,selection.classes,stale);
 const bare=await settleSuspectsV1({index,suspects});
 expect([...bare.changed.keys()].map(name=>questionOf.get(name))).toEqual(["q1"]);expect(bare.held.size).toBe(0);
 const failing=await settleSuspectsV1({index,suspects,derive:async()=>{throw Error("boom");}});
 expect(failing.failed).toBe(1);expect(failing.changed.size).toBe(1);
 const controller=new AbortController();controller.abort();
 await expect(settleSuspectsV1({index,suspects,derive:async()=>{throw Error("boom");},signal:controller.signal})).rejects.toBeDefined();
});

test("a plan that starts again asks only the questions whose last inputs changed, and the answers equal asking everything again",async()=>{
 const world=worlds.find(row=>row.id==="support stale, same answer: cutoff")!,{index}=indexOf(world),selection=selectFactClassesV1(index);
 const{stale}=await staleFactRowsV1(index,verifiableRowIdsV1(index,selection.ids),async rows=>rows.every(row=>row.path!=="s1")),{suspects}=splitStaleConsumersV1(index,selection.classes,stale);
 const root=mkdtempSync(join(tmpdir(),"da-")),moved=new Set<string>(),asked:string[]=[],hold=async(rows:readonly{path:string}[])=>rows.every(row=>!moved.has(row.path));
 let round=0;
 try{
  const open=()=>createNodeStoreV1({root});
  const derive=async(row:{path:string;fact:unknown})=>{asked.push(row.path);return{fact:row.fact,rows:[raw("r"+round+"-"+row.path)]};};
  const first=await settleSuspectsV1({index,suspects,derive,store:open(),hold});
  expect(asked.length).toBe(suspects.size);expect(first.reused).toBe(0);
  const resumed=await settleSuspectsV1({index,suspects,derive,store:open(),hold});
  expect(asked.length,"a new process asks nothing the old one answered").toBe(suspects.size);expect(resumed.reused).toBe(suspects.size);expect([...resumed.held.keys()].sort()).toEqual([...first.held.keys()].sort());
  for(const path of asked)moved.add("r0-"+path);round=1;
  const third=await settleSuspectsV1({index,suspects,derive,store:open(),hold});
  expect(asked.length).toBe(2*suspects.size);expect(third.reused).toBe(0);
  const plain=await settleSuspectsV1({index,suspects,derive});
  expect([...third.held.keys()].sort()).toEqual([...plain.held.keys()].sort());expect([...third.changed.keys()].sort()).toEqual([...plain.changed.keys()].sort());
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("rows that closures share are checked once per verification, and one failing shared row refuses exactly the closures that read it",async()=>{
 const world=worlds.find(row=>row.id==="support stale, same answer: cutoff")!,{index}=indexOf(world),selection=selectFactClassesV1(index);
 const{stale}=await staleFactRowsV1(index,verifiableRowIdsV1(index,selection.ids),async rows=>rows.every(row=>row.path!=="s1")),{suspects}=splitStaleConsumersV1(index,selection.classes,stale);
 const shared=Array.from({length:50},(_,i)=>raw("shared-"+i)),calls:number[]=[],bad=new Set<string>(),root=mkdtempSync(join(tmpdir(),"da-"));
 const hold=async(rows:readonly{path:string}[])=>{calls.push(rows.length);return rows.every(row=>!bad.has(row.path));};
 const derive=async(row:{path:string;fact:unknown})=>({fact:row.fact,rows:[...shared,raw("own-"+row.path)]});
 try{
  await settleSuspectsV1({index,suspects,derive,store:createNodeStoreV1({root}),hold});calls.length=0;
  const again=await settleSuspectsV1({index,suspects,derive,store:createNodeStoreV1({root}),hold});
  expect(again.reused).toBe(suspects.size);expect(calls.reduce((sum,count)=>sum+count,0),"every unique row once").toBeLessThanOrEqual(shared.length+suspects.size);
  bad.add("shared-7");calls.length=0;
  const refused=await settleSuspectsV1({index,suspects,derive,store:createNodeStoreV1({root}),hold});
  expect(refused.reused).toBe(0);expect(calls.length,"the failing row is found by bisection, not by one call per closure").toBeLessThan(40);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a held fact replaces its support, so the next verification finds nothing stale; the index equals a cold build of the same ownership",async()=>{
 const world=worlds.find(row=>row.id==="support stale, same answer: cutoff")!,first=await decide(world);
 expect(first.dirty).toEqual([]);expect(first.held).toEqual(["q1"]);
 const patched=patchFactIndexV1(first.index,{upserts:derivedUpsertsV1([],first.settled.held)});
 const next={...world,stale:[]};
 const{questionOf}=indexOf(next),name=[...questionOf.keys()][0]!;
 const owners=new Map<string,GenerationRowV1[]>(Object.entries(world.owners).map(([owner,row])=>[owner,[...row.raw.map(raw),...row.derived.map(question=>derivedRow(world,question))]]));
 owners.set(name,[derivedRow(world,"q1"),raw("fresh-q1")]);
 const byKey=(index:Parameters<typeof factConsumersV1>[0])=>factConsumersV1(index).sort((a,b)=>a[0]<b[0]?-1:1);
 expect(byKey(patched)).toEqual(byKey(createFactIndexV1(owners)));
 const selection=selectFactClassesV1(patched),stale=new Set(["s1","s2"]);
 const again=await staleFactRowsV1(patched,verifiableRowIdsV1(patched,selection.ids),async rows=>rows.every(row=>!stale.has(row.path)));
 expect(again.stale.size).toBe(0);
});

test("a pseudo owner whose derived fact no project holds any more is orphaned and removed by the patch",()=>{
 const world=worlds.find(row=>row.id==="support stale, other answer: all holders")!,{index}=indexOf(world);
 expect(orphanedDerivedOwnersV1(index)).toEqual([]);
 const bare=patchFactIndexV1(index,{upserts:new Map([["a",[raw("r1")]],["b",[]]])}),orphans=orphanedDerivedOwnersV1(bare);
 expect(orphans).toHaveLength(1);
 expect(patchFactIndexV1(bare,{drop:orphans}).owners.some(isDerivedOwnerV1)).toBe(false);
});

test("compute results win over held supports and every pseudo owner holds its own derived row",()=>{
 const row:GenerationRowV1={kind:"derived",path:"q",fact:"d"},fresh=derivedUpsertsV1([{row,support:[raw("a"),raw("b")]}],new Map([[derivedOwnerNameV1(row),[row,raw("old")]],["@derived:other",[row]]]));
 expect(fresh.get(derivedOwnerNameV1(row))).toEqual([row,raw("a"),raw("b")]);
 expect(fresh.get("@derived:other")).toEqual([row]);
});

test("a derived row that projects hold without a supporting pseudo owner is reported",()=>{
 const row:GenerationRowV1={kind:"derived",path:"q",fact:"d"},bare=createFactIndexV1(new Map([["a",[row]],["b",[raw("x")]]]));
 expect(unsupportedDerivedV1(bare)).toEqual([row]);
 const supported=createFactIndexV1(new Map([["a",[row]],[derivedOwnerNameV1(row),[row,raw("s")]]]));
 expect(unsupportedDerivedV1(supported)).toEqual([]);
});

test("progress counts the suspect questions and says nothing when there are none",async()=>{
 const world=worlds.find(row=>row.id==="support stale, same answer: cutoff")!,{index}=indexOf(world),events:{phase:string;done:number;total:number}[]=[];
 await settleSuspectsV1({index,suspects:new Set(),progress:event=>{events.push(event);}});
 expect(events).toEqual([]);
 const selection=selectFactClassesV1(index),{stale}=await staleFactRowsV1(index,verifiableRowIdsV1(index,selection.ids),async rows=>rows.every(row=>row.path!=="s1")),{suspects}=splitStaleConsumersV1(index,selection.classes,stale);
 await settleSuspectsV1({index,suspects,derive:async row=>({fact:row.fact,rows:[]}),progress:event=>{events.push(event);}});
 expect(events.filter(event=>event.phase==="verification")).toEqual([{phase:"verification",done:0,total:1},{phase:"verification",done:1,total:1}] as never);
 expect(events.map(event=>event.phase).filter(phase=>phase!=="verification"),"the flush of the chunk is a counted stage").toEqual(["flush","flush"]);
});

test("a changed derived fact is told in both languages with the first entry of its walk, and the cutoff summary is announced",()=>{
 const row={kind:"derived",path:JSON.stringify({walk:1,roots:["/w/tools/entry.ts"]}),fact:"d"};
 for(const[language,text]of[["en","the derived result of tools/entry.ts changed"],["de","das abgeleitete Ergebnis von tools/entry.ts hat sich geändert"]] as const)
  expect(describeGenerationReasonV1({row,owners:["p1"]},{workspaceRoot:"/w",language}).why).toBe(text);
 expect(renderGenerationMessageV1("cutoff",{questions:9,rows:3,held:9,changed:0},"en")).toBe("graph: 9 derived results asked again after 3 stale facts: 9 unchanged, 0 changed");
 expect(renderGenerationMessageV1("cutoff",{questions:9,rows:3,held:9,changed:0},"de")).toBe("Graph: 9 abgeleitete Ergebnisse nach 3 veralteten Fakten erneut abgefragt: 9 unverändert, 0 verändert");
 expect(typeof announceGenerationBuildV1).toBe("function");
});

test("shaping hands the derived facts of the inference to the store and keeps the derived row among the rows of its project",()=>{
 const head:GenerationRowV1={kind:"derived",path:"q",fact:"d"},support=[raw("s1"),raw("s2")];
 const part={owners:[{name:"a",root:"a",node:{},nodeRows:[raw("own"),head]}],edgeOnly:[],edges:[],rows:[],env:{digest:"e".repeat(64),rows:[]},derived:[{row:head,support}]};
 const shaped=shapeComputeResultV1({workspaceRoot:"/w",part:part as never,stubs:{}});
 expect(shaped.derived).toEqual([{row:head,support}]);
 expect(shaped.owners[0]!.rows).toContainEqual(head);
 const plain=shapeComputeResultV1({workspaceRoot:"/w",part:{...part,derived:undefined} as never,stubs:{}});
 expect("derived"in plain).toBe(false);
});

test("a retained complete answer is compared to the current generation row and refreshes its current support",async()=>{
 const first:GenerationRowV1={kind:"derived",path:"q",fact:"answer"},next={...first,fact:"peer"},support=[raw("fresh")],root=mkdtempSync(join(tmpdir(),"da-")),store=createNodeStoreV1({root});let calls=0;
 const check=async(row:GenerationRowV1)=>{
  const name=derivedOwnerNameV1(row),index=createFactIndexV1(new Map([["holder",[row]],[name,[row,raw("old")]]]));
  return settleSuspectsV1({index,suspects:new Set([name]),store,hold:async rows=>rows.every(read=>read.path==="fresh"),derive:async()=>{calls++;return{fact:"answer",rows:support};}});
 };
 try{
  const before=await check(first);expect([...before.held.get(derivedOwnerNameV1(first))!]).toEqual([first,...support]);
  const after=await check(next);expect(calls).toBe(1);expect(after.reused).toBe(1);
  expect([...after.changed.values()]).toEqual([next]);expect(after.held.size).toBe(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a question is a script walk, a TypeScript program or a derived walk, and an unknown shape is a script walk the walker refuses",()=>{
 expect([...CLOSURE_KINDS_V1]).toEqual(["script-walk","ts-program","derived-walk"]);
 expect(closureKindOfV1(JSON.stringify({walk:1,roots:[]}))).toBe("script-walk");
 expect(closureKindOfV1(JSON.stringify({program:1,projects:[]}))).toBe("ts-program");
 expect(closureKindOfV1(JSON.stringify({derive:1,recipe:{}}))).toBe("derived-walk");
 expect(closureKindOfV1("q1")).toBe("script-walk");
});

test("the settler of a workspace runs on the workspace graph: its disk world, stat index and node store, shared by every plan of the workspace",async()=>{
 const workspace=mkdtempSync(join(tmpdir(),"da-ws-")),cache=mkdtempSync(join(tmpdir(),"da-cache-"));
 try{
  writeFileSync(join(workspace,"package.json"),JSON.stringify({name:"pkg",dependencies:{left:"1"}}));
  const settler=sharedClosureSettlerV1({workspaceRoot:workspace,cacheRoot:cache});
  expect(sharedClosureSettlerV1({workspaceRoot:workspace,cacheRoot:cache}),"one settler per workspace").toBe(settler);
  const graph=settler.graph as {key(kind:string,subject:string,params?:unknown):unknown;get(key:unknown):Promise<{value:{dependencies?:string[]}}>};
  const manifest=await graph.get(graph.key("manifest","package.json"));
  expect(JSON.stringify(manifest.value)).toContain("left");
  const row:GenerationRowV1={kind:"derived",path:"q",fact:"answer"},name=derivedOwnerNameV1(row);
  const index=createFactIndexV1(new Map([["holder",[row]],[name,[row,raw("support")]]]));
  let asked=0;
  const derive=async()=>{asked++;return{fact:"answer",rows:[raw("support")]};};
  const first=await settler.settle({pseudoRows:suspectRowsV1(index,new Set([name])),derive,hold:async()=>true});
  expect(first.held.size).toBe(1);expect(asked).toBe(1);
  const second=await settler.settle({pseudoRows:suspectRowsV1(index,new Set([name])),derive,hold:async()=>true});
  expect(second.held.size).toBe(1);expect(asked,"the second plan of the workspace takes the recorded answer").toBe(1);
  void mkdirSync;
 }finally{rmSync(workspace,{recursive:true,force:true});rmSync(cache,{recursive:true,force:true});}
});

test("the journal dirty set decides which recorded answers are verified: a clean set trusts every answer without one check, a set naming a support path or marking an unwatched path checks exactly those",async()=>{
 const settler=createClosureSettlerV1(),row:GenerationRowV1={kind:"derived",path:"q",fact:"answer"},name=derivedOwnerNameV1(row),index=createFactIndexV1(new Map([["holder",[row]],[name,[row,raw("support")]]]));
 let asked=0,checked=0;const derive=async()=>{asked++;return{fact:"answer",rows:[raw("support")]};},hold=async()=>{checked++;return true;},rows=()=>suspectRowsV1(index,new Set([name]));
 await settler.settle({pseudoRows:rows(),derive,hold});expect(asked).toBe(1);
 const clean=await settler.settle({pseudoRows:rows(),derive,hold,dirty:{paths:[]}});
 expect(clean.held.size).toBe(1);expect(checked,"nothing dirty: no row is checked").toBe(0);expect(asked).toBe(1);
 const touched=await settler.settle({pseudoRows:rows(),derive,hold,dirty:{paths:["support"]}});
 expect(touched.held.size).toBe(1);expect(checked,"the dirty support path is checked").toBe(1);expect(asked).toBe(1);
 const failing=await settler.settle({pseudoRows:rows(),derive,hold:async()=>false,dirty:{paths:["support"]}});
 expect(failing.held.size).toBe(1);expect(asked,"a support that no longer holds asks the question again").toBe(2);
 const unwatched=await settler.settle({pseudoRows:rows(),derive,hold,dirty:{paths:[],excluded:path=>path==="support"}});
 expect(unwatched.held.size).toBe(1);expect(checked,"an unwatched path is always checked").toBe(2);
 const overflow=await settler.settle({pseudoRows:rows(),derive,hold,dirty:{paths:[],overflow:true}});
 expect(overflow.held.size).toBe(1);expect(checked,"an overflowing set checks everything").toBe(3);
});

test("a closure is stored as references to the row parts it reached: shared parts are kept once, the record stays small however many rows the closure holds, and the held support is the same rows",async()=>{
 const names=["q1","q2","q3"],shared=Array.from({length:3000},(_,at)=>raw("shared-"+at)),own=names.map(name=>raw("own-"+name));
 const index=createFactIndexV1(new Map([["holder",names.map(name=>({kind:"derived",path:name,fact:"old"}) as GenerationRowV1)],...names.map(name=>[derivedOwnerNameV1({kind:"derived",path:name,fact:"old"}),[{kind:"derived",path:name,fact:"old"} as GenerationRowV1,raw("old-"+name)]] as [string,GenerationRowV1[]])]));
 const suspects=new Set(names.map(name=>derivedOwnerNameV1({kind:"derived",path:name,fact:"old"}))),root=mkdtempSync(join(tmpdir(),"parts-"));
 try{
  const settled=await createClosureSettlerV1({store:createNodeStoreV1({root})}).settle({pseudoRows:suspectRowsV1(index,suspects),derive:async(row:{path:string})=>({fact:"old",parts:[shared,[own[names.indexOf(row.path)]!]]}),hold:async()=>true});
  expect(settled.failed).toBe(0);
  const files=readdirSync(join(root,"n")),closureBytes=files.map(file=>statSync(join(root,"n",file)).size);
  expect(files.length,"three closures, four parts, the four path summaries of the parts and the settle manifest").toBe(3+4+4+1);
  for(const name of suspects){const rows=[...settled.held.get(name)!];expect(rows.length).toBe(1+3000+1);expect(new Set(rows.map(row=>row.path)).size).toBe(rows.length);}
  expect(Math.max(...closureBytes.filter(size=>size<20000)),"a closure record holds ids, not 3,000 rows").toBeLessThan(6000);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a resolution or an owner row names what can move it, so a dirty set elsewhere leaves it alone and one that reaches it does not",()=>{
 const marker="\0volatile",relative=(path:string)=>path==="/w"?"":path.startsWith("/w/")?path.slice(3):marker;
 const meta=(row:{kind:string;path:string;fact?:unknown})=>rowPathsV1(row as never,relative,marker),touched=(row:{kind:string;path:string;fact?:unknown},paths:string[])=>rowTouchedV1(meta(row),dirtySummaryV1({paths}));
 const near={kind:"resolve",path:JSON.stringify(["/w/a/x.ts","./y"]),fact:"/w/a/y.ts"},bare={kind:"resolve",path:JSON.stringify(["/w/a/x.ts","left-pad"]),fact:"/w/node_modules/left-pad/index.js"},owner={kind:"registry",path:JSON.stringify(["/w","p"]),fact:"null"};
 expect(touched(near,["b/z.ts"])).toBe(false);expect(touched(near,["a/new.ts"]),"a new entry of the probed directory").toBe(true);expect(touched(near,["a/y.ts"]),"the resolved file").toBe(true);
 expect(touched(bare,["b/z.ts"])).toBe(false);expect(touched(bare,["node_modules/left-pad/package.json"])).toBe(true);expect(touched(bare,["package.json"]),"a manifest").toBe(true);expect(touched(bare,["a/other.ts"]),"an entry of an ancestor").toBe(true);
 expect(touched(owner,["b/z.ts"])).toBe(false);expect(touched(owner,["p/📋️project.json"])).toBe(true);
 expect(meta({kind:"resolve",path:JSON.stringify(["/elsewhere/x.ts","./y"]),fact:"!MODULE_NOT_FOUND"}).volatile,"outside the workspace nothing is known").toBe(true);
 expect(rowTouchedV1(meta({kind:"stamp",path:"s",fact:"x"}),dirtySummaryV1({paths:[]})),"a row nothing is known about is always checked").toBe(true);
});

test("a clean settle round releases what it built: five rounds over the same closures leave the heap where the first left it",async()=>{
 const names=Array.from({length:40},(_,at)=>"q"+at),shared=Array.from({length:4000},(_,at)=>raw("shared-"+at)),rowOf=(name:string)=>({kind:"derived",path:name,fact:"old"}) as GenerationRowV1;
 const index=createFactIndexV1(new Map([["holder",names.map(rowOf)],...names.map(name=>[derivedOwnerNameV1(rowOf(name)),[rowOf(name),raw("old-"+name)]] as [string,GenerationRowV1[]])]));
 const suspects=new Set(names.map(name=>derivedOwnerNameV1(rowOf(name)))),root=mkdtempSync(join(tmpdir(),"rounds-")),settler=createClosureSettlerV1({store:createNodeStoreV1({root})});
 const heap=()=>{Bun.gc(true);return process.memoryUsage().heapUsed;};
 try{
  const round=async()=>{const settled=await settler.settle({pseudoRows:suspectRowsV1(index,suspects),derive:async(row:{path:string})=>({fact:"old",parts:[shared,[raw("own-"+row.path)]]}),hold:async()=>true,dirty:{paths:[]}});expect(settled.failed).toBe(0);expect(settled.held.size).toBe(40);};
  await round();const first=heap();
  for(let at=0;at<4;at++)await round();
  expect((heap()-first)/1048576,"megabytes the clean rounds kept").toBeLessThan(5);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a settle killed in the middle leaves its flushed chunks durable and the resumed settle derives only what was unfinished",async()=>{
 const count=60,chunk=10,killAt=35,names=Array.from({length:count},(_,at)=>"q"+String(at).padStart(4,"0")),rowOf=(name:string)=>({kind:"derived",path:name,fact:"old"}) as GenerationRowV1,root=mkdtempSync(join(tmpdir(),"killed-"));
 try{
  const child=Bun.spawnSync([process.execPath,join(dirname(fileURLToPath(import.meta.url)),"🪓️killed-settle/🟨️.mjs"),root,String(count),String(chunk),String(killAt)]);
  expect(child.signalCode ?? child.exitCode,"the child was killed").not.toBe(0);
  const index=createFactIndexV1(new Map([["holder",names.map(rowOf)],...names.map(name=>[derivedOwnerNameV1(rowOf(name)),[rowOf(name),raw("old-"+name)]] as [string,GenerationRowV1[]])]));
  const derived:string[]=[],settled=await createClosureSettlerV1({store:createNodeStoreV1({root}),limits:{...CLOSURE_SETTLE_V1,settleChunkClosures:chunk}}).settle({pseudoRows:suspectRowsV1(index,new Set(names.map(name=>derivedOwnerNameV1(rowOf(name))))),derive:async(row:{path:string})=>{derived.push(row.path);return{fact:"old",parts:[[raw("own-"+row.path)]]};},hold:async()=>true});
  expect(settled.failed).toBe(0);
  expect(settled.held.size).toBe(count);
  const flushed=Math.floor((killAt-1)/chunk)*chunk;
  expect(derived.length,"only the unfinished closures were derived again").toBeLessThanOrEqual(count-flushed);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a held closure whose support the index already carries is kept: it is neither refreshed nor written again, and a changed support is refreshed once",async()=>{
 const names=["q1","q2","q3"],rowOf=(name:string)=>({kind:"derived",path:name,fact:"old"}) as GenerationRowV1,root=mkdtempSync(join(tmpdir(),"kept-"));
 const index=createFactIndexV1(new Map([["holder",names.map(rowOf)],...names.map(name=>[derivedOwnerNameV1(rowOf(name)),[rowOf(name),raw("old-"+name)]] as [string,GenerationRowV1[]])]));
 const suspects=new Set(names.map(name=>derivedOwnerNameV1(rowOf(name)))),settler=createClosureSettlerV1({store:createNodeStoreV1({root})});
 let version="a";
 const round=()=>settler.settle({pseudoRows:suspectRowsV1(index,suspects),derive:async(row:{path:string})=>({fact:"old",parts:[[raw("own-"+row.path+version)]]}),hold:async()=>true});
 try{
  const first=await round();
  expect(first.held.size,"nothing is indexed yet").toBe(3);expect(first.kept.size).toBe(0);
  await settler.markIndexed({held:first.held});
  const second=await round();
  expect(second.held.size).toBe(0);expect(second.kept.size,"the indexed supports are kept").toBe(3);
  version="b";await settler.invalidate({paths:[]});
  const named=[...suspects][0]!;await settler.markIndexed({dropped:[named]});
  const third=await round();
  expect(third.kept.has(named),"a dropped marker is refreshed").toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a clean round over a fresh settler checks no row and derives nothing, and a dirty path asks exactly the closures whose reach contains it",async()=>{
 const names=Array.from({length:30},(_,at)=>"q"+String(at).padStart(3,"0")),rowOf=(name:string)=>({kind:"derived",path:name,fact:"old"}) as GenerationRowV1,root=mkdtempSync(join(tmpdir(),"clean-"));
 const index=createFactIndexV1(new Map([["holder",names.map(rowOf)],...names.map(name=>[derivedOwnerNameV1(rowOf(name)),[rowOf(name),raw("old-"+name)]] as [string,GenerationRowV1[]])]));
 const suspects=new Set(names.map(name=>derivedOwnerNameV1(rowOf(name)))),shared=Array.from({length:200},(_,at)=>raw("shared/"+at));
 const settle=async(settler:ReturnType<typeof createClosureSettlerV1>,dirty:{paths:string[]}|undefined,counts:{derived:string[];held:number})=>settler.settle({pseudoRows:suspectRowsV1(index,suspects),derive:async(row:{path:string})=>{counts.derived.push(row.path);return{fact:"old",parts:[shared,[raw("own/"+row.path)]]};},hold:async rows=>{counts.held+=rows.length;return!rows.some(row=>dirty?.paths.includes(row.path));},dirty});
 try{
  const first={derived:[] as string[],held:0},one=createClosureSettlerV1({store:createNodeStoreV1({root})}),cold=await settle(one,undefined,first);
  expect(first.derived.length).toBe(30);await one.markIndexed({held:cold.held});
  const clean={derived:[] as string[],held:0},again=await settle(createClosureSettlerV1({store:createNodeStoreV1({root})}),{paths:[]},clean);
  expect(clean.derived,"nothing is derived on a clean journal").toEqual([]);expect(clean.held,"no row is checked on a clean journal").toBe(0);
  expect(again.kept.size).toBe(30);expect(again.held.size).toBe(0);
  const edit={derived:[] as string[],held:0},touched=await settle(createClosureSettlerV1({store:createNodeStoreV1({root})}),{paths:["own/q007"]},edit);
  expect(edit.derived,"only the closure that reaches the dirty path is derived again").toEqual(["q007"]);
  expect(touched.kept.size+touched.held.size).toBe(30);
  const wide={derived:[] as string[],held:0};await settle(createClosureSettlerV1({store:createNodeStoreV1({root})}),{paths:["shared/5"]},wide);
  expect(wide.derived.length,"a path of the shared part reaches every closure").toBe(30);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a settle keeps the row lists of its answers only when asked, and the settler holds neither the deriver nor its captures after it",async()=>{
 const world=worlds.find(row=>Object.keys(row.derived).length>0&&row.stale.length>0)??worlds[0]!,{index,suspects}=await decide(world),answers=world.answers as Record<string,string>;
 const make=()=>{const capture={payload:new Array(1000).fill("x")};return{capture,derive:async(row:{path:string})=>{if(capture.payload.length===0)throw Error("emptied");const answer=answers[row.path];if(answer===undefined||answer==="!error")throw Error("computation failed");return{fact:answer,rows:[raw("fresh-"+row.path)]};}};};
 const settler=createClosureSettlerV1(),weak:WeakRef<object>[]=[];
 const run=async(keep:boolean)=>{const{capture,derive}=make();weak.push(new WeakRef(capture));const settled=await settler.settle({pseudoRows:suspectRowsV1(index,suspects),derive,hold:async()=>false,...keep?{answers:true}:{}} as never);return settled.answers.size;};
 const kept=await run(true),dropped=await run(false);
 expect(suspects.size).toBeGreaterThan(0);expect(dropped).toBe(0);expect(kept).toBeGreaterThan(0);
 for(let round=0;round<5;round++){await new Promise(resolve=>setTimeout(resolve,5));Bun.gc(true);}
 expect(weak.map(ref=>ref.deref()!==undefined)).toEqual([false,false]);
});

const heapChild=(mode:string,questions:number,perQuestion:number,chunk:number)=>{
 const run=Bun.spawnSync([process.execPath,join(dirname(fileURLToPath(import.meta.url)),"🧮️heap-child/🟨️.mjs"),mode,String(questions),String(perQuestion),String(chunk)],{stdout:"pipe",stderr:"pipe"});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 return JSON.parse(run.stdout.toString().trim().split("\n").pop()!) as{outcome:{changed:number;answers:number};growth:number;after:number[]};
};

test("settling a large generated fixture of questions leaves the heap under its bound once the result is dropped",()=>{
 const questions=Number(process.env.SEMIO_SETTLE_QUESTIONS??3000),bound=Number(process.env.SEMIO_SETTLE_HEAP_MB??24),result=heapChild("growth",questions,150,256);
 expect(result.outcome).toEqual({changed:questions,answers:0});
 expect(result.growth,"heap growth after settling "+questions+" questions: "+result.growth.toFixed(1)+" MB").toBeLessThan(bound);
},300000);

test("the heap after each settle chunk stays flat while thousands of closures with distinct supports settle through a durable store",()=>{
 const questions=Number(process.env.SEMIO_CHUNK_QUESTIONS??2560),tolerance=Number(process.env.SEMIO_CHUNK_HEAP_MB??50),result=heapChild("chunks",questions,400,256),settled=result.after.slice(1);
 expect(result.after.length).toBeGreaterThanOrEqual(questions/256-1);
 expect(Math.max(...settled)-Math.min(...settled),"heap after each chunk (MB): "+result.after.map(value=>value.toFixed(0)).join(", ")).toBeLessThan(tolerance);
},300000);

test("rounds answered from the settle manifest equal the full re-verification under random edits, adds, deletes and renames, with directory listings, absence rows and configuration rows",async()=>{
 let state=99;const draw=(limit:number)=>{state=(state*1103515245+12345)%2147483648;return(state>>8)%limit;};
 const world=new Map<string,string>(),dirs=["a","b","c","d","e","f"],nameOf=(dir:string,at:number)=>dir+"/m"+at+".ts";
 for(const dir of dirs)for(let at=0;at<10;at++)world.set(nameOf(dir,at),"v"+draw(1000));
 world.set("package.json","c0");
 const listing=(dir:string)=>[...world.keys()].filter(path=>path.startsWith(dir+"/")).sort().join(",");
 const rowOf=(kind:string,path:string):GenerationRowV1=>kind==="file"?{kind,path,fact:world.get(path)??"!absent"}:kind==="exists"?{kind,path,fact:world.has(path)}:kind==="readdir"?{kind,path,fact:listing(path)}:{kind:"registry",path,fact:world.get("package.json")};
 const plans=new Map<string,[string,string][]>(),names=Array.from({length:30},(_,at)=>"q"+String(at).padStart(2,"0"));
 for(const name of names){const reads:[string,string][]=[];for(let item=0;item<8+draw(8);item++){const dir=dirs[draw(dirs.length)]!,kind=["file","file","exists","readdir"][draw(4)]!;reads.push(kind==="readdir"?[kind,dir]:[kind,nameOf(dir,draw(14))]);}if(draw(3)===0)reads.push(["registry","package.json"]);plans.set(name,reads);}
 const shared=(dir:string)=>[rowOf("readdir",dir)];
 const derive=async(row:{path:string})=>{const reads=plans.get(row.path)!,rows=reads.map(([kind,path])=>rowOf(kind,path));return{fact:rows.map(item=>String(item.fact)).join("|").length+":"+rows.map(item=>JSON.stringify(item.fact)).join("").split("").reduce((sum,char)=>(sum*31+char.charCodeAt(0))>>>0,7),parts:[shared("a"),rows]};};
 const hold=async(rows:GenerationRowV1[])=>rows.every(item=>JSON.stringify(rowOf(item.kind,item.path).fact)===JSON.stringify(item.fact));
 const rootA=mkdtempSync(join(tmpdir(),"manifest-a-")),rootB=mkdtempSync(join(tmpdir(),"manifest-b-"));
 try{
  const fast=createClosureSettlerV1({store:createNodeStoreV1({root:rootA})}),full=()=>createClosureSettlerV1({store:createNodeStoreV1({root:rootB+"/"+draw(1e9)})});
  const previous=new Map<string,unknown>();
  const rowsOf=()=>new Map(names.map(name=>[name,{kind:"derived",path:name,fact:previous.get(name)??"none"} as GenerationRowV1]));
  const verdict=async(settled:any)=>({held:[...settled.held.keys()].sort(),changed:[...settled.changed.keys()].sort(),failed:settled.failed});
  const first=await fast.settle({pseudoRows:rowsOf(),derive,hold});
  for(const name of names)previous.set(name,(await derive({path:name})).fact);
  let moved=0;
  for(let round=0;round<14;round++){
   const touched=new Set<string>();
   for(let edit=0;edit<1+draw(3);edit++){
    const paths=[...world.keys()].filter(path=>path!=="package.json"),kind=draw(5),dir=dirs[draw(dirs.length)]!;
    if(kind===0){const path=paths[draw(paths.length)]!;world.set(path,"v"+draw(1000)+"x");touched.add(path);}
    else if(kind===1){const path=paths[draw(paths.length)]!;world.delete(path);touched.add(path);}
    else if(kind===2){const path=nameOf(dir,draw(14));world.set(path,"n"+draw(1000));touched.add(path);}
    else if(kind===3){const path=paths[draw(paths.length)]!,target=nameOf(dir,draw(14)),text=world.get(path)!;world.delete(path);world.set(target,text);touched.add(path);touched.add(target);}
    else{world.set("package.json","c"+draw(1000));touched.add("package.json");}
   }
   const listed=new Set(touched);for(const path of touched)if(path.includes("/"))listed.add(path.slice(0,path.indexOf("/")));const dirty={paths:[...listed]},oracle=await verdict(await full().settle({pseudoRows:rowsOf(),derive,hold})),got=await verdict(await fast.settle({pseudoRows:rowsOf(),derive,hold,dirty}));
   expect(got,"round "+round+" after "+[...touched].join(",")).toEqual(oracle);
   if(oracle.changed.length)moved++;
   for(const name of names)previous.set(name,(await derive({path:name})).fact);
  }
  expect(first.failed).toBe(0);expect(moved).toBeGreaterThan(3);
 }finally{rmSync(rootA,{recursive:true,force:true});rmSync(rootB,{recursive:true,force:true});}
},300000);
