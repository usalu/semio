import type { BuildGraphV1 } from "../🕸️graph/🟦️.ts";
import { checkScriptInvocation, scriptInvocationBudget, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand, captureOwnedCommandV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import cachePolicy from "../../⚡️caching/🔣️policy.json";
import ownership from "../../🗂️workspaces/🔣️policy.json";
import { watch } from "node:fs";

/** 🔎️ Reads portable options without crossing the task argument boundary. */
export function buildOptionV1(args:readonly string[],name:string,short?:string):string|undefined {
  for(let index=0;index<args.length&&args[index]!=="--";index++){
    const argument=args[index];
    if(argument===`--${name}`||short&&argument===`-${short}`)return args[index+1]?.startsWith("-")?undefined:args[index+1];
    if(argument.startsWith(`--${name}=`)||short&&argument.startsWith(`-${short}=`))return argument.slice(argument.indexOf("=")+1);
  }
}

/** 🧵️ Matches project selectors using literal text and portable wildcards. */
export function buildPatternV1(pattern:string,value:string):boolean { return new RegExp("^"+pattern.split("*").map(part=>part.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")).join(".*")+"$").test(value); }

/** 🎯️ Selects projects by identity, root or declared tag. */
export function selectBuildProjectsV1(graph:BuildGraphV1,patterns:readonly string[]=[],exclude:readonly string[]=[]):string[] {
  const match=(name:string,pattern:string)=>pattern.startsWith("tag:")?(graph.projects[name].tags as string[]??[]).some(tag=>buildPatternV1(pattern.slice(4),tag)):buildPatternV1(pattern,name)||buildPatternV1(pattern,graph.projects[name].root);
  const positive=patterns.filter(pattern=>!pattern.startsWith("!")),negative=[...exclude,...patterns.filter(pattern=>pattern.startsWith("!")).map(pattern=>pattern.slice(1))];
  return Object.keys(graph.projects).filter(name=>(!positive.length||positive.some(pattern=>match(name,pattern)))&&!negative.some(pattern=>match(name,pattern))).sort();
}

/** 🐚️ Quotes literal command arguments for the platform shell. */
function shellArgument(value:string):string { return process.platform==="win32"?'"'+value.replaceAll('"','""')+'"':"'"+value.replaceAll("'","'\\''")+"'"; }

/** 🧮️ Resolves portable execution selections and forwards literal task arguments. */
export async function selectBuildTasksV1(root:string,graph:BuildGraphV1,args:readonly string[],invocation:ScriptInvocation):Promise<{tasks:string[];args:string[];parallel?:number;skipCache:boolean;skipRemoteCache:boolean;excludeDependencies:boolean;bail:boolean;verbose:boolean;outputStyle?:string}> {
  checkScriptInvocation(invocation);
  const boundary=args.indexOf("--"),head=args.slice(0,boundary<0?undefined:boundary),forwarded=boundary<0?[]:args.slice(boundary+1),operation=head[0];
  const configuration=buildOptionV1(head,"configuration","c"),projectOptions=(buildOptionV1(head,"projects","p")??"").split(",").filter(Boolean),excluded=(buildOptionV1(head,"exclude")??"").split(",").filter(Boolean);
  let projects=selectBuildProjectsV1(graph,projectOptions,excluded),tasks:string[]=[];
  if(operation==="run"){
    const parts=head[1]?.split(":")??[];
    if(parts.length<2||parts.length>3||parts.some(part=>!part))throw Error("Expected repo run project:target[:configuration]");
    const project=graph.projects[parts[0]];
    if(!project||!project.targets[parts[1]])throw Error(`Unknown build task: ${head[1]}`);
    if(configuration&&parts[2]&&parts[2]!==configuration)throw Error("Conflicting task configurations");
    const mode=parts[2]??configuration??(process.env.SEMIO_BUILD_MODE&&project.targets[parts[1]].configurations?.[process.env.SEMIO_BUILD_MODE]?process.env.SEMIO_BUILD_MODE:undefined);
    tasks=[parts.slice(0,2).join(":")+(mode?":"+mode:"")];
  }else if(operation==="exec"){
    if(!forwarded.length)throw Error("Expected repo exec --projects=<projects> -- <command>");
    for(const project of projects){
      graph.projects[project].targets["repo-exec"]={execution:{kind:"command",command:forwarded.map(shellArgument).join(" "),cwd:graph.projects[project].root,forwardAllArgs:false},cache:false,outputs:[]};
      tasks.push(project+":repo-exec");
    }
  }else if(operation==="run-many"||operation==="affected"){
    if(operation==="affected"){
      let files=(buildOptionV1(head,"files")??"").split(",").filter(Boolean);
      if(!files.length&&!head.includes("--all")){
        const control=scriptCompilerAdmissionControlV1(invocation,"build:affected-files",30_000),base=buildOptionV1(head,"base")??"HEAD",revision=buildOptionV1(head,"head"),uncommitted=head.includes("--uncommitted"),untracked=head.includes("--untracked");
        const commands:Promise<string>[]=[];
        if(!untracked||uncommitted)commands.push(captureOwnedCommandV1("git",["diff","--name-only","-z",uncommitted?"HEAD":base,...(revision&&!uncommitted?[revision]:[])],root,{control}));
        if(untracked||!uncommitted&&!revision)commands.push(captureOwnedCommandV1("git",["ls-files","--others","--exclude-standard","-z"],root,{control}));
        files=[...new Set((await Promise.all(commands)).flatMap(output=>output.split("\0").filter(Boolean)))];
      }
      const owners=Object.keys(graph.projects).sort((a,b)=>graph.projects[b].root.length-graph.projects[a].root.length),affected=new Set(files.flatMap(file=>{const name=owners.find(name=>graph.projects[name].root==="."||file.startsWith(graph.projects[name].root+"/"));return name?[name]:[];}));
      let changed=true;while(changed){changed=false;for(const edge of graph.dependencies)if(affected.has(edge.target)&&!affected.has(edge.source)){affected.add(edge.source);changed=true;}}
      if(!head.includes("--all"))projects=projects.filter(project=>affected.has(project));
    }
    const targetOptions=(buildOptionV1(head,"targets","t")??buildOptionV1(head,"target")??"").split(",").filter(Boolean);
    if(!targetOptions.length)throw Error(`Expected repo ${operation} --targets=<targets>`);
    for(const project of projects)for(const target of Object.keys(graph.projects[project].targets).sort())if(targetOptions.some(pattern=>buildPatternV1(pattern,target))){const mode=configuration??(process.env.SEMIO_BUILD_MODE&&graph.projects[project].targets[target].configurations?.[process.env.SEMIO_BUILD_MODE]?process.env.SEMIO_BUILD_MODE:undefined);tasks.push(project+":"+target+(mode?":"+mode:""));}
  }else throw Error(`Unknown repository build command: ${operation??""}`);
  const parallelValue=buildOptionV1(head,"parallel")??process.env.SEMIO_BUILD_PARALLEL,parallel=parallelValue===undefined?undefined:Number(parallelValue);
  if(parallel!==undefined&&(!Number.isSafeInteger(parallel)||parallel<1))throw Error("Build parallelism must be a positive integer");
  const enabled=(names:string[])=>head.some(argument=>names.some(name=>argument===name||argument===name+"=true"));
  return {bail:!head.includes("--bail=false"),verbose:enabled(["--verbose"])||process.env.SEMIO_BUILD_VERBOSE==="true",outputStyle:buildOptionV1(head,"output")??buildOptionV1(head,"output-style"),tasks,args:operation==="exec"?[]:[...forwarded],...(parallel?{parallel}:{}),skipCache:enabled(["--skip-cache"]),skipRemoteCache:enabled(["--skip-remote-cache"]),excludeDependencies:enabled(["--exclude-dependencies","--exclude-task-dependencies"])};
}

/** 👁️ Watches source events with bounded reruns and original cancellation. */
export async function watchBuildInvocationV1(root:string,graph:BuildGraphV1,args:readonly string[],invocation:ScriptInvocation,options:{initial?:boolean}={}):Promise<void> {
  const boundary=args.indexOf("--"),command=boundary<0?[]:args.slice(boundary+1);
  if(!command.length)throw Error("Expected repo watch --projects=<projects> -- <command>");
  const names=selectBuildProjectsV1(graph,(buildOptionV1(args,"projects")??"").split(",").filter(Boolean)),changed=new Set<string>();
  let pending:ReturnType<typeof setTimeout>|undefined,running=Promise.resolve();
  const flush=()=>{const files=[...changed];changed.clear();if(!files.length)return;running=running.then(async()=>{checkScriptInvocation(invocation);await runOwnedCommand(command[0],command.slice(1),root,"build:watch",scriptInvocationBudget(invocation,0),{signal:invocation.control.signal,env:{...process.env,SEMIO_BUILD_CHANGED_FILES:JSON.stringify(files),SEMIO_BUILD_CHANGED_PROJECTS:JSON.stringify(names)}});}).catch(error=>{if(!invocation.control.signal.aborted)console.error(error instanceof Error?error.message:String(error));});};
  const watcher=watch(root,{recursive:true},(_event,filename)=>{
    if(!filename)return;const path=filename.toString().replaceAll("\\","/");
    if(path.split("/").some(part=>[...cachePolicy.generatedDirectories,".git",".🧬semio","temp"].includes(part)))return;
    if(path.split("/").some((part,index,parts)=>ownership.collections.names.includes(part)&&parts[index-1]!==ownership.collections.moduleMember))return;
    if(!names.some(name=>graph.projects[name].root==="."||path.startsWith(graph.projects[name].root+"/")))return;
    changed.add(path);if(pending)clearTimeout(pending);pending=setTimeout(flush,200);
  });
  if(options.initial){changed.add(".");flush();}
  try{await new Promise<void>((done,reject)=>{const stop=()=>done();invocation.control.signal.addEventListener("abort",stop,{once:true});watcher.once("error",reject);if(invocation.control.signal.aborted)done();});}
  finally{watcher.close();if(pending)clearTimeout(pending);await running;}
}

/** 🚦️ Refuses unsupported backend flags before publishing or executing any task. */
export function validateBuildTaskArgumentsV1(args:readonly string[]):void {
  const values=new Set(["projects","p","targets","target","t","exclude","parallel","configuration","c","output","output-style","base","head","files"]),booleans=new Set(["all","skip-cache","skip-remote-cache","exclude-dependencies","exclude-task-dependencies","dry-run","help","verbose","bail","uncommitted","untracked","include-global-files"]),optional=new Set(["graph"]);
  for(let index=args[0]==="run"?2:1;index<args.length&&args[index]!=="--";index++){
    const argument=args[index];
    if(!argument.startsWith("-"))throw Error(`Unexpected build argument before --: ${argument}`);
    const equal=argument.indexOf("="),name=(equal<0?argument:argument.slice(0,equal)).replace(/^--?/u,""),value=equal<0?undefined:argument.slice(equal+1);
    if(values.has(name)){if(value===undefined&&(!args[index+1]||args[index+1].startsWith("-")))throw Error(`Build option requires a value: ${argument}`);if(value===undefined)index++;}
    else if(booleans.has(name)){if(value!==undefined&&!["true","false"].includes(value))throw Error(`Build option requires a boolean: ${argument}`);}
    else if(optional.has(name)){if(value===undefined&&args[index+1]&&!args[index+1].startsWith("-"))index++;}
    else throw Error(`Unsupported build option: ${argument}`);
  }
  const output=buildOptionV1(args,"output")??buildOptionV1(args,"output-style");
  if(output&&!["stream","stream-without-prefixes","static"].includes(output))throw Error(`Unsupported build output style: ${output}`);
}
