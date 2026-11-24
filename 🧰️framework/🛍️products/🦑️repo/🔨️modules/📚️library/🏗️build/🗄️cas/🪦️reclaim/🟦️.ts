import {lstat,unlink} from "node:fs/promises";
import {openCasShardViewV1,type CasShardViewV1} from "../📒️ledger/🟦️.ts";
import {readCasPinsV1} from "../📌️pins/🟦️.ts";
import {casKeeperCountSyncV1,dropCasKeepersSyncV1,readCasHeldCachedV1} from "../🫂️holders/🟦️.ts";
import type {CasStoreV1} from "../🗃️store/🟦️.ts";

/** 🪦️ What a reclaim did: objects removed with their bytes, and why the others stayed (pinned by a live holder, kept by a live entry of a registered reference holder, accessed within the minimum age, hard-linked into a workspace, already gone). */
export type CasReclaimV1=Readonly<{entries:number;bytes:number;pinned:number;held:number;recent:number;shared:number;missing:number}>;

/** 🪦️ Removes the named objects from the host store when nobody needs them any more: an owner that knows its entries no longer reference a blob offers the digest here, and the store applies its own protections (a live pin, an access within the minimum age, a hard link into a workspace) just before every removal, so the shared store stays correct for its other users; a reader that loses the race sees a miss, never a partial object. `dryRun` only counts. */
export async function reclaimCasObjectsV1(store:Pick<CasStoreV1,"root"|"policy"|"objectPath">,hashes:Iterable<string>,options:Readonly<{now?():number;dryRun?:boolean;signal?:AbortSignal;held?:ReadonlyMap<string,string>}>={}):Promise<CasReclaimV1>{
 const now=options.now??Date.now,fan=store.policy.fanOutHexLength,views=new Map<string,CasShardViewV1>(),seen=new Set<string>();
 let pins=await readCasPinsV1(store.root,now()),entries=0,bytes=0,pinned=0,kept=0,recent=0,shared=0,missing=0,checked=0;
 const held=options.held??(await readCasHeldCachedV1(store.root,store.policy,now())).digests;
 for(const hash of hashes){
  if(seen.has(hash)||!/^[0-9a-f]{64}$/u.test(hash))continue;seen.add(hash);options.signal?.throwIfAborted();
  const path=store.objectPath(hash);let info;try{info=await lstat(path);}catch{missing++;continue;}
  if(!info.isFile()){missing++;continue;}
  if(!held.has(hash)&&info.nlink-casKeeperCountSyncV1(store.root,store.policy,hash)>1){shared++;continue;}
  if(++checked%64===0)pins=await readCasPinsV1(store.root,now());
  if(pins.has(hash)){pinned++;continue;}
  if(held.has(hash)){kept++;continue;}
  const shard=hash.slice(0,fan);let view=views.get(shard);if(!view){view=await openCasShardViewV1(store.root,shard,{nowMs:now(),quietMs:store.policy.ledgerQuietMs});views.set(shard,view);}
  await view.refresh();
  if(now()-Math.max(info.mtimeMs,view.latest(hash))<store.policy.minAgeMs){recent++;continue;}
  if(options.dryRun!==true){
   pins=await readCasPinsV1(store.root,now());
   if(pins.has(hash)){pinned++;continue;}
   dropCasKeepersSyncV1(store.root,store.policy,hash);
   try{await unlink(path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT"){missing++;continue;}throw error;}
  }
  entries++;bytes+=info.size;
 }
 return Object.freeze({entries,bytes,pinned,held:kept,recent,shared,missing});
}
