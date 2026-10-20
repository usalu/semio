import {nativeStatfsV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

/** 💾️ Bounds automatic Nx cache growth to one quarter of available disk space and four GiB. */
export function nxCacheBudgetV1(availableBytes:number,override?:string):string {
 if(!Number.isSafeInteger(availableBytes)||availableBytes<0)throw Error("Available cache bytes must be a nonnegative safe integer");
 if(override!==undefined){if(!override.trim())throw Error("Explicit Nx cache budget must not be empty");return override;}
 return String(Math.max(1,Math.min(4*1024**3,Math.floor(availableBytes/4))));
}

/** 🗄️ Applies the portable filesystem budget while retaining the caller's explicit Nx cache choice. */
export async function nxCacheEnvironmentV1(root:string,environment:NodeJS.ProcessEnv):Promise<NodeJS.ProcessEnv> {
 if(environment.NX_MAX_CACHE_SIZE!==undefined)return {...environment,NX_MAX_CACHE_SIZE:nxCacheBudgetV1(0,environment.NX_MAX_CACHE_SIZE)};
 const disk=await nativeStatfsV1(root);
 return {...environment,NX_MAX_CACHE_SIZE:nxCacheBudgetV1(Math.min(Number.MAX_SAFE_INTEGER,disk.bavail*disk.bsize))};
}
