#!/usr/bin/env repo
import {spawnSync} from "node:child_process";
import {existsSync,mkdtempSync,mkdirSync,readFileSync,readdirSync,realpathSync,rmdirSync,statSync,unlinkSync,writeFileSync} from "node:fs";
import {tmpdir,totalmem} from "node:os";
import {basename,join,resolve} from "node:path";
import {admitFactsV1,expandRustcArgumentsV1,extraFilenameV1,extraNeutralV1,extraRestoreV1,withCanonicalMetadataV1,keyEnvironmentV1,laneRestoreV1,localCompilerInputKeyV1,normalizeRustcArgumentsV1,parseRustcUnitV1,rustcBypassV1,rustSysrootV1,sha256V1,sourceValueV1,toolchainIdentityV1,withoutIncrementalV1,unitKeyComponentsV1,unitKeyPartsV1,unitKeyV1,UNIT_CACHE_POLICY_V1} from "./🔏️key/🟦️.ts";
import {acquireLightSlotV1,acquireSlotV1,acquireUnitClaimV1,tryAcquireSlotV1,type ClaimV1} from "./🔒️lock/🟦️.ts";
import {digestFileV1,emitUnitEventV1,forgetCrateEntriesV1,lookupUnitV1,newestEntryV1,publishUnitV1,churnReportV1,readEvictionsV1,readLookupsV1,recordLookupV1,restoreUnitV1,type UnitEventV1} from "./💾️store/🟦️.ts";
import {compilerTemporaryArgumentsV1,lanePathsV1} from "./🧵️lane/🟦️.ts";
import {jobserverChildEnvironmentV1,jobserverClosedMessageV1,jobserverPlanV1,jobserverStdioV1} from "./🎰️jobserver/🟦️.ts";
import {buildScriptsEnabledV1,installBuildScriptShimV1,isBuildScriptUnitV1} from "./🏗️buildscript/🟦️.ts";
import {brokerOfV1,brokerWouldWaitV1,compilerBrokerOfV1,recallProvenanceV1,replayCachedUnitV1} from "./🛰️broker/🟦️.ts";
import {collectGarbageSingleFlightV1,collectGarbageV1,measureGcV1,measureVolumeV1,planGcV1,watchUnitCachePressureV1} from "./🧹️gc/🟦️.ts";
import {fillRatesV1,readFillSamplesV1} from "./🧹️gc/📈️fill/🟦️.ts";
import {keyGapV1,sourceEnvironmentV1,unitSourcesV1} from "./🔏️key/🔗️sources/🟦️.ts";
import {manifestCoversV1,manifestDeclarationV1,manifestFilesV1,manifestIdV1,manifestOfV1,manifestUnionV1,manifestWithinBoundsV1,observedManifestV1,obtainManifestV1,refreshManifestV1,shareManifestV1,writeManifestV1,type InputManifestV1} from "./🛰️fleet/📋️manifest/🟦️.ts";
import {fleetClientV1} from "./🛰️fleet/🟦️.ts";
import {laneMessageV1} from "./🚪️launch/🟦️.ts";
import {buildRunKeyV1,markBuildScriptIdentityV1} from "./🏗️buildscript/🔑️run-key/🟦️.ts";
import {canonicalOutDirectoryArgumentsV1,canonicalSourceArgumentsV1,linkRootArgumentsV1,remappedNameV1,rustSourceRemapArgumentsV1,remappedV1,remapsOfV1} from "./🧭️remap/🟦️.ts";
import {pathSensitiveV1} from "./🧭️remap/🔍️embedding/🟦️.ts";
import {externIdentityV1,markExternIdentityV1,markProducerKeyV1,producerKeyV1} from "./🪪️extern/🟦️.ts";
import {currentHashV1,dependencyDirectoriesV1,interfaceEligibilityV1,interfaceIdentityV1,rebaseRestoredUnitV1,restoredClosureViolationV1,settleUnitInterfaceV1,writeDependenciesV1} from "./🔭️interface/🟦️.ts";
import {announcedPathsV1,expectedOutputsV1,missingOutputsV1,uncoveredOutputsV1} from "./✅️complete/🟦️.ts";
import {localKeyV1,platformOfV1,portableKeyV1,portableRootsV1,portableTextV1,slotMetadataV1,toolRootsV1} from "./🛰️fleet/🪪️identity/🟦️.ts";
import {awaitFleetSettlesV1,explainFleetMissV1,fleetUnidentifiedReasonV1,holdFleetKeyV1,leakedRootKindsV1,obtainFleetUnitV1,offerFleetUnitV1,prepareFleetV1,publishRootHintV1,rootHintV1,rootPathsV1,ROOT_KINDS_V1,shareFleetUnitAwaitingV1,shareFleetUnitV1,unitBasicsV1,unsharedReasonV1,unitToolchainV1,type FleetHoldV1,type FleetUnitV1} from "./🛰️fleet/🟦️.ts";
import {drainUploadsV1} from "./🛰️fleet/📤️uploader/🟦️.ts";
import {countFallbackV1,countSkipV1,countUnitV1,readFallbacksV1,skipNameV1} from "./🛰️fleet/📊️counters/🟦️.ts";
import {offloadCrateTypeRefusalV1,offloadModeV1} from "../../../🏗️build/🌐️remote-execution/🚀️offload/📐️policy/🟦️.ts";
import type {OffloadAdmissionPortsV1} from "../../../🏗️build/🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
import {exitWithStatusOfV1,UNFINISHED_EXIT_STATUS_V1} from "./🏁️exit/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1;
type Environment=Readonly<Record<string,string|undefined>>;
type Settings=Readonly<{root:string;lane:string;laneRoot:string|undefined;inner:readonly string[];slots:number;lightSlots:number;focus:ReadonlySet<string>;stamps:string|undefined}>;

/** ⚙️ Reads the wrapper's settings, which the repository launcher exports; `undefined` when the cache is off. */
export function readSettingsV1(environment:Environment):Settings|undefined{
 const root=environment.SEMIO_UNIT_CACHE_ROOT;if(!root||environment.SEMIO_UNIT_CACHE==="off")return undefined;
 const lane=environment.SEMIO_UNIT_CACHE_LANE??"d",inner=environment.SEMIO_UNIT_CACHE_INNER?JSON.parse(environment.SEMIO_UNIT_CACHE_INNER) as string[]:["rustc"];
 if(!Array.isArray(inner)||inner.length<1||inner.some(part=>typeof part!=="string"))throw Error("SEMIO_UNIT_CACHE_INNER must be a JSON array of strings");
 const laneRoot=environment.SEMIO_UNIT_CACHE_LANE_ROOT;
 return Object.freeze({root:resolve(root),lane,laneRoot,inner:Object.freeze(inner),slots:Number(environment.SEMIO_UNIT_CACHE_SLOTS??0),lightSlots:Number(environment.SEMIO_UNIT_CACHE_LIGHT_SLOTS??0),focus:new Set((environment.SEMIO_UNIT_CACHE_FOCUS??"").split(",").filter(Boolean)),stamps:laneRoot?join(laneRoot,policy.layout.stamps):lanePathsV1(resolve(root),lane).stamps});
}

type Outcome=Readonly<{status:number;stdout:string;stderr:string;peakRssBytes?:number}>;
/** 📜️ Keeps existing response references and unrepresentable arguments in place while shortening a Windows compiler invocation. */
function compilerArgumentsV1(inner:readonly string[],args:readonly string[]):Readonly<{args:readonly string[];close:()=>void}>{
 if(process.platform!=="win32"||[...inner,...args].reduce((n,arg)=>n+2*arg.length+3,0)<32767)return{args,close:()=>{}};
 const directory=mkdtempSync(join(tmpdir(),"semio-rustc-")),files:string[]=[],result:string[]=[],pending:string[]=[];
 const close=()=>{for(const file of files)try{unlinkSync(file);}catch{}try{rmdirSync(directory);}catch{}};
 const flush=()=>{if(!pending.length)return;const file=join(directory,String(files.length)+".rsp");files.push(file);writeFileSync(file,pending.join("\n")+"\n",{flag:"wx"});result.push("@"+file);pending.length=0;};
 try{for(const arg of args){if(arg===""||arg.startsWith("@")||/[\r\n]/u.test(arg)){flush();result.push(arg);}else pending.push(arg);}flush();return{args:result,close};}catch(error){close();throw error;}
}
function run(inner:readonly string[],args:readonly string[],environment:Environment,capture:boolean):Promise<Outcome>{
 const jobs=jobserverPlanV1(environment),message=jobserverClosedMessageV1(jobs);
 if(message)process.stderr.write(message+"\n");
 const prepared=compilerArgumentsV1(inner,args);let child:Bun.Subprocess<"inherit","pipe"|"inherit","pipe"|"inherit">;
 try{child=Bun.spawn([inner[0]!,...inner.slice(1),...prepared.args],{windowsHide:true,env:jobserverChildEnvironmentV1(environment,jobs),stdio:jobserverStdioV1(jobs.descriptors,capture)}) as typeof child;}catch(error){prepared.close();jobs.release?.();return Promise.reject(error);}
 for(const signal of["SIGINT","SIGTERM"] as const)process.once(signal,()=>child.kill(signal));
 const out:Buffer[]=[],err:Buffer[]=[];
 const pump=async(stream:unknown,chunks:Buffer[],sink:NodeJS.WriteStream):Promise<void>=>{if(!(stream instanceof ReadableStream))return;for await(const chunk of stream as AsyncIterable<Uint8Array>){const buffer=Buffer.from(chunk);chunks.push(buffer);sink.write(buffer);}};
 return Promise.all([child.exited,pump(child.stdout,out,process.stdout),pump(child.stderr,err,process.stderr)]).then(async([code])=>{
  const signal=child.signalCode,peak=child.resourceUsage()?.maxRSS;
  if(signal==="SIGTERM"||signal==="SIGKILL")await reportWardenCancelV1(child.pid);
  return{status:signal?128+({SIGINT:2,SIGTERM:15,SIGKILL:9} as Record<string,number>)[signal]!:code,stdout:Buffer.concat(out).toString("utf8"),stderr:Buffer.concat(err).toString("utf8"),...(peak!==undefined&&peak>0?{peakRssBytes:peak}:{})};
 }).finally(()=>{prepared.close();jobs.release?.();});
}

/** 🧯️ Tells where the build log is read when the host warden killed the compiler of this unit (its cancel record names the pid): the bilingual line with the typed reason, for a unit over its ceiling the unit, its peak and its ceiling and that it must be carved. The build owner ends the build with the same record, see `HostWardenCancelledErrorV1`. */
export async function reportWardenCancelV1(pid:number,options:Readonly<{directory?:string;write?:(line:string)=>void}>={}):Promise<void>{
 try{
  const warden=await import("../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts"),words=await import("../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🟦️.ts"),record=warden.readHostWardenCancelV1(pid,options.directory);
  if(record!==null)(options.write??(line=>{process.stderr.write(line);}))(`[unit-cache] ${(["en","de"] as const).map(language=>words.formatHostWardenCancelLineV1(language,record)).join(" / ")}\n`);
 }catch{}
}

/** 🏔️ Compares the peak resident memory a compiler run of this unit reached with the library budget of the admission policy (the verdict applies the higher allowance of a final) and writes the bilingual alarm line to the build log when an unlisted unit is above it (the same verdict `unitCeilingAlarmV1` gives the component pass; the peak itself is in the `miss` event of the lane log). */
async function reportUnitCeilingV1(crate:string,target:string|undefined,types:readonly string[],environment:Environment,peakRssBytes:number):Promise<void>{
 try{
  const policy=(await import("../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧬️schema/🔣️.json")).default.definitions.Policy.const.unitCeiling,ceiling=policy.classes[policy.workerClass as keyof typeof policy.classes].libraryMegabytes as number;
  if(peakRssBytes<=ceiling*1048576)return;
  const admission=await import("../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts");
  const key=`${crate}@${crate}|${target??"host"}|${environment.SEMIO_BUILD_MODE??"-"}|${types.join(",")||"-"}`,alarm=admission.unitCeilingAlarmV1(admission.ADMISSION_POLICY_V1.unitCeiling,{key,peakRssBytes});
  if(alarm)process.stderr.write(`[unit-cache] ${alarm}\n`);
 }catch{}
}

/** 🪞️ Drops the compiler path Cargo puts before the arguments when it runs the stand-in as `<wrapper> <rustc> <arguments>`; every compiler argument list starts with a flag, a response file or a stdin dash. */
export function withoutCompilerPathV1(argv:readonly string[]):readonly string[]{
 const first=argv[0];
 return first!==undefined&&!first.startsWith("-")&&!first.startsWith("@")&&!first.endsWith(".rs")?argv.slice(1):argv;
}

/** 🧭️ Appended to every compilation that Cargo gave a `--remap-path-scope` (the repository's `trim-paths = "object"` profile remaps only object code): rustc then remaps the source names of the metadata as well, so a unit gives the same bytes on every machine and at every checkout path, whoever compiles it (an extern's content hash is part of the key of its dependants). The scope Cargo gave is replaced (rustc refuses the option twice). */
export const PATH_NEUTRAL_FLAGS_V1:readonly string[]=Object.freeze(["--remap-path-scope=all"]);

/** 🛠️ Serves one rustc command line from the cache or compiles it exactly once across all lanes. Never throws on cache faults: any fault is a miss. A miss compiles the command line Cargo gave, so a workspace unit Cargo compiles incrementally keeps rustc's incremental session in the lane (the next edit of that crate, also when it is a dependency of the focused one, recompiles incrementally) and is published locally with its session marked (`session`), so it is never shared with the fleet, neither on the compile nor on a later local hit; a unit that is shared through the fleet is always compiled without a session, the focused crates keep theirs and stay local. */
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
 argv=(()=>{try{return canonicalSourceArgumentsV1(argv,parseRustcUnitV1(expandRustcArgumentsV1(argv,cwd)).source,cwd);}catch{return argv;}})();
 argv=canonicalOutDirectoryArgumentsV1(argv,environment.OUT_DIR);
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
 let args=expandRustcArgumentsV1(argv,cwd);const unit=parseRustcUnitV1(args),refused=rustcBypassV1(unit,args),natives=refused==="native-link"?await(async()=>{try{return (await import("../../../🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts")).nativeKeyInputsV1({args,cwd,digest:path=>digestFileV1(path,settings.stamps)});}catch{return undefined;}})():undefined,bypass=natives===undefined?refused:undefined,crate=unit.crateName||"-";
 const event=(kind:UnitEventV1["kind"],key:string|null,milliseconds:number,waitedMs=0,reason?:string,peakRssBytes?:number)=>emitUnitEventV1(settings.root,{lane:settings.lane,kind,crate,key,milliseconds,waitedMs,...(reason?{reason}:{}),...(peakRssBytes?{peakRssBytes}:{})});
 let shared=false,held:ClaimV1|undefined,ranMs=0;
 const stripIncremental=()=>shared&&unit.incremental!==undefined&&!settings.focus.has(crate);
 const incrementalSession=()=>unit.incremental!==undefined&&!stripIncremental();
 const compile=async(capture:boolean,localInputKey?:string):Promise<Outcome>=>{
  let slot=held;held=undefined;
  if(slot===undefined&&settings.slots>0&&unit.source.endsWith(".rs")){slot=await acquireSlotV1(settings.root,settings.slots,{since:started,onWait:waited=>event("slot-wait",null,0,waited)});}
  const began=Date.now();
  try{return await run(settings.inner,[...(stripIncremental()?withoutIncrementalV1(argv):argv),...linkRootArgumentsV1(args,resolve(cwd,unit.outDir)),...compilerTemporaryArgumentsV1(settings.laneRoot?join(settings.laneRoot,policy.layout.compilerTemporaries):lanePathsV1(settings.root,settings.lane).temporaries,args)],{...environment,SEMIO_RUSTC_UNIT_LOCAL_INPUT_KEY:localInputKey},capture);}finally{ranMs=Date.now()-began;slot?.release();}
 };
 const started=Date.now(),workspaceOf=environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||undefined;
 if(bypass){
  const outcome=await compile(false);
  if(unit.source.endsWith(".rs")){event("bypass",null,Date.now()-started,0,bypass);if(environment.SEMIO_REMOTE_CACHE_ENDPOINT)countSkipV1(workspaceOf,"bypass-"+skipNameV1(bypass));}
  return outcome.status;
 }
 const expected=expectedOutputsV1(unit,args),extra=extraFilenameV1(args),neutralExpected=expected.map(name=>extraNeutralV1(name,extra));
 let key:string,parts="",keyInput:Parameters<typeof unitKeyV1>[0],sourcesRead:Awaited<ReturnType<typeof unitSourcesV1>>=null,keyedNames:ReadonlySet<string>=new Set();
 const digests=new Map<string,string>(),rawExterns:string[]=[],producers:string[]=[],metadataOnly=unit.emit.every(kind=>kind==="metadata"||kind==="dep-info");let ifaceKeyed=false;
 const remaps=remapsOfV1(args),covered=remappedV1(remaps,cwd),manifestDir=environment.CARGO_MANIFEST_DIR,tools=toolRootsV1(environment,(()=>{try{return rustSysrootV1(settings.root,settings.inner,environment);}catch{return undefined;}})()),roots=portableRootsV1({cwd,workspace:workspaceOf,hints:[manifestDir,environment.PWD],...tools});
 const outDir=resolve(cwd,unit.outDir),named:Readonly<Record<string,string|undefined>>={cwd,manifest:manifestDir,workspace:workspaceOf,"cargo-home":tools.cargoHome,"rustup-home":tools.rustupHome,sysroot:tools.sysroot,lane:settings.laneRoot,output:outDir,generated:environment.OUT_DIR};
 const scope=(kinds:readonly string[])=>{
  const paths=rootPathsV1(kinds,named);if(paths===undefined)return undefined;
  return paths.flatMap(path=>{const index=path.indexOf("="),value=path.slice(index+1);try{const actual=realpathSync(value);if(actual!==value)return[path,path.slice(0,index)+"-resolved="+actual];}catch{}return[path];});
 };
 const basicsEarly=unitBasicsV1({root:settings.root,environment,inner:settings.inner,unit,args,cwd}),slotEarly=basicsEarly?.slot,manifestId=slotEarly===undefined||!unit.source.endsWith(".rs")?undefined:manifestIdV1(slotEarly),manifestClient=manifestId===undefined?undefined:fleetClientV1(settings.root,environment);
 const manifestHeld=manifestId===undefined?undefined:await obtainManifestV1(settings.root,manifestId,manifestClient),manifestUsed0=manifestHeld?.state?.manifest,recorded=manifestUsed0===undefined?[]:manifestFilesV1(manifestUsed0,roots);
 let manifestUsed=manifestUsed0;
 if(manifestHeld?.from==="fleet")event("manifest-fetch",null,Date.now()-started,0,"fleet");
 let sensitive:readonly string[]=[];
 let scopedPaths:readonly string[]|undefined;
 const scopedKey=()=>localKeyV1(keyInput,roots,[...sensitive,"⟨output-root-scope⟩",...(scopedPaths??=scope(Object.keys(named).filter(kind=>Boolean(named[kind])))!)]);
 const sensitiveTo=(paths:readonly string[])=>[...new Set([...sensitive,...paths])].sort();
 let fixed:Pick<Parameters<typeof unitKeyV1>[0],"toolchain"|"normalized"|"externs">|undefined;
 const inputsOf=(sources:NonNullable<typeof sourcesRead>):Parameters<typeof unitKeyV1>[0]=>({...fixed!,sourceSha256:sources.sha256,environment:[...keyEnvironmentV1(environment,settings.laneRoot),...sourceEnvironmentV1(sources.names,environment,new Set(keyEnvironmentV1(environment,settings.laneRoot).map(row=>row[0])),(name,text)=>sourceValueV1(name,text,settings.laneRoot)),...covered?[["⟨recorded-name⟩",remappedNameV1(remaps,cwd)] as [string,string],["⟨output-root-scan⟩",policy.key.outputRootScanVersion] as [string,string]]:[]]});
 try{
  const sourceFile=resolve(cwd,unit.source),sources=await unitSourcesV1(sourceFile,manifestDir,environment.OUT_DIR,path=>digestFileV1(path,settings.stamps),recorded,manifestUsed0?.env??[]);if(sources===null)throw Error("source");
  const sourceSha=sources.sha256;sourcesRead=sources;
  for(const row of unit.externs){if(row.path==="")continue;const path=resolve(cwd,row.path),sha=digestFileV1(path,settings.stamps);if(sha===null)throw Error("extern "+row.name);rawExterns.push(sha);const producer=producerKeyV1(path,sha);if(producer!==undefined)producers.push(producer);const identity=metadataOnly?interfaceIdentityV1(path,sha):undefined;if(identity!==undefined)ifaceKeyed=true;digests.set(row.path,identity??externIdentityV1(path,sha,settings.stamps)??sha);}
  sensitive=covered?pathSensitiveV1({source:resolve(cwd,unit.source),manifest:manifestDir,cwd}):[...new Set([manifestDir,cwd].filter((path):path is string=>path!==undefined))].sort();
  fixed={toolchain:unitToolchainV1({root:settings.root,environment,inner:settings.inner,unit,args},()=>toolchainIdentityV1(settings.root,settings.inner,environment)),normalized:normalizeRustcArgumentsV1(args,digests,settings.laneRoot),externs:[...unit.externs.map(row=>({name:row.flags+row.name,sha256:row.path===""?"builtin":digests.get(row.path)!})),...natives??[]]};keyInput=inputsOf(sources);key=localKeyV1(keyInput,roots,sensitive);{const metadata=slotMetadataV1(keyInput,roots,basicsEarly?.slotParts);argv=withCanonicalMetadataV1(argv,metadata);args=withCanonicalMetadataV1(args,metadata);}parts=unitKeyPartsV1(keyInput);keyedNames=new Set([...keyInput.environment.map(row=>row[0]),...sources.names]);
 }catch(error){const outcome=await compile(false);const unreadable=/^extern /u.test(String((error as Error)?.message));event("bypass",null,Date.now()-started,0,"unreadable-input"+(unreadable?": "+(error as Error).message:""));if(environment.SEMIO_REMOTE_CACHE_ENDPOINT)countSkipV1(workspaceOf,unreadable?"unreadable-extern":"unreadable-source");return outcome.status;}
 const producer=interfaceEligibilityV1(unit,args)===undefined,crateHashOf=(path:string):string|undefined=>currentHashV1(path,file=>digestFileV1(file,settings.stamps),settings.inner,environment);
 let outputSensitive:readonly string[]=sensitive,restoredKinds:readonly string[]|undefined,verifiedOutputs=true;
 const settle=(outputs:readonly string[],entry:Parameters<typeof settleUnitInterfaceV1>[0]["entry"]):void=>{
  if(!ifaceKeyed&&!producer)return;
  try{settleUnitInterfaceV1({outputs,entry,producer,...covered?{identityScope:[policy.key.outputRootScanVersion,...outputSensitive]}:{},identityOf:digest=>sha256V1(JSON.stringify(["iface",portableKeyV1({...keyInput,sourceSha256:"iface:"+digest},roots),outputSensitive,entry.environment.map(row=>[row.name,row.value===null?null:portableTextV1(row.value,roots)])])),neutralName:path=>portableTextV1(path,roots),readFile:path=>{try{return readFileSync(resolve(cwd,laneRestoreV1(path,settings.laneRoot)));}catch{return null;}},digest:path=>digestFileV1(path,settings.stamps),inner:settings.inner,environment});}catch{}
 };
 const lookup=(outcome:"local-hit"|"fleet-hit"|"miss",remote?:string):void=>{
  const portable=keyInput.normalized.map(argument=>portableTextV1(argument,roots)),named=unitKeyComponentsV1({...keyInput,normalized:portable,environment:keyInput.environment.map(([name,value])=>[name,portableTextV1(value,roots)] as [string,string])},{platform:process.platform+"/"+process.arch,roots:JSON.stringify(keyInput.normalized.filter((argument,index)=>portable[index]!==argument))});
  const platform=()=>{try{const basics=unitBasicsV1({root:settings.root,environment,inner:settings.inner,unit,args,cwd});return basics?JSON.stringify(platformOfV1({release:basics.release,unit,args,environment,toolchain:keyInput.toolchain})):undefined;}catch{return undefined;}};
  const slot=unitBasicsV1({root:settings.root,environment,inner:settings.inner,unit,args,cwd})?.slot;
  recordLookupV1(settings.root,{lane:settings.lane,crate,key,...(remote===undefined?{}:{remote}),...(slot===undefined?{}:{slot}),outcome,components:named.components,values:named.values,...environment.SEMIO_UNIT_CACHE_TRACE_KEYS?{trace:{args:portable,environment:keyInput.environment.map(([name,value])=>[name,portableTextV1(value,roots)] as [string,string]),...platform()===undefined?{}:{platform:platform()!}}}:{}});
 };
 const context={cwd,laneRoot:settings.laneRoot,environment,stampDirectory:settings.stamps},shimming=isBuildScriptUnitV1(unit)&&buildScriptsEnabledV1(environment),facts=compilerBrokerOfV1(environment)?admitFactsV1(args,environment,cwd):undefined;
 const shim=(outputs:readonly string[]):void=>{if(!shimming)return;const artifact=outputs.find(path=>!path.endsWith(".d")&&!path.endsWith(".rmeta"));if(artifact){const real=installBuildScriptShimV1(artifact);if(real&&verifiedOutputs)markBuildScriptIdentityV1(real,portableKeyV1(keyInput,roots),outputSensitive,digestFileV1(real,settings.stamps),key);}};
 const serve=async(candidate=key):Promise<boolean>=>{
  const entry=lookupUnitV1(settings.root,candidate,{...context,expected:neutralExpected});if(!entry)return false;
  const messages=restoreUnitV1(settings.root,entry,{outDir,laneRoot:settings.laneRoot,stampDirectory:settings.stamps,extra,...environment.OUT_DIR?{generated:environment.OUT_DIR}:{}}),absent=missingOutputsV1({directory:outDir,cwd,expected,text:messages.stderr+messages.stdout});
  if(absent.length>0){event("incomplete",key,0,0,"restored unit lacks "+absent.join(","));return false;}
  const restoredFiles=entry.outputs.filter(row=>!row.template).map(row=>join(outDir,extraRestoreV1(row.name,extra)));
  if(ifaceKeyed&&!rebaseRestoredUnitV1({outputs:restoredFiles,directories:dependencyDirectoriesV1(args,unit,cwd),digest:path=>digestFileV1(path,settings.stamps),inner:settings.inner,environment})){event("incomplete",key,0,0,"interface: the restored metadata cannot be rebased onto its dependencies");return false;}
  if(!ifaceKeyed){const violation=restoredClosureViolationV1({outputs:restoredFiles,directories:dependencyDirectoriesV1(args,unit,cwd),digest:path=>digestFileV1(path,settings.stamps),inner:settings.inner,environment});if(violation!==undefined){event("incomplete",key,0,0,"closure: "+violation);return false;}}
  if(candidate!==key){
   const leak=leakedRootKindsV1(restoredFiles,named),paths=leak?scope(leak.kinds):[];
   if(paths===undefined)return false;
   outputSensitive=sensitiveTo(paths);restoredKinds=leak?.kinds??[];key=candidate;
  }
  settle(restoredFiles,entry);
  shim(restoredFiles);
  markExternIdentityV1(restoredFiles,portableKeyV1(keyInput,roots),outputSensitive,settings.stamps,crateHashOf);
  markProducerKeyV1(restoredFiles,key,settings.stamps);
  if(facts&&brokerOfV1(environment)){const replayed=await replayCachedUnitV1(environment,facts,key,entry.provenance);if(!replayed)event("broker-unreachable",key,0,0,"replay");else event("broker-replay",key,0);}
  process.stdout.write(messages.stdout);process.stderr.write(messages.stderr);return true;
 };
 const serveLocal=async()=>covered&&await serve(scopedKey())||await serve();
 const forecastKey=()=>facts?localCompilerInputKeyV1(key,facts.package,facts.version,incrementalSession(),rawExterns):undefined;
 const offloadUnitV1=async():Promise<(Outcome&{origin:string})|undefined>=>{
  const mode=offloadModeV1(environment);
  if(mode==="never")return undefined;
  const refusal=!unit.source.endsWith(".rs")?"source-kind" as const:offloadCrateTypeRefusalV1(unit.crateTypes,unit.target)!==undefined?"crate-type" as const:undefined;
  if(refusal!==undefined){try{(await import("../../../🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts")).declineOffloadV1(settings.root,environment,{lane:settings.lane,crate,key,reason:refusal,detail:refusal==="source-kind"?unit.source:unit.crateTypes.join(",")});}catch{}return undefined;}
  const controller=new AbortController();let received:number|undefined;
  const handlers=(["SIGINT","SIGTERM"] as const).map(name=>{const handler=()=>{received=name==="SIGINT"?130:143;controller.abort();};process.once(name,handler);return[name,handler] as const;});
  try{
   const offload=await import("../../../🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts"),admission=await import("../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts");
   const spend=offload.readUnitExpenseV1(settings.root,crate,unit.crateTypes,unit.target),beyondHost=()=>{const learned=[true,false].map(linked=>offload.readUnitPeakV1(settings.root,crate,offload.offloadDeclaredThreadsV1(linked))).filter((value):value is number=>value!==undefined);return admission.unitBeyondHostV1(admission.ADMISSION_POLICY_V1.unitCeiling,{crate,target:unit.target??"host"},learned.length>0?Math.max(...learned):undefined,Math.round(totalmem()/1048576),admission.localHoldableMegabytesV1(Math.round(totalmem()/1048576)));},gate=await offload.offloadGateV1({mode,fleetRequired:environment[admission.ADMISSION_POLICY_V1.placementEnvironment]==="fleet-only",door:environment.SEMIO_OFFLOAD_PRESSURE==="door",expensive:()=>spend.expense.expensive,workerFits:waitMilliseconds=>offload.awaitFleetFitV1(environment,{peakMegabytes:spend.peak,signal:controller.signal,waitMilliseconds,pollMilliseconds:offload.OFFLOAD_POLICY_V1.finals.pollMilliseconds}),localSlot:async waitMilliseconds=>{if(settings.slots<1||!unit.source.endsWith(".rs"))return false;if(held!==undefined)return true;try{held=waitMilliseconds>0?await acquireSlotV1(settings.root,settings.slots,{since:started,signal:AbortSignal.any([controller.signal,AbortSignal.timeout(waitMilliseconds)])}):tryAcquireSlotV1(settings.root,settings.slots);}catch{held=undefined;}return held!==undefined;},worth:()=>offload.worthARoundTripV1(settings.root,crate),beyondHost:()=>beyondHost(),slotQueueMilliseconds:()=>offload.slotQueueMillisecondsV1(settings.root,settings.slots),host:async()=>(await offload.sharedUnitAdmissionV1(settings.root,ports.admission)).verdict,broker:async()=>{const wait=facts?await brokerWouldWaitV1(environment,facts,forecastKey()):undefined;return wait===undefined?"unknown":wait?"wait":"admit";}});
   if(!gate.offload){offload.declineOffloadV1(settings.root,environment,{crate,key,reason:gate.reason==="local-slot"||gate.reason==="unmeasured"||gate.reason==="final-wait"?gate.reason:"cheap",silent:true});return undefined;}
   const light=settings.lightSlots>0?await acquireLightSlotV1(settings.root,settings.lightSlots,{signal:controller.signal,onWait:waited=>event("slot-wait",null,0,waited)}):undefined;
   const answer=await offload.offloadRustcUnitV1({args,environment,cwd,recorded:recorded.map(row=>row.path),root:settings.root,lane:settings.lane,crate,key,compiler:settings.inner,digest:path=>digestFileV1(path,settings.stamps),signal:controller.signal,queue:gate.queue,...gate.reason==="final-beyond-host"?{beyondHost:{report:text=>{process.stderr.write(`[unit-cache] ${text}\n`);}}}:{}}).finally(()=>light?.release());
   if(received!==undefined)return{status:received,stdout:"",stderr:"",origin:""};
   if(gate.reason==="final-beyond-host"&&answer.kind==="local"&&answer.reason==="beyond-host-refused")return{status:1,stdout:"",stderr:offload.offloadTextsV1("beyondHost.refused",{crate,reason:answer.detail})+"\n",origin:""};
   return answer.kind==="done"?{status:answer.status,stdout:answer.stdout,stderr:answer.stderr,origin:answer.origin}:undefined;
  }catch{return received===undefined?undefined:{status:received,stdout:"",stderr:"",origin:""};}
  finally{for(const [name,handler] of handlers)process.off(name,handler);}
 };
 const declareManifest=()=>manifestDeclarationV1({toolchain:keyInput.toolchain,entry:resolve(cwd,unit.source),entrySha256:digestFileV1(resolve(cwd,unit.source),settings.stamps),cwd,workspace:workspaceOf});
 const shareHeldManifest=async():Promise<void>=>{if(manifestId!==undefined&&manifestHeld?.from==="local"&&manifestHeld.state?.shared===false&&manifestClient?.settings.upload===true&&manifestClient.usable()){const shared=await shareManifestV1(settings.root,manifestId,manifestHeld.state.manifest,declareManifest(),manifestClient);event("manifest-publish",null,0,0,shared+": "+manifestHeld.state.manifest.files.length+" files, "+manifestHeld.state.manifest.env.length+" names");}};
 const basics=unitBasicsV1({root:settings.root,environment,inner:settings.inner,unit,args,cwd}),prepare=(paths:readonly string[],localKey=key)=>basics?prepareFleetV1({root:settings.root,environment,basics,unit,args,cwd,laneRoot:settings.laneRoot,localKey,keyInput,toolchain:keyInput.toolchain,crate,sensitive:paths,expected:neutralExpected,tools}):undefined;
 const offer=()=>{
  if(!environment.SEMIO_REMOTE_CACHE_ENDPOINT||environment.SEMIO_REMOTE_CACHE_ACCESS!=="read-write"||newestEntryV1(settings.root,key)?.session===true)return;
  if(restoredKinds===undefined){const unit=prepare(sensitive);if(unit)offerFleetUnitV1(unit);return;}
  if(restoredKinds.length===0||!restoredKinds.every(kind=>(ROOT_KINDS_V1 as readonly string[]).includes(kind)))return;
  const unit=prepare(outputSensitive),base=prepare(sensitive);
  if(unit&&base)offerFleetUnitV1(unit,{neutralKey:base.key,kinds:restoredKinds});
 };
 try{if(await serveLocal()){event("hit",key,Date.now()-started);await shareHeldManifest();lookup("local-hit");countUnitV1(workspaceOf,"localHit");offer();return 0;}}catch{}
 let fleet=prepare(sensitive),rooted:readonly string[]|undefined;shared=fleet!==undefined;let neutral=fleet;
 const adopt=async():Promise<FleetUnitV1|undefined>=>{if(!covered||rooted!==undefined||neutral===undefined)return undefined;const kinds=await rootHintV1(neutral),paths=kinds&&scope(kinds),alt=paths?prepare(sensitiveTo(paths),scopedKey()):undefined;if(alt&&kinds){rooted=kinds;fleet=alt;return alt;}return undefined;};
 if(!fleet){const why=fleetUnidentifiedReasonV1(settings.root,environment);if(why!==undefined){event("remote-skip",key,0,0,why);countSkipV1(workspaceOf,why);countFallbackV1(settings.root,why);}}
 const breaker=fleet?.client.state()??"ok";if(fleet&&breaker!=="ok"){event("remote-skip",key,0,0,"breaker: "+breaker+", the upload waits in the spool");countFallbackV1(settings.root,"breaker-"+breaker);}

 let hold:FleetHoldV1|undefined,refreshed=false;
 const rekey=async(manifest:InputManifestV1):Promise<boolean>=>{
  const sources=await unitSourcesV1(resolve(cwd,unit.source),manifestDir,environment.OUT_DIR,path=>digestFileV1(path,settings.stamps),manifestFilesV1(manifest,roots),manifest.env);if(sources===null)return false;
  manifestUsed=manifest;sourcesRead=sources;keyInput=inputsOf(sources);key=localKeyV1(keyInput,roots,sensitive);parts=unitKeyPartsV1(keyInput);keyedNames=new Set([...keyInput.environment.map(row=>row[0]),...sources.names]);return true;
 };
 const refreshManifest=async():Promise<boolean>=>{
  if(refreshed||manifestId===undefined||manifestHeld?.from!=="local")return false;
  refreshed=true;
  const newer=await refreshManifestV1(manifestClient,manifestId);if(newer===undefined||manifestCoversV1(manifestUsed,newer))return false;
  const union=manifestUnionV1(manifestUsed,newer);if(!manifestWithinBoundsV1(union)||!await rekey(union))return false;
  writeManifestV1(settings.root,manifestId,union,manifestCoversV1(newer,union));return true;
 };
 const completeInputsV1=async(outputs:readonly string[]):Promise<string[]>=>{
  if(sourcesRead===null)return[];
  const depInfo=outputs.find(path=>path.endsWith(".d"));if(depInfo===undefined)return["dep-info"];
  let found:string[];try{found=keyGapV1({cwd,covered:new Set(sourcesRead.files.map(file=>resolve(file))),keyedNames,depInfo:readFileSync(depInfo,"utf8"),...manifestDir===undefined?{}:{crateDirectory:manifestDir}});}catch{return["dep-info"];}
  const observed=observedManifestV1(found,{cwd,roots}),grown=!manifestCoversV1(manifestUsed,observed),learned=grown?manifestUnionV1(manifestUsed,observed):manifestUsed??manifestOfV1([],[]);
  if(!manifestWithinBoundsV1(learned))return["manifest-bound"];
  if(grown){
   if(!await rekey(learned))return["dep-info"];
   if(fleet){await hold?.release();hold=undefined;rooted=undefined;fleet=prepare(sensitive);neutral=fleet;if(fleet&&fleet.client.state()==="ok")hold=await holdFleetKeyV1(fleet);}
  }
  if(manifestId!==undefined&&(grown||manifestHeld?.state===undefined)){writeManifestV1(settings.root,manifestId,learned,false);if(manifestHeld?.from!=="unknown"){const shared=await shareManifestV1(settings.root,manifestId,learned,declareManifest(),manifestClient);event("manifest-publish",null,0,0,shared+": "+learned.files.length+" files, "+learned.env.length+" names");}}
  return[];
 };
 await shareHeldManifest();
 if(await refreshManifest()){
  try{if(await serveLocal()){event("hit",key,Date.now()-started,0,"manifest-refreshed");lookup("local-hit");countUnitV1(workspaceOf,"localHit");return 0;}}catch{}
  fleet=prepare(sensitive);neutral=fleet;shared=fleet!==undefined;
 }
 let claim:ClaimV1|undefined;
 try{claim=await acquireUnitClaimV1(join(settings.root,policy.layout.locks),key.slice(0,40),{});}catch{claim=undefined;}
 const waited=claim?.waitedMs??0;
 if(waited>50||!claim)event(claim?"lock-wait":"lock-timeout",key,0,waited);
 try{
  if(claim){try{if(await serveLocal()){event("hit",key,Date.now()-started,waited,"after-wait");lookup("local-hit");countUnitV1(workspaceOf,"localHit");return 0;}}catch{}}
  let explaining:ReturnType<typeof explainFleetMissV1>|undefined;
  if(fleet&&breaker==="ok"){
   await adopt();
   const decision=await obtainFleetUnitV1(fleet!,(kind,milliseconds,waitedMs,reason)=>event(kind,key,milliseconds,waitedMs,reason),{},adopt);
   if(decision.outcome==="hit"){try{if(await serve(fleet!.localKey)){lookup("fleet-hit",fleet!.key);countUnitV1(workspaceOf,"remoteHit");await awaitFleetSettlesV1();return 0;}}catch{}countFallbackV1(settings.root,"hit-unservable");}
   else{hold=decision.hold;if(decision.reason!=="granted")countFallbackV1(settings.root,decision.reason);if(fleet.keyComponents!==undefined)explaining=explainFleetMissV1(fleet);}
  }
  const compiled=Date.now();let remote=await offloadUnitV1();
  if(remote&&remote.status===0){const absent=missingOutputsV1({directory:outDir,cwd,expected,text:remote.stderr+remote.stdout});if(absent.length>0){event("incomplete",key,Date.now()-compiled,0,"offload "+remote.origin+" returned a unit without "+absent.join(","));remote=undefined;}}
  const localInputKey=remote?undefined:forecastKey();
  const outcome=remote??await compile(true,localInputKey);
  if(remote){process.stdout.write(remote.stdout);process.stderr.write(remote.stderr);}
  if(outcome.status!==0){await hold?.release();event("failed",key,Date.now()-compiled,waited);return outcome.status;}
  if(explaining!==undefined){const nearest=await Promise.race([explaining,new Promise<undefined>(done=>{const wait=setTimeout(()=>done(undefined),1500);wait.unref?.();})]);if(nearest!==undefined)process.stderr.write((["en","de"] as const).map(language=>laneMessageV1("missNearest",{crate,key:nearest.key.slice(0,12),differs:nearest.differs.length===0?"-":nearest.differs.join(",")},language)).join(" / ")+"\n");}
  if(!remote&&outcome.peakRssBytes)void reportUnitCeilingV1(crate,unit.target,unit.crateTypes,environment,outcome.peakRssBytes);countUnitV1(workspaceOf,"miss");
  const outputs=[...new Set([...announcedPathsV1(outcome.stderr),...announcedPathsV1(outcome.stdout)])].map(path=>resolve(cwd,path)).filter(path=>existsSync(path)),lacking=uncoveredOutputsV1(outputs.map(path=>basename(path)),expected);
  const published=Date.now();
  try{
   const gap=outputs.length===0||lacking.length>0?[]:await completeInputsV1(outputs);
   const leak=covered?leakedRootKindsV1(outputs,named):undefined,paths=scope(leak?.kinds??[]);
   verifiedOutputs=paths!==undefined;
   if(leak&&paths!==undefined){outputSensitive=sensitiveTo(paths);key=scopedKey();}
   event("miss",key,Date.now()-compiled,waited,(unit.incremental===undefined?parts:parts+" incremental:lane")+(remote?" offloaded:"+remote.origin:""),remote?undefined:outcome.peakRssBytes);lookup("miss",fleet?.key);
   if(paths!==undefined){
    markExternIdentityV1(outputs,portableKeyV1(keyInput,roots),outputSensitive,settings.stamps,crateHashOf);
    markProducerKeyV1(outputs,key,settings.stamps);
   }
   let tables=true;
   for(const path of outputs.filter(path=>path.endsWith(".rmeta"))){const written=writeDependenciesV1(path,settings.inner,environment);if(written!==undefined)outputs.push(written);else if(ifaceKeyed)tables=false;}
   const incrementalBuild=remote===undefined&&incrementalSession();
   const verifiedInput=verifiedOutputs&&outputs.length>0&&lacking.length===0&&gap.length===0&&localInputKey===forecastKey()?localInputKey:undefined;
   const provenance=facts&&!remote?await recallProvenanceV1(environment,facts,verifiedInput):undefined;
   if(!remote){const took=ranMs,measure=provenance===undefined?undefined:{wallMs:provenance.wallMs,userMs:provenance.userMs,sysMs:provenance.sysMs};void import("../../../🏗️build/🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts").then(module=>{module.writeUnitCostV1(settings.root,crate,took,measure);}).catch(()=>undefined);}
   if(provenance!==undefined&&provenance.peakRssBytes>0)void import("../../../🏗️build/🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts").then(module=>{module.writeUnitPeakV1(settings.root,crate,provenance.peakRssBytes/1048576,1);}).catch(()=>undefined);
   const publish=(spill=false)=>publishUnitV1(settings.root,{key,crate,cwd,outDir,...environment.OUT_DIR?{generated:environment.OUT_DIR}:{},laneRoot:settings.laneRoot,outputs,expected,stdout:outcome.stdout,stderr:outcome.stderr,stampDirectory:settings.stamps,extra,provenance:provenance??null,retention:{workspace:workspaceOf,externs:[...keyInput.externs.map(row=>row.sha256),...rawExterns],units:[...producers,...[buildRunKeyV1(environment.OUT_DIR)].filter((name):name is string=>name!==undefined)],toolchain:keyInput.toolchain},...(basics?{slot:basics.slot}:{}),...(incrementalBuild?{session:true}:{}),...(spill?{spill:true}:{})});
   let entry:ReturnType<typeof publishUnitV1>;
   if(paths===undefined||outputs.length===0||lacking.length>0||!tables)entry=undefined;else try{entry=publish();}catch{await new Promise(done=>setTimeout(done,250));try{entry=publish();}catch(error){if(!fleet)throw error;entry=publish(true);if(entry)event("publish",key,Date.now()-published,0,"spill: the host store refused the objects, the fleet upload reads them from the spill folder");}}
   if(entry)event("publish",key,Date.now()-published,0);
   else{const why=paths===undefined?"unreadable-outputs":outputs.length===0?"no-outputs":lacking.length>0?"incomplete: "+lacking.join(","):!tables?"interface: the compiler did not list the crate hashes":outputs.some(path=>path.endsWith(".d"))?"refused":"no-dep-info";event("publish-skip",key,Date.now()-compiled,0,why);if(fleet)countSkipV1(workspaceOf,"publish-"+skipNameV1(why));}
   if(entry)settle(outputs,entry);
   const kinds=leak?.kinds,shareable=kinds!==undefined&&paths!==undefined&&kinds.every(kind=>(ROOT_KINDS_V1 as readonly string[]).includes(kind)&&(rooted===undefined||rooted.includes(kind)));
   if(fleet&&entry&&incrementalBuild){event("remote-skip",key,0,0,"incremental: the unit was compiled with an incremental session, whose bytes differ on every run");countSkipV1(workspaceOf,"incremental");await hold?.release();}
   else if(fleet&&entry&&gap.length>0){event("remote-skip",key,0,0,"key-incomplete: "+gap.slice(0,4).join(", ")+(gap.length>4?" and "+(gap.length-4)+" more":""));countSkipV1(workspaceOf,"key-incomplete");await hold?.release();}
   else if(fleet&&entry&&(rooted===undefined?leak===undefined:shareable)){if(!await shareFleetUnitAwaitingV1(fleet,{claimed:hold,costMs:Date.now()-compiled})){countSkipV1(workspaceOf,unsharedReasonV1(fleet));await hold?.release();}}
   else if(fleet&&entry&&shareable&&neutral&&!neutral.client.usable()){
    const alt0=prepare(outputSensitive);
    await hold?.release();
    if(alt0&&shareFleetUnitV1(alt0,{claimed:undefined,costMs:Date.now()-compiled,hint:{neutralKey:neutral.key,kinds:kinds!}}))countFallbackV1(settings.root,"root-keyed-deferred");
    else{countSkipV1(workspaceOf,"root-key-failed");countFallbackV1(settings.root,"root-key-failed");}
   }
   else if(fleet&&entry&&shareable&&neutral){
    const alt0=prepare(outputSensitive),held0=alt0?await holdFleetKeyV1(alt0):undefined,effective=alt0?await publishRootHintV1(neutral,kinds!):undefined;
    let alt=effective===undefined?undefined:alt0,held=held0;
    if(effective!==undefined&&alt0!==undefined&&(effective.length!==kinds!.length||!kinds!.every(kind=>(effective as readonly string[]).includes(kind)))){await held0?.release();const effectivePaths=kinds!.every(kind=>(effective as readonly string[]).includes(kind))?scope(effective):undefined;alt=effectivePaths?prepare(sensitiveTo(effectivePaths)):undefined;held=alt?await holdFleetKeyV1(alt):undefined;}
    await hold?.release();
    if(alt&&shareFleetUnitV1(alt,{claimed:held,costMs:Date.now()-compiled}))countFallbackV1(settings.root,"root-keyed");
    else{await held?.release();countSkipV1(workspaceOf,"root-key-failed");countFallbackV1(settings.root,"root-key-failed");}
   }
   else{if(leak!==undefined){event("remote-skip",key,0,0,"path-dependent: the compiled bytes hold "+leak.file+", a directory of this machine that cannot be keyed ("+leak.kinds.join(",")+"), so the unit is not shared");countSkipV1(workspaceOf,"path-dependent");countFallbackV1(settings.root,"path-dependent");}await hold?.release();}
  }catch(error){event("publish-skip",key,Date.now()-compiled,0,"error: "+String((error as Error)?.message??error).slice(0,200));if(fleet)countSkipV1(workspaceOf,"publish-error");await hold?.release();}
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
 if(command==="fleet-upload"){const idle=rest.includes("--once")?0:undefined;return await drainUploadsV1(root,process.env,{...idle===undefined?{}:{idleMilliseconds:idle},...rest.includes("--takeover")?{takeover:true}:{}});}
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
