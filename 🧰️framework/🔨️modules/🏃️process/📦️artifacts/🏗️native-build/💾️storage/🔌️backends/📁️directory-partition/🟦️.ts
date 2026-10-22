import {openNativeDirectoryImageStorageV1,retireNativeDirectoryImageStorageV1} from "../../📁️image/🟦️.ts";
import {captureFixedNativeImageCleanupV1,captureFixedNativeProtectedMaintenanceV1,captureFixedNativeProtectedParentV1,requireFixedNativeImageCloseV1,retireFixedNativeFundedChildV1,retireNativeFixedPlatformWithWriterV1,withFixedNativeChildStorageV1,withFixedNativeDisposableChildStorageV1,withFixedNativeFundedChildStorageV1,withFixedNativeImageCleanupReadV1,withFixedNativeMaintenanceChildStorageV1} from "../../🧱️fixed/🟦️.ts";
import {bindNativeStorageOperationsV1,type NativeStorageOperationsV1} from "../📋️contract/🟦️.ts";
import "./📐️geometry/🟦️.ts";

/** 📁️ Routes the storage operations to the portable directory partition that runs unprivileged on every desktop host. */
export const directoryPartitionStorageOperationsV1:NativeStorageOperationsV1=Object.freeze({
 open:openNativeDirectoryImageStorageV1,
 retire:retireNativeDirectoryImageStorageV1,
 retireWithWriter:retireNativeFixedPlatformWithWriterV1,
 captureCleanup:captureFixedNativeImageCleanupV1,
 withCleanupRead:withFixedNativeImageCleanupReadV1,
 requireClose:requireFixedNativeImageCloseV1,
 withChild:withFixedNativeChildStorageV1,
 withMaintenanceChild:withFixedNativeMaintenanceChildStorageV1,
 withDisposableChild:withFixedNativeDisposableChildStorageV1,
 withFundedChild:withFixedNativeFundedChildStorageV1,
 retireFundedChild:retireFixedNativeFundedChildV1,
 captureProtectedParent:captureFixedNativeProtectedParentV1,
 captureProtectedMaintenance:captureFixedNativeProtectedMaintenanceV1
});
bindNativeStorageOperationsV1("directory-partition",directoryPartitionStorageOperationsV1);
