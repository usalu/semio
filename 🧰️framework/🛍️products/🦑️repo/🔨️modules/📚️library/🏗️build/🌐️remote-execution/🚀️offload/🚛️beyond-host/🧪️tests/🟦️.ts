/** 🚛️ Contract: a unit beyond the host of its requester waits at most the schema's claim deadline (seconds) for a larger machine to claim it, reports its wait in English and German, honours cancellation and then answers `beyond-host-local` so the caller compiles it alone under the memory warden; it never waits passively for a machine that does not take it.
 * @see ../🟦️.ts */
import {expect,test} from "bun:test";
import {OFFLOAD_POLICY_V1,offloadTextV1} from "../../📐️policy/🟦️.ts";
import {awaitBeyondHostV1,type BeyondHostAttemptV1} from "../🟦️.ts";
import cases from "./🔣️.json";

type Row=Readonly<{kind:"done"}>|Readonly<{kind:"local";reason:never;detail:string}>;
const clock=()=>{let at=0;return{now:()=>at,sleep:async(milliseconds:number)=>{at+=milliseconds;},advance:(milliseconds:number)=>{at+=milliseconds;}};};
const local=(reason:string,detail=""):Row=>({kind:"local",reason:reason as never,detail});

test("the declared claim deadline is seconds, never an hour, with a progress line well inside it",()=>{
 expect(OFFLOAD_POLICY_V1.beyondHost).toEqual({claimMilliseconds:20000,progressMilliseconds:5000,retryMilliseconds:2000});
 expect(OFFLOAD_POLICY_V1.beyondHost.claimMilliseconds).toBeLessThanOrEqual(60000);
});

test("a busy fleet is waited for and the unit runs there when a machine takes it: no refusal, no local answer",async()=>{
 const time=clock();let tries=0;const lines:string[]=[];
 const answer=await awaitBeyondHostV1<Row>({crate:"semio_hub_cad_component",now:time.now,sleep:time.sleep,attempt:async()=>++tries<3?local("no-idle-machine"):{kind:"done"},queue:async()=>({queued:2,holders:["g1-server"],typicalSeconds:90}),report:text=>lines.push(text)});
 expect(answer).toEqual({kind:"done"});
 expect(tries).toBe(3);
 expect(lines.length).toBeGreaterThan(0);
 for(const line of lines){const [en,de]=line.split("\n");expect(en).toContain("larger one");expect(de).toContain("größere");expect(en).toContain("2 waiting");expect(en).toContain("g1-server");expect(en).toContain("90 s");}
});

test("the wait ends at the claim deadline with the local fallback, announced in English and German",async()=>{
 const time=clock();const lines:string[]=[];let tries=0;
 const answer=await awaitBeyondHostV1<Row>({crate:"c",now:time.now,sleep:time.sleep,claimMilliseconds:6000,retryMilliseconds:2000,attempt:async()=>{tries++;return local("infrastructure","scheduler away");},queue:async()=>undefined,report:text=>lines.push(text)});
 expect(answer).toMatchObject({kind:"local",reason:"beyond-host-local"});
 expect(tries).toBeGreaterThanOrEqual(2);
 expect(time.now()).toBeLessThanOrEqual(6000);
 const last=lines.at(-1)!.split("\n");
 expect(last[0]).toContain("compiles here alone");expect(last[1]).toContain("allein");
 const unclaimed=await awaitBeyondHostV1<Row>({crate:"c",now:time.now,sleep:time.sleep,claimMilliseconds:6000,attempt:async()=>local("queue-timeout"),queue:async()=>undefined,report:()=>undefined});
 expect(unclaimed).toMatchObject({reason:"beyond-host-local"});
});

test("a reason no larger machine can fix falls back at once instead of waiting",async()=>{
 for(const reason of ["no-compatible-worker","remote-failed-local-redo","incomplete-inputs","not-portable","pin-failed"]){
  let tries=0;
  const answer=await awaitBeyondHostV1<Row>({crate:"c",attempt:async()=>{tries++;return local(reason,"x");},queue:async()=>undefined,report:()=>undefined});
  expect(answer,reason).toMatchObject({kind:"local",reason:"beyond-host-local"});
  expect(tries,reason).toBe(1);
 }
});

test("a machine that accepts the connection but never takes the action costs the claim deadline, not an hour",async()=>{
 const started=Date.now();let tries=0;
 const answer=await awaitBeyondHostV1<Row>({crate:"c",claimMilliseconds:300,retryMilliseconds:20,progressMilliseconds:50,attempt:async(remaining)=>{tries++;await new Promise(done=>setTimeout(done,remaining));return local("queue-timeout","unclaimed");},queue:async()=>undefined,report:()=>undefined});
 expect(answer).toMatchObject({kind:"local",reason:"beyond-host-local"});
 expect(tries).toBe(1);
 expect(Date.now()-started).toBeLessThan(3000);
});

test("cancellation ends the wait at once as cancelled, before and during an attempt",async()=>{
 const before=new AbortController();before.abort();let tries=0;
 expect(await awaitBeyondHostV1<Row>({crate:"c",signal:before.signal,attempt:async()=>{tries++;return{kind:"done"};},queue:async()=>undefined,report:()=>undefined})).toEqual({kind:"local",reason:"cancelled",detail:""});
 expect(tries).toBe(0);
 const during=new AbortController();
 const answer=await awaitBeyondHostV1<Row>({crate:"c",signal:during.signal,attempt:async()=>{during.abort();return local("cancelled");},queue:async()=>undefined,report:()=>undefined});
 expect(answer).toEqual({kind:"local",reason:"cancelled",detail:""});
});

test("progress is reported while an attempt waits in the queue, every progress interval",async()=>{
 const lines:string[]=[];
 const answer=await awaitBeyondHostV1<Row>({crate:"c",progressMilliseconds:10,attempt:async()=>{await new Promise(done=>setTimeout(done,75));return{kind:"done"};},queue:async()=>({queued:1,holders:[],typicalSeconds:5}),report:text=>lines.push(text)});
 expect(answer).toEqual({kind:"done"});
 expect(lines.length).toBeGreaterThanOrEqual(3);
 expect(lines[0]!.split("\n").length).toBe(2);
 expect(offloadTextV1("en","beyondHost.fallback",{crate:"c",reason:"r",seconds:20})).toContain("compiles here alone");
});

test("every case of the language-agnostic fixture answers as written, inside the claim deadline",async()=>{
 expect(cases.claimMilliseconds).toBe(OFFLOAD_POLICY_V1.beyondHost.claimMilliseconds);
 expect(cases.retryMilliseconds).toBe(OFFLOAD_POLICY_V1.beyondHost.retryMilliseconds);
 for(const row of cases.cases){
  const time=clock();let tries=0;const controller=new AbortController();
  const answer=await awaitBeyondHostV1<Row>({crate:"c",now:time.now,sleep:time.sleep,signal:controller.signal,attempt:async()=>{const step=row.repeat===true?row.attempts[0]!:row.attempts[Math.min(tries,row.attempts.length-1)]!;tries++;if(step==="cancelled")controller.abort();return step==="done"?{kind:"done"}:local(step);},queue:async()=>undefined,report:()=>undefined});
  expect(answer.kind==="done"?"done":answer.reason,row.name).toBe(row.result);
  if("tries" in row)expect(tries,row.name).toBe(row.tries);
  if("minimumTries" in row)expect(tries,row.name).toBeGreaterThanOrEqual(row.minimumTries as number);
  expect(time.now(),row.name).toBeLessThanOrEqual(cases.claimMilliseconds);
 }
});
