import {randomUUID} from "node:crypto";
import {copyFile,chmod,lstat,readdir,readFile,rename,unlink,utimes} from "node:fs/promises";
import {join} from "node:path";

/** 🔒️ The entries of a Bun package cache that hold the pristine files of a patched package (`name@version` keys of `patchedDependencies`): Bun copies such an entry and writes the patch into the copy, so its files must stay writable and unshared. Entries Bun already patched (`_patch_hash=`) are final and not listed. */
export async function patchedBunCacheEntriesV1(cache:string,patched:Iterable<string>):Promise<readonly string[]>{
 const prefixes=[...patched].map(key=>key+"@@");
 let names:string[];try{names=await readdir(cache);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return[];throw error;}
 return Object.freeze(names.filter(name=>!name.includes("_patch_hash=")&&prefixes.some(prefix=>name.startsWith(prefix))).sort().map(name=>join(cache,name)));
}

/** 🔒️ The `patchedDependencies` keys that a `package.json` text declares. */
export const patchedDependencyKeysV1=(text:string|Uint8Array):readonly string[]=>{
 try{const value=JSON.parse(Buffer.from(text).toString("utf8")) as {patchedDependencies?:Record<string,unknown>};return Object.keys(value?.patchedDependencies??{});}catch{return[];}
};

/** 🔒️ The `patchedDependencies` keys of a workspace: its root manifest and every installed generation manifest below `generations`. */
export async function patchedDependencyKeysOfWorkspaceV1(workspace:string,generations:string):Promise<ReadonlySet<string>>{
 const keys=new Set<string>(),read=async(path:string)=>{try{for(const key of patchedDependencyKeysV1(await readFile(path)))keys.add(key);}catch{}};
 await read(join(workspace,"package.json"));
 let names:string[]=[];try{names=await readdir(generations);}catch{}
 for(const name of names)await read(join(generations,name,"package.json"));
 return keys;
}

/** 🔒️ Replaces every read-only or shared file below the directories by a writable private copy of the same bytes, mode and times, so a writer can patch the entry without touching the shared store object. Returns the number of files replaced. */
export async function privatizeCasEntriesV1(directories:readonly string[],signal?:AbortSignal):Promise<number>{
 let replaced=0;
 const walk=async(directory:string):Promise<void>=>{
  for(const entry of await readdir(directory,{withFileTypes:true})){
   signal?.throwIfAborted();const path=join(directory,entry.name);
   if(entry.isDirectory()){await walk(path);continue;}
   if(!entry.isFile())continue;
   const info=await lstat(path);
   if(info.nlink<2&&(info.mode&0o200)!==0)continue;
   const temporary=join(directory,`.private-${process.pid}-${randomUUID().slice(0,8)}`);
   await copyFile(path,temporary);await chmod(temporary,(info.mode&0o7777)|0o200);await utimes(temporary,info.atime,info.mtime);
   try{await unlink(path);await rename(temporary,path);}catch(error){await unlink(temporary).catch(()=>undefined);throw error;}
   replaced++;
  }
 };
 for(const directory of directories)try{await walk(directory);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
 return replaced;
}
