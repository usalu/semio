/** 📋️ The vocabulary types the platform contract, the backend contract and the storage owner share, declared once below all three so that none of them imports another one's module for a type. */
import type {ResourceLeaseControl,ResourceLeaseWriterProofV1} from "../../../../🔒️leases/🟦️.ts";
import type {NativeStorageAllocationV1,NativeStorageBackingV1,NativeStoragePolicyV1} from "../📜️journal/🟦️.ts";
import type {NativeStoragePortV1} from "../🔌️port/🟦️.ts";
import type {ScriptInvocation} from "../../../../🧭️routing/📥️invocation/🟦️.ts";

declare const storageCleanupBrand:unique symbol;
declare const retirementBrand:unique symbol;
declare const imageCleanupBrand:unique symbol;
declare const disposableBrand:unique symbol;

/** 🧷️ Carries current physical CAS only; opaque receipt bytes grant no historical publication authority. */
export type NativeProtectedParentRequestV1=Readonly<{version:1;purpose:"mounted-parent-read";wait:false;receiptSha256:string;directory:Readonly<{dev:number;ino:number;mode:number}>;backing:Readonly<{dev:number;ino:number;size:number;allocatedBytes:number}>;root:Readonly<{dev:number;ino:number;mode:number}>}>;
/** 🚦️ Chooses required quiescence or an immediate optional-maintenance attempt. */
export type NativeImageMaintenanceRequestV1=Readonly<{version:1;wait:boolean;yielding?:true}>;
/** 🧲️ Binds optional historical backing authority to one exact stable receipt and physical allocation. */
export type NativeImageRetirementExpectationV1=Readonly<{receiptSha256:string;dev:number;ino:number;size:number;allocatedBytes:number}>;
/** ♻️ Separates historical retirement admission from reader maintenance. */
export type NativeImageRetirementRequestV1=Readonly<{version:1;wait:boolean;expectedImage?:NativeImageRetirementExpectationV1}>;
/** 📎️ Distinguishes retained readers from an actual exclusive maintenance result. */
export type NativeImageMaintenanceResultV1<T>=Readonly<{acquired:false;withdrawn?:true}|{acquired:true;value:T}>;
/** 📊️ Carries immutable privately verified free space without image or control authority. */
export type NativeImageMaintenanceWaitObservationV1=Readonly<{version:1;availableBytes:number}>;
/** ⏸️ Requests withdrawal of this queued writer, never admission or writable custody. */
export type NativeImageMaintenanceWaitObserverV1=(observation:NativeImageMaintenanceWaitObservationV1)=>boolean;
/** 🔐️ Distinguishes an actual retained kernel writer from a fresh private physical owner. */
export type NativeImageMaintenanceCustodyV1=Readonly<{kind:"kernel-ex";proof:ResourceLeaseWriterProofV1;control:ResourceLeaseControl}|{kind:"fresh-private"}>;
/** 💾️ Retains one enforcing native fixed-volume identity and original usage custody. */
export interface NativeImageStorageV1{
 readonly root:string;
 readonly image:string;
 readonly maximumPhysicalBytes:number;
 readonly filesystemBytes:number;
 readonly port:NativeStoragePortV1;
 recheck():Promise<void>;
 close():Promise<void>;
 withExclusiveMaintenance<T>(request:NativeImageMaintenanceRequestV1,body:(port:NativeStoragePortV1,custody:NativeImageMaintenanceCustodyV1)=>Promise<T>,withdrawWhen?:NativeImageMaintenanceWaitObserverV1):Promise<NativeImageMaintenanceResultV1<T>>;
}
/** 📖️ Carries observed descendant names as DATA without enrolling any child. */
export type NativeProtectedParentReadV1=Readonly<{descendants:readonly string[];port:import("../🔌️port/🟦️.ts").NativeStorageObservationPortV1;verify(control:ResourceLeaseControl):Promise<readonly string[]>}>;
/** 🏛️ Captures a distinct maintenance request without promoting an existing read purpose. */
export type NativeProtectedMaintenanceRequestV1=Omit<NativeProtectedParentRequestV1,"purpose">&Readonly<{purpose:"mounted-parent-maintenance"}>;
/** 🗺️ Describes one complete actual association as DATA; it grants no mapping or image owner. */
export type NativeProtectedMaintenanceAssociationV1=Readonly<{root:string;image:string;backingIdentity:string;kernelIdentity:string}>;
/** 📦️ Declares an independently owned current image; this DATA never creates a funded child history. */
export type NativeProtectedMaintenanceDeclarationV1=Readonly<{version:1;classification:"independent-domain-image";operation:"container"|"publish"|"retire"|"retain-inactive";directory:string;leaseDirectory:string;maximumLeaseDatabaseBytes?:number;filesystemBytes:number;maximumPhysicalBytes:number;current:NativeProtectedMaintenanceRequestV1}>;
/** 📚️ Carries current private-provider observation only, without an ordinary image or writable port. */
export type NativeProtectedMaintenanceProviderV1=Readonly<{root:string;backing:import("../📜️journal/🟦️.ts").NativeStorageBackingV1;associations:readonly NativeProtectedMaintenanceAssociationV1[];port:import("../🔌️port/🟦️.ts").NativeStorageObservationPortV1;verify(control:ResourceLeaseControl):Promise<readonly NativeProtectedMaintenanceAssociationV1[]>;captureChild(value:NativeProtectedMaintenanceDeclarationV1,control:ResourceLeaseControl):NativeProtectedMaintenanceAssociationV1|Promise<NativeProtectedMaintenanceAssociationV1>}>;
/** 🎟️ Keeps original custody and charge until an enforcing owner acknowledges physical retirement. */
export interface NativeStorageReservationV1{
 readonly allocation:NativeStorageAllocationV1;
 readonly backing:NativeStorageBackingV1;
 recheck():Promise<void>;
 commit():Promise<void>;
 retire():Promise<void>;
}
/** 🧹️ Retains a real allocation's original enforcing port and actual birth-captured journal exclusion. */
export type NativeStorageCleanupV1=Readonly<{[storageCleanupBrand]:true}>;
/** 🎫️ Names an exact private original reservation captured under actual Native EX. */
export type NativePlatformRecomputableRetirementV1=Readonly<{readonly[retirementBrand]:true}>;
/** 🧾️ Declares finite funding DATA beside its authentic Original; it grants no provider authority. */
export type NativeStorageChildFundingV1=Readonly<{policy:NativeStoragePolicyV1;invocation:ScriptInvocation;owner:string;metadataBytes:number;policySha256:string}>;
/** 🪪️ Identifies only an actual canonical receiver retained by its privately captured kernel guard. */
export type NativeImageRetainedCleanupV1=Readonly<{[imageCleanupBrand]:true}>;
/** 🖼️ Immutable observations carry no Native port, resource control, release method or deletion authority. */
export type NativeImageCleanupFactsV1=Readonly<{root:string;leaseDirectory:string;capacityBytes:number;backing:NativeStorageBackingV1}>;
/** 🔎️ Rechecks the genuine retained reader only while its accepted terminal callback is live. */
export type NativeImageCleanupReadV1=Readonly<{facts:NativeImageCleanupFactsV1;recheck():void}>;
/** 💿️ Identifies one accepting actual disposable child; it exposes no path, writer or command authority. */
export type NativeDisposableImageV1=Readonly<{[disposableBrand]:true}>;
