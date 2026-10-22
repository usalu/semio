/** 📐️ Law: with the real Bazel, hermetic actions of a deep emoji checkout read only aliased inputs and work in the declared short private root, the output base is reached through the short per-checkout output root, outputs are byte-identical to what the repository layout produced, a second checkout hits the disk cache, and every path the build created beneath the output base stays within the worst-case budget the path law computed for the target. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {cpSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {dirname,join,relative} from "node:path";
import schema from "../../../🏃️execution/🧬️schema/🔣️.json";
import {createNativeBuildCallerCapabilitiesV1} from "../../../../../📥️caller/🟦️.ts";
import {createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import * as execution from "../../../🏃️execution/🟦️.ts";
import {BAZEL_PATH_PLAN_V1,bazelHermeticRootV1,bazelOutputRootV1,bazelPathLawV1,linkBazelOutputRootV1} from "../../🟦️.ts";

const binary=process.env.SEMIO_TEST_BAZEL_BINARY,baseline=process.env.SEMIO_TEST_BASELINE==="1",contract=schema.definitions.Contract.const,host=execution.bazelExecutionPlatformV1(process.platform,process.arch),here=process.cwd(),sha=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex"),units=(text:string)=>text.length;
const adapter=await import("../../../📜️script.ts");
type Spawn={mnemonic:string;runner:string;commandArgs:string[];environmentVariables:{name:string;value:string}[];inputs:{path:string;isTool?:boolean}[];platform?:{properties?:{name:string;value:string}[]};targetLabel:string};
const repair=(value:string)=>/[^\x00-\x7f]/u.test(value)&&[...value].every(char=>char.charCodeAt(0)<256)?Buffer.from(value,"latin1").toString("utf8"):value;
const readLog=(path:string):Spawn[]=>{const text=readFileSync(path,"utf8"),rows:Spawn[]=[];let depth=0,start=-1,quoted=false,escaped=false;for(let index=0;index<text.length;index++){const char=text[index]!;if(quoted){if(escaped)escaped=false;else if(char==="\\")escaped=true;else if(char==='"')quoted=false;continue;}if(char==='"')quoted=true;else if(char==="{"){if(depth===0)start=index;depth++;}else if(char==="}"&&--depth===0)rows.push(JSON.parse(text.slice(start,index+1)));}return rows.map(row=>({...row,inputs:row.inputs.map(input=>({...input,path:repair(input.path)}))}));};

const deep="deep/"+Array.from({length:8},(_value,index)=>"🏗️native-build-"+index+"-unit").join("/");
const files:Record<string,string>={
 "lib/src/a.txt":"alpha\n","lib/gen.ts":`import{mkdirSync,readFileSync,writeFileSync}from"node:fs";const base=process.env.SEMIO_WORKSPACE_ROOT+"/lib";mkdirSync(base+"/gen",{recursive:true});writeFileSync(base+"/gen/out.txt",readFileSync(base+"/src/a.txt","utf8").toUpperCase());`,
 ["app/"+deep+"/data.txt"]:"deep data\n",
 "app/build.ts":`import{mkdirSync,readFileSync,writeFileSync}from"node:fs";const base=process.env.SEMIO_WORKSPACE_ROOT;mkdirSync(base+"/app/dist",{recursive:true});writeFileSync(base+"/app/dist/result.txt",readFileSync(base+"/lib/gen/out.txt","utf8")+readFileSync(base+"/app/${deep}/data.txt","utf8"));writeFileSync(base+"/app/dist/where.txt",base);`,
 "neg/leak.ts":`import{readFileSync}from"node:fs";console.log(readFileSync(process.env.SEMIO_WORKSPACE_ROOT+"/lib/secret.txt","utf8"));`,
 "lib/secret.txt":"undeclared\n",
};
const hermetic={version:1,tools:["bun"]},target=(command:string,inputs:string[],outputs:string[],dependsOn:string[]=[])=>({execution:{kind:"command",command},outputs,inputs,cache:true,dependsOn,metadata:{semio:{hermetic}}});
const graph={version:1 as const,projects:{
 lib:{name:"lib",root:"lib",targets:{generate:target("bun {projectRoot}/gen.ts",["{projectRoot}/src/**/*","{projectRoot}/gen.ts"],["{projectRoot}/gen"])}},
 app:{name:"app",root:"app",targets:{build:target("bun {projectRoot}/build.ts",["{projectRoot}/**/*"],["{projectRoot}/dist"],["lib:generate"])}},
 neg:{name:"neg",root:"neg",targets:{leak:target("bun {projectRoot}/leak.ts",["{projectRoot}/leak.ts"],["{projectRoot}/out"])}}},dependencies:[]} as any;

type Parsed={status:number|null;stderr:string;stdout:string};
test.skipIf(!binary)("law: aliased hermetic inputs, the short private root and the short output root build byte-identical outputs, hit the disk cache from another checkout and stay within the computed path budget",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"pathlaw-")),policy={...BAZEL_PATH_PLAN_V1,outputRoot:{...BAZEL_PATH_PLAN_V1.outputRoot,darwin:{template:join(directory,"b/{checkout}"),checkoutDigits:8},linux:{template:join(directory,"b/{checkout}"),checkoutDigits:8}}};
 const environment=()=>({...process.env,...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{})});
 const roots:string[]=[];
 try{
  const checkoutOf=(name:string)=>{const checkout=join(directory,name,"checkout");for(const[path,text]of Object.entries(files)){mkdirSync(join(checkout,dirname(path)),{recursive:true});writeFileSync(join(checkout,path),text);}return checkout;};
  const generate=async(name:string,checkout:string)=>{
   const workspace=join(directory,name,"ws"),request={tasks:["app:build","neg:leak"]};
   return withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:paths-law",maximumElapsedMilliseconds:240000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["run","app:build"],"bazel",{}),Date.now()),async original=>{
    const planned=adapter.planBazelTasksV1(graph,request),implementation={version:1 as const,snapshot:join(workspace,"implementation.json"),runner:join(workspace,"📜️script.ts"),sha256:"0".repeat(64),loaderSha256:"1".repeat(64)};
    await adapter.generateBazelWorkspaceV1(checkout,graph,request,planned,workspace,original,createIssuedResourceControlV1(original,"bazel:paths-law",0),implementation,()=>{},undefined,{host:host.id,tools:{[host.id]:process.execPath}});
    return{workspace,planned};
   });
  };
  const flags=execution.bazelExecutionFlagsV1({host:{platform:process.platform,arch:process.arch},hermetic:true,jobs:2});
  const session=(name:string,checkout:string)=>{
   const real=join(directory,name,"server"),short=baseline?undefined:bazelOutputRootV1(process.platform,checkout,policy as never);mkdirSync(real,{recursive:true});
   if(short){linkBazelOutputRootV1(short.root,real);roots.push(short.root);}
   const base=short?short.base("s0"):join(real,"s0");
   return{base,real,startup:[...(short?[`--output_user_root=${short.root}`]:[]),`--output_base=${base}`,`--install_base=${join(directory,"install")}`,"--nosystem_rc","--nohome_rc",`--bazelrc=${join(here,".bazelrc")}`,"--max_idle_secs=120"]};
  };
  const bazel=(started:ReturnType<typeof session>,workspace:string,args:string[]):Parsed=>spawnSync(binary!,[...started.startup,...args],{cwd:workspace,encoding:"utf8",timeout:540000,env:environment()});
  const build=(started:ReturnType<typeof session>,workspace:string,labels:string[],extra:string[]=[])=>bazel(started,workspace,["build","--lockfile_mode=off","--incompatible_strict_action_env","--incompatible_autoload_externally=",...flags.command,...extra,...labels.map(label=>"//:"+label)]);

  const checkout=checkoutOf("one"),{workspace,planned}=await generate("one",checkout),label=(id:string)=>planned.tasks[id]!.label,one=session("one",checkout),cache=join(directory,"disk"),log=join(directory,"a.json");
  const first=build(one,workspace,[label("app:build")],[`--execution_log_json_file=${log}`,`--disk_cache=${cache}`]);expect(first.status,first.stderr+first.stdout).toBe(0);
  const bin=bazel(one,workspace,["info","bazel-bin","--lockfile_mode=off","--incompatible_autoload_externally=",...flags.command]).stdout.trim().split("\n").at(-1)!;
  const snapshot=JSON.parse(readFileSync(join(bin,label("app:build")+".snapshot/snapshot.json"),"utf8")),text=(path:string)=>readFileSync(join(bin,label("app:build")+".snapshot/files",snapshot.entries.find((entry:any)=>entry.path===path).file),"utf8");
  expect(snapshot.entries.map((entry:any)=>[entry.path,entry.mode.toString(8)])).toEqual([["app/dist","755"],["app/dist/result.txt","644"],["app/dist/where.txt","644"]]);
  expect(text("app/dist/result.txt")).toBe("ALPHA\ndeep data\n");
  const digests=snapshot.entries.filter((entry:any)=>entry.kind==="file"&&entry.path!=="app/dist/where.txt").map((entry:any)=>[entry.path,entry.sha256]);console.log("DIGESTS "+JSON.stringify(digests));
  expect(digests).toEqual([["app/dist/result.txt",sha("ALPHA\ndeep data\n")]]);

  const spawns=readLog(log).filter(row=>row.mnemonic===contract.toolchain.mnemonic),manifest=JSON.parse(readFileSync(join(workspace,label("app:build")+".json"),"utf8")),tool=`tools/bun/${host.id}/${execution.bazelBunExecutableNameV1(host)}`;
  expect(spawns.length).toBe(2);
  for(const spawn of spawns){
   const id=Object.keys(planned.tasks).find(key=>"//:"+planned.tasks[key]!.label===spawn.targetLabel)!,own=JSON.parse(readFileSync(join(workspace,planned.tasks[id]!.label+".json"),"utf8"));
   expect(execution.bazelHermeticViolationsV1(spawn,{inputs:[...own.inputs.map((path:string)=>baseline?"s/"+path:adapter.bazelSourceAliasLabelV1(path)),"hermetic-runner.mjs",planned.tasks[id]!.label+".json"],tool,properties:host.properties}),`hermetic violations of ${id}`).toEqual([]);
   if(!baseline)expect(spawn.inputs.some(input=>input.path.startsWith("s/")),"no hermetic input is named by its repository layout").toBe(false);
  }
  if(!baseline){
   expect(manifest.scratch).toEqual(BAZEL_PATH_PLAN_V1.hermeticRoot);expect(Object.values(manifest.aliases).every(alias=>/^a\/[a-f0-9]{10}$/u.test(alias as string))).toBe(true);
   const where=text("app/dist/where.txt");expect(where).toMatch(/^\/(?:private\/)?tmp\/[0-9a-f]{12}\/r$/u);expect(where.length).toBe(bazelHermeticRootV1("darwin",undefined,policy as never)!.units-1);
   const outputRoot=bazelOutputRootV1(process.platform,checkout,policy as never)!;expect(existsSync(outputRoot.root)).toBe(true);expect(lstatSync(outputRoot.root).isSymbolicLink()).toBe(true);
   const sources=["lib/gen.ts","lib/src/a.txt","app/"+deep+"/data.txt","app/build.ts"],law=bazelPathLawV1({platform:process.platform as "darwin",arch:process.arch as "arm64",longPaths:"unprobed",outputBase:one.base,checkout},[{id:"app:build",kind:"hermetic",sources}],adapter.bazelSourceAliasLabelV1,policy as never);
   let witness="";const longest=(root:string):number=>{let best=0;const visit=(path:string)=>{for(const entry of readdirSync(path,{withFileTypes:true})){if(entry.name==="external")continue;const full=join(path,entry.name),size=units(relative(join(one.real,"s0"),full));if(size>best&&size+units(one.base)+1>=best){witness=relative(join(one.real,"s0"),full);}best=Math.max(best,size);if(entry.isDirectory()&&!entry.isSymbolicLink())visit(full);}};visit(root);return best;};
   const created=units(one.base)+1+Math.max(longest(join(one.real,"s0/execroot")),existsSync(join(one.real,"s0/sandbox"))?longest(join(one.real,"s0/sandbox")):0),families=law.rows[0]!.families;
   const alias=families.find(family=>family.name==="alias-source")!;
   expect(units(one.base)+1+units("execroot/_main/")+Math.max(...sources.map(path=>units(adapter.bazelSourceAliasLabelV1(path))))).toBe(alias.units);
   expect(created,"every path created beneath the short output base fits the worst case of the families the law computed: "+witness).toBeLessThanOrEqual(Math.max(...families.map(family=>family.units)));
   expect(law.fits).toBe(true);
  }

  const second=await (async()=>{const moved=checkoutOf("two"),gen=await generate("two",moved),started=session("two",moved);return{started,gen,moved};})();
  const shared=bazel(second.started,second.gen.workspace,["build","--lockfile_mode=off","--incompatible_strict_action_env","--incompatible_autoload_externally=",...flags.command,`--disk_cache=${cache}`,"//:"+second.gen.planned.tasks["app:build"]!.label]);
  expect(shared.status,shared.stderr).toBe(0);expect(shared.stderr,"the same declared content in another checkout with its own output root reuses the cached actions").toMatch(/\d+ disk cache hits?/u);expect(shared.stderr).not.toMatch(/\d+ (darwin-sandbox|linux-sandbox|processwrapper-sandbox)/u);
  bazel(second.started,second.gen.workspace,["shutdown"]);

  const leak=build(one,workspace,[label("neg:leak")]);expect(leak.status,"an undeclared read must fail").not.toBe(0);expect(leak.stderr+leak.stdout).toMatch(/secret\.txt/u);
  const local=build(session("local",checkout),workspace,[label("app:build")],["--strategy=RepoHermetic=local"]);expect(local.status,local.stderr).toBe(0);
 }finally{
  for(const name of ["one","two","local"]){const real=join(directory,name,"server"),base=baseline?join(real,"s0"):undefined;if(base)spawnSync(binary!,[`--output_base=${base}`,`--install_base=${join(directory,"install")}`,"--nosystem_rc","--nohome_rc","shutdown"],{timeout:120000});}
  for(const root of roots)spawnSync(binary!,[`--output_user_root=${root}`,`--output_base=${root}/s0`,`--install_base=${join(directory,"install")}`,"--nosystem_rc","--nohome_rc","shutdown"],{timeout:120000});
  spawnSync("chmod",["-R","u+w",directory]);rmSync(directory,{recursive:true,force:true});
 }
},1800000);
