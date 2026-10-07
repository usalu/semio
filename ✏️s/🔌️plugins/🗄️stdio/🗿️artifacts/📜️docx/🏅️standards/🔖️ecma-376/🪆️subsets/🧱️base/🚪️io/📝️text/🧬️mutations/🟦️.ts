import {parseXmlDocumentJson,xmlDocumentJsonValue} from "../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts";
/** 📝️ Native SetPart grammar with explicit owned JSON payloads. */
import {parseDocxSetPart} from "../../../🧬️schema/🧬️mutations/📦set-part/🟦️.ts";
import type {DocxSetPart} from "../../../🧬️schema/🧬️mutations/🟦️.ts";
export type DocxMutationsText=string;
const encoder=new TextEncoder(),decoder=new TextDecoder("utf-8",{fatal:true});
const hex=(value:string):string=>Array.from(encoder.encode(value),byte=>byte.toString(16).padStart(2,"0")).join("");
function unhex(value:string):string{if(value.length%2!==0||!/^[0-9a-f]*$/i.test(value))throw Error("DOCX text requires hexadecimal octets");return decoder.decode(Uint8Array.from(value.match(/../g)??[],byte=>parseInt(byte,16)));}
/** 📤️ Encodes a SetPart target and semantic content using the authored grammar. */
export function encodeDocxSetPartText(input:DocxSetPart):string{const value=parseDocxSetPart(input);return `set-part path=${hex(value.path)} content-type=${hex(value.content_type)} payload=${hex(JSON.stringify(value.payload.kind==="xml"?{kind:"xml",document:xmlDocumentJsonValue(value.payload.document)}:value.payload))}`;}
/** 📥️ Decodes physical text before canonical semantic payload admission. */
export function decodeDocxSetPartText(text:string):DocxSetPart{const match=/^set-part path=([0-9a-f]*) content-type=([0-9a-f]*) payload=([0-9a-f]+)$/i.exec(text);if(!match)throw Error("DOCX SetPart text grammar refused");return parseDocxSetPart({mutation:"setPart",path:unhex(match[1]!),content_type:unhex(match[2]!),payload:bindPayload(JSON.parse(unhex(match[3]!)))});}

function bindPayload(value:unknown):unknown{if(value===null||typeof value!=="object"||Array.isArray(value))return value;const payload=value as Record<string,unknown>;return payload.kind==="xml"?{...payload,document:parseXmlDocumentJson(payload.document)}:payload;}
