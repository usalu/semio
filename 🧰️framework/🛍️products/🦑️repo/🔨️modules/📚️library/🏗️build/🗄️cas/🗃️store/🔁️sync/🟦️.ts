import {createHash,randomUUID} from "node:crypto";
import {chmodSync,existsSync,mkdirSync,readFileSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {appendCasAccessSyncV1} from "../../📒️ledger/🟦️.ts";
import {CAS_POLICY_V1} from "../../🧬️schema/🟦️.ts";
import {casObjectPathV1} from "../🟦️.ts";

const touched=new Map<string,number>(),writer=`${process.pid}-${randomUUID().slice(0,8)}`;
const fan=CAS_POLICY_V1.fanOutHexLength;

/** ✋ Records an access of an object with the same throttle as the asynchronous store; the table is bounded. */
export function touchCasObjectSyncV1(root:string,hash:string,nowMs:number=Date.now()):void{
 const key=root+"\0"+hash,last=touched.get(key);
 if(last!==undefined&&nowMs-last<CAS_POLICY_V1.touchGranularityMs)return;
 if(touched.size>=4096)touched.clear();
 touched.set(key,nowMs);
 try{appendCasAccessSyncV1(root,hash.slice(0,fan),writer,hash,nowMs);}catch{}
}

/** 📥️ Stores bytes under their sha256 in the host store (atomic rename, read-only, one copy per host) and returns the digest; an existing object is only touched. For callers that cannot await. */
export function putCasObjectSyncV1(root:string,bytes:Uint8Array|string):string{
 const hash=createHash("sha256").update(bytes).digest("hex"),target=casObjectPathV1(root,hash,fan);
 if(existsSync(target)){touchCasObjectSyncV1(root,hash);return hash;}
 const directory=join(root,"v1","t");mkdirSync(directory,{recursive:true});mkdirSync(dirname(target),{recursive:true});
 const temporary=join(directory,`${writer}-${randomUUID().slice(0,12)}`);
 writeFileSync(temporary,bytes);
 try{renameSync(temporary,target);}catch(error){rmSync(temporary,{force:true});if(!existsSync(target))throw error;}
 try{chmodSync(target,0o444);}catch{}
 touchCasObjectSyncV1(root,hash);return hash;
}

/** 📖️ Reads an object completely; absent (never stored, or collected by the host policy) is `undefined`, a plain miss. */
export function readCasObjectSyncV1(root:string,hash:string):Buffer|undefined{
 try{const bytes=readFileSync(casObjectPathV1(root,hash,fan));touchCasObjectSyncV1(root,hash);return bytes;}catch{return undefined;}
}

/** 🔎️ Whether an object is in the host store. */
export const hasCasObjectSyncV1=(root:string,hash:string):boolean=>{try{return statSync(casObjectPathV1(root,hash,fan)).isFile();}catch{return false;}};
