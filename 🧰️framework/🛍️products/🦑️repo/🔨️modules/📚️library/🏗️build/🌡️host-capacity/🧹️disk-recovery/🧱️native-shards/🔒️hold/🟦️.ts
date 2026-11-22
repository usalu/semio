import {join} from "node:path";
import {NATIVE_SHARDS_POLICY_V1,nativeShardNamespaceV1} from "../🟦️.ts";

/** 🔒️ Takes the exclusive lease of a shard's namespace without waiting, so a shard that a live Nx run holds is never touched; a lease that cannot be taken at once, for whatever reason, answers `undefined`. */
export function nativeShardHoldV1(workspace:string):(id:string)=>Promise<(()=>void)|undefined>{
 return async id=>{
  try{
   const{acquireResourceLease}=await import("../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts"),lease=await acquireResourceLease({directory:join(workspace,...NATIVE_SHARDS_POLICY_V1.leaseDirectory.split("/")),resource:nativeShardNamespaceV1(workspace,id).resource,mode:"exclusive",signal:new AbortController().signal,timeoutMs:0});
   return()=>{void Promise.resolve(lease.release()).catch(()=>undefined);};
  }catch{return undefined;}
 };
}
