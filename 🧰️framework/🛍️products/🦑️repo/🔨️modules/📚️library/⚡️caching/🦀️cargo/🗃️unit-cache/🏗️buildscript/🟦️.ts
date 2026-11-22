import {spawn} from "node:child_process";
import {chmodSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import {join} from "node:path";
import {parseBuildDirectivesV1,toolchainIdentityV1,UNIT_CACHE_POLICY_V1,type RustcUnitV1} from "../🔏️key/🟦️.ts";
import {buildRunKeyInputV1,buildScriptIdentityV1} from "./🔑️run-key/🟦️.ts";
import {packRunV1} from "./📦️run-wire/🟦️.ts";
import {casRootV1} from "../../../../🏗️build/🗄️cas/🗃️store/🟦️.ts";
import {readCasObjectSyncV1} from "../../../../🏗️build/🗄️cas/🗃️store/🔁️sync/🟦️.ts";
import {compilerReleaseV1,localKeyV1,portableRootsV1,slotIdV1,toolRootsV1,type PortableRootV1} from "../🛰️fleet/🪪️identity/🟦️.ts";
import {awaitFleetSettlesV1,obtainFleetUnitV1,offerFleetUnitV1,prepareFleetV1,shareFleetUnitV1,unsharedReasonV1,type FleetHoldV1,type FleetUnitV1} from "../🛰️fleet/🟦️.ts";
import {countFallbackV1,countSkipV1,skipNameV1} from "../🛰️fleet/📊️counters/🟦️.ts";
import {acquireUnitClaimV1,type ClaimV1} from "../🔒️lock/🟦️.ts";
import {digestFileV1,emitUnitEventV1,lookupBuildRunV1,publishBuildRunV1,restoreBuildRunV1,type UnitEventV1} from "../💾️store/🟦️.ts";
import {jobserverChildEnvironmentV1,jobserverClosedMessageV1,jobserverPlanV1,jobserverProbeShellV1,jobserverStdioV1,shellQuoteV1} from "../🎰️jobserver/🟦️.ts";
import {isMainModuleV1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
import {exitWithStatusOfV1} from "../🏁️exit/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1;
type Environment=Readonly<Record<string,string|undefined>>;
const module=fileURLToPath(new URL("./🟦️.ts",import.meta.url));

/** 🏗️ Whether a rustc unit compiles a package's build script, the only executable whose runs are shared between lanes. */
export function isBuildScriptUnitV1(unit:RustcUnitV1):boolean{return unit.crateName.startsWith("build_script_")&&unit.crateTypes.every(type=>type==="bin")&&unit.emit.includes("link");}

/** 🎚️ Whether build script runs are shared: switched on by the launcher, and only where an executable text file can stand in for a binary. */
export function buildScriptsEnabledV1(environment:Environment,platform:NodeJS.Platform=process.platform):boolean{return environment.SEMIO_UNIT_CACHE_BUILD_SCRIPTS==="on"&&(policy.buildScript.platforms as readonly string[]).includes(platform);}

/** 🪞️ The text of the stand-in Cargo runs instead of the compiled build script: a POSIX `sh` script, because it must look at the descriptors Cargo handed down BEFORE the runtime opens descriptors of its own ({@link jobserverProbeShellV1}), then it starts this module, which consults the cache and otherwise runs the real executable beside it with the pool of Cargo's job tokens handed on. */
export function shimSourceV1(real:string,runtime:string=process.execPath):string{
 return["#!/bin/sh",`# ${policy.buildScript.shimMarker}`,...jobserverProbeShellV1(),`exec ${shellQuoteV1(runtime)} ${shellQuoteV1(module)} ${shellQuoteV1(real)} "$@"`,""].join("\n");
}

/** 🔁️ Replaces the compiled build script `artifact` by its stand-in; the real executable moves to `<artifact>.real`. Idempotent. */
export function installBuildScriptShimV1(artifact:string,runtime:string=process.execPath):string|undefined{
 const real=artifact+".real";
 if(!isShimV1(artifact)){rmSync(real,{force:true});renameSync(artifact,real);}
 rmSync(artifact,{force:true});writeFileSync(artifact,shimSourceV1(real,runtime));chmodSync(artifact,0o755);
 return real;
}

/** 🔎️ Whether a file is already a stand-in (a script carrying the marker), not a compiled executable. */
export function isShimV1(path:string):boolean{
 try{const head=readFileSync(path).subarray(0,256).toString("latin1");return head.startsWith("#!")&&head.includes(policy.buildScript.shimMarker);}catch{return false;}
}

function runReal(real:string,args:readonly string[],environment:Environment,cwd:string,capture:boolean):Promise<{status:number;stdout:string;stderr:string}>{
 return new Promise((done,fail)=>{
  const jobs=jobserverPlanV1(environment),message=jobserverClosedMessageV1(jobs);
  if(message)process.stderr.write(message+"\n");
  const child=spawn(real,[...args],{cwd,env:jobserverChildEnvironmentV1(environment,jobs),stdio:jobserverStdioV1(jobs.descriptors,capture)}),out:Buffer[]=[],err:Buffer[]=[];
  for(const signal of["SIGINT","SIGTERM"] as const)process.once(signal,()=>child.kill(signal));
  if(capture){child.stdout!.on("data",chunk=>{out.push(chunk);process.stdout.write(chunk);});child.stderr!.on("data",chunk=>{err.push(chunk);process.stderr.write(chunk);});}
  child.on("error",fail);
  child.on("close",(status,signal)=>done({status:status??(signal?128+({SIGINT:2,SIGTERM:15,SIGKILL:9} as Record<string,number>)[signal]!:1),stdout:Buffer.concat(out).toString("utf8"),stderr:Buffer.concat(err).toString("utf8")}));
 });
}

const RUN_UNIT_V1:RustcUnitV1=Object.freeze({crateName:"build_script_run",crateTypes:Object.freeze(["lib"]),emit:Object.freeze(["link"]),outDir:"",source:"",target:undefined,externs:Object.freeze([]),links:Object.freeze([]),nativeSearch:false,incremental:undefined,artifacts:false});

/** 🛰️ The fleet identity of one build script run: its portable key (the run's key inputs with every root of the machine as a token, so the same run has the same key on every machine of the platform), its slot (package, version, target and compiler build), the host platform and the roots. `undefined` when the fleet tier is off or the compiler build cannot be determined. */
export function prepareRunFleetV1(input:Readonly<{root:string;environment:Environment;inner:readonly string[];cwd:string;laneRoot:string|undefined;localKey:string;keyInput:Parameters<typeof localKeyV1>[0];crate:string}>):FleetUnitV1|undefined{
 try{
  const release=compilerReleaseV1(input.root,input.inner,input.environment),environment=input.environment;
  const parts=[release.digest,release.host,"build-script-run",(environment.CARGO_PKG_NAME??"")+"@"+(environment.CARGO_PKG_VERSION??""),environment.TARGET??"",environment.CARGO_MANIFEST_LINKS??""];
  const fleet=prepareFleetV1({root:input.root,environment,basics:{release,slot:slotIdV1(parts),slotParts:parts},unit:RUN_UNIT_V1,args:[],cwd:input.cwd,laneRoot:input.laneRoot,localKey:input.localKey,keyInput:input.keyInput,toolchain:input.keyInput.toolchain,crate:input.crate,tools:toolRootsV1(environment)});
  return fleet===undefined?undefined:Object.freeze({...fleet,run:true as const});
 }catch{return undefined;}
}

/** ▶️ Runs a package's build script through the cache. A hit writes the recorded OUT_DIR and replays the recorded `cargo:` lines without starting the script; a miss first asks the fleet cache (the same unit tier as compiled units: lookup, claim, wait for the machine that runs it, fetch) and otherwise runs the script once across all lanes and records its OUT_DIR, output and the facts that make it rerun (declared paths, the package tree, declared environment variables); a run that names no directory of this machine is then offered to the fleet. Any fault is a miss. */
export async function runBuildScriptV1(real:string,args:readonly string[],environment:Environment,cwd:string):Promise<number>{
 const root=environment.SEMIO_UNIT_CACHE_ROOT,outDir=environment.OUT_DIR;
 if(!root||!outDir||!buildScriptsEnabledV1(environment))return(await runReal(real,args,environment,cwd,false)).status;
 const lane=environment.SEMIO_UNIT_CACHE_LANE??"d",laneRoot=environment.SEMIO_UNIT_CACHE_LANE_ROOT,stampDirectory=laneRoot?join(laneRoot,policy.layout.stamps):undefined,crate=environment.CARGO_PKG_NAME??"build-script",workspace=environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||undefined;
 const event=(kind:UnitEventV1["kind"],key:string|null,milliseconds:number,waitedMs=0,reason?:string)=>emitUnitEventV1(root,{lane,kind,crate,key,milliseconds,waitedMs,...(reason?{reason}:{})});
 const started=Date.now();let digest:string|null=null,key:string,keyInput:Parameters<typeof localKeyV1>[0],roots:readonly PortableRootV1[],inner:string[];
 try{
  digest=digestFileV1(real,stampDirectory);if(digest===null)throw Error("real");
  inner=environment.SEMIO_UNIT_CACHE_INNER?JSON.parse(environment.SEMIO_UNIT_CACHE_INNER) as string[]:["rustc"];
  roots=portableRootsV1({cwd,workspace,hints:[environment.CARGO_MANIFEST_DIR,environment.PWD],...toolRootsV1(environment)});
  keyInput=buildRunKeyInputV1({toolchain:toolchainIdentityV1(root,inner,environment),identity:buildScriptIdentityV1(real,digest!),args,environment,laneRoot});
  key=localKeyV1(keyInput,roots);
 }catch{const outcome=await runReal(real,args,environment,cwd,false);event("script-bypass",null,Date.now()-started,0,"unreadable-input");return outcome.status;}
 const context={cwd,laneRoot,environment,stampDirectory},serve=():boolean=>{
  const entry=lookupBuildRunV1(root,key,context);if(!entry)return false;
  const messages=restoreBuildRunV1(root,entry,{outDir,laneRoot});process.stdout.write(messages.stdout);process.stderr.write(messages.stderr);return true;
 };
 const fleet=prepareRunFleetV1({root,environment,inner,cwd,laneRoot,localKey:key,keyInput,crate});
 try{if(serve()){event("script-hit",key,Date.now()-started);if(fleet&&environment.SEMIO_REMOTE_CACHE_ACCESS==="read-write")offerFleetUnitV1(fleet);return 0;}}catch{}
 let claim:ClaimV1|undefined;
 try{claim=await acquireUnitClaimV1(join(root,policy.layout.locks),"run-"+key.slice(0,36),{});}catch{claim=undefined;}
 const waited=claim?.waitedMs??0;let hold:FleetHoldV1|undefined;
 try{
  if(claim){try{if(serve()){event("script-hit",key,Date.now()-started,waited,"after-wait");return 0;}}catch{}}
  if(fleet&&fleet.client.state()==="ok"){
   const decision=await obtainFleetUnitV1(fleet,(kind,milliseconds,waitedMs,reason)=>event(kind,key,milliseconds,waitedMs,reason));
   if(decision.outcome==="hit"){try{if(serve()){event("script-hit",key,Date.now()-started,waited+decision.waitedMs,"fleet");await awaitFleetSettlesV1();return 0;}}catch{}countFallbackV1(root,"run-hit-unservable");}
   else{hold=decision.hold;if(decision.reason!=="granted")countFallbackV1(root,"run-"+decision.reason);}
  }
  const compiled=Date.now(),outcome=await runOffloadedV1({real,digest,args,environment,cwd,root,lane,crate,key,outDir,stampDirectory,capture:()=>runReal(real,args,environment,cwd,true)});
  if(outcome.status!==0){await hold?.release();event("failed",key,Date.now()-compiled,waited);return outcome.status;}
  event("script-miss",key,Date.now()-compiled,waited);
  let entry:ReturnType<typeof publishBuildRunV1>;
  try{
   entry=publishBuildRunV1(root,{key,package:crate,cwd,laneRoot,directives:parseBuildDirectivesV1(outcome.stdout),environment,outDir,stdout:outcome.stdout,stderr:outcome.stderr,stampDirectory});
   event(entry?"script-publish":"script-bypass",key,Date.now()-compiled,0,entry?undefined:"not-cacheable");
  }catch{entry=undefined;}
  if(fleet)await shareRunV1(fleet,{entry,hold,workspace,environment,costMs:Date.now()-compiled,key,event});else await hold?.release();
  return 0;
 }finally{claim?.release();}
}

/** ☁️ Produces the outcome of one build script run on a fleet miss, after the fleet claim: on an idle machine of the fleet when the script is hermetic (`offloadBuildScriptRunV1`, in `distribute` mode, same cancellation and timeouts as every command offload), its streams replayed here exactly as a local run prints them; otherwise, or on any fault, by running the real executable here with the reason recorded. */
async function runOffloadedV1(input:Readonly<{real:string;digest:string|null;args:readonly string[];environment:Environment;cwd:string;root:string;lane:string;crate:string;key:string;outDir:string;stampDirectory:string|undefined;capture:()=>Promise<{status:number;stdout:string;stderr:string}>}>):Promise<{status:number;stdout:string;stderr:string}>{
 if(input.digest!==null){
  const controller=new AbortController(),abort=()=>controller.abort(),signals=["SIGINT","SIGTERM"] as const;
  for(const signal of signals)process.once(signal,abort);
  try{
   const {offloadBuildScriptRunV1}=await import("./☁️offload/🟦️.ts"),answer=await offloadBuildScriptRunV1({real:input.real,args:input.args,environment:input.environment,cwd:input.cwd,root:input.root,lane:input.lane,crate:input.crate,key:input.key,outDir:input.outDir,signal:controller.signal,digest:path=>digestFileV1(path,input.stampDirectory)});
   if(answer.kind==="done"){process.stdout.write(answer.stdout);process.stderr.write(answer.stderr);return{status:0,stdout:answer.stdout,stderr:answer.stderr};}
   if(controller.signal.aborted)return{status:130,stdout:"",stderr:""};
  }catch{}
  finally{for(const signal of signals)process.removeListener(signal,abort);}
 }
 return input.capture();
}

/** 📮️ Offers a published run to the fleet: the run is packed with every root of the machine as a token (also what the upload worker will send), and one that still names a directory of this machine (a file or stream with the home directory, the lane or an unlisted checkout in it) stays local with its reason recorded; otherwise it is queued for upload with the claim handed over. */
async function shareRunV1(fleet:FleetUnitV1,input:Readonly<{entry:ReturnType<typeof publishBuildRunV1>;hold:FleetHoldV1|undefined;workspace:string|undefined;environment:Environment;costMs:number;key:string;event:(kind:UnitEventV1["kind"],key:string|null,milliseconds:number,waitedMs?:number,reason?:string)=>void}>):Promise<void>{
 const {entry,hold,workspace,environment}=input;
 if(entry===undefined){await hold?.release();return;}
 const cas=casRootV1(),packed=packRunV1(entry,{key:fleet.key,localKey:fleet.localKey,slot:fleet.slot,platform:fleet.platform,costMs:input.costMs},fleet.roots,digest=>readCasObjectSyncV1(cas,digest),[environment.HOME,environment.USERPROFILE,fleet.laneRoot]);
 if(typeof packed==="string"){
  input.event("remote-skip",input.key,0,0,packed.startsWith("path-dependent")?packed:"run: "+packed);countSkipV1(workspace,"run-"+skipNameV1(packed));await hold?.release();return;
 }
 if(!shareFleetUnitV1(fleet,{claimed:hold,costMs:input.costMs})){countSkipV1(workspace,unsharedReasonV1(fleet));await hold?.release();}
}

/** 🏁️ What the stand-in runs: the real executable is the first argument, the arguments Cargo passed follow. Resolves to the exit status. */
export function runBuildScriptMainV1(argv:readonly string[],environment:Environment,cwd:string):Promise<number>{return runBuildScriptV1(argv[0]!,argv.slice(1),environment,cwd);}

if(isMainModuleV1(import.meta.url))exitWithStatusOfV1(()=>runBuildScriptMainV1(process.argv.slice(2),process.env,process.cwd()));
