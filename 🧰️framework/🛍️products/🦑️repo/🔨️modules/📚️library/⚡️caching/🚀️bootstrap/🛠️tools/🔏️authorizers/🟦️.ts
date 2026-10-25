const viaV1=new Map<string,string>(),viaChainV1=(via:ReadonlyMap<string,string>,from:string):string=>{const out=[from];for(let at=via.get(from);at!==undefined&&out.length<14;at=via.get(at))out.push(at);return out.join(" <- ");};
import{nxAuthorizerWorkspaceCandidateV1,nxAuthorizerWorkspaceIndexV1,resolveNxAuthorizerWorkspaceV1,type NxAuthorizerResolutionV1}from"./📦️workspace/🟦️.ts";
import authorizerSchema from "./🧬️schema/🔣️.json" with { type: "json" };
import {closureLiteralSitesV1,closurePackageNameV1,closureSpecifierKindV1,closureTargetSchemesV1} from "./🧭️sites/🟦️.ts";
import{constants}from"node:fs";
import {createHash} from "node:crypto";
import {lstat,open,opendir,readFile,realpath} from "node:fs/promises";
import {dirname,extname,isAbsolute,join,relative,resolve} from "node:path";
import { runtimeResolveV1 } from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🚀️process/🟦️.ts";

/** 🧬️ Exact runtime source names and bytes compiled into one release toolkit. */
export interface NxAuthorizerSourceManifestV1 {readonly version:1;readonly mode:"release";readonly entry:string;readonly resolutions:readonly NxAuthorizerResolutionV1[];readonly sources:readonly {readonly path:string;readonly sha256:string}[];}
/** 🧊️ Frozen compiler inputs stay behind repository-owned values. */
export interface NxAuthorizerSourcesV1 {readonly manifest:NxAuthorizerSourceManifestV1;readonly digest:string;}
/** 📌️ The exact pinned tool intrinsics shared by runtime source capture and compilation. */
export const nxAuthorizerExternalModulesV1=Object.freeze(["nx/src/tasks-runner/create-task-graph","nx/src/hasher/task-hasher","nx/src/utils/workspace-context","nx/src/daemon/client/client","nx/src/native","nx/src/utils/consume-messages-from-socket","nx/src/daemon/socket-utils","nx/src/daemon/client/daemon-environment"]);
const capturedBytes=new WeakMap<NxAuthorizerSourcesV1,ReadonlyMap<string,Uint8Array>>(),capturedInstalled=new WeakMap<NxAuthorizerSourcesV1,Readonly<{installed:Readonly<Record<string,string>>;roots:ReadonlyMap<string,readonly string[]>}>>();
/** 🛠️ Selects what a capture derives beyond the compiler toolkit: installed third-party packages and computed-site roots for command closures. */
export interface NxAuthorizerCaptureOptionsV1{readonly installed?:boolean;readonly roots?:ReadonlyMap<string,readonly string[]>}
/** 🧷️ Returns only the byte census issued by an actual no-follow Source capture. */
export function nxAuthorizerCapturedBytesV1(source:NxAuthorizerSourcesV1):ReadonlyMap<string,Uint8Array>{const bytes=capturedBytes.get(source);if(!bytes)throw Error("Original captured authorizer bytes required");return new Map([...bytes].map(([path,value])=>[path,new Uint8Array(value)]));}
/** 📦️ Installed third-party tool packages that stay outside the captured closure and load natively from the pinned installation. */
export const nxAuthorizerInstalledPackagesV1=Object.freeze(["typescript"]);
const external=new Set([...nxAuthorizerExternalModulesV1,...nxAuthorizerInstalledPackagesV1]),maximumSourceFiles=authorizerSchema.properties.sources.maxItems,computedRoots=new Map<string,readonly string[]>(authorizerSchema.$defs.ComputedImportSites.const.filter(row=>row.roots.length).map(row=>[row.owner,Object.freeze([...row.roots])] as const)),runtimeDataCompanions=new Map<string,readonly string[]>([...new Set([...authorizerSchema.$defs.RuntimeDataCompanions.const.map(row=>row.owner),...computedRoots.keys()])].map(owner=>[owner,Object.freeze([...authorizerSchema.$defs.RuntimeDataCompanions.const.filter(row=>row.owner===owner).flatMap(row=>row.paths),...(computedRoots.get(owner)??[])])] as const));
const hash=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex");
const compare=(left:string,right:string)=>Buffer.compare(Buffer.from(left),Buffer.from(right));
/** 🔎️ Reads the literal sites a capture follows: scanner imports and relative createRequire calls, plus, when third-party packages are derived, every literal require, resolve and package site. */
function scannedImportRowsV1(path:string,source:string,installed:boolean):readonly {path:string;lenient:boolean}[]{
 return closureLiteralSitesV1(path,source).filter(row=>installed||row.scanned||row.kind==="create-require"&&row.specifier.startsWith(".")).map(row=>({path:row.specifier,lenient:row.lenient}));
}
function scannedImport(path:string):string{
 if(!/[\u0080-\u00ff]/u.test(path)||/[\u0100-\uffff]/u.test(path))return path;
 const decoded=Buffer.from(path,"latin1").toString("utf8");return Buffer.from(decoded,"utf8").toString("latin1")===path?decoded:path;
}
function sourcePath(root:string,path:string):string{
 if(!path||isAbsolute(path)||/[:\\\0]/u.test(path)||path.split("/").some(part=>!part||part==="."||part===".."))throw Error("Closed authorizer relative source path required");
 const absolute=join(root,path);if(absolute.length>256||[...absolute].length>256)throw Error("Authorizer materialized source path exceeds 256 characters");return absolute;
}
/** 🔒️ Reads actual regular source bytes without following substituted ancestry. */
export async function readNxAuthorizerSourceV1(root:string,path:string,signal:AbortSignal):Promise<Uint8Array>{
 signal.throwIfAborted();const absolute=sourcePath(root,path),parents=[root];let current=root;
 for(const part of path.split("/").slice(0,-1)){current=join(current,part);parents.push(current);}
 const ancestry=await Promise.all(parents.map(async path=>{const status=await lstat(path,{bigint:true});if(!status.isDirectory()||status.isSymbolicLink())throw Error("Authorizer source requires no-follow directories");return {path,device:status.dev,inode:status.ino};}));
 const before=await lstat(absolute,{bigint:true});signal.throwIfAborted();if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size>4_194_304n)throw Error("Authorizer requires bounded original regular source");const fd=await open(absolute,constants.O_RDONLY|(constants.O_NOFOLLOW??0));let failed=false,failure:unknown,bytes:Uint8Array|undefined;
 try{signal.throwIfAborted();const original=await fd.stat({bigint:true});signal.throwIfAborted();const stamp=(row:typeof original)=>[row.dev,row.ino,row.size,row.nlink,row.mtimeNs,row.ctimeNs].join(":");if(!original.isFile()||stamp(original)!==stamp(before))throw Error("Original authorizer descriptor required");const buffer=Buffer.alloc(Number(original.size));for(let offset=0;offset<buffer.length;){const row=await fd.read(buffer,offset,buffer.length-offset,offset);signal.throwIfAborted();if(!row.bytesRead)throw Error("Short authorizer Source read");offset+=row.bytesRead;}const tail=await fd.read(Buffer.alloc(1),0,1,buffer.length);signal.throwIfAborted();const after=await fd.stat({bigint:true});signal.throwIfAborted();const named=await lstat(absolute,{bigint:true});signal.throwIfAborted();if(tail.bytesRead||!named.isFile()||named.isSymbolicLink()||stamp(original)!==stamp(after)||stamp(original)!==stamp(named))throw Error("Authorizer source changed during capture");for(const row of ancestry){const status=await lstat(row.path,{bigint:true});signal.throwIfAborted();if(!status.isDirectory()||status.isSymbolicLink()||status.dev!==row.device||status.ino!==row.inode)throw Error("Authorizer source ancestry changed during capture");}bytes=buffer;}catch(error){failed=true;failure=error;}try{await fd.close();}catch(error){if(!failed){failed=true;failure=error;}else try{console.error("[authorizer] Source descriptor close failed after primary",error);}catch{}}if(failed)throw failure;signal.throwIfAborted();return bytes!;
}
interface RuntimeDataDiscoveryV1{readonly directories:Set<string>;readonly results:Set<string>;entries:number;bytes:number}
const discoveryPolicy=authorizerSchema.$defs.RuntimeDataDiscoveryPolicy.const;
function discoveryStampV1(value:Awaited<ReturnType<typeof lstat>>):string{return["dev","ino","mode","nlink","size","mtimeNs","ctimeNs"].map(key=>String(value[key as keyof typeof value])).join(":");}
async function discoveryAncestorsV1(root:string,path:string,signal:AbortSignal):Promise<readonly{path:string;stamp:string;device:bigint;inode:bigint}[]>{
 sourcePath(root,path);const rows=[],parts=path.split("/");let target=root;for(let index=-1;index<parts.length;index++){signal.throwIfAborted();if(index>=0)target=join(target,parts[index]!);if(target.length>256)throw Error("Bounded authorizer DATA ancestry required");const facts=await lstat(target,{bigint:true});signal.throwIfAborted();if(!facts.isDirectory()||facts.isSymbolicLink())throw Error("No-follow authorizer DATA directory required");rows.push({path:target,stamp:discoveryStampV1(facts),device:facts.dev,inode:facts.ino});}return rows;
}
async function discoveryDirectoryHoldsV1(rows:readonly{path:string;stamp:string;device:bigint;inode:bigint}[],signal:AbortSignal):Promise<void>{for(let index=0;index<rows.length;index++){const row=rows[index]!;signal.throwIfAborted();const facts=await lstat(row.path,{bigint:true});signal.throwIfAborted();if(!facts.isDirectory()||facts.isSymbolicLink()||facts.dev!==row.device||facts.ino!==row.inode||index===rows.length-1&&discoveryStampV1(facts)!==row.stamp)throw Error("Authorizer DATA directory changed");}}
/** 🔣️ Discovers bounded module-owned JSON DATA without granting executable import authority. */
async function runtimeDataDiscoveryV1(root:string,path:string,source:string,signal:AbortSignal,state:RuntimeDataDiscoveryV1):Promise<string[]>{
 sourcePath(root,path);signal.throwIfAborted();if(!/\.[cm]?[jt]sx?$/u.test(path))return[];const rows=new Set<string>(),directory=dirname(path)==="."?"":dirname(path)+"/",add=(path:string,facts:Awaited<ReturnType<typeof lstat>>)=>{if(!facts.isFile()||facts.isSymbolicLink()||facts.size<0||facts.size>4_194_304n)throw Error("Bounded ordinary authorizer JSON DATA required");if(!state.results.has(path)){if(state.results.size>=discoveryPolicy.maximumResults||state.bytes+Number(facts.size)>discoveryPolicy.maximumDeclaredBytes)throw Error("Authorizer DATA result capacity exceeded");state.results.add(path);state.bytes+=Number(facts.size);}rows.add(path);},pending=[{path:directory+"🧬️schema",depth:0}];
 while(pending.length){signal.throwIfAborted();const current=pending.pop()!;sourcePath(root,current.path);if(current.depth>discoveryPolicy.maximumDepth)throw Error("Authorizer DATA directory depth exceeded");if(state.directories.has(current.path))continue;let ancestry;try{ancestry=await discoveryAncestorsV1(root,current.path,signal);}catch(error){signal.throwIfAborted();if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}if(state.directories.size>=discoveryPolicy.maximumDirectories)throw Error("Authorizer DATA directory capacity exceeded");state.directories.add(current.path);signal.throwIfAborted();const stream=await opendir(sourcePath(root,current.path),{bufferSize:1});let failed=false,primary:unknown;
  try{signal.throwIfAborted();await discoveryDirectoryHoldsV1(ancestry,signal);for(;;){signal.throwIfAborted();const entry=await stream.read();signal.throwIfAborted();await discoveryDirectoryHoldsV1(ancestry,signal);if(!entry)break;if(state.entries>=discoveryPolicy.maximumEntries)throw Error("Authorizer DATA entry capacity exceeded");state.entries++;const child=current.path+"/"+entry.name;sourcePath(root,child);await discoveryDirectoryHoldsV1(ancestry,signal);const facts=await lstat(sourcePath(root,child),{bigint:true});signal.throwIfAborted();await discoveryDirectoryHoldsV1(ancestry,signal);if(facts.isSymbolicLink())throw Error("No-follow authorizer DATA entry required");if(facts.isDirectory()){if(pending.length+state.directories.size>=discoveryPolicy.maximumDirectories)throw Error("Authorizer DATA directory capacity exceeded");pending.push({path:child,depth:current.depth+1});}else if(facts.isFile()){if(extname(child)===".json")add(child,facts);}else throw Error("Ordinary authorizer DATA entry required");}await discoveryDirectoryHoldsV1(ancestry,signal);}catch(error){failed=true;primary=error;}try{await stream.close();signal.throwIfAborted();}catch(error){if(!failed){failed=true;primary=error;}}if(failed)throw primary;await discoveryDirectoryHoldsV1(ancestry,signal);signal.throwIfAborted();
 }
 for(const match of source.matchAll(/(["'\x60])(\.{1,2}\/[^"'\x60\\\r\n$]+\.json)\1/gu)){signal.throwIfAborted();const child=relative(root,resolve(root,directory,match[2]!)).replaceAll("\\","/");if(child.split("/").some(part=>part===".."||part==="node_modules"||part===".cache"))continue;sourcePath(root,child);let ancestry;try{const parent=dirname(child);ancestry=parent==="."?[]:await discoveryAncestorsV1(root,parent,signal);}catch(error){signal.throwIfAborted();if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}if(dirname(child)==="."){signal.throwIfAborted();const facts=await lstat(root,{bigint:true});signal.throwIfAborted();if(!facts.isDirectory()||facts.isSymbolicLink())throw Error("No-follow authorizer DATA root required");ancestry=[{path:root,stamp:discoveryStampV1(facts),device:facts.dev,inode:facts.ino}];}let facts;try{signal.throwIfAborted();facts=await lstat(sourcePath(root,child),{bigint:true});signal.throwIfAborted();}catch(error){signal.throwIfAborted();if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}await discoveryDirectoryHoldsV1(ancestry,signal);add(child,facts);}
 return[...rows].sort(compare);
}
/** 📥️ Uses the system compiler's import scanner while refusing unresolved runtime dependencies. */
function installedPathV1(inside:string):boolean{return !inside.startsWith("../")&&inside.split("/").includes("node_modules");}
/** 🎉️ Pins one installed third-party package by name and installed version, the declared dependency image a command closure resolves natively. */
async function installedMarkV1(root:string,inside:string,specifier:string,versions:Map<string,string>,signal:AbortSignal):Promise<string>{
 const parts=inside.split("/"),at=parts.lastIndexOf("node_modules"),name=closurePackageNameV1(specifier),count=name.split("/").length,directory=parts.slice(0,at+1+count).join("/");
 let version=versions.get(directory);if(version===undefined){const manifest=join(root,directory,"package.json"),facts=await lstat(manifest);signal.throwIfAborted();if(!facts.isFile()||facts.size>1_048_576)throw Error("Bounded installed package manifest required: "+directory);const value:unknown=JSON.parse(Buffer.from(await readFile(manifest)).toString("utf8"));signal.throwIfAborted();version=value!==null&&typeof value==="object"&&typeof (value as {version?:unknown}).version==="string"?(value as {version:string}).version:"unversioned";versions.set(directory,version);}
 return closureTargetSchemesV1.installed+name+"@"+version;
}
export async function captureNxAuthorizerSourcesV1(workspace:string,entry:string,signal:AbortSignal,options:NxAuthorizerCaptureOptionsV1={}):Promise<NxAuthorizerSourcesV1>{
 const installed=options.installed===true,roots=options.roots??computedRoots,root=resolve(workspace),files=new Map<string,string>(),contents=new Map<string,Uint8Array>(),pending=[entry],resolutions=new Map<string,string>(),marks=new Map<string,string>(),versions=new Map<string,string>(),discovery:RuntimeDataDiscoveryV1={directories:new Set(),results:new Set(),entries:0,bytes:0};let total=0;
 const capture=async(path:string)=>{if(contents.has(path))return contents.get(path)!;const bytes=await readNxAuthorizerSourceV1(root,path,signal);signal.throwIfAborted();files.set(path,hash(bytes));contents.set(path,new Uint8Array(bytes));total+=bytes.byteLength;if(files.size>maximumSourceFiles||total>16_777_216)throw Error(`Authorizer source closure capacity exceeded: files=${files.size}/${maximumSourceFiles}, bytes=${total}/16777216`);return bytes;};
 while(pending.length){signal.throwIfAborted();const path=pending.pop()!;if(files.has(path))continue;const bytes=await capture(path);signal.throwIfAborted();
  for(const companion of [...(runtimeDataCompanions.get(path)??[]),...(options.roots?.get(path)??[])]){sourcePath(root,companion);pending.push(companion);}
  if(extname(path)===".json"){JSON.parse(Buffer.from(bytes).toString("utf8"));continue;}
  const source=Buffer.from(bytes).toString("utf8").replace(/^#![^\r\n]*/u,"");
  for(const companion of await runtimeDataDiscoveryV1(root,path,source,signal,discovery))pending.push(companion);
  for(const row of scannedImportRowsV1(path,source,installed)){
   const specifier=scannedImport(row.path),kind=closureSpecifierKindV1(specifier);
   if(kind==="builtin"||installed&&(kind==="absolute"||kind==="scheme")||!installed&&external.has(specifier))continue;
   let child:string;
   if(kind==="relative"){child=relative(root,resolve(dirname(join(root,path)),specifier)).replaceAll("\\","/");if(row.lenient){try{await lstat(sourcePath(root,child));}catch(error){signal.throwIfAborted();continue;}signal.throwIfAborted();}}
   else{const earlier=resolutions.get(specifier);if(earlier!==undefined)child=earlier;else if(marks.has(specifier))continue;else{
    try{const rootBytes=await capture("package.json"),located=await runtimeResolveV1(specifier,dirname(join(root,path)));signal.throwIfAborted();const actual=await realpath(located),physicalRoot=await realpath(root);signal.throwIfAborted();const candidate=relative(physicalRoot,actual).replaceAll("\\","/");
     if(installed&&installedPathV1(candidate)){marks.set(specifier,await installedMarkV1(root,candidate,specifier,versions,signal));continue;}
     sourcePath(root,candidate);const manifest=nxAuthorizerWorkspaceCandidateV1(rootBytes,candidate);await capture(manifest);signal.throwIfAborted();child=resolveNxAuthorizerWorkspaceV1(nxAuthorizerWorkspaceIndexV1(contents),specifier);if(child!==candidate)throw Error("Located package differs from its captured exact export");resolutions.set(specifier,child);
    }catch(error){signal.throwIfAborted();if(row.lenient)continue;throw error instanceof Error?new Error(error.message+" (importer chain "+viaChainV1(viaV1,path)+", specifier "+specifier+")",{cause:error}):error;}
   }}
   sourcePath(root,child);if(!viaV1.has(child))viaV1.set(child,path);pending.push(child);
  }
 }
 const manifest:NxAuthorizerSourceManifestV1=Object.freeze({version:1,mode:"release",entry,resolutions:Object.freeze([...resolutions].sort(([left],[right])=>compare(left,right)).map(([specifier,path])=>Object.freeze({specifier,path}))),sources:Object.freeze([...files].sort(([left],[right])=>compare(left,right)).map(([path,sha256])=>Object.freeze({path,sha256})))});
 const result=Object.freeze({manifest,digest:hash(JSON.stringify(manifest)+"\n")});capturedBytes.set(result,contents);if(installed)capturedInstalled.set(result,Object.freeze({installed:Object.freeze(Object.fromEntries([...marks].sort(([left],[right])=>compare(left,right)))),roots}));return result;
}
/** 🕸️ Resolves only executable compiler imports against the same privately captured byte census; with installed markers it also resolves third-party packages and computed-site roots. */
export function nxAuthorizerImportProjectionV1(workspace:string,bytes:ReadonlyMap<string,Uint8Array>,resolutions:readonly NxAuthorizerResolutionV1[],derived?:Readonly<{installed:Readonly<Record<string,string>>;roots:ReadonlyMap<string,readonly string[]>}>):ReadonlyMap<string,Readonly<Record<string,string>>>{
 const root=resolve(workspace),packages=new Map(resolutions.map(row=>[row.specifier,row.path])),result=new Map<string,Readonly<Record<string,string>>>();
 for(const[path,value]of bytes){const imports:Record<string,string>={};if(extname(path)!==".json"){const text=Buffer.from(value).toString("utf8").replace(/^#![^\r\n]*/u,"");
  for(const row of scannedImportRowsV1(path,text,derived!==undefined)){const specifier=scannedImport(row.path),kind=closureSpecifierKindV1(specifier);if(kind==="builtin"||derived!==undefined&&(kind==="absolute"||kind==="scheme")||derived===undefined&&external.has(specifier))continue;const target=kind==="relative"?relative(root,resolve(dirname(join(root,path)),specifier)).replaceAll("\\","/"):packages.get(specifier)??derived?.installed[specifier];if(!target||!target.startsWith(closureTargetSchemesV1.installed)&&!bytes.has(target)){if(row.lenient&&derived!==undefined)continue;throw Error("Complete immutable compiler import required: "+specifier);}if(!target.startsWith(closureTargetSchemesV1.installed))sourcePath(root,target);imports[specifier]=target;}
  if(derived!==undefined)for(const target of derived.roots.get(path)??[])if(bytes.has(target)){sourcePath(root,target);imports[closureTargetSchemesV1.root+target]=target;}}
  result.set(path,Object.freeze(imports));}
 return result;}
/** 🔗️ Reads import edges only from an actual privately issued compiler capture. */
export function nxAuthorizerCapturedImportsV1(workspace:string,source:NxAuthorizerSourcesV1):ReadonlyMap<string,Readonly<Record<string,string>>>{return nxAuthorizerImportProjectionV1(workspace,nxAuthorizerCapturedBytesV1(source),source.manifest.resolutions,capturedInstalled.get(source));}
/** ✅️ Re-enumerates the actual runtime closure before publishing compiled authorizers. */
export async function nxAuthorizerSourcesHoldV1(root:string,original:NxAuthorizerSourcesV1,signal:AbortSignal):Promise<boolean>{
 try{return (await captureNxAuthorizerSourcesV1(root,original.manifest.entry,signal,{installed:capturedInstalled.has(original),roots:capturedInstalled.get(original)?.roots})).digest===original.digest;}catch(error){signal.throwIfAborted();return false;}
}
