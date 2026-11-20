import {lstatSync,readdirSync} from "node:fs";
import {rm} from "node:fs/promises";
import {join} from "node:path";
import {setImmediate as yieldTurn} from "node:timers/promises";
import {CACHE_RUNTIME_V1} from "../../../../💾️storage/🧾️ledger/🟦️.ts";

/** 📶️ Reports visited entries, scanned groups, and finished removals to the owning collector. */
export type CargoGcProgressV1=(done:number,total:number,phase:"scan"|"measure"|"remove",path:string)=>unknown;

/** 🎛️ Connects a Cargo collector to its original cancellation, progress, and deletion ports. */
export type CargoGcControlV1=Readonly<{signal?:AbortSignal;progress?:CargoGcProgressV1;remove?:(path:string)=>Promise<void>}>;

/** 📏️ Measures each file's share of allocated blocks without following links, yielding between bounded batches. */
export async function allocatedCargoBytesV1(path:string,limit:number,control:CargoGcControlV1):Promise<number>{
 const {signal,progress}=control;
 let bytes=0,entries=0;
 const stack=[path];
 signal?.throwIfAborted();
 scan:while(stack.length){
  const parent=stack.pop()!;
  let children;
  try{children=readdirSync(parent,{withFileTypes:true});}catch{continue;}
  for(const child of children){
   if(entries===limit)break scan;
   const full=join(parent,child.name);
   let info;
   try{info=lstatSync(full);}catch{}
   if(info?.isDirectory())stack.push(full);
   else if(info&&!info.isSymbolicLink())bytes+=Math.round((info.blocks??Math.ceil(info.size/512))*512/Math.max(1,info.nlink));
   if(++entries%CACHE_RUNTIME_V1.checkEvery===0){
    await progress?.(entries,0,"measure",path);
    await yieldTurn();
    signal?.throwIfAborted();
   }
  }
 }
 if(entries%CACHE_RUNTIME_V1.checkEvery!==0)await progress?.(entries,entries,"measure",path);
 signal?.throwIfAborted();
 return bytes;
}

/** 🗑️ Deletes a candidate using the owner's physical removal operation when supplied. */
export async function removeCargoGcTreeV1(path:string,control:CargoGcControlV1):Promise<void>{
 control.signal?.throwIfAborted();
 if(control.remove)await control.remove(path);
 else await rm(path,{recursive:true,force:true});
}
