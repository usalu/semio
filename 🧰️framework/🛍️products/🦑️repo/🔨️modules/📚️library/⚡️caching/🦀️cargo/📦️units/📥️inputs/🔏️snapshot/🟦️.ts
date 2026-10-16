import{validateJsonSchemaSubset}from"../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from"./🧬️schema/🔣️.json";
export interface RustUnitSourceSnapshotRowV1{readonly path:string;readonly digest:string;readonly bytes:number}
export interface RustUnitSourceSnapshotV1{readonly version:1;readonly expected:readonly RustUnitSourceSnapshotRowV1[];readonly observed:readonly RustUnitSourceSnapshotRowV1[]}
/** 🔏️ Compares the complete observed authored closure with the descriptor before warm reuse. */
export function rustUnitSourceSnapshotMatchesV1(value:RustUnitSourceSnapshotV1):boolean{if(validateJsonSchemaSubset(schema,value).length)return false;const expected=new Map(value.expected.map(row=>[row.path,row]));if(expected.size!==value.expected.length||new Set(value.observed.map(row=>row.path)).size!==value.observed.length||expected.size!==value.observed.length)return false;return value.observed.every(row=>{const prior=expected.get(row.path);return prior?.digest===row.digest&&prior.bytes===row.bytes;});}
