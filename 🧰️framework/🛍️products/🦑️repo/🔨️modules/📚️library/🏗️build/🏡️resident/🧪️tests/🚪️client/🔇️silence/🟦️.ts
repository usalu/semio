import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {createServer,type Socket} from "node:net";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {testScratchRootV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";
import {createFrameDecoderV1,encodeFrameV1} from "../../../📨️protocol/🟨️.mjs";
import {HOST_POLICY_V1} from "../../../📨️protocol/🎚️policy/🟨️.mjs";
import {hostEndpointsV1} from "../../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {requestPlanV1} from "../../../🚪️client/🛰️plan/🟨️.mjs";
import {createHostServiceV1} from "../../../🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../../🔌️ports/🧭️plan/🟨️.mjs";
import graph from "../../🧫️fixtures/🛰️host-graph.json" with {type:"json"};
import cases from "./🔣️.json" with {type:"json"};
import handoff from "./🔁️reattach.json" with {type:"json"};

async function observe(row:typeof cases[number],interruption?:string){
 const implementation=createHash("sha256").update(row.name).digest("hex"),runtimeDir="r-"+implementation.slice(0,6),environment={...process.env,XDG_RUNTIME_DIR:runtimeDir,LOCALAPPDATA:join(process.cwd(),runtimeDir)},runtime=hostEndpointsV1(implementation,environment);
 const welcome={t:"welcome",v:1,features:["plan"],service:{pid:process.pid,instance:"1".repeat(16),startedAt:0,implementation,generation:null,watcher:{backend:"none",trust:"none",cursor:"0"}},limits:{maximumFrameBytes:HOST_POLICY_V1.maximumFrameBytes,maximumInFlight:1}};
 const silenceMilliseconds=80,message={en:"fixture progress",de:"fixture progress"};
 const frames:Record<string,(id:number)=>Record<string,unknown>>={
  accepted:id=>({t:"accepted",id,queue:0}),
  progress:id=>({t:"progress",id,phase:"slice",done:1,total:2,message}),
  "invalid-accepted":id=>({t:"accepted",id,queue:-1}),
  "invalid-progress":id=>({t:"progress",id,phase:"unknown",done:1,total:2,message}),
  "wrong-accepted":id=>({t:"accepted",id:id+1,queue:0}),
  "wrong-progress":id=>({t:"progress",id:id+1,phase:"slice",done:7,total:8,message}),
  "wrong-refused":id=>({t:"refused",id:id+1,code:"invalid",retry:"never",message}),
  "duplicate-welcome":()=>welcome
 };
 mkdirSync(runtime.directory,{recursive:true});writeFileSync(runtime.key,Buffer.alloc(32,7));
 let connections=0,progress=0,overlap=0;
 const stop=new AbortController();
 const sockets=new Set<Socket>(),server=createServer(socket=>{
  let connection=0;
  socket.once("close",()=>sockets.delete(socket));
  const decoder=createFrameDecoderV1({maximumFrameBytes:HOST_POLICY_V1.maximumFrameBytes,onError:()=>socket.destroy(),onFrame:frame=>{
   if(frame.t==="hello"&&(frame as {client?:{name?:string}}).client?.name!=="repo-probe"){connection=++connections;sockets.add(socket);overlap=Math.max(overlap,sockets.size);}
   if(frame.t==="hello"&&row.welcome)socket.write(encodeFrameV1(welcome));
   if(frame.t==="plan"){
    if(interruption&&connection>1){
     if(interruption==="cancel")stop.abort(new Error("fixture cancelled"));
     else for(const previous of sockets)if(previous!==socket)previous.write(encodeFrameV1({t:"plan-reply",id:frame.id,revision:"7".repeat(64),plan:{tasks:[]},progress:[],cacheHits:0}));
     return;
    }
    const send=()=>{for(const event of row.frames)socket.write(encodeFrameV1(frames[event]!(frame.id??0)));};
    send();
    if(row.repeat){const timer=setInterval(send,20);socket.once("close",()=>clearInterval(timer));}
   }
  }});
  socket.on("data",chunk=>decoder.push(chunk));socket.on("error",()=>undefined);
 });
 await new Promise<void>((ready,fail)=>{server.once("error",fail);server.listen(runtime.endpoint,ready);});
 try{
  const error=await requestPlanV1({root:process.cwd(),implementation,targets:["app:test"],agent:"a",language:"en",environment,silenceMilliseconds,attempts:1,signal:AbortSignal.any([AbortSignal.timeout(5000),stop.signal]),onProgress:()=>progress++}).then(()=>undefined,value=>value);
  if(interruption){for(let i=0;i<100&&sockets.size>0;i++)await Bun.sleep(10);return{outcome:error?.reason??error?.message,overlap,remaining:sockets.size};}
  return{reason:error?.reason,stage:error?.planSilence?.stage,phase:error?.planSilence?.lastProgress??null,limit:error?.planSilence?.limitMilliseconds,connections,progress,english:error?.notice?.("en"),german:error?.notice?.("de")};
 }finally{for(const socket of sockets)socket.destroy();await new Promise<void>(done=>server.close(()=>done()));}
}

async function observeHeldPlan(){
 const implementation="1".repeat(64),runtimeDir="r-live",environment={...process.env,XDG_RUNTIME_DIR:runtimeDir,LOCALAPPDATA:join(process.cwd(),runtimeDir)},runtime=hostEndpointsV1(implementation,environment);
 const inner=createGraphPlannerPortV1({identify:async()=>"2".repeat(64),loadGraph:async()=>graph});
 let release!:()=>void;
 const pending=new Promise<void>(done=>{release=done;}),service=createHostServiceV1({runtime,implementation,environment,ports:{planner:{...inner,plan:async(options:Parameters<typeof inner.plan>[0])=>{await pending;return inner.plan(options);}}},policy:{requestKeepaliveMilliseconds:40}});
 let keepalives=0;
 try{
  await service.start();
  const request=requestPlanV1({root:process.cwd(),implementation,targets:["app:test"],agent:"a",language:"en",environment,silenceMilliseconds:150,attempts:1,signal:AbortSignal.timeout(5000),onProgress:frame=>{if(frame.phase==="alive")keepalives++;}});
  for(let i=0;i<30&&keepalives<3;i++)await Bun.sleep(20);
  release();
  return{reply:(await request).reply.t,keepalives};
 }finally{release();await service.stop("test");}
}

async function observeReattachedPlan(){
 const implementation="3".repeat(64),runtimeDir="r-retry",environment={...process.env,XDG_RUNTIME_DIR:runtimeDir,LOCALAPPDATA:join(process.cwd(),runtimeDir)},runtime=hostEndpointsV1(implementation,environment);
 const inner=createGraphPlannerPortV1({identify:async()=>"4".repeat(64),loadGraph:async()=>graph});
 let release!:()=>void,reports=0;
 const pending=new Promise<void>(done=>{release=done;}),trace:string[]=[];
 const service=createHostServiceV1({runtime,implementation,environment,ports:{planner:{...inner,plan:async(options:Parameters<typeof inner.plan>[0])=>{
  trace.push("start");
  await options.progress?.(1000);
  await new Promise<void>((resolve,reject)=>{
   const abort=()=>{trace.push("abort");reject(options.signal?.reason);};
   options.signal?.addEventListener("abort",abort,{once:true});
   pending.then(()=>{options.signal?.removeEventListener("abort",abort);resolve();});
  });
  trace.push("answer");
  return inner.plan(options);
 }}},policy:{requestKeepaliveMilliseconds:10_000}});
 try{
  await service.start();
  const targets=[...handoff.targets];
  const result=await requestPlanV1({root:process.cwd(),implementation,targets,agent:"a",language:"en",environment,silenceMilliseconds:handoff.silenceMilliseconds,attempts:1,signal:AbortSignal.timeout(5000),onProgress:frame=>{
   if(frame.phase!=="slice")return;
   if(++reports===1)targets[0]="fixture:absent";
   if(reports===2)release();
  }});
  return{reply:result.reply.t,trace,reports,jobs:service.counters.jobs,coalesced:service.counters.coalesced};
 }finally{release();await service.stop("test");}
}

if(process.argv.includes("--silence-fixture")){
 for(const row of cases)console.log(JSON.stringify(await observe(row)));
 console.log(JSON.stringify(await observeHeldPlan()));
 console.log(JSON.stringify(await observeReattachedPlan()));
 for(const row of handoff.interruptions)console.log(JSON.stringify(await observe(cases.find(row=>row.name==="exact slice progress")!,row.mode)));
}else test("plan silence identifies the last evidence for this request over a private socket",async()=>{
 const cwd=mkdtempSync(join(testScratchRootV1("resident-plan-silence"),"ps-"));
 try{
  const child=Bun.spawn([process.execPath,fileURLToPath(import.meta.url),"--silence-fixture"],{cwd,stdout:"pipe",stderr:"pipe"});
  const [output,errors,status]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
  expect(status,errors).toBe(0);
  const observed=output.trim().split("\n").map(line=>JSON.parse(line));
  expect(observed).toHaveLength(cases.length+2+handoff.interruptions.length);
  using oracle=new Database(":memory:");
  const decision=oracle.query("SELECT CASE WHEN ?1=0 THEN 'before-welcome' WHEN (?2 OR ?3) THEN 'after-acceptance' ELSE 'before-acceptance' END AS stage");
  const languageEvidence:Record<string,readonly [string,string]>={"before-welcome":["waiting for the service welcome","auf die Begrüßung des Dienstes"],"before-acceptance":["waiting for acknowledgment","auf die Bestätigung"],"after-acceptance":["acknowledged this plan request","hatte diese Plananfrage bestätigt"]};
  for(const [index,row] of cases.entries()){
   const expected=decision.get(Number(row.welcome),Number(row.frames.includes("accepted")),Number(row.frames.includes("progress"))) as {stage:string};
   expect(expected.stage,row.name).toBe(row.stage);
   expect(observed[index],row.name).toMatchObject({reason:"silent",stage:row.stage,phase:row.phase,limit:80,connections:1+(row.extra?HOST_POLICY_V1.reattachMaximum:0),progress:row.progress*(row.extra?1+HOST_POLICY_V1.reattachMaximum:1)});
   expect(observed[index].english,row.name).toContain("The graph service");
   expect(observed[index].german,row.name).toContain("Der Graphdienst");
   expect(observed[index].english,row.name).toContain(languageEvidence[row.stage]![0]);
   expect(observed[index].german,row.name).toContain(languageEvidence[row.stage]![1]);
   expect(observed[index].english,row.name).not.toContain("Der Graphdienst");
   expect(observed[index].german,row.name).not.toContain("The graph service");
  }
  expect(observed[cases.length]?.reply).toBe("plan-reply");
  expect(observed[cases.length]?.keepalives).toBeGreaterThanOrEqual(3);
  const resumed=observed[cases.length+1];
  const lifecycle=oracle.query("SELECT sum(value='start') AS starts, sum(value='abort') AS aborts, sum(value='answer') AS answers FROM json_each(?)");
  expect(lifecycle.get(JSON.stringify(resumed.trace))).toEqual(handoff.lifecycle);
  expect(resumed).toMatchObject({reply:"plan-reply",reports:2,jobs:handoff.lifecycle.starts,coalesced:1});
  expect(observed.slice(cases.length+2)).toEqual(handoff.interruptions.map(({mode,...expected})=>expected));
 }finally{rmSync(cwd,{recursive:true,force:true});}
},15000);
