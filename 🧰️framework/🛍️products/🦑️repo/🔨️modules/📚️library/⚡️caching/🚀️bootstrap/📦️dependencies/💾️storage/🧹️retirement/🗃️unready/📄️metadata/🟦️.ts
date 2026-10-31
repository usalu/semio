/** 📄️ Observes bounded opaque current capsule bytes without conferring intent or deletion authority. */
import {createHash} from "node:crypto";
import {lstatSync} from "node:fs";
import {basename,dirname,isAbsolute,join,parse,relative,resolve,sep} from "node:path";
import {types} from "node:util";
import {withArtifactReadFileSyncV1,type ArtifactReadFileIdentityV1} from "../../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🪪️ Captures exact current physical fields as finite portable DATA. */
export interface JavascriptDependencyUnreadyMetadataStampV1{readonly dev:string;readonly ino:string;readonly mode:number;readonly nlink:number;readonly size:number;readonly mtimeNs:string;readonly ctimeNs:string}
/** 🧾️ Retains opaque text alongside its actual-byte digest and unchanged physical identity. */
export interface JavascriptDependencyUnreadyMetadataV1{readonly path:string;readonly text:string;readonly sha256:string;readonly stamp:JavascriptDependencyUnreadyMetadataStampV1;readonly before:JavascriptDependencyUnreadyMetadataStampV1;readonly after:JavascriptDependencyUnreadyMetadataStampV1;readonly authority:false}

const policy=schema.$defs.Protocol.const;
/** 🔎️ Copies only the canonical seven physical fields from a first-party read capability. */
function stamp(value:ArtifactReadFileIdentityV1):JavascriptDependencyUnreadyMetadataStampV1{
 const size=Number(value.size),mode=Number(value.mode),nlink=Number(value.nlink);
 if(![size,mode,nlink].every(Number.isSafeInteger)||size<0||size>policy.maximumBytes||nlink!==1||(mode&0o170000)!==0o100000)throw Error("Bounded ordinary single-link capsule metadata required");
 return Object.freeze({dev:String(value.dev),ino:String(value.ino),mode,nlink,size,mtimeNs:String(value.mtimeNs),ctimeNs:String(value.ctimeNs)});
}
/** 📥️ Reads malformed or partial UTF8 as inert current metadata through one genuine synchronous nofollow owner. */
export function readJavascriptDependencyUnreadyMetadataV1(path:string,maximumBytes:number,check:()=>void):JavascriptDependencyUnreadyMetadataV1{
 if(arguments.length!==3||typeof path!=="string"||path.length>256||path.includes("\0")||!isAbsolute(path)||resolve(path)!==path||Buffer.from(path).toString("utf8")!==path||!policy.roles.includes(basename(path))||!Number.isSafeInteger(maximumBytes)||maximumBytes<1||maximumBytes>policy.maximumBytes||typeof check!=="function"||types.isProxy(check)||types.isAsyncFunction(check))throw Error("Exact bounded synchronous capsule metadata request required");
 const checkpoint=()=>{if(check()!==undefined)throw Error("Synchronous capsule metadata checkpoint required");},ancestors:Array<{path:string;dev:bigint;ino:bigint;mode:bigint}>=[];
 const directory=(path:string)=>{checkpoint();const value=lstatSync(path,{bigint:true});if(!value.isDirectory()||value.isSymbolicLink())throw Error("Ordinary current capsule metadata ancestor required");return value;};
 let parent=parse(path).root;const parts=relative(parent,dirname(path)).split(sep).filter(Boolean);
 for(const part of ["",...parts]){if(part)parent=join(parent,part);const value=directory(parent);ancestors.push({path:parent,dev:value.dev,ino:value.ino,mode:value.mode});}
 const recheck=()=>{for(const before of ancestors){const after=directory(before.path);if(before.dev!==after.dev||before.ino!==after.ino||before.mode!==after.mode)throw Error("Capsule metadata ancestor changed");}},named=()=>{checkpoint();const value=lstatSync(path,{bigint:true});if(!value.isFile()||value.isSymbolicLink())throw Error("Actual nofollow capsule metadata file required");return stamp(value);},before=named();
 if(before.size>maximumBytes)throw Error("Capsule metadata byte bound exceeded");
 const same=(actual:JavascriptDependencyUnreadyMetadataStampV1)=>{if(JSON.stringify(before)!==JSON.stringify(actual))throw Error("Exact unchanged capsule metadata entry required");};
 const result=withArtifactReadFileSyncV1(path,checkpoint,file=>{
  same(stamp(file.stat()));const bytes=Buffer.alloc(before.size);let offset=0;
  while(offset<bytes.length){checkpoint();same(stamp(file.stat()));const wanted=Math.min(policy.readBytes,bytes.length-offset),count=file.read(bytes,offset,wanted,offset);if(!Number.isSafeInteger(count)||count<1||count>wanted)throw Error("Complete bounded capsule metadata bytes required");offset+=count;same(stamp(file.stat()));}
  checkpoint();if(file.read(Buffer.alloc(1),0,1,offset)!==0)throw Error("Capsule metadata grew during observation");same(stamp(file.stat()));same(named());recheck();
  return{text:new TextDecoder("utf-8",{fatal:false,ignoreBOM:true}).decode(bytes),sha256:createHash("sha256").update(bytes).digest("hex")};
 });
 checkpoint();const after=named();same(after);recheck();
 return Object.freeze({path,text:result.text,sha256:result.sha256,stamp:before,before,after,authority:false});
}
