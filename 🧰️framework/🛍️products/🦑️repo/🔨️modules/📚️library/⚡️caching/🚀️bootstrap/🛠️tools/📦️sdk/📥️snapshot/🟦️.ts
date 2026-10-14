import {constants,type BigIntStats} from "node:fs";
import {createHash} from "node:crypto";
import {copyFile,lstat,mkdir,readdir,readlink,realpath,symlink} from "node:fs/promises";
import {dirname,isAbsolute,join,relative,resolve} from "node:path";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";

/** 🔏️ One applied SDK receipt is independent of authorizer source revisions. */
export interface NxSdkSnapshotReceiptV1 {readonly version:1;readonly digest:string;readonly patches:Readonly<Record<string,{readonly version:1;readonly files:Readonly<Record<string,string>>}>>;}
/** 🧮️ Only pinned installation inputs participate in reusable SDK identity. */
export function nxSdkSnapshotIdentityV1(environment:Readonly<{platform:string;architecture:string;libc:string}>,files:ReadonlyMap<string,Uint8Array>):string{
 const hash=createHash("sha256").update("nx-sdk-snapshot-v1\0"+environment.platform+"\0"+environment.architecture+"\0"+environment.libc+"\0");
 if(!files.has("package.json")||!files.has("bun.lock"))throw Error("Pinned SDK manifest and lock required");
 for(const[path,bytes]of [...files].sort(([a],[b])=>Buffer.compare(Buffer.from(a),Buffer.from(b)))){if(path!=="package.json"&&path!=="bun.lock"&&!/^patches\/[a-f0-9]{64}\.patch$/u.test(path))throw Error("Closed pinned SDK input required");hash.update(path+"\0").update(bytes).update("\0");}
 return hash.digest("hex");
}
/** ✅️ Rejects incomplete or differently owned SDK snapshot markers. */
export function nxSdkSnapshotReceiptHoldsV1(value:unknown,digest:string):value is NxSdkSnapshotReceiptV1{
 return !validateJsonSchemaSubset({$schema:schema.$schema,$defs:schema.$defs,$ref:"#/$defs/Receipt"},value).length&&(value as NxSdkSnapshotReceiptV1).digest===digest;
}

/** 📦️ One private toolkit receives independently writable bytes from an immutable dependency snapshot. */
export interface NxSdkSnapshotCloneV1 {readonly version:1;readonly files:number;readonly directories:number;readonly links:number;readonly bytes:number;}
const policy=schema.$defs.Policy.const;
type Stamp=BigIntStats;
const stable=(left:Stamp,right:Stamp)=>left.dev===right.dev&&left.ino===right.ino&&left.size===right.size&&left.mtimeNs===right.mtimeNs&&left.ctimeNs===right.ctimeNs;
const contained=(root:string,path:string)=>{const child=relative(root,path);return child!==""&&!isAbsolute(child)&&child!==".."&&!child.startsWith(".."+(process.platform==="win32"?"\\":"/"));};
function bounded(path:string):void{if(path.length>policy.maximumPath||[...path].length>policy.maximumPath)throw Error("SDK snapshot materialized path exceeds 256 characters");}
async function ancestry(path:string):Promise<readonly {path:string;stamp:Stamp}[]>{
 const rows:{path:string;stamp:Stamp}[]=[];
 for(let current=path;;current=dirname(current)){const stamp=await lstat(current,{bigint:true});if(!stamp.isDirectory()||stamp.isSymbolicLink())throw Error("SDK snapshot requires no-follow directory ancestry");rows.push({path:current,stamp});if(dirname(current)===current)return rows;}
}

/** 🧊️ Copies already patched SDK files through bounded copy-on-write operations and original caller ports. */
export async function cloneNxSdkSnapshotV1(source:string,destination:string,control:ResourceLeaseControl):Promise<NxSdkSnapshotCloneV1>{
 checkResourceLeaseControl(control);const root=resolve(source),target=resolve(destination);bounded(root);bounded(target);
 if(root===target||contained(root,target)||contained(target,root))throw Error("SDK snapshot requires independent directories");
 const parents=await ancestry(root),targetParents=await ancestry(target),directories:{path:string;stamp:Stamp}[]=[],files:{path:string;stamp:Stamp;link?:string;directory?:boolean}[]=[];let count=0,bytes=0;
 const checkpoint=async()=>{checkResourceLeaseControl(control);await control.publish({resource:"nx-sdk-snapshot",mode:"shared",elapsedMs:0});checkResourceLeaseControl(control);await control.yieldContinuation();checkResourceLeaseControl(control);};
 await checkpoint();
 const walk=async(path:string):Promise<void>=>{
  checkResourceLeaseControl(control);const original=join(root,path),output=join(target,path);bounded(original);bounded(output);if(++count>policy.maximumEntries)throw Error("SDK snapshot entry capacity exceeded");
  const stamp=await lstat(original,{bigint:true});
  if(stamp.isSymbolicLink()){
   const link=await readlink(original),resolved=resolve(dirname(original),link);
   if(isAbsolute(link)||!contained(root,resolved)||!contained(root,await realpath(resolved)))throw Error("SDK snapshot link must remain contained");
   const actual=await lstat(resolved);if(!actual.isFile()&&!actual.isDirectory())throw Error("SDK snapshot link target must be regular");
   files.push({path,stamp,link,directory:actual.isDirectory()});
  }else if(stamp.isDirectory()){
   await mkdir(output);directories.push({path,stamp});
   for(const name of await readdir(original))await walk(path+"/"+name);
  }else if(stamp.isFile()){files.push({path,stamp});bytes+=Number(stamp.size);if(!Number.isSafeInteger(bytes))throw Error("SDK snapshot byte capacity exceeded");}
  else throw Error("SDK snapshot refuses nonregular dependency files");
  if(count%128===0)await checkpoint();
 };
 await walk("node_modules");let next=0,failed=false,failure:unknown;
 await Promise.allSettled(Array.from({length:Math.min(policy.concurrency,files.length)},async()=>{
  try{while(!failed&&next<files.length){checkResourceLeaseControl(control);const row=files[next++]!,original=join(root,row.path),output=join(target,row.path);
   if(!stable(row.stamp,await lstat(original,{bigint:true})))throw Error("SDK snapshot source changed during clone");
   if(row.link!==undefined)await symlink(row.link,output,row.directory?"dir":"file");else await copyFile(original,output,constants.COPYFILE_FICLONE|constants.COPYFILE_EXCL);
   if(!stable(row.stamp,await lstat(original,{bigint:true}))||row.link!==undefined&&await readlink(original)!==row.link)throw Error("SDK snapshot source changed during clone");
   checkResourceLeaseControl(control);
  }}catch(error){if(!failed){failed=true;failure=error;}}
 }));
 if(failed)throw failure;
 for(const row of directories)if(!stable(row.stamp,await lstat(join(root,row.path),{bigint:true})))throw Error("SDK snapshot directory changed during clone");
 for(const row of [...parents,...targetParents]){const after=await lstat(row.path,{bigint:true});if(!after.isDirectory()||after.isSymbolicLink()||after.dev!==row.stamp.dev||after.ino!==row.stamp.ino)throw Error("SDK snapshot ancestry changed during clone");}
 await checkpoint();
 return Object.freeze({version:1,files:files.filter(row=>row.link===undefined).length,directories:directories.length,links:files.filter(row=>row.link!==undefined).length,bytes});
}
