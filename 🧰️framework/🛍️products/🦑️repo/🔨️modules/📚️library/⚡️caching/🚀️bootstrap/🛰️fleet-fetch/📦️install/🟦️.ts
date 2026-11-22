import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {createHash,randomBytes} from "node:crypto";
import {createReadStream,createWriteStream,readdirSync,type BigIntStats} from "node:fs";
import {lstat,mkdir,readdir,readFile,readlink,rename,rm,stat,symlink,utimes,writeFile} from "node:fs/promises";
import {hostname} from "node:os";
import {dirname,isAbsolute,join,posix} from "node:path";
import {spawn} from "node:child_process";
import {createGunzip,createGzip,gunzipSync,gzipSync} from "node:zlib";
import {Writable} from "node:stream";
import {pipeline} from "node:stream/promises";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {FLEET_FETCH_POLICY_V1 as policy,type FleetFetchContextV1} from "../🟦️.ts";
import {createFleetRegistryV1,readFleetMachineV1,signFleetDocumentV1,verifyFleetDocumentV1} from "../../../../🏗️build/🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {captureFleetInstallHardlinksV1,fleetInstallPathFitsV1,planFleetInstallHardlinkRestoreV1,type FleetInstallHardlinkV1} from "./🔗️hardlinks/🟦️.ts";

/** 🔒️ The tarball integrities a lockfile pins: `name@version integrity` for every entry of the lockfile that has one, sorted, with their count and digest. */
export function fleetInstallLockV1(text:string):Readonly<{packages:number;sha256:string}>{
 const lines:string[]=[];
 const parsed=JSON.parse(text.replace(/,(\s*[}\]])/gu,"$1")) as {packages?:Record<string,unknown[]>};
 for(const row of Object.values(parsed.packages??{})){
  const spec=row[0],integrity=[...row].reverse().find(item=>typeof item==="string"&&/^sha(512|256|1)-/u.test(item));
  if(typeof spec==="string"&&typeof integrity==="string")lines.push(`${spec} ${integrity}`);
 }
 lines.sort();
 return {packages:lines.length,sha256:createHash("sha256").update(lines.join("\n")).digest("hex")};
}
const lockOf=async(directory:string,file:string):Promise<Readonly<{packages:number;sha256:string}>|undefined>=>{try{return fleetInstallLockV1(await readFile(join(directory,file),"utf8"));}catch{return undefined;}};
const installIndexVersion=schema.definitions.InstallIndex.properties.version.const;
const signedText=(manifest:Omit<FleetInstallManifestV1,"publisher">,origin:string):string=>JSON.stringify([installIndexVersion,origin,manifest.key,manifest.index.sha256,manifest.fingerprint,manifest.shape.sha256,manifest.lock.sha256,manifest.projectionSha256,manifest.installed,manifest.roots,manifest.storeRoot]);

/** 🧬️ What a restored tree must equal: the counts and digest the installer's own scan reports. */
export type FleetInstallShapeV1=Readonly<{entries:number;directories:number;files:number;links:number;sha256:string}>;
/** 🏷️ The runtime an install is a function of, next to the lock projection. */
export type FleetInstallRuntimeV1=Readonly<{bun:string;platform:"darwin"|"linux"|"win32"|"posix"|"any";arch:string;recipeSha256:string}>;
/** 🖥️ The facts of a machine that decide its class; the libc of Linux is told by the musl loader's presence in `/lib`. */
export type FleetInstallHostV1=Readonly<{platform:string;arch:string;libc?:"gnu"|"musl"}>;
export function fleetInstallHostV1(platform:string=process.platform,arch:string=process.arch,list:()=>readonly string[]=()=>{try{return readdirSync("/lib");}catch{return [];}}):FleetInstallHostV1{
 return platform==="linux"?{platform,arch,libc:list().some(name=>name.startsWith("ld-musl"))?"musl":"gnu"}:{platform,arch};
}
/** 🐧️ The runtime of an install key on this host: native optional packages differ by libc, so a Linux machine keys on `x64-gnu` or `x64-musl`; macOS and Windows keep the plain architecture, which keeps their keys. */
export function fleetInstallRuntimeV1(base:FleetInstallRuntimeV1,host:FleetInstallHostV1=fleetInstallHostV1(base.platform,base.arch)):FleetInstallRuntimeV1{
 return host.libc===undefined?base:{...base,arch:`${base.arch}-${host.libc}`};
}

/** 📋️ The published description of one installed state. */
export type FleetInstallManifestV1=Readonly<{version:1;key:string;projectionSha256:string;runtime:FleetInstallRuntimeV1;installed:string;shape:FleetInstallShapeV1;storeRoot:string;fingerprint:string;roots:readonly string[];index:Readonly<{sha256:string;bytes:number;units:number;links:number;packedBytes:number}>;lock:Readonly<{packages:number;sha256:string}>;publisher:Readonly<{origin:string;signature:string}>;createdAtMs:number}>;

/** 📣️ A progress or outcome event; the caller turns it into its own report. */
export type FleetInstallEventV1=Readonly<{stage:"restore"|"publish"|"wait"|"claim"|"skip";detail:string;bytes?:number}>;

/** 🌳️ Everything one install state needs from its host. */
export type FleetInstallRequestV1=Readonly<{
 context:FleetFetchContextV1;
 projectionSha256:string;
 runtime:FleetInstallRuntimeV1;
 directory:string;
 lockFile:string;
 storeRoot:string;
 roots:readonly string[];
 check:()=>void;
 scan:()=>Promise<FleetInstallShapeV1>;
 remove:(path:string)=>Promise<void>;
 installed:()=>string|undefined;
 event?:(event:FleetInstallEventV1)=>void;
 owner?:string;
 space?:string;
 lock?:Readonly<{packages:number;sha256:string}>;
 keep?:number;
}>;

/** 📚️ What the index blob of a pack lists: the directories to create, the links to recreate and the unit blobs to extract. */
export type FleetInstallIndexV1=Readonly<{version:typeof installIndexVersion;directories:readonly string[];links:readonly Readonly<{path:string;target:string}>[];hardlinks:readonly FleetInstallHardlinkV1[];units:readonly Readonly<{path:string;sha256:string;bytes:number;maximumPathUnits:number}>[]}>;
const KEY=/^inst-[a-f0-9]{64}$/u;
const guard=(value:unknown,message:string):void=>{const errors=validateJsonSchemaSubset({$ref:"#/definitions/InstallManifest"},value,schema);if(errors.length)throw Error(`${message}: ${errors[0]}`);};
const guardIndex=(value:unknown):void=>{const errors=validateJsonSchemaSubset({$ref:"#/definitions/InstallIndex"},value,schema);if(errors.length)throw Error(`Closed install index required: ${errors[0]}`);};

/** 🔑️ The key of an install state: a digest of the lock projection and everything about the runtime that changes the installed bytes (Bun version, platform, architecture, producer recipe). */
export function fleetInstallKeyV1(projectionSha256:string,runtime:FleetInstallRuntimeV1):string{
 return policy.installKeyPrefix+createHash("sha256").update(JSON.stringify([installIndexVersion,projectionSha256,runtime.bun,runtime.platform,runtime.arch,runtime.recipeSha256])).digest("hex");
}

const directoryOf=(context:FleetFetchContextV1,space?:string):string|undefined=>context.storeRoot===undefined?undefined:space===undefined?join(context.storeRoot,...policy.installDirectory):join(context.storeRoot,...policy.installDirectory.slice(0,-1),space);

/** 📖️ Reads and validates the manifest of a key from the group store; `undefined` when absent or invalid. */
export async function fleetInstallManifestV1(context:FleetFetchContextV1,key:string,space?:string):Promise<FleetInstallManifestV1|undefined>{
 const directory=directoryOf(context,space);if(directory===undefined||!KEY.test(key))return undefined;
 try{
  const facts=await stat(join(directory,key+policy.installManifestSuffix));if(!facts.isFile()||facts.size>65536)return undefined;
  const value=JSON.parse(await readFile(join(directory,key+policy.installManifestSuffix),"utf8")) as FleetInstallManifestV1;
  guard(value,"Closed install manifest required");return value.key===key?value:undefined;
 }catch{return undefined;}
}

const runTar=(argv:readonly string[],cwd:string,check:()=>void,stdio:["pipe"|"ignore","pipe"|"ignore","pipe"]):ReturnType<typeof spawn>=>{
 const child=spawn("tar",argv as string[],{cwd,stdio,env:{...process.env,COPYFILE_DISABLE:"1"},windowsHide:true}),poll=setInterval(()=>{try{check();}catch{child.kill();}},250);
 child.once("close",()=>clearInterval(poll));child.once("error",()=>clearInterval(poll));
 return child;
};
const closed=(child:ReturnType<typeof spawn>):Promise<number>=>new Promise((resolve,reject)=>{child.once("error",reject);child.once("close",code=>resolve(code??-1));});

/** 🧭️ Checks whether a pack can reproduce the exact raw link target in another worktree. */
function fleetInstallLinkRelocatableV1(target:string,relative:string,storeRoot:string):boolean{
 if(/^[a-zA-Z]:[\\/]/u.test(target))return false;
 if(isAbsolute(target))return storeRoot!==""&&(target===storeRoot||target.startsWith(storeRoot.replace(/[\\/]+$/u,"")+"/"));
 return !posix.normalize(posix.join(posix.dirname(relative),target.replaceAll("\\","/"))).startsWith("..");
}

/** 🚫️ Names the first symbolic link below the roots that cannot be moved to another worktree path: an absolute target outside `storeRoot` (the prefix a restore rewrites to the local one; the caller passes the worktree root, which holds both the dependency store and the workspace packages the generation links to) or a relative target that escapes the roots. */
export async function firstNonRelocatableLinkV1(directory:string,roots:readonly string[],check:()=>void,storeRoot:string=""):Promise<string|undefined>{
 const walk=async(path:string,relative:string):Promise<string|undefined>=>{
  check();
  let names:string[];try{names=await readdir(path);}catch{return undefined;}
  for(const name of names){
   const child=join(path,name),facts=await lstat(child).catch(()=>undefined),here=posix.join(relative,name);
   if(!facts)continue;
   if(facts.isSymbolicLink()){if(!fleetInstallLinkRelocatableV1(await readlink(child).catch(()=>"/"),here,storeRoot))return here;}
   else if(facts.isDirectory()){const found=await walk(child,here);if(found)return found;}
  }
  return undefined;
 };
 for(const root of roots){const path=join(directory,...root.split("/")),facts=await lstat(path).catch(()=>undefined);if(!facts)continue;if(facts.isSymbolicLink())return root;const found=await walk(path,root);if(found)return found;}
 return undefined;
}

/** 🪟️ The text of a link target in the one spelling the pack compares and rewrites: on Windows a junction reads back with backslashes and often the `\\?\` prefix, which would never match the store prefix written with `/`; elsewhere the text is kept. */
export function fleetInstallLinkTextV1(text:string,platform:NodeJS.Platform=process.platform):string{
 return platform==="win32"?text.replace(/^\\\\\?\\/u,"").replaceAll("\\","/"):text;
}

/** 🔬️ Digests what a restore must reproduce independently of the checkout path. */
export async function fleetInstallFingerprintV1(directory:string,roots:readonly string[],storeRoot:string,check:()=>void,platform:NodeJS.Platform=process.platform):Promise<string>{
 return (await inspectFleetInstallMetadataV1(directory,roots,storeRoot,check,platform,false)).fingerprint!;
}

/** 🧾️ Reads the portable fingerprint and, for publication, verifies the same links can be relocated. */
async function inspectFleetInstallMetadataV1(directory:string,roots:readonly string[],storeRoot:string,check:()=>void,platform:NodeJS.Platform,requireRelocatable:boolean):Promise<{fingerprint:string;bad?:never}|{bad:string;fingerprint?:never}>{
 const rows:string[]=[],prefix=fleetInstallLinkTextV1(storeRoot,platform).replace(/[\\/]+$/u,"");
 const walk=async(path:string,relative:string):Promise<string|undefined>=>{
  check();
  for(const name of (await readdir(path)).sort()){
   const child=join(path,name),facts=await lstat(child),here=posix.join(relative,name);
   if(facts.isSymbolicLink()){
    const raw=await readlink(child);
    if(requireRelocatable&&!fleetInstallLinkRelocatableV1(raw,here,storeRoot))return here;
    const target=fleetInstallLinkTextV1(raw,platform);
    rows.push(`l ${here} ${prefix!==""&&target.startsWith(prefix+"/")?"<store>"+target.slice(prefix.length):target}`);
   }else if(facts.isDirectory()){
    rows.push(`d ${here}`);
    const bad=await walk(child,here);if(bad!==undefined)return bad;
   }else rows.push(`f ${here} ${facts.size} ${facts.mode&0o111?1:0}`);
  }
 };
 for(const root of roots){
  const path=join(directory,...root.split("/")),facts=await lstat(path).catch(()=>undefined);
  if(!facts)continue;
  if(requireRelocatable&&facts.isSymbolicLink())return {bad:root};
  rows.push(`r ${root}`);
  const bad=await walk(path,root);if(bad!==undefined)return {bad};
 }
 return {fingerprint:createHash("sha256").update(rows.join("\n")).digest("hex")};
}

/** 🔁️ Points every absolute link below the roots that starts with the publisher's dependency store at the local one. */
export async function rewriteFleetInstallLinksV1(directory:string,roots:readonly string[],from:string,to:string,check:()=>void,platform:NodeJS.Platform=process.platform):Promise<number>{
 const source=fleetInstallLinkTextV1(from,platform).replace(/[\\/]+$/u,""),target=to.replace(/[\\/]+$/u,"");let changed=0;
 if(source===fleetInstallLinkTextV1(target,platform))return 0;
 const walk=async(path:string)=>{
  check();
  for(const name of await readdir(path)){
   const child=join(path,name),facts=await lstat(child);
   if(facts.isSymbolicLink()){const text=fleetInstallLinkTextV1(await readlink(child),platform);if(text.startsWith(source+"/")){const next=target+text.slice(source.length);await rm(child);await symlink(platform==="win32"?next.replaceAll("/","\\"):next,child,platform==="win32"?"junction":"dir");changed++;}}
   else if(facts.isDirectory())await walk(child);
  }
 };
 for(const root of roots){const path=join(directory,...root.split("/"));if(await lstat(path).then(()=>true,()=>false))await walk(path);}
 return changed;
}

/** 🧱️ One unit of a pack: a directory subtree or a single file at the depth where packages live (`node_modules/<name>`, `node_modules/@scope/<name>`), listed pre-order and sorted, with the stat signature that lets a later publish reuse its blob. */
export type PackUnitV1=Readonly<{path:string;entries:readonly string[];signature:string}>;
export type PackBlobV1=Readonly<{sha256:string;bytes:number}>;

export const blobPathOf=(directory:string,sha256:string):string=>join(directory,...policy.installBlobDirectory,sha256.slice(0,2),sha256);
const blobTemporaryDirectory=(directory:string):string=>join(directory,...policy.installBlobDirectory);
const tarOwnerArguments=(platform:NodeJS.Platform):readonly string[]=>platform==="linux"?["--owner=0","--group=0","--numeric-owner"]:["--uid","0","--gid","0","--uname","","--gname",""];

export async function placeBlobV1(directory:string,temporary:string,blob:PackBlobV1):Promise<void>{
 const target=blobPathOf(directory,blob.sha256),have=await stat(target).catch(()=>undefined);
 if(have?.isFile()&&have.size===blob.bytes){await rm(temporary,{force:true});const now=new Date();await utimes(target,now,now).catch(()=>{});return;}
 await mkdir(dirname(target),{recursive:true});await rename(temporary,target);
}

/** 🗺️ Splits the roots of a generation into the pieces a pack stores: directories to create, links to recreate and units to archive. A scope folder (`@scope`) is a directory whose children are units; an empty folder is a directory entry, not a unit. Neither directories (their modification times) nor links (theirs) are archived inside a unit, because either would make identical packages differ: extraction creates the directories, the index lists the links and the empty directories. */
export async function planFleetInstallPackV1(directory:string,roots:readonly string[],check:()=>void,omitFile?:(path:string,facts:BigIntStats)=>boolean):Promise<Readonly<{directories:readonly string[];links:readonly Readonly<{path:string;target:string}>[];units:readonly PackUnitV1[]}>>{
 const directories:string[]=[],links:{path:string;target:string}[]=[],units:PackUnitV1[]=[];
 const checkPath=(relative:string)=>{if(omitFile&&!fleetInstallPathFitsV1(directory,relative))throw Error("Install path exceeds its portable budget");};
 const collect=async(absolute:string,relative:string,entries:string[],lines:string[]):Promise<void>=>{
  check();checkPath(relative);
  const facts=await lstat(absolute,{bigint:true});
  if(facts.isSymbolicLink()){const target=await readlink(absolute);links.push({path:relative,target});lines.push(`l ${relative} ${target}`);return;}
  if(facts.isDirectory()){
   const names=(await readdir(absolute)).sort();
   if(names.length===0){entries.push(relative);lines.push(`d ${relative}`);}
   for(const name of names)await collect(join(absolute,name),`${relative}/${name}`,entries,lines);
   return;
  }
  if(omitFile?.(relative,facts))return;
  entries.push(relative);lines.push(`f ${relative} ${facts.size} ${Math.floor(Number(facts.mtimeMs)/1000)} ${facts.mode&0o777n}`);
 };
 const level=async(absolute:string,relative:string,scoped:boolean):Promise<void>=>{
  for(const name of (await readdir(absolute)).sort()){
   check();
   const child=join(absolute,name),here=`${relative}/${name}`;
   checkPath(here);const facts=await lstat(child);
   if(facts.isSymbolicLink()){links.push({path:here,target:await readlink(child)});continue;}
   if(facts.isDirectory()&&scoped&&name.startsWith("@")){directories.push(here);await level(child,here,false);continue;}
   if(facts.isDirectory()&&(await readdir(child)).length===0){directories.push(here);continue;}
   const entries:string[]=[],lines:string[]=[];await collect(child,here,entries,lines);
   if(entries.length>0)units.push({path:here,entries,signature:createHash("sha256").update(lines.join("\n")).digest("hex")});
  }
 };
 for(const root of roots){
  const absolute=join(directory,...root.split("/")),facts=await lstat(absolute).catch(()=>undefined);
  if(!facts)continue;
  if(facts.isSymbolicLink()){links.push({path:root,target:await readlink(absolute)});continue;}
  directories.push(root);await level(absolute,root,true);
 }
 return {directories,links,units};
}

/** 📦️ Packs one unit into a content-addressed blob below `blobs` (reusing the blob of an earlier pack when the unit's stat signature is in `cache`). */
export async function packFleetUnitV1(request:Pick<FleetInstallRequestV1,"directory"|"check">,blobs:string,unit:PackUnitV1,cache:Map<string,PackBlobV1>):Promise<PackBlobV1>{
 const known=cache.get(unit.signature);
 if(known){const have=await stat(blobPathOf(blobs,known.sha256)).catch(()=>undefined);if(have?.isFile()&&have.size===known.bytes){const now=new Date();await utimes(blobPathOf(blobs,known.sha256),now,now).catch(()=>{});return known;}}
 const scratch=blobTemporaryDirectory(blobs);await mkdir(scratch,{recursive:true});
 const stem=join(scratch,`.${randomBytes(8).toString("hex")}`),listing=stem+".list",temporary=stem+".tmp",hash=createHash("sha256");
 let bytes=0;
 try{
  await writeFile(listing,unit.entries.map(name=>name.startsWith("-")?"./"+name:name).join("\n")+"\n");
  const child=runTar(["-cf","-","--no-xattrs","--no-recursion",...tarOwnerArguments(process.platform),"-C",request.directory,"-T",listing],request.directory,request.check,["ignore","pipe","pipe"]);
  child.stderr?.resume();
  const exit=closed(child);
  const counted=async function*(source:AsyncIterable<Buffer>){for await(const chunk of source){hash.update(chunk);bytes+=chunk.byteLength;yield chunk;}};
  await pipeline(child.stdout!,createGzip({level:1}),counted,createWriteStream(temporary,{flags:"wx",mode:0o644}));
  if(await exit!==0)throw Error("tar refused a unit of the install roots");
  const blob={sha256:hash.digest("hex"),bytes};
  await placeBlobV1(blobs,temporary,blob);
  cache.set(unit.signature,blob);
  return blob;
 }finally{await rm(listing,{force:true}).catch(()=>{});await rm(temporary,{force:true}).catch(()=>{});}
}

export const unitCachePath=(root:string):string=>machinePathV1(root,".cache","f","install-pack-units.json");
export const readUnitCache=async(root:string):Promise<Map<string,PackBlobV1>>=>{
 try{const value=JSON.parse(await readFile(unitCachePath(root),"utf8")) as {version?:number;rows?:Record<string,PackBlobV1>};return new Map(value.version===1?Object.entries(value.rows??{}):[]);}catch{return new Map();}
};
export const writeUnitCache=async(root:string,cache:Map<string,PackBlobV1>):Promise<void>=>{
 try{
  const rows=Object.fromEntries([...cache].slice(-policy.installUnitCacheEntries)),path=unitCachePath(root),temporary=path+`.${randomBytes(4).toString("hex")}.tmp`;
  await mkdir(dirname(path),{recursive:true});await writeFile(temporary,JSON.stringify({version:1,rows}));await rename(temporary,path);
 }catch{}
};

export const poolV1=async<T>(items:readonly T[],width:number,work:(item:T)=>Promise<void>):Promise<void>=>{
 let next=0,failure:unknown;
 await Promise.all(Array.from({length:Math.min(width,items.length)},async()=>{for(;;){if(failure!==undefined)return;const index=next++;if(index>=items.length)return;try{await work(items[index]!);}catch(error){failure??=error;return;}}}));
 if(failure!==undefined)throw failure;
};

/** 📤️ Packs the roots of a complete installed generation into content-addressed unit blobs in the group store (one gzip tar per package, named by its own SHA-256, so a package that several generations share is stored once), then an index blob, then the signed manifest. Blobs are renamed into place before the index and the index before the manifest, so a reader that sees the manifest sees every blob. Units whose stat signature a previous publish of this machine packed are reused without reading them. Refuses a tree with a link that cannot move to another worktree or past the size bound; `false` leaves nothing referenced behind. */
export async function fleetInstallPublishV1(request:FleetInstallRequestV1,replace:boolean=false):Promise<boolean>{
 const {context}=request,directory=directoryOf(context,request.space),key=fleetInstallKeyV1(request.projectionSha256,request.runtime),installed=request.installed();
 if(directory===undefined||installed===undefined||context.root===undefined)return false;
 const lock=request.lock??await lockOf(request.directory,request.lockFile);
 if(lock===undefined){request.event?.({stage:"skip",detail:"no lockfile to bind the pack to"});return false;}
 if(!replace&&await fleetInstallManifestV1(context,key,request.space)){request.event?.({stage:"skip",detail:"published"});return true;}
 const roots=[];for(const root of request.roots){if(!fleetInstallPathFitsV1(request.directory,root))return false;if(await lstat(join(request.directory,...root.split("/"))).then(()=>true,()=>false))roots.push(root);}
 if(roots.length===0)return false;
 const origin=readFleetMachineV1(context.root)?.originId;
 if(origin===undefined||signFleetDocumentV1(context.root,"cache-write","install-pack","probe")?.origin!==origin){request.event?.({stage:"skip",detail:"this machine has no fleet identity to sign the pack"});return false;}
 const scanned=await request.scan(),shape:FleetInstallShapeV1={entries:scanned.entries,directories:scanned.directories,files:scanned.files,links:scanned.links,sha256:scanned.sha256};request.check();
 const metadata=await inspectFleetInstallMetadataV1(request.directory,roots,request.storeRoot,request.check,process.platform,true);
 if(metadata.bad!==undefined){request.event?.({stage:"skip",detail:`non-relocatable link ${metadata.bad}`});return false;}
 const fingerprint=metadata.fingerprint;
 await mkdir(directory,{recursive:true});
 try{
  const linking=captureFleetInstallHardlinksV1(request.directory,request.check),plan=await planFleetInstallPackV1(request.directory,roots,request.check,linking.omit);
  const proof=await linking.prove(),cache=await readUnitCache(context.root),packed:Array<FleetInstallIndexV1["units"][number]>=[];
  let total=0;
  await poolV1(plan.units,policy.installPackConcurrency,async unit=>{
   const targets=unit.entries.flatMap(path=>{const target=proof.targets.get(path);return target?[[path,target.sha256]]:[];});
   const signature=targets.length?createHash("sha256").update(JSON.stringify([unit.signature,targets])).digest("hex"):unit.signature;
   const blob=await packFleetUnitV1(request,directory,{...unit,signature},cache);
   const maximumPathUnits=unit.entries.reduce((width,path)=>Math.max(width,path.length),unit.path.length);
   packed.push({path:unit.path,...blob,maximumPathUnits});total+=blob.bytes;
   if(total>policy.installMaximumBytes)throw Error("Install pack past its bound");
  });
  request.check();
  packed.sort((left,right)=>left.path<right.path?-1:left.path>right.path?1:0);
  const index={version:installIndexVersion,directories:[...plan.directories].sort(),links:[...plan.links].sort((left,right)=>left.path<right.path?-1:1),hardlinks:proof.rows,units:packed};
  guardIndex(index);
  const encoded=Buffer.from(JSON.stringify(index));
  if(encoded.byteLength>policy.installIndexMaximumBytes)throw Error("Install index exceeds its bound");
  const text=gzipSync(encoded,{level:6}),indexBlob={sha256:createHash("sha256").update(text).digest("hex"),bytes:text.byteLength};
  if(text.byteLength>policy.installIndexMaximumBytes)throw Error("Packed install index exceeds its bound");
  const scratch=blobTemporaryDirectory(directory);await mkdir(scratch,{recursive:true});
  const temporary=join(scratch,`.${randomBytes(8).toString("hex")}.tmp`);await writeFile(temporary,text,{flag:"wx",mode:0o644});await placeBlobV1(directory,temporary,indexBlob);
  await writeUnitCache(context.root,cache);
  const draft={version:1 as const,key,projectionSha256:request.projectionSha256,runtime:request.runtime,installed,shape,storeRoot:request.storeRoot,fingerprint,roots,index:{...indexBlob,units:packed.length,links:plan.links.length,packedBytes:total},lock,createdAtMs:context.now()};
  await proof.recheck();request.check();
  const signed=signFleetDocumentV1(context.root,"cache-write","install-pack",signedText(draft,origin));
  if(signed===undefined||signed.origin!==origin){request.event?.({stage:"skip",detail:"this machine has no fleet identity to sign the pack"});return false;}
  const manifest:FleetInstallManifestV1={...draft,publisher:{origin:signed.origin,signature:signed.signature}};
  guard(manifest,"Closed install manifest required");
  const staged=join(directory,`.${randomBytes(8).toString("hex")}.json.tmp`);
  await writeFile(staged,JSON.stringify(manifest)+"\n",{flag:"wx",mode:0o644});
  await rename(staged,join(directory,key+policy.installManifestSuffix));
  request.event?.({stage:"publish",detail:key,bytes:total});
  await announce(context,key,manifest);
  await pruneFleetInstallsV1(directory,context.now(),request.keep).catch(()=>{});
  return true;
 }catch(error){
  if(error instanceof Error&&/cancel|abort/iu.test(error.message))throw error;
  request.event?.({stage:"skip",detail:`publish failed: ${error instanceof Error?error.message:String(error)}`});
  return false;
 }
}

/** 📚️ Reads the index blob a manifest names; `undefined` when it is missing, short, corrupt or not a closed index. */
export async function fleetInstallIndexV1(directory:string,manifest:FleetInstallManifestV1):Promise<FleetInstallIndexV1|undefined>{
 try{
  const path=blobPathOf(directory,manifest.index.sha256),facts=await stat(path);
  if(!facts.isFile()||facts.size<1||facts.size!==manifest.index.bytes||facts.size>policy.installIndexMaximumBytes)return undefined;
  const chunks:Buffer[]=[];
  for await(const chunk of createReadStream(path,{end:facts.size-1}))chunks.push(chunk as Buffer);
  const bytes=Buffer.concat(chunks);
  if(bytes.byteLength!==manifest.index.bytes||createHash("sha256").update(bytes).digest("hex")!==manifest.index.sha256)return undefined;
  const value=JSON.parse(gunzipSync(bytes,{maxOutputLength:policy.installIndexMaximumBytes}).toString("utf8")) as FleetInstallIndexV1;guardIndex(value);return value;
 }catch{return undefined;}
}

/** 🧹️ Keeps the newest `installKeepStates` published states and drops every older manifest, then every blob no kept manifest or index names (blobs and temporary files younger than an hour stay: a publisher may be about to name them); a store holds a few lock states, never every one the fleet has seen. */
export async function pruneFleetInstallsV1(directory:string,now:number,keep:number=policy.installKeepStates):Promise<number>{
 const rows:{name:string;createdAtMs:number;manifest:FleetInstallManifestV1}[]=[];let removed=0;
 for(const name of await readdir(directory).catch(()=>[] as string[])){
  const path=join(directory,name);
  if(name.endsWith(".tmp")){const facts=await stat(path).catch(()=>undefined);if(facts&&now-facts.mtimeMs>3600000){await rm(path,{force:true});removed++;}continue;}
  if(!name.endsWith(policy.installManifestSuffix))continue;
  try{const value=JSON.parse(await readFile(path,"utf8")) as FleetInstallManifestV1;guard(value,"Closed install manifest required");rows.push({name,createdAtMs:value.createdAtMs,manifest:value});}catch{}
 }
 rows.sort((left,right)=>right.createdAtMs-left.createdAtMs);
 for(const row of rows.slice(keep)){await rm(join(directory,row.name),{force:true});removed++;}
 const referenced=new Set<string>();
 for(const row of rows.slice(0,keep)){
  const index=await fleetInstallIndexV1(directory,row.manifest);
  if(index===undefined)return removed;
  referenced.add(row.manifest.index.sha256);for(const unit of index.units)referenced.add(unit.sha256);
 }
 const base=join(directory,...policy.installBlobDirectory);
 for(const folder of await readdir(base).catch(()=>[] as string[])){
  const inside=join(base,folder);
  if(folder.startsWith(".")){const facts=await stat(inside).catch(()=>undefined);if(facts&&now-facts.mtimeMs>3600000){await rm(inside,{force:true});removed++;}continue;}
  for(const name of await readdir(inside).catch(()=>[] as string[])){
   if(referenced.has(name))continue;
   const facts=await stat(join(inside,name)).catch(()=>undefined);
   if(facts&&now-facts.mtimeMs>3600000){await rm(join(inside,name),{force:true});removed++;}
  }
 }
 return removed;
}

const authorised=(context:FleetFetchContextV1,json:boolean):Record<string,string>=>({...(context.remote?.token?{authorization:`Bearer ${context.remote.token}`}:{}),...(json?{"content-type":"application/json"}:{})});

async function announce(context:FleetFetchContextV1,key:string,manifest:FleetInstallManifestV1):Promise<void>{
 const remote=context.remote;if(remote?.authenticated!==true)return;
 const body=JSON.stringify(manifest);
 try{const response=await context.fetch(`${remote.endpoint}/${remote.instance}/v1/cache/${key}`,{method:"PUT",headers:{...authorised(context,false),"content-length":String(Buffer.byteLength(body))},body,signal:AbortSignal.timeout(policy.httpTimeoutMilliseconds)});await response.body?.cancel().catch(()=>{});}catch{}
}

/** 📤️ Extracts unit blobs of a pack into `target`: each blob is size-checked, hashed while it streams into `tar`, and refused when its SHA-256 differs from the one the index names. */
export async function extractFleetPackUnitsV1(directory:string,units:readonly Readonly<{path:string;sha256:string;bytes:number}>[],target:string,check:()=>void):Promise<void>{
 await poolV1(units,policy.installPackConcurrency,async unit=>{
  check();
  const file=blobPathOf(directory,unit.sha256),facts=await stat(file);
  if(!facts.isFile()||facts.size!==unit.bytes)throw Error("A pack blob is missing or has the wrong size");
  const child=runTar(["-xf","-","--no-same-owner","-C",target],target,check,["pipe","ignore","pipe"]),hash=createHash("sha256");
  let said="";child.stderr?.on("data",chunk=>{said=(said+chunk).slice(-300);});
  const exit=closed(child);
  const source=createReadStream(file,{highWaterMark:1<<20});source.on("data",chunk=>{hash.update(chunk as Buffer);});
  child.stdin!.on("error",()=>{});
  const sink=new Writable({write(chunk,_,done){if(child.stdin!.destroyed||child.stdin!.writableEnded){done();return;}child.stdin!.write(chunk,()=>{done();});},final(done){if(child.stdin!.destroyed){done();return;}child.stdin!.end(()=>{done();});}});
  await pipeline(source,createGunzip(),sink);
  if(await exit!==0)throw Error(`tar refused a pack blob ${said}`.trim());
  if(hash.digest("hex")!==unit.sha256)throw Error("A pack blob does not match its digest");
 });
}

/** 📥️ Restores the roots of an install state from the group store: verifies the manifest's signature, lockfile binding and index, extracts every unit blob (its SHA-256 is checked while it streams into `tar`, so a corrupt blob is never trusted), recreates links and empty directories, then proves the tree equals the publisher's per-file fingerprint and the installer's own shape. Any disagreement removes what was placed and answers `false`, which costs one real installation and never a wrong tree. */
export async function fleetInstallRestoreV1(request:FleetInstallRequestV1,manifest?:FleetInstallManifestV1):Promise<boolean>{
 const {context}=request,directory=directoryOf(context,request.space),key=fleetInstallKeyV1(request.projectionSha256,request.runtime),found=manifest??await fleetInstallManifestV1(context,key,request.space);
 if(directory===undefined||!found||found.key!==key||found.projectionSha256!==request.projectionSha256||JSON.stringify(found.runtime)!==JSON.stringify(request.runtime))return false;
 if(new Set(found.roots).size!==found.roots.length||found.roots.some(root=>!fleetInstallPathFitsV1(request.directory,root)||!request.roots.includes(root)))return false;
 for(const root of found.roots)if(await lstat(join(request.directory,...root.split("/"))).then(()=>true,()=>false))return false;
 const machine=context.root===undefined?undefined:readFleetMachineV1(context.root),currentLock=request.lock??await lockOf(request.directory,request.lockFile);
 if(machine===undefined||machine.storeRoot===undefined||currentLock===undefined)return false;
 if(found.lock.sha256!==currentLock.sha256||found.lock.packages!==currentLock.packages){request.event?.({stage:"skip",detail:"the pack was installed from other packages than this lockfile pins"});return false;}
 const registry=createFleetRegistryV1({storeRoot:machine.storeRoot,group:machine.group,pinsPath:machinePathV1(context.root!,".cache","f","identity","pins-install.json")});
 const {publisher,...bare}=found;
 if(!await verifyFleetDocumentV1(registry,publisher.origin,"cache-write","install-pack",signedText(bare,publisher.origin),publisher.signature)){request.event?.({stage:"skip",detail:"the pack is not signed by a registered fleet origin"});return false;}
 const index=await fleetInstallIndexV1(directory,found);
 if(index===undefined){request.event?.({stage:"skip",detail:"the pack index is missing or corrupt"});return false;}
 const packedBytes=index.units.reduce((sum,unit)=>sum+unit.bytes,0);
 if(index.units.length!==found.index.units||index.links.length!==found.index.links||packedBytes!==found.index.packedBytes||packedBytes>policy.installMaximumBytes){
  request.event?.({stage:"skip",detail:"the archive counts or sizes disagree with the index"});return false;
 }
 let restoreHardlinks:ReturnType<typeof planFleetInstallHardlinkRestoreV1>;
 try{restoreHardlinks=planFleetInstallHardlinkRestoreV1(request.directory,found.roots,index);}
 catch(error){request.event?.({stage:"skip",detail:error instanceof Error?error.message:String(error)});return false;}
 const placed=found.roots.map(root=>join(request.directory,...root.split("/")));
 const undo=async()=>{for(const path of [...placed].reverse())await request.remove(path).catch(()=>{});};
 try{
  request.event?.({stage:"restore",detail:"extract",bytes:found.index.packedBytes});
  for(const path of index.directories)await mkdir(join(request.directory,...path.split("/")),{recursive:true});
  await extractFleetPackUnitsV1(directory,index.units,request.directory,request.check);
  await restoreHardlinks(request.check);
  request.check();
  const platform=process.platform,source=fleetInstallLinkTextV1(found.storeRoot,platform).replace(/[\\/]+$/u,""),target=request.storeRoot.replace(/[\\/]+$/u,"");
  for(const link of index.links){
   request.check();
   const path=join(request.directory,...link.path.split("/")),text=fleetInstallLinkTextV1(link.target,platform);
   await mkdir(dirname(path),{recursive:true});
   const next=source!==target&&text.startsWith(source+"/")?target+text.slice(source.length):link.target;
   await symlink(platform==="win32"?next.replaceAll("/","\\"):next,path,platform==="win32"?"junction":undefined);
  }
  await rewriteFleetInstallLinksV1(request.directory,found.roots,found.storeRoot,request.storeRoot,request.check);
  if(await fleetInstallFingerprintV1(request.directory,found.roots,request.storeRoot,request.check)!==found.fingerprint)throw Error("Restored tree differs from its publisher");
  const shape=await request.scan(),expected=found.shape;
  if(found.storeRoot===request.storeRoot&&(shape.entries!==expected.entries||shape.directories!==expected.directories||shape.files!==expected.files||shape.links!==expected.links||shape.sha256!==expected.sha256))throw Error("Restored tree differs from its publisher");
  if(found.storeRoot!==request.storeRoot&&(shape.entries!==expected.entries||shape.directories!==expected.directories||shape.files!==expected.files||shape.links!==expected.links))throw Error("Restored tree differs from its publisher");
  request.event?.({stage:"restore",detail:key});
  return true;
 }catch(error){
  await undo();request.check();
  request.event?.({stage:"skip",detail:`restore failed: ${error instanceof Error?error.message:String(error)}`});
  return false;
 }
}

/** 🌱️ Publishes every already installed and sealed generation of this machine that the fleet does not have yet, one after the other (the same operation as the publish after an install, so a warm machine seeds the fleet); returns how many were published now, how many the fleet already had and how many could not be published. */
export async function fleetInstallSeedV1(requests:readonly FleetInstallRequestV1[]):Promise<Readonly<{published:number;present:number;failed:number}>>{
 let published=0,present=0,failed=0;
 for(const request of requests){
  const directory=directoryOf(request.context,request.space),key=fleetInstallKeyV1(request.projectionSha256,request.runtime);
  if(directory!==undefined&&await fleetInstallManifestV1(request.context,key,request.space)){present++;continue;}
  if(await fleetInstallPublishV1(request).catch(()=>false))published++;else failed++;
 }
 return {published,present,failed};
}

/** 🎟️ What an acquisition hands back: whether the tree was restored, and the publish and release steps of the claim it may hold. */
export type FleetInstallAcquisitionV1=Readonly<{restored:boolean;publish():Promise<boolean>;release():Promise<void>}>;

const callClaim=async(context:FleetFetchContextV1,key:string,action:"claim"|"renew"|"release",body:Record<string,unknown>):Promise<Readonly<{status:number;state:string}>|undefined>=>{
 const remote=context.remote;if(remote?.authenticated!==true)return undefined;
 try{
  const response=await context.fetch(`${remote.endpoint}/${remote.instance}/claim/nx/${key}${action==="claim"?"":"/"+action}`,{method:"POST",headers:authorised(context,true),body:JSON.stringify(body),signal:AbortSignal.timeout(policy.httpTimeoutMilliseconds)});
  const value=await response.json().catch(()=>({})) as {state?:string};
  return {status:response.status,state:value.state??""};
 }catch{return undefined;}
};
const awaitStored=async(context:FleetFetchContextV1,key:string):Promise<string|undefined>=>{
 const remote=context.remote;if(remote?.authenticated!==true)return undefined;
 try{
  const response=await context.fetch(`${remote.endpoint}/${remote.instance}/claim/nx/${key}/await?timeoutMilliseconds=${policy.installAwaitMilliseconds}`,{headers:authorised(context,false),signal:AbortSignal.timeout(policy.installAwaitMilliseconds+policy.httpTimeoutMilliseconds)});
  return ((await response.json().catch(()=>({}))) as {state?:string}).state;
 }catch{return undefined;}
};

/** 🛎️ Restores the install state from the fleet when it is published, otherwise takes the fleet claim so that exactly one machine installs it while the others wait for the published archive; every failure of the claim route, the store or the wait degrades to a plain local installation. The caller installs when `restored` is false and then calls `publish`, always `release` at the end. */
export async function fleetInstallAcquireV1(request:FleetInstallRequestV1):Promise<FleetInstallAcquisitionV1>{
 const {context}=request,none:FleetInstallAcquisitionV1={restored:false,publish:async()=>false,release:async()=>{}};
 if(directoryOf(context,request.space)===undefined)return none;
 const key=fleetInstallKeyV1(request.projectionSha256,request.runtime),owner=request.owner??`${hostname().replace(/[^A-Za-z0-9._-]/gu,"-")}:${process.pid}:${randomBytes(3).toString("hex")}`;
 let stale=false;
 const attempt=async():Promise<boolean>=>{const manifest=await fleetInstallManifestV1(context,key,request.space);if(!manifest)return false;if(await fleetInstallRestoreV1(request,manifest))return true;stale=true;return false;};
 if(await attempt())return {...none,restored:true};
 if(stale||context.remote?.authenticated!==true)return {restored:false,publish:()=>fleetInstallPublishV1(request,stale),release:async()=>{}};
 const deadline=context.now()+policy.installWaitMaximumMilliseconds;
 let timer:ReturnType<typeof setInterval>|undefined,held=false;
 const release=async():Promise<void>=>{if(timer){clearInterval(timer);timer=undefined;}if(held){held=false;await callClaim(context,key,"release",{owner});}};
 try{
  for(;;){
   request.check();
   const claim=await callClaim(context,key,"claim",{owner,ttlMilliseconds:policy.installClaimTtlMilliseconds});
   if(claim===undefined||claim.state==="full"||context.now()>deadline)return {restored:false,publish:()=>fleetInstallPublishV1(request,stale),release};
   if(claim.state==="granted"){
    held=true;
    let beats=0;
    timer=setInterval(()=>{if(++beats>policy.installClaimMaximumBeats){clearInterval(timer);timer=undefined;return;}void callClaim(context,key,"renew",{owner,ttlMilliseconds:policy.installClaimTtlMilliseconds});},policy.installClaimRenewMilliseconds);
    timer.unref?.();
    request.event?.({stage:"claim",detail:key});
    if(await attempt()){await release();return {...none,restored:true};}
    return {restored:false,publish:()=>fleetInstallPublishV1(request,stale),release};
   }
   request.event?.({stage:"wait",detail:key});
   const outcome=await awaitStored(context,key);
   if(outcome===undefined)return {restored:false,publish:()=>fleetInstallPublishV1(request,stale),release};
   if(outcome==="stored"&&await attempt())return {...none,restored:true};
  }
 }catch(error){await release();throw error;}
}

/** 🧩️ How one scope of a funding was obtained: `restored` from the fleet (no installation ran) or `installed` here, and whether the fleet received the tree. */
export type FleetInstallScopeResultV1<T>=Readonly<{origin:"restored"|"installed";value:T;published:boolean;key:string}>;

/** 🧩️ One scope of a funding: a pack of the same inputs and host class is restored and must qualify, otherwise (no pack, a damaged pack, a tree that does not qualify) the roots a restore placed are removed, `install` runs and the tree is published once it qualified. `qualify` is the caller's own proof of a complete tree. A cancellation propagates, anything else ends in a local installation. */
export async function fleetInstallScopeV1<T>(request:FleetInstallRequestV1,steps:Readonly<{install:()=>Promise<void>;qualify:()=>Promise<T>}>):Promise<FleetInstallScopeResultV1<T>>{
 const key=fleetInstallKeyV1(request.projectionSha256,request.runtime);
 let acquisition:FleetInstallAcquisitionV1={restored:false,publish:async()=>false,release:async()=>{}};
 try{acquisition=await fleetInstallAcquireV1(request);}catch(error){request.check();request.event?.({stage:"skip",detail:`fleet lookup failed: ${error instanceof Error?error.message:String(error)}`});}
 let repaired=false;
 try{
  if(acquisition.restored){
   try{return {origin:"restored",value:await steps.qualify(),published:false,key};}
   catch(error){
    request.check();request.event?.({stage:"skip",detail:`restored tree did not qualify: ${error instanceof Error?error.message:String(error)}`});
    for(const root of request.roots)await request.remove(join(request.directory,...root.split("/"))).catch(()=>{});
    repaired=true;
   }
  }
  await steps.install();
  const value=await steps.qualify(),published=repaired?await fleetInstallPublishV1(request,true).catch(()=>false):await acquisition.publish().catch(()=>false);
  return {origin:"installed",value,published,key};
 }finally{await acquisition.release().catch(()=>{});}
}
