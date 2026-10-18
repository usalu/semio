import type { BuildGraphV1 } from "../🕸️graph/🟦️.ts";
import { checkScriptInvocation, scriptInvocationBudget, scriptInvocationEnvironment, advanceScriptInvocation, createRequestedIssuedResourceControlV1, requireIssuedResourceControlInvocationV1, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand, captureOwnedCommandV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { scriptCompilerAdmissionControlV1, compilerAdmissionBudgetV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import cachePolicy from "../../⚡️caching/🔣️policy.json";
import ownership from "../../🗂️workspaces/🔣️policy.json";
import { watch } from "node:fs";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";

/** 🔎️ Reads portable options without crossing the task argument boundary. */
export function buildOptionV1(args:readonly string[],name:string,short?:string):string|undefined {
  return buildOptionsV1(args,short?[name,short]:[name]).at(-1);
}

/** 🧺️ Retains repeatable option occurrences in order without reading literal task arguments. */
export function buildOptionsV1(args:readonly string[],names:readonly string[]):string[] {
  const selected:string[]=[];
  for(let index=0;index<args.length&&args[index]!=="--";index++){
    const argument=args[index]!;
    if(names.some(name=>argument===`--${name}`||argument===`-${name}`)){const value=args[index+1];if(value!==undefined&&!value.startsWith("-")){selected.push(value);index++;}}
    else if(names.some(name=>argument.startsWith(`--${name}=`)||argument.startsWith(`-${name}=`)))selected.push(argument.slice(argument.indexOf("=")+1));
  }
  return selected;
}

/** 🛑️ Reads the last explicit boolean value without consuming literal callback arguments. */
export function buildBooleanOptionV1(args:readonly string[],name:string|readonly string[]):boolean|undefined {
  const names=typeof name==="string"?[name]:name;let selected:boolean|undefined;
  for(let index=0;index<args.length&&args[index]!=="--";index++){
    const argument=args[index]!;
    if(names.some(name=>argument===`--${name}`)){const next=args[index+1];selected=next!=="false";if(next==="true"||next==="false")index++;}
    else if(names.some(name=>argument.startsWith(`--${name}=`))){const value=argument.slice(argument.indexOf("=")+1);if(value!=="true"&&value!=="false")throw Error(`Build option requires a boolean: ${argument}`);selected=value==="true";}
  }
  return selected;
}

/** 🧵️ Matches project selectors using literal text and portable wildcards. */
export function buildPatternV1(pattern:string,value:string):boolean { if(/[?\[\]{}()\\]/u.test(pattern))throw Error("Unsupported build selector pattern: "+pattern);return new RegExp("^"+pattern.split("*").map(part=>part.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")).join(".*")+"$").test(value); }

/** 🎯️ Selects projects by identity, root or declared tag. */
export function selectBuildProjectsV1(graph:BuildGraphV1,patterns:readonly string[]=[],exclude:readonly string[]=[]):string[] {
  const match=(name:string,pattern:string)=>pattern.startsWith("tag:")?(graph.projects[name]!.tags as string[]??[]).some(tag=>buildPatternV1(pattern.slice(4),tag)):pattern.startsWith("name:")?buildPatternV1(pattern.slice(5),name):pattern.startsWith("directory:")?buildPatternV1(pattern.slice(10),graph.projects[name]!.root):buildPatternV1(pattern,name)||buildPatternV1(pattern,graph.projects[name]!.root);
  const names=Object.keys(graph.projects),selected=new Set<string>(),ordered=patterns.length?patterns:["*"];if(ordered[0]!.startsWith("!"))for(const name of names)selected.add(name);
  for(const value of ordered){const negative=value.startsWith("!"),pattern=negative?value.slice(1):value;buildPatternV1(pattern,"");for(const name of names)if(match(name,pattern)){if(negative)selected.delete(name);else selected.add(name);}}
  for(const pattern of exclude){buildPatternV1(pattern,"");for(const name of selected)if(match(name,pattern))selected.delete(name);}return [...selected].sort();
}

/** 🐚️ Quotes literal command arguments for the platform shell. */
function shellArgument(value:string):string { return process.platform==="win32"?'"'+value.replaceAll('"','""')+'"':"'"+value.replaceAll("'","'\\''")+"'"; }

/** 🧮️ Resolves portable execution selections and forwards literal task arguments. */
export async function selectBuildTasksV1(root:string,graph:BuildGraphV1,args:readonly string[],invocation:ScriptInvocation):Promise<{tasks:string[];args:string[];parallel?:number;skipCache:boolean;skipRemoteCache:boolean;excludeDependencies:boolean;bail:boolean;verbose:boolean;outputStyle?:"stream"|"stream-without-prefixes"|"static"}> {
  checkScriptInvocation(invocation);
  const boundary=args.indexOf("--"),head=args.slice(0,boundary<0?undefined:boundary),forwarded=boundary<0?[]:args.slice(boundary+1),operation=head[0];
  const enabled=(names:string[])=>buildBooleanOptionV1(head,names.map(name=>name.slice(2)))??false;
  const configuration=buildOptionV1(head,"configuration","c"),projectOptions=buildOptionsV1(head,["projects","p"]).flatMap(value=>value.split(",").filter(Boolean)),excluded=buildOptionsV1(head,["exclude"]).flatMap(value=>value.split(",").filter(Boolean));
  let projects=selectBuildProjectsV1(graph,projectOptions,excluded),tasks:string[]=[];
  if(operation==="run"){
    const parts=head[1]?.split(":")??[];
    if(parts.length<2||parts.length>3||parts.some(part=>!part))throw Error("Expected repo run project:target[:configuration]");
    const project=graph.projects[parts[0]!];
    if(!project||!project.targets[parts[1]!])throw Error(`Unknown build task: ${head[1]}`);
    if(configuration&&parts[2]&&parts[2]!==configuration)throw Error("Conflicting task configurations");
    const mode=parts[2]??configuration??(process.env.SEMIO_BUILD_MODE&&project.targets[parts[1]!]!.configurations?.[process.env.SEMIO_BUILD_MODE]?process.env.SEMIO_BUILD_MODE:undefined);
    tasks=[parts.slice(0,2).join(":")+(mode?":"+mode:"")];
  }else if(operation==="exec"){
    if(!forwarded.length)throw Error("Expected repo exec --projects=<projects> -- <command>");
    for(const project of projects){
      graph.projects[project]!.targets["repo-exec"]={execution:{kind:"command",program:forwarded[0]!,args:forwarded.slice(1),cwd:graph.projects[project]!.root,forwardAllArgs:true},cache:false,outputs:[]};
      tasks.push(project+":repo-exec");
    }
  }else if(operation==="run-many"||operation==="affected"){
    if(operation==="affected"){
      let files=buildOptionsV1(head,["files"]).flatMap(value=>value.split(",").filter(Boolean));
      if(!files.length&&!enabled(["--all"])){
        const control=scriptCompilerAdmissionControlV1(invocation,"build:affected-files",30_000),base=buildOptionV1(head,"base")??"HEAD",revision=buildOptionV1(head,"head"),uncommitted=enabled(["--uncommitted"]),untracked=enabled(["--untracked"]);
        const commands:Promise<string>[]=[];
        if(!untracked||uncommitted)commands.push(captureOwnedCommandV1("git",["-c","core.fsmonitor=false","diff","--name-only","-z",uncommitted?"HEAD":base,...(revision&&!uncommitted?[revision]:[])],root,{control}));
        if(untracked||!uncommitted&&!revision)commands.push(captureOwnedCommandV1("git",["-c","core.fsmonitor=false","ls-files","--others","--exclude-standard","-z"],root,{control}));
        files=[...new Set((await Promise.all(commands)).flatMap(output=>output.split("\0").filter(Boolean)))];
      }
      const owners=Object.keys(graph.projects).sort((a,b)=>graph.projects[b]!.root.length-graph.projects[a]!.root.length),affected=new Set(files.flatMap(file=>{const name=owners.find(name=>graph.projects[name]!.root==="."||file.startsWith(graph.projects[name]!.root+"/"));return name?[name]:[];}));
      let changed=true;while(changed){changed=false;for(const edge of graph.dependencies)if(affected.has(edge.target)&&!affected.has(edge.source)){affected.add(edge.source);changed=true;}}
      if(!enabled(["--all"]))projects=projects.filter(project=>affected.has(project));
    }
    const targetOptions=buildOptionsV1(head,["targets","target","t"]).flatMap(value=>value.split(",").filter(Boolean));
    for(const pattern of targetOptions)buildPatternV1(pattern,"");
    if(!targetOptions.length)throw Error(`Expected repo ${operation} --targets=<targets>`);
    for(const project of projects)for(const target of Object.keys(graph.projects[project]!.targets).sort())if(targetOptions.some(pattern=>buildPatternV1(pattern,target))){const mode=configuration??(process.env.SEMIO_BUILD_MODE&&graph.projects[project]!.targets[target]!.configurations?.[process.env.SEMIO_BUILD_MODE]?process.env.SEMIO_BUILD_MODE:undefined);tasks.push(project+":"+target+(mode?":"+mode:""));}
  }else throw Error(`Unknown repository build command: ${operation??""}`);
  const parallelValue=buildOptionV1(head,"parallel")??process.env.SEMIO_BUILD_PARALLEL,parallel=parallelValue===undefined?undefined:Number(parallelValue);
  if(parallel!==undefined&&(!Number.isSafeInteger(parallel)||parallel<1))throw Error("Build parallelism must be a positive integer");
  return {bail:buildBooleanOptionV1(head,"bail")??false,verbose:buildBooleanOptionV1(head,"verbose")??process.env.SEMIO_BUILD_VERBOSE==="true",outputStyle:(buildOptionV1(head,"output")??buildOptionV1(head,"output-style")) as "stream"|"stream-without-prefixes"|"static"|undefined,tasks,args:operation==="exec"?[]:[...forwarded],...(parallel?{parallel}:{}),skipCache:enabled(["--skip-cache"]),skipRemoteCache:enabled(["--skip-remote-cache"]),excludeDependencies:enabled(["--exclude-dependencies","--exclude-task-dependencies"])};
}

/** 👁️ Watches source events beneath the exact active issued phase through descendant drainage. */
export async function watchBuildInvocationV1(root:string,graph:BuildGraphV1,args:readonly string[],invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{initial?:boolean}={}):Promise<void> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);
  const controller=new AbortController(),control=createRequestedIssuedResourceControlV1(phaseControl,0,controller.signal),check=()=>checkResourceLeaseControl(control);
  const boundary=args.indexOf("--"),command=boundary<0?[]:args.slice(boundary+1);
  if(!command.length)throw Error("Expected repo watch --projects=<projects> -- <command>");
  const includeGlobal=buildBooleanOptionV1(args.slice(0,boundary),"include-global-files")===true;
  const {watchBuildChildArgumentsV1,buildWatchChangesEnvironmentV1,watchBuildChangeLimitsV1}=await import("./👁️watch/🟦️.ts");check();const child=watchBuildChildArgumentsV1(root,command);
  if(child){const {readOwnedBuildInvocationV1}=await import("../📥️caller/🟦️.ts");check();readOwnedBuildInvocationV1(root,invocation,child);check();}
  const names=selectBuildProjectsV1(graph,buildOptionsV1(args,["projects","p"]).flatMap(value=>value.split(",").filter(Boolean)),buildOptionsV1(args,["exclude"]).flatMap(value=>value.split(",").filter(Boolean))),changed=new Set<string>();
  let pending:ReturnType<typeof setTimeout>|undefined,deadline:ReturnType<typeof setTimeout>|undefined,running=Promise.resolve(),busy=false,stopped=false,failure:{error:unknown}|undefined,finish:(()=>void)|undefined;
  const stop=()=>{stopped=true;finish?.();},error=(error:unknown)=>{failure??={error};controller.abort(error);stop();},armDeadline=()=>{const remaining=check();if(remaining!==null)deadline=setTimeout(()=>{try{armDeadline();}catch(fault){error(fault);}},Math.max(1,Math.ceil(remaining)));},progress=async()=>{check();await advanceScriptInvocation(invocation,"build:watch","running");check();};
  const flush=()=>{if(stopped||busy||!changed.size)return;const files=[...changed];changed.clear();busy=true;running=Promise.resolve().then(async()=>{if(stopped)return;check();const changes={files,projects:names};
   if(child){const {runOwnedBuildChildInvocationV1}=await import("../📥️caller/📤️child/🟦️.ts");check();await runOwnedBuildChildInvocationV1(root,child,invocation,control,changes);check();}
   else{await progress();const milliseconds=compilerAdmissionBudgetV1(control);await runOwnedCommand(command[0]!,command.slice(1),root,"build:watch",scriptInvocationBudget(invocation,milliseconds),{signal:control.signal,env:scriptInvocationEnvironment(invocation,{...process.env,...buildWatchChangesEnvironmentV1(changes)}),onProgress:progress});check();}
  }).catch(error=>{failure??={error};controller.abort(failure.error);stop();}).finally(()=>{busy=false;if(!stopped&&changed.size)pending=setTimeout(flush,200);});};
  check();const watcher=watch(root,{recursive:true},(_event,filename)=>{
    if(stopped||!filename)return;try{check();}catch(fault){error(fault);return;}const path=filename.toString().replaceAll("\\","/");
    if(path.split("/").some(part=>[...cachePolicy.generatedDirectories,".git",".🧬semio","temp"].includes(part)))return;
    if(path.split("/").some((part,index,parts)=>ownership.collections.names.includes(part)&&parts[index-1]!==ownership.collections.moduleMember))return;
    const selected=names.some(name=>graph.projects[name]!.root==="."||path.startsWith(graph.projects[name]!.root+"/")),global=includeGlobal&&!Object.values(graph.projects).some(project=>project.root!=="."&&path.startsWith(project.root+"/"));
    if(!selected&&!global)return;
    if(path.length>watchBuildChangeLimitsV1.path||!changed.has(path)&&changed.size>=watchBuildChangeLimitsV1.files){error(Error("Bounded watch source event set required"));return;}
    changed.add(path);if(pending)clearTimeout(pending);pending=setTimeout(flush,200);
  });
  try{
   await new Promise<void>(done=>{finish=done;control.signal.addEventListener("abort",stop,{once:true});watcher.once("error",error);if(control.signal.aborted)stop();else{armDeadline();if(options.initial){changed.add(".");flush();}}});
  }finally{stopped=true;control.signal.removeEventListener("abort",stop);watcher.removeListener("error",error);try{watcher.close();}catch(fault){failure??={error:fault};controller.abort(fault);}if(pending)clearTimeout(pending);if(deadline)clearTimeout(deadline);await running;}
  if(failure)throw failure.error;if(!control.signal.aborted)check();
}

/** 🚦️ Refuses unsupported backend flags before publishing or executing any task. */
export function validateBuildTaskArgumentsV1(args:readonly string[]):void {
  if(args[0]==="watch"){const boundary=args.indexOf("--");if(boundary<0||!args[boundary+1]?.length)throw Error("Expected repo watch --projects=<projects> -- <command>");}
  const values=new Set(["projects","p","targets","target","t","exclude","parallel","configuration","c","output","output-style","base","head","files"]),booleans=new Set(["all","skip-cache","skip-remote-cache","exclude-dependencies","exclude-task-dependencies","dry-run","help","verbose","bail","uncommitted","untracked","include-global-files"]),optional=new Set(["graph"]);
  for(let index=args[0]==="run"?2:1;index<args.length&&args[index]!=="--";index++){
    const argument=args[index]!;
    if(!argument.startsWith("-"))throw Error(`Unexpected build argument before --: ${argument}`);
    const equal=argument.indexOf("="),name=(equal<0?argument:argument.slice(0,equal)).replace(/^--?/u,""),value=equal<0?undefined:argument.slice(equal+1);
    if(values.has(name)){if(value===undefined&&(!args[index+1]||args[index+1]!.startsWith("-")))throw Error(`Build option requires a value: ${argument}`);if(value===undefined)index++;}
    else if(booleans.has(name)){if(value!==undefined&&!["true","false"].includes(value))throw Error(`Build option requires a boolean: ${argument}`);if(name==="bail"&&value===undefined&&["true","false"].includes(args[index+1]??""))index++;}
    else if(optional.has(name)){if(value===undefined&&args[index+1]&&!args[index+1]!.startsWith("-"))index++;}
    else throw Error(`Unsupported build option: ${argument}`);
  }
  const output=buildOptionV1(args,"output")??buildOptionV1(args,"output-style");
  for(const name of ["projects","p","targets","target","t","exclude"])for(const pattern of buildOptionsV1(args,[name]).flatMap(value=>value.split(",").filter(Boolean)))buildPatternV1(pattern,"");
  if(output&&!["stream","stream-without-prefixes","static"].includes(output))throw Error(`Unsupported build output style: ${output}`);
}
