import {createHash} from "node:crypto";
import {lstatSync,readdirSync,readFileSync,renameSync,writeFileSync,type BigIntStats,type Dirent} from "node:fs";
import {homedir} from "node:os";
import {dirname,isAbsolute,join,relative,sep} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {cargoPreparationInputV1,type CargoPreparationInputV1} from "../🧾️custody/🟦️.ts";
import {parseCargoPreparationStorageV1,type CargoPreparationStorageV1} from "../📦️storage/🟦️.ts";
import synchronizationSchema from "../🧾️synchronization/🧬️schema/🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {refuseBuildV1} from "../../../../🏗️build/📖️texts/🟦️.ts";

export type CargoPhysicalOperationKindV1="state"|"presence"|"list"|"text";
export type CargoPhysicalObservationV1=Readonly<{operation:CargoPhysicalOperationKindV1;path:string;stamp:string;stat?:string;entries?:readonly string[]}>;
export type CargoPhysicalObservationsV1=Map<string,CargoPhysicalObservationV1>;
export type CargoToolStampV1=Readonly<{path:string;stamp:string}>;
export type CargoPreparationWitnessV1=Readonly<{version:1;receipt:Readonly<{inputsSha256:string;lockSha256:string}>;lock:string;hashed:Readonly<Record<string,unknown>>;observations:readonly CargoPhysicalObservationV1[];tools:readonly CargoToolStampV1[]}>;
type Hashed=Readonly<{sources:readonly Readonly<{path:string;sha256:string}>[];inputs:readonly CargoPreparationInputV1[];configuration:readonly CargoPreparationInputV1[];globalConfiguration:unknown;compilerEnvironment:unknown;recipes:readonly unknown[]}>;
const protocol=schema.definitions.Protocol.const;
const sha256=(value:string|Buffer):string=>createHash("sha256").update(value).digest("hex");
const kind=(info:Readonly<{isFile():boolean;isDirectory():boolean;isSymbolicLink():boolean}>):string=>info.isSymbolicLink()?"symlink":info.isFile()?"file":info.isDirectory()?"directory":"other";
const statText=(info:BigIntStats):string=>[info.dev,info.ino,info.size,info.mtimeNs,info.ctimeNs].join(":");
const bounded=(path:string):unknown=>{const info=lstatSync(path);if(!info.isFile()||info.size>protocol.maximumBytes)throw refuseBuildV1("invalid","cargo.preparation-record-bounded");return JSON.parse(readFileSync(path,"utf8"));};

/** 🏷️ Reduces one physical answer to exactly the facts a Cargo workspace plan reads from it: kind, presence, the listed directories and manifest, or text bytes. */
export function cargoPhysicalStampV1(operation:CargoPhysicalOperationKindV1,value:unknown):string{
 if(operation==="state")return kind(value as Parameters<typeof kind>[0]);
 if(operation==="presence")return String(value===true);
 if(operation==="list")return sha256(JSON.stringify(cargoListedEntriesV1(value as readonly Dirent[])));
 return sha256(String(value));
}
/** 📇️ Names the entries of one listing a Cargo workspace plan can act on: its sub-directories and its manifest, sorted. */
export function cargoListedEntriesV1(entries:readonly Dirent[]):string[]{return entries.filter(entry=>entry.isDirectory()||entry.isFile()&&entry.name===protocol.listedFile).map(entry=>entry.name).sort();}
const skippedV1=(name:string):boolean=>name.startsWith(".")||protocol.skippedNames.includes(name);
/** 🕳️ Holds when no listing plan can find a manifest below a directory that appeared: every plan skips its name, or its plan-visible subtree holds no manifest within the protocol's entry bound. */
function manifestFreeV1(path:string,name:string):boolean{
 if(name===protocol.listedFile)return false;if(skippedV1(name))return true;
 let budget=protocol.maximumAppearedEntries;const stack=[path];
 while(stack.length){const directory=stack.pop()!;for(const entry of readdirSync(directory,{withFileTypes:true})){if(--budget<0)return false;if(skippedV1(entry.name))continue;if(entry.isFile()&&entry.name===protocol.listedFile)return false;if(entry.isDirectory())stack.push(join(directory,entry.name));}}
 return true;
}
/** 📼️ Answers one physical question through `answer` and records the answer's stamp; a listing or text answer also keeps the stat taken before it when that stat is older than the racy window, so a replay may trust an unchanged stat without reading again. A second, different answer to the same question poisons the set. */
export function observeCargoPhysicalV1<T>(observations:CargoPhysicalObservationsV1,operation:CargoPhysicalOperationKindV1,path:string,answer:()=>T):T{
 let stat:string|undefined;if(operation==="list"||operation==="text")try{const info=lstatSync(path,{bigint:true}),settled=BigInt(Date.now()-protocol.racyMilliseconds)*1000000n;if(info.mtimeNs<settled&&info.ctimeNs<settled)stat=statText(info);}catch{}
 const value=answer(),entries=operation==="list"?cargoListedEntriesV1(value as unknown as readonly Dirent[]):undefined,stamp=entries?sha256(JSON.stringify(entries)):cargoPhysicalStampV1(operation,value),key=operation+"\0"+path,previous=observations.get(key),agreed=!previous||previous.stamp===stamp;
 observations.set(key,{operation,path,stamp:agreed?stamp:"",...(agreed&&stat&&(!previous||previous.stat===stat)?{stat}:{}),...(agreed&&entries?{entries}:{})});return value;
}
/** 🔁️ Asks every recorded physical question again under the plan's own directory and symlink guards and holds only when every answer is unchanged or cannot change what a plan derives: an unchanged settled stat answers a listing or text question without reading it; a listing may differ only by appeared directories no plan can find a manifest in and by vanished directories below which no manifest, file kind or present path was read (their recorded questions are then not asked).
 * @see ../../🟦️.ts — `operate`, the answers being replayed */
export function cargoPhysicalObservationsHoldV1(root:string,observations:readonly CargoPhysicalObservationV1[]):boolean{
 const stats=new Map<string,BigIntStats|null>(),stat=(path:string):BigIntStats|null=>{let value=stats.get(path);if(value===undefined){try{value=lstatSync(path,{bigint:true});}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;value=null;}stats.set(path,value);}return value;},state=(path:string):string=>{const info=stat(path);return info?kind(info):"absent";};
 const directory=(path:string):boolean=>{const local=relative(root,path);if(local===".."||local.startsWith(".."+sep)||isAbsolute(local)||state(root)!=="directory")return false;let current=root;for(const part of local.split(sep).filter(Boolean)){current=join(current,part);if(state(current)!=="directory")return false;}return true;};
 const settled=(row:CargoPhysicalObservationV1):boolean=>row.stat!==undefined&&row.stat===statText(stat(row.path)!);
 const vanished:string[]=[],within=(path:string):boolean=>vanished.some(prefix=>path===prefix||path.startsWith(prefix+sep));
 try{
  for(const row of observations){
   if(!row.stamp)return false;let stamp:string;
   if(within(row.path)){if(row.operation==="text"||row.operation==="presence"&&row.stamp==="true"||row.operation==="state"&&row.stamp==="file")return false;continue;}
   if(row.operation==="list"){
    if(!directory(row.path))return false;if(settled(row))continue;
    const entries=cargoListedEntriesV1(readdirSync(row.path,{withFileTypes:true}));stamp=sha256(JSON.stringify(entries));
    if(stamp!==row.stamp){if(!row.entries||sha256(JSON.stringify(row.entries))!==row.stamp)return false;const now=new Set(entries),then=new Set(row.entries);for(const name of row.entries)if(!now.has(name))vanished.push(join(row.path,name));if(entries.some(name=>!then.has(name)&&!manifestFreeV1(join(row.path,name),name)))return false;continue;}
   }
   else{if(!directory(row.path===root?root:dirname(row.path)))return false;const current=state(row.path);if(row.operation==="state"){if(current==="absent")return false;stamp=current;}else if(row.operation==="presence"){if(current==="symlink")return false;stamp=String(current!=="absent");}else{if(current!=="file")return false;stamp=settled(row)?row.stamp:cargoPhysicalStampV1("text",readFileSync(row.path,"utf8"));}}
   if(stamp!==row.stamp)return false;
  }
  return true;
 }catch{return false;}
}
/** 🌍️ Selects the compiler environment a synchronization receipt binds, from the schema's declared names. */
export function cargoPreparationCompilerEnvironmentV1(environment:Readonly<Record<string,string|undefined>>):Record<string,string|undefined>{
 const rule=protocol.compilerEnvironment;return Object.fromEntries(Object.entries(environment).filter(([name])=>(name.startsWith(rule.prefix)&&!rule.excludedPrefixes.some(prefix=>name.startsWith(prefix))&&!rule.excludedNames.includes(name))||rule.names.includes(name)).sort(([a],[b])=>a.localeCompare(b)));
}
/** 🏠️ Hashes the user-level Cargo configuration a synchronization receipt binds; an absent file binds as `null`. */
export function cargoPreparationGlobalConfigurationV1(environment:Readonly<Record<string,string|undefined>>):readonly Readonly<{path:string;sha256:string|null}>[]{
 const home=environment.CARGO_HOME??join(homedir(),".cargo");return protocol.globalConfiguration.map(name=>{const path=join(home,name);try{return{path,sha256:sha256(readFileSync(path))};}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;return{path,sha256:null};}});
}
/** 🗝️ Names the receipt and witness files of one selected owner and package set. */
export function cargoPreparationRecordPathsV1(storage:CargoPreparationStorageV1,root:string,manifest:string,packages:readonly string[]):Readonly<{receipt:string;witness:string}>{
 const key=sha256(JSON.stringify([root,manifest,[...packages].sort()]));return{receipt:join(storage.directory,key+protocol.receiptSuffix),witness:join(storage.directory,key+protocol.witnessSuffix)};
}
/** 🔩️ Stamps one tool executable by kind, device, inode, size and change times; a sealed toolchain keeps it. */
export function cargoToolStampV1(path:string):CargoToolStampV1{const info=lstatSync(path,{bigint:true});return{path,stamp:[kind(info),info.dev,info.ino,info.size,info.mtimeNs,info.ctimeNs].join(":")};}
/** ✍️ Publishes the witness of a held receipt beside it; a poisoned observation set, a recipe run or a tool-less identity publishes nothing. */
export function writeCargoPreparationWitnessV1(storage:CargoPreparationStorageV1,root:string,manifest:string,packages:readonly string[],scratch:string,witness:CargoPreparationWitnessV1):boolean{
 const text=JSON.stringify(witness);if(witness.observations.some(row=>!row.stamp)||Buffer.byteLength(text)>protocol.maximumBytes||validateJsonSchemaSubset(schema,witness).length)return false;
 const temporary=join(scratch,"witness.json");writeFileSync(temporary,text);renameSync(temporary,cargoPreparationRecordPathsV1(storage,root,manifest,packages).witness);return true;
}
/** 🧾️ Holds when the receipt a preparation child would compare still matches: every source, manifest, configuration file, physical answer, environment value and tool it hashed is unchanged.
 * @see ../🟦️.ts — `withPreparedCargoDependencyPairV1`, the child-side comparison this decides ahead of */
export function cargoPreparationWitnessHoldsV1(storage:CargoPreparationStorageV1,root:string,manifest:string,packages:readonly string[],environment:Readonly<Record<string,string|undefined>>):boolean{
 try{
  const paths=cargoPreparationRecordPathsV1(parseCargoPreparationStorageV1(storage),root,manifest,packages),witness=bounded(paths.witness) as CargoPreparationWitnessV1,receipt=bounded(paths.receipt) as CargoPreparationWitnessV1["receipt"];
  if(validateJsonSchemaSubset(schema,witness).length||validateJsonSchemaSubset(synchronizationSchema,receipt).length||receipt.inputsSha256!==witness.receipt.inputsSha256||receipt.lockSha256!==witness.receipt.lockSha256||sha256(JSON.stringify(witness.hashed))!==receipt.inputsSha256)return false;
  const hashed=witness.hashed as Hashed;
  if(hashed.recipes.length||JSON.stringify(hashed.compilerEnvironment)!==JSON.stringify(cargoPreparationCompilerEnvironmentV1(environment))||JSON.stringify(hashed.globalConfiguration)!==JSON.stringify(cargoPreparationGlobalConfigurationV1(environment))||cargoPreparationInputV1(root,witness.lock,"file").sha256!==receipt.lockSha256)return false;
  for(const source of hashed.sources){const info=lstatSync(source.path);if(!info.isFile()||sha256(readFileSync(source.path))!==source.sha256)return false;}
  for(const input of [...hashed.inputs,...hashed.configuration])if(cargoPreparationInputV1(root,input.path,input.kind).sha256!==input.sha256)return false;
  for(const tool of witness.tools)if(cargoToolStampV1(tool.path).stamp!==tool.stamp)return false;
  return cargoPhysicalObservationsHoldV1(root,witness.observations);
 }catch{return false;}
}
