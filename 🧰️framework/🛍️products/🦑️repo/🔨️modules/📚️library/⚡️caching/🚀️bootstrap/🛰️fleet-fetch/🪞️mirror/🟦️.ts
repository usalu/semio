import {mkdir,rename,rm,writeFile} from "node:fs/promises";
import {readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import type {FleetFetchContextV1} from "../🟦️.ts";

const BASE=/^http:\/\/[A-Za-z0-9.\-[\]:]+$|^https:\/\/[A-Za-z0-9.\-[\]:]+$/u,PROBE="/mirror/healthz",TTL_MILLISECONDS=300000,PROBE_MILLISECONDS=1500;

/** 🪞️ What the last probe of the group server's mirror found; machine-local, never replicated. */
export type FleetMirrorStateV1=Readonly<{version:1;checkedAtMs:number;base:string;healthy:boolean}>;

const statePath=(root:string)=>join(root,".cache","f","mirror-state.json");

/** 🔭️ The mirror base this machine may use right now: the last probe found it healthy within five minutes, `SEMIO_FLEET_MIRROR` is not `off`. Synchronous and local, so an environment builder can call it. */
export function fleetMirrorBaseV1(ambient:Readonly<Record<string,string|undefined>>=process.env,now:number=Date.now()):string|undefined{
 const root=ambient.SEMIO_WORKSPACE_ROOT;
 if(ambient.SEMIO_FLEET_MIRROR==="off"||ambient.SEMIO_FLEET_FETCH==="off"||!root)return undefined;
 try{
  const state=JSON.parse(readFileSync(statePath(root),"utf8")) as FleetMirrorStateV1;
  return state.version===1&&state.healthy&&now-state.checkedAtMs<TTL_MILLISECONDS&&BASE.test(state.base)?state.base:undefined;
 }catch{return undefined;}
}

/** 🩺️ Probes `GET <cache endpoint>/mirror/healthz` with a 1.5 s timeout and records the result; a missing endpoint, an old server without the mirror or any failure records "unhealthy", so the tools keep their upstream settings. */
export async function refreshFleetMirrorStateV1(context:FleetFetchContextV1):Promise<FleetMirrorStateV1|undefined>{
 const root=context.root;if(!root||context.remote===undefined)return undefined;
 const base=context.remote.endpoint.replace(/\/+$/u,"");
 let healthy=false;
 try{const response=await context.fetch(base+PROBE,{signal:AbortSignal.timeout(PROBE_MILLISECONDS)});healthy=response.status===200&&((await response.json().catch(()=>({}))) as {status?:string}).status==="ok";}catch{}
 const state:FleetMirrorStateV1={version:1,checkedAtMs:context.now(),base,healthy};
 try{const path=statePath(root),temporary=path+".tmp";await mkdir(dirname(path),{recursive:true});await writeFile(temporary,JSON.stringify(state));await rename(temporary,path).catch(async()=>{await rm(temporary,{force:true});});}catch{}
 return state;
}

/** 🚚️ The transport settings that point a package manager at the mirror. They belong to the child process environment only: they are added after the sealed recipe and the ambient observation, so no generation or cache key sees them. An explicit registry of the user wins; Go keeps the upstream behind the mirror with `|`, which falls back on any error. */
export function fleetMirrorTransportEnvironmentV1(manager:string,environment:Readonly<Record<string,string|undefined>>,ambient:Readonly<Record<string,string|undefined>>=process.env):Readonly<Record<string,string>>{
 const base=fleetMirrorBaseV1(ambient);if(base===undefined)return {};
 if(manager==="bun"){
  if(environment.BUN_CONFIG_REGISTRY||environment.npm_config_registry||environment.NPM_CONFIG_REGISTRY||ambient.BUN_CONFIG_REGISTRY||ambient.npm_config_registry||ambient.NPM_CONFIG_REGISTRY)return {};
  return {BUN_CONFIG_REGISTRY:`${base}/mirror/npm/`};
 }
 if(manager==="go"){
  const existing=environment.GOPROXY||ambient.GOPROXY||"https://proxy.golang.org,direct";
  return existing.startsWith(`${base}/mirror/go`)?{}:{GOPROXY:`${base}/mirror/go|${existing}`};
 }
 return {};
}
