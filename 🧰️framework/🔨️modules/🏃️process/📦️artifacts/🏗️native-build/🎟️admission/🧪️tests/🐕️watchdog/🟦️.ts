/** 🐕️ Contract: the admission broker never lets one wedged client hold the build still. A running unit whose scheduler reports nothing is taken for lost and its booking is given back; a probe whose estimate never finishes is dropped at the estimate bound; a unit queued behind a running one while the driver's own memory grew is rescued by a bounded growth instead of waiting for the running unit to end.
 * @see ../../🟦️.ts (`startAdmissionBrokerV1`, `Policy.bounds.runningSilenceMilliseconds`, `rescueStalled`)
 * @see ../../🦀️.rs (the scheduler reports `rss` every second) */
import {createConnection} from "node:net";
import {expect,test} from "bun:test";
import {ADMISSION_POLICY_V1,startAdmissionBrokerV1,unitKeyV1,type AdmitRequestV1,type GrantGrowthPortV1,type GrowthHostV1,type UnitObservationV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import requests from "../🧫️fixtures/🔣️.json" with {type:"json"};

const MiB=1<<20,base=requests.requests.valid[0]! as unknown as AdmitRequestV1,pass={id:"wasm-release-00",profile:"release",target:"host"};
const unit=(crate:string):AdmitRequestV1=>({...base,crate,package:crate,crateTypes:["lib"],target:"host",externs:[],pid:1});
const history=(units:Readonly<Record<string,number>>)=>new Map<string,readonly UnitObservationV1[]>(Object.entries(units).map(([crate,peak])=>{
 const key=unitKeyV1(unit(crate),pass.profile);
 return [key,[{v:1,key,peakRssBytes:peak*MiB,outputBytes:0,wallMs:1000,cpuMs:1000,remainingMs:0,atMs:1,lines:0}]];
}));
const connect=(endpoint:string,request:Record<string,unknown>)=>{
 const socket=createConnection(endpoint),replies:Record<string,unknown>[]=[];let partial="",closed=false;
 socket.on("error",()=>undefined);
 socket.on("connect",()=>socket.write(JSON.stringify(request)+"\n"));
 socket.on("close",()=>{closed=true;});
 socket.on("data",chunk=>{partial+=chunk;let next:number;while((next=partial.indexOf("\n"))>=0){replies.push(JSON.parse(partial.slice(0,next)));partial=partial.slice(next+1);}});
 return {socket,replies,closed:()=>closed};
};
const eventually=async(check:()=>boolean,limit=6000)=>{const started=Date.now();while(!check()&&Date.now()-started<limit)await new Promise(done=>setTimeout(done,10));return check();};
const common={available:()=>undefined,warden:()=>"calm" as const,cancels:()=>[]};
const host:GrowthHostV1={origin:"g1-server",totalMegabytes:32768,availableMegabytes:20000,floorMegabytes:2048,interactiveReserveMegabytes:4096,budgetBasisPoints:9000};

test("the policy names the silence bound and both languages have its text with the same placeholders",()=>{
 expect(ADMISSION_POLICY_V1.bounds.runningSilenceMilliseconds).toBeGreaterThan(ADMISSION_POLICY_V1.bounds.stallMilliseconds);
 const placeholders=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(match=>match[1]).sort();
 expect(placeholders(schema.definitions.Messages.const.en.waits.silent)).toEqual(placeholders(schema.definitions.Messages.const.de.waits.silent));
 expect(placeholders(schema.definitions.Messages.const.en.waits.silent)).toEqual(["seconds","unit"]);
});

test("a running unit whose scheduler goes silent is dropped at the bound, its booking is given back and the unit queued behind it runs; a unit that keeps reporting stays",async()=>{
 const lines:string[]=[],broker=await startAdmissionBrokerV1({physicalBytes:1024*MiB,reserveBytes:0,history:history({lost:600,alive:100,next:600}),...common,log:line=>lines.push(line),waits:{publishMilliseconds:20,progressMilliseconds:60000},bounds:{heartbeatMilliseconds:60000,runningSilenceMilliseconds:300}});
 broker.registerPass(pass);
 const lost=connect(broker.endpoint,unit("lost"));
 try{
  expect(await eventually(()=>lost.replies.some(reply=>reply.admitted===true))).toBe(true);
  const next=connect(broker.endpoint,unit("next"));
  expect(await eventually(()=>broker.stats().waiting===1)).toBe(true);
  expect(await eventually(()=>lost.closed())).toBe(true);
  expect(await eventually(()=>next.replies.some(reply=>reply.admitted===true))).toBe(true);
  expect(lines.some(line=>line.includes("unit lost sent no report")&&line.includes(" / Einheit lost hat seit"))).toBe(true);
  expect(broker.stats()).toMatchObject({running:1,waiting:0});
  next.socket.destroy();
  expect(await eventually(()=>broker.stats().running===0)).toBe(true);
  const alive=connect(broker.endpoint,unit("alive"));
  expect(await eventually(()=>alive.replies.some(reply=>reply.admitted===true))).toBe(true);
  const beat=setInterval(()=>{if(!alive.closed())alive.socket.write(JSON.stringify({v:1,op:"rss",bytes:50*MiB,chargedBytes:50*MiB})+"\n");},50);
  await new Promise(done=>setTimeout(done,900));clearInterval(beat);
  expect(alive.closed()).toBe(false);expect(broker.stats().running).toBe(1);
  alive.socket.destroy();
 }finally{lost.socket.destroy();await broker.close();}
});

test("a probe whose estimate never finishes is dropped at the estimate bound with a bilingual line, and the broker still answers the next unit",async()=>{
 const lines:string[]=[],broker=await startAdmissionBrokerV1({physicalBytes:1024*MiB,reserveBytes:0,history:history({first:100}),available:()=>undefined,warden:()=>"calm",cancels:()=>new Promise(()=>undefined),log:line=>lines.push(line),waits:{publishMilliseconds:20,progressMilliseconds:60000},bounds:{predictMilliseconds:200}});
 broker.registerPass(pass);
 const probe=connect(broker.endpoint,{...unit("first"),op:"probe"});
 try{
  expect(await eventually(()=>probe.closed())).toBe(true);
  expect(probe.replies).toEqual([]);
  expect(lines.some(line=>line.includes("unit first: the broker did not finish estimating")&&line.includes(" / Einheit first: der Broker"))).toBe(true);
 }finally{probe.socket.destroy();await broker.close();}
});

test("a unit queued behind a running one while the driver's own memory grew is rescued by growth beside the running unit, not after it",async()=>{
 const asks:number[]=[],port:GrantGrowthPortV1={local:()=>host,fleet:()=>[],acquire:async ask=>{asks.push(ask.extraMegabytes);return{granted:true,release:()=>undefined};}};
 const broker=await startAdmissionBrokerV1({physicalBytes:16*1024*MiB,history:history({first:300,second:300}),parentMemoryAllowanceBytes:2000*MiB,growth:port,...common,waits:{publishMilliseconds:20,progressMilliseconds:40}});
 broker.registerPass(pass);
 const first=connect(broker.endpoint,unit("first"));
 try{
  expect(await eventually(()=>first.replies.some(reply=>reply.admitted===true))).toBe(true);
  broker.observe({nonRustcBytes:1500*MiB,swapOutBytesPerSecond:null});
  const second=connect(broker.endpoint,unit("second"));
  expect(await eventually(()=>second.replies.some(reply=>reply.admitted===true))).toBe(true);
  expect(broker.stats().running).toBe(2);
  expect(asks).toHaveLength(1);
  const predicted=Math.ceil(1.15*300*MiB);
  expect(asks[0]).toBe(Math.ceil((2*predicted+1500*MiB-2000*MiB)/MiB));
  second.socket.destroy();
 }finally{first.socket.destroy();await broker.close();}
});

test("a unit that waits behind running units for the grant itself (not for the driver's memory) asks for no growth: the grant bounds the parallelism",async()=>{
 const asks:number[]=[],port:GrantGrowthPortV1={local:()=>host,fleet:()=>[],acquire:async ask=>{asks.push(ask.extraMegabytes);return{granted:true,release:()=>undefined};}};
 const broker=await startAdmissionBrokerV1({physicalBytes:16*1024*MiB,history:history({first:900,second:900}),parentMemoryAllowanceBytes:1500*MiB,growth:port,...common,waits:{publishMilliseconds:20,progressMilliseconds:40}});
 broker.registerPass(pass);
 const first=connect(broker.endpoint,unit("first")),second=connect(broker.endpoint,unit("second"));
 try{
  expect(await eventually(()=>first.replies.some(reply=>reply.admitted===true))).toBe(true);
  await new Promise(done=>setTimeout(done,400));
  expect(second.replies.some(reply=>reply.admitted===true)).toBe(false);
  expect(asks).toEqual([]);
  expect(broker.stats()).toMatchObject({running:1,waiting:1});
 }finally{first.socket.destroy();second.socket.destroy();await broker.close();}
});
