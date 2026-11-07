import{CARGO_TOOL_IDS_V1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🟦️.ts";
import{requireIssuedResourceControlInvocationV1,withIssuedResourceOperationV1,type ScriptInvocation}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type{ResourceLeaseControl}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{withRepositoryToolingStoreV1}from"../../🛠️tools/🟦️.ts";
import{withRepositoryRustToolchainV1,type RepositoryRustToolchainV1}from"../../../../🚀️bootstrap/🛠️tools/🦀️rustup/📥️installation/🟦️.ts";
import{ensureRepositoryWindowsCompilerV1,withRepositoryWindowsCompilerV1}from"../🛠️native-compiler/🪟️windows/🟦️.ts";

/** 🛠️ Carries the actual held native compiler environment through one consumer. */
export interface RepositoryNativeToolchainV1{readonly environment:Readonly<Record<string,string|undefined>>;readonly rust:RepositoryRustToolchainV1;recheck():Promise<void>}

/** 📸️ Retains caller choices before managed tools can update process environment. */
export function snapshotNativeCallerEnvironmentV1(environment:Readonly<Record<string,string|undefined>>):Readonly<Record<string,string|undefined>>{return Object.freeze({...environment});}

/** 🪢️ Preserves the Rust boundary check around both successful and failed consumers. */
export async function withNativeToolchainBoundaryV1<T>(check:()=>Promise<void>,body:()=>Promise<T>):Promise<T>{await check();let failed=false,failure:unknown,result:T|undefined;try{result=await body();}catch(error){failed=true;failure=error;}try{await check();}catch(error){if(failed)throw new AggregateError([failure,error],"Native toolchain consumer and Rust recheck failed");throw error;}if(failed)throw failure;return result!;}

/** 🔐️ Holds tools, pinned Rust and the verified Windows compiler for the complete consumer lifetime. */
export function withRepositoryNativeToolchainV1<T>(workspace:string,original:ScriptInvocation,control:ResourceLeaseControl,environment:Readonly<Record<string,string|undefined>>,body:(scope:RepositoryNativeToolchainV1,control:ResourceLeaseControl)=>Promise<T>):Promise<T>{
 requireIssuedResourceControlInvocationV1(control,original);
 const caller=snapshotNativeCallerEnvironmentV1(environment);
 return withRepositoryToolingStoreV1(workspace,original,control,(tools,toolsControl)=>withRepositoryRustToolchainV1(workspace,tools,toolsControl,async(rust,actual)=>{
  if(caller.RUSTUP_TOOLCHAIN&&![rust.channel,rust.environment.RUSTUP_TOOLCHAIN].includes(caller.RUSTUP_TOOLCHAIN))throw Error("Incoming compiler toolchain disagrees with Source pin");
  const projected={...rust.environment,...caller,...Object.fromEntries(CARGO_TOOL_IDS_V1.map(tool=>{const name="SEMIO_TOOL_"+tool.toUpperCase().replaceAll("-","_");return[name,rust.environment[name]];})),PATH:rust.environment.PATH,RUSTUP_HOME:rust.environment.RUSTUP_HOME,RUSTUP_TOOLCHAIN:rust.environment.RUSTUP_TOOLCHAIN,RUSTUP_AUTO_INSTALL:"0",RUSTUP_TRACE_DIR:undefined,RUSTUP_DIST_ROOT:undefined,RUSTUP_DIST_SERVER:rust.environment.RUSTUP_DIST_SERVER,RUSTUP_UPDATE_ROOT:rust.environment.RUSTUP_UPDATE_ROOT};
  await ensureRepositoryWindowsCompilerV1(workspace,tools,actual,caller,null);
  return withRepositoryWindowsCompilerV1(workspace,tools,actual,caller,projected,null,async windows=>{
   let live=true;const scope:RepositoryNativeToolchainV1=Object.freeze({environment:windows?.environment??projected,rust,recheck:()=>{if(!live)return Promise.reject(Error("Original native toolchain scope required"));return withIssuedResourceOperationV1(actual,async()=>{await rust.recheck();await windows?.recheck();});}});
   try{return await withNativeToolchainBoundaryV1(()=>rust.recheck(),()=>body(scope,actual));}finally{live=false;}
  });
 }));
}
