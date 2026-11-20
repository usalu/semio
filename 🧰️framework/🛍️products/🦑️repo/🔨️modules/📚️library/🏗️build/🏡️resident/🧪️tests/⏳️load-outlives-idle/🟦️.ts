/** ⏳️ Contract: a workspace whose index is still loading is never closed, neither by the idle sweep nor by the eviction that makes room for another workspace; the plan that waits for the load is answered instead of failing with `closed`.
 * @see ../../🏛️service/🌐️host/🟨️.mjs */
import {afterAll,expect,test} from "bun:test";
import {copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {HERE} from "../🟦️.ts";
import {createHostServiceV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../🔌️ports/🧭️plan/🟨️.mjs";
import {requestHostStatusV1,requestPlanV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";

const GRAPH=join(HERE,"🧫️fixtures/🛰️host-graph.json");
const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","lo-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));

const setup=(name:string,policy:Record<string,number>)=>{
 const dir=join(base,name);mkdirSync(dir,{recursive:true});
 const env={...process.env,XDG_RUNTIME_DIR:dir,TMPDIR:dir,LOCALAPPDATA:dir} as Record<string,string>,implementation=createHash("sha256").update("impl-"+name).digest("hex"),runtime=hostEndpointsV1(implementation,env);
 const roots=["a","b"].map(id=>{const root=join(dir,id);mkdirSync(root,{recursive:true});copyFileSync(GRAPH,join(root,"graph.json"));return realpathSync(root);});
 const gates=new Map<string,{release:()=>void,started:Promise<void>}>();
 const gate=(root:string)=>{let release!:()=>void,begun!:()=>void;const opened=new Promise<void>(resolve=>{release=resolve;}),started=new Promise<void>(resolve=>{begun=resolve;});gates.set(root,{release,started});return{opened,begun};};
 const waits=new Map<string,ReturnType<typeof gate>>();roots.forEach(root=>waits.set(root,gate(root)));
 const planner=createGraphPlannerPortV1({identify:async({root}:any)=>createHash("sha256").update(root).digest("hex"),loadGraph:async({root}:any)=>{const wait=waits.get(root)!;wait.begun();await wait.opened;return JSON.parse(readFileSync(GRAPH,"utf8"));}} as never);
 let clock=1_000_000;
 const service=createHostServiceV1({runtime,implementation,ports:{planner},now:()=>clock,policy:{residentCheckMilliseconds:60000,...policy}} as never);
 const ask=(root:string)=>requestPlanV1({root,implementation,targets:["app:test"],agent:"agent-1",language:"en",environment:env,spawnService:async()=>{await service.start();}});
 const status=async(): Promise<any>=>requestHostStatusV1({implementation,environment:env} as never);
 return{service,roots,gates,ask,status,advance:(milliseconds:number)=>{clock+=milliseconds;}};
};
const timeout=<T>(work:Promise<T>)=>Promise.race([work,new Promise<never>((_,reject)=>setTimeout(()=>reject(new Error("a client was left without a reply")),20000))]);

test("an idle sweep that runs while the index loads leaves the workspace open and the plan is answered",async()=>{
 const h=setup("sweep",{workspaceIdleMilliseconds:1000});
 const first=h.ask(h.roots[0]);
 await h.gates.get(h.roots[0])!.started;
 h.advance(5000);h.service.sweep();
 expect((await h.status()).workspaces.map((w:any)=>w.root)).toEqual([h.roots[0]]);
 h.gates.get(h.roots[0])!.release();
 const answer=await timeout(first);
 expect(answer.reply.t).toBe("plan-reply");
 expect((await h.status()).workspaces.map((w:any)=>w.root)).toEqual([h.roots[0]]);
 await h.service.stop("test");
},60000);

test("a workspace opened at the workspace limit does not evict the workspace that is still loading",async()=>{
 const h=setup("limit",{maximumWorkspaces:1});
 const first=h.ask(h.roots[0]);
 await h.gates.get(h.roots[0])!.started;
 const second=h.ask(h.roots[1]);
 await h.gates.get(h.roots[1])!.started;
 expect((await h.status()).workspaces.map((w:any)=>w.root).sort()).toEqual([...h.roots].sort());
 h.gates.get(h.roots[0])!.release();h.gates.get(h.roots[1])!.release();
 const [one,two]=await timeout(Promise.all([first,second]));
 expect(one.reply.t).toBe("plan-reply");expect(two.reply.t).toBe("plan-reply");
 await h.service.stop("test");
},60000);
