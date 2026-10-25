import {cacheTextV1,formatCacheBytesV1,type CacheLanguageV1,type CacheLedgerV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";

/** 🔌️ What the reclaim needs from the repository: the single-flight lease, the retained graph partition owner, the optional member indexing of its groups, its free space and the two shared recoveries; tests replace them. */
export type GraphReclaimPortsV1=Readonly<{
 exclusive(resource:string):Promise<Readonly<{release():void}>|undefined>;
 open():Promise<Readonly<{close():Promise<void>}>>;
 index?(image:object):Promise<Readonly<{groups:number;indexed:number;busy:number;refused:number;compacted:number;cancelled:boolean}>>;
 supersede?(image:object,dryRun:boolean):Promise<Readonly<{groups:number;busy:number;skipped:number;headers:number;removed:number;bytes:number;cancelled:boolean}>>;
 closures?(image:object,dryRun:boolean):Promise<Readonly<{candidates:number;removed:number;bytes:number;busy:number;unvouched:number;cancelled:boolean}>>;
 free(image:object):Promise<number>;
 selected(image:object,request:Readonly<{minimumAvailableBytes:number;wait:false}>):Promise<unknown>;
 optional(image:object,request:Readonly<{minimumAvailableBytes:number}>):Promise<unknown>;
}>;

/** 🚰️ Reclaims optional graph caches up to the ledger's target free space, in the background and under the shared hold of the retained graph image: first it removes the observation blocks no record names any more (the inventory of the graph image is finite and every re-inference supersedes blocks), then it indexes the groups no member index vouches for yet (leaves are read without a lease, each group's non-blocking writer lease is held only while rows are appended, because a group is retired by its index and an lstat of each member), then reclaims scope by scope, never the exclusive maintenance of the image unless that is not enough. Returns the localized report lines. */
export async function reclaimGraphOptionalV1(workspace:string,ledger:CacheLedgerV1,dryRun:boolean,language:CacheLanguageV1,ports:GraphReclaimPortsV1):Promise<string>{
 const text=(key:string,variables:Readonly<Record<string,string|number>>={})=>cacheTextV1(language,key,variables),reclaim=ledger.areas.find(row=>row.id==="graph-inference")?.reclaim;
 if(!reclaim)return text("gc.graph.undeclared");
 const lease=await ports.exclusive("repository-graph-optional-collector:"+workspace);
 if(!lease)return text("gc.graph.running");
 let image:Awaited<ReturnType<GraphReclaimPortsV1["open"]>>|undefined;
 try{
  image=await ports.open();
  const lines:string[]=[],finish=(line:string)=>[...lines,line].join("\n");
  if(ports.closures){const report=await ports.closures(image,dryRun);if(report.unvouched)lines.push(text("gc.graph.closures.unvouched"));else if(report.candidates)lines.push(text(dryRun?"gc.graph.closures.plan":"gc.graph.closures.done",{removed:report.removed,candidates:report.candidates,megabytes:Math.round(report.bytes/1048576),busy:report.busy}));if(report.cancelled)return finish(text("error.cancelled"));}
  if(ports.supersede){const report=await ports.supersede(image,dryRun);if(report.removed||report.busy||report.skipped)lines.push(text(dryRun?"gc.graph.supersede.plan":"gc.graph.superseded",{removed:report.removed,megabytes:Math.round(report.bytes/1048576),groups:report.groups,kept:report.skipped,busy:report.busy}));if(report.cancelled)return finish(text("error.cancelled"));}
  if(!dryRun&&ports.index){const report=await ports.index(image);if(report.indexed||report.refused||report.compacted||report.busy)lines.push(text("gc.graph.indexed",{indexed:report.indexed,groups:report.groups,busy:report.busy,refused:report.refused,compacted:report.compacted}));if(report.cancelled)return finish(text("error.cancelled"));}
  const before=await ports.free(image);
  if(before>=reclaim.targetAvailableBytes)return finish(text("gc.graph.enough",{free:formatCacheBytesV1(before),target:formatCacheBytesV1(reclaim.targetAvailableBytes)}));
  if(dryRun)return finish(text("gc.graph.plan",{free:formatCacheBytesV1(before),target:formatCacheBytesV1(reclaim.targetAvailableBytes)}));
  await ports.selected(image,{minimumAvailableBytes:reclaim.targetAvailableBytes,wait:false});
  if(await ports.free(image)<reclaim.targetAvailableBytes)await ports.optional(image,{minimumAvailableBytes:reclaim.targetAvailableBytes});
  const after=await ports.free(image);
  return finish(text("gc.graph.done",{freed:formatCacheBytesV1(Math.max(0,after-before)),free:formatCacheBytesV1(after),target:formatCacheBytesV1(reclaim.targetAvailableBytes)}));
 }finally{
  try{await image?.close();}finally{lease.release();}
 }
}
