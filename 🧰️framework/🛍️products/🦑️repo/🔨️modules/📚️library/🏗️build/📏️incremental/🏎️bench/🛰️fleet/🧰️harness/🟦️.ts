import {spawnSync} from "node:child_process";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {startRemoteCacheServerV1} from "../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {httpRemoteExecutionContentV1} from "../../../../🌐️remote-execution/🗄️content/🟦️.ts";
import {startRemoteExecutionServerV1} from "../../../../🌐️remote-execution/🖥️server/🟦️.ts";
import type {HostProbeV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";

/** 🧰️ The pinned compiler the probes run, named by environment so requester and workers see the same build. */
export const FLEET_TOOLCHAIN_V1="nightly-2026-07-20";
/** 🦀️ The environment of a compiler call: the host environment with the pinned toolchain, and what the caller adds. */
export const rustEnvironmentV1=(extra:Readonly<Record<string,string|undefined>>={}):Record<string,string>=>Object.fromEntries(Object.entries({...process.env,RUSTUP_TOOLCHAIN:FLEET_TOOLCHAIN_V1,...extra}).filter((row):row is [string,string]=>row[1]!==undefined));
/** #️⃣ A stable 16-hex `extra-filename` per crate name, the shape Cargo gives. */
export const externHashV1=(name:string):string=>createHash("sha256").update(name).digest("hex").slice(0,16);

/** 📁️ One scratch checkout of tiny crates below a fresh directory, so two checkouts differ in their absolute path; the directory name carries an emoji like the repository's. */
export type FleetCheckoutV1=Readonly<{base:string;root:string;deps:string;crate:(name:string,source:string)=>string;compile:(name:string,source:string,externs?:readonly string[])=>void;arguments:(name:string,externs?:readonly string[])=>string[];environment:(name:string)=>Record<string,string>}>;

/** 📁️ Creates a scratch checkout under `scratch`. */
export function createFleetCheckoutV1(label:string,scratch:string):FleetCheckoutV1{
 const base=realpathSync(mkdtempSync(join(realpathSync(scratch),`fleet-${label}-`))),root=join(base,"ws✏️"),deps=join(root,"t","debug","deps");
 mkdirSync(deps,{recursive:true});
 const crate=(name:string,source:string):string=>{const directory=join(root,"pk",name);mkdirSync(join(directory,"src"),{recursive:true});writeFileSync(join(directory,"src","lib.rs"),source);writeFileSync(join(directory,"Cargo.toml"),`[package]\nname = "${name}"\nversion = "0.1.0"\n`);return directory;};
 const environment=(name:string)=>({CARGO_MANIFEST_DIR:join(root,"pk",name),CARGO_PKG_NAME:name,CARGO_PKG_VERSION:"0.1.0",CARGO_CRATE_NAME:name,CARGO_PRIMARY_PACKAGE:"1",RUST_MIN_STACK:"8388608"});
 const args=(name:string,externs:readonly string[]=[])=>["--crate-name",name,"--edition=2021",`pk/${name}/src/lib.rs`,"--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--crate-type","lib","--emit=dep-info,metadata,link","-C","embed-bitcode=no","-C","debuginfo=2","--remap-path-prefix",`${root}=.`,"-C",`metadata=${externHashV1(name)}`,"-C",`extra-filename=-${externHashV1(name)}`,"--out-dir",deps,"-L",`dependency=${deps}`,...externs.flatMap(external=>["--extern",`${external}=${join(deps,`lib${external}-${externHashV1(external)}.rlib`)}`])];
 const compile=(name:string,source:string,externs:readonly string[]=[])=>{
  crate(name,source);
  const result=spawnSync("rustc",args(name,externs),{cwd:root,encoding:"utf8",env:rustEnvironmentV1(environment(name))});
  if(result.status!==0)throw Error(`rustc failed for ${name}: ${result.stderr}`);
 };
 return Object.freeze({base,root,deps,crate,compile,arguments:args,environment});
}

const PRESSURED_VM_STAT="Mach Virtual Memory Statistics: (page size of 16384 bytes)\nPages free:                                    18078.\nPages active:                                 783944.\nPages inactive:                               778846.\nPages speculative:                              3800.\nPages throttled:                                   0.\nPages wired down:                             170347.\nPages purgeable:                                5079.\nFile-backed pages:                            388587.\nAnonymous pages:                             1178003.\nPages stored in compressor:                   625878.\nPages occupied by compressor:                 293529.\nPageins:                                    14533076.\nPageouts:                                        148.\nSwapins:                                           0.\nSwapouts:                                          0.\n";
/** 🌡️ A host probe that reports a machine under critical memory pressure with nine gigabytes of swap in use, so that the local admission says wait whatever this machine really is. */
export function pressuredHostProbeV1():HostProbeV1{
 return {platform:"darwin" as const,read:async()=>null,run:async(command:string,args:readonly string[])=>command==="vm_stat"?PRESSURED_VM_STAT:command==="sysctl"&&args.includes("vm.swapusage")?"total = 12.00G  used = 9.50G  free = 2.50G  (encrypted)\n":command==="sysctl"&&args.includes("kern.memorystatus_vm_pressure_level")?"4\n":""} as unknown as HostProbeV1;
}

/** 🧮️ A rustc stand-in that counts every compile of a crate in a log and then runs the real compiler. */
export function countingRustcV1(directory:string):Readonly<{path:string;calls:()=>number}>{
 const path=join(directory,"counting-rustc.sh"),log=join(directory,"rustc-calls.log");
 writeFileSync(path,`#!/bin/sh\ncase " $* " in *" --crate-name "*) echo call >> '${log}';; esac\nexec rustc "$@"\n`);chmodSync(path,0o755);
 return {path,calls:()=>existsSync(log)?readFileSync(log,"utf8").split("\n").filter(Boolean).length:0};
}

/** ⏳️ Polls until the check holds or the time is spent. */
export async function eventuallyV1(check:()=>Promise<boolean>,milliseconds:number,what:string):Promise<void>{
 for(const end=Date.now()+milliseconds;Date.now()<end;await new Promise(done=>setTimeout(done,250)))if(await check().catch(()=>false))return;
 throw Error(`Timed out waiting for ${what}\nZeitüberschreitung beim Warten auf ${what}`);
}
/** ⏱️ Answers the work's value, or `timeout` when it does not finish in time. */
export function withinV1<T>(work:Promise<T>,milliseconds:number):Promise<T|"timeout">{
 let timer:ReturnType<typeof setTimeout>|undefined;
 return Promise.race([work,new Promise<"timeout">(done=>{timer=setTimeout(()=>done("timeout"),milliseconds);})]).finally(()=>{if(timer)clearTimeout(timer);});
}

/** 🛰️ The environment of a worker process of the loopback fleet: the pinned toolchain, an idle host view and the worker's permission to run any action (its own, scoped to this child). */
export const workerEnvironmentV1=(hostView:string):Record<string,string>=>rustEnvironmentV1({SEMIO_CAS_ROOT:undefined,SEMIO_REMOTE_EXECUTION_HOST_VIEW:hostView,SEMIO_REMOTE_EXECUTION_WORKER_ACTIONS:"any",SEMIO_REMOTE_EXECUTION_UNCONFINED_ANY:"I-ACCEPT-THE-RISK"});

/** 🛰️ A loopback cache server with the execution scheduler beside it, as one machine of the fleet would run them. */
export type FleetServicesV1=Readonly<{cache:Awaited<ReturnType<typeof startRemoteCacheServerV1>>;server:Awaited<ReturnType<typeof startRemoteExecutionServerV1>>;stop():Promise<void>}>;
/** 🚀️ Starts the loopback services in `directory`. */
export async function startFleetServicesV1(directory:string):Promise<FleetServicesV1>{
 const cache=await startRemoteCacheServerV1({directory:join(directory,"cache"),host:"127.0.0.1",port:0,maximumBytes:1<<30,maximumEntryBytes:1<<28,maximumAgeDays:30,defaultInstance:"main",tokens:{read:undefined,write:undefined}});
 const content=httpRemoteExecutionContentV1({endpoint:cache.url.replace("//0.0.0.0","//127.0.0.1"),token:undefined});
 const server=await startRemoteExecutionServerV1({host:"127.0.0.1",grpcPort:0,workerPort:0,content,cacheUrl:cache.url,advertise:undefined,tokens:{},maximumMessageBytes:1<<25,maximumBatchBytes:1<<22,scheduler:{unmatchedSeconds:30,lingerSeconds:1,leaseSeconds:5,maximumAttempts:3,defaultActionSeconds:60,maximumActionSeconds:600,maximumQueued:1000,heartbeatSeconds:1,pollSeconds:2,loadExpireSeconds:15,refusalCooldownSeconds:2,sweepMilliseconds:250},onEvent:()=>{}});
 return Object.freeze({cache,server,stop:async()=>{await server.stop().catch(()=>{});await cache.stop().catch(()=>{});}});
}
