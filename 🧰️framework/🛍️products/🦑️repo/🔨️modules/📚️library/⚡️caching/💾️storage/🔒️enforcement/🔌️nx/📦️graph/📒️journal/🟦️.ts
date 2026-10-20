import {AsyncLocalStorage} from "node:async_hooks";
import {createHash} from "node:crypto";
import {constants,closeSync,existsSync,fstatSync,lstatSync,openSync,readSync,type BigIntStats} from "node:fs";
import {dirname,join} from "node:path";
import {compileJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
/** 📒️ Reads the durable Nx graph publication journal and its file allocations; the storage namespace inventory and the publication transaction both import it, neither imports the other. */
const checks=new AsyncLocalStorage<()=>void>();
const check=()=>{const current=checks.getStore();if(!current)throw Error("Original graph transaction scope required");current();};
const contract=compileJsonSchemaSubset({...schema,$ref:"#/definitions/GraphPublicationJournal"}),policy=schema.definitions.GraphPublicationPolicy.const,names=["project-graph.json","source-maps.json","file-map.json"] as const;
type Name=typeof names[number];
interface Fact{identity:string;bytes:number;sha256:string}
interface Row{name:Name;prior:Fact|null;next:{identity:string|null;bytes:number;sha256:string}}
interface Journal{version:1;generation:string;phase:"preparing"|"publishing"|"committed";files:Row[]}
const digest=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex"),identity=(stat:Pick<BigIntStats,"dev"|"ino">)=>String(stat.dev)+":"+String(stat.ino),stamp=(stat:BigIntStats)=>[stat.dev,stat.ino,stat.size,stat.nlink,stat.mtimeNs,stat.ctimeNs].join(":");
function regular(path:string,maximum:number,links:number|bigint=1){const stat=lstatSync(path,{bigint:true});if(!stat.isFile()||stat.isSymbolicLink()||stat.nlink!==BigInt(links)||stat.size>BigInt(maximum))throw Error("Closed graph publication file required");return stat;}
function descriptor<T>(fd:number,body:()=>T):T{let failed=false,primary:unknown;try{return body();}catch(error){failed=true;primary=error;throw error;}finally{try{closeSync(fd);}catch(error){if(!failed)throw error;if(primary instanceof Error)try{Object.defineProperty(primary,"graphPublicationCloseErrors",{value:[error]});}catch{}}}}
function bytes(path:string,maximum:number,links:number|bigint=1){check();const before=regular(path,maximum,links),fd=openSync(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0));return descriptor(fd,()=>{if(stamp(fstatSync(fd,{bigint:true}))!==stamp(before))throw Error("Graph publication file changed");const value=Buffer.alloc(Number(before.size));for(let offset=0;offset<value.length;){check();const count=readSync(fd,value,offset,Math.min(1048576,value.length-offset),offset);if(!count)throw Error("Graph publication file truncated");offset+=count;}check();if(stamp(fstatSync(fd,{bigint:true}))!==stamp(before)||stamp(lstatSync(path,{bigint:true}))!==stamp(before))throw Error("Graph publication file changed");return value;});}
function journalValue(value:unknown):Journal{if(contract.validate(value).length)throw Error("Closed graph publication journal required");const journal=value as Journal,rows=journal.files;if(new Set(rows.map(row=>row.name)).size!==rows.length||!rows.some(row=>row.name==="project-graph.json")||!rows.some(row=>row.name==="source-maps.json")||journal.phase!=="preparing"&&rows.some(row=>row.next.identity===null))throw Error("Complete graph publication journal required");return journal;}
function journalRead(directory:string){const path=join(directory,"publication.json");if(!existsSync(path))return undefined;const raw=bytes(path,policy.journalMaximumBytes);return {raw,journal:journalValue(JSON.parse(raw.toString()))};}
function paths(directory:string,journal:Journal,row:Row){const file=join(directory,row.name);return{file,prior:file+"~"+journal.generation+".prior",next:file+"~"+journal.generation+".next"};}
/** 🧮️ Admits only normal files or a journal-authenticated original canonical/prior hardlink pair. */
function allocation(path:string):{bytes:number;identity:string}{
 const stat=lstatSync(path,{bigint:true});if(!stat.isFile()||stat.isSymbolicLink()||stat.nlink<1n||stat.nlink>2n)throw Error("Graph publication allocation refused");if(stat.nlink===2n){const directory=dirname(path),read=journalRead(directory);if(!read)throw Error("Graph publication hardlink journal required");const row=read.journal.files.find(row=>{const p=paths(directory,read.journal,row);return path===p.file||path===p.prior;});if(!row?.prior)throw Error("Graph publication prior pair required");const p=paths(directory,read.journal,row),a=regular(p.file,policy.maximumFileBytes,2),b=regular(p.prior,policy.maximumFileBytes,2);if(identity(a)!==identity(b)||identity(a)!==row.prior.identity)throw Error("Graph publication hardlink identity refused");}return{bytes:Math.max(Number(stat.blocks)*512,Math.ceil(Number(stat.size)/4096)*4096),identity:identity(stat)};
}
/** 🧮️ Inventory holds original pool custody while validating journal-authenticated links. */
export function graphPublicationFileAllocationV1(path:string,control:ResourceLeaseControl):{bytes:number;identity:string}{return checks.run(()=>checkResourceLeaseControl(control),()=>allocation(path));}
/** 🔒️ Runs one body under the original transaction scope check. */
export function withGraphPublicationChecksV1<T>(current:()=>void,body:()=>T):T{return checks.run(current,body);}
export type {Name as GraphPublicationNameV1,Fact as GraphPublicationFactV1,Row as GraphPublicationRowV1,Journal as GraphPublicationJournalV1};
export {check as requireGraphPublicationScopeV1,policy as graphPublicationPolicyV1,names as graphPublicationNamesV1,digest as graphPublicationDigestV1,identity as graphPublicationIdentityV1,stamp as graphPublicationStampV1,regular as requireGraphPublicationRegularFileV1,descriptor as withGraphPublicationDescriptorV1,bytes as readGraphPublicationBytesV1,journalValue as readGraphPublicationJournalValueV1,journalRead as readGraphPublicationJournalV1,paths as graphPublicationPathsV1};
