/** 🔒️ Law: hermetic class members, run through the generated workspace and the real Bazel, execute the repository's own script routing with an original process authority issued inside the action, read only declared inputs (workspace links, declared sources, one content-addressed dependency tree), fail loudly on an undeclared read, an undeclared package, an undeclared write and a compile error, behave the same under the sandbox and under the local strategy, share their cache across checkout paths and route to a remote pool without running locally. */
import {expect,test} from "bun:test";
import {makeTestTreeWritableV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {cpSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {dirname,join,relative} from "node:path";
import schema from "../../🧬️schema/🔣️.json";
import {createNativeBuildCallerCapabilitiesV1} from "../../../../../📥️caller/🟦️.ts";
import {createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import * as execution from "../../🟦️.ts";

const binary=process.env.SEMIO_TEST_BAZEL_BINARY,contract=schema.definitions.Contract.const,host=execution.bazelExecutionPlatformV1(process.platform,process.arch),here=process.cwd();
const framework="🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript",routing=framework+"/🧭️routing/🟦️.ts",typecheckPort=framework+"/🏗️build/🟦️typescript/🟦️.ts";
const library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",hermetic=library+"/🏗️build/🏃️execution/🔒️hermetic";

type Spawn={mnemonic:string;runner:string;commandArgs:string[];environmentVariables:{name:string;value:string}[];inputs:{path:string;isTool?:boolean}[];platform?:{properties?:{name:string;value:string}[]};targetLabel:string};
const repair=(value:string)=>/[^\x00-\x7f]/u.test(value)&&[...value].every(char=>char.charCodeAt(0)<256)?Buffer.from(value,"latin1").toString("utf8"):value;
const readLogRaw=(path:string):Spawn[]=>{const text=readFileSync(path,"utf8"),rows:Spawn[]=[];let depth=0,start=-1,quoted=false,escaped=false;for(let index=0;index<text.length;index++){const char=text[index]!;if(quoted){if(escaped)escaped=false;else if(char==="\\")escaped=true;else if(char==='"')quoted=false;continue;}if(char==='"')quoted=true;else if(char==="{"){if(depth===0)start=index;depth++;}else if(char==="}"&&--depth===0)rows.push(JSON.parse(text.slice(start,index+1)));}return rows;};
const readLog=(path:string):Spawn[]=>readLogRaw(path).map(row=>({...row,inputs:row.inputs.map(input=>({...input,path:repair(input.path)}))}));

const closure=(entries:string[]):string[]=>{
 const found=new Set<string>(),scratch=mkdtempSync(join(tmpdir(),"hermetic-closure-"));
 try{for(const entry of entries){const metafile=join(scratch,"meta.json"),result=spawnSync(process.execPath,["build","--target=bun","--outdir",join(scratch,"out"),"--metafile="+metafile,join(here,entry)],{cwd:here,encoding:"utf8"});if(result.status!==0)throw new Error(result.stderr);for(const input of Object.keys(JSON.parse(readFileSync(metafile,"utf8")).inputs))found.add(relative(here,join(here,input)));}}finally{rmSync(scratch,{recursive:true,force:true});}
 const extra=new Set<string>(),siblings=(directory:string,depth:number)=>{for(const name of readdirSync(join(here,directory))){const path=directory+"/"+name,info=lstatSync(join(here,path));if(!info.isDirectory()||info.isSymbolicLink())continue;if(name==="🧬️schema")for(const file of walk(path))extra.add(file);else if(depth<3)siblings(path,depth+1);}},walk=(directory:string):string[]=>readdirSync(join(here,directory)).flatMap(name=>{const path=directory+"/"+name;return lstatSync(join(here,path)).isDirectory()?walk(path):[path];});
 for(const directory of new Set([...found].map(path=>dirname(path))))siblings(directory,1);
 return [...new Set([...found,...extra])].filter(path=>!path.startsWith("..")).sort();
};

const script=`import{existsSync,mkdirSync,readFileSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{receiveScriptProcessInvocation}from"../${routing}";
import{typecheckAdmittedProjectV1}from"../${typecheckPort}";
const verb=process.argv[2],root=process.env.SEMIO_WORKSPACE_ROOT!,here=import.meta.dir;
await receiveScriptProcessInvocation(process.env,async original=>{
 if(verb==="test"){const{answer}=await import("@fx/lib"),pad=(await import("leftpad-fx")).default;console.log(pad(readFileSync(join(here,"data/in.txt"),"utf8").trim(),6)+answer);return;}
 if(verb==="build"){const{answer}=await import("@fx/lib"),pad=(await import("leftpad-fx")).default;mkdirSync(join(here,"dist"),{recursive:true});writeFileSync(join(here,"dist/out.txt"),pad(readFileSync(join(here,"data/in.txt"),"utf8").trim(),6)+answer+"|"+process.env.SEMIO_TEST_LEVEL);return;}
 if(verb==="typecheck"||verb==="typecheck-bad"){await typecheckAdmittedProjectV1({workspace:root,cwd:here,project:verb==="typecheck"?"tsconfig.json":"tsconfig.bad.json",owner:"fixture",timeoutMs:600000,invocation:original,control:{}} as never);return;}
 if(verb==="check-leak"){console.log(readFileSync(join(root,"secret.txt"),"utf8"));return;}
 if(verb==="check-stray"){writeFileSync(join(here,"stray.txt"),"x");return;}
 if(verb==="check-import"){await import("hidden-fx");return;}
 if(verb==="check-authority"){writeFileSync(join(here,"authority.json"),JSON.stringify([process.env.SEMIO_HERMETIC,existsSync(join(root,"📋️project.json")),(original.capabilities as {tool:string}).tool,Object.keys(process.env).filter(name=>name.startsWith("SEMIO_")).sort()]));return;}
 throw new Error("unknown verb "+verb);
});
`;

const files=(typescript:string):Record<string,string>=>({
 "package.json":JSON.stringify({name:"checkout",workspaces:["lib","proj"]}),"secret.txt":"undeclared\n",
 "lib/package.json":JSON.stringify({name:"@fx/lib",type:"module",exports:{".":"./index.ts"}}),"lib/index.ts":"export const answer=42;\n",
 "proj/package.json":JSON.stringify({name:"@fx/proj",type:"module",dependencies:{"@fx/lib":"workspace:*","leftpad-fx":"1.0.0",typescript}}),
 "proj/tsconfig.json":JSON.stringify({compilerOptions:{strict:true,noEmit:true,types:[],lib:["es2022"],target:"es2022",module:"esnext",moduleResolution:"bundler"},include:["src/good.ts"]}),
 "proj/tsconfig.bad.json":JSON.stringify({extends:"./tsconfig.json",include:["src/bad.ts"]}),
 "proj/src/good.ts":"export const x:number=1;\n","proj/src/bad.ts":"export const y:number=\"not a number\";\n",
 "proj/data/in.txt":"alpha\n","proj/📜️script.ts":script,
});

test.skipIf(!binary)("law: hermetic class members read only declared inputs and a declared dependency tree, issue their own original authority and fail loudly on every undeclared access",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"hermetic-classes-")),checkout=join(directory,"checkout"),generation=join(directory,"cache/gen"),store=join(generation,"node_modules/.bun");
 const environment=(extra:Record<string,string>={})=>({...process.env,...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{}),...extra});
 try{
  const typescript=JSON.parse(readFileSync(join(here,"node_modules/typescript/package.json"),"utf8")).version as string;
  const put=(path:string,text:string)=>{mkdirSync(dirname(path),{recursive:true});writeFileSync(path,text);},link=(target:string,path:string)=>{mkdirSync(dirname(path),{recursive:true});symlinkSync(target,path);};
  const install=(root:string,gen:string)=>{
   const space=join(gen,"node_modules/.bun");
   put(join(space,"leftpad-fx@1.0.0/node_modules/leftpad-fx/package.json"),JSON.stringify({name:"leftpad-fx",version:"1.0.0",main:"index.js"}));put(join(space,"leftpad-fx@1.0.0/node_modules/leftpad-fx/index.js"),"module.exports=(text,width)=>text.padStart(width,\"-\");");
   put(join(space,"hidden-fx@1.0.0/node_modules/hidden-fx/package.json"),JSON.stringify({name:"hidden-fx",version:"1.0.0",main:"index.js"}));put(join(space,"hidden-fx@1.0.0/node_modules/hidden-fx/index.js"),"module.exports={};");
   cpSync(realpathSync(join(here,"node_modules/typescript")),join(space,`typescript@${typescript}/node_modules/typescript`),{recursive:true});
   for(const[name,id]of [["leftpad-fx","leftpad-fx@1.0.0"],["hidden-fx","hidden-fx@1.0.0"],["typescript",`typescript@${typescript}`]] as const)link(`.bun/${id}/node_modules/${name}`,join(gen,"node_modules",name));
   link(join(root,"lib"),join(gen,"node_modules/@fx/lib"));link(join(root,"proj"),join(gen,"node_modules/@fx/proj"));link(join(gen,"node_modules"),join(root,"node_modules"));
  };
  for(const[path,text]of Object.entries(files(typescript))){mkdirSync(join(checkout,dirname(path)),{recursive:true});writeFileSync(join(checkout,path),text);}
  for(const path of [...closure([hermetic+"/🚪️entry/🟦️.ts",hermetic+"/🔌️providers/🔷️typescript/🟦️.ts",routing,typecheckPort,library+"/🏃️process/🔌️providers/🟦️.ts"]),library+"/🏃️process/🔌️providers/🔣️.json"]){mkdirSync(join(checkout,dirname(path)),{recursive:true});cpSync(join(here,path),join(checkout,path));}
  install(checkout,generation);

  const inputs=["{projectRoot}/**/*","{workspaceRoot}/lib/**/*","{workspaceRoot}/package.json","{workspaceRoot}/🧰️framework/**/*",{env:"SEMIO_TEST_LEVEL"},{runtime:"bun --version"},{externalDependencies:["leftpad-fx","typescript"]}];
  const target=(verb:string,outputs:string[])=>({execution:{kind:"command",command:`bun ./📜️script.ts ${verb}`,cwd:"proj"},outputs,inputs,cache:true});
  const graph={version:1 as const,projects:{"@teaching/architecture-course":{name:"@teaching/architecture-course",root:"proj",targets:{test:target("test",[]),build:target("build",["{projectRoot}/dist"]),typecheck:target("typecheck",[]),"typecheck-bad":target("typecheck-bad",[]),"check-leak":target("check-leak",[]),"check-stray":target("check-stray",[]),"check-import":target("check-import",[]),"check-authority":target("check-authority",["{projectRoot}/authority.json"])}}},dependencies:[]} as any;
  process.env.SEMIO_TEST_LEVEL="quick";
  const generate=async(name:string,root:string,plan:execution.BazelExecutionPlanV1)=>{
   const api=await import("../../../📜️script.ts"),workspace=join(directory,name),request={tasks:Object.keys(graph.projects["@teaching/architecture-course"].targets).map(target=>"@teaching/architecture-course:"+target)};
   return withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:hermetic-classes-law",maximumElapsedMilliseconds:240000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["run","x"],"bazel",{}),Date.now()),async original=>{
    const planned=api.planBazelTasksV1(graph,request),implementation={version:1 as const,snapshot:join(workspace,"implementation.json"),runner:join(workspace,"📜️script.ts"),sha256:"0".repeat(64),loaderSha256:"1".repeat(64)};
    await api.generateBazelWorkspaceV1(root,graph,request,planned,workspace,original,createIssuedResourceControlV1(original,"bazel:hermetic-classes-law",0),implementation,()=>{},undefined,plan);
    return{workspace,planned,api};
   });
  };
  const{workspace,planned,api}=await generate("ws",checkout,{host:host.id,tools:{[host.id]:process.execPath}}),label=(verb:string)=>planned.tasks["@teaching/architecture-course:"+verb]!.label;
  const actions=api.bazelActionsV1(graph,planned,false,{hermetic:true});
  expect(Object.fromEntries([...actions.values()].map(action=>[action.id.split(":")[1],action.hermeticClass])),"every target is a member of the class its name selects").toEqual({test:"ts-test",build:"ts-build",typecheck:"ts-typecheck","typecheck-bad":"ts-typecheck","check-leak":"ts-check","check-stray":"ts-check","check-import":"ts-check","check-authority":"ts-check"});
  const flags=execution.bazelExecutionFlagsV1({host:{platform:process.platform,arch:process.arch},hermetic:true,jobs:2});
  const common=(base:string)=>[`--output_base=${join(directory,base)}`,`--install_base=${join(directory,"install")}`,"--nosystem_rc","--nohome_rc",`--bazelrc=${join(here,".bazelrc")}`,"--max_idle_secs=120"];
  const bazel=(base:string,args:string[],cwd=workspace)=>spawnSync(binary!,[...common(base),...args],{cwd,encoding:"utf8",timeout:540000,env:environment()});
  const build=(base:string,verbs:string[],extra:string[]=[],cwd=workspace,labels=label)=>bazel(base,["build","--lockfile_mode=off","--incompatible_strict_action_env","--incompatible_autoload_externally=",...flags.command,...extra,...verbs.map(verb=>"//:"+labels(verb))],cwd);
  const bin=(base:string)=>bazel(base,["info","bazel-bin","--lockfile_mode=off","--incompatible_autoload_externally=",...flags.command]).stdout.trim().split("\n").at(-1)!;
  const snapshotText=(outputs:string,verb:string,path:string)=>{const snapshot=JSON.parse(readFileSync(join(outputs,label(verb)+".snapshot/snapshot.json"),"utf8")),entry=snapshot.entries.find((row:any)=>row.path===path);return readFileSync(join(outputs,label(verb)+".snapshot/files",entry.file),"utf8");};

  const cache=join(directory,"disk"),log=join(directory,"a.json"),positive=["test","build","typecheck"],first=build("base-a",positive,[`--execution_log_json_file=${log}`,`--disk_cache=${cache}`]);
  expect(first.status,first.stderr+first.stdout).toBe(0);
  const spawns=readLog(log).filter(row=>row.mnemonic===contract.toolchain.mnemonic);
  expect(spawns.map(row=>row.targetLabel).sort()).toEqual(positive.map(verb=>"//:"+label(verb)).sort());
  const manifests=Object.fromEntries(positive.map(verb=>[verb,JSON.parse(readFileSync(join(workspace,label(verb)+".json"),"utf8"))]));
  for(const verb of positive){
   const spawn=spawns.find(row=>row.targetLabel==="//:"+label(verb))!,manifest=manifests[verb],tool=`tools/bun/${host.id}/${execution.bazelBunExecutableNameV1(host)}`,trees=[...new Set(spawn.inputs.map(input=>/^(bazel-out\/[^/]+\/bin\/m_[a-f0-9]{16})\//u.exec(input.path)?.[1]).filter((value):value is string=>value!==undefined))].map(prefix=>prefix+"/");
   expect(manifest.class,verb).toBe(verb==="typecheck"?"ts-typecheck":verb==="build"?"ts-build":"ts-test");expect(manifest.inputEnv).toEqual({SEMIO_TEST_LEVEL:"quick"});
   expect(trees.length===(verb==="typecheck"||verb==="test"||verb==="build"?1:0),`${verb} takes exactly one dependency tree`).toBe(true);
   expect(execution.bazelHermeticViolationsV1(spawn,{inputs:[...manifest.inputs.map((path:string)=>"s/"+path),"hermetic-runner.mjs",label(verb)+".json"],tool,properties:host.properties,trees}),`hermetic violations of ${verb}`).toEqual([]);
   expect(manifest.inputs).toContain("proj/data/in.txt");expect(manifest.inputs).not.toContain("secret.txt");expect(manifest.inputs).toContain(hermetic+"/🚪️entry/🟦️.ts");
  }
  expect(Object.keys(manifests.test.nodeModules.find((row:any)=>row.path==="node_modules").workspace)).toEqual(["@fx/lib","@fx/proj"]);
  const outputs=bin("base-a");
  expect(snapshotText(outputs,"build","proj/dist/out.txt")).toBe("-alpha42|quick");

  const second=build("base-b",positive,[`--disk_cache=${cache}`]);expect(second.status,second.stderr).toBe(0);expect(Number(/(\d+) disk cache hits?/u.exec(second.stderr)?.[1]??0),second.stderr).toBeGreaterThanOrEqual(3);

  const moved=join(directory,"elsewhere/checkout");mkdirSync(join(directory,"elsewhere"));cpSync(checkout,moved,{recursive:true,verbatimSymlinks:true});rmSync(join(moved,"node_modules"),{force:true});install(moved,join(directory,"elsewhere/gen"));
  const copy=await generate("ws-copy",moved,{host:host.id,tools:{[host.id]:process.execPath}}),shared=bazel("base-c",["build","--lockfile_mode=off","--incompatible_strict_action_env","--incompatible_autoload_externally=",...flags.command,`--disk_cache=${cache}`,`--execution_log_json_file=${join(directory,"c.json")}`,"//:"+copy.planned.tasks["@teaching/architecture-course:test"]!.label],copy.workspace);
  expect(shared.status,shared.stderr).toBe(0);expect(shared.stderr,"the same declared content at another checkout path reuses the cached action").toMatch(/\d+ disk cache hits?/u);
  bazel("base-c",["shutdown"],copy.workspace);

  const fails=(verb:string,pattern:RegExp,extra:string[]=[],base="base-a")=>{const result=build(base,[verb],extra);expect(result.status,verb+" must fail").not.toBe(0);expect(result.stderr+result.stdout,verb).toMatch(pattern);};
  fails("check-leak",/secret\.txt/u);
  fails("check-import",/Cannot find (module|package) 'hidden-fx'/u);
  fails("check-stray",/undeclared writes outside the declared outputs: proj\/stray\.txt/u);
  fails("typecheck-bad",/not assignable to type 'number'|TS2322/u);
  const authority=build("base-a",["check-authority"]);expect(authority.status,authority.stderr+authority.stdout).toBe(0);const issued=JSON.parse(snapshotText(bin("base-a"),"check-authority","proj/authority.json")) as [string,boolean,string,string[]];expect(issued.slice(0,3)).toEqual(["1",true,"bazel"]);expect(issued[3]).toEqual(expect.arrayContaining(["SEMIO_SCRIPT_PROCESS_INVOCATION","SEMIO_SCRIPT_PROVIDERS","SEMIO_WORKSPACE_ROOT"]));expect(issued[3].filter(name=>/REMOTE|TOKEN/u.test(name))).toEqual([]);

  const local=build("base-local",["test","typecheck"],["--strategy=RepoHermetic=local"]);expect(local.status,local.stderr+local.stdout).toBe(0);expect(local.stderr).toMatch(/\d+ local/u);
  fails("check-leak",/secret\.txt/u,["--strategy=RepoHermetic=local"],"base-local");
  fails("check-import",/Cannot find (module|package) 'hidden-fx'/u,["--strategy=RepoHermetic=local"],"base-local");

  const{workspace:remoteWorkspace,planned:remotePlanned}=await generate("ws-remote",checkout,{host:host.id,remote:{platform:"linux-x64",properties:{Arch:"amd64",OSFamily:"linux"}},tools:{[host.id]:process.execPath,"linux-x64":process.execPath}});
  const routed=execution.bazelExecutionFlagsV1({host:{platform:process.platform,arch:process.arch},hermetic:true,jobs:2,environment:{SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://127.0.0.1:9"}}),logR=join(directory,"r.json");
  const remote=spawnSync(binary!,[...common("base-r"),"build","--lockfile_mode=off","--incompatible_strict_action_env","--incompatible_autoload_externally=",...routed.command,`--execution_log_json_file=${logR}`,"//:"+remotePlanned.tasks["@teaching/architecture-course:test"]!.label],{cwd:remoteWorkspace,encoding:"utf8",timeout:420000,env:environment()});
  expect(remote.status,"a class member must route to the remote pool and never run locally").not.toBe(0);
  expect(remote.stderr).toMatch(/127\.0\.0\.1:9|UNAVAILABLE|Connection refused|remote/u);
  expect(existsSync(logR)?readLog(logR).filter(row=>row.mnemonic===contract.toolchain.mnemonic&&/sandbox|local|standalone/u.test(row.runner)):[]).toEqual([]);
 }finally{
  for(const base of ["base-a","base-b","base-c","base-r","base-local"])spawnSync(binary!,[`--output_base=${join(directory,base)}`,`--install_base=${join(directory,"install")}`,"--nosystem_rc","--nohome_rc","shutdown"],{timeout:120000});
  delete process.env.SEMIO_TEST_LEVEL;if(process.env.SEMIO_TEST_KEEP)console.log("kept "+directory);else{makeTestTreeWritableV1(directory);rmSync(directory,{recursive:true,force:true});}
 }
},1800000);
