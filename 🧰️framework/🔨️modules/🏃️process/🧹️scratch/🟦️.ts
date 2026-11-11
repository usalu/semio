import {lstatSync,mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import protocol from "./🔣️.json" with {type:"json"};

/** 🧹️ Where scratch lives: the checkout the scratch belongs to (found from `root`, else the working directory) and the process environment that may switch it off. */
export type ScratchOptionsV1=Readonly<{root?:string;environment?:Readonly<Record<string,string|undefined>>}>;

/** 🧭️ The checkout root at or above a directory: the nearest one holding both build markers; undefined outside any checkout. */
function checkoutOfV1(start:string):string|undefined{
 for(let directory=resolve(start);;directory=dirname(directory)){
  if(["📋️project.json","package.json"].every(name=>lstatSync(join(directory,name),{throwIfNoEntry:false})?.isFile()))return directory;
  if(dirname(directory)===directory)return undefined;
 }
}

/** 📏️ Whether a path lies inside a directory (or is it). */
function insideV1(directory:string,path:string):boolean{
 const distance=relative(directory,resolve(path));
 return distance===""||distance!==".."&&!distance.startsWith(".."+sep)&&!isAbsolute(distance);
}

/** 🏠️ The one declared home of repo-owned scratch: `.cache/scratch` of the checkout, a disk directory that is budgeted and swept by `repo cache gc` (the area `scratch` of the cache ledger); outside a checkout the operating system's temporary directory, because nothing declares a home there.
 * @see ./🔣️.json */
export function scratchHomeV1(options:ScratchOptionsV1={}):string{
 const root=checkoutOfV1(options.root??process.cwd());
 return root===undefined?join(tmpdir(),protocol.fallbackDirectory):join(root,protocol.root,protocol.directory);
}

/** 🆕️ Creates one scratch directory named `<pid>-<prefix>-<random>` below the home, so the sweep of the cache ledger knows which live process owns it; the caller removes it. */
export function createScratchDirectoryV1(prefix:string,options:ScratchOptionsV1={}):string{
 const home=scratchHomeV1(options);
 mkdirSync(home,{recursive:true});
 return mkdtempSync(join(home,`${process.pid}-${prefix.replace(/[^A-Za-z0-9._]/gu,"_").slice(0,protocol.maximumPrefixLength)}-`));
}

/** 🗑️ Removes a scratch directory and everything below it; a directory that is already gone is fine. */
export function removeScratchDirectoryV1(path:string):void{
 rmSync(path,{recursive:true,force:true,maxRetries:protocol.removal.maxRetries,retryDelay:protocol.removal.retryDelayMilliseconds});
}

/** 🔁️ Runs a body with a fresh scratch directory and removes it when the body completes, throws or is cancelled (a cancelled body rejects, so the removal below always runs). */
export async function withScratchDirectoryV1<T>(prefix:string,body:(directory:string)=>Promise<T>|T,options:ScratchOptionsV1={}):Promise<T>{
 const directory=createScratchDirectoryV1(prefix,options);
 try{return await body(directory);}finally{removeScratchDirectoryV1(directory);}
}

/** 🏝️ Gives this process a private scratch directory as its temporary directory (`TMPDIR`, `TMP`, `TEMP`, so every library and child process that asks the operating system lands on disk below the declared home) and removes it when the process exits or is stopped by a signal; a process that already runs below the home, or with scratch switched off, keeps what it has. Answers the directory, or undefined when it changed nothing. */
export function isolateProcessScratchV1(options:ScratchOptionsV1={},target:NodeJS.Process=process):string|undefined{
 const environment=options.environment??target.env;
 if(environment[protocol.off]==="off")return undefined;
 let directory:string;
 try{
  const home=scratchHomeV1(options),current=environment.TMPDIR??environment.TMP??environment.TEMP;
  if(current!==undefined&&insideV1(home,current))return undefined;
  directory=createScratchDirectoryV1("process",options);
 }catch{return undefined;}
 for(const name of protocol.environment)target.env[name]=directory;
 const clean=():void=>{try{removeScratchDirectoryV1(directory);}catch{}};
 target.once("exit",clean);
 for(const signal of ["SIGINT","SIGTERM","SIGHUP"] as const)target.once(signal,()=>{clean();target.kill(target.pid,signal);});
 return directory;
}
