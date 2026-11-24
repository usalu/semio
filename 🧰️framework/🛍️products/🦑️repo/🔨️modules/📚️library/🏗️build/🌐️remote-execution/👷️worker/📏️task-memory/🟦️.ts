import {execFile} from "node:child_process";

/** 📏️ The resident megabytes of every process group in a `ps -A -o pgid= -o rss=` table (kilobytes in, rounded up), for the groups asked for. Pure. */
export function remoteExecutionGroupMegabytesV1(table:string,groups:ReadonlySet<number>):ReadonlyMap<number,number>{
 const kilobytes=new Map<number,number>();
 for(const line of table.split("\n")){
  const match=/^\s*(\d+)\s+(\d+)\s*$/u.exec(line);if(!match)continue;
  const group=Number(match[1]);if(groups.has(group))kilobytes.set(group,(kilobytes.get(group)??0)+Number(match[2]));
 }
 return new Map([...kilobytes].map(([group,kb])=>[group,Math.ceil(kb/1024)]));
}

/** 🌡️ Reads the process table of this machine, or nothing where the host has no `ps` with process groups (Windows). */
export function remoteExecutionProcessTableV1():Promise<string|undefined>{
 if(process.platform==="win32")return Promise.resolve(undefined);
 return new Promise(resolve=>{execFile("ps",["-A","-o","pgid=","-o","rss="],{maxBuffer:64*1048576,timeout:5000},(error,stdout)=>{resolve(error?undefined:stdout);});});
}

/** 📋️ What a worker knows about the memory of its running tasks: the resident megabytes of each task's process group now, and, when the task ends, the largest sampled value. */
export type RemoteExecutionMeterV1=Readonly<{attach(task:string,pid:number):void;usage():readonly Readonly<{task:string;megabytes:number}>[];finish(task:string):number|undefined;close():void}>;

/** 🧮️ Samples the memory of the process groups of the attached tasks (one table read per `intervalMilliseconds` for all of them, only while one is attached), keeps the current and the peak value of each; a task that was never sampled has no value (its worker reports none). Bounded by the number of running tasks. */
export function createRemoteExecutionMeterV1(read:()=>Promise<string|undefined>,intervalMilliseconds:number):RemoteExecutionMeterV1{
 const tasks=new Map<string,{pid:number;current?:number;peak?:number}>();
 let timer:ReturnType<typeof setInterval>|undefined,reading=false;
 const sample=async():Promise<void>=>{
  if(reading||tasks.size===0)return;
  reading=true;
  try{
   const table=await read();if(table===undefined)return;
   const sizes=remoteExecutionGroupMegabytesV1(table,new Set([...tasks.values()].map(row=>row.pid)));
   for(const row of tasks.values()){const megabytes=sizes.get(row.pid);if(megabytes!==undefined){row.current=megabytes;row.peak=Math.max(row.peak??0,megabytes);}}
  }catch{}finally{reading=false;}
 };
 const stop=():void=>{if(timer!==undefined){clearInterval(timer);timer=undefined;}};
 return Object.freeze({
  attach(task,pid){tasks.set(task,{pid});if(timer===undefined){timer=setInterval(()=>{void sample();},intervalMilliseconds);timer.unref();}void sample();},
  usage(){return [...tasks].flatMap(([task,row])=>row.current===undefined?[]:[{task,megabytes:row.current}]);},
  finish(task){const peak=tasks.get(task)?.peak;tasks.delete(task);if(tasks.size===0)stop();return peak;},
  close(){tasks.clear();stop();},
 });
}
