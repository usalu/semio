import {readRemoteCachePolicyV1,remoteCacheBaseUrlV1} from "../🟦️.ts";
import {fleetCacheRootV1,resolveAuthorisedRemoteCacheV1,resolveRemoteCacheV1,type AuthorisedRemoteCacheV1} from "../🛰️fleet/🟦️.ts";

const SIGNED="SEMIO_REMOTE_CACHE_SIGNED";

const probes=new Map<string,Promise<AuthorisedRemoteCacheV1>>();

const SERVER="NX_SELF_HOSTED_REMOTE_CACHE_SERVER",TOKEN="NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN",SWITCH="SEMIO_REMOTE_CACHE_NX";

/** 🧶️ The environment that routes Nx task caching through the remote cache in front of the host store. The cache is resolved like every client resolves it (`resolveAuthorisedRemoteCacheV1`: the environment first, then the fleet facts of the machine, refreshed within a short timeout) and probed once per process and endpoint on Nx's own archive route within `probeMilliseconds`; a rotated fleet credential is picked up by one forced refresh. An answer other than 200 or 404 (unreachable, unauthorized, failing) keeps Nx local, removes the routing and prints one bilingual warning through `warn`, so no task ever fails or stalls on a dead endpoint. Without any contract the local server of the policy is used when it answers its health check; nothing (Nx stays local) when the switch `SEMIO_REMOTE_CACHE_NX=off` is set, when the caller configured Nx itself, or when no server answers. The token of an open loopback server is the placeholder. */
export async function nxRemoteCacheEnvironmentV1(environment:Readonly<Record<string,string|undefined>>,options:Readonly<{endpoint?:string;probeMilliseconds?:number;warn?:(text:string)=>void;root?:string;fetch?:typeof fetch}>={}):Promise<Readonly<Record<string,string|undefined>>>{
 if(environment[SWITCH]==="off")return {};
 const policy=readRemoteCachePolicyV1(),root=options.root??fleetCacheRootV1(environment),before=resolveRemoteCacheV1(environment,root===undefined?{}:{root}),preset=environment[SERVER]===""?undefined:environment[SERVER];
 if(preset!==undefined&&(before===undefined||preset!==remoteCacheBaseUrlV1(before.contract)))return {};
 if(before!==undefined||root!==undefined){
  const key=`${root??""}\0${before===undefined?"":remoteCacheBaseUrlV1(before.contract)}`;
  let found=probes.get(key);
  if(found===undefined){
   found=resolveAuthorisedRemoteCacheV1(environment,{...(root===undefined?{}:{root}),probeMilliseconds:options.probeMilliseconds??1500,...(options.fetch===undefined?{}:{fetch:options.fetch})});probes.set(key,found);
   const outcome=await found;
   if(outcome.resolution!==undefined&&!outcome.authorised){
    const {remoteCacheLanguagesV1,remoteCacheTextV1}=await import("../⌨️command/🟦️.ts"),text=remoteCacheLanguagesV1(undefined,environment).map(language=>remoteCacheTextV1(language,"warning.unreachable",{endpoint:outcome.resolution!.contract.endpoint})).join("\n");
    (options.warn??(line=>{process.stderr.write(line+"\n");}))(text);
   }
  }
  const outcome=await found;
  if(outcome.resolution!==undefined){
   if(!outcome.authorised)return {[SERVER]:undefined,[TOKEN]:undefined};
   const base=remoteCacheBaseUrlV1(outcome.resolution.contract),token=outcome.resolution.token;
   return preset===base&&environment[TOKEN]===token&&outcome.resolution.tokenFrom!=="fleet-identity"?{}:{[SERVER]:base,...(token===undefined?{}:{[TOKEN]:token}),...(outcome.resolution.tokenFrom==="fleet-identity"?{[SIGNED]:"1"}:{})};
  }
 }
 if(preset!==undefined)return {};
 const origin=options.endpoint??`http://${policy.server.host}:${policy.server.port}`,base=`${origin}/${policy.defaults.instance}`,names=policy.server.tokenEnvironment;
 try{const answer=await fetch(`${origin}/healthz`,{signal:AbortSignal.timeout(options.probeMilliseconds??300)});if(!answer.ok)return {};}catch{return {};}
 const token=environment[names.write]??environment[names.read]??"local";
 return Object.freeze({[SERVER]:base,[TOKEN]:token});
}
