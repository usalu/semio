import {type CargoDependencyNetworkV1} from "./../../📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import {readBuildMode,type BuildMode} from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import {compilerAdmissionBudgetV1,type CompilerAdmissionControl} from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {runOwnedCommand} from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import installationSchema from "./🧬️schema/🔣️.json";
export type DashboardExecutionOptions={readonly mode:BuildMode;readonly network:CargoDependencyNetworkV1;readonly control:CompilerAdmissionControl};
/** 🚀️ Starts an installed immutable Rust dashboard without graph discovery or executable hashing. */
import { existsSync, readFileSync, writeFileSync, renameSync, mkdirSync } from "node:fs";
import { readdir, stat } from "node:fs/promises";
import { createHash } from "node:crypto";
import { dirname, basename, join, relative, resolve, isAbsolute } from "node:path";
import { spawn } from "node:child_process";

const cache=(root:string)=>join(root,".🧬semio/🦑️repo/⚡️cache");
const manifest=(root:string,mode:BuildMode)=>join(cache(root),"🎛️dashboard",readBuildMode(mode),"installed.json");
const slashes=(path:string)=>path.replaceAll("\\","/");

/** 📦️ The crate directory of the dashboard executable; its sources live in the sibling modules of `🎛️dashboard`. */
export const dashboardPackageRoot=resolve(import.meta.dir,"../📦️packages/🦀️rust");

/** 🧾️ The source trees an installation was built from and the cheap digest they had at that moment. */
export type DashboardSources={readonly roots:readonly string[];readonly digest:string};

const skippedDirectories=new Set(["target","node_modules","dist","build",".git","🗑️generated","🧪️tests","🧫️fixtures","coverage","__pycache__","storybook-static","temp"]);
const skippedFiles=/\.(?:tsx?|mts|mjs|js|py|log|md|feature|tsv|tsbuildinfo)$/;
const workspaceFiles=["Cargo.toml","Cargo.lock","rust-toolchain.toml",".cargo/config.toml"];

/** 🔎️ Whether an installation record exists; a present but invalid record is rebuilt by the next launch. */
export function dashboardInstalled(root:string,mode:BuildMode):boolean { return existsSync(manifest(root,mode)); }

/** 📍️ Resolves one small installation record; launch never reads the executable contents. */
export function installedDashboard(root:string,mode:BuildMode):string {
  const file=manifest(root,mode);
  if(!existsSync(file))throw new Error("Dashboard is not installed. Run bun run dashboard:install once, then start bun run dashboard.");
  const value=JSON.parse(readFileSync(file,"utf8")),name=process.platform==="win32"?"semio.exe":"semio";
  if(validateJsonSchemaSubset(installationSchema,value).length||value.mode!==mode||value.platform!==process.platform||!new RegExp(`^\\.🧬semio/🦑️repo/⚡️cache/tools/dashboard-cli/${mode}/[a-f0-9]{64}/${name.replace(".","\\.")}$`).test(value.path))throw new Error("Invalid dashboard installation record; run bun run dashboard:install.");
  const executable=resolve(root,value.path);
  if(!existsSync(executable))throw new Error("Installed dashboard executable is missing; run bun run dashboard:install.");
  return executable;
}

/** 🧭️ The source roots of one crate: its module directory (`<module>/📦️packages/🦀️rust` owns its siblings) or itself. */
function crateRoot(directory:string):string {
  return basename(directory)==="🦀️rust"&&basename(dirname(directory))==="📦️packages"?dirname(dirname(directory)):directory;
}

/** 🔗️ Path dependencies of one manifest; `workspace = true` entries resolve through the workspace table. */
function pathDependencies(manifestFile:string,workspaceTable:ReadonlyMap<string,string>,workspace:string):string[] {
  const text=readFileSync(manifestFile,"utf8"),found:string[]=[];
  let section="";
  for(const raw of text.split(/\r?\n/)){
    const line=raw.trim(),heading=/^\[\[?([^\]]+)\]\]?$/.exec(line);
    if(heading){section=heading[1]!;continue;}
    if(!/(?:^|\.)(?:build-)?dependencies(?:\.|$)/.test(section)||/dev-dependencies/.test(section))continue;
    const path=/(?:^|[\s{,])path\s*=\s*"([^"]+)"/.exec(line);
    if(path){found.push(resolve(dirname(manifestFile),path[1]!));continue;}
    const shared=/^([A-Za-z0-9_-]+)\s*(?:=\s*\{[^}]*\bworkspace\s*=\s*true|\.workspace\s*=\s*true)/.exec(line),table=shared?workspaceTable.get(shared[1]!):undefined;
    if(table)found.push(resolve(workspace,table));
  }
  return found;
}

/** 🗂️ The `[workspace.dependencies]` path entries of the root manifest. */
function workspaceDependencyTable(workspace:string):Map<string,string> {
  const table=new Map<string,string>(),file=join(workspace,"Cargo.toml");
  if(!existsSync(file))return table;
  let inside=false;
  for(const raw of readFileSync(file,"utf8").split(/\r?\n/)){
    const line=raw.trim();
    if(line.startsWith("[")){inside=line==="[workspace.dependencies]";continue;}
    const entry=inside?/^([A-Za-z0-9_-]+)\s*=\s*\{[^}]*\bpath\s*=\s*"([^"]+)"/.exec(line):null;
    if(entry)table.set(entry[1]!,entry[2]!);
  }
  return table;
}

/** 🌳️ The closure of source roots the dashboard crate compiles from, as workspace-relative paths. */
function sourceRoots(packageRoot:string,workspace:string):string[] {
  const table=workspaceDependencyTable(workspace),seen=new Set<string>(),pending=[resolve(packageRoot)],roots=new Set<string>();
  while(pending.length){
    const directory=pending.pop()!;
    if(seen.has(directory))continue;
    seen.add(directory);
    const file=join(directory,"Cargo.toml");
    if(!existsSync(file))continue;
    roots.add(crateRoot(directory));
    pending.push(...pathDependencies(file,table,workspace));
  }
  const ordered=[...roots].sort((a,b)=>a.length-b.length).filter((root,index,all)=>!all.slice(0,index).some(parent=>root===parent||root.startsWith(parent+"/")||root.startsWith(parent+"\\")));
  return ordered.map(root=>slashes(relative(workspace,root))).sort();
}

/** 🏃️ Every relevant source file below one root with its size and modification time, read concurrently. */
async function collect(directory:string,workspace:string,entries:string[]):Promise<void> {
  let children;
  try { children=await readdir(directory,{withFileTypes:true}); } catch { return; }
  await Promise.all(children.map(async child=>{
    const path=join(directory,child.name);
    if(child.isDirectory()){if(!skippedDirectories.has(child.name))await collect(path,workspace,entries);return;}
    if(skippedFiles.test(child.name))return;
    try { const info=await stat(path);entries.push(`${slashes(relative(workspace,path))}\0${info.size}\0${Math.trunc(info.mtimeMs)}`); } catch {}
  }));
}

/** 🔏️ The digest of the file names, sizes and modification times below the roots; no file is read. */
export async function digestDashboardSources(workspace:string,roots:readonly string[]):Promise<string> {
  const entries:string[]=[];
  await Promise.all([...roots.map(root=>collect(resolve(workspace,root),workspace,entries)),...workspaceFiles.map(async name=>{
    try { const info=await stat(join(workspace,name));entries.push(`${name}\0${info.size}\0${Math.trunc(info.mtimeMs)}`); } catch {}
  })]);
  entries.sort();
  return `${entries.length}:${createHash("sha1").update(entries.join("\n")).digest("hex")}`;
}

/** 📸️ Captures the source roots and their digest; take it before building so edits made during the build stay detectable. */
export async function captureDashboardSources(packageRoot:string,workspace:string):Promise<DashboardSources> {
  const roots=sourceRoots(packageRoot,workspace);
  return {roots,digest:await digestDashboardSources(workspace,roots)};
}

/** ⏳️ Why the installed executable no longer matches the sources, or nothing while it is current. */
export async function staleDashboard(root:string,mode:BuildMode):Promise<string|undefined> {
  if(!dashboardInstalled(root,mode))return "it is not installed";
  let record;
  try { record=JSON.parse(readFileSync(manifest(root,mode),"utf8")); } catch { return "the installation record is unreadable"; }
  const sources=record?.sources;
  if(validateJsonSchemaSubset(installationSchema,record).length||record.mode!==mode||!Array.isArray(sources?.roots)||typeof sources?.digest!=="string")return "the installation record does not name its sources";
  const current=await digestDashboardSources(root,sources.roots);
  if(current===sources.digest)return;
  const before=Number.parseInt(sources.digest),after=Number.parseInt(current);
  return `its sources changed since installation (${before===after?`${after} files`:`${before} -> ${after} files`})`;
}

/** 🧊️ Publishes immutable executable bytes and one atomic native installation record. */
export async function installDashboard(packageRoot:string,root:string,options:DashboardExecutionOptions,sources?:DashboardSources):Promise<string> {
  const mode=readBuildMode(options.mode),control=options.control,budget=compilerAdmissionBudgetV1(control),controller=new AbortController(),signal=AbortSignal.any([control.signal,controller.signal]);
  const expiry=budget?setTimeout(()=>controller.abort(Error("Original dashboard installation deadline exhausted")),budget):undefined;
  const {pinExecutableArtifact}=await import("../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/🟦️.ts");
  try {
    const recorded=sources??await captureDashboardSources(packageRoot,root);compilerAdmissionBudgetV1(control);signal.throwIfAborted();if(validateJsonSchemaSubset(installationSchema.properties.sources,recorded).length)throw Error("Dashboard installation source ownership refused");
    const name=process.platform==="win32"?"semio.exe":"semio";
    const executable=await pinExecutableArtifact(resolve(packageRoot,"dist/build",mode,name),join(cache(root),"tools/dashboard-cli",mode),`dashboard-installation:${mode}`,{signal,leaseDirectory:join(root,".cache/framework/leases"),onWait:async progress=>{compilerAdmissionBudgetV1(control);await control.publish(progress);await control.yieldContinuation();}});
    compilerAdmissionBudgetV1(control);signal.throwIfAborted();
    const path=slashes(relative(root,executable));
    if(isAbsolute(path)||path.startsWith("../"))throw new Error("Dashboard installation must stay inside the workspace");
    const target=manifest(root,mode),temporary=`${target}.${process.pid}.tmp`;
    mkdirSync(resolve(target,".."),{recursive:true});
    writeFileSync(temporary,JSON.stringify({version:2,platform:process.platform,mode,path,sources:recorded})+"\n");
    renameSync(temporary,target);return executable;
  } finally {if(expiry)clearTimeout(expiry);}
}

/** 🧭️ Recognizes native installed-tool launches while retaining the build runner for authoring tasks. */
export function dashboardInvocation(args:readonly string[]):string[]|undefined {
  if(args[0]!=="run")return;
  const prefix="@semio-tech/repo-dashboard-rs:",target=args[1]?.startsWith(prefix)?args[1].slice(prefix.length):undefined;
  if(!target||!["run","daemon","workflow","preferences"].includes(target))return;
  const rest=args.slice(2),separator=rest.indexOf("--"),options=separator<0?rest:rest.slice(0,separator);
  if(options.some(option=>/^--(?:graph|batch|parallel|exclude-dependencies)(?:=|$)/.test(option)))return;
  const forwarded=options.filter(option=>!/^--(?:output)=stream$/.test(option)&&!["--skip-cache"].includes(option));
  return [...(target==="run"?[]:[target]),...forwarded,...(separator<0?[]:rest.slice(separator+1))];
}

/** 🛟️ Whether these native arguments only observe or end what already runs, so an installation that is behind the sources may still answer them: a broken working tree must not stop a developer from stopping the daemon. */
export function mayRunStale(args:readonly string[]):boolean {
  const [verb,action]=args;
  return ["tasks","logs","stop","kill","open"].includes(verb??"")||(verb==="daemon"&&["stop","status"].includes(action??""));
}

/** 🔨️ Rebuilds and reinstalls through the package script with inherited output; Ctrl+C cancels the build. */
async function rebuildDashboard(reason:string,options:DashboardExecutionOptions):Promise<void> {
  console.log(`[dashboard] Rebuilding ${options.mode} because ${reason}`);
  await runOwnedCommand(process.execPath,[join(dashboardPackageRoot,"📜️script.ts"),"build"],dashboardPackageRoot,"dashboard:rebuild",compilerAdmissionBudgetV1(options.control),{env:{...process.env,SEMIO_BUILD_MODE:options.mode},signal:options.control.signal});
}

/** ▶️ Hands the terminal to the selected immutable binary under original cancellation and deadline. */
export async function launchDashboard(root:string,args:readonly string[],options:DashboardExecutionOptions):Promise<number> {
  const mode=readBuildMode(options.mode),control=options.control;compilerAdmissionBudgetV1(control);
  const reason=mayRunStale(args)?undefined:await staleDashboard(root,mode);compilerAdmissionBudgetV1(control);
  if(reason)await rebuildDashboard(reason,options);
  const executable=installedDashboard(root,mode),controller=new AbortController(),signal=AbortSignal.any([control.signal,controller.signal]),budget=compilerAdmissionBudgetV1(control),expiry=budget?setTimeout(()=>controller.abort(Error("Original dashboard launch deadline exhausted")),budget):undefined;
  try{return await new Promise<number>((accept,reject)=>{
    const child=spawn(executable,args,{cwd:root,env:{...process.env,SEMIO_BUILD_MODE:mode},signal,stdio:"inherit",windowsHide:true});
    child.once("error",reject);child.once("close",(code,closedSignal)=>accept(code??(closedSignal==="SIGINT"?130:1)));
  });}finally{if(expiry)clearTimeout(expiry);}
}
