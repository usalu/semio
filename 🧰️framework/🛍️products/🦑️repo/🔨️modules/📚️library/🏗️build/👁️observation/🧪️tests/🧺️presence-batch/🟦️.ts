/** 🧺️ Contract: the presence checks of concurrent looks share one storage consumer per batch, a name found present is remembered for the validation round, a refusal reaches only the caller whose names caused it. @see ../../📥️reuse/🧺️coalesce/🟨️.mjs */
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import ts from"typescript";
import{createPresenceCoalescerV1}from"../../📥️reuse/🧺️coalesce/🟨️.mjs";
import withdrawal from"./withdrawal.json"with{type:"json"};
import coalescing from"./coalescing.json"with{type:"json"};
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
 const rows=await Promise.allSettled([[pool[1]!],[bad,pool[2]!],[pool[3]!]].map(names=>coalescer.ask(names,round,new AbortController().signal)));
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

test("one presence boundary serves independently cancellable looks",async()=>{
 const db=new Database(":memory:"),calls:string[][]=[],closing=new AbortController(),remaining=new AbortController(),round={};
 let entered!:()=>void,release!:()=>void,group:AbortSignal|undefined;
 const started=new Promise<void>(resolve=>{entered=resolve;}),held=new Promise<void>(resolve=>{release=resolve;});
 const coalescer=createPresenceCoalescerV1({run:async(names,signal)=>{
  calls.push([...names]);if(calls.length===1){group=signal;entered();await held;}
  return new Set();
 }});
 const first=coalescer.ask(coalescing.mixed[0]!,round,closing.signal);
 const second=coalescer.ask(coalescing.mixed[1]!,round,remaining.signal);
 try{
  await started;
  closing.abort();
  expect(group?.aborted).toBe(false);
  release();
  expect(await Promise.all([first,second])).toEqual([false,true]);
  const oracle=db.query(`select count(distinct name.value) as names from
   json_each(?1) caller, json_each(caller.value) name`).get(JSON.stringify(coalescing.mixed)) as {names:number};
  expect(calls).toHaveLength(1);
  expect(calls[0]).toHaveLength(oracle.names);
 }finally{release();await Promise.allSettled([first,second]);db.close();}
});

test("overlapping presence questions share the batch when their distinct names fit",async()=>{
 const db=new Database(":memory:"),calls:string[][]=[],round={};
 const expected=db.query(`select distinct name.value as name from
   json_each(?1) caller, json_each(caller.value) name order by name`)
  .all(JSON.stringify(coalescing.mixed)) as {name:string}[];
 const coalescer=createPresenceCoalescerV1({maximumNames:expected.length,run:async names=>{
  calls.push([...names]);
  return new Set();
 }});
 try{
  expect(await Promise.all(coalescing.mixed.map(names=>coalescer.ask(names,round)))).toEqual([true,true]);
  expect(calls).toEqual([expected.map(row=>row.name)]);
 }finally{db.close();}
});

test("the next queued presence batch reuses names established by its own validation round",async()=>{
 const db=new Database(":memory:"),calls:string[][]=[],round={};
 let entered!:()=>void,release!:()=>void;
 const started=new Promise<void>(resolve=>{entered=resolve;}),held=new Promise<void>(resolve=>{release=resolve;});
 const coalescer=createPresenceCoalescerV1({run:async names=>{calls.push([...names]);if(calls.length===1){entered();await held;}return new Set();}});
 const first=coalescer.ask(coalescing.queued[0]!,round);
 let next:Promise<boolean>|undefined;
 try{
  await started;
  next=coalescer.ask(coalescing.queued[1]!,round);
  release();
  expect(await Promise.all([first,next])).toEqual([true,true]);
  const oracle=db.query(`select value as name from json_each(?1)
   except select value as name from json_each(?2) order by name`)
   .all(JSON.stringify(coalescing.queued[1]),JSON.stringify(coalescing.queued[0])) as {name:string}[];
  expect(calls).toEqual([coalescing.queued[0],oracle.map(row=>row.name)]);
 }finally{release();await Promise.allSettled([first,next]);db.close();}
});

test("when every mixed look withdraws, a subsequent batch can still finish",async()=>{
 const controllers=coalescing.mixed.map(()=>new AbortController()),round={};
 let entered!:()=>void,release!:()=>void,group:AbortSignal|undefined,calls=0;
 const started=new Promise<void>(resolve=>{entered=resolve;}),held=new Promise<void>(resolve=>{release=resolve;});
 const coalescer=createPresenceCoalescerV1({run:async(_names,signal)=>{
  if(++calls===1){group=signal;entered();await held;}
  return new Set();
 }});
 const pending=coalescing.mixed.map((names,index)=>coalescer.ask(names,round,controllers[index]!.signal));
 try{
  await started;
  controllers[0]!.abort();
  expect(group?.aborted).toBe(false);
  controllers[1]!.abort();
  expect(group?.aborted).toBe(true);
  const later=coalescer.ask(coalescing.queued[0]!,round);
  release();
  expect(await Promise.all(pending)).toEqual([false,false]);
  expect(await later).toBe(true);
  expect(calls).toBe(2);
 }finally{release();await Promise.allSettled(pending);}
});

test("the actual check runs the names of a batch under one storage consumer with its guards",()=>{
 const source=readFileSync(new URL("../../📥️reuse/🟨️.mjs",import.meta.url),"utf8"),tree=ts.createSourceFile("reuse.mjs",source,ts.ScriptTarget.Latest,true),functions=new Map(tree.statements.filter(ts.isFunctionDeclaration).map(row=>[row.name?.text,row.body?.getText(tree)??""]));
 const check=functions.get("missingInventoryNamesV1")??"",entry=functions.get("inferenceInventoryReferencesHoldV1")??"";
 expect(check).toContain("held!==undefined?await body(held.store,held.actual):await withRepositoryBuildGraphConsumerV1(");expect(check.indexOf("const body=")).toBeLessThan(check.indexOf("inventoryRecordStillHoldsV1("));
 const session=functions.get("presenceSessionV1")??"";expect(session).toContain("withRepositoryBuildGraphConsumerV1(");expect(session).toContain("Object.freeze({store,actual})");expect(session).toContain("if(state.held===mine)state.held=undefined");
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

test("batches that follow each other run inside one session, a quiet linger ends it, a failing session refuses the waiting callers",async()=>{
 let sessions=0,inside=0,batches=0;
 const coalescer=createPresenceCoalescerV1({lingerMilliseconds:40,maximumSessionMilliseconds:10000,session:async work=>{sessions++;inside++;try{return await work();}finally{inside--;}},run:async names=>{expect(inside).toBe(1);batches++;await new Promise(done=>setTimeout(done,2));return new Set();}});
 for(let wave=0;wave<5;wave++){expect(await coalescer.ask([pool[wave]!],{})).toBe(true);await new Promise(done=>setTimeout(done,5));}
 expect(sessions).toBe(1);expect(batches).toBe(5);
 await new Promise(done=>setTimeout(done,120));
 expect(await coalescer.ask([pool[9]!],{})).toBe(true);expect(sessions).toBe(2);
 const failing=createPresenceCoalescerV1({session:async()=>{throw Error("custody refused");},run:async()=>new Set()});
 const rows=await Promise.allSettled([failing.ask([pool[1]!],{}),failing.ask([pool[2]!],{})]);
 expect(rows.map(row=>row.status)).toEqual(["rejected","rejected"]);
 expect((rows[0] as PromiseRejectedResult).reason.message).toBe("custody refused");
});

test("a session ends at its maximum even while asks keep coming",async()=>{
 let sessions=0,clock=0;
 const coalescer=createPresenceCoalescerV1({lingerMilliseconds:20,maximumSessionMilliseconds:100,now:()=>clock,linger:async()=>{clock+=60;},session:async work=>{sessions++;return work();},run:async()=>{clock+=10;return new Set();}});
 const asks=[];for(let at=0;at<4;at++){asks.push(coalescer.ask([pool[at]!],{}));}
 await Promise.all(asks);
 expect(sessions).toBe(1);
 await Promise.all([coalescer.ask([pool[20]!],{})]);
 expect(sessions).toBe(2);
});

test("a session that costs more to open than its maximum lasts a multiple of its measured overhead, so a take pass stops paying the open cost every 20 s",async()=>{
 const simulate=async(factor:number)=>{
  let sessions=0,clock=0;
  const coalescer=createPresenceCoalescerV1({maximumNames:1,lingerMilliseconds:20,maximumSessionMilliseconds:20000,sessionOverheadFactor:factor,sessionCeilingMilliseconds:120000,now:()=>clock,linger:async()=>{},session:async work=>{sessions++;clock+=6000;try{return await work();}finally{clock+=10000;}},run:async()=>{clock+=1000;return new Set();}});
  const names=Array.from({length:100},(_,at)=>pool[at]!);
  for(const name of names)expect(await coalescer.ask([name],{})).toBe(true);
  return{sessions,milliseconds:clock};
 };
 const before=await simulate(0),after=await simulate(4);
 console.log("[bench] presence sessions for 100 batches at 16 s open+close, 1 s per batch: before "+JSON.stringify(before)+" after "+JSON.stringify(after));
 expect(before.sessions).toBeGreaterThanOrEqual(5);
 expect(after.sessions).toBeLessThanOrEqual(3);
 expect(after.milliseconds).toBeLessThan(before.milliseconds*0.8);
});

test("the shipped session policy lengthens a session only in proportion to its overhead and never past the ceiling",()=>{
 const policy=timingSchema.definitions.Policy.const as {sessionMaximumMilliseconds:number;sessionOverheadFactor:number;sessionCeilingMilliseconds:number};
 expect(policy.sessionOverheadFactor).toBeGreaterThan(0);
 expect(policy.sessionCeilingMilliseconds).toBeGreaterThanOrEqual(policy.sessionMaximumMilliseconds);
});

const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const settleSoon=async(check:()=>boolean,milliseconds:number)=>{const end=Date.now()+milliseconds;while(!check()&&Date.now()<end)await sleep(2);return check();};

test("the next ask is served by a new session while the previous session's tail still verifies",async()=>{
 let release!:()=>void;const tail=new Promise<void>(done=>{release=done;}),opened:number[]=[];
 const coalescer=createPresenceCoalescerV1({run:async names=>new Set<string>(),session:async work=>{opened.push(opened.length);await work();if(opened.length===1)await tail;}});
 expect(await coalescer.ask(["a"],{})).toBe(true);
 const second=coalescer.ask(["b"],{});
 expect(await Promise.race([second,sleep(1000).then(()=>"stalled")]),"the second ask waited for the first session's tail").toBe(true);
 expect(opened.length).toBe(2);
 release();
});

test("a failing tail rejects nobody who is waiting for another session and tails are bounded",async()=>{
 let failTail!:()=>void;const gate=new Promise<void>((_,no)=>{failTail=()=>no(Error("post verify refused"));});gate.catch(()=>{});
 const coalescer=createPresenceCoalescerV1({run:async()=>new Set<string>(),session:async work=>{await work();await gate;}});
 expect(await coalescer.ask(["a"],{})).toBe(true);
 const second=coalescer.ask(["b"],{});
 expect(await second).toBe(true);
 failTail();await sleep(20);
 expect(await coalescer.ask(["c"],{})).toBe(true);
 let free!:()=>void;const hold=new Promise<void>(done=>{free=done;});let sessions=0;
 const bounded=createPresenceCoalescerV1({maximumTails:1,run:async()=>new Set<string>(),session:async work=>{sessions++;await work();await hold;}});
 expect(await bounded.ask(["a"],{})).toBe(true);
 expect(await bounded.ask(["b"],{})).toBe(true);
 const third=bounded.ask(["c"],{});
 expect(await Promise.race([third,sleep(150).then(()=>"queued")]),"one tail is allowed, the next session keeps the queue").toBe("queued");
 expect(sessions).toBe(2);
 free();expect(await third).toBe(true);
});

test("a take pass of 600 owners finishes in bounded time while a stub exclusive mapping holder cycles, and overlapping tails beat the serial session",async()=>{
 const period=80,heldFor=40,pre=5,start=performance.now();
 const grant=async()=>{for(;;){const phase=(performance.now()-start)%period;if(phase>=heldFor)return;await sleep(heldFor-phase);}};
 const pass=async(maximumTails:number)=>{
  const coalescer=createPresenceCoalescerV1({maximumTails,lingerMilliseconds:0,run:async names=>new Set<string>(),session:async work=>{await sleep(pre);await work();await grant();await sleep(1);}});
  const began=performance.now();
  await Promise.all(Array.from({length:8},async(_,lane)=>{for(let at=lane;at<600;at+=8)expect(await coalescer.ask(["owner-"+at],{})).toBe(true);}));
  return performance.now()-began;
 };
 const serial=await pass(0),overlapped=await pass(3);
 console.log("[bench] 600 owners, stub exclusive holder 40 of every 80 ms: serial tail "+Math.round(serial)+" ms, overlapped tails "+Math.round(overlapped)+" ms");
 expect(overlapped).toBeLessThan(serial*0.7);
 expect(overlapped).toBeLessThan(6000);
 expect(await settleSoon(()=>true,1)).toBe(true);
});
