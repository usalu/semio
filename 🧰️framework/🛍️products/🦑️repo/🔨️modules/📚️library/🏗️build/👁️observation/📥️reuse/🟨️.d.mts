import type{ResourceLeaseControl}from"@semio-tech/process/leases";
/** 🧷️ Requires the defining loaded Source and policy epoch. */
export function requireCurrentInferenceInventoryReuseV1():void;
/** 📋️ Captures only inert finite inventory reference names. */
export function captureInferenceInventoryReferencesV1(value:unknown):readonly string[];
/** 🧮️ Bounds the next cohort using only the defining private policies. */
export function admitInferenceInventoryCohortV1(usedBytes:number,records:number,maximumBytes?:number):number;
/** 📥️ Retains genuine image custody and rechecks physical ancestry/leaf identity before every memo hit. */
export function inferenceInventoryReferencesHoldV1(root:string,value:unknown,control:ResourceLeaseControl):Promise<boolean>;
/** 📊️ How the inventory presence checks went: asked, answered from the memory of the round, batches that took a lease, names they checked, batches asked again caller by caller. */
export function inferenceInventoryPresenceStatsV1(root:string,control:ResourceLeaseControl):Readonly<{asked:number;remembered:number;batches:number;retried:number;names:number}>;

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
