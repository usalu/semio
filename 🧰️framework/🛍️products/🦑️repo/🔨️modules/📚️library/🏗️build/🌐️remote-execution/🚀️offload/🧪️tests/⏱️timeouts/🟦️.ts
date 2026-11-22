import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startClusterV1} from "../../../🧪️tests/🧰️cluster/🟦️.ts";
import {emitOffloadStageV1,offloadRustcUnitV1} from "../../🟦️.ts";
import {createTestCheckoutV1,testRustEnvironmentV1} from "../🧰️workspace/🟦️.ts";
import {httpRemoteExecutionContentV1} from "../../../🗄️content/🟦️.ts";
import {executeOffloadRidingThroughV1,offloadExecuteBudgetMillisecondsV1,runBoundedOffloadPhaseV1} from "../../🔌️client/🟦️.ts";
import {OFFLOAD_POLICY_V1} from "../../📐️policy/🟦️.ts";

const timeouts=OFFLOAD_POLICY_V1.timeouts;
const never=():Promise<never>=>new Promise(()=>{});

test("the client waits for the action at most as long as the action may run plus one small margin, and that margin is small",()=>{
 expect(offloadExecuteBudgetMillisecondsV1()).toBe(timeouts.actionSeconds*1000+timeouts.executeMarginMilliseconds);
 expect(timeouts.executeMarginMilliseconds).toBeGreaterThan(0);expect(timeouts.executeMarginMilliseconds).toBeLessThanOrEqual(60000);
 expect(offloadExecuteBudgetMillisecondsV1()).toBeGreaterThan(timeouts.actionSeconds*1000);
});

test("a bounded phase aborts the work it started when its time is spent, ends on time even for work that ignores the signal, reports a cancelled build as cancelled and passes real faults on",async()=>{
 let observed=false;
 const listening=await runBoundedOffloadPhaseV1(signal=>new Promise<string>((_,reject)=>{signal.addEventListener("abort",()=>{observed=true;reject(signal.reason);},{once:true});}),{cancelled:new AbortController().signal,milliseconds:60});
 expect(listening).toEqual({kind:"timeout"});expect(observed,"the work saw the abort").toBe(true);
 const started=Date.now();
 expect(await runBoundedOffloadPhaseV1(never,{cancelled:new AbortController().signal,milliseconds:60})).toEqual({kind:"timeout"});expect(Date.now()-started).toBeLessThan(1500);
 const build=new AbortController();setTimeout(()=>build.abort(),30);
 expect(await runBoundedOffloadPhaseV1(never,{cancelled:build.signal,milliseconds:5000})).toEqual({kind:"cancelled"});
 expect(await runBoundedOffloadPhaseV1(async()=>7,{cancelled:new AbortController().signal,milliseconds:5000})).toEqual({kind:"done",value:7});
 await expect(runBoundedOffloadPhaseV1(async()=>{throw Error("disk");},{cancelled:new AbortController().signal,milliseconds:5000})).rejects.toThrow("disk");
});

test("an upload that the timeout aborts is aborted in the transport: the request is cancelled and the server sees the connection close",async()=>{
 let closed=false,received=false;
 const server=Bun.serve({port:0,hostname:"127.0.0.1",fetch(request){received=true;request.signal.addEventListener("abort",()=>{closed=true;});return new Promise<Response>(()=>{});}});
 try{
  const content=httpRemoteExecutionContentV1({endpoint:`http://127.0.0.1:${server.port}`,retries:0,timeoutSeconds:600}),bytes=new TextEncoder().encode("blob"),hash=createHash("sha256").update(bytes).digest("hex");
  const started=Date.now(),phase=await runBoundedOffloadPhaseV1(signal=>content.write("main",{hash,size:bytes.length},bytes,signal),{cancelled:new AbortController().signal,milliseconds:150});
  expect(phase).toEqual({kind:"timeout"});expect(Date.now()-started).toBeLessThan(3000);expect(received).toBe(true);
  for(let waited=0;!closed&&waited<3000;waited+=25)await Bun.sleep(25);
  expect(closed,"the aborted upload stopped on the wire").toBe(true);
 }finally{await server.stop(true);}
});

test("riding through a lost scheduler never waits past the budget of the action, however long the ride-through budget is",async()=>{
 const action={hash:"0".repeat(64),size_bytes:1},started=Date.now();
 const outcome=await executeOffloadRidingThroughV1({endpoint:"http://127.0.0.1:1",token:undefined,instance:"main",action,executeMilliseconds:700,connectMilliseconds:300},60000,async()=>{});
 expect(outcome.kind).toBe("declined");expect(Date.now()-started).toBeLessThan(4000);
});

const chain=(label:string)=>{
 const checkout=createTestCheckoutV1(label);
 checkout.compile("a","pub fn a()->u32{1}\n");
 checkout.compile("b","pub fn b()->u32{a::a()+1}\n",["a"]);
 checkout.crate("c","pub fn c()->u32{b::b()+1}\n");
 return checkout;
};

test("a unit whose upload hangs is cut at the upload limit, the request is cancelled on the wire and the unit compiles here",async()=>{
 let closed=false;
 const store=Bun.serve({port:0,hostname:"127.0.0.1",fetch(request){if(new URL(request.url).pathname==="/healthz")return new Response("{\"status\":\"ok\"}");request.signal.addEventListener("abort",()=>{closed=true;});return new Promise<Response>(()=>{});}});
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-hang-"))),checkout=chain("hang");
 try{
  const environment={...checkout.environment("c"),...testRustEnvironmentV1(),SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://127.0.0.1:1",SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${store.port}`,SEMIO_REMOTE_CACHE_INSTANCE:"main"};
  const started=Date.now(),outcome=await offloadRustcUnitV1({args:checkout.arguments("c",["b"]),environment,cwd:checkout.root,root,crate:"c",key:"a".repeat(64),compiler:["rustc"],ports:{uploadMilliseconds:300}});
  expect(outcome).toMatchObject({kind:"local",reason:"infrastructure",detail:"upload"});expect(Date.now()-started).toBeLessThan(8000);
  for(let waited=0;!closed&&waited<3000;waited+=25)await Bun.sleep(25);
  expect(closed).toBe(true);
 }finally{await store.stop(true);rmSync(root,{recursive:true,force:true});}
});

test("the stages of an offloaded unit reach the caller and the machine's progress lines in both languages",async()=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-stage-")));
 try{
  emitOffloadStageV1(root,"c","CACHE_CHECK");expect(existsSync(join(root,"x","offload.progress"))).toBe(false);
  emitOffloadStageV1(root,"c","QUEUED");emitOffloadStageV1(root,"c","EXECUTING");
  const lines=readFileSync(join(root,"x","offload.progress"),"utf8").trim().split("\n");
  expect(lines).toEqual(["Unit c waits for a machine.","Einheit c wartet auf eine Maschine.","Unit c is compiling on another machine.","Einheit c wird auf einer anderen Maschine kompiliert."]);
 }finally{rmSync(root,{recursive:true,force:true});}
 const cluster=await startClusterV1({scheduler:{unmatchedSeconds:1}}),scratch=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-stage-run-"))),checkout=chain("stage");
 try{
  const environment={...checkout.environment("c"),...testRustEnvironmentV1(),SEMIO_REMOTE_EXECUTION_ENDPOINT:cluster.server.grpcUrl,SEMIO_REMOTE_CACHE_ENDPOINT:cluster.cache.url.replace("//0.0.0.0","//127.0.0.1"),SEMIO_REMOTE_CACHE_INSTANCE:"main"},stages:string[]=[];
  const outcome=await offloadRustcUnitV1({args:checkout.arguments("c",["b"]),environment,cwd:checkout.root,root:scratch,crate:"c",key:"b".repeat(64),compiler:["rustc"],onStage:stage=>stages.push(stage)});
  expect(outcome.kind).toBe("local");expect(stages.length,"the caller heard the stages of the request").toBeGreaterThan(0);expect(new Set(stages).size,"each stage once").toBe(stages.length);
 }finally{await cluster.stop();rmSync(scratch,{recursive:true,force:true});}
});
