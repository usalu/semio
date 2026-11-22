/** 🔒️ Contract: a slice load that waits for a graph writer lock names the lock, its place in line and its holder (process, agent, hold time, state, progress) in both languages, and the pool hands the notice of the worker on to the client with every heartbeat.
 * @see ../../🏛️service/🌐️host/🟨️.mjs `renderLeaseWaitV1` */
import {expect,test} from "bun:test";
import {renderLeaseWaitV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {HOST_POLICY_V1,POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const wait={lock:{kind:"facts",number:56},resource:"repository-graph-facts:/w:shard:56",position:2,queued:3,pid:3936,seconds:12000,state:"progressing",agent:"claude-code",phase:"inference",done:303,total:600,etaMinutes:138};

test("the wait names lock, place, holder, agent, hold time, state, progress and time left in English and German",()=>{
 const text=renderLeaseWaitV1(wait,481,["en","de"]);
 for(const language of["en","de"] as const){const line=text[language]!;expect(line).not.toMatch(/\{[A-Za-z]+\}/u);for(const part of["481","56","2","3","3936","claude-code","200 min","inference","303","600","138"])expect(line).toContain(part);}
 expect(text.en).toContain("progressing");expect(text.en).toContain("fact writer lock");expect(text.de).toContain("Fortschritt");expect(text.de).toContain("Schreibsperre");
});

test("every lock kind, holder state and hold time unit has a text in both languages, a missing holder detail leaves its part out",()=>{
 for(const kind of["facts","publication","scope","other"])for(const state of["progressing","silent","stuck","stale","unknown"]){
  const text=renderLeaseWaitV1({lock:{kind},resource:"some-lock",position:2,queued:2,pid:7,seconds:30,state},1,["en","de"]);
  for(const language of["en","de"] as const){expect(text[language]).not.toMatch(/\{[A-Za-z]+\}/u);expect(text[language]).toContain("30 s");expect(text[language]).not.toContain("(agent");expect(text[language]).not.toContain("(Agent");}
 }
 const bare=renderLeaseWaitV1({lock:{kind:"scope",number:9},resource:"r",position:2,queued:2,pid:7,seconds:5},1,["en"]);
 expect(bare.en).toContain("claim on this selection (bucket 9)");expect(bare.en).not.toContain("holder is");expect(bare.de).toBeUndefined();
 expect(Object.keys(POLICY_V1.messages.en).filter(key=>key.startsWith("host-lease-")).sort()).toEqual(Object.keys(POLICY_V1.messages.de).filter(key=>key.startsWith("host-lease-")).sort());
});

test("the pool hands the lease wait of the worker on to the client until the worker reports other progress",async()=>{
 const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number},seen:any[]=[];let emit:(value:unknown)=>void=()=>undefined,started=false;
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,heartbeatMilliseconds:20,graceMilliseconds:50,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,
  reaper:{reap:async()=>0,killNow:()=>undefined},
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),
  createWorker:()=>{const listeners:Record<string,(value:unknown)=>void>={};started=true;emit=value=>listeners.message?.(value);return{pid:undefined,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},terminate:async()=>0};}});
 const run=pool.run({root:"/tmp/lease",name:"lease"},{onProgress:(_ms:number,detail:any)=>{seen.push(detail);}});
 const until=async(check:()=>boolean)=>{for(let at=0;!check()&&at<200;at++)await new Promise(done=>setTimeout(done,10));expect(check()).toBe(true);};
 await until(()=>started);
 emit({type:"progress",phase:"lease-wait",done:303,total:600,leaseWait:wait});
 await until(()=>seen.some(detail=>detail?.leaseWait?.pid===3936));
 expect(seen.find(detail=>detail?.leaseWait)!.leaseWait).toEqual(wait);
 emit({type:"progress",phase:"inference",done:1,total:2});
 const mark=seen.length;await until(()=>seen.length>mark+1);
 expect(seen.slice(mark+1).every(detail=>detail?.leaseWait===undefined)).toBe(true);
 emit({type:"done",graph:{}});await run;await pool.close();
});
