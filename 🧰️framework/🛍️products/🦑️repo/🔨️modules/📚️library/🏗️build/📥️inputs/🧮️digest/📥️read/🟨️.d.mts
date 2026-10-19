import type{ResourceLeaseControl}from"@semio-tech/process/leases";
declare const batchBrand:unique symbol,managedBrand:unique symbol;
/** 🧷️ Resolves only to an actual live, root-bound canonical Source I/O batch. */
export interface ExactSourceReadBatchV1{readonly[batchBrand]:true}
/** 🏪️ Resolves only to a live inventory batch enrolled in its original Graph store consumer. */
export interface ManagedGraphSourceReadBatchV1 extends ExactSourceReadBatchV1{readonly[managedBrand]:true}
export function withManagedGraphSourceReadBatchV1<T>(root:string,control:ResourceLeaseControl,run:(batch:ManagedGraphSourceReadBatchV1)=>Promise<T>):Promise<T>;
export type ExactSourceFileV1=Readonly<{sha256:string;byteLength:number}>;
export type ExactSourceMetadataV1=ExactSourceFileV1&Readonly<{value:unknown}>;
export function withExactSourceReadBatchV1<T>(root:string,control:ResourceLeaseControl,run:(batch:ExactSourceReadBatchV1)=>Promise<T>):Promise<T>;
export function requireExactSourceReadBatchV1(batch:ExactSourceReadBatchV1,root:string):void;
export function readExactSourceFileV1(batch:ExactSourceReadBatchV1,name:string,maximumBytes?:number):Promise<ExactSourceFileV1|null>;
export function readExactSourceJsonV1(batch:ExactSourceReadBatchV1,name:string,maximumBytes?:number):Promise<ExactSourceMetadataV1|null>;
export function readExactSourceTomlV1(batch:ExactSourceReadBatchV1,name:string):Promise<ExactSourceMetadataV1|null>;
export function exactSourceReadBatchFactsV1(batch:ExactSourceReadBatchV1):Readonly<{files:number;sourceBytes:number;metadataBytes:number;readOperations:number;bytesRead:number}>;
export function readExactSourceInventoryV1(batch:ManagedGraphSourceReadBatchV1,namespace:"build-input-closures"|"build-lock-closures",key:string):Promise<ExactSourceMetadataV1|null>;
export function admitExactSourceExtentV1(input:unknown):Readonly<{sourceBytes:number;metadataBytes:number;inventories:number}>;

export function requireCurrentExactSourceReadV1():void;
