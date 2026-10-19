import {createHash} from "node:crypto";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
export interface NxSdkBinaryObjectV1{readonly path:string;readonly sha256:string;readonly bytes:number;}
export interface NxSdkBinaryReceiptV1{readonly version:1;readonly digest:string;readonly objects:readonly NxSdkBinaryObjectV1[];}
/** 🧊️ Distinguishes immutable binary publication from writable package/configuration Source. */
export function nxSdkBinaryPathV1(path:string):boolean{return path.startsWith("node_modules/")&&!path.includes("\\")&&!path.includes("\0")&&path.split("/").every(part=>part&&part!=="."&&part!=="..")&&schema.definitions.Protocol.const.extensions.some(extension=>path.endsWith(extension));}
/** 🔏️ Seals exact generation paths and shared content identities without trusting an extension alone. */
export function nxSdkBinaryReceiptV1(rows:readonly NxSdkBinaryObjectV1[]):NxSdkBinaryReceiptV1{const objects=[...rows].sort((a,b)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path)));if(objects.length>schema.definitions.Protocol.const.maximumObjects||objects.some((row,index)=>!nxSdkBinaryPathV1(row.path)||index>0&&objects[index-1]!.path===row.path))throw Error("Closed unique SDK binary objects required");const digest=createHash("sha256").update(JSON.stringify({version:1,objects})+"\n").digest("hex"),receipt={version:1 as const,digest,objects};if(validateJsonSchemaSubset(schema,receipt).length)throw Error("Closed SDK binary receipt required");return Object.freeze({...receipt,objects:Object.freeze(objects.map(row=>Object.freeze({...row})))});}
