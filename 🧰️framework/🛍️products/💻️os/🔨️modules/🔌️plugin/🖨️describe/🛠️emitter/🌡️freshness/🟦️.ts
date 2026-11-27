import {createHash} from "node:crypto";
import {readFile} from "node:fs/promises";
import {basename,isAbsolute,join,relative,resolve,sep} from "node:path";
import {cargoLockRootsV1,cargoResolverLockPathV1,projectCargoLockClosureV1,projectCargoManifestV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import type {CargoProvenanceV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
import {checkoutSnapshotRelativeV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/📸️checkout-snapshot/🟦️.ts";
import {mapBoundedV1,SOURCE_FRESHNESS_FILE_CONCURRENCY} from "../../../🏗️build/🔍️freshness/🟦️.ts";

export const DESCRIPTOR_EMITTER_BUILD_TARGET_V1="@semio-tech/os-plugin-describe-rs:build";
const sha=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");

/** 🔁️ Names the file of `root` that a recorded absolute path stands for: the path itself when it lies in `root`, the same workspace-relative file of `root` when it was recorded below a checkout snapshot or below the live `origin` workspace, else the path unchanged. A snapshot-built emitter is fresh for another generation exactly when those files are equal. */
export function descriptorEmitterRecordedPathV1(root:string,path:string,origin?:string):string{
 const snapshot=checkoutSnapshotRelativeV1(path);
 if(snapshot!==undefined)return join(root,...snapshot.split("/"));
 const local=relative(root,path);
 if(local!==""&&!local.startsWith("..")&&!isAbsolute(local))return path;
 if(origin===undefined||resolve(origin)===root)return path;
 const live=relative(resolve(origin),path);
 return live===""||live.startsWith("..")||isAbsolute(live)||live.split(sep)[0]===".cache"?path:join(root,live);
}

/** 🌡️ Lists the workspace sources, as posix paths relative to `workspace`, that the emitter's compiler receipt hashed and that are missing or differ now.
 * Dependency sources under `.cache` are versioned and immutable, so only authored files count.
 * @see ../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts */
export async function staleDescriptorEmitterInputsV1(receipt:string,workspace:string,signal?:AbortSignal,origin?:string):Promise<Readonly<{checked:number;stale:readonly string[]}>>{
 const root=resolve(workspace),parsed:unknown=JSON.parse(receipt),row=parsed as {strings?:unknown;inputs?:unknown};
 if(!row||!Array.isArray(row.strings)||!Array.isArray(row.inputs))throw Error("Descriptor emitter compiler receipt without hashed inputs");
 const sources=row.inputs.flatMap((input:unknown)=>{
  const [index,kind,sha256]=input as [unknown,unknown,unknown],recorded=typeof index==="number"?(row.strings as unknown[])[index]:undefined,path=typeof recorded==="string"&&isAbsolute(recorded)?descriptorEmitterRecordedPathV1(root,recorded,origin):recorded;
  if(kind!=="file"||typeof path!=="string"||typeof sha256!=="string"||!isAbsolute(path)||!/^[a-f0-9]{64}$/u.test(sha256))return [];
  const local=relative(root,path);
  return local===""||local.startsWith("..")||isAbsolute(local)||local.split(sep)[0]===".cache"?[]:[{path,local:local.split(sep).join("/"),sha256}];
 });
 const fresh=await mapBoundedV1(sources,SOURCE_FRESHNESS_FILE_CONCURRENCY,async source=>{
  try{return createHash("sha256").update(await readFile(source.path,{signal})).digest("hex")===source.sha256;}catch(error){signal?.throwIfAborted();return false;}
 },signal);
 return Object.freeze({checked:sources.length,stale:Object.freeze(sources.filter((_,index)=>!fresh[index]).map(source=>source.local).sort())});
}

/** 🧷️ Checks the manifests and configuration of the captured emitter, comparing the workspace lock's selected package closure to its verified retained bytes. */
export async function staleDescriptorEmitterCargoInputsV1(
 recordedObservation:Pick<CargoProvenanceV1,"manifest"|"args"|"units"|"invocationInputs">,
 workspace:string,
 retained:Readonly<{origin:Readonly<{cargoHome:string}>;files:ReadonlyMap<string,Readonly<{path:string}>>}>,
 signal?:AbortSignal,
 liveOrigin?:string,
):Promise<Readonly<{stale:readonly string[]}>>{
 const root=resolve(workspace),to=(path:string)=>descriptorEmitterRecordedPathV1(root,path,liveOrigin),observation={...recordedObservation,manifest:to(recordedObservation.manifest),args:recordedObservation.args.map(argument=>isAbsolute(argument)?to(argument):argument),units:recordedObservation.units.map(unit=>typeof unit.message.manifest_path==="string"?{...unit,message:{...unit.message,manifest_path:to(unit.message.manifest_path)}}:unit),invocationInputs:recordedObservation.invocationInputs.map(row=>({...row,path:to(row.path)}))},lock=join(root,"Cargo.lock"),rootManifest=join(root,"Cargo.toml"),manifests=new Set([observation.manifest,rootManifest]);
 const recordedLock=cargoResolverLockPathV1(observation.args)??lock;
 const cargoHomeConfig=new Set(["config","config.toml"].map(name=>join(retained.origin.cargoHome,name)));
 for(const unit of observation.units)if(typeof unit.message.manifest_path==="string")manifests.add(resolve(unit.message.manifest_path));
 const inputs=observation.invocationInputs.flatMap(row=>row.path===recordedLock?[{path:lock,sha256:row.sha256}]:row.path===lock?[]:[row]).filter(({path})=>{
  if(cargoHomeConfig.has(path))return true;
  const local=relative(root,path);
  return local!==""&&!local.startsWith("..")&&!isAbsolute(local)&&local.split(sep)[0]!==".cache"&&
   (manifests.has(path)||path===lock||["config","config.toml","rust-toolchain","rust-toolchain.toml"].includes(basename(path)));
 });
 if(!inputs.some(row=>row.path===lock&&row.sha256!==null)||!inputs.some(row=>row.path===observation.manifest&&row.sha256!==null))throw Error("Descriptor emitter receipt without its Cargo lock and manifest");
 const manifest=await readFile(observation.manifest,{encoding:"utf8",signal}),roots=cargoLockRootsV1(manifest,observation.args);
 const fresh=await mapBoundedV1(inputs,SOURCE_FRESHNESS_FILE_CONCURRENCY,async({path,sha256})=>{
  try{
   const current=await readFile(path,{signal});
   if(sha(current)===sha256)return true;
   if(!sha256||path!==lock&&path!==rootManifest)return false;
   const historical=retained.files.get("blobs/"+sha256);
   if(!historical)return false;
   const original=await readFile(historical.path,{signal});
   if(sha(original)!==sha256)return false;
   const project=path===lock?(bytes:Uint8Array)=>projectCargoLockClosureV1(bytes,roots):projectCargoManifestV1;
   return project(current).digest===project(original).digest;
  }catch(error){
   signal?.throwIfAborted();
   return sha256===null&&(error as NodeJS.ErrnoException).code==="ENOENT";
  }
 },signal);
 return Object.freeze({stale:Object.freeze(inputs.filter((_,index)=>!fresh[index]).map(({path})=>relative(root,path).split(sep).join("/")).sort())});
}
