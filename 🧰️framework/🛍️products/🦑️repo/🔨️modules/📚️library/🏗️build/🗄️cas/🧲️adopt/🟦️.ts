import {createHash,randomUUID} from "node:crypto";
import {constants} from "node:fs";
import {chmod,copyFile,link,lstat,mkdir,open,readdir,rename,unlink,utimes} from "node:fs/promises";
import {dirname,join} from "node:path";
import {touchCasObjectSyncV1} from "../🗃️store/🔁️sync/🟦️.ts";
import {casObjectPathV1} from "../🗃️store/🟦️.ts";
import {CAS_ADOPTION_V1,CAS_POLICY_V1,type CasAdoptionPolicyV1,type CasPlaceV1} from "../🧬️schema/🟦️.ts";

/** 🧲️ A place to adopt: a directory with the id its report carries. */
export type CasAdoptPlaceV1=Readonly<{id:string;directory:string;mode:"hardlink"|"reflink"}>;
/** 🧲️ What one adoption run needs. */
export type CasAdoptOptionsV1=Readonly<{
 root:string;places:readonly CasAdoptPlaceV1[];leave?:readonly string[];policy?:CasAdoptionPolicyV1;dryRun?:boolean;signal?:AbortSignal;now?():number;
 progress?(event:Readonly<{entries:number;savedBytes:number}>):void|Promise<void>;
}>;
/** 🧾️ What an adoption run did: files now sharing an object, new objects created, bytes released, files left alone by reason. */
export type CasAdoptionV1=Readonly<{partial:boolean;dryRun:boolean;scanned:number;linked:number;objects:number;savedBytes:number;failed:number;skipped:Readonly<Record<"young"|"small"|"volatile"|"linked"|"device"|"mode"|"changed"|"clone",number>>}>;

const sorted=(names:string[]):string[]=>names.sort((left,right)=>left<right?-1:left>right?1:0);

async function digest(path:string,signal?:AbortSignal):Promise<string|undefined>{
 let handle;try{handle=await open(path,constants.O_RDONLY|(constants.O_NOATIME??0)).catch(()=>open(path,"r"));}catch{return undefined;}
 try{const hash=createHash("sha256"),buffer=Buffer.allocUnsafe(1<<20);for(;;){signal?.throwIfAborted();const{bytesRead}=await handle.read(buffer,0,buffer.length,null);if(!bytesRead)break;hash.update(buffer.subarray(0,bytesRead));}return hash.digest("hex");}
 catch(error){if(signal?.aborted)throw error;return undefined;}
 finally{await handle.close().catch(()=>undefined);}
}

async function* walk(directory:string,left:ReadonlySet<string>,signal?:AbortSignal):AsyncGenerator<string>{
 let names:string[];try{names=sorted(await readdir(directory));}catch{return;}
 for(const name of names){
  signal?.throwIfAborted();const path=join(directory,name);let info;
  try{info=await lstat(path);}catch{continue;}
  if(info.isDirectory()){if(!left.has(path))yield*walk(path,left,signal);}else if(info.isFile())yield path;
 }
}

/** 🧲️ Makes the finished files of the declared places share one host object per content: the first file with a content joins the store as a hard link (its write bits cleared), every later file with equal bytes, mode and size is replaced by a hard link of that object, atomically (a temporary link renamed over the file after the file was seen unchanged). A file on another volume than the store, still being written, small, volatile, executable differently from its object or already shared stays as it is. Hashing never touches access times, the run is cancellable and reports progress, and the store collector treats an object that workspaces still link as in use. */
export async function adoptCasV1(options:CasAdoptOptionsV1):Promise<CasAdoptionV1>{
 const policy=options.policy??CAS_ADOPTION_V1,now=options.now??Date.now,skip=new RegExp(policy.skipPattern,"u"),fan=CAS_POLICY_V1.fanOutHexLength,signal=options.signal;
 const skipped={young:0,small:0,volatile:0,linked:0,device:0,mode:0,changed:0,clone:0};const planned=new Set<string>();let scanned=0,linked=0,objects=0,saved=0,failed=0,partial=false,last=-Infinity,counter=0;
 await mkdir(join(options.root,"v1","o"),{recursive:true});const storeDevice=(await lstat(join(options.root,"v1","o"))).dev;
 const report=async():Promise<void>=>{signal?.throwIfAborted();const at=now();if(at-last<CAS_POLICY_V1.progressMs)return;last=at;await options.progress?.({entries:scanned,savedBytes:saved});signal?.throwIfAborted();};
 const unchanged=async(path:string,stamp:{ino:number;mtimeMs:number;size:number}):Promise<boolean>=>{const again=await lstat(path).catch(()=>undefined);return again!==undefined&&again.isFile()&&again.ino===stamp.ino&&again.mtimeMs===stamp.mtimeMs&&again.size===stamp.size;};
 try{
  const roots=new Set([...options.places.map(place=>place.directory),...(options.leave??[])]),cloneable=new Map<string,boolean>();
  for(const place of options.places)for await(const path of walk(place.directory,new Set([...roots].filter(root=>root!==place.directory)),signal)){
   scanned++;if(scanned%CAS_POLICY_V1.checkEvery===0)await report();
   const info=await lstat(path).catch(()=>undefined);if(!info?.isFile())continue;
   const name=path.slice(path.lastIndexOf("/")+1);
   if(skip.test(name)){skipped.volatile++;continue;}
   if(info.size<policy.minBytes){skipped.small++;continue;}
   if(now()-info.mtimeMs<policy.quietMs){skipped.young++;continue;}
   if(info.nlink>1){skipped.linked++;continue;}
   if(info.dev!==storeDevice){skipped.device++;continue;}
   if(place.mode==="reflink"){
    let can=cloneable.get(place.id);
    if(can===undefined){const probe=join(options.root,"v1","t",`.clone-${process.pid}`);can=await (async()=>{try{await mkdir(dirname(probe),{recursive:true});await copyFile(path,probe,constants.COPYFILE_FICLONE_FORCE);return true;}catch{return false;}finally{await unlink(probe).catch(()=>undefined);}})();cloneable.set(place.id,can);}
    if(!can){skipped.clone++;continue;}
   }
   const hash=await digest(path,signal);if(hash===undefined)continue;
   const object=casObjectPathV1(options.root,hash,fan),stamp={ino:info.ino,mtimeMs:info.mtimeMs,size:info.size},mode=(info.mode&0o7777)&(policy.clearWriteBits?~0o222:0o7777);
   if(place.mode==="reflink"){
    try{
     let existing=await lstat(object).catch(()=>undefined);
     if(!existing){
      if(options.dryRun){if(planned.has(hash)){linked++;saved+=info.size;}else{planned.add(hash);linked++;objects++;}continue;}
      if(!await unchanged(path,stamp)){skipped.changed++;continue;}
      const seed=join(options.root,"v1","t",`${process.pid}-${randomUUID().slice(0,12)}`);
      await mkdir(dirname(object),{recursive:true});await copyFile(path,seed,constants.COPYFILE_FICLONE_FORCE);await chmod(seed,0o444);
      try{await rename(seed,object);objects++;}catch(error){await unlink(seed).catch(()=>undefined);if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;}
      existing=await lstat(object).catch(()=>undefined);
     }
     if(!existing)continue;
     if(options.dryRun){linked++;saved+=info.size;continue;}
     if(existing.size!==info.size){skipped.mode++;continue;}
     const temporary=join(dirname(path),`.adopt-${process.pid}-${randomUUID().slice(0,8)}`);
     await copyFile(object,temporary,constants.COPYFILE_FICLONE_FORCE);await chmod(temporary,info.mode&0o7777);await utimes(temporary,info.atime,info.mtime);
     if(!await unchanged(path,stamp)){await unlink(temporary).catch(()=>undefined);skipped.changed++;continue;}
     try{await rename(temporary,path);}catch(error){await unlink(temporary).catch(()=>undefined);throw error;}
     linked++;saved+=info.size;touchCasObjectSyncV1(options.root,hash,now());
    }catch(error){if(signal?.aborted)throw error;failed++;}
    continue;
   }
   try{
    let existing=await lstat(object).catch(()=>undefined);
    if(!existing){
     if(options.dryRun){if(planned.has(hash)){linked++;saved+=info.size;}else{planned.add(hash);linked++;objects++;}continue;}
     if(!await unchanged(path,stamp)){skipped.changed++;continue;}
     if(policy.clearWriteBits&&mode!==(info.mode&0o7777))await chmod(path,mode);
     await mkdir(dirname(object),{recursive:true});
     try{await link(path,object);linked++;objects++;touchCasObjectSyncV1(options.root,hash,now());continue;}
     catch(error){if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;existing=await lstat(object).catch(()=>undefined);}
    }
    if(!existing)continue;
    if(existing.ino===info.ino&&existing.dev===info.dev)continue;
    if(existing.size!==info.size||(existing.mode&0o7777)!==mode||existing.dev!==info.dev){skipped.mode++;continue;}
    if(options.dryRun){linked++;saved+=info.size;continue;}
    const temporary=join(dirname(path),`.adopt-${process.pid}-${randomUUID().slice(0,8)}`);
    await link(object,temporary);
    if(!await unchanged(path,stamp)){await unlink(temporary).catch(()=>undefined);skipped.changed++;continue;}
    try{await rename(temporary,path);}catch(error){await unlink(temporary).catch(()=>undefined);throw error;}
    linked++;saved+=info.size;touchCasObjectSyncV1(options.root,hash,now());
   }catch(error){if(signal?.aborted)throw error;failed++;}
   counter++;
  }
 }catch(error){if(signal?.aborted)partial=true;else throw error;}
 return Object.freeze({partial,dryRun:options.dryRun===true,scanned,linked,objects,savedBytes:saved,failed,skipped:Object.freeze(skipped)});
}

/** 🧲️ The places of one workspace: the declared relative paths below its root. */
export const casPlacesOfWorkspaceV1=(workspace:string,places:readonly CasPlaceV1[]=CAS_ADOPTION_V1.places):readonly CasAdoptPlaceV1[]=>places.map(place=>({id:place.id,directory:join(workspace,...place.path.split("/")),mode:place.mode}));
