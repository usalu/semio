/** 🏃️ Contract: the Bazel execution contract declares digest-pinned bun toolchains and exec platforms from the neutral schema, routes only hermetic actions to a remote pool, and its self-contained runner is deterministic and isolated. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {chmodSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readlinkSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import hermeticSchema from "../../../../🏃️execution/🧬️schema/🔒️hermetic.json";
import bunManifest from "../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🟦️bun/🧬️schema/🔣️.json";
import * as api from "../🟦️.ts";

const python=testPythonV1(),sha=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex"),contract=schema.definitions.Contract.const;
const run=(program:string,input:unknown)=>Bun.spawnSync([python!,"-I","-c",program],{stdin:new TextEncoder().encode(JSON.stringify(input)),stdout:"pipe",stderr:"pipe"});
const task=(definition:Record<string,unknown>)=>({id:"fixture:task",project:"fixture",target:"task",label:"t_0000000000000000",dependencies:[],args:[],root:"fixture",definition}) as any;

test("the execution contract is closed, names six platforms and pins each bun archive from the release manifest",()=>{
 const ajv=new Ajv({strict:true}).addSchema(schema,"component");
 expect(ajv.compile({$ref:"component#/definitions/ContractShape"})(contract)).toBe(true);
 expect(ajv.compile({$ref:"component#/definitions/AuditShape"})(schema.definitions.Audit.const)).toBe(true);
 expect(contract.platforms.map(row=>row.id).sort()).toEqual(["darwin-arm64","darwin-x64","linux-arm64","linux-x64","windows-arm64","windows-x64"]);
 expect(contract.remote.localFallback).toBe(false);
 expect(contract.platforms.some(row=>row.id===contract.remote.defaultPlatform)).toBe(true);
 const names=Object.keys(schema.definitions.Environment.properties);for(const name of Object.values(contract.environment))expect(names).toContain(name);
 for(const row of contract.platforms){
  const pin=api.bazelBunDistributionV1(row.id);
  expect(pin.sha256).toMatch(/^[a-f0-9]{64}$/u);expect(pin.bytes).toBeGreaterThan(1000000);
  if(python){const result=run("import json,sys\nrows,platform,arch=json.loads(sys.stdin.read())\nprint(json.dumps([r['sha256'] for r in rows if r['platform']==platform and r['arch']==arch and not r['musl']]))",[bunManifest.definitions.Release.const.distributions,row.platform,row.arch]);expect(result.exitCode,result.stderr.toString()).toBe(0);expect(JSON.parse(result.stdout.toString())).toEqual([pin.sha256]);}
  expect(api.bazelBunExecutableNameV1(row)).toBe(row.platform==="win32"?"bun.exe":"bun");
  expect(api.bazelBunToolPathV1(row)).toBe(`tools/bun/${row.id}/${api.bazelBunExecutableNameV1(row)}`);
 }
 expect(ajv.compile({$ref:"component#/definitions/ContractShape"})({...contract,remote:{...contract.remote,localFallback:true}})).toBe(false);
 expect(ajv.compile({$ref:"component#/definitions/ContractShape"})({...contract,platforms:contract.platforms.slice(1)})).toBe(false);
});

test("the remote execution switch agrees with an independent Python parser and the closed environment schema",()=>{
 const admit=new Ajv({strict:true}).addSchema(schema,"component").compile({$ref:"component#/definitions/Environment"});
 for(const row of fixture.environment){
  const environment=row.environment as Record<string,string>,selected=Object.fromEntries(Object.entries(environment).filter(([,value])=>value!==""));
  if(row.expected&&"error" in row.expected){expect(admit(selected),row.name).toBe(false);expect(()=>api.bazelRemoteExecutionV1(environment),row.name).toThrow(row.expected.error);continue;}
  expect(admit(selected),row.name).toBe(true);
  const actual=api.bazelRemoteExecutionV1(environment);expect((actual===undefined?null:{...actual}) as unknown,row.name).toEqual(row.expected);
  if(python&&actual){
   const result=run("import json,sys\nenvironment,platforms,defaults=json.loads(sys.stdin.read())\nplatform=environment.get('SEMIO_REMOTE_EXECUTION_PLATFORM',defaults['platform'])\nproperties=dict(next(p for p in platforms if p['id']==platform)['properties'])\nfor pair in filter(None,environment.get('SEMIO_REMOTE_EXECUTION_PROPERTIES','').split(',')):\n    key,_,value=pair.partition('=')\n    properties[key]=value\njobs=min(defaults['maximum'],int(environment.get('SEMIO_REMOTE_EXECUTION_JOBS',defaults['jobs'])))\nprint(json.dumps({'endpoint':environment['SEMIO_REMOTE_EXECUTION_ENDPOINT'],'platform':platform,'properties':dict(sorted(properties.items())),'jobs':jobs}))",[environment,contract.platforms,{platform:contract.remote.defaultPlatform,maximum:contract.remote.maximumJobs,jobs:contract.remote.defaultJobs}]);
   expect(result.exitCode,result.stderr.toString()).toBe(0);expect(JSON.parse(result.stdout.toString()),row.name).toEqual({...actual});
  }
 }
});

test("execution flags register only what hermetic actions need, route them to the remote pool without local fallback and leave cache flags alone",()=>{
 const admit=new Ajv({strict:true}).addSchema(schema,"component").compile({$ref:"component#/definitions/Flags"});
 for(const row of fixture.flags){
  const options={host:row.host,hermetic:row.hermetic,jobs:row.jobs,environment:row.environment as Record<string,string>};
  if("error" in row.expected){expect(()=>api.bazelExecutionFlagsV1(options),row.name).toThrow(row.expected.error);continue;}
  const actual=api.bazelExecutionFlagsV1(options);expect({...actual,command:[...actual.command]},row.name).toEqual(row.expected);expect(admit(actual),row.name).toBe(true);
  for(const flag of actual.command)expect(flag,row.name).not.toMatch(/remote_cache|disk_cache|remote_upload|remote_instance_name|credential_helper/u);
 }
});

test("hermetic decisions fail closed with their authored reason and agree with the neutral declaration schema",()=>{
 const admit=new Ajv({strict:true}).compile(hermeticSchema),reasons=schema.definitions.Reasons.const;
 for(const row of fixture.decisions){
  const definition=row.definition as Record<string,any>,declared=definition.metadata?.semio?.hermetic;
  if(declared!==undefined)expect(admit(declared),row.name).toBe("error" in row.expected?false:true);
  if("error" in row.expected){expect(()=>api.bazelHermeticDecisionV1(task(definition),{cache:row.cache},row.inputs),row.name).toThrow(row.expected.error);continue;}
  const actual=api.bazelHermeticDecisionV1(task(definition),{cache:row.cache},row.inputs);expect(actual,row.name).toEqual(row.expected as unknown as typeof actual);
  if(!actual.hermetic)expect(Object.keys(reasons),row.name).toContain(actual.reason);
 }
 expect(admit({version:1,tools:[]})).toBe(false);expect(admit({version:1,tools:["bun"],network:true})).toBe(false);expect(admit({version:2,tools:["bun"]})).toBe(false);
});

test("hermetic manifests are closed, content-stable and carry no path, environment or implementation identity",()=>{
 const definition={execution:{kind:"command",commands:["bun {projectRoot}/a.ts","bun {projectRoot}/b.ts"],env:{MODE:"x"},parallel:false},outputs:["{projectRoot}/gen"]},admit=new Ajv({strict:true}).addSchema(schema,"component").compile({$ref:"component#/definitions/Manifest"});
 const first=JSON.stringify(api.bazelHermeticManifestV1(task(definition),["--flag"])),second=JSON.stringify(api.bazelHermeticManifestV1(task(structuredClone(definition)),["--flag"])),manifest=JSON.parse(first);
 expect(first).toBe(second);expect(admit(manifest)).toBe(true);
 expect(first).not.toMatch(/\/Users\/|\/home\/|implementation|runtime|workspaces/u);
 for(const extra of [{workspace:"/repo"},{runtime:"/x/runtime.json"},{implementation:{sha256:"a"}}])expect(admit({...manifest,...extra}),JSON.stringify(extra)).toBe(false);
 expect(admit({...manifest,commands:[]})).toBe(false);expect(admit({...manifest,version:2})).toBe(false);
});

test.skipIf(!python)("the rendered execution BUILD and hermetic rules parse as Starlark and declare exactly the contract's platforms, pools and pinned toolchains",()=>{
 const parse=(source:string)=>run("import ast,json,sys\ntree=ast.parse(json.loads(sys.stdin.read()))\ndef value(node):\n    try:\n        return ast.literal_eval(node)\n    except Exception:\n        return ast.unparse(node)\nrows=[]\nfor statement in tree.body:\n    if isinstance(statement,ast.Expr) and isinstance(statement.value,ast.Call):\n        call=statement.value\n        rows.append({'function':call.func.id,'attributes':{keyword.arg:value(keyword.value) for keyword in call.keywords}})\nprint(json.dumps(rows))",source);
 const host=api.bazelExecutionPlatformByIdV1("darwin-arm64"),local=parse(api.renderBazelExecutionBuildV1({host:host.id,tools:{"darwin-arm64":"/store/bun"}}));
 expect(local.exitCode,local.stderr.toString()).toBe(0);
 const rows=JSON.parse(local.stdout.toString()) as {function:string;attributes:Record<string,any>}[],named=(name:string)=>rows.find(row=>row.attributes.name===name)!;
 expect(rows.filter(row=>row.function==="platform").map(row=>row.attributes.name)).toEqual(["exec_darwin_arm64"]);
 expect(named("exec_darwin_arm64").attributes.exec_properties).toEqual(host.properties);expect(named("exec_darwin_arm64").attributes.constraint_values).toEqual([":exec_os_macos",":exec_cpu_aarch64",":exec_pool_local"]);
 expect(rows.filter(row=>row.function==="constraint_value").length).toBe(3+2+2);
 expect(named("bun_tool_darwin_arm64").attributes).toEqual({name:"bun_tool_darwin_arm64",executable:"tools/bun/darwin-arm64/bun",pin:api.bazelBunDistributionV1("darwin-arm64").sha256});
 expect(named("bun_toolchain_darwin_arm64").attributes.exec_compatible_with).toEqual([":exec_os_macos",":exec_cpu_aarch64"]);expect(named("bun_toolchain_darwin_arm64").attributes.toolchain_type).toBe(":bun_toolchain_type");
 const remote=parse(api.renderBazelExecutionBuildV1({host:host.id,remote:{platform:"linux-x64",properties:{Arch:"amd64",OSFamily:"linux",pool:"repo"}},tools:{"darwin-arm64":"/store/bun","linux-x64":"/store/bun-linux"}}));
 expect(remote.exitCode,remote.stderr.toString()).toBe(0);
 const rows2=JSON.parse(remote.stdout.toString()) as {function:string;attributes:Record<string,any>}[],pool=rows2.find(row=>row.attributes.name==="exec_remote_linux_x64")!;
 expect(pool.attributes.constraint_values).toEqual([":exec_os_linux",":exec_cpu_x86_64",":exec_pool_remote"]);expect(pool.attributes.exec_properties).toEqual({Arch:"amd64",OSFamily:"linux",pool:"repo"});
 expect(rows2.filter(row=>row.function==="toolchain").map(row=>row.attributes.name)).toEqual(["bun_toolchain_darwin_arm64","bun_toolchain_linux_x64"]);
 const rules=parse(api.BAZEL_HERMETIC_STARLARK);expect(rules.exitCode,rules.stderr.toString()).toBe(0);
 expect(api.BAZEL_HERMETIC_STARLARK).not.toMatch(/no-remote|no-sandbox|use_default_shell_env = True|local"/u);
 expect(api.BAZEL_HERMETIC_STARLARK).toContain('"block-network": "1"');expect(api.BAZEL_HERMETIC_STARLARK).toContain('mnemonic = "RepoHermetic"');
 for(const[key,value]of Object.entries(contract.actionEnvironment))expect(api.BAZEL_HERMETIC_STARLARK).toContain(`"${key}": "${value}"`);
});

const graph={version:1 as const,projects:{
 lib:{name:"lib",root:"lib",targets:{generate:{execution:{kind:"command",command:"bun {projectRoot}/gen.ts"},outputs:["{projectRoot}/gen"],inputs:["{projectRoot}/src/**/*","{projectRoot}/gen.ts"],cache:true,metadata:{semio:{hermetic:{version:1,tools:["bun"]}}}}}},
 app:{name:"app",root:"app",targets:{build:{execution:{kind:"command",command:"bun {projectRoot}/build.ts"},outputs:["{projectRoot}/dist"],inputs:["{projectRoot}/**/*"],dependsOn:["lib:generate"],cache:true,metadata:{semio:{hermetic:{version:1,tools:["bun"]}}}},host:{execution:{kind:"command",command:"bun {projectRoot}/host.ts"},outputs:["{projectRoot}/host"],inputs:["{projectRoot}/**/*"],dependsOn:["lib:generate"],cache:true}}}},dependencies:[]} as any;

test("hermetic classification is opt-in per target and per run, leaves host actions untouched and renders a routable BUILD",async()=>{
 const sources=await import("../../📜️script.ts"),plan=sources.planBazelTasksV1(graph,{tasks:["app:build","app:host"]}),off=sources.bazelActionsV1(graph,plan,false),on=sources.bazelActionsV1(graph,plan,false,{hermetic:true});
 expect([...off.values()].map(action=>action.kind).sort()).toEqual(["task","task","task"]);
 expect(Object.fromEntries([...on.values()].map(action=>[action.id,action.kind]))).toEqual({"lib:generate":"hermetic","app:build":"hermetic","app:host":"task"});
 expect(on.get("app:build")!.restore).toEqual([plan.tasks["lib:generate"]!.label]);
 const build=(execution?:string,remote?:boolean)=>sources.renderBazelBuildV1({actions:[...on.values()].map(action=>({...action,...(action.kind==="hermetic"?{sources:"i_0123456789abcdef"}:{})})),groups:new Map([["i_0123456789abcdef",{groups:[],files:["lib/gen.ts"]}]]),bun:"/opt/bun",sourceIdentity:"a".repeat(64),extension:"",execution,remote}),plain=build(),routed=build(api.renderBazelExecutionBuildV1({host:"darwin-arm64",remote:{platform:"linux-x64",properties:{Arch:"amd64",OSFamily:"linux"}},tools:{"darwin-arm64":"/s/a","linux-x64":"/s/b"}}),true);
 expect(plain.split("\n")[0]).toBe('load(":tasks.bzl", "repo_hermetic_task", "repo_launch", "repo_noop", "repo_task")');
 expect(routed.split("\n")[0]).toBe('load(":tasks.bzl", "repo_bun_toolchain", "repo_hermetic_task", "repo_launch", "repo_noop", "repo_task")');
 expect(plain.match(/^repo_hermetic_task\(/gmu)?.length).toBe(2);expect(plain.match(/^repo_task\(/gmu)?.length).toBe(1);
 expect(plain).not.toContain("exec_compatible_with");expect(routed.match(/exec_compatible_with = \[":exec_pool_remote"\]/gu)?.length).toBe(2);
 expect(plain).toContain('runner = "hermetic-runner.mjs"');
 const baseline=sources.renderBazelBuildV1({actions:[...off.values()],groups:new Map(),bun:"/opt/bun",sourceIdentity:"a".repeat(64),extension:""});expect(baseline.split("\n")[0]).toBe('load(":tasks.bzl", "repo_launch", "repo_noop", "repo_task")');
 expect(()=>sources.bazelActionsV1(graph,sources.planBazelTasksV1({...graph,projects:{...graph.projects,lib:{...graph.projects.lib,targets:{generate:{...graph.projects.lib.targets.generate,metadata:{semio:{hermetic:{version:1,tools:["cargo"]}}}}}}}},{tasks:["lib:generate"]}),false,{hermetic:true})).toThrow("Closed hermetic declaration required: lib:generate");
});

test.skipIf(!python)("bun archives are verified against their pin before extraction and agree with Python's zipfile",()=>{
 const directory=mkdtempSync(join(tmpdir(),"bun-pin-")),archive=join(directory,"fixture.zip");
 try{
  const made=run("import json,sys,zipfile\npath,member,payload=json.loads(sys.stdin.read())\nwith zipfile.ZipFile(path,'w',zipfile.ZIP_DEFLATED) as z:\n    z.writestr(member,payload)",[archive,fixture.archive.member,fixture.archive.payload]);expect(made.exitCode,made.stderr.toString()).toBe(0);
  const bytes=new Uint8Array(readFileSync(archive)),pin={bytes:bytes.length,sha256:sha(bytes)};
  expect(api.verifyBazelBunArchiveV1(bytes,pin,fixture.archive.member).toString()).toBe(fixture.archive.payload);
  const oracle=run("import hashlib,json,sys,zipfile\npath,member=json.loads(sys.stdin.read())\nz=zipfile.ZipFile(path)\nprint(json.dumps({'sha':hashlib.sha256(open(path,'rb').read()).hexdigest(),'payload':z.read(member).decode()}))",[archive,fixture.archive.member]);expect(JSON.parse(oracle.stdout.toString())).toEqual({sha:pin.sha256,payload:fixture.archive.payload});
  expect(()=>api.verifyBazelBunArchiveV1(bytes,{...pin,sha256:"0".repeat(64)},fixture.archive.member)).toThrow("Bun archive checksum mismatch");
  expect(()=>api.verifyBazelBunArchiveV1(bytes,{...pin,bytes:pin.bytes+1},fixture.archive.member)).toThrow("Bun archive checksum mismatch");
  expect(()=>api.verifyBazelBunArchiveV1(bytes,pin,"missing/bun")).toThrow();
  const tampered=new Uint8Array(bytes);tampered[tampered.length>>1]^=1;expect(()=>api.verifyBazelBunArchiveV1(tampered,pin,fixture.archive.member)).toThrow("Bun archive checksum mismatch");
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("toolchain links point at the provisioned executable and are replaced when it moves",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"bun-link-")),first=join(directory,"first"),second=join(directory,"second");
 try{
  writeFileSync(first,"1");writeFileSync(second,"2");const link=join(directory,"ws",api.bazelBunToolPathV1(api.bazelExecutionPlatformByIdV1("linux-x64")));
  await api.linkBazelToolchainsV1(join(directory,"ws"),{"linux-x64":first});expect(readlinkSync(link)).toBe(first);
  await api.linkBazelToolchainsV1(join(directory,"ws"),{"linux-x64":first});expect(readlinkSync(link)).toBe(first);
  await api.linkBazelToolchainsV1(join(directory,"ws"),{"linux-x64":second});expect(readlinkSync(link)).toBe(second);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

const exec=(root:string,args:string[],options:{umask?:string}={})=>Bun.spawnSync(options.umask?["/bin/sh","-c",`umask ${options.umask}; exec "$0" "$@"`,process.execPath,...args]:[process.execPath,...args],{cwd:root,env:{},stdout:"pipe",stderr:"pipe"});
const sandboxRoot=(files:Record<string,string>,manifest:Record<string,unknown>)=>{
 const root=mkdtempSync(join(tmpdir(),"hermetic-run-"));
 for(const[path,text]of Object.entries(files)){mkdirSync(join(root,"s",path.split("/").slice(0,-1).join("/")),{recursive:true});writeFileSync(join(root,"s",path),text);}
 writeFileSync(join(root,"hermetic-runner.mjs"),api.BAZEL_HERMETIC_RUNNER);writeFileSync(join(root,"manifest.json"),JSON.stringify({version:1,id:"fixture:gen",project:"fixture",target:"gen",root:"lib",args:[],commands:["bun {projectRoot}/gen.ts"],outputs:["{projectRoot}/gen"],...manifest}));
 return root;
};
const generator=`import{chmodSync,mkdirSync,readFileSync,writeFileSync}from"node:fs";
const root=process.env.SEMIO_WORKSPACE_ROOT+"/lib";mkdirSync(root+"/gen/sub",{recursive:true});
const lines=readFileSync(root+"/src/a.txt","utf8").split("\\n").filter(Boolean).map(line=>line.toUpperCase());
writeFileSync(root+"/gen/out.txt",lines.join("\\n")+"\\n");writeFileSync(root+"/gen/sub/private.txt","p");chmodSync(root+"/gen/sub/private.txt",0o600);writeFileSync(root+"/gen/sub/group.txt","g");chmodSync(root+"/gen/sub/group.txt",0o664);writeFileSync(root+"/gen/sub/tool.sh","#!/bin/sh\\n");chmodSync(root+"/gen/sub/tool.sh",0o700);
writeFileSync(root+"/gen/env.txt",Object.keys(process.env).sort().join("\\n")+"\\n");
`;

test.skipIf(!python)("the hermetic runner snapshots declared outputs deterministically, independent of umask, and matches an independent Python verifier",()=>{
 const roots=[0,1].map(()=>sandboxRoot({"lib/src/a.txt":"alpha\nbeta\n","lib/gen.ts":generator},{commands:["bun {projectRoot}/gen.ts"]})),results=roots.map((root,index)=>exec(root,["hermetic-runner.mjs","manifest.json","out"],{umask:index?"077":"022"}));
 try{
  for(const result of results)expect(result.exitCode,result.stderr.toString()).toBe(0);
  const texts=roots.map(root=>readFileSync(join(root,"out/snapshot.json"),"utf8"));expect(texts[0]).toBe(texts[1]);
  const snapshot=JSON.parse(texts[0]!);expect(snapshot.roots).toEqual(["lib/gen"]);
  expect(snapshot.entries.map((entry:any)=>[entry.path,entry.kind,entry.mode.toString(8)])).toEqual([["lib/gen","directory","755"],["lib/gen/env.txt","file","644"],["lib/gen/out.txt","file","644"],["lib/gen/sub","directory","755"],["lib/gen/sub/group.txt","file","644"],["lib/gen/sub/private.txt","file","644"],["lib/gen/sub/tool.sh","file","755"]]);
  for(const index of [0,1])expect(sha(readFileSync(join(roots[index]!,"out/files",snapshot.entries.find((entry:any)=>entry.path==="lib/gen/out.txt").file)))).toBe(sha("ALPHA\nBETA\n"));
  const verify=run("import hashlib,json,os,sys\nout,snapshot=json.loads(sys.stdin.read())\nbad=[]\npaths=[e['path'] for e in snapshot['entries']]\nif paths!=sorted(paths): bad.append('unsorted')\nfor e in snapshot['entries']:\n    if e['kind']=='file':\n        data=open(os.path.join(out,'files',e['file']),'rb').read()\n        if hashlib.sha256(data).hexdigest()!=e['sha256']: bad.append('digest '+e['path'])\n        if hashlib.sha256(e['path'].encode()).hexdigest()[:32]!=e['file']: bad.append('id '+e['path'])\nprint(json.dumps(bad))",[join(roots[0]!,"out"),snapshot]);expect(JSON.parse(verify.stdout.toString())).toEqual([]);
  const keys=readFileSync(join(roots[0]!,"s/lib/gen/env.txt"),"utf8").trim().split("\n");
  expect(keys).toEqual(expect.arrayContaining(["HOME","LC_ALL","PATH","SEMIO_BUILD_TOOL","SEMIO_HERMETIC","SEMIO_TASK_PROJECT","SEMIO_TASK_TARGET","SEMIO_WORKSPACE_ROOT","SOURCE_DATE_EPOCH","TEMP","TMP","TMPDIR","TZ"]));
  for(const key of keys)expect(key,"leaked host variable").toMatch(/^(HOME|LC_ALL|PATH|SEMIO_[A-Z_]+|SOURCE_DATE_EPOCH|TEMP|TMP|TMPDIR|TZ|BUN_[A-Z_]+|_|PWD|SHLVL|OLDPWD)$/u);
 }finally{for(const root of roots)rmSync(root,{recursive:true,force:true});}
});

test("the hermetic runner refuses non-isolated roots, undeclared writes, escaping paths, symbolic outputs and overlapping dependency outputs",()=>{
 const cases:{name:string;files:Record<string,string>;manifest:Record<string,unknown>;prepare?:(root:string)=>void;snapshots?:(root:string)=>string[];message:RegExp}[]=[
  {name:"undeclared write",files:{"lib/src/a.txt":"a\n","lib/gen.ts":generator,"lib/stray.ts":"import{writeFileSync}from'node:fs';writeFileSync(process.env.SEMIO_WORKSPACE_ROOT+'/lib/stray.txt','x')"},manifest:{commands:["bun {projectRoot}/gen.ts","bun {projectRoot}/stray.ts"],parallel:false},message:/undeclared writes outside the declared outputs: lib\/stray\.txt/u},
  {name:"escaping output pattern",files:{"lib/src/a.txt":"a\n","lib/gen.ts":generator},manifest:{outputs:["{projectRoot}/../../escape"]},message:/path escapes the execution root/u},
  {name:"failing command",files:{"lib/fail.ts":"process.exit(3)"},manifest:{commands:["bun {projectRoot}/fail.ts"]},message:/command failed \(3\)/u},
  {name:"symbolic output",files:{"lib/link.ts":"import{mkdirSync,symlinkSync}from'node:fs';const base=process.env.SEMIO_WORKSPACE_ROOT;mkdirSync(base+'/lib/gen',{recursive:true});symlinkSync('../src',base+'/lib/gen/up');"},manifest:{commands:["bun {projectRoot}/link.ts"],outputs:["{projectRoot}/gen"]},message:/must not be a symbolic link/u},
  {name:"overlapping dependency output",files:{"lib/src/a.txt":"a\n"},manifest:{commands:["echo ok"],outputs:[]},snapshots:root=>{mkdirSync(join(root,"dep/files"),{recursive:true});writeFileSync(join(root,"dep/files/"+sha("lib/src/a.txt").slice(0,32)),"b");writeFileSync(join(root,"dep/snapshot.json"),JSON.stringify({version:1,roots:["lib/src/a.txt"],entries:[{path:"lib/src/a.txt",kind:"file",mode:420,file:sha("lib/src/a.txt").slice(0,32),sha256:sha("b")}]}));return ["dep"];},message:/overlaps a declared input/u},
  {name:"symbolic execution root",files:{"lib/src/a.txt":"a\n"},manifest:{commands:["echo ok"],outputs:[]},prepare:root=>{rmSync(join(root,"s"),{recursive:true});mkdirSync(join(root,"checkout"));symlinkSync(join(root,"checkout"),join(root,"s"),"dir");},message:/isolated execution root is required/u},
 ];
 for(const row of cases){
  const root=sandboxRoot(row.files,row.manifest);
  try{row.prepare?.(root);const result=exec(root,["hermetic-runner.mjs","manifest.json","out",...(row.snapshots?.(root)??[])]);expect(result.exitCode,row.name).not.toBe(0);expect(result.stderr.toString(),row.name).toMatch(row.message);}
  finally{rmSync(root,{recursive:true,force:true});}
 }
});

test("the hermetic runner restores dependency snapshots into the execution root before the task runs",()=>{
 const root=sandboxRoot({"app/build.ts":"import{mkdirSync,readFileSync,writeFileSync}from'node:fs';const base=process.env.SEMIO_WORKSPACE_ROOT;mkdirSync(base+'/app/dist',{recursive:true});writeFileSync(base+'/app/dist/result.txt',readFileSync(base+'/lib/gen/out.txt','utf8').toLowerCase());"},{project:"app",target:"build",root:"app",commands:["bun {projectRoot}/build.ts"],outputs:["{projectRoot}/dist"]});
 try{
  const id=sha("lib/gen/out.txt").slice(0,32);mkdirSync(join(root,"dep/files"),{recursive:true});writeFileSync(join(root,"dep/files",id),"ALPHA\n");
  writeFileSync(join(root,"dep/snapshot.json"),JSON.stringify({version:1,roots:["lib/gen"],entries:[{path:"lib/gen",kind:"directory",mode:493},{path:"lib/gen/out.txt",kind:"file",mode:420,file:id,sha256:sha("ALPHA\n")}]}));
  const result=exec(root,["hermetic-runner.mjs","manifest.json","out","dep"]);expect(result.exitCode,result.stderr.toString()).toBe(0);
  const snapshot=JSON.parse(readFileSync(join(root,"out/snapshot.json"),"utf8"));expect(snapshot.entries.map((entry:any)=>entry.path)).toEqual(["app/dist","app/dist/result.txt"]);
  expect(readFileSync(join(root,"out/files",snapshot.entries[1].file),"utf8")).toBe("alpha\n");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the hermeticity judge accepts a clean spawn and flags every violation class the law guards",()=>{
 const host=api.bazelExecutionPlatformByIdV1("darwin-arm64"),tool="tools/bun/darwin-arm64/bun",inputs=["s/lib/a.txt","hermetic-runner.mjs","t_1.json"],environmentVariables=Object.entries(contract.actionEnvironment).map(([name,value])=>({name,value})),properties=Object.entries(host.properties).map(([name,value])=>({name,value}));
 const clean={mnemonic:"RepoHermetic",runner:"darwin-sandbox",commandArgs:[tool,"hermetic-runner.mjs","t_1.json","bazel-out/x/t_1.snapshot"],environmentVariables,inputs:[...inputs.map(path=>({path})),{path:tool,isTool:true},{path:"bazel-out/x/t_0.snapshot/snapshot.json"},{path:`bazel-out/x/t_0.snapshot/files/${"a".repeat(32)}`}],platform:{properties},targetLabel:"//:t_1"},expected={inputs,tool,properties:host.properties};
 expect(api.bazelHermeticViolationsV1(clean,expected)).toEqual([]);
 expect(api.bazelHermeticViolationsV1({...clean,runner:"remote"},expected)).toEqual([]);
 const rows:[string,Partial<typeof clean>,string][]=[
  ["unisolated runner",{runner:"local"},"runner: local"],
  ["undeclared input",{inputs:[...clean.inputs,{path:"s/lib/secret.txt"}]},"undeclared input: s/lib/secret.txt"],
  ["undeclared dependency file",{inputs:[...clean.inputs,{path:"bazel-out/x/t_0.snapshot/other"}]},"undeclared input: bazel-out/x/t_0.snapshot/other"],
  ["missing input",{inputs:clean.inputs.filter(input=>input.path!=="s/lib/a.txt")},"missing input: s/lib/a.txt"],
  ["tool not declared",{inputs:clean.inputs.map(input=>({path:input.path}))},"tool inputs: none"],
  ["host variable",{environmentVariables:[...environmentVariables,{name:"PATH",value:"/usr/bin"}]},`environment: ${[...Object.keys(contract.actionEnvironment),"PATH"].join(",")}`],
  ["absolute path",{commandArgs:["/usr/local/bin/bun",...clean.commandArgs.slice(1)]},"absolute argument: /usr/local/bin/bun"],
  ["wrong platform",{platform:{properties:[]}},"platform properties"],
 ];
 for(const[name,change,message]of rows)expect(api.bazelHermeticViolationsV1({...clean,...change},expected),name).toEqual([message]);
});

test("the hermeticity audit names a violation for every host-state action kind and none for the hermetic ones",()=>{
 const audit=schema.definitions.Audit.const,names=Object.keys(audit.violations);
 expect(Object.keys(audit.kinds).sort()).toEqual(["hermetic","launch","launcher","noop","task"]);
 for(const[kind,row]of Object.entries(audit.kinds)){
  for(const violation of row.violations)expect(names,kind).toContain(violation);
  expect(row.violations.length===0,kind).toBe(row.hermetic);
 }
 expect(audit.kinds.task.violations).toEqual(names);
 const sandbox=schema.definitions.Audit.const.kinds;expect(sandbox.noop.hermetic&&sandbox.hermetic.hermetic).toBe(true);
});

test("hermetic actions disable Bun's content-keyed transpiler cache so identical sources under a second root resolve their own imports",()=>{
 const sources={"lib/gen.ts":`import{mkdirSync,writeFileSync}from"node:fs";const spec="."+"/dep.ts",{where}=await import(spec);const base=process.env.SEMIO_WORKSPACE_ROOT+"/lib";mkdirSync(base+"/gen",{recursive:true});writeFileSync(base+"/gen/where.txt",where+"|"+process.env.BUN_RUNTIME_TRANSPILER_CACHE_PATH);`,"lib/dep.ts":`export const where=import.meta.dir;`,"lib/src/a.txt":"a\n"};
 expect(schema.definitions.Contract.const.actionEnvironment.BUN_RUNTIME_TRANSPILER_CACHE_PATH).toBe("0");expect(api.BAZEL_HERMETIC_STARLARK).toContain('"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0"');
 const roots=[0,1].map(()=>sandboxRoot(sources,{}));
 try{
  const seen=roots.map(root=>{const result=exec(root,["hermetic-runner.mjs","manifest.json","out"]);expect(result.exitCode,result.stderr.toString()).toBe(0);return readFileSync(join(root,"s/lib/gen/where.txt"),"utf8").split("|");});
  roots.forEach((root,index)=>{expect(seen[index]![1]).toBe("0");expect(seen[index]![0]!.replace(/^\/private/u,"")).toBe(join(root,"s/lib").replace(/^\/private/u,""));});
  expect(seen[0]![0]).not.toBe(seen[1]![0]);
 }finally{for(const root of roots)rmSync(root,{recursive:true,force:true});}
});

test("hermetic class membership is decided from the class table, admits only declared host inputs and yields closed, location-independent manifests",()=>{
 const member=(definition:Record<string,unknown>,target="test",project="@teaching/architecture-course")=>({...task(definition),project,target,id:project+":"+target,root:"proj"}) as any,base={execution:{kind:"command",command:"bun ./📜️script.ts test",cwd:"proj"},outputs:[]};
 const decide=(definition:Record<string,unknown>,inputs:unknown[],cache=true,target="test",project="@teaching/architecture-course")=>api.bazelHermeticDecisionV1(member(definition,target,project),{cache},inputs);
 expect(decide(base,["{projectRoot}/**/*",{env:"SEMIO_TEST_LEVEL"},{runtime:"bun --version"},{externalDependencies:["zod"]}])).toEqual({hermetic:true,class:"ts-test"});
 expect(decide(base,[{env:"HOME"}])).toEqual({hermetic:false,reason:"host-inputs"});
 expect(decide(base,[{runtime:"git rev-parse HEAD"}])).toEqual({hermetic:false,reason:"host-inputs"});
 expect(decide({...base,outputs:undefined},[])).toEqual({hermetic:false,reason:"uncached"});
 expect(decide(base,[],false)).toEqual({hermetic:false,reason:"uncached"});
 expect(decide(base,[],true,"test","@semio-tech/other")).toEqual({hermetic:false,reason:"undeclared"});
 expect(decide(base,[],true,"publish")).toEqual({hermetic:false,reason:"undeclared"});
 expect(decide({...base,execution:{...base.execution,program:"x"}},[])).toEqual({hermetic:false,reason:"unsupported-execution"});
 expect(decide({...base,metadata:{semio:{hermetic:{version:1,tools:["bun"]}}}},[],true,"publish")).toEqual({hermetic:true});
 expect(decide({...base,continuous:true},[])).toEqual({hermetic:false,reason:"continuous"});
 const reasons=Object.keys(schema.definitions.Reasons.const);for(const reason of ["excepted","dependencies"])expect(reasons).toContain(reason);
 const admit=new Ajv({strict:true}).addSchema(schema,"component").compile({$ref:"component#/definitions/Manifest"}),row={path:"node_modules",workspace:{"@a/b":"lib"},tree:{zod:"0/zod@1/node_modules/zod"},bin:{tsc:"../typescript/bin/tsc"}};
 const manifest=api.bazelHermeticManifestV1(member(base),["--x"],{inputs:["proj/a.ts","proj/📜️script.ts"],class:"ts-test",inputEnv:{SEMIO_TEST_LEVEL:"quick"},providers:{"typescript-check":"p/🟦️.ts"},rows:[row]});
 expect(admit(manifest),JSON.stringify(admit.errors)).toBe(true);expect(manifest.class).toBe("ts-test");expect(manifest.inputs).toEqual(["proj/a.ts","proj/📜️script.ts"]);expect(manifest.nodeModules).toEqual([row]);
 expect(JSON.stringify(manifest)).not.toMatch(/\/Users\/|\/home\/|\/var\/|\/private\//u);
 expect(JSON.stringify(api.bazelHermeticManifestV1(member(structuredClone(base)),["--x"],{inputs:["proj/a.ts","proj/📜️script.ts"],class:"ts-test",inputEnv:{SEMIO_TEST_LEVEL:"quick"},providers:{"typescript-check":"p/🟦️.ts"},rows:[row]}))).toBe(JSON.stringify(manifest));
 for(const extra of [{workspace:"/repo"},{scratch:".x"},{inputs:[1]},{class:"Bad"},{nodeModules:[{...row,extra:1}]}])expect(admit({...manifest,...extra}),JSON.stringify(extra)).toBe(false);
 const tree={stores:["/a/node_modules/.bun","/b/node_modules/.bun"],ids:["0/x@1","1/y@2"]},moved={stores:["/c/node_modules/.bun","/d/node_modules/.bun"],ids:["0/x@1","1/y@2"]};
 expect(api.bazelHermeticModulesLabelV1(tree)).toBe(api.bazelHermeticModulesLabelV1(moved));expect(api.bazelHermeticModulesLabelV1(tree)).toMatch(/^m_[a-f0-9]{16}$/u);expect(api.bazelHermeticModulesLabelV1({...tree,ids:["0/x@2","1/y@2"]})).not.toBe(api.bazelHermeticModulesLabelV1(tree));
 expect(()=>api.bazelHermeticModulesManifestV1({stores:[],ids:[]})).toThrow("Closed hermetic dependency manifest required");
 const hostRules=api.BAZEL_HERMETIC_MODULES_STARLARK;expect(hostRules).toContain('"no-sandbox": "1"');expect(hostRules).toContain('"no-remote-exec": "1"');expect(api.BAZEL_HERMETIC_STARLARK).toContain("--modules=");
 expect(api.BAZEL_HERMETIC_DEPENDENCIES).not.toMatch(/Bun\.|from"\.\.|\/Users\//u);
});
