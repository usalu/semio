import type {RemoteCacheToolV1} from "../../../🌐️remote-cache/⌨️command/🟦️.ts";
import {fleetCacheRootV1,remoteCacheFetchV1,resolveRemoteCacheV1} from "../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {acquireRemoteCacheSigningProxyV1} from "../../../🌐️remote-cache/🔏️signing-proxy/🟦️.ts";
import {readRemoteCacheTokenV1,remoteCacheBaseUrlV1,remoteCacheEffectiveUploadV1,requireRemoteCacheShapeV1,type RemoteCacheContractV1,type RemoteCacheRenderingV1} from "../../../🌐️remote-cache/🟦️.ts";

/** 📜️ The tiny program Bazel runs to ask for request headers; it holds the name of the token variable and never the token. */
export type BazelCredentialHelperV1=Readonly<{path:string;content:string;mode:number}>;

/** 🧩️ What Bazel needs to talk to the remote cache: command flags, the one environment entry that carries the token to the helper, and the helper itself. */
export type BazelRemoteCacheV1=Readonly<{flags:readonly string[];environment:Readonly<Record<string,string>>;helper?:BazelCredentialHelperV1;rendering:RemoteCacheRenderingV1}>;

/** 🖋️ Writes the credential helper of a platform: it answers Bazel's `get` with a bearer header read from the token variable. */
export function bazelCredentialHelperV1(directory:string,tokenEnvironment:string,platform:NodeJS.Platform=process.platform):BazelCredentialHelperV1{
 requireRemoteCacheShapeV1("EnvironmentName",tokenEnvironment,"Closed remote cache token variable required");
 const separator=platform==="win32"?"\\":"/",path=`${directory.replace(/[\\/]+$/u,"")}${separator}remote-cache-credentials.${platform==="win32"?"cmd":"sh"}`;
 if(platform==="win32")return Object.freeze({path,mode:0o644,content:`@echo off\r\nmore >nul\r\necho {"headers":{"Authorization":["Bearer %${tokenEnvironment}%"]}}\r\n`});
 return Object.freeze({path,mode:0o755,content:`#!/bin/sh\ncat >/dev/null\nprintf '{"headers":{"Authorization":["Bearer %s"]}}\\n' "$${tokenEnvironment}"\n`});
}

/** 🎛️ Renders the neutral contract into Bazel: `--remote_cache`, `--remote_instance_name`, `--[no]remote_upload_local_results`, `--[no]remote_cache_compression`, `--remote_download_outputs`, `--remote_timeout`, `--remote_retries`, and the token through a credential helper scoped to the cache host, because `--remote_header` copies the token into Bazel's java.log and build event stream on disk. A machine that is authorised by its signing identity has no token Bazel could send and Bazel cannot sign (its credential helper is given no HTTP method), so Bazel talks to the loopback signing proxy (`proxy`, see `startBazelRemoteCacheProxyV1`) with the same flags and no credentials; without a running proxy it renders no remote flags and Bazel builds locally. The contract has no `minimal` download, since the task adapter restores the snapshots of the requested targets from bazel-bin. */
export function bazelRemoteCacheV1(contract:RemoteCacheContractV1,environment:Readonly<Record<string,string|undefined>>,options:Readonly<{helperDirectory:string;platform?:NodeJS.Platform;proxy?:string}>):BazelRemoteCacheV1{
 const signed=environment.SEMIO_REMOTE_CACHE_SIGNED==="1";
 if(signed&&options.proxy===undefined)return Object.freeze({flags:Object.freeze([]),environment:Object.freeze({}),rendering:Object.freeze({mechanism:"bazel-flags",applied:Object.freeze([]),unsupported:Object.freeze([]),uploadEnforcedBy:"client"as const})});
 const upload=remoteCacheEffectiveUploadV1(contract),token=signed?undefined:readRemoteCacheTokenV1(contract,environment),host=new URL(contract.endpoint).hostname.replace(/^\[|\]$/gu,"");
 const flags=[`--remote_cache=${signed?`${options.proxy}/${contract.instance}`:remoteCacheBaseUrlV1(contract)}`,`--remote_instance_name=${contract.instance}`,`--${upload?"":"no"}remote_upload_local_results`,`--${contract.compression?"":"no"}remote_cache_compression`,`--remote_download_outputs=${contract.download}`,`--remote_timeout=${contract.timeoutSeconds}s`,`--remote_retries=${contract.retries}`];
 const helper=token===undefined?undefined:bazelCredentialHelperV1(options.helperDirectory,contract.auth.tokenEnvironment,options.platform);
 if(helper)flags.push(`--credential_helper=${host}=${helper.path}`);
 const rendering:RemoteCacheRenderingV1=Object.freeze({mechanism:"bazel-flags",applied:Object.freeze([...flags]),unsupported:Object.freeze([]),uploadEnforcedBy:upload?"none":"client"});
 return Object.freeze({flags:Object.freeze(flags),environment:Object.freeze(token===undefined?{}:{[contract.auth.tokenEnvironment]:token}),...(helper?{helper}:{}),rendering});
}

/** 🔌️ Bazel's side of `remote-cache status`: it speaks the HTTP cache protocol and proves reachability on the action cache route. */
export function bazelRemoteCacheToolV1():RemoteCacheToolV1{
 return Object.freeze<RemoteCacheToolV1>({tool:"bazel",protocol:"bazel-http",probePath:"/ac/"+"0".repeat(64),render:(contract,environment)=>bazelRemoteCacheV1(contract,environment,{helperDirectory:"<tmp>",proxy:"http://127.0.0.1:<proxy>"}).rendering});
}

/** 🔏️ Starts, or reuses when healthy, the loopback signing proxy a machine with a fleet identity needs for Bazel's unsigned HTTP cache requests, for the time of one Bazel invocation: `url` is what `bazelRemoteCacheV1` takes as `proxy`, `stop` releases this holder (the proxy ends with its last holder). Without a fleet identity, with the remote cache off or on any failure it answers no `url`, and Bazel runs with its local caches only. Every upstream call is bounded by the contract's `timeoutSeconds` and a fleet cache that fails is skipped for a few seconds, so the cache never delays or fails the build. */
export async function startBazelRemoteCacheProxyV1(environment:Readonly<Record<string,string|undefined>>,options:Readonly<{skip?:boolean;downMilliseconds?:number}>={}):Promise<Readonly<{url?:string;stop():Promise<void>}>>{
 const none=Object.freeze({stop:async()=>{}});
 try{
  const resolved=options.skip===true?undefined:resolveRemoteCacheV1(environment);
  if(resolved===undefined||resolved.environment.SEMIO_REMOTE_CACHE_SIGNED!=="1")return none;
  const root=fleetCacheRootV1(environment),lease=await acquireRemoteCacheSigningProxyV1({upstream:resolved.contract.endpoint,fetch:remoteCacheFetchV1(resolved.environment,{...(root===undefined?{}:{root})}),timeoutMilliseconds:resolved.contract.timeoutSeconds*1000,...(options.downMilliseconds===undefined?{}:{downMilliseconds:options.downMilliseconds})});
  return lease===undefined?none:Object.freeze({url:lease.url,stop:()=>lease.release()});
 }catch{return none;}
}
