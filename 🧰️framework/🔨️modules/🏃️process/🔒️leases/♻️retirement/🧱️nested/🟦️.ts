import{createHash}from"node:crypto";
import{lstatSync,readdirSync,rmdirSync,unlinkSync}from"node:fs";
import{join}from"node:path";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../🎛️control/🟦️.ts";
import authority from"../../🧬️schema/📜️ticket.json"with{type:"json"};
import metadata from"../../📦️metadata/🧬️schema/🔣️.json"with{type:"json"};
import{finishResourceLeaseCleanupV1,openResourceLeaseDatabaseV1,resourceLeaseBusyV1,resourceLeaseRetirementIdlePolicyV1,type ResourceLeaseDatabaseV1}from"../🟦️.ts";
import{retireIdleResourceLeasesV1,retireResourceQueueClaimsV1}from"../../🟦️.ts";
const policy=authority.definitions.RetirementPolicy.properties,ticket=new RegExp(authority.definitions.Protocol.const.ticketPattern,"u"),custody=authority.definitions.ClaimCustodyPolicy.properties,gateFile=createHash("sha256").update(policy.admissionResource.const).digest("hex")+".sqlite",database=/^[a-f0-9]{64}\.sqlite$/u,queue=/^[a-f0-9]{64}\.queue$/u,stamp=".retired";

/** 🧱️ Names the recognized files of one nested lease store: its resource databases, its empty queues, its claim databases and its two bookkeeping files. */
export type NestedLeaseStoreV1=Readonly<{directory:string;databases:readonly string[];queues:readonly string[];claims:readonly string[];extras:readonly string[];allocatedBytes:number;newestMs:number}>;
/** 📊️ Reports the nested stores examined, retired and kept in one bounded turn. */
export type NestedLeaseRetirementV1=Readonly<{examined:number;retired:number;kept:number;allocatedBytes:number}>;
const regular=(path:string,limit:number):{blocks:number;mtimeMs:number}|undefined=>{const row=lstatSync(path,{throwIfNoEntry:false});return row&&row.isFile()&&!row.isSymbolicLink()&&row.nlink===1&&row.size<=limit?{blocks:row.blocks,mtimeMs:row.mtimeMs}:undefined;};

/** 🔎️ Recognizes a directory that holds nothing but lease store files; one unknown entry, link or sidecar makes it foreign and therefore untouchable, and with `drained` a queue that still holds a ticket does too. */
export function inspectNestedLeaseStoreV1(directory:string,drained=false):NestedLeaseStoreV1|undefined{
 const root=lstatSync(directory,{throwIfNoEntry:false});if(!root||!root.isDirectory()||root.isSymbolicLink())return;
 const limit=resourceLeaseRetirementIdlePolicyV1.nestedEntries,databases:string[]=[],queues:string[]=[],claims:string[]=[],extras:string[]=[];let allocated=0,newest=root.mtimeMs,names:string[];
 try{names=readdirSync(directory);}catch{return;}
 if(names.length>limit)return;
 for(const name of names){
  const path=join(directory,name);
  if(database.test(name)){const row=regular(path,custody.maximumDatabaseBytes.const);if(!row)return;databases.push(name);allocated+=row.blocks*512;newest=Math.max(newest,row.mtimeMs);}
  else if(queue.test(name)){const row=lstatSync(path),tickets=row.isDirectory()&&!row.isSymbolicLink()?readdirSync(path):undefined;if(!tickets||tickets.length>limit||drained&&tickets.length||!tickets.every(entry=>ticket.test(entry)&&regular(join(path,entry),limit)))return;queues.push(name);newest=Math.max(newest,row.mtimeMs);for(const entry of tickets)newest=Math.max(newest,regular(join(path,entry),limit)!.mtimeMs);}
  else if(name===custody.claimDirectory.const){const row=lstatSync(path);if(!row.isDirectory()||row.isSymbolicLink())return;const inner=readdirSync(path);if(inner.length>limit)return;newest=Math.max(newest,row.mtimeMs);for(const claim of inner){const found=regular(join(path,claim),custody.maximumDatabaseBytes.const);if(!database.test(claim)||!found)return;claims.push(claim);allocated+=found.blocks*512;newest=Math.max(newest,found.mtimeMs);}}
  else if(name===metadata.properties.filename.const){const row=regular(path,metadata.properties.capacityBytes.const);if(!row)return;extras.push(name);allocated+=row.blocks*512;newest=Math.max(newest,row.mtimeMs);}
  else if(name===stamp){const row=regular(path,0);if(!row)return;extras.push(name);newest=Math.max(newest,row.mtimeMs);}
  else return;
 }
 if(!databases.length&&!claims.length)return;
 return Object.freeze({directory,databases:Object.freeze(databases.sort((left,right)=>(right===gateFile?1:0)-(left===gateFile?1:0)||left.localeCompare(right,"en"))),queues:Object.freeze(queues),claims:Object.freeze(claims.sort()),extras:Object.freeze(extras),allocatedBytes:allocated,newestMs:newest});
}

async function dismantle(store:NestedLeaseStoreV1,control:ResourceLeaseControl):Promise<boolean>{
 const held:ResourceLeaseDatabaseV1[]=[];let failed=false,failure:unknown,removed=false;
 try{
  const paths=[...store.databases.map(name=>join(store.directory,name)),...store.claims.map(name=>join(store.directory,custody.claimDirectory.const,name))];
  for(const path of paths){
   checkResourceLeaseControl(control);let opened:ResourceLeaseDatabaseV1;
   try{opened=await openResourceLeaseDatabaseV1(path,false,false);}catch(error){if(["ENOENT","SQLITE_CANTOPEN"].includes((error as{code?:string}).code??""))return false;throw error;}
   held.push(opened);
   try{opened.exec("PRAGMA busy_timeout=0;PRAGMA trusted_schema=OFF");if(opened.read("PRAGMA application_id")?.application_id!==policy.applicationId.const)return false;opened.exec("BEGIN EXCLUSIVE");}catch(error){if(resourceLeaseBusyV1(error)||["SQLITE_NOTADB","SQLITE_CORRUPT","SQLITE_CANTOPEN"].includes((error as{code?:string}).code??""))return false;throw error;}
  }
  const again=inspectNestedLeaseStoreV1(store.directory,true);
  if(!again||again.databases.join()!==store.databases.join()||again.claims.join()!==store.claims.join()||again.queues.join()!==store.queues.join())return false;
  checkResourceLeaseControl(control);
  for(const name of store.claims)unlinkSync(join(store.directory,custody.claimDirectory.const,name));
  if(store.claims.length||lstatSync(join(store.directory,custody.claimDirectory.const),{throwIfNoEntry:false}))rmdirSync(join(store.directory,custody.claimDirectory.const));
  for(const name of store.queues)rmdirSync(join(store.directory,name));
  for(const name of store.extras)unlinkSync(join(store.directory,name));
  for(const name of store.databases)unlinkSync(join(store.directory,name));
  removed=true;
  return true;
 }catch(error){failed=true;failure=error;throw error;}
 finally{
  const closing=held.map(row=>()=>row.close());
  finishResourceLeaseCleanupV1(failed,failure,[...closing,()=>{if(removed)try{rmdirSync(store.directory);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}}]);
 }
}

/** ♻️ Retires abandoned nested lease stores of a lease directory (test runs that were killed leave them behind, each with its own preallocated headroom file): a store goes only when every file is recognized, nothing in it was used for the idle period, no queue holds a ticket and the kernel grants an exclusive transaction on every one of its databases at once; a holder or a returning owner makes it stay or recreates it cleanly. */
export async function retireNestedLeaseStoresV1(directory:string,control:ResourceLeaseControl,limits:Readonly<{idleMilliseconds?:number;nowMilliseconds?:number;maximumStores?:number}>={}):Promise<NestedLeaseRetirementV1>{
 checkResourceLeaseControl(control);const idle=limits.idleMilliseconds??resourceLeaseRetirementIdlePolicyV1.nestedIdleMilliseconds,now=limits.nowMilliseconds??Date.now(),cap=limits.maximumStores??resourceLeaseRetirementIdlePolicyV1.nestedStoresPerSweep;
 let examined=0,retired=0,kept=0,allocatedBytes=0,names:string[];
 try{names=readdirSync(directory,{withFileTypes:true}).filter(row=>row.isDirectory()&&!row.isSymbolicLink()&&row.name!==custody.claimDirectory.const&&!queue.test(row.name)).map(row=>row.name).sort();}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return Object.freeze({examined,retired,kept,allocatedBytes});throw error;}
 for(const name of names){
  if(examined>=cap)break;checkResourceLeaseControl(control);await control.yieldContinuation();
  const store=inspectNestedLeaseStoreV1(join(directory,name));
  if(!store||idle>0&&now-store.newestMs<idle){kept++;continue;}
  examined++;
  const patience=resourceLeaseRetirementIdlePolicyV1.admissionPatienceMilliseconds;
  try{await retireResourceQueueClaimsV1(store.directory,control,0,{admissionPatienceMs:patience});await retireIdleResourceLeasesV1(store.directory,control,0,{admissionPatienceMs:patience});}catch(error){if(!resourceLeaseBusyV1(error))throw error;kept++;continue;}
  const drained=inspectNestedLeaseStoreV1(store.directory,true);
  if(drained&&await dismantle(drained,control)){retired++;allocatedBytes+=store.allocatedBytes;}else kept++;
 }
 return Object.freeze({examined,retired,kept,allocatedBytes});
}
