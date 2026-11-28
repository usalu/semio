/** 🫥️ Contract: the silence of one plan is not the silence of the service. A service process that lives but answers nothing (paged out by a swapping host) does not lose the plan of its client, a service that answers a fresh connection while the plan stays quiet has lost the plan and the client asks again, a dead service ends the wait, and a hand-over or a change of the service generation is followed without using up the attempts. The worker of a plan that the host holds is kept audible by the service (a typed notice in the progress line).
 * @see ../../🚪️client/🛰️plan/🟨️.mjs
 * @see ../../🏛️service/🌐️host/🟨️.mjs
 * @see ../💓️keepalive/🟦️.ts */
import {afterAll,expect,test} from "bun:test";
import {copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {createServer,type Socket} from "node:net";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {HostUnavailableErrorV1,probeServiceV1,requestPlanV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {createFrameDecoderV1,encodeFrameV1} from "../../📨️protocol/🟨️.mjs";
import {HOST_POLICY_V1,renderBothV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {createHostServiceV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../🔌️ports/🧭️plan/🟨️.mjs";
import {HERE} from "../🟦️.ts";

const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","qs-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const REPLY={t:"plan-reply",revision:"r",plan:{tasks:[]},progress:[],cacheHits:false};
const fixture=async(name:string,behave:(plan:{n:number;socket:Socket;id:number},connection:{hello:number})=>void,options:{answerHello?:(count:number)=>boolean}={})=>{
 const dir=join(base,name);mkdirSync(dir,{recursive:true});
 const env={...process.env,XDG_RUNTIME_DIR:dir,TMPDIR:dir,LOCALAPPDATA:dir} as Record<string,string>,implementation=createHash("sha256").update("quiet-"+name).digest("hex"),runtime=hostEndpointsV1(implementation,env);
 mkdirSync(runtime.directory,{recursive:true});writeFileSync(runtime.key,Buffer.alloc(32,7));
 const root=join(base,"w-"+name);mkdirSync(root,{recursive:true});
 let plans=0,hellos=0;const sockets=new Set<Socket>(),server=createServer(socket=>{
  sockets.add(socket);
  const decoder=createFrameDecoderV1({maximumFrameBytes:HOST_POLICY_V1.maximumFrameBytes,onError:()=>undefined,onFrame:(frame:any)=>{
   if(frame.t==="hello"){hellos++;if(options.answerHello?.(hellos)!==false)socket.write(encodeFrameV1({t:"welcome",v:1,features:["plan","host-status"],service:{pid:process.pid}}));}
   if(frame.t==="plan"){plans++;behave({n:plans,socket,id:frame.id},{hello:hellos});}
  }});
  socket.on("data",chunk=>decoder.push(chunk));socket.on("error",()=>undefined);
 });
 await new Promise<void>(resolve=>server.listen(runtime.endpoint,resolve));
 const close=async()=>{for(const socket of sockets)socket.destroy();await new Promise<void>(resolve=>server.close(()=>resolve()));};
 const ask=(extra:Record<string,unknown>={})=>requestPlanV1({root,implementation,targets:["app:test"],agent:"a",language:"en",environment:env,arguments:["run","x"],tool:"nx",...extra} as never);
 return {ask,close,runtime,implementation,root,counts:()=>({plans,hellos})};
};
const progress=(socket:Socket,id:number)=>socket.write(encodeFrameV1({t:"progress",id,phase:"load",done:0,total:1,message:{en:"Loading",de:"Lade"}}));

test("a service process that lives but answers nothing does not lose the plan: the client keeps waiting, says so in both languages, and receives the reply when the service wakes",async()=>{
 let starved=true,late!:()=>void;
 const f=await fixture("starved",({socket,id})=>{progress(socket,id);late=()=>socket.write(encodeFrameV1({...REPLY,id}));},{answerHello:count=>count===1||!starved});
 const texts:string[]=[];
 try{
  const asked=f.ask({silenceMilliseconds:150,quietHoldMilliseconds:20000,probe:(input:any)=>probeServiceV1({...input,milliseconds:80}),onProgress:(frame:any)=>{if(frame.phase==="alive")texts.push(frame.message.en+" | "+frame.message.de);}});
  await Bun.sleep(900);
  expect(texts.length).toBeGreaterThan(0);expect(texts[0]).toContain("process lives but answers nothing");expect(texts[0]).toContain("Graphdienst-Prozess lebt");
  expect(f.counts().plans).toBe(1);
  starved=false;late();
  expect((await asked).reply.t).toBe("plan-reply");expect(f.counts().plans).toBe(1);
 }finally{await f.close();}
},20000);

test("a service that answers a fresh connection while the plan stays quiet has lost the plan: silent, and the client asks again",async()=>{
 const f=await fixture("answering",({socket,id,n})=>{progress(socket,id);if(n>=2)socket.write(encodeFrameV1({...REPLY,id}));});
 try{
  const answer=await f.ask({silenceMilliseconds:150,quietHoldMilliseconds:20000,probe:(input:any)=>probeServiceV1({...input,milliseconds:200})});
  expect(answer.reply.t).toBe("plan-reply");expect(f.counts().plans).toBe(2);
 }finally{await f.close();}
},20000);

test("a service nobody listens to any more ends the wait as silent",async()=>{
 const f=await fixture("dead",({socket,id})=>progress(socket,id));
 const first=f.ask({silenceMilliseconds:150,attempts:1,probe:async()=>"dead"}).then(()=>undefined,error=>error);
 const error:any=await first;
 expect(error).toBeInstanceOf(HostUnavailableErrorV1);expect(error.reason).toBe("silent");
 await f.close();
 expect(await probeServiceV1({runtime:f.runtime,implementation:f.implementation,agent:"a",workspace:f.root})).toBe("dead");
},20000);

test("a plan that is held past the declared hold of a starved service is given up as silent",async()=>{
 const f=await fixture("hold",({socket,id})=>progress(socket,id),{answerHello:count=>count===1});
 try{
  const error:any=await f.ask({silenceMilliseconds:100,attempts:1,quietHoldMilliseconds:500,probe:(input:any)=>probeServiceV1({...input,milliseconds:60})}).then(()=>undefined,reason=>reason);
  expect(error.reason).toBe("silent");
 }finally{await f.close();}
},20000);

test("a hand-over refusal after progress is followed: the client asks the successor again, keeps its attempts and says which follow it is",async()=>{
 const f=await fixture("handover",({socket,id,n})=>{progress(socket,id);if(n===1)socket.write(encodeFrameV1({t:"refused",id,code:"handover",message:renderBothV1("host-service-handover",{reason:"superseded"}),retry:"later"}));else socket.write(encodeFrameV1({...REPLY,id}));});
 const lines:string[]=[];
 try{
  const answer=await f.ask({attempts:1,onProgress:(frame:any)=>{if(frame.phase==="start")lines.push(frame.message.en+" | "+frame.message.de);}});
  expect(answer.reply.t).toBe("plan-reply");expect(f.counts().plans).toBe(2);
  expect(lines.join("\n")).toContain("follows the successor");expect(lines.join("\n")).toContain("folgt dem Nachfolger");expect(lines.join("\n")).toContain("follow 1 of "+HOST_POLICY_V1.generationFollowMaximum);
 }finally{await f.close();}
},30000);

test("a lost connection after progress while the code of the service changed follows the new generation without using up the attempts",async()=>{
 const f=await fixture("generation",({socket,id,n})=>{progress(socket,id);if(n<=2)setTimeout(()=>socket.destroy(),30);else socket.write(encodeFrameV1({...REPLY,id}));});
 try{
  const answer=await f.ask({attempts:1,identify:()=>"b".repeat(64)});
  expect(answer.reply.t).toBe("plan-reply");expect(f.counts().plans).toBe(3);
 }finally{await f.close();}
},40000);

test("the declared policy keeps the follow bound, the probe and the hold finite, and the quiet notices exist in both languages",()=>{
 for(const name of["generationFollowMaximum","generationFollowMilliseconds","quietProbeMilliseconds","quietHoldMilliseconds","workerQuietNoticeMilliseconds"])expect(Number.isSafeInteger((HOST_POLICY_V1 as any)[name])&&(HOST_POLICY_V1 as any)[name]>0,name).toBe(true);
 expect(HOST_POLICY_V1.quietProbeMilliseconds).toBeLessThan(HOST_POLICY_V1.planSilenceMilliseconds);
 for(const [key,fields] of[["host-slice-owners-worker-quiet",{seconds:5,limit:40,owners:"",used:0,budget:0,kind:"",minimum:0,floor:0,cause:""}],["host-service-quiet",{seconds:130,held:130,limit:1800}],["host-service-handover-follow",{count:1,limit:64}]] as const){
  const text=renderBothV1(key,fields as never) as {en:string;de:string};
  expect(text.en.length,key).toBeGreaterThan(20);expect(text.de.length,key).toBeGreaterThan(20);expect(text.en).not.toContain("{");expect(text.de).not.toContain("{");
 }
});

test("while the worker of a plan reports nothing (held by the host, waiting for capacity) the service keeps sending typed progress with a notice that says so in both languages",async()=>{
 const dir=join(base,"worker-quiet");mkdirSync(dir,{recursive:true});
 const env={...process.env,XDG_RUNTIME_DIR:dir,TMPDIR:dir,LOCALAPPDATA:dir} as Record<string,string>,implementation=createHash("sha256").update("quiet-worker").digest("hex"),runtime=hostEndpointsV1(implementation,env);
 const root=join(base,"w-worker");mkdirSync(root,{recursive:true});copyFileSync(join(HERE,"🧫️fixtures/🛰️host-graph.json"),join(root,"graph.json"));
 const graph=JSON.parse(readFileSync(join(HERE,"🧫️fixtures/🛰️host-graph.json"),"utf8"));
 let release!:()=>void;const gate=new Promise<void>(resolve=>{release=resolve;});
 const planner=createGraphPlannerPortV1({identify:async()=>"w".repeat(64),loadGraph:async()=>graph,loadSlice:async({onProgress}:any)=>{
  onProgress(10,{phase:"reservation",done:0,total:0,reservation:{requested:2604}},true);
  for(let tick=0;tick<12;tick++){await Bun.sleep(40);onProgress(50+tick*40,{phase:"reservation",done:0,total:0,reservation:{requested:2604,wait:{}}},false);}
  await gate;return {graph};
 }});
 const service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50,requestKeepaliveMilliseconds:100,workerQuietNoticeMilliseconds:120}});
 const lines:string[]=[];
 await service.start();
 try{
  const asked=requestPlanV1({root:realpathSync(root),implementation,targets:["app:test"],agent:"a",language:"en",environment:env,arguments:["run","x"],tool:"nx",onProgress:(frame:any)=>{if(frame.phase==="slice")lines.push(frame.message.en+" | "+frame.message.de);}});
  for(let i=0;i<200&&!lines.some(line=>line.includes("reported nothing"));i++)await Bun.sleep(20);
  const quiet=lines.find(line=>line.includes("reported nothing"));
  expect(quiet).toBeDefined();expect(quiet).toContain("keeps this plan");expect(quiet).toContain("behält diesen Plan");
  release();await asked.catch(()=>undefined);
 }finally{release();await service.stop("test");}
},20000);
