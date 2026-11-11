#!/usr/bin/env repo
import {isMainModuleV1,onTerminationV1} from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
import {runTopologyCommandAsyncV1} from "./⌨️command/🟦️.ts";

/** 📜️ Runs one topology command (`env`, `mount-plan`, `server-plan`, `worker-service`, `facts`, `validate`, `preflight`) with explicit sinks; returns the process exit code. It starts without the invocation envelope of the build door: the provisioning scripts call it as `repo ./📜️script.ts <command>` on a machine that has only the launcher. */
export async function runTopologyScriptV1(args:readonly string[],platform:string,signal:AbortSignal,write:(text:string)=>void,notice:(line:string)=>void,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<number>{
 try{
  const {text,exitCode}=await runTopologyCommandAsyncV1(args,{platform,signal,progress:notice,environment});
  write(text.endsWith("\n")?text:`${text}\n`);
  return exitCode;
 }catch(error){notice(error instanceof Error?error.message:String(error));return 1;}
}

if(isMainModuleV1(import.meta.url)){
 const controller=new AbortController(),stop=onTerminationV1(()=>controller.abort());
 process.exitCode=await runTopologyScriptV1(process.argv.slice(2),process.platform,controller.signal,text=>{process.stdout.write(text);},line=>{process.stderr.write(`${line}\n`);});
 stop();
}
