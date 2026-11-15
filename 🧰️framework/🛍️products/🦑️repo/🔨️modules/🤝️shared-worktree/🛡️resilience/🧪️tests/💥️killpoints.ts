import {mock} from "bun:test";

/** 💥️ The error that stands for a process that died at one filesystem mutation. */
export class KillPointV1 extends Error{
 readonly index:number;
 readonly mode:"before"|"after";
 constructor(index:number,mode:"before"|"after",operation:string){super(`kill point ${index} ${mode} ${operation}`);this.name="KillPointV1";this.index=index;this.mode=mode;}
}

/** 🎛️ The control of the installed kill points: `reset` restarts the count at zero, `arm` makes the mutation with the given index throw before it happened or right after it happened (with `transient` only that one call fails like an ordinary I/O error and the process lives on, otherwise every later mutation fails too, as in a dead process), `disarm` waits until the dead process has stopped trying to mutate anything and then stops that, `count` is the number of mutations since the last reset, `last` the operation names in order. */
export type KillPointsV1=Readonly<{reset():void;arm(index:number,mode:"before"|"after",transient?:boolean):void;disarm():Promise<void>;count():number;last():readonly string[]}>;

const mutating=["rename","writeFile","appendFile","unlink","rm","rmdir","truncate","copyFile","mkdir","symlink","link","chmod","utimes"],handleMutating=new Set(["write","writeFile","appendFile","truncate","chmod","utimes","sync"]);

/** 💥️ Replaces `node:fs/promises` for every module imported afterwards with a copy whose mutating functions (and the mutating methods of the file handles that `open` returns for writing) are numbered, so that a test can abort a replica operation at the n-th mutation, before or after it, reopen the replica and check that every kill point leaves a state that converges. The caller must import the code under test dynamically after this call: the mock applies only to later imports. */
export async function installKillPointsV1():Promise<KillPointsV1>{
 const real={...await import("node:fs/promises")} as unknown as Record<string,(...args:unknown[])=>Promise<unknown>>;
 let armed:Readonly<{index:number;mode:"before"|"after";transient:boolean}>|undefined,counter=0,attempts=0,dead=false,names:string[]=[];
 const point=async<T>(name:string,run:()=>Promise<T>,release?:(value:T)=>Promise<unknown>):Promise<T>=>{
  attempts++;
  if(dead)throw new KillPointV1(-1,"before",name);
  const index=counter++;
  names.push(name);
  if(armed?.index===index&&armed.mode==="before"){dead=!armed.transient;armed=undefined;throw new KillPointV1(index,"before",name);}
  const value=await run();
  if(armed?.index===index&&armed.mode==="after"){dead=!armed.transient;armed=undefined;await release?.(value);throw new KillPointV1(index,"after",name);}
  return value;
 };
 const wrapped:Record<string,unknown>={...real};
 for(const name of mutating)wrapped[name]=(...args:unknown[])=>point(name,()=>real[name]!(...args));
 wrapped.open=async(...args:unknown[])=>{
  const flags=String(args[1]??"r"),writes=/[wa+x]/u.test(flags),open=()=>real.open!(...args) as Promise<object>;
  const handle=writes?await point("open",open,value=>(value as {close():Promise<void>}).close()):await open();
  return new Proxy(handle,{get(target,property){
   const value=Reflect.get(target,property,target) as unknown;
   if(typeof value!=="function")return value;
   if(typeof property==="string"&&handleMutating.has(property))return (...call:unknown[])=>point(`handle.${property}`,()=>(value as (...call:unknown[])=>Promise<unknown>).apply(target,call));
   return (value as (...call:unknown[])=>unknown).bind(target);
  }});
 };
 mock.module("node:fs/promises",()=>({...wrapped,default:wrapped}));
 return {
  reset:()=>{counter=0;attempts=0;dead=false;names=[];armed=undefined;},
  arm:(index,mode,transient=false)=>{armed={index,mode,transient};},
  disarm:async()=>{
   let seen=-1;
   for(let quiet=0;quiet<3;){const now=attempts;quiet=now===seen?quiet+1:0;seen=now;await new Promise<void>(done=>{setTimeout(done,15);});}
   armed=undefined;dead=false;
  },
  count:()=>counter,
  last:()=>names
 };
}
