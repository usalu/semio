import {createHash,randomBytes} from "node:crypto";
import {existsSync,mkdirSync,readFileSync,statSync,writeFileSync} from "node:fs";
import {mkdir,readFile,rename,rm,symlink,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {gunzipSync,gzipSync} from "node:zlib";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {FLEET_FETCH_POLICY_V1 as policy,type FleetFetchContextV1} from "../🟦️.ts";
import {blobPathOf,extractFleetPackUnitsV1,packFleetUnitV1,placeBlobV1,planFleetInstallPackV1,poolV1,readUnitCache,unitCachePath,writeUnitCache} from "../📦️install/🟦️.ts";
import {createFleetRegistryV1,readFleetMachineV1,signFleetDocumentV1,verifyFleetDocumentV1} from "../../../../🏗️build/🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {setupClassV1,setupHostV1,setupTextV1,type SetupHostV1} from "../🧰️setup/🟦️.ts";

/** 🐍️ One wheel the lock pins: its normalized name, the cache stem (`<version>-<tags>`) and the SHA-256 the lock records. */
export type FleetUvWheelV1=Readonly<{name:string;stem:string;sha256:string}>;
/** 🐍️ The published description of one seeded uv cache. */
export type FleetUvManifestV1=Readonly<{version:1;lockSha256:string;wheels:number;index:Readonly<{sha256:string;bytes:number}>;packedBytes:number;publisher:Readonly<{origin:string;signature:string}>;createdAtMs:number}>;
type UvIndexV1=Readonly<{version:1;directories:readonly string[];links:readonly Readonly<{path:string;target:string}>[];units:readonly Readonly<{path:string;sha256:string;bytes:number}>[];files:readonly Readonly<{path:string;wheelSha256:string;archive:string;base64:string}>[]}>;
/** 📣️ A progress or outcome event of a seed or a restore. */
export type FleetUvEventV1=Readonly<{stage:"publish"|"restore"|"skip";detail:string;bytes?:number}>;

const guard=(definition:"UvManifest"|"UvIndex",value:unknown):void=>{const errors=validateJsonSchemaSubset({$ref:`#/definitions/${definition}`},value,schema);if(errors.length)throw Error(`Closed ${definition} required: ${errors[0]}`);};
/** 🏷️ The file a seed is stored under: the digest of the lock and the host class, because the wheels a cache holds carry the tags of the platform that fetched them (a seed of Linux must never replace the seed of macOS). */
export const fleetUvManifestNameV1=(lockSha256:string,host:SetupHostV1=setupHostV1()):string=>`${policy.uvManifestPrefix}${lockSha256}-${setupClassV1("host",host)?.tag??"unclassified"}${policy.installManifestSuffix}`;
const directoryOf=(context:FleetFetchContextV1):string|undefined=>context.storeRoot===undefined?undefined:join(context.storeRoot,...policy.uvDirectory);
const signedText=(manifest:Omit<FleetUvManifestV1,"publisher">,origin:string):string=>[origin,manifest.lockSha256,manifest.index.sha256,manifest.wheels,manifest.packedBytes].join("\n");

/** 🔎️ The wheels a `uv.lock` pins, from its text: `name`, the wheel file names below it and their `sha256:` hashes. */
export function fleetUvWheelsV1(lock:string):readonly FleetUvWheelV1[]{
 const rows:FleetUvWheelV1[]=[];
 for(const match of lock.matchAll(/url = "[^"]*\/([^"\/]+)\.whl", hash = "sha256:([0-9a-f]{64})"/gu)){
  const file=match[1]!,parts=file.split("-");
  if(parts.length<5)continue;
  rows.push({name:parts[0]!.toLowerCase().replaceAll("_","-").replaceAll(".","-"),stem:parts.slice(1).join("-"),sha256:match[2]!});
 }
 return rows;
}

/** 🔬️ Reads a cache pointer (`wheels-v6/pypi/<name>/<stem>.http`): the archive id uv unpacked the wheel into and the SHA-256 it recorded. `undefined` for any other shape. */
export function fleetUvPointerV1(bytes:Uint8Array):Readonly<{archive:string;sha256:string}>|undefined{
 const text=Buffer.from(bytes).toString("latin1");
 if(bytes.length<20||bytes[0]!==0x94||bytes[1]!==0xb0)return undefined;
 const archive=text.slice(2,18),hash=/Sha256[^0-9a-f]{1,4}([0-9a-f]{64})/u.exec(text);
 return /^[A-Za-z0-9_-]{16}$/u.test(archive)&&hash?{archive,sha256:hash[1]!}:undefined;
}

/** 🌱️ Seeds the fleet from a warm uv cache: for every wheel of the lock whose cache pointer records the lock's SHA-256 and whose unpacked archive exists, packs the archive into content-addressed unit blobs (one tar per top-level entry, deduplicated across archives) and writes one signed manifest named by the SHA-256 of the lock. Returns the number of wheels, or `undefined` when nothing could be published (no store, no identity, nothing in the cache). */
export async function fleetUvPublishV1(context:FleetFetchContextV1,input:Readonly<{cacheDirectory:string;lock:string;check?:()=>void;event?:(event:FleetUvEventV1)=>void;host?:SetupHostV1}>):Promise<number|undefined>{
 const directory=directoryOf(context),check=input.check??(()=>{});
 if(directory===undefined||context.root===undefined)return undefined;
 const origin=readFleetMachineV1(context.root)?.originId;
 if(origin===undefined||signFleetDocumentV1(context.root,"cache-write","uv-seed","probe")?.origin!==origin){input.event?.({stage:"skip",detail:"this machine has no fleet identity to sign the seed"});return undefined;}
 const lockSha256=createHash("sha256").update(input.lock).digest("hex");
 const files:{path:string;wheelSha256:string;archive:string;base64:string}[]=[],roots:string[]=[],seen=new Set<string>();
 for(const wheel of fleetUvWheelsV1(input.lock)){
  for(const suffix of [".http",".msgpack"]){
   const path=`wheels-v6/pypi/${wheel.name}/${wheel.stem}${suffix}`,absolute=join(input.cacheDirectory,...path.split("/"));
   if(!existsSync(absolute))break;
   const bytes=readFileSync(absolute),pointer=suffix===".http"?fleetUvPointerV1(bytes):undefined;
   if(suffix===".http"){
    if(pointer===undefined||pointer.sha256!==wheel.sha256||!statSync(join(input.cacheDirectory,"archive-v0",pointer.archive),{throwIfNoEntry:false})?.isDirectory())break;
    if(!seen.has(pointer.archive)){seen.add(pointer.archive);roots.push(`archive-v0/${pointer.archive}`);}
    files.push({path,wheelSha256:wheel.sha256,archive:pointer.archive,base64:bytes.toString("base64")});
   }else{
    const last=files.at(-1);if(last===undefined||last.path!==path.replace(/\.msgpack$/u,".http"))break;
    files.push({path,wheelSha256:wheel.sha256,archive:last.archive,base64:bytes.toString("base64")});
   }
  }
 }
 if(roots.length===0){input.event?.({stage:"skip",detail:"the uv cache holds none of the locked wheels"});return undefined;}
 const plan=await planFleetInstallPackV1(input.cacheDirectory,roots,check),cache=await readUnitCache(context.root),packed:{path:string;sha256:string;bytes:number}[]=[];
 let total=0;
 await poolV1(plan.units,policy.installPackConcurrency,async unit=>{const blob=await packFleetUnitV1({directory:input.cacheDirectory,check},directory,unit,cache);packed.push({path:unit.path,...blob});total+=blob.bytes;});
 packed.sort((left,right)=>left.path<right.path?-1:left.path>right.path?1:0);
 const index:UvIndexV1={version:1,directories:[...plan.directories].sort(),links:[...plan.links].sort((left,right)=>left.path<right.path?-1:1),units:packed,files};
 guard("UvIndex",index);
 const text=gzipSync(Buffer.from(JSON.stringify(index)),{level:6}),indexBlob={sha256:createHash("sha256").update(text).digest("hex"),bytes:text.byteLength};
 const scratch=join(directory,...policy.installBlobDirectory);await mkdir(scratch,{recursive:true});
 const temporary=join(scratch,`.${randomBytes(8).toString("hex")}.tmp`);await writeFile(temporary,text,{flag:"wx",mode:0o644});await placeBlobV1(directory,temporary,indexBlob);
 await writeUnitCache(context.root,cache);
 const draft={version:1 as const,lockSha256,wheels:new Set(files.map(file=>file.wheelSha256)).size,index:indexBlob,packedBytes:total,createdAtMs:context.now()};
 const signed=signFleetDocumentV1(context.root,"cache-write","uv-seed",signedText(draft,origin));
 if(signed===undefined||signed.origin!==origin)return undefined;
 const manifest:FleetUvManifestV1={...draft,publisher:{origin:signed.origin,signature:signed.signature}};
 guard("UvManifest",manifest);
 const staged=join(directory,`.${randomBytes(8).toString("hex")}.json.tmp`);
 await writeFile(staged,JSON.stringify(manifest)+"\n",{flag:"wx",mode:0o644});
 await rename(staged,join(directory,fleetUvManifestNameV1(lockSha256,input.host)));
 input.event?.({stage:"publish",detail:`${manifest.wheels} wheels`,bytes:total});
 return manifest.wheels;
}

/** 🌿️ Seeds an empty or partial uv cache from the fleet before `uv sync --locked` runs: verifies the manifest's signature against the registered origin and its lock digest against this machine's `uv.lock`, extracts the unpacked archives of the wheels the cache does not hold yet and writes uv's own pointer files and the link from each wheel entry to its archive for them. uv then finds every locked wheel in its cache and downloads nothing; if anything is missing or wrong uv downloads it as before, so a bad seed costs time, never correctness. Returns the number of wheels placed. */
export async function fleetUvRestoreV1(context:FleetFetchContextV1,input:Readonly<{cacheDirectory:string;lock:string;check?:()=>void;event?:(event:FleetUvEventV1)=>void;host?:SetupHostV1}>):Promise<number>{
 const directory=directoryOf(context),check=input.check??(()=>{});
 if(directory===undefined||context.root===undefined)return 0;
 const machine=readFleetMachineV1(context.root);if(machine?.storeRoot===undefined)return 0;
 const lockSha256=createHash("sha256").update(input.lock).digest("hex");
 let manifest:FleetUvManifestV1,index:UvIndexV1;
 try{
  manifest=JSON.parse(await readFile(join(directory,fleetUvManifestNameV1(lockSha256,input.host)),"utf8")) as FleetUvManifestV1;guard("UvManifest",manifest);
  if(manifest.lockSha256!==lockSha256)return 0;
  const registry=createFleetRegistryV1({storeRoot:machine.storeRoot,group:machine.group,pinsPath:join(context.root,".cache","f","identity","pins-uv.json")}),{publisher,...bare}=manifest;
  if(!await verifyFleetDocumentV1(registry,publisher.origin,"cache-write","uv-seed",signedText(bare,publisher.origin),publisher.signature)){input.event?.({stage:"skip",detail:"the seed is not signed by a registered fleet origin"});return 0;}
  const bytes=await readFile(blobPathOf(directory,manifest.index.sha256));
  if(bytes.byteLength!==manifest.index.bytes||createHash("sha256").update(bytes).digest("hex")!==manifest.index.sha256)return 0;
  index=JSON.parse(gunzipSync(bytes).toString("utf8")) as UvIndexV1;guard("UvIndex",index);
 }catch{return 0;}
 const missing=index.files.filter(file=>file.path.endsWith(".http")&&!existsSync(join(input.cacheDirectory,...file.path.split("/"))));
 if(missing.length===0)return 0;
 const wanted=new Set(missing.map(file=>file.archive)),absent=new Set([...wanted].filter(id=>!existsSync(join(input.cacheDirectory,"archive-v0",id)))),needs=(path:string):boolean=>{const id=path.split("/")[1];return path.startsWith("archive-v0/")&&id!==undefined&&absent.has(id);};
 const placed:string[]=[];
 try{
  mkdirSync(input.cacheDirectory,{recursive:true});
  for(const path of index.directories)if(needs(path))await mkdir(join(input.cacheDirectory,...path.split("/")),{recursive:true});
  input.event?.({stage:"restore",detail:`${missing.length} wheels`,bytes:manifest.packedBytes});
  await extractFleetPackUnitsV1(directory,index.units.filter(unit=>needs(unit.path)),input.cacheDirectory,check);
  for(const link of index.links)if(needs(link.path)){const path=join(input.cacheDirectory,...link.path.split("/"));await mkdir(dirname(path),{recursive:true});await symlink(link.target,path);}
  for(const file of index.files){
   const path=join(input.cacheDirectory,...file.path.split("/"));
   if(!wanted.has(file.archive)||existsSync(path)||!existsSync(join(input.cacheDirectory,"archive-v0",file.archive)))continue;
   await mkdir(dirname(path),{recursive:true});const staged=path+`.${randomBytes(4).toString("hex")}.tmp`;writeFileSync(staged,Buffer.from(file.base64,"base64"));await rename(staged,path);placed.push(path);
   if(path.endsWith(".http")){const link=path.slice(0,-5);if(!existsSync(link)){await symlink(join(input.cacheDirectory,"archive-v0",file.archive),link);placed.push(link);}}
  }
  return placed.filter(path=>path.endsWith(".http")).length;
 }catch(error){
  for(const path of placed)await rm(path,{force:true}).catch(()=>{});
  input.event?.({stage:"skip",detail:`restore failed: ${error instanceof Error?error.message:String(error)}`});
  return 0;
 }
}

/** 🔌️ The fleet seam of the uv manager for `ensureGenerationV1` and the Python branch of the native dependency sync: restores the cache (`UV_CACHE_DIR` of the sync environment) from the fleet before `uv sync --locked`, publishes it afterwards; never throws, and says what it did in both languages. */
export function fleetUvPortV1(context:FleetFetchContextV1,lock:string,say:(line:string)=>void=()=>{},host:SetupHostV1=setupHostV1(),environment:Readonly<Record<string,string|undefined>>=process.env):Readonly<{before(environment:Readonly<Record<string,string>>):Promise<void>;after(environment:Readonly<Record<string,string>>):Promise<void>}>{
 const tag=setupClassV1("host",host)?.tag??"unclassified";
 return {
  before:async cache=>{const cacheDirectory=cache.UV_CACHE_DIR;if(cacheDirectory===undefined)return;const count=await fleetUvRestoreV1(context,{cacheDirectory,lock,host}).catch(()=>0);say(count>0?setupTextV1("uvRestored",{count},environment):setupTextV1("uvMiss",{class:tag},environment));},
  after:async cache=>{const cacheDirectory=cache.UV_CACHE_DIR;if(cacheDirectory===undefined)return;const count=await fleetUvPublishV1(context,{cacheDirectory,lock,host}).catch(()=>undefined);if(count!==undefined&&count>0)say(setupTextV1("uvPublished",{count,class:tag},environment));},
 };
}

export {unitCachePath};
