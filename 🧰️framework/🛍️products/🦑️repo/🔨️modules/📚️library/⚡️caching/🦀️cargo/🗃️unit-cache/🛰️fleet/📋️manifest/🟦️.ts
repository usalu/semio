import {createHash,randomBytes} from "node:crypto";
import {mkdirSync,readFileSync,renameSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {decodeProtobufV1,encodeProtobufV1,REAPI_V1} from "../../../../../🏗️build/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {declareV1,workspaceRowsV1} from "../../../../../🏗️build/🌐️remote-cache/🏷️retention/🍂️staleness/🟦️.ts";
import buildPolicy from "../../../../../🏗️build/🔣️policy.json" with {type:"json"};
import {type FleetClientV1} from "../📡️client/🟦️.ts";
import {FLEET_POLICY_V1,localTextV1,portableTextV1,type PortableRootV1} from "../🪪️identity/🟦️.ts";
import type {RecordedGapV1} from "../../🔏️key/🔗️sources/🟦️.ts";

const fleet=FLEET_POLICY_V1,MAXIMUM_FILES=512,MAXIMUM_NAMES=64,HEX=/^[0-9a-f]{64}$/u,NAMESPACE="semio.cargo.unit-cache.input-manifest/v1",NEGATIVE_MILLISECONDS=30000;

/** 📋️ The input manifest of one unit slot: the files (portable text, roots as tokens) and the environment names the compiler read beyond what the scan of the sources reaches, learned from the dependency file of a finished compilation. Names only, never digests: every machine digests the listed files itself, so the unit key covers each input without any machine having compiled the unit before. Both lists are sorted and without duplicates. */
export type InputManifestV1=Readonly<{files:readonly string[];env:readonly string[]}>;

/** 📋️ The slot's manifest as one machine holds it: the manifest, and whether the fleet cache is known to hold exactly it. */
export type ManifestStateV1=Readonly<{manifest:InputManifestV1;shared:boolean}>;

/** 🆔️ The address of the manifest of a slot (`slot` is `slotIdV1`: compiler release, target, profile, package, crate, kinds, features): the same on every machine, after every edit. */
export const manifestIdV1=(slot:string):string=>createHash("sha256").update(NAMESPACE+"\0"+slot).digest("hex");

const clean=(names:readonly unknown[],limit:number):string[]=>[...new Set(names.filter((name):name is string=>typeof name==="string"&&name!==""))].sort().slice(0,limit);

/** 📋️ A manifest from file names and environment names (sorted, bounded: more than 512 files or 64 names are not kept, the unit then stays unshared). */
export const manifestOfV1=(files:readonly string[],env:readonly string[]):InputManifestV1=>Object.freeze({files:Object.freeze(clean(files,MAXIMUM_FILES+1)),env:Object.freeze(clean(env,MAXIMUM_NAMES+1))});

/** 📋️ Whether a manifest is within the bounds (`manifestOfV1` keeps one entry more than the bound so that this can tell). */
export const manifestWithinBoundsV1=(manifest:InputManifestV1):boolean=>manifest.files.length<=MAXIMUM_FILES&&manifest.env.length<=MAXIMUM_NAMES;

/** ➕️ The union of two manifests: what the next compilation of a slot must key when its input set grew. */
export const manifestUnionV1=(left:InputManifestV1|undefined,right:InputManifestV1):InputManifestV1=>manifestOfV1([...left?.files??[],...right.files],[...left?.env??[],...right.env]);

/** 🔎️ Whether a manifest names everything another one does. */
export const manifestCoversV1=(manifest:InputManifestV1|undefined,other:InputManifestV1):boolean=>{const files=new Set(manifest?.files),env=new Set(manifest?.env);return other.files.every(name=>files.has(name))&&other.env.every(name=>env.has(name));};

/** 🧭️ The files of a manifest as this machine spells them (the path put back for each token). */
export const manifestFilesV1=(manifest:InputManifestV1,roots:readonly PortableRootV1[]):readonly RecordedGapV1[]=>manifest.files.map(name=>({name,path:localTextV1(name,roots)}));

/** 🧾️ The portable names of the files a finished compilation read that the key did not cover (paths of its dependency file, resolved against `cwd`, and `env:NAME` entries split off): the manifest part they make. */
export function observedManifestV1(gap:readonly string[],input:Readonly<{cwd:string;roots:readonly PortableRootV1[]}>):InputManifestV1{
 const files:string[]=[],env:string[]=[];
 for(const item of gap){if(item==="dep-info")continue;if(item.startsWith("env:"))env.push(item.slice(4));else files.push(portableTextV1(resolve(input.cwd,item).replaceAll("\\","/"),input.roots));}
 return manifestOfV1(files,env);
}

const stateFile=(root:string,id:string)=>join(root,fleet.layout.state,fleet.layout.manifests,id.slice(0,2),id);

/** 📋️ The manifest this machine holds for a slot, `undefined` when it holds none. A copy that is not of the closed shape counts as none. */
export function readManifestV1(root:string,id:string):ManifestStateV1|undefined{
 if(!HEX.test(id))return undefined;
 try{
  const value=JSON.parse(readFileSync(stateFile(root,id),"utf8")) as {version?:unknown;files?:unknown;env?:unknown;shared?:unknown};
  if(value.version!==1||!Array.isArray(value.files)||!Array.isArray(value.env))return undefined;
  return Object.freeze({manifest:manifestOfV1(value.files,value.env),shared:value.shared===true});
 }catch{return undefined;}
}

/** 💾️ Keeps the manifest of a slot on this machine (atomic). Best effort. */
export function writeManifestV1(root:string,id:string,manifest:InputManifestV1,shared:boolean):void{
 if(!HEX.test(id))return;
 try{
  const file=stateFile(root,id),temporary=file+"."+randomBytes(3).toString("hex");
  mkdirSync(dirname(file),{recursive:true});writeFileSync(temporary,JSON.stringify({version:1,files:manifest.files,env:manifest.env,shared}));renameSync(temporary,file);
 }catch{}
}

const noneFile=(root:string,id:string)=>stateFile(root,id)+".none";
const noneFresh=(root:string,id:string):boolean=>{try{return Date.now()-statSync(noneFile(root,id)).mtimeMs<NEGATIVE_MILLISECONDS&&readManifestV1(root,id)===undefined;}catch{return false;}};
const markNone=(root:string,id:string):void=>{try{const file=noneFile(root,id);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,"");const now=new Date();utimesSync(file,now,now);}catch{}};

/** 📜️ The retention declaration of a manifest: kind `input-manifest`, the compiler identity, and for a crate of the workspace the path of its entry file below the workspace root (the digest is recorded but never judged: an edit does not retire the manifest, only the crate leaving the tree does). A crate outside the workspace (registry, standard library) declares no source and is never stale. */
export function manifestDeclarationV1(input:Readonly<{toolchain:string;entry:string;entrySha256:string|null;cwd:string;workspace:string|undefined}>):ReturnType<typeof declareV1>{
 return declareV1({kind:"input-manifest",toolchain:input.toolchain,sources:workspaceRowsV1({dependencies:[{path:input.entry,sha256:input.entrySha256}],cwd:input.cwd,workspace:input.workspace,generatedRoots:buildPolicy.remoteCache.server.staleness.generatedRoots}),externs:[]});
}

/** 📦️ The two things the fleet cache stores for a manifest: the blob (`{version, retention, files, env}`) and the action result that names it twice (`stage.json`, which the server reads the declaration from, and `inputs.json`, which marks the class) and carries the manifest inline, so a lookup is one request. */
export function manifestWireV1(manifest:InputManifestV1,declaration:ReturnType<typeof declareV1>):Readonly<{blob:Uint8Array;hash:string;result:Uint8Array}>{
 const blob=Buffer.from(JSON.stringify({version:1,retention:declaration,files:manifest.files,env:manifest.env})),hash=createHash("sha256").update(blob).digest("hex"),digest={hash,size_bytes:blob.length};
 return Object.freeze({blob,hash,result:encodeProtobufV1(REAPI_V1.actionResult,{output_files:[{path:"inputs.json",digest},{path:"stage.json",digest}],stdout_raw:blob,exit_code:0})});
}

/** 📦️ The manifest an action result carries; `undefined` for anything that is not of the closed shape. */
export function parseManifestV1(bytes:Uint8Array):InputManifestV1|undefined{
 try{
  const result=decodeProtobufV1(REAPI_V1.actionResult,bytes),value=JSON.parse(Buffer.from(result.stdout_raw as Uint8Array).toString("utf8")) as {version?:unknown;files?:unknown;env?:unknown};
  if(value.version!==1||!Array.isArray(value.files)||!Array.isArray(value.env))return undefined;
  const manifest=manifestOfV1(value.files,value.env);return manifestWithinBoundsV1(manifest)?manifest:undefined;
 }catch{return undefined;}
}

/** 📥️ The manifest of a slot for the lookup of its unit: this machine's copy first, then one request to the fleet cache (a fresh miss is remembered for 30 s so a slot nobody compiled costs one request per half minute, not one per process). The copy of the fleet is kept locally, so every later lookup of the slot on this machine is one small file read. `from` says where it came from: `local`, `fleet`, `absent` (the fleet answered that there is none, or the tier is off) or `unknown` (the fleet could not say: nothing may be published over what it may hold). Never throws. */
export async function obtainManifestV1(root:string,id:string,client:FleetClientV1|undefined):Promise<Readonly<{state:ManifestStateV1|undefined;from:"local"|"fleet"|"absent"|"unknown"}>>{
 const local=readManifestV1(root,id);if(local!==undefined)return{state:local,from:"local"};
 try{
  if(client===undefined)return{state:undefined,from:"absent"};
  if(!client.usable())return{state:undefined,from:"unknown"};
  if(noneFresh(root,id))return{state:undefined,from:"absent"};
  const answer=await client.getResult(id);
  if(answer==="miss"){markNone(root,id);return{state:undefined,from:"absent"};}
  if(typeof answer==="string")return{state:undefined,from:"unknown"};
  const manifest=parseManifestV1(answer);if(manifest===undefined)return{state:undefined,from:"unknown"};
  writeManifestV1(root,id,manifest,true);return{state:Object.freeze({manifest,shared:true}),from:"fleet"};
 }catch{return{state:undefined,from:"unknown"};}
}

/** 🔄️ The fleet's current manifest of a slot, read again although this machine holds a copy: another machine may have learned more since (an input set that grew), so a unit that missed under the local copy is looked up once more under the union before it is compiled. `undefined` when the fleet holds none or cannot say. Never throws. */
export async function refreshManifestV1(client:FleetClientV1|undefined,id:string):Promise<InputManifestV1|undefined>{
 try{
  if(client===undefined||!client.usable())return undefined;
  const answer=await client.getResult(id);
  return typeof answer==="string"?undefined:parseManifestV1(answer);
 }catch{return undefined;}
}

/** 📮️ Publishes the manifest of a slot to the fleet cache (the blob, then the result) and, when stored, remembers locally that the fleet holds it. The server keeps the newest manifest of a slot (the input set only grows, so the newest is the union). `stored`; `diverged` when the server holds another manifest and refuses to replace it; `unshared` when this machine may not write or the cache cannot be reached. Never throws. */
export async function shareManifestV1(root:string,id:string,manifest:InputManifestV1,declaration:ReturnType<typeof declareV1>,client:FleetClientV1|undefined):Promise<"stored"|"diverged"|"unshared">{
 try{
  if(client===undefined||!client.settings.upload||!client.usable()||!manifestWithinBoundsV1(manifest))return "unshared";
  const wire=manifestWireV1(manifest,declaration);
  if(await client.storeBlob(wire.hash,wire.blob)!=="stored")return "unshared";
  const verdict=await client.storeResult(id,wire.result,id);
  if(verdict==="stored"){writeManifestV1(root,id,manifest,true);return "stored";}
  return verdict==="diverged"?"diverged":"unshared";
 }catch{return "unshared";}
}
