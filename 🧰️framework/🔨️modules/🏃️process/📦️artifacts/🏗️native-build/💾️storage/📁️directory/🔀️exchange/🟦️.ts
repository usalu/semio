import{openSync,closeSync,fsyncSync,constants}from"node:fs";
import{createRequire}from"node:module";
import type{dlopen as BunDlopen}from"bun:ffi";
import{nativeDirectoryProtocolV1,type NativeDirectoryPlatformPortV1}from"../📋️contract/🟦️.ts";
const protocol=nativeDirectoryProtocolV1();

/** 🔌️ Isolates the one kernel call that atomically exchanges two directory names. */
export type NativeDirectoryExchangeCallV1=Readonly<{exchange(first:string,second:string):Readonly<{status:number;errno:number}>}>;

/** 💾️ Flushes one directory entry table through a no-follow directory descriptor. */
export function flushNativePosixDirectoryV1(path:string):void{const descriptor=openSync(path,constants.O_RDONLY|(constants.O_DIRECTORY??0)|(constants.O_NOFOLLOW??0));let failed=false,failure:unknown;try{fsyncSync(descriptor);}catch(error){failed=true;failure=error;}try{closeSync(descriptor);}catch(error){if(!failed)throw error;}if(failed)throw failure;}

const exchangeRow=()=>process.platform==="darwin"?protocol.exchange.darwin:protocol.exchange.linux;
/** 🧲️ Binds libc through bun:ffi once; an unsupported libc is a precise refusal, never a fallback. */
export function nativeDirectoryExchangeCallV1():NativeDirectoryExchangeCallV1{
 const platform=process.platform;if(platform!=="darwin"&&platform!=="linux")throw Error("Atomic directory exchange requires Linux or macOS");
 const row=exchangeRow(),library=platform==="darwin"?protocol.exchange.darwin.library:process.arch==="arm64"?protocol.exchange.linux.arm64:protocol.exchange.linux.x64,symbol=row.symbol,error=row.error;
 const{dlopen,FFIType,read}=createRequire(import.meta.url)("bun:ffi")as typeof import("bun:ffi");
 let binding:ReturnType<typeof BunDlopen>;try{binding=dlopen(library,{[symbol]:{args:[FFIType.i32,FFIType.ptr,FFIType.i32,FFIType.ptr,FFIType.u32],returns:FFIType.i32},[error]:{args:[],returns:FFIType.ptr}});}catch(cause){throw Error(protocol.refusals.unsupported+": "+library+" lacks "+symbol,{cause});}
 const call=binding.symbols[symbol]as unknown as(...args:unknown[])=>number,locate=binding.symbols[error]as unknown as()=>number;
 return Object.freeze({exchange:(first:string,second:string)=>{const status=Number(call(row.atFdcwd,Buffer.from(first+"\0"),row.atFdcwd,Buffer.from(second+"\0"),row.flag));return Object.freeze({status,errno:status===0?0:read.i32(locate()as never)});}});
}

/** 🔁️ Builds the atomic platform port: one exchange call, nothing journaled, nothing to recover. */
export function createNativeDirectoryExchangePortV1(call:NativeDirectoryExchangeCallV1=nativeDirectoryExchangeCallV1(),flush:(path:string)=>void=flushNativePosixDirectoryV1):NativeDirectoryPlatformPortV1{
 return Object.freeze({kind:"exchange"as const,
  swap:(first:string,second:string)=>{const outcome=call.exchange(first,second);if(outcome.status===0)return;if((exchangeRow().unsupported as readonly number[]).includes(outcome.errno))throw Error(protocol.refusals.unsupported+" (errno "+outcome.errno+")");throw Error("Directory exchange failed (status "+outcome.status+", errno "+outcome.errno+")");},
  flush,pending:()=>false,recover:()=>"clean"as const,prepareVolume:()=>{}});
}
