export type SourceInputInventoryV1=Readonly<{version:1;files:readonly string[]}>;
export type CargoLockInventoryV1=Readonly<{version:1;manifest:string}>;
export type CapturedInputInventoryV1<T>=Readonly<{value:T;text:string;utf8Bytes:number}>;
export interface SourceInventoryDataV1{
captureRepositorySourceRootV1(root:unknown):string;
captureRepositorySourcePathV1(value:unknown):string;
captureExactSourcePathV1(root:unknown,value:unknown):string;
captureSourceInputInventoryV1(value:unknown,root:unknown):CapturedInputInventoryV1<SourceInputInventoryV1>;
captureCargoLockInventoryV1(value:unknown,root:unknown):CapturedInputInventoryV1<CargoLockInventoryV1>;
admitExactSourceExtentV1(input:unknown):Readonly<{sourceBytes:number;metadataBytes:number;inventories:number}>;
}
export const loadedSourceInventoryURLV1:string;
export function createSourceInventoryDataV1(policy:Readonly<Record<string,number|Readonly<{prefixes:readonly string[];digestHex:number}>>>):Readonly<SourceInventoryDataV1>;
