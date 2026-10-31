import schema from"./🔣️schema.json"with{type:"json"};
import storageSchema from"../🧬️schema/🔣️.json"with{type:"json"};
import{validateJsonSchemaSubset}from"../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type{NativeStorageAllocationV1}from"../📜️journal/🟦️.ts";

export type NativeStorageOwnerRequestV1=Readonly<{version:1;owner:string;kind:NativeStorageAllocationV1["kind"];bytes:number;expectedAllocation?:string}>;

/** 🎟️ Admits the exact singleton owner quote without issuing storage custody. */
export function readNativeStorageOwnerRequestV1(value:unknown):NativeStorageOwnerRequestV1{
 if(validateJsonSchemaSubset(schema,value,schema,[storageSchema]).length)throw Error("Closed Native storage owner quote required");return Object.freeze({...value as NativeStorageOwnerRequestV1});
}

/** 🧮️ Selects only one matching durable owner, retaining every ambiguous or changed charge. */
export function nativeStorageOwnerAllocationV1(allocations:readonly NativeStorageAllocationV1[],request:NativeStorageOwnerRequestV1):NativeStorageAllocationV1|undefined{
 const rows=allocations.filter(row=>row.owner===request.owner);if(rows.length>1)throw Error("Original Native owner allocation is ambiguous");const row=rows[0];if(request.expectedAllocation!==undefined&&row?.id!==request.expectedAllocation||row&&(row.kind!==request.kind||row.bytes!==request.bytes||row.phase==="retiring"))throw Error("Original Native owner allocation disagrees with quote");return row;
}
