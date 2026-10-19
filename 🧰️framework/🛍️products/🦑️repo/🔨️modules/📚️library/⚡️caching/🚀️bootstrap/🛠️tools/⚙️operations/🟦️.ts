import {lstat,readdir} from "node:fs/promises";
import {join} from "node:path";
import {checkResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {repositoryCachePartitionRootDataV1} from "../../../../📦️packages/🟦️typescript/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
import {nxBuildArgumentsV1} from "../../../../🏗️build/🧭️invocation/🟦️.ts";
import type {BuildCacheContextV1,BuildCacheStatusV1,BuildCacheToolV1} from "../../../../🏗️build/⚙️operations/🟦️.ts";

/** 🧬️ Aligns Nx `affected` with the portable changed-file rule: tracked changes against HEAD plus untracked files unless a base, head, file list or one change class is chosen. */
export function nxAffectedArgumentsV1(translated:readonly string[]):string[] {
  const boundary=translated.indexOf("--"),head=translated.slice(0,boundary<0?undefined:boundary),tail=boundary<0?[]:translated.slice(boundary);
  if(head.some(argument=>/^(?:--projects|-p)(?:=|$)/u.test(argument)))throw Error("Nx affected cannot be narrowed with --projects; select the projects with run-many or drop --projects");
  const names=["all","base","head","files","uncommitted","untracked"],valued=new Set(["base","head","files"]),state:Record<string,string|boolean>={},kept:string[]=[];
  for(let index=0;index<head.length;index++){
    const argument=head[index]!,separator=argument.indexOf("="),name=(separator<0?argument:argument.slice(0,separator)).replace(/^--/u,"");
    if(!argument.startsWith("--")||!names.includes(name)){kept.push(argument);continue;}
    if(valued.has(name)){const value=separator<0?head[++index]:argument.slice(separator+1);if(value!==undefined)state[name]=value;}
    else state[name]=separator<0||argument.slice(separator+1)!=="false";
  }
  if(state.all===true)return ["run-many",...kept.slice(1),...tail];
  const uncommitted=state.uncommitted===true,untracked=state.untracked===true;
  const selection=state.files!==undefined?[`--files=${state.files}`]:uncommitted&&untracked?["--base=HEAD"]:uncommitted?["--uncommitted"]:untracked?["--untracked"]:[`--base=${state.base??"HEAD"}`,...(state.head!==undefined?[`--head=${state.head}`]:[])];
  return [...kept,...selection,...tail];
}

/** 🪞️ Names what Nx's own `exec` exports to each command and the neutral names the repository promises instead. */
export const nxExecEnvironmentMappingV1=Object.freeze({NX_PROJECT_NAME:"SEMIO_PROJECT_NAME",NX_PROJECT_ROOT_PATH:"SEMIO_PROJECT_ROOT"} as const);

/** 🎁️ Wraps an `exec` command in a Bun process that exports the neutral names from Nx's; Nx double-quotes every argument, so the program text avoids shell metacharacters. */
export function nxExecWrapperV1():string[]{
 const names=Object.entries(nxExecEnvironmentMappingV1).map(([nx,neutral])=>`${neutral}:process.env.${nx}`).join(","),
  program=`process.exit(require('node:child_process').spawnSync(process.argv[1],process.argv.slice(2),{stdio:'inherit',shell:process.platform==='win32',env:{...process.env,${names}}}).status??1)`;
 if(/["$`\\%!]/u.test(program))throw Error("Nx exec wrapper must stay free of shell metacharacters");
 return [process.execPath,"-e",program];
}

/** 🧭️ Translates one neutral operation into the Nx arguments it launches; `undefined` means no Nx process is needed. */
export function nxOperationArgumentsV1(args:readonly string[]):string[]|undefined{
 if(args[0]==="cache")return args[1]==="clear"?["reset","--onlyCache"]:undefined;
 const translated=nxBuildArgumentsV1(args),boundary=translated.indexOf("--");
 if(args[0]==="exec"&&boundary>=0&&boundary<translated.length-1)return [...translated.slice(0,boundary+1),...nxExecWrapperV1(),...translated.slice(boundary+1)];
 return args[0]==="affected"?nxAffectedArgumentsV1(translated):translated;
}

/** 📍️ Names the directory Nx stores task results in: the artifacts folder of the finite rust cache partition. */
export function nxCacheDirectoryV1(root:string):string{return join(repositoryCachePartitionRootDataV1(root,"rust"),"nx/artifacts");}

/** 📏️ Counts files and bytes beneath the Nx cache without following links, publishing progress and stopping when cancelled. */
async function measureNxCacheV1(context:BuildCacheContextV1):Promise<BuildCacheStatusV1>{
 const directory=nxCacheDirectoryV1(context.root),pending=[directory];let entries=0,bytes=0,visited=0,available=false;
 while(pending.length){
  checkResourceLeaseControl(context.phaseControl);
  const current=pending.pop()!;let rows;
  try{rows=await readdir(current,{withFileTypes:true});}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}
  if(current===directory)available=true;
  for(const row of rows){
   const path=join(current,row.name);
   if(row.isDirectory())pending.push(path);
   else{entries++;if(row.isFile())bytes+=(await lstat(path)).size;}
  }
  if(++visited%64===0)await context.progress("measuring");
 }
 checkResourceLeaseControl(context.phaseControl);
 return Object.freeze({version:1,tool:"nx",available,directory,entries,bytes});
}

/** 🗄️ Reads the Nx cache by walking its directory and clears it through Nx itself inside the bounded Nx namespace. */
export function nxBuildCacheToolV1():BuildCacheToolV1{
 return Object.freeze<BuildCacheToolV1>({
  tool:"nx",
  status:measureNxCacheV1,
  clear:async({root,invocation,phaseControl}:BuildCacheContextV1)=>{const {runOwnedNxInvocationV1}=await import("../../../../📦️packages/🟦️typescript/💾️storage/⚡️caching/🚀️bootstrap/📜️script/🟦️.ts");await runOwnedNxInvocationV1(root,nxOperationArgumentsV1(["cache","clear"])!,invocation,phaseControl);}
 });
}
