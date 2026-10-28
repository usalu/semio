import {AsyncLocalStorage} from "node:async_hooks";
import {existsSync} from "node:fs";
import type {ResourceLeaseControl} from "../../../../../🔒️leases/🟦️.ts";
import {dirname,join} from "node:path";
import {nativeStorageBackendsV1,nativeStorageEnvironmentV1,selectNativeStorageBackendV1,type NativeStorageBackendDeclarationV1,type NativeStorageSelectionV1} from "../🧭️selection/🟦️.ts";
import type {NativeStorageGeometryV1,NativeStorageOperationsV1} from "./🔌️operations/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
export {nativeStorageBackendsV1,selectNativeStorageBackendV1,type NativeStorageBackendDeclarationV1,type NativeStorageSelectionV1,type NativeStorageGeometryV1,type NativeStorageOperationsV1};

const refusals=schema.definitions.Protocol.const.refusals;
const operations=new Map<string,NativeStorageOperationsV1>(),geometries=new Map<string,NativeStorageGeometryV1>();
const scoped=new AsyncLocalStorage<NativeStorageSelectionV1>();
const list=(text:string|undefined)=>text?text.split(",").map(item=>item.trim()).filter(Boolean):undefined;
const declaration=(id:string)=>{const row=nativeStorageBackendsV1.find(item=>item.id===id);if(!row)throw Error("Declared native storage backend required: "+id);return row;};

/** 🔗️ Binds the operations of one declared backend when its adapter module loads; a second, different binding is a wiring fault. */
export function bindNativeStorageOperationsV1(id:string,value:NativeStorageOperationsV1):void{
 declaration(id);if(operations.has(id)&&operations.get(id)!==value)throw Error("Native storage backend adapter bound twice: "+id);
 operations.set(id,Object.freeze(value));
}

/** 📐️ Binds the volume measurements of one declared backend when its implementation module loads. */
export function bindNativeStorageGeometryV1(id:string,value:NativeStorageGeometryV1):void{
 declaration(id);if(geometries.has(id)&&geometries.get(id)!==value)throw Error("Native storage geometry bound twice: "+id);
 geometries.set(id,Object.freeze(value));
}

/** 🎚️ Reads the effective selection: the enclosing scope, then the declared environment variables, then the host platform. */
export function nativeStorageSelectionV1(environment:Readonly<Record<string,string|undefined>>=process.env,platform:string=process.platform):NativeStorageSelectionV1{
 const scope=scoped.getStore(),names=nativeStorageEnvironmentV1,backend=scope?.backend??environment[names.backend],require=scope?.require??list(environment[names.require]),prefer=scope?.prefer??list(environment[names.prefer]);
 return Object.freeze({platform:scope?.platform??platform,...(backend?{backend}:{}),...(require?{require}:{}),...(prefer?{prefer}:{})});
}

/** 🏃️ Runs a body, including everything it awaits, under an explicit selection; concurrent bodies keep their own. */
export function withNativeStorageSelectionV1<T>(selection:NativeStorageSelectionV1,body:()=>T):T{return scoped.run(Object.freeze({...nativeStorageSelectionV1(),...selection}),body);}

/** 📜️ Declares the backend that the effective selection (completed by the given one) chooses. */
export function selectedNativeStorageDeclarationV1(selection:NativeStorageSelectionV1={}):NativeStorageBackendDeclarationV1{return selectNativeStorageBackendV1({...nativeStorageSelectionV1(),...selection});}

/** 🔌️ Returns the bound operations of a declared backend, or of the selected one. */
export function nativeStorageOperationsV1(id:string=selectedNativeStorageDeclarationV1().id):NativeStorageOperationsV1{
 declaration(id);const value=operations.get(id);if(!value)throw Error(refusals.unbound+": "+id);return value;
}

/** 🔑️ Names the row of every platform-keyed schema table that the selected backend reads on this host. */
export function nativeStorageDispatchKeyV1(selection:NativeStorageSelectionV1={}):"darwin"|"linux"|"win32"{
 const effective={...nativeStorageSelectionV1(),...selection},key=selectNativeStorageBackendV1(effective).dispatch[effective.platform!];
 if(key!=="darwin"&&key!=="linux"&&key!=="win32")throw Error("Declared native storage dispatch key required");return key;
}

/** 🗄️ Names the row of every platform-keyed schema table for the partition stored in a directory on this host. */
export function nativeStorageDispatchKeyAtV1(directory:string,platform:string=nativeStorageSelectionV1().platform!):"darwin"|"linux"|"win32"{
 const key=nativeStorageBackendOfDirectoryV1(directory).dispatch[platform];
 if(key!=="darwin"&&key!=="linux"&&key!=="win32")throw Error("Declared native storage dispatch key required");return key;
}

/** 🏷️ Names the physical backing file of a declared backend (default: the selected one). */
export function nativeStorageBackingFilenameV1(id?:string):string{return id===undefined?selectedNativeStorageDeclarationV1().backingFilename:declaration(id).backingFilename;}

/** 🧿️ Tells whether the selected backend keeps its partitions as kernel-mounted volumes. */
export function nativeStorageMountedVolumeV1(selection:NativeStorageSelectionV1={}):boolean{return selectedNativeStorageDeclarationV1(selection).capabilities.includes("mounted-volume");}

/** 🗃️ Tells whether the partition stored in a directory is a kernel-mounted volume, by the backing file lying in it (the selected backend when none does). */
export function nativeStorageMountedAtV1(directory:string):boolean{return nativeStorageBackendOfDirectoryV1(directory).capabilities.includes("mounted-volume");}

/** 🌍️ Tells whether any declared backend with kernel-mounted volumes exists on this host, whatever is selected. */
export function nativeStorageHostMountsVolumesV1(platform:string=nativeStorageSelectionV1().platform!):boolean{return nativeStorageBackendsV1.some(row=>row.platforms.includes(platform)&&row.capabilities.includes("mounted-volume"));}

/** 🔎️ Finds the backend whose backing file lies in a directory, else the one the given selection (default: the effective one) chooses. */
export function nativeStorageBackendOfDirectoryV1(directory:string,selection:NativeStorageSelectionV1={}):NativeStorageBackendDeclarationV1{return nativeStorageBackendsV1.find(row=>existsSync(join(directory,row.backingFilename)))??selectedNativeStorageDeclarationV1(selection);}

const geometry=(id:string)=>{declaration(id);const value=geometries.get(id);if(!value)throw Error(refusals.unbound+": "+id);return value;};
const ofImage=(image:string)=>nativeStorageBackendsV1.find(row=>row.backingFilename===image.split(/[\\/]/u).pop())??selectedNativeStorageDeclarationV1();
/** 🧲️ Tells whether the root shows the attached volume of the backing file, by the backend that owns that file. */
export const nativeStorageAttachedV1=(image:string,root:string):boolean=>geometry(ofImage(image).id).attached(image,root);
/** 💻️ Reports the whole-volume capacity of the backend that owns the root's backing file. */
export const nativeStorageFilesystemBytesV1=(root:string,declared:number):number=>geometry(nativeStorageBackendOfDirectoryV1(dirname(root)).id).filesystemBytes(root,declared);
/** 🧮️ Reports the bytes in use below the root by the backend that owns its backing file. */
export const nativeStorageUsedBytesV1=(root:string):number=>geometry(nativeStorageBackendOfDirectoryV1(dirname(root)).id).usedBytes(root);
/** 🧹️ Drops the cached host attachment facts of every bound backend after the physical state changed. */
export const invalidateNativeStorageRostersV1=():void=>{for(const value of geometries.values())value.invalidate();};
/** 🗺️ Tells whether the host still maps the backing file or the volume root of any given directory, by the backend that owns the first directory's backing file. */
export const nativeStorageMappedV1=(directories:readonly string[],control:ResourceLeaseControl):Promise<boolean>=>geometry(nativeStorageBackendOfDirectoryV1(directories[0]!).id).mapped(directories,control);
/** 🪣️ Reports allocation unit and free units of the backend that owns the root's backing file. */
export const nativeStorageSpaceV1=(root:string,declared:number):Readonly<{bsize:number;bavail:number}>=>geometry(nativeStorageBackendOfDirectoryV1(dirname(root)).id).space(root,declared);
