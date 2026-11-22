import {machinePathV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {existsSync} from "node:fs";
import {acquireResourceLease,type ResourceLease} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {REPOSITORY_LEASE_DIRECTORY_V1} from "../../../../🔒️leases/⌨️command/🟦️.ts";
import type {DoctorPortsV1} from "../../../🔌️ports/🟦️.ts";

/** 🔒️ The only lease a doctor repair takes: the exclusive lease of one cache image, tried once without waiting so the doctor never queues behind a build and a live reclaim cycle or opener is never raced. `undefined` when the lease directory is not mounted (nothing can hold a lease then, and the leases image itself has none) and `"busy"` when somebody holds it.
 * @see https://www.sqlite.org/lockingv3.html */
export async function tryImageLeaseV1(ports:DoctorPortsV1,id:string,directory:string):Promise<ResourceLease|undefined|"busy">{
 const leaseDirectory=machinePathV1(ports.root,REPOSITORY_LEASE_DIRECTORY_V1);
 if(id==="leases"||!existsSync(leaseDirectory))return undefined;
 try{return await acquireResourceLease({directory:leaseDirectory,resource:"native-fixed-image:"+directory,mode:"exclusive",signal:ports.signal,timeoutMs:0});}
 catch(error){if(/timed out/iu.test(error instanceof Error?error.message:String(error)))return "busy";throw error;}
}
