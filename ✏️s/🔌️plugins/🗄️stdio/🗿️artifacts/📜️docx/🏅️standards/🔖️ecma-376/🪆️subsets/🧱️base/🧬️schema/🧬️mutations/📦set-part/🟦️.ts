/** 📦️ Typed DOCX package content admission. */
import {parseXmlDocument} from "../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import type {DocxPartContent,DocxSetPart} from "../🟦️.ts";
function record(value:unknown,keys:readonly string[]):Record<string,unknown>{if(value===null||typeof value!=="object"||Array.isArray(value)||Object.keys(value).some(key=>!keys.includes(key)))throw Error("DOCX part content requires a closed record");return value as Record<string,unknown>;}
/** 🧩️ Validates intrinsic XML trees or binary octets without interpreting an encoding. */
export function parseDocxPartContent(value:unknown):DocxPartContent{
 const row=record(value,["kind","document","bytes"]);
 if(row.kind==="xml"){record(row,["kind","document"]);return{kind:"xml",document:parseXmlDocument(row.document)};}
 if(row.kind==="binary"){record(row,["kind","bytes"]);if(!Array.isArray(row.bytes)||row.bytes.some(byte=>typeof byte!=="number"||!Number.isInteger(byte)||byte<0||byte>255))throw Error("DOCX binary content requires octets");return{kind:"binary",bytes:[...row.bytes]};}
 throw Error("DOCX part content kind is unknown");
}
/** 🛡️ Admits the complete SetPart command with owned semantic content. */
export function parseDocxSetPart(value:unknown):DocxSetPart{const row=record(value,["mutation","path","content_type","payload","index","override_index"]);if(row.mutation!=="setPart"||typeof row.path!=="string"||typeof row.content_type!=="string")throw Error("DOCX SetPart requires its typed target");const position=(name:"index"|"override_index"):number|undefined=>{const found=row[name];if(found===undefined)return undefined;if(typeof found!=="number"||!Number.isSafeInteger(found)||found<0)throw Error("DOCX SetPart positions are non-negative integers");return found;};const index=position("index"),override_index=position("override_index");return{mutation:"setPart",path:row.path,content_type:row.content_type,payload:parseDocxPartContent(row.payload),...(index===undefined?{}:{index}),...(override_index===undefined?{}:{override_index})};}
