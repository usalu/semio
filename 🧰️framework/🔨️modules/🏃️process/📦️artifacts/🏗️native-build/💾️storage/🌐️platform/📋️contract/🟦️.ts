import {lstatSync} from "node:fs";
import {join,resolve,relative,isAbsolute,sep} from "node:path";
import {requireMaterializedPathV1} from "../../../../../🔒️leases/🟦️.ts";
import type {NativeStoragePortV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
const protocol=schema.definitions.ImageProtocol.const;

/** 🚦️ Chooses required quiescence or an immediate optional-maintenance attempt. */
export type NativeImageMaintenanceRequestV1=Readonly<{version:1;wait:boolean}>;
/** 📎️ Distinguishes retained readers from an actual exclusive maintenance result. */
export type NativeImageMaintenanceResultV1<T>=Readonly<{acquired:false}|{acquired:true;value:T}>;
/** 🛂️ Refuses undeclared image maintenance authority before releasing a reader. */
export function readNativeImageMaintenanceRequestV1(value:unknown):NativeImageMaintenanceRequestV1{if(validateJsonSchemaSubset(schema.definitions.ImageMaintenanceRequest,value,schema).length)throw Error("Closed original image maintenance request required");return Object.freeze({version:1,wait:(value as NativeImageMaintenanceRequestV1).wait});}

/** 💾️ Retains one enforcing native fixed-volume identity and original usage custody. */
export interface NativeImageStorageV1{
 readonly root:string;
 readonly image:string;
 readonly maximumPhysicalBytes:number;
 readonly filesystemBytes:number;
 readonly port:NativeStoragePortV1;
 recheck():Promise<void>;
 close():Promise<void>;
 withExclusiveMaintenance<T>(request:NativeImageMaintenanceRequestV1,body:(port:NativeStoragePortV1)=>Promise<T>):Promise<NativeImageMaintenanceResultV1<T>>;
}

/** 🧭️ Resolves the compact canonical mount namespace without a path alias. */
export function nativeImageRootV1(directory:string):string{return requireMaterializedPathV1(join(directory,protocol.mountDirectory));}

/** 📂️ Rejects symlinks and paths escaping the enforced filesystem before materialization. */
export function requireNativeStoragePathV1(root:string,path:string):string{
 root=resolve(root);path=requireMaterializedPathV1(path);const suffix=relative(root,path);if(suffix===".."||suffix.startsWith(".."+sep)||isAbsolute(suffix))throw Error("Native storage path escapes enforcing boundary");
 const owner=lstatSync(root);if(!owner.isDirectory()||owner.isSymbolicLink())throw Error("Native storage root identity changed");const device=owner.dev,parts=suffix.split(/[\\/]/u).filter(Boolean);
 let current=root;for(const[index,part]of parts.entries()){current=join(current,part);let status;try{status=lstatSync(current);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}if(status.isSymbolicLink())throw Error("Native storage symlink escapes enforcing boundary");if(status.dev!==device)throw Error("Native storage path crosses filesystem boundary");if(index<parts.length-1&&!status.isDirectory())throw Error("Native storage ancestor must be an original directory");}return path;
}


/** 📏️ Refuses missing, mutable or non-finite physical envelopes before image or lease side effects. */
export function requireNativeImageEnvelopeV1(options:Readonly<{filesystemBytes:number;maximumPhysicalBytes:number}>):void{if(validateJsonSchemaSubset(schema.definitions.ImageRequest,{filesystemBytes:options.filesystemBytes,maximumPhysicalBytes:options.maximumPhysicalBytes},schema).length||options.filesystemBytes%protocol.unitBytes!==0||options.maximumPhysicalBytes<options.filesystemBytes+protocol.unitBytes)throw Error("Invalid fixed image allocation envelope");}
