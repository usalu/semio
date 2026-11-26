/** 🔀️ Contract: a host service that a newer one superseded keeps every load in flight that still progresses until it answers on the code it started with (drain), and ends only once idle; a load that observed no original work for the stall bound, and every load still running when the service ends for any other reason than missing file access, is handed over: its client receives the refusal `handover` (retry later, both languages) instead of a dead connection or `shutdown`, asks again, and the service that follows answers it while its progress keeps reaching the person. The drain decision matches an independent SQLite oracle on the shared case table.
 * @see ../../🏛️service/🌐️host/🟨️.mjs `supersededDrainDueV1`, `drainAndExit`
 * @see ../../../🧭️invocation/🛰️plan/🟦️.ts `requestInvocationPlanV1` */
import {afterAll,expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import table from "./🔣️.json" with {type:"json"};
import {copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {HERE} from "../🟦️.ts";
import {createHostServiceV1,supersededDrainDueV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../🔌️ports/🧭️plan/🟨️.mjs";
import {PlanRefusedErrorV1,requestHostStatusV1,requestPlanV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {HOST_POLICY_V1,renderTextV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {requestInvocationPlanV1} from "../../../🧭️invocation/🛰️plan/🟦️.ts";

const GRAPH=join(HERE,"🧫️fixtures/🛰️host-graph.json");
const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","sd-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const within=<T>(work:Promise<T>,milliseconds=20000)=>Promise.race([work,new Promise<never>((_,reject)=>setTimeout(()=>reject(new Error("no answer within "+milliseconds+" ms")),milliseconds))]);

const host=(name:string,env:Record<string,string>,policy:Record<string,unknown>={})=>{
 const implementation=createHash("sha256").update("impl-"+name).digest("hex"),runtime=hostEndpointsV1(implementation,env);
 const inner=createGraphPlannerPortV1({identify:async()=>"k".repeat(64),loadGraph:async()=>JSON.parse(readFileSync(GRAPH,"utf8"))});
 let release!:()=>void,begun!:()=>void,beat:(done:number)=>void=()=>undefined;
 const opened=new Promise<void>(resolve=>{release=resolve;}),started=new Promise<void>(resolve=>{begun=resolve;});
 const state={clock:1_000_000,held:true};
 const planner={...inner,async plan(request:any){
  beat=done=>{void request.progress?.(1000,{phase:name,done,total:7});};
  beat(3);begun();
  if(state.held)await new Promise<void>((resolve,reject)=>{void opened.then(resolve);request.signal?.addEventListener("abort",()=>reject(request.signal.reason),{once:true});});
  return inner.plan(request);
 }};
 const service=createHostServiceV1({runtime,implementation,ports:{planner},now:()=>state.clock,policy:{residentCheckMilliseconds:60000,...policy}} as never);
 return{service,implementation,runtime,state,started,release,beat:(done:number)=>beat(done),advance:(milliseconds:number)=>{state.clock+=milliseconds;},ask:(root:string,onProgress:(frame:any)=>void=()=>undefined)=>requestPlanV1({root,implementation,targets:["app:test"],agent:"agent-1",language:"en",environment:env,onProgress,spawnService:async()=>{await service.start();}})};
};
const setup=(name:string)=>{
 const dir=join(base,name);mkdirSync(dir,{recursive:true});
 const env={...process.env,XDG_RUNTIME_DIR:dir,TMPDIR:dir,LOCALAPPDATA:dir} as Record<string,string>,root=join(dir,"w");
 mkdirSync(root,{recursive:true});copyFileSync(GRAPH,join(root,"graph.json"));
 return{env,root:realpathSync(root)};
};
const supersede=(runtime:{directory:string;tag:string})=>writeFileSync(join(runtime.directory,"host-"+(runtime.tag==="0".repeat(12)?"1":"0").repeat(12)+".json"),JSON.stringify({v:1,pid:process.pid,instance:"newer",startedAt:9_000_000_000_000,endpoint:"none",protocol:1,implementation:"f".repeat(64)}));

test("the drain decision of every case of the table equals the table and the SQLite oracle",()=>{
 const db=new Database(":memory:");
 try{
  const oracle=db.query<{due:number},[number|null,number,number,number,number,number,number]>("SELECT ?1 IS NOT NULL AND ?2-?1>=?6 AND ((?4=0 AND ?5=0) OR (?5>0 AND ?2-MAX(?1,?3)>=?7)) AS due");
  for(const row of table.cases){
   const due=supersededDrainDueV1({supersededSinceMs:row.supersededSinceMs??undefined,nowMs:row.nowMs,lastProgressMs:row.lastProgressMs,connections:row.connections,inFlight:row.inFlight,graceMilliseconds:table.graceMilliseconds,stallMilliseconds:table.stallMilliseconds});
   expect(due,row.name).toBe(row.due);
   expect(due,row.name).toBe(Boolean(oracle.get(row.supersededSinceMs,row.nowMs,row.lastProgressMs,row.connections,row.inFlight,table.graceMilliseconds,table.stallMilliseconds)!.due));
  }
 }finally{db.close();}
 expect(HOST_POLICY_V1.handoverFlushMilliseconds).toBeGreaterThan(0);
 const text=renderTextV1("host-service-handover",{reason:"superseded"},["en","de"])!;
 for(const language of["en","de"] as const){expect(text[language]).toContain("superseded");expect(text[language]).not.toMatch(/\{[a-z]+\}/u);}
});

test("a superseded service keeps a progressing load until it answers on its own code, and retires once idle",async()=>{
 const{env,root}=setup("drain"),h=host("drain",env,{supersededGraceMilliseconds:1000,reload:{drainStallMilliseconds:30000}});
 const answer=h.ask(root);
 await within(h.started);
 supersede(h.runtime);h.service.sweep();
 for(const step of[20000,20000,20000]){h.advance(step);h.beat(4);h.service.sweep();}
 expect((await requestHostStatusV1({implementation:h.implementation,environment:env} as never))?.inFlight,"60 s after the supersession the progressing load still runs").toBe(1);
 h.release();
 expect((await within(answer)).reply.t).toBe("plan-reply");
 let reason:string|undefined;void h.service.closed.then(value=>{reason=value;});
 for(let turn=0;turn<200&&reason===undefined;turn++){h.advance(2000);h.service.sweep();await Bun.sleep(20);}
 expect(reason).toBe("superseded");
},60000);

test("a superseded load without original progress for the stall bound is handed over to its client, in both languages",async()=>{
 const{env,root}=setup("stall"),h=host("stall",env,{supersededGraceMilliseconds:1000,reload:{drainStallMilliseconds:30000}});
 const answer=h.ask(root).then(()=>undefined,(error:unknown)=>error);
 await within(h.started);
 supersede(h.runtime);h.service.sweep();h.advance(31000);
 const ticking=setInterval(()=>h.advance(1000),5);
 try{h.service.sweep();const error=await within(answer) as PlanRefusedErrorV1&{message_:Record<string,string>};
  expect(error).toBeInstanceOf(PlanRefusedErrorV1);expect(error.code).toBe("handover");expect(error.retry).toBe("later");
  expect(error.message_.en).toContain("superseded");expect(error.message_.de).toContain("superseded");expect(error.message_.de).toContain("nachfolgenden Dienst");
  expect(await within(h.service.closed)).toBe("superseded");
 }finally{clearInterval(ticking);}
},60000);

test("a service that stops while a load runs hands it over instead of ending it with shutdown",async()=>{
 const{env,root}=setup("stop"),h=host("stop",env);
 const answer=h.ask(root).then(()=>undefined,(error:unknown)=>error);
 await within(h.started);
 const ticking=setInterval(()=>h.advance(1000),5);
 try{void h.service.stop("stop");const error=await within(answer) as PlanRefusedErrorV1&{message_:Record<string,string>};
  expect(error.code).toBe("handover");expect(error.retry).toBe("later");expect(error.message_.en).toContain("(stop)");
 }finally{clearInterval(ticking);}
},60000);

test("after a hand-over the client asks again, the following service answers, and progress of both reaches the person",async()=>{
 const{env,root}=setup("reattach"),first=host("first",env,{reload:{drainStallMilliseconds:30000}}),second=host("second",env);
 second.state.held=false;
 const written:string[]=[],answered:{instance?:string}[]=[];let asked=0;
 const ticking=setInterval(()=>first.advance(1000),5);
 const handOver=first.started.then(()=>first.service.stop("reload"));
 try{
  const outcome=await within(requestInvocationPlanV1({root:process.cwd(),request:{arguments:["run","app:test"],tool:"nx",targets:["app:test"]} as never,environment:{SEMIO_BUILD_TOOL:"nx"},agent:"agent-1",write:(line:string)=>written.push(line),ports:{
   requestPlanV1:async(options:any)=>{const target=asked++===0?first:second;const result=await requestPlanV1({...options,root,implementation:target.implementation,identify:undefined,environment:env,spawnService:async()=>{await target.service.start();}});answered.push({instance:result.welcome?.service?.instance});return result;},
   spawnHostServiceV1:()=>undefined,probeConsentV1:async()=>({state:"unprotected",folder:"Documents"}),
  }} as never).then(()=>undefined,(error:unknown)=>error));
  await handOver;
  expect(asked).toBe(2);expect(answered.map(row=>row.instance)).toEqual([second.service.instance]);
  expect(String((outcome as Error)?.message)).toContain("answered without a graph slice");
  const handed=written.findIndex(line=>line.includes("handed over")&&line.includes("übergeben"));
  expect(handed).toBeGreaterThan(-1);
  expect(written.slice(0,handed).some(line=>/first 3 (of|von) 7/u.test(line)),"progress of the first service before the hand-over").toBe(true);
  expect(written.slice(handed+1).some(line=>/second 3 (of|von) 7/u.test(line)),"progress of the following service after it").toBe(true);
 }finally{clearInterval(ticking);await second.service.stop("test");}
},60000);
