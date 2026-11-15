import {createHash} from "node:crypto";
import {mkdir,readFile,rename,rm,stat,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {gitV1} from "../../../../../../../🔨️modules/🏃️process/🧰️tools/🌿️git/🟦️.ts";
import {GRAPH_STATE_POLICY_V1 as policy,classifyGraphInputV1,type SharedFileV1} from "../🟦️.ts";

type Stamp=readonly [size:number,mtimeMs:number,ino:number,ctimeMs:number,sha:string];
/** 🌳️ The tree walk of one worktree: how many files were read and how many digests the stamp memo supplied. */
export type TreeFilesV1=Readonly<{files:readonly SharedFileV1[];read:number;memoised:number}>;
const NONE="0".repeat(64);

/** 📚️ The shared universe of a worktree without a replica: the files git tracks plus the untracked files that no ignore rule hides, which is what the replica publishes. */
export async function sharedPathsV1(root:string,signal?:AbortSignal):Promise<string[]>{
 const result=await gitV1(["-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","-z","--cached","--others","--exclude-standard","--deduplicate"],{cwd:root,maxBuffer:1<<30,...(signal===undefined?{}:{signal})});
 if(result.status!==0)throw Error("Git file inventory refused / Git-Dateiverzeichnis abgelehnt: "+result.stderr.slice(0,200));
 const removed=await gitV1(["-c","core.fsmonitor=false","ls-files","-z","--deleted"],{cwd:root,maxBuffer:1<<28,...(signal===undefined?{}:{signal})}),gone=new Set(removed.stdout.split("\0").filter(Boolean));
 return result.stdout.split("\0").filter(path=>path!==""&&!gone.has(path));
}

async function loadMemo(file:string|undefined):Promise<Map<string,Stamp>>{
 if(file===undefined)return new Map();
 try{const value=JSON.parse(await readFile(file,"utf8")) as {version:number;rows:Record<string,Stamp>};return value.version===1?new Map(Object.entries(value.rows)):new Map();}catch{return new Map();}
}

async function saveMemo(file:string|undefined,memo:ReadonlyMap<string,Stamp>):Promise<void>{
 if(file===undefined)return;
 const temporary=`${file}.${process.pid}.tmp`;
 try{await mkdir(dirname(file),{recursive:true});await writeFile(temporary,JSON.stringify({version:1,rows:Object.fromEntries(memo)}));await rename(temporary,file);}catch{await rm(temporary,{force:true}).catch(()=>undefined);}
}

/** 🌳️ Lists the shared files of a worktree and digests every file whose bytes an inference can read, reusing the digest of a file whose size, modification time, change time and inode are unchanged since the memo. Files of the `name` class carry no digest. The memo lives in `memoFile` (written atomically, never replicated: it is an ignored cache file). */
export async function readTreeFilesV1(root:string,options:Readonly<{memoFile?:string;signal?:AbortSignal;paths?:readonly string[]}>={}):Promise<TreeFilesV1>{
 const paths=options.paths??await sharedPathsV1(root,options.signal),memo=await loadMemo(options.memoFile),next=new Map<string,Stamp>(),rows:SharedFileV1[]=[];
 const content:string[]=[];
 for(const path of paths){const kind=classifyGraphInputV1(path);if(kind==="ignored")continue;if(kind==="content")content.push(path);else rows.push([path,NONE]);}
 let cursor=0,read=0,memoised=0;
 await Promise.all(Array.from({length:Math.min(policy.readConcurrency,content.length)},async()=>{
  while(cursor<content.length){
   options.signal?.throwIfAborted();
   const path=content[cursor++]!,file=join(root,path);
   try{
    const before=await stat(file),known=memo.get(path);
    if(known&&known[0]===before.size&&known[1]===before.mtimeMs&&known[2]===before.ino&&known[3]===before.ctimeMs){memoised++;rows.push([path,known[4]]);next.set(path,known);continue;}
    const bytes=await readFile(file),sha=createHash("sha256").update(bytes).digest("hex"),after=await stat(file);
    read++;
    if(after.size===before.size&&after.mtimeMs===before.mtimeMs&&after.ctimeMs===before.ctimeMs)next.set(path,[after.size,after.mtimeMs,after.ino,after.ctimeMs,sha]);
    rows.push([path,sha]);
   }catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT"||(error as NodeJS.ErrnoException).code==="EISDIR"||(error as NodeJS.ErrnoException).code==="ENOTDIR")continue;throw error;}
  }
 }));
 await saveMemo(options.memoFile,next);
 return Object.freeze({files:rows,read,memoised});
}

/** 🗒️ The default memo file of a worktree. */
export const treeMemoFileV1=(root:string):string=>join(root,".cache","f",policy.treeStampFile);
