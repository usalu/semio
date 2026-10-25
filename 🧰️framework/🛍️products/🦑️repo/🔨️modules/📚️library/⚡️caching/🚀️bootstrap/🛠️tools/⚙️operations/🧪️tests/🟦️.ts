import {test,expect} from "bun:test";
import {spawnSync} from "node:child_process";
import {createRequire} from "node:module";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {createIssuedResourceControlV1,createRequestedIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {buildChangedFilesCommandsV1} from "../../../../../📦️packages/🟦️typescript/🏗️build/🧭️selection/🟦️.ts";
import {nxAffectedArgumentsV1,nxBuildCacheToolV1,nxCacheDirectoryV1,nxExecEnvironmentMappingV1,nxExecWrapperV1,nxOperationArgumentsV1} from "../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

const root=join(import.meta.dir,"../../../../../../../../../..").replace(/[\\/]+$/u,""),installed=createRequire(join(root,".nx/installation/package.json")),parse=installed("yargs-parser");

function git(directory:string,args:readonly string[]):string{
 const result=spawnSync("git",["-c","user.name=probe","-c","user.email=probe@example.invalid","-c","commit.gpgsign=false","-c","core.fsmonitor=false",...args],{cwd:directory,encoding:"utf8"});
 if(result.status!==0)throw Error("git "+args.join(" ")+": "+result.stderr);
 return result.stdout;
}

function write(directory:string,files:Readonly<Record<string,string>>):void{for(const [path,text] of Object.entries(files)){mkdirSync(dirname(join(directory,path)),{recursive:true});writeFileSync(join(directory,path),text);}}

/** 🧾️ The neutral changed-file rule, executed literally from its planned git queries. */
function neutralFiles(directory:string,args:readonly string[]):string[]{
 const head=args.slice(0,args.indexOf("--")<0?undefined:args.indexOf("--")),listed=head.find(argument=>argument.startsWith("--files="));
 if(listed)return listed.slice(8).split(",").sort();
 return [...new Set(buildChangedFilesCommandsV1(head).flatMap(command=>git(directory,command).split("\0").filter(Boolean)))].sort();
}

/** 🌳️ The installed Nx decides the same changed files from the translated arguments. */
function nxFiles(directory:string,nxArguments:readonly string[]):string[]{
 const parsed=parse(nxArguments.slice(1),{boolean:["uncommitted","untracked"],string:["base","head","files"]}),
  source="const u=require(process.env.NX_UTILS);const a=JSON.parse(process.env.NX_FLAGS);const r=u.splitArgsIntoNxArgsAndOverrides({...a,_:[]},'affected',{printWarnings:false},{});const f=u.parseFiles(r.nxArgs).files;process.stdout.write(JSON.stringify((typeof f==='string'?f.split(','):f).sort()));",
  result=spawnSync("node",["-e",source],{cwd:directory,encoding:"utf8",env:{...process.env,NX_UTILS:installed.resolve("nx/src/utils/command-line-utils"),NX_FLAGS:JSON.stringify(parsed),NX_WORKSPACE_ROOT_PATH:directory,NX_DAEMON:"false",NX_BASE:undefined,NX_HEAD:undefined} as NodeJS.ProcessEnv});
 if(result.status!==0)throw Error("nx oracle: "+result.stderr);
 return JSON.parse(result.stdout);
}

test("affected translation follows the language-agnostic corpus and cache operations map to Nx arguments",()=>{
 for(const row of fixture.affected)expect(nxOperationArgumentsV1(row.args),row.name).toEqual(row.nx);
 for(const row of fixture.cache)expect(nxOperationArgumentsV1(row.args),row.name).toEqual(row.nx??undefined);
 for(const args of fixture.affectedRefused)expect(()=>nxOperationArgumentsV1(args),args.join(" ")).toThrow("--projects");
 expect(nxAffectedArgumentsV1(["affected","--targets=build"])).toEqual(["affected","--targets=build","--base=HEAD"]);
});

test("inputs maps to the Nx command that prints the hash inputs of one task and refuses everything else",()=>{
 for(const row of fixture.inputs)expect(nxOperationArgumentsV1(row.args),row.name).toEqual(row.nx);
 for(const args of fixture.inputsRefused)expect(()=>nxOperationArgumentsV1(args),args.join(" ")).toThrow("Expected repo inputs");
 const source=readFileSync(installed.resolve("nx/src/command-line/show/command-object.js"),"utf8");
 expect(source).toContain("inputs [target]");expect(source).toContain("json");
});

test("dry-run and graph translate to the Nx graph on stdout or a file and exec exports the neutral names",()=>{
 const wrapper=nxExecWrapperV1();
 for(const row of fixture.planning)expect(nxOperationArgumentsV1(row.args),row.name).toEqual(row.nx);
 for(const row of fixture.exec)expect(nxOperationArgumentsV1(row.args),row.name).toEqual(row.nx.flatMap(argument=>argument==="<wrapper>"?wrapper:[argument]));
});

test("the exec wrapper maps the names Nx's own exec exports onto the neutral environment and forwards the command",()=>{
 const source=readFileSync(installed.resolve("nx/src/command-line/exec/exec.js"),"utf8");
 for(const nx of Object.keys(nxExecEnvironmentMappingV1))expect(source,nx).toContain(nx+":");
 expect(Object.values(nxExecEnvironmentMappingV1)).toEqual(["SEMIO_PROJECT_NAME","SEMIO_PROJECT_ROOT"]);
 const wrapper=nxExecWrapperV1(),shell=process.platform==="win32"?undefined:spawnSync("sh",["-c",[...wrapper,process.execPath,"-e","process.stdout.write([process.env.SEMIO_PROJECT_NAME,process.env.SEMIO_PROJECT_ROOT,process.argv.length].join(String.fromCharCode(124)))","two words"].map(argument=>`"${argument}"`).join(" ")],{encoding:"utf8",env:{...process.env,NX_PROJECT_NAME:"app",NX_PROJECT_ROOT_PATH:"packages/app"}});
 if(shell){expect(shell.stderr).toBe("");expect(shell.stdout).toBe("app|packages/app|2");}
 const failing=spawnSync(wrapper[0]!,[...wrapper.slice(1),process.execPath,"-e","process.exit(7)"],{encoding:"utf8"});
 expect(failing.status).toBe(7);
});

test("the Nx cache directory is the authored nx.json default inside the finite rust cache partition",()=>{
 expect(nxCacheDirectoryV1(root)).toBe(join(root,JSON.parse(readFileSync(join(root,"nx.json"),"utf8")).cacheDirectory));
});

test("every translated flag is declared by the installed Nx command grammar",()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"nx-grammar-")));
 try{
  writeFileSync(join(directory,"nx.json"),"{}\n");
  const source="const record=builder=>{const names=[],proxy=new Proxy(function(){},{get:(_target,property)=>property==='option'?name=>{names.push(name);return proxy;}:()=>proxy,apply:()=>proxy});builder(proxy);return names;};process.stdout.write(JSON.stringify({affected:record(require(process.env.NX_AFFECTED).yargsAffectedCommand.builder),reset:record(require(process.env.NX_RESET).yargsResetCommand.builder)}));",
   result=spawnSync("node",["-e",source],{cwd:directory,encoding:"utf8",env:{...process.env,NX_AFFECTED:installed.resolve("nx/src/command-line/affected/command-object"),NX_RESET:installed.resolve("nx/src/command-line/reset/command-object"),NX_WORKSPACE_ROOT_PATH:directory,NX_DAEMON:"false"} as NodeJS.ProcessEnv});
  if(result.status!==0)throw Error("nx grammar: "+result.stderr);
  const declared=JSON.parse(result.stdout) as Record<string,string[]>;
  for(const [command,names] of Object.entries(fixture.yargsOptions))for(const name of names)expect(declared[command],command+" "+name).toContain(name);
 }finally{rmSync(directory,{recursive:true,force:true});}
},60_000);

test("translated affected selection yields the same changed files in the installed Nx as the neutral git rule",()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"nx-affected-")));
 try{
  git(directory,["init","--quiet"]);write(directory,fixture.git.files);git(directory,["add","-A"]);git(directory,["commit","--quiet","-m","first"]);
  write(directory,fixture.git.second);git(directory,["add","-A"]);git(directory,["commit","--quiet","-m","second"]);
  write(directory,fixture.git.worktree);write(directory,fixture.git.untracked);
  for(const row of fixture.git.scenarios){
   const neutral=neutralFiles(directory,row.args),translated=nxOperationArgumentsV1(row.args)!,nx=nxFiles(directory,translated);
   expect(neutral,row.name).toEqual(row.files);
   expect(nx,row.name).toEqual(row.files);
  }
 }finally{rmSync(directory,{recursive:true,force:true});}
},60_000);

test("Nx cache status counts the finite cache tree, reports progress and stops when cancelled",async()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"nx-cache-operation-model",maximumElapsedMilliseconds:60000},{},Date.now()),async original=>{
 const workspace=realpathSync(mkdtempSync(join(tmpdir(),"nx-cache-")));
 try{
  const phase=createIssuedResourceControlV1(original,"test:nx-cache-status",0),stages:string[]=[],context={root:workspace,invocation:original,phaseControl:phase,progress:async(stage:string)=>{stages.push(stage);}},tool=nxBuildCacheToolV1(),directory=nxCacheDirectoryV1(workspace);
  expect(await tool.status(context)).toEqual({version:1,tool:"nx",available:false,directory,entries:0,bytes:0});
  for(const [path,size] of Object.entries(fixture.cacheTree.files)){mkdirSync(dirname(join(directory,path)),{recursive:true});writeFileSync(join(directory,path),Buffer.alloc(size as number));}
  expect(await tool.status(context)).toEqual({version:1,tool:"nx",available:true,directory,entries:fixture.cacheTree.entries,bytes:fixture.cacheTree.bytes});
  const controller=new AbortController(),cancelled=createRequestedIssuedResourceControlV1(phase,0,controller.signal);
  controller.abort(Error("cancelled by caller"));
  await expect(tool.status({...context,phaseControl:cancelled})).rejects.toThrow();
 }finally{rmSync(workspace,{recursive:true,force:true});}
}),60_000);
