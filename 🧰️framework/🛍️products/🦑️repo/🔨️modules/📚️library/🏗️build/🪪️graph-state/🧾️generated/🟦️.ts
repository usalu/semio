import {createHash} from "node:crypto";
import {mkdir,readFile,readdir,readlink,rename,rm,lstat,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {GRAPH_STATE_POLICY_V1 as policy} from "../🟦️.ts";

type Stamp=readonly [size:number,mtimeMs:number,ino:number,ctimeMs:number,sha:string];
/** 🧾️ The digest of a set of ignored directories and how it was reached. */
export type GeneratedDigestV1=Readonly<{digest:string;files:number;read:number;memoised:number}>;

async function listing(root:string,directory:string,rows:string[]):Promise<void>{
 let entries;
 try{entries=await readdir(join(root,directory),{withFileTypes:true});}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return;throw error;}
 for(const entry of entries){
  const path=directory===""?entry.name:`${directory}/${entry.name}`;
  if(entry.isDirectory())await listing(root,path,rows);else if(entry.isFile()||entry.isSymbolicLink())rows.push(path);
 }
}

/** 🧾️ Digests the bytes of every file below the given directories (relative to `root`), whether or not git ignores them: Nx hashes only the files git sees, so a result that depends on a generated catalogue needs this digest as a runtime input to be shared and never to hit falsely. The digest folds `path NUL sha256` lines sorted by UTF-8 bytes (a symbolic link folds its target), is independent of the root and of file times, and reuses the digest of a file whose size, times and inode did not change since `memoFile`. A missing directory is the empty set. */
export async function generatedInputsDigestV1(root:string,directories:readonly string[],memoFile?:string):Promise<GeneratedDigestV1>{
 const paths:string[]=[];
 for(const directory of [...directories].sort())await listing(root,directory.replace(/[\\/]+$/u,"").replaceAll("\\","/"),paths);
 let memo=new Map<string,Stamp>();
 if(memoFile!==undefined)try{const value=JSON.parse(await readFile(memoFile,"utf8")) as {version:number;rows:Record<string,Stamp>};if(value.version===1)memo=new Map(Object.entries(value.rows));}catch{}
 const next=new Map<string,Stamp>(),lines:string[]=[];let read=0,memoised=0,cursor=0;
 paths.sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b)));
 const results=new Array<string>(paths.length);
 await Promise.all(Array.from({length:Math.min(policy.readConcurrency,paths.length)},async()=>{
  while(cursor<paths.length){
   const index=cursor++,path=paths[index]!,file=join(root,path);
   try{
    const before=await lstat(file);
    if(before.isSymbolicLink()){results[index]=`${path}\0->${await readlink(file)}`;continue;}
    const known=memo.get(path);
    if(known&&known[0]===before.size&&known[1]===before.mtimeMs&&known[2]===before.ino&&known[3]===before.ctimeMs){memoised++;next.set(path,known);results[index]=`${path}\0${known[4]}`;continue;}
    const sha=createHash("sha256").update(await readFile(file)).digest("hex"),after=await lstat(file);read++;
    if(after.size===before.size&&after.mtimeMs===before.mtimeMs&&after.ctimeMs===before.ctimeMs)next.set(path,[after.size,after.mtimeMs,after.ino,after.ctimeMs,sha]);
    results[index]=`${path}\0${sha}`;
   }catch(error){if(!["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))throw error;}
  }
 }));
 for(const line of results)if(line!==undefined)lines.push(line);
 if(memoFile!==undefined){
  const temporary=`${memoFile}.${process.pid}.tmp`;
  try{await mkdir(dirname(memoFile),{recursive:true});await writeFile(temporary,JSON.stringify({version:1,rows:Object.fromEntries(next)}));await rename(temporary,memoFile);}catch{await rm(temporary,{force:true}).catch(()=>undefined);}
 }
 return Object.freeze({digest:createHash("sha256").update(JSON.stringify([policy.domain,"generated",lines])).digest("hex"),files:lines.length,read,memoised});
}
