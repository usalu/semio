import {expect,test} from "bun:test";
import {stretchedBudgetMillisecondsV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import {createHash} from "node:crypto";
import {existsSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startClusterV1} from "../../../🧪️tests/🧰️cluster/🟦️.ts";
import {emitOffloadStageV1,offloadRustcUnitV1} from "../../🟦️.ts";
import {createTestCheckoutV1,testRustEnvironmentV1} from "../🧰️workspace/🟦️.ts";
import {httpRemoteExecutionContentV1} from "../../../🗄️content/🟦️.ts";
import {executeOffloadV1,offloadExecuteBudgetMillisecondsV1,runBoundedOffloadPhaseV1} from "../../🔌️client/🟦️.ts";
import {OFFLOAD_POLICY_V1,offloadDeadlineMillisecondsV1} from "../../📐️policy/🟦️.ts";
import {GRPC_CODES_V1,GrpcErrorV1,startGrpcServerV1,type GrpcHandlersV1} from "../../../📡️wire/🚪️grpc/🟦️.ts";
import {remoteExecutionOperationV1} from "../../../🖥️server/🟦️.ts";
import {spawnSync} from "node:child_process";
import {encodeProtobufV1,REAPI_V1} from "../../../📡️wire/📦️protobuf/🟦️.ts";
import {fileURLToPath} from "node:url";
import {CAS_POLICY_V1} from "../../../../🗄️cas/🧬️schema/🟦️.ts";

const timeouts=OFFLOAD_POLICY_V1.timeouts;
test("content request allows CAS admission and the server's ordinary write budget",()=>{expect(timeouts.requestMilliseconds).toBeGreaterThanOrEqual(CAS_POLICY_V1.admissionBudgetMs+10000);expect(timeouts.requestMilliseconds).toBeLessThan(timeouts.uploadMilliseconds);});
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
 const outcome=await executeOffloadV1({endpoint:"http://127.0.0.1:1",token:undefined,instance:"main",action,executeMilliseconds:700,connectMilliseconds:300,rideThroughMilliseconds:60000,pause:async(milliseconds,signal)=>{await new Promise<void>(done=>{const timer=setTimeout(done,Math.min(milliseconds,20));signal.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});});}});
 expect(outcome.kind).toBe("declined");expect(Date.now()-started,"the action budget, not the 60 s ride-through, ends the wait").toBeLessThan(stretchedBudgetMillisecondsV1(4000));
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

const vectors=JSON.parse(readFileSync(fileURLToPath(new URL("./🔣️.json",import.meta.url)),"utf8")) as {policy:unknown;cases:{predicted:number|null;linked:boolean;expected:number}[]};

test("the deadline of an offloaded unit is factor times its recorded cost between the floor and the cap, the unknown value without a record, and the same numbers come out of an independent implementation",()=>{
 expect(vectors.policy).toEqual(OFFLOAD_POLICY_V1.deadline);
 for(const row of vectors.cases)expect(offloadDeadlineMillisecondsV1(row.predicted??undefined,row.linked),JSON.stringify(row)).toBe(row.expected);
 const oracle=spawnSync("python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{encoding:"utf8",input:JSON.stringify(vectors)});
 if(oracle.error===undefined)expect(oracle.stdout.trim(),oracle.stderr).toBe("ok");
});

const NS="build.bazel.remote.execution.v2",DIGEST={hash:"0".repeat(64),size:1};
const operation=(stage:"QUEUED"|"EXECUTING"|"COMPLETED",outcome?:object)=>remoteExecutionOperationV1({name:"operations/test",instance:"main",action:DIGEST,stage,...outcome===undefined?{}:{outcome:outcome as never}});
const served=async(handlers:GrpcHandlersV1)=>{const server=await startGrpcServerV1({host:"127.0.0.1",port:0,handlers});return {server,endpoint:server.url};};
const hang=():Promise<never>=>new Promise(()=>{});
const input=(endpoint:string,extra:object={})=>({endpoint,token:undefined,instance:"main",action:{hash:DIGEST.hash,size_bytes:1},...extra});

test("a stream that falls silent is closed and re-attached by name on a new connection, and the answer the re-attach delivers is the answer of the action",async()=>{
 let attaches=0;
 const {server,endpoint}=await served({
  [`${NS}.Execution/Execute`]:async function*(){yield operation("EXECUTING");await hang();},
  [`${NS}.Execution/WaitExecution`]:async function*(request:{name:string}){attaches++;expect(request.name).toBe("operations/test");yield operation("COMPLETED",{status:{code:0,message:""},actionResult:encodeProtobufV1(REAPI_V1.actionResult,{exit_code:0}),cached:false,worker:"w9"});},
 });
 try{
  const started=Date.now(),outcome=await executeOffloadV1(input(endpoint,{silenceMilliseconds:150,reattachMilliseconds:500,executeMilliseconds:20000}));
  expect(attaches).toBe(1);expect(Date.now()-started).toBeLessThan(5000);
  expect(outcome).toMatchObject({kind:"done",origin:"w9"});
 }finally{await server.stop();}
});

test("an attach that stays silent ends the wait at once and the unit compiles here",async()=>{
 let attaches=0;
 const {server,endpoint}=await served({[`${NS}.Execution/Execute`]:async function*(){yield operation("EXECUTING");await hang();},[`${NS}.Execution/WaitExecution`]:async function*(){attaches++;await hang();}});
 try{
  const started=Date.now(),outcome=await executeOffloadV1(input(endpoint,{silenceMilliseconds:150,reattachMilliseconds:200,executeMilliseconds:20000}));
  expect(outcome).toMatchObject({kind:"declined",reason:"infrastructure",detail:"silent after attach"});expect(attaches).toBe(1);expect(Date.now()-started).toBeLessThan(5000);
 }finally{await server.stop();}
});

test("an operation that the server no longer knows is a lost operation: no waiting, the unit compiles here",async()=>{
 const {server,endpoint}=await served({[`${NS}.Execution/Execute`]:async function*(){yield operation("QUEUED");await hang();},[`${NS}.Execution/WaitExecution`]:async function*(){throw new GrpcErrorV1(GRPC_CODES_V1.NOT_FOUND,"Unknown operation");}});
 try{
  const started=Date.now(),outcome=await executeOffloadV1(input(endpoint,{silenceMilliseconds:150,executeMilliseconds:20000}));
  expect(outcome).toMatchObject({kind:"declined",reason:"infrastructure",detail:"operation lost"});expect(Date.now()-started).toBeLessThan(3000);
 }finally{await server.stop();}
});

test("a server that keeps sending its operation but never finishes it is left at the deadline of the unit, not at the action timeout",async()=>{
 const {server,endpoint}=await served({[`${NS}.Execution/Execute`]:async function*(){for(;;){yield operation("EXECUTING");await Bun.sleep(40);}}});
 try{
  const started=Date.now(),outcome=await executeOffloadV1(input(endpoint,{deadlineMilliseconds:400,silenceMilliseconds:5000,executeMilliseconds:600000}));
  expect(outcome).toMatchObject({kind:"declined",reason:"infrastructure",detail:"deadline 400 ms"});expect(Date.now()-started).toBeLessThan(4000);
 }finally{await server.stop();}
});

test("a worker that is lost with the action fails the waiting client back within the deadline: the unit compiles here and nothing hangs",async()=>{
 const cluster=await startClusterV1({scheduler:{leaseSeconds:2,sweepMilliseconds:100,lingerSeconds:1}});
 try{
  const action=await cluster.action({arguments:["/bin/sh","-c","sleep 30"],salt:"lost-worker",platform:[{name:"semio-offload",value:"queue"},{name:"semio-origin",value:"requester"}]});
  const worker={version:1 as const,name:"silent",host:"h",platform:"darwin-arm64",properties:[{name:"OSFamily",value:"macos"}],concurrency:1,sandbox:"isolated-root" as const,runtime:"test",origin:"silent",capacity:{cores:8,memoryMegabytes:16384},confinement:"none" as const,network:"unenforced" as const};
  const registered=cluster.server.scheduler.register(worker as never);
  const started=Date.now(),waiting=executeOffloadV1(input(cluster.server.grpcUrl,{action,deadlineMilliseconds:15000,silenceMilliseconds:5000,executeMilliseconds:60000}));
  const task=await cluster.server.scheduler.poll(registered.id,5000,new AbortController().signal);
  expect(task.task,"the action reached the worker that then goes silent").not.toBeNull();
  const outcome=await waiting;
  expect(outcome.kind).toBe("declined");expect(Date.now()-started).toBeLessThan(14000);
 }finally{await cluster.stop();}
});

const removeBlobV1=(directory:string,hash:string):number=>{let removed=0;for(const name of readdirSync(directory,{recursive:true}) as string[])if(name.split("/").at(-1)===hash){rmSync(join(directory,name),{force:true});removed++;}return removed;};

test("a blob the store collected between the existence check and the fetch is sent again and the same action runs once more: the unit is answered by the fleet, not compiled here",async()=>{
 const {shipOffloadActionV1}=await import("../../📜️action/🟦️.ts"),cluster=await startClusterV1({workers:1}),work=mkdtempSync(join(tmpdir(),"repair-"));
 try{
  const file=join(work,"a.txt");writeFileSync(file,"hello");
  const built=await shipOffloadActionV1({spec:{files:new Map([["a.txt",file]]),arguments:["/bin/sh","-c","cat a.txt > r.txt"],environment:[],outputPaths:["r.txt"],seconds:30},content:cluster.content,instance:"main",platform:[]});
  if("declined" in built)throw Error(built.detail);
  const hello=createHash("sha256").update("hello").digest("hex");
  expect(removeBlobV1(join(cluster.directory,"cache"),hello),"the store collected the input after the existence check").toBeGreaterThan(0);
  const rows:{round:number;named:number;blobs:number;bytes:number}[]=[];
  const outcome=await executeOffloadV1({endpoint:cluster.server.grpcUrl,token:undefined,instance:"main",action:built.digest,repair:built.repair,onRepair:row=>rows.push(row),executeMilliseconds:60000});
  expect(outcome.kind).toBe("done");
  expect(rows).toEqual([{round:1,named:1,blobs:1,bytes:5}]);
  if(outcome.kind==="done"){
   const row=(outcome.actionResult.output_files as {path:string;digest:{hash:string;size_bytes:number}}[]).find(item=>item.path==="r.txt")!;
   expect(Buffer.from((await cluster.content.read("main",{hash:row.digest.hash,size:row.digest.size_bytes}))!).toString("utf8")).toBe("hello");
  }
  await built.release();
 }finally{await cluster.stop();rmSync(work,{recursive:true,force:true});}
},120000);

test("a store that keeps losing the blob is given three rounds of re-uploads, the bytes sent again are recorded, and then the unit is declined as an incomplete input",async()=>{
 const {shipOffloadActionV1}=await import("../../📜️action/🟦️.ts"),cluster=await startClusterV1({workers:1}),work=mkdtempSync(join(tmpdir(),"repair-"));
 try{
  const file=join(work,"a.txt");writeFileSync(file,"hello");
  const hello=createHash("sha256").update("hello").digest("hex"),lossy={...cluster.content,write:async(instance:string,digest:{hash:string;size:number},source:Uint8Array|AsyncIterable<Uint8Array>,signal?:AbortSignal,intent?:string)=>{if(digest.hash===hello)return;return cluster.content.write(instance,digest,source,signal,intent as never);}};
  const built=await shipOffloadActionV1({spec:{files:new Map([["a.txt",file]]),arguments:["/bin/sh","-c","cat a.txt > r.txt"],environment:[],outputPaths:["r.txt"],seconds:30},content:lossy,instance:"main",platform:[]});
  if("declined" in built)throw Error(built.detail);
  const rows:{round:number;named:number;blobs:number;bytes:number}[]=[];
  const outcome=await executeOffloadV1({endpoint:cluster.server.grpcUrl,token:undefined,instance:"main",action:built.digest,repair:built.repair,onRepair:row=>rows.push(row),executeMilliseconds:60000});
  expect(outcome.kind).toBe("declined");
  if(outcome.kind==="declined"){expect(outcome.reason).toBe("incomplete-inputs");expect(outcome.detail).toContain("after 3 re-uploads (15 bytes sent again)");}
  expect(rows.map(row=>row.round)).toEqual([1,2,3]);expect(rows.reduce((sum,row)=>sum+row.bytes,0)).toBe(15);
  await built.release();
 }finally{await cluster.stop();rmSync(work,{recursive:true,force:true});}
},120000);

test("without a repair the same failure is a decline at once, so a caller that cannot send blobs again still compiles here",async()=>{
 const {shipOffloadActionV1}=await import("../../📜️action/🟦️.ts"),cluster=await startClusterV1({workers:1}),work=mkdtempSync(join(tmpdir(),"repair-"));
 try{
  const file=join(work,"a.txt");writeFileSync(file,"hello");
  const built=await shipOffloadActionV1({spec:{files:new Map([["a.txt",file]]),arguments:["/bin/sh","-c","cat a.txt > r.txt"],environment:[],outputPaths:["r.txt"],seconds:30},content:cluster.content,instance:"main",platform:[]});
  if("declined" in built)throw Error(built.detail);
  removeBlobV1(join(cluster.directory,"cache"),createHash("sha256").update("hello").digest("hex"));
  const outcome=await executeOffloadV1({endpoint:cluster.server.grpcUrl,token:undefined,instance:"main",action:built.digest,executeMilliseconds:60000});
  expect(outcome.kind).toBe("declined");if(outcome.kind==="declined")expect(outcome.reason).toBe("incomplete-inputs");
 }finally{await cluster.stop();rmSync(work,{recursive:true,force:true});}
},120000);
