#!/usr/bin/env repo
import {spawn,spawnSync} from "node:child_process";
import {existsSync,mkdtempSync,mkdirSync,readFileSync,readdirSync,realpathSync,rmdirSync,statSync,unlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {basename,join,resolve} from "node:path";
import {admitFactsV1,expandRustcArgumentsV1,keyEnvironmentV1,laneRestoreV1,normalizeRustcArgumentsV1,parseRustcUnitV1,rustcBypassV1,rustSysrootV1,sha256V1,toolchainIdentityV1,withoutIncrementalV1,unitKeyComponentsV1,unitKeyPartsV1,unitKeyV1,UNIT_CACHE_POLICY_V1} from "./🔏️key/🟦️.ts";
import {acquireSlotV1,acquireUnitClaimV1,type ClaimV1} from "./🔒️lock/🟦️.ts";
import {digestFileV1,emitUnitEventV1,forgetCrateEntriesV1,lookupUnitV1,publishUnitV1,churnReportV1,readEvictionsV1,readLookupsV1,recordLookupV1,restoreUnitV1,type UnitEventV1} from "./💾️store/🟦️.ts";
import {compilerTemporaryArgumentsV1,lanePathsV1} from "./🧵️lane/🟦️.ts";
import {jobserverChildEnvironmentV1,jobserverClosedMessageV1,jobserverPlanV1,jobserverStdioV1} from "./🎰️jobserver/🟦️.ts";
import {buildScriptsEnabledV1,installBuildScriptShimV1,isBuildScriptUnitV1} from "./🏗️buildscript/🟦️.ts";
import {brokerOfV1,recallProvenanceV1,replayCachedUnitV1} from "./🛰️broker/🟦️.ts";
import {collectGarbageSingleFlightV1,collectGarbageV1,measureGcV1,measureVolumeV1,planGcV1,watchUnitCachePressureV1} from "./🧹️gc/🟦️.ts";
import {fillRatesV1,readFillSamplesV1} from "./🧹️gc/📈️fill/🟦️.ts";
import {keyGapV1,sourceEnvironmentV1,unitSourcesV1} from "./🔏️key/🔗️sources/🟦️.ts";
import {linkRootArgumentsV1,remappedNameV1,rustSourceRemapArgumentsV1,remappedV1,remapsOfV1} from "./🧭️remap/🟦️.ts";
import {pathSensitiveV1} from "./🧭️remap/🔍️embedding/🟦️.ts";
import {externIdentityV1,markExternIdentityV1} from "./🪪️extern/🟦️.ts";
import {currentHashV1,dependencyDirectoriesV1,interfaceEligibilityV1,interfaceIdentityV1,rebaseRestoredUnitV1,restoredClosureViolationV1,settleUnitInterfaceV1,writeDependenciesV1} from "./🔭️interface/🟦️.ts";
import {announcedPathsV1,expectedOutputsV1,missingOutputsV1,uncoveredOutputsV1} from "./✅️complete/🟦️.ts";
import {localKeyV1,platformOfV1,portableKeyV1,portableRootsV1,portableTextV1} from "./🛰️fleet/🪪️identity/🟦️.ts";
import {awaitFleetSettlesV1,fleetUnidentifiedReasonV1,holdFleetKeyV1,leakedRootKindsV1,obtainFleetUnitV1,offerFleetUnitV1,prepareFleetV1,publishRootHintV1,rootHintV1,rootPathsV1,ROOT_KINDS_V1,shareFleetUnitV1,unitBasicsV1,unsharedReasonV1,unitToolchainV1,type FleetHoldV1,type FleetUnitV1} from "./🛰️fleet/🟦️.ts";
import {drainUploadsV1} from "./🛰️fleet/📤️uploader/🟦️.ts";
import {countFallbackV1,countSkipV1,countUnitV1,readFallbacksV1,skipNameV1} from "./🛰️fleet/📊️counters/🟦️.ts";
import {OFFLOAD_POLICY_V1,offloadModeV1} from "../../../🏗️build/🌐️remote-execution/🚀️offload/📐️policy/🟦️.ts";
import type {OffloadAdmissionPortsV1} from "../../../🏗️build/🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
import {exitWithStatusOfV1,UNFINISHED_EXIT_STATUS_V1} from "./🏁️exit/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1;
type Environment=Readonly<Record<string,string|undefined>>;
type Settings=Readonly<{root:string;lane:string;laneRoot:string|undefined;inner:readonly string[];slots:number;focus:ReadonlySet<string>;stamps:string|undefined}>;

/** ⚙️ Reads the wrapper's settings, which the repository launcher exports; `undefined` when the cache is off. */
export function readSettingsV1(environment:Environment):Settings|undefined{
 const root=environment.SEMIO_UNIT_CACHE_ROOT;if(!root||environment.SEMIO_UNIT_CACHE==="off")return undefined;
 const lane=environment.SEMIO_UNIT_CACHE_LANE??"d",inner=environment.SEMIO_UNIT_CACHE_INNER?JSON.parse(environment.SEMIO_UNIT_CACHE_INNER) as string[]:["rustc"];
 if(!Array.isArray(inner)||inner.length<1||inner.some(part=>typeof part!=="string"))throw Error("SEMIO_UNIT_CACHE_INNER must be a JSON array of strings");
 const laneRoot=environment.SEMIO_UNIT_CACHE_LANE_ROOT;
 return Object.freeze({root:resolve(root),lane,laneRoot,inner:Object.freeze(inner),slots:Number(environment.SEMIO_UNIT_CACHE_SLOTS??0),focus:new Set((environment.SEMIO_UNIT_CACHE_FOCUS??"").split(",").filter(Boolean)),stamps:laneRoot?join(laneRoot,policy.layout.stamps):lanePathsV1(resolve(root),lane).stamps});
}

type Outcome=Readonly<{status:number;stdout:string;stderr:string}>;
/** 📜️ Keeps existing response references and unrepresentable arguments in place while shortening a Windows compiler invocation. */
function compilerArgumentsV1(inner:readonly string[],args:readonly string[]):Readonly<{args:readonly string[];close:()=>void}>{
 if(process.platform!=="win32"||[...inner,...args].reduce((n,arg)=>n+2*arg.length+3,0)<32767)return{args,close:()=>{}};
 const directory=mkdtempSync(join(tmpdir(),"semio-rustc-")),files:string[]=[],result:string[]=[],pending:string[]=[];
 const close=()=>{for(const file of files)try{unlinkSync(file);}catch{}try{rmdirSync(directory);}catch{}};
 const flush=()=>{if(!pending.length)return;const file=join(directory,String(files.length)+".rsp");files.push(file);writeFileSync(file,pending.join("\n")+"\n",{flag:"wx"});result.push("@"+file);pending.length=0;};
 try{for(const arg of args){if(arg===""||arg.startsWith("@")||/[\r\n]/u.test(arg)){flush();result.push(arg);}else pending.push(arg);}flush();return{args:result,close};}catch(error){close();throw error;}
}
function run(inner:readonly string[],args:readonly string[],environment:Environment,capture:boolean):Promise<Outcome>{
 return new Promise((done,fail)=>{
  const jobs=jobserverPlanV1(environment),message=jobserverClosedMessageV1(jobs);
  if(message)process.stderr.write(message+"\n");
  const prepared=compilerArgumentsV1(inner,args);let child:ReturnType<typeof spawn>;
  try{child=spawn(inner[0]!,[...inner.slice(1),...prepared.args],{env:jobserverChildEnvironmentV1(environment,jobs),stdio:jobserverStdioV1(jobs.descriptors,capture)});}catch(error){prepared.close();jobs.release?.();throw error;}
  const out:Buffer[]=[],err:Buffer[]=[];let failure:Error|undefined;
  const forward=(signal:NodeJS.Signals)=>{child.kill(signal);};
  for(const signal of["SIGINT","SIGTERM"] as const)process.once(signal,()=>forward(signal));
  if(capture){child.stdout!.on("data",chunk=>{out.push(chunk);process.stdout.write(chunk);});child.stderr!.on("data",chunk=>{err.push(chunk);process.stderr.write(chunk);});}
  child.on("error",error=>{failure??=error;});
  child.on("close",(status,signal)=>{prepared.close();jobs.release?.();if(failure){fail(failure);return;}done({status:status??(signal?128+({SIGINT:2,SIGTERM:15,SIGKILL:9} as Record<string,number>)[signal]!:1),stdout:Buffer.concat(out).toString("utf8"),stderr:Buffer.concat(err).toString("utf8")});});
 });
}

/** 🪞️ Drops the compiler path Cargo puts before the arguments when it runs the stand-in as `<wrapper> <rustc> <arguments>`; every compiler argument list starts with a flag, a response file or a stdin dash. */
export function withoutCompilerPathV1(argv:readonly string[]):readonly string[]{
 const first=argv[0];
 return first!==undefined&&!first.startsWith("-")&&!first.startsWith("@")&&!first.endsWith(".rs")?argv.slice(1):argv;
}

/** 🧭️ Appended to every compilation that Cargo gave a `--remap-path-scope` (the repository's `trim-paths = "object"` profile remaps only object code): rustc then remaps the source names of the metadata as well, so a unit gives the same bytes on every machine and at every checkout path, whoever compiles it (an extern's content hash is part of the key of its dependants). The scope Cargo gave is replaced (rustc refuses the option twice). */
export const PATH_NEUTRAL_FLAGS_V1:readonly string[]=Object.freeze(["--remap-path-scope=all"]);

/** 🔍️ The first compiled output (not the dependency file) whose bytes contain one of this machine's roots or their resolved spelling (the checkout, the Cargo home and the Rust home among them), as `<file>: <root>`, `undefined` when none does: such a unit holds a real path of this machine and may be shared only between machines with the same paths (`pathSensitiveV1`), never under a key that ignores them. */
export function leakedPathsV1(outputs:readonly string[],roots:readonly (string|undefined)[]):string|undefined{
 const given=roots.filter((root):root is string=>root!==undefined&&root.length>=4),known=[...new Set([...given,...given.flatMap(root=>{try{return[realpathSync(root)];}catch{return[];}})])];
 for(const path of outputs){
  if(path.endsWith(".d"))continue;
  let bytes:Buffer;try{bytes=readFileSync(path);}catch{continue;}
  for(const root of known)if(bytes.includes(root))return path.split("/").at(-1)+": "+root;
 }
 return undefined;
}

/** 🛠️ Serves one rustc command line from the cache or compiles it exactly once across all lanes. Never throws on cache faults: any fault is a miss. A miss compiles the command line Cargo gave, so a workspace unit Cargo compiles incrementally keeps rustc's incremental session in the lane (the next edit of that crate, also when it is a dependency of the focused one, recompiles incrementally) and is published like every other unit; the focused crates are served and published like all others (every machine holds the same synchronized worktree, so a crate compiled on one machine is a hit on the others), and a focused miss still compiles incrementally in the lane. */
export async function wrapRustcV1(argv:readonly string[],environment:Environment,cwd:string,ports:Readonly<{admission?:OffloadAdmissionPortsV1}>={}):Promise<number>{
 argv=withoutCompilerPathV1(argv);const settings=readSettingsV1(environment);
 if(settings&&argv.some(argument=>argument.startsWith("--remap-path-scope")))argv=[...argv.filter(argument=>!argument.startsWith("--remap-path-scope")),...PATH_NEUTRAL_FLAGS_V1];
 if(!settings)return(await run(environment.SEMIO_UNIT_CACHE_INNER?JSON.parse(environment.SEMIO_UNIT_CACHE_INNER):["rustc"],argv,environment,false)).status;
 const moduleRemapped=async():Promise<readonly string[]|undefined>=>{
  const manifest=environment.CARGO_MANIFEST_DIR,workspace=environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||undefined;
  if(manifest===undefined||workspace===undefined||!(cwd===manifest||cwd.startsWith(manifest+"/")||cwd.startsWith(manifest+"\\")))return undefined;
  try{return (await import("../../../🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts")).moduleRemapArgumentsV1(expandRustcArgumentsV1(argv,cwd),environment,cwd,workspace);}catch{return undefined;}
 };
 argv=(await moduleRemapped())??argv;
 const sourceRemapped=():readonly string[]=>{
  if(!argv.some(argument=>argument.startsWith("--remap-path-scope")))return argv;
  try{
   const commit=/^commit-hash: ([0-9a-f]{40})$/mu.exec(toolchainIdentityV1(settings.root,settings.inner,environment))?.[1],extra=rustSourceRemapArgumentsV1(expandRustcArgumentsV1(argv,cwd),rustSysrootV1(settings.root,settings.inner,environment),commit);
   if(extra.length===0)return argv;
   const first=argv.findIndex(argument=>argument==="--remap-path-prefix"||argument.startsWith("--remap-path-prefix=")),at=first<0?argv.length:first;
   return Object.freeze([...argv.slice(0,at),...extra,...argv.slice(at)]);
  }catch{return argv;}
 };
 argv=sourceRemapped();
 const args=expandRustcArgumentsV1(argv,cwd),unit=parseRustcUnitV1(args),bypass=rustcBypassV1(unit,args),crate=unit.crateName||"-";
 const event=(kind:UnitEventV1["kind"],key:string|null,milliseconds:number,waitedMs=0,reason?:string)=>emitUnitEventV1(settings.root,{lane:settings.lane,kind,crate,key,milliseconds,waitedMs,...(reason?{reason}:{})});
 let shared=false;
 const compile=async(capture:boolean):Promise<Outcome>=>{
  let slot:ClaimV1|undefined;
  if(settings.slots>0&&unit.source.endsWith(".rs")){slot=await acquireSlotV1(settings.root,settings.slots,{onWait:waited=>event("slot-wait",null,0,waited)});}
  try{return await run(settings.inner,[...(shared&&unit.incremental!==undefined&&!settings.focus.has(crate)?withoutIncrementalV1(argv):argv),...linkRootArgumentsV1(args,resolve(cwd,unit.outDir)),...compilerTemporaryArgumentsV1(settings.laneRoot?join(settings.laneRoot,policy.layout.compilerTemporaries):lanePathsV1(settings.root,settings.lane).temporaries,args)],environment,capture);}finally{slot?.release();}
 };
 const started=Date.now(),workspaceOf=environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||undefined;
 if(bypass){
  const outcome=await compile(false);
  if(unit.source.endsWith(".rs")){event("bypass",null,Date.now()-started,0,bypass);if(environment.SEMIO_REMOTE_CACHE_ENDPOINT)countSkipV1(workspaceOf,"bypass-"+skipNameV1(bypass));}
  return outcome.status;
 }
 const expected=expectedOutputsV1(unit,args);
 let key:string,parts="",keyInput:Parameters<typeof unitKeyV1>[0],sourcesRead:Awaited<ReturnType<typeof unitSourcesV1>>=null,keyedNames:ReadonlySet<string>=new Set();
 const digests=new Map<string,string>(),metadataOnly=unit.emit.every(kind=>kind==="metadata"||kind==="dep-info");let ifaceKeyed=false;
 const remaps=remapsOfV1(args),covered=remappedV1(remaps,cwd),manifestDir=environment.CARGO_MANIFEST_DIR,roots=portableRootsV1({cwd,workspace:workspaceOf,hints:[manifestDir,environment.PWD]});
 let sensitive:readonly string[]=[];
 try{
  const sourceFile=resolve(cwd,unit.source),sources=await unitSourcesV1(sourceFile,manifestDir,environment.OUT_DIR,path=>digestFileV1(path,settings.stamps));if(sources===null)throw Error("source");
  const sourceSha=sources.sha256;sourcesRead=sources;
  for(const row of unit.externs){if(row.path==="")continue;const path=resolve(cwd,row.path),sha=digestFileV1(path,settings.stamps);if(sha===null)throw Error("extern "+row.name);const identity=metadataOnly?interfaceIdentityV1(path,sha):undefined;if(identity!==undefined)ifaceKeyed=true;digests.set(row.path,identity??externIdentityV1(path,sha,settings.stamps)??sha);}
  sensitive=covered?pathSensitiveV1({source:resolve(cwd,unit.source),manifest:manifestDir,cwd}):[...new Set([manifestDir,cwd].filter((path):path is string=>path!==undefined))].sort();
  keyInput={toolchain:unitToolchainV1({root:settings.root,environment,inner:settings.inner,unit,args},()=>toolchainIdentityV1(settings.root,settings.inner,environment)),normalized:normalizeRustcArgumentsV1(args,digests,settings.laneRoot),sourceSha256:sourceSha,externs:unit.externs.map(row=>({name:row.flags+row.name,sha256:row.path===""?"builtin":digests.get(row.path)!})),environment:[...keyEnvironmentV1(environment,settings.laneRoot),...sourceEnvironmentV1(sources.names,environment,new Set(keyEnvironmentV1(environment,settings.laneRoot).map(row=>row[0])),(name,text)=>keyEnvironmentV1({[name]:text},settings.laneRoot)[0]?.[1]??text),...covered?[["⟨recorded-name⟩",remappedNameV1(remaps,cwd)] as [string,string]]:[]]};key=localKeyV1(keyInput,roots,sensitive);parts=unitKeyPartsV1(keyInput);keyedNames=new Set([...keyInput.environment.map(row=>row[0]),...sources.names]);
 }catch(error){const outcome=await compile(false);event("bypass",null,Date.now()-started,0,"unreadable-input"+(/^extern /u.test(String((error as Error)?.message))?": "+(error as Error).message:""));return outcome.status;}
 const producer=interfaceEligibilityV1(unit,args)===undefined,crateHashOf=(path:string):string|undefined=>currentHashV1(path,file=>digestFileV1(file,settings.stamps),settings.inner,environment);
 const settle=(outputs:readonly string[],entry:Parameters<typeof settleUnitInterfaceV1>[0]["entry"]):void=>{
  if(!ifaceKeyed&&!producer)return;
  try{settleUnitInterfaceV1({outputs,entry,producer,identityOf:digest=>sha256V1(JSON.stringify(["iface",portableKeyV1({...keyInput,sourceSha256:"iface:"+digest},roots),sensitive,entry.environment.map(row=>[row.name,row.value===null?null:portableTextV1(row.value,roots)])])),neutralName:path=>portableTextV1(path,roots),readFile:path=>{try{return readFileSync(resolve(cwd,laneRestoreV1(path,settings.laneRoot)));}catch{return null;}},digest:path=>digestFileV1(path,settings.stamps),inner:settings.inner,environment});}catch{}
 };
 const lookup=(outcome:"local-hit"|"fleet-hit"|"miss",remote?:string):void=>{
  const portable=keyInput.normalized.map(argument=>portableTextV1(argument,roots)),named=unitKeyComponentsV1({...keyInput,normalized:portable,environment:keyInput.environment.map(([name,value])=>[name,portableTextV1(value,roots)] as [string,string])},{platform:process.platform+"/"+process.arch,roots:JSON.stringify(keyInput.normalized.filter((argument,index)=>portable[index]!==argument))});
  const platform=()=>{try{const basics=unitBasicsV1({root:settings.root,environment,inner:settings.inner,unit,args,cwd});return basics?JSON.stringify(platformOfV1({release:basics.release,unit,args,environment,toolchain:keyInput.toolchain})):undefined;}catch{return undefined;}};
  const slot=unitBasicsV1({root:settings.root,environment,inner:settings.inner,unit,args,cwd})?.slot;
  recordLookupV1(settings.root,{lane:settings.lane,crate,key,...(remote===undefined?{}:{remote}),...(slot===undefined?{}:{slot}),outcome,components:named.components,values:named.values,...environment.SEMIO_UNIT_CACHE_TRACE_KEYS?{trace:{args:portable,environment:keyInput.environment.map(([name,value])=>[name,portableTextV1(value,roots)] as [string,string]),...platform()===undefined?{}:{platform:platform()!}}}:{}});
 };
 const context={cwd,laneRoot:settings.laneRoot,environment,stampDirectory:settings.stamps},outDir=resolve(cwd,unit.outDir),shimming=isBuildScriptUnitV1(unit)&&buildScriptsEnabledV1(environment),facts=brokerOfV1(environment)?admitFactsV1(args,environment,cwd):undefined;
 const shim=(outputs:readonly string[]):void=>{if(!shimming)return;const artifact=outputs.find(path=>!path.endsWith(".d")&&!path.endsWith(".rmeta"));if(artifact)installBuildScriptShimV1(artifact);};
 const serve=async():Promise<boolean>=>{
  const entry=lookupUnitV1(settings.root,key,{...context,expected});if(!entry)return false;
  const messages=restoreUnitV1(settings.root,entry,{outDir,laneRoot:settings.laneRoot,stampDirectory:settings.stamps}),absent=missingOutputsV1({directory:outDir,cwd,expected,text:messages.stderr+messages.stdout});
  if(absent.length>0){event("incomplete",key,0,0,"restored unit lacks "+absent.join(","));return false;}
  const restoredFiles=entry.outputs.filter(row=>!row.template).map(row=>join(outDir,row.name));
  if(ifaceKeyed&&!rebaseRestoredUnitV1({outputs:restoredFiles,directories:dependencyDirectoriesV1(args,unit,cwd),digest:path=>digestFileV1(path,settings.stamps),inner:settings.inner,environment})){event("incomplete",key,0,0,"interface: the restored metadata cannot be rebased onto its dependencies");return false;}
  if(!ifaceKeyed){const violation=restoredClosureViolationV1({outputs:restoredFiles,directories:dependencyDirectoriesV1(args,unit,cwd),digest:path=>digestFileV1(path,settings.stamps),inner:settings.inner,environment});if(violation!==undefined){event("incomplete",key,0,0,"closure: "+violation);return false;}}
  settle(restoredFiles,entry);
  shim(entry.outputs.filter(row=>!row.template).map(row=>join(outDir,row.name)));
  markExternIdentityV1(entry.outputs.filter(row=>!row.template).map(row=>join(outDir,row.name)),portableKeyV1(keyInput,roots),sensitive,settings.stamps,crateHashOf);
  if(facts){const replayed=await replayCachedUnitV1(environment,facts,key,entry.provenance);if(!replayed)event("broker-unreachable",key,0,0,"replay");else event("broker-replay",key,0);}
  process.stdout.write(messages.stdout);process.stderr.write(messages.stderr);return true;
 };
 const offloadUnitV1=async():Promise<(Outcome&{origin:string})|undefined>=>{
  const mode=offloadModeV1(environment);
  if(mode==="never"||!unit.source.endsWith(".rs")||(unit.crateTypes.length?unit.crateTypes:["bin"]).some(type=>!OFFLOAD_POLICY_V1.crateTypes.includes(type)))return undefined;
  const controller=new AbortController();let received:number|undefined;
  const handlers=(["SIGINT","SIGTERM"] as const).map(name=>{const handler=()=>{received=name==="SIGINT"?130:143;controller.abort();};process.once(name,handler);return[name,handler] as const;});
  try{
   const offload=await import("../../../🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts");
   if(mode==="auto"&&!offload.worthARoundTripV1(settings.root,crate))return undefined;
   if(mode==="auto"&&environment.SEMIO_OFFLOAD_PRESSURE!=="door"&&offload.slotQueueMillisecondsV1(settings.root,settings.slots)<OFFLOAD_POLICY_V1.limits.queueThresholdMilliseconds&&(await offload.sharedUnitAdmissionV1(settings.root,ports.admission)).verdict!=="wait")return undefined;
   const answer=await offload.offloadRustcUnitV1({args,environment,cwd,root:settings.root,lane:settings.lane,crate,key,compiler:settings.inner,digest:path=>digestFileV1(path,settings.stamps),signal:controller.signal});
   if(received!==undefined)return{status:received,stdout:"",stderr:"",origin:""};
   return answer.kind==="done"?{status:answer.status,stdout:answer.stdout,stderr:answer.stderr,origin:answer.origin}:undefined;
  }catch{return received===undefined?undefined:{status:received,stdout:"",stderr:"",origin:""};}
  finally{for(const [name,handler] of handlers)process.off(name,handler);}
 };
 try{if(await serve()){event("hit",key,Date.now()-started);lookup("local-hit");countUnitV1(workspaceOf,"localHit");if(environment.SEMIO_REMOTE_CACHE_ENDPOINT&&environment.SEMIO_REMOTE_CACHE_ACCESS==="read-write"){const basics=unitBasicsV1({root:settings.root,environment,inner:settings.inner,unit,args,cwd}),shared=basics?prepareFleetV1({root:settings.root,environment,basics,unit,args,cwd,laneRoot:settings.laneRoot,localKey:key,keyInput,toolchain:keyInput.toolchain,crate,sensitive,expected}):undefined;if(shared)offerFleetUnitV1(shared);}return 0;}}catch{}
 const basics=unitBasicsV1({root:settings.root,environment,inner:settings.inner,unit,args,cwd}),prepare=(paths:readonly string[])=>basics?prepareFleetV1({root:settings.root,environment,basics,unit,args,cwd,laneRoot:settings.laneRoot,localKey:key,keyInput,toolchain:keyInput.toolchain,crate,sensitive:paths,expected}):undefined;
 let fleet=prepare(sensitive),rooted:readonly string[]|undefined;shared=fleet!==undefined;const neutral=fleet,named={cwd,manifest:manifestDir,workspace:workspaceOf,"cargo-home":environment.CARGO_HOME,"rustup-home":environment.RUSTUP_HOME,lane:settings.laneRoot};
 const adopt=async():Promise<FleetUnitV1|undefined>=>{if(rooted!==undefined||neutral===undefined||sensitive.length>0)return undefined;const kinds=await rootHintV1(neutral),paths=kinds&&rootPathsV1(kinds,named),alt=paths?prepare(paths):undefined;if(alt&&kinds){rooted=kinds;fleet=alt;return alt;}return undefined;};
 if(!fleet){const why=fleetUnidentifiedReasonV1(settings.root,environment);if(why!==undefined){event("remote-skip",key,0,0,why);countSkipV1(workspaceOf,why);countFallbackV1(settings.root,why);}}
 const breaker=fleet?.client.state()??"ok";if(fleet&&breaker!=="ok"){event("remote-skip",key,0,0,"breaker: "+breaker+", the upload waits in the spool");countFallbackV1(settings.root,"breaker-"+breaker);}
 let claim:ClaimV1|undefined;
 try{claim=await acquireUnitClaimV1(join(settings.root,policy.layout.locks),key.slice(0,40),{});}catch{claim=undefined;}
 const waited=claim?.waitedMs??0;
 if(waited>50||!claim)event(claim?"lock-wait":"lock-timeout",key,0,waited);
 try{
  if(claim){try{if(await serve()){event("hit",key,Date.now()-started,waited,"after-wait");lookup("local-hit");countUnitV1(workspaceOf,"localHit");return 0;}}catch{}}
  let hold:FleetHoldV1|undefined;
  if(fleet&&breaker==="ok"){
   await adopt();
   const decision=await obtainFleetUnitV1(fleet!,(kind,milliseconds,waitedMs,reason)=>event(kind,key,milliseconds,waitedMs,reason),{},adopt);
   if(decision.outcome==="hit"){try{if(await serve()){lookup("fleet-hit",fleet.key);countUnitV1(workspaceOf,"remoteHit");await awaitFleetSettlesV1();return 0;}}catch{}countFallbackV1(settings.root,"hit-unservable");}
   else{hold=decision.hold;if(decision.reason!=="granted")countFallbackV1(settings.root,decision.reason);}
  }
  const compiled=Date.now();let remote=await offloadUnitV1();
  if(remote&&remote.status===0){const absent=missingOutputsV1({directory:outDir,cwd,expected,text:remote.stderr+remote.stdout});if(absent.length>0){event("incomplete",key,Date.now()-compiled,0,"offload "+remote.origin+" returned a unit without "+absent.join(","));remote=undefined;}}
  const outcome=remote??await compile(true);
  if(remote){process.stdout.write(remote.stdout);process.stderr.write(remote.stderr);}
  if(outcome.status!==0){await hold?.release();event("failed",key,Date.now()-compiled,waited);return outcome.status;}
  if(!remote){const took=Date.now()-compiled;void import("../../../🏗️build/🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts").then(module=>{module.writeUnitCostV1(settings.root,crate,took);}).catch(()=>undefined);}event("miss",key,Date.now()-compiled,waited,(unit.incremental===undefined?parts:parts+" incremental:lane")+(remote?" offloaded:"+remote.origin:""));countUnitV1(workspaceOf,"miss");lookup("miss",fleet?.key);
  const outputs=[...new Set([...announcedPathsV1(outcome.stderr),...announcedPathsV1(outcome.stdout)])].map(path=>resolve(cwd,path)).filter(path=>existsSync(path)),lacking=uncoveredOutputsV1(outputs.map(path=>basename(path)),expected);
  try{
   markExternIdentityV1(outputs,portableKeyV1(keyInput,roots),sensitive,settings.stamps,crateHashOf);
   let tables=true;
   for(const path of outputs.filter(path=>path.endsWith(".rmeta"))){const written=writeDependenciesV1(path,settings.inner,environment);if(written!==undefined)outputs.push(written);else if(ifaceKeyed)tables=false;}
   const provenance=facts&&!remote?await recallProvenanceV1(environment,facts):undefined;
   const publish=(spill=false)=>publishUnitV1(settings.root,{key,crate,cwd,outDir,laneRoot:settings.laneRoot,outputs,expected,stdout:outcome.stdout,stderr:outcome.stderr,stampDirectory:settings.stamps,provenance:provenance??null,...(basics?{slot:basics.slot}:{}),...(spill?{spill:true}:{})});
   let entry:ReturnType<typeof publishUnitV1>;
   if(outputs.length===0||lacking.length>0||!tables)entry=undefined;else try{entry=publish();}catch{await new Promise(done=>setTimeout(done,250));try{entry=publish();}catch(error){if(!fleet)throw error;entry=publish(true);if(entry)event("publish",key,Date.now()-compiled,0,"spill: the host store refused the objects, the fleet upload reads them from the spill folder");}}
   if(entry)event("publish",key,Date.now()-compiled,0);
   else{const why=outputs.length===0?"no-outputs":lacking.length>0?"incomplete: "+lacking.join(","):!tables?"interface: the compiler did not list the crate hashes":outputs.some(path=>path.endsWith(".d"))?"refused":"no-dep-info";event("publish-skip",key,Date.now()-compiled,0,why);if(fleet)countSkipV1(workspaceOf,"publish-"+skipNameV1(why));}
   if(entry)settle(outputs,entry);
   const gap=fleet&&entry&&sourcesRead!==null?(()=>{const depInfo=outputs.find(path=>path.endsWith(".d"));if(depInfo===undefined)return["dep-info"];try{return keyGapV1({cwd,covered:new Set(sourcesRead.files.map(file=>resolve(file))),keyedNames,depInfo:readFileSync(depInfo,"utf8")});}catch{return["dep-info"];}})():[];
   const leak=fleet&&entry&&sensitive.length===0&&gap.length===0?leakedRootKindsV1(outputs,named):undefined,kinds=leak?.kinds.filter(kind=>(ROOT_KINDS_V1 as readonly string[]).includes(kind)),shareable=leak!==undefined&&kinds!==undefined&&kinds.length===leak.kinds.length&&rootPathsV1(kinds,named)!==undefined&&(rooted===undefined||kinds.every(kind=>rooted!.includes(kind)));
   const incrementalBuild=unit.incremental!==undefined&&remote===undefined&&!(shared&&!settings.focus.has(crate));
   if(fleet&&entry&&incrementalBuild){event("remote-skip",key,0,0,"incremental: the unit was compiled with an incremental session, whose bytes differ on every run");countSkipV1(workspaceOf,"incremental");await hold?.release();}
   else if(fleet&&entry&&gap.length>0){event("remote-skip",key,0,0,"key-incomplete: "+gap.slice(0,4).join(", ")+(gap.length>4?" and "+(gap.length-4)+" more":""));countSkipV1(workspaceOf,"key-incomplete");await hold?.release();}
   else if(fleet&&entry&&(leak===undefined||(shareable&&rooted!==undefined))){if(!shareFleetUnitV1(fleet,{claimed:hold,costMs:Date.now()-compiled})){countSkipV1(workspaceOf,unsharedReasonV1(fleet));await hold?.release();}}
   else if(fleet&&entry&&shareable&&neutral&&!neutral.client.usable()){
    const paths0=rootPathsV1(kinds!,named),alt0=paths0?prepare(paths0):undefined;
    await hold?.release();
    if(alt0&&shareFleetUnitV1(alt0,{claimed:undefined,costMs:Date.now()-compiled,hint:{neutralKey:neutral.key,kinds:kinds!}}))countFallbackV1(settings.root,"root-keyed-deferred");
    else{countSkipV1(workspaceOf,"root-key-failed");countFallbackV1(settings.root,"root-key-failed");}
   }
   else if(fleet&&entry&&shareable&&neutral){
    const paths0=rootPathsV1(kinds!,named),alt0=paths0?prepare(paths0):undefined,held0=alt0?await holdFleetKeyV1(alt0):undefined,effective=alt0?await publishRootHintV1(neutral,kinds!):undefined;
    let alt=effective===undefined?undefined:alt0,held=held0;
    if(effective!==undefined&&alt0!==undefined&&(effective.length!==kinds!.length||!kinds!.every(kind=>(effective as readonly string[]).includes(kind)))){await held0?.release();const paths=kinds!.every(kind=>(effective as readonly string[]).includes(kind))?rootPathsV1(effective,named):undefined;alt=paths?prepare(paths):undefined;held=alt?await holdFleetKeyV1(alt):undefined;}
    await hold?.release();
    if(alt&&shareFleetUnitV1(alt,{claimed:held,costMs:Date.now()-compiled}))countFallbackV1(settings.root,"root-keyed");
    else{await held?.release();countSkipV1(workspaceOf,"root-key-failed");countFallbackV1(settings.root,"root-key-failed");}
   }
   else{if(leak!==undefined){event("remote-skip",key,0,0,"path-dependent: the compiled bytes hold "+leak.file+", a directory of this machine that cannot be keyed ("+leak.kinds.join(",")+"), so the unit is not shared");countSkipV1(workspaceOf,"path-dependent");countFallbackV1(settings.root,"path-dependent");}await hold?.release();}
  }catch(error){event("publish-skip",key,Date.now()-compiled,0,"error: "+String((error as Error)?.message??error).slice(0,200));await hold?.release();}
  try{shim(outputs);}catch{}
  return 0;
 }finally{claim?.release();}
}

/** 🧰️ Operator commands, run as `--semio-unit-cache <command> --root <dir> …`: `forget --crate <name>` drops the local entries of one crate (the fleet copy stays), `stats` summarises the event logs (decisions, the reasons units were not shared, evictions, lookup outcomes), `churn [--since-minutes <n>] [--crate <name>] [--limit <n>]` explains every crate that was looked up under more than one key by the named key components that differ between its keys (see `unitKeyComponentsV1`), `plan` and `gc` run the bounded cleanup; `gc --single-flight` is the background collector (one at a time, progress lines on stderr, SIGINT and SIGTERM cancel it), `gc --single-flight --watch` keeps it alive while compiles run (it collects whenever the image falls below its floor), `fill` prints the fill rates of the last half hour, `fleet-upload` is the machine's upload worker for the fleet cache (`--once` drains the spool and exits). */
export async function manageV1(args:readonly string[]):Promise<unknown>{
 const [command,...rest]=args,value=(flag:string)=>{const index=rest.indexOf(flag);return index<0?undefined:rest[index+1];},root=resolve(value("--root")??"");
 if(command==="stats"){
  const totals:Record<string,number>={};let maximumWait=0,files=0;const directory=join(root,policy.layout.events);
  if(existsSync(directory))for(const name of readdirSync(directory)){files++;for(const line of readFileSync(join(directory,name),"utf8").split("\n")){if(!line)continue;try{const row=JSON.parse(line);totals[row.kind]=(totals[row.kind]??0)+1;maximumWait=Math.max(maximumWait,row.waitedMs);}catch{}}}
  const reasons:Record<string,number>={};
  if(existsSync(directory))for(const name of readdirSync(directory)){for(const line of readFileSync(join(directory,name),"utf8").split("\n")){if(!line)continue;try{const row=JSON.parse(line) as {kind:string;reason?:string};if(row.kind==="bypass"||row.kind==="remote-skip"||row.kind==="publish-skip"||row.kind==="incomplete"){const label=row.kind+": "+(row.reason??"").split(/[:,]/u)[0]!.trim();reasons[label]=(reasons[label]??0)+1;}}catch{}}}
  return{lanes:files,totals,maximumWaitMs:maximumWait,notShared:reasons,fallbacks:readFallbacksV1(root),evictions:readEvictionsV1(root),lookups:readLookupsV1(root).reduce<Record<string,number>>((counts,row)=>{counts[row.outcome]=(counts[row.outcome]??0)+1;return counts;},{})};
 }
 if(command==="churn"){const minutes=Number(value("--since-minutes")??0),limit=Number(value("--limit")??50),wanted=value("--crate"),report=churnReportV1(readLookupsV1(root,minutes>0?Date.now()-minutes*60000:0).filter(row=>wanted===undefined||row.crate===wanted),Number.isSafeInteger(limit)&&limit>0?limit:50);return report;}
 if(command==="forget"){const crate=value("--crate");if(!crate)throw Error("--crate <name> required");return forgetCrateEntriesV1(root,crate);}
 if(command==="fleet-upload"){const idle=rest.includes("--once")?0:undefined;return await drainUploadsV1(root,process.env,idle===undefined?{}:{idleMilliseconds:idle});}
 const capacity=Number(value("--capacity")??0);if(!Number.isSafeInteger(capacity)||capacity<1)throw Error("--capacity <bytes> required");
 if(command==="plan")return planGcV1(measureGcV1(root,capacity,Date.now()));
 if(command==="gc"&&!rest.includes("--single-flight"))return collectGarbageV1(root,capacity);
 if(command==="fill"){const volume=measureVolumeV1(root,capacity);return{...fillRatesV1(readFillSamplesV1(root),Date.now()),freeBytes:volume.freeBytes,volumeBytes:volume.volumeBytes};}
 if(command==="gc"){
  const controller=new AbortController();for(const signal of["SIGINT","SIGTERM"] as const)process.once(signal,()=>controller.abort(new Error("cancelled")));
  if(rest.includes("--watch"))return watchUnitCachePressureV1(root,capacity,{signal:controller.signal});
  return collectGarbageSingleFlightV1(root,capacity,{signal:controller.signal,progress:(done,total)=>{process.stderr.write(JSON.stringify({progress:done,total})+"\n");}});
 }
 throw Error("Unknown unit-cache command");
}

/** 🏁️ Runs the wrapper as a process: `--semio-unit-cache <command> …` is an operator command (its JSON result on standard output), anything else is one rustc command line Cargo hands to the wrapper. Resolves to the exit status; shared by the script entry and the native executable entry (`🏁️entry.ts`), whose `argv[1]` is no file the main-module test could match. */
export async function runUnitCacheMainV1(argv:readonly string[],environment:Environment,cwd:string):Promise<number>{
 try{
  if(argv[0]==="--semio-unit-cache"){console.log(JSON.stringify(await manageV1(argv.slice(1))));return 0;}
  return await wrapRustcV1(argv,environment,cwd);
 }catch(error){console.error(String((error as Error)?.message??error));return 1;}
}

export {UNFINISHED_EXIT_STATUS_V1};

if(isMainModuleV1(import.meta.url))exitWithStatusOfV1(()=>runUnitCacheMainV1(process.argv.slice(2),process.env,process.cwd()));
