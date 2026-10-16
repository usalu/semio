import type {NativeFixedPlatformOptionsV1} from "../🧱️fixed/🟦️.ts";
import {openNativeImageStorageV1,retireNativeImageStorageV1} from "../🍎️image/🟦️.ts";
import {openNativeLinuxImageStorageV1,retireNativeLinuxImageStorageV1} from "../🐧️image/🟦️.ts";
import {openNativeWindowsImageStorageV1,retireNativeWindowsImageStorageV1} from "../🪟️image/🟦️.ts";
import type {NativeImageStorageV1,NativeImageMaintenanceRequestV1,NativeImageRetirementRequestV1} from "./📋️contract/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
export {nativeImageRootV1,requireNativeStoragePathV1} from "./📋️contract/🟦️.ts";
export type {NativeImageStorageV1,NativeImageMaintenanceRequestV1,NativeImageRetirementRequestV1};

/** 🧭️ Selects the enforcing backend for one schema-declared desktop host. */
export function nativeStoragePlatformV1(platform:string):"macos"|"linux"|"windows"{if(!schema.enum.includes(platform))throw Error("Declared native fixed storage platform required");return platform==="darwin"?"macos":platform==="linux"?"linux":"windows";}

/** 🌐️ Opens the host's actual finite backing while preserving original system capability admission. */
export async function openNativePlatformStorageV1(options:NativeFixedPlatformOptionsV1):Promise<NativeImageStorageV1>{const backend=nativeStoragePlatformV1(process.platform);return backend==="macos"?openNativeImageStorageV1(options):backend==="linux"?openNativeLinuxImageStorageV1(options):openNativeWindowsImageStorageV1(options);}

/** ♻️ Retires only an exclusively held exact native backing through its declared host protocol. */
export async function retireNativePlatformStorageV1(options:NativeFixedPlatformOptionsV1,request?:NativeImageRetirementRequestV1):Promise<boolean>{requireNativeImageEnvelopeV1(options);const admitted=readNativeImageRetirementRequestV1(request??{version:1,wait:true}),backend=nativeStoragePlatformV1(process.platform);return backend==="macos"?retireNativeImageStorageV1(options,admitted):backend==="linux"?retireNativeLinuxImageStorageV1(options,admitted):retireNativeWindowsImageStorageV1(options,admitted);}

import {requireNativeImageEnvelopeV1,readNativeImageRetirementRequestV1} from "./📋️contract/🟦️.ts";
