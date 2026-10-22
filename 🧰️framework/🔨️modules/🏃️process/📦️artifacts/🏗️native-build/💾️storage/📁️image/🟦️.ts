import type{NativePlatformRecomputableRetirementV1}from"../🧹️whole/🟦️.ts";
import{checkResourceLeaseControl}from"../../../../🔒️leases/🟦️.ts";
import{openNativeFixedPlatformV1,retireNativeFixedPlatformV1,type NativeFixedPlatformOptionsV1}from"../🧱️fixed/🟦️.ts";
import type{NativeImageStorageV1,NativeImageRetirementRequestV1}from"../🌐️platform/📋️contract/🟦️.ts";
import{directoryPartitionHostV1}from"../🔌️backends/📁️directory-partition/🏠️host/🟦️.ts";
const directoryHost=()=>directoryPartitionHostV1();
/** 📁️ Opens the first-party directory-partition backend without a caller-supplied platform capability. */
export function openNativeDirectoryImageStorageV1(options:NativeFixedPlatformOptionsV1):Promise<NativeImageStorageV1>{checkResourceLeaseControl(options.control);if(!directoryHost())throw Error("Directory partition backend unavailable on this platform");return openNativeFixedPlatformV1(options);}
/** ♻️ Retires only through the first-party directory-partition backend and exact retained writer protocol. */
export function retireNativeDirectoryImageStorageV1(options:NativeFixedPlatformOptionsV1,request?:NativeImageRetirementRequestV1,recomputable?:NativePlatformRecomputableRetirementV1):Promise<boolean>{if(!directoryHost())throw Error("Directory partition backend unavailable on this platform");return retireNativeFixedPlatformV1(options,request,recomputable);}
