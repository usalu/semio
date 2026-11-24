import {decodeProtobufV1,REAPI_V1} from "../../📡️wire/📦️protobuf/🟦️.ts";
import type {RemoteExecutionContentV1} from "../../🗄️content/🟦️.ts";
import {remoteExecutionDigestV1,type RemoteExecutionDigestV1} from "../../🟦️.ts";

type WireDigestV1=Readonly<{hash:string;size_bytes:number}>;
const unwired=(digest:WireDigestV1):RemoteExecutionDigestV1=>remoteExecutionDigestV1(digest.hash,digest.size_bytes);

/** 🔐️ The lease name under which the scheduler holds the inputs of one operation (`input:op-<action>`): distinct from the lease of the requester that uploaded them (`input:<action>`), so the requester ending its own lease never ends the operation's. */
export const operationInputLeaseV1=(action:RemoteExecutionDigestV1):`input:${string}`=>`input:op-${action.hash.slice(0,32)}`;

/** 🌳️ Every blob an action needs: the action, its command, the input directories and every file they name, found by reading the directory blobs level by level (at most 16 at a time). A directory blob the store cannot read is `unreadable` (its children are unknown). */
export async function collectOperationInputsV1(content:RemoteExecutionContentV1,instance:string,action:RemoteExecutionDigestV1,actionBytes:Uint8Array,signal?:AbortSignal):Promise<Readonly<{digests:readonly RemoteExecutionDigestV1[];unreadable:readonly RemoteExecutionDigestV1[]}>>{
 const found=new Map<string,RemoteExecutionDigestV1>(),unreadable:RemoteExecutionDigestV1[]=[],seen=new Set<string>(),add=(digest:RemoteExecutionDigestV1):void=>{if(digest.size>0)found.set(digest.hash,digest);};
 const message=decodeProtobufV1(REAPI_V1.action,actionBytes);
 add(action);
 if(message.command_digest)add(unwired(message.command_digest));
 let level:RemoteExecutionDigestV1[]=message.input_root_digest?[unwired(message.input_root_digest)]:[];
 while(level.length>0){
  const next:RemoteExecutionDigestV1[]=[];let cursor=0;
  await Promise.all(Array.from({length:Math.min(16,level.length)},async()=>{
   for(;;){
    const digest=level[cursor++];if(digest===undefined)return;
    if(seen.has(digest.hash))continue;seen.add(digest.hash);add(digest);
    if(digest.size===0)continue;
    const bytes=await content.read(instance,digest,signal);
    if(!bytes){unreadable.push(digest);continue;}
    const node=decodeProtobufV1(REAPI_V1.directory,bytes);
    for(const file of node.files as {digest:WireDigestV1}[])add(unwired(file.digest));
    for(const child of node.directories as {digest:WireDigestV1}[])next.push(unwired(child.digest));
   }
  }));
  level=next;
 }
 return{digests:[...found.values()],unreadable};
}

type HeldV1={instance:string;lease:`input:${string}`;digests:readonly RemoteExecutionDigestV1[];at:number};

/** 🔐️ The leases a scheduler holds on the input roots of its live operations: taken when an operation is accepted (one existence check per blob under the operation's own lease, which the cache server answers only after the lease is written, so a blob that was already present is leased as well as one that was just uploaded), renewed every `renewMilliseconds` while the operation exists (queued, executing, however long) and ended when it completes (the blobs then run only the store's short grace longer). The store's own collector, sweep and reclaim never remove a leased blob.
 * @see ../🧪️tests/🟦️.ts */
export class OperationInputLeasesV1{
 private readonly held=new Map<string,HeldV1>();
 private readonly content:RemoteExecutionContentV1;private readonly renewMilliseconds:number;private readonly clock:()=>number;
 constructor(content:RemoteExecutionContentV1,renewMilliseconds:number,clock:()=>number){this.content=content;this.renewMilliseconds=renewMilliseconds;this.clock=clock;}

 /** 📥️ Leases the whole input root of an operation and answers the blobs the store lacks (every one, not the first): the operation is then known to the lease table whatever the answer, and its owner ends it with {@link release}. */
 async hold(id:string,instance:string,action:RemoteExecutionDigestV1,actionBytes:Uint8Array,signal?:AbortSignal):Promise<readonly RemoteExecutionDigestV1[]>{
  const {digests,unreadable}=await collectOperationInputsV1(this.content,instance,action,actionBytes,signal),lease=operationInputLeaseV1(action);
  const absent=digests.length===0?[]:await this.content.missing(instance,digests,signal,lease);
  this.held.set(id,{instance,lease,digests,at:this.clock()});
  const all=new Map<string,RemoteExecutionDigestV1>();
  for(const digest of [...unreadable,...absent])all.set(digest.hash,digest);
  return [...all.values()];
 }

 /** ♻️ Takes the lease again on every blob of an operation and answers the blobs the store lost although they were leased. */
 async renew(id:string):Promise<readonly RemoteExecutionDigestV1[]>{
  const held=this.held.get(id);if(held===undefined)return [];
  const absent=held.digests.length===0?[]:await this.content.missing(held.instance,held.digests,undefined,held.lease);
  const now=this.held.get(id);if(now!==undefined)now.at=this.clock();
  return absent;
 }

 /** ⏰️ The operations whose lease is older than the renewal period, oldest first. */
 due(now:number):readonly string[]{return [...this.held].filter(([,row])=>now-row.at>=this.renewMilliseconds).sort((left,right)=>left[1].at-right[1].at).map(([id])=>id);}

 /** 🧮️ Whether a lease is held for the operation. */
 has(id:string):boolean{return this.held.has(id);}

 /** 🔓️ Ends the lease of an operation: its blobs run the store's short grace longer (a retry leases again). Never throws. */
 release(id:string):void{
  const held=this.held.get(id);if(held===undefined)return;
  this.held.delete(id);
  if(this.content.release===undefined||held.digests.length===0)return;
  void this.content.release(held.instance,held.digests.map(digest=>digest.hash),held.lease).catch(()=>undefined);
 }
}
