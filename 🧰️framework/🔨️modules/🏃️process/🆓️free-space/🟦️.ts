import{lstatSync,statfsSync}from"node:fs";
import{dirname,resolve}from"node:path";

/** 🆓️ The free space of the volume that holds a path: the bytes available to the caller and, where the host reports it, the size of the volume (a Windows host path reports the free bytes only). */
export type FreeSpaceV1=Readonly<{availableBytes:number;totalBytes:number|undefined}>;
/** 🧮️ The three counters of a statfs row that the judgement needs. */
export type FreeSpaceRowV1=Readonly<{bsize:number|bigint;blocks:number|bigint;bavail:number|bigint}>;

/** 🧪️ Judges one statfs row: `undefined` ("unknown") unless the block size is at least one and the available blocks are a finite number that is not negative; the volume size is reported only when the row holds blocks and no fewer than the available ones. */
export function freeSpaceOfRowV1(row:FreeSpaceRowV1):FreeSpaceV1|undefined{
 const bsize=Number(row.bsize),blocks=Number(row.blocks),bavail=Number(row.bavail),available=bavail*bsize;
 if(!(bsize>=1)||!(bavail>=0)||!Number.isFinite(available))return undefined;
 return Object.freeze({availableBytes:available,totalBytes:blocks>0&&blocks>=bavail&&Number.isFinite(blocks*bsize)?blocks*bsize:undefined});
}

/** 🏷️ The one free-space question of the build system, answered as "unknown" (`undefined`) whenever the volume cannot be measured: the reader throws, its row is incoherent, or the path is missing (with `nearest`, the nearest existing ancestor answers instead, on every platform). Never a made-up number: every caller decides what unknown means for it (refuse, report it, treat it as pressure). The reader returns a statfs row or an already judged answer; the default reader is the host's `statfs`. @see https://nodejs.org/api/fs.html#fsstatfssyncpath-options */
export function freeSpaceSyncV1(path:string,options?:{nearest?:boolean;read?:(path:string)=>FreeSpaceRowV1|FreeSpaceV1}):FreeSpaceV1|undefined{
 const read=options?.read??((target:string):FreeSpaceRowV1=>statfsSync(target));
 for(let current=resolve(path);;current=dirname(current)){
  if(options?.nearest===true){try{lstatSync(current);}catch{if(dirname(current)===current)return undefined;continue;}}
  try{
   const value=read(current);
   if("availableBytes" in value){const bytes=value.availableBytes;return Number.isFinite(bytes)&&bytes>=0?value:undefined;}
   return freeSpaceOfRowV1(value);
  }catch{return undefined;}
 }
}
