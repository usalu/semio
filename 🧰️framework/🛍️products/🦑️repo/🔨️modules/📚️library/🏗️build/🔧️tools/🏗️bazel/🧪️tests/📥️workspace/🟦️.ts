/** 🗺️ Contract: the generated Bazel workspace holds private copies of the declared sources instead of links into the checkout (so the copies stay in the execroot, actions are cached and the checkout may change after planning), and a task's own Cargo directories are task data resolved against the workspace. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️schema.json";
import component from "../../🧬️schema/🔣️.json";
import {createNativeBuildCallerCapabilitiesV1,issueBuildTaskEnvironmentV1} from "../../../../📥️caller/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const api=await import("../../📜️script.ts"),python=testPythonV1(),workspace=join(tmpdir(),"bazel-workspace-fixture"),task=(env:Record<string,unknown>|null)=>({id:"fixture:build",project:"fixture",target:"build",root:"fixture",args:[],definition:{execution:{kind:"command",command:"repo ./📜️script.ts",...(env?{env}:{})}}}) as any;

test("Bazel workspace fixtures are closed and name the link the action policy declares",()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 expect(corpus.link.name).toBe(component.definitions.Actions.const.sources.link);
});

test("Bazel task Cargo directories are task data: they resolve against the workspace like an independent Python join and leave the base environment",()=>{
 const run=(program:string,input:unknown)=>Bun.spawnSync([python!,"-I","-c",program],{stdin:new TextEncoder().encode(JSON.stringify(input)),stdout:"pipe",stderr:"pipe"});
 const values=corpus.cargo.cases.flatMap(row=>Object.values(row.directories)),independent=python?(()=>{const result=run("import json,os,sys\nworkspace,values=json.loads(sys.stdin.read())\nprint(json.dumps([os.path.normpath(os.path.join(workspace,value)) for value in values]))",[workspace,values]);expect(result.exitCode,result.stderr.toString()).toBe(0);return JSON.parse(result.stdout.toString()) as string[];})():undefined;
 let position=0;
 for(const row of corpus.cargo.cases){
  const actual=api.bazelTaskCargoDirectoriesV1(workspace,task(row.env)),expected=Object.fromEntries(Object.entries(row.directories).map(([key,value])=>[key,independent?independent[position++]!:resolve(workspace,value)]));
  expect(actual,row.name).toEqual(expected);for(const key of Object.keys(actual))expect(corpus.cargo.keys,row.name).toContain(key);
  const base=api.bazelTaskBaseEnvironmentV1(workspace,task(row.env),{AMBIENT:"yes"});
  for(const key of corpus.cargo.keys)expect(Object.hasOwn(base,key),row.name+" "+key).toBe(false);
  for(const[key,value]of Object.entries(row.base))expect(base[key],row.name+" "+key).toBe(value);
  expect(base.AMBIENT).toBe("yes");expect(base.SEMIO_BUILD_TOOL).toBe("bazel");expect(base.SEMIO_WORKSPACE_ROOT).toBe(workspace);
 }
});

test("Bazel issues a task's own Cargo directories on top of the caller's authority, which refuses them when declared as ordinary task environment",async()=>{
 const row=corpus.cargo.cases[0]!,root=process.cwd(),capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","fixture:build"],"bazel",{});
 await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:cargo-directories",maximumElapsedMilliseconds:30000},capabilities,Date.now()),async original=>{
  const declared=task(row.env),environment=api.issueBazelTaskEnvironmentV1(root,original,declared,api.bazelTaskBaseEnvironmentV1(root,declared,process.env));
  for(const[key,value]of Object.entries(row.directories))expect(environment[key],key).toBe(resolve(root,value));
  expect(()=>issueBuildTaskEnvironmentV1(root,original,declared.definition.execution,declared.args,{...process.env,...row.env as unknown as Record<string,string>})).toThrow();
 });
},30000);

test("Bazel captures the declared sources into the workspace as private copies: no link into the checkout, no .bazelignore, a stale link replaced without writing through it",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"bazel-link-")),checkout=join(directory,"checkout"),space=join(directory,"workspace"),snapshot=await import("../../📸️snapshot/🟦️.ts"),views={alias:"a",real:corpus.link.name},capture=()=>snapshot.captureBazelSourceSnapshotV1({root:checkout,directory:space,files:["a.txt"],real:new Set(["a.txt"]),views,alias:()=>"x",check:()=>{},progress:async()=>{}});
 try{
  mkdirSync(checkout);mkdirSync(space);writeFileSync(join(checkout,"a.txt"),"a\n");utimesSync(join(checkout,"a.txt"),1e9,1e9);symlinkSync(checkout,join(space,corpus.link.name),"dir");
  await capture();
  expect(existsSync(join(space,".bazelignore"))).toBe(false);expect(lstatSync(join(space,corpus.link.name)).isSymbolicLink()).toBe(false);expect(readFileSync(join(space,corpus.link.name,"a.txt"),"utf8")).toBe("a\n");
  expect(readdirSync(checkout)).toEqual(["a.txt"]);expect(lstatSync(join(space,corpus.link.name,"a.txt")).ino).not.toBe(lstatSync(join(checkout,"a.txt")).ino);
  const before=lstatSync(join(space,corpus.link.name,"a.txt")).ino;await capture();expect(lstatSync(join(space,corpus.link.name,"a.txt")).ino).toBe(before);
  writeFileSync(join(checkout,"a.txt"),"b\n");expect(readFileSync(join(space,corpus.link.name,"a.txt"),"utf8")).toBe("a\n");
 }finally{rmSync(directory,{recursive:true,force:true});}
});

const binary=process.env.SEMIO_TEST_BAZEL_BINARY;

test.skipIf(!binary||process.platform==="win32")("real Bazel: the captured copies stay in the execroot so an action output reaches the disk cache, the checkout can change or vanish after planning, //... is a finite crawl",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"bazel-link-")),checkout=join(directory,"checkout"),space=join(directory,"workspace"),base=join(directory,"base"),install=join(directory,"install"),disk=join(directory,"disk"),snapshot=await import("../../📸️snapshot/🟦️.ts"),
  bazel=(args:string[])=>spawnSync(binary!,[`--output_base=${base}`,`--install_base=${install}`,"--nosystem_rc","--nohome_rc",`--bazelrc=${join(process.cwd(),".bazelrc")}`,"--max_idle_secs=60",...args],{cwd:space,encoding:"utf8",timeout:240000,env:{...process.env,...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{})}});
 try{
  mkdirSync(checkout);mkdirSync(space);writeFileSync(join(checkout,"a.txt"),"a\n");writeFileSync(join(checkout,"b.txt"),"b\n");symlinkSync(space,join(checkout,"workspace-loop"),"dir");
  await snapshot.captureBazelSourceSnapshotV1({root:checkout,directory:space,files:["a.txt","b.txt"],real:new Set(["a.txt"]),views:{alias:"a",real:corpus.link.name},alias:()=>"x",check:()=>{},progress:async()=>{}});
  writeFileSync(join(space,"MODULE.bazel"),'module(name="link")\n');writeFileSync(join(space,"BUILD.bazel"),`filegroup(name = "x", srcs = ["${corpus.link.name}/a.txt"])\ngenrule(name = "g", srcs = ["${corpus.link.name}/a.txt", "a/x/b.txt"], outs = ["g.txt"], cmd = "cat $(SRCS) > $@")\n`);
  const all=bazel(["query","--lockfile_mode=off","//:all"]);expect(all.status,all.stderr).toBe(0);expect(all.stdout).toContain("//:x");
  const label=bazel(["query","--lockfile_mode=off",`//:${corpus.link.name}/a.txt`]);expect(label.status,label.stderr).toBe(0);
  writeFileSync(join(checkout,"a.txt"),"changed after planning\n");rmSync(join(checkout,"b.txt"));
  const build=()=>bazel(["build","--lockfile_mode=off","--spawn_strategy=local","--guard_against_concurrent_changes",`--disk_cache=${disk}`,"//:g"]);
  const cold=build();expect(cold.status,cold.stderr).toBe(0);expect(cold.stderr).not.toContain("Skipping uploading");
  const bin=bazel(["info","bazel-bin"]).stdout.trim().split("\n").at(-1)!;expect(readFileSync(join(bin,"g.txt"),"utf8")).toBe("a\nb\n");
  expect(bazel(["clean"]).status).toBe(0);
  const warm=build();expect(warm.status,warm.stderr).toBe(0);expect(warm.stderr).toContain("1 disk cache hit");
  const crawled=bazel(["query","--lockfile_mode=off","//..."]);expect(crawled.stderr).not.toMatch(/infinite symlink expansion/iu);
 }finally{spawnSync(binary!,[`--output_base=${base}`,`--install_base=${install}`,"--nosystem_rc","--nohome_rc","shutdown"],{cwd:space,timeout:120000});rmSync(directory,{recursive:true,force:true});}
},600000);
