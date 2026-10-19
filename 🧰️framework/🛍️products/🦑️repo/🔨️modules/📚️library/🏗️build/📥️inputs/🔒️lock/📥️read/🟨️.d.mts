import type{ResourceLeaseControl}from"@semio-tech/process/leases";
import type{ExactSourceReadBatchV1}from"../../🧮️digest/📥️read/🟨️.mjs";
/** 🔒️ Uses actual selected Cargo metadata from one live root-bound Source batch. */
export function selectedCargoLockDigestInBatchV1(batch:ExactSourceReadBatchV1,workspaceRoot:string,manifest:string):Promise<string>;
/** 🔒️ Starts a fresh controlled selected-lock observation with no cross-batch cache. */
export function selectedCargoLockDigest(workspaceRoot:string,manifest:string,control:ResourceLeaseControl):Promise<string>;
