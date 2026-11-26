#!/usr/bin/env repo
import {isMainModuleV1,onTerminationV1} from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
import {readFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {runTopologyCommandAsyncV1} from "./⌨️command/🟦️.ts";
import {fleetTcpProbeV1,resolveFleetEndpointV1} from "./🧭️addresses/🟦️.ts";

/** 🧭️ Resolves the origins of the roles to addresses through the one resolver of the fleet (last good, the addresses published in the store or kept from it on this machine, then DNS-SD discovery): the store of this machine comes from `.cache/f/machine.json` of `SEMIO_WORKSPACE_ROOT`; a machine that has neither (first provisioning) relies on discovery or on `--addresses`. An origin that does not answer is left out. */
export async function resolveRoleHostsV1(requests:readonly Readonly<{origin:string;port:number}>[],environment:Readonly<Record<string,string|undefined>>,notice:(line:string)=>void,signal:AbortSignal):Promise<Readonly<Record<string,string>>>{
 const workspace=environment.SEMIO_WORKSPACE_ROOT,root=workspace!==undefined&&workspace!==""?workspace:join(tmpdir(),"semio-fleet-bootstrap");
 let storeRoot:string|undefined;
 try{const value=(JSON.parse(readFileSync(join(root,".cache","f","machine.json"),"utf8")) as {storeRoot?:unknown}).storeRoot;storeRoot=typeof value==="string"&&value!==""?value:undefined;}catch{}
 const found:Record<string,string>={};
 await Promise.all(requests.map(async request=>{
  const answer=await resolveFleetEndpointV1({root,...storeRoot===undefined?{}:{storeRoot},origin:request.origin,service:`role-${request.port}`,port:request.port,probe:fleetTcpProbeV1,signal,report:notice,language:environment.SEMIO_LANGUAGE==="de"?"de":"en"});
  if(answer!==undefined)found[request.origin]=new URL(answer.endpoint).hostname.replace(/^\[|\]$/gu,"");
 }));
 return found;
}

/** 📜️ Runs one topology command (`env`, `mount-plan`, `server-plan`, `worker-service`, `facts`, `validate`, `preflight`) with explicit sinks; returns the process exit code. It starts without the invocation envelope of the build door: the provisioning scripts call it as `repo ./📜️script.ts <command>` on a machine that has only the launcher. */
export async function runTopologyScriptV1(args:readonly string[],platform:string,signal:AbortSignal,write:(text:string)=>void,notice:(line:string)=>void,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<number>{
 try{
  const {text,exitCode}=await runTopologyCommandAsyncV1(args,{platform,signal,progress:notice,environment,resolveHosts:requests=>resolveRoleHostsV1(requests,environment,notice,signal)});
  write(text.endsWith("\n")?text:`${text}\n`);
  return exitCode;
 }catch(error){notice(error instanceof Error?error.message:String(error));return 1;}
}

if(isMainModuleV1(import.meta.url)){
 const controller=new AbortController(),stop=onTerminationV1(()=>controller.abort());
 process.exitCode=await runTopologyScriptV1(process.argv.slice(2),process.platform,controller.signal,text=>{process.stdout.write(text);},line=>{process.stderr.write(`${line}\n`);});
 stop();
}
