import {lstatSync,readFileSync,readdirSync,readlinkSync,unlinkSync} from "node:fs";
import {join} from "node:path";
import type {ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts";
import { javascriptDependencyGenerationDirectoryV1, javascriptDependencySourceOwnerRuntimeV1, selectObsoleteJavascriptDependencyGenerationsV1, type JavascriptDependencyReadContextV1 } from "../../💾️storage/📥️read-context/🟦️.ts";
import {javascriptInstallSchemaV1 as installSchema} from "../../💾️storage/🧱️schemas/🟦️.ts";
import {verifyJavascriptDependencyGenerationReceiptV1,writeJavascriptDependencyGenerationFilesV1} from "../../🧾️generation/💾️storage/🟦️.ts";
import {generationCauseV1,generationSha256V1} from "../../🧾️generation/🟦️.ts";
import {decodeInstalledLedgerV1,decodeOwnershipIndexV1,encodeInstalledLedgerV1,encodeOwnershipIndexV1,incrementalProtocolV1 as protocol,installedCountsV1,installedRowV1,installedStampV1,installedThresholdNsV1,lockConvergedV1,parseInstalledReceiptV1,parseLockReceiptV1,parseOwnershipReceiptV1,parseSyncReceiptV1,replayExecutionObservationsV1,sealInstalledReceiptV1,sealLockReceiptV1,sealOwnershipReceiptV1,sealSyncReceiptV1,verifyInstalledLedgerV1,type ExecutionObservationV1,type ExecutionReplayResultV1,type IncrementalControlV1,type InstalledChildV1,type InstalledPortsV1,type InstalledRowV1,type InstalledShapeV1,type LockReceiptV1,type OwnershipRecordV1,type StampSourceV1,type SyncReceiptV1} from "../🟦️.ts";

const directory=(storeRoot:string)=>join(storeRoot,protocol.directory),keyOf=(label:string)=>generationSha256V1(label).slice(0,16);
const bounded=(path:string,maximum:number):Buffer=>{const facts=lstatSync(path,{bigint:true});if(!facts.isFile()||facts.size>BigInt(maximum))throw Error("Sealed incremental file required");return readFileSync(path);};
const names=(storeRoot:string,prefix:string,suffix:string):string[]=>{
 try{return readdirSync(directory(storeRoot)).filter(name=>name.startsWith(prefix)&&name.endsWith(suffix)).map(name=>({name,at:lstatSync(join(directory(storeRoot),name)).mtimeMs})).sort((left,right)=>right.at-left.at).map(row=>row.name);}catch{return[];}
};
const unlinkQuietly=(storeRoot:string,name:string)=>{try{unlinkSync(join(directory(storeRoot),name));}catch{}};
const referenced=(storeRoot:string,suffix:string,parse:(bytes:Uint8Array)=>{evidence:{name:string}}):Set<string>=>{
 const keep=new Set<string>();
 for(const name of names(storeRoot,"",suffix))try{keep.add(parse(bounded(join(directory(storeRoot),name),protocol.sync.maximumReceiptBytes)).evidence.name);}catch{}
 return keep;
};
const sweepEvidence=(storeRoot:string,receiptSuffix:string,evidenceSuffix:string,parse:(bytes:Uint8Array)=>{evidence:{name:string}}):void=>{
 const keep=referenced(storeRoot,receiptSuffix,parse);
 for(const name of names(storeRoot,"",evidenceSuffix))if(!keep.has(name))unlinkQuietly(storeRoot,name);
};
const controlOf=(context:JavascriptDependencyReadContextV1,control:ResourceLeaseControl,label:string,check:()=>void=()=>{}):IncrementalControlV1=>{
 const started=performance.now(),verify=()=>{context.check(control);check();};
 return Object.freeze({check:verify,tick:async(work:number)=>{verify();await control.publish({resource:"dependencies:"+label+":"+work,mode:"shared",elapsedMs:performance.now()-started});await control.yieldContinuation();verify();}});
};
const kindOf=(facts:{isSymbolicLink():boolean;isDirectory():boolean;isFile():boolean}):string=>facts.isSymbolicLink()?"link":facts.isDirectory()?"directory":facts.isFile()?"file":"foreign";

/** 🪣️ Collects the ledger rows of one installed-paths scan as it closes each directory. */
export interface InstalledLedgerSinkV1{readonly directory:(portable:string,facts:StampSourceV1,children:readonly InstalledChildV1[])=>void;readonly rows:InstalledRowV1[]}
/** 📒️ Creates the sink for a scan that starts now; its threshold is the settle window before that start. */
export function installedLedgerSinkV1(startedAtMs:number=Date.now()):InstalledLedgerSinkV1{
 const threshold=installedThresholdNsV1(startedAtMs),rows:InstalledRowV1[]=[];
 return Object.freeze({rows,directory:(portable:string,facts:StampSourceV1,children:readonly InstalledChildV1[])=>{rows.push(installedRowV1(portable,facts,children,threshold));}});
}
/** 🔌️ Binds the two no-follow operations a ledger verification needs to one generation root. */
export function installedPortsV1(root:string,control:IncrementalControlV1):InstalledPortsV1{
 const path=(portable:string)=>portable==="."?root:join(root,...portable.split("/"));
 return Object.freeze({control,
  stat:(portable:string)=>{const facts=lstatSync(path(portable),{bigint:true,throwIfNoEntry:false});return facts?Object.freeze({kind:kindOf(facts),stamp:installedStampV1(facts)}):undefined;},
  children:(portable:string)=>readdirSync(path(portable),{withFileTypes:true}).map(entry=>Object.freeze({name:entry.name,kind:kindOf(entry)})),
 });
}

/** 🔬️ Proves an installed generation by its newest sealed ledger and the live directory stamps, or reports that the complete scan is required. */
export async function verifyJavascriptDependencyInstalledReceiptV1(storeRoot:string,root:string,generation:string,control:IncrementalControlV1):Promise<Readonly<{shape:InstalledShapeV1;sha256:string}>|undefined>{
 for(const name of names(storeRoot,generation.slice(0,20)+".",protocol.installed.receiptSuffix)){
  control.check();let receipt:ReturnType<typeof parseInstalledReceiptV1>,rows:readonly InstalledRowV1[];
  try{receipt=parseInstalledReceiptV1(bounded(join(directory(storeRoot),name),protocol.installed.maximumReceiptBytes));if(receipt.generation!==generation)continue;rows=decodeInstalledLedgerV1(bounded(join(directory(storeRoot),receipt.evidence.name),protocol.installed.maximumEvidenceBytes),receipt.evidence);}catch{continue;}
  const result=await verifyInstalledLedgerV1(receipt,rows,installedPortsV1(root,control));
  if(result.status==="match")return Object.freeze({shape:receipt.shape,sha256:receipt.sha256});
 }
 return undefined;
}
/** 🧾️ Seals the ledger of a completed scan under the caller's exclusive Store custody and retires the superseded ledgers of the same generation. */
export async function sealJavascriptDependencyInstalledReceiptV1(storeRoot:string,generation:string,scan:Readonly<{entries:number;directories:number;files:number;links:number;maximumPathUnits:number;sha256:string}>,rows:readonly InstalledRowV1[],checkpoint:()=>void,yieldContinuation:()=>Promise<void>):Promise<string>{
 const counts=installedCountsV1(rows);
 if(counts.entries!==scan.entries||counts.directories!==scan.directories||counts.files!==scan.files||counts.links!==scan.links)throw Error("Complete installed ledger of the scan required");
 const{evidence,bytes}=encodeInstalledLedgerV1(rows),shape:InstalledShapeV1=Object.freeze({entries:scan.entries,directories:scan.directories,files:scan.files,links:scan.links,maximumPathUnits:scan.maximumPathUnits,sha256:scan.sha256}),
  receipt=sealInstalledReceiptV1({version:1,purpose:"dependency-installed-paths-receipt",generation,device:rows[0]!.stamp[0],shape,evidence}),name=generation.slice(0,20)+"."+receipt.sha256+protocol.installed.receiptSuffix;
 await writeJavascriptDependencyGenerationFilesV1(storeRoot,[{name:evidence.name,bytes},{name,bytes:Buffer.from(JSON.stringify(receipt)+"\n")}],checkpoint,yieldContinuation);
 for(const other of names(storeRoot,generation.slice(0,20)+".",protocol.installed.receiptSuffix))if(other!==name)unlinkQuietly(storeRoot,other);
 sweepEvidence(storeRoot,protocol.installed.receiptSuffix,protocol.installed.evidenceSuffix,parseInstalledReceiptV1);
 return receipt.sha256;
}
/** 🧹️ Removes every installed-paths ledger whose generation no current scope names; failure only costs disk. */
export function retireJavascriptDependencyInstalledReceiptsV1(storeRoot:string,live:readonly string[]):void{
 const keep=new Set(live.map(generation=>generation.slice(0,20)));
 for(const name of names(storeRoot,"",protocol.installed.receiptSuffix))if(!keep.has(name.slice(0,20)))unlinkQuietly(storeRoot,name);
 sweepEvidence(storeRoot,protocol.installed.receiptSuffix,protocol.installed.evidenceSuffix,parseInstalledReceiptV1);
}

/** 📮️ Publishes the sealed synchronization receipt and retires every earlier one. */
export async function publishJavascriptDependencySyncReceiptV1(storeRoot:string,body:Omit<SyncReceiptV1,"sha256">,checkpoint:()=>void,yieldContinuation:()=>Promise<void>):Promise<string>{
 const receipt=sealSyncReceiptV1(body),name=keyOf(protocol.sync.keyLabel)+"."+receipt.sha256+protocol.sync.receiptSuffix;
 await writeJavascriptDependencyGenerationFilesV1(storeRoot,[{name,bytes:Buffer.from(JSON.stringify(receipt)+"\n")}],checkpoint,yieldContinuation);
 for(const other of names(storeRoot,keyOf(protocol.sync.keyLabel)+".",protocol.sync.receiptSuffix))if(other!==name)unlinkQuietly(storeRoot,other);
 return receipt.sha256;
}
/** 🚪️ Answers whether the last complete synchronization still holds: every sealed scope read matches its generation receipt, every installed ledger matches its directories and no obsolete generation remains. */
export async function verifyJavascriptDependencySyncCurrentV1(workspace:string,context:JavascriptDependencyReadContextV1,control:ResourceLeaseControl):Promise<Readonly<{status:"current";scopes:number}>|Readonly<{status:"stale";reason:string;cause?:Readonly<{class:string;path?:string;detail?:string}>}>>{
 const stale=(reason:string,cause?:Readonly<{class:string;path?:string;detail?:string}>)=>Object.freeze({status:"stale"as const,reason,...(cause===undefined?{}:{cause})}),root=context.root;let sync:SyncReceiptV1|undefined;
 for(const name of names(root,keyOf(protocol.sync.keyLabel)+".",protocol.sync.receiptSuffix))try{sync=parseSyncReceiptV1(bounded(join(directory(root),name),protocol.sync.maximumReceiptBytes));break;}catch{}
 if(!sync)return stale("absent");
 if(sync.runtimeSha256!==javascriptDependencySourceOwnerRuntimeV1())return stale("runtime");
 let present:string[];try{present=readdirSync(join(root,javascriptDependencyGenerationDirectoryV1));}catch{return stale("generations");}
 if(selectObsoleteJavascriptDependencyGenerationsV1(present,sync.scopes.map(row=>row.generation.slice(0,20)),[]).length)return stale("obsolete-generations");
 const incremental=controlOf(context,control,"sync-gate");
 for(const row of sync.scopes){
  const read=await verifyJavascriptDependencyGenerationReceiptV1(workspace,context,control,row.scope);
  if(read.status!=="match")return stale("generation:"+row.scope,generationCauseV1(read,row.scope));
  if(read.sha256!==row.read||read.receipts.find(receipt=>receipt.scope===row.scope)?.generation!==row.generation)return stale("generation:"+row.scope,{class:"sha-changed"});
  const installed=await verifyJavascriptDependencyInstalledReceiptV1(root,requireNativeStoragePathV1(root,join(root,javascriptDependencyGenerationDirectoryV1,row.generation.slice(0,20))),row.generation,incremental);
  if(!installed||installed.sha256!==row.installed)return stale("installed:"+row.scope);
 }
 return Object.freeze({status:"current",scopes:sync.scopes.length});
}
/** 🔎️ Names the sha of the installed-paths receipt that currently proves a generation, without verifying it. */
export function newestJavascriptDependencyInstalledReceiptV1(storeRoot:string,generation:string):string|undefined{
 for(const name of names(storeRoot,generation.slice(0,20)+".",protocol.installed.receiptSuffix))try{const receipt=parseInstalledReceiptV1(bounded(join(directory(storeRoot),name),protocol.installed.maximumReceiptBytes));if(receipt.generation===generation)return receipt.sha256;}catch{}
 return undefined;
}

/** 🏁️ Seals the synchronization state from the scope reads this run just proved and published, or reports that a scope lacks its installed-paths ledger. */
export async function publishJavascriptDependencySyncFromReadsV1(storeRoot:string,reads:readonly Readonly<{scope:string;generation:string;sha256:string}>[],checkpoint:()=>void,yieldContinuation:()=>Promise<void>):Promise<"published"|"incomplete">{
 const scopes:SyncReceiptV1["scopes"][number][]=[];
 for(const read of reads){const installed=newestJavascriptDependencyInstalledReceiptV1(storeRoot,read.generation);if(!installed)return"incomplete";scopes.push({scope:read.scope,generation:read.generation,read:read.sha256,installed});}
 await publishJavascriptDependencySyncReceiptV1(storeRoot,{version:1,purpose:"dependency-sync-receipt",selection:"all",runtimeSha256:javascriptDependencySourceOwnerRuntimeV1(),scopes},checkpoint,yieldContinuation);
 retireJavascriptDependencyInstalledReceiptsV1(storeRoot,reads.map(read=>read.generation));
 return"published";
}

/** 🔂️ Replays the recorded physical observations of an earlier provider walk against the live tree under the caller's custody, bracketing one custody recheck exactly like the walk itself. */
export async function replayJavascriptDependencyExecutionV1(context:JavascriptDependencyReadContextV1,control:ResourceLeaseControl,observations:readonly ExecutionObservationV1[],startedAtMs:number):Promise<ExecutionReplayResultV1>{
 const policy=installSchema.$defs.SourceOwnerExecutionProtocol.const,ports=Object.freeze({control:controlOf(context,control,"execution-replay"),
  stat:(path:string)=>{const facts=lstatSync(path,{bigint:true,throwIfNoEntry:false});return facts?Object.freeze({stamp:JSON.stringify(policy.selectedFields.map((name:string)=>String(facts[name as keyof typeof facts]))),anchor:JSON.stringify(policy.ancestorFields.map((name:string)=>String(facts[name as keyof typeof facts])))}):undefined;},
  readlink:(path:string)=>readlinkSync(path)});
 const first=await replayExecutionObservationsV1(observations,startedAtMs,ports);if(first.status!=="match")return first;
 await context.recheck(control);return replayExecutionObservationsV1(observations,startedAtMs,ports);
}

/** 🗂️ Reads the newest valid persisted ownership listing index as absolute records under the workspace. */
export function readJavascriptDependencyOwnershipIndexV1(storeRoot:string,workspace:string):readonly OwnershipRecordV1[]|undefined{
 for(const name of names(storeRoot,keyOf(protocol.ownership.keyLabel)+".",protocol.ownership.receiptSuffix))try{
  const receipt=parseOwnershipReceiptV1(bounded(join(directory(storeRoot),name),protocol.ownership.maximumReceiptBytes));
  return decodeOwnershipIndexV1(workspace,bounded(join(directory(storeRoot),receipt.evidence.name),protocol.ownership.maximumEvidenceBytes),receipt.evidence);
 }catch{}
 return undefined;
}
/** 💾️ Persists the settled listing records of the ownership walk that just succeeded and retires the superseded index. */
export async function writeJavascriptDependencyOwnershipIndexV1(storeRoot:string,workspace:string,records:readonly OwnershipRecordV1[],checkpoint:()=>void,yieldContinuation:()=>Promise<void>):Promise<"written"|"present"|"empty">{
 const{evidence,bytes}=encodeOwnershipIndexV1(workspace,records);if(!evidence.rows)return"empty";
 const receipt=sealOwnershipReceiptV1({version:1,purpose:"dependency-ownership-index",evidence}),prefix=keyOf(protocol.ownership.keyLabel)+".",name=prefix+receipt.sha256+protocol.ownership.receiptSuffix;
 if(names(storeRoot,prefix,protocol.ownership.receiptSuffix)[0]===name)return"present";
 await writeJavascriptDependencyGenerationFilesV1(storeRoot,[{name:evidence.name,bytes},{name,bytes:Buffer.from(JSON.stringify(receipt)+"\n")}],checkpoint,yieldContinuation);
 for(const other of names(storeRoot,prefix,protocol.ownership.receiptSuffix))if(other!==name)unlinkQuietly(storeRoot,other);
 sweepEvidence(storeRoot,protocol.ownership.receiptSuffix,protocol.ownership.evidenceSuffix,parseOwnershipReceiptV1);
 return"written";
}

/** 🔒️ Answers whether a lock-only run already proved this scope's published lock under identical inputs. */
export function javascriptDependencyLockConvergedV1(storeRoot:string,current:Pick<LockReceiptV1,"scope"|"generation"|"lockSha256"|"recipeSha256"|"runtimeSha256">):boolean{
 for(const name of names(storeRoot,keyOf("lock:"+current.scope)+".",protocol.lock.receiptSuffix))try{if(lockConvergedV1(parseLockReceiptV1(bounded(join(directory(storeRoot),name),protocol.lock.maximumReceiptBytes)),current))return true;}catch{}
 return false;
}
/** 🔏️ Seals that a lock-only run reproduced the published lock of one scope and retires its earlier receipts. */
export async function sealJavascriptDependencyLockConvergedV1(storeRoot:string,body:Omit<LockReceiptV1,"sha256">,checkpoint:()=>void,yieldContinuation:()=>Promise<void>):Promise<void>{
 const receipt=sealLockReceiptV1(body),prefix=keyOf("lock:"+body.scope)+".",name=prefix+receipt.sha256+protocol.lock.receiptSuffix;
 await writeJavascriptDependencyGenerationFilesV1(storeRoot,[{name,bytes:Buffer.from(JSON.stringify(receipt)+"\n")}],checkpoint,yieldContinuation);
 for(const other of names(storeRoot,prefix,protocol.lock.receiptSuffix))if(other!==name)unlinkQuietly(storeRoot,other);
}
/** 🧯️ Removes the converged lock receipts of scopes that no longer exist. */
export function retireJavascriptDependencyLockReceiptsV1(storeRoot:string,scopes:readonly string[]):void{
 const keep=new Set(scopes.map(scope=>keyOf("lock:"+scope)));
 for(const name of names(storeRoot,"",protocol.lock.receiptSuffix))if(!keep.has(name.slice(0,16)))unlinkQuietly(storeRoot,name);
}
