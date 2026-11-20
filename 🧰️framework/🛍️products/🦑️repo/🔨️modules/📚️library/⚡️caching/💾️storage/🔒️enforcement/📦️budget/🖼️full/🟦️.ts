import{freeSpaceSyncV1}from"../../../../../../../../../🔨️modules/🏃️process/🆓️free-space/🟦️.ts";
import{execFileSync}from"node:child_process";
import{statSync,statfsSync}from"node:fs";
import{join,resolve,sep}from"node:path";
import policy from"../🔣️policy.json"with{type:"json"};

/** 🖼️ A cache image that cannot take a write: its partition id, directory, size, bytes used and free inside it, the free bytes of the host volume below it and what fills it most. */
export type FullImageV1=Readonly<{id:string;directory:string;sizeBytes:number;usedBytes:number;freeBytes:number;hostFreeBytes:number|undefined;filledBy:string}>;

const FREE_BYTES=64*1048576,FREE_PERCENT=2;

/** 📊️ The largest subtrees of a mounted image as `name (size)` lines, measured with `du` two levels deep; empty where the host has no `du` or it takes too long. */
export function imageFilledByV1(mount:string,count:number=3):string{
 try{
  const rows=execFileSync("du",["-k","-d","2",mount],{encoding:"utf8",timeout:30000,maxBuffer:1<<24,stdio:["ignore","pipe","ignore"]}).split("\n").map(line=>/^(\d+)\t(.+)$/u.exec(line)).filter((row):row is RegExpExecArray=>row!==null).map(row=>({kb:Number(row[1]),name:row[2]!.slice(mount.length+1)})).filter(row=>row.name.includes(sep)||row.name.includes("/")).sort((a,b)=>b.kb-a.kb).slice(0,count);
  return rows.map(row=>row.name+" ("+(row.kb/1048576).toFixed(1)+" GiB)").join(", ");
 }catch{return"";}
}

/** 🔎️ The cache image of a worktree that a failed write most likely met: the one whose mount holds a path of the error, else the one with less than 64 MiB or 2 percent free (the fullest first); `undefined` when no image is full, so the failure is about the host volume. */
export function fullImageOfV1(workspace:string,options:Readonly<{paths?:readonly string[];measure?:(mount:string)=>string;space?:(path:string)=>Readonly<{sizeBytes:number;usedBytes:number;freeBytes:number}>|undefined;mounted?:(directory:string,mount:string)=>boolean}>={}):FullImageV1|undefined{
 const root=resolve(workspace),rows:FullImageV1[]=[],host=freeSpaceSyncV1(root)?.availableBytes;
 for(const[id,declared]of Object.entries(policy.partitions) as [string,{directory:string}][]){
  const directory=join(root,declared.directory),mount=join(directory,"m");
  try{
   if(!(options.mounted??((folder,at)=>statSync(at).dev!==statSync(folder).dev))(directory,mount))continue;
   const space=(options.space??(path=>{const image=statfsSync(path);return{sizeBytes:image.blocks*image.bsize,usedBytes:(image.blocks-image.bfree)*image.bsize,freeBytes:image.bavail*image.bsize};}))(mount);
   if(space)rows.push({id,directory:declared.directory,...space,hostFreeBytes:host,filledBy:mount});
  }catch{}
 }
 const named=rows.find(row=>(options.paths??[]).some(path=>path===row.filledBy||path.startsWith(row.filledBy+sep))),pressed=rows.filter(row=>row.freeBytes<Math.max(FREE_BYTES,row.sizeBytes*FREE_PERCENT/100)).sort((a,b)=>a.freeBytes/a.sizeBytes-b.freeBytes/b.sizeBytes)[0],found=named??pressed;
 return found&&Object.freeze({...found,filledBy:(options.measure??imageFilledByV1)(found.filledBy)});
}
