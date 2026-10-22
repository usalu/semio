import type {RemoteCacheToolV1} from "../../../🌐️remote-cache/⌨️command/🟦️.ts";
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

/** 🎛️ Renders the neutral contract into Bazel: `--remote_cache`, `--remote_instance_name`, `--[no]remote_upload_local_results`, `--[no]remote_cache_compression`, `--remote_download_outputs`, `--remote_timeout`, `--remote_retries`, and the token through a credential helper scoped to the cache host, because `--remote_header` copies the token into Bazel's java.log and build event stream on disk. The contract has no `minimal` download, since the task adapter restores the snapshots of the requested targets from bazel-bin. */
export function bazelRemoteCacheV1(contract:RemoteCacheContractV1,environment:Readonly<Record<string,string|undefined>>,options:Readonly<{helperDirectory:string;platform?:NodeJS.Platform}>):BazelRemoteCacheV1{
 const upload=remoteCacheEffectiveUploadV1(contract),token=readRemoteCacheTokenV1(contract,environment),host=new URL(contract.endpoint).hostname.replace(/^\[|\]$/gu,"");
 const flags=[`--remote_cache=${remoteCacheBaseUrlV1(contract)}`,`--remote_instance_name=${contract.instance}`,`--${upload?"":"no"}remote_upload_local_results`,`--${contract.compression?"":"no"}remote_cache_compression`,`--remote_download_outputs=${contract.download}`,`--remote_timeout=${contract.timeoutSeconds}s`,`--remote_retries=${contract.retries}`];
 const helper=token===undefined?undefined:bazelCredentialHelperV1(options.helperDirectory,contract.auth.tokenEnvironment,options.platform);
 if(helper)flags.push(`--credential_helper=${host}=${helper.path}`);
 const rendering:RemoteCacheRenderingV1=Object.freeze({mechanism:"bazel-flags",applied:Object.freeze([...flags]),unsupported:Object.freeze([]),uploadEnforcedBy:upload?"none":"client"});
 return Object.freeze({flags:Object.freeze(flags),environment:Object.freeze(token===undefined?{}:{[contract.auth.tokenEnvironment]:token}),...(helper?{helper}:{}),rendering});
}

/** 🔌️ Bazel's side of `remote-cache status`: it speaks the HTTP cache protocol and proves reachability on the action cache route. */
export function bazelRemoteCacheToolV1():RemoteCacheToolV1{
 return Object.freeze<RemoteCacheToolV1>({tool:"bazel",protocol:"bazel-http",probePath:"/ac/"+"0".repeat(64),render:(contract,environment)=>bazelRemoteCacheV1(contract,environment,{helperDirectory:"<tmp>"}).rendering});
}
