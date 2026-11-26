/** 🧭️ Contract: a client that starts the graph service and waits for it listens on every implementation it saw while waiting (the service names itself by the code it reads at boot, so under constant edits it can come up as another implementation than the client computed), connects to the one that answers and says which, announces the move in both languages, and ends a wait that reached the start ceiling with a typed error carrying both languages instead of waiting on silently.
 * @see ../../🚪️client/🛰️plan/🟨️.mjs */
import {expect,test} from "bun:test";
import {appendFileSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {createServer} from "node:net";
import {tmpdir} from "node:os";
const startPlatform=process.platform==="win32"?"win32":"linux",shortBase=startPlatform==="win32"?tmpdir():"/tmp";
const privateEnvironment=(base:string):Record<string,string>=>startPlatform==="win32"?{LOCALAPPDATA:base}:{XDG_RUNTIME_DIR:base};
import {basename,join} from "node:path";
import {Database} from "bun:sqlite";
import {refusalOfPlanV1} from "../../../🧭️invocation/🛰️plan/🟦️.ts";
import {PlanRefusedErrorV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import waitSchema from "../../🚪️client/🛰️plan/🧬️schema/🔣️.json" with {type:"json"};
import {HostUnavailableErrorV1,connectHostV1,waitMessagesV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";

const listen=(endpoint:string)=>new Promise<import("node:net").Server>(resolve=>{const server=createServer(socket=>socket.destroy());server.listen(endpoint,()=>resolve(server));});

test("the client switches to the implementation the tree moved to and connects to the service that came up on it",async()=>{
 const base=mkdtempSync(join(shortBase,"start-identity-")),environment=privateEnvironment(base),firstCode=basename(base)+"-first",movedCode=basename(base)+"-moved";
 const moved=hostEndpointsV1(movedCode,environment,startPlatform),first=hostEndpointsV1(firstCode,environment,startPlatform);
 expect(moved.endpoint).not.toBe(first.endpoint);
 const progress:any[]=[];let asked=0;
 try{
  const connecting=connectHostV1({implementation:firstCode,environment,platform:startPlatform,languages:["en","de"],spawnService:async()=>({alive:()=>true}),identify:()=>{asked++;return asked<2?firstCode:movedCode;},onProgress:(frame:any)=>progress.push(frame),policy:{startCeilingMilliseconds:20000,startTimeoutMilliseconds:20000,startPollMilliseconds:20,startLivenessMilliseconds:20} as never});
  await new Promise(done=>setTimeout(done,50));
  const server=await listen(moved.endpoint);
  const answer=await connecting;answer.socket.destroy();server.close();
  expect(answer.implementation).toBe(movedCode);expect(answer.runtime.endpoint).toBe(moved.endpoint);expect(answer.started).toBe(true);
  const said=progress.find(frame=>frame.message?.en?.includes("changed while the graph service started"));
  expect(said?.message?.de).toContain("geändert");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a wait that reaches the start ceiling ends with a typed error in both languages, and the wait texts name their fields",async()=>{
 const base=mkdtempSync(join(shortBase,"start-identity-")),environment=privateEnvironment(base);
 try{
  const failure:any=await connectHostV1({implementation:basename(base)+"-nobody-listens",environment,platform:startPlatform,languages:["en"],spawnService:async()=>({alive:()=>true}),policy:{startCeilingMilliseconds:300,startTimeoutMilliseconds:300,startPollMilliseconds:20,startLivenessMilliseconds:20} as never}).then(()=>undefined,error=>error);
  expect(failure).toBeInstanceOf(HostUnavailableErrorV1);expect(failure.reason).toBe("timeout");
  expect(failure.message).toContain("did not answer");expect(failure.message).toContain("antwortet");
  for(const language of["en","de"])expect(waitMessagesV1("ceiling",{seconds:1,seen:1},[language])[language]).not.toMatch(/\{\w+\}/u);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("startup refusal keeps the live ceiling, unknown PID, and bounded logs for both candidate endpoints",async()=>{
 const db=new Database(":memory:");
 try{
  for(const row of waitSchema.definitions.TimeoutCases.const){
   const base=mkdtempSync(join(shortBase,"start-identity-")),environment=privateEnvironment(base),implementation=basename(base)+"-first",next=basename(base)+"-moved",first=hostEndpointsV1(implementation,environment,startPlatform),moved=hostEndpointsV1(next,environment,startPlatform),alive=row.pid!==null;
   try{
    mkdirSync(first.directory,{recursive:true,mode:0o700});mkdirSync(moved.directory,{recursive:true,mode:0o700});
    writeFileSync(first.log,"expired résumé 🙂 line\n".repeat(1200));
    let calls=0;
    const failure:any=await connectHostV1({implementation,environment,platform:startPlatform,languages:["en","de"],identify:()=>++calls>1&&alive?next:implementation,spawnService:()=>{
     appendFileSync(first.log,"first startup marker\n");
     if(alive){writeFileSync(moved.log,"ignored large moved line with résumé 🙂\n".repeat(1200));appendFileSync(moved.log,"moved startup marker\n");}
     return alive?{pid:row.pid,alive:()=>true}:undefined;
    },policy:{startCeilingMilliseconds:2350,startTimeoutMilliseconds:250,startPollMilliseconds:20,startLivenessMilliseconds:20}}).then(()=>undefined,error=>error);
    expect(failure,String(failure?.stack)).toBeInstanceOf(HostUnavailableErrorV1);
    expect(failure.reason).toBe("timeout");
    expect(failure.startup.mode).toBe(row.mode);expect(failure.startup.pid).toBe(row.pid);expect(failure.startup.seen).toBe(row.implementationCount);
    const oracle=db.query("select 1+(?1 is not null) seen, ?1 pid").get(row.pid);
    expect<unknown>({seen:failure.startup.seen,pid:failure.startup.pid}).toEqual(oracle);
    expect(failure.startup.candidates.map((candidate:{tag:string})=>candidate.tag)).toEqual(alive?[first.tag,moved.tag]:[first.tag]);
    expect(failure.startup.candidates[0].log).toBe("first startup marker");
    if(alive)expect(failure.startup.candidates[1].log).toContain("moved startup marker");
    const rendered:any=refusalOfPlanV1(failure,"de",{HostUnavailableErrorV1,PlanRefusedErrorV1});
    expect(rendered.values.reason).toContain(alive?"PID 4242":"kein laufender Prozess");
    expect(rendered.values.reason).toContain(first.tag);expect(rendered.values.reason).toContain("first startup marker");expect(rendered.values.reason).not.toContain("expired résumé");
    expect(rendered.values.reason.length).toBeLessThan(760);
   }finally{rmSync(base,{recursive:true,force:true});}
  }
 }finally{db.close();}
},20000);
