import {readRemoteCachePolicyV1,resolveRemoteCacheContractV1} from "../🟦️.ts";

const SERVER="NX_SELF_HOSTED_REMOTE_CACHE_SERVER",TOKEN="NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN",SWITCH="SEMIO_REMOTE_CACHE_NX";

/** 🧶️ The environment that routes Nx task caching through the remote cache in front of the host store: nothing when a contract is declared (the declared route renders it), else the local server of the policy when it answers its health check within `probeMilliseconds`; nothing (Nx stays local) when the switch `SEMIO_REMOTE_CACHE_NX=off` is set, when Nx is already configured, or when no server answers. The token comes from the server token variables, an open loopback server accepts the placeholder. */
export async function nxRemoteCacheEnvironmentV1(environment:Readonly<Record<string,string|undefined>>,options:Readonly<{endpoint?:string;probeMilliseconds?:number}>={}):Promise<Readonly<Record<string,string>>>{
 if(environment[SWITCH]==="off"||environment[SERVER]!==undefined&&environment[SERVER]!=="")return {};
 const policy=readRemoteCachePolicyV1(),names=policy.server.tokenEnvironment;
 if(resolveRemoteCacheContractV1(environment,policy)!==undefined)return {};
 const origin=options.endpoint??`http://${policy.server.host}:${policy.server.port}`,base=`${origin}/${policy.defaults.instance}`;
 try{const answer=await fetch(`${origin}/healthz`,{signal:AbortSignal.timeout(options.probeMilliseconds??300)});if(!answer.ok)return {};}catch{return {};}
 const token=environment[names.write]??environment[names.read]??"local";
 return Object.freeze({[SERVER]:base,[TOKEN]:token});
}
