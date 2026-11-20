import {spawn} from "node:child_process";
import {createHash,randomBytes} from "node:crypto";
import {existsSync,realpathSync} from "node:fs";
import {lstat,mkdir,readFile,readdir,realpath,rename,rm,writeFile,copyFile,stat} from "node:fs/promises";
import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import {digestTaskViewInputsV1,taskViewInputsDigestV1,type TaskViewEnvironmentV1} from "../🟦️.ts";
import {runOnTaskRootV1} from "../🏃️executor/🟦️.ts";
import {machinePathV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🧱️ The authored policy of hermetic tasks: the layout of the state root, the environment a task may see, the Cargo closure rules and the bounds.
 * @see ./🧬️schema/🔣️.json */
export const HERMETIC_POLICY_V1=schema.definitions.Policy.const;
/** 🧱️ The texts of every refusal, English and German. */
export const HERMETIC_TEXTS_V1=schema.definitions.Texts.const;
export type HermeticPolicyV1=typeof HERMETIC_POLICY_V1;
type Code="unavailable"|"ambient-config"|"outside-workspace"|"undeclared-input"|"wrote-into-view"|"read-outside-view"|"path-leak"|"output-missing"|"output-unexpected"|"failed"|"aborted";
type TextKey=keyof typeof HERMETIC_TEXTS_V1["en"];

/** 🗄️ The machine-local half of a task: everything a build writes or caches lives below one state root and never in the replicated source. */
export type HermeticRootsV1=Readonly<{state:string;views:string;target:string;home:string;outputs:string}>;
/** 📤️ One declared output: a file the task leaves in its output directory (`staging`) or in the shared target directory (`target`), named by a relative path. */
export type HermeticOutputV1=Readonly<{name:string;from:"staging"|"target";path:string}>;
/** 🔌️ What one run of a task sees: the view as the only source root, the state roots, the directory its outputs go to, the start hash. */
export type HermeticContextV1=Readonly<{root:string;roots:HermeticRootsV1;outputDirectory:string;hash:string;environment:Readonly<Record<string,string>>}>;
export type HermeticCommandV1=Readonly<{file:string;args:readonly string[];environment?:Readonly<Record<string,string>>}>;
export type HermeticRequestV1=Readonly<{
 workspace:string;stateRoot:string;paths:readonly string[];outputs:readonly HermeticOutputV1[];
 command(context:HermeticContextV1):HermeticCommandV1;
 identity?(context:HermeticContextV1,environment:Readonly<Record<string,string>>):Promise<readonly string[]>;
 environment?:TaskViewEnvironmentV1;signal?:AbortSignal;cargo?:boolean;policy?:HermeticPolicyV1;
}>;
export type HermeticRowV1=Readonly<{name:string;digest:string;bytes:number;path:string}>;
/** 🧾️ A finished run: `hash` names the declared inputs, `key` adds what else decides the bytes (the toolchain), `directory` holds the outputs and `receipt.json`. */
export type HermeticOutcomeV1=Readonly<{kind:"done";hash:string;key:string;directory:string;outputs:readonly HermeticRowV1[];reused:boolean}>|Readonly<{kind:"refused";code:Code;paths:readonly string[];en:string;de:string}>;

const sha=(value:string|Uint8Array):string=>createHash("sha256").update(value).digest("hex");
const fill=(text:string,values:Readonly<Record<string,string|number>>):string=>text.replace(/\{(\w+)\}/gu,(_,name:string)=>String(values[name]??""));
const refuse=(code:Code,key:TextKey,values:Readonly<Record<string,string|number>>,paths:readonly string[]=[]):HermeticOutcomeV1=>({kind:"refused",code,paths,en:fill(HERMETIC_TEXTS_V1.en[key],values),de:fill(HERMETIC_TEXTS_V1.de[key],values)});
const list=(paths:readonly string[],most:number):string=>paths.slice(0,most).join(", ")+(paths.length>most?", ...":"");
const inside=(root:string,path:string):boolean=>{const way=relative(root,path);return way===""||!way.startsWith("..")&&!isAbsolute(way);};
const slash=(path:string):string=>path.split(sep).join("/");
const lstatOrNothing=async(path:string)=>lstat(path).catch(()=>undefined);

/** 🗺️ The roots of one state root: views, target directory, tool home and outputs, each its own directory.
 * @see ./🧬️schema/🔣️.json */
export const hermeticRootsV1=(stateRoot:string,policy:HermeticPolicyV1=HERMETIC_POLICY_V1):HermeticRootsV1=>{
 const state=resolve(stateRoot),layout=policy.layout;
 return Object.freeze({state,views:join(state,layout.views),target:join(state,layout.target),home:join(state,layout.home),outputs:join(state,layout.outputs)});
};

/** 🏠️ The state root of a workspace: `SEMIO_HERMETIC_STATE_ROOT` when set, else `<machine state>/f/hm`. Below a directory with a Cargo configuration a Cargo task is refused (`ambient-config`), so a checkout whose machine state lies inside it names another root.
 * @see ../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts */
export const hermeticStateRootV1=(workspace:string,environment:TaskViewEnvironmentV1=process.env,policy:HermeticPolicyV1=HERMETIC_POLICY_V1):string=>environment[policy.environment.stateRoot]??machinePathV1(workspace,".cache",...policy.layout.directory);

async function walk(workspace:string,directory:string,ignored:ReadonlySet<string>,rows:string[],limit:number):Promise<void>{
 for(const entry of (await readdir(join(workspace,directory),{withFileTypes:true})).sort((a,b)=>a.name<b.name?-1:1)){
  const path=directory===""?entry.name:directory+"/"+entry.name;
  if(entry.isDirectory()){if(!ignored.has(entry.name))await walk(workspace,path,ignored,rows,limit);}
  else rows.push(path);
  if(rows.length>limit)throw Error("Too many files in the closure");
 }
}

/** 📋️ The declared inputs of a Cargo task: the workspace manifest, lock, toolchain file and `.cargo` directory, and for every path package of `cargo metadata` its manifest, build script and source directories (a virtual workspace root contributes only its own files). A package outside the workspace is reported in `outside`: it cannot be part of a view.
 * @see https://doc.rust-lang.org/cargo/commands/cargo-metadata.html */
export async function cargoInputClosureV1(workspace:string,environment:TaskViewEnvironmentV1=process.env,policy:HermeticPolicyV1=HERMETIC_POLICY_V1):Promise<Readonly<{paths:readonly string[];outside:readonly string[]}>>{
 const cargo=policy.cargo,root=await realpath(workspace),ignored=new Set<string>(cargo.ignoredDirectories);
 const metadata=await capture("cargo",["metadata","--format-version","1","--offline","--locked"],root,cleanEnvironment({...process.env,...environment},{},policy));
 if(metadata.code!==0)throw Error("cargo metadata failed: "+metadata.stderr.trim());
 const packages=(JSON.parse(metadata.stdout) as {packages:{source:string|null;manifest_path:string}[]}).packages.filter(row=>row.source===null);
 const rows=new Set<string>(),outside:string[]=[];
 for(const name of cargo.workspaceFiles)if(await lstatOrNothing(join(root,name))!==undefined)rows.add(name);
 if(await lstatOrNothing(join(root,cargo.configDirectory))!==undefined){const found:string[]=[];await walk(root,cargo.configDirectory,ignored,found,policy.bounds.maximumFiles);for(const path of found)rows.add(path);}
 for(const row of packages){
  const manifest=await realpath(row.manifest_path),directory=dirname(manifest);
  if(!inside(root,directory)){outside.push(directory);continue;}
  const base=slash(relative(root,directory)),prefix=base===""?"":base+"/",found:string[]=[];
  rows.add(prefix+"Cargo.toml");
  if(base===""){
   for(const name of cargo.packageDirectories)if(await lstatOrNothing(join(root,name))!==undefined)await walk(root,name,ignored,found,policy.bounds.maximumFiles);
   for(const name of cargo.packageFiles)if(await lstatOrNothing(join(root,name))!==undefined)found.push(name);
  }else await walk(root,base,ignored,found,policy.bounds.maximumFiles);
  for(const path of found)rows.add(path);
 }
 return{paths:[...rows].sort(),outside:outside.sort()};
}

function cleanEnvironment(base:TaskViewEnvironmentV1,extra:Readonly<Record<string,string>>,policy:HermeticPolicyV1):Record<string,string>{
 const result:Record<string,string>={};
 for(const name of policy.environment.allow){const value=base[name];if(value!==undefined)result[name]=value;}
 return{...result,...policy.environment.fixed,...extra};
}

type Captured=Readonly<{code:number|null;stdout:string;stderr:string;aborted:boolean}>;
function capture(file:string,args:readonly string[],cwd:string,env:Record<string,string>,signal?:AbortSignal,limit:number=HERMETIC_POLICY_V1.bounds.maximumStderrBytes):Promise<Captured>{
 return new Promise(done=>{
  let stdout="",stderr="",aborted=false;
  const child=spawn(file,[...args],{cwd,env,stdio:["ignore","pipe","pipe"]});
  const stop=()=>{aborted=true;child.kill("SIGKILL");};
  if(signal?.aborted)stop();else signal?.addEventListener("abort",stop,{once:true});
  child.stdout.on("data",chunk=>{if(stdout.length<limit*64)stdout+=chunk;});
  child.stderr.on("data",chunk=>{if(stderr.length<limit)stderr+=chunk;});
  child.on("error",error=>done({code:127,stdout,stderr:stderr+String(error),aborted}));
  child.on("close",code=>{signal?.removeEventListener("abort",stop);done({code,stdout,stderr,aborted});});
 });
}

async function ambientConfig(start:string,policy:HermeticPolicyV1):Promise<string|undefined>{
 for(let directory=resolve(start);;directory=dirname(directory)){
  for(const name of policy.cargo.configNames){const file=join(directory,policy.cargo.configDirectory,name);if(existsSync(file))return file;}
  if(dirname(directory)===directory)return undefined;
 }
}

function namedFiles(stderr:string,root:string,workspace:string,declared:ReadonlySet<string>,limit:number):string[]{
 const directories=new Set<string>([""]);
 for(const path of declared)for(let at=dirname(path);at!=="."&&at!=="/";at=dirname(at))directories.add(at);
 const found=new Set<string>();
 for(const token of stderr.match(/[^\s"'`(),;<>]+/gu)??[]){
  if(!token.includes("/")&&!token.includes("."))continue;
  const bases=isAbsolute(token)?[""]:[...directories];
  for(const base of bases){
   const full=resolve(isAbsolute(token)?token:join(root,base,token));
   if(!inside(root,full)||full===root)continue;
   const way=slash(relative(root,full));
   if(declared.has(way))continue;
   if(existsSync(join(workspace,way))&&!existsSync(full))found.add(way);
  }
 }
 return[...found].sort().slice(0,limit);
}

async function sweepView(root:string,declared:ReadonlySet<string>,limit:number):Promise<string[]>{
 const directories=new Set<string>([""]),removed:string[]=[];
 for(const path of declared)for(let at=dirname(path);at!==".";at=dirname(at))directories.add(at);
 let seen=0;
 const go=async(directory:string):Promise<void>=>{
  for(const entry of await readdir(join(root,directory),{withFileTypes:true})){
   const path=directory===""?entry.name:directory+"/"+entry.name;
   if(++seen>limit)throw Error("Too many files in the view");
   if(entry.isDirectory()){
    if(directories.has(path))await go(path);
    else{removed.push(path+"/");await rm(join(root,path),{recursive:true,force:true});}
   }else if(!declared.has(path)){removed.push(path);await rm(join(root,path),{force:true});}
  }
 };
 await go("");
 return removed.sort();
}

async function listFiles(root:string,limit:number):Promise<string[]>{
 const rows:string[]=[];
 const go=async(directory:string):Promise<void>=>{
  for(const entry of await readdir(join(root,directory),{withFileTypes:true})){
   const path=directory===""?entry.name:directory+"/"+entry.name;
   if(entry.isDirectory())await go(path);else rows.push(path);
   if(rows.length>limit)throw Error("Too many files in the output");
  }
 };
 await go("");
 return rows.sort();
}

function dependencyPaths(text:string):string[]{
 return text.split("\n").filter(line=>line.endsWith(":")&&!line.includes(": ")).map(line=>line.slice(0,-1).replace(/\\ /gu," "));
}

async function readsOutside(target:string,root:string,allowed:readonly string[],since:number,policy:HermeticPolicyV1):Promise<string[]>{
 const found=new Set<string>();
 for(const profile of await readdir(target).catch(()=>[] as string[])){
  const directory=join(target,profile,policy.cargo.dependencyInfoDirectory);
  for(const name of await readdir(directory).catch(()=>[] as string[])){
   if(!name.endsWith(".d"))continue;
   const info=await stat(join(directory,name)).catch(()=>undefined);
   if(info===undefined||info.mtimeMs<since)continue;
   const text=await readFile(join(directory,name),"utf8");
   if(!text.includes(root)&&!text.split("\n").some(line=>line.endsWith(":")&&isAbsolute(line)))continue;
   for(const path of dependencyPaths(text)){
    const full=resolve(root,path);
    if(!inside(root,full)&&!allowed.some(base=>inside(base,full)))found.add(full);
   }
  }
 }
 return[...found].sort();
}

async function firstLeak(file:string,needles:readonly string[],limit:number):Promise<string|undefined>{
 const info=await stat(file);if(info.size>limit)return undefined;
 const bytes=await readFile(file);
 return needles.find(needle=>needle!==""&&bytes.includes(needle));
}

/** 🧱️ Runs a task hermetically: the declared inputs are pinned as a view (the only source root the task sees), the state (target directory, tool home, views, outputs) lives in a state root apart from the replicated source, the environment is an allow list, and afterwards the run is judged. Refused with a typed bilingual reason when it read an undeclared file, wrote into its view or beside its declared outputs, or when an output carries the view or state path (it would differ between machines). When no view can be made nothing runs live: the caller keeps its existing path. The result is the same on two machines: `hash` names the declared inputs, `key` adds the toolchain identity, `directory` is `<outputs>/<key>`.
 * @see ../🏃️executor/🟦️.ts */
export async function runHermeticV1(request:HermeticRequestV1):Promise<HermeticOutcomeV1>{
 const policy=request.policy??HERMETIC_POLICY_V1,roots=hermeticRootsV1(request.stateRoot,policy),layout=policy.layout,bounds=policy.bounds;
 const workspace=await realpath(request.workspace),declared=new Set(request.paths);
 await mkdir(roots.state,{recursive:true});
 for(const directory of [roots.views,roots.target,roots.home,roots.outputs])await mkdir(directory,{recursive:true});
 const state=await realpath(roots.state);
 if(request.cargo===true){
  const file=await ambientConfig(state,policy);
  if(file!==undefined)return refuse("ambient-config","ambientConfig",{directory:state,file},[file]);
 }
 let outcome:HermeticOutcomeV1|undefined;
 const result=await runOnTaskRootV1<void>({
  workspace,directory:roots.views,targetDirectory:roots.target,outputs:[roots.outputs],environment:{...request.environment,SEMIO_TASK_ROOT:"view"},
  paths:async()=>request.paths,signal:request.signal,
  async run(run){
   if(run.mode!=="view"){outcome=refuse("unavailable","unavailable",{reason:"no view"});return;}
   const staging=join(roots.outputs,layout.temporaryPrefix+randomBytes(6).toString("hex")),context:HermeticContextV1={root:run.root,roots,outputDirectory:staging,hash:run.hash!,environment:run.environment};
   await mkdir(staging,{recursive:true});
   try{outcome=await judge(request,context,policy,roots,workspace,declared,staging,bounds);}
   finally{await rm(staging,{recursive:true,force:true}).catch(()=>undefined);}
  },
 });
 if(outcome===undefined)return refuse("unavailable","unavailable",{reason:"not run"});
 if(result.fallback==="refused"||result.fallback==="pins-exhausted"||result.fallback==="reruns-exhausted")return refuse("unavailable","unavailable",{reason:result.fallback});
 return outcome;
}

async function judge(request:HermeticRequestV1,context:HermeticContextV1,policy:HermeticPolicyV1,roots:HermeticRootsV1,workspace:string,declared:ReadonlySet<string>,staging:string,bounds:HermeticPolicyV1["bounds"]):Promise<HermeticOutcomeV1>{
 const layout=policy.layout,since=Date.now()-policy.cargo.dependencyInfoSlackMilliseconds;
 await sweepView(context.root,declared,bounds.maximumFiles);
 const command=request.command(context),environment=cleanEnvironment(process.env,{...context.environment,...request.cargo===true?{CARGO_HOME:roots.home}:{},[policy.environment.outputDirectory]:staging,...command.environment},policy);
 if(request.environment?.RUSTUP_TOOLCHAIN!==undefined)environment.RUSTUP_TOOLCHAIN=request.environment.RUSTUP_TOOLCHAIN;
 const ran=await capture(command.file,command.args,context.root,environment,request.signal,bounds.maximumStderrBytes);
 if(ran.aborted||request.signal?.aborted===true)return refuse("aborted","aborted",{});
 if(ran.code!==0){
  const named=namedFiles(ran.stderr,context.root,workspace,declared,bounds.maximumNamedPaths);
  if(named.length>0)return refuse("undeclared-input","undeclaredInput",{paths:list(named,bounds.maximumNamedPaths)},named);
  return{...refuse("failed","failed",{code:String(ran.code)}),paths:[ran.stderr.trim().split("\n").slice(-5).join("\n")]};
 }
 const extra=await sweepView(context.root,declared,bounds.maximumFiles);
 if(extra.length>0)return refuse("wrote-into-view","wroteIntoView",{paths:list(extra,bounds.maximumNamedPaths)},extra);
 if(request.cargo===true){
  const allowed=[roots.home,...environment.RUSTUP_HOME===undefined?[]:[environment.RUSTUP_HOME]].map(path=>{try{return realpathSync(path);}catch{return path;}});
  const reads=await readsOutside(roots.target,context.root,allowed,since,policy);
  if(reads.length>0)return refuse("read-outside-view","readOutsideView",{paths:list(reads,bounds.maximumNamedPaths)},reads);
 }
 const produced=new Set(await listFiles(staging,bounds.maximumFiles)),wanted=new Set(request.outputs.filter(row=>row.from==="staging").map(row=>row.path));
 const unexpected=[...produced].filter(path=>!wanted.has(path));
 if(unexpected.length>0)return refuse("output-unexpected","outputUnexpected",{paths:list(unexpected,bounds.maximumNamedPaths)},unexpected);
 const place=(row:HermeticOutputV1):string=>row.from==="target"?join(roots.target,row.path):join(staging,row.path);
 for(const row of request.outputs)if(await lstatOrNothing(place(row))===undefined)return refuse("output-missing","outputMissing",{name:row.name},[row.name]);
 const needles=[context.root,roots.state,workspace],rows:HermeticRowV1[]=[];
 for(const row of request.outputs){
  const leak=await firstLeak(place(row),needles,bounds.maximumOutputBytes);
  if(leak!==undefined)return refuse("path-leak","pathLeak",{name:row.name,path:leak},[row.name]);
 }
 const identity=await(request.identity?.(context,environment)??Promise.resolve([])),key=sha(["hermetic-v1",context.hash,...identity].join("\0"));
 const final=join(roots.outputs,key.slice(0,layout.keyCharacters));
 for(const row of [...request.outputs].sort((a,b)=>a.name<b.name?-1:1)){
  const file=place(row),name=encodeURIComponent(row.name),bytes=await readFile(file);
  rows.push({name:row.name,digest:sha(bytes),bytes:bytes.length,path:join(final,name)});
  if(row.from==="target"){await mkdir(dirname(join(staging,name)),{recursive:true});await copyFile(file,join(staging,name));}
  else if(name!==row.path)await rename(file,join(staging,name));
 }
 await writeFile(join(staging,layout.receipt),JSON.stringify({version:1,hash:context.hash,key,outputs:rows.map(row=>({name:row.name,digest:row.digest,bytes:row.bytes}))},null,1));
 const reused=await lstatOrNothing(final)!==undefined;
 if(!reused){
  const next=join(roots.outputs,layout.temporaryPrefix+"p-"+randomBytes(6).toString("hex"));
  await rename(staging,next);
  try{await rename(next,final);}catch{await rm(next,{recursive:true,force:true});}
 }
 return{kind:"done",hash:context.hash,key,directory:final,outputs:rows,reused};
}

/** 🦀️ A Cargo build as a hermetic task: the closure of `cargoInputClosureV1` (or `paths`) is the view, `--locked --offline` and the pinned toolchain are fixed, the target directory and tool home are in the state root, the declared artifacts are paths below the target directory. The key adds `rustc -vV`.
 * @see https://doc.rust-lang.org/cargo/reference/unstable.html#profile-trim-paths-option */
export async function runHermeticCargoV1(request:Readonly<{workspace:string;stateRoot:string;artifacts:readonly string[];args?:readonly string[];paths?:readonly string[];toolchain?:string;signal?:AbortSignal;policy?:HermeticPolicyV1}>):Promise<HermeticOutcomeV1>{
 const policy=request.policy??HERMETIC_POLICY_V1,environment:TaskViewEnvironmentV1={...process.env,...request.toolchain===undefined?{}:{RUSTUP_TOOLCHAIN:request.toolchain}};
 let paths=request.paths;
 if(paths===undefined){
  const closure=await cargoInputClosureV1(request.workspace,environment,policy);
  if(closure.outside.length>0)return refuse("outside-workspace","outsideWorkspace",{path:closure.outside[0]!},closure.outside);
  paths=closure.paths;
 }
 return runHermeticV1({
  workspace:request.workspace,stateRoot:request.stateRoot,paths,cargo:true,environment,policy,...request.signal===undefined?{}:{signal:request.signal},
  outputs:request.artifacts.map(path=>({name:path,from:"target",path})),
  command:()=>({file:"cargo",args:[...request.args??["build"],...policy.cargo.arguments]}),
  identity:async(context,env)=>[(await capture("rustc",["-vV"],context.root,env)).stdout],
 });
}
