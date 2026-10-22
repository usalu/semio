import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import{lstatSync,realpathSync}from"node:fs";
import{join,sep}from"node:path";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 📦️ Describes exact locked parser package payloads in their original installed binding. */
export type BuildParserReceiptV1=Readonly<{version:1;key:string;generation:string;bindingDigest:string;parsers:readonly Readonly<{name:string;version:string;digest:string;files:number;bytes:number}>[]}>;

/** 🧾️ Admits only the complete original parser material and dependency binding witness. */
export function buildParserReceiptMatchesV1(receipt:unknown,current:unknown):boolean {const valid=(value:unknown):value is BuildParserReceiptV1=>validateJsonSchemaSubset(schema,value).length===0&&new Set((value as BuildParserReceiptV1).parsers.map(row=>row.name)).size===2;
 return valid(receipt)&&valid(current)&&receipt.key===current.key&&receipt.generation===current.generation&&receipt.bindingDigest===current.bindingDigest&&receipt.parsers.every(row=>{const actual=current.parsers.find(item=>item.name===row.name);return actual?.version===row.version&&actual.digest===row.digest&&actual.files===row.files&&actual.bytes===row.bytes;});}

/** 📮️ Admits only the three proven parser bootstrap observations before choosing maintenance. */
export function readBuildParserInspectionV1(value:unknown):"ready"|"receipt-required"|"material-required"{if(validateJsonSchemaSubset(schema.definitions.Inspection,value,schema).length)throw Error("Closed parser bootstrap inspection required");return value as"ready"|"receipt-required"|"material-required";}

/** 📍️ Selects only the declared parser entry without resolving or executing external code. */
export function readInstalledBuildParserEntryV1(name:string,manifest:unknown):string{
 const expected=schema.definitions.InstalledParserEntries.const.find(row=>row.name===name);if(!expected||manifest===null||typeof manifest!=="object"||Object.getPrototypeOf(manifest)!==Object.prototype)throw Error("Closed installed parser manifest required");const rows=Object.getOwnPropertyDescriptors(manifest),packageName=rows.name,main=rows.main;if(!packageName||!("value"in packageName)||packageName.value!==name||!main||!("value"in main)||typeof main.value!=="string"||(main.value.startsWith("./")?main.value.slice(2):main.value)!==expected.entry)throw Error("Declared installed parser entry required");return expected.entry;
}

/** 🔎️ Returns only an ordinary stable entry path; generation custody remains with the caller. */
export function readInstalledBuildParserEntryPathV1(directory:string,name:string,manifest:unknown):string{
 const declared=join(directory,readInstalledBuildParserEntryV1(name,manifest));if(declared.length>256)throw Error("Bounded installed parser entry path required");const before=lstatSync(declared,{bigint:true});if(!before.isFile()||before.isSymbolicLink())throw Error("Ordinary installed parser entry required");const entry=realpathSync(declared),after=lstatSync(declared,{bigint:true}),stamp=(value:typeof before)=>[value.dev,value.ino,value.mode,value.nlink,value.size,value.mtimeNs,value.ctimeNs].join(":");if(!after.isFile()||after.isSymbolicLink()||stamp(before)!==stamp(after)||!entry.startsWith(directory+sep))throw Error("Original installed parser entry required");return entry;
}
