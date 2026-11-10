import {join} from "node:path";
import {createCasStoreV1,casRootV1} from "../../🗄️cas/🗃️store/🟦️.ts";
import {readRemoteCachePolicyV1} from "../🟦️.ts";
import {RemoteCacheDirectoryBusyV1,startRemoteCacheServerV1} from "../🖥️server/🟦️.ts";
import type {RemoteCacheValidationV1} from "../🔬️action-result/🟦️.ts";

/** 🏠️ What the per-host graph service needs to carry the remote cache endpoint: the environment (tokens, store root), the service's own cancellation and a sink for refusals. */
export type HostedRemoteCacheOptionsV1=Readonly<{environment:Readonly<Record<string,string|undefined>>;signal:AbortSignal;port?:number;onUnavailable?(reason:"busy"|"failed",detail:string):void}>;
/** 🏠️ The running endpoint inside the service. */
export type HostedRemoteCacheV1=Readonly<{url:string;stop():Promise<void>}>;

/** 🏠️ Starts the remote cache endpoint (protocol of `repo remote-cache serve`, blobs and Nx archives in the host store) as part of the per-host graph service: it lives and dies with the service, shares its process, memory bound and cancellation, and binds the policy's loopback address and port. Another holder of the port or of the cache directory means someone already serves it, which is reported as `busy` and is no error; nothing here ever throws into the service. The directory lies in the host store root, so every workspace of the user shares one cache. */
export async function startHostedRemoteCacheV1(options:HostedRemoteCacheOptionsV1):Promise<HostedRemoteCacheV1|undefined>{
 try{
  const all=readRemoteCachePolicyV1(),policy=all.server,names=policy.tokenEnvironment,root=casRootV1(options.environment),value=(name:string):string|undefined=>{const text=options.environment[name];return text===undefined||text===""?undefined:text;};
  const server=await startRemoteCacheServerV1({
   hostStore:createCasStoreV1({root}),directory:join(root,"remote-cache"),host:policy.host,port:options.port??policy.port,
   maximumBytes:policy.maximumBytes,maximumEntryBytes:Math.min(policy.maximumEntryBytes,Math.floor(policy.maximumBytes/2)),maximumAgeDays:policy.maximumAgeDays,
   defaultInstance:all.defaults.instance,tokens:{read:value(names.read),write:value(names.write)},readOnly:policy.readOnly,
   validation:policy.actionResults as RemoteCacheValidationV1,maximumActionResultBytes:policy.maximumActionResultBytes,maintenanceSeconds:policy.maintenanceSeconds,signal:options.signal
  });
  return Object.freeze({url:server.url,stop:()=>server.stop()});
 }catch(error){
  options.onUnavailable?.(error instanceof RemoteCacheDirectoryBusyV1||(error as NodeJS.ErrnoException).code==="EADDRINUSE"?"busy":"failed",String((error as Error).message??error));
  return undefined;
 }
}
