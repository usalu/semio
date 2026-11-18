/** 🤝️ Contract: a client that asks for the plan of a job that has computed its answer and is still binding it to the clients already attached is answered too; it never waits on an open connection for a reply no one will send. The late client either receives the reply of that job or is attached to a new job.
 * @see ../../🏛️service/🌐️host/🟨️.mjs */
import {afterAll,expect,test} from "bun:test";
import {copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {HERE} from "../🟦️.ts";
import {createHostServiceV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../🔌️ports/🧭️plan/🟨️.mjs";
import {requestPlanV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";

const GRAPH=join(HERE,"🧫️fixtures/🛰️host-graph.json");
const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","la-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));

test("a client attached while the job binds its reply to the first client is answered",async()=>{
 const dir=join(base,"run");mkdirSync(dir,{recursive:true});
 const env={...process.env,XDG_RUNTIME_DIR:dir,TMPDIR:dir,LOCALAPPDATA:dir} as Record<string,string>,implementation=createHash("sha256").update("impl-late-attach").digest("hex"),runtime=hostEndpointsV1(implementation,env);
 const root=join(base,"w");mkdirSync(root,{recursive:true});copyFileSync(GRAPH,join(root,"graph.json"));const workspace=realpathSync(root);
 const inner=createGraphPlannerPortV1({identify:async()=>"k".repeat(64),loadGraph:async()=>JSON.parse(readFileSync(GRAPH,"utf8"))});
 let binding!:()=>void,release!:()=>void;const bindingStarted=new Promise<void>(resolve=>{binding=resolve;}),released=new Promise<void>(resolve=>{release=resolve;});
 const planner={...inner,bind:async()=>{binding();await released;return undefined;}};
 const service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50}});
 const ask=(agent:string)=>requestPlanV1({root:workspace,implementation,targets:["app:test"],agent,language:"en",environment:env,spawnService:async()=>{await service.start();}});
 const first=ask("agent-1");
 await bindingStarted;
 const second=ask("agent-1");
 await Bun.sleep(200);
 release();
 const timeout=new Promise<never>((_,reject)=>setTimeout(()=>reject(new Error("a client was left without a reply")),20000));
 const [one,two]=await Promise.race([Promise.all([first,second]),timeout]);
 expect(one.reply.t).toBe("plan-reply");expect(two.reply.t).toBe("plan-reply");
 await service.stop("test");
},60000);
