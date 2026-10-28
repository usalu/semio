import {buildPreparationBudgetMs,progressDeadlinePolicyV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import {withRepositoryToolingStoreV1} from "../../../../📦️packages/🟦️typescript/💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import {withRepositoryPublishedCargoExecutableV1} from "../../../../📦️packages/🟦️typescript/🦀️cargo/⚡️caching/📦️artifacts/🏗️native-build/📥️published/🟦️.ts";
import {withResourceLeases,checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {scriptInvocationEnvironment,scriptResourceProcessEnvironmentV1,withRequestedIssuedResourceControlV1,withRenewableIssuedResourceControlV1,type ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {type CargoDependencyNetworkV1} from "../../../../📦️packages/🟦️typescript/🦀️cargo/📥️dependencies/📋️contract/🟦️.ts";
import {parseCargoBinarySourcesV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import {refuseBootstrapV1,sayBootstrapV1} from "../../📖️texts/🟦️.ts";
import {readBuildMode,type BuildMode} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import {compilerAdmissionBudgetV1,type CompilerAdmissionControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {runOwnedCommand,startOwnedCommandTimeoutV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import installationSchema from "./🧬️schema/🔣️.json" with { type: "json" };
import orchestrationSchema from "../../../../🏗️build/📥️caller/📤️orchestration/🧬️schema/🔣️.json" with { type: "json" };
import executionSchema from "../../../../🏗️build/🏃️execution/🧬️schema/🔣️.json" with { type: "json" };
export type DashboardExecutionOptions={readonly mode:BuildMode;readonly network:CargoDependencyNetworkV1;readonly control:CompilerAdmissionControl;readonly invocation:ScriptInvocation;readonly workspace:string};
import { existsSync, readFileSync, writeFileSync, renameSync, mkdirSync, lstatSync, statSync, readdirSync, copyFileSync, chmodSync, rmSync, constants as fsConstants } from "node:fs";
import { readdir, stat, open } from "node:fs/promises";
import { createHash } from "node:crypto";
import { dirname, basename, join, relative, resolve, isAbsolute } from "node:path";
import { spawn } from "node:child_process";
import { planNxToolingRetentionV1 } from "../../🛠️tools/🧹️retention/🟦️.ts";
import { terminateOwnedChildTree } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { runtimeParseTomlV1 } from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";

const cache=(root:string)=>join(root,".🧬semio/🦑️repo/⚡️cache");
const manifest=(root:string,mode:BuildMode)=>join(cache(root),"🎛️dashboard",readBuildMode(mode),"installed.json");
const slashes=(path:string)=>path.replaceAll("\\","/");
const executableName=()=>process.platform==="win32"?"semio.exe":"semio";
const artifactProtocol=installationSchema.definitions.ArtifactProtocol.const;
const artifactDirectory=(root:string,mode:BuildMode)=>join(cache(root),"🎛️dashboard",readBuildMode(mode),artifactProtocol.directory);
const readRecord=(file:string):any=>{try{return JSON.parse(readFileSync(file,"utf8"));}catch{return undefined;}};
const writeRecord=(file:string,value:unknown)=>{const temporary=file+"."+process.pid+".tmp";mkdirSync(dirname(file),{recursive:true});writeFileSync(temporary,JSON.stringify(value)+"\n");renameSync(temporary,file);};
const sha256=(file:string)=>createHash("sha256").update(readFileSync(file)).digest("hex");

/** 📦️ The crate directory of the dashboard executable; its sources live in the sibling modules of `🎛️dashboard`. */
export const dashboardPackageRoot=resolve(import.meta.dirname,installationSchema.definitions.PackageOwner.const.installationRelative);

/** 🪵️ The source trees an installation was built from and the cheap digest they had at that moment. */
export type DashboardSources={readonly roots:readonly string[];readonly manifests:readonly string[];readonly digest:string};

/** 🗃️ What an executable was compiled from: the files of its own Cargo dep-info and every Cargo manifest, lock, toolchain file and the package declaration (Cargo's whole freshness basis), relative to the installation root, when that build started, and the sha256 each file held then (empty when it was already touched after the start). */
export type DashboardCompiledSources={readonly builtAtMs:number;readonly sources:readonly string[];readonly digests:readonly string[]};

/** 🧾️ Checks current host, mode and immutable path against the closed installation contract. */
function validDashboardRecord(value:unknown,mode:BuildMode):boolean{
 if(validateJsonSchemaSubset(installationSchema,value).length)return false;
 const record=value as {platform:string;mode:string;path:string;digest:string},name=process.platform==="win32"?"semio.exe":"semio";
 return record.platform===process.platform&&record.mode===mode&&record.path===`.cache/b/t/m/dashboard-cli/${mode}/${record.digest}/${name}`;
}

/** 🏺️ Checks current host and mode and that the record names the executable inside this mode's artifact directory (relative to the installation root). */
function validDashboardArtifactV1(root:string,value:unknown,mode:BuildMode):boolean{
 if(validateJsonSchemaSubset(installationSchema,value).length)return false;
 const record=value as {platform:string;mode:string;path:string};
 return record.platform===process.platform&&record.mode===mode&&record.path===slashes(relative(root,join(artifactDirectory(root,mode),executableName())));
}

const skippedDirectories=new Set(["target","node_modules","dist","build",".git","🗑️generated","🧪️tests","🧫️fixtures","coverage","__pycache__","storybook-static","temp"]);
const skippedFiles=/\.(?:tsx?|mts|mjs|js|py|log|md|feature|tsv|tsbuildinfo)$/;
const workspaceFiles=installationSchema.definitions.SourceInputProtocol.const.rootFiles;

/** 🔎️ Whether this host and mode have a valid installation record; no executable bytes are read. */
export function dashboardInstalled(root:string,mode:BuildMode):boolean { try{installedDashboard(root,mode,root);return true;}catch{return false;} }

/** 📍️ Resolves the installation record before the finite tools reader admits executable bytes. */
export function installedDashboard(root:string,mode:BuildMode,workspace:string):string {
  const file=manifest(root,mode);
  if(!existsSync(file))throw new Error("Dashboard is not installed. Start repo dashboard to install it automatically.");
  const value=JSON.parse(readFileSync(file,"utf8"));
  if(!validDashboardRecord(value,mode))throw new Error("Invalid dashboard installation record. Start repo dashboard to rebuild it automatically.");
  const executable=resolve(workspace,value.path);
  return executable;
}

/** 🧭️ The source roots of one crate: its module directory (`<module>/📦️packages/🦀️rust` owns its siblings) or itself. */
function crateRoot(directory:string):string {
  return basename(directory)==="🦀️rust"&&basename(dirname(directory))==="📦️packages"?dirname(dirname(directory)):directory;
}

type CargoTable=Readonly<Record<string,unknown>>;
const cargoTable=(value:unknown):CargoTable=>value!==null&&typeof value==="object"&&!Array.isArray(value)?value as CargoTable:{};

/** 🌳️ Resolves actual workspace manifests and the selected binary's dependency lock. */
function sourceInputs(packageRoot:string,workspace:string):Pick<DashboardSources,"roots"|"manifests">&{readonly crates:readonly string[]} {
  workspace=resolve(workspace);const cache=new Map<string,CargoTable>(),seen=new Set<string>(),pending=[resolve(packageRoot)],roots=new Set<string>(),manifests=new Set<string>(),crates=new Set<string>();
  const inside=(path:string)=>{const local=relative(workspace,path);if(local===".."||local.startsWith("../")||local.startsWith("..\\")||isAbsolute(local))throw Error("Dashboard Cargo source must belong to the workspace");return path;};
  const read=(directory:string)=>{const file=join(inside(directory),"Cargo.toml");let value=cache.get(file);if(!value){value=cargoTable(runtimeParseTomlV1(readFileSync(file,"utf8")));cache.set(file,value);}return value;};
  const owner=(directory:string,manifest:CargoTable)=>{const declared=cargoTable(manifest.package).workspace;if(declared!==undefined){if(typeof declared!=="string")throw Error("Declared Cargo workspace path required");const actual=inside(resolve(directory,declared));if(!("workspace" in read(actual)))throw Error("Declared Cargo workspace manifest required");return actual;}for(let candidate=directory;;candidate=dirname(candidate)){if(existsSync(join(candidate,"Cargo.toml"))&&"workspace" in read(candidate))return candidate;if(candidate===workspace)break;}return directory;};
  const selected=inside(resolve(packageRoot));manifests.add(slashes(relative(workspace,join(owner(selected,read(selected)),installationSchema.definitions.SourceInputProtocol.const.selectedWorkspaceLock))));
  while(pending.length){
    const directory=inside(pending.pop()!);
    if(seen.has(directory))continue;
    seen.add(directory);
    const file=join(directory,"Cargo.toml");
    if(!existsSync(file))continue;
    crates.add(slashes(relative(workspace,file)));
    const manifest=read(directory),workspaceRoot=owner(directory,manifest),shared=cargoTable(cargoTable(read(workspaceRoot).workspace).dependencies);
    manifests.add(slashes(relative(workspace,join(workspaceRoot,"Cargo.toml"))));
    roots.add(crateRoot(directory));
    const blocks=[manifest.dependencies,manifest["build-dependencies"],...Object.values(cargoTable(manifest.target)).flatMap(value=>{const target=cargoTable(value);return[target.dependencies,target["build-dependencies"]];})];
    for(const block of blocks)for(const[name,value]of Object.entries(cargoTable(block))){const dependency=cargoTable(value);if(typeof dependency.path==="string")pending.push(resolve(directory,dependency.path));else if(dependency.workspace===true){if(!(name in shared))throw Error("Declared inherited Cargo dependency required: "+name);const inherited=cargoTable(shared[name]);if(typeof inherited.path==="string")pending.push(resolve(workspaceRoot,inherited.path));}}
  }
  const ordered=[...roots].sort((a,b)=>a.length-b.length).filter((root,index,all)=>!all.slice(0,index).some(parent=>root===parent||root.startsWith(parent+"/")||root.startsWith(parent+"\\")));
  return{roots:ordered.map(root=>slashes(relative(workspace,root))).sort(),manifests:[...manifests].sort(),crates:[...crates].sort()};
}

/** 🪪️ Retains exact filesystem identity and nanosecond metadata without reading Source bytes. */
function sourceSignature(name:string,facts:Readonly<{dev:bigint;ino:bigint;size:bigint;mtimeNs:bigint;ctimeNs:bigint}>):string{
 return[slashes(name),...installationSchema.definitions.SourceSignatureProtocol.const.metadataFields.map(field=>String(facts[field as keyof typeof facts]))].join("\0");
}

/** 🏃️ Observes every relevant Source file below one root concurrently. */
async function collect(directory:string,workspace:string,entries:string[]):Promise<void> {
  let children;
  try { children=await readdir(directory,{withFileTypes:true}); } catch { return; }
  await Promise.all(children.map(async child=>{
    const path=join(directory,child.name);
    if(child.isDirectory()){if(!skippedDirectories.has(child.name))await collect(path,workspace,entries);return;}
    if(skippedFiles.test(child.name))return;
    try { const info=await stat(path,{bigint:true});entries.push(sourceSignature(relative(workspace,path),info)); } catch {}
  }));
}

/** 🔏️ Digests file identity, size and nanosecond timestamps; no Source content is read. */
export async function digestDashboardSources(workspace:string,sources:Pick<DashboardSources,"roots"|"manifests">):Promise<string> {
  const entries:string[]=[];
  await Promise.all([...sources.roots.map(root=>collect(resolve(workspace,root),workspace,entries)),...[...new Set([...workspaceFiles,...sources.manifests])].map(async name=>{
    try { const info=await stat(join(workspace,name),{bigint:true});entries.push(sourceSignature(name,info)); } catch {}
  })]);
  const ordered=[...new Set(entries)].sort();
  return `${ordered.length}:${createHash("sha1").update(ordered.join("\n")).digest("hex")}`;
}

/** 📸️ Captures the source roots and their digest; take it before building so edits made during the build stay detectable. */
export async function captureDashboardSources(packageRoot:string,workspace:string):Promise<DashboardSources> {
  const {roots,manifests}=sourceInputs(packageRoot,workspace);
  return {roots,manifests,digest:await digestDashboardSources(workspace,{roots,manifests})};
}

/** 🕰️ The first compiled source that is missing, or that was written, replaced or re-linked at or after the build started (the later of modification and inode change time) and no longer holds the bytes it held then, or nothing: Cargo's own freshness rule over the whole freshness basis of the binary, made content-exact so an identical rewrite (a revert, a checkout, a restored cached artifact) stays current. Stats every file, reads only the touched ones. */
export function changedDashboardSourceV1(root:string,compiled:DashboardCompiledSources):string|undefined{
 if(compiled.digests.length!==compiled.sources.length)return compiled.sources[0];
 return compiled.sources.find((path,index)=>{const file=resolve(root,path);try{const facts=statSync(file);return Math.max(facts.mtimeMs,facts.ctimeMs)>=compiled.builtAtMs&&(compiled.digests[index]===""||sha256(file)!==compiled.digests[index]);}catch{return true;}});
}

/** 🌀️ The first file of a freshness basis that was written at or after its build started (its digest is unknown, `""`), or nothing when the basis held still: an artifact with such a file is stale at birth, and as the cacheable output of `build` it would come back on every action-cache hit of the same input bytes, so it is never exported. */
export function movedDashboardSourceV1(compiled:DashboardCompiledSources):string|undefined{return compiled.sources.find((_,index)=>compiled.digests[index]==="");}

/** 📥️ Admits the sources record Cargo published beside the executable and adds the rest of Cargo's freshness basis (every crate manifest, the workspace manifests and lock, the toolchain files, the package declaration), as installation-root-relative paths with the bytes each held at the build start; a missing or malformed record refuses the artifact. */
function compiledDashboardSourcesV1(packageRoot:string,root:string,file:string):DashboardCompiledSources{
 let text:string;try{text=readFileSync(file,"utf8");}catch{throw refuseBootstrapV1("invalid","dashboard.sources-record-required");}
 const record=parseCargoBinarySourcesV1(text);if(record===null)throw refuseBootstrapV1("invalid","dashboard.sources-record-required");
 const {manifests,crates}=sourceInputs(packageRoot,root),basis=[...record.sources.map(source=>slashes(relative(root,resolve(root,source)))),...crates,...manifests,...workspaceFiles,slashes(relative(root,join(packageRoot,"📋️project.json")))];
 const sources=[...new Set(basis)].filter(path=>existsSync(resolve(root,path))).sort();
 return {builtAtMs:record.builtAtMs,sources,digests:sources.map(path=>{const file=resolve(root,path),facts=statSync(file);return Math.max(facts.mtimeMs,facts.ctimeMs)<record.builtAtMs?sha256(file):"";})};
}

/** 🧷️ Whether an executable is in place with the size and permissions its record names. Only stats, no bytes. */
function executableInPlaceV1(executable:string,record:{bytes:number;permissions:number}):boolean{
 try{const facts=lstatSync(executable);return facts.isFile()&&!facts.isSymbolicLink()&&facts.size===record.bytes&&(facts.mode&0o777)===record.permissions;}catch{return false;}
}

/** ✅️ This mode's built artifact and the number of files of its freshness basis when its record is valid for this host, none of those files changed since its build and its executable is in place; nothing otherwise. */
export function currentDashboardArtifactV1(root:string,mode:BuildMode):{readonly executable:string;readonly compiled:number}|undefined{
 const record=readRecord(join(artifactDirectory(root,mode),artifactProtocol.record));
 if(!validDashboardArtifactV1(root,record,mode)||changedDashboardSourceV1(root,record.compiled)!==undefined)return;
 const executable=resolve(root,record.path);
 return executableInPlaceV1(executable,record)?{executable,compiled:record.compiled.sources.length}:undefined;
}

/** ⏭️ Says that a build of this mode has nothing to do, in the language of the command. */
export function sayDashboardCurrentV1(mode:BuildMode,sources:DashboardSources,compiled:number):void{sayBootstrapV1("dashboard.installation-current",{mode,files:Number.parseInt(sources.digest),compiled});}

/** ⏳️ Why the installed executable no longer matches the sources, or nothing while it is current. */
export async function staleDashboard(root:string,mode:BuildMode):Promise<string|undefined> {
  if(!existsSync(manifest(root,mode)))return "it is not installed";
  let record;
  try { record=JSON.parse(readFileSync(manifest(root,mode),"utf8")); } catch { return "the installation record is unreadable"; }
  const sources=record?.sources;
  if(!validDashboardRecord(record,mode))return "the installation record does not name its sources";
  const current=await digestDashboardSources(root,sources);
  if(current!==sources.digest){const before=Number.parseInt(sources.digest),after=Number.parseInt(current);return `its sources changed since installation (${before===after?`${after} files`:`${before} -> ${after} files`})`;}
  const changed=changedDashboardSourceV1(root,record.compiled);
  return changed===undefined?undefined:`its compiled source ${changed} changed after the build that produced it`;
}

/** 🗑️ Reclaims dashboard executables beyond the newest few once the installation record names the current one; a digest that any process still runs or reads keeps its directory, and a failed reclaim never fails an installation (the next one plans it again). */
export async function retireOldDashboardExecutablesV1(parent:string,current:string,workspace:string,control:ResourceLeaseControl,retire:(directories:readonly string[],options:Readonly<{leaseDirectory:string;signal:AbortSignal;control?:ResourceLeaseControl}>)=>Promise<unknown>):Promise<void> {
 try{
  const images=readdirSync(parent).filter(name=>/^[a-f0-9]{64}$/u.test(name)).map(identity=>({identity,createdAt:statSync(join(parent,identity)).mtimeMs}));
  await retire(planNxToolingRetentionV1(images,current,installationSchema.definitions.ExecutableRetentionProtocol.const.keepNewest).map(identity=>join(parent,identity)),{leaseDirectory:join(workspace,".cache/b/l/m/data"),signal:control.signal,control});
 }catch{checkResourceLeaseControl(control);}
}

/** 📤️ Copies the published executable into this mode's artifact directory (the cacheable output of `build`) with a record of what it was compiled from, under the exclusive artifact lease; a build whose freshness basis moved while it compiled is refused before anything is written (`movedDashboardSourceV1`), so the failed action caches nothing and the next build compiles again; the executable first, then the record that names it, each by an atomic rename, so a reader never sees a record without its bytes. */
export async function exportDashboardArtifactV1(packageRoot:string,root:string,options:DashboardExecutionOptions,sources:DashboardSources):Promise<string>{
 const mode=readBuildMode(options.mode),control=options.control,name=executableName(),directory=artifactDirectory(root,mode);compilerAdmissionBudgetV1(control);if(validateJsonSchemaSubset(installationSchema.properties.sources,sources).length)throw Error("Dashboard installation source ownership refused");
 return withRepositoryPublishedCargoExecutableV1(join(packageRoot,"Cargo.toml"),name,options.workspace,mode,options.invocation,control,async(artifact,control)=>withResourceLeases({directory:join(options.workspace,".cache/b/l/m/data"),control,signal:control.signal,resources:[{resource:"artifact:"+directory,mode:"exclusive"}]},async()=>{
  const compiled=compiledDashboardSourcesV1(packageRoot,root,join(dirname(artifact.path),installationSchema.definitions.CompiledSourceProtocol.const.record)),executable=join(directory,name),staged=`${directory}.${process.pid}.bin`,moved=movedDashboardSourceV1(compiled);
  if(moved!==undefined)throw refuseBootstrapV1("stale","dashboard.artifact-sources-moved",{mode,path:moved});
  mkdirSync(directory,{recursive:true});rmSync(staged,{force:true});copyFileSync(artifact.path,staged,fsConstants.COPYFILE_FICLONE);chmodSync(staged,artifact.receipt.permissions);renameSync(staged,executable);compilerAdmissionBudgetV1(control);
  writeRecord(join(directory,artifactProtocol.record),{version:5,platform:process.platform,mode,path:slashes(relative(root,executable)),sources,compiled,digest:artifact.receipt.digest,bytes:artifact.receipt.bytes,permissions:artifact.receipt.permissions});return executable;
 }));
}

/** 🧊️ Installs this mode's built artifact: local, idempotent and receipt-checked. It refuses an artifact that is missing, foreign or behind a file of its freshness basis; when the installation already names the same bytes and its immutable copy is in place it only refreshes the record (no tools store); otherwise it pins the artifact executable into the tools store. The record is the artifact record with the pinned `path` and the sources captured before the check. */
export async function installDashboardArtifactV1(packageRoot:string,root:string,options:DashboardExecutionOptions):Promise<string>{
 const mode=readBuildMode(options.mode),control=options.control,artifact=readRecord(join(artifactDirectory(root,mode),artifactProtocol.record));compilerAdmissionBudgetV1(control);
 if(!validDashboardArtifactV1(root,artifact,mode)||!executableInPlaceV1(resolve(root,artifact.path),artifact))throw refuseBootstrapV1("missing","dashboard.artifact-required",{mode});
 const sources=await captureDashboardSources(packageRoot,root),changed=changedDashboardSourceV1(root,artifact.compiled);if(changed!==undefined)throw refuseBootstrapV1("stale","dashboard.artifact-behind-sources",{mode,path:changed});
 const installed=readRecord(manifest(root,mode)),current=validDashboardRecord(installed,mode)&&installed.digest===artifact.digest?resolve(options.workspace,installed.path):undefined;
 if(current&&executableInPlaceV1(current,installed)){if(JSON.stringify(installed.sources)!==JSON.stringify(sources)||JSON.stringify(installed.compiled)!==JSON.stringify(artifact.compiled))writeRecord(manifest(root,mode),{...artifact,sources,path:installed.path});return current;}
 const {pinExecutableArtifact,retireArtifactDirectoriesV1}=await import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/🟦️.ts");
 return withRepositoryToolingStoreV1(options.workspace,options.invocation,control,async tools=>{
  const executable=await pinExecutableArtifact(resolve(root,artifact.path),join(tools.root,"dashboard-cli",mode),`dashboard-installation:${mode}`,{control,signal:control.signal,leaseDirectory:join(options.workspace,".cache/b/l/m/data"),onWait:async progress=>{compilerAdmissionBudgetV1(control);await control.publish(progress);await control.yieldContinuation();}});await tools.recheck();
  if(basename(dirname(executable))!==artifact.digest)throw refuseBootstrapV1("stale","dashboard.artifact-changed");const path=slashes(relative(options.workspace,executable));if(isAbsolute(path)||path.startsWith("../"))throw Error("Dashboard installation must stay inside workspace");
  compilerAdmissionBudgetV1(control);writeRecord(manifest(root,mode),{...artifact,sources,path});await retireOldDashboardExecutablesV1(join(tools.root,"dashboard-cli",mode),artifact.digest,options.workspace,control,retireArtifactDirectoriesV1);return executable;
 });
}

/** 📣️ Exports the published executable as this mode's artifact and installs it (the route of the store-backed installation laws and of callers holding a fresh compile). */
export async function installDashboard(packageRoot:string,root:string,options:DashboardExecutionOptions,sources?:DashboardSources):Promise<string> {
 await exportDashboardArtifactV1(packageRoot,root,options,sources??await captureDashboardSources(packageRoot,root));
 return installDashboardArtifactV1(packageRoot,root,options);
}

/** 🏪️ Holds actual tools image and immutable executable reader custody through its complete consumer lifetime. */
export async function withInstalledDashboardV1<T>(root:string,options:DashboardExecutionOptions,body:(executable:string)=>Promise<T>):Promise<T>{
 const mode=readBuildMode(options.mode),control=options.control;compilerAdmissionBudgetV1(control);
 return withRepositoryToolingStoreV1(options.workspace,options.invocation,control,async tools=>{const executable=installedDashboard(root,mode,options.workspace),record=JSON.parse(readFileSync(manifest(root,mode),"utf8")),expected=join(tools.root,"dashboard-cli",mode,record.digest,process.platform==="win32"?"semio.exe":"semio");if(executable!==expected)throw Error("Original finite dashboard tools ownership changed");return withResourceLeases({directory:join(options.workspace,".cache/b/l/m/data"),control,signal:control.signal,resources:[{resource:"artifact:"+dirname(executable),mode:"shared"}]},async()=>{await tools.recheck();for(let parent=dirname(executable);;parent=dirname(parent)){const facts=lstatSync(parent);if(!facts.isDirectory()||facts.isSymbolicLink())throw Error("Physical dashboard tools ancestry required");if(parent===tools.root)break;}const facts=lstatSync(executable);if(!facts.isFile()||facts.isSymbolicLink()||facts.nlink!==1||facts.size!==record.bytes||(facts.mode&0o777)!==record.permissions||process.platform!=="win32"&&(facts.mode&0o111)===0)throw Error("Exact immutable dashboard executable required");const file=await open(executable,"r"),hash=createHash("sha256"),buffer=Buffer.alloc(65536);let bytes=0;try{for(;;){checkResourceLeaseControl(control);const row=await file.read(buffer,0,buffer.length,null);if(!row.bytesRead)break;bytes+=row.bytesRead;if(bytes>record.bytes)throw Error("Dashboard executable changed during admission");hash.update(buffer.subarray(0,row.bytesRead));if(bytes%4194304===0){await control.publish({resource:"dashboard:installed-reader",mode:"shared",elapsedMs:0});await control.yieldContinuation();}}}finally{await file.close();}if(bytes!==record.bytes||hash.digest("hex")!==record.digest)throw Error("Dashboard executable content changed");const result=await body(executable);await tools.recheck();checkResourceLeaseControl(control);return result;});});
}

/** 🚀️ Recognizes native installed-tool launches while retaining the build runner for authoring tasks. */
export function dashboardInvocation(args:readonly string[]):string[]|undefined {
  if(args[0]!=="run")return;
  const prefix="@semio-tech/repo-dashboard-rs:",target=args[1]?.startsWith(prefix)?args[1].slice(prefix.length):undefined;
  if(!target||!["run","daemon","preferences"].includes(target))return;
  const rest=args.slice(2),separator=rest.indexOf("--"),options=separator<0?rest:rest.slice(0,separator);
  if(options.some(option=>/^--(?:graph|batch|parallel|exclude-dependencies)(?:=|$)/.test(option)))return;
  const forwarded=options.filter(option=>!/^--(?:output)=stream$/.test(option)&&!["--skip-cache"].includes(option));
  return [...(target==="run"?[]:[target]),...forwarded,...(separator<0?[]:rest.slice(separator+1))];
}

/** 🧩️ Reads exact conditional children from the physical dashboard owner before Original issuance. */
export function dashboardBuildDescendantsV1(args:readonly string[]):string[][] {
  const prefix="@semio-tech/repo-dashboard-rs:";if(args[0]!=="run"||!args[1]?.startsWith(prefix))return[];
  const target=args[1].slice(prefix.length).split(":")[0]!,file=join(dashboardPackageRoot,"📋️project.json"),facts=lstatSync(file);
  if(file.length>256||!facts.isFile()||facts.isSymbolicLink()||facts.nlink!==1||facts.size>4194304)throw Error("Physical dashboard bootstrap owner required");
  const owner=JSON.parse(readFileSync(file,"utf8"));if(owner.name!==prefix.slice(0,-1))throw Error("Exact dashboard bootstrap owner required");
  const declaration=owner.targets?.[target]?.metadata?.semio?.execution;if(declaration===undefined)return[];
  if(declaration?.kind!=="orchestration"){if(validateJsonSchemaSubset(executionSchema,declaration).length)throw Error("Closed dashboard bootstrap declaration required");return[];}
  if(validateJsonSchemaSubset(orchestrationSchema,declaration).length)throw Error("Closed dashboard bootstrap declaration required");
  return declaration.children.map((child:{project:string;target:string;configuration?:string})=>{if(child.project!==owner.name||!Object.hasOwn(owner.targets,child.target))throw Error("Owned dashboard bootstrap child required");return["run",child.project+":"+child.target+(child.configuration?":"+child.configuration:"")];});
}

/** 🛟️ Whether these native arguments only observe or end what already runs, so an installation that is behind the sources may still answer them: a broken working tree must not stop a developer from stopping the daemon. */
export function mayRunStale(args:readonly string[]):boolean {
  const [verb,action]=args;
  return ["tasks","logs","stop","kill","open"].includes(verb??"")||(verb==="daemon"&&["stop","status"].includes(action??""));
}

/** 🔨️ Rebuilds and reinstalls through the package script with inherited output; Ctrl+C cancels the build. */
async function rebuildDashboard(reason:string,options:DashboardExecutionOptions):Promise<void> {
  console.log(`[dashboard] Rebuilding ${options.mode} because ${reason}`);
  await runOwnedCommand(process.execPath,[join(dashboardPackageRoot,"📜️script.ts"),"build"],dashboardPackageRoot,"dashboard:rebuild",compilerAdmissionBudgetV1(options.control),{env:scriptResourceProcessEnvironmentV1(options.control,scriptInvocationEnvironment(options.invocation,{...process.env,SEMIO_BUILD_MODE:options.mode,SEMIO_SCRIPT_POLICY:undefined})),signal:options.control.signal});
}

/** ▶️ Hands the terminal to the selected immutable binary under original cancellation and deadline. */
export async function launchDashboard(root:string,args:readonly string[],options:DashboardExecutionOptions):Promise<number> {
  const mode=readBuildMode(options.mode),control=options.control;compilerAdmissionBudgetV1(control);
  const reason=mayRunStale(args)&&dashboardInstalled(root,mode)?undefined:await staleDashboard(root,mode);compilerAdmissionBudgetV1(control);
  if(reason)await withRenewableIssuedResourceControlV1(control,progressDeadlinePolicyV1("native-child",0,process.env,{maximumMilliseconds:buildPreparationBudgetMs()}),0,preparation=>rebuildDashboard(reason,{...options,control:preparation}));
  const controller=new AbortController(),signal=AbortSignal.any([control.signal,controller.signal]),budget=compilerAdmissionBudgetV1(control),stopTimeout=startOwnedCommandTimeoutV1(budget,()=>controller.abort(Error("Original dashboard launch deadline exhausted")));
  try{return await withInstalledDashboardV1(root,options,executable=>new Promise<number>((accept,reject)=>{
    const child=spawn(executable,args,{cwd:root,env:scriptInvocationEnvironment(options.invocation,{...process.env,SEMIO_BUILD_MODE:mode}),signal,stdio:"inherit",windowsHide:true});
    child.once("error",reject);child.once("close",(code,closedSignal)=>accept(code??(closedSignal==="SIGINT"?130:1)));
  }));}finally{stopTimeout();}
}
