import{lstatSync,statfsSync}from"node:fs";
import{dirname,resolve}from"node:path";
import{nativeWindowsSystemHostPortV1}from"../📦️artifacts/🏗️native-build/💾️storage/📁️directory/🪟️host/🟦️.ts";

/** 🆓️ The free space of the volume that holds a path: the bytes available to the caller and, where the host reports it, the size of the volume (on Windows the calling user’s available capacity, which may be quota-limited). */
export type FreeSpaceV1=Readonly<{availableBytes:number;totalBytes:number|undefined}>;
/** 🧮️ The counters of a statfs row that the judgement needs; `files` and `ffree` are the inode table (a filesystem without one reports `files` 0). */
export type FreeSpaceRowV1=Readonly<{bsize:number|bigint;blocks:number|bigint;bavail:number|bigint;files?:number|bigint;ffree?:number|bigint}>;
/** 🕳️ Why a volume could not be measured: the reader failed (`unreadable`), the path is not there (`missing`), the row has no usable block size or available count (`incoherent-row`), the row reports nothing at all, as a runtime without `statfs` on a host does (`empty-row`), or the Windows call failed or answered nonsense (`host-call`). */
export type FreeSpaceUnknownReasonV1="unreadable"|"missing"|"incoherent-row"|"empty-row"|"host-call";
/** 🔎️ What a measurement of a volume says: the bytes available to the caller, the size of the volume when the host reports it, the free inodes (`null` on a filesystem without an inode table, as NTFS, btrfs or a network share, never 0) and where the number came from; or that it is unknown, with the reason and a short detail. Unknown is never a number: a caller decides what it means (refuse, warn, treat as pressure). */
export type FreeSpaceFactsV1=Readonly<{known:true;availableBytes:number;totalBytes:number|undefined;freeInodes:number|null;source:"statfs"|"win32-host"|"reader"}>|Readonly<{known:false;reason:FreeSpaceUnknownReasonV1;detail:string}>;

/** 🎛️ How a measurement is made: `nearest` lets the nearest existing ancestor answer for a path that is not there yet, `platform` stands for the host (default: this process), `read` replaces the reader (a statfs row or an already judged answer), `windowsAvailableBytes` replaces the Windows volume call with caller bytes or both caller counters. */
export type FreeSpaceOptionsV1=Readonly<{nearest?:boolean;platform?:string;read?:(path:string)=>FreeSpaceRowV1|FreeSpaceV1;windowsAvailableBytes?:(path:string)=>number|FreeSpaceV1}>;

const totalFor=(available:number,total:number|undefined):number|undefined=>total!==undefined&&Number.isFinite(total)&&total>0&&total>=available?total:undefined;
const unknown=(reason:FreeSpaceUnknownReasonV1,detail:string):FreeSpaceFactsV1=>Object.freeze({known:false as const,reason,detail});
const count=(value:number|bigint|undefined):number|undefined=>{if(value===undefined)return undefined;const number=Number(value);return Number.isFinite(number)&&number>=0?number:undefined;};

/** 🧪️ Judges one statfs row: unknown unless the block size is at least one, the available blocks are a finite number that is not negative and the row reports something (a row of zero blocks and zero available blocks is a runtime that did not fill it in, not a full volume: a full volume still has blocks); the volume size is reported only when the row holds blocks and no fewer than the available ones; the inodes only when the row holds an inode table and a coherent free count. @see https://man7.org/linux/man-pages/man2/statfs.2.html */
export function judgeFreeSpaceRowV1(row:FreeSpaceRowV1):FreeSpaceFactsV1{
 const bsize=Number(row.bsize),blocks=Number(row.blocks),bavail=Number(row.bavail),available=bavail*bsize;
 if(!(bsize>=1)||!(bavail>=0)||!Number.isFinite(available))return unknown("incoherent-row",`bsize ${String(row.bsize)}, blocks ${String(row.blocks)}, bavail ${String(row.bavail)}`);
 if(!(blocks>0)&&bavail===0)return unknown("empty-row",`bsize ${String(row.bsize)}, blocks ${String(row.blocks)}, bavail 0`);
 const files=count(row.files),ffree=count(row.ffree);
 return Object.freeze({known:true as const,availableBytes:available,totalBytes:blocks>0&&blocks>=bavail&&Number.isFinite(blocks*bsize)?blocks*bsize:undefined,freeInodes:files!==undefined&&files>0&&ffree!==undefined&&ffree<=files?ffree:null,source:"statfs" as const});
}

/** 🧪️ Judges one statfs row as the byte answer alone: `undefined` ("unknown") where `judgeFreeSpaceRowV1` is unknown. */
export function freeSpaceOfRowV1(row:FreeSpaceRowV1):FreeSpaceV1|undefined{
 const facts=judgeFreeSpaceRowV1(row);
 return facts.known?Object.freeze({availableBytes:facts.availableBytes,totalBytes:facts.totalBytes}):undefined;
}

/** 🪟️ Reads both calling-user capacity counters through the native Windows volume port. @see https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getdiskfreespaceexw */
export function windowsFreeSpaceV1(path:string):FreeSpaceV1{
 const result=nativeWindowsSystemHostPortV1().availableBytes(path);
 if(!result.ok)throw Error("Host free space could not be read (Win32 error "+result.error+")");
 return Object.freeze({availableBytes:result.bytes,totalBytes:totalFor(result.bytes,result.totalBytes)});
}

/** 📏️ Reads caller-available bytes for consumers that do not need a capacity ratio. */
export function windowsAvailableBytesV1(path:string):number{return windowsFreeSpaceV1(path).availableBytes;}

/** 🏷️ The one free-space question of the build system, answered as typed facts: the bytes (and, where the filesystem has an inode table, the free inodes) or unknown with the reason. Never a made-up number: the reader throws, its row is incoherent or empty, or the path is missing (with `nearest`, the nearest existing ancestor answers instead, on every platform). On Windows the volume is asked for both caller capacity counters (`GetDiskFreeSpaceExW`) and the facts carry no inodes; elsewhere the host's `statfs` answers. `read` replaces the reader (a statfs row or an already judged answer). @see https://nodejs.org/api/fs.html#fsstatfssyncpath-options */
export function readFreeSpaceV1(path:string,options?:FreeSpaceOptionsV1):FreeSpaceFactsV1{
 const windows=options?.read===undefined&&(options?.platform??process.platform)==="win32";
 for(let current=resolve(path);;current=dirname(current)){
  if(options?.nearest===true){try{lstatSync(current);}catch{if(dirname(current)===current)return unknown("missing",path);continue;}}
  try{
   if(windows){
    const observed=(options?.windowsAvailableBytes??windowsFreeSpaceV1)(current);
    const bytes=typeof observed==="number"?observed:observed.availableBytes,total=typeof observed==="number"?undefined:observed.totalBytes;
    return Number.isFinite(bytes)&&bytes>=0?Object.freeze({known:true as const,availableBytes:bytes,totalBytes:totalFor(bytes,total),freeInodes:null,source:"win32-host" as const}):unknown("host-call",`answered ${String(bytes)}`);
   }
   const value=(options?.read??((target:string):FreeSpaceRowV1=>statfsSync(target)))(current);
   if("availableBytes" in value){const bytes=value.availableBytes;return Number.isFinite(bytes)&&bytes>=0?Object.freeze({known:true as const,availableBytes:bytes,totalBytes:value.totalBytes,freeInodes:null,source:"reader" as const}):unknown("incoherent-row",`availableBytes ${String(bytes)}`);}
   return judgeFreeSpaceRowV1(value);
  }catch(error){return unknown(windows?"host-call":"unreadable",String((error as Error)?.message??error).slice(0,200));}
 }
}

/** 🏷️ The byte answer of `readFreeSpaceV1`: `undefined` ("unknown") whenever the volume cannot be measured. */
export function freeSpaceSyncV1(path:string,options?:FreeSpaceOptionsV1):FreeSpaceV1|undefined{
 const facts=readFreeSpaceV1(path,options);
 return facts.known?Object.freeze({availableBytes:facts.availableBytes,totalBytes:facts.totalBytes}):undefined;
}
