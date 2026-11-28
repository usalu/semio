import {afterEach,expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {decodeProtobufV1,encodeProtobufV1,REAPI_V1} from "../../../📡️wire/📦️protobuf/🟦️.ts";
import type {RemoteExecutionContentV1} from "../../../🗄️content/🟦️.ts";
import type {RemoteExecutionDigestV1} from "../../../🟦️.ts";
import {executeRemoteExecutionActionV1} from "../../../👷️worker/🟦️.ts";
import {selectRemoteExecutionSandboxV1} from "../../../🔒️sandbox/🟦️.ts";
import type {OffloadSessionV1} from "../../🟦️.ts";
import type {executeOffloadV1} from "../../🔌️client/🟦️.ts";
import {followCheckoutTestChunksV1,offloadCheckoutTestV1} from "../🔌️client/🟦️.ts";
import {checkoutTestGateV1,checkoutTestLoadPerCoreV1,checkoutTestLoadVerdictV1,readCheckoutTestMeasuresV1,recordCheckoutTestMeasureV1} from "../🚦️gate/🟦️.ts";
import {checkoutTestForwardedV1,CHECKOUT_TEST_POLICY_V1,checkoutTestBothV1,checkoutTestCoversV1,checkoutTestModeV1,checkoutTestPathIsV1,checkoutTestPlatformV1,checkoutTestProblemV1,checkoutTestTextV1,validateCheckoutTestActionV1,checkoutTestChunkKeyV1,decodeCheckoutTestChunkV1,encodeCheckoutTestChunkV1,type CheckoutTestChunkV1,type CheckoutTestCutV1} from "../📐️policy/🟦️.ts";
import {awaitCheckoutRevisionV1,type CheckoutTestReplicaV1} from "../🧭️replica/🟦️.ts";
import {createCheckoutTestChunkWriterV1,createCheckoutTestsV1} from "../👷️worker/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import vectors from "./🔣️.json" with {type:"json"};

const scratch:string[]=[];
afterEach(()=>{for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const temporary=():string=>{const path=mkdtempSync(join(tmpdir(),"semio-checkout-test-"));scratch.push(path);return path;};
const sha=(data:Uint8Array|string):string=>createHash("sha256").update(data).digest("hex");
const ajv=new Ajv({strict:false,allowUnionTypes:true}),definitions=schema.definitions,validator=(name:string)=>ajv.compile({$schema:schema.$schema,definitions,$ref:`#/definitions/${name}`});

class Memory implements RemoteExecutionContentV1{
 blobs=new Map<string,Uint8Array>();results=new Map<string,Uint8Array>();
 async has(_:string,hash:string){return this.blobs.has(hash);}
 async missing(_:string,digests:readonly RemoteExecutionDigestV1[]){return digests.filter(digest=>!this.blobs.has(digest.hash));}
 async read(_:string,digest:RemoteExecutionDigestV1){return this.blobs.get(digest.hash);}
 async stream(_:string,digest:RemoteExecutionDigestV1){const bytes=this.blobs.get(digest.hash);return bytes?(async function*(){yield bytes;})():undefined;}
 async write(_:string,digest:RemoteExecutionDigestV1,source:Uint8Array|AsyncIterable<Uint8Array>){if(source instanceof Uint8Array)this.blobs.set(digest.hash,source);else{const parts:Uint8Array[]=[];for await(const part of source)parts.push(part);this.blobs.set(digest.hash,Buffer.concat(parts));}}
 async readActionResult(_:string,hash:string){return this.results.get(hash);}
 async writeActionResult(_:string,hash:string,bytes:Uint8Array){this.results.set(hash,bytes);}
}

const python=(input:unknown):{cover:{covered:boolean;behind:string[]}[];path:boolean[];gate:{offload:boolean;reason:string;asked:string[]}[];load:string[]}=>{
 const result=spawnSync(testPythonV1()??"python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{input:JSON.stringify(input),encoding:"utf8",maxBuffer:1<<26});
 expect(result.status,result.stderr).toBe(0);
 return JSON.parse(result.stdout);
};

const cut=(segment:number,bytes:number):CheckoutTestCutV1=>({"g1-m01":{segment,bytes}});
const replica=(own:Partial<CheckoutTestReplicaV1>&{origin:string}):CheckoutTestReplicaV1=>({cut:async()=>cut(2,50),ready:async()=>({ready:true,detail:"ready"}),digest:()=>sha("file"),...own});

test("the policy satisfies its closed schema under the in-repo validator and under Ajv, and both reject the same mutations",()=>{
 const check=validator("Policy");
 expect(check(CHECKOUT_TEST_POLICY_V1),JSON.stringify(check.errors?.[0])).toBe(true);
 expect(checkoutTestProblemV1("Policy",CHECKOUT_TEST_POLICY_V1)).toBeUndefined();
 const mutations:Record<string,(value:any)=>void>={extra:value=>{value.extra=1;},noProgram:value=>{delete value.program;},noWait:value=>{delete value.wait;},badPoll:value=>{value.wait.pollMilliseconds=0;},badLimit:value=>{value.limits.maximumRequestBytes=0;},noCandidates:value=>{value.limits.maximumCandidates=0;},badVersion:value=>{value.version=2;}};
 for(const [name,mutate] of Object.entries(mutations)){
  const copy=JSON.parse(JSON.stringify(CHECKOUT_TEST_POLICY_V1));mutate(copy);
  expect(check(copy),name+" ajv").toBe(false);
  expect(checkoutTestProblemV1("Policy",copy),name+" in-repo").toBeDefined();
 }
});

test("the wire messages satisfy their closed schemas under both validators and both reject the same mutations",()=>{
 const request={version:1,nonce:"0".repeat(32),file:"a/b.ts",fileSha256:sha("x"),cut:cut(1,2),language:"en",budgetSeconds:60},result={version:1,origin:"g1-m06",cut:cut(1,2),result:{any:"thing"}};
 for(const [name,value,mutations] of [["Request",request,{extra:(v:any)=>{v.extra=1;},badSha:(v:any)=>{v.fileSha256="zz";},badNonce:(v:any)=>{v.nonce="x";},noNonce:(v:any)=>{delete v.nonce;},badLanguage:(v:any)=>{v.language="fr";},zeroBudget:(v:any)=>{v.budgetSeconds=0;},badCut:(v:any)=>{v.cut={"g1-m01":{segment:-1,bytes:0}};},noFile:(v:any)=>{delete v.file;}}],["Result",result,{extra:(v:any)=>{v.extra=1;},noOrigin:(v:any)=>{delete v.origin;},badVersion:(v:any)=>{v.version=2;},notObject:(v:any)=>{v.result="x";}}]] as const){
  const check=validator(name);
  expect(check(value),name+JSON.stringify(check.errors?.[0])).toBe(true);
  expect(checkoutTestProblemV1(name,value),name).toBeUndefined();
  for(const [mutation,mutate] of Object.entries(mutations)){
   const copy=JSON.parse(JSON.stringify(value));mutate(copy);
   expect(check(copy),name+" "+mutation+" ajv").toBe(false);
   expect(checkoutTestProblemV1(name,copy),name+" "+mutation+" in-repo").toBeDefined();
  }
 }
});

test("every text exists in both languages with the same placeholders, and differs between them",()=>{
 const keys=Object.keys(schema.messages.en);
 expect(Object.keys(schema.messages.de).sort()).toEqual([...keys].sort());
 for(const key of keys){
  const names=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(match=>match[1]!).sort(),en=(schema.messages.en as Record<string,string>)[key]!,de=(schema.messages.de as Record<string,string>)[key]!;
  expect(names(de),key).toEqual(names(en));
  expect(de,key).not.toBe(en);
  expect(checkoutTestBothV1(key,{file:"f",origin:"o",seconds:1,reason:"r",detail:"d"})).toBe(`${checkoutTestTextV1("en",key,{file:"f",origin:"o",seconds:1,reason:"r",detail:"d"})} / ${checkoutTestTextV1("de",key,{file:"f",origin:"o",seconds:1,reason:"r",detail:"d"})}`);
 }
 expect(()=>checkoutTestTextV1("en","missing")).toThrow();
});

test("replica cover, path validity and the offload gate follow the language-neutral vectors and an independent Python implementation of them",async()=>{
 const oracle=python({cover:vectors.cover,path:vectors.path,gate:vectors.gate});
 vectors.cover.forEach((row,index)=>{
  expect(checkoutTestCoversV1(row.need as CheckoutTestCutV1,row.have as CheckoutTestCutV1),row.name).toEqual({covered:row.covered,behind:row.behind});
  expect(oracle.cover[index],row.name+" python").toEqual({covered:row.covered,behind:row.behind});
 });
 vectors.path.forEach((row,index)=>{
  expect(checkoutTestPathIsV1(row.path),JSON.stringify(row.path)).toBe(row.valid);
  expect(oracle.path[index],JSON.stringify(row.path)+" python").toBe(row.valid);
 });
 for(const [index,row] of vectors.gate.entries()){
  const asked:string[]=[],answer=await checkoutTestGateV1({mode:row.mode as "auto"|"never"|"always",host:async()=>{asked.push("host");return row.host as "admit"|"wait";},idle:async()=>{asked.push("idle");return row.idle;}});
  expect(answer,row.name).toEqual({offload:row.offload,reason:row.reason as never});
  expect(asked,row.name).toEqual(row.asked);
  expect(oracle.gate[index],row.name+" python").toEqual({offload:row.offload,reason:row.reason,asked:row.asked});
 }
});

test("the mode comes from the environment, an unknown value is the default",()=>{
 expect(checkoutTestModeV1({})).toBe("auto");
 expect(checkoutTestModeV1({SEMIO_TEST_FILE_OFFLOAD:"never"})).toBe("never");
 expect(checkoutTestModeV1({SEMIO_TEST_FILE_OFFLOAD:"always"})).toBe("always");
 expect(checkoutTestModeV1({SEMIO_TEST_FILE_OFFLOAD:"maybe"})).toBe("auto");
});

test("the platform names the machine kind, the control properties and the checkout test format, sorted, and the machine it is pinned to only when asked",()=>{
 const host=[{name:"OSFamily",value:"macos"},{name:"Arch",value:"arm64"}],auto=checkoutTestPlatformV1(host,"g1-server"),pinned=checkoutTestPlatformV1(host,"g1-server","g1-m06");
 expect(auto.map(row=>row.name)).toEqual([...auto.map(row=>row.name)].sort());
 expect(auto).toContainEqual({name:CHECKOUT_TEST_POLICY_V1.property.format,value:CHECKOUT_TEST_POLICY_V1.property.formatValue});
 expect(auto).toContainEqual({name:"semio-origin",value:"g1-server"});
 expect(auto).toContainEqual({name:"semio-offload",value:"if-idle"});
 expect(auto.some(row=>row.name==="semio-pin")).toBe(false);
 expect(pinned).toContainEqual({name:"semio-pin",value:"g1-m06"});
 expect(pinned).toContainEqual({name:"semio-offload",value:"if-idle"});
 expect(checkoutTestPlatformV1(host,undefined).some(row=>row.name==="semio-origin")).toBe(false);
});

test("an action is accepted only as the bare program: no arguments, environment, outputs or working directory, in English and German",()=>{
 const good={arguments:[CHECKOUT_TEST_POLICY_V1.program.name],environment_variables:[],output_paths:[],working_directory:""};
 expect(validateCheckoutTestActionV1(good)).toBeUndefined();
 const bad:Record<string,object>={program:{...good,arguments:["/bin/sh"]},arguments:{...good,arguments:[good.arguments[0]!,"x"]},environment:{...good,environment_variables:[{name:"A",value:"b"}]},outputs:{...good,output_paths:["out/x"]},files:{...good,output_files:["x"]},directories:{...good,output_directories:["x"]},directory:{...good,working_directory:"x"}};
 for(const [name,command] of Object.entries(bad)){
  const reason=validateCheckoutTestActionV1(command as never);
  expect(reason,name).toBeDefined();
  expect(reason,name).toContain(" / ");
 }
});

test("a replica is awaited until it holds the cut and the file of the requester, and is never guessed",async()=>{
 let polls=0,clock=0;
 const sleep=async(milliseconds:number)=>{clock+=milliseconds;},now=()=>clock;
 const advancing=replica({origin:"g1-m06",cut:async()=>++polls<4?cut(1,5):cut(1,10)});
 expect(await awaitCheckoutRevisionV1({replica:advancing,need:cut(1,10),file:"f.ts",fileSha256:sha("file"),waitMilliseconds:20000,pollMilliseconds:1000,now,sleep})).toEqual({held:true});
 expect(polls).toBe(4);
 clock=0;
 const stuck=await awaitCheckoutRevisionV1({replica:replica({origin:"g1-m06",cut:async()=>cut(1,5)}),need:cut(1,10),file:"f.ts",fileSha256:sha("file"),waitMilliseconds:3000,pollMilliseconds:1000,now,sleep});
 expect(stuck).toEqual({held:false,reason:"behind",detail:"g1-m01"});
 expect(clock).toBeLessThanOrEqual(3000);
 clock=0;
 const other=await awaitCheckoutRevisionV1({replica:replica({origin:"g1-m06",digest:()=>sha("other")}),need:cut(1,10),file:"f.ts",fileSha256:sha("file"),waitMilliseconds:2000,pollMilliseconds:1000,now,sleep});
 expect(other).toEqual({held:false,reason:"file",detail:"f.ts"});
 const missing=await awaitCheckoutRevisionV1({replica:replica({origin:"g1-m06",cut:async()=>undefined}),need:cut(1,10),file:"f.ts",fileSha256:sha("file"),waitMilliseconds:0,pollMilliseconds:1000,now,sleep});
 expect(missing).toMatchObject({held:false,reason:"behind"});
 const aborted=new AbortController();aborted.abort();
 await expect(awaitCheckoutRevisionV1({replica:replica({origin:"g1-m06"}),need:cut(1,1),file:"f.ts",fileSha256:sha("file"),signal:aborted.signal})).rejects.toThrow();
});

const request=(extra:object={})=>new TextEncoder().encode(JSON.stringify({version:1,nonce:"1".repeat(32),file:"a/b.ts",fileSha256:sha("file"),cut:cut(2,40),language:"de",budgetSeconds:60,...extra}));

test("a worker runs a request only on a Ready machine whose replica holds the revision: not Ready, behind, a different file, an unreadable request and a path out of the checkout are each answered, never run",async()=>{
 const tests=(own:Partial<CheckoutTestReplicaV1>,wait=0)=>createCheckoutTestsV1({root:"/checkout",replica:replica({origin:"g1-m06",...own}),repo:"/bin/repo",environment:{PATH:"/bin",KEEP:"1",DROP:undefined},waitMilliseconds:wait,pollMilliseconds:1,sleep:async()=>{},now:(()=>{let at=0;return()=>at+=1;})()}),signal=new AbortController().signal;
 expect(tests({}).properties).toEqual([{name:CHECKOUT_TEST_POLICY_V1.property.format,value:CHECKOUT_TEST_POLICY_V1.property.formatValue}]);
 const ready=await tests({}).begin(request(),signal);
 expect(ready.kind).toBe("run");
 if(ready.kind==="run"){
  expect(ready.argv).toEqual(["/bin/repo","test-file","--json","--","a/b.ts"]);
  expect(ready.cwd).toBe("/checkout");
  expect(ready.env).toMatchObject({PATH:"/bin",KEEP:"1",SEMIO_TEST_FILE_OFFLOAD:"never",SEMIO_LANGUAGE:"de"});
  expect("DROP" in ready.env).toBe(false);
  expect("SEMIO_TEST_BUDGET_STRETCH_BASIS_POINTS" in ready.env).toBe(false);
  const floored=await tests({}).begin(request({stretchBasisPoints:30000}),signal);
  expect(floored.kind==="run"?floored.env.SEMIO_TEST_BUDGET_STRETCH_BASIS_POINTS:undefined).toBe("30000");
  expect((await tests({}).begin(request({stretchBasisPoints:5}),signal)).kind).toBe("fail");
  const served=await tests({}).begin(request({environment:{SEMIO_TEST_SERVICE_ORIGIN:"http://192.168.1.7:6058"}}),signal);
  expect(served.kind==="run"?served.env.SEMIO_TEST_SERVICE_ORIGIN:undefined).toBe("http://192.168.1.7:6058");
  expect("SEMIO_TEST_SERVICE_ORIGIN" in ready.env).toBe(false);
  for(const environment of [{SEMIO_TEST_SERVICE_ORIGIN:"file:///etc"},{SEMIO_TEST_SERVICE_ORIGIN:"http://u:p@h"},{PATH:"/evil"}])expect((await tests({}).begin(request({environment}),signal)).kind).toBe("fail");
  expect(checkoutTestForwardedV1({SEMIO_TEST_SERVICE_ORIGIN:"http://h:1",PATH:"/bin"})).toEqual({environment:{SEMIO_TEST_SERVICE_ORIGIN:"http://h:1"}});
  expect(checkoutTestForwardedV1({PATH:"/bin",SEMIO_TEST_SERVICE_ORIGIN:""})).toEqual({});
  const inner={status:"passed"},wrapped=await ready.finish({exitCode:0,stdout:new TextEncoder().encode(JSON.stringify(inner)),stderr:new TextEncoder().encode("e")});
  expect(JSON.parse(new TextDecoder().decode(wrapped.stdout))).toEqual({version:1,origin:"g1-m06",cut:cut(2,50),result:inner});
  expect(new TextDecoder().decode(wrapped.stderr)).toBe("e");
  const raw=await ready.finish({exitCode:2,stdout:new TextEncoder().encode("not json"),stderr:new Uint8Array()});
  expect(new TextDecoder().decode(raw.stdout)).toBe("not json");expect(raw.exitCode).toBe(2);
 }
 const notReady=await tests({ready:async()=>({ready:false,detail:"working"})}).begin(request(),signal);
 expect(notReady).toMatchObject({kind:"refuse",reason:"not-ready"});
 expect((notReady as {message:string}).message).toContain("working");
 expect(await tests({cut:async()=>cut(1,1)}).begin(request(),signal)).toMatchObject({kind:"refuse",reason:"not-ready"});
 expect(await tests({digest:()=>sha("other")}).begin(request(),signal)).toMatchObject({kind:"refuse",reason:"not-ready"});
 for(const [name,bytes] of [["empty",new Uint8Array()],["json",new TextEncoder().encode("{")],["shape",request({extra:1})],["path",request({file:"../x.ts"})],["absolute",request({file:"/etc/passwd"})],["big",new Uint8Array(CHECKOUT_TEST_POLICY_V1.limits.maximumRequestBytes+1)]] as const){
  expect(await tests({}).begin(bytes,signal),name).toMatchObject({kind:"fail",code:3});
 }
});

const RESULT=(file:string,failed:boolean)=>JSON.stringify({version:1,status:failed?"failed":"passed",seconds:0.1,bail:false,counts:{passed:failed?0:1,failed:failed?1:0,skipped:0,notRun:0},files:[{file,mode:"plain",table:null,suite:"plain",status:failed?"failed":"passed",seconds:0.1,pass:1,fail:failed?1:0}]});
const program=(file:string,body:string)=>[process.execPath,"-e",`const file=${JSON.stringify(file)};${body}`];
const PASS_OR_FAIL=(file:string):readonly string[]=>program(file,`process.stderr.write('progress '+file+' offload='+process.env.SEMIO_TEST_FILE_OFFLOAD+' language='+process.env.SEMIO_LANGUAGE+'\\n');process.stdout.write(${JSON.stringify(RESULT(file,file.includes('fail')))});process.exit(${file.includes('fail')?1:0});`);

function fixture(input:Readonly<{worker?:Partial<CheckoutTestReplicaV1>;argv?:(repo:string,file:string)=>readonly string[];actions?:"any"|"none";mount?:boolean;wait?:number;peers?:readonly string[];busyFor?:number;others?:Readonly<Record<string,Partial<CheckoutTestReplicaV1>>>}>={}){
 const memory=new Memory(),log:{properties:readonly {name:string;value:string}[];stage:string[];actions:string[];pins:(string|undefined)[];pauses:number[]}={properties:[],stage:[],actions:[],pins:[],pauses:[]},directory=temporary(),root=temporary();
 const requester=replica({origin:"g1-server",cut:async()=>cut(2,50),digest:()=>sha("file"),...input.peers===undefined?{}:{peers:async()=>input.peers!}}),worker=replica({origin:"g1-m06",cut:async()=>cut(2,60),...input.worker});
 const build=(own:CheckoutTestReplicaV1)=>createCheckoutTestsV1({root,replica:own,repo:process.execPath,environment:{...process.env},argv:input.argv??((_,file)=>PASS_OR_FAIL(file)),waitMilliseconds:input.wait??0,pollMilliseconds:1,sleep:async()=>{}}),tests=build(worker),machines:Record<string,ReturnType<typeof build>>={"g1-m06":tests,...Object.fromEntries(Object.entries(input.others??{}).map(([origin,overrides])=>[origin,build(replica({origin,cut:async()=>cut(2,60),...overrides}))]))};
 const execute=(async(call:Parameters<typeof executeOffloadV1>[0])=>{
  const actionBytes=memory.blobs.get(call.action.hash)!,action=decodeProtobufV1(REAPI_V1.action,actionBytes) as {platform:{properties:{name:string;value:string}[]}};
  log.properties=action.platform.properties;log.actions.push(call.action.hash);
  const pin=action.platform.properties.find(row=>row.name==="semio-pin")?.value;log.pins.push(pin);
  if(log.actions.length<=(input.busyFor??0))return{kind:"declined",reason:"no-idle-machine",detail:"No other machine has headroom"} as const;
  const chosen=pin===undefined?tests:machines[pin];
  if(chosen===undefined)return{kind:"declined",reason:"no-idle-machine",detail:"no such machine"} as const;
  call.onStage?.("EXECUTING");log.stage.push("EXECUTING");
  const result=await executeRemoteExecutionActionV1({instance:call.instance,action:{hash:call.action.hash,size:call.action.size_bytes},timeoutSeconds:60,content:memory,directory,workerName:"host-1",sandbox:selectRemoteExecutionSandboxV1("isolated-root"),materialize:"copy",inlineOutputBytes:1<<20,signal:call.signal??new AbortController().signal,actions:input.actions??"any",...input.mount===false?{}:{checkoutTests:chosen}});
  if(result.outcome==="refused")return{kind:"declined",reason:"no-idle-machine",detail:result.status.message} as const;
  if(result.outcome==="cancelled")return{kind:"declined",reason:"cancelled",detail:""} as const;
  if(result.outcome!=="completed")return{kind:"declined",reason:"worker-policy",detail:result.status.message} as const;
  return{kind:"done",actionResult:decodeProtobufV1(REAPI_V1.actionResult,result.actionResult!),cached:false,origin:"host-1"} as const;
 }) as typeof executeOffloadV1;
 const session:OffloadSessionV1={endpoint:"http://x",content:memory,instance:"main",token:undefined,executionToken:undefined,workspace:undefined};
 let clock=1_000_000;
 const ask=(file:string,extra:Partial<Parameters<typeof offloadCheckoutTestV1>[0]>={})=>offloadCheckoutTestV1({root,memoRoot:temporary(),environment:{},file,budgetSeconds:60,language:"en",...extra,ports:{replica:requester,openSession:async()=>session,execute,sleep:async milliseconds=>{log.pauses.push(milliseconds);clock+=milliseconds;},now:()=>clock}});
 return{ask,log,memory};
}

test("a test file goes through the scheduler to a worker and its result and exit status come back, attributed to the worker's origin, with the worker's own output and the offload mode never",async()=>{
 const {ask,log}=fixture(),stages:string[]=[];
 const passed=await ask("a/ok.ts",{onStage:stage=>{stages.push(stage);}});
 expect(passed.kind).toBe("done");
 if(passed.kind==="done"){
  expect(passed.origin).toBe("g1-m06");
  expect(passed.exitCode).toBe(0);
  expect(passed.result).toMatchObject({status:"passed",files:[{file:"a/ok.ts",status:"passed"}]});
  expect(passed.stderr).toContain("progress a/ok.ts offload=never language=en");
 }
 expect(stages).toEqual(["EXECUTING"]);
 expect(log.properties).toContainEqual({name:CHECKOUT_TEST_POLICY_V1.property.format,value:CHECKOUT_TEST_POLICY_V1.property.formatValue});
 expect(log.properties).toContainEqual({name:"semio-offload",value:"if-idle"});
 const failed=await ask("a/fail.ts");
 expect(failed.kind,"a failing file is a verdict, not a reason to run it again here").toBe("done");
 if(failed.kind==="done"){expect(failed.exitCode).toBe(1);expect(failed.result).toMatchObject({status:"failed"});expect(failed.origin).toBe("g1-m06");}
 const again=await ask("a/ok.ts");
 expect(again.kind).toBe("done");
 expect(new Set(log.actions).size,"the same file asked twice is two actions, so no result is taken from a cache").toBe(log.actions.length);
});

test("a file is tried on the Ready machines one after the other, at most the declared number, and ends here when none takes it",async()=>{
 const notReady={ready:async()=>({ready:false,detail:"working"})},behind={cut:async()=>cut(1,1)};
 const {ask,log}=fixture({peers:["g1-m05","g1-m04","g1-m06"],others:{"g1-m05":notReady,"g1-m04":behind}});
 const done=await ask("a/ok.ts");
 expect(done.kind).toBe("done");
 if(done.kind==="done")expect(done.origin).toBe("g1-m06");
 expect(log.pins).toEqual(["g1-m05","g1-m04","g1-m06"]);
 const crowded=fixture({peers:["g1-m01","g1-m02","g1-m03","g1-m04","g1-m05"],others:Object.fromEntries(["g1-m01","g1-m02","g1-m03","g1-m04","g1-m05"].map(origin=>[origin,notReady]))});
 expect(await crowded.ask("a/ok.ts")).toMatchObject({kind:"local",reason:"no-idle-machine"});
 expect(crowded.log.pins.slice(0,CHECKOUT_TEST_POLICY_V1.limits.maximumCandidates)).toHaveLength(CHECKOUT_TEST_POLICY_V1.limits.maximumCandidates);
 const busy=fixture({peers:["g1-m05","g1-m06"],busyFor:3,others:{"g1-m05":{}}}),late=await busy.ask("a/ok.ts");
 expect(late.kind,"every machine is full: all are asked again after a pause and one takes the file when it has room").toBe("done");
 expect(busy.log.pauses).toEqual([CHECKOUT_TEST_POLICY_V1.wait.pauseMilliseconds]);
 expect(busy.log.pins).toHaveLength(4);
 const none=fixture({peers:[]});
 expect(await none.ask("a/ok.ts")).toMatchObject({kind:"local",reason:"no-idle-machine",detail:expect.stringContaining("Ready")});
 expect(none.log.pins).toEqual([]);
 const patient=fixture({peers:["g1-m06"],busyFor:1e9});
 expect(await patient.ask("a/ok.ts",{retryMilliseconds:CHECKOUT_TEST_POLICY_V1.wait.pauseMilliseconds*2+1}),"the wait is bounded").toMatchObject({kind:"local",reason:"no-idle-machine"});
 expect(patient.log.pauses).toHaveLength(2);
 expect(await fixture({peers:["g1-m06"],busyFor:1e9}).ask("a/ok.ts",{retryMilliseconds:0}),"without a wait there is no second try").toMatchObject({kind:"local",reason:"no-idle-machine"});
 const failing=fixture({peers:["g1-m05","g1-m06"],others:{"g1-m05":{}},argv:(_,file)=>program(file,"process.stderr.write('boom');process.exit(2);")});
 expect(await failing.ask("a/ok.ts")).toMatchObject({kind:"local",reason:"remote-failed-local-redo"});
 expect(failing.log.pins,"a file that failed to run is not tried on a second machine").toEqual(["g1-m05"]);
});

test("every fault returns the file to this machine with its reason: worker not Ready, replica behind, other file content, no checkout test capability, an unreadable or oversized answer",async()=>{
 const reasons=async(input:Parameters<typeof fixture>[0],file="a/ok.ts")=>{const outcome=await fixture(input).ask(file);expect(outcome.kind).toBe("local");return outcome.kind==="local"?outcome:undefined;};
 expect(await reasons({worker:{ready:async()=>({ready:false,detail:"working"})}})).toMatchObject({reason:"no-idle-machine"});
 expect(await reasons({worker:{cut:async()=>cut(1,1)}})).toMatchObject({reason:"no-idle-machine"});
 expect(await reasons({worker:{digest:()=>sha("other")}})).toMatchObject({reason:"no-idle-machine"});
 expect(await reasons({mount:false})).toMatchObject({reason:"worker-policy"});
 expect(await reasons({actions:"none"})).toMatchObject({reason:"worker-policy"});
 expect(await reasons({argv:(_,file)=>program(file,"process.stdout.write('not json');")})).toMatchObject({reason:"remote-failed-local-redo",detail:expect.stringContaining("not JSON")});
 expect(await reasons({argv:(_,file)=>program(file,"process.stderr.write('boom');process.exit(2);")})).toMatchObject({reason:"remote-failed-local-redo",detail:expect.stringContaining("status 2")});
});

test("no replica, no cut, an unreadable file and a declining session each end as a local run without throwing",async()=>{
 const base={root:temporary(),memoRoot:temporary(),environment:{},file:"a/ok.ts",budgetSeconds:5,language:"en" as const};
 expect(await offloadCheckoutTestV1({...base,ports:{replica:undefined,openSession:async()=>({decline:"no-endpoint",detail:"x"} as never)}})).toMatchObject({kind:"local"});
 const noCut=await offloadCheckoutTestV1({...base,ports:{replica:replica({origin:"g1-server",cut:async()=>undefined})}});
 expect(noCut).toMatchObject({kind:"local",reason:"infrastructure"});
 const noFile=await offloadCheckoutTestV1({...base,ports:{replica:replica({origin:"g1-server",digest:()=>undefined})}});
 expect(noFile).toMatchObject({kind:"local",reason:"unreadable-input"});
 const declined=await offloadCheckoutTestV1({...base,ports:{replica:replica({origin:"g1-server"}),openSession:async()=>({decline:"no-credential",detail:""})}});
 expect(declined).toMatchObject({kind:"local",reason:"no-credential"});
 const thrown=await offloadCheckoutTestV1({...base,ports:{replica:replica({origin:"g1-server"}),openSession:async()=>{throw Error("store down");}}});
 expect(thrown).toMatchObject({kind:"local",reason:"infrastructure",detail:"store down"});
});

test("cancelling the request stops the test process on the worker and the requester answers cancelled; the scheduler's decline reasons pass through",async()=>{
 const pidFile=join(temporary(),"pid"),sleeper=`require('fs').writeFileSync(${JSON.stringify(pidFile)},String(process.pid));setInterval(()=>{},1000);`;
 const {ask}=fixture({argv:(_,file)=>program(file,sleeper)}),controller=new AbortController();
 const pending=ask("a/slow.ts",{signal:controller.signal});
 for(let waited=0;!existsSync(pidFile)&&waited<20000;waited+=50)await new Promise(done=>setTimeout(done,50));
 expect(existsSync(pidFile)).toBe(true);
 const pid=Number(readFileSync(pidFile,"utf8"));
 controller.abort();
 expect(await pending).toEqual({kind:"local",reason:"cancelled",detail:""});
 let alive=true;
 for(let waited=0;alive&&waited<10000;waited+=50){try{process.kill(pid,0);await new Promise(done=>setTimeout(done,50));}catch{alive=false;}}
 expect(alive,"the test process of the worker is gone").toBe(false);
 const memory=new Memory(),session:OffloadSessionV1={endpoint:"http://x",content:memory,instance:"main",token:undefined,executionToken:undefined,workspace:undefined};
 const refused=await offloadCheckoutTestV1({root:temporary(),memoRoot:temporary(),environment:{},file:"a/ok.ts",budgetSeconds:5,language:"en",ports:{replica:replica({origin:"g1-server"}),openSession:async()=>session,execute:(async()=>({kind:"declined",reason:"no-idle-machine",detail:"nobody"})) as typeof executeOffloadV1}});
 expect(refused).toEqual({kind:"local",reason:"no-idle-machine",detail:"nobody"});
});

const memoryFor=(chunkBytes:readonly Uint8Array[],nonce:string,origin="g1-m06")=>{
 const memory=new Memory();
 let offset=0;
 chunkBytes.forEach((data,index)=>{
  const chunk:CheckoutTestChunkV1={version:1,nonce,index,offset,origin,data:Buffer.from(data).toString("base64"),last:index===chunkBytes.length-1};
  offset+=data.length;
  memory.results.set(checkoutTestChunkKeyV1(nonce,index),encodeProtobufV1(REAPI_V1.actionResult,{stderr_raw:encodeCheckoutTestChunkV1(chunk),exit_code:0}));
 });
 return memory;
};
const NONCE="a".repeat(32);

test("H44: the live output pieces, the measured durations and the policy numbers satisfy their closed schemas under both validators and both reject the same mutations",()=>{
 const chunk={version:1,nonce:NONCE,index:0,offset:0,origin:"g1-m06",data:Buffer.from("héllo\n").toString("base64"),last:false},measures={version:1,files:{"a/b.ts":{local:{seconds:12.5,at:1700000000000},remote:{seconds:3.1,at:1700000001000}},"c.ts":{}}};
 for(const [name,value,mutations] of [["Chunk",chunk,{extra:(v:any)=>{v.extra=1;},badNonce:(v:any)=>{v.nonce="x";},negativeIndex:(v:any)=>{v.index=-1;},badOffset:(v:any)=>{v.offset=1.5;},noOrigin:(v:any)=>{delete v.origin;},badData:(v:any)=>{v.data="***";},lastText:(v:any)=>{v.last="yes";},noLast:(v:any)=>{delete v.last;}}],["Measures",measures,{extra:(v:any)=>{v.extra=1;},noFiles:(v:any)=>{delete v.files;},negative:(v:any)=>{v.files["a/b.ts"].local.seconds=-1;},noAt:(v:any)=>{delete v.files["a/b.ts"].remote.at;},unknownSide:(v:any)=>{v.files["c.ts"].other={seconds:1,at:1};}}]] as const){
  const check=validator(name);
  expect(check(value),name+JSON.stringify(check.errors?.[0])).toBe(true);
  expect(checkoutTestProblemV1(name,value),name).toBeUndefined();
  for(const [mutation,mutate] of Object.entries(mutations)){
   const copy=JSON.parse(JSON.stringify(value));mutate(copy);
   expect(check(copy),name+" "+mutation+" ajv").toBe(false);
   expect(checkoutTestProblemV1(name,copy),name+" "+mutation+" in-repo").toBeDefined();
  }
 }
 const policy=validator("Policy");
 for(const [name,mutate] of Object.entries({noLoad:(v:any)=>{delete v.load;},noStream:(v:any)=>{delete v.stream;},badRatio:(v:any)=>{v.load.remoteWorthRatio=2;},zeroParallel:(v:any)=>{v.limits.maximumParallel=0;},zeroChunk:(v:any)=>{v.stream.maximumChunkBytes=0;}})){
  const copy=JSON.parse(JSON.stringify(CHECKOUT_TEST_POLICY_V1));mutate(copy);
  expect(policy(copy),name+" ajv").toBe(false);
  expect(checkoutTestProblemV1("Policy",copy),name+" in-repo").toBeDefined();
 }
 expect(CHECKOUT_TEST_POLICY_V1.wait).toMatchObject({retryMilliseconds:8000,pauseMilliseconds:3000});
});

test("H44: the load verdict follows the language-neutral vectors and an independent Python implementation, and the vectors name the policy they were made for",()=>{
 expect(vectors.policy).toEqual(CHECKOUT_TEST_POLICY_V1.load);
 const now=1_000_000_000,oracle=python({load:vectors.load,policy:vectors.policy,gateLoad:vectors.gateLoad});
 vectors.load.forEach((row,index)=>{
  const side=(timing:{seconds:number;ageSeconds:number}|null)=>timing===null?undefined:{seconds:timing.seconds,at:now-timing.ageSeconds*1000};
  const measure=row.local===null&&row.remote===null?undefined:{...side(row.local)===undefined?{}:{local:side(row.local)!},...side(row.remote)===undefined?{}:{remote:side(row.remote)!}};
  expect(checkoutTestLoadVerdictV1({loadPerCore:row.loadPerCore,measure,now}),row.name).toBe(row.verdict as "offload"|"keep");
  expect(oracle.load[index],row.name+" python").toBe(row.verdict);
 });
 expect(checkoutTestLoadPerCoreV1([24,10,5],8)).toBe(3);
 expect(checkoutTestLoadPerCoreV1([0,0,0],8)).toBe(0);
 expect(checkoutTestLoadPerCoreV1([5],0)).toBe(5);
});

test("H44: the gate asks the CPU load only when the host admits, and a saturated host offers the file with the reason host-load",async()=>{
 const oracle=python({gate:vectors.gateLoad,policy:vectors.policy});
 for(const [index,row] of vectors.gateLoad.entries()){
  const asked:string[]=[],answer=await checkoutTestGateV1({mode:row.mode as "auto"|"never"|"always",host:async()=>{asked.push("host");return row.host as "admit"|"wait";},idle:async()=>{asked.push("idle");return row.idle;},load:async()=>{asked.push("load");return row.load as "offload"|"keep";}});
  expect(answer,row.name).toEqual({offload:row.offload,reason:row.reason as never});
  expect(asked,row.name).toEqual(row.asked);
  expect(oracle.gate[index],row.name+" python").toEqual({offload:row.offload,reason:row.reason,asked:row.asked});
 }
});

test("H44: measured durations are recorded atomically per file and side, bounded to the newest files, and a damaged document reads as empty",()=>{
 const root=temporary();
 expect(readCheckoutTestMeasuresV1(root)).toEqual({version:1,files:{}});
 recordCheckoutTestMeasureV1(root,"a.ts","local",100.04,1000);
 recordCheckoutTestMeasureV1(root,"a.ts","remote",14.46,2000);
 recordCheckoutTestMeasureV1(root,"b.ts","local",1,3000);
 expect(readCheckoutTestMeasuresV1(root).files).toEqual({"a.ts":{local:{seconds:100,at:1000},remote:{seconds:14.5,at:2000}},"b.ts":{local:{seconds:1,at:3000}}});
 expect(checkoutTestProblemV1("Measures",readCheckoutTestMeasuresV1(root))).toBeUndefined();
 for(let index=0;index<CHECKOUT_TEST_POLICY_V1.load.maximumFiles+5;index++)recordCheckoutTestMeasureV1(root,`many/${index}.ts`,"local",1,10_000+index);
 const kept=Object.keys(readCheckoutTestMeasuresV1(root).files);
 expect(kept).toHaveLength(CHECKOUT_TEST_POLICY_V1.load.maximumFiles);
 expect(kept).not.toContain("a.ts");
 expect(kept).toContain(`many/${CHECKOUT_TEST_POLICY_V1.load.maximumFiles+4}.ts`);
 writeFileSync(join(root,CHECKOUT_TEST_POLICY_V1.load.file),"{broken");
 expect(readCheckoutTestMeasuresV1(root)).toEqual({version:1,files:{}});
 recordCheckoutTestMeasureV1(root,"x.ts","remote",2,5);
 expect(Object.keys(readCheckoutTestMeasuresV1(root).files)).toEqual(["x.ts"]);
 expect(readdirSync(root).filter(name=>name!==CHECKOUT_TEST_POLICY_V1.load.file)).toEqual([]);
});

test("H44: a piece of live output is keyed by nonce and index, round-trips, and anything else decodes to nothing",()=>{
 const chunk:CheckoutTestChunkV1={version:1,nonce:NONCE,index:3,offset:30,origin:"g1-m06",data:Buffer.from("abc").toString("base64"),last:true};
 expect(decodeCheckoutTestChunkV1(encodeCheckoutTestChunkV1(chunk))).toEqual(chunk);
 expect(checkoutTestChunkKeyV1(NONCE,3)).toBe(sha(`${CHECKOUT_TEST_POLICY_V1.stream.key}/${NONCE}/3`));
 expect(checkoutTestChunkKeyV1(NONCE,3)).not.toBe(checkoutTestChunkKeyV1(NONCE,4));
 expect(checkoutTestChunkKeyV1(NONCE,3)).not.toBe(checkoutTestChunkKeyV1("b".repeat(32),3));
 expect(()=>encodeCheckoutTestChunkV1({...chunk,index:-1})).toThrow();
 for(const bytes of [undefined,new Uint8Array(),new TextEncoder().encode("{"),new TextEncoder().encode(JSON.stringify({...chunk,extra:1})),new Uint8Array(CHECKOUT_TEST_POLICY_V1.stream.maximumChunkBytes*2+1).fill(32)])expect(decodeCheckoutTestChunkV1(bytes)).toBeUndefined();
});

test("H44: the chunk writer keeps order and offsets, cuts large writes at the piece size, flushes on a timer, ends with a last piece, and stops quietly when a write fails or the pieces are used up",async()=>{
 const pieces:CheckoutTestChunkV1[]=[],writer=createCheckoutTestChunkWriterV1({nonce:NONCE,origin:"g1-m06",flushMilliseconds:20,maximumChunkBytes:8,write:async(_,chunk)=>{pieces.push(chunk);}});
 writer.push(new TextEncoder().encode("ab"));
 expect(pieces).toHaveLength(0);
 await new Promise(done=>setTimeout(done,80));
 expect(pieces.map(piece=>Buffer.from(piece.data,"base64").toString())).toEqual(["ab"]);
 writer.push(new TextEncoder().encode("0123456789ABCDEF!"));
 writer.push(new TextEncoder().encode("xyz"));
 await writer.close();
 const text=pieces.map(piece=>Buffer.from(piece.data,"base64").toString()).join("");
 expect(text).toBe("ab0123456789ABCDEF!xyz");
 expect(pieces.map(piece=>piece.index)).toEqual(pieces.map((_,index)=>index));
 let offset=0;
 for(const piece of pieces){expect(piece.offset).toBe(offset);offset+=Buffer.from(piece.data,"base64").length;expect(Buffer.from(piece.data,"base64").length).toBeLessThanOrEqual(8);expect(piece.origin).toBe("g1-m06");}
 expect(pieces.filter(piece=>piece.last)).toHaveLength(1);
 expect(pieces[pieces.length-1]!.last).toBe(true);
 writer.push(new TextEncoder().encode("late"));
 await writer.close();
 expect(pieces.map(piece=>Buffer.from(piece.data,"base64").toString()).join("")).toBe(text);
 const failing:number[]=[],broken=createCheckoutTestChunkWriterV1({nonce:NONCE,origin:"o",flushMilliseconds:1,maximumChunkBytes:2,write:async index=>{failing.push(index);throw Error("store down");}});
 broken.push(new TextEncoder().encode("abcdef"));
 await broken.close();
 expect(failing).toEqual([0]);
 const few:CheckoutTestChunkV1[]=[],limited=createCheckoutTestChunkWriterV1({nonce:NONCE,origin:"o",maximumChunkBytes:1,maximumChunks:2,write:async(_,chunk)=>{few.push(chunk);}});
 limited.push(new TextEncoder().encode("abcd"));
 await limited.close();
 expect(few.map(piece=>Buffer.from(piece.data,"base64").toString())).toEqual(["a","b"]);
 const stuck=createCheckoutTestChunkWriterV1({nonce:NONCE,origin:"o",drainMilliseconds:50,write:()=>new Promise<void>(()=>{})}),started=Date.now();
 stuck.push(new TextEncoder().encode("x"));
 await stuck.close();
 expect(Date.now()-started,"close never waits longer than the drain").toBeLessThan(2000);
});

test("H44: the follower shows the pieces in order while a file runs, never splits a character, refuses a piece out of order, and finishes the rest of the output with the same decoder",async()=>{
 const euro=new TextEncoder().encode("a€b\n"),memory=memoryFor([euro.subarray(0,2),euro.subarray(2,3),euro.subarray(3)],NONCE),seen:string[]=[],origins:string[]=[];
 const follow=followCheckoutTestChunksV1({nonce:NONCE,read:async hash=>memory.results.get(hash),onText:(text,origin)=>{seen.push(text);origins.push(origin);},signal:new AbortController().signal,pollMilliseconds:5});
 await new Promise(done=>setTimeout(done,60));
 await follow.stop();
 expect(seen.join("")).toBe("a€b\n");
 expect(seen.every(text=>!text.includes("�"))).toBe(true);
 expect(new Set(origins)).toEqual(new Set(["g1-m06"]));
 expect(follow.bytes()).toBe(euro.length);
 const late=memoryFor([new TextEncoder().encode("one\n"),new TextEncoder().encode("two\n")],NONCE),kept:string[]=[];
 const hidden=new Map(late.results);late.results.delete(checkoutTestChunkKeyV1(NONCE,1));
 const waiting=followCheckoutTestChunksV1({nonce:NONCE,read:async hash=>late.results.get(hash),onText:text=>{kept.push(text);},signal:new AbortController().signal,pollMilliseconds:5});
 await new Promise(done=>setTimeout(done,40));
 expect(kept).toEqual(["one\n"]);
 late.results.set(checkoutTestChunkKeyV1(NONCE,1),hidden.get(checkoutTestChunkKeyV1(NONCE,1))!);
 await waiting.stop();
 expect(kept.join("")).toBe("one\ntwo\n");
 expect(waiting.finish(new TextEncoder().encode("rest"))).toBe("rest");
 const skewed=memoryFor([new TextEncoder().encode("zero\n"),new TextEncoder().encode("one\n")],NONCE),swapped=new Map(skewed.results),got:string[]=[];
 skewed.results.set(checkoutTestChunkKeyV1(NONCE,1),swapped.get(checkoutTestChunkKeyV1(NONCE,0))!);
 const checked=followCheckoutTestChunksV1({nonce:NONCE,read:async hash=>skewed.results.get(hash),onText:text=>{got.push(text);},signal:new AbortController().signal,pollMilliseconds:5});
 await checked.stop();
 expect(got.join(""),"a piece that is not the next one ends the following").toBe("zero\n");
 const garbage=new Memory();garbage.results.set(checkoutTestChunkKeyV1(NONCE,0),new Uint8Array([1,2,3]));
 const none:string[]=[],ignoring=followCheckoutTestChunksV1({nonce:NONCE,read:async hash=>garbage.results.get(hash),onText:text=>{none.push(text);},signal:new AbortController().signal,pollMilliseconds:5});
 await ignoring.stop();
 expect(none).toEqual([]);
 expect(ignoring.bytes()).toBe(0);
});

const SLOW=(file:string,milliseconds=1500):readonly string[]=>program(file,`process.stderr.write('first line of '+file+'\\n');setTimeout(()=>{process.stderr.write('second line\\n');process.stdout.write(${JSON.stringify(RESULT("X",false))}.replace('"X"',JSON.stringify(file)));process.exit(0);},${milliseconds});`);

test("H44: a running test file shows its output live: the first line arrives long before the file ends, live text and the remainder add up to the whole output, and a store that refuses the pieces only means the output comes at the end",async()=>{
 const {ask}=fixture({argv:(_,file)=>SLOW(file,3000)}),received:{text:string;origin:string;at:number}[]=[],start=Date.now();
 const done=await ask("a/slow.ts",{onOutput:(text,origin)=>{received.push({text,origin,at:Date.now()});}});
 const end=Date.now();
 expect(done.kind).toBe("done");
 if(done.kind==="done"){
  expect(done.streamed).toBe(true);
  expect(received.length).toBeGreaterThan(0);
  expect(received[0]!.text).toContain("first line of a/slow.ts");
  expect(received[0]!.origin).toBe("g1-m06");
  expect(received[0]!.at,"the first line did not wait for the end of the file").toBeLessThan(end-1000);
  expect(received.map(row=>row.text).join("")+done.stderr).toContain("first line of a/slow.ts");
  expect(received.map(row=>row.text).join("")+done.stderr).toContain("second line");
  expect((received.map(row=>row.text).join("")+done.stderr).match(/first line/gu)).toHaveLength(1);
 }
 expect(end-start).toBeGreaterThan(2500);
 const refusing=fixture({argv:(_,file)=>SLOW(file,500)}),shown:string[]=[];
 refusing.memory.writeActionResult=async()=>{throw Error("cache read-only");};
 const whole=await refusing.ask("a/slow.ts",{onOutput:text=>{shown.push(text);}});
 expect(whole.kind).toBe("done");
 if(whole.kind==="done"){expect(whole.streamed).toBe(false);expect(shown).toEqual([]);expect(whole.stderr).toContain("first line of a/slow.ts");expect(whole.stderr).toContain("second line");}
},30000);

test("H44: files that are out at once never share a machine: a claimed machine is skipped, and a run whose Ready machines are all claimed ends here at once",async()=>{
 const claim=new Set<string>(),first=fixture({peers:["g1-m05","g1-m06"],others:{"g1-m05":{}}});
 claim.add("g1-m05");
 const done=await first.ask("a/ok.ts",{claim});
 expect(done.kind).toBe("done");
 expect(first.log.pins).toEqual(["g1-m06"]);
 expect(claim.size,"every claim is released when the run ends").toBe(1);
 const full=fixture({peers:["g1-m05"],others:{"g1-m05":{}}}),taken=new Set(["g1-m05"]);
 const outcome=await full.ask("a/ok.ts",{claim:taken});
 expect(outcome).toMatchObject({kind:"local",reason:"no-idle-machine",detail:expect.stringContaining("already runs")});
 expect(full.log.pins).toEqual([]);
 expect(full.log.pauses,"no retry while every machine is claimed").toEqual([]);
 const both=fixture({peers:["g1-m05","g1-m06"],others:{"g1-m05":{}},argv:(_,file)=>SLOW(file)}),shared=new Set<string>();
 const [one,two]=await Promise.all([both.ask("a/one.ts",{claim:shared}),both.ask("a/two.ts",{claim:shared})]);
 expect([one.kind,two.kind]).toEqual(["done","done"]);
 expect(new Set(both.log.pins).size,"two files in parallel went to two machines").toBe(2);
 expect(shared.size).toBe(0);
});
