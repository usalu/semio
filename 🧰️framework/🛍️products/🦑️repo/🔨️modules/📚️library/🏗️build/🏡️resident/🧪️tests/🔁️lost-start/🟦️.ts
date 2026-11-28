/** 🔁️ Contract: a graph service that ended while it started (no exit record: its sources moved under a landing) is started again by the request, bounded and with a bilingual notice; only a service lost every time ends the request as `lost`, and a restart that succeeds answers the request.
 * @see ../../🚪️client/🛰️plan/🟨️.mjs */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {createHostServiceV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../🔌️ports/🧭️plan/🟨️.mjs";
import {requestPlanV1,HostUnavailableErrorV1,HOST_LOST_START_RESTARTS_V1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";

const base=mkdtempSync(join(process.platform==="win32"?process.env.TEMP??"C:\\Temp":"/tmp","ls-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const environment=(name:string)=>{const dir=join(base,"run-"+name);mkdirSync(dir,{recursive:true});return{...process.env,XDG_RUNTIME_DIR:dir,TMPDIR:dir,LOCALAPPDATA:dir} as Record<string,string>;};
const workspace=(name:string)=>{const root=join(base,name);mkdirSync(root,{recursive:true});writeFileSync(join(root,"graph.json"),"{}");return realpathSync(root);};
const digest=(name:string)=>Buffer.from(name.padEnd(32,"x")).toString("hex").slice(0,64).padEnd(64,"0");

test("the policy is bounded",()=>{expect(HOST_LOST_START_RESTARTS_V1.maximum).toBeGreaterThan(0);expect(HOST_LOST_START_RESTARTS_V1.maximum).toBeLessThanOrEqual(5);});

test("a service lost at every start ends the request as lost after exactly the bounded number of restarts, each announced in both languages",async()=>{
  const env=environment("always"),implementation=digest("impl-always"),root=workspace("always-w"),notices:Record<string,string>[]=[];let spawns=0;
  const error=await requestPlanV1({root,implementation,targets:["core:build"],agent:"a",language:"en",environment:env,lostStarts:{maximum:2,milliseconds:1},spawnService:()=>{spawns++;return{alive:()=>false};},onProgress:frame=>{if(frame.message)notices.push(frame.message);}}).then(()=>undefined,reason=>reason as any);
  expect(error).toBeInstanceOf(HostUnavailableErrorV1);expect(error.reason).toBe("lost");
  expect(spawns).toBe(3);
  const restarts=notices.filter(notice=>notice.en.includes("starting it again"));
  expect(restarts.length).toBe(2);expect(restarts[0]!.de).toContain("erneut gestartet");expect(restarts[1]!.en).toContain("2 of 2");
});

test("a service that is lost once and starts the second time answers the request",async()=>{
  const env=environment("once"),implementation=digest("impl-once"),runtime=hostEndpointsV1(implementation,env),root=workspace("once-w");
  const planner=createGraphPlannerPortV1({identify:async()=>"id",loadGraph:async()=>{throw new Error("boom");}});
  let spawns=0,service:ReturnType<typeof createHostServiceV1>|undefined;
  try{
   const error=await requestPlanV1({root,implementation,targets:["core:build"],agent:"a",language:"en",environment:env,lostStarts:{maximum:2,milliseconds:1},spawnService:async()=>{spawns++;if(spawns===1)return{alive:()=>false};service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50}});await service.start();return{alive:()=>true};}}).then(()=>undefined,reason=>reason as any);
   expect(spawns).toBe(2);expect(error?.name).not.toBe("HostUnavailableErrorV1");expect(error?.code).toBe("internal");
  }finally{await service?.stop("test");}
},20000);
