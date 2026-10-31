const viaV1=new Map<string,string>(),viaChainV1=(via:ReadonlyMap<string,string>,from:string):string=>{const out=[from];for(let at=via.get(from);at!==undefined&&out.length<14;at=via.get(at))out.push(at);return out.join(" <- ");};
import{nxAuthorizerWorkspaceCandidateV1,nxAuthorizerWorkspaceIndexV1,resolveNxAuthorizerWorkspaceV1,type NxAuthorizerResolutionV1}from"./📦️workspace/🟦️.ts";
import authorizerSchema from "./🧬️schema/🔣️.json" with { type: "json" };
import {closureLiteralSitesV1,closurePackageNameV1,closureSpecifierKindV1,closureTargetSchemesV1} from "./🧭️sites/🟦️.ts";
import{constants}from"node:fs";
import {createHash} from "node:crypto";
import {lstat,open,opendir,readFile,realpath} from "node:fs/promises";
import{nxAuthorizerMemoDirectoriesV1,nxAuthorizerMemoImplementationV1,readNxAuthorizerMemoV1,replayNxAuthorizerMemoV1,writeNxAuthorizerMemoV1,type NxAuthorizerObservationV1}from"./🧊️memo/🟦️.ts";
import {dirname,extname,isAbsolute,join,relative,resolve} from "node:path";
import { runtimeResolveV1 } from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🚀️process/🟦️.ts";
import {refuseBootstrapV1} from "../../📖️texts/🟦️.ts";

/** 🧬️ Exact runtime source names and bytes compiled into one release toolkit. */
export interface NxAuthorizerSourceManifestV1 {readonly version:1;readonly mode:"release";readonly entry:string;readonly resolutions:readonly NxAuthorizerResolutionV1[];readonly sources:readonly {readonly path:string;readonly sha256:string}[];}
/** 🧊️ Frozen compiler inputs stay behind repository-owned values. */
export interface NxAuthorizerSourcesV1 {readonly manifest:NxAuthorizerSourceManifestV1;readonly digest:string;}
/** 📌️ The exact pinned tool intrinsics shared by runtime source capture and compilation. */
export const nxAuthorizerExternalModulesV1=Object.freeze(["nx/src/tasks-runner/create-task-graph","nx/src/hasher/task-hasher","nx/src/utils/workspace-context","nx/src/daemon/client/client","nx/src/native","nx/src/utils/consume-messages-from-socket","nx/src/daemon/socket-utils","nx/src/daemon/client/daemon-environment"]);
const capturedBytes=new WeakMap<NxAuthorizerSourcesV1,ReadonlyMap<string,Uint8Array>>(),capturedInstalled=new WeakMap<NxAuthorizerSourcesV1,Readonly<{installed:Readonly<Record<string,string>>;roots:ReadonlyMap<string,readonly string[]>}>>(),capturedImports=new WeakMap<NxAuthorizerSourcesV1,ReadonlyMap<string,Readonly<Record<string,string>>>>();
/** 🛠️ Selects what a capture derives beyond the compiler toolkit: installed third-party packages and computed-site roots for command closures. */
export interface NxAuthorizerCaptureOptionsV1{readonly installed?:boolean;readonly roots?:ReadonlyMap<string,readonly string[]>;readonly memo?:Readonly<{file:string;served?:(served:"memo"|"capture")=>void}>}
/** 🧷️ Returns only the byte census issued by an actual no-follow Source capture. */
export function nxAuthorizerCapturedBytesV1(source:NxAuthorizerSourcesV1):ReadonlyMap<string,Uint8Array>{const bytes=capturedBytes.get(source);if(!bytes)throw Error("Original captured authorizer bytes required");return new Map([...bytes].map(([path,value])=>[path,new Uint8Array(value)]));}
/** 📦️ Installed third-party tool packages that stay outside the captured closure and load natively from the pinned installation. */
export const nxAuthorizerInstalledPackagesV1=Object.freeze(["typescript"]);
const runtimeDataCompanionPathsV1=new Set(authorizerSchema.$defs.RuntimeDataCompanions.const.flatMap(row=>row.paths));
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
/** 🧭️ Remembers the identity of every directory one capture reads below, so the ancestors of a thousand files and of every listed entry are looked at once and once more when the capture ends instead of twice per file and several times per entry (93,000 `lstat` calls for 1,400 files become about 7,000). */
export interface NxAuthorizerAncestryV1{readonly known:Map<string,Promise<Readonly<{device:bigint;inode:bigint}>>>;readonly absent:Set<string>}
/** 📒️ Opens the empty ancestry ledger of one capture. */
export function createNxAuthorizerAncestryV1():NxAuthorizerAncestryV1{return{known:new Map(),absent:new Set()};}
async function directoryIdentityV1(path:string):Promise<Readonly<{device:bigint;inode:bigint}>>{
 const status=await lstat(path,{bigint:true});if(!status.isDirectory()||status.isSymbolicLink())throw refuseBootstrapV1("refused","tools.authorizer-directory-linked",{path});
 return Object.freeze({device:status.dev,inode:status.ino});
}
function admitDirectoryV1(ledger:NxAuthorizerAncestryV1,path:string):Promise<Readonly<{device:bigint;inode:bigint}>>{
 let known=ledger.known.get(path);if(!known){known=directoryIdentityV1(path);ledger.known.set(path,known);}
 return known;
}
/** 🔁️ Looks once more at every directory the capture relied on and refuses when any of them is no longer the directory it first was. */
export async function verifyNxAuthorizerAncestryV1(ledger:NxAuthorizerAncestryV1,signal:AbortSignal):Promise<void>{
 const rows=[...ledger.known];
 for(let offset=0;offset<rows.length;offset+=32){signal.throwIfAborted();await Promise.all(rows.slice(offset,offset+32).map(async([path,identity])=>{let first;try{first=await identity;}catch{return;}const now=await directoryIdentityV1(path).catch(()=>undefined);if(!now||now.device!==first.device||now.inode!==first.inode)throw refuseBootstrapV1("stale","tools.authorizer-ancestry-changed");}));}
 signal.throwIfAborted();
}
/** 🔒️ Reads actual regular source bytes without following substituted ancestry; with a ledger the ancestors are admitted once per capture and checked again when the capture ends. */
export async function readNxAuthorizerSourceV1(root:string,path:string,signal:AbortSignal,ledger?:NxAuthorizerAncestryV1):Promise<Uint8Array>{
 signal.throwIfAborted();const absolute=sourcePath(root,path),parents=[root];let current=root;
 for(const part of path.split("/").slice(0,-1)){current=join(current,part);parents.push(current);}
 const ancestry=await Promise.all(parents.map(async path=>{if(ledger){await admitDirectoryV1(ledger,path);return undefined;}const status=await lstat(path,{bigint:true});if(!status.isDirectory()||status.isSymbolicLink())throw Error("Authorizer source requires no-follow directories");return {path,device:status.dev,inode:status.ino};}));
 const before=await lstat(absolute,{bigint:true});signal.throwIfAborted();if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size>4_194_304n)throw Error("Authorizer requires bounded original regular source");const fd=await open(absolute,constants.O_RDONLY|(constants.O_NOFOLLOW??0));let failed=false,failure:unknown,bytes:Uint8Array|undefined;
 try{signal.throwIfAborted();const original=await fd.stat({bigint:true});signal.throwIfAborted();const stamp=(row:typeof original)=>[row.dev,row.ino,row.size,row.nlink,row.mtimeNs,row.ctimeNs].join(":");if(!original.isFile()||stamp(original)!==stamp(before))throw Error("Original authorizer descriptor required");const buffer=Buffer.alloc(Number(original.size));for(let offset=0;offset<buffer.length;){const row=await fd.read(buffer,offset,buffer.length-offset,offset);signal.throwIfAborted();if(!row.bytesRead)throw Error("Short authorizer Source read");offset+=row.bytesRead;}const tail=await fd.read(Buffer.alloc(1),0,1,buffer.length);signal.throwIfAborted();const after=await fd.stat({bigint:true});signal.throwIfAborted();const named=await lstat(absolute,{bigint:true});signal.throwIfAborted();if(tail.bytesRead||!named.isFile()||named.isSymbolicLink()||stamp(original)!==stamp(after)||stamp(original)!==stamp(named))throw Error("Authorizer source changed during capture");for(const row of ancestry){if(!row)continue;const status=await lstat(row.path,{bigint:true});signal.throwIfAborted();if(!status.isDirectory()||status.isSymbolicLink()||status.dev!==row.device||status.ino!==row.inode)throw refuseBootstrapV1("stale","tools.authorizer-ancestry-changed");}bytes=buffer;}catch(error){failed=true;failure=error;}try{await fd.close();}catch(error){if(!failed){failed=true;failure=error;}else try{console.error("[authorizer] Source descriptor close failed after primary",error);}catch{}}if(failed)throw failure;signal.throwIfAborted();return bytes!;
}
interface RuntimeDataDiscoveryV1{readonly directories:Set<string>;readonly results:Set<string>;readonly observed?:NxAuthorizerObservationV1[];readonly ancestry?:NxAuthorizerAncestryV1;entries:number;bytes:number}
const discoveryPolicy=authorizerSchema.$defs.RuntimeDataDiscoveryPolicy.const;
function discoveryStampV1(value:Awaited<ReturnType<typeof lstat>>):string{return["dev","ino","mode","nlink","size","mtimeNs","ctimeNs"].map(key=>String(value[key as keyof typeof value])).join(":");}
async function discoveryAncestorsV1(root:string,path:string,signal:AbortSignal,ledger?:NxAuthorizerAncestryV1):Promise<readonly{path:string;stamp:string;device:bigint;inode:bigint}[]>{
 sourcePath(root,path);const rows=[],parts=path.split("/");let target=root;for(let index=-1;index<parts.length;index++){signal.throwIfAborted();if(index>=0)target=join(target,parts[index]!);if(target.length>256)throw Error("Bounded authorizer DATA ancestry required");if(ledger&&index<parts.length-1){await admitDirectoryV1(ledger,target);signal.throwIfAborted();continue;}const facts=await lstat(target,{bigint:true});signal.throwIfAborted();if(!facts.isDirectory()||facts.isSymbolicLink())throw Error("No-follow authorizer DATA directory required");rows.push({path:target,stamp:discoveryStampV1(facts),device:facts.dev,inode:facts.ino});}return rows;
}
async function discoveryDirectoryHoldsV1(rows:readonly{path:string;stamp:string;device:bigint;inode:bigint}[],signal:AbortSignal):Promise<void>{for(let index=0;index<rows.length;index++){const row=rows[index]!;signal.throwIfAborted();const facts=await lstat(row.path,{bigint:true});signal.throwIfAborted();if(!facts.isDirectory()||facts.isSymbolicLink()||facts.dev!==row.device||facts.ino!==row.inode||index===rows.length-1&&discoveryStampV1(facts)!==row.stamp)throw Error("Authorizer DATA directory changed");}}
/** 🔣️ Discovers bounded module-owned JSON DATA without granting executable import authority. */
async function runtimeDataDiscoveryV1(root:string,path:string,source:string,signal:AbortSignal,state:RuntimeDataDiscoveryV1):Promise<string[]>{
 sourcePath(root,path);signal.throwIfAborted();if(!/\.[cm]?[jt]sx?$/u.test(path))return[];const rows=new Set<string>(),directory=dirname(path)==="."?"":dirname(path)+"/",add=(path:string,facts:Awaited<ReturnType<typeof lstat>>)=>{if(!facts.isFile()||facts.isSymbolicLink()||facts.size<0||facts.size>4_194_304n)throw Error("Bounded ordinary authorizer JSON DATA required");if(!state.results.has(path)){if(state.results.size>=discoveryPolicy.maximumResults||state.bytes+Number(facts.size)>discoveryPolicy.maximumDeclaredBytes)throw Error("Authorizer DATA result capacity exceeded");state.results.add(path);state.bytes+=Number(facts.size);}rows.add(path);},pending=[{path:directory+"🧬️schema",depth:0}];
 while(pending.length){signal.throwIfAborted();const current=pending.pop()!;sourcePath(root,current.path);if(current.depth>discoveryPolicy.maximumDepth)throw Error("Authorizer DATA directory depth exceeded");if(state.directories.has(current.path))continue;if(state.ancestry?.absent.has(current.path))continue;let ancestry;try{ancestry=await discoveryAncestorsV1(root,current.path,signal,state.ancestry);}catch(error){signal.throwIfAborted();if((error as NodeJS.ErrnoException).code==="ENOENT"){state.ancestry?.absent.add(current.path);state.observed?.push({kind:"absent",path:current.path});continue;}throw error;}if(state.directories.size>=discoveryPolicy.maximumDirectories)throw Error("Authorizer DATA directory capacity exceeded");state.directories.add(current.path);signal.throwIfAborted();const stream=await opendir(sourcePath(root,current.path),{bufferSize:1}),seen:(readonly[string,"d"|"f"])[]=[];let failed=false,primary:unknown;
  try{signal.throwIfAborted();await discoveryDirectoryHoldsV1(ancestry,signal);for(;;){signal.throwIfAborted();const entry=await stream.read();signal.throwIfAborted();await discoveryDirectoryHoldsV1(ancestry,signal);if(!entry)break;if(state.entries>=discoveryPolicy.maximumEntries)throw Error("Authorizer DATA entry capacity exceeded");state.entries++;const child=current.path+"/"+entry.name;sourcePath(root,child);await discoveryDirectoryHoldsV1(ancestry,signal);const facts=await lstat(sourcePath(root,child),{bigint:true});signal.throwIfAborted();await discoveryDirectoryHoldsV1(ancestry,signal);if(facts.isSymbolicLink())throw Error("No-follow authorizer DATA entry required");seen.push([entry.name,facts.isDirectory()?"d":"f"]);if(facts.isDirectory()){if(pending.length+state.directories.size>=discoveryPolicy.maximumDirectories)throw Error("Authorizer DATA directory capacity exceeded");pending.push({path:child,depth:current.depth+1});}else if(facts.isFile()){if(extname(child)===".json")add(child,facts);}else throw Error("Ordinary authorizer DATA entry required");}await discoveryDirectoryHoldsV1(ancestry,signal);}catch(error){failed=true;primary=error;}try{await stream.close();signal.throwIfAborted();}catch(error){if(!failed){failed=true;primary=error;}}if(failed)throw primary;await discoveryDirectoryHoldsV1(ancestry,signal);signal.throwIfAborted();state.observed?.push({kind:"directory",path:current.path,entries:seen.sort((a,b)=>compare(a[0],b[0]))});
 }
 for(const match of source.matchAll(/(["'\x60])(\.{1,2}\/[^"'\x60\\\r\n$]+\.json)\1/gu)){signal.throwIfAborted();const child=relative(root,resolve(root,directory,match[2]!)).replaceAll("\\","/");if(child.split("/").some(part=>part===".."||part==="node_modules"||part===".cache"))continue;sourcePath(root,child);let ancestry;try{const parent=dirname(child);ancestry=parent==="."?[]:await discoveryAncestorsV1(root,parent,signal,state.ancestry);}catch(error){signal.throwIfAborted();if((error as NodeJS.ErrnoException).code==="ENOENT"){state.observed?.push({kind:"absent",path:child});continue;}throw error;}if(dirname(child)==="."){signal.throwIfAborted();const facts=await lstat(root,{bigint:true});signal.throwIfAborted();if(!facts.isDirectory()||facts.isSymbolicLink())throw Error("No-follow authorizer DATA root required");ancestry=[{path:root,stamp:discoveryStampV1(facts),device:facts.dev,inode:facts.ino}];}let facts;try{signal.throwIfAborted();facts=await lstat(sourcePath(root,child),{bigint:true});signal.throwIfAborted();}catch(error){signal.throwIfAborted();if((error as NodeJS.ErrnoException).code==="ENOENT"){state.observed?.push({kind:"absent",path:child});continue;}throw error;}await discoveryDirectoryHoldsV1(ancestry,signal);add(child,facts);}
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
/** 💾️ Remembers a finished capture together with the filesystem facts it depended on, so that the next process proves it instead of repeating it; a memo that cannot be written costs only that next capture. */
    async function rememberCapturedV1(root:string,entry:string,installed:boolean,file:string,result:NxAuthorizerSourcesV1,marks:ReadonlyMap<string,string>,observations:readonly NxAuthorizerObservationV1[]):Promise<void>{
     const directories=await nxAuthorizerMemoDirectoriesV1(root,result.manifest.sources.map(row=>row.path));
     if(!directories)return;
     const imports=nxAuthorizerImportProjectionV1(root,nxAuthorizerCapturedBytesV1(result),result.manifest.resolutions,capturedInstalled.get(result));
     capturedImports.set(result,imports);
     await writeNxAuthorizerMemoV1(file,{version:1,installed,implementation:nxAuthorizerMemoImplementationV1(),root,entry,directories,observations,manifest:result.manifest,marks:Object.fromEntries([...marks].sort(([left],[right])=>compare(left,right))),imports:Object.fromEntries(imports)});
    }
    /** ♻️ Serves a capture from its memo when the present tree proves every fact the memo rests on: file digests, ancestor identities, remembered absences and listings and each package resolution; otherwise nothing. */
    async function replayCapturedV1(workspace:string,entry:string,signal:AbortSignal,installed:boolean,file:string):Promise<NxAuthorizerSourcesV1|undefined>{
     const root=resolve(workspace),memo=await readNxAuthorizerMemoV1(file);
     if(!memo||memo.entry!==entry||memo.root!==root||memo.installed!==installed)return undefined;
     const contents=await replayNxAuthorizerMemoV1(root,memo,signal);
     if(!contents)return undefined;
     const versions=new Map<string,string>();
     for(const row of memo.observations){
      if(row.kind!=="package")continue;
      let candidate:string|null=null,mark:string|null=null;
      try{
       const located=await runtimeResolveV1(row.specifier,join(root,row.from));signal.throwIfAborted();
       const actual=await realpath(located),physicalRoot=await realpath(root);signal.throwIfAborted();
       candidate=relative(physicalRoot,actual).replaceAll("\\","/");
       if(installed&&installedPathV1(candidate))mark=await installedMarkV1(root,candidate,row.specifier,versions,signal);
      }catch(error){signal.throwIfAborted();if(row.candidate===null)continue;return undefined;}
      if(row.candidate===null||candidate!==row.candidate||mark!==row.mark)return undefined;
     }
     const manifest:NxAuthorizerSourceManifestV1=Object.freeze({version:1,mode:"release",entry:memo.manifest.entry,resolutions:Object.freeze(memo.manifest.resolutions.map(row=>Object.freeze({specifier:row.specifier,path:row.path}))),sources:Object.freeze(memo.manifest.sources.map(row=>Object.freeze({path:row.path,sha256:row.sha256})))});
     const result=Object.freeze({manifest,digest:hash(JSON.stringify(manifest)+"\n")});
     capturedBytes.set(result,contents);
     if(installed)capturedInstalled.set(result,Object.freeze({installed:Object.freeze({...memo.marks}),roots:computedRoots}));
     capturedImports.set(result,new Map(Object.entries(memo.imports).map(([path,map])=>[path,Object.freeze({...map})] as const)));
     return result;
    }
    export async function captureNxAuthorizerSourcesV1(workspace:string,entry:string,signal:AbortSignal,options:NxAuthorizerCaptureOptionsV1={}):Promise<NxAuthorizerSourcesV1>{
 const remembered=options.memo!==undefined&&options.roots===undefined?await replayCapturedV1(workspace,entry,signal,options.installed===true,options.memo.file):undefined;
 if(remembered){options.memo!.served?.("memo");return remembered;}
 const installed=options.installed===true,roots=options.roots??computedRoots,root=resolve(workspace),files=new Map<string,string>(),contents=new Map<string,Uint8Array>(),pending=[entry],resolutions=new Map<string,string>(),marks=new Map<string,string>(),versions=new Map<string,string>(),observed:NxAuthorizerObservationV1[]=[],ancestry=createNxAuthorizerAncestryV1(),discovery:RuntimeDataDiscoveryV1={directories:new Set(),results:new Set(),observed,ancestry,entries:0,bytes:0};let total=0;
 const origins=new Map<string,{kind:string;importer:string|null;specifier?:string}>([[entry,{kind:"entry",importer:null}]]);
 const capture=async(path:string)=>{if(contents.has(path))return contents.get(path)!;let bytes:Uint8Array;try{bytes=await readNxAuthorizerSourceV1(root,path,signal,ancestry);}catch(error){try{const detail=error as NodeJS.ErrnoException,text=JSON.stringify({root,entry,installed,operation:"required-source-read",request:path,origin:origins.get(path)??null,code:detail?.code??null,failedPath:detail?.path??null},(_key,value)=>typeof value==="string"?value.slice(0,256):value);if(Buffer.byteLength(text)<=8192)console.error("[DEBUG] Authorizer required Source read failed",text);}catch{}throw error;}signal.throwIfAborted();files.set(path,hash(bytes));contents.set(path,new Uint8Array(bytes));total+=bytes.byteLength;if(files.size>maximumSourceFiles||total>16_777_216)throw Error(`Authorizer source closure capacity exceeded: files=${files.size}/${maximumSourceFiles}, bytes=${total}/16777216`);return bytes;};
 while(pending.length){signal.throwIfAborted();const path=pending.pop()!;if(files.has(path))continue;const bytes=await capture(path);signal.throwIfAborted();
  for(const companion of [...(runtimeDataCompanions.get(path)??[]),...(options.roots?.get(path)??[])]){sourcePath(root,companion);origins.set(companion,{kind:"declared-companion",importer:path});pending.push(companion);}
  if(extname(path)===".json"){JSON.parse(Buffer.from(bytes).toString("utf8"));continue;}
  if(runtimeDataCompanionPathsV1.has(path))continue;
  const source=Buffer.from(bytes).toString("utf8").replace(/^#![^\r\n]*/u,"");
  for(const companion of await runtimeDataDiscoveryV1(root,path,source,signal,discovery)){origins.set(companion,{kind:"discovered-data",importer:path});pending.push(companion);}
  for(const row of scannedImportRowsV1(path,source,installed)){
   const specifier=scannedImport(row.path),kind=closureSpecifierKindV1(specifier);
   if(kind==="builtin"||installed&&(kind==="absolute"||kind==="scheme")||!installed&&external.has(specifier))continue;
   let child:string;
   if(kind==="relative"){child=relative(root,resolve(dirname(join(root,path)),specifier)).replaceAll("\\","/");if(row.lenient){try{await lstat(sourcePath(root,child));}catch(error){signal.throwIfAborted();observed.push({kind:"absent",path:child});continue;}signal.throwIfAborted();}}
   else{const earlier=resolutions.get(specifier);if(earlier!==undefined)child=earlier;else if(marks.has(specifier))continue;else{
    try{const rootBytes=await capture("package.json"),located=await runtimeResolveV1(specifier,dirname(join(root,path)));signal.throwIfAborted();const actual=await realpath(located),physicalRoot=await realpath(root);signal.throwIfAborted();const candidate=relative(physicalRoot,actual).replaceAll("\\","/");
     if(installed&&installedPathV1(candidate)){const mark=await installedMarkV1(root,candidate,specifier,versions,signal);marks.set(specifier,mark);observed.push({kind:"package",specifier,from:dirname(path),candidate,mark});continue;}
     sourcePath(root,candidate);const manifest=nxAuthorizerWorkspaceCandidateV1(rootBytes,candidate);await capture(manifest);signal.throwIfAborted();child=resolveNxAuthorizerWorkspaceV1(nxAuthorizerWorkspaceIndexV1(contents),specifier);if(child!==candidate)throw Error("Located package differs from its captured exact export");resolutions.set(specifier,child);observed.push({kind:"package",specifier,from:dirname(path),candidate:child,mark:null});
    }catch(error){signal.throwIfAborted();if(row.lenient){observed.push({kind:"package",specifier,from:dirname(path),candidate:null,mark:null});continue;}throw error instanceof Error?new Error(error.message+" (importer chain "+viaChainV1(viaV1,path)+", specifier "+specifier+")",{cause:error}):error;}
   }}
   sourcePath(root,child);if(!viaV1.has(child))viaV1.set(child,path);origins.set(child,{kind:"import",importer:path,specifier});pending.push(child);
  }
 }
 await verifyNxAuthorizerAncestryV1(ancestry,signal);
 const manifest:NxAuthorizerSourceManifestV1=Object.freeze({version:1,mode:"release",entry,resolutions:Object.freeze([...resolutions].sort(([left],[right])=>compare(left,right)).map(([specifier,path])=>Object.freeze({specifier,path}))),sources:Object.freeze([...files].sort(([left],[right])=>compare(left,right)).map(([path,sha256])=>Object.freeze({path,sha256})))});
 const result=Object.freeze({manifest,digest:hash(JSON.stringify(manifest)+"\n")});capturedBytes.set(result,contents);if(installed)capturedInstalled.set(result,Object.freeze({installed:Object.freeze(Object.fromEntries([...marks].sort(([left],[right])=>compare(left,right)))),roots}));
 if(options.memo!==undefined&&options.roots===undefined){options.memo.served?.("capture");await rememberCapturedV1(root,entry,installed,options.memo.file,result,marks,observed);}
 return result;
}
/** 🕸️ Resolves only executable compiler imports against the same privately captured byte census; with installed markers it also resolves third-party packages and computed-site roots. */
export function nxAuthorizerImportProjectionV1(workspace:string,bytes:ReadonlyMap<string,Uint8Array>,resolutions:readonly NxAuthorizerResolutionV1[],derived?:Readonly<{installed:Readonly<Record<string,string>>;roots:ReadonlyMap<string,readonly string[]>}>):ReadonlyMap<string,Readonly<Record<string,string>>>{
 const root=resolve(workspace),packages=new Map(resolutions.map(row=>[row.specifier,row.path])),result=new Map<string,Readonly<Record<string,string>>>();
 for(const[path,value]of bytes){const imports:Record<string,string>={};if(extname(path)!==".json"&&!runtimeDataCompanionPathsV1.has(path)){const text=Buffer.from(value).toString("utf8").replace(/^#![^\r\n]*/u,"");
  for(const row of scannedImportRowsV1(path,text,derived!==undefined)){const specifier=scannedImport(row.path),kind=closureSpecifierKindV1(specifier);if(kind==="builtin"||derived!==undefined&&(kind==="absolute"||kind==="scheme")||derived===undefined&&external.has(specifier))continue;const target=kind==="relative"?relative(root,resolve(dirname(join(root,path)),specifier)).replaceAll("\\","/"):packages.get(specifier)??derived?.installed[specifier];if(!target||!target.startsWith(closureTargetSchemesV1.installed)&&!bytes.has(target)){if(row.lenient&&derived!==undefined)continue;throw Error("Complete immutable compiler import required: "+specifier);}if(!target.startsWith(closureTargetSchemesV1.installed))sourcePath(root,target);imports[specifier]=target;}
  if(derived!==undefined)for(const target of derived.roots.get(path)??[])if(bytes.has(target)){sourcePath(root,target);imports[closureTargetSchemesV1.root+target]=target;}}
  result.set(path,Object.freeze(imports));}
 return result;}
/** 🔗️ Reads import edges only from an actual privately issued compiler capture. */
export function nxAuthorizerCapturedImportsV1(workspace:string,source:NxAuthorizerSourcesV1):ReadonlyMap<string,Readonly<Record<string,string>>>{return capturedImports.get(source)??nxAuthorizerImportProjectionV1(workspace,nxAuthorizerCapturedBytesV1(source),source.manifest.resolutions,capturedInstalled.get(source));}
/** ✅️ Re-enumerates the actual runtime closure before publishing compiled authorizers. */
export async function nxAuthorizerSourcesHoldV1(root:string,original:NxAuthorizerSourcesV1,signal:AbortSignal):Promise<boolean>{
 try{return (await captureNxAuthorizerSourcesV1(root,original.manifest.entry,signal,{installed:capturedInstalled.has(original),roots:capturedInstalled.get(original)?.roots})).digest===original.digest;}catch(error){signal.throwIfAborted();return false;}
}
