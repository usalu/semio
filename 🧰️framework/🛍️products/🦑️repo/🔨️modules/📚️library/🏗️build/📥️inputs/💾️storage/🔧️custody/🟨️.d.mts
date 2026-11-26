import type {ResourceLeaseControl} from "@semio-tech/process/leases";
import type {ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {RepositoryCacheStoreV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import type {BuildGraphOptionalQuotaV1} from "../🟨️.mjs";
import type {NativeImageStorageV1,NativeImageMaintenanceCustodyV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
/** 🧷️ Refuses stale Graph mapping, privately captured policies or its defining Store owner. */
export function requireCurrentRepositoryBuildGraphConsumerV1():void;
/** 🪪️ Requires exact privately owned fixture or canonical Graph maintenance image. */
export function requireRepositoryBuildGraphMaintenanceImageV1(image:NativeImageStorageV1,custody:NativeImageMaintenanceCustodyV1):void;
/** 🔒️ Holds the existing actual writer namespace under genuine Graph EX. */
export function withRepositoryBuildGraphMaintenanceWriterV1<T>(image:NativeImageStorageV1,custody:NativeImageMaintenanceCustodyV1,coordinate:Readonly<{kind:"canonical"}|{kind:"selection"|"dataset";identity:string}>,body:()=>Promise<T>):Promise<Readonly<{acquired:false}|{acquired:true;value:T}>>;

/** 🧾️ A JSON document read from a captured file. */
export type JsonValueV1=null|boolean|number|string|readonly JsonValueV1[]|{readonly [key:string]:JsonValueV1};
/** 📐️ A captured JSON object (schema or policy). */
export type JsonObjectV1={readonly [key:string]:JsonValueV1};
/** 🪣️ The staging root a graph writer publishes leaves into, and the observer of each published leaf. */
export interface BuildGraphStagingScopeV1{readonly root:string;readonly published?:(path:string)=>void}
/** 🔒️ The writer custody of one graph store: its workspace, the store, the device and inode, the open optional handles, the ledger, the optional quota and the phase control. */
export interface BuildGraphWriterScopeV1{readonly workspace:string;readonly store:RepositoryCacheStoreV1;readonly device:number|bigint;readonly inode:number|bigint;live:boolean;readonly handles:Set<()=>void>;readonly ledger:Awaited<ReturnType<typeof import("../📒️ledger/🟨️.mjs").openBuildGraphOptionalLedgerV1>>;readonly optionalQuota:BuildGraphOptionalQuotaV1;readonly namespace?:string;readonly phaseControl:ResourceLeaseControl}
/** 📦️ The inference dataset custody: the actual workspace, its source root, identity and lifetime. */
export interface BuildGraphDatasetScopeV1{readonly workspace:string;readonly sourceRoot:string;readonly identity:string;live:boolean}
/** 🎯️ The selected graph scope custody: its workspace, identity and lifetime. */
export interface BuildGraphSelectionScopeV1{readonly workspace:string;readonly identity:string;live:boolean}
/** 🧪️ The graph fixture custody: root, image, control, lifetime and the optional publication counters. */
export interface BuildGraphBackingScopeV1{readonly workspace:string;readonly root:string;readonly image:RepositoryCacheStoreV1["image"];readonly invocation:ScriptInvocation;readonly control:ResourceLeaseControl;live:boolean;accepting:boolean;optionalAttempts:number;optionalDeclined:number;observeOptionalPublication(kind:"attempted"|"declined"):void;readonly optionalMinimumAvailableBytes:number}
/** 📖️ The consumer custody of one graph read. */
export interface BuildGraphReadScopeV1{readonly workspace:string;readonly store:RepositoryCacheStoreV1;readonly control:ResourceLeaseControl}
export const stagings: import("node:async_hooks").AsyncLocalStorage<BuildGraphStagingScopeV1>;
export const writers: import("node:async_hooks").AsyncLocalStorage<BuildGraphWriterScopeV1>;
export const datasets: import("node:async_hooks").AsyncLocalStorage<BuildGraphDatasetScopeV1>;
export const selections: import("node:async_hooks").AsyncLocalStorage<BuildGraphSelectionScopeV1>;
export const backings: import("node:async_hooks").AsyncLocalStorage<BuildGraphBackingScopeV1>;
export const graphReadScopes: import("node:async_hooks").AsyncLocalStorage<BuildGraphReadScopeV1>;
export const graphFixtureSchema: JsonObjectV1;
export const policy: JsonObjectV1;
/** 📐️ The closed Graph storage schema and policy exactly as pinned by the consumer revisions above. */
export const schema: JsonObjectV1;
