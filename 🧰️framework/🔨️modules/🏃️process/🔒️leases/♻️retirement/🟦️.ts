import{requireIssuedResourceControlV1,withRequestedIssuedResourceControlV1}from"../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{withArtifactReadDirectoryV1}from"../../../../📦️packages/🟦️typescript/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import type{ResourceLeaseControl}from"../🎛️control/🟦️.ts";
import {lstatSync,type Stats} from "node:fs";
import {createHash} from "node:crypto";
import {dirname,join,resolve} from "node:path";
import authoritySchema from "../🧬️schema/📜️ticket.json" with {type:"json"};
import scanSchema from"./🧬️schema/🔣️.json" with{type:"json"};
const schema=authoritySchema.definitions.RetirementPolicy;
/** 📐️ Bounds cooperative scanning even when unfinished entries consume no candidate slot. */
export const resourceLeaseRetirementScanLimitsV1=Object.freeze({...scanSchema.definitions.Scan.const});

/** 🪨️ Keeps both native SQLite implementations behind one repository-owned contract. */
export interface ResourceLeaseDatabaseV1{exec(sql:string):void;read(sql:string,...values:string[]):Record<string,unknown>|undefined;close():void;}
/** 📸️ Retains the exact immediate owned candidate before SQLite opens it. */
export type ResourceLeaseCandidateV1=Readonly<{path:string;directory:string;file:Readonly<{dev:number;ino:number}>;parent:Readonly<{dev:number;ino:number}>;allocatedBytes:number}>;
/** 🚥️ Recognizes only actual SQLite contention and the owner's closed nonwaiting refusal. */
export function resourceLeaseBusyV1(error:unknown):boolean{const value=error!==null&&typeof error==="object"?error as{code?:string;errcode?:number;errno?:number}:undefined;return value?.code==="SQLITE_BUSY"||value?.code==="SQLITE_LOCKED"||[5,6].includes((value?.errcode??value?.errno??-1)&255)||error instanceof Error&&error.message.startsWith("Resource lease timed out: ");}
/** 🧹️ Attempts every cleanup independently while preserving any arbitrary primary failure. */
export function finishResourceLeaseCleanupV1(failed:boolean,failure:unknown,operations:readonly (()=>void)[]):void{for(const operation of operations)try{operation();}catch(error){if(!failed){failed=true;failure=error;}}if(failed)throw failure;}
/** 🛟️ Retains private admission custody until every failed connection close and gate close confirms completion. */
export class ResourceLeaseAdmissionFinalizerV1<T extends{release():void}>{
 readonly #pending=new Map<T,Set<ResourceLeaseDatabaseV1>>();
 protect(gate:T,database:ResourceLeaseDatabaseV1):void{const pending=this.#pending.get(gate)??new Set<ResourceLeaseDatabaseV1>();pending.add(database);this.#pending.set(gate,pending);}
 release(gate:T|undefined):void{if(!gate)return;const pending=this.#pending.get(gate)??new Set<ResourceLeaseDatabaseV1>();this.#pending.set(gate,pending);let failed=false,failure:unknown;for(const database of pending)try{database.close();pending.delete(database);}catch(error){if(!failed){failed=true;failure=error;}}if(failed)throw failure;gate.release();this.#pending.delete(gate);}
 retry():void{for(const gate of this.#pending.keys())try{this.release(gate);}catch{}}
}
/** 🗂️ Retains cooperative immediate names through the existing tracked buffered directory port. */
export function resourceLeaseRetirementNamesV1(directory:string,control:ResourceLeaseControl):Promise<readonly string[]>{requireIssuedResourceControlV1(control);if(typeof directory!=="string"||!directory||directory.length>256||directory.includes("\0"))throw Error("Bounded resource inventory directory required");return withRequestedIssuedResourceControlV1(control,0,async actual=>{const started=Date.now(),names=await withArtifactReadDirectoryV1(directory,()=>requireIssuedResourceControlV1(actual),async handle=>{const names:string[]=[];for(;;){requireIssuedResourceControlV1(actual);const row=handle.read();requireIssuedResourceControlV1(actual);if(!row)break;if(names.length>=schema.properties.maximumDirectoryEntries.const)throw Error("Bounded resource directory inventory required");names.push(row.name);if(names.length%resourceLeaseRetirementScanLimitsV1.entriesPerYield===0){if(names.length===resourceLeaseRetirementScanLimitsV1.entriesPerYield||names.length%resourceLeaseRetirementScanLimitsV1.entriesPerProgress===0)await actual.publish({resource:"resource-retirement-inventory:"+names.length,mode:"shared",elapsedMs:Math.max(0,Date.now()-started)});await actual.yieldContinuation();requireIssuedResourceControlV1(actual);}}requireIssuedResourceControlV1(actual);return Object.freeze(names.sort());});requireIssuedResourceControlV1(actual);return names;});}
function sidecars(path:string):boolean{return ["-journal","-wal","-shm"].some(suffix=>lstatSync(path+suffix,{throwIfNoEntry:false})!==undefined);}
/** 🛣️ Requires physical directory ancestry without following symlinks. */
function parent(directory:string):Stats{let held:Stats|undefined;for(let path=resolve(directory);;path=dirname(path)){const row=lstatSync(path);if(!row.isDirectory()||row.isSymbolicLink())throw Error("Original retirement directory required");held??=row;if(dirname(path)===path)break;}return held!;}
/** 📌️ Admits only one nonempty regular single-link same-device SQLite file without sidecars. */
export function captureResourceLeaseCandidateV1(directory:string,name:string):ResourceLeaseCandidateV1|undefined{if(!/^[a-f0-9]{64}\.sqlite$/u.test(name))return;const path=join(directory,name),file=lstatSync(path,{throwIfNoEntry:false});if(!file||!file.isFile()||file.isSymbolicLink()||file.nlink!==1||file.size<1||file.size>schema.properties.maximumDatabaseBytes.const)return;const owner=parent(directory);if(file.dev!==owner.dev||sidecars(path))return;return Object.freeze({path,directory,file,parent:owner,allocatedBytes:file.blocks*512});}
/** 🧷️ Rechecks physical identity after the actual connection closes and immediately before unlink. */
export function requireResourceLeaseCandidateV1(candidate:ResourceLeaseCandidateV1):void{const owner=parent(candidate.directory),file=lstatSync(candidate.path);if(owner.dev!==candidate.parent.dev||owner.ino!==candidate.parent.ino||!file.isFile()||file.isSymbolicLink()||file.nlink!==1||file.dev!==candidate.file.dev||file.ino!==candidate.file.ino||file.size<1||file.size>schema.properties.maximumDatabaseBytes.const||sidecars(candidate.path))throw Error("Original retirement database inode required");}
/** 🪪️ Authenticates the exact recognized schema, sole identity row and full logical filename relation. */
export function readOwnedResourceLeaseV1(database:ResourceLeaseDatabaseV1,directory:string,path:string):string|undefined{if(database.read("PRAGMA application_id")?.application_id!==schema.properties.applicationId.const||database.read("PRAGMA journal_mode")?.journal_mode!=="delete"||database.read("SELECT count(*) AS count FROM sqlite_schema")?.count!==1)return;const table=database.read("SELECT type,name,tbl_name,sql FROM sqlite_schema");if(table?.type!=="table"||table.name!=="semio_resource_lease"||table.tbl_name!=="semio_resource_lease"||table.sql!==schema.properties.tableSql.const||database.read("SELECT count(*) AS count FROM semio_resource_lease")?.count!==1)return;const row=database.read("SELECT id,version,resource FROM semio_resource_lease");if(row?.id!==1||row.version!==1||typeof row.resource!=="string"||!row.resource.length||row.resource.length>4096||row.resource.includes("\0")||join(directory,createHash("sha256").update(row.resource).digest("hex")+".sqlite")!==path)return;return row.resource;}
