/** 🧭️ Contract: a client that starts the graph service and waits for it listens on every implementation it saw while waiting (the service names itself by the code it reads at boot, so under constant edits it can come up as another implementation than the client computed), connects to the one that answers and says which, announces the move in both languages, and ends a wait that reached the start ceiling with a typed error carrying both languages instead of waiting on silently.
 * @see ../../🚪️client/🛰️plan/🟨️.mjs */
import {expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {createServer} from "node:net";
import {tmpdir} from "node:os";
const shortBase=process.platform==="win32"?tmpdir():"/tmp";
import {join} from "node:path";
import {HostUnavailableErrorV1,connectHostV1,waitMessagesV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";

const listen=(endpoint:string)=>new Promise<import("node:net").Server>(resolve=>{const server=createServer(socket=>socket.destroy());server.listen(endpoint,()=>resolve(server));});

test("the client switches to the implementation the tree moved to and connects to the service that came up on it",async()=>{
 const base=mkdtempSync(join(shortBase,"start-identity-")),environment={XDG_RUNTIME_DIR:base} as Record<string,string>;
 const moved=hostEndpointsV1("moved-implementation",environment),first=hostEndpointsV1("first-implementation",environment);
 expect(moved.endpoint).not.toBe(first.endpoint);
 const progress:any[]=[];let asked=0;
 try{
  const connecting=connectHostV1({implementation:"first-implementation",environment,platform:"linux",languages:["en","de"],spawnService:async()=>({alive:()=>true}),identify:()=>{asked++;return asked<2?"first-implementation":"moved-implementation";},onProgress:(frame:any)=>progress.push(frame),policy:{startCeilingMilliseconds:20000,startTimeoutMilliseconds:20000,startPollMilliseconds:20,startLivenessMilliseconds:20} as never});
  await new Promise(done=>setTimeout(done,50));
  const server=await listen(moved.endpoint);
  const answer=await connecting;answer.socket.destroy();server.close();
  expect(answer.implementation).toBe("moved-implementation");expect(answer.runtime.endpoint).toBe(moved.endpoint);expect(answer.started).toBe(true);
  const said=progress.find(frame=>frame.message?.en?.includes("changed while the graph service started"));
  expect(said?.message?.de).toContain("geändert");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a wait that reaches the start ceiling ends with a typed error in both languages, and the wait texts name their fields",async()=>{
 const base=mkdtempSync(join(shortBase,"start-identity-")),environment={XDG_RUNTIME_DIR:base} as Record<string,string>;
 try{
  const failure:any=await connectHostV1({implementation:"nobody-listens",environment,platform:"linux",languages:["en"],spawnService:async()=>({alive:()=>true}),policy:{startCeilingMilliseconds:300,startTimeoutMilliseconds:300,startPollMilliseconds:20,startLivenessMilliseconds:20} as never}).then(()=>undefined,error=>error);
  expect(failure).toBeInstanceOf(HostUnavailableErrorV1);expect(failure.reason).toBe("timeout");
  expect(failure.message).toContain("did not answer");expect(failure.message).toContain("antwortet");
  for(const language of["en","de"])expect(waitMessagesV1("ceiling",{seconds:1,seen:1},[language])[language]).not.toMatch(/\{\w+\}/u);
 }finally{rmSync(base,{recursive:true,force:true});}
});
