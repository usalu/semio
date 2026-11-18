/** 🔭️ Contract: a graph read that outlived the process that started it is adopted by the next process of the graph server. The server asks for it before it asks anything else, starts no second read for the generation, keeps the steering files the read works with (so the projects that were fresh before the restart are told at once and not after the next wave), answers the held requests when the adopted read ends, and lets go of it when it is stopped itself. */
import{afterAll,expect,test}from"bun:test";
import{existsSync,mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{graphFleetClientV1}from"../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../🧶️nx/🟦️.ts";
import{publishWantedV1,readServedV1}from"../../🟦️.ts";
import{readPendingV1,readPriorityV1,writePendingV1}from"../../../../🕸️graph/🕰️generations/🧭️steering/🟨️.mjs";
import{runWantedServerV1,type WantedInferV1}from"../../🛠️server/🟦️.ts";

const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`wanted-adopt-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const PLATFORM="darwin-arm64",KEY="b".repeat(64),roomy=async()=>({freeMegabytes:20000,pressure:"normal" as const}),poor=async()=>({freeMegabytes:100,pressure:"warn" as const});
const names=Array.from({length:6},(_,index)=>"p"+index);

async function world(){
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 const client=(label:string)=>{const root=temp(label);return graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});};
 return{server,client};
}
type World=Awaited<ReturnType<typeof world>>;

async function firstProcess(input:Readonly<{client:ReturnType<World["client"]>;requester:ReturnType<World["client"]>;steer:string;time:()=>number}>){
 const controller=new AbortController(),steer={SEMIO_GRAPH_STEER_DIRECTORY:input.steer};let round=0;
 const steps:(()=>Promise<void>|void)[]=[
  async()=>{await publishWantedV1({client:input.requester,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:names,now:input.time});},
  ()=>{writePendingV1(steer,-1,["p3","p4","p5"],readPriorityV1(steer));},
  ()=>{},
  ()=>{controller.abort();},
 ];
 await runWantedServerV1({client:input.client,owner:"g1-server",platform:PLATFORM,steerDirectory:input.steer,language:"en",requesters:async()=>["g1-m06"],headroom:roomy,
  infer:(_reason,signal)=>new Promise<WantedInferV1>(resolve=>{signal?.addEventListener("abort",()=>resolve({ok:false,status:130,seconds:1,generation:null,detail:"detached"}),{once:true});}),
  quietMilliseconds:async()=>10**9,say:()=>{},signal:controller.signal,now:input.time,sleep:async()=>{await steps[round++]?.();}});
}

test("the next process adopts the read in flight: no second read, the fresh projects are told at once from the kept steering files, the held request is answered when the adopted read ends",async()=>{
 const{server,client}=await world();
 try{
  const requester=client("requester"),serving=client("serving"),again=client("again"),steer=temp("steer");let clock=2_000_000_000;const time=()=>clock;
  await firstProcess({client:serving,requester,steer,time});
  expect((await readServedV1({client:requester,requester:"g1-m06"}))?.projects).toEqual(["p0","p1","p2"]);
  const priority=readPriorityV1({SEMIO_GRAPH_STEER_DIRECTORY:steer})!;
  clock+=60_000;
  let finish!:(result:WantedInferV1)=>void,inferred=0,adopted=0;const lines:string[]=[],answers:(readonly string[]|undefined)[]=[];
  const done=new Promise<WantedInferV1>(resolve=>{finish=resolve;});
  await runWantedServerV1({client:again,owner:"g1-server",platform:PLATFORM,steerDirectory:steer,language:"en",requesters:async()=>["g1-m06"],headroom:poor,
   adopt:()=>{adopted++;return{reason:"request",done};},
   infer:async()=>{inferred++;return{ok:true,status:0,seconds:1,generation:1,detail:""};},
   quietMilliseconds:async()=>10**9,say:line=>lines.push(line),signal:new AbortController().signal,now:time,
   sleep:async()=>{answers.push((await readServedV1({client:requester,requester:"g1-m06"}))?.projects);if(answers.length===3)finish({ok:true,status:0,seconds:30,generation:9,detail:"request"});await Bun.sleep(5);},rounds:5});
  expect(adopted,"asked once, before the first round").toBe(1);expect(inferred,"no second read, however little memory there is: the adopted read has its memory already").toBe(0);
  expect(readPriorityV1({SEMIO_GRAPH_STEER_DIRECTORY:steer})?.atMs,"the priority of the read in flight is the same file, not a new one").toBe(priority.atMs);
  expect(answers[0],"the projects that were fresh before the restart are told in the first round").toEqual(["p0","p1","p2"]);
  expect(existsSync(join(steer,"pending.json")),"the read's own pending file was not removed").toBe(true);
  const last=await readServedV1({client:requester,requester:"g1-m06"});expect(last?.projects).toEqual(names);expect(last?.generation).toBe(9);
  expect(lines.some(line=>line.includes("served 6 projects for g1-m06"))).toBe(true);
 }finally{await server.stop(true);}
},90000);

test("without an adoptable read the server behaves as before: it starts one read when a request is held and memory allows",async()=>{
 const{server,client}=await world();
 try{
  const requester=client("requester"),serving=client("serving"),steer=temp("steer");const time=()=>3_000_000_000;let inferred=0,asked=0;
  await publishWantedV1({client:requester,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:names,now:time});
  await runWantedServerV1({client:serving,owner:"g1-server",platform:PLATFORM,steerDirectory:steer,language:"en",requesters:async()=>["g1-m06"],headroom:roomy,
   adopt:()=>{asked++;return undefined;},infer:async()=>{inferred++;return{ok:true,status:0,seconds:1,generation:2,detail:""};},
   quietMilliseconds:async()=>10**9,say:()=>{},signal:new AbortController().signal,now:time,sleep:async()=>{await Bun.sleep(5);},rounds:3});
  expect(asked).toBe(1);expect(inferred).toBe(1);
  expect((await readServedV1({client:requester,requester:"g1-m06"}))?.generation).toBe(2);
 }finally{await server.stop(true);}
},60000);

test("a server that is stopped lets go of the adopted read: it answers nobody and waits for the detach, not for the end of the read",async()=>{
 const{server,client}=await world();
 try{
  const requester=client("requester"),serving=client("serving"),steer=temp("steer");const time=()=>4_000_000_000,controller=new AbortController();
  await publishWantedV1({client:requester,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:names,now:time});
  let detached=false;
  await runWantedServerV1({client:serving,owner:"g1-server",platform:PLATFORM,steerDirectory:steer,language:"en",requesters:async()=>["g1-m06"],headroom:poor,
   adopt:signal=>({reason:"request",done:new Promise<WantedInferV1>(resolve=>{signal?.addEventListener("abort",()=>{detached=true;resolve({ok:false,status:1,seconds:1,generation:null,detail:"detached"});},{once:true});})}),
   infer:async()=>({ok:true,status:0,seconds:1,generation:1,detail:""}),quietMilliseconds:async()=>10**9,say:()=>{},signal:controller.signal,now:time,sleep:async()=>{controller.abort();}});
  expect(detached).toBe(true);
  const served=await readServedV1({client:requester,requester:"g1-m06"});expect(served?.generation ?? null,"not answered as complete").toBeNull();
  expect(readPendingV1(steer)).toBeUndefined();
 }finally{await server.stop(true);}
},60000);
