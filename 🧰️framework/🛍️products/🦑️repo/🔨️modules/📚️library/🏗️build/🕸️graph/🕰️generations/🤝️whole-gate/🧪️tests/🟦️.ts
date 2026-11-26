/** 🚪️ Contract: of the machines that need the graph of the whole workspace one reads and every other fetches what it sealed; a complete head is fetched at once, an incomplete head is never taken by a waiting machine, a vanished holder is replaced, and every fleet fault ends in a local read.
 * @see ../🟦️.ts */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import capacityCases from "./🔣️.json" with {type:"json"};
import {createHash} from "node:crypto";
import {WHOLE_GATE_POLICY_V1,WHOLE_GATE_TEXTS_V1,wholeGateKeyV1,wholeGateTextV1,wholeReadGateV1,type WholeGateInputV1} from "../🟦️.ts";
import {makeFleet} from "../../../📥️load/🤝️selected-exchange/🧪️tests/🌍️world/🟦️.ts";
import type {GenerationLookV1} from "../../🌐️remote-exchange/🟦️.ts";

const identity=createHash("sha256").update("registry").digest("hex"),platform="darwin-arm64";
const none:GenerationLookV1={state:"none",ahead:false,held:0},complete:GenerationLookV1={state:"complete",ahead:true,held:0,owners:20,stale:0,ageSeconds:120},partial:GenerationLookV1={state:"partial",ahead:true,held:0,owners:12,stale:8};
const quick={claimTtlMilliseconds:400,claimHeartbeatMilliseconds:60,waitSliceMilliseconds:40,waitMaximumMilliseconds:5000};
const gate=(fleet:ReturnType<typeof makeFleet>,who:string,look:()=>Promise<GenerationLookV1>,extra:Partial<WholeGateInputV1>={})=>wholeReadGateV1({client:fleet.client(who),identity,platform,owner:who,language:"en",look,...quick,...extra});

test("the key names the registry identity and the platform and nothing of a machine",()=>{
 const key=wholeGateKeyV1({identity,platform});
 expect(key).toBe(createHash("sha256").update(JSON.stringify([WHOLE_GATE_POLICY_V1.domain,identity,platform])).digest("hex"));
 expect(wholeGateKeyV1({identity,platform:"linux-x64"})).not.toBe(key);
 expect(wholeGateKeyV1({identity:createHash("sha256").update("other").digest("hex"),platform})).not.toBe(key);
});

test("every text exists in English and German with the same fields, and the lease outlives three heartbeats",()=>{
 const fields=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(row=>row[1]).sort();
 for(const[key,row]of Object.entries(WHOLE_GATE_TEXTS_V1)){expect(row.en.length).toBeGreaterThan(10);expect(row.de.length).toBeGreaterThan(10);expect(fields(row.de)).toEqual(fields(row.en));expect(key).toMatch(/^[a-zA-Z]+$/u);}
 expect(WHOLE_GATE_POLICY_V1.claimTtlMilliseconds).toBeGreaterThan(3*WHOLE_GATE_POLICY_V1.claimHeartbeatMilliseconds);
 expect(wholeGateTextV1("de","waiting",{by:"m01",seconds:3,limit:720,progress:""})).toContain("m01");
});

test("a complete head that a restore would take is fetched at once, without a claim",async()=>{
 const fleet=makeFleet(),said:string[]=[];
 const outcome=await gate(fleet,"b",async()=>complete,{say:line=>said.push(line)});
 expect(outcome.outcome).toBe("fetch");expect(fleet.log.filter(row=>row.includes("claim"))).toEqual([]);
 expect(said).toEqual([wholeGateTextV1("en","fetching",{owners:20,age:2})]);
});

test("a machine that already holds owners still fetches a newer complete head for the rest instead of skipping it",async()=>{
 const fleet=makeFleet(),outcome=await gate(fleet,"b",async()=>({...complete,held:7}));
 expect(outcome.outcome).toBe("fetch");expect(fleet.log.filter(row=>row.includes("claim"))).toEqual([]);
});

test("a machine that holds a generation of its own reads the difference alone and takes no claim",async()=>{
 const fleet=makeFleet(),outcome=await gate(fleet,"b",async()=>({...none,held:20}));
 expect(outcome.outcome).toBe("skipped");expect(fleet.log.filter(row=>row.includes("claim"))).toEqual([]);
});

test("nine machines start at once with no head: one reads, eight wait and fetch the complete head the reader offered",async()=>{
 const fleet=makeFleet();let head:GenerationLookV1=none;const reads:string[]=[],said=new Map<string,string[]>();
 const machine=async(who:string)=>{
  const lines:string[]=[];said.set(who,lines);
  const outcome=await gate(fleet,who,async()=>head,{say:line=>lines.push(line)});
  if(outcome.outcome==="read"){reads.push(who);await Bun.sleep(300);head=complete;await outcome.release();}
  return outcome.outcome;
 };
 const outcomes=await Promise.all(Array.from({length:9},(_,index)=>machine("m0"+index)));
 expect(reads.length).toBe(1);expect(outcomes.filter(row=>row==="read").length).toBe(1);expect(outcomes.filter(row=>row==="waited").length).toBe(8);
 const waiter=[...said].find(([who])=>who!==reads[0])![1];
 expect(waiter.some(line=>line.includes(reads[0]!)&&line.includes("reading the whole workspace"))).toBe(true);
 expect(waiter.at(-1)).toContain("fetching it instead of reading");
 expect(fleet.claims.size).toBe(0);
});

test("a head its producer knew incomplete is never fetched by a waiting machine; the wave of the reader goes on, the complete head ends the wait",async()=>{
 const fleet=makeFleet();let head:GenerationLookV1=partial;const said:string[]=[];
 const holder=await gate(fleet,"holder",async()=>none);expect(holder.outcome).toBe("read");
 const heartbeat=setInterval(()=>undefined,10);
 const waiting=gate(fleet,"waiter",async()=>head,{say:line=>said.push(line)});
 await Bun.sleep(250);expect(said.some(line=>line.includes("12 projects, 8 still stale"))).toBe(true);
 head=complete;const outcome=await waiting;clearInterval(heartbeat);await holder.release();
 expect(outcome.outcome).toBe("waited");
});

test("a holder that stops renewing is replaced: the next machine takes the claim over and reads",async()=>{
 const fleet=makeFleet(),said:string[]=[];
 fleet.claims.set(wholeGateKeyV1({identity,platform}),{owner:"dead",until:Date.now()+150});
 const outcome=await gate(fleet,"next",async()=>none,{say:line=>said.push(line)});
 expect(outcome.outcome).toBe("taken-over");expect(said.at(-1)).toBe(wholeGateTextV1("en","takenOver"));
 expect(fleet.claims.get(wholeGateKeyV1({identity,platform}))?.owner).toBe("next");
 await outcome.release();expect(fleet.claims.size).toBe(0);
});

test("the heartbeat keeps a long read's claim alive beyond its lease, so a third machine keeps waiting",async()=>{
 const fleet=makeFleet(),holder=await gate(fleet,"holder",async()=>none);
 const waiter=gate(fleet,"waiter",async()=>none,{waitMaximumMilliseconds:1200});
 await Bun.sleep(1000);
 expect(fleet.claims.get(wholeGateKeyV1({identity,platform}))?.owner).toBe("holder");
 expect((await waiter).outcome).toBe("unreachable");await holder.release();
});

test("a broken fleet never fails or hangs a read: down, read-only, a client that raises and the end of the bound all end in a local read; cancelling throws",async()=>{
 const down=makeFleet();down.down=true;
 expect((await gate(down,"a",async()=>none)).outcome).toBe("unreachable");
 const readOnly=makeFleet(),client=readOnly.client("b");(client.settings as {upload:boolean}).upload=false;
 expect((await wholeReadGateV1({client,identity,platform,owner:"b",language:"de",look:async()=>none,...quick})).outcome).toBe("unreachable");
 const raising=makeFleet(),broken=raising.client("c");(broken as unknown as {claim:()=>Promise<never>}).claim=async()=>{throw Error("boom");};
 expect((await wholeReadGateV1({client:broken,identity,platform,owner:"c",language:"en",look:async()=>none,...quick})).outcome).toBe("unreachable");
 const bounded=makeFleet();bounded.claims.set(wholeGateKeyV1({identity,platform}),{owner:"slow",until:Date.now()+60000});
 const said:string[]=[];expect((await gate(bounded,"d",async()=>none,{waitMaximumMilliseconds:120,say:line=>said.push(line)})).outcome).toBe("unreachable");
 expect(said.at(-1)).toContain("waited");
 const controller=new AbortController(),cancelled=gate(bounded,"e",async()=>none,{signal:controller.signal});
 setTimeout(()=>controller.abort(),60);await expect(cancelled).rejects.toBeDefined();
});

test("a look that raises is no head: the machine claims and reads",async()=>{
 const fleet=makeFleet(),outcome=await gate(fleet,"a",async()=>{throw Error("look");});
 expect(outcome.outcome).toBe("read");await outcome.release();
});

test("a machine with less memory than a whole read wants lets the larger machines claim first, fetches their head, and reads only when nobody did",async()=>{
 const fleet=makeFleet();let head:GenerationLookV1=none;const said:string[]=[];
 const small=gate(fleet,"small",async()=>head,{capable:false,smallMachineDelayMilliseconds:600,say:line=>said.push(line)});
 const large=await gate(fleet,"large",async()=>head);expect(large.outcome).toBe("read");
 await Bun.sleep(100);head=complete;await large.release();
 expect((await small).outcome).toBe("waited");expect(said[0]).toContain("less memory");expect(fleet.log.filter(row=>row.startsWith("small claim"))).toEqual([]);
 const alone=makeFleet(),started=Date.now(),outcome=await gate(alone,"small",async()=>none,{capable:false,smallMachineDelayMilliseconds:150});
 expect(outcome.outcome).toBe("read");expect(Date.now()-started).toBeGreaterThanOrEqual(140);await outcome.release();
});

test("a funded loser gives its reservation back before waiting on an existing claim",async()=>{
 const row=capacityCases.contestedClaim,fleet=makeFleet(),winner=await gate(fleet,row.first,async()=>none),client=fleet.client(row.next),db=new Database(":memory:"),events:string[]=[];
 let awaiting!:()=>void,bookings=0,held=0;
 const entered=new Promise<void>(done=>awaiting=done),count:number[]=[];
 const follow=wholeReadGateV1({client:{...client,await:(...args:Parameters<typeof client.await>)=>{
  events.push("await");count.push(held);awaiting();return client.await(...args);
 }},owner:row.next,language:"en",identity,platform,look:async()=>none,...quick,fund:async()=>{
  events.push("book");bookings++;held++;return{release:()=>{events.push("release");held--;}};
 }});
 try{
  await entered;
  expect(count).toEqual([0]);
  expect(bookings).toBe(1);
  await winner.release();
  const next=await follow;
  expect(next.outcome).toBe("taken-over");
  expect(next.funded).toBe(true);
  expect(bookings).toBe(2);
  expect(held).toBe(1);
  await next.release();
  expect(held).toBe(0);
  const result=db.query(`WITH balance AS (
   SELECT value AS event,sum(CASE value WHEN 'book' THEN 1 WHEN 'release' THEN -1 ELSE 0 END) OVER(ORDER BY key) AS reserved FROM json_each(?)
  ) SELECT max(reserved) AS maximumReservations,sum(event='await' AND reserved<>0) AS waitingWithReservation,(SELECT reserved FROM balance LIMIT 1 OFFSET (SELECT count(*) FROM balance)-1) AS finalReservations FROM balance`).get(JSON.stringify(events));
  expect(result).toEqual({maximumReservations:row.maximumReservations,waitingWithReservation:row.waitingWithReservation,finalReservations:row.finalReservations});
 }finally{try{await winner.release();await(await follow).release();}finally{db.close();}}
});
