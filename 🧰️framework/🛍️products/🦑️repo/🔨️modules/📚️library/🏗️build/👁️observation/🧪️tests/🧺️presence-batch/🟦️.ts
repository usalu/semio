/** 🧺️ Contract: the presence checks of concurrent looks share one storage consumer per batch, a name found present is remembered for the validation round, a refusal reaches only the caller whose names caused it. @see ../../📥️reuse/🧺️coalesce/🟨️.mjs */
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import ts from"typescript";
import{createPresenceCoalescerV1}from"../../📥️reuse/🧺️coalesce/🟨️.mjs";
import withdrawal from"./withdrawal.json"with{type:"json"};
import Ajv from"ajv";
import timingCorpus from"./timing.json"with{type:"json"};
import timingSchema from"../../📥️reuse/🧬️schema/🔣️.json"with{type:"json"};
import{createInferenceInventoryTimingsV1}from"../../📥️reuse/⏱️timing/🟨️.mjs";

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

test("withdrawing a presence look drains a started batch and preserves a different signal's reader",async()=>{
 const db=new Database(":memory:");
 try{
  for(const row of withdrawal){
   let start!:()=>void,release!:()=>void,settled=false;
   const started=new Promise<void>(resolve=>{start=resolve;}),drain=new Promise<void>(resolve=>{release=resolve;});
   const closing=new AbortController(),remaining=new AbortController(),round={},signals:(AbortSignal|undefined)[]=[];
   const coalescer=createPresenceCoalescerV1({run:async(_names,signal)=>{
    signals.push(signal);
    if(signal===closing.signal){start();await drain;}
    return new Set();
   }});
   const look=coalescer.ask(["held"],round,closing.signal).finally(()=>{settled=true;});
   if(row.phase==="active")await started;
   const retained=coalescer.ask(["held"],round,remaining.signal);
   closing.abort();
   try{
    if(row.entered){await Promise.resolve();expect(settled,row.phase).toBe(false);}
    else expect(await look,row.phase).toBe(false);
   }finally{release();}
   expect(await look,row.phase).toBe(false);
   expect(await retained,row.phase).toBe(true);
   const oracle=db.query("select ?1 = 'active' as entered").get(row.phase)as{entered:number};
   expect(signals.includes(closing.signal),row.phase).toBe(row.entered);
   expect(signals.includes(closing.signal),row.phase).toBe(Boolean(oracle.entered));
   expect(signals.at(-1),row.phase).toBe(remaining.signal);
  }
 }finally{db.close();}
});

test("the actual check runs the names of a batch under one storage consumer with its guards",()=>{
 const source=readFileSync(new URL("../../📥️reuse/🟨️.mjs",import.meta.url),"utf8"),tree=ts.createSourceFile("reuse.mjs",source,ts.ScriptTarget.Latest,true),functions=new Map(tree.statements.filter(ts.isFunctionDeclaration).map(row=>[row.name?.text,row.body?.getText(tree)??""]));
 const check=functions.get("missingInventoryNamesV1")??"",entry=functions.get("inferenceInventoryReferencesHoldV1")??"";
 expect(check.indexOf("withRepositoryBuildGraphConsumerV1(")).toBeLessThan(check.indexOf("inventoryRecordStillHoldsV1("));
 expect(check).toContain("await store.recheck()");expect(check).toContain("Inference inventory leaf changed before delivery");
 expect(check.lastIndexOf("requireCurrentInferenceInventoryReuseV1()")).toBeGreaterThan(check.indexOf("Inference inventory leaf changed before delivery"));
 expect(entry).toContain("requireIssuedResourceControlV1(control)");expect(entry).toContain("presenceCoalescerV1(root,control).ask(names,observationRoundV1(),withdraw)");expect(entry).not.toContain("withRepositoryBuildGraphConsumerV1(");
});

/** ⏱️ Checks overlapping, withdrawn, and saturating inventory read phases against SQLite. */
test("inventory reader phase counters track live and settled work",()=>{
 const phases=["reader","preVerify","body","postVerify"],db=new Database(":memory:"),limit=timingSchema.definitions.TimingNumber.maximum;
 const ajv=new Ajv({strict:true});ajv.addSchema(timingSchema);
 const validate=ajv.getSchema(timingSchema.$id+"#/definitions/ConsumerTimings")!;
 const source=timingCorpus.batches.flat(),events=timingCorpus.batches.flatMap((batch,index)=>[...batch.map(row=>({at:row.start,index,phase:row.phase})),{at:batch.at(-1)!.end,index,phase:undefined}]).sort((a,b)=>a.at-b.at);
 let now=0,event=0;
 const meter=createInferenceInventoryTimingsV1(()=>now),batches=timingCorpus.batches.map(()=>meter.batch());
 try{for(const at of timingCorpus.samples){
  while(events[event]&&events[event]!.at<=at){const row=events[event++]!;now=row.at;batches[row.index]!(row.phase);}
  now=at;
  const actual=meter.stats();
  expect(validate(actual)).toBe(true);
  const expected=db.query<{phase:string;started:number;active:number;milliseconds:number},[string,string,number,number]>(`select p.value phase,count(s.value) started,
    count(s.value) filter (where json_extract(s.value,'$.end')>?3) active,
    min(?4,coalesce(sum(min(?3,json_extract(s.value,'$.end'))-json_extract(s.value,'$.start')),0)) milliseconds
    from json_each(?1) p left join json_each(?2) s on p.value=json_extract(s.value,'$.phase') and json_extract(s.value,'$.start')<=?3 group by p.value`).all(JSON.stringify(phases),JSON.stringify(source),at,limit);
  expect(actual).toEqual(Object.fromEntries(expected.map(({phase,...row})=>[phase,row])));
 }}finally{db.close();}
});

test("inventory body timings retain an in-flight operation and settle only its elapsed interval",async()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(timingSchema);
 const validate=ajv.getSchema(timingSchema.$id+"#/definitions/BodyTimings")!;
 let now=0;
 const meter=createInferenceInventoryTimingsV1(()=>now);
 expect(meter.bodyOperations()).toBeUndefined();
 const stopRecheck=meter.body("storeRecheck");
 now=30;const stopContinuation=meter.body("continuation");
 now=50;stopRecheck();
 now=65;
 expect(meter.bodyOperations()).toEqual({storeRecheck:{started:1,active:0,milliseconds:50},continuation:{started:1,active:1,milliseconds:35}});
 stopContinuation();
 now=100;
 const settled=meter.bodyOperations();
 expect(validate(settled)).toBe(true);
 expect(settled).toEqual({storeRecheck:{started:1,active:0,milliseconds:50},continuation:{started:1,active:0,milliseconds:35}});
});
