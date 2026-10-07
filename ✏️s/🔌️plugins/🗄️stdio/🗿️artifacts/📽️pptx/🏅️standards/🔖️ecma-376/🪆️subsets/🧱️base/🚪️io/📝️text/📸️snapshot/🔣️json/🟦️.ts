/** 🔣️ Native PPTX JSON binds XML integer positions before canonical admission. */
import {parsePptxSnapshot,type PptxSnapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {parseXmlDocumentJson} from "../../../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts";
export function parsePptxSnapshotJson(value:unknown):PptxSnapshot{if(value===null||typeof value!=="object"||Array.isArray(value))return parsePptxSnapshot(value);const snapshot=value as Record<string,unknown>;return parsePptxSnapshot({...snapshot,xmlParts:Array.isArray(snapshot.xmlParts)?snapshot.xmlParts.map(part=>{if(part===null||typeof part!=="object"||Array.isArray(part))return part;return{...part,document:parseXmlDocumentJson(part.document)};}):snapshot.xmlParts});}
