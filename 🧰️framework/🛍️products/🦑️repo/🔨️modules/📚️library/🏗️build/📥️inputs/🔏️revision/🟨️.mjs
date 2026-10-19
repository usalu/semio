import{constants,lstatSync,openSync,fstatSync,readSync,closeSync}from"node:fs";
import{createHash}from"node:crypto";
import{resolve,isAbsolute,parse,relative,sep,join}from"node:path";
import{fileURLToPath}from"node:url";
const maximumSourceBytes=8388608,maximumTextBytes=65536,maximumChunkBytes=65536,maximumPathUnits=256,maximumAncestors=64;
/** 🧊️ Remembers one settled revision under its file identity, so an unchanged Source is hashed once instead of on every self-check. */
const settledRevisions=new Map(),settleMilliseconds=2000,maximumSettledRevisions=4096;
const stamp=row=>[row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].join(":");
function capture(path,maximumBytes,retainText){
 const base=parse(path).root,parts=relative(base,path).split(sep).filter(Boolean),parents=[];let current=base;
 if(parts.length>maximumAncestors)throw Error("Source revision ancestry capacity exceeded");
 for(const part of["",...parts.slice(0,-1)]){if(part)current=join(current,part);const row=lstatSync(current,{bigint:true});if(!row.isDirectory()||row.isSymbolicLink())throw Error("Physical Source revision ancestry required");parents.push({path:current,dev:row.dev,ino:row.ino,mode:row.mode});}
 const before=lstatSync(path,{bigint:true});
 if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size<0n||before.size>BigInt(maximumBytes))throw Error("Bounded physical Source revision required");
 const size=Number(before.size),identity=stamp(before),file=openSync(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0));let failed=false,primary,value,window,retained;
 try{
 const verify=()=>{for(const before of parents){const row=lstatSync(before.path,{bigint:true});if(!row.isDirectory()||row.isSymbolicLink()||row.dev!==before.dev||row.ino!==before.ino||row.mode!==before.mode)throw Error("Source revision ancestry changed");}const linked=lstatSync(path,{bigint:true}),opened=fstatSync(file,{bigint:true});if(!linked.isFile()||linked.isSymbolicLink()||stamp(linked)!==identity||stamp(opened)!==identity)throw Error("Source revision descriptor or path changed");};
 verify();window=Buffer.alloc(Math.min(maximumChunkBytes,Math.max(1,size)));retained=retainText?Buffer.alloc(size):undefined;const hash=createHash("sha256");let offset=0;
 while(offset<size){if(stamp(fstatSync(file,{bigint:true}))!==identity)throw Error("Source revision descriptor changed");const length=Math.min(window.length,size-offset),bytes=readSync(file,window,0,length,offset);if(bytes!==length)throw Error("Source revision shortened during capture");hash.update(window.subarray(0,length));retained?.set(window.subarray(0,length),offset);offset+=length;}
 verify();if(readSync(file,window,0,1,offset)!==0)throw Error("Source revision grew beyond admitted EOF");verify();value=Object.freeze({sha256:hash.digest("hex"),byteLength:size,text:retained===undefined?null:new TextDecoder("utf-8",{fatal:true,ignoreBOM:true}).decode(retained)});
 }catch(error){failed=true;primary=error;}
 try{closeSync(file);}catch(error){if(!failed){failed=true;primary=error;}}finally{window?.fill(0);retained?.fill(0);}
 if(failed)throw primary;return value;
}
const ownPath=fileURLToPath(import.meta.url),loaded=capture(ownPath,maximumSourceBytes,false);
function current(){if(capture(ownPath,loaded.byteLength,false).sha256!==loaded.sha256)throw Error("Stale loaded Source revision reader; restart required");}
/** 🔏️ Streams one finite ordinary Source revision; returned DATA grants no physical resource custody. */
export function captureBoundedSourceRevisionV1(path,maximumBytes,retainText=false){
 if(typeof path!=="string"||!isAbsolute(path)||resolve(path)!==path||path.length>maximumPathUnits||path.includes("\0")||!Number.isSafeInteger(maximumBytes)||maximumBytes<0||maximumBytes>maximumSourceBytes||typeof retainText!=="boolean"||retainText&&maximumBytes>maximumTextBytes)throw Error("Reducing finite Source revision request required");
 const cached=settledRevisions.get(path);
 if(cached&&cached.maximumBytes===maximumBytes&&cached.retainText===retainText){try{const row=lstatSync(path,{bigint:true});if(row.isFile()&&!row.isSymbolicLink()&&stamp(row)===cached.identity)return cached.value;}catch{}}
 current();const value=capture(path,maximumBytes,retainText),taken=Date.now();current();
 try{const row=lstatSync(path,{bigint:true});if(row.isFile()&&!row.isSymbolicLink()&&Number(row.ctimeNs/1000000n)<taken-settleMilliseconds&&Number(row.mtimeNs/1000000n)<taken-settleMilliseconds){if(settledRevisions.size>=maximumSettledRevisions)settledRevisions.clear();settledRevisions.set(path,{maximumBytes,retainText,identity:stamp(row),value});}}catch{}
 return value;
}
