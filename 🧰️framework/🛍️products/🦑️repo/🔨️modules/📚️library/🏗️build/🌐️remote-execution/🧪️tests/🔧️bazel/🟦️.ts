/** 🔧️ Law: real Bazel offloads hermetic actions to the in-house executor. With a worker in its own process, the generated hermetic workspace builds with zero local spawns of the hermetic mnemonic, the worker runs the actions inside its sandbox, the results land in the shared store and a build from a clean output base is answered by the action cache without any worker run, and the outputs equal those of a local build. */
import {expect,test} from "bun:test";
import {makeTestTreeWritableV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,statSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {join,relative} from "node:path";
import schema from "../../../🔧️tools/🏗️bazel/🏃️execution/🧬️schema/🔣️.json";
import {createNativeBuildCallerCapabilitiesV1} from "../../../📥️caller/🟦️.ts";
import {createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import * as execution from "../../../🔧️tools/🏗️bazel/🏃️execution/🟦️.ts";
import {eventuallyV1,startClusterV1} from "../🧰️cluster/🟦️.ts";
import {testScratchRootV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";

const binary=process.env.SEMIO_TEST_BAZEL_BINARY,sha=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex"),contract=schema.definitions.Contract.const,host=execution.bazelExecutionPlatformV1(process.platform,process.arch);

const files:Record<string,string>={
 "lib/src/a.txt":"alpha\n","lib/src/b.txt":"beta\n","lib/secret.txt":"undeclared\n",
 "lib/gen.ts":`import{mkdirSync,readFileSync,readdirSync,writeFileSync}from"node:fs";const base=process.env.SEMIO_WORKSPACE_ROOT+"/lib";mkdirSync(base+"/gen",{recursive:true});writeFileSync(base+"/gen/out.txt",readdirSync(base+"/src").sort().map(name=>readFileSync(base+"/src/"+name,"utf8").toUpperCase()).join(""));writeFileSync(base+"/gen/tool.sh","#!/bin/sh\\n",{mode:0o700});`,
 "app/src/c.txt":"gamma\n",
 "app/build.ts":`import{mkdirSync,readFileSync,writeFileSync}from"node:fs";const base=process.env.SEMIO_WORKSPACE_ROOT;mkdirSync(base+"/app/dist",{recursive:true});writeFileSync(base+"/app/dist/result.txt",readFileSync(base+"/lib/gen/out.txt","utf8")+readFileSync(base+"/app/src/c.txt","utf8")+(process.env.SEMIO_REMOTE_EXECUTION_ENDPOINT===undefined?"":"leak\\n"),{mode:0o664});`,
 "neg/leak.ts":`import{readFileSync}from"node:fs";console.log(readFileSync(process.env.SEMIO_WORKSPACE_ROOT+"/lib/secret.txt","utf8"));`,
};
const hermetic={version:1,tools:["bun"]},target=(command:string,inputs:string[],outputs:string[],dependsOn:string[]=[])=>({execution:{kind:"command",command},outputs,inputs,cache:true,dependsOn,metadata:{semio:{hermetic}}});
const graph={version:1 as const,projects:{
 lib:{name:"lib",root:"lib",targets:{generate:target("bun {projectRoot}/gen.ts",["{projectRoot}/src/**/*","{projectRoot}/gen.ts"],["{projectRoot}/gen"])}},
 app:{name:"app",root:"app",targets:{build:target("bun {projectRoot}/build.ts",["{projectRoot}/**/*"],["{projectRoot}/dist"],["lib:generate"])}},
 neg:{name:"neg",root:"neg",targets:{leak:target("bun {projectRoot}/leak.ts",["{projectRoot}/leak.ts"],["{projectRoot}/out"])}}},dependencies:[]} as any;

type Spawn={mnemonic:string;runner:string;targetLabel:string;remoteable?:boolean};
const readLog=(path:string):Spawn[]=>{const text=readFileSync(path,"utf8"),rows:Spawn[]=[];let depth=0,start=-1,quoted=false,escaped=false;for(let index=0;index<text.length;index++){const char=text[index]!;if(quoted){if(escaped)escaped=false;else if(char==="\\")escaped=true;else if(char==='"')quoted=false;continue;}if(char==='"')quoted=true;else if(char==="{"){if(depth===0)start=index;depth++;}else if(char==="}"&&--depth===0)rows.push(JSON.parse(text.slice(start,index+1)));}return rows;};
const treeDigest=(directory:string):string=>{const rows:string[]=[],visit=(path:string)=>{for(const name of readdirSync(path).sort()){const full=join(path,name),info=statSync(full);if(info.isDirectory())visit(full);else rows.push(relative(directory,full)+"\0"+(info.mode&0o777).toString(8)+"\0"+sha(readFileSync(full)));}};visit(directory);return sha(rows.join("\n"));};

test.skipIf(!binary)("law: real Bazel runs hermetic actions on a worker in its own process, caches the results in the shared store and spawns nothing locally",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("rexec-inhouse"),"bazel-")),checkout=join(directory,"checkout"),cluster=await startClusterV1({workers:0,root:directory}),entry=join(import.meta.dir,"..","..","🏃️entry","🟦️.ts");
 const environment=(extra:Record<string,string>={})=>({...process.env,...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{}),...extra});
 const common=(base:string)=>[`--output_base=${join(directory,base)}`,`--install_base=${join(directory,"install")}`,"--nosystem_rc","--nohome_rc",`--bazelrc=${join(process.cwd(),".bazelrc")}`,"--max_idle_secs=120"];
 let workerProcess:ReturnType<typeof Bun.spawn>|undefined;
 try{
  for(const[path,text]of Object.entries(files)){mkdirSync(join(checkout,path,".."),{recursive:true});writeFileSync(join(checkout,path),text);}
  const api=await import("../../../🔧️tools/🏗️bazel/📜️script.ts"),request={tasks:["app:build","neg:leak"]};
  const generate=(name:string,plan:execution.BazelExecutionPlanV1)=>{
   const workspace=join(directory,name);
   return withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:remote-execution-law",maximumElapsedMilliseconds:120000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["run","app:build"],"bazel",{}),Date.now()),async original=>{
    const planned=api.planBazelTasksV1(graph,request),implementation={version:1 as const,snapshot:join(workspace,"implementation.json"),runner:join(workspace,"📜️script.ts"),sha256:"0".repeat(64),loaderSha256:"1".repeat(64)};
    await api.generateBazelWorkspaceV1(checkout,graph,request,planned,workspace,original,createIssuedResourceControlV1(original,"bazel:remote-execution-law",0),implementation,()=>{},undefined,plan);
    return {workspace,planned};
   });
  };
  const {workspace,planned}=await generate("ws",{host:host.id,remote:{platform:host.id,properties:{...host.properties}},tools:{[host.id]:process.execPath}}),local=await generate("ws-local",{host:host.id,tools:{[host.id]:process.execPath}}),label=(id:string)=>planned.tasks[id]!.label;
  expect(local.planned.tasks["app:build"]!.label).toBe(label("app:build"));
  workerProcess=Bun.spawn(["bun",entry,"--join",cluster.server.workerUrl,"--name","second-process","--concurrency","2","--work-directory",join(directory,"worker"),"--lang","en"],{stdout:"pipe",stderr:"pipe",cwd:directory,env:{...process.env}});
  await eventuallyV1(()=>cluster.server.scheduler.status().workers.length===1,30000,"the worker process must register");

  const environmentVariables={SEMIO_REMOTE_EXECUTION_ENDPOINT:cluster.server.grpcUrl,SEMIO_REMOTE_EXECUTION_PLATFORM:host.id,SEMIO_REMOTE_EXECUTION_INSTANCE:"main"};
  const flags=execution.bazelExecutionFlagsV1({host:{platform:process.platform,arch:process.arch},hermetic:true,jobs:4,environment:environmentVariables});
  expect(flags.remote).toBe(true);expect(flags.command).toContain(`--remote_executor=${cluster.server.grpcUrl}`);expect(flags.command).toContain("--remote_instance_name=main");
  const helper=join(directory,"helper.sh");writeFileSync(helper,"#!/bin/sh\ncat >/dev/null\nprintf '{\"headers\":{}}\\n'\n",{mode:0o755});
  const cacheFlags=["--remote_cache=http://127.0.0.1:9/ignored","--remote_instance_name=elsewhere",`--credential_helper=cache.example=${helper}`,"--remote_download_outputs=toplevel","--remote_cache_compression"];
  const reconciled=execution.bazelExecutorCacheFlagsV1(cacheFlags,cluster.server.grpcUrl);
  expect(reconciled).toEqual(["--remote_instance_name=elsewhere",`--credential_helper=127.0.0.1=${helper}`,"--remote_download_outputs=toplevel","--remote_cache_compression"]);
  const all=[...reconciled,...flags.command];
  const bazel=async(base:string,args:string[],where=workspace)=>{
   const child=Bun.spawn([binary!,...common(base),...args],{cwd:where,stdout:"pipe",stderr:"pipe",env:environment()}),[stdout,stderr]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text()]);
   return {status:await child.exited,stdout,stderr};
  };
  const build=(base:string,extra:readonly string[],targets=["app:build"],where=workspace)=>bazel(base,["build","--lockfile_mode=off","--incompatible_strict_action_env","--incompatible_autoload_externally=",...extra,...targets.map(id=>"//:"+label(id))],where);
  const bin=async(base:string,extra:readonly string[],where=workspace)=>(await bazel(base,["info","bazel-bin","--lockfile_mode=off","--incompatible_autoload_externally=",...extra],where)).stdout.trim().split("\n").at(-1)!;

  const logRemote=join(directory,"remote.json"),cold=await build("base-remote",[...all,`--execution_log_json_file=${logRemote}`]);
  expect(cold.status,cold.stderr).toBe(0);
  const spawns=readLog(logRemote).filter(row=>row.mnemonic===contract.toolchain.mnemonic);
  expect(spawns.map(row=>row.targetLabel).sort()).toEqual(["lib:generate","app:build"].map(id=>"//:"+label(id)).sort());
  expect(spawns.map(row=>row.runner),"every hermetic spawn ran remotely").toEqual(["remote","remote"]);
  const summary=/INFO: (\d+) processes?: (.*)\./u.exec(cold.stderr);
  expect(summary?.[2],cold.stderr).toMatch(/\b2 remote\b/u);expect(summary?.[2]).not.toMatch(/sandbox|local|worker|standalone/u);
  expect(cluster.events.filter(row=>row.startsWith("assigned")).every(row=>row.endsWith("-> second-process"))).toBe(true);
  expect(cluster.events.filter(row=>row.startsWith("assigned"))).toHaveLength(2);
  expect(cluster.server.scheduler.status().counters).toMatchObject({completed:2,failed:0});
  const outputsRemote=await bin("base-remote",all),snapshotRemote=join(outputsRemote,label("app:build")+".snapshot"),remoteDigest=treeDigest(snapshotRemote);
  const content=JSON.parse(readFileSync(join(snapshotRemote,"snapshot.json"),"utf8"));
  expect(readFileSync(join(snapshotRemote,"files",content.entries[1].file),"utf8")).toBe("ALPHA\nBETA\ngamma\n");

  const logCached=join(directory,"cached.json"),warm=await build("base-warm",[...all,`--execution_log_json_file=${logCached}`]);
  expect(warm.status,warm.stderr).toBe(0);
  expect(warm.stderr,"a clean output base is answered by the shared action cache").toMatch(/\b2 remote cache hits?\b/u);expect(warm.stderr).not.toMatch(/\d+ remote[,.]/u);
  expect(readLog(logCached).filter(row=>row.mnemonic===contract.toolchain.mnemonic).every(row=>/cache hit|remote/u.test(row.runner))).toBe(true);
  expect(cluster.events.filter(row=>row.startsWith("assigned")),"the worker ran nothing for the cached build").toHaveLength(2);

  const localFlags=execution.bazelExecutionFlagsV1({host:{platform:process.platform,arch:process.arch},hermetic:true,jobs:2}).command;
  const localBuild=await build("base-local2",localFlags,["app:build"],local.workspace);expect(localBuild.status,localBuild.stderr).toBe(0);expect(localBuild.stderr).toMatch(/\d+ (darwin-sandbox|linux-sandbox|processwrapper-sandbox|local)/u);
  expect(treeDigest(join(await bin("base-local2",localFlags,local.workspace),label("app:build")+".snapshot")),"the remote result equals the local result byte for byte").toBe(remoteDigest);

  const leak=await build("base-remote",all,["neg:leak"]);expect(leak.status,"an undeclared read fails on the worker as it does locally").not.toBe(0);expect(leak.stderr+leak.stdout).toMatch(/secret\.txt|ENOENT/u);
 }finally{
  workerProcess?.kill("SIGTERM");await workerProcess?.exited;
  for(const base of["base-remote","base-warm","base-local2"])spawnSync(binary!,[`--output_base=${join(directory,base)}`,`--install_base=${join(directory,"install")}`,"--nosystem_rc","--nohome_rc","shutdown"],{timeout:120000});
  await cluster.stop();makeTestTreeWritableV1(directory);rmSync(directory,{recursive:true,force:true});
 }
},1800000);
