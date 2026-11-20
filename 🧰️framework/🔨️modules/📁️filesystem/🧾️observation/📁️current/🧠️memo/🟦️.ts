import {mkdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const protocol=Object.freeze({...schema.definitions.Protocol.const});
const shared=new Map<string,DigestMemoV1>();
const digestPattern=/^[a-f0-9]{64}$/u,decimalPattern=/^[0-9]{1,24}$/u;

/** 🪪️ Device, inode, size, modification and change time of one regular file, as a stat reports them in nanoseconds. */
export type FileIdentityV1=readonly [bigint,bigint,bigint,bigint,bigint];
/** 🧾️ What a memo recalls about one file: its digest and byte length. */
export type RememberedDigestV1=Readonly<{sha256:string;byteLength:number}>;
type RowV1=Readonly<{identity:FileIdentityV1;sha256:string;byteLength:number}>;

const sameIdentity=(left:FileIdentityV1,right:FileIdentityV1):boolean=>left.every((value,index)=>value===right[index]);
const rowOf=(value:unknown):[string,RowV1]|undefined=>{
 if(!Array.isArray(value)||value.length!==4)return;
 const [path,identity,sha256,byteLength]=value as [unknown,unknown,unknown,unknown];
 if(typeof path!=="string"||!path||path.length>256||!Array.isArray(identity)||identity.length!==5||!identity.every(item=>typeof item==="string"&&decimalPattern.test(item))||typeof sha256!=="string"||!digestPattern.test(sha256)||!Number.isSafeInteger(byteLength)||(byteLength as number)<0)return;
 return [path,Object.freeze({identity:Object.freeze(identity.map(item=>BigInt(item as string))) as unknown as FileIdentityV1,sha256,byteLength:byteLength as number})];
};

/** 🧠️ Remembers file digests by identity, in memory for the process and in one file per directory for the next one; the digest of a file is answered only while its identity is exactly the remembered one.
 * @see ./🧬️schema/🔣️.json
 */
export class DigestMemoV1 {
 private readonly rows=new Map<string,RowV1>();private readonly file:string;private dirty=0;
 constructor(directory:string){
  this.file=join(resolve(directory),protocol.fileName);
  try{const parsed=JSON.parse(readFileSync(this.file,"utf8")) as {version?:unknown;entries?:unknown};if(parsed.version===protocol.version&&Array.isArray(parsed.entries))for(const entry of parsed.entries){const row=rowOf(entry);if(row)this.rows.set(row[0],row[1]);}}catch{}
 }
 /** 🔎️ The digest remembered for `path` when `identity` is exactly the remembered one. */
 lookup(path:string,identity:FileIdentityV1):RememberedDigestV1|undefined{const row=this.rows.get(path);return row&&sameIdentity(row.identity,identity)?{sha256:row.sha256,byteLength:row.byteLength}:undefined;}
 /** 📝️ Remembers the digest of a file hashed from `startedMs` on under an identity that held before and after; refuses (false) a file modified or changed less than the racy window before the hash started. */
 remember(path:string,identity:FileIdentityV1,digest:RememberedDigestV1,startedMs:number):boolean{
  const newest=identity[3]>identity[4]?identity[3]:identity[4];
  if(newest>BigInt(Math.floor(startedMs-protocol.racyMilliseconds))*1000000n)return false;
  this.rows.delete(path);this.rows.set(path,Object.freeze({identity:Object.freeze([...identity]) as unknown as FileIdentityV1,sha256:digest.sha256,byteLength:digest.byteLength}));
  if(this.rows.size>protocol.maximumEntries)this.rows.delete(this.rows.keys().next().value!);
  if(++this.dirty>=protocol.flushEntries)this.flush();
  return true;
 }
 /** 📊️ How many files are remembered. */
 get size():number{return this.rows.size;}
 /** 💾️ Writes the memo atomically when anything was remembered since the last write; a cache that cannot be written is skipped, never a failure. */
 flush():void{
  if(!this.dirty)return;this.dirty=0;
  const temporary=this.file+"."+process.pid+".tmp";
  try{mkdirSync(dirname(this.file),{recursive:true});writeFileSync(temporary,JSON.stringify({version:protocol.version,entries:[...this.rows].map(([path,row])=>[path,row.identity.map(String),row.sha256,row.byteLength])}));renameSync(temporary,this.file);}
  catch{try{rmSync(temporary,{force:true});}catch{}}
 }
}

/** 🗃️ The one memo of `directory` in this process, loaded from the directory when first asked for and written there when the process ends and every schema-declared `flushEntries` remembered files. */
export function digestMemoV1(directory:string):DigestMemoV1{
 const key=resolve(directory);let memo=shared.get(key);
 if(!memo){memo=new DigestMemoV1(key);shared.set(key,memo);const held=memo;process.once("exit",()=>held.flush());}
 return memo;
}
