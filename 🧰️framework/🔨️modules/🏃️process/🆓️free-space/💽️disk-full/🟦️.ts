import{lstatSync}from"node:fs";
import{dirname,parse,resolve}from"node:path";
import{freeSpaceSyncV1,type FreeSpaceV1}from"../🟦️.ts";

/** 💽️ The partition a full-disk failure points at: the mount point of its volume and what the volume still holds (`undefined` where the host cannot say). */
export type DiskFullV1=Readonly<{partition:string;freeBytes:number|undefined;totalBytes:number|undefined}>;

const ERRNO=new Set(["ENOSPC","EDQUOT"]);
const TEXT=/ENOSPC|EDQUOT|no space left on device|database or disk is full|SQLITE_FULL|disk quota exceeded/iu;
const DEPTH=4;

/** 🔎️ Whether an error, or one of its causes, says that the disk is full: the errno codes `ENOSPC` and `EDQUOT`, or the text of a runtime or database that lost the code (`database or disk is full`, `SQLITE_FULL`, `no space left on device`). @see https://man7.org/linux/man-pages/man3/errno.3.html */
export function isDiskFullV1(error:unknown,depth:number=DEPTH):boolean{
 if(error===null||error===undefined||depth<0)return false;
 if(typeof error==="string")return TEXT.test(error);
 if(typeof error!=="object")return false;
 const row=error as{code?:unknown;errno?:unknown;message?:unknown;cause?:unknown};
 if(typeof row.code==="string"&&(ERRNO.has(row.code)||TEXT.test(row.code)))return true;
 if(typeof row.message==="string"&&TEXT.test(row.message))return true;
 return isDiskFullV1(row.cause,depth-1);
}

/** 📍️ The mount point of the volume that holds a path: the highest ancestor on the same device as the nearest existing part of the path (the drive root on Windows, which has no device numbers). */
export function volumeMountOfV1(path:string):string{
 let current=resolve(path);
 if(process.platform==="win32")return parse(current).root;
 let device:number|bigint|undefined;
 for(;;){try{device=lstatSync(current).dev;break;}catch{const parent=dirname(current);if(parent===current)return current;current=parent;}}
 for(;;){
  const parent=dirname(current);if(parent===current)return current;
  try{if(lstatSync(parent).dev!==device)return current;}catch{return current;}
  current=parent;
 }
}

const pathsOf=(error:unknown,depth:number=DEPTH):string[]=>{
 if(typeof error!=="object"||error===null||depth<0)return[];
 const row=error as{path?:unknown;dest?:unknown;cause?:unknown};
 return[...[row.path,row.dest].filter((value):value is string=>typeof value==="string"&&value.length>0),...pathsOf(row.cause,depth-1)];
};

/** 💽️ Names the partition of a full-disk failure: the volume of the path the error carries (else of the first probe, by default the working directory) and the free bytes it holds now; `undefined` when the error is not a full-disk failure. The partition is named even when its free space cannot be read. */
export function diskFullOfV1(error:unknown,options:Readonly<{probes?:readonly string[];free?:(path:string)=>FreeSpaceV1|undefined;mount?:(path:string)=>string}>={}):DiskFullV1|undefined{
 if(!isDiskFullV1(error))return undefined;
 const free=options.free??(path=>freeSpaceSyncV1(path,{nearest:true})),mount=options.mount??volumeMountOfV1,target=[...pathsOf(error),...(options.probes??[process.cwd()])][0]??process.cwd(),space=free(target);
 return Object.freeze({partition:mount(target),freeBytes:space?.availableBytes,totalBytes:space?.totalBytes});
}
