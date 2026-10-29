import {randomBytes} from "node:crypto";
import {closeSync,openSync,renameSync,rmSync,statSync,writeSync} from "node:fs";
import {dirname,join} from "node:path";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import {tryClaimV1,type ClaimV1} from "../../🔒️lock/🟦️.ts";
import {nativeFreeSpaceSyncV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

const rules=UNIT_CACHE_POLICY_V1.gc,chunk=1048576;
/** 🛟️ Where the ballast of a cache root lives: a file of real bytes that is given back the moment the image is full, so the collector and the admission can still create their claim and record files and free the rest. */
export function ballastPathV1(root:string):string{return join(root,rules.ballastFile);}

/** 🆘️ Gives the ballast back; `true` when bytes were freed. Never throws. */
export function releaseBallastV1(root:string):boolean{
 try{const bytes=statSync(ballastPathV1(root)).size;rmSync(ballastPathV1(root));return bytes>0;}catch{return false;}
}

/** 🧱️ Keeps the ballast whole while the image has room for it (four times its size free, so it never takes the last bytes); `present`, `created` or `skipped` when the image is too full or the write failed. Never throws. */
export function ensureBallastV1(root:string,space:(root:string)=>Readonly<{availableBytes:number}>|undefined=nativeFreeSpaceSyncV1):"present"|"created"|"skipped"{
 const path=ballastPathV1(root);
 try{if(statSync(path).size>=rules.ballastBytes)return "present";}catch{}
 let temporary="";
 try{
  const free=space(root);if(!free||free.availableBytes<rules.ballastBytes*4)return "skipped";
  temporary=path+"."+randomBytes(3).toString("hex")+".tmp";const fd=openSync(temporary,"w"),block=Buffer.alloc(chunk,0x5a);
  try{for(let written=0;written<rules.ballastBytes;written+=chunk)writeSync(fd,block,0,Math.min(chunk,rules.ballastBytes-written));}finally{closeSync(fd);}
  renameSync(temporary,path);return "created";
 }catch{if(temporary)try{rmSync(temporary,{force:true});}catch{}return "skipped";}
}

/** 🚪️ Takes a claim like `tryClaimV1`, but on a full image gives the ballast back and tries once more, so the collector that has to free space is not locked out by the lack of it. */
export function tryClaimWithBallastV1(directory:string,name:string,claim:(directory:string,name:string)=>ClaimV1|undefined=tryClaimV1):ClaimV1|undefined{
 try{return claim(directory,name);}
 catch(error){if((error as NodeJS.ErrnoException).code!=="ENOSPC"||!releaseBallastV1(dirname(directory)))throw error;return claim(directory,name);}
}
