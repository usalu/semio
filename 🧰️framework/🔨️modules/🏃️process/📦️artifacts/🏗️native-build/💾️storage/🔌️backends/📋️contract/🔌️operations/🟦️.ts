import type {ResourceLeaseControl,ResourceLeaseWriterProofV1} from "../../../../../../🔒️leases/🟦️.ts";
import type {AcceptedPhysicalCleanupV1} from "../../../../../../🧭️routing/📥️invocation/🏃️process/🧹️cleanup/🟦️.ts";
import type {NativeFixedPlatformOptionsV1} from "../../../🔌️port/🟦️.ts";
import type {NativeImageBorrowerV1} from "../../../📥️borrower/🟦️.ts";
import type {NativeStorageChildFundingV1} from "../../../🌱️birth/🟦️.ts";
import type {NativeStorageChildRequestV1} from "../../../📜️journal/🟦️.ts";
import type {NativeStorageCleanupV1,NativeStorageReservationV1} from "../../../🟦️.ts";
import type {NativePlatformRecomputableRetirementV1} from "../../../🌐️platform/🟦️.ts";
import type {NativeImageCleanupReadV1,NativeImageRetainedCleanupV1,NativeDisposableImageV1} from "../../../🧹️cleanup/♻️retirement/📋️contract/🟦️.ts";
import type {NativeImageMaintenanceCustodyV1,NativeImageRetirementRequestV1,NativeImageStorageV1,NativeProtectedMaintenanceProviderV1,NativeProtectedParentReadV1} from "../../../🌐️platform/📋️contract/🟦️.ts";

/** 📐️ The volume measurements one backend answers for its own backing file and mount root; the neutral helpers dispatch here by the backing file they are asked about. */
export interface NativeStorageGeometryV1{
 attached(image:string,root:string):boolean;
 filesystemBytes(root:string,declared:number):number;
 usedBytes(root:string):number;
 space(root:string,declared:number):Readonly<{bsize:number;bavail:number}>;
}

/** 🔌️ Every operation the platform facade routes to one backend; both first-party backends implement the same closed list. */
export interface NativeStorageOperationsV1{
 open(options:NativeFixedPlatformOptionsV1):Promise<NativeImageStorageV1>;
 retire(options:NativeFixedPlatformOptionsV1,request:NativeImageRetirementRequestV1,recomputable?:NativePlatformRecomputableRetirementV1):Promise<boolean>;
 retireWithWriter(options:NativeFixedPlatformOptionsV1,request:NativeImageRetirementRequestV1,proof:ResourceLeaseWriterProofV1):Promise<boolean>;
 captureCleanup(image:NativeImageStorageV1,body:ResourceLeaseControl,enrollment:AcceptedPhysicalCleanupV1):NativeImageRetainedCleanupV1;
 withCleanupRead<T>(token:NativeImageRetainedCleanupV1,body:(read:NativeImageCleanupReadV1)=>Promise<T>):Promise<T>;
 requireClose(image:NativeImageStorageV1):void;
 withChild<T>(parent:NativeImageStorageV1,input:NativeFixedPlatformOptionsV1,body:(child:NativeImageStorageV1,control:ResourceLeaseControl)=>Promise<T>):Promise<T>;
 withMaintenanceChild<T>(parent:NativeImageStorageV1,custody:NativeImageMaintenanceCustodyV1,input:NativeFixedPlatformOptionsV1,body:(child:NativeImageStorageV1,control:ResourceLeaseControl)=>Promise<T>):Promise<T>;
 withDisposableChild<T>(parent:NativeImageStorageV1,input:NativeFixedPlatformOptionsV1,body:(child:NativeImageStorageV1,control:ResourceLeaseControl,owner:NativeDisposableImageV1)=>Promise<T>):Promise<T>;
 withFundedChild<T>(parent:NativeImageStorageV1,input:NativeFixedPlatformOptionsV1,funding:NativeStorageChildFundingV1,body:(child:NativeImageStorageV1,reservation:NativeStorageReservationV1,control:ResourceLeaseControl)=>Promise<T>):Promise<T>;
 retireFundedChild(image:NativeImageStorageV1,custody:NativeImageMaintenanceCustodyV1,cleanup:NativeStorageCleanupV1,enrollment:AcceptedPhysicalCleanupV1,request:NativeStorageChildRequestV1):Promise<void>;
 captureProtectedParent(input:NativeFixedPlatformOptionsV1,value:unknown,owner:NativeImageBorrowerV1,proof:ResourceLeaseWriterProofV1):Promise<NativeProtectedParentReadV1>;
 captureProtectedMaintenance(input:NativeFixedPlatformOptionsV1,value:unknown,owner:NativeImageBorrowerV1,proof:ResourceLeaseWriterProofV1):Promise<NativeProtectedMaintenanceProviderV1>;
}
