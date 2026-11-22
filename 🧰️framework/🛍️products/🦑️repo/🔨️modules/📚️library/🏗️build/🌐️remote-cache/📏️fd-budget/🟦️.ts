import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";

/** 📏️ The open file limits of this machine that bound a process: the kernel's table (`maxFiles`), what one process may hold (`perProcess`) and the soft limit of this process (`soft`); `null` where a number cannot be read. */
export type RemoteCacheFileLimitsV1=Readonly<{maxFiles:number|null;perProcess:number|null;soft:number|null}>;

/** 💽️ The number of file descriptors the cache may use at once for fan-out work, as a share of the limits above: a quarter of what this process may hold and at most one percent of the kernel's whole table, so that no scan, handover or burst of requests of this process can be what fills the table of the machine. Without readable limits a conservative number. Pure. */
export function remoteCacheFileBudgetV1(limits:RemoteCacheFileLimitsV1):number{
 const shares=[limits.soft===null?null:limits.soft/4,limits.perProcess===null?null:limits.perProcess/4,limits.maxFiles===null?null:limits.maxFiles/100].filter((value):value is number=>value!==null);
 return Math.max(64,Math.floor(shares.length===0?256:Math.min(...shares)));
}

/** 🔭️ Reads the limits from the system (`sysctl` on macOS, `/proc` on Linux, nothing on other systems). */
export function readRemoteCacheFileLimitsV1(platform:NodeJS.Platform=process.platform):RemoteCacheFileLimitsV1{
 const number=(text:string|undefined):number|null=>{const value=Number(String(text??"").trim());return Number.isFinite(value)&&value>0?value:null;};
 try{
  if(platform==="darwin"){
   const run=(name:string)=>number(spawnSync("sysctl",["-n",name],{encoding:"utf8",timeout:2000}).stdout),soft=number(spawnSync("/bin/sh",["-c","ulimit -n"],{encoding:"utf8",timeout:2000}).stdout);
   return {maxFiles:run("kern.maxfiles"),perProcess:run("kern.maxfilesperproc"),soft};
  }
  if(platform==="linux"){
   const limits=readFileSync("/proc/self/limits","utf8").split("\n").find(line=>line.startsWith("Max open files"))?.split(/\s+/u),max=number(readFileSync("/proc/sys/fs/file-max","utf8"));
   return {maxFiles:max,perProcess:number(limits?.[5]),soft:number(limits?.[4])};
  }
 }catch{}
 return {maxFiles:null,perProcess:null,soft:null};
}

let memo:number|undefined;
/** 🧮️ The budget of this machine, read once. `SEMIO_REMOTE_CACHE_FILE_BUDGET` states it for tests and odd machines. */
export function remoteCacheFileBudgetOfMachineV1(environment:Readonly<Record<string,string|undefined>>=process.env):number{
 const stated=Number(environment.SEMIO_REMOTE_CACHE_FILE_BUDGET);
 if(Number.isInteger(stated)&&stated>=1)return stated;
 return memo??=remoteCacheFileBudgetV1(readRemoteCacheFileLimitsV1());
}

/** 🪜️ How wide a fan-out of `requested` file operations may be: never more than the budget allows at a cost of `perOperation` descriptors each, never less than one. */
export const remoteCacheWidthV1=(requested:number,budget:number,perOperation:number=1):number=>Math.max(1,Math.min(requested,Math.floor(budget/Math.max(1,perOperation))));
