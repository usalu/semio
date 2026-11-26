/** ⌛️ Contract: every request to the admission broker ends in a verdict, never in silence. A waiting unit hears from the broker every heartbeat, a unit that waits with nothing running runs alone after the stall bound, a unit older than the queue bound is refused, a grant growth that never answers ends at its own bound, and the Rust scheduler's silence bound and attempts equal the policy.
 * @see ../../🟦️.ts (`startAdmissionBrokerV1`, `Policy.bounds`)
 * @see ../../🦀️.rs (`ADMISSION_SILENCE`, `admit_with_retries`) */
import {readFileSync} from "node:fs";
import {createConnection} from "node:net";
import {expect,test} from "bun:test";
import {ADMISSION_POLICY_V1,startAdmissionBrokerV1,unitKeyV1,type AdmitRequestV1,type GrantGrowthPortV1,type GrowthHostV1,type UnitObservationV1} from "../../🟦️.ts";
import requests from "../🧫️fixtures/🔣️.json" with {type:"json"};

const MiB=1<<20,base=requests.requests.valid[0]! as unknown as AdmitRequestV1,pass={id:"wasm-release-00",profile:"release",target:"host"};
const unit=(crate:string):AdmitRequestV1=>({...base,crate,package:crate,crateTypes:["lib"],target:"host",externs:[],pid:1});
const history=(units:Readonly<Record<string,number>>)=>new Map<string,readonly UnitObservationV1[]>(Object.entries(units).map(([crate,peak])=>{
 const key=unitKeyV1(unit(crate),pass.profile);
 return [key,[{v:1,key,peakRssBytes:peak*MiB,outputBytes:0,wallMs:1000,cpuMs:1000,remainingMs:0,atMs:1,lines:0}]];
}));
const connect=(endpoint:string,request:Record<string,unknown>)=>{
 const socket=createConnection(endpoint),replies:Record<string,unknown>[]=[],readers:((reply:Record<string,unknown>|undefined)=>void)[]=[];
 let partial="",closed=false;
 socket.setTimeout(8000);
 socket.on("error",()=>undefined);
 socket.on("connect",()=>socket.write(JSON.stringify(request)+"\n"));
 socket.on("close",()=>{closed=true;for(const reader of readers.splice(0))reader(undefined);});
 socket.on("data",chunk=>{partial+=chunk;let next:number;while((next=partial.indexOf("\n"))>=0){const reply=JSON.parse(partial.slice(0,next));partial=partial.slice(next+1);replies.push(reply);const reader=readers.shift();if(reader)reader(reply);}});
 return {socket,replies,next:()=>new Promise<Record<string,unknown>|undefined>(accept=>{const pending=replies.length>seen?replies[seen]:undefined;if(pending){seen++;accept(pending);return;}if(closed){accept(undefined);return;}readers.push(reply=>{if(reply)seen++;accept(reply);});})};
 var seen=0;
};
const eventually=async(check:()=>boolean,limit=6000)=>{const started=Date.now();while(!check()&&Date.now()-started<limit)await new Promise(done=>setTimeout(done,10));return check();};
const common={available:()=>undefined,warden:()=>"calm" as const,cancels:()=>[]};

test("the Rust scheduler's silence bound and attempts equal the policy",()=>{
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8"),silence=/const ADMISSION_SILENCE: Duration = Duration::from_millis\(([\d_]+)\);/u.exec(source)?.[1],attempts=/const ADMISSION_ATTEMPTS: u32 = (\d+);/u.exec(source)?.[1];
 expect(Number(silence?.replaceAll("_",""))).toBe(ADMISSION_POLICY_V1.bounds.silenceMilliseconds);
 expect(Number(attempts)).toBe(ADMISSION_POLICY_V1.bounds.attempts);
 expect(ADMISSION_POLICY_V1.bounds.heartbeatMilliseconds*2).toBeLessThan(ADMISSION_POLICY_V1.bounds.silenceMilliseconds);
 expect(ADMISSION_POLICY_V1.bounds.predictMilliseconds+ADMISSION_POLICY_V1.bounds.heartbeatMilliseconds).toBeLessThan(ADMISSION_POLICY_V1.bounds.silenceMilliseconds);
});

test("a unit that waits with nothing running (the driver holds the whole grant) is heard from and then runs alone, never silent for ever",async()=>{
 const lines:string[]=[],broker=await startAdmissionBrokerV1({physicalBytes:1024*MiB,reserveBytes:0,parentMemoryAllowanceBytes:512*MiB,history:history({first:100}),...common,log:line=>lines.push(line),waits:{publishMilliseconds:20,progressMilliseconds:60000},bounds:{heartbeatMilliseconds:60,stallMilliseconds:250}});
 broker.registerPass(pass);
 broker.observe({nonRustcBytes:512*MiB,swapOutBytesPerSecond:null});
 const first=connect(broker.endpoint,unit("first"));
 try{
  expect(await first.next()).toMatchObject({op:"verdict",admitted:false});
  expect(await eventually(()=>first.replies.some(reply=>reply.admitted===true))).toBe(true);
  expect(first.replies.filter(reply=>reply.admitted===false).length).toBeGreaterThanOrEqual(2);
  expect(lines.some(line=>line.includes("unit first waited")&&line.includes("with nothing running")&&line.includes(" / Einheit first wartete"))).toBe(true);
  expect(broker.stats()).toMatchObject({running:1,waiting:0});
 }finally{first.socket.destroy();await broker.close();}
});

test("a unit queued behind a running unit gets a heartbeat of the broker at every bound",async()=>{
 const broker=await startAdmissionBrokerV1({physicalBytes:1024*MiB,reserveBytes:0,history:history({first:600,second:600}),...common,waits:{publishMilliseconds:20,progressMilliseconds:60000},bounds:{heartbeatMilliseconds:60}});
 broker.registerPass(pass);
 const first=connect(broker.endpoint,unit("first")),second=connect(broker.endpoint,unit("second"));
 try{
  expect(await first.next()).toMatchObject({op:"verdict",admitted:true});
  expect(await second.next()).toMatchObject({op:"verdict",admitted:false});
  expect(await eventually(()=>second.replies.length>=4)).toBe(true);
  expect(second.replies.every(reply=>reply.op==="verdict"&&reply.admitted===false)).toBe(true);
 }finally{first.socket.destroy();second.socket.destroy();await broker.close();}
});

test("a unit older than the queue bound is refused with a verdict, in both languages in the log",async()=>{
 const lines:string[]=[],broker=await startAdmissionBrokerV1({physicalBytes:1024*MiB,reserveBytes:0,history:history({first:600,second:600}),...common,log:line=>lines.push(line),waits:{publishMilliseconds:20,progressMilliseconds:60000},bounds:{heartbeatMilliseconds:60,queueMilliseconds:300}});
 broker.registerPass(pass);
 const first=connect(broker.endpoint,unit("first")),second=connect(broker.endpoint,unit("second"));
 try{
  expect(await first.next()).toMatchObject({op:"verdict",admitted:true});
  expect(await eventually(()=>second.replies.some(reply=>reply.op==="refused"))).toBe(true);
  expect(second.replies.at(-1)).toMatchObject({v:1,op:"refused",reason:"parent-memory"});
  expect(await eventually(()=>broker.stats().waiting===0)).toBe(true);
  expect(lines.some(line=>line.includes("unit second waited")&&line.includes("is refused")&&line.includes(" / Einheit second wartete"))).toBe(true);
  expect(broker.stats()).toMatchObject({running:1,waiting:0});
 }finally{first.socket.destroy();second.socket.destroy();await broker.close();}
});

test("a grant growth whose gate never answers (it ignores its signal) ends at its bound, the waiter is withdrawn and the unit still gets its verdict",async()=>{
 const host:GrowthHostV1={origin:"g1-server",totalMegabytes:32768,availableMegabytes:20000,floorMegabytes:2048,interactiveReserveMegabytes:4096,budgetBasisPoints:9000};
 let asks=0,signal:AbortSignal|undefined;
 const port:GrantGrowthPortV1={local:()=>host,fleet:()=>[],acquire:ask=>{asks++;signal=ask.signal;return new Promise(()=>undefined);}};
 const broker=await startAdmissionBrokerV1({physicalBytes:16*1024*MiB,history:history({first:256}),parentMemoryAllowanceBytes:1333*MiB,growth:port,...common,waits:{publishMilliseconds:20,progressMilliseconds:60000},growthPolicy:{overheadWaitMilliseconds:100,progressMilliseconds:50},bounds:{growthGraceMilliseconds:100,stallMilliseconds:300}});
 broker.registerPass(pass);
 broker.observe({nonRustcBytes:1500*MiB,swapOutBytesPerSecond:null});
 const first=connect(broker.endpoint,unit("first"));
 try{
  expect(await first.next()).toMatchObject({op:"verdict",admitted:false});
  expect(await eventually(()=>first.replies.some(reply=>reply.admitted===true))).toBe(true);
  expect(asks).toBe(1);
  expect(signal?.aborted).toBe(true);
  expect(first.replies.filter(reply=>reply.admitted===false).length).toBeGreaterThanOrEqual(2);
 }finally{first.socket.destroy();await broker.close();}
});

test("a fleet-only pass says so in its headroom probe, so that the wrapper never waits outside the slot for a refusal",async()=>{
 const broker=await startAdmissionBrokerV1({physicalBytes:1024*MiB,reserveBytes:0,history:history({}),...common});
 broker.registerPass({...pass,id:"fleet-pass",placement:"fleet-only"});
 const probe=connect(broker.endpoint,{...unit("first"),pass:"fleet-pass",op:"probe"});
 try{expect(await probe.next()).toEqual({v:1,op:"headroom",wait:true,fleetOnly:true});}finally{probe.socket.destroy();await broker.close();}
});
