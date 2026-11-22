import {readRemoteCachePolicyV1,remoteCacheBaseUrlV1} from "../🟦️.ts";
import {fleetCacheRootV1,readLocalFleetCacheFactsV1,resolveAuthorisedRemoteCacheV1,resolveRemoteCacheV1,type AuthorisedRemoteCacheV1,type FleetCacheLocalCopyV1} from "../🛰️fleet/🟦️.ts";

const SIGNED="SEMIO_REMOTE_CACHE_SIGNED";

const probes=new Map<string,{promise:Promise<AuthorisedRemoteCacheV1>;at:number;verdict:boolean}>();

/** ⏳️ How long a healthy answer of the fleet facts (an endpoint that answered its health check at the last refresh) stands for a machine: within it a build routes at once and the probe runs behind it. */
export const NX_VERDICT_MILLISECONDS_V1=600000;
/** 🔁️ How often a cold probe is repeated when it times out or fails to connect, how long it waits before each repetition (times the attempt) and how long a negative answer holds before a later call asks again. */
export const NX_PROBE_RETRIES_V1=3,NX_PROBE_BACKOFF_MILLISECONDS_V1=150,NX_NEGATIVE_MILLISECONDS_V1=5000;

/** 🩺️ Whether the local copy of the fleet facts carries a healthy verdict that is still fresh: an endpoint was selected by a successful health check no longer than `NX_VERDICT_MILLISECONDS_V1` ago. */
export function nxCachedVerdictV1(copy:Pick<FleetCacheLocalCopyV1,"selectedEndpoint"|"checkedAtMs">|undefined,now:number=Date.now()):boolean{
 return copy!==undefined&&copy.selectedEndpoint!==null&&now-copy.checkedAtMs<NX_VERDICT_MILLISECONDS_V1&&now>=copy.checkedAtMs;
}

const SERVER="NX_SELF_HOSTED_REMOTE_CACHE_SERVER",TOKEN="NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN",SWITCH="SEMIO_REMOTE_CACHE_NX";

const pause=(milliseconds:number):Promise<void>=>new Promise(done=>setTimeout(done,milliseconds));

/** 🔁️ Asks for the authorised remote cache and repeats a probe that got no answer at all (a timeout or a refused connection, as a loaded cache host gives) up to `retries` times with a growing pause; an answer, even a refusal, ends it, so a wrong credential is never hammered. Never throws. */
async function probeWithRetriesV1(environment:Readonly<Record<string,string|undefined>>,root:string|undefined,options:Readonly<{probeMilliseconds?:number;retries?:number;backoffMilliseconds?:number;fetch?:typeof fetch}>):Promise<AuthorisedRemoteCacheV1>{
 const attempts=Math.max(1,options.retries??NX_PROBE_RETRIES_V1),backoff=options.backoffMilliseconds??NX_PROBE_BACKOFF_MILLISECONDS_V1;
 for(let attempt=1;;attempt++){
  const outcome=await resolveAuthorisedRemoteCacheV1(environment,{...(root===undefined?{}:{root}),probeMilliseconds:options.probeMilliseconds??1500,...(options.fetch===undefined?{}:{fetch:options.fetch})});
  if(outcome.authorised||outcome.resolution===undefined||outcome.status!==null||attempt>=attempts)return outcome;
  await pause(backoff*attempt);
 }
}

/** 🧶️ The environment that routes Nx task caching through the remote cache in front of the host store. The cache is resolved like every client resolves it (`resolveAuthorisedRemoteCacheV1`: the environment first, then the fleet facts of the machine, refreshed within a short timeout) and probed on Nx's own archive route within `probeMilliseconds` (a probe that got no answer is repeated up to three times; a negative answer holds five seconds, a positive one for the process). A healthy verdict of the fleet facts that is at most ten minutes old (`nxCachedVerdictV1`) routes at once and lets the probe run behind it, so a build never pays a cold probe on its critical path; a rotated fleet credential is picked up by one forced refresh. An answer other than 200 or 404 (unreachable, unauthorized, failing) keeps Nx local, removes the routing and prints one bilingual warning through `warn`, so no task ever fails or stalls on a dead endpoint. Without any contract the local server of the policy is used when it answers its health check; nothing (Nx stays local) when the switch `SEMIO_REMOTE_CACHE_NX=off` is set, when the caller configured Nx itself, or when no server answers. The token of an open loopback server is the placeholder. */
export async function nxRemoteCacheEnvironmentV1(environment:Readonly<Record<string,string|undefined>>,options:Readonly<{endpoint?:string;probeMilliseconds?:number;retries?:number;backoffMilliseconds?:number;warn?:(text:string)=>void;root?:string;fetch?:typeof fetch}>={}):Promise<Readonly<Record<string,string|undefined>>>{
 if(environment[SWITCH]==="off")return {};
 const policy=readRemoteCachePolicyV1(),root=options.root??fleetCacheRootV1(environment),before=resolveRemoteCacheV1(environment,root===undefined?{}:{root}),preset=environment[SERVER]===""?undefined:environment[SERVER];
 if(preset!==undefined&&(before===undefined||preset!==remoteCacheBaseUrlV1(before.contract)))return {};
 if(before!==undefined||root!==undefined){
  const key=`${root??""}\0${before===undefined?"":remoteCacheBaseUrlV1(before.contract)}`,now=Date.now(),known=probes.get(key);
  const route=(resolution:NonNullable<AuthorisedRemoteCacheV1["resolution"]>):Readonly<Record<string,string|undefined>>=>{
   const base=remoteCacheBaseUrlV1(resolution.contract),token=resolution.token;
   return preset===base&&environment[TOKEN]===token&&resolution.tokenFrom!=="fleet-identity"?{}:{[SERVER]:base,...(token===undefined?{}:{[TOKEN]:token}),...(resolution.tokenFrom==="fleet-identity"?{[SIGNED]:"1"}:{})};
  };
  const complain=async(outcome:AuthorisedRemoteCacheV1):Promise<void>=>{
   if(outcome.resolution===undefined||outcome.authorised)return;
   const {remoteCacheLanguagesV1,remoteCacheTextV1}=await import("../⌨️command/🟦️.ts"),text=remoteCacheLanguagesV1(undefined,environment).map(language=>remoteCacheTextV1(language,"warning.unreachable",{endpoint:outcome.resolution!.contract.endpoint})).join("\n");
   (options.warn??(line=>{process.stderr.write(line+"\n");}))(text);
  };
  if(known!==undefined&&known.verdict&&before!==undefined)return route(before);
  let entry=known!==undefined&&now-known.at<NX_NEGATIVE_MILLISECONDS_V1?known:undefined;
  if(entry===undefined){
   const cached=before!==undefined&&nxCachedVerdictV1(root===undefined?undefined:readLocalFleetCacheFactsV1(root),now),promise=probeWithRetriesV1(environment,root,options);
   entry={promise,at:now,verdict:cached};probes.set(key,entry);
   if(cached){void promise.then(complain,()=>undefined);return route(before!);}
   await complain(await promise);
  }
  const outcome=await entry.promise;
  if(outcome.resolution!==undefined)return outcome.authorised?route(outcome.resolution):{[SERVER]:undefined,[TOKEN]:undefined};
 }
 if(preset!==undefined)return {};
 const origin=options.endpoint??`http://${policy.server.host}:${policy.server.port}`,base=`${origin}/${policy.defaults.instance}`,names=policy.server.tokenEnvironment;
 try{const answer=await fetch(`${origin}/healthz`,{signal:AbortSignal.timeout(options.probeMilliseconds??300)});if(!answer.ok)return {};}catch{return {};}
 const token=environment[names.write]??environment[names.read]??"local";
 return Object.freeze({[SERVER]:base,[TOKEN]:token});
}
