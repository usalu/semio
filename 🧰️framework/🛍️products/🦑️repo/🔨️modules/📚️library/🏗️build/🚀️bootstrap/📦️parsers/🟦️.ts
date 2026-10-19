import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 📦️ Describes exact locked parser package payloads in their original installed binding. */
export type BuildParserReceiptV1=Readonly<{version:1;key:string;generation:string;bindingDigest:string;parsers:readonly Readonly<{name:string;version:string;digest:string;files:number;bytes:number}>[]}>;

/** 🧾️ Admits only the complete original parser material and dependency binding witness. */
export function buildParserReceiptMatchesV1(receipt:unknown,current:unknown):boolean {const valid=(value:unknown):value is BuildParserReceiptV1=>validateJsonSchemaSubset(schema,value).length===0&&new Set((value as BuildParserReceiptV1).parsers.map(row=>row.name)).size===2;
 return valid(receipt)&&valid(current)&&receipt.key===current.key&&receipt.generation===current.generation&&receipt.bindingDigest===current.bindingDigest&&receipt.parsers.every(row=>{const actual=current.parsers.find(item=>item.name===row.name);return actual?.version===row.version&&actual.digest===row.digest&&actual.files===row.files&&actual.bytes===row.bytes;});}

/** 📮️ Admits only the three proven parser bootstrap observations before choosing maintenance. */
export function readBuildParserInspectionV1(value:unknown):"ready"|"receipt-required"|"material-required"{if(validateJsonSchemaSubset(schema.definitions.Inspection,value,schema).length)throw Error("Closed parser bootstrap inspection required");return value as"ready"|"receipt-required"|"material-required";}
