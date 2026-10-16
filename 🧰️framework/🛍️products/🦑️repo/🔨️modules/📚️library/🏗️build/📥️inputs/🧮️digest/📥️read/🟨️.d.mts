import type{ResourceLeaseControl}from"../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
declare const batchBrand:unique symbol;
/** 🧷️ Resolves only to an actual live, root-bound canonical Source I/O batch. */
export interface ExactSourceReadBatchV1{readonly[batchBrand]:true}
export type ExactSourceFileV1=Readonly<{sha256:string;byteLength:number}>;
export type ExactSourceMetadataV1=ExactSourceFileV1&Readonly<{value:unknown}>;
export function withExactSourceReadBatchV1<T>(root:string,control:ResourceLeaseControl,run:(batch:ExactSourceReadBatchV1)=>Promise<T>):Promise<T>;
export function requireExactSourceReadBatchV1(batch:ExactSourceReadBatchV1,root:string):void;
export function readExactSourceFileV1(batch:ExactSourceReadBatchV1,name:string):Promise<ExactSourceFileV1|null>;
export function readExactSourceJsonV1(batch:ExactSourceReadBatchV1,name:string):Promise<ExactSourceMetadataV1|null>;
export function readExactSourceTomlV1(batch:ExactSourceReadBatchV1,name:string):Promise<ExactSourceMetadataV1|null>;
export function exactSourceReadBatchFactsV1(batch:ExactSourceReadBatchV1):Readonly<{files:number;sourceBytes:number;metadataBytes:number;readOperations:number;bytesRead:number}>;
export function readExactSourceInventoryV1(batch:ExactSourceReadBatchV1,namespace:"build-input-closures"|"build-lock-closures",key:string):Promise<ExactSourceMetadataV1|null>;
export function admitExactSourceExtentV1(input:unknown):Readonly<{sourceBytes:number;metadataBytes:number;inventories:number}>;
