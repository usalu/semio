import type{AcceptedPhysicalCleanupV1}from"../../../../🧭️routing/📥️invocation/🏃️process/🧹️cleanup/🟦️.ts";
import type{ResourceLeaseControl}from"../../../../🔒️leases/🎛️control/🟦️.ts";
import type{NativeStorageAllocationV1,NativeStorageBackingV1,NativeStoragePolicyV1,NativeStorageStateV1}from"../📜️journal/🟦️.ts";
/** 🔌️ Native storage port contracts: the enforcing port, its cleanup capture and the detached journal observation; types only, so platform contracts depend on this leaf and never on the storage index. */
export type NativeStorageCleanupAccessV1=Readonly<{backing:NativeStorageBackingV1;recheck():void;syncMetadata(directory:string):Promise<void>}>;
/** 🛟️ Exists only when privately captured from the original enforcing port before its body ends. */
export interface NativeStorageCleanupPortV1{withOperation<T>(body:(access:NativeStorageCleanupAccessV1)=>Promise<T>):Promise<T>;close():Promise<void>;}
/** 🔌️ Owns actual platform enforcement, identity revalidation and complete physical retirement. */
export interface NativeStoragePortV1{
 withJournal<T>(control:ResourceLeaseControl,run:(actualControl:ResourceLeaseControl)=>Promise<T>):Promise<T>;
 captureCleanup?(enrollment:AcceptedPhysicalCleanupV1,bodyControl:ResourceLeaseControl):NativeStorageCleanupPortV1;
 withOperation<T>(control:ResourceLeaseControl,run:(actualControl:ResourceLeaseControl)=>Promise<T>):Promise<T>;
 observe(policy:NativeStoragePolicyV1,control:ResourceLeaseControl):Promise<NativeStorageBackingV1|null>;
 recheck(backing:NativeStorageBackingV1,control:ResourceLeaseControl):Promise<void>;
 syncMetadata(backing:NativeStorageBackingV1,directory:string,control:ResourceLeaseControl):Promise<void>;
 retire(allocation:NativeStorageAllocationV1,backing:NativeStorageBackingV1,control:ResourceLeaseControl):Promise<void>;
}
export type NativeStorageObservationPortV1=Pick<NativeStoragePortV1,"withOperation"|"observe"|"recheck">&Readonly<{check(control:ResourceLeaseControl):void}>;
/** 📚️ Returns detached current journal DATA, never an allocation or a writer continuation. */
export type NativeStorageJournalObservationV1=Readonly<{version:1;backing:NativeStorageBackingV1;state:NativeStorageStateV1;journalSha256:string;journalBytes:number}>;
/** 🧱️ The directory, lease root, byte bounds and caller control one fixed native platform image is opened with. */
export type NativeFixedPlatformOptionsV1=Readonly<{directory:string;leaseDirectory:string;filesystemBytes:number;maximumPhysicalBytes:number;control:ResourceLeaseControl;maximumLeaseDatabaseBytes?:number}>;
