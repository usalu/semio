import {lstatSync} from "node:fs";
import {dirname,isAbsolute,relative,resolve,sep,join} from "node:path";
import {checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{acquireNativeStorageV1,type NativeStoragePortV1,type NativeStorageReservationV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import type{NativeStoragePolicyV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🗃️ Defines the complete artifact writer envelope before any Nx native copy. */
export interface BoundedNxArtifactRequestV1 {readonly version:1;readonly kind:"artifact-publication"|"artifact-restore";readonly owner:string;readonly maximumBytes:number;readonly paths:readonly string[]}
/** 📂️ Admits the exact managed write paths. */
export function validateBoundedNxPathsV1(root:string,paths:readonly string[]):readonly string[]{
 if(resolve(root)!==root||!paths.length||paths.length>256)throw Error("Closed Nx enforcing boundary required");requireMaterializedPathV1(root);
 for(let parent=root;;parent=dirname(parent)){const status=lstatSync(parent);if(!status.isDirectory()||status.isSymbolicLink())throw Error("Nx storage boundary requires no-follow ancestry");if(dirname(parent)===parent)break;}
 const device=lstatSync(root).dev;
 for(const path of paths){requireMaterializedPathV1(path);const suffix=relative(root,path);if(resolve(path)!==path||suffix===".."||suffix.startsWith(".."+sep)||isAbsolute(suffix))throw Error("Nx output escapes enforcing boundary");
  let current=root;for(const part of suffix.split(sep).filter(Boolean)){current=join(current,part);let status;try{status=lstatSync(current);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}if(status.isSymbolicLink()||status.dev!==device||!status.isDirectory()&&!status.isFile())throw Error("Nx output escapes enforcing boundary through physical substitution");}
 }
 return paths;
}
/** 🎟️ Keeps artifact allocation charge through the original publication lifetime. */
export async function withBoundedNxArtifactWriteV1<T>(options:Readonly<{policy:NativeStoragePolicyV1;control:ResourceLeaseControl;port:NativeStoragePortV1;request:BoundedNxArtifactRequestV1}>,body:(reservation:NativeStorageReservationV1)=>Promise<T>):Promise<Readonly<{value:T;reservation:NativeStorageReservationV1}>>{
 const {policy,control,port,request}=options;checkResourceLeaseControl(control);const errors=validateJsonSchemaSubset(schema,request);if(errors.length)throw Error("Invalid bounded Nx artifact request: "+errors.join("; "));
 validateBoundedNxPathsV1(policy.root,request.paths);
 const reservation=await acquireNativeStorageV1({policy,control,port,owner:request.owner,kind:request.kind,bytes:request.maximumBytes});
 try{await reservation.recheck();validateBoundedNxPathsV1(policy.root,request.paths);checkResourceLeaseControl(control);const value=await body(reservation);checkResourceLeaseControl(control);await reservation.recheck();validateBoundedNxPathsV1(policy.root,request.paths);await reservation.commit();return Object.freeze({value,reservation});}
 catch(error){if(error instanceof Error)Object.defineProperty(error,"nativeStorageAllocation",{value:reservation.allocation});throw error;}
}
