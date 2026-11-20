/** ⏳️ Contract: a reader waits for another machine's claim on an owner only while the holder's lease is live, only while it has nothing else to infer, never longer than the lease time since it first saw the owner held, and a large machine never waits for a smaller one.
 * @see ../../🛰️exchange/🟦️.ts */
import{expect,test}from"bun:test";
import{getEventListeners}from"node:events";
import waits from "./⏳️waits.json" with {type:"json"};
import{decideGraphEntryV1,graphFleetWaitingV1,graphHeldVerdictV1,type GraphFleetClientV1}from"../../🛰️exchange/🟦️.ts";
import{GRAPH_STATE_POLICY_V1 as policy,type GraphStateV1}from"../../🟦️.ts";

const hex=(seed:string)=>seed.repeat(64).slice(0,64);
const stateOf=(seed:string):GraphStateV1=>({version:1,frame:{kind:"project-inference",tooling:hex("a"),implementation:hex("b"),platform:"neutral",extra:{}},names:hex("c"),content:hex("d"),files:0,contentFiles:0,digest:hex(seed)}) as unknown as GraphStateV1;
type Script=readonly Readonly<{state:"held";by:string;untilMilliseconds:number}|{state:"granted";untilMilliseconds:number}>[];
function fake(script:Script,awaited:number[]=[]):GraphFleetClientV1&{claims:number}{
 let at=0;
 const client={claims:0,settings:{endpoint:"http://127.0.0.1:1",instance:"main",token:undefined,upload:true,stateDirectory:"/nonexistent"},usable:()=>true,claimsUsable:()=>true,getResult:async()=>"miss" as const,getBlob:async()=>undefined,hasBlob:async()=>false,putBlob:async()=>({stored:true as const}),putResult:async()=>({stored:true as const}),
  claim:async()=>{client.claims++;return script[Math.min(at++,script.length-1)]!;},renew:async()=>"granted" as const,release:async()=>undefined,await:async(_key:string,timeout:number)=>{awaited.push(timeout);return "timeout" as const;}};
 return client as unknown as GraphFleetClientV1&{claims:number};
}
const held=(untilMilliseconds:number)=>({state:"held" as const,by:"g1-m05|DESKTOP:1:abc",untilMilliseconds});

test("a lease that lapsed is no reason to wait, a live one is",()=>{
 const base={now:1000,sinceMilliseconds:1000,busy:false,strong:false};
 expect(graphHeldVerdictV1({...base,untilMilliseconds:900})).toBe("lapsed");
 expect(graphHeldVerdictV1({...base,untilMilliseconds:1000})).toBe("lapsed");
 expect(graphHeldVerdictV1({...base,untilMilliseconds:5000})).toBe("wait");
});

test("a reader that has other owners to infer leaves a held owner and comes back to it",()=>{
 expect(graphHeldVerdictV1({now:1000,untilMilliseconds:5000,sinceMilliseconds:1000,busy:true,strong:false})).toBe("skip");
});

test("a large machine never waits for a smaller one, busy or not",()=>{
 for(const busy of[true,false])expect(graphHeldVerdictV1({now:1000,untilMilliseconds:5000,sinceMilliseconds:1000,busy,strong:true})).toBe("compute");
});

test("an owner held for the lease time by live peers is taken over by the reader that has nothing else, even while the holder keeps renewing",()=>{
 const since=1000,take=policy.leaseTakeOverMilliseconds;
 expect(graphHeldVerdictV1({now:since+take-1,untilMilliseconds:since+take+30000,sinceMilliseconds:since,busy:false,strong:false})).toBe("wait");
 expect(graphHeldVerdictV1({now:since+take,untilMilliseconds:since+take+30000,sinceMilliseconds:since,busy:false,strong:false})).toBe("compute");
 expect(graphHeldVerdictV1({now:since+take,untilMilliseconds:since+take+30000,sinceMilliseconds:since,busy:true,strong:false})).toBe("compute");
});

test("the lease time is declared and far shorter than the old wait of fifteen minutes",()=>{
 expect(policy.leaseTakeOverMilliseconds).toBeGreaterThan(0);
 expect(policy.leaseTakeOverMilliseconds).toBeLessThanOrEqual(policy.claimWaitMaximumMilliseconds/10);
});

test("a held claim whose lease lapsed is claimed again without waiting for the holder",async()=>{
 const awaited:number[]=[],client=fake([held(Date.now()-5000),{state:"granted",untilMilliseconds:Date.now()+30000}],awaited);
 const decision=await decideGraphEntryV1({client,state:stateOf("1"),owner:"me"});
 expect(decision.outcome).toBe("compute");
 expect(awaited).toEqual([]);
 expect(client.claims).toBe(2);
});

test("a busy reader skips an owner held by a live peer at once and never waits",async()=>{
 const awaited:number[]=[],client=fake([held(Date.now()+30000)],awaited),began=Date.now();
 const decision=await decideGraphEntryV1({client,state:stateOf("2"),owner:"me",busy:()=>true});
 expect(decision.outcome).toBe("skip");
 expect(awaited).toEqual([]);
 expect(Date.now()-began).toBeLessThan(500);
});

test("an idle reader waits for a live holder at most the lease time, not the old fifteen minutes",async()=>{
 const awaited:number[]=[],client=fake([held(Date.now()+30000)],awaited);
 const decision=await decideGraphEntryV1({client,state:stateOf("3"),owner:"me",busy:()=>false});
 expect(decision.outcome).toBe("compute");
 expect(awaited.length).toBeGreaterThan(0);
 expect(Math.max(...awaited)).toBeLessThanOrEqual(policy.leaseTakeOverMilliseconds);
});

test("a large machine computes at once and never asks the holder",async()=>{
 const awaited:number[]=[],client=fake([held(Date.now()+30000)],awaited);
 const decision=await decideGraphEntryV1({client,state:stateOf("4"),owner:"me",strong:true,busy:()=>false});
 expect(decision.outcome).toBe("compute");
 if(decision.outcome==="compute")expect(decision.reason).toBe("strong");
 expect(awaited).toEqual([]);
});

test("the owners a reader waits for are listed with the machine that holds them and removed when the wait ends",async()=>{
 let seen=0;
 const awaited:number[]=[],client=fake([held(Date.now()+30000)],awaited);
 const original=client.await.bind(client);
 (client as unknown as {await:typeof client.await}).await=async(key,timeout)=>{seen=graphFleetWaitingV1().length;return original(key,timeout);};
 await decideGraphEntryV1({client,state:stateOf("5"),owner:"me",busy:()=>false});
 expect(seen).toBe(1);
 expect(graphFleetWaitingV1().length).toBe(0);
});

test("an owner skipped as held keeps its first sight, so the oldest-held owner is the first one taken over",async()=>{
 const started=1000,later=started+policy.leaseTakeOverMilliseconds+1,client=fake([held(later+30000)]);
 const first=await decideGraphEntryV1({client,state:stateOf("6"),owner:"me",busy:()=>true,now:()=>started});
 expect(first.outcome).toBe("skip");
 const second=await decideGraphEntryV1({client,state:stateOf("6"),owner:"me",busy:()=>true,now:()=>later});
 expect(second.outcome).toBe("compute");
});


for(const [caseIndex,row] of waits.cases.entries())test(row.name,async()=>{
 const controller=new AbortController(),signal=controller.signal,initial=()=>{};
 signal.addEventListener("abort",initial);
 expect(getEventListeners(signal,"abort")).toHaveLength(waits.baselineListeners);
 for(let repeat=0;repeat<row.repetitions;repeat++){
  const client=fake([held(row.lease==="busy"?2000:999),{state:"granted",untilMilliseconds:2000}]);
  let listenersDuring:number|undefined;
  const decision=decideGraphEntryV1({client,state:stateOf((7+caseIndex*3+repeat).toString(16).padStart(2,"0")),owner:"me",now:()=>1000,signal,busy:()=>{
   if(row.abort==="before")controller.abort();
   queueMicrotask(()=>{listenersDuring=getEventListeners(signal,"abort").length;if(row.abort==="during")controller.abort();});
   return row.lease==="busy";
  }});
  if(row.abort==="before"){
   const nextTurn=new Promise(resolve=>setImmediate(()=>resolve("pending")));
   expect(await Promise.race([decision,nextTurn])).not.toBe("pending");
  }
  const result=await decision;
  if(result.outcome==="compute")result.hold?.stop();
  expect({outcome:result.outcome,reason:result.outcome==="hit"?null:result.reason,claims:client.claims,listenersDuring,listenersAfter:getEventListeners(signal,"abort").length}).toEqual(row.expected);
  expect(getEventListeners(signal,"abort")).toEqual([initial]);
 }
});
