import {createHash} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/📦️inventory.json" with {type:"json"};

/** 📦️ Canonical inventory metadata does not itself grant store or compiler authority. */
export interface PortableMsvcInventoryV1{readonly version:1;readonly recipeSha256:string;readonly files:readonly Readonly<{path:string;bytes:number;sha256:string}>[]}
/** 📏️ Validates a deterministic Windows inventory and returns its exact canonical bytes and digest. */
export function portableMsvcInventoryV1(input:unknown):Readonly<{text:string;sha256:string;files:number;bytes:number}>{
 if(validateJsonSchemaSubset(schema,input,schema).length)throw Error("Closed native Windows compiler inventory required");
 const value=input as PortableMsvcInventoryV1,seen=new Set<string>();let bytes=0,previous:string|undefined;
 for(const row of value.files){
  const parts=row.path.split("/"),fold=row.path.toUpperCase();
  if(!["VC","sdk"].includes(parts[0]!)||parts.length<2||parts.some(part=>!part||part==="."||part===".."||/[<>:"|?*\\\x00-\x1f]/u.test(part)||/[. ]$/u.test(part)||/^(?:con|prn|aux|nul|conin\$|conout\$|com[1-9¹²³]|lpt[1-9¹²³])(?:\.|$)/iu.test(part))||seen.has(fold)||previous!==undefined&&Buffer.compare(Buffer.from(previous),Buffer.from(row.path))>=0)throw Error("Closed unique Windows compiler inventory path required");
  seen.add(fold);previous=row.path;bytes+=row.bytes;if(!Number.isSafeInteger(bytes)||bytes>schema.definitions.Policy.const.maximumTotalBytes)throw Error("Bounded Windows compiler inventory bytes required");
 }
 const text=JSON.stringify({version:1,recipeSha256:value.recipeSha256,files:value.files.map(row=>({path:row.path,bytes:row.bytes,sha256:row.sha256}))})+"\n";
 return Object.freeze({text,sha256:createHash("sha256").update(text).digest("hex"),files:value.files.length,bytes});
}
