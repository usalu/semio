/** 🔐️ Contract: the capability token of a plan request is refused with the reason that is true (missing, signed with another key, expired), in both languages and as retry later; a client asks again with a fresh handshake (it reads the key file of that service again and mints a new token) up to the declared bound and does not report the refusal as fatal; every generation has its own key file, so a draining older generation and a younger one never judge a token against each other's key.
 * @see ../../🏛️service/🌐️host/🟨️.mjs
 * @see ../../🚪️client/🛰️plan/🟨️.mjs */
import {afterAll,expect,test} from "bun:test";
import {copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {connect} from "node:net";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {HERE} from "../🟦️.ts";
import {createHostServiceV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../🔌️ports/🧭️plan/🟨️.mjs";
import {PlanRefusedErrorV1,requestPlanV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {createFrameDecoderV1,encodeFrameV1} from "../../📨️protocol/🟨️.mjs";
import {HOST_POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {hostEndpointsV1,workspaceIdV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";

const GRAPH=join(HERE,"🧫️fixtures/🛰️host-graph.json");
const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","ct-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const setup=async(name:string,options:Record<string,unknown>={})=>{
 const dir=join(base,name);mkdirSync(dir,{recursive:true});
 const env={...process.env,XDG_RUNTIME_DIR:dir,TMPDIR:dir,LOCALAPPDATA:dir} as Record<string,string>,implementation=createHash("sha256").update("ct-"+name).digest("hex"),runtime=hostEndpointsV1(implementation,env);
 const root=join(base,"w-"+name);mkdirSync(root,{recursive:true});copyFileSync(GRAPH,join(root,"graph.json"));
 const planner=createGraphPlannerPortV1({identify:async()=>"c".repeat(64),loadGraph:async()=>JSON.parse(readFileSync(GRAPH,"utf8"))});
 const service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50},...options});
 await service.start();
 const ask=(extra:Record<string,unknown>={})=>requestPlanV1({root:realpathSync(root),implementation,targets:["app:test"],agent:"a",language:"en",environment:env,...extra} as never);
 return {service,runtime,ask,env,implementation,root:realpathSync(root)};
};
const raw=(endpoint:string,frames:unknown[])=>new Promise<any[]>(resolve=>{
 const got:any[]=[],socket=connect(endpoint),decoder=createFrameDecoderV1({maximumFrameBytes:HOST_POLICY_V1.maximumFrameBytes,onError:()=>undefined,onFrame:(frame:any)=>{got.push(frame);if(frame.t==="refused"){socket.destroy();resolve(got);}}});
 socket.on("data",chunk=>decoder.push(chunk));socket.on("connect",()=>{for(const frame of frames)socket.write(encodeFrameV1(frame as never));});setTimeout(()=>{socket.destroy();resolve(got);},3000);
});

test("every generation has its own key file: two implementations of one checkout never share a key",()=>{
 const env={XDG_RUNTIME_DIR:join(base,"keys")} as Record<string,string>,a=hostEndpointsV1("a".repeat(64),env),b=hostEndpointsV1("b".repeat(64),env);
 expect(a.directory).toBe(b.directory);expect(a.key).not.toBe(b.key);expect(a.key.endsWith("key-"+a.tag)).toBe(true);
});

test("a token with no capability is refused as missing, in both languages, as retry later",async()=>{
 const {service,runtime,root}=await setup("missing");
 try{
  const frames=await raw(runtime.endpoint,[{t:"hello",v:1,client:{name:"t",pid:1,lane:"a"},languages:["en","de"]},{t:"plan",id:1,workspace:workspaceIdV1(root),root,targets:["app:test"],agent:"a",priority:5,language:"en"}]);
  const refused=frames.find(frame=>frame.t==="refused");
  expect(refused).toMatchObject({code:"capability",retry:"later"});expect(refused.message.en).toContain("was missing");expect(refused.message.de).toContain("fehlte");
 }finally{await service.stop("test");}
},15000);

test("a token signed with another key is refused as forged and the client, after it read the key of this service again, succeeds without reporting a failure",async()=>{
 const {service,runtime,ask}=await setup("forged"),right=readFileSync(runtime.key);
 try{
  writeFileSync(runtime.key,Buffer.alloc(32,9));
  const lines:string[]=[];
  const answer=await ask({onProgress:(frame:any)=>{if(frame.phase==="start"){lines.push(frame.message.en+" | "+frame.message.de);writeFileSync(runtime.key,right);}}});
  expect(answer.reply.t).toBe("plan-reply");
  expect(lines.join("\n")).toContain("signed with another key");expect(lines.join("\n")).toContain("anderen Schlüssel signiert");
 }finally{await service.stop("test");}
},20000);

test("a token that expired before the service read it is refused as expired and the client asks again with a fresh token",async()=>{
 let skew=HOST_POLICY_V1.capabilityLifetimeMilliseconds+5000;
 const {service,ask}=await setup("expired",{now:()=>Date.now()+skew});
 try{
  const lines:string[]=[];
  const answer=await ask({onProgress:(frame:any)=>{if(frame.phase==="start"){lines.push(frame.message.en+" | "+frame.message.de);skew=0;}}});
  expect(answer.reply.t).toBe("plan-reply");
  expect(lines.join("\n")).toContain("had expired");expect(lines.join("\n")).toContain("abgelaufen");
 }finally{await service.stop("test");}
},20000);

test("a refusal that stays is reported after the declared number of fresh handshakes, as the typed refusal with both languages",async()=>{
 const {service,runtime,ask}=await setup("stays");
 try{
  writeFileSync(runtime.key,Buffer.alloc(32,9));
  const error:any=await ask().then(()=>undefined,reason=>reason);
  expect(error).toBeInstanceOf(PlanRefusedErrorV1);expect(error.code).toBe("capability");expect(error.message_.de).toContain("anderen Schlüssel");
  expect(HOST_POLICY_V1.capabilityRetryMaximum).toBeGreaterThan(0);expect(HOST_POLICY_V1.capabilityLifetimeMilliseconds).toBeGreaterThan(HOST_POLICY_V1.planSilenceMilliseconds);
 }finally{await service.stop("test");}
},30000);

test("forged token after a follow to the successor, then retry, then success: the plan of a client that started on the older implementation is answered by the successor and the process has a result",async()=>{
 const successor=await setup("follow-b"),older=createHash("sha256").update("ct-follow-a").digest("hex"),right=readFileSync(successor.runtime.key);
 try{
  writeFileSync(successor.runtime.key,Buffer.alloc(32,9));
  const lines:string[]=[];
  const answer=await requestPlanV1({root:successor.root,implementation:older,identify:()=>successor.implementation,spawnService:async()=>undefined,targets:["app:test"],agent:"a",language:"en",environment:successor.env,onProgress:(frame:any)=>{
   const text=frame.message?.en??"";lines.push(text);if(text.includes("signed with another key"))writeFileSync(successor.runtime.key,right);
  }} as never);
  expect(answer.reply.t).toBe("plan-reply");
  expect(lines.some(line=>line.includes("signed with another key"))).toBe(true);
 }finally{await successor.service.stop("test");}
},60000);
