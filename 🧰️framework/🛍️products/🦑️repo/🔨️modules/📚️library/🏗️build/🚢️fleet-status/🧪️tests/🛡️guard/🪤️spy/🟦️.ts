/** 🪤️ Preload of the fleet status guard tests: records every filesystem write and every child process of the process into `FLEET_SPY_OUT` (one JSON line each), before any module under test is loaded, so a read-only command can be proven read-only by what it did. */
import {appendFileSync} from "node:fs";
const out=process.env.FLEET_SPY_OUT!,log=appendFileSync,record=(op:string,subject:unknown)=>{try{log(out,JSON.stringify({op,subject:typeof subject==="string"?subject:String((subject as {toString?:()=>string})?.toString?.()??subject)})+"\n");}catch{}};
const fs=require("node:fs"),promises=require("node:fs/promises"),child=require("node:child_process");
const writing=(flags:unknown):boolean=>typeof flags==="number"?(flags&(fs.constants.O_WRONLY|fs.constants.O_RDWR|fs.constants.O_CREAT|fs.constants.O_APPEND|fs.constants.O_TRUNC))!==0:typeof flags==="string"&&/[wa+]/u.test(flags);
const plain=["writeFile","appendFile","mkdir","rm","rmdir","unlink","rename","copyFile","link","symlink","truncate","utimes","chmod","cp","mkdtemp"];
for(const name of plain){
 for(const [owner,suffix] of [[fs,"Sync"],[fs,""],[promises,""]] as const){
  const key=name+suffix,original=owner[key];if(typeof original!=="function")continue;
  owner[key]=function(this:unknown,...args:unknown[]){record(owner===promises?"promises."+name:key,args[0]);return original.apply(this,args);};
 }
}
for(const [owner,key] of [[fs,"openSync"],[fs,"open"],[promises,"open"]] as const){
 const original=owner[key];if(typeof original!=="function")continue;
 owner[key]=function(this:unknown,...args:unknown[]){if(writing(args[1]))record("open-for-write",args[0]);return original.apply(this,args);};
}
for(const key of ["spawn","spawnSync","execFile","execFileSync","exec","execSync","fork"]){
 const original=child[key];if(typeof original!=="function")continue;
 child[key]=function(this:unknown,...args:unknown[]){record("child",[args[0],...Array.isArray(args[1])?args[1]:[]].join(" "));return original.apply(this,args);};
}
for(const key of ["spawn","spawnSync"] as const){
 const original=(Bun as unknown as Record<string,unknown>)[key];if(typeof original!=="function")continue;
 (Bun as unknown as Record<string,unknown>)[key]=function(this:unknown,...args:unknown[]){const command=Array.isArray(args[0])?args[0]:(args[0] as {cmd?:unknown[]}|undefined)?.cmd;record("child",Array.isArray(command)?command.map(String).join(" "):"?");return (original as (...a:unknown[])=>unknown).apply(this,args);};
}
{
 const sqlite=require("bun:sqlite"),Original=sqlite.Database;
 sqlite.Database=class extends Original{constructor(path:unknown,...rest:unknown[]){record("sqlite-open",typeof path==="string"?path:String(path));super(path,...rest);}};
}
