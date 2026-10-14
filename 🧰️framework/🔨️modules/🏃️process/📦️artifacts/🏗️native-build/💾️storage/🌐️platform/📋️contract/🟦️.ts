import {lstatSync} from "node:fs";
import {join,resolve,relative,isAbsolute,sep} from "node:path";
import {requireMaterializedPathV1} from "../../../../../🔒️leases/🟦️.ts";
import type {NativeStoragePortV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
const protocol=schema.definitions.ImageProtocol.const;

/** 💾️ Retains one enforcing native fixed-volume identity and original usage custody. */
export interface NativeImageStorageV1{
 readonly root:string;
 readonly image:string;
 readonly maximumPhysicalBytes:number;
 readonly filesystemBytes:number;
 readonly port:NativeStoragePortV1;
 recheck():Promise<void>;
 close():Promise<void>;
 withExclusiveMaintenance<T>(body:(port:NativeStoragePortV1)=>Promise<T>):Promise<T>;
}

/** 🧭️ Resolves the compact canonical mount namespace without a path alias. */
export function nativeImageRootV1(directory:string):string{return requireMaterializedPathV1(join(directory,protocol.mountDirectory));}

/** 📂️ Rejects symlinks and paths escaping the enforced filesystem before materialization. */
export function requireNativeStoragePathV1(root:string,path:string):string{
 root=resolve(root);path=requireMaterializedPathV1(path);const suffix=relative(root,path);if(suffix===".."||suffix.startsWith(".."+sep)||isAbsolute(suffix))throw Error("Native storage path escapes enforcing boundary");
 const owner=lstatSync(root);if(!owner.isDirectory()||owner.isSymbolicLink())throw Error("Native storage root identity changed");const device=owner.dev,parts=suffix.split(/[\\/]/u).filter(Boolean);
 let current=root;for(const[index,part]of parts.entries()){current=join(current,part);let status;try{status=lstatSync(current);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}if(status.isSymbolicLink())throw Error("Native storage symlink escapes enforcing boundary");if(status.dev!==device)throw Error("Native storage path crosses filesystem boundary");if(index<parts.length-1&&!status.isDirectory())throw Error("Native storage ancestor must be an original directory");}return path;
}

