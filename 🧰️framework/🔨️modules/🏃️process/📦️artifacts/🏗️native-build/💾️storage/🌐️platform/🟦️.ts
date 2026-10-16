import{requireIssuedResourceControlRefinementV1}from"../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import type{AcceptedPhysicalCleanupV1}from"../../../../🧭️routing/📥️invocation/🏃️process/🧹️cleanup/🟦️.ts";
import{withPersistentNativeDisposableChildStorageV1,requirePersistentNativeImageCloseV1,withPersistentNativeChildStorageV1,withPersistentNativeMaintenanceChildStorageV1,capturePersistentNativeImageCleanupV1,withPersistentNativeImageCleanupReadV1}from"../🧹️cleanup/🎛️command/🟦️.ts";
import{withFixedNativeDisposableChildStorageV1,requireFixedNativeImageCloseV1,withFixedNativeChildStorageV1,withFixedNativeMaintenanceChildStorageV1,captureFixedNativeImageCleanupV1,withFixedNativeImageCleanupReadV1}from"../🧱️fixed/🟦️.ts";
import type{NativeImageRetainedCleanupV1,NativeImageCleanupFactsV1,NativeImageCleanupReadV1}from"../🧹️cleanup/♻️retirement/📋️contract/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../🔒️leases/🟦️.ts";
import type {NativeFixedPlatformOptionsV1} from "../🧱️fixed/🟦️.ts";
import {openNativeImageStorageV1,retireNativeImageStorageV1} from "../🍎️image/🟦️.ts";
import {openNativeLinuxImageStorageV1,retireNativeLinuxImageStorageV1} from "../🐧️image/🟦️.ts";
import {openNativeWindowsImageStorageV1,retireNativeWindowsImageStorageV1} from "../🪟️image/🟦️.ts";
import type {NativeImageStorageV1,NativeImageMaintenanceRequestV1,NativeImageRetirementRequestV1,NativeImageMaintenanceCustodyV1} from "./📋️contract/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
export {nativeImageRootV1,requireNativeStoragePathV1} from "./📋️contract/🟦️.ts";
export type {NativeImageStorageV1,NativeImageMaintenanceRequestV1,NativeImageRetirementRequestV1};

/** 🧭️ Selects the enforcing backend for one schema-declared desktop host. */
export function nativeStoragePlatformV1(platform:string):"macos"|"linux"|"windows"{if(!schema.enum.includes(platform))throw Error("Declared native fixed storage platform required");return platform==="darwin"?"macos":platform==="linux"?"linux":"windows";}

/** 🌐️ Opens the host's actual finite backing while preserving original system capability admission. */
export async function openNativePlatformStorageV1(options:NativeFixedPlatformOptionsV1):Promise<NativeImageStorageV1>{options=Object.freeze({...options});const backend=nativeStoragePlatformV1(process.platform);const image=await(backend==="macos"?openNativeImageStorageV1(options):backend==="linux"?openNativeLinuxImageStorageV1(options):openNativeWindowsImageStorageV1(options));return registerNativePlatformImageV1(image,options);}

/** ♻️ Retires only an exclusively held exact native backing through its declared host protocol. */
export async function retireNativePlatformStorageV1(options:NativeFixedPlatformOptionsV1,request?:NativeImageRetirementRequestV1):Promise<boolean>{requireNativeImageEnvelopeV1(options);const admitted=readNativeImageRetirementRequestV1(request??{version:1,wait:true}),backend=nativeStoragePlatformV1(process.platform);return backend==="macos"?retireNativeImageStorageV1(options,admitted):backend==="linux"?retireNativeLinuxImageStorageV1(options,admitted):retireNativeWindowsImageStorageV1(options,admitted);}

import {requireNativeImageEnvelopeV1,readNativeImageRetirementRequestV1} from "./📋️contract/🟦️.ts";

const platformImages=new WeakMap<NativeImageStorageV1,Readonly<{control:ResourceLeaseControl;leaseDirectory:string;root:string;image:string;maximumPhysicalBytes:number;filesystemBytes:number;port:NativeImageStorageV1["port"];recheck:NativeImageStorageV1["recheck"];close:NativeImageStorageV1["close"];maintenance:NativeImageStorageV1["withExclusiveMaintenance"]}>>();
/** 🪢️ Retains only the actual canonical platform result through its authentic Native operation. */
export function requireNativePlatformImageV1(image:NativeImageStorageV1,control:ResourceLeaseControl):void{const scope=platformImages.get(image);if(!scope||scope.root!==image.root||scope.image!==image.image||scope.maximumPhysicalBytes!==image.maximumPhysicalBytes||scope.filesystemBytes!==image.filesystemBytes||scope.port!==image.port||scope.recheck!==image.recheck||scope.close!==image.close||scope.maintenance!==image.withExclusiveMaintenance)throw Error("Exact canonical Native image required");requireIssuedResourceControlRefinementV1(scope.control,control);}
/** 🪢️ Joins actual Native descendants beneath the captured canonical image. */
export function withNativePlatformImageOperationV1<T>(image:NativeImageStorageV1,control:ResourceLeaseControl,body:(actualControl:ResourceLeaseControl)=>Promise<T>):Promise<T>{requireNativePlatformImageV1(image,control);return platformImages.get(image)!.port.withOperation(control,body);}
const platformCleanups=new WeakMap<NativeImageRetainedCleanupV1,Readonly<{backend:"macos"|"linux"|"windows";token:NativeImageRetainedCleanupV1}>>();
/** 🛟️ Binds genuine canonical image custody to its exact already funded body before cleanup starts. */
export function captureNativePlatformImageCleanupV1(image:NativeImageStorageV1,body:ResourceLeaseControl,enrollment:AcceptedPhysicalCleanupV1):NativeImageRetainedCleanupV1{requireNativePlatformImageV1(image,body);const backend=nativeStoragePlatformV1(process.platform),actual=backend==="macos"?capturePersistentNativeImageCleanupV1(image,body,enrollment):captureFixedNativeImageCleanupV1(image,body,enrollment),token=Object.freeze({})as NativeImageRetainedCleanupV1;platformCleanups.set(token,Object.freeze({backend,token:actual}));return token;}
/** 📖️ Dispatches only privately issued provider custody; immutable observations never become a Native port. */
export function withNativePlatformImageCleanupReadV1<T>(token:NativeImageRetainedCleanupV1,body:(read:NativeImageCleanupReadV1)=>Promise<T>):Promise<T>{const scope=platformCleanups.get(token);if(!scope)throw Error("Actual canonical Native cleanup owner required");return scope.backend==="macos"?withPersistentNativeImageCleanupReadV1(scope.token,body):withFixedNativeImageCleanupReadV1(scope.token,body);}

/** 📋️ Observes backing only from the actual factory envelope retained with the canonical image. */
export async function readNativePlatformImageBackingV1(image:NativeImageStorageV1,control:ResourceLeaseControl):Promise<import("../🟦️.ts").NativeStorageBackingV1>{requireNativePlatformImageV1(image,control);const scope=platformImages.get(image)!,backing=await scope.port.observe({version:1,root:scope.root,leaseDirectory:scope.leaseDirectory,capacityBytes:scope.maximumPhysicalBytes,maximumWorkspaceBytes:scope.filesystemBytes,maximumArtifactBytes:scope.filesystemBytes},control);requireNativePlatformImageV1(image,control);if(!backing||backing.root!==scope.root||backing.maximumPhysicalBytes!==scope.maximumPhysicalBytes||typeof backing.identity!=="string"||!backing.identity)throw Error("Exact actual platform backing required");return Object.freeze({...backing});}

function registerNativePlatformImageV1(image:NativeImageStorageV1,options:NativeFixedPlatformOptionsV1):NativeImageStorageV1{Object.freeze(image.port);Object.freeze(image);platformImages.set(image,Object.freeze({control:options.control,leaseDirectory:options.leaseDirectory,root:image.root,image:image.image,maximumPhysicalBytes:image.maximumPhysicalBytes,filesystemBytes:image.filesystemBytes,port:image.port,recheck:image.recheck,close:image.close,maintenance:image.withExclusiveMaintenance}));return image;}
/** 🪢️ Opens only beneath an actual factory-issued parent retained through complete child close. */
export function withNativePlatformChildStorageV1<T>(parent:NativeImageStorageV1,input:NativeFixedPlatformOptionsV1,body:(child:NativeImageStorageV1,control:ResourceLeaseControl)=>Promise<T>):Promise<T>{const options=Object.freeze({...input}),owner=parent;checkResourceLeaseControl(options.control);requireNativePlatformImageV1(owner,options.control);if(typeof body!=="function")throw Error("Actual Native child body required");const run=nativeStoragePlatformV1(process.platform)==="macos"?withPersistentNativeChildStorageV1:withFixedNativeChildStorageV1;return run(owner,options,(child,actual)=>body(registerNativePlatformImageV1(child,Object.freeze({...options,control:actual})),actual));}
/** 🏦️ Joins exact privately registered parent EX custody without a new shared acquisition. */
export function withNativePlatformMaintenanceChildStorageV1<T>(parent:NativeImageStorageV1,custody:NativeImageMaintenanceCustodyV1,input:NativeFixedPlatformOptionsV1,body:(child:NativeImageStorageV1,control:ResourceLeaseControl)=>Promise<T>):Promise<T>{const options=Object.freeze({...input}),owner=parent,held=custody;checkResourceLeaseControl(options.control);requireNativePlatformImageV1(owner,options.control);if(typeof body!=="function")throw Error("Actual Native maintenance child body required");const run=nativeStoragePlatformV1(process.platform)==="macos"?withPersistentNativeMaintenanceChildStorageV1:withFixedNativeMaintenanceChildStorageV1;return run(owner,held,options,(child,actual)=>body(registerNativePlatformImageV1(child,Object.freeze({...options,control:actual})),actual));}

/** 🛑️ Checks actual registered parent context before any caller-owned close admission or drainage changes. */
export function requireNativePlatformImageCloseV1(image:NativeImageStorageV1):void{if(!platformImages.has(image))throw Error("Actual Native platform image required");if(nativeStoragePlatformV1(process.platform)==="macos")requirePersistentNativeImageCloseV1(image);else requireFixedNativeImageCloseV1(image);}

import{bindNativeDisposableImageAllocationV1,type NativeDisposableImageV1}from"../🧹️cleanup/♻️retirement/📋️contract/🟦️.ts";
import type{NativeStorageReservationV1}from"../🟦️.ts";
export type{NativeDisposableImageV1}from"../🧹️cleanup/♻️retirement/📋️contract/🟦️.ts";
/** 🧪️ Retains a real parent through pre-funded disposable birth, body drain and exact terminal retirement. */
export function withNativePlatformDisposableChildStorageV1<T>(parent:NativeImageStorageV1,input:NativeFixedPlatformOptionsV1,body:(child:NativeImageStorageV1,control:ResourceLeaseControl,owner:NativeDisposableImageV1)=>Promise<T>):Promise<T>{const options=Object.freeze({...input}),image=parent;checkResourceLeaseControl(options.control);requireNativePlatformImageV1(image,options.control);if(typeof body!=="function")throw Error("Actual disposable child body required");const run=nativeStoragePlatformV1(process.platform)==="macos"?withPersistentNativeDisposableChildStorageV1:withFixedNativeDisposableChildStorageV1;return run(image,options,(child,actual,owner)=>body(registerNativePlatformImageV1(child,Object.freeze({...options,control:actual})),actual,owner));}
/** 🪢️ Binds only the real child reservation to its exact once-only live terminal owner. */
export function bindNativePlatformDisposableAllocationV1(owner:NativeDisposableImageV1,reservation:NativeStorageReservationV1,control:ResourceLeaseControl):void{bindNativeDisposableImageAllocationV1(owner,reservation,control);}
