import type{ResourceLeaseControl}from"@semio-tech/process/leases";
import type{ExactSourceReadBatchV1}from"../../🧮️digest/📥️read/🟨️.mjs";
/** 🔒️ Uses actual selected Cargo metadata from one live root-bound Source batch. */
export function selectedCargoLockDigestInBatchV1(batch:ExactSourceReadBatchV1,workspaceRoot:string,manifest:string):Promise<string>;
/** 🔒️ Starts a fresh controlled selected-lock observation with no cross-batch cache. */
export function selectedCargoLockDigest(workspaceRoot:string,manifest:string,control:ResourceLeaseControl):Promise<string>;
/** 🦀️ One resolution run over a live Source batch: manifests are read once through it. */
export function cargoResolutionContextInBatchV1(batch:ExactSourceReadBatchV1,workspaceRoot:string):unknown;
/** 🦀️ The resolution identity of one crate under a live batch: locked slice, workspace inheritance of its path closure, pinned toolchain and configuration. */
export function cargoResolutionDigestInBatchV1(batch:ExactSourceReadBatchV1,workspaceRoot:string,manifest:string,context?:unknown):Promise<string>;
/** 🦀️ The resolution identities of many crates under one fresh batch and one shared reading state. */
export function selectedCargoResolutionDigests(workspaceRoot:string,manifests:readonly string[],control:ResourceLeaseControl):Promise<ReadonlyMap<string,string>>;
