#!/usr/bin/env repo
import {resolve} from "node:path";
import {isMainModuleV1,onTerminationV1} from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
import {runReplicaCommandV1} from "./⌨️command/🟦️.ts";

/** 📜️ Runs one replica command against a repository root with explicit sinks: stdout lines and stderr lines; returns the process exit code. */
export async function runReplicaScriptV1(args:readonly string[],root:string,environment:Readonly<Record<string,string|undefined>>,signal:AbortSignal,write:(line:string)=>void,notice:(line:string)=>void):Promise<number>{
 try{return (await runReplicaCommandV1(args,{root,environment,signal,write,notice})).exitCode;}
 catch(error){notice(error instanceof Error?error.message:String(error));return 1;}
}

if(isMainModuleV1(import.meta.url)){
 const controller=new AbortController(),stop=onTerminationV1(()=>controller.abort());
 process.exitCode=await runReplicaScriptV1(process.argv.slice(2),resolve(import.meta.dirname,"../../../../../.."),process.env,controller.signal,line=>{process.stdout.write(`${line}\n`);},line=>{process.stderr.write(`${line}\n`);});
 stop();
}
