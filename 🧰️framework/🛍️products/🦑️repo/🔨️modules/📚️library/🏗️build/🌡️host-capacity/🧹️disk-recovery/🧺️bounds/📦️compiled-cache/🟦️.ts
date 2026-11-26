import {lstat,readdir,unlink} from "node:fs/promises";
import {join} from "node:path";
import {DISK_BOUNDS_V1,type DiskBoundsV1} from "../🟦️.ts";

/** 📄️ One file of the compiled Wasmtime cache: its path, bytes, when it was last used (modification time, which the plugin host refreshes on a cache hit) and whether it is a writer's staging file. */
export type CompiledCacheEntryV1=Readonly<{path:string;bytes:number;usedMs:number;scratch:boolean}>;

/** ⚖️ Chooses what to retire from the compiled cache, as the same rule the plugin host applies at engine start: staging files past their grace, complete files past the age limit, then the least recently used complete files (ties by path) until the rest fits the cap; a complete file used within the minimum idle time is never chosen, whatever the cap says. Pure; the Rust selector of the engine and this one judge the same fixture.
 * @see `🔧️engine/🧹️cache-retirement/🧬️schema/🔣️.json` of the plugin host */
export function selectCompiledCacheV1(entries:readonly CompiledCacheEntryV1[],now:number,bound:DiskBoundsV1["compiledCache"]=DISK_BOUNDS_V1.compiledCache):readonly string[]{
 const retired=new Set<string>(),candidates:CompiledCacheEntryV1[]=[],idle=(entry:CompiledCacheEntryV1):number=>Math.max(0,now-entry.usedMs);
 let kept=0;
 for(const entry of entries){
  if(entry.scratch){if(idle(entry)>bound.scratchGraceMilliseconds)retired.add(entry.path);continue;}
  if(idle(entry)>=bound.minimumIdleMilliseconds&&idle(entry)>bound.maximumAgeMilliseconds){retired.add(entry.path);continue;}
  kept+=entry.bytes;
  if(idle(entry)>=bound.minimumIdleMilliseconds)candidates.push(entry);
 }
 candidates.sort((first,second)=>idle(second)-idle(first)||(first.path<second.path?-1:1));
 for(const entry of candidates){if(kept<=bound.maximumBytes)break;kept-=entry.bytes;retired.add(entry.path);}
 return [...retired].sort();
}

/** 🔭️ Lists the `.cwasm` and `.scratch` files two levels below the cache root (`<engine-config-hash>/<file>`), at most `limit` of them; the cold compile lock and the retirement stamp are not the retirement's. */
export async function scanCompiledCacheV1(root:string,limit:number,signal?:AbortSignal):Promise<readonly CompiledCacheEntryV1[]>{
 const entries:CompiledCacheEntryV1[]=[];
 for(const group of await readdir(root).catch(()=>[] as string[])){
  const directory=join(root,group),info=await lstat(directory).catch(()=>undefined);
  if(info===undefined||!info.isDirectory())continue;
  for(const name of await readdir(directory).catch(()=>[] as string[])){
   signal?.throwIfAborted();
   const scratch=name.endsWith(".scratch");
   if(!scratch&&!name.endsWith(".cwasm"))continue;
   if(entries.length>=limit)return entries;
   const path=join(directory,name),file=await lstat(path).catch(()=>undefined);
   if(file!==undefined&&file.isFile())entries.push({path,bytes:file.size,usedMs:file.mtimeMs,scratch});
  }
 }
 return entries;
}

/** 🧹️ Retires the selected files of one cache root by single `unlink` calls (a refusal leaves the file for the next run; a missing root yields nothing and is never created); answers the bytes removed, or that would be in a dry run. */
export async function retireCompiledCacheV1(root:string,options:Readonly<{now:number;dryRun?:boolean;signal?:AbortSignal;limit:number;bound?:DiskBoundsV1["compiledCache"]}>):Promise<Readonly<{files:number;bytes:number}>>{
 const entries=await scanCompiledCacheV1(root,options.limit,options.signal),chosen=new Set(selectCompiledCacheV1(entries,options.now,options.bound));
 let files=0,bytes=0;
 for(const entry of entries){
  if(!chosen.has(entry.path))continue;
  options.signal?.throwIfAborted();
  if(options.dryRun!==true&&!await unlink(entry.path).then(()=>true,error=>(error as NodeJS.ErrnoException).code==="ENOENT"))continue;
  files++;bytes+=entry.bytes;
 }
 return{files,bytes};
}
