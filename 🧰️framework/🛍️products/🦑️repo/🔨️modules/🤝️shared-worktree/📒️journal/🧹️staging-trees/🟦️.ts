import {randomBytes} from "node:crypto";
import {mkdirSync} from "node:fs";
import {lstat,readdir,rm} from "node:fs/promises";
import {join} from "node:path";
import {ownProcessIdentityV1,processHolderAliveV1,type ProcessIdentityV1} from "../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";
import {storePathsV1} from "../🟦️.ts";
import document from "./🔣️.json" with {type:"json"};

/** 📐️ The declared grace and walk bound of the staging sweep.
 * @see ./🔣️.json */
export const STAGING_TREES_POLICY_V1:Readonly<{version:1;graceMilliseconds:number;fileGraceMilliseconds:number;maximumEntries:number}>=Object.freeze({...document.policy,version:1 as const});

/** 🪪️ The owner a staging directory name registers: the machine's origin, a label, and the shared process identity. */
export type StagingTreeOwnerV1=Readonly<{originId:string;label:string}&ProcessIdentityV1>;

const namePattern=/^([a-z0-9][a-z0-9-]{0,39})\.([a-z0-9-]{1,32})\.([1-9][0-9]{0,9})\.([1-9][0-9]{0,15})\.[0-9a-f]{16}$/u;

/** 🔬️ Reads the owner out of a staging directory name; `undefined` for a name that registers none. */
export function parseStagingTreeNameV1(name:string):StagingTreeOwnerV1|undefined{
 const found=namePattern.exec(name);
 return found===null?undefined:{originId:found[1]!,label:found[2]!,pid:Number(found[3]),startMs:Number(found[4])};
}

/** 🆕️ Creates a staging directory below `<store>/v1/tmp` that registers this process as its owner (`<originId>.<label>.<pid>.<startMs>.<hex16>`), so the sweep can prove whether the owner still lives; the caller removes it.
 * @see ./🔣️.json */
export function createStagingTreeV1(storeRoot:string,originId:string,label:string):string{
 const identity=ownProcessIdentityV1(),name=`${originId}.${label}.${identity.pid}.${identity.startMs}.${randomBytes(8).toString("hex")}`;
 if(parseStagingTreeNameV1(name)===undefined)throw Error("A staging directory needs a lowercase origin id and a lowercase label of at most 32 characters / Ein Staging-Verzeichnis braucht eine kleingeschriebene Origin-Kennung und ein kleingeschriebenes Label von höchstens 32 Zeichen");
 const path=join(storePathsV1(storeRoot).tmp,name);
 mkdirSync(path,{recursive:true});
 return path;
}

type TreeReadV1=Readonly<{newestMs:number;bytes:number;complete:boolean}>;
async function walkTreeV1(root:string,limit:number,signal?:AbortSignal):Promise<TreeReadV1>{
 let newestMs=0,bytes=0,seen=0;
 const stack=[root];
 while(stack.length>0){
  signal?.throwIfAborted();
  const current=stack.pop()!,info=await lstat(current).catch(()=>undefined);
  if(info===undefined)continue;
  newestMs=Math.max(newestMs,info.mtimeMs);bytes+=info.blocks*512;
  if(++seen>limit)return{newestMs,bytes,complete:false};
  if(info.isDirectory()&&!info.isSymbolicLink())for(const name of await readdir(current).catch(()=>[] as string[]))stack.push(join(current,name));
 }
 return{newestMs,bytes,complete:true};
}

/** 📊️ What one sweep of the staging directories did: the directories and bytes it removed (or would remove in a dry run) and what it kept, by reason. */
export type StagingTreesOutcomeV1=Readonly<{trees:number;bytes:number;kept:Readonly<{live:number;young:number;incomplete:number}>}>;

/** 🧹️ Retires the staging directories of `<store>/v1/tmp` that no live owner holds and nothing wrote to for the grace; a directory whose registered owner is this machine's and is proven alive by `processHolderAliveV1` (pid and start time) is never touched. Loose files stay for `reclaimStagingV1`. A store without `tmp` yields nothing and is never created; the signal cancels between directories.
 * @see ./🔣️.json */
export async function reclaimStagingTreesV1(storeRoot:string,options:Readonly<{originId?:string;now?:number;graceMilliseconds?:number;maximumEntries?:number;alive?:(holder:ProcessIdentityV1)=>boolean;dryRun?:boolean;signal?:AbortSignal}>={}):Promise<StagingTreesOutcomeV1>{
 const {tmp}=storePathsV1(storeRoot),now=options.now??Date.now(),grace=options.graceMilliseconds??STAGING_TREES_POLICY_V1.graceMilliseconds,limit=options.maximumEntries??STAGING_TREES_POLICY_V1.maximumEntries,alive=options.alive??processHolderAliveV1;
 let trees=0,bytes=0,live=0,young=0,incomplete=0;
 for(const name of await readdir(tmp).catch(()=>[] as string[])){
  options.signal?.throwIfAborted();
  if(name.startsWith(".nfs"))continue;
  const path=join(tmp,name),info=await lstat(path).catch(()=>undefined);
  if(info===undefined||!info.isDirectory()||info.isSymbolicLink())continue;
  const owner=parseStagingTreeNameV1(name);
  if(owner!==undefined&&options.originId!==undefined&&owner.originId===options.originId&&alive({pid:owner.pid,startMs:owner.startMs})){live++;continue;}
  const read=await walkTreeV1(path,limit,options.signal);
  if(!read.complete){incomplete++;continue;}
  if(now-read.newestMs<=grace){young++;continue;}
  if(options.dryRun!==true&&!await rm(path,{recursive:true,force:true,maxRetries:3,retryDelay:50}).then(()=>true,()=>false))continue;
  trees++;bytes+=read.bytes;
 }
 return{trees,bytes,kept:{live,young,incomplete}};
}
