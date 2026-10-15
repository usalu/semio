import{createHash}from"node:crypto";
import{createGunzip}from"node:zlib";
import{finished}from"node:stream/promises";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import policy from"./📋️policy.json"with{type:"json"};
import schema from"./🧬️schema.json"with{type:"json"};
if(validateJsonSchemaSubset(schema.definitions.Policy,policy).length)throw Error("Closed SDK package archive policy required");
/** 📄️ Borrows one regular package file or directory from the privately decoded archive. */
export interface NxPackageArchiveEntryV1{readonly path:string;readonly kind:"file"|"directory";readonly mode:number;readonly bytes:Uint8Array}
/** 📦️ Carries only bounded decoded package material and the exact consumed compressed-byte identity. */
export interface NxPackageArchiveV1{readonly compressedBytes:number;readonly inflatedBytes:number;readonly integrity:string;readonly entries:readonly NxPackageArchiveEntryV1[]}
const utf8=new TextDecoder("utf-8",{fatal:true}),refuse=(message:string):never=>{throw Error("SDK package archive "+message);};
const zero=(bytes:Uint8Array)=>bytes.every(byte=>byte===0);
function text(bytes:Uint8Array):string{const end=bytes.indexOf(0);if(end>=0&&!zero(bytes.subarray(end)))refuse("text padding changed");try{return utf8.decode(end<0?bytes:bytes.subarray(0,end));}catch{return refuse("UTF-8 required");}}
function octal(bytes:Uint8Array):number{if(bytes[0]!>=128)refuse("base256 field refused");const value=Buffer.from(bytes).toString("ascii").replace(/[\0 ]+$/u,"").replace(/^ +/u,"");if(!/^[0-7]*$/u.test(value))refuse("octal field required");const number=value?parseInt(value,8):0;if(!Number.isSafeInteger(number)||number<0)refuse("numeric field exceeds ceiling");return number;}
function packagePath(value:string,directory:boolean):string{
 if(Buffer.byteLength(value)>policy.maximumPathBytes||value.includes("\\")||value.includes("\0"))refuse("path ceiling or separator refused");const raw=directory?value.replace(/\/$/u,""):value,parts=raw.split("/");
 if(parts[0]!=="package"||parts.some(part=>!part||part==="."||part===".."||/[<>:"|?*\x00-\x1f]/u.test(part)||/[ .]$/u.test(part)||/^(CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])(?:\.|$)/iu.test(part)))refuse("canonical package path required");
 if(parts.length===1&&!directory)refuse("package file path required");return parts.slice(1).join("/");
}
function pax(bytes:Uint8Array,global:boolean):ReadonlyMap<string,string>{
 if(bytes.byteLength>policy.maximumMetadataBytes)refuse("PAX metadata ceiling exceeded");const result=new Map<string,string>();let offset=0;
 while(offset<bytes.length){const space=bytes.indexOf(32,offset);if(space<0||space-offset>10)refuse("PAX length required");const digits=Buffer.from(bytes.subarray(offset,space)).toString("ascii");if(!/^[1-9][0-9]*$/u.test(digits))refuse("PAX decimal length required");const length=Number(digits),end=offset+length;if(!Number.isSafeInteger(length)||end>bytes.length||end<=space+2||bytes[end-1]!==10)refuse("PAX record boundary refused");let record:string;try{record=utf8.decode(bytes.subarray(space+1,end-1));}catch{return refuse("PAX UTF-8 required");}const equals=record.indexOf("=");if(equals<=0)refuse("PAX key required");const key=record.slice(0,equals),value=record.slice(equals+1);if(result.has(key)||value.includes("\0"))refuse("duplicate PAX key refused");
  if(key==="path"){if(global)refuse("global PAX path refused");}
  else if(key==="size"){if(global||!/^\d+$/u.test(value)||!Number.isSafeInteger(Number(value)))refuse("PAX size refused");}
  else if(["mtime","atime","ctime"].includes(key)){if(!/^-?\d+(?:\.\d+)?$/u.test(value))refuse("PAX time refused");}
  else if(["uid","gid"].includes(key)){if(!/^\d+$/u.test(value))refuse("PAX identity refused");}
  else if(["uname","gname"].includes(key)){if(Buffer.byteLength(value)>policy.maximumPathBytes)refuse("PAX identity ceiling exceeded");}
  else refuse("unsupported PAX key "+key);result.set(key,value);offset=end;
 }return result;
}
async function inflate(source:Uint8Array,control:ResourceLeaseControl,started:number):Promise<Buffer>{
 const decoder=createGunzip({chunkSize:65536}),completion=finished(decoder,{cleanup:true});void completion.catch(()=>{});const arena=Buffer.alloc(policy.maximumInflatedBytes);let bytes=0,published=0;const abort=()=>decoder.destroy(control.signal.reason instanceof Error?control.signal.reason:Error("SDK package archive canceled"));control.signal.addEventListener("abort",abort,{once:true});
 try{checkResourceLeaseControl(control);decoder.end(source);for await(const value of decoder){checkResourceLeaseControl(control);const chunk=Buffer.isBuffer(value)?value:Buffer.from(value);if(bytes+chunk.byteLength>policy.maximumInflatedBytes)refuse("inflated byte ceiling exceeded");chunk.copy(arena,bytes);bytes+=chunk.byteLength;if(bytes-published>=1048576){published=bytes;await control.publish({resource:"nx:package-archive:inflate",mode:"shared",elapsedMs:performance.now()-started});await control.yieldContinuation();checkResourceLeaseControl(control);}}await completion;checkResourceLeaseControl(control);return arena.subarray(0,bytes);}
 finally{control.signal.removeEventListener("abort",abort);decoder.destroy();await completion.catch(()=>{});}
}
/** 🗃️ Validates finite gzip/tar package framing before any caller can materialize a path or byte. */
export async function decodeNxPackageArchiveV1(input:Uint8Array,control:ResourceLeaseControl):Promise<NxPackageArchiveV1>{
 checkResourceLeaseControl(control);if(input.byteLength===0||input.byteLength>policy.maximumCompressedBytes)refuse("compressed byte ceiling exceeded");const source=Uint8Array.from(input),integrity="sha512-"+createHash("sha512").update(source).digest("base64"),started=performance.now();await control.publish({resource:"nx:package-archive:inflate",mode:"shared",elapsedMs:0});await control.yieldContinuation();const bytes=await inflate(source,control,started),entries:NxPackageArchiveEntryV1[]=[],seen=new Map<string,"file"|"directory">();if(bytes.length%512!==0)refuse("whole tar blocks required");let offset=0,headers=0,ended=false,pending:ReadonlyMap<string,string>|undefined,longName:string|undefined;
 while(offset<bytes.length){checkResourceLeaseControl(control);if(offset+512>bytes.length)refuse("truncated header");const header=bytes.subarray(offset,offset+512);if(zero(header)){if(pending||longName!==undefined||offset+1024>bytes.length||!zero(bytes.subarray(offset)))refuse("two zero footer blocks required");ended=true;break;}
  if(++headers>policy.maximumEntries)refuse("entry ceiling exceeded");const checksum=octal(header.subarray(148,156)),actual=header.reduce((sum,byte,index)=>sum+(index>=148&&index<156?32:byte),0);if(checksum!==actual)refuse("header checksum changed");const magic=text(header.subarray(257,263)).trim();if(magic!=="ustar")refuse("POSIX tar magic required");const prefix=text(header.subarray(345,500)),name=(prefix?prefix+"/":"")+text(header.subarray(0,100)),kind=header[156]!,size=octal(header.subarray(124,136)),mode=octal(header.subarray(100,108));if(size>policy.maximumFileBytes||mode>0o777)refuse("file byte or mode ceiling exceeded");const start=offset+512,end=start+size,next=start+Math.ceil(size/512)*512;if(next>bytes.length)refuse("truncated body");if(!zero(bytes.subarray(end,next)))refuse("body padding changed");const body=bytes.subarray(start,end);offset=next;
  if(kind===120||kind===103){if(pending||longName!==undefined)refuse("stacked path metadata refused");const metadata=pax(body,kind===103);if(kind===120)pending=metadata;}
  else if(kind===76){if(pending||longName!==undefined||body.length>policy.maximumPathBytes+1||body.at(-1)!==0)refuse("long path metadata refused");longName=text(body);}
  else{if(kind!==0&&kind!==48&&kind!==53)refuse("links, sparse and special entries refused");if(pending?.has("size")&&Number(pending.get("size"))!==size)refuse("ambiguous PAX body size refused");const directory=kind===53;if(directory&&size!==0)refuse("directory body refused");const path=packagePath(pending?.get("path")??longName??name,directory),key=path.normalize("NFC").toLowerCase(),entryKind=directory?"directory":"file";pending=undefined;longName=undefined;if(seen.has(key))refuse("duplicate normalized package entry");const parts=key.split("/");for(let count=1;count<parts.length;count++)if(seen.get(parts.slice(0,count).join("/"))==="file")refuse("file ancestor refused");if(entryKind==="file"&&[...seen.keys()].some(previous=>previous.startsWith(key+"/")))refuse("file replaces parent directory");seen.set(key,entryKind);entries.push(Object.freeze({path,kind:entryKind,mode,bytes:body}));}
  if(headers%policy.progressEntries===0){await control.publish({resource:"nx:package-archive:headers",mode:"shared",elapsedMs:performance.now()-started});await control.yieldContinuation();checkResourceLeaseControl(control);}
 }if(!ended||!entries.some(row=>row.kind==="file"))refuse("complete package archive required");checkResourceLeaseControl(control);return Object.freeze({compressedBytes:source.byteLength,inflatedBytes:bytes.byteLength,integrity,entries:Object.freeze(entries)});
}
