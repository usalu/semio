import * as fs from "node:fs";
import {processPidExistsV1} from "../../../../../../../🔨️modules/🏃️process/🔖️identity/🟨️.mjs"
import {lstat,readdir,realpath,rm} from "node:fs/promises";
import {join,sep} from "node:path";

/** 🪦️ The declared shape of a per-process producer publication: the folder `p-<pid>` below `inference` of a selection, and how long a folder of a gone process must have been quiet before a sweep may remove it. */
export const PRODUCER_PUBLICATION_V1=Object.freeze({prefix:"p-",quietMs:60000});
const NAME=/^p-([1-9][0-9]{0,9})$/u,folders=new Set();

/** 🧟️ Asks the operating system whether a process still exists; a process that may not be signalled still exists. */
export function producerAliveV1(pid){
  return processPidExistsV1(pid);
}

/** 🏚️ Retires the calling process's own producer publication folder when the process ends: the receipt of the publisher keeps the bytes in the archive, so nobody reads the folder afterwards. A folder that is not a real directory any more is left alone, a link is never followed, a kill that skips the exit event is left to the sweep. */
export function retireProducerPublicationAtExitV1(folder){
  if(folders.has(folder))return;
  if(folders.size===0)process.once("exit",()=>{for(const path of folders)try{if(fs.lstatSync(path).isDirectory())fs.rmSync(path,{recursive:true,force:true});}catch{}});
  folders.add(folder);
}

async function allocated(path){
  let total=0;
  for(const entry of await readdir(path,{withFileTypes:true}).catch(()=>[])){
    const child=join(path,entry.name);
    if(entry.isDirectory())total+=await allocated(child);
    else{const status=await lstat(child).catch(()=>undefined);if(status)total+=status.blocks?status.blocks*512:status.size;}
  }
  return total;
}

/** 🧹️ Sweeps the producer publications of processes that are gone: below every `<selections>/<hash>/inference` the folders `p-<pid>` whose process does not exist and that were quiet for `quietMs`. A live process, a young folder, a link and anything not shaped `p-<digits>` stays; each folder is checked again right before it goes and nothing outside `selections` is touched. `dryRun` only lists. Cancellable through `signal`.
 * @param {string} selections
 * @param {{alive?:(pid:number)=>boolean;now?:()=>number;quietMs?:number;dryRun?:boolean;signal?:AbortSignal}} [ports]
 * @returns {Promise<{candidates:string[];removed:number;kept:number;bytes:number;cancelled:boolean}>} */
export async function sweepDeadProducerPublicationsV1(selections,ports={}){
  const alive=ports.alive??producerAliveV1,now=ports.now??Date.now,quietMs=ports.quietMs??PRODUCER_PUBLICATION_V1.quietMs,candidates=[];
  let removed=0,kept=0,bytes=0;
  const boundary=await realpath(selections).catch(()=>undefined);
  if(boundary===undefined)return {candidates,removed,kept,bytes,cancelled:false};
  for(const name of await readdir(selections).catch(()=>[])){
    if(ports.signal?.aborted)return {candidates,removed,kept,bytes,cancelled:true};
    const folder=join(selections,name,"inference");
    for(const entry of await readdir(folder,{withFileTypes:true}).catch(()=>[])){
      const match=NAME.exec(entry.name);if(!match||!entry.isDirectory())continue;
      const pid=Number(match[1]),path=join(folder,entry.name),status=await lstat(path).catch(()=>undefined);
      if(status===undefined||status.isSymbolicLink()||alive(pid)||now()-status.mtimeMs<quietMs){kept++;continue;}
      candidates.push(path);bytes+=await allocated(path);
      if(ports.dryRun)continue;
      const real=await realpath(path).catch(()=>"");
      if(!real.startsWith(boundary+sep)||alive(pid)){kept++;continue;}
      await rm(path,{recursive:true,force:true});removed++;
    }
  }
  return {candidates,removed,kept,bytes,cancelled:false};
}
