import type {RemoteCacheToolV1} from "../../../../🏗️build/🌐️remote-cache/⌨️command/🟦️.ts";
import {readRemoteCacheTokenV1,remoteCacheBaseUrlV1,remoteCacheEffectiveUploadV1,REMOTE_CACHE_REDACTED_V1,type RemoteCacheContractV1,type RemoteCacheRenderingV1} from "../../../../🏗️build/🌐️remote-cache/🟦️.ts";

/** 🏷️ Names the two variables through which Nx 23 reads its self-hosted remote cache. */
export const NX_REMOTE_CACHE_VARIABLES_V1=Object.freeze({server:"NX_SELF_HOSTED_REMOTE_CACHE_SERVER",token:"NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN"} as const);

/** 🧩️ The environment entries Nx needs, where `undefined` removes an inherited entry so a disabled contract can never leave a stale cache behind. */
export type NxRemoteCacheV1=Readonly<{environment:Readonly<Record<string,string|undefined>>;rendering:RemoteCacheRenderingV1}>;

/** 🎛️ Renders the neutral contract into Nx's self-hosted remote cache: the server is the origin plus the namespace, the bearer token comes from the contract's token variable. Nx has no switch for compression, download mode, timeouts, retries or withholding uploads, so those are reported as unsupported and read-only is left to the server's token scope. */
export function nxRemoteCacheV1(contract:RemoteCacheContractV1|undefined,environment:Readonly<Record<string,string|undefined>>):NxRemoteCacheV1{
 if(contract===undefined)return Object.freeze({environment:Object.freeze({[NX_REMOTE_CACHE_VARIABLES_V1.server]:undefined,[NX_REMOTE_CACHE_VARIABLES_V1.token]:undefined}),rendering:Object.freeze({mechanism:"nx-environment",applied:Object.freeze([]),unsupported:Object.freeze([]),uploadEnforcedBy:"none"})});
 const upload=remoteCacheEffectiveUploadV1(contract),token=readRemoteCacheTokenV1(contract,environment),server=remoteCacheBaseUrlV1(contract);
 const rendering:RemoteCacheRenderingV1=Object.freeze({mechanism:"nx-environment",applied:Object.freeze([`${NX_REMOTE_CACHE_VARIABLES_V1.server}=${server}`,...(token===undefined?[]:[`${NX_REMOTE_CACHE_VARIABLES_V1.token}=${REMOTE_CACHE_REDACTED_V1}`])]),unsupported:Object.freeze(["compression","download","timeoutSeconds","retries"] as const),uploadEnforcedBy:upload?"none":"server"});
 return Object.freeze({environment:Object.freeze({[NX_REMOTE_CACHE_VARIABLES_V1.server]:server,[NX_REMOTE_CACHE_VARIABLES_V1.token]:token}),rendering});
}

/** 🔌️ Nx's side of `remote-cache status`: it speaks the self-hosted cache API and proves reachability on the task archive route. */
export function nxRemoteCacheToolV1():RemoteCacheToolV1{
 return Object.freeze<RemoteCacheToolV1>({tool:"nx",protocol:"nx-self-hosted",probePath:"/v1/cache/semio-probe",render:(contract,environment)=>nxRemoteCacheV1(contract,environment).rendering});
}
