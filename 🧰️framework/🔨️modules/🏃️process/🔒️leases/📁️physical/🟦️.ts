import {constants,openSync,closeSync,fstatSync,lstatSync} from "node:fs";
import {checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "../🟦️.ts";
import {validateJsonSchemaSubset} from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🔐️ Retains exact physical compiler lock descriptors under the original process lifetime. */
export interface PhysicalFileLocksV1 {readonly pid:number;readonly paths:readonly string[];check():void;close():void;}

/** 🏪️ Uses system flock through the required Bun runtime; unsupported backends refuse before mutation. */
export async function acquirePhysicalFileLocksV1(paths:readonly string[],control:ResourceLeaseControl,request:Readonly<{version:1;wait:boolean}>):Promise<PhysicalFileLocksV1>{
 checkResourceLeaseControl(control);if(validateJsonSchemaSubset(schema.definitions.Request,request,schema).length)throw Error("Closed physical compiler lock request required");if(!process.versions.bun||!["darwin","linux"].includes(process.platform))throw Error("Qualified physical compiler file lock backend required");
 const admitted=paths.map(requireMaterializedPathV1).sort();if(validateJsonSchemaSubset(schema,admitted).length)throw Error("Exact physical compiler lock paths required");
 const {dlopen,read}=await import("bun:ffi"),errnoName=process.platform==="darwin"?"__error":"__errno_location",library=dlopen(process.platform==="darwin"?"/usr/lib/libSystem.B.dylib":"libc.so.6",{flock:{args:["i32","i32"],returns:"i32"},[errnoName]:{args:[],returns:"ptr"}}),protocol=schema.definitions.Protocol.const,held:{fd:number;path:string;dev:number;ino:number}[]=[];let closed=false;
 const close=():void=>{if(closed)return;closed=true;for(const row of held.reverse()){library.symbols.flock!(row.fd,protocol.unlock);closeSync(row.fd);}library.close();};
 try{
  for(const path of admitted){checkResourceLeaseControl(control);const fd=openSync(path,constants.O_RDWR|constants.O_NOFOLLOW),actual=fstatSync(fd),named=lstatSync(path);held.push({fd,path,dev:actual.dev,ino:actual.ino});if(!actual.isFile()||named.isSymbolicLink()||actual.dev!==named.dev||actual.ino!==named.ino)throw Error("Physical compiler lock identity changed");
   while(library.symbols.flock!(fd,protocol.exclusive|protocol.nonblocking)!==0){const pointer=library.symbols[errnoName]!();if(pointer===null||read.i32(pointer)!==protocol.contentionErrno[process.platform as"darwin"|"linux"])throw Error("Physical compiler lock admission failed");if(!request.wait)throw Error("Original physical compiler lock remains held");checkResourceLeaseControl(control);await control.publish({resource:"physical-compiler-lock:"+path,mode:"exclusive",elapsedMs:0});await control.yieldContinuation();const remaining=checkResourceLeaseControl(control);await new Promise<void>(accept=>setTimeout(accept,Math.min(remaining??protocol.retryMilliseconds,protocol.retryMilliseconds)));checkResourceLeaseControl(control);}
  }
  const check=():void=>{checkResourceLeaseControl(control);if(closed)throw Error("Physical compiler lock owner closed");for(const row of held){const actual=fstatSync(row.fd),named=lstatSync(row.path);if(!actual.isFile()||named.isSymbolicLink()||actual.dev!==row.dev||actual.ino!==row.ino||named.dev!==row.dev||named.ino!==row.ino)throw Error("Original physical compiler lock identity changed");}};
  check();return Object.freeze({pid:process.pid,paths:Object.freeze(admitted),check,close});
 }catch(error){close();throw error;}
}
