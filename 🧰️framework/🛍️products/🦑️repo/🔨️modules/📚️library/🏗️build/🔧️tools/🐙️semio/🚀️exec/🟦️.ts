import {spawn} from "node:child_process";
import {resolve} from "node:path";
import {SEMIO_POLICY_V1 as policy} from "../🟦️.ts";
import type {BuildTaskV1} from "../../../🧮️task-model/🟦️.ts";

/** 🏁️ The result of running one task body. */
export type SemioExecutionV1=Readonly<{exitCode:number}>;
/** 🚀️ Runs the body of a task and stops it, with its whole process tree, when the signal aborts. */
export type SemioExecutorV1=(task:BuildTaskV1,signal:AbortSignal,write:(chunk:string)=>void,started?:()=>void)=>Promise<SemioExecutionV1>;
/** 🧪️ Runs a body with the environment a task runs in; a scope may hold resources open around it (orchestration authority) and may add its own cancellation. */
export type SemioEnvironmentScopeV1=<T>(task:BuildTaskV1,body:(environment:NodeJS.ProcessEnv,extra?:AbortSignal)=>Promise<T>)=>Promise<T>;

/** 🔡️ Fills `{workspaceRoot}`, `{projectRoot}`, `{projectName}` and `{options.*}` into a command line. */
export function semioInterpolateV1(value:string,workspace:string,task:Pick<BuildTaskV1,"project"|"root"|"definition"|"id">):string{
 return value.replaceAll("{workspaceRoot}",workspace).replaceAll("{projectRoot}",resolve(workspace,task.root)).replaceAll("{projectName}",task.project).replace(/\{options\.([^}]+)\}/gu,(_match,key:string)=>{
  let option:unknown=task.definition.execution;
  for(const part of key.split("."))option=option&&typeof option==="object"?(option as Record<string,unknown>)[part]:undefined;
  if(typeof option!=="string"&&typeof option!=="number"&&typeof option!=="boolean")throw Error(`Build task requires option ${key}: ${task.id}`);
  return String(option);
 });
}

const quote=(value:string)=>"'"+value.replaceAll("'","'\\''")+"'";

/** 🧵️ The processes one task runs, in order or together: the declared program, or each declared command through the runtime's portable shell. */
export function semioCommandsV1(task:BuildTaskV1,workspace:string,runtime:string):readonly Readonly<{command:string;args:readonly string[]}>[]{
 const execution=task.definition.execution;
 if(execution.kind==="noop")return [];
 const forwarded=[...(execution.args??[]),...task.args];
 if(execution.program!==undefined)return [{command:execution.program,args:forwarded}];
 const operations=execution.commands??(execution.command?[execution.command]:[]);
 if(!operations.length)throw Error(`Build command required: ${task.id}`);
 return operations.map(operation=>{
  const text=typeof operation==="string"?operation:operation.command,forward=typeof operation==="object"?operation.forwardAllArgs??execution.forwardAllArgs:execution.forwardAllArgs;
  let source=semioInterpolateV1(text,workspace,task);
  if(forward!==false&&forwarded.length&&!execution.ownerArguments)source+=" "+forwarded.map(quote).join(" ");
  return {command:runtime,args:["exec",source]};
 });
}

function terminate(child:ReturnType<typeof spawn>):void{
 if(child.pid===undefined)return;
 if(process.platform==="win32"){spawn("taskkill",["/pid",String(child.pid),"/T","/F"],{stdio:"ignore",windowsHide:true});return;}
 try{process.kill(-child.pid,"SIGTERM");}catch{try{child.kill("SIGTERM");}catch{}}
 setTimeout(()=>{try{process.kill(-child.pid!,"SIGKILL");}catch{}},policy.killGraceMilliseconds).unref();
}

/** 🏃️ The local executor: runs the commands of a task in the workspace with the given environment, forwards output, reports when the first process started, and kills the process tree on abort. A scope replaces the plain environment function when the environment must be issued under held authority. */
export function semioLocalExecutorV1(workspace:string,environment:(task:BuildTaskV1)=>NodeJS.ProcessEnv,runtime:string=process.execPath,scope?:SemioEnvironmentScopeV1):SemioExecutorV1{
 const run=async(task:BuildTaskV1,signal:AbortSignal,write:(chunk:string)=>void,started:(()=>void)|undefined,env:NodeJS.ProcessEnv):Promise<SemioExecutionV1>=>{
  const commands=semioCommandsV1(task,workspace,runtime),execution=task.definition.execution,cwd=execution.cwd?resolve(workspace,execution.program===undefined?semioInterpolateV1(execution.cwd,workspace,task):execution.cwd):workspace;
  let announced=false;const announce=()=>{if(!announced){announced=true;started?.();}};
  const launch=(entry:{command:string;args:readonly string[]},stopper:AbortSignal):Promise<number>=>new Promise(accept=>{
   if(stopper.aborted){accept(1);return;}
   const child=spawn(entry.command,[...entry.args],{cwd,env,stdio:["ignore","pipe","pipe"],detached:process.platform!=="win32",windowsHide:true}),stop=()=>terminate(child);
   stopper.addEventListener("abort",stop,{once:true});
   child.once("spawn",announce);
   child.stdout!.setEncoding("utf8").on("data",write);child.stderr!.setEncoding("utf8").on("data",write);
   child.once("error",()=>{stopper.removeEventListener("abort",stop);accept(127);});
   child.once("close",code=>{stopper.removeEventListener("abort",stop);accept(code??1);});
  });
  const cancelled=()=>signal.reason??Error("Cancelled");
  if(!commands.length){announce();return {exitCode:0};}
  if(execution.parallel===false||commands.length<2){for(const entry of commands){const code=await launch(entry,signal);if(signal.aborted)throw cancelled();if(code!==0)return {exitCode:code};}return {exitCode:0};}
  const sibling=new AbortController(),both=AbortSignal.any([signal,sibling.signal]);
  let first=0;
  await Promise.all(commands.map(async entry=>{const code=await launch(entry,both);if(code!==0){if(first===0)first=code;sibling.abort(Error("Sibling failed"));}}));
  if(signal.aborted)throw cancelled();
  return {exitCode:first};
 };
 return async(task,signal,write,started)=>scope?scope(task,(env,extra)=>run(task,extra?AbortSignal.any([signal,extra]):signal,write,started,env)):run(task,signal,write,started,environment(task));
}

/** 🧮️ Answers the output of a runtime input command, run through the runtime's portable shell in the workspace; a failing command is an error, so a cache never keys on a missing answer. */
export function semioRuntimeOutputV1(workspace:string,environment:NodeJS.ProcessEnv,runtime:string=process.execPath):(command:string,signal:AbortSignal)=>Promise<string>{
 return (command,signal)=>new Promise((accept,refuse)=>{
  const child=spawn(runtime,["exec",command],{cwd:workspace,env:environment,stdio:["ignore","pipe","pipe"],detached:process.platform!=="win32",windowsHide:true}),chunks:string[]=[],stop=()=>terminate(child);
  signal.addEventListener("abort",stop,{once:true});
  child.stdout!.setEncoding("utf8").on("data",chunk=>chunks.push(chunk));child.stderr!.resume();
  child.once("error",error=>{signal.removeEventListener("abort",stop);refuse(error);});
  child.once("close",code=>{signal.removeEventListener("abort",stop);if(signal.aborted)refuse(signal.reason??Error("Cancelled"));else if(code===0)accept(chunks.join("").trim());else refuse(Error(`Runtime input command failed (${code}): ${command}`));});
 });
}
