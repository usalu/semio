/** 🚛️ Contract: a unit beyond the host of its requester waits for a larger machine, reports its wait in English and German, honours cancellation, is bounded by the schema timeout and then fails with `beyond-host-refused`; it never produces a local answer other than a refusal or a cancellation.
 * @see ../🟦️.ts */
import {expect,test} from "bun:test";
import {OFFLOAD_POLICY_V1,offloadTextV1} from "../../📐️policy/🟦️.ts";
import {awaitBeyondHostV1,type BeyondHostAttemptV1} from "../🟦️.ts";

type Row=Readonly<{kind:"done"}>|Readonly<{kind:"local";reason:never;detail:string}>;
const clock=()=>{let at=0;return{now:()=>at,sleep:async(milliseconds:number)=>{at+=milliseconds;},advance:(milliseconds:number)=>{at+=milliseconds;}};};
const local=(reason:string,detail=""):Row=>({kind:"local",reason:reason as never,detail});

test("the declared timeout is one hour with a progress line every 30 s",()=>{
 expect(OFFLOAD_POLICY_V1.beyondHost).toEqual({timeoutMilliseconds:3600000,progressMilliseconds:30000,retryMilliseconds:5000});
});

test("a busy fleet is waited for and the unit runs there when a machine takes it: no refusal, no local answer",async()=>{
 const time=clock();let tries=0;const lines:string[]=[];
 const answer=await awaitBeyondHostV1<Row>({crate:"semio_hub_cad_component",now:time.now,sleep:time.sleep,attempt:async()=>++tries<3?local("no-idle-machine"):{kind:"done"},queue:async()=>({queued:2,holders:["g1-server"],typicalSeconds:90}),report:text=>lines.push(text)});
 expect(answer).toEqual({kind:"done"});
 expect(tries).toBe(3);
 expect(lines.length).toBeGreaterThan(0);
 for(const line of lines){const [en,de]=line.split("\n");expect(en).toContain("larger one");expect(de).toContain("größere");expect(en).toContain("2 waiting");expect(en).toContain("g1-server");expect(en).toContain("90 s");}
});

test("the wait ends at the timeout with the refusal, in English and German, and never with a local answer",async()=>{
 const time=clock();const lines:string[]=[];let tries=0;
 const answer=await awaitBeyondHostV1<Row>({crate:"c",now:time.now,sleep:time.sleep,timeoutMilliseconds:60000,retryMilliseconds:5000,attempt:async()=>{tries++;return local("infrastructure","scheduler away");},queue:async()=>undefined,report:text=>lines.push(text)});
 expect(answer).toMatchObject({kind:"local",reason:"beyond-host-refused"});
 expect(tries).toBeGreaterThan(5);
 const last=lines.at(-1)!.split("\n");
 expect(last[0]).toContain("no larger machine took it");expect(last[1]).toContain("keine größere Maschine");
 const queued=await awaitBeyondHostV1<Row>({crate:"c",now:time.now,sleep:time.sleep,timeoutMilliseconds:60000,attempt:async()=>local("queue-timeout"),queue:async()=>undefined,report:()=>undefined});
 expect(queued).toMatchObject({reason:"beyond-host-refused"});
});

test("a reason no larger machine can fix is refused at once instead of compiling here",async()=>{
 for(const reason of ["no-compatible-worker","remote-failed-local-redo","incomplete-inputs","not-portable","pin-failed"]){
  let tries=0;
  const answer=await awaitBeyondHostV1<Row>({crate:"c",attempt:async()=>{tries++;return local(reason,"x");},queue:async()=>undefined,report:()=>undefined});
  expect(answer,reason).toMatchObject({kind:"local",reason:"beyond-host-refused"});
  expect(tries,reason).toBe(1);
 }
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
 expect(offloadTextV1("en","beyondHost.refused",{crate:"c",reason:"r"})).toContain("no larger machine took it");
});
