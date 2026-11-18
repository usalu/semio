/** 🧺️ Contract: the presence checks of concurrent looks share one storage consumer per batch, a name found present is remembered for the validation round, a refusal reaches only the caller whose names caused it. @see ../../📥️reuse/🧺️coalesce/🟨️.mjs */
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import ts from"typescript";
import{createPresenceCoalescerV1}from"../../📥️reuse/🧺️coalesce/🟨️.mjs";

const pool=Array.from({length:200},(_,at)=>"SEMIO_SOURCE_CLOSURE_"+at.toString(16).padStart(64,"0"));
const callers=Array.from({length:48},(_,at)=>[pool[at*3%200]!,pool[(at*7+1)%200]!,pool[(at*11+2)%200]!]);
const sqlAnswer=(db:Database,present:ReadonlySet<string>,names:readonly string[])=>db.query<{ok:number},[string,string]>("select not exists(select 1 from json_each(?1) n where n.value not in (select value from json_each(?2))) ok").get(JSON.stringify(names),JSON.stringify([...present]))?.ok===1;

test("concurrent callers are answered by a few batches that check each distinct name once",async()=>{
 const present=new Set(pool.filter((_,at)=>at%9!==4)),db=new Database(":memory:"),calls:string[][]=[];
 try{
  const coalescer=createPresenceCoalescerV1({run:async names=>{calls.push([...names]);await new Promise(done=>setTimeout(done,5));return new Set(names.filter(name=>!present.has(name)));}});
  const round={},answers=await Promise.all(callers.map(names=>coalescer.ask(names,round)));
  answers.forEach((answer,at)=>{expect(answer).toBe(callers[at]!.every(name=>present.has(name)));expect(answer).toBe(sqlAnswer(db,present,callers[at]!));});
  expect(answers.some(Boolean)&&answers.some(answer=>!answer)).toBe(true);
  const distinct=new Set(callers.flat());
  expect(calls.length).toBeLessThanOrEqual(2);
  expect(calls.flat().length).toBeLessThanOrEqual(distinct.size);
  expect(new Set(calls.flat())).toEqual(distinct);
  expect(coalescer.stats().batches).toBe(calls.length);
  expect(callers.length/calls.length).toBeGreaterThanOrEqual(24);
 }finally{db.close();}
});

test("a name found present is remembered for its round only, a missing name is asked again",async()=>{
 const present=new Set(pool.slice(0,50)),calls:string[][]=[],coalescer=createPresenceCoalescerV1({run:async names=>{calls.push([...names]);return new Set(names.filter(name=>!present.has(name)));}}),round={};
 expect(await coalescer.ask(pool.slice(0,3),round)).toBe(true);expect(calls.length).toBe(1);
 expect(await coalescer.ask(pool.slice(1,3),round)).toBe(true);expect(calls.length).toBe(1);expect(coalescer.stats().remembered).toBe(1);
 expect(await coalescer.ask([pool[0]!,pool[60]!],round)).toBe(false);expect(calls.at(-1)).toEqual([pool[60]!]);
 present.add(pool[60]!);expect(await coalescer.ask([pool[60]!],round)).toBe(true);
 const before=calls.length;expect(await coalescer.ask(pool.slice(0,3),{})).toBe(true);expect(calls.length).toBe(before+1);
 expect(await coalescer.ask(pool.slice(0,3))).toBe(true);expect(calls.length).toBe(before+2);
});

test("a refusal reaches only the caller whose names caused it",async()=>{
 const bad=pool[7]!,coalescer=createPresenceCoalescerV1({run:async names=>{if(names.includes(bad))throw Error("refused "+bad);return new Set();}}),round={};
 const rows=await Promise.allSettled([coalescer.ask([pool[1]!],round),coalescer.ask([bad,pool[2]!],round),coalescer.ask([pool[3]!],round)]);
 expect(rows.map(row=>row.status)).toEqual(["fulfilled","rejected","fulfilled"]);
 expect((rows[1] as PromiseRejectedResult).reason.message).toContain("refused");
 expect(coalescer.stats().retried).toBe(3);
});

test("a batch holds at most the named number of names unless one caller needs more",async()=>{
 const sizes:number[]=[],coalescer=createPresenceCoalescerV1({maximumNames:4,run:async names=>{sizes.push(names.length);return new Set();}});
 await Promise.all([[0,1,2],[3,4,5],[6,7,8],[9,10,11,12,13,14]].map(list=>coalescer.ask(list.map(at=>pool[at]!),{})));
 expect(sizes.every(size=>size<=4||size===6)).toBe(true);expect(sizes.reduce((sum,size)=>sum+size,0)).toBe(15);
});

test("the actual check runs the names of a batch under one storage consumer with its guards",()=>{
 const source=readFileSync(new URL("../../📥️reuse/🟨️.mjs",import.meta.url),"utf8"),tree=ts.createSourceFile("reuse.mjs",source,ts.ScriptTarget.Latest,true),functions=new Map(tree.statements.filter(ts.isFunctionDeclaration).map(row=>[row.name?.text,row.body?.getText(tree)??""]));
 const check=functions.get("missingInventoryNamesV1")??"",entry=functions.get("inferenceInventoryReferencesHoldV1")??"";
 expect(check.indexOf("withRepositoryBuildGraphConsumerV1(")).toBeLessThan(check.indexOf("inventoryRecordStillHoldsV1("));
 expect(check).toContain("await store.recheck()");expect(check).toContain("Inference inventory leaf changed before delivery");
 expect(check.lastIndexOf("requireCurrentInferenceInventoryReuseV1()")).toBeGreaterThan(check.indexOf("Inference inventory leaf changed before delivery"));
 expect(entry).toContain("requireIssuedResourceControlV1(control)");expect(entry).toContain("presenceCoalescerV1(root,control).ask(names,observationRoundV1())");expect(entry).not.toContain("withRepositoryBuildGraphConsumerV1(");
});
