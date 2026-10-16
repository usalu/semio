import{checkResourceLeaseControl}from"../../../../🔒️leases/🟦️.ts";
import{openNativeFixedPlatformV1,retireNativeFixedPlatformV1,type NativeFixedPlatformOptionsV1}from"../🧱️fixed/🟦️.ts";
import type{NativeImageStorageV1,NativeImageRetirementRequestV1}from"../🌐️platform/📋️contract/🟦️.ts";
/** 🪟️ Opens the first-party fixed backend without a caller-supplied platform capability. */
export function openNativeWindowsImageStorageV1(options:NativeFixedPlatformOptionsV1):Promise<NativeImageStorageV1>{checkResourceLeaseControl(options.control);if(process.platform!=="win32")throw Error("Windows fixed backend unavailable on this platform");return openNativeFixedPlatformV1(options);}
/** ♻️ Retires only through the first-party fixed backend and exact retained writer protocol. */
export function retireNativeWindowsImageStorageV1(options:NativeFixedPlatformOptionsV1,request?:NativeImageRetirementRequestV1):Promise<boolean>{if(process.platform!=="win32")throw Error("Windows fixed backend unavailable on this platform");return retireNativeFixedPlatformV1(options,request);}
