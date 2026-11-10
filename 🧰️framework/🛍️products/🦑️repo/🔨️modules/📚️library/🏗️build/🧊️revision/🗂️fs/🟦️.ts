import {createHash} from "node:crypto";
import * as nodeFs from "node:fs";
import {isAbsolute,join,relative,resolve,sep} from "node:path";
import type {PinnedWorldSyncV1} from "../🔓️view/🟦️.ts";

/** 🗂️ The read half of `node:fs` that observed inference reads through. */
export type ReadFsV1=Pick<typeof nodeFs,"statSync"|"lstatSync"|"readFileSync"|"readdirSync"|"realpathSync"|"existsSync">;

const sizeOf=(world:PinnedWorldSyncV1)=>{const sizes=new Map(world.manifest.entries.map(entry=>[entry.path,entry.size]));return (path:string):number=>sizes.get(path)??0;};
const inodeOf=(path:string):number=>Number(BigInt("0x"+createHash("sha256").update(path).digest("hex").slice(0,12)));
const fail=(code:string,path:string):never=>{throw Object.assign(new Error(`${code}: no such file or directory, '${path}'`),{code,errno:-2,syscall:"stat",path});};

/** 🗂️ A `node:fs` read port over a pinned revision: paths below `workspaceRoot` answer from the manifest and store objects (symlinks resolve inside the revision, times are zero, inodes derive from the path), every other path from `fallback`, so one inference run sees one frozen workspace and the unchanging tool and dependency trees on disk. */
export function createPinnedFsV1(options:{world:PinnedWorldSyncV1;workspaceRoot:string;fallback?:ReadFsV1}):ReadFsV1{
 const {world}=options,fallback=options.fallback??nodeFs,root=resolve(options.workspaceRoot),size=sizeOf(world);
 const inside=(path:string):string|undefined=>{
  const absolute=resolve(isAbsolute(path)?path:join(process.cwd(),path)),inner=relative(root,absolute);
  return inner===""?"":inner.startsWith("..")||isAbsolute(inner)?undefined:inner.split(sep).join("/");
 };
 const resolveLink=(inner:string,depth=0):string=>{
  if(depth>40)return fail("ELOOP",inner);
  const parts=inner===""?[]:inner.split("/");
  for(let index=0;index<parts.length;index++){
   const prefix=parts.slice(0,index+1).join("/");
   if(world.kindSync(prefix)==="symlink"){
    const target=Buffer.from(world.readSync(prefix)!).toString("utf8"),base=parts.slice(0,index).join("/"),joined=resolve("/"+base,target).slice(1).split(sep).join("/");
    return resolveLink([joined,...parts.slice(index+1)].filter(part=>part!=="").join("/"),depth+1);
   }
  }
  return inner;
 };
 const stats=(inner:string,kind:string,bigint:boolean)=>{
  const directory=kind==="directory",link=kind==="symlink",file=!directory&&!link,length=directory?4096:size(inner),mode=(directory?0o040755:link?0o120777:kind==="executable"?0o100755:0o100644),ino=inodeOf(inner);
  const body={dev:bigint?1n:1,ino:bigint?BigInt(ino):ino,mode:bigint?BigInt(mode):mode,nlink:bigint?1n:1,uid:0,gid:0,rdev:0,size:bigint?BigInt(length):length,blksize:4096,blocks:Math.ceil(length/512),atimeMs:bigint?0n:0,mtimeMs:bigint?0n:0,ctimeMs:bigint?0n:0,birthtimeMs:bigint?0n:0,atime:new Date(0),mtime:new Date(0),ctime:new Date(0),birthtime:new Date(0)};
  return Object.assign(Object.create(null),body,{isFile:()=>file,isDirectory:()=>directory,isSymbolicLink:()=>link,isFIFO:()=>false,isSocket:()=>false,isBlockDevice:()=>false,isCharacterDevice:()=>false});
 };
 const dirent=(parent:string,name:string,kind:string)=>Object.assign(Object.create(null),{name,parentPath:parent,path:parent,isFile:()=>kind==="file"||kind==="executable",isDirectory:()=>kind==="directory",isSymbolicLink:()=>kind==="symlink",isFIFO:()=>false,isSocket:()=>false,isBlockDevice:()=>false,isCharacterDevice:()=>false});
 const kindOf=(inner:string,follow:boolean,path:string)=>{
  const target=follow?resolveLink(inner):inner,kind=world.kindSync(target);
  return kind===undefined?fail("ENOENT",path):kind;
 };
 const port={
  statSync:((path:string,opts?:{bigint?:boolean;throwIfNoEntry?:boolean})=>{
   const inner=inside(path);if(inner===undefined)return (fallback.statSync as any)(path,opts);
   try{const target=resolveLink(inner);return stats(target,kindOf(target,false,path),opts?.bigint===true);}catch(error){if(opts?.throwIfNoEntry===false&&(error as {code?:string}).code==="ENOENT")return undefined;throw error;}
  }) as unknown as ReadFsV1["statSync"],
  lstatSync:((path:string,opts?:{bigint?:boolean;throwIfNoEntry?:boolean})=>{
   const inner=inside(path);if(inner===undefined)return (fallback.lstatSync as any)(path,opts);
   try{const parent=inner.includes("/")?resolveLink(inner.slice(0,inner.lastIndexOf("/"))):"",full=parent===""?inner.split("/").pop()!:parent+"/"+inner.split("/").pop();return stats(full,kindOf(full,false,path),opts?.bigint===true);}catch(error){if(opts?.throwIfNoEntry===false&&(error as {code?:string}).code==="ENOENT")return undefined;throw error;}
  }) as unknown as ReadFsV1["lstatSync"],
  existsSync:((path:string)=>{const inner=inside(path);if(inner===undefined)return fallback.existsSync(path);try{kindOf(inner,true,path);return true;}catch{return false;}}) as ReadFsV1["existsSync"],
  readFileSync:((path:string,opts?:string|{encoding?:string|null}):unknown=>{
   if(typeof path!=="string")return (fallback.readFileSync as any)(path,opts);
   const inner=inside(path);if(inner===undefined)return (fallback.readFileSync as any)(path,opts);
   const target=resolveLink(inner),kind=kindOf(target,false,path);
   if(kind==="directory")throw Object.assign(new Error(`EISDIR: illegal operation on a directory, read '${path}'`),{code:"EISDIR",errno:-21,syscall:"read",path});
   const bytes=Buffer.from(world.readSync(target)!),encoding=typeof opts==="string"?opts:opts?.encoding;
   return encoding?bytes.toString(encoding as BufferEncoding):bytes;
  }) as unknown as ReadFsV1["readFileSync"],
  readdirSync:((path:string,opts?:{withFileTypes?:boolean}|string)=>{
   const inner=inside(path);if(inner===undefined)return (fallback.readdirSync as any)(path,opts);
   const target=resolveLink(inner),rows=world.listSync(target);
   if(rows===undefined)return fail(world.kindSync(target)===undefined?"ENOENT":"ENOTDIR",path);
   const typed=typeof opts==="object"&&opts?.withFileTypes===true;
   return typed?rows.map(row=>dirent(path,row.name,row.directory?"directory":world.kindSync(target===""?row.name:target+"/"+row.name)!)):rows.map(row=>row.name);
  }) as unknown as ReadFsV1["readdirSync"],
 };
 const real=(path:string,opts?:unknown)=>{
  const inner=inside(path);if(inner===undefined)return (fallback.realpathSync as any)(path,opts);
  const target=resolveLink(inner);kindOf(target,false,path);return target===""?root:join(root,...target.split("/"));
 };
 const realpathSync=Object.assign(real,{native:real}) as unknown as ReadFsV1["realpathSync"];
 return Object.freeze({...port,realpathSync}) as ReadFsV1;
}
export {CAPTURE_POLICY_V1,createCapturingFsV1} from "./📸️capture.ts";
export {withCapturedPlanV1} from "./🎬️plan.ts";
export type {CapturedPlanOptionsV1} from "./🎬️plan.ts";
export {pinModuleV1} from "./🧩️module-loader.ts";
