import {randomBytes} from "node:crypto";
import {mkdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {sha256V1,UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import {RACY_NANOSECONDS_V1} from "../🟦️.ts";

/** 🗂️ The no-follow stamp of one foreign crate file; an empty `ino` is a stamp that proves nothing. */
export type ForeignStampV1=Readonly<{dev:string;ino:string;size:string;mtimeNs:string;ctimeNs:string}>;
/** 📒️ One open book of a foreign crate directory: digests recalled by stamp, digests learned in this walk. */
export type ForeignBookV1=Readonly<{recall(path:string,stamp:ForeignStampV1):string|undefined;remember(path:string,stamp:ForeignStampV1,digest:string):void;close():void}>;
type Row=readonly [dev:string,ino:string,size:string,mtimeNs:string,ctimeNs:string,digest:string];
const DIGEST=/^[0-9a-f]{64}$/u;

/** 📚️ Remembers the sha-256 of foreign (registry, git, external path) crate files across invocations: a file whose device, inode, size, mtime and ctime are unchanged is not read again, and a stamp younger than the racy window is never remembered. One small book per crate directory under the cache root; a book that cannot be read or written only costs the next walk its hashes.
 * @see ../🟦️.ts */
export function foreignDigestMemoV1(root:string){
 const book=(directory:string)=>{const name=sha256V1(directory);return join(root,UNIT_CACHE_POLICY_V1.layout.foreign,name.slice(0,2),name+".json");};
 return Object.freeze({open(directory:string,startedMs:number):ForeignBookV1{
  const file=book(directory),horizon=BigInt(Math.floor(startedMs))*1_000_000n-RACY_NANOSECONDS_V1,loaded=new Map<string,Row>(),next=new Map<string,Row>();let changed=false;
  try{const parsed=JSON.parse(readFileSync(file,"utf8"));if(parsed.version===1&&parsed.directory===directory&&parsed.files&&typeof parsed.files==="object")for(const[path,row]of Object.entries(parsed.files as Record<string,Row>))if(Array.isArray(row)&&row.length===6&&row.every(value=>typeof value==="string")&&DIGEST.test(row[5]!))loaded.set(path,row);}catch{}
  return Object.freeze({
   recall(path:string,stamp:ForeignStampV1):string|undefined{const row=loaded.get(path);if(!row||stamp.ino===""||row[0]!==stamp.dev||row[1]!==stamp.ino||row[2]!==stamp.size||row[3]!==stamp.mtimeNs||row[4]!==stamp.ctimeNs)return undefined;next.set(path,row);return row[5];},
   remember(path:string,stamp:ForeignStampV1,digest:string):void{if(stamp.ino===""||!DIGEST.test(digest)||BigInt(stamp.mtimeNs)>=horizon||BigInt(stamp.ctimeNs)>=horizon)return;next.set(path,[stamp.dev,stamp.ino,stamp.size,stamp.mtimeNs,stamp.ctimeNs,digest]);changed=true;},
   close():void{
    if(!changed&&next.size===loaded.size)return;
    const temporary=file+"."+randomBytes(4).toString("hex");
    try{mkdirSync(dirname(file),{recursive:true});writeFileSync(temporary,JSON.stringify({version:1,directory,files:Object.fromEntries(next)}));renameSync(temporary,file);}catch{rmSync(temporary,{force:true});}
   },
  });
 }});
}
