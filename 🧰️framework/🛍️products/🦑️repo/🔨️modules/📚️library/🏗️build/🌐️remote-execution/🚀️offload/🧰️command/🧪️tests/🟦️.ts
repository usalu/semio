import{stretchedBudgetMillisecondsV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import {afterAll,beforeAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import offloadSchema from "../../🧬️schema/🔣️.json" with {type:"json"};
import fixtures from "./🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {eventuallyV1,startClusterV1,textOfV1,type ClusterV1} from "../../../🧪️tests/🧰️cluster/🟦️.ts";
import {remoteExecutionHostPropertiesV1} from "../../../🟦️.ts";
import {validateRemoteExecutionCommandActionV1} from "../../../🧱️actions/🟦️.ts";
import {offloadCommandV1,offloadCommandPlatformV1} from "../../🟦️.ts";
import {classifyOffloadStatusV1} from "../../🩺️failure/🟦️.ts";
import {commandProgramV1,commandTextV1,parseCommandProgramV1} from "../📐️policy/🟦️.ts";
import {detectWorkerCommandsV1,commandToolDigestV1} from "../🛠️worker-tools/🟦️.ts";
import {stageCacheV1,type StageKeyV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1} from "../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📡️client/🟦️.ts";
import {sharedStageCacheV1} from "../../../../../⚡️caching/☁️stage-fleet/🟦️.ts";

const ajv=new Ajv({strict:false});ajv.addSchema(schema);
const rustcAjv=new Ajv({strict:false});rustcAjv.addSchema(offloadSchema);
const scratch=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"command-offload-")));
const sha=(bytes:Uint8Array|string):string=>createHash("sha256").update(bytes).digest("hex");
const WASM_OPT_ARGS=["-Oz","--low-memory-unused","--strip-debug","--strip-producers","--enable-bulk-memory","--enable-sign-ext","--enable-mutable-globals"];
const MODULE=Uint8Array.from([0,0x61,0x73,0x6d,1,0,0,0,1,7,1,0x60,2,0x7f,0x7f,1,0x7f,3,2,1,0,7,7,1,3,0x61,0x64,0x64,0,0,0x0a,9,1,7,0,0x20,0,0x20,1,0x6a,0x0b]);

const agentStop=new AbortController();let reporter:Promise<void>|undefined,cluster:ClusterV1,wasmOpt:string|undefined,digest="",detected:Awaited<ReturnType<typeof detectWorkerCommandsV1>>;
const environment={PATH:process.env.PATH??"",HOME:scratch};
const offloadRoot=join(scratch,"offload-root");

const ports=()=>({content:cluster.content,endpoint:cluster.server.grpcUrl,instance:"main",cacheEndpoint:cluster.cache.url.replace("//0.0.0.0","//127.0.0.1")});

beforeAll(async()=>{
 const root=join(import.meta.dir,"../../../../../../../../../..");
 detected=await detectWorkerCommandsV1(root,process.env);
 const row=Object.entries(detected.tools).find(([key])=>key.startsWith("wasm-opt/"));
 wasmOpt=row?.[1].path;digest=row?.[0].split("/")[1]??"";
 cluster=await startClusterV1({workers:0,root:scratch,scheduler:{leaseSeconds:5,lingerSeconds:1,unmatchedSeconds:6}});
 cluster.startWorker({name:"wA",actions:"rustc",unconfinedForTests:false,properties:[...remoteExecutionHostPropertiesV1(),...detected.properties],commands:detected.tools});
 const host={version:1,atMs:Date.now()+86_400_000,platform:"darwin",cores:8,totalMegabytes:65536,availableMegabytes:60000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0.5,openFiles:1000,openFilesMaximum:200000,attachedImages:0};
 const {buildRemoteExecutionLoadReportV1,readRemoteExecutionCountersV1,remoteExecutionPlatformIdentityV1,runRemoteExecutionReporterV1}=await import("../../../🌡️load/🟦️.ts");
 const report=async()=>buildRemoteExecutionLoadReportV1({origin:"wA",identity:remoteExecutionPlatformIdentityV1("darwin","arm64","29a34002290ca2fc","aarch64-apple-darwin"),view:{host,grants:[],waiters:[]} as never,worker:{state:"idle",concurrency:2,running:0,sandbox:"darwin-sandbox",confinement:"full",network:"denied"},counters:readRemoteExecutionCountersV1(scratch),nowMs:Date.now()});
 reporter=runRemoteExecutionReporterV1({scheduler:cluster.server.workerUrl,token:undefined,fetch,intervalSeconds:1,signal:agentStop.signal,report});
 await eventuallyV1(async()=>{const status=await (await fetch(cluster.server.workerUrl+"/status")).json() as {workers?:unknown[]};return(status.workers?.length??0)>=1;},60000,"the worker joined");
 await eventuallyV1(async()=>{const table=await (await fetch(cluster.server.workerUrl+"/v1/fleet/load")).json() as {machines?:{origin:string;headroom?:boolean}[]};return table.machines?.some(row=>row.origin==="wA"&&row.headroom===true)===true;},60000,"the worker's machine reports its load");
},stretchedBudgetMillisecondsV1(120000));
afterAll(async()=>{agentStop.abort();await reporter?.catch(()=>{});await cluster?.stop().catch(()=>{});rmSync(scratch,{recursive:true,force:true});});

test("the policy and the texts validate against the closed schema, in both validators, and both languages carry the same texts",()=>{
 expect(validateJsonSchemaSubset({$ref:"#/definitions/Policy"},schema.policy,schema as never)).toEqual([]);
 expect(ajv.validate({$ref:`${schema.$id}#/definitions/Policy`},schema.policy),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate({$ref:`${schema.$id}#/definitions/Policy`},{...schema.policy,extra:1})).toBe(false);
 expect(Object.keys(schema.messages.en).sort()).toEqual(Object.keys(schema.messages.de).sort());
 for(const key of Object.keys(schema.messages.en))expect(commandTextV1("de",key,{crate:"c",origin:"o",milliseconds:1,reason:"r",tool:"t",detail:"d"})).not.toBe("");
 expect(parseCommandProgramV1(commandProgramV1("wasm-opt","a".repeat(64)))).toEqual({name:"wasm-opt",digest:"a".repeat(64)});
 expect(parseCommandProgramV1("@command:wasm-opt/short")).toBeUndefined();expect(parseCommandProgramV1("@tool:rustc")).toBeUndefined();
});

test("the worker validator gives the verdicts of the language-agnostic fixture: only declared tools, relative paths, declared environment and a confining sandbox without network",()=>{
 for(const row of fixtures.cases as {name:string;arguments:string[];environment:Record<string,string>;outputs:string[];offered:string[];confined?:boolean;network?:boolean;expect:string|null}[]){
  const verdict=validateRemoteExecutionCommandActionV1({arguments:row.arguments,environment_variables:Object.entries(row.environment).map(([name,value])=>({name,value})),output_paths:row.outputs,working_directory:""},{tools:Object.fromEntries(row.offered.map(key=>[key,{}])),confined:row.confined??true,network:row.network??false});
  if(row.expect===null)expect(verdict,row.name).toBeUndefined();
  else{expect(verdict,row.name).toBeDefined();const prefix=commandTextV1("en",row.expect,{tool:"\u0001",detail:"\u0001"}).split("\u0001")[0]!;expect(verdict!.split(" / ")[0]!.startsWith(prefix),`${row.name}: ${verdict}`).toBe(true);}
 }
});

test("the platform of a command names the tool and its digest, carries the scheduler's control properties and is sorted",()=>{
 const rows=offloadCommandPlatformV1([{name:"OSFamily",value:"macos"},{name:"Arch",value:"arm64"}],{name:"wasm-opt",digest:"a".repeat(64)},"g1-m03",2048);
 expect(rows.map(row=>row.name)).toEqual([...rows.map(row=>row.name)].sort());
 expect(rows).toContainEqual({name:"semio-command-wasm-opt",value:"a".repeat(64)});expect(rows).toContainEqual({name:"semio-offload",value:"if-idle"});expect(rows).toContainEqual({name:"semio-origin",value:"g1-m03"});expect(rows).toContainEqual({name:"semio-peak-mb",value:"2048"});
 expect(offloadCommandPlatformV1([],{name:"build-script",digest:"b".repeat(64)},undefined).some(row=>row.name.startsWith("semio-command-"))).toBe(false);
});

test("a wasm-opt step offloaded to a worker through the command path gives the bytes of a local run of the same wasm-opt and is then served from the action cache",async()=>{
 expect(wasmOpt,"the pinned Binaryen of this checkout is prepared").toBeDefined();
 const work=join(scratch,"wasm-opt");mkdirSync(work,{recursive:true});
 const input=join(work,"core.wasm"),local=join(work,"local.wasm"),remote=join(work,"remote.wasm");
 writeFileSync(input,MODULE);
 expect(spawnSync(wasmOpt!,[input,...WASM_OPT_ARGS,"-o",local],{encoding:"utf8"}).status).toBe(0);
 const run=()=>offloadCommandV1({tool:{name:"wasm-opt",digest},arguments:["@root/in/core.wasm",...WASM_OPT_ARGS,"-o","@root/out/core.wasm"],inputs:new Map([["in/core.wasm",input]]),outputs:new Map([["out/core.wasm",remote]]),variables:[["BINARYEN_CORES","1"]],root:offloadRoot,environment,label:"wasm-opt core",key:null,ports:ports()});
 const first=await run();
 expect(first,JSON.stringify(first)).toMatchObject({kind:"done",status:0});
 expect(readFileSync(remote)).toEqual(readFileSync(local));
 rmSync(remote);
 const again=await run();
 expect(again).toMatchObject({kind:"done",cached:true});expect(readFileSync(remote)).toEqual(readFileSync(local));
 const events=readFileSync(join(offloadRoot,"x","offload.jsonl"),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line));
 for(const event of events)expect(rustcAjv.validate({$ref:`${offloadSchema.$id}#/definitions/Event`},event),JSON.stringify(rustcAjv.errors)).toBe(true);
 expect(events.filter(event=>event.kind==="offload-run").length).toBe(2);
 expect(readFileSync(join(offloadRoot,"x","offload.progress"),"utf8")).toContain("Schritt wasm-opt core lief auf ");
},stretchedBudgetMillisecondsV1(60000));

test("a step with a predicted peak runs on a machine that reports its load and has the memory, and its outputs equal a local run",async()=>{
 const work=join(scratch,"peak");mkdirSync(work,{recursive:true});
 const input=join(work,"core.wasm"),local=join(work,"local.wasm"),remote=join(work,"remote.wasm");writeFileSync(input,MODULE);
 expect(spawnSync(wasmOpt!,[input,...WASM_OPT_ARGS,"-o",local],{encoding:"utf8"}).status).toBe(0);
 const outcome=await offloadCommandV1({tool:{name:"wasm-opt",digest},arguments:["@root/in/core.wasm",...WASM_OPT_ARGS,"-o","@root/out/core.wasm"],inputs:new Map([["in/core.wasm",input]]),outputs:new Map([["out/core.wasm",remote]]),root:join(scratch,"peak-root"),environment,label:"wasm-opt peak",key:null,ports:ports()});
 expect(outcome,JSON.stringify(outcome)).toMatchObject({kind:"done"});expect(readFileSync(remote)).toEqual(readFileSync(local));
},stretchedBudgetMillisecondsV1(60000));

test("a tool that is not on the allow-list never leaves the requester and the worker refuses one that arrives anyway, as a policy failure that makes the step run here",async()=>{
 const outcome=await offloadCommandV1({tool:{name:"curl",digest:"c".repeat(64)},arguments:["https://example.com"],inputs:new Map(),outputs:new Map([["out/x",join(scratch,"x")]]),root:offloadRoot,environment,label:"curl",key:null,ports:ports()});
 expect(outcome).toMatchObject({kind:"local",reason:"not-portable"});
 const props=[...remoteExecutionHostPropertiesV1(),{name:"semio-offload-format",value:"command-1"}];
 const forged=await cluster.action({arguments:[commandProgramV1("curl","c".repeat(64)),"https://example.com"],outputPaths:["out/x"],platform:props,timeoutSeconds:20});
 const answer=await cluster.execute(forged);
 expect(answer.response.status.code).toBe(9);expect(String(answer.response.status.message)).toContain("not on the allow-list");
 expect(classifyOffloadStatusV1({code:9,message:String(answer.response.status.message)}).reason).toBe("worker-policy");
 const wrong=await cluster.action({arguments:[commandProgramV1("wasm-opt","d".repeat(64)),"-h"],outputPaths:["out/x"],platform:props,timeoutSeconds:20});
 const refused=await cluster.execute(wrong);
 expect(String(refused.response.status.message)).toContain("does not offer the tool wasm-opt");
 const shell=await cluster.action({arguments:["/bin/sh","-c","echo hi > out/x"],outputPaths:["out/x"],platform:props,timeoutSeconds:20});
 const plain=await cluster.execute(shell);
 expect(plain.response.status.code).toBe(9);expect(String(plain.response.status.message)).toContain("Action refused by the worker policy");
},stretchedBudgetMillisecondsV1(60000));

test("a requester that names a digest no worker owns is told at once that no machine can take the step, and the step runs here",async()=>{
 const started=Date.now(),outcome=await offloadCommandV1({tool:{name:"wasm-opt",digest:"e".repeat(64)},arguments:["@root/in/core.wasm","-o","@root/out/core.wasm"],inputs:new Map([["in/core.wasm",join(scratch,"wasm-opt","core.wasm")]]),outputs:new Map([["out/core.wasm",join(scratch,"never.wasm")]]),root:join(scratch,"other-root"),environment,label:"wasm-opt core",key:null,ports:ports()});
 expect(outcome.kind).toBe("local");expect(existsSync(join(scratch,"never.wasm"))).toBe(false);expect(Date.now()-started).toBeLessThan(20000);
},stretchedBudgetMillisecondsV1(60000));

test("a shipped tool runs only when the file the worker received has the digest the action names, and its declared output lands where a local run writes it",async()=>{
 const tool=join(scratch,"shipped-tool.sh"),text="#!/bin/sh\nprintf 'shipped %s' \"$(cat \"$1\")\" > \"$2\"\n";
 writeFileSync(tool,text);chmodSync(tool,0o755);
 const input=join(scratch,"in.txt"),output=join(scratch,"out.txt");writeFileSync(input,"payload");
 const run=(digestOfTool:string)=>offloadCommandV1({tool:{name:"build-script",digest:digestOfTool,file:tool},arguments:["@root/in/a.txt","@root/out/a.txt"],inputs:new Map([["in/a.txt",input]]),outputs:new Map([["out/a.txt",output]]),root:join(scratch,"shipped-root"),environment,label:"build script",key:null,ports:ports()});
 const good=await run(sha(text));
 expect(good,JSON.stringify(good)).toMatchObject({kind:"done",status:0});
 expect(readFileSync(output,"utf8")).toBe("shipped payload");
 rmSync(output);
 const bad=await offloadCommandV1({tool:{name:"build-script",digest:sha("another"),file:tool},arguments:["@root/in/a.txt","@root/out/a.txt"],inputs:new Map([["in/a.txt",input]]),outputs:new Map([["out/a.txt",output]]),root:join(scratch,"shipped-root-2"),environment,label:"build script",key:null,ports:ports()});
 expect(bad).toMatchObject({kind:"local",reason:"worker-policy"});expect(existsSync(output)).toBe(false);
},stretchedBudgetMillisecondsV1(60000));

test("a step whose tool exits non-zero on the worker leaves no output and runs here",async()=>{
 const tool=join(scratch,"failing.sh"),text="#!/bin/sh\necho oops >&2\nexit 3\n";writeFileSync(tool,text);chmodSync(tool,0o755);
 const output=join(scratch,"failed.txt");
 const outcome=await offloadCommandV1({tool:{name:"build-script",digest:sha(text),file:tool},arguments:[],inputs:new Map(),outputs:new Map([["out/a.txt",output]]),root:join(scratch,"failing-root"),environment,label:"failing",key:null,ports:ports()});
 expect(outcome).toMatchObject({kind:"local",reason:"incomplete-inputs",detail:"exit 3"});expect(existsSync(output)).toBe(false);
},stretchedBudgetMillisecondsV1(60000));

test("an unreadable input and a cancelled build end as a local run without touching the fleet twice",async()=>{
 const gone=await offloadCommandV1({tool:{name:"wasm-opt",digest},arguments:[],inputs:new Map([["in/x",join(scratch,"does-not-exist")]]),outputs:new Map([["out/x",join(scratch,"y")]]),root:join(scratch,"gone-root"),environment,label:"gone",key:null,ports:ports()});
 expect(gone).toMatchObject({kind:"local",reason:"unreadable-input"});
 const stop=new AbortController();stop.abort();
 const cancelled=await offloadCommandV1({tool:{name:"wasm-opt",digest},arguments:[],inputs:new Map(),outputs:new Map([["out/x",join(scratch,"y")]]),root:join(scratch,"cancel-root"),environment,label:"cancel",key:null,signal:stop.signal,ports:ports()});
 expect(cancelled).toMatchObject({kind:"local",reason:"cancelled"});
 expect(textOfV1(new Uint8Array(0))).toBe("");expect(commandToolDigestV1(process.execPath)).toMatch(/^[0-9a-f]{64}$/u);
},stretchedBudgetMillisecondsV1(60000));

test("a stage that misses at the fleet is produced on another machine through the command offload, never here, shared by its digests, and a second machine is served those exact bytes",async()=>{
 const work=join(scratch,"stage");mkdirSync(work,{recursive:true});
 const input=join(work,"core.wasm"),local=join(work,"local.wasm");writeFileSync(input,MODULE);
 expect(spawnSync(wasmOpt!,[input,...WASM_OPT_ARGS,"-o",local],{encoding:"utf8"}).status).toBe(0);
 const url=cluster.cache.url.replace("//0.0.0.0","//127.0.0.1"),fleet={SEMIO_REMOTE_CACHE_ENDPOINT:url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"test-token-not-a-secret"};
 const machine=(name:string)=>{const root=join(scratch,"machine-"+name);mkdirSync(root,{recursive:true});return{root,cache:sharedStageCacheV1(stageCacheV1(join(root,"stages")),{client:createFleetClientV1(readFleetSettingsV1(join(root,"state"),fleet)!)})};};
 const key:StageKeyV1={stage:"optimize",schemaVersion:2,inputs:{core:sha(MODULE)},tools:{"wasm-opt":"test",binary:digest,args:WASM_OPT_ARGS.join(" ")}};
 const first=machine("one"),second=machine("two");let produced=0;
 const remote=async(directory:string,signal:AbortSignal)=>(await offloadCommandV1({tool:{name:"wasm-opt",digest},arguments:["@root/in/core.wasm",...WASM_OPT_ARGS,"-o","@root/out/core.wasm"],inputs:new Map([["in/core.wasm",input]]),outputs:new Map([["out/core.wasm",join(directory,"core.wasm")]]),root:offloadRoot,environment,label:"wasm-opt stage",key:null,signal,ports:ports()})).kind==="done";
 const out1=join(first.root,"out");
 const made=await first.cache.run(key,out1,async()=>{produced++;throw Error("produced here");},undefined,"core/slot",remote);
 expect(produced,"the step ran on another machine").toBe(0);expect(made.hit).toBe(false);
 expect(readFileSync(join(out1,"core.wasm"))).toEqual(readFileSync(local));expect(first.cache.fleet().shared).toBe(1);
 const out2=join(second.root,"out");
 const served=await second.cache.run(key,out2,async()=>{produced++;throw Error("must be served by the fleet");},undefined,"core/slot",remote);
 expect(served.hit).toBe(true);expect(produced).toBe(0);expect(second.cache.fleet().served).toBe(1);expect(readFileSync(join(out2,"core.wasm"))).toEqual(readFileSync(local));
 const declined=machine("three"),out3=join(declined.root,"out");let ran=0;
 const fallback=await declined.cache.run({...key,inputs:{core:sha("other")}},out3,async directory=>{ran++;writeFileSync(join(directory,"core.wasm"),"local");},undefined,"core/slot",async()=>false);
 expect(ran,"a producer that answers false leaves the stage to the local run").toBe(1);expect(fallback.hit).toBe(false);expect(readFileSync(join(out3,"core.wasm"),"utf8")).toBe("local");
},stretchedBudgetMillisecondsV1(90000));
