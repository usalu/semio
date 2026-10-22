import{closeSync,constants,fstatSync,lstatSync,openSync,readSync}from"node:fs";
import{createHash}from"node:crypto";
import{join}from"node:path";

const NAMESPACES=Object.freeze(["build-input-closures","build-lock-closures"]),KEY=/^[a-f0-9]{64}$/u,MAXIMUM_STAMPS=262144,verified=new Map();
const stampOf=row=>[row.dev,row.ino,row.size,row.mtimeNs,row.ctimeNs].join(":");
const same=(left,right)=>["dev","ino","size","nlink","mtimeNs","ctimeNs"].every(name=>left[name]===right[name]);

/** 🪶️ Reads one content addressed closure inventory leaf with a no-follow descriptor, two identity checks and the leaf's own sha256, without the per-read storage and ancestry round trips of the managed batch; the caller keeps the storage consumer custody and validates the value. */
export function readInventoryRecordLeanV1(directory,key,maximumBytes){
 if(typeof directory!=="string"||!NAMESPACES.includes(directory.split(/[\\/]/u).at(-1))||typeof key!=="string"||!KEY.test(key)||!Number.isSafeInteger(maximumBytes)||maximumBytes<1)throw Error("Closed inventory leaf identity required");
 const path=join(directory,key+".json"),before=lstatSync(path,{bigint:true});
 if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size<1n||before.size>BigInt(maximumBytes))throw Error("Single-link bounded regular inventory record required");
 const size=Number(before.size),bytes=Buffer.allocUnsafe(size),fd=openSync(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0));
 let failed=false;try{
  const opened=fstatSync(fd,{bigint:true});if(!opened.isFile()||!same(before,opened))throw Error("Inventory record changed while opening");
  for(let offset=0;offset<size;){const count=readSync(fd,bytes,offset,size-offset,offset);if(!count)throw Error("Inventory record shortened");offset+=count;}
  if(readSync(fd,Buffer.allocUnsafe(1),0,1,size))throw Error("Inventory record grew");
  if(!same(before,fstatSync(fd,{bigint:true})))throw Error("Inventory record changed while reading");
 }catch(error){failed=true;throw error;}finally{try{closeSync(fd);}catch(error){if(!failed)throw error;}}
 const sha256=createHash("sha256").update(bytes).digest("hex");
 if(sha256!==key)throw Error("Inventory record content differs from its name");
 return Object.freeze({sha256,byteLength:size,value:JSON.parse(new TextDecoder("utf-8",{fatal:true,ignoreBOM:true}).decode(bytes)),stamp:stampOf(before)});
}

/** 🧊️ Remembers that a leaf with exactly this identity was read, hashed and validated; a content addressed leaf with an unchanged identity cannot have changed. */
export function rememberInventoryRecordV1(root,directory,key,stamp){
 if(verified.size>=MAXIMUM_STAMPS)verified.clear();
 verified.set(root+"\0"+join(directory,key+".json"),stamp);
}

/** ❓️ Answers whether the leaf still has the identity it was validated with: one lstat instead of a read, a hash and a validation. */
export function inventoryRecordStillHoldsV1(root,directory,key){
 const path=join(directory,key+".json"),stamp=verified.get(root+"\0"+path);
 if(stamp===undefined)return false;
 try{const row=lstatSync(path,{bigint:true});return row.isFile()&&!row.isSymbolicLink()&&row.nlink===1n&&stampOf(row)===stamp;}catch{verified.delete(root+"\0"+path);return false;}
}

/** 🗑️ Forgets every remembered identity (tests). */
export function forgetInventoryRecordsV1(){verified.clear();}
