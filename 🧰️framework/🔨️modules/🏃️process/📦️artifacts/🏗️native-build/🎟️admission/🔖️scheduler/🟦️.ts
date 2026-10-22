import {createHash} from "node:crypto";
import {lstatSync,readFileSync,renameSync,writeFileSync,rmSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import { runtimeParseTomlV1 } from "../../../../../⚙️runtime/📄️parse/🟦️.ts";

const protocol=Object.freeze({...schema.definitions.Protocol.const});
const digestOf=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex");

/** 🧾️ Proves that the scheduler executables on disk are the build of exactly these sources under this compiler identity. */
export type SchedulerReceiptV1=Readonly<{version:1;identity:string;inputs:readonly Readonly<{path:string;digest:string}>[];executables:readonly Readonly<{name:string;path:string;size:number;ino:string;mtimeNs:string}>[]}>;

/** 🏷️ Hashes the compiler generation with every Cargo and Rust environment entry that can change the produced binary. */
export function schedulerIdentityV1(toolchain:string,environment:Readonly<Record<string,string|undefined>>):string{
 const entries=Object.entries(environment).filter(([name,value])=>value!==undefined&&protocol.environmentPrefixes.some(prefix=>name.startsWith(prefix))).sort(([a],[b])=>a<b?-1:a>b?1:0);
 return digestOf(JSON.stringify({version:protocol.version,toolchain,entries}));
}

/** 📂️ Lists the manifest, its lockfile and every binary source Cargo reads for the bins. */
export function schedulerInputsV1(manifest:string):readonly string[]{
 const document=runtimeParseTomlV1(readFileSync(manifest,"utf8")) as {bin?:Array<{path?:string}>},directory=dirname(manifest);
 return [manifest,join(directory,"Cargo.lock"),...(document.bin??[]).map(row=>resolve(directory,row.path??"src/main.rs"))];
}

const stamp=(path:string)=>{const row=lstatSync(path,{bigint:true});if(!row.isFile()||row.isSymbolicLink()||row.nlink!==1n)throw Error("Plain scheduler file required");return row;};
const inputRows=(manifest:string)=>schedulerInputsV1(manifest).map(path=>{const bytes=readFileSync(path);if(bytes.byteLength>protocol.maximumInputBytes)throw Error("Bounded scheduler input required");return Object.freeze({path,digest:digestOf(bytes)});});

/** 📸️ Records the receipt after a successful build; any unreadable or unusual file refuses it. */
export function captureSchedulerReceiptV1(identity:string,manifest:string,executables:ReadonlyMap<string,string>):SchedulerReceiptV1|undefined{
 try{
  const rows=[...executables].sort(([a],[b])=>a<b?-1:a>b?1:0).map(([name,path])=>{const row=stamp(path);return Object.freeze({name,path,size:Number(row.size),ino:String(row.ino),mtimeNs:String(row.mtimeNs)});});
  const receipt:SchedulerReceiptV1=Object.freeze({version:1,identity,inputs:Object.freeze(inputRows(manifest)),executables:Object.freeze(rows)});
  return validateJsonSchemaSubset(schema.definitions.Receipt,receipt,schema).length?undefined:receipt;
 }catch{return undefined;}
}

/** ✅️ Returns the recorded executables only while sources, compiler identity and every executable stamp are unchanged. */
export function schedulerReceiptHoldsV1(receipt:SchedulerReceiptV1|undefined,identity:string,manifest:string):ReadonlyMap<string,string>|undefined{
 if(!receipt||receipt.identity!==identity)return undefined;
 try{
  const now=inputRows(manifest);
  if(now.length!==receipt.inputs.length||now.some((row,index)=>row.path!==receipt.inputs[index]!.path||row.digest!==receipt.inputs[index]!.digest))return undefined;
  for(const row of receipt.executables){const actual=stamp(row.path);if(Number(actual.size)!==row.size||String(actual.ino)!==row.ino||String(actual.mtimeNs)!==row.mtimeNs)return undefined;}
  return new Map(receipt.executables.map(row=>[row.name,row.path]));
 }catch{return undefined;}
}

/** 📖️ Reads and schema-admits the stored receipt text. */
export function readSchedulerReceiptV1(path:string):SchedulerReceiptV1|undefined{
 try{const bytes=readFileSync(path);if(bytes.byteLength>protocol.maximumReceiptBytes)return undefined;const value=JSON.parse(bytes.toString("utf8"));return validateJsonSchemaSubset(schema.definitions.Receipt,value,schema).length?undefined:value as SchedulerReceiptV1;}catch{return undefined;}
}

/** 💾️ Publishes the receipt by atomic rename beside the executables. */
export function writeSchedulerReceiptV1(path:string,receipt:SchedulerReceiptV1):void{
 const temporary=path+"~"+process.pid;try{writeFileSync(temporary,JSON.stringify(receipt)+"\n",{flag:"w"});renameSync(temporary,path);}finally{rmSync(temporary,{force:true});}
}

/** 🏷️ The receipt file name inside the compiler target directory. */
export function schedulerReceiptFileV1():string{return protocol.file;}
