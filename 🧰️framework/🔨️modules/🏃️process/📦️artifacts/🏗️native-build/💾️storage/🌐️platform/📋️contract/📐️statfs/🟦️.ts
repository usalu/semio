import{lstatSync,statfsSync,type BigIntStatsFs,type StatsFs}from"node:fs";
import{statfs}from"node:fs/promises";
import{basename,dirname,join,resolve}from"node:path";
import{freeSpaceSyncV1,type FreeSpaceV1}from"../../../../../../🆓️free-space/🟦️.ts";
import{nativeStorageBackendsV1}from"../../../🔌️backends/🧭️selection/🟦️.ts";
import{measureNativeDirectoryUsageV1,nativeDirectoryEntryBoundV1,nativeDirectoryHostAvailableBytesV1,nativeDirectoryProtocolV1,nativeDirectoryUsageV1,observeNativeDirectoryStateV1,readNativeDirectoryAnchorFileV1}from"../../../📁️directory/🟦️.ts";

const protocol=nativeDirectoryProtocolV1();
/** 🧭️ One directory partition found above a path: its directory, the volume root now holding the data and its declared envelope. */
export type NativePartitionV1=Readonly<{directory:string;root:string;attached:boolean;filesystemBytes:number;maximumPhysicalBytes:number}>;
type RowV1={bsize:number;blocks:number;bfree:number;bavail:number};

/** 🔎️ Finds the directory partition that holds a path (the path lies in or at its root `m`, and `fixed.dir` sits beside it); undefined for every host path and for partitions kept as mounted images. */
export function nativePartitionOfV1(path:string,platform:string=process.platform):NativePartitionV1|undefined{
 if(!nativeStorageBackendsV1.some(row=>row.capabilities.includes("declared-envelope")&&row.platforms.includes(platform)))return undefined;
 let current=resolve(path);
 for(let depth=0;depth<=protocol.maximumDepth;depth++){
  if(basename(current)===protocol.root){
   const directory=dirname(current);let anchored=false;
   try{anchored=lstatSync(join(directory,protocol.anchor)).isFile();}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT"&&(error as NodeJS.ErrnoException).code!=="ENOTDIR")throw error;}
   if(anchored){const anchor=readNativeDirectoryAnchorFileV1(directory);let state:"attached"|"inactive"="attached";try{state=observeNativeDirectoryStateV1(directory);}catch{}return Object.freeze({directory,root:join(directory,state==="attached"?protocol.root:protocol.volume),attached:state==="attached",filesystemBytes:anchor.filesystemBytes,maximumPhysicalBytes:anchor.maximumPhysicalBytes});}
  }
  const parent=dirname(current);if(parent===current)return undefined;current=parent;
 }
 return undefined;
}

function statsFor(bigint:true,row:RowV1):BigIntStatsFs;
function statsFor(bigint:false,row:RowV1):StatsFs;
function statsFor(bigint:boolean,row:RowV1):BigIntStatsFs|StatsFs{
 const number=(value:number)=>bigint?BigInt(value):value;
 return Object.freeze({type:number(0),bsize:number(row.bsize),blocks:number(row.blocks),bfree:number(row.bfree),bavail:number(row.bavail),files:number(0),ffree:number(0)}) as BigIntStatsFs|StatsFs;
}
const rowOf=(partition:NativePartitionV1,usedBytes:number):RowV1=>{
 const bsize=protocol.usage.blockBytes,blocks=Math.floor(partition.filesystemBytes/bsize),used=Math.min(blocks,Math.ceil(usedBytes/bsize));
 return{bsize,blocks,bfree:blocks-used,bavail:blocks-used};
};
const hostRow=(path:string):RowV1=>{
 const bsize=protocol.usage.blockBytes,free=Math.floor(nativeDirectoryHostAvailableBytesV1(path)/bsize);
 return{bsize,blocks:free,bfree:free,bavail:free};
};

/** 💽️ Answers statfs for any path: the host filesystem as before, except below a directory partition, where the volume is the declared envelope and its used bytes come from the shared measured usage, never from the host disk that merely stores the directories; Windows has no statfs, so host paths read the free space of their volume. Asynchronous callers share one walk and accept a measurement of a few seconds unless they ask for a fresh one, as a before-and-after delta does. */
export function nativeStatfsV1(path:string,options?:{bigint?:false;platform?:string;fresh?:boolean}):Promise<StatsFs>;
export function nativeStatfsV1(path:string,options:{bigint:true;platform?:string;fresh?:boolean}):Promise<BigIntStatsFs>;
export async function nativeStatfsV1(path:string,options?:{bigint?:boolean;platform?:string;fresh?:boolean}):Promise<StatsFs|BigIntStatsFs>{
 const bigint=options?.bigint===true,platform=options?.platform??process.platform,partition=nativePartitionOfV1(path,platform);
 if(partition){const usage=nativeDirectoryUsageV1(partition.root,partition.maximumPhysicalBytes),bytes=options?.fresh===true?await usage.fresh():await usage.recent(protocol.usage.recentMilliseconds);return bigint?statsFor(true,rowOf(partition,bytes)):statsFor(false,rowOf(partition,bytes));}
 if(platform==="win32")return bigint?statsFor(true,hostRow(path)):statsFor(false,hostRow(path));
 return bigint?statfs(path,{bigint:true}):statfs(path);
}

/** 🧲️ Synchronous statfs with the same meaning: below a directory partition it serves the last measurement of the shared usage cell, refreshes an old one in the background and walks the tree synchronously only when none is younger than a minute. */
export function nativeStatfsSyncV1(path:string,options?:{bigint?:false;platform?:string}):StatsFs;
export function nativeStatfsSyncV1(path:string,options:{bigint:true;platform?:string}):BigIntStatsFs;
export function nativeStatfsSyncV1(path:string,options?:{bigint?:boolean;platform?:string}):StatsFs|BigIntStatsFs{
 const bigint=options?.bigint===true,platform=options?.platform??process.platform,partition=nativePartitionOfV1(path,platform);
 if(partition){const bytes=nativeDirectoryUsedBytesSyncV1(partition.root,partition.maximumPhysicalBytes);return bigint?statsFor(true,rowOf(partition,bytes)):statsFor(false,rowOf(partition,bytes));}
 if(platform==="win32")return bigint?statsFor(true,hostRow(path)):statsFor(false,hostRow(path));
 return bigint?statfsSync(path,{bigint:true}):statfsSync(path);
}

/** 🧮️ Reports the bytes in use below a partition volume without awaiting: a measurement younger than the recent bound as it is; an older one up to the synchronous bound as it is, while one shared asynchronous walk refreshes it in the background; one bounded synchronous walk only when nothing younger than that bound exists. */
export function nativeDirectoryUsedBytesSyncV1(usedRoot:string,maximumPhysicalBytes:number,now:()=>number=Date.now):number{
 const cell=nativeDirectoryUsageV1(usedRoot,maximumPhysicalBytes),known=cell.peek(),age=known?now()-known.atMs:Infinity;
 if(known&&age<=protocol.usage.recentMilliseconds)return known.bytes;
 if(known&&age<=protocol.usage.synchronousMaximumAgeMilliseconds){cell.fresh().catch(()=>undefined);return known.bytes;}
 const bytes=measureNativeDirectoryUsageV1(usedRoot,nativeDirectoryEntryBoundV1(maximumPhysicalBytes)).allocatedBytes;cell.record(bytes,now());return bytes;
}

/** 🆓️ The free space of the volume that holds a path, below a directory partition the declared envelope. @see ../../../../../../🆓️free-space/🟦️.ts */
export type NativeFreeSpaceV1=FreeSpaceV1;

/** 🏷️ The partition-aware and Windows-aware free-space question: the one answer of the library, "unknown" (`undefined`) whenever the volume cannot be measured. Below a directory partition the volume is the declared envelope, a Windows host path asks the volume for its free bytes (`availableBytes` is the injection point of that call), every other path is the host's statfs. @see nativeStatfsSyncV1 */
export function nativeFreeSpaceSyncV1(path:string,options?:{platform?:string;nearest?:boolean;availableBytes?:(path:string)=>number}):NativeFreeSpaceV1|undefined{
 const platform=options?.platform??process.platform;
 return freeSpaceSyncV1(path,{nearest:options?.nearest,read:target=>platform==="win32"&&!nativePartitionOfV1(target,platform)?Object.freeze({availableBytes:(options?.availableBytes??nativeDirectoryHostAvailableBytesV1)(target),totalBytes:undefined}):nativeStatfsSyncV1(target,{platform})});
}
