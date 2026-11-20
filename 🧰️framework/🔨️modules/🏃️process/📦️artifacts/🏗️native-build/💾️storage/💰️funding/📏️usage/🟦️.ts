import{readNativeStorageDataV1}from"../../📜️journal/🟦️.ts";
/** 🧮️ Captures measured whole-artifact bytes before deciding whether the retained charge admits them. */
export function readNativeStorageObservedArtifactUsageV1(value:unknown):Readonly<{version:1;purpose:"current-artifact-funding-usage";allocation:string;bytes:number;maximumBytes:number;authority:false}>{
 return Object.freeze({...readNativeStorageDataV1<{version:1;purpose:"current-artifact-funding-usage";allocation:string;bytes:number;maximumBytes:number;authority:false}>("FundedArtifactUsage",value)});
}
/** 📐️ Admits measured whole-artifact bytes against only its actual retained charge. */
export function readNativeStorageFundedArtifactUsageV1(value:unknown):ReturnType<typeof readNativeStorageObservedArtifactUsageV1>{
 const row=readNativeStorageObservedArtifactUsageV1(value);
 if(row.bytes>row.maximumBytes)throw Object.assign(Error(`Actual whole artifact exceeds its retained Native funding (${row.bytes} bytes allocated, ${row.maximumBytes} bytes reserved)`,{cause:row}),{code:"NATIVE_ARTIFACT_FUNDING_EXCEEDED"});
 return row;
}
