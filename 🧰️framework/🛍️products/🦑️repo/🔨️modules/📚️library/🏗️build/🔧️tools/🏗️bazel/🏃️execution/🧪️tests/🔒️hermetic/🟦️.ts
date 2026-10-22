/** 🔒️ Law: a sample hermetic build, run through the generated workspace and the real Bazel with its execution log, reads only declared inputs, holds only the declared toolchain and the strict environment, writes only declared outputs, and produces byte-identical outputs from two clean builds under different umasks. */
import {expect,test} from "bun:test";
import {makeTestTreeWritableV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {createHash} from "node:crypto";
import {cpSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,statSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {networkInterfaces,tmpdir} from "node:os";
import {join,relative} from "node:path";
import schema from "../../🧬️schema/🔣️.json";
import {createNativeBuildCallerCapabilitiesV1} from "../../../../../📥️caller/🟦️.ts";
import {createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import * as execution from "../../🟦️.ts";

const adapter=await import("../../../📜️script.ts");
const binary=process.env.SEMIO_TEST_BAZEL_BINARY,sha=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex"),contract=schema.definitions.Contract.const;
const host=execution.bazelExecutionPlatformV1(process.platform,process.arch);

const files:Record<string,string>={
 "lib/src/a.txt":"alpha\n","lib/src/b.txt":"beta\n","lib/secret.txt":"undeclared\n",
 "lib/gen.ts":`import{mkdirSync,readFileSync,readdirSync,writeFileSync}from"node:fs";const base=process.env.SEMIO_WORKSPACE_ROOT+"/lib";mkdirSync(base+"/gen",{recursive:true});writeFileSync(base+"/gen/out.txt",readdirSync(base+"/src").sort().map(name=>readFileSync(base+"/src/"+name,"utf8").toUpperCase()).join(""));writeFileSync(base+"/gen/tool.sh","#!/bin/sh\\n",{mode:0o700});`,
 "app/src/c.txt":"gamma\n",
 "app/build.ts":`import{mkdirSync,readFileSync,writeFileSync}from"node:fs";const base=process.env.SEMIO_WORKSPACE_ROOT;mkdirSync(base+"/app/dist",{recursive:true});writeFileSync(base+"/app/dist/result.txt",readFileSync(base+"/lib/gen/out.txt","utf8")+readFileSync(base+"/app/src/c.txt","utf8"),{mode:0o664});`,
 "neg/leak.ts":`import{readFileSync}from"node:fs";console.log(readFileSync(process.env.SEMIO_WORKSPACE_ROOT+"/lib/secret.txt","utf8"));`,
 "neg/stray.ts":`import{mkdirSync,writeFileSync}from"node:fs";const base=process.env.SEMIO_WORKSPACE_ROOT+"/neg";mkdirSync(base+"/out",{recursive:true});writeFileSync(base+"/out/ok.txt","ok");writeFileSync(base+"/stray.txt","x");`,
 "neg/net.ts":`import{mkdirSync,writeFileSync}from"node:fs";const base=process.env.SEMIO_WORKSPACE_ROOT+"/neg";await Bun.connect({hostname:process.env.HOST,port:Number(process.env.PORT),socket:{open(socket){socket.end();},data(){},error(){}}});mkdirSync(base+"/net",{recursive:true});writeFileSync(base+"/net/reached.txt","reached");`,
 "neg/iso.ts":`import{mkdirSync,writeFileSync}from"node:fs";const base=process.env.SEMIO_WORKSPACE_ROOT+"/neg";mkdirSync(base+"/iso",{recursive:true});writeFileSync(base+"/iso/ok.txt","ok");`,
};
const hermetic={version:1,tools:["bun"]},target=(command:string,inputs:string[],outputs:string[],dependsOn:string[]=[])=>({execution:{kind:"command",command},outputs,inputs,cache:true,dependsOn,metadata:{semio:{hermetic}}});
const graph={version:1 as const,projects:{
 lib:{name:"lib",root:"lib",targets:{generate:target("bun {projectRoot}/gen.ts",["{projectRoot}/src/**/*","{projectRoot}/gen.ts"],["{projectRoot}/gen"])}},
 app:{name:"app",root:"app",targets:{build:target("bun {projectRoot}/build.ts",["{projectRoot}/**/*"],["{projectRoot}/dist"],["lib:generate"])}},
 neg:{name:"neg",root:"neg",targets:{leak:target("bun {projectRoot}/leak.ts",["{projectRoot}/leak.ts"],["{projectRoot}/out"]),stray:target("bun {projectRoot}/stray.ts",["{projectRoot}/stray.ts"],["{projectRoot}/out"]),iso:target("bun {projectRoot}/iso.ts",["{projectRoot}/iso.ts"],["{projectRoot}/iso"]),net:target("bun {projectRoot}/net.ts",["{projectRoot}/net.ts"],["{projectRoot}/net"])}}},dependencies:[]} as any;
const declared={"lib:generate":["lib/gen.ts","lib/src/a.txt","lib/src/b.txt"],"app:build":["app/build.ts","app/src/c.txt"]};

type Spawn={mnemonic:string;runner:string;commandArgs:string[];environmentVariables:{name:string;value:string}[];inputs:{path:string;isTool?:boolean}[];platform?:{properties?:{name:string;value:string}[]};targetLabel:string};
const readLog=(path:string):Spawn[]=>{const text=readFileSync(path,"utf8"),rows:Spawn[]=[];let depth=0,start=-1,quoted=false,escaped=false;for(let index=0;index<text.length;index++){const char=text[index]!;if(quoted){if(escaped)escaped=false;else if(char==="\\")escaped=true;else if(char==='"')quoted=false;continue;}if(char==='"')quoted=true;else if(char==="{"){if(depth===0)start=index;depth++;}else if(char==="}"&&--depth===0)rows.push(JSON.parse(text.slice(start,index+1)));}return rows;};
const treeDigest=(directory:string):string=>{const rows:string[]=[],visit=(path:string)=>{for(const name of readdirSync(path).sort()){const full=join(path,name),info=statSync(full);if(info.isDirectory())visit(full);else rows.push(relative(directory,full)+"\0"+(info.mode&0o777).toString(8)+"\0"+sha(readFileSync(full)));}};visit(directory);return sha(rows.join("\n"));};

test.skipIf(!binary)("law: hermetic actions read only declared inputs, hold the strict environment and toolchain, write only declared outputs and rebuild byte-identically",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"hermetic-law-")),checkout=join(directory,"checkout");let reached=0;const lan=Object.values(networkInterfaces()).flat().find(row=>row&&row.family==="IPv4"&&!row.internal)?.address,server=Bun.listen({hostname:"0.0.0.0",port:0,socket:{open(socket){reached++;socket.end();},data(){},error(){}}});graph.projects.neg.targets.net.execution.env={PORT:String(server.port),HOST:lan??"127.0.0.1"};
 const environment=(extra:Record<string,string>={})=>({...process.env,...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{}),...extra});
 try{
  for(const[path,text]of Object.entries(files)){mkdirSync(join(checkout,path,".."),{recursive:true});writeFileSync(join(checkout,path),text);}
  const generate=async(name:string,plan:execution.BazelExecutionPlanV1,root=checkout)=>{
   const api=await import("../../../📜️script.ts"),workspace=join(directory,name),request={tasks:["app:build","neg:leak","neg:stray","neg:iso","neg:net"]};
   return withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:hermetic-law",maximumElapsedMilliseconds:120000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["run","app:build"],"bazel",{}),Date.now()),async original=>{
    const planned=api.planBazelTasksV1(graph,request),implementation={version:1 as const,snapshot:join(workspace,"implementation.json"),runner:join(workspace,"📜️script.ts"),sha256:"0".repeat(64),loaderSha256:"1".repeat(64)};
    await api.generateBazelWorkspaceV1(root,graph,request,planned,workspace,original,createIssuedResourceControlV1(original,"bazel:hermetic-law",0),implementation,()=>{},undefined,plan);
    return{workspace,planned};
   });
  };
  const{workspace,planned}=await generate("ws",{host:host.id,tools:{[host.id]:process.execPath}}),label=(id:string)=>planned.tasks[id]!.label;
  const flags=execution.bazelExecutionFlagsV1({host:{platform:process.platform,arch:process.arch},hermetic:true,jobs:2});
  const common=(base:string)=>[`--output_base=${join(directory,base)}`,`--install_base=${join(directory,"install")}`,"--nosystem_rc","--nohome_rc",`--bazelrc=${join(process.cwd(),".bazelrc")}`,"--max_idle_secs=120"];
  const bazel=(base:string,args:string[],options:{umask?:string;env?:Record<string,string>}={})=>{const command=[...common(base),...args];return options.umask?spawnSync("/bin/sh",["-c",`umask ${options.umask}; exec "$0" "$@"`,binary!,...command],{cwd:workspace,encoding:"utf8",timeout:420000,env:environment(options.env)}):spawnSync(binary!,command,{cwd:workspace,encoding:"utf8",timeout:420000,env:environment(options.env)});};
  const build=(base:string,targets:string[],extra:string[]=[],options:{umask?:string}={})=>bazel(base,["build","--lockfile_mode=off","--incompatible_strict_action_env","--incompatible_autoload_externally=",...flags.command,...extra,...targets.map(id=>"//:"+label(id))],options);
  const bin=(base:string)=>bazel(base,["info","bazel-bin","--lockfile_mode=off","--incompatible_autoload_externally=",...flags.command]).stdout.trim().split("\n").at(-1)!;

  const cache=join(directory,"disk"),logA=join(directory,"a.json"),first=build("base-a",["app:build"],[`--execution_log_json_file=${logA}`,`--disk_cache=${cache}`]);expect(first.status,first.stderr).toBe(0);
  const spawns=readLog(logA).filter(row=>row.mnemonic===contract.toolchain.mnemonic);
  expect(spawns.map(row=>row.targetLabel).sort()).toEqual(["lib:generate","app:build"].map(id=>"//:"+label(id)).sort());
  for(const spawn of spawns){
   const id=Object.keys(planned.tasks).find(key=>"//:"+planned.tasks[key]!.label===spawn.targetLabel)!,tool=`tools/bun/${host.id}/${execution.bazelBunExecutableNameV1(host)}`;
   expect(execution.bazelHermeticViolationsV1(spawn,{inputs:[...(declared as Record<string,string[]>)[id]!.map(path=>adapter.bazelSourceAliasLabelV1(path)),"hermetic-runner.mjs",label(id)+".json"],tool,properties:host.properties}),`hermetic violations of ${id}`).toEqual([]);
  }
  const outputs=bin("base-a"),treeA=treeDigest(join(outputs,label("app:build")+".snapshot")),libA=treeDigest(join(outputs,label("lib:generate")+".snapshot"));
  const snapshot=JSON.parse(readFileSync(join(outputs,label("app:build")+".snapshot/snapshot.json"),"utf8"));
  expect(snapshot.entries.map((entry:any)=>[entry.path,entry.mode.toString(8)])).toEqual([["app/dist","755"],["app/dist/result.txt","644"]]);
  expect(readFileSync(join(outputs,label("app:build")+".snapshot/files",snapshot.entries[1].file),"utf8")).toBe("ALPHA\nBETA\ngamma\n");

  const second=build("base-b",["app:build"],[],{umask:"077"});expect(second.status,second.stderr).toBe(0);
  expect(treeDigest(join(bin("base-b"),label("app:build")+".snapshot"))).toBe(treeA);expect(treeDigest(join(bin("base-b"),label("lib:generate")+".snapshot"))).toBe(libA);

  const moved=join(directory,"elsewhere","checkout-copy");mkdirSync(join(directory,"elsewhere"));cpSync(checkout,moved,{recursive:true});
  const copy=await generate("ws-copy",{host:host.id,tools:{[host.id]:process.execPath}},moved),copyBazel=(args:string[])=>spawnSync(binary!,[...common("base-c"),...args],{cwd:copy.workspace,encoding:"utf8",timeout:420000,env:environment()});
  const shared=copyBazel(["build","--lockfile_mode=off","--incompatible_strict_action_env","--incompatible_autoload_externally=",...flags.command,`--disk_cache=${cache}`,"//:"+copy.planned.tasks["app:build"]!.label]);expect(shared.status,shared.stderr).toBe(0);
  expect(shared.stderr,"identical inputs at another checkout path and workspace must reuse the disk cache entries").toMatch(/2 disk cache hits?/u);expect(shared.stderr).not.toMatch(/\d+ (darwin-sandbox|linux-sandbox|processwrapper-sandbox)/u);
  copyBazel(["shutdown"]);

  const leak=build("base-a",["neg:leak"]);expect(leak.status,"an undeclared read must fail").not.toBe(0);expect(leak.stderr+leak.stdout).toMatch(/secret\.txt/u);
  const stray=build("base-a",["neg:stray"]);expect(stray.status,"an undeclared write must fail").not.toBe(0);expect(stray.stderr+stray.stdout).toMatch(/undeclared writes outside the declared outputs: neg\/stray\.txt/u);
  {const local=build("base-local",["neg:iso"],["--strategy=RepoHermetic=local"]);expect(local.status,local.stderr).toBe(0);expect(local.stderr).toMatch(/\d+ local/u);const localLeak=build("base-local",["neg:leak"],["--strategy=RepoHermetic=local"]);expect(localLeak.status,"an undeclared read must fail under the local strategy too").not.toBe(0);expect(localLeak.stderr+localLeak.stdout).toMatch(/secret\.txt/u);}
  if(process.platform==="linux"&&lan!==undefined){const net=build("base-a",["neg:net"]);expect(net.status,"a hermetic action must not reach a non-loopback address").not.toBe(0);expect(reached,"the listener must never be reached").toBe(0);}
  const iso=build("base-a",["neg:iso"]);expect(iso.status,iso.stderr).toBe(0);

  const{workspace:remoteWorkspace,planned:remotePlanned}=await generate("ws-remote",{host:host.id,remote:{platform:"linux-x64",properties:{Arch:"amd64",OSFamily:"linux"}},tools:{[host.id]:process.execPath,"linux-x64":process.execPath}});
  const routed=execution.bazelExecutionFlagsV1({host:{platform:process.platform,arch:process.arch},hermetic:true,jobs:2,environment:{SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://127.0.0.1:9"}}),logR=join(directory,"r.json");
  const remote=spawnSync(binary!,[...common("base-r"),"build","--lockfile_mode=off","--incompatible_strict_action_env","--incompatible_autoload_externally=",...routed.command,`--execution_log_json_file=${logR}`,"//:"+remotePlanned.tasks["neg:iso"]!.label],{cwd:remoteWorkspace,encoding:"utf8",timeout:420000,env:environment()});
  expect(remote.status,"remote execution without a reachable executor must fail, never run locally").not.toBe(0);
  expect(remote.stderr).toMatch(/127\.0\.0\.1:9|UNAVAILABLE|Connection refused|remote/u);
  expect(existsSync(logR)?readLog(logR).filter(row=>row.mnemonic===contract.toolchain.mnemonic&&/sandbox|local|standalone/u.test(row.runner)):[]).toEqual([]);
 }finally{
  server.stop(true);
  spawnSync(binary!,[`--output_base=${join(directory,"base-a")}`,`--install_base=${join(directory,"install")}`,"--nosystem_rc","--nohome_rc","shutdown"],{timeout:120000});
  for(const base of ["base-b","base-r","base-local"])spawnSync(binary!,[`--output_base=${join(directory,base)}`,`--install_base=${join(directory,"install")}`,"--nosystem_rc","--nohome_rc","shutdown"],{timeout:120000});
  makeTestTreeWritableV1(directory);rmSync(directory,{recursive:true,force:true});
 }
},1200000);
