import type{ResourceLeaseControl}from"@semio-tech/process/leases";
import type{NativeStorageMappingQueueV1}from"../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔒️mapping/🟦️.ts";
import type{NativeStorageVerificationSnapshotV1,NativeStorageVerificationTimingV1}from"../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/⏱️verification/🟦️.ts";
/** 🧷️ Requires the defining loaded Source and policy epoch. */
export function requireCurrentInferenceInventoryReuseV1():void;
/** 📋️ Captures only inert finite inventory reference names. */
export function captureInferenceInventoryReferencesV1(value:unknown):readonly string[];
/** 🧮️ Bounds the next cohort using only the defining private policies. */
export function admitInferenceInventoryCohortV1(usedBytes:number,records:number,maximumBytes?:number):number;
/** 📥️ Retains genuine image custody and rechecks physical ancestry/leaf identity before every memo hit. */
export function inferenceInventoryReferencesHoldV1(root:string,value:unknown,control:ResourceLeaseControl,withdraw?:AbortSignal):Promise<boolean>;
/** 📊️ How the inventory presence checks went: asked, answered from the memory of the round, batches that took a lease, names they checked, batches asked again caller by caller. */
export function inferenceInventoryPresenceStatsV1(root:string,control:ResourceLeaseControl):Readonly<{asked:number;remembered:number;batches:number;retried:number;names:number;timings:Readonly<Record<"reader"|"preVerify"|"body"|"postVerify",NativeStorageVerificationTimingV1>>;bodyOperations?:Readonly<Partial<Record<"storeRecheck"|"continuation"|"fileChecks"|"recordRead"|"implementationCheck",NativeStorageVerificationTimingV1>>>;nativeVerification?:Readonly<Record<"preVerify"|"postVerify",NativeStorageVerificationSnapshotV1>>;mappingQueue:NativeStorageMappingQueueV1}>;

export function inferenceInventoryRecordHoldsV1(root:string,name:string,file:import("../../📥️inputs/🧮️digest/📥️read/🟨️.mjs").ExactSourceMetadataV1|null):boolean;

/** 📦️ Plans finite DATA extents without granting descriptor or store authority. */
export function planInferenceInventoryCohortsV1(extents:readonly number[]):readonly Readonly<{offset:number;records:number;maximumBytes:number}>[];

/** 🕳️ Classifies one inert error code without granting an absence or managed custody. */
export function inferenceInventoryLeafMissingV1(code:unknown):boolean;

/** 🛝️ Looks at `names` again while a concurrent writer changed the store under the look, at most `attempts` times. */
export function inventoryNamesSettledV1<T>(run:(names:readonly string[])=>Promise<T>,names:readonly string[],options?:{attempts?:number;say?:(line:string)=>void}):Promise<T>;

/** 🫙️ The no-follow directory identities down to the namespace directory of a store; the namespace directory itself may be absent. */
export function inventoryAncestryV1(storeRoot:string,directory:string,check:()=>void):Readonly<{rows:readonly Readonly<{path:string;dev:bigint;ino:bigint;mode:bigint}>[];device:bigint;absent?:string}>;
/** 🫙️ Rechecks that ancestry: an absent namespace directory must still be absent. */
export function recheckInventoryAncestryV1(parents:Readonly<{rows:readonly Readonly<{path:string;dev:bigint;ino:bigint;mode:bigint}>[];device:bigint;absent?:string}>,check:()=>void):void;
export function prefetchInferenceInventoryPresenceV1(root: string, names: Iterable<string>, control: ResourceLeaseControl, withdraw?: AbortSignal): Promise<Readonly<{asked:number;present:number}>>;
/** 🎟️ Runs `work` as one take pass of verified-once presence: inside it every look answers a name that a session of this process found present by one stat of its leaf (a pass is one snapshot of the store, as a validation round is one of the tree); outside it every ask goes to a session as before, so a look whose store changes while it looks is still refused by that session. */
export function withTakePassPresenceV1<T>(work: () => T): T;
