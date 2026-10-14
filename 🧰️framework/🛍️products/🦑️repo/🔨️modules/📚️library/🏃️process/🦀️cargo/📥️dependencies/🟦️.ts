import {admitRepositoryCargoStorageArgumentsV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/📥️arguments/🟦️.ts";
import {randomBytes} from "node:crypto";
import {resolve,join} from "node:path";
import {acquireQueuedResourceLease,checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "./../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {requireNativeStorageReservationV1,type NativeStorageReservationV1} from "./../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "./../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {captureOwnedCommandV1} from "./../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {validateJsonSchemaSubset} from "./../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type {ScriptInvocation} from "./../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {readNativeCallerAuthorityV1} from "./../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import schema from "./🧬️schema/🔣️.json";

/** 🌐️ Retains the original caller's explicit offline grant without ambient network inference. */
export interface CargoDependencyNetworkV1 {readonly offline:boolean}
/** 📥️ Names one bounded registry owner and the exact original manifest selection. */
export interface RepositoryCargoInputsV1 {readonly workspace:string;readonly manifest:string;readonly args:readonly string[];readonly additionalManifests?:readonly Readonly<{manifest:string;args:readonly string[]}>[];readonly network:CargoDependencyNetworkV1;readonly control:ResourceLeaseControl;readonly storage:NativeStorageReservationV1;readonly environment:Readonly<Record<string,string|undefined>>}

/** 🔐️ Projects only an actual native caller's authored network grant. */
export function repositoryCargoNetworkV1(invocation:ScriptInvocation,workspace:string):CargoDependencyNetworkV1{const authority=readNativeCallerAuthorityV1(invocation.capabilities);if(resolve(authority.repositoryRoot)!==resolve(workspace))throw Error("Original Cargo network grant belongs to another workspace");return authority.network;}

/** 📋️ Fetches the locked original manifest for every platform its lockfile resolves, respecting its unchanged offline grant: compiler planning reads
 * `cargo metadata` over the complete resolve (host build scripts and proc-macros included), so a target-filtered fetch could never be planned offline.
 * @see ../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts */
export function cargoDependencyFetchArgumentsV1(manifest:string,args:readonly string[],network:CargoDependencyNetworkV1):string[]{
 if(validateJsonSchemaSubset(schema,network).length)throw Error("Original Cargo dependency network grant required");manifest=requireMaterializedPathV1(manifest);const result=["fetch","--locked","--manifest-path",manifest];if(network.offline||args.includes("--offline")||args.includes("--frozen"))result.push("--offline");return result;
}

/** 🧾️ Admits the bounded registry home, temporary and writer namespaces before any registry child observes them. */
async function admitRepositoryCargoInputsV1(options:RepositoryCargoInputsV1){
 const {control,storage}=options;checkResourceLeaseControl(control);const selections=[{manifest:options.manifest,args:options.args},...(options.additionalManifests??[])];if(selections.length>64)throw Error("Original Cargo dependency selection limit exceeded");const requests=selections.map(row=>({manifest:row.manifest,args:cargoDependencyFetchArgumentsV1(row.manifest,row.args,options.network)})),args=requests[0]!.args;requireNativeStorageReservationV1(storage,control);if(storage.allocation.kind!=="compiler-workspace")throw Error("Original dependency compiler storage required");await storage.recheck();const home=options.environment.CARGO_HOME;if(!home)throw Error("Bounded Cargo dependency home required");requireNativeStoragePathV1(storage.backing.root,home);for(const name of ["TMPDIR","TMP","TEMP"]){const path=options.environment[name];if(!path)throw Error("Bounded dependency temporary storage required");requireNativeStoragePathV1(storage.backing.root,path);}
 const buildDirectory=options.environment.CARGO_BUILD_BUILD_DIR,targetDirectory=options.environment.CARGO_TARGET_DIR;if(!buildDirectory||!targetDirectory)throw Error("Bounded Cargo compiler writer environment required");for(const row of selections)await admitRepositoryCargoStorageArgumentsV1(row.args,{version:1,compilerConfiguration:join(options.workspace,".cargo/config.toml"),buildDirectory,targetDirectory},storage,control);
 return{requests,home,environment:Object.freeze({...options.environment,CARGO_NET_OFFLINE:String(args.includes("--offline"))}),leaseOptions:{directory:join(options.workspace,".cache/b/l/m/data"),resource:"cargo:registry:"+home,signal:control.signal,control}};
}

/** 🏪️ Holds genuine bounded registry writer/read custody across fetch, planning and terminal compiler settlement. */
export async function withRepositoryCargoInputsV1<T>(options:RepositoryCargoInputsV1,body:(environment:Readonly<Record<string,string|undefined>>)=>Promise<T>):Promise<T>{
 const {control,storage}=options,{requests,home,environment,leaseOptions}=await admitRepositoryCargoInputsV1(options),writer=await acquireQueuedResourceLease({...leaseOptions,owner:randomBytes(8).toString("base64url"),mode:"exclusive"});
 try{await storage.recheck();process.stderr.write("[DEBUG] Original bounded Cargo dependency fetch "+JSON.stringify({manifest:options.manifest,offline:environment.CARGO_NET_OFFLINE==="true",home})+"\n");try{for(const request of requests){await storage.recheck();const output=await captureOwnedCommandV1("cargo",request.args,options.workspace,{control,env:{...environment,CARGO_NET_OFFLINE:String(request.args.includes("--offline"))},output:{maximumBytes:16777216,maximumLines:65536}});if(output)process.stderr.write(output);}}catch(error){if(environment.CARGO_NET_OFFLINE==="true")throw Error("Original offline Cargo dependency acquisition refused: "+(error instanceof Error?error.message:String(error)));throw error;}await storage.recheck();}finally{writer.release();}
 const reader=await acquireQueuedResourceLease({...leaseOptions,owner:randomBytes(8).toString("base64url"),mode:"shared",sharedCohort:true});try{await storage.recheck();return await body(environment);}finally{reader.release();}
}

/** 🔐️ Holds exclusive bounded registry writer custody for a lockfile resolution that must not first fetch its stale locked graph. */
export async function withRepositoryCargoResolutionV1<T>(options:RepositoryCargoInputsV1,body:(environment:Readonly<Record<string,string|undefined>>)=>Promise<T>):Promise<T>{
 const {storage}=options,{environment,leaseOptions}=await admitRepositoryCargoInputsV1(options),writer=await acquireQueuedResourceLease({...leaseOptions,owner:randomBytes(8).toString("base64url"),mode:"exclusive"});
 try{await storage.recheck();return await body(environment);}finally{writer.release();}
}
