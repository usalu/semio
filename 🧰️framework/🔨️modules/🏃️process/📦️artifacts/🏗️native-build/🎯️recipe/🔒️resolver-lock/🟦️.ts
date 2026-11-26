import {chmodSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {basename,isAbsolute,join,resolve} from "node:path";
import {readPhysicalFileV1} from "../../../../../📁️filesystem/🧾️observation/🟦️.ts";
import {concurrentEditV1,retryOnConcurrentEditV1} from "../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {projectCargoLockClosureV1} from "../../../../🦀️cargo-workspace/🔒️lock/📐️closure/🟦️.ts";
import schema from "../../../../🦀️cargo-workspace/🔒️lock/📐️closure/🧬️schema/🔣️.json" with {type:"json"};
import {checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "../../../../🔒️leases/🟦️.ts";
import {requireIssuedResourceControlRefinementV1} from "../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {cargoProvenancePhysicalControlV1} from "../../🧾️receipt/🟦️.ts";
import {requireNativeStorageReservationV1,type NativeStorageReservationV1} from "../../💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../../💾️storage/🌐️platform/📋️contract/🟦️.ts";

export const CARGO_RESOLVER_LOCK_KEY_V1="resolver.lockfile-path=";
type Scope={control:ResourceLeaseControl;storage:NativeStorageReservationV1;source:string;path:string;digest:string;closure:string;roots:readonly string[]};
declare const brand:unique symbol;
export type CargoResolverLockV1=Readonly<{[brand]:true}>;
const scopes=new WeakMap<CargoResolverLockV1,Scope>();

const physical=(control:ResourceLeaseControl)=>({
 ...cargoProvenancePhysicalControlV1({cancelled:()=>control.signal.aborted,remainingMs:()=>checkResourceLeaseControl(control),onProgress:()=>checkResourceLeaseControl(control)}),
 maxBytes:schema.definitions.Protocol.const.maximumBytes,
});
const readLive=(path:string,control:ResourceLeaseControl)=>retryOnConcurrentEditV1("snapshot",()=>readPhysicalFileV1(path,physical(control)),{signal:control.signal});

/** 🔐️ Retains one exact workspace lock within the admitted compiler store until planning, build and publication settle. */
export async function withCargoResolverLockV1<T>(
 workspace:string,directory:string,roots:readonly string[],storage:NativeStorageReservationV1,control:ResourceLeaseControl,body:(lock:CargoResolverLockV1)=>Promise<T>,
):Promise<T>{
 requireNativeStorageReservationV1(storage,control);
 if(storage.allocation.kind!=="compiler-workspace"||!roots.length)throw Error("Selected compiler workspace and Cargo lock roots required");
 await storage.recheck();
 const source=join(resolve(workspace),"Cargo.lock"),observed=await readLive(source,control);
 requireNativeStoragePathV1(storage.backing.root,directory);
 mkdirSync(directory,{recursive:true});
 const temporary=mkdtempSync(join(directory,"lk-")),path=join(temporary,"Cargo.lock");
 const lock=Object.freeze({}) as CargoResolverLockV1;
 try{
  requireMaterializedPathV1(requireNativeStoragePathV1(storage.backing.root,path));
  checkResourceLeaseControl(control);
  writeFileSync(path,observed.bytes,{flag:"wx",mode:0o400});
  scopes.set(lock,{control,storage,source,path,digest:observed.claim.sha256,closure:projectCargoLockClosureV1(observed.bytes,roots).digest,roots:Object.freeze([...roots])});
  return await body(lock);
 }finally{
  scopes.delete(lock);
  if(process.platform==="win32")try{chmodSync(path,0o600);}catch{}
  rmSync(temporary,{recursive:true,force:true});
 }
}

/** 🪪️ Projects only the actual live compiler-owned resolver lock. */
export function cargoResolverLockViewV1(lock:CargoResolverLockV1,control:ResourceLeaseControl):Readonly<{source:string;path:string;argument:string;digest:string}>{
 const scope=scopes.get(lock);
 if(!scope)throw Error("Live owned Cargo resolver lock required");
 requireIssuedResourceControlRefinementV1(scope.control,control);
 requireNativeStorageReservationV1(scope.storage,control);
 return Object.freeze({source:scope.source,path:scope.path,argument:CARGO_RESOLVER_LOCK_KEY_V1+JSON.stringify(scope.path),digest:scope.digest});
}

/** 📎️ Adds the private selector after ordinary Cargo argument admission, before any compiler-tail arguments. */
export function cargoResolverLockArgumentsV1(lock:CargoResolverLockV1,control:ResourceLeaseControl,args:readonly string[]):string[]{
 const selector=cargoResolverLockViewV1(lock,control).argument;
 const tail=args.indexOf("--"),split=tail<0?args.length:tail;
 return [...args.slice(0,split),"--config",selector,...args.slice(split)];
}

/** 📄️ Reads the exact alternate lock selected by a recorded Cargo command. */
export function cargoResolverLockPathV1(args:readonly string[]):string|undefined{
 const tail=args.indexOf("--"),selected=tail<0?args:args.slice(0,tail);
 const paths=selected.map((arg,index)=>arg==="--config"?selected[index+1]:arg.startsWith("--config=")?arg.slice(9):undefined).filter((arg):arg is string=>arg?.startsWith(CARGO_RESOLVER_LOCK_KEY_V1)===true);
 if(!paths.length)return;
 if(paths.length!==1)throw Error("One exact recorded Cargo resolver lock required");
 const path:unknown=JSON.parse(paths[0]!.slice(CARGO_RESOLVER_LOCK_KEY_V1.length));
 if(typeof path!=="string"||!isAbsolute(path)||resolve(path)!==path||basename(path)!=="Cargo.lock")throw Error("Canonical recorded Cargo resolver lock required");
 return path;
}

/** 🔎️ Checks exact private bytes and that the shared workspace still selects the same lock closure. */
export async function recheckCargoResolverLockV1(lock:CargoResolverLockV1,control:ResourceLeaseControl):Promise<void>{
 const view=cargoResolverLockViewV1(lock,control),scope=scopes.get(lock)!;
 await scope.storage.recheck();
 const pinned=await readPhysicalFileV1(view.path,physical(control));
 if(pinned.claim.sha256!==view.digest)throw Error("Owned Cargo resolver lock bytes changed");
 const real=await readLive(view.source,control);
 if(projectCargoLockClosureV1(real.bytes,scope.roots).digest!==scope.closure)throw concurrentEditV1("input",view.source,"Cargo.lock changed in the packages the build depends on","build");
}
