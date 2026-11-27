import {machinePathV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {existsSync} from "node:fs";
import {NATIVE_IMAGE_SHARED_READ_POLICY_V1,withNativeImageSharedReadV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤲️shared-read/🟦️.ts";
import {acquireResourceLease,type ResourceLease} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {REPOSITORY_LEASE_DIRECTORY_V1} from "../../../../🔒️leases/⌨️command/🟦️.ts";
import type {DoctorPortsV1} from "../../../🔌️ports/🟦️.ts";

/** 👓️ Runs a read of the files of one cache image under the shared lease of that image, so the doctor never reads behind a retirement, regrow or detach that holds the image exclusively: it waits at most `sampleWaitMilliseconds` and answers `"busy"` when a writer holds or awaits the image. A lease directory that is not mounted holds no lease and the read runs unguarded, as nothing can retire the image then.
 * @see ../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤲️shared-read/🟦️.ts */
export async function readUnderImageLeaseV1<T>(ports:DoctorPortsV1,relativeDirectory:string,body:()=>T|Promise<T>):Promise<T|"busy">{
 const leaseDirectory=machinePathV1(ports.root,REPOSITORY_LEASE_DIRECTORY_V1),imageDirectory=machinePathV1(ports.root,...relativeDirectory.split("/"));
 try{return await withNativeImageSharedReadV1({leaseDirectory,imageDirectory,signal:ports.signal,timeoutMs:NATIVE_IMAGE_SHARED_READ_POLICY_V1.sampleWaitMilliseconds},body);}
 catch(error){if(/timed out/iu.test(error instanceof Error?error.message:String(error)))return "busy";throw error;}
}

/** 🔒️ The only exclusive lease a doctor repair takes: the exclusive lease of one cache image, tried once without waiting so the doctor never queues behind a build and a live reclaim cycle or opener is never raced. `undefined` when the lease directory is not mounted (nothing can hold a lease then, and the leases image itself has none) and `"busy"` when somebody holds it.
 * @see https://www.sqlite.org/lockingv3.html */
export async function tryImageLeaseV1(ports:DoctorPortsV1,id:string,directory:string):Promise<ResourceLease|undefined|"busy">{
 const leaseDirectory=machinePathV1(ports.root,REPOSITORY_LEASE_DIRECTORY_V1);
 if(id==="leases"||!existsSync(leaseDirectory))return undefined;
 try{return await acquireResourceLease({directory:leaseDirectory,resource:"native-fixed-image:"+directory,mode:"exclusive",signal:ports.signal,timeoutMs:0});}
 catch(error){if(/timed out/iu.test(error instanceof Error?error.message:String(error)))return "busy";throw error;}
}
