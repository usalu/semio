/** ✂️ Contract: a derived fact is the answer of a question put to a pure computation; the projects that consumed it hold one row and the raw rows it read are the support of a pseudo owner. A stale support row asks the question again: an equal answer keeps every holder valid, a different or failing answer stales exactly the holders, and a stale raw row a project reads itself stales it as before. The decisions equal the independent Python model on hand written and seeded random worlds, and a cutoff never leaves a stale row in a valid generation. */
import{expect,test}from"bun:test";
import Ajv2020 from"ajv";
import{dirname,join}from"node:path";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{canonicalJsonV1,createFactIndexV1,factConsumersV1,patchFactIndexV1,selectFactClassesV1,staleFactRowsV1,type GenerationRowV1}from"../../🟨️.mjs";
import{CUTOFF_POLICY_V1,derivedOwnerNameV1,derivedRowIdsV1,derivedUpsertsV1,holdersOfDerivedV1,isDerivedOwnerV1,orphanedDerivedOwnersV1,scopeWithDerivedV1,settleSuspectsV1,splitStaleConsumersV1,suspectsWithinV1,unsupportedDerivedV1,verifiableRowIdsV1}from"../🟨️.mjs";
import{createDurableAnswersV1}from"../💾️durable/🟨️.mjs";
import{createFileLeasePortV1,createNodeFilePortV1}from"../../🟨️.mjs";
import{announceGenerationBuildV1,describeGenerationReasonV1,renderGenerationMessageV1,shapeComputeResultV1}from"../../🔌️load/🟨️.mjs";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";

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
 const moved=new Set<string>(),asked:string[]=[],memo=new Map(),hold=async(rows:readonly{path:string}[])=>rows.every(row=>!moved.has(row.path));
 const derive=async(row:{path:string;fact:unknown})=>{asked.push(row.path);return{fact:row.fact,rows:[raw("fresh-"+row.path)]};};
 const first=await settleSuspectsV1({index,suspects,derive,memo,hold} as never);
 expect(asked.length).toBe(suspects.size);expect(first.reused).toBe(0);
 const second=await settleSuspectsV1({index,suspects,derive,memo,hold} as never);
 expect(asked.length).toBe(suspects.size);expect(second.reused).toBe(suspects.size);expect([...second.held.keys()].sort()).toEqual([...first.held.keys()].sort());
 for(const row of memo.values())for(const read of row.rows)moved.add(read.path);
 const third=await settleSuspectsV1({index,suspects,derive,memo,hold} as never);
 expect(asked.length).toBe(2*suspects.size);expect(third.reused).toBe(0);
 const plain=await settleSuspectsV1({index,suspects,derive} as never);
 expect([...third.held.keys()].sort()).toEqual([...plain.held.keys()].sort());expect([...third.changed.keys()].sort()).toEqual([...plain.changed.keys()].sort());
});

test("a killed plan resumes: durable answers survive the process, are verified against the disk before they are taken, and a peer waits for the holder of a question instead of deriving it again",async()=>{
 const world=worlds.find(row=>row.id==="support stale, same answer: cutoff")!,{index}=indexOf(world),selection=selectFactClassesV1(index);
 const{stale}=await staleFactRowsV1(index,verifiableRowIdsV1(index,selection.ids),async rows=>rows.every(row=>row.path!=="s1")),{suspects}=splitStaleConsumersV1(index,selection.classes,stale);
 let round=0;const root=mkdtempSync(join(tmpdir(),"da-")),moved=new Set<string>(),asked:string[]=[],hold=async(rows:readonly{path:string}[])=>rows.every(row=>!moved.has(row.path));
 try{
  const open=()=>createDurableAnswersV1({root,fs:createNodeFilePortV1(),lease:createFileLeasePortV1(join(root,"l")),resource:"facts"});
  const derive=async(row:{path:string;fact:unknown})=>{asked.push(row.path);await new Promise(done=>setTimeout(done,5));return{fact:row.fact,rows:[raw("r"+round+"-"+row.path)]};};
  const first=await settleSuspectsV1({index,suspects,derive,memo:open(),hold} as never);
  expect(asked.length).toBe(suspects.size);expect(first.reused).toBe(0);
  const resumed=await settleSuspectsV1({index,suspects,derive,memo:open(),hold} as never);
  expect(asked.length,"a new process asks nothing the old one answered").toBe(suspects.size);expect(resumed.reused).toBe(suspects.size);expect([...resumed.held.keys()].sort()).toEqual([...first.held.keys()].sort());
  for(const path of asked)moved.add("r0-"+path);round=1;
  const peers=await Promise.all([settleSuspectsV1({index,suspects,derive,memo:open(),hold} as never),settleSuspectsV1({index,suspects,derive,memo:open(),hold} as never)]);
  expect(asked.length,"two concurrent plans derive each question once").toBe(2*suspects.size);
  expect(peers[0]!.reused+peers[1]!.reused).toBe(suspects.size);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("rows that closures share are checked once per verification, and one failing shared row refuses exactly the closures that read it",async()=>{
 const world=worlds.find(row=>row.id==="support stale, same answer: cutoff")!,{index}=indexOf(world),selection=selectFactClassesV1(index);
 const{stale}=await staleFactRowsV1(index,verifiableRowIdsV1(index,selection.ids),async rows=>rows.every(row=>row.path!=="s1")),{suspects}=splitStaleConsumersV1(index,selection.classes,stale);
 const shared=Array.from({length:50},(_,i)=>raw("shared-"+i)),calls:number[]=[],bad=new Set<string>();
 const hold=async(rows:readonly{path:string}[])=>{calls.push(rows.length);return rows.every(row=>!bad.has(row.path));};
 const memo=new Map(),derive=async(row:{path:string;fact:unknown})=>({fact:row.fact,rows:[...shared,raw("own-"+row.path)]});
 await settleSuspectsV1({index,suspects,derive,memo,hold} as never);calls.length=0;
 const again=await settleSuspectsV1({index,suspects,derive,memo,hold} as never);
 expect(again.reused).toBe(suspects.size);expect(calls.reduce((sum,count)=>sum+count,0),"every unique row once").toBeLessThanOrEqual(shared.length+suspects.size);
 bad.add("shared-7");calls.length=0;
 const refused=await settleSuspectsV1({index,suspects,derive,memo,hold} as never);
 expect(refused.reused).toBe(0);expect(calls.length,"the failing row is found by bisection, not by one call per closure").toBeLessThan(40);
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
 const world=worlds.find(row=>row.id==="support stale, same answer: cutoff")!,{index}=indexOf(world),events:{done:number;total:number}[]=[];
 await settleSuspectsV1({index,suspects:new Set(),progress:event=>{events.push(event);}});
 expect(events).toEqual([]);
 const selection=selectFactClassesV1(index),{stale}=await staleFactRowsV1(index,verifiableRowIdsV1(index,selection.ids),async rows=>rows.every(row=>row.path!=="s1")),{suspects}=splitStaleConsumersV1(index,selection.classes,stale);
 await settleSuspectsV1({index,suspects,derive:async row=>({fact:row.fact,rows:[]}),progress:event=>{events.push(event);}});
 expect(events).toEqual([{phase:"verification",done:0,total:1},{phase:"verification",done:1,total:1}] as never);
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
 const first:GenerationRowV1={kind:"derived",path:"q",fact:"answer"},next={...first,fact:"peer"},support=[raw("fresh")],memo=new Map();let calls=0;
 const check=async(row:GenerationRowV1)=>{
  const name=derivedOwnerNameV1(row),index=createFactIndexV1(new Map([["holder",[row]],[name,[row,raw("old")]]]));
  return settleSuspectsV1({index,suspects:new Set([name]),memo,hold:async rows=>rows.every(read=>read.path==="fresh"),derive:async()=>{calls++;return{fact:"answer",rows:support};}});
 };
 const before=await check(first);expect(before.held.get(derivedOwnerNameV1(first))).toEqual([first,...support]);
 const after=await check(next);expect(calls).toBe(1);expect(after.reused).toBe(1);
 expect([...after.changed.values()]).toEqual([next]);expect(after.held.size).toBe(0);
});

for(const bound of fixture.memoBounds)test("complete answer retention bound: "+bound.id,async()=>{
 const world=worlds.find(row=>row.id==="support stale, same answer: cutoff")!,{index}=indexOf(world),row=derivedRow(world,"q1"),name=derivedOwnerNameV1(row),key=canonicalJsonV1([row.kind,row.path]),memo=new Map<string,{fact:unknown;rows:GenerationRowV1[]}>();
 const filler={fact:"filler",rows:[]},total=CUTOFF_POLICY_V1.maximumSuspects-(bound.fill==="below"?1:0);
 for(let at=0;at<total-(bound.known?1:0);at++)memo.set("filler:"+at,filler);
 if(bound.known)memo.set(key,{fact:"old",rows:[raw("stale")]});
 let queries=0;
 const settle=()=>settleSuspectsV1({index,suspects:new Set([name]),memo,hold:async rows=>rows.every(read=>read.path==="fresh"),derive:async()=>{queries++;return{fact:row.fact,rows:[raw("fresh")]};}});
 for(let round=0;round<2;round++){
  const result=await settle(),expected=model(world);
  expect([...result.held.keys()]).toEqual(expected.held.map((question:string)=>derivedOwnerNameV1(derivedRow(world,question))));
  expect(result.changed.size).toBe(expected.changed.length);
  expect(derivedUpsertsV1([],result.held).get(name)).toEqual([row,raw("fresh")]);
 }
 expect(queries).toBe(bound.queries);expect(memo.has(key)).toBe(bound.cached);
 expect(memo.size).toBe(CUTOFF_POLICY_V1.maximumSuspects);expect(memo.get("filler:0")).toBe(filler);
 memo.clear();
});
