/** 🎞️ Actual workflow payload binding preserves its declared semantic media form. */
import{parseSchemaRecord}from"../../🧬️schema/🧾️record/🟦️.ts";
import{parseIntrinsicValue,type IntrinsicValue}from"../../🌱️value/🧬️schema/🟦️.ts";
export type MediaPayload={kind:"structured";schema:string;json:string}|{kind:"binary";formatKind:string;blobHash:string}|{kind:"intrinsic";schema:string;value:IntrinsicValue};
/** 📥️ Bind the mounted literal payload owners. */
export function parseMediaPayload(source:unknown):MediaPayload{
 if(source===null||typeof source!=="object"||!("kind"in source))throw Error("media payload kind required");
 switch(source.kind){
  case"structured":{const row=parseSchemaRecord(source,["kind","schema","json"]);if(typeof row.schema!=="string"||typeof row.json!=="string")throw Error("structured media fields required");return{kind:"structured",schema:row.schema,json:row.json};}
  case"binary":{const row=parseSchemaRecord(source,["kind","formatKind","blobHash"]);if(typeof row.formatKind!=="string"||typeof row.blobHash!=="string")throw Error("binary media fields required");return{kind:"binary",formatKind:row.formatKind,blobHash:row.blobHash};}
  case"intrinsic":{const row=parseSchemaRecord(source,["kind","schema","value"]);if(typeof row.schema!=="string")throw Error("intrinsic media schema required");return{kind:"intrinsic",schema:row.schema,value:parseIntrinsicValue(row.value)};}
  default:throw Error("unsupported media payload owner");
 }
}
