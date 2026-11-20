import {createGovernorV1} from "../../🏛️service/🚦️governor/🟨️.mjs";
import waitCase from "./🔣️.json";
/** 💓️ Contract: no wait for the graph service is unbounded and silent. A service that holds a request keeps it audible (a keepalive frame while a job holds the client or before it is attached), a request the service lost (a reply that never comes) ends on the client with the bilingual failure `silent` after the declared silence and is asked again, and a keepalive never lets a lost request look alive.
 * @see ../../🏛️service/🌐️host/🟨️.mjs
 * @see ../../🚪️client/🛰️plan/🟨️.mjs */
import {afterAll,expect,test} from "bun:test";
import {copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {createServer} from "node:net";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {HERE} from "../🟦️.ts";
import {createHostServiceV1,keepaliveDueV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../🔌️ports/🧭️plan/🟨️.mjs";
import {HostUnavailableErrorV1,requestPlanV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {createFrameDecoderV1,encodeFrameV1} from "../../📨️protocol/🟨️.mjs";
import {HOST_POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";

const GRAPH=join(HERE,"🧫️fixtures/🛰️host-graph.json");
const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","ka-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const environmentOf=(name:string)=>{const dir=join(base,name);mkdirSync(dir,{recursive:true});return{dir,env:{...process.env,XDG_RUNTIME_DIR:dir,TMPDIR:dir,LOCALAPPDATA:dir} as Record<string,string>};};
const workspaceOf=(name:string)=>{const root=join(base,"w-"+name);mkdirSync(root,{recursive:true});copyFileSync(GRAPH,join(root,"graph.json"));return realpathSync(root);};

test("the declared policy names a keepalive that is shorter than the silence deadline",()=>{
 expect(HOST_POLICY_V1.requestKeepaliveMilliseconds).toBeGreaterThan(0);
 expect(HOST_POLICY_V1.planSilenceMilliseconds).toBeGreaterThanOrEqual(HOST_POLICY_V1.requestKeepaliveMilliseconds*3);
});

test("a keepalive is due only after the quiet interval and only while the service holds the request",()=>{
 const quiet={sentAt:0,now:1000,intervalMilliseconds:1000};
 expect(keepaliveDueV1({...quiet,stage:"starting",held:undefined})).toBe(true);
 expect(keepaliveDueV1({...quiet,stage:"attached",held:true})).toBe(true);
 expect(keepaliveDueV1({...quiet,stage:"attached",held:false})).toBe(false);
 expect(keepaliveDueV1({...quiet,stage:"attached",held:undefined})).toBe(false);
 expect(keepaliveDueV1({...quiet,now:999,stage:"attached",held:true})).toBe(false);
});

test("a request whose reply is lost ends with the bilingual failure silent instead of waiting for ever, and is asked again",async()=>{
 const{dir,env}=environmentOf("lost"),implementation=createHash("sha256").update("impl-keepalive-lost").digest("hex"),runtime=hostEndpointsV1(implementation,env),workspace=workspaceOf("lost");
 mkdirSync(runtime.directory,{recursive:true});writeFileSync(runtime.key,Buffer.alloc(32,7));
 let connections=0,plans=0;const sockets=new Set<import("node:net").Socket>();
 const server=createServer(socket=>{
  connections++;sockets.add(socket);
  const decoder=createFrameDecoderV1({maximumFrameBytes:HOST_POLICY_V1.maximumFrameBytes,onError:()=>undefined,onFrame:(frame:any)=>{
   if(frame.t==="hello")socket.write(encodeFrameV1({t:"welcome",v:1,features:["plan","host-status"],service:{pid:process.pid}}));
   if(frame.t==="plan"){plans++;socket.write(encodeFrameV1({t:"progress",id:frame.id,phase:"load",done:0,total:1,message:{en:"Loading",de:"Lade"}}));}
  }});
  socket.on("data",chunk=>decoder.push(chunk));socket.on("error",()=>undefined);
 });
 await new Promise<void>(resolve=>server.listen(runtime.endpoint,resolve));
 const ask=(silenceMilliseconds:number,signal?:AbortSignal)=>requestPlanV1({root:workspace,implementation,targets:["app:test"],agent:"a",language:"en",environment:env,silenceMilliseconds,...signal?{signal}:{},arguments:["run","x"],tool:"nx"});
 const cancel=new AbortController(),unbounded=await Promise.race([ask(0,cancel.signal).then(()=>"answered",()=>"failed"),new Promise<string>(resolve=>setTimeout(()=>resolve("still waiting"),800))]);
 expect(unbounded).toBe("still waiting");cancel.abort(new Error("done"));
 const began=Date.now(),error:any=await ask(200).then(()=>undefined,reason=>reason);
 expect(error).toBeInstanceOf(HostUnavailableErrorV1);expect(error.reason).toBe("silent");
 expect(error.message).toContain("sent nothing for");expect(error.message).toContain("nichts gesendet");
 expect(Date.now()-began).toBeLessThan(10000);expect(plans).toBeGreaterThan(1);expect(connections).toBeGreaterThan(1);
 for(const socket of sockets)socket.destroy();await new Promise<void>(resolve=>server.close(()=>resolve()));rmSync(dir,{recursive:true,force:true});
},30000);

test("a service that holds a slow request keeps it audible, so a short silence deadline does not end it",async()=>{
 const{env}=environmentOf("slow"),implementation=createHash("sha256").update("impl-keepalive-slow").digest("hex"),runtime=hostEndpointsV1(implementation,env),workspace=workspaceOf("slow");
 const inner=createGraphPlannerPortV1({identify:async()=>"k".repeat(64),loadGraph:async()=>JSON.parse(readFileSync(GRAPH,"utf8"))});
 let release!:()=>void;const gate=new Promise<void>(resolve=>{release=resolve;});
 const planner={...inner,plan:async(options:any)=>{await gate;return inner.plan(options);}};
 const service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50,requestKeepaliveMilliseconds:100}});
 const phases:string[]=[],texts:string[]=[];
 const asked=requestPlanV1({root:workspace,implementation,targets:["app:test"],agent:"a",language:"en",environment:env,silenceMilliseconds:600,spawnService:async()=>{await service.start();},onProgress:(frame:any)=>{phases.push(frame.phase);if(frame.phase==="alive")texts.push(frame.message.en+" | "+frame.message.de);}});
 await Bun.sleep(1500);release();
 const answer=await asked;
 expect(answer.reply.t).toBe("plan-reply");expect(phases.filter(phase=>phase==="alive").length).toBeGreaterThanOrEqual(3);
 expect(texts[0]).toContain("holds this request");expect(texts[0]).toContain("hält diese Anfrage");
 await service.stop("test");
},30000);

test("the first and joined clients learn the real slot wait before the slot opens",async()=>{
 const {env}=environmentOf("admission"),implementation=createHash("sha256").update("queue-replay").digest("hex");
 const runtime=hostEndpointsV1(implementation,env),root=workspaceOf("admission");
 const admission=createGovernorV1({maximumInFlight:1,maximumQueued:8});
 const release=await admission.enter("occupied",new AbortController().signal);
 let finish!:()=>void;
 const loading=new Promise<void>(resolve=>{finish=resolve;});
 const planner=createGraphPlannerPortV1({identify:async()=>"q".repeat(64),loadGraph:async()=>{await loading;return JSON.parse(readFileSync(GRAPH,"utf8"));}});
 const service=createHostServiceV1({runtime,implementation,ports:{planner,admission},policy:{requestKeepaliveMilliseconds:80}});
 const frames:any[][]=[[],[]];
 const ask=(i:number)=>requestPlanV1({root,implementation,targets:["app:test"],agent:"a",language:"en",environment:env,onProgress:(frame:any)=>frames[i]!.push(frame)});
 const asked:Promise<any>[]=[];
 try{
  await service.start();asked.push(ask(0));
  for(let i=0;i<150&&admission.queued!==1;i++)await Bun.sleep(10);
  expect(admission.queued).toBe(1);
  asked.push(ask(1));
  for(let i=0;i<150&&!frames.every(group=>group.some(f=>f.phase===waitCase.phase));i++)await Bun.sleep(10);
  for(const group of frames){
   const message=group.find(f=>f.phase===waitCase.phase)?.message;
   expect(message?.en).toContain(waitCase.en);expect(message?.de).toContain(waitCase.de);expect(message?.en).toContain(String(waitCase.ahead));
  }
  release();
  for(let i=0;i<150&&!frames[0]!.some(f=>f.phase==="load");i++)await Bun.sleep(10);
  expect(frames[0]!.some(f=>f.phase==="load")).toBe(true);
  frames.push([]);asked.push(ask(2));
  for(let i=0;i<150&&!frames[2]!.some(f=>f.phase==="load");i++)await Bun.sleep(10);
  expect(frames[2]!.some(f=>f.phase==="load")).toBe(true);
  expect(frames[2]!.some(f=>f.phase===waitCase.phase)).toBe(false);
  finish();
  for(const answer of await Promise.all(asked))expect(answer.reply.t).toBe("plan-reply");
 }finally{
  release();finish();await Promise.allSettled(asked);await service.stop("test");
 }
},15000);

for(const row of waitCase.beforeAdmission)test("quiet startup identifies the active planner operation "+row.method,async()=>{
 const {env}=environmentOf(row.method),implementation=createHash("sha256").update(row.method).digest("hex");
 const runtime=hostEndpointsV1(implementation,env),root=workspaceOf(row.method);
 let release!:()=>void;
 const gate=new Promise<void>(resolve=>{release=resolve;});
 const inner=createGraphPlannerPortV1({loadGraph:async()=>JSON.parse(readFileSync(GRAPH,"utf8"))});
 const planner={...inner,identify:async()=>{if(row.method==="identify")await gate;return "i".repeat(64);},
  contentDigest:async()=>{if(row.method==="contentDigest")await gate;return "d".repeat(64);},bind:({reply}:any)=>reply};
 const service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{requestKeepaliveMilliseconds:80}});
 const frames:any[]=[];
 await service.start();
 const answer=requestPlanV1({root,implementation,targets:["app:test"],agent:"a",language:"en",environment:env,onProgress:(frame:any)=>frames.push(frame)});
 try{
  for(let i=0;i<150&&!frames.some(f=>f.phase==="alive");i++)await Bun.sleep(10);
  const alive=frames.find(f=>f.phase==="alive");
  expect(alive?.message.en).toContain(row.en);expect(alive?.message.de).toContain(row.de);
  expect([alive.done,alive.total]).toEqual([0,0]);
 }finally{release();try{await answer;}finally{await service.stop("test");}}
},15000);

test("a reset during endpoint connection has the same availability outcome as a reset after connection",async()=>{
 const modulePath=new URL("../../🚪️client/🛰️plan/🟨️.mjs",import.meta.url).href;
 const fixture=waitCase.connectErrors.map(([code])=>code);
 const script=`import {Socket} from "node:net";
 let code;
 Socket.prototype.connect=function(){queueMicrotask(()=>this.emit("error",Object.assign(Error(code),{code})));return this;};
 const {requestHostStatusV1}=await import(${JSON.stringify(modulePath)});
 const outcomes=[];
 for(code of ${JSON.stringify(fixture)}){try{const status=await requestHostStatusV1({implementation:"a".repeat(64)});outcomes.push(status??"unavailable");}catch(error){outcomes.push(error.code);}}
 console.log(JSON.stringify(outcomes));`;
 const child=Bun.spawn([process.execPath,"-e",script],{env:environmentOf("reset").env,stdout:"pipe",stderr:"pipe"});
 const [output,errors,exit]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
 expect(exit,errors).toBe(0);expect(JSON.parse(output.trim())).toEqual(waitCase.connectErrors.map(([,outcome])=>outcome));
});
