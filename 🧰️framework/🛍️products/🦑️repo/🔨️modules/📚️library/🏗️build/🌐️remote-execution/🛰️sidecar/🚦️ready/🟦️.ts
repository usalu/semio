import {processPidExistsV1} from "../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts"
import{TERMINATION_POLICY_V1}from"../../../../../../../../🔨️modules/🏃️process/🪓️termination/🟦️.ts";
import{toolsProgressPaths,recordToolsProgress,sampleToolsProgress,toolsGraceActive,toolsGraceDenial,currentToolsCheckpoint,type ToolsCheckpoint,type ToolsGraceDenial}from"./⏳️tools-grace/🟦️.ts";
import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import{readFreeSpaceV1,type FreeSpaceOptionsV1,type FreeSpaceUnknownReasonV1}from"../../../../../../../../🔨️modules/🏃️process/🆓️free-space/🟦️.ts";
import{nativeShardConsumerV1,nativeShardTextV1,readNativeShardsV1,relieveNativeShardsV1,type NativeShardConsumerV1}from"../../../🌡️host-capacity/🧹️disk-recovery/🧱️native-shards/🟦️.ts";
import{volumeFactsTextV1}from"../../../🌡️host-capacity/🧹️disk-recovery/📊️volume-facts/🟦️.ts";
import{nativeShardHoldV1}from"../../../🌡️host-capacity/🧹️disk-recovery/🧱️native-shards/🔒️hold/🟦️.ts";
import{currentHostWardenMirrorV1,ensureHostWardenV1,readHostWardenCancelsSinceV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts";
import{formatHostWardenCancelLineV1,type HostWardenCancelV1,type HostWardenMirrorV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🟦️.ts";
import{createHash}from"node:crypto";
import{execFile,spawn,spawnSync}from"node:child_process";
import{closeSync,constants,existsSync,fstatSync,mkdirSync,openSync,readFileSync,readSync,readdirSync,realpathSync,renameSync,rmSync,statSync,writeFileSync,writeSync,type BigIntStats}from"node:fs";
import{availableParallelism,homedir,hostname,tmpdir,totalmem}from"node:os";
import{delimiter,dirname,isAbsolute,join,relative,sep}from"node:path";
import{StringDecoder}from"node:string_decoder";
import{readToolSetupRefusalLineV1}from"../../../🧰️tool-setup/🛑️refusal/🟦️.ts";
import{nxFailedTaskTailsV1}from"../../../🔧️tools/🧶️nx/🪵️failure-tail/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{createPartitionCensusMirrorV1}from"./🧱️partition-census/🟦️.ts";
import{READY_IMAGE_POLICY_V1,readyImageFactsV1,readyImageNeedBytesV1,readyImageRowV1,readyImageVerdictV1,readyImagePartitionOfLinesV1,readyImagePurposeV1,readyImageRecheckDueV1,readyImageReliefV1,readyImageRetainBytesV1,readyImageShortV1,readyImageStoreV1,readyImageWritersV1,readyNoSpaceEvidenceV1,type ReadyImageShortV1}from"./🖼️image-guard/🟦️.ts";
import{createTicketGeneratedSweepV1}from"./🧹️ticket-generated/🟦️.ts";
import{fleetHostReadingV1}from"../../../🧠️memory/🟨️.mjs";
import{READY_SLICE_REPORT_POLICY_V1,ownedSliceReportV1,prepareOwnedSliceReportV1}from"../../../🧠️memory/📋️slice-report/🏷️owner/🟦️.ts";
import{remoteExecutionOriginV1}from"../../🌡️load/🟦️.ts";
import{startFleetMetricsV1}from"../📈️metrics/🟦️.ts";
import{createReadyStageClaimPublisherV1}from"../../🌡️load/🪑️stage-claim/🟦️.ts";
import{hostCapacityDirectoryV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/📖️status/🟦️.ts";
import type{RemoteExecutionSidecarContextV1}from"../🧰️shared/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import nativeBootstrap from"../../../../../🔩️native/🥾️bootstrap/🧬️schema/🔣️.json"with{type:"json"};
import{readyWaitedMillisecondsV1,readyCodeStopV1,readyStaleWaitingV1,readyWaitLineV1,readyCapacityWaitingV1,readyCapacityWaitLiveV1,readyDeadlineV1,readyForgetTreeV1,readyStallTrackerV1,readyStalledV1,readyCpuOnlyStalledV1,readyGraphCounterTrackerV1,readyAbsoluteDeadlineV1,readyReadingLiveV1,readyStallCountV1,readyWardenHoldsV1,readyWardenCancelOfStageV1,readyKillRowsV1,readyStopSenderV1,readyAdoptionClockV1,readyGrantWaitV1,readySliceSilentV1,readySliceMovedSignatureV1,readyIdentityV1,readyPinStageRootV1,readyStageProcessVerdictV1,readyConfirmProcessReplacementV1,readyKillTreeV1,readyLivenessV1,readyTreeCpuV1,readyLeaseVerdictV1,readyHolderRestartV1,readyStaleStopV1,readyStagePhaseV1,type ReadyPhaseV1,readyHolderWaitTrackerV1,readyHolderWaitOfV1,readyHolderCustodyV1,readyBackgroundReaderV1,readyForegroundBusyV1,type ReadyHolderWaitV1,type ReadyReadRecordV1,readyScaledTreePolicyV1,readyStallRestartV1,readyStepWatchV1,readyStepOverdueSecondsV1,type ReadyStepV1,type ReadyStepWatchCheckpointV1,readySemanticsV1,readySemanticsChangedV1,readyProcessStartSyncV1,readyProgressKeyV1,readyReadTableV1,readyReadTreeV1,readyTreeMembersV1,readyWindowsTreeSampleV1,readyWriteTreeV1,READY_TREE_POLICY_V1,type ReadyWindowsWatchdogSampleV1,type ReadyKnownV1,type ReadyProcessV1}from"./🌲️process-tree/🟦️.ts";
import{CAS_RESERVE_FACTOR_V1}from"../../../🗄️cas/🧬️schema/🟦️.ts";
import{readySliceSampleDueV1,readySliceSampleV1}from"./🌲️process-tree/🔬️sample/🟦️.ts";
import{localMachineSpeedFactorV1,scaledMillisecondsV1}from"../../../🐢️speed/🟨️.mjs";
import{inspectTypescriptCompilerV1}from"../../../🚀️bootstrap/🔷️typescript-compiler/🟨️.mjs";
import{NATIVE_IMAGE_SHARED_READ_POLICY_V1,withNativeImageSharedReadV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤲️shared-read/🟦️.ts";
import{discardDependencyGenerationsV1}from"../../../../⚡️caching/🚀️bootstrap/📦️dependencies/♻️collection/🟦️.ts";
import cachePolicy from"../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json"with{type:"json"};
import{readyManifestDigestV1}from"./📦️manifest-digest/🟦️.ts";
import{createSidecarCodeMirrorV1}from"../📇️code-record/🟦️.ts";
import{readBuildCountersV1,unitLedgerV1,type UnitCountKindV1,type UnitLedgerV1}from"../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📊️counters/🟦️.ts";
import{javascriptPublishedLockInputsV1}from"../../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔒️lock-sync/📋️inputs/🟦️.ts";
import{READY_CONTAINMENT_POLICY_V1,readyContainmentLineV1,readyContainmentStatusV1,readyHeirsV1,readyKillRecorderV1,readyOwnerTokenV1,readyReapOrphansV1,readyRegistryEntriesV1,readyRegistryRootV1,readySharedPidsV1,readySampleDelayV1,readyStopV1,readyStoppedWindowsV1,readySweepRostersV1,type ReadyKillReasonV1,type ReadyContainmentResultV1,type ReadyContainmentV1,type ReadyPersonV1,type ReadyReapedV1,type ReadyWindowV1}from"./🌲️process-tree/🧹️containment/🟦️.ts";

/** 🚦️ The numbers of the readiness loop (see `./🧬️schema/🔣️.json`). */
export const READY_POLICY_V1=Object.freeze(schema.definitions.Policy.const);
/** 🪜️ The stages in order: what each runs and what it depends on. */
export const READY_STAGES_V1=schema.definitions.Stages.const as readonly ReadyStageV1[];
/** 🎯️ The declared probes of the `workspace` stage: one target of a handful of projects of different kinds, so readiness reads their closure and never the whole workspace. */
export const READY_PROBES_V1=schema.definitions.Probes.const as Readonly<{version:1;target:string;projects:readonly Readonly<{name:string;kind:string}>[]}>;
/** 🧬️ The schema of `ready.json`. */
export const READY_STATUS_SCHEMA_V1=schema.definitions.Status;
/** 🏔️ Whether the readiness loop of `origin` is busy (see `readyForegroundBusyV1`), read from its `ready.json` in `store`; a store that is not there or a file that cannot be read says no, so a machine without a readiness loop (the group server) is never held back. */
export function readyForegroundBusyOfStoreV1(store:string|undefined,origin:string,nowMs:number=Date.now()):boolean{
 if(store===undefined||store==="")return false;
 try{return readyForegroundBusyV1(JSON.parse(readFileSync(join(store,"v1","origins",origin,READY_POLICY_V1.statusName),"utf8")),nowMs);}catch{return false;}
}
const P=READY_POLICY_V1,ZERO="0".repeat(64),sha=(text:string|Uint8Array):string=>createHash("sha256").update(text).digest("hex");

export type ReadyStageV1=Readonly<{name:string;kind:"bootstrap"|"repo";arguments:readonly string[];timeoutMilliseconds:number;inputs:readonly string[];bootstrapInput:boolean;dependencyInputs?:true}>;
export type ReadyPlatformV1="linux"|"macos"|"windows";
export type ReadyCodeV1="human-running"|"lease-held"|"memory-low"|"stage-failed"|"tooling-compiler"|"stage-timeout"|"stage-stalled"|"stage-stall-exhausted"|"stage-code-changed"|"stage-code-stale"|"stage-code-stale-waiting"|"stage-step-overdue"|"stage-holder-silent"|"store-full"|"source-transient"|"disk-short"|"disk-unmeasured"|"disk-collecting"|"disk-consumers"|"disk-unfit"|"loop-failed"|"spawn-failed"|"interrupted"|"memory-cancelled"|"disk-guard"|"image-full"|"image-short"|"server";
export type ReadyReasonCodeV1="first-run"|"inputs-changed"|"previous-failed"|"stalled"|"code-changed"|"code-stale"|"code-stale-waiting"|"step-overdue"|"holder-silent"|"adopted"|"binary-missing"|"refreshing";
export type ReadyReasonV1=Readonly<{code:ReadyReasonCodeV1;en:string;de:string}>;
export type ReadyRefusalV1=Readonly<{code:ReadyCodeV1;en:string;de:string}>;
export type ReadyStateNameV1="ready"|"running"|"waiting"|"backoff"|"observing"|"server";
export type ReadyStatusV1={version:1;origin:string;host:string;platform:ReadyPlatformV1;pid:number;state:ReadyStateNameV1;digest:string;ready:boolean;stage:string|null;stageIndex:number;stageCount:number;attempts:number;exit:number|null;refusal:ReadyRefusalV1|null;reason:ReadyReasonV1|null;containment?:ReadyContainmentV1|null;peakMegabytes:number|null;tail:string[];nextAttemptMs:number|null;readyMs:number|null;refreshing?:boolean;startedMs:number;updatedMs:number};
type ReadyOwnedActivityV1=Readonly<{resource:string;counter:number}>;
type ReadyDeadlineStopV1="hard-cap"|"soft-cap-idle";
type ReadyStepOverdueEvidenceV1=Readonly<{seconds:number;bound:number;limit:number;quietMs:number}>;
type ReadyStopEvidenceV1=Readonly<{deadline?:ReadyDeadlineStopV1;toolsGraceDenial?:ToolsGraceDenial;stepOverdue?:ReadyStepOverdueEvidenceV1}>;
type ReadyTimingWatchdogV1=ReadyStopEvidenceV1&Readonly<{lastActivity?:ReadyOwnedActivityV1}>;
export type ReadyTimingV1=Readonly<{units?:UnitLedgerV1;watchdog?:ReadyTimingWatchdogV1;stage:string;startedMs:number;endedMs:number;seconds:number;exit:number|null;outcome:"ok"|"stage-failed"|"stage-timeout"|"stalled"|"spawn-failed"|"interrupted";peakMegabytes:number|null;growthMegabytes?:number|null;growthInodes?:number|null;attempts:number|null;digest:string|null;endEvidence?:ReadyEndEvidenceV1;signal?:string|null;sender?:string;stopBy?:Readonly<{supervisor:number;reason:string;atMs:number}>}>;
export type ReadyFailureV1=Readonly<{digest:string;stage:string;attempts:number;exit:number|null;signal?:string|null;code:ReadyCodeV1;nextAttemptMs:number;atMs:number;tail:readonly string[];tree?:string}>;
export type ReadyLocalStateV1={version:1;stages:Record<string,{digest:string;doneMs:number}>;appliedCompletions?:Readonly<Record<string,string>>;failure:ReadyFailureV1|null;attempts:number;peakMegabytes:number|null;readyMs:number|null;adopted:boolean};

/** 🖥️ The platform name of the fleet for a Node platform. */
export function readyPlatformV1(platform:NodeJS.Platform=process.platform):ReadyPlatformV1{return platform==="darwin"?"macos":platform==="win32"?"windows":"linux";}

/** 💡️ Why a stage is (re)started, with its text in English and German. */
export function readyReasonV1(code:ReadyReasonCodeV1,stage:string):ReadyReasonV1{
 const fill=(language:"en"|"de"):string=>(schema.definitions.Reasons.const[language] as Record<string,string>)[code]!.replace("{stage}",stage);
 return Object.freeze({code,en:fill("en"),de:fill("de")});
}

/** 🪦️ How a stage process ended in words of both languages: an exit status, the signal that ended it, or neither (the process vanished without leaving an exit file, so something outside the supervisor ended its wrapper). A pure function.
 * @see ./🧪️tests/🪦️ending */
export function readyEndingV1(exit:number|null,signal:string|null|undefined):Readonly<{en:string;de:string}>{
 if(exit!==null)return{en:`exit status ${exit}`,de:`Exit-Status ${exit}`};
 if(signal!==null&&signal!==undefined)return{en:`no exit status, ended by signal ${signal}`,de:`kein Exit-Status, durch das Signal ${signal} beendet`};
 return{en:"no exit status and no signal: the stage process vanished, the last lines of its log say how",de:"kein Exit-Status und kein Signal: der Stufenprozess verschwand, die letzten Zeilen seines Protokolls sagen wie"};
}

/** 🗣️ The refusal with its text in English and German. */
export function readyRefusalV1(code:ReadyCodeV1,fields:Readonly<Record<string,string|number|Readonly<{en:string;de:string}>>>={}):ReadyRefusalV1{
 const fill=(language:"en"|"de"):string=>(schema.definitions.Texts.const[language] as Record<string,string>)[code]!.replace(/\{(\w+)\}/gu,(_all,name:string)=>{const value=fields[name];return String(value!==null&&typeof value==="object"?value[language]:value??"");});
 return Object.freeze({code,en:fill("en"),de:fill("de")});
}

/** 📨️ Who ended a stage run, for the run record of `ready-timings.json`: the kind of the sender the supervisor decided on (`supervisor`, `deadline`, `warden`, `runner`, `outside`) and, when this supervisor stopped the stage itself, the reason, its pid and the time it wrote into the stage record before it sent the signal, so a census of stops needs no log parsing. A pure function. */
export const readyTimingSenderV1=(result:Pick<ReadyRunResultV1,"sender"|"stopBy">):Readonly<{sender?:string;stopBy?:Readonly<{supervisor:number;reason:string;atMs:number}>}>=>({...result.sender===undefined?{}:{sender:result.sender},...result.stopBy===undefined?{}:{stopBy:{supervisor:result.stopBy.supervisor,reason:result.stopBy.reason,atMs:result.stopBy.atMs}}});

/** #️⃣ Hashes each stage independently from its name, host platform and admitted input hashes, then combines the stage digests in bootstrap order. */
export function readyDigestsV1(input:Readonly<{platform:ReadyPlatformV1;hash:(path:string,stage:ReadyStageV1)=>string|undefined;stages?:readonly ReadyStageV1[]}>):Readonly<{digest:string;stages:readonly Readonly<{name:string;digest:string}>[]}>{
 const windows=input.platform==="windows",bootstrap=(windows?P.bootstrap.windows:P.bootstrap.unix).join("/");
 const nativeInputs=windows?[bootstrap,nativeBootstrap.definitions.RustupInstallation.const.windowsProcess]:[bootstrap];
 const stages=(input.stages??READY_STAGES_V1).map(stage=>{
  const paths=[...stage.inputs,...stage.bootstrapInput?nativeInputs:[]];
  const digest=sha(`${stage.name}\n${input.platform}\n${ZERO}\n`+paths.map(path=>`${path}\0${input.hash(path,stage)??"absent"}\n`).join(""));
  return{name:stage.name,digest};
 });
 return Object.freeze({digest:sha(stages.map(stage=>stage.digest).join("\n")),stages:Object.freeze(stages)});
}

/** 🧪️ Hashes the bootstrap assets, executable locations and verification recipes consumed on the host. */
export function readyInputHashV1(path:string,content:string,platform:NodeJS.Platform=process.platform):string{
 try{
  if(path.endsWith("🥾️bootstrap/🧬️schema/🔣️.json")){
   const {definitions}=JSON.parse(content);
   const assets=definitions.Pins.const.assets.filter((asset:{platform:string})=>asset.platform===platform||asset.platform==="any");
   if(platform==="win32")return sha(JSON.stringify([assets,definitions.MsvcBootstrap?.const,definitions.WindowsNodeInstallation?.const]));
   const pinRows=new Set(content.split(/\r?\n/u).map(line=>line.trim().replace(/,$/u,"")));
   if(!assets.every((asset:unknown)=>pinRows.has(JSON.stringify(asset))))return sha(content);
   return sha(JSON.stringify(assets));
  }
  if(path.endsWith("🧰️tool-setup/🔣️.json")){
   type Locator={kind:string;windowsProgramDirectories?:string[]};
   type Tool={id:string;pin:unknown;probe?:unknown;locate:Locator[];setup:{kind:string}};
   type Step={id:string;kind:string;configuration:unknown;platforms:string[];timeoutSeconds:number;optional?:boolean};
   const doc=JSON.parse(content) as {groups:unknown;tools:Tool[];steps:Step[]};
   const tools=doc.tools.map(row=>{
    const locate=platform==="win32"?row.locate:row.locate.filter(entry=>entry.kind!=="bootstrap-node").map(({windowsProgramDirectories,...entry})=>entry);
    const setup=platform!=="win32"&&row.setup.kind==="bootstrap-node"?{...row.setup,kind:"bootstrap"}:row.setup;
    return [row.id,row.pin,row.probe,locate,setup];
   });
   const steps=doc.steps.filter(step=>step.platforms.includes(platform)).map(step=>[step.id,step.kind,step.configuration,step.timeoutSeconds,step.optional]);
   return sha(JSON.stringify({groups:doc.groups,tools,steps}));
  }
  if(path==="package.json"){const doc=JSON.parse(content) as {packageManager?:unknown;engines?:unknown;workspaces?:unknown};return sha(JSON.stringify({packageManager:doc.packageManager??null,engines:doc.engines??null,workspaces:doc.workspaces??null}));}
 }catch{}
 return sha(content);
}

/** ⏳️ The wait after the `attempts`-th failure in a row: it doubles from the initial wait to the maximum, grows by up to the jitter share (`unit` in [0,1), fixed per machine so the fleet does not retry in step) and never exceeds the maximum. */
export function readyBackoffV1(attempts:number,unit:number):number{
 const base=Math.min(P.backoffMaximumMilliseconds,P.backoffInitialMilliseconds*2**Math.max(0,attempts-1));
 return Math.min(P.backoffMaximumMilliseconds,Math.floor(base*(10000+Math.floor(unit*P.backoffJitterBasisPoints))/10000));
}

/** 💽️ The disk or quota that a failed stage reported as full, from the lines of its log: the errno (`EDQUOT` or `ENOSPC`) and the directory the lease store or a database could not create files in (see `storeFullPattern` of the schema; the message comes from the SQLite open explanation); `null` when no line says so. */
export function readyStoreFullV1(lines:readonly string[]):Readonly<{errno:string;path:string}>|null{
 const expression=new RegExp(P.storeFullPattern,"u");
 for(let index=lines.length-1;index>=0;index--){const found=expression.exec(lines[index]!);if(found!==null)return Object.freeze({errno:found[1]!,path:found[2]!});}
 return null;
}

/** 📦️ What a filesystem says it has free at a path: bytes available to the user and free inodes, `null` for a fact the platform does not report (Windows reports no inodes, a network share may report nothing). `freeBytes` is `null` exactly when the volume could not be measured, and then `unknown` says why: an unknown measurement is never read as zero bytes free. */
export type ReadyFilesystemFactsV1=Readonly<{freeBytes:number|null;freeInodes:number|null;totalBytes?:number|null;totalInodes?:number|null;unknown?:Readonly<{reason:FreeSpaceUnknownReasonV1;detail:string}>}>;
/** 🕳️ A path whose free space could not be measured, with the reason and a short detail. */
export type ReadyDiskUnknownV1=Readonly<{path:string;reason:string;detail:string}>;
/** 💽️ The first resource a stage lacks before it starts: the directory, `space` or `inodes`, what the stage needs and what is free (megabytes for `space`). */
export type ReadyDiskShortV1=Readonly<{path:string;resource:"space"|"inodes";needed:number;available:number;partition?:string;total?:number;host?:number|null;limitedByHost?:boolean}>;

/** 🧾️ One consumer a short disk names: the writer, its root, the bytes it holds and the exact command that frees it. */
export type ReadyDiskConsumerV1=Readonly<{id:string;root:string;bytes:number;command:string}>;
/** 🧹️ What the collection for a waiting stage answered: whether the need fits now and, when it does not, the largest consumers that remain. */
export type ReadyDiskRecoveryV1=Readonly<{fits:boolean;consumers:readonly ReadyDiskConsumerV1[]}>;
/** 📐️ What a stage needs free, in megabytes and inodes, and where the number comes from: the declared estimate (`diskRequirements`) until the machine has measured the stage, then, once one run succeeded with a measured growth (the cold fill of the images, which counts every writer of the machine and is never a typical need), the upper median of the net growth of free space over the last finished runs (successful or failed) times the safety factor of the workspace scale family (read from the cas budget, not copied), never below the floor and never above `diskGrowth.measuredCeilingMegabytes` (the net growth of free space also counts every other writer of the machine, replica sync, caches of other stages, so a measured need is bounded by what one stage may plausibly write: the one measured run that grew 15.6 GB asked 31 GB of a 121 GB volume and kept a worker from ever starting) and never above `diskGrowth.ceilingPercent` of the volume (a volume of 60 GB never asks for more than a quarter of itself, so a stage can always start once the build-owned caches are collected; the floor of the need wins on a volume that small); the inodes the stage needs never exceed that share of the inode table either. */
export function readyDiskNeedV1(stage:string,runs:readonly ReadyTimingV1[]=[],totalBytes?:number|null,totalInodes?:number|null,derivedInodes?:number|null):Readonly<{megabytes:number;inodes:number;source:"declared"|"measured"}>{
 const declared=(P.diskRequirements as Record<string,{megabytes:number;inodes:number}>)[stage]??P.diskDefaultRequirement;
 const ceiling=typeof totalBytes==="number"&&Number.isFinite(totalBytes)&&totalBytes>0?Math.max(P.diskGrowth.floorMegabytes,Math.floor(totalBytes/1048576*P.diskGrowth.ceilingPercent/100)):Infinity;
 const inodeCeiling=typeof totalInodes==="number"&&Number.isFinite(totalInodes)&&totalInodes>0?Math.max(P.diskDefaultRequirement.inodes,Math.floor(totalInodes*P.diskGrowth.ceilingPercent/100)):Infinity,inodes=Math.min(declared.inodes,inodeCeiling);
 const own=runs.filter(run=>run.stage===stage),derived=typeof derivedInodes==="number"&&Number.isFinite(derivedInodes)&&derivedInodes>0?Math.min(inodeCeiling,Math.max(P.diskDefaultRequirement.inodes,Math.ceil(derivedInodes))):inodes,upper=(key:"growthMegabytes"|"growthInodes"):number|null=>{
  const rows=own.filter(run=>(run.outcome==="ok"||run.outcome==="stage-failed")&&typeof run[key]==="number").slice(-P.diskGrowth.keptRuns).map(run=>run[key] as number).sort((first,second)=>first-second);
  return rows.length===0||!own.some(run=>run.outcome==="ok"&&typeof run[key]==="number")?null:rows[Math.floor(rows.length/2)]!;
 },grownInodes=upper("growthInodes"),needInodes=grownInodes===null?derived:Math.min(inodeCeiling,declared.inodes,Math.max(P.diskDefaultRequirement.inodes,Math.ceil(grownInodes*CAS_RESERVE_FACTOR_V1))),median=upper("growthMegabytes");
 if(median===null)return Object.freeze({megabytes:Math.min(declared.megabytes,ceiling),inodes:needInodes,source:"declared" as const});
 return Object.freeze({megabytes:Math.min(ceiling,P.diskGrowth.measuredCeilingMegabytes,Math.max(P.diskGrowth.floorMegabytes,Math.ceil(median*CAS_RESERVE_FACTOR_V1))),inodes:needInodes,source:"measured" as const});
}

/** 🔎️ The facts of the filesystem that holds `path` (its nearest existing ancestor when it is not there yet), from the one free-space question of the build system (`readFreeSpaceV1`: `statfs` on POSIX, `GetDiskFreeSpaceExW` on Windows, where the runtime's `statfs` answers a row of zeros). A volume that cannot be measured has `freeBytes` `null` and the reason; a filesystem that reports no inode table (NTFS, btrfs, some network shares) has `freeInodes` `null`, never 0. Disk quotas of a user are not visible to `statfs` (only project quotas of a directory are), so a quota shortage still shows as the write error that the stage classifier names.
 * @see ../../../../../../../../🔨️modules/🏃️process/🆓️free-space/🟦️.ts
 * @see https://man7.org/linux/man-pages/man2/statfs.2.html */
export function readyFilesystemFactsV1(path:string,options?:Omit<FreeSpaceOptionsV1,"nearest">):ReadyFilesystemFactsV1{
 const facts=readFreeSpaceV1(path,{nearest:true,...options});
 return facts.known?Object.freeze({freeBytes:facts.availableBytes,freeInodes:facts.freeInodes,...facts.totalBytes===undefined?{}:{totalBytes:facts.totalBytes},...facts.totalInodes===undefined?{}:{totalInodes:facts.totalInodes}}):Object.freeze({freeBytes:null,freeInodes:null,unknown:Object.freeze({reason:facts.reason,detail:facts.detail})});
}

/** 🧮️ The inodes a stage creates before it has ever been measured, derived from its own input: the dependency generations hold about `entriesPerLockfileLine` entries per line of the lockfile of each scope (the root and every top-level folder with a `bun.lock`; calibrated on the six real generations of this repository, 3.20 to 3.52, rounded up), so the cold need is the sum of the lockfile lines times that factor. `null` for any other stage or when no lockfile is readable. A pure function of the workspace folder.
 * @see ./🧪️tests/💽️disk-recovery/🟦️.ts */
export function readyDerivedInodesV1(stage:string,root:string,owners?:readonly string[]|null):number|null{
 const rule=P.diskDerivation;
 if(stage!==rule.stage)return null;
 let lines=0;
 const count=(path:string):void=>{try{const bytes=readFileSync(path);let at=-1;while((at=bytes.indexOf(10,at+1))!==-1)lines++;}catch{}};
 const chosen=owners===undefined||owners===null?undefined:new Set(owners);
 if(chosen===undefined||chosen.has(P.scopeDerivation.rootOwner))count(join(root,rule.lockfile));
 try{for(const entry of readdirSync(root,{withFileTypes:true}))if(entry.isDirectory()&&!entry.name.startsWith(".")&&entry.name!=="node_modules"&&(chosen===undefined||chosen.has(entry.name)))count(join(root,entry.name,rule.lockfile));}catch{}
 return lines===0?null:Math.ceil(lines*rule.entriesPerLockfileLine);
}

/** 🔍️ The owners a stage of the plan was given by `--scope` flags, `null` when it runs every owner. */
export function readyScopesOfStageV1(stages:readonly ReadyStageV1[],name:string):readonly string[]|null{
 const row=stages.find(stage=>stage.name===name),flag=P.scopeDerivation.flag;
 if(row===undefined)return null;
 const owners=row.arguments.flatMap((argument,index)=>argument===flag&&row.arguments[index+1]!==undefined?[row.arguments[index+1]!]:[]);
 return owners.length===0?null:owners;
}
/** 🗂️ The dependency owners of a checkout: the root `.` and every top-level folder that holds its own manifest and lockfile (each one a generation of the dependency store). A pure function of the workspace folder.
 * @see ./🧪️tests/🎯️scopes/🟦️.ts */
export function readyScopeOwnersV1(root:string):readonly string[]{
 const rule=P.scopeDerivation,owners=[rule.rootOwner];
 try{for(const entry of readdirSync(root,{withFileTypes:true}))if(entry.isDirectory()&&!entry.name.startsWith(".")&&entry.name!=="node_modules"&&existsSync(join(root,entry.name,rule.ownerLockfile))&&existsSync(join(root,entry.name,rule.ownerManifest)))owners.push(entry.name);}catch{}
 return Object.freeze(owners.sort());
}
/** 🧭️ The folder of every named project, read from the project files below the checkout (`name` of each `📋️project.json`; build outputs, caches, tickets and hidden folders are not walked): a name nobody declares is missing from the answer, and the walk stops after `maximumEntries` entries. A pure read of the tree.
 * @see ./🧪️tests/🎯️scopes/🟦️.ts */
export function readyProjectRootsV1(root:string,names:readonly string[]):ReadonlyMap<string,string>{
 const rule=P.scopeDerivation,wanted=new Set(names),found=new Map<string,string>(),skip=new Set(rule.skip),stack=[""];let budget=rule.maximumEntries;
 while(stack.length>0&&found.size<wanted.size&&budget>0){
  const at=stack.pop()!;let entries:import("node:fs").Dirent[];
  try{entries=readdirSync(join(root,at),{withFileTypes:true});}catch{continue;}
  for(const entry of entries){
   budget--;
   if(entry.isDirectory()){if(!entry.name.startsWith(".")&&!skip.has(entry.name))stack.push(at===""?entry.name:at+"/"+entry.name);continue;}
   if(entry.name!==rule.projectFile)continue;
   try{const name=(JSON.parse(readFileSync(join(root,at,entry.name),"utf8")) as Record<string,unknown>)[rule.nameKey];if(typeof name==="string"&&wanted.has(name)&&!found.has(name))found.set(name,at);}catch{}
  }
 }
 return found;
}
/** 🎯️ The dependency owners the stages of the plan need: the owner of a project folder is its first folder when that is an owner of the checkout, else the root; `null` (install every owner) when the plan names no projects, a project is not declared anywhere, or the owners are all of them. A pure function of the plan and the folder.
 * @see ./🧪️tests/🎯️scopes/🟦️.ts */
export function readyDependencyScopesV1(root:string,stages:readonly ReadyStageV1[]):readonly string[]|null{
 const rule=P.scopeDerivation,source=stages.find(stage=>stage.name===rule.from);
 if(source===undefined)return null;
 const names=source.arguments.flatMap(argument=>argument.startsWith(rule.projectsPrefix)?argument.slice(rule.projectsPrefix.length).split(",").filter(name=>name!==""):[]);
 if(names.length===0)return null;
 const folders=readyProjectRootsV1(root,names),owners=readyScopeOwnersV1(root);
 if(names.some(name=>!folders.has(name)))return null;
 const needed=new Set<string>([rule.rootOwner]);
 for(const name of names){const first=folders.get(name)!.split("/")[0]!;needed.add(owners.includes(first)?first:rule.rootOwner);}
 return needed.size===owners.length?null:Object.freeze([...needed].sort());
}
/** 🪜️ The stage plan of this checkout: the dependency stage installs only the owners the workspace stage needs (`--scope` once per owner), every other stage and a plan without a derivable owner set stay as declared. Stage digests are over the inputs, not the arguments, so a machine that is ready stays ready.
 * @see ./🧪️tests/🎯️scopes/🟦️.ts */
export function readyStagesForRootV1(stages:readonly ReadyStageV1[],root:string):readonly ReadyStageV1[]{
 const rule=P.scopeDerivation,owners=readyDependencyScopesV1(root,stages);
 if(owners===null)return stages;
 return stages.map(stage=>stage.name===rule.stage?{...stage,arguments:[...stage.arguments.filter(argument=>argument!=="--all"),...owners.flatMap(owner=>[rule.flag,owner])]}:stage);
}

/** 💽️ Whether the paths a stage writes to hold what it needs (`diskRequirements` of the schema, megabytes and inodes): the first resource that is short, `null` when all hold or nothing is known. Checked before the stage starts, so a full disk or an exhausted inode table is a typed refusal that names path, need and what is free, and not the raw database error the stage dies with half an hour later. A pure function of the facts provider.
 * @see ./🧪️tests/🔁️resume/🟦️.ts */
export function readyDiskShortV1(input:Readonly<{stage:string;paths:readonly string[];facts(path:string):ReadyFilesystemFactsV1|null;runs?:readonly ReadyTimingV1[];derivedInodes?:number|null}>):ReadyDiskShortV1|null{
 for(const path of input.paths){
  const facts=input.facts(path);if(facts===null)continue;
  const need=readyDiskNeedV1(input.stage,input.runs,facts.totalBytes,facts.totalInodes,input.derivedInodes);
  const bytes=facts.freeBytes!==null&&Number.isFinite(facts.freeBytes)&&facts.freeBytes>=0?facts.freeBytes:null,inodes=facts.freeInodes!==null&&Number.isFinite(facts.freeInodes)&&facts.freeInodes>=0?facts.freeInodes:null;
  if(bytes!==null&&bytes<need.megabytes*1048576)return Object.freeze({path,resource:"space" as const,needed:need.megabytes,available:Math.floor(bytes/1048576)});
  if(inodes!==null&&inodes<need.inodes)return Object.freeze({path,resource:"inodes" as const,needed:need.inodes,available:inodes});
 }
 return null;
}

/** 🛑️ The volume that crossed the floor of the disk guard: the resource (`space` in megabytes, `inodes`), the path, what is free and the floor. */
export type ReadyDiskGuardHitV1=Readonly<{resource:"space"|"inodes";path:string;available:number;floor:number}>;
/** 🛑️ Whether the volume is nearly full: free bytes below the larger of `diskGuard.floorMegabytes` and `floorBasisPoints` of the volume, or free inodes below the larger of `floorInodes` and `floorInodeBasisPoints` of the table, `null` when it is fine or unknown. The floor lies below every stage need, so a stopped stage is not started again before the collectors made room. A running stage is checked against it every `checkMilliseconds`; a writer of ours therefore cannot take the last bytes of a worker, however long it is stuck.
 * @see ./🧪️tests/🛑️disk-guard/🟦️.ts */
export function readyDiskGuardV1(path:string,facts:ReadyFilesystemFactsV1|null):ReadyDiskGuardHitV1|null{
 if(facts===null)return null;
 const rule=P.diskGuard,total=typeof facts.totalBytes==="number"?facts.totalBytes:0,totalInodes=typeof facts.totalInodes==="number"?facts.totalInodes:0;
 const floorBytes=Math.max(rule.floorMegabytes*1048576,Math.floor(total*rule.floorBasisPoints/10000)),floorInodes=Math.max(rule.floorInodes,Math.floor(totalInodes*rule.floorInodeBasisPoints/10000));
 if(typeof facts.freeBytes==="number"&&facts.freeBytes<floorBytes)return Object.freeze({resource:"space" as const,path,available:Math.floor(facts.freeBytes/1048576),floor:Math.ceil(floorBytes/1048576)});
 if(typeof facts.freeInodes==="number"&&facts.freeInodes<floorInodes)return Object.freeze({resource:"inodes" as const,path,available:facts.freeInodes,floor:floorInodes});
 return null;
}
/** 🗣️ The tail line of a stage the disk guard stopped, English and German, with the volume and its numbers. */
export function readyDiskGuardLineV1(hit:ReadyDiskGuardHitV1):string{
 const unit=hit.resource==="space"?"MB":"inodes";
 return `disk guard stopped the stage: ${hit.path} has ${hit.available} ${unit} free, floor ${hit.floor} ${unit} / Plattenwächter hat die Stufe angehalten: ${hit.path} hat ${hit.available} ${unit} frei, Untergrenze ${hit.floor} ${unit}`;
}
/** 🕳️ The paths a stage writes to whose free bytes could not be measured (the provider answered nothing, or `freeBytes` is `null`, not finite or negative): the stage starts anyway with a warning, because "unknown" is not "full"; the gate of the stage and the write error itself stay. A pure function of the facts provider. */
export function readyDiskUnknownV1(input:Readonly<{paths:readonly string[];facts(path:string):ReadyFilesystemFactsV1|null}>):readonly ReadyDiskUnknownV1[]{
 const rows:ReadyDiskUnknownV1[]=[];
 for(const path of input.paths){
  const facts=input.facts(path);
  if(facts===null)rows.push(Object.freeze({path,reason:"unreadable",detail:"-"}));
  else if(facts.freeBytes===null||!Number.isFinite(facts.freeBytes)||facts.freeBytes<0)rows.push(Object.freeze({path,reason:facts.unknown?.reason??"incoherent-row",detail:facts.unknown?.detail??String(facts.freeBytes)}));
 }
 return Object.freeze(rows);
}

/** 🗣️ The warning of a stage that starts although a path could not be measured: English and German. */
export function readyDiskUnmeasuredV1(stage:string,row:ReadyDiskUnknownV1):ReadyRefusalV1{
 return readyRefusalV1("disk-unmeasured",{stage,path:row.path,reason:row.reason,detail:row.detail});
}

/** 🗣️ The refusal of a stage that lacks disk: English and German with the unit of the resource. */
export function readyDiskRefusalV1(stage:string,short:ReadyDiskShortV1,consumers?:readonly ReadyDiskConsumerV1[]):ReadyRefusalV1{
 const fields={stage,path:short.path,resource:short.resource,unit:short.resource==="space"?"MB":"inodes",needed:short.needed,available:short.available,missing:Math.max(0,short.needed-short.available)};
 return consumers===undefined?readyRefusalV1("disk-short",fields):readyRefusalV1("disk-consumers",{...fields,consumers:consumers.map(row=>`${row.root} (${Math.round(row.bytes/1048576)} MB): \`${row.command}\``).join("; ")||"-"});
}
/** 🧹️ The refusal of a stage whose need is being freed right now by the collectors of the registry. */
export function readyDiskCollectingV1(stage:string,short:ReadyDiskShortV1):ReadyRefusalV1{
 return readyRefusalV1("disk-collecting",{stage,path:short.path,unit:short.resource==="space"?"MB":"inodes",needed:short.needed,available:short.available,missing:Math.max(0,short.needed-short.available)});
}

/** ⛔️ What a worker remembers once a collection proved its disk too small: when it was said, the free megabytes at that time and the largest consumers named then. */
export type ReadyDiskUnfitV1=Readonly<{atMs:number;availableMegabytes:number;consumers:readonly ReadyDiskConsumerV1[]}>;
/** 🩹️ What the repair of a stage's storage before it starts answered: how many images grew in place and, when an image cannot grow because the host volume lacks the free bytes of the larger envelope, the volume and the numbers (the worker is then unfit for the stage). */
export type ReadyHealV1=Readonly<{grown:number;unfit:Readonly<{path:string;neededMegabytes:number;availableMegabytes:number}>|null}>;
/** 🗣️ The refusal of a worker whose disk cannot hold a stage even after every derivable cache was collected: it names the stage, the volume, the numbers, the largest consumers and when the machine is checked again. */
export function readyDiskUnfitRefusalV1(stage:string,short:ReadyDiskShortV1,consumers:readonly ReadyDiskConsumerV1[]):ReadyRefusalV1{
 return readyRefusalV1("disk-unfit",{stage,path:short.path,unit:short.resource==="space"?"MB":"inodes",needed:short.needed,available:short.available,missing:Math.max(0,short.needed-short.available),growth:P.unfitRecheckGrowthMegabytes,hours:Math.round(P.unfitRecheckMilliseconds/3600000),consumers:consumers.map(row=>`${row.root} (${Math.round(row.bytes/1048576)} MB): \`${row.command}\``).join("; ")||"-"});
}
/** 🖼️ The purpose and the folder a failed write names, read from the tail of the failed stage: what the refusal of a full image says about the image. */
export function readyImageFullFieldsV1(tail:readonly string[]):Readonly<{purpose:Readonly<{en:string;de:string}>;path:string}>{
 const partition=readyImagePartitionOfLinesV1(tail,P.noSpacePatterns)??"?";
 return Object.freeze({purpose:readyImagePurposeV1(partition),path:[...READY_IMAGE_POLICY_V1.directory,partition,READY_IMAGE_POLICY_V1.mount].join("/")});
}
/** 🖼️ The tail line that names the full image of a failed stage in both languages, in the words the classifier reads (`No space left on device` below `.cache/b/<partition>/m`), so that the failure stays an image failure when it is read again from the stored tail. */
export function readyImageFullLineV1(partition:string):string{
 const purpose=readyImagePurposeV1(partition),path=["…",...READY_IMAGE_POLICY_V1.directory,partition,READY_IMAGE_POLICY_V1.mount].join("/");
 return `${path}: No space left on device (the ${purpose.en} cache image is full) / Kein Platz mehr auf dem Gerät (das Cache-Image ${purpose.de} ist voll)`;
}
/** 🖼️ The refusal of a stage whose cache image is short after the collection inside it: the purpose, the image, what is free of its size, what is needed, the largest consumers with the command that frees each and how soon it is looked at again; English and German. */
export function readyImageShortRefusalV1(stage:string,short:ReadyDiskShortV1,consumers:readonly ReadyDiskConsumerV1[]=[]):ReadyRefusalV1{
 return readyRefusalV1("image-short",{stage,purpose:readyImagePurposeV1(short.partition??"?"),path:short.path,needed:short.needed,available:short.available,total:short.total??0,host:short.host??"?",hint:short.limitedByHost===true?READY_IMAGE_POLICY_V1.hostLimited:{en:"",de:""},minutes:Math.round(READY_IMAGE_POLICY_V1.recheckMilliseconds/60000),growth:READY_IMAGE_POLICY_V1.recheckGrowthMegabytes,consumers:consumers.map(row=>`${row.root} (${Math.round(row.bytes/1048576)} MB): \`${row.command}\``).join("; ")||"-"});
}
/** 🔁️ Whether an unfit worker is looked at again: the free megabytes grew by `unfitRecheckGrowthMegabytes` since it was said, or `unfitRecheckMilliseconds` passed; nothing else retries a collection that found nothing to collect. A pure function. */
export function readyUnfitRecheckDueV1(unfit:Pick<ReadyDiskUnfitV1,"atMs"|"availableMegabytes">,availableMegabytes:number,nowMs:number):boolean{
 return availableMegabytes-unfit.availableMegabytes>=P.unfitRecheckGrowthMegabytes||nowMs-unfit.atMs>=P.unfitRecheckMilliseconds;
}

/** 🕳️ Whether the lines of a failed stage say that a write found no space left, in a form `storeFullPattern` does not cover (the operating system's own message: `No space left on device`, `os error 28`, `ENOSPC`, the Windows disk-full text; see `noSpacePatterns` of the schema), so that a full image, quota or workspace volume is recognised from any tool that reports it. */
export function readyNoSpaceV1(lines:readonly string[]):boolean{
 return readyStoreFullV1(lines)!==null||lines.some(line=>P.noSpacePatterns.some(pattern=>line.includes(pattern)));
}

/** 🩹️ The tail of a failed stage with the cause named: a database that "unable to open database file" while a path is out of space or inodes is a full disk, so the tail gets the line the classifier knows (`storeFullPattern`) for that path; a bare "no space left" from a tool that does not say where (Nx creating its lock file) gets the line for the native shard that cannot rewrite its file map, followed by the sentence that names it in both languages. */
export function readyTailWithDiskCauseV1(tail:readonly string[],short:ReadyDiskShortV1|null,native:NativeShardConsumerV1|null=null):readonly string[]{
 const named=short!==null&&tail.some(line=>line.includes(P.sqliteOpenPattern))?[...tail,`No space left (ENOSPC) in ${short.path}: the disk`]:tail;
 if(native===null||readyStoreFullV1(named)!==null||!readyNoSpaceV1(named))return named;
 const values={id:native.id.slice(0,12),path:native.path,used:Math.round(native.usedBytes/1048576),total:Math.round(native.totalBytes/1048576),file:native.file,map:Math.round(native.fileMapBytes/1048576),free:Math.round(native.freeBytes/1048576)};
 return[...named,`No space left (ENOSPC) in ${native.path}: the disk`,nativeShardTextV1("consumer","both",values)];
}

/** 🧮️ How many jobs a stage's build tools may run at once: one fewer than the cores, and no more than the free memory pays for at `megabytesPerJob` each, at least one. */
export function readyJobsV1(input:Readonly<{cores:number;availableMegabytes:number|null}>):number{
 const byCores=Math.max(1,Math.floor(input.cores)-1);
 return input.availableMegabytes===null?byCores:Math.max(1,Math.min(byCores,Math.floor(input.availableMegabytes/P.megabytesPerJob)));
}

/** 🌳️ The resident megabytes of a process and all its descendants from `pid,ppid,rss(KB)` rows. */
export function readyTreeMegabytesV1(rows:string,root:number):number{
 const parent=new Map<number,number>(),size=new Map<number,number>();
 for(const line of rows.split("\n")){const [pid,ppid,rss]=line.trim().split(/\s+/u).map(Number);if(Number.isInteger(pid)&&Number.isInteger(ppid)){parent.set(pid!,ppid!);size.set(pid!,Number.isFinite(rss)?rss!:0);}}
 let kilobytes=0;
 for(const [pid,rss] of size){for(let at:number|undefined=pid,depth=0;at!==undefined&&depth<64;at=parent.get(at),depth++)if(at===root){kilobytes+=rss;break;}}
 return Math.round(kilobytes/1024);
}

export type ReadyDecisionInputV1=Readonly<{server:boolean;digest:string;stages:readonly Readonly<{name:string;digest:string}>[];done:Readonly<Record<string,string>>;marker:boolean;repo:boolean;human:boolean;lease:"free"|"own"|"other";inflight:string|null;memoryOk:boolean;failure:Readonly<{digest:string;nextAttemptMs:number;code?:ReadyCodeV1;atMs?:number;tree?:string}>|null;tree?:Readonly<{stamp:string;changedMs:number}>|null;now:number}>;
export type ReadyDecisionV1=Readonly<{action:"server"|"idle"|"mark"|"observe"|"wait"|"backoff"|"run";stageIndex:number;code:ReadyCodeV1|null;waitMilliseconds:number}>;

/** 🧱️ Whether the evidence lines of a failed stage name a source-level cause in the checkout: a missing or refused source file, import or module (the `sourceTransientPatterns` of the schema, `{root}` standing for the checkout root). A peer's half-finished edit looks like this and heals with the next change of the replicated tree, so it is no evidence about this machine.
 * @see ./🧬️schema/🔣️.json */
export function readySourceTransientV1(lines:readonly string[],root:string):boolean{
 const escaped=root.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&"),plain=lines.map(line=>line.replace(/\u001b\[[0-9;]*m/gu,""));
 return P.sourceTransientPatterns.some(pattern=>{if(root===""&&pattern.includes("{root}"))return false;const expression=new RegExp(pattern.replaceAll("{root}",escaped),"u");return plain.some(line=>expression.test(line));});
}

/** 🛑️ Whether a stage ended because another process stopped it, not because it failed: the stage process ended by a stop signal or its exit status, or its own evidence lines say that a stop signal reached the run (`externalStopSignals`, `externalStopExits`, `externalStopPatterns` of the schema). Such an end says nothing about this machine, so it is the `interrupted` class that keeps a Ready worker Ready, never the real evidence `stage-failed`.
 * @see ./🧬️schema/🔣️.json */
export function readyExternallyStoppedV1(result:Readonly<{exit:number|null;signal?:string|null;tail:readonly string[];failure?:readonly string[]}>):boolean{
 if(result.signal!==undefined&&result.signal!==null&&(P.externalStopSignals as readonly string[]).includes(result.signal)||result.exit!==null&&(P.externalStopExits as readonly number[]).includes(result.exit))return true;
 const plain=[...result.failure??[],...result.tail].map(line=>line.replace(/\u001b\[[0-9;]*m/gu,""));
 return P.externalStopPatterns.some(pattern=>{const expression=new RegExp(pattern,"u");return plain.some(line=>expression.test(line));});
}

/** ⏩️ The time the next attempt of a failed stage starts: a `source-transient` failure retries a debounce after the replicated tree changed since the stage started (the tree stamp differs from the one the failure recorded; a change during the run counts from the end of the run), never later than its back-off; any other failure, or an unchanged tree, keeps the back-off.
 * @see ./🧬️schema/🔣️.json */
export function readySourceRetryV1(failure:Readonly<{code?:ReadyCodeV1;atMs?:number;tree?:string;nextAttemptMs:number}>,tree:Readonly<{stamp:string;changedMs:number}>|null):number{
 if(failure.code!=="source-transient"||failure.tree===undefined||tree===null||tree.stamp===failure.tree)return failure.nextAttemptMs;
 return Math.min(failure.nextAttemptMs,Math.max(failure.atMs??0,tree.changedMs)+P.sourceTransientDebounceMilliseconds);
}

/** ⚖️ The whole decision of one turn: the group server never; every stage current with the marker and the `repo` binary there is ready and does nothing; current stages without the marker only write it; without the binary the run starts again at the first stage; a person running the setup or another runner holding the lease is observed; a stage already in flight (`inflight`, started by a former supervisor) is adopted at once: it already holds its memory and its back-off was served before it started, so waiting would only leave it unwatched; a recorded failure of the same digest waits for its back-off (a changed input is a new cause and retries at once); too little free memory waits; otherwise the first stale stage runs. A supervisor restart that is merely deferred holds no stage back: a stage in flight is adopted by the restarted supervisor and the restart waits for it, so waiting for the restart first would only idle the worker. */
export function readyDecideV1(input:ReadyDecisionInputV1):ReadyDecisionV1{
 const act=(action:ReadyDecisionV1["action"],stageIndex:number,code:ReadyCodeV1|null,waitMilliseconds:number):ReadyDecisionV1=>Object.freeze({action,stageIndex,code,waitMilliseconds});
 if(input.server)return act("server",-1,"server",P.idlePollMilliseconds);
 const stale=input.stages.findIndex(stage=>input.done[stage.name]!==stage.digest);
 if(stale<0&&input.repo)return input.marker?act("idle",-1,null,P.idlePollMilliseconds):act("mark",-1,null,0);
 const index=stale<0||!input.repo?0:stale;
 if(input.human)return act("observe",index,"human-running",P.observePollMilliseconds);
 if(input.lease==="other")return act("observe",index,"lease-held",P.observePollMilliseconds);
 if(input.inflight!==null)return act("run",index,null,0);
 if(input.failure!==null&&input.failure.digest===input.digest){const retry=readySourceRetryV1(input.failure,input.tree??null);if(retry>input.now)return act("backoff",index,null,input.failure.code==="source-transient"?Math.min(retry-input.now,P.sourceTransientPollMilliseconds):retry-input.now);}
 if(!input.memoryOk)return act("wait",index,"memory-low",P.memoryPollMilliseconds);
 return act("run",index,null,0);
}

/** 🔄️ The stages a worker that was ready re-verifies in the background while it stays ready: every stage whose own input digest changed, in bootstrap order, `null` when none is stale or the worker is not in a state to stay ready (no marker or `repo` binary, never ready, a recorded failure of real evidence (`stage-failed`, `tooling-compiler`: the stage ran and said no; a timeout, an exhausted stall budget, a full disk, an interruption, a spawn or loop failure says nothing about the machine and keeps Ready, `refreshKeepingFailureCodes`), a stale stage that was never verified here or last verified more than `refreshMaximumAgeMilliseconds` ago: old evidence is no evidence). A pure function.
 * @see ./🧪️tests/🔄️refresh/🟦️.ts */
export function readyRefreshV1(input:Readonly<{stages:readonly Readonly<{name:string;digest:string}>[];done:Readonly<Record<string,string>>;doneMs:Readonly<Record<string,number>>;now:number;marker:boolean;repo:boolean;readyMs:number|null;failure:unknown}>):readonly string[]|null{
 if(!input.marker||!input.repo||input.readyMs===null||input.failure!==null&&!(P.refreshKeepingFailureCodes as readonly string[]).includes((input.failure as {code?:string}).code??""))return null;
 const stale=input.stages.filter(stage=>input.done[stage.name]!==stage.digest);
 if(stale.length===0||stale.some(stage=>input.done[stage.name]===undefined||!(input.now-(input.doneMs[stage.name]??-Infinity)<=P.refreshMaximumAgeMilliseconds)))return null;
 return Object.freeze(stale.map(stage=>stage.name));
}

/** 🔄️ A status patch of a stage that runs while the worker stays ready: working states say `ready` with `refreshing` and the stage, the refusal is dropped and the reason names the refresh; a back-off of a transient failure keeps its refusal and stays ready, and every other state change clears `refreshing` (a back-off of a failure that left Ready is never patched, its stage is `null`). A pure function.
 * @see ./🧪️tests/🔄️refresh/🟦️.ts */
export function readyRefreshingPatchV1(patch:Partial<ReadyStatusV1>,stage:string|null):Partial<ReadyStatusV1>{
 if(patch.state===undefined)return patch;
 if(stage===null||patch.state==="ready"||patch.state==="server")return{...patch,refreshing:false};
 if(patch.state==="backoff")return{...patch,state:"ready",ready:true,refreshing:true,reason:readyReasonV1("refreshing",patch.stage??stage)};
 return{...patch,state:"ready",ready:true,refreshing:true,refusal:null,reason:readyReasonV1("refreshing",patch.stage??stage)};
}

/** 🔒️ The one lease per machine in `<worktree>/.cache/f/ready/lease.json`: created exclusively, renewed by its holder, taken over only when its wall time is older than the stale limit or its process is gone, and the takeover itself goes through an exclusive directory so two takers cannot both win. */
export function createReadyLeaseV1(input:Readonly<{directory:string;owner:string;pid?:number;startMs?:number;now?:()=>number;alive?:(pid:number)=>boolean;startOf?:(pid:number)=>number|null|undefined}>):Readonly<{acquire():"own"|"other";renew():boolean;release():void;peek():"free"|"own"|"other";holder():Readonly<{pid:number;owner:string;ageMilliseconds:number}>|undefined}>{
 const now=input.now??Date.now,pid=input.pid??process.pid,file=join(input.directory,P.leaseName),takeover=join(input.directory,"takeover"),startMs=input.startMs??Math.round(Date.now()-process.uptime()*1000);
 const alive=input.alive??((other:number):boolean=>processPidExistsV1(other));
 const startOf=input.startOf??((other:number)=>readyProcessStartSyncV1(other));
 const read=():{owner:string;pid:number;startMs?:number;wallMs:number}|undefined=>{try{return JSON.parse(readFileSync(file,"utf8"));}catch{return undefined;}};
 const verdict=(lease:{pid:number;startMs?:number;wallMs:number}|undefined)=>{
  if(lease===undefined)return readyLeaseVerdictV1({lease,nowMs:now(),staleMs:P.leaseStaleMilliseconds,process:"gone"});
  const stale=now()-lease.wallMs>P.leaseStaleMilliseconds,running=stale?"alive":alive(lease.pid)?"alive":"gone";
  return readyLeaseVerdictV1({lease,nowMs:now(),staleMs:P.leaseStaleMilliseconds,process:running,processStartMs:running==="alive"&&!stale&&lease.startMs!==undefined?startOf(lease.pid):null});
 };
 const stale=(lease:{pid:number;startMs?:number;wallMs:number}|undefined):boolean=>verdict(lease).state==="free";
 const record=():string=>JSON.stringify({owner:input.owner,pid,startMs,wallMs:now()});
 const create=():boolean=>{try{writeFileSync(file,record(),{flag:"wx"});return true;}catch{return false;}};
 return Object.freeze({
  acquire(){
   mkdirSync(input.directory,{recursive:true});
   if(create())return"own";
   const held=read();
   if(held?.owner===input.owner){this.renew();return"own";}
   if(!stale(held))return"other";
   try{if(now()-statSync(takeover).mtimeMs>30000)rmSync(takeover,{recursive:true,force:true});}catch{}
   try{mkdirSync(takeover);}catch{return"other";}
   try{
    if(!stale(read()))return"other";
    rmSync(file,{force:true});
    return create()?"own":"other";
   }finally{rmSync(takeover,{recursive:true,force:true});}
  },
  renew(){if(read()?.owner!==input.owner)return false;try{const temporary=`${file}.${pid}.tmp`;writeFileSync(temporary,record());renameSync(temporary,file);return true;}catch{return false;}},
  release(){if(read()?.owner===input.owner)rmSync(file,{force:true});},
  peek(){const held=read();return stale(held)?"free":held!.owner===input.owner?"own":"other";},
  holder(){const held=read();return held===undefined||stale(held)?undefined:{pid:held.pid,owner:held.owner,ageMilliseconds:now()-held.wallMs};},
 });
}

/** 🧑️ The evidence of one turn about a person's setup: how long ago `human.json` was touched, the `{pid,startedMs}` the person's wrapper wrote into it (`null` for the empty file of an older wrapper), whether that process lives, whether an older wrapper's command is in the process list, and the file age by its creation time. */
export type ReadyHumanEvidenceV1=Readonly<{now:number;touchedAgeMilliseconds:number;birthAgeMilliseconds:number;record:Readonly<{pid:number;startedMs:number}>|null;pidAlive:boolean;legacyProcess:boolean}>;

/** 🧑️ Whether a person is running the setup, decided on evidence only: the marker must be touched within the fresh limit AND its owner must still be there (the recorded process lives and started less than the maximum ago; an empty marker of an older wrapper counts only while that wrapper's command is in the process list), so a marker left by a dead or hung process expires by itself instead of blocking the machine. */
export function readyHumanVerdictV1(evidence:ReadyHumanEvidenceV1):boolean{
 if(evidence.touchedAgeMilliseconds>=P.humanFreshMilliseconds)return false;
 if(evidence.record===null)return evidence.legacyProcess&&evidence.birthAgeMilliseconds<P.humanMaximumMilliseconds;
 return evidence.pidAlive&&evidence.now-evidence.record.startedMs<P.humanMaximumMilliseconds;
}

/** 🧾️ The `{pid,startedMs}` of a `human.json` written by the wrapper of a person (schema `Human`), `null` when the text is empty or not that record. */
export function readyHumanRecordV1(text:string):Readonly<{pid:number;startedMs:number}>|null{
 try{const row=JSON.parse(text) as {version?:unknown;pid?:unknown;startedMs?:unknown};return row.version===1&&Number.isSafeInteger(row.pid)&&(row.pid as number)>0&&Number.isSafeInteger(row.startedMs)&&(row.startedMs as number)>0?{pid:row.pid as number,startedMs:row.startedMs as number}:null;}catch{return null;}
}

/** 🧑️ Reads the evidence from `<directory>/human.json` and the process table (`ps` on Linux and macOS; an empty marker has no owner to find on Windows, so it never counts there) and decides with `readyHumanVerdictV1`. */
export function readyHumanActiveV1(directory:string,now:number=Date.now(),probe:Readonly<{alive?(pid:number):boolean;commands?():readonly string[]}>={}):boolean{
 try{
  const file=join(directory,P.humanName),stat=statSync(file),record=readyHumanRecordV1(readFileSync(file,"utf8"));
  const commands=probe.commands??(():readonly string[]=>{if(process.platform==="win32")return[];const rows=spawnSync("ps",["-A","-o","args="],{encoding:"utf8"});return rows.status===0?rows.stdout.split("\n"):[];});
  return readyHumanVerdictV1({now,touchedAgeMilliseconds:now-stat.mtimeMs,birthAgeMilliseconds:now-stat.birthtimeMs,record,pidAlive:record!==null&&(probe.alive??alive)(record.pid),legacyProcess:record===null&&now-stat.mtimeMs<P.humanFreshMilliseconds&&commands().some(line=>line.includes(P.humanLegacyCommand))});
 }catch{return false;}
}

export const emptyReadyStateV1=():ReadyLocalStateV1=>({version:1,stages:{},failure:null,attempts:0,peakMegabytes:null,readyMs:null,adopted:false});

/** 📊️ Publishes validated Ready receipts on state changes and coalesces ordinary updates. Explicit heartbeats renew running or retry evidence at the slower heartbeat interval. */
export function createReadyStatusV1(input:Readonly<{store:string|undefined;origin:string;platform?:ReadyPlatformV1;now?:()=>number;rename?:(from:string,to:string)=>void}>):Readonly<{update(patch:Partial<ReadyStatusV1>):void;heartbeat():void;current():ReadyStatusV1}>{
 const now=input.now??Date.now,startedMs=now();
 let doc:ReadyStatusV1={version:1,origin:input.origin,host:hostname(),platform:input.platform??readyPlatformV1(),pid:process.pid,state:"waiting",digest:ZERO,ready:false,stage:null,stageIndex:0,stageCount:READY_STAGES_V1.length,attempts:0,exit:null,refusal:null,reason:null,peakMegabytes:null,tail:[],nextAttemptMs:null,readyMs:null,startedMs,updatedMs:startedMs},written=0,timer:ReturnType<typeof setTimeout>|undefined;
 const write=():void=>{
  if(timer!==undefined)clearTimeout(timer);
  timer=undefined;written=now();
  if(input.store===undefined||input.store==="")return;
  try{
   const out={...doc,updatedMs:now(),beatMilliseconds:P.beatByState[doc.refreshing===true?"running":doc.state]};
   if(validateJsonSchemaSubset(READY_STATUS_SCHEMA_V1,out,schema).length)return;
   const directory=join(input.store,"v1","origins",input.origin),file=join(directory,P.statusName),temporary=`${file}.${process.pid}.tmp`;
   const text=JSON.stringify(out);mkdirSync(directory,{recursive:true});writeFileSync(temporary,text);try{(input.rename??renameSync)(temporary,file);}catch{try{writeFileSync(file,text);}catch{}try{rmSync(temporary,{force:true});}catch{}}
  }catch{}
 };
 return Object.freeze({
  update(patch){
   const changed=patch.state!==undefined&&patch.state!==doc.state||patch.stage!==undefined&&patch.stage!==doc.stage||patch.ready!==undefined&&patch.ready!==doc.ready||patch.refreshing!==undefined&&patch.refreshing!==doc.refreshing;
   doc={...doc,...patch};
   if(changed||now()-written>=P.statusIntervalMilliseconds)write();
   else if(timer===undefined){timer=setTimeout(write,P.statusIntervalMilliseconds);timer.unref?.();}
  },
  heartbeat(){if((doc.refreshing===true||!doc.ready)&&(doc.refreshing===true||doc.state==="running"||doc.state==="backoff"||doc.state==="waiting"&&(doc.refusal?.code==="disk-collecting"||doc.refusal?.code==="disk-unfit"))&&now()-written>=P.statusHeartbeatMilliseconds)write();},
  current:()=>doc,
 });
}

type ReadyEndEvidenceV1="exit-receipt"|"observed";
export type ReadyRunResultV1=Readonly<{watchdog?:ReadyTimingWatchdogV1;stage?:string;physicalEnd?:Readonly<{endedMs:number;endEvidence:ReadyEndEvidenceV1;digest:string|null}>;attribution?:"applied"|"known"|"unproven";containment?:ReadyContainmentResultV1;wardenCancel?:Pick<HostWardenCancelV1,"reason"|"kind"|"pid"|"megabytes">;sender?:string;stopBy?:Readonly<{supervisor:number;reason:ReadyStageStopV1;atMs:number}>;diskGuard?:ReadyDiskGuardHitV1;signal?:string|null;stepOverdue?:Readonly<ReadyStepV1&{limit:number;quiet:number}>;holderSilent?:Readonly<{pid:number;command:string;seconds:number}>;exit:number|null;timedOut:boolean;spawnFailed:boolean;cancelled:boolean;stalled:boolean;progressed?:boolean;codeChanged?:string;codeStale?:boolean;codeStaleWaiting?:boolean;detached:boolean;startedMs:number;peakMegabytes:number|null;tail:readonly string[];failure?:readonly string[]}>;
type ReadyLogMirrorV1=(stage:string,lines:readonly string[],owner?:string)=>void;
export type ReadyPortsV1=Readonly<{
 server:boolean;stages?:readonly ReadyStageV1[];now():number;sleep(milliseconds:number,signal:AbortSignal):Promise<void>;jitter:number;cores:number;
 digests():Promise<Readonly<{digest:string;stages:readonly Readonly<{name:string;digest:string}>[]}>>;
 state:Readonly<{read():ReadyLocalStateV1;write(state:ReadyLocalStateV1):void}>;
 marker:Readonly<{exists():boolean;write():void}>;repoPresent():boolean;human():boolean;
 lease:ReturnType<typeof createReadyLeaseV1>;inflight?():string|null|undefined;units?():Readonly<Record<UnitCountKindV1,number>>|null;
 memory():Readonly<{availableMegabytes:number|null;ok:boolean;floorMegabytes?:number}>;disk?(stage:string):ReadyDiskShortV1|null;diskUnknown?(stage:string):readonly ReadyDiskUnknownV1[];low?(stage:string):ReadyDiskShortV1|null;recover?(request:Readonly<{stage:string;short:ReadyDiskShortV1;relief?:boolean;signal:AbortSignal;report(line:string):void}>):Promise<ReadyDiskRecoveryV1>;sweep?(request:Readonly<{stage:string;neededMegabytes:number;scarce?:"inodes";signal:AbortSignal;report(line:string):void}>):Promise<ReadyDiskRecoveryV1>;native?():NativeShardConsumerV1|null;relieve?(request:Readonly<{signal:AbortSignal;report(line:string):void}>):Promise<number>;freeMegabytes?():number|null;freeInodes?():number|null;compiler?():Readonly<{en:string;de:string}>|null;discardDependencies?():Promise<Readonly<{receipts:number;generations:number}>>;
 run(stage:ReadyStageV1,options:Readonly<{signal:AbortSignal;jobs:number;onLine(line:string):void;onStage?(stage:string,adopted:boolean):void;onActivity?():void;mirror?:ReadyLogMirrorV1;collectImage?:(image:ReadyImageShortV1,signal:AbortSignal)=>Promise<void>}>):Promise<ReadyRunResultV1>;
 heal?(request:Readonly<{stage:string;signal:AbortSignal;report(line:string):void}>):Promise<ReadyHealV1>;
 tree?():string|null;root?:string;
 status:Readonly<{update(patch:Partial<ReadyStatusV1>):void;heartbeat?():void;current?():ReadyStatusV1}>;
 mirror?:ReadyLogMirrorV1;timing?(run:ReadyTimingV1):void;
 say(line:string):void;busy?(busy:boolean):void;
}>;

/** 🔁️ The readiness loop that does not die: it runs the loop (`runReadyLoopV1`) and, when it throws (a store that cannot be written, a port that fails), says so in `ready.json` (refusal `loop-failed`, state `backoff`), waits the back-off of the failures in a row (they start again from one after a run that lasted `loopStableMilliseconds`) and starts the loop again, for as long as the signal does not abort. The state of the stages is on disk, so nothing is lost by the restart.
 * @see ./🧪️tests/🔁️resume/🟦️.ts */
export async function runReadyV1(ports:ReadyPortsV1,signal:AbortSignal):Promise<void>{
 let failures=0;
 while(!signal.aborted){
  const started=ports.now();
  try{await runReadyLoopV1(ports,signal);return;}
  catch(error){
   if(signal.aborted)return;
   failures=ports.now()-started>=P.loopStableMilliseconds?1:failures+1;
   const pause=readyBackoffV1(failures,ports.jitter),nextAttemptMs=ports.now()+pause,message=String((error as Error|undefined)?.message??error).replace(/\s+/gu," ").slice(0,P.tailCharacters),seconds=Math.round(pause/1000);
   try{ports.status.update({state:"backoff",ready:false,refusal:readyRefusalV1("loop-failed",{message,seconds,count:failures}),nextAttemptMs});}catch{}
   try{ports.say(`readiness loop failed (${message}), it starts again in ${seconds} s (try ${failures} in a row) / Bereitschaftsschleife fehlgeschlagen (${message}), sie startet in ${seconds} s neu (Versuch ${failures} in Folge)`);}catch{}
   while(!signal.aborted){
    const remaining=nextAttemptMs-ports.now();
    if(remaining<=0)break;
    await ports.sleep(Math.min(remaining,P.statusHeartbeatMilliseconds),signal);
    if(!signal.aborted&&ports.now()<nextAttemptMs)ports.status.heartbeat?.();
   }
  }
 }
}

/** 🔁️ The loop of one worker, turn by turn: reads the digest, decides (`readyDecideV1`), runs the first stale stage under the lease with cancellation (the signal stops the child and records nothing as failed), records the result in the local state and in `ready.json`, and writes the marker `.cache/f/native-bootstrap.done` after the last stage. A machine whose marker and `repo` binary exist but that never ran a stage here adopts the marker as the current digest, so a machine a person made ready is not set up again. The group server returns at once and writes nothing. */
async function runReadyLoopV1(ports:ReadyPortsV1,signal:AbortSignal):Promise<void>{
 if(ports.server)return;
 const plan=ports.stages??READY_STAGES_V1,names=plan.map(stage=>stage.name);
 const stalls=new Map<string,number>(),warnedUnmeasured=new Set<string>(),swept=new Set<string>(),inodeSweptAt=new Map<string,number>(),discarded=new Set<string>();
 const reliefAt=new Map<string,number>(),unfit=new Map<string,ReadyDiskUnfitV1>(),unfitSaid=new Set<string>(),healedAt=new Map<string,number>(),healUnfit=new Map<string,Readonly<{atMs:number;availableMegabytes:number;short:ReadyDiskShortV1}>>();
 const discardDependencies=async(digest:string):Promise<void>=>{if(discarded.has(digest)||ports.discardDependencies===undefined)return;discarded.add(digest);try{const gone=await ports.discardDependencies();ports.say(`the dependency store verified current but lacks the tooling compiler: ${gone.receipts} synchronization receipt(s) and ${gone.generations} ready marker(s) discarded, the next dependencies stage rebuilds them / der Abhängigkeitsspeicher galt als aktuell, es fehlt aber der Werkzeug-Compiler: ${gone.receipts} Synchronisationsbeleg(e) und ${gone.generations} Fertigmarkierung(en) verworfen, die nächste Stufe dependencies baut sie neu`);}catch{discarded.delete(digest);}};
 let memoryLowSince:number|undefined,memoryWaitSaid=false,refreshStage:string|null=null,treeSeen=null as Readonly<{stamp:string;changedMs:number}>|null;
 const status={update:(patch:Partial<ReadyStatusV1>):void=>{ports.status.update(readyRefreshingPatchV1(patch,refreshStage));}};
 let renew:ReturnType<typeof setInterval>|undefined,stalledStage:string|undefined,stalledReason:ReadyReasonCodeV1="stalled",lastContainment:ReadyContainmentV1|null=null;
 const leave=():void=>{if(renew!==undefined)clearInterval(renew);renew=undefined;ports.lease.release();};
 try{
  while(!signal.aborted){
   const {digest,stages}=await ports.digests(),stored=ports.state.read(),marker=ports.marker.exists(),repo=ports.repoPresent(),now=ports.now(),stamp=ports.tree?.()??null;
   if(stamp!==null&&treeSeen?.stamp!==stamp)treeSeen={stamp,changedMs:now};
   let state=stored,done:Record<string,string>=Object.fromEntries(Object.entries(stored.stages).map(([name,row])=>[name,row.digest]));
   if(Object.keys(done).length===0&&marker&&repo){
    state={...stored,adopted:true,stages:Object.fromEntries(stages.map(stage=>[stage.name,{digest:stage.digest,doneMs:now}])),readyMs:now};
    ports.state.write(state);done=Object.fromEntries(stages.map(stage=>[stage.name,stage.digest]));
   }
   if(done.dependencies!==undefined){const wrong=ports.compiler?.()??null;if(wrong!==null){const at=ports.now(),attempts=state.attempts+1,{dependencies:_dropped,...rest}=state.stages;state={...state,stages:rest,failure:{digest,stage:"dependencies",attempts,exit:null,code:"tooling-compiler",nextAttemptMs:at+readyBackoffV1(attempts,ports.jitter),atMs:at,tail:[wrong.en,wrong.de]},attempts};ports.state.write(state);delete done.dependencies;await discardDependencies(digest);}}
   refreshStage=readyRefreshV1({stages,done,doneMs:Object.fromEntries(Object.entries(state.stages).map(([name,row])=>[name,row.doneMs])),now,marker,repo,readyMs:state.readyMs,failure:state.failure})?.[0]??null;
   const memory=ports.memory(),lease=ports.lease.peek();
   if(memory.ok)memoryLowSince=undefined;else memoryLowSince??=ports.now();
   const memoryWaitedOut=!memory.ok&&ports.now()-memoryLowSince!>=P.memoryWaitMaximumMilliseconds;
   if(memoryWaitedOut&&!memoryWaitSaid){memoryWaitSaid=true;ports.say(`free memory stayed under ${memory.floorMegabytes??P.memoryFloorMegabytes} MB for ${Math.round(P.memoryWaitMaximumMilliseconds/60000)} min: the stage starts anyway and its own capacity gate sizes, shrinks or refuses its grant / freier Speicher blieb ${Math.round(P.memoryWaitMaximumMilliseconds/60000)} min unter ${memory.floorMegabytes??P.memoryFloorMegabytes} MB: die Stufe startet trotzdem, ihr eigenes Kapazitätstor bemisst, verkleinert oder verweigert ihre Zuteilung`);}
   if(memory.ok)memoryWaitSaid=false;
   const decision=readyDecideV1({server:false,digest,stages,done,marker,repo,human:ports.human(),lease,inflight:ports.inflight?.()??null,memoryOk:memory.ok||memoryWaitedOut,failure:state.failure===null?null:{digest:state.failure.digest,nextAttemptMs:state.failure.nextAttemptMs,code:state.failure.code,atMs:state.failure.atMs,...state.failure.tree===undefined?{}:{tree:state.failure.tree}},tree:treeSeen,now});
   const base:Partial<ReadyStatusV1>={digest,attempts:state.attempts,peakMegabytes:state.peakMegabytes,readyMs:state.readyMs,stageCount:names.length,exit:state.failure?.exit??null,tail:[...state.failure?.tail??[]],containment:lastContainment};
   const full=state.failure===null?null:readyStoreFullV1(state.failure.tail),failure=state.failure!==null&&state.failure.digest===digest?readyRefusalV1(state.failure.code,{stage:state.failure.stage,ending:readyEndingV1(state.failure.exit,state.failure.signal),path:full?.path??"-",errno:full?.errno??"-",count:READY_TREE_POLICY_V1.stallRestartMaximum,seconds:Math.round(P.sourceTransientDebounceMilliseconds/1000),...state.failure.code==="image-full"?readyImageFullFieldsV1(state.failure.tail):{}}):null;
   const stage=decision.stageIndex>=0?names[decision.stageIndex]!:null;
   if(decision.action==="idle"){leave();lastContainment=null;status.update({...base,state:"ready",ready:true,stage:null,stageIndex:0,refusal:null,reason:null,containment:null,nextAttemptMs:null,exit:null,tail:[]});}
   else if(decision.action==="mark"){ports.marker.write();continue;}
   else if(decision.action==="observe"||decision.action==="wait"){
    leave();
    const refusal=decision.code===null?failure:readyRefusalV1(decision.code,{stage:stage??"-",available:memory.availableMegabytes??"?",needed:memory.floorMegabytes??P.memoryFloorMegabytes});
    status.update({...base,state:decision.action==="observe"?"observing":"waiting",ready:false,stage,stageIndex:decision.stageIndex+1,refusal,nextAttemptMs:null});
   }
   else if(decision.action==="backoff"){
    leave();
    status.update({...base,state:"backoff",ready:false,stage,stageIndex:decision.stageIndex+1,refusal:failure,nextAttemptMs:state.failure!.nextAttemptMs});
   }
   else if(decision.action==="run"){
    const diskStage=plan[decision.stageIndex]!.name;
    if(ports.heal!==undefined&&ports.inflight?.()!==diskStage){
     const parked=healUnfit.get(diskStage),free=ports.freeMegabytes?.()??null;
     if(parked!==undefined&&(free!==null&&free-parked.availableMegabytes>=P.unfitRecheckGrowthMegabytes||ports.now()-parked.atMs>=P.unfitRecheckMilliseconds)){healUnfit.delete(diskStage);unfitSaid.delete("heal:"+diskStage);healedAt.delete(diskStage);}
     if(!healUnfit.has(diskStage)&&ports.now()-(healedAt.get(diskStage)??-Infinity)>=P.healRetryMilliseconds){
      healedAt.set(diskStage,ports.now());leave();
      status.update({...base,state:"waiting",ready:false,stage,stageIndex:decision.stageIndex+1,refusal:null,reason:null,tail:[],nextAttemptMs:null});
      const beat=setInterval(()=>{if(!signal.aborted)try{ports.status.heartbeat?.();}catch{}},P.statusHeartbeatMilliseconds);beat.unref?.();
      try{
       const healed=await ports.heal({stage:diskStage,signal,report:line=>{if(signal.aborted)return;ports.say(line);const progress=formatLogLineV1(line,P.tailCharacters);if(progress!=="")try{status.update({tail:[progress]});}catch{}}});
       if(signal.aborted)break;
       if(healed.unfit!==null)healUnfit.set(diskStage,{atMs:ports.now(),availableMegabytes:healed.unfit.availableMegabytes,short:Object.freeze({path:healed.unfit.path,resource:"space" as const,needed:healed.unfit.neededMegabytes,available:healed.unfit.availableMegabytes})});
      }catch(error){if(signal.aborted)break;ports.say(`repairing the storage for stage ${diskStage} failed (${String((error as Error)?.message??error).slice(0,200)}) / Speicher für Stufe ${diskStage} reparieren fehlgeschlagen`);}
      finally{clearInterval(beat);}
     }
     const parkedNow=healUnfit.get(diskStage);
     if(parkedNow!==undefined){
      const refusal=readyDiskUnfitRefusalV1(diskStage,parkedNow.short,[]);
      status.update({...base,state:"waiting",ready:false,stage,stageIndex:decision.stageIndex+1,refusal,tail:[],nextAttemptMs:ports.now()+P.memoryPollMilliseconds});
      if(!unfitSaid.has("heal:"+diskStage)){unfitSaid.add("heal:"+diskStage);ports.say(`${refusal.en} / ${refusal.de}`);}
      await ports.sleep(P.memoryPollMilliseconds,signal);
      continue;
     }
    }
    let short=ports.inflight?.()===diskStage?null:ports.disk?.(diskStage)??null,remaining:readonly ReadyDiskConsumerV1[]|undefined;
    if(ports.inflight?.()!==diskStage)for(const row of ports.diskUnknown?.(diskStage)??[]){const key=`${diskStage}\0${row.path}\0${row.reason}`;if(!warnedUnmeasured.has(key)){warnedUnmeasured.add(key);const warning=readyDiskUnmeasuredV1(diskStage,row);ports.say(`${warning.en} / ${warning.de}`);}}
    if(short===null){unfit.delete(diskStage);unfitSaid.delete(diskStage);}
    else if(unfit.has(diskStage)&&(short.partition===undefined?readyUnfitRecheckDueV1(unfit.get(diskStage)!,short.available,ports.now()):readyImageRecheckDueV1(unfit.get(diskStage)!,short.available,ports.now()))){unfit.delete(diskStage);unfitSaid.delete(diskStage);}
    if(short===null&&ports.low!==undefined&&ports.recover!==undefined&&ports.inflight?.()!==diskStage&&ports.now()-(reliefAt.get(diskStage)??-Infinity)>=READY_IMAGE_POLICY_V1.reliefRetryMilliseconds){
     const low=ports.low(diskStage);
     if(low!==null){
      reliefAt.set(diskStage,ports.now());leave();
      ports.say(`image ${low.path} has ${low.available} MB free of ${low.total??0} MB, below its low-water mark of ${low.needed} MB: collecting inside it before stage ${diskStage} starts / Image ${low.path} hat ${low.available} MB frei von ${low.total??0} MB, unter der Untergrenze von ${low.needed} MB: vor Stufe ${diskStage} wird darin gesammelt`);
      status.update({...base,state:"waiting",ready:false,stage,stageIndex:decision.stageIndex+1,refusal:readyDiskCollectingV1(diskStage,low),tail:[],nextAttemptMs:null});
      let relieving=true;
      const reliefBeat=setInterval(()=>{if(relieving&&!signal.aborted)try{ports.status.heartbeat?.();}catch{}},P.statusHeartbeatMilliseconds);reliefBeat.unref?.();
      try{await ports.recover({stage:diskStage,short:low,relief:true,signal,report:line=>{if(!relieving||signal.aborted)return;ports.say(line);const progress=formatLogLineV1(line,P.tailCharacters);if(progress!=="")try{status.update({tail:[progress]});}catch{}}});}
      catch(error){if(signal.aborted)break;ports.say(`collecting inside the image of stage ${diskStage} failed (${String((error as Error)?.message??error).slice(0,200)}) / Sammeln im Image der Stufe ${diskStage} fehlgeschlagen`);}
      finally{relieving=false;clearInterval(reliefBeat);}
      if(signal.aborted)break;
      short=ports.disk?.(diskStage)??null;
     }
    }
    if(short!==null&&short.resource==="space"&&ports.recover!==undefined&&!unfit.has(diskStage)){
     leave();
     status.update({...base,state:"waiting",ready:false,stage,stageIndex:decision.stageIndex+1,refusal:readyDiskCollectingV1(diskStage,short),tail:[],nextAttemptMs:null});
     let recovering=true;
     const heartbeat=setInterval(()=>{if(recovering&&!signal.aborted)try{ports.status.heartbeat?.();}catch{}},P.statusHeartbeatMilliseconds);
     heartbeat.unref?.();
     try{
      const outcome=await ports.recover({stage:diskStage,short,signal,report:line=>{
       if(!recovering||signal.aborted)return;
       ports.say(line);
       const progress=formatLogLineV1(line,P.tailCharacters);
       if(progress!=="")try{status.update({tail:[progress]});}catch{}
      }});
      if(signal.aborted)break;
      if(outcome.fits)short=ports.disk?.(diskStage)??null;else{remaining=outcome.consumers;unfit.set(diskStage,Object.freeze({atMs:ports.now(),availableMegabytes:short.available,consumers:outcome.consumers}));}
     }catch(error){
      if(signal.aborted)break;
      remaining=[];ports.say(`collecting space for stage ${diskStage} failed (${String((error as Error)?.message??error).slice(0,200)}) / Platz für Stufe ${diskStage} sammeln fehlgeschlagen`);
     }finally{recovering=false;clearInterval(heartbeat);}
    }
    if(short===null)inodeSweptAt.delete(diskStage);
    else if(short.resource==="inodes"&&ports.sweep!==undefined&&ports.now()-(inodeSweptAt.get(diskStage)??-Infinity)>=P.inodeSweepRetryMilliseconds){
     inodeSweptAt.set(diskStage,ports.now());leave();
     ports.say(`stage ${diskStage} lacks inodes in ${short.path} (${short.available} free, ${short.needed} needed): collecting every build-owned cache once / Stufe ${diskStage} hat zu wenige Inodes in ${short.path} (${short.available} frei, ${short.needed} nötig): alle vom Build besessenen Caches werden einmal bereinigt`);
     status.update({...base,state:"waiting",ready:false,stage,stageIndex:decision.stageIndex+1,refusal:readyDiskCollectingV1(diskStage,short),tail:[],nextAttemptMs:null});
     let sweeping=true;
     const beat=setInterval(()=>{if(sweeping&&!signal.aborted)try{ports.status.heartbeat?.();}catch{}},P.statusHeartbeatMilliseconds);
     beat.unref?.();
     try{await ports.sweep({stage:diskStage,neededMegabytes:0,scarce:"inodes",signal,report:line=>{if(sweeping&&!signal.aborted){ports.say(line);const progress=formatLogLineV1(line,P.tailCharacters);if(progress!=="")try{status.update({tail:[progress]});}catch{}}}});}
     catch(error){if(signal.aborted)break;ports.say(`collecting inodes for stage ${diskStage} failed (${String((error as Error)?.message??error).slice(0,200)}) / Inodes für Stufe ${diskStage} sammeln fehlgeschlagen`);}
     finally{sweeping=false;clearInterval(beat);}
     short=ports.disk?.(diskStage)??null;
    }
    if(short!==null){
     leave();
     const marked=unfit.get(diskStage),refusal=short.partition!==undefined?readyImageShortRefusalV1(diskStage,short,marked?.consumers??remaining??[]):marked!==undefined?readyDiskUnfitRefusalV1(diskStage,short,marked.consumers):readyDiskRefusalV1(diskStage,short,remaining);
     status.update({...base,state:"waiting",ready:false,stage,stageIndex:decision.stageIndex+1,refusal,nextAttemptMs:ports.now()+P.memoryPollMilliseconds});
     if(marked===undefined||!unfitSaid.has(diskStage)){if(marked!==undefined)unfitSaid.add(diskStage);ports.say(`${refusal.en} / ${refusal.de}`);}
     await ports.sleep(P.memoryPollMilliseconds,signal);
     continue;
    }
    if(ports.lease.acquire()==="other"){status.update({...base,state:"observing",ready:false,stage,stageIndex:decision.stageIndex+1,refusal:readyRefusalV1("lease-held",{holder:ports.lease.holder?.()?.pid??"?"}),nextAttemptMs:null});await ports.sleep(P.observePollMilliseconds,signal);continue;}
    renew??=setInterval(()=>{ports.lease.renew();},P.leaseRenewMilliseconds);renew.unref?.();
    const row=plan[decision.stageIndex]!,jobs=readyJobsV1({cores:ports.cores,availableMegabytes:memory.availableMegabytes});
    const requestedReason=stalledStage===row.name?readyReasonV1(stalledReason,row.name):state.failure!==null&&state.failure.stage===row.name?readyReasonV1("previous-failed",row.name):!repo?readyReasonV1("binary-missing",row.name):stored.stages[row.name]!==undefined?readyReasonV1("inputs-changed",row.name):readyReasonV1("first-run",row.name);
    const startTree=ports.tree?.()??null,inflightStage=ports.inflight?.();
    let physicalStage=inflightStage??row.name,adopted=inflightStage!==undefined;
    const reason=adopted?readyReasonV1("adopted",physicalStage):requestedReason;
    status.update({...base,state:"running",ready:false,stage:physicalStage,stageIndex:names.indexOf(physicalStage)+1,exit:null,tail:[],refusal:null,reason,containment:null,nextAttemptMs:null});
    const announceStart=(name:string)=>ports.say(`stage ${name} starts with ${jobs} job(s) / Stufe ${name} startet mit ${jobs} Job(s)`);
    if(adopted)ports.say(`${reason.en} / ${reason.de}`);else announceStart(physicalStage);
    const onStage=(name:string,isAdopted:boolean)=>{
     if(name===physicalStage&&isAdopted===adopted)return;
     physicalStage=name;adopted=isAdopted;
     const why=isAdopted?readyReasonV1("adopted",name):requestedReason;
     status.update({stage:name,stageIndex:names.indexOf(name)+1,reason:why,tail:[]});
     if(isAdopted)ports.say(`${why.en} / ${why.de}`);else announceStart(name);
    };
    ports.busy?.(true);stalledStage=undefined;
    const freeBefore=ports.inflight?.()!==undefined?null:ports.freeMegabytes?.()??null,inodesBefore=ports.inflight?.()!==undefined?null:ports.freeInodes?.()??null,unitsBefore=ports.inflight?.()!==undefined?null:ports.units?.()??null;
    if(inflightStage===undefined&&ports.relieve!==undefined)try{await ports.relieve({signal,report:line=>{if(!signal.aborted)ports.say(line);}});}catch{}
    let result:ReadyRunResultV1,watching=true;
    const onActivity=()=>{if(watching&&!signal.aborted)ports.status.heartbeat?.();};
    try{result=await ports.run(row,{signal,jobs,onStage,onActivity,onLine:line=>{status.update({tail:[line.slice(0,P.tailCharacters)]});},...ports.recover===undefined?{}:{collectImage:async(image,collectSignal)=>{await ports.recover!({stage:row.name,short:image,relief:true,signal:collectSignal,report:()=>{}});}},...ports.mirror===undefined?{}:{mirror:(...args)=>ports.mirror!(...args)}});}
    finally{watching=false;ports.busy?.(false);}
    if(result.stage!==undefined&&result.stage!==row.name&&!result.detached&&!signal.aborted){
      lastContainment=result.containment===undefined?null:readyContainmentStatusV1({stage:result.stage,...result.containment});
      const {endedMs,endEvidence,digest}=result.physicalEnd??{endedMs:ports.now(),endEvidence:"observed" as const,digest:null};
      ports.timing?.({stage:result.stage,startedMs:result.startedMs,endedMs,seconds:Math.max(0,Math.round((endedMs-result.startedMs)/1000)),
       exit:result.exit,signal:result.signal??null,outcome:readyTerminalOutcomeV1(result),endEvidence,peakMegabytes:result.peakMegabytes,growthMegabytes:null,attempts:null,digest,...readyTimingSenderV1(result),...timingWatchdogV1(result.watchdog)});
      continue;
     }
     if(!result.detached&&!signal.aborted)lastContainment=result.containment===undefined?null:readyContainmentStatusV1({stage:row.name,...result.containment});
    const peak=Math.max(state.peakMegabytes??0,result.peakMegabytes??0)||state.peakMegabytes;
    const wrong=row.name==="dependencies"&&readyCleanExitV1(result)?ports.compiler?.()??null:null;
    const stalledOut=result.stalled&&!result.timedOut&&!result.cancelled&&!signal.aborted,outcome=readyTerminalOutcomeV1(result,readyCleanExitV1(result)&&!(row.name==="native"&&!ports.repoPresent())&&wrong===null);
     if(!result.detached&&!signal.aborted&&(outcome!=="ok"||result.attribution===undefined||result.attribution==="applied")){
      const physical=result.physicalEnd,endedMs=physical?.endedMs??ports.now();
      ports.timing?.({stage:row.name,startedMs:result.startedMs,endedMs,seconds:Math.max(0,Math.round((endedMs-result.startedMs)/1000)),
       ...readyTimingUnitsV1(unitsBefore,ports.units?.()??null),exit:result.exit,outcome,peakMegabytes:result.peakMegabytes,growthMegabytes:freeBefore===null?null:((free:number|null)=>free===null?null:Math.max(0,freeBefore-free))(ports.freeMegabytes?.()??null),
       growthInodes:inodesBefore===null?null:((free:number|null)=>free===null?null:Math.max(0,inodesBefore-free))(ports.freeInodes?.()??null),
       attempts:state.attempts+(outcome==="ok"?0:1),digest:physical===undefined?stages[decision.stageIndex]!.digest.slice(0,12):physical.digest,
       ...physical===undefined?{}:{endEvidence:physical.endEvidence,signal:result.signal??null},...readyTimingSenderV1(result),...timingWatchdogV1(result.watchdog)});
     }
    if(result.detached||signal.aborted){const latest=ports.state.read();ports.state.write({...latest,peakMegabytes:Math.max(latest.peakMegabytes??0,peak??0)||latest.peakMegabytes});break;}
    if(!stalledOut)stalls.delete(row.name);
    const stallCount=stalledOut&&result.codeChanged===undefined?readyStallCountV1(stalls.get(row.name)??0,result.progressed===true):0,stallVerdict=stallCount>0?readyStallRestartV1({consecutive:stallCount}):undefined,stallExhausted=stallVerdict?.action==="fail";
    if(stalledOut&&!stallExhausted){
     const waitingStale=result.codeStaleWaiting===true,byCode=result.codeChanged!==undefined&&!waitingStale,stale=result.codeStale===true,silent=byCode?undefined:result.holderSilent,recorded=result.watchdog?.stepOverdue;
     const over=byCode||silent!==undefined?undefined:result.stepOverdue??(readyValidStepEvidenceV1(recorded)?{text:"recorded read / aufgezeichneter Lesevorgang",seconds:recorded.seconds,bound:recorded.bound,limit:recorded.limit,quiet:Math.round(recorded.quietMs/1000)}:undefined);
     const minutes=Math.round((waitingStale?READY_TREE_POLICY_V1.staleWaitingMilliseconds:stale?READY_TREE_POLICY_V1.staleStageMilliseconds:byCode?READY_TREE_POLICY_V1.holderWaitRestartMilliseconds:READY_TREE_POLICY_V1.stallStopMilliseconds)/60000),pause=stallVerdict?.waitMilliseconds??0;
     if(stallCount>0)stalls.set(row.name,stallCount);
     stalledStage=row.name;stalledReason=waitingStale?"code-stale-waiting":stale?"code-stale":byCode?"code-changed":silent!==undefined?"holder-silent":over!==undefined?"step-overdue":"stalled";
     ports.mirror?.(`${row.name}-stalled`,result.failure??result.tail);
     if(waitingStale)ports.say(`stage ${row.name} runs older code than this supervisor and only waited for ${minutes} min, its process tree was stopped, it starts again at once with the current code and keeps what it finished / Stufe ${row.name} führt älteren Code aus als dieser Aufseher und wartete nur ${minutes} Min., ihr Prozessbaum wurde beendet, sie startet sofort neu mit dem aktuellen Code und behält alles Fertige`);
     else if(stale)ports.say(`stage ${row.name} had run for at least ${minutes} min when its code changed (${result.codeChanged}), its process tree was stopped, it starts again at once with the new code and keeps what it finished / Stufe ${row.name} lief mindestens ${minutes} Min., als sich ihr Code änderte (${result.codeChanged}), ihr Prozessbaum wurde beendet, sie startet sofort neu mit dem neuen Code und behält alles Fertige`);
     else if(byCode)ports.say(`stage ${row.name} waited ${minutes} min for a named holder while its code changed (${result.codeChanged}), its process tree was stopped, it starts again at once with the new code / Stufe ${row.name} wartete ${minutes} Min. auf einen benannten Halter, während sich ihr Code änderte (${result.codeChanged}), ihr Prozessbaum wurde beendet, sie startet sofort neu mit dem neuen Code`);
     else if(silent!==undefined)ports.say(`stage ${row.name} waited for process ${silent.pid} (${silent.command}) that holds the dependency store but has not renewed its registration for ${silent.seconds} s, it may be blocked; its process tree was stopped, the stage starts again in ${Math.round(pause/1000)} s / Stufe ${row.name} wartete auf Prozess ${silent.pid} (${silent.command}), der den Abhängigkeitsspeicher hält, aber seine Anmeldung seit ${silent.seconds} s nicht erneuert hat und womöglich blockiert ist; ihr Prozessbaum wurde beendet, sie startet in ${Math.round(pause/1000)} s neu`);
     else if(over!==undefined)ports.say(`stage ${row.name} ran the step "${over.text}" for ${over.seconds} s although it declares ${over.bound} s (overdue on this machine after ${over.limit} s) and nothing else moved for ${over.quiet} s; its process tree was stopped, the stage starts again with the current code in ${Math.round(pause/1000)} s (stop ${stallCount} of ${READY_TREE_POLICY_V1.stallRestartMaximum}) / Stufe ${row.name} führte den Schritt "${over.text}" seit ${over.seconds} s aus, obwohl er ${over.bound} s erklärt (auf dieser Maschine überfällig nach ${over.limit} s), und nichts anderes bewegte sich seit ${over.quiet} s; ihr Prozessbaum wurde beendet, die Stufe startet in ${Math.round(pause/1000)} s mit dem aktuellen Code neu (Stopp ${stallCount} von ${READY_TREE_POLICY_V1.stallRestartMaximum})`);
     else ports.say(`stage ${row.name} showed nothing new for ${minutes} min, its process tree was stopped, it starts again in ${Math.round(pause/1000)} s (stop ${stallCount} of ${READY_TREE_POLICY_V1.stallRestartMaximum}) / Stufe ${row.name} zeigte ${minutes} Min. nichts Neues, ihr Prozessbaum wurde beendet, sie startet in ${Math.round(pause/1000)} s neu (Stopp ${stallCount} von ${READY_TREE_POLICY_V1.stallRestartMaximum})`);
     status.update({...base,state:"waiting",ready:false,stage:row.name,stageIndex:decision.stageIndex+1,exit:result.exit,tail:[...result.tail],refusal:waitingStale?readyRefusalV1("stage-code-stale-waiting",{stage:row.name,minutes}):stale?readyRefusalV1("stage-code-stale",{stage:row.name,minutes,file:result.codeChanged!}):byCode?readyRefusalV1("stage-code-changed",{stage:row.name,minutes,file:result.codeChanged!}):silent!==undefined?readyRefusalV1("stage-holder-silent",{stage:row.name,minutes,pid:silent.pid,command:silent.command,silent:silent.seconds,seconds:Math.round(pause/1000),count:stallCount,maximum:READY_TREE_POLICY_V1.stallRestartMaximum}):over!==undefined?readyRefusalV1("stage-step-overdue",{stage:row.name,step:over.text,seconds:over.seconds,bound:over.bound,limit:over.limit,quiet:over.quiet,pause:Math.round(pause/1000),count:stallCount,maximum:READY_TREE_POLICY_V1.stallRestartMaximum}):readyRefusalV1("stage-stalled",{stage:row.name,minutes,seconds:Math.round(pause/1000),count:stallCount,maximum:READY_TREE_POLICY_V1.stallRestartMaximum}),containment:lastContainment,nextAttemptMs:pause>0?ports.now()+pause:null});
     leave();
     if(pause>0)await ports.sleep(pause,signal);
     continue;
    }
    if(stallExhausted)stalls.delete(row.name);
    const binaryMissing=row.name==="native"&&result.exit===0&&!ports.repoPresent();
    if(readyCleanExitV1(result)&&!binaryMissing&&wrong===null){
      if(result.attribution!==undefined)continue;
     const written:ReadyLocalStateV1={...state,stages:{...state.stages,[row.name]:{digest:stages[decision.stageIndex]!.digest,doneMs:ports.now()}},failure:null,attempts:0,peakMegabytes:peak};
     swept.delete(row.name);
     if(decision.stageIndex===names.length-1){ports.marker.write();written.readyMs=ports.now();}
     ports.state.write(written);
     continue;
    }
    if(wrong===null&&!result.cancelled&&!result.timedOut&&!swept.has(row.name)&&ports.sweep!==undefined&&readyNoSpaceV1(result.tail)){
     swept.add(row.name);leave();
     const neededMegabytes=readyDiskNeedV1(row.name,[]).megabytes;
     ports.say(`stage ${row.name} ran out of space: collecting every build-owned cache once, then retrying once / Stufe ${row.name} hat keinen Platz mehr: alle vom Build besessenen Caches werden einmal bereinigt, danach genau ein neuer Versuch`);
     status.update({...base,state:"waiting",ready:false,stage:row.name,stageIndex:decision.stageIndex+1,refusal:readyDiskCollectingV1(row.name,{path:readyStoreFullV1(result.tail)?.path??ports.native?.()?.path??"-",resource:"space",needed:neededMegabytes,available:ports.freeMegabytes?.()??0}),tail:[],nextAttemptMs:null});
     try{await ports.sweep({stage:row.name,neededMegabytes,signal,report:line=>{if(!signal.aborted)ports.say(line);}});}
     catch(error){if(signal.aborted)break;ports.say(`collecting space for stage ${row.name} failed (${String((error as Error)?.message??error).slice(0,200)}) / Platz für Stufe ${row.name} sammeln fehlgeschlagen`);}
     continue;
    }
    const imagePartition=wrong!==null?null:readyImagePartitionOfLinesV1(result.tail,P.noSpacePatterns),baseTail=wrong!==null?[wrong.en,wrong.de]:readyTailWithDiskCauseV1(result.tail,result.tail.some(line=>line.includes(P.sqliteOpenPattern))?ports.disk?.(row.name)??null:null,readyNoSpaceV1(result.tail)?ports.native?.()??null:null),failedTail=imagePartition===null?baseTail:[...baseTail,readyImageFullLineV1(imagePartition)];
    const attempts=state.attempts+1,code:ReadyCodeV1=wrong!==null?"tooling-compiler":stallExhausted?"stage-stall-exhausted":result.timedOut?"stage-timeout":result.spawnFailed?"spawn-failed":result.diskGuard!==undefined?"disk-guard":result.cancelled?"interrupted":result.wardenCancel!==undefined?"memory-cancelled":imagePartition!==null?"image-full":readyStoreFullV1(failedTail)!==null?"store-full":readySourceTransientV1([...result.failure??[],...failedTail],ports.root??"")?"source-transient":readyExternallyStoppedV1({exit:result.exit,signal:result.signal,tail:failedTail,...result.failure===undefined?{}:{failure:result.failure}})?"interrupted":"stage-failed",at=ports.now();
    const wardenLines=result.wardenCancel===undefined?[]:(["en","de"] as const).map(language=>formatHostWardenCancelLineV1(language,result.wardenCancel!));
    const next:ReadyFailureV1={digest,stage:row.name,attempts,exit:result.exit,signal:result.signal??null,code,nextAttemptMs:at+readyBackoffV1(result.exit===P.retryExit||code==="memory-cancelled"||code==="disk-guard"||code==="image-full"||code==="interrupted"?1:attempts,ports.jitter),atMs:at,tail:[...failedTail,...wardenLines].slice(-P.tailLines),...startTree===null?{}:{tree:startTree}};
    ports.state.write({...state,failure:next,attempts,peakMegabytes:peak});
    if(wrong!==null)await discardDependencies(digest);
    ports.mirror?.(`${row.name}-failed`,wrong!==null?next.tail:result.failure??next.tail);
    ports.say(`stage ${row.name} failed (${code}, ${readyEndingV1(result.exit,result.signal).en}), next attempt in ${Math.round((next.nextAttemptMs-at)/1000)} s / Stufe ${row.name} fehlgeschlagen (${readyEndingV1(result.exit,result.signal).de}), nächster Versuch in ${Math.round((next.nextAttemptMs-at)/1000)} s`);
    leave();
    continue;
   }
   else leave();
   await ports.sleep(Math.max(250,Math.min(decision.waitMilliseconds,P.idlePollMilliseconds)),signal);
  }
 }finally{leave();}
}

const isServer=(origin:string,group:string|undefined):boolean=>group!==undefined&&group!==""&&origin===`${group}${P.serverSuffix}`;

/** 🏷️ Whether this origin is the group server of its group, which is never set up by this service. */
export function readyIsServerV1(origin:string,group:string|undefined):boolean{return isServer(origin,group);}

function stagePathV1(environment:Readonly<Record<string,string|undefined>>,root:string,home:string):NodeJS.ProcessEnv{
 const windows=process.platform==="win32",entries=[...P.pathDirectories.map(entry=>join(home,...entry.split("/"))),...windows?P.systemPath.windows:P.systemPath.unix,...(environment.PATH??"").split(delimiter)].filter(Boolean);
 const env:NodeJS.ProcessEnv={...environment,PATH:[...new Set(entries)].join(delimiter),HOME:environment.HOME??home,SEMIO_WORKSPACE_ROOT:root,[P.languageEnvironment]:environment[P.languageEnvironment]??"en",[P.capacityPriorityEnvironment]:P.capacityPriority,[P.faultDetailEnvironment]:P.faultDetailEnabled};
 for(const name of P.droppedEnvironment)delete env[name];
 if(!windows&&env.LC_ALL===undefined&&(env.LANG===undefined||!/utf-?8/iu.test(env.LANG)))env.LANG=P.locale[readyPlatformV1() as "macos"|"linux"];
 return env;
}

/** 🪄️ The environment a stage command of this machine runs in (the tool directories on the path, the checkout as workspace root, the language, no dropped variable): what the checkout test worker gives `repo test-file` so it runs under the same environment as the stages that made the machine Ready. */
export const readyStageEnvironmentV1=(environment:Readonly<Record<string,string|undefined>>,root:string,home:string):NodeJS.ProcessEnv=>stagePathV1(environment,root,home);

/** 🔎️ The `repo` binary of the account: the first of its tool directory and the launcher directory that holds it (on Windows the launcher lives in `%LOCALAPPDATA%\\semio\\launcher\\bin` and is linked into `~/.local/bin` only later), `undefined` when none does. */
export function readyRepoBinaryV1(home:string,environment:Readonly<Record<string,string|undefined>>=process.env,platform:NodeJS.Platform=process.platform):string|undefined{
 const windows=platform==="win32",name=windows?P.repoNames.windows:P.repoNames.unix;
 const directories=[...(windows?P.repoDirectories.windows:P.repoDirectories.unix).map(entry=>join(home,...entry.split("/"))),...windows&&environment[P.launcherEnvironment]!==undefined?[join(environment[P.launcherEnvironment]!,"semio","launcher","bin")]:[]];
 return directories.map(directory=>join(directory,name)).find(path=>existsSync(path));
}

type ReadyStageCommandV1=Readonly<{command:string;args:readonly string[];env:NodeJS.ProcessEnv}>;
type ReadyStageLaunchV1=Readonly<{version:1;platform:ReadyPlatformV1;digest:string;recipe:string;prerequisites:readonly Readonly<{stage:string;digest:string;doneMs:number;completion?:string}>[]}>;
type ReadyStageCompletionV1=Readonly<{capture(stage:ReadyStageV1,recipe:string):ReadyStageLaunchV1|undefined;apply(flight:ReadyInflightV1,endedMs:number,result:ReadyRunResultV1):NonNullable<ReadyRunResultV1["attribution"]>}>;

function readyStageCommandV1(stage:ReadyStageV1,root:string,home:string,environment:Readonly<Record<string,string|undefined>>):ReadyStageCommandV1|undefined{
 const env=stagePathV1(environment,root,home),windows=process.platform==="win32";
 if(stage.kind==="repo"){
  const command=readyRepoBinaryV1(home,environment);
  return command===undefined?undefined:{command,args:stage.arguments,env};
 }
 const script=join(root,...(windows?P.bootstrap.windows:P.bootstrap.unix));
 if(windows)return{command:"powershell.exe",args:["-NoProfile","-ExecutionPolicy","Bypass","-File",script,...stage.arguments,"-SkipRepoBootstrap"],env};
 env.SKIP_REPO_BOOTSTRAP="1";
 return{command:"bash",args:[script,...stage.arguments],env};
}

function readyStageRecipeV1(stage:ReadyStageV1,launch:ReadyStageCommandV1):string{
 const environment=Object.keys(launch.env).filter(name=>name!==P.capacityPriorityEnvironment).sort().flatMap(name=>launch.env[name]===undefined?[]:[[name,launch.env[name]]]);
 return sha(JSON.stringify([1,stage.name,stage.kind,stage.arguments,stage.timeoutMilliseconds,stage.inputs,stage.bootstrapInput,process.execPath,launch.command,launch.args,environment]));
}

function sameReadyLaunchV1(left:ReadyStageLaunchV1,right:ReadyStageLaunchV1):boolean{
 return left.platform===right.platform&&left.digest===right.digest&&left.recipe===right.recipe&&left.prerequisites.length===right.prerequisites.length&&left.prerequisites.every((row,index)=>{
  const other=right.prerequisites[index]!;
  return row.stage===other.stage&&row.digest===other.digest&&row.doneMs===other.doneMs&&row.completion===other.completion;
 });
}

function writeReadyLocalRecordV1(file:string,value:unknown):void{
 mkdirSync(dirname(file),{recursive:true});
 const temporary=file+"."+process.pid+".tmp";
 try{writeFileSync(temporary,JSON.stringify(value));renameSync(temporary,file);}
 catch(error){try{rmSync(temporary,{force:true});}catch{}throw error;}
}

/** 🧮️ The `units` of a timing row: what the stage run did with compile units (restored from this machine, restored from the fleet, compiled, stored, not shared), the growth of the unit counters between its start and its end; nothing when either reading is missing (an adopted run has no start reading), so a row never claims a count nobody measured.
 * @see ../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📊️counters/🟦️.ts */
export const readyTimingUnitsV1=(before:Readonly<Record<UnitCountKindV1,number>>|null,after:Readonly<Record<UnitCountKindV1,number>>|null):{units?:UnitLedgerV1}=>before===null||after===null?{}:{units:unitLedgerV1(before,after)};
const timingWatchdogV1=(watchdog:ReadyTimingWatchdogV1|undefined)=>watchdog===undefined?{}:{watchdog};
const readyTimingWatchdogV1=(lastActivity:ReadyOwnedActivityV1|undefined,evidence:ReadyStopEvidenceV1)=>timingWatchdogV1(lastActivity===undefined&&evidence.deadline===undefined&&evidence.stepOverdue===undefined?undefined:{...lastActivity===undefined?{}:{lastActivity},...evidence});
const readyTerminalOutcomeV1=(result:ReadyRunResultV1,ok=readyCleanExitV1(result)):ReadyTimingV1["outcome"]=>ok?"ok":result.timedOut?"stage-timeout":result.stalled&&!result.cancelled?"stalled":result.spawnFailed?"spawn-failed":result.cancelled||result.wardenCancel!==undefined||readyExternallyStoppedV1(result)?"interrupted":"stage-failed";
const readyCleanExitV1=(result:ReadyRunResultV1):boolean=>result.exit===0&&result.signal==null&&!result.detached&&!result.timedOut&&!result.spawnFailed&&!result.cancelled&&!result.stalled;

const alive=(pid:number):boolean=>processPidExistsV1(pid);

/** 🧬️ Whether the process of a stage in flight is alive as that stage's process: its pid exists and the creation time of the process there is not another one than the stage was spawned with (a reused pid is another process and the stage is gone). */
const stageProcessAliveV1=(directory:string,row:Readonly<{stage:string;pid:number;startedMs:number}>,read:(pid:number)=>number|null=readyProcessStartSyncV1):boolean=>alive(row.pid)&&readyStageProcessVerdictV1(row,readyReadTreeV1(directory,row.stage)?.root,read(row.pid))=="same";

/** 🧾️ The record of the stage in flight, `<worktree>/.cache/f/ready/running.json`: which stage, which process group, when it started, where its exit file and log are. */
type ReadyWatchdogV1=Readonly<{
 toolsGrace?:ToolsCheckpoint|null;
 version:1;owner:string;peakMegabytes:number;progressAt:number;cpuSince?:number;stallAt:number;watchedAt:number;unwatchedMilliseconds:number;armedAt:number;progressKey:string|null;phase:ReadyPhaseV1;phaseAt:number;offset:number;
 seen:readonly string[];step:ReadyStepWatchCheckpointV1;holderSince:number|null;holderEvidence:string|null;
 graphCounter?:number;semantics:Readonly<Record<string,string|null>>;cpu:readonly (readonly [number,number])[];windows?:ReadyWindowsWatchdogSampleV1;lastActivity?:ReadyOwnedActivityV1;
}>;
type ReadyStageStopV1=Extract<ReadyKillReasonV1,"stall"|"overdue"|"restart"|"timeout"|"cancel">;
export type ReadyInflightV1=Readonly<{stage:string;pid:number;startedMs:number;exitFile:string;log:string;supervisor?:number;code?:string;watchdog?:ReadyWatchdogV1;launch?:ReadyStageLaunchV1;stop?:ReadyStageStopV1;stopBy?:Readonly<{supervisor:number;reason:ReadyStageStopV1;atMs:number}>}>&ReadyStopEvidenceV1;

function readInflightV1(file:string):ReadyInflightV1|undefined{
 try{
  const fd=openSync(file,"r");
  try{
   const stat=fstatSync(fd);
   if(!stat.isFile()||stat.size>P.watchdogRecordBytes)return undefined;
   return JSON.parse(readFileSync(fd,"utf8")) as ReadyInflightV1;
  }finally{closeSync(fd);}
 }catch{return undefined;}
}

const readyPhysicalLaunchV1=(flight:ReadyInflightV1):ReadyStageLaunchV1|undefined=>flight.launch!==undefined&&!validateJsonSchemaSubset(schema.definitions.StageLaunch,flight.launch,schema).length?flight.launch:undefined;
const readyRecordedStageStopV1=(flight:ReadyInflightV1):ReadyStageStopV1|undefined=>flight.stop!==undefined&&schema.definitions.StageStop.enum.includes(flight.stop)&&readyPhysicalLaunchV1(flight)!==undefined?flight.stop:undefined;
const readyValidStepEvidenceV1=(step:ReadyStepOverdueEvidenceV1|undefined):step is ReadyStepOverdueEvidenceV1=>step!==undefined&&!validateJsonSchemaSubset(schema.definitions.StepOverdueEvidence,step,schema).length&&step.seconds>=step.limit&&step.limit>=step.bound;
const readyRecordedStopEvidenceV1=(flight:ReadyInflightV1,stop:ReadyStageStopV1|undefined):ReadyStopEvidenceV1=>{
 if(stop==="timeout"&&flight.deadline!==undefined&&schema.definitions.DeadlineStop.enum.includes(flight.deadline)){
  const denial=flight.toolsGraceDenial;
  return{deadline:flight.deadline,...flight.stage==="tools"&&flight.deadline==="hard-cap"&&denial!==undefined&&schema.definitions.ToolsGraceDenial.enum.includes(denial)?{toolsGraceDenial:denial}:{}};
 }
 if(stop==="overdue"&&readyValidStepEvidenceV1(flight.stepOverdue))return{stepOverdue:flight.stepOverdue};
 return{};
};
const readyStoppedForStallV1=(stop:ReadyStageStopV1|undefined):boolean=>stop==="stall"||stop==="overdue"||stop==="restart";

function readyPhysicalEndV1(flight:ReadyInflightV1,observedMs:number,exitMs?:number):NonNullable<ReadyRunResultV1["physicalEnd"]>{
 const ended=exitMs!==undefined&&exitMs>=flight.startedMs&&exitMs<=observedMs;
 const digest=readyPhysicalLaunchV1(flight)?.digest.slice(0,12)??null;
 return {endedMs:ended?exitMs:observedMs,endEvidence:ended?"exit-receipt":"observed",digest};
}

const watchdogOwnerV1=(inflight:ReadyInflightV1):string=>sha(JSON.stringify([inflight.stage,inflight.pid,inflight.startedMs]));

function restoredWatchdogV1(inflight:ReadyInflightV1,now:number,policy:typeof READY_TREE_POLICY_V1):ReadyWatchdogV1|undefined{
 const row=inflight.watchdog;
 if(row===undefined||validateJsonSchemaSubset(schema.definitions.Watchdog,row,schema).length||row.owner!==watchdogOwnerV1(inflight))return undefined;
 const times=[row.progressAt,row.stallAt,row.watchedAt,row.phaseAt,row.step.movedAt,row.holderSince];
 if(times.some(at=>at!==null&&(at<inflight.startedMs||at>now))||row.armedAt<inflight.startedMs||row.armedAt>now+2*P.samplerMilliseconds+P.pollMilliseconds||
  row.seen.length>policy.stallSeenMaximum||row.step.seen.length>policy.stallSeenMaximum)return undefined;
 if(row.toolsGrace!==undefined&&row.toolsGrace!==null&&!currentToolsCheckpoint(row.toolsGrace,inflight.startedMs,now)){const {toolsGrace:_invalid,...retained}=row;return retained;}
 return row;
}

/** 💾️ Writes a file in the store so a reader never sees half of it: a temporary file renamed over the target; where the rename is refused (Windows with an open reader, a network share) the text is written to the target directly and the temporary file is removed, so a mirror never goes stale and no `.tmp` is left. */
export function readyWriteStoreFileV1(file:string,text:string,rename:(from:string,to:string)=>void=renameSync):void{
 const temporary=`${file}.${process.pid}.tmp`;
 writeFileSync(temporary,text);
 try{rename(temporary,file);}catch{try{writeFileSync(file,text);}catch{}try{rmSync(temporary,{force:true});}catch{}}
}

/** 🪞️ Keeps the host warden alive and mirrors its state to the group store as `v1/origins/<origin>/memory.json` (schema `Mirror` of the warden: the state as written, or `{"version":1,"atMs":<now>,"absent":true}` when no fresh state exists), so the coordinator sees available memory, swap, level and interventions per worker without the scheduler. `tick` runs the cheap warden ensure, then writes at most every `memoryMirrorMilliseconds` and at once when the level or an intervention counter changed; it answers whether it wrote.
 * @see ../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🧬️schema/🔣️.json */
export function createReadyMemoryMirrorV1(input:Readonly<{store:string|undefined;origin:string;now?:()=>number;ensure?:()=>unknown;current?:(nowMs:number)=>HostWardenMirrorV1;write?:(file:string,text:string)=>void}>):Readonly<{tick():boolean}>{
 const now=input.now??Date.now,ensure=input.ensure??(()=>ensureHostWardenV1()),current=input.current??(at=>currentHostWardenMirrorV1(undefined,at)),write=input.write??((file,text)=>{mkdirSync(dirname(file),{recursive:true});readyWriteStoreFileV1(file,text);});
 let lastKey:string|undefined,lastAt=-Infinity;
 return Object.freeze({tick():boolean{
  try{ensure();}catch{}
  if(input.store===undefined||input.store==="")return false;
  try{
   const at=now(),record=current(at),key="absent"in record?"absent":`${record.level}|${record.counters.suspended}|${record.counters.resumed}|${record.counters.cancelled}`;
   if(key===lastKey&&at-lastAt<P.memoryMirrorMilliseconds)return false;
   write(join(input.store,"v1","origins",input.origin,P.memoryMirrorName),JSON.stringify(record));
   lastKey=key;lastAt=at;return true;
  }catch{return false;}
 }});
}

/** 🧾️ The schema names refusal and plan-guard lines whose diagnostic fields need the larger log tail budget. */
const longRefusalPatternsV1=P.longRefusalPatterns.map(pattern=>new RegExp(pattern,"u")),errorLinesV1=P.mirrorErrorPatterns.map(pattern=>new RegExp(pattern,"u"));

/** 📜️ The whole lines a stage wrote to its log since `offset` (at most the last four mebibytes of it; a half line stays for the next call), with the offset after them: the supervisor sees every line once, however much the stage prints between two looks, so a marker such as the first line of the tasks is never lost to the tail window. */
function logSinceV1(log:string,offset:number):Readonly<{lines:string[];offset:number}>{
 try{
  const size=statSync(log).size,from=size<offset?0:Math.max(offset,size-4194304);
  if(size<=from)return{lines:[],offset:Math.min(offset,size)};
  const buffer=Buffer.alloc(size-from),handle=openSync(log,"r");
  try{readSync(handle,buffer,0,buffer.length,from);}finally{closeSync(handle);}
  const end=buffer.lastIndexOf(10);
  if(end<0)return{lines:[],offset:from};
  return{lines:buffer.subarray(0,end).toString("utf8").split(/\r?\n/u),offset:from+end+1};
 }catch{return{lines:[],offset};}
}


const readyActivityResourceV1=new RegExp(schema.definitions.OwnedActivity.properties.resource.pattern,"u");
const readyActivityCounterV1=/^([A-Za-z-]+(?::[A-Za-z-]+)*):([1-9][0-9]{0,15})(?:\/[1-9][0-9]{0,3}:canonical:[0-9]{1,4}:changed:[0-9]{1,4})?$/u;

/** 📍️ Retains the latest productive counter from the bounded batch of complete stage log lines. */
function readyOwnedActivityV1(lines:readonly string[],last:ReadyOwnedActivityV1|undefined):ReadyOwnedActivityV1|undefined{
 for(const line of lines){
  if(line.length>P.refusalLineCharacters||!line.startsWith("[script] {"))continue;
  let row:{stage?:string;activity?:{resource?:string;mode?:string;elapsedMs?:number}}|null;
  try{row=JSON.parse(line.slice(9));}catch{continue;}
  if(row?.stage!=="running"||typeof row.activity?.resource!=="string"||(row.activity.mode!=="shared"&&row.activity.mode!=="exclusive")||
   typeof row.activity.elapsedMs!=="number"||!Number.isFinite(row.activity.elapsedMs)||row.activity.elapsedMs<0)continue;
  const matched=readyActivityCounterV1.exec(row.activity.resource);
  if(matched===null||!readyActivityResourceV1.test(matched[1]!))continue;
  const resource=matched[1]!,counter=Number(matched[2]);
  if(Number.isSafeInteger(counter)&&(last?.resource!==resource||counter>last.counter))last={resource,counter};
 }
 return last;
}

/** 🗂️ Recovers structured tool refusals from a failed stage using bounded buffers and complete lines. */
function toolFailureEvidenceV1(stage:string,log:string):string[]{
 if(stage!=="tools")return[];
 const lines=new Map<string,string>(),buffer=Buffer.alloc(P.failureEvidenceChunkBytes),decoder=new StringDecoder("utf8");
 let carry="";
 try{
  const handle=openSync(log,"r");
  try{
   const size=fstatSync(handle).size;
   for(let position=0;position<size;){
    const bytes=readSync(handle,buffer,0,Math.min(size-position,buffer.length),position);
    if(bytes===0)break;
    position+=bytes;
    const current=(carry+decoder.write(buffer.subarray(0,bytes))).split(/\r?\n/u);
    carry=current.pop()!;
    if(carry.length>P.refusalLineCharacters)carry="\0";
    for(const line of current){
     const row=readToolSetupRefusalLineV1(line);
     if(row===undefined)continue;
     if(lines.has(row.id)||lines.size<P.failureEvidenceLines)lines.set(row.id,line);
    }
   }
  }finally{closeSync(handle);}
 }catch{}
 return [...lines.values()];
}

/** 🧯️ The last `count` (the Nx summary policy bound by default) lines of the own output of every task that the Nx footer of the stage log names as failed, each under a bilingual header line (Nx hides nothing of a failed task, but a flood of progress rows pushes its text out of the failure window and out of `ready.json`'s tail, so it is cut from the whole window of the log and appended whole); empty when the log has no Nx footer or cannot be read. */
function failedTaskTailLinesV1(log:string,count?:number):string[]{
 try{
  const size=statSync(log).size,readFrom=Math.max(0,size-P.failureWindowBytes),buffer=Buffer.alloc(size-readFrom),handle=openSync(log,"r");
  let countRead:number;
  try{countRead=readSync(handle,buffer,0,buffer.length,readFrom);}finally{closeSync(handle);}
  return nxFailedTaskTailsV1(buffer.subarray(0,countRead).toString("utf8"),count).flatMap(row=>[`[nx] failed task ${row.task}, its last ${row.lines.length} output lines / Fehlgeschlagene Aufgabe ${row.task}, ihre letzten ${row.lines.length} Ausgabezeilen`,...row.lines]);
 }catch{return[];}
}

/** 🧷️ The lines that tell why a stage failed (`errors` puts the first `mirrorErrorLines` error lines of the window, whole, and a count of the lines left out before the last ones, so that the cause stays visible when a flood of other lines follows it): the last `failureWindowBytes` of its log without the machine rows and the heartbeat sentences of `failureNoise` (a flood of them after the failure text, or before it, is no cover), the last `count` of them cut to `width`; a log that has nothing else keeps its own last lines. */
function failureLinesV1(log:string,count:number,width:number,evidence:readonly string[],errors:boolean=false):string[]{
 try{
  const size=statSync(log).size,readFrom=Math.max(0,size-P.failureWindowBytes-1),buffer=Buffer.alloc(size-readFrom),handle=openSync(log,"r");
  let countRead:number;
  try{countRead=readSync(handle,buffer,0,buffer.length,readFrom);}finally{closeSync(handle);}
  const input=buffer.subarray(0,countRead),boundary=size>P.failureWindowBytes?input.indexOf(10):-1;
  if(size>P.failureWindowBytes&&boundary<0)return [...evidence];
  const all=input.subarray(boundary+1).toString("utf8").split(/\r?\n/u).filter(line=>line!==""&&!evidence.includes(line)),noise=P.failureNoise.map(pattern=>new RegExp(pattern,"u")),quiet=all.filter(line=>!noise.some(expression=>expression.test(line)));
  const retained=quiet.length>0?quiet:all,keep=Math.max(0,count-evidence.length),earlier=retained.slice(0,Math.max(0,retained.length-keep));
  const first=errors?earlier.filter(line=>errorLinesV1.some(expression=>expression.test(line))).slice(0,P.mirrorErrorLines):[];
  const head=first.length===0?[]:[...first.map(line=>line.slice(0,P.refusalLineCharacters)),`[...] ${earlier.length-first.length} earlier lines left out / ${earlier.length-first.length} frühere Zeilen ausgelassen`];
  return [...head,...retained.slice(Math.max(0,retained.length-keep+head.length)).map(line=>formatLogLineV1(line,width)),...evidence];
 }catch{return [...evidence];}
}

/** 🪦️ The line that says a stage process vanished: no exit file, no signal, so its wrapper was ended from outside the supervisor; when it was last seen alive and whether this supervisor had sent a stop signal itself. */
const vanishedLineV1=(pid:number,lastSeenSeconds:number,stopped:boolean):string=>`the stage process ${pid} is gone: it left no exit file and no signal was recorded, so its wrapper was ended without a chance to write the end (a SIGKILL from a person or a program, the system under memory pressure, or a restart of the machine); it was last seen alive ${lastSeenSeconds} s ago and this supervisor ${stopped?"had sent a stop signal to its tree":"sent no signal"} / Der Stufenprozess ${pid} ist weg: er hinterließ keine Exit-Datei und es wurde kein Signal festgehalten, sein Wrapper wurde also beendet, ohne das Ende schreiben zu können (ein SIGKILL einer Person oder eines Programms, das System unter Speicherdruck oder ein Neustart der Maschine); zuletzt lebend gesehen vor ${lastSeenSeconds} s, und dieser Aufseher ${stopped?"hatte ein Stoppsignal an seinen Baum gesandt":"sandte kein Signal"}`;

/** 🪦️ The line that says the process table gave the stage pid another start time twice in a row: the recorded and the seen start, the tolerance and how long the table read took, so a slow table can be told from a reused pid. */
const changedLineV1=(pid:number,recordedMs:number,seenMs:number,firstMs:number,toleranceMs:number,tableMs:number):string=>`process ${pid} stands in the process table with a start time ${Math.round(Math.abs(seenMs-recordedMs)/1000)} s away from the recorded one (recorded ${new Date(recordedMs).toISOString()}, seen ${new Date(seenMs).toISOString()} and before ${new Date(firstMs).toISOString()}, tolerance ${Math.round(toleranceMs/1000)} s, the last table read took ${Math.round(tableMs/1000)} s) in two reads in a row, so it is judged another process that reused the pid and the stage is ended / Prozess ${pid} steht mit einer um ${Math.round(Math.abs(seenMs-recordedMs)/1000)} s abweichenden Startzeit in der Prozesstabelle (festgehalten ${new Date(recordedMs).toISOString()}, gesehen ${new Date(seenMs).toISOString()} und zuvor ${new Date(firstMs).toISOString()}, Toleranz ${Math.round(toleranceMs/1000)} s, das letzte Lesen der Tabelle dauerte ${Math.round(tableMs/1000)} s) in zwei Lesungen hintereinander, er gilt daher als anderer Prozess mit wiederverwendeter pid und die Stufe wird beendet`;

/** 🪦️ The line that names the signal that ended a stage process (the wrapper records it; a bare exit status cannot tell a kill from a failure). */
const exitSignalLineV1=(signal:string):string=>`the stage process was ended by signal ${signal} / Der Stufenprozess wurde durch das Signal ${signal} beendet`;

const formatLogLineV1=(line:string,width:number):string=>line.slice(0,longRefusalPatternsV1.some(pattern=>pattern.test(line))?P.refusalLineCharacters:width);
const analysisTailV1=(log:string):string[]=>logTailV1(log,60,P.refusalLineCharacters);

/** 🪞️ The live mirror of a stage log: the last `mirrorLines` lines without the machine rows and heartbeat sentences of `failureNoise`, so a stage that waits for hours keeps its last real words in the mirror instead of a wall of `still running after N s`, followed by the newest raw line, which shows that the stage still lives. */
export function mirrorLinesV1(log:string):string[]{
 const last=logTailV1(log,1,P.mirrorLineCharacters)[0],full=failureLinesV1(log,P.mirrorLines,P.mirrorLineCharacters,[]);
 return last===undefined||full.at(-1)===last?full:[...failureLinesV1(log,P.mirrorLines-1,P.mirrorLineCharacters,[]),last];
}

function logTailV1(log:string,count:number=P.tailLines,width:number=P.tailCharacters):string[]{
 try{
  const handle=openSync(log,"r");
  try{
   const size=fstatSync(handle).size,start=Math.max(0,size-P.tailBytes-1),buffer=Buffer.alloc(size-start);
   const input=buffer.subarray(0,readSync(handle,buffer,0,buffer.length,start)),clipped=size>P.tailBytes,boundary=clipped?input.indexOf(10):-1;
   if(clipped&&boundary<0)return[];
   return input.subarray(boundary+1).toString("utf8").split(/\r?\n/u).filter(line=>line!=="").slice(-count).map(line=>formatLogLineV1(line,width));
  }finally{closeSync(handle);}
 }catch{return[];}
}

/** 🧹️ What a stage runner needs to find the orphans of earlier tries: the lease registry folder of the user, the windows of this worker's stopped tries and the working window of a person. */
export type ReadyOrphanPortsV1=Readonly<{registryRoot:string;windows():readonly ReadyWindowV1[];human():ReadyPersonV1|null}>;

/** 🧬️ The digest of the code this supervisor runs: the sha256 over every source and schema file below this module directory (tests excluded, sorted by path), read once per process. A stage records it at its start (`code` of its in-flight record); a stage whose record differs, or carries none while another supervisor adopted it, runs older code than the supervisor that watches it. */
let moduleCodeV1:string|undefined;
export function readyModuleCodeV1():string{
 if(moduleCodeV1!==undefined)return moduleCodeV1;
 const hash=createHash("sha256"),walk=(directory:string,prefix:string):void=>{for(const entry of readdirSync(directory,{withFileTypes:true}).sort((left,right)=>left.name<right.name?-1:1)){if(entry.name==="🧪️tests")continue;const path=prefix+entry.name;if(entry.isDirectory())walk(join(directory,entry.name),path+"/");else if(/\.(?:ts|json|mjs)$/u.test(entry.name))try{const data=readFileSync(join(directory,entry.name));hash.update(`${path}\0${data.length}\0`);hash.update(data);}catch{}}};
 try{walk(import.meta.dir,"");}catch{}
 return moduleCodeV1=hash.digest("hex").slice(0,12);
}

type ReadyStageInputV1=Readonly<{code?:string;completion?:ReadyStageCompletionV1;root:string;environment:Readonly<Record<string,string|undefined>>;stage:ReadyStageV1;signal:AbortSignal;jobs:number;home?:string;onLine(line:string):void;onStage?(stage:string,adopted:boolean):void;onActivity?():void;mirror?:ReadyLogMirrorV1;facts?:(path:string)=>ReadyFilesystemFactsV1|null;tree?:typeof READY_TREE_POLICY_V1;orphans?:ReadyOrphanPortsV1;containment?:typeof READY_CONTAINMENT_POLICY_V1;speed?:number;collectImage?:(image:ReadyImageShortV1,signal:AbortSignal)=>Promise<void>}>;
type ReadyStageObserverV1=Pick<ReadyStageInputV1,"onLine"|"mirror"|"onStage"|"onActivity">;
type ReadyStageWatchV1={stage:string;stop:AbortController;observers:Set<ReadyStageObserverV1>;result:Promise<ReadyRunResultV1>;line?:string;physical?:Readonly<{stage:string;adopted:boolean}>;snapshot?:()=>ReadyRunResultV1};
const readyStageWatchesV1=new Map<string,ReadyStageWatchV1>();
const detachedReadyStageV1=(startedMs:number=Date.now(),tail:readonly string[]=[],peakMegabytes:number|null=null):ReadyRunResultV1=>({exit:null,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:true,startedMs,peakMegabytes,tail});
const sameReadyFlightV1=(left:ReadyInflightV1|undefined,right:ReadyInflightV1):boolean=>left?.stage===right.stage&&left.pid===right.pid&&left.startedMs===right.startedMs&&left.exitFile===right.exitFile;

function followReadyStageV1(watch:ReadyStageWatchV1,input:ReadyStageInputV1):Promise<ReadyRunResultV1>{
 const observer={onLine:input.onLine,mirror:input.mirror,onStage:input.onStage,onActivity:input.onActivity};
 return new Promise((finish,fail)=>{
  const leave=()=>{input.signal.removeEventListener("abort",detach);return watch.observers.delete(observer);};
  const detach=()=>{
   if(!leave())return;
   const result=watch.snapshot?.()??detachedReadyStageV1();
   if(watch.observers.size===0)watch.stop.abort();
   finish(result);
  };
  watch.observers.add(observer);
  input.signal.addEventListener("abort",detach,{once:true});
  void watch.result.then(result=>{if(leave())finish(result);},error=>{if(leave())fail(error);});
  if(input.signal.aborted)detach();
  else if(!watch.stop.signal.aborted){if(watch.physical)observer.onStage?.(watch.physical.stage,watch.physical.adopted);if(watch.line!==undefined)observer.onLine(watch.line);}
 });
}

/** 🏃️ Runs one stage, or adopts the one already in flight, as a process of its own that outlives this one: a detached wrapper (`./🏃️wrapper/🟦️.ts`) starts `bash <bootstrap> setup` with `SKIP_REPO_BOOTSTRAP=1` (PowerShell with `-SkipRepoBootstrap` on Windows) or the `repo` binary of the account, with the account's tool directories on PATH, the language of the machine, the fleet variables and no cache override, `CARGO_BUILD_JOBS` and `MAKEFLAGS` bounded by `jobs`, its output in `.cache/f/ready/<stage>.log` (the file descriptor belongs to the stage, so no supervisor restart loses a line; the log of the try before stays as `<stage>.previous.log`, and a failure is mirrored with its provenance and without the machine rows, see `failureLinesV1`); the record `running.json` lets a restarted runner (the agent restarts for new code every few minutes, and a stage takes longer) wait for the same process instead of killing it. The runner polls for the exit file, the resident size of the process tree (the peak), the last output lines, the timeout and the cancel request (a file `.cache/f/ready/cancel`, which ends the whole process group: SIGTERM, SIGKILL after the grace). Aborting `signal` only stops waiting (`detached: true`), the stage goes on. */
export async function runReadyStageV1(input:ReadyStageInputV1):Promise<ReadyRunResultV1>{
 if(input.signal.aborted)return detachedReadyStageV1();
 const directory=machinePathV1(input.root,...P.directory);
 mkdirSync(directory,{recursive:true});
 const key=realpathSync(directory);
 while(!input.signal.aborted){
  let watch=readyStageWatchesV1.get(key);
  if(watch===undefined){
   const stop=new AbortController(),observers=new Set<ReadyStageObserverV1>();
   const line=(text:string):void=>{owned.line=text;for(const observer of observers)observer.onLine(text);};
   const mirror:ReadyLogMirrorV1=(...args)=>{for(const observer of observers)observer.mirror?.(...args);};
   const onStage=(stage:string,adopted:boolean)=>{delete owned.line;owned.physical={stage,adopted};for(const observer of observers)observer.onStage?.(stage,adopted);};
   const onActivity=()=>{for(const observer of observers)observer.onActivity?.();};
   const owned:ReadyStageWatchV1={stage:input.stage.name,stop,observers,result:Promise.resolve().then(()=>runOwnedReadyStageV1({...input,signal:stop.signal,onLine:line,mirror,onStage,onActivity},directory,owned)).finally(()=>{if(readyStageWatchesV1.get(key)===owned)readyStageWatchesV1.delete(key);})};
   readyStageWatchesV1.set(key,watch=owned);
  }
  const result=await followReadyStageV1(watch,input);
  if(input.signal.aborted)return result;
  if(!result.detached&&input.completion!==undefined)return {...result,stage:watch.stage,...readyCleanExitV1(result)&&result.attribution===undefined?{attribution:"unproven"}:{}};
  if(!result.detached&&watch.stage===input.stage.name)return result;
 }
 return detachedReadyStageV1();
}

async function runOwnedReadyStageV1(input:ReadyStageInputV1,directory:string,watch:ReadyStageWatchV1):Promise<ReadyRunResultV1>{
 if(input.signal.aborted)return detachedReadyStageV1();
 const speed=input.speed??localMachineSpeedFactorV1(input.environment);
 const windows=process.platform==="win32",home=input.home??homedir(),inflightFile=join(directory,P.inflightName),cancelFile=join(directory,P.cancelName);
 const failed=(line:string,spawnFailed:boolean):ReadyRunResultV1=>({exit:null,timedOut:false,spawnFailed,cancelled:false,stalled:false,detached:false,startedMs:Date.now(),peakMegabytes:null,tail:[line]});
 const contain=input.containment??READY_CONTAINMENT_POLICY_V1,lines:string[]=[],say=(line:string):void=>{lines.push(line);input.onLine(line);},earlier={reaped:[] as ReadyReapedV1[],survivors:[] as ReadyKnownV1[]};
 const ownerOf=(stage:string,startedMs:number):string=>readyOwnerTokenV1(input.root,stage,startedMs);
 const spare=(table:readonly ReadyProcessV1[]):ReadonlySet<number>=>input.orphans===undefined?new Set<number>():readySharedPidsV1(readyRegistryEntriesV1(input.orphans.registryRoot,contain),table,contain);
 const sweepRosters=async():Promise<void>=>{const swept=await readySweepRostersV1({directory,say,policy:contain,owner:ownerOf,spare});earlier.reaped.push(...swept.reaped);earlier.survivors.push(...swept.survivors);};
 const reapRegistry=async(stage:Readonly<{root:ReadyKnownV1;known:readonly ReadyKnownV1[]}>|null,kind:"registry"|"holder",stageName=input.stage.name):Promise<ReadyContainmentResultV1>=>input.orphans===undefined?{reaped:[],survivors:[]}:readyReapOrphansV1({registryRoot:input.orphans.registryRoot,root:input.root,windows:input.orphans.windows(),human:input.orphans.human(),stage,stageName,kind,directory,spare,say,policy:contain,explain:lines=>input.mirror?.(`${stageName}-containment`,lines)});
 let record=readInflightV1(inflightFile);
 if(record!==undefined)watch.stage=record.stage;
 if(record!==undefined&&!stageProcessAliveV1(directory,record)&&!existsSync(record.exitFile)){
  const physicalEnd=readyPhysicalEndV1(record,Date.now());
  input.mirror?.(record.stage,mirrorLinesV1(record.log),ownerOf(record.stage,record.startedMs));
  let stop:ReadyStageStopV1|undefined,evidenceAtStop:ReadyStopEvidenceV1={};
  if(sameReadyFlightV1(readInflightV1(inflightFile),record)){
   await sweepRosters();
   const held=readInflightV1(inflightFile);
   if(held!==undefined&&sameReadyFlightV1(held,record)){
    stop=readyRecordedStageStopV1(held);
    evidenceAtStop=readyRecordedStopEvidenceV1(held,stop);
    rmSync(inflightFile,{force:true});
   }
  }
  const failure=failed("the process of the stage vanished without an exit status / der Prozess der Stufe verschwand ohne Exit-Status",false);
  const observed=restoredWatchdogV1(record,Date.now(),input.tree??readyScaledTreePolicyV1(speed));
  const evidence=toolFailureEvidenceV1(record.stage,record.log);
  return {...failure,...readyTimingWatchdogV1(observed?.lastActivity,evidenceAtStop),timedOut:stop==="timeout",cancelled:stop==="cancel",stalled:readyStoppedForStallV1(stop),startedMs:record.startedMs,physicalEnd,peakMegabytes:observed?.peakMegabytes||null,tail:[...failureLinesV1(record.log,P.tailLines-failure.tail.length,P.tailCharacters,evidence),...failure.tail],failure:[...failureLinesV1(record.log,P.mirrorLines-failure.tail.length,P.mirrorLineCharacters,evidence),...failure.tail],containment:{reaped:earlier.reaped,survivors:earlier.survivors}};
 }
 if(record===undefined){await sweepRosters();const swept=await reapRegistry(null,"registry");earlier.reaped.push(...swept.reaped);earlier.survivors.push(...swept.survivors);record=readInflightV1(inflightFile);if(record!==undefined)watch.stage=record.stage;}
 const wait=async(inflight:ReadyInflightV1,fresh=false):Promise<ReadyRunResultV1>=>new Promise<ReadyRunResultV1>((finish,fail)=>{
  input.onStage?.(inflight.stage,!fresh);
  const owns=()=>sameReadyFlightV1(readInflightV1(inflightFile),inflight);
  const stage=watch.stage===input.stage.name?input.stage:READY_STAGES_V1.find(row=>row.name===watch.stage)??input.stage;
  const stageTimeout=scaledMillisecondsV1(stage.timeoutMilliseconds,speed);
   const owner=ownerOf(inflight.stage,inflight.startedMs),mirror=()=>input.mirror?.(inflight.stage,mirrorLinesV1(inflight.log),owner);
   if(!fresh)prepareOwnedSliceReportV1(input.root,owner);
  const stop=readyRecordedStageStopV1(inflight);
  let evidenceAtStop=readyRecordedStopEvidenceV1(inflight,stop);
  let timedOut=stop==="timeout",cancelled=stop==="cancel",stalled=readyStoppedForStallV1(stop),tick=0,last="",killed=stop!==undefined,known:readonly ReadyKnownV1[]=readyReadTreeV1(directory,inflight.stage)?.processes??[],sampling=false,silentSampled=false,codeFile:string|undefined,codeStale=false,codeStaleWaiting=false,codeCheckedAt=0,workedAt=Date.now(),waitingNow=false,finishing=false,reaping=false,reapedAt=0,nextSample=0,sampledAt=Date.now(),watching=true;
  const treePolicy=input.tree??readyScaledTreePolicyV1(speed),observedAt=Date.now(),prior=restoredWatchdogV1(inflight,observedAt,treePolicy);
  const olderCode=inflight.code===undefined?inflight.supervisor!==undefined&&inflight.supervisor!==process.pid:inflight.code!==(input.code??readyModuleCodeV1()),workTracker=readyStallTrackerV1(treePolicy),semanticsAtStart=prior?.semantics??readySemanticsV1(input.root,treePolicy,sha),tracker=readyStallTrackerV1(treePolicy,prior?.seen),graphCounter=readyGraphCounterTrackerV1(treePolicy,prior?.graphCounter),stepWatch=readyStepWatchV1(treePolicy,prior?.step),holder=readyHolderWaitTrackerV1(treePolicy,prior?.holderSince);
  const needsProbe=!fresh&&(prior===undefined||prior.armedAt===inflight.startedMs&&(windows?prior.windows?.cpu.length??0:prior.cpu.length)===0);
  const armedAt=needsProbe?observedAt+2*P.samplerMilliseconds+P.pollMilliseconds:prior?.armedAt??inflight.startedMs;
  let awaitingProbe=observedAt<armedAt;
  const stageClaim=(()=>{try{return input.environment.SEMIO_FLEET_STORE_ROOT?createReadyStageClaimPublisherV1({directory:hostCapacityDirectoryV1(input.environment),stage:inflight.stage,pid:inflight.pid,totalMegabytes:Math.round(totalmem()/1048576),runs:()=>{try{return(JSON.parse(readFileSync(join(directory,P.timingsName),"utf8")) as {runs:ReadyTimingV1[]}).runs;}catch{return[];}}}):undefined;}catch{return undefined;}})();
  stageClaim?.observe(0);
  let peak=prior?.peakMegabytes??0,progressKey=prior?.progressKey??null,progressAt=prior?.progressAt??inflight.startedMs,cpuSince=prior?.cpuSince,stallAt=prior?.stallAt??inflight.startedMs,unwatched=prior===undefined?0:prior.unwatchedMilliseconds+Math.max(0,observedAt-prior.watchedAt),
   phase=prior?.phase??null,phaseAt=prior?.phaseAt??inflight.startedMs,phaseOffset=prior?.offset??0,holderEvidence=prior?.holderEvidence??null,
   lastActivity=prior?.lastActivity,cpuBefore:ReadonlyMap<number,number>=new Map(prior?.cpu),windowsBefore=windows?prior?.windows:undefined,dirty=prior===undefined||needsProbe,lastWrite=0;
  if(prior===undefined&&!fresh){
   const recent=analysisTailV1(inflight.log),key=readyProgressKeyV1(recent.slice(-P.tailLines)),fed=logSinceV1(inflight.log,0);
   lastActivity=readyOwnedActivityV1(fed.lines,lastActivity);
   phase=readyStagePhaseV1(fed.lines,phase,treePolicy);phaseOffset=fed.offset;progressKey=key===null?null:sha(key);
   tracker.observe(recent);graphCounter.observe(recent);
   progressAt=stallAt=readyAdoptionClockV1({startedMs:inflight.startedMs,observedMs:observedAt});
   stepWatch.observe(recent,stallAt);
  }
  let reportedActivityAt=stallAt;
  const checkpoint=(force=false):void=>{
   if(!watching||!force&&Date.now()-lastWrite<(dirty?P.watchdogWriteMilliseconds:P.watchdogIdleWriteMilliseconds))return;
   lastWrite=Date.now();
   try{
    const held=readInflightV1(inflightFile);
    if(held===undefined||!sameReadyFlightV1(held,inflight))return;
    const watchdog:ReadyWatchdogV1={version:1,owner:watchdogOwnerV1(inflight),peakMegabytes:peak,progressAt,...cpuSince===undefined?{}:{cpuSince},stallAt,watchedAt:Date.now(),unwatchedMilliseconds:unwatched,armedAt,progressKey,phase,phaseAt,offset:phaseOffset,seen:tracker.snapshot(),step:stepWatch.snapshot(),holderSince:holder.snapshot(),holderEvidence,semantics:semanticsAtStart,...graphCounter.snapshot()<0?{}:{graphCounter:graphCounter.snapshot()},cpu:cpuBefore.size<=P.watchdogCpuMaximum?[...cpuBefore]:[],...windowsBefore===undefined?{}:{windows:windowsBefore},...lastActivity===undefined?{}:{lastActivity},...toolsPaths===undefined?{}:{toolsGrace}};
    const text=JSON.stringify({...held,watchdog});
    if(Buffer.byteLength(text)>P.watchdogRecordBytes)return;
    const temporary=inflightFile+"."+process.pid+".tmp";
    try{writeFileSync(temporary,text);renameSync(temporary,inflightFile);dirty=false;}finally{rmSync(temporary,{force:true});}
    if(!killed&&!finishing&&!input.signal.aborted&&stallAt>reportedActivityAt){reportedActivityAt=stallAt;input.onActivity?.();}
   }catch{}
  };
  const moved=(now=Date.now()):void=>{stallAt=now;progressAt=now;unwatched=0;cpuSince=undefined;dirty=true;};const movedByCpu=(now=Date.now()):void=>{const since=cpuSince??now;moved(now);cpuSince=since;workedAt=now;};let linesAt=Date.now();let capacityWaitSince:number|null=null;let staleWaitingCounter=-1;let waitedSeeded=false;let sliceSeen="";let tickedStepSeconds=-1;
  const holderLines=treePolicy.holderWaitPatterns.map(pattern=>new RegExp(pattern,"u")),left=new Map<number,ReadyKnownV1>(earlier.survivors.map(row=>[row.pid,row])),reaped:ReadyReapedV1[]=[...earlier.reaped];
  let pinnedRoot=readyPinStageRootV1(inflight,[],readyReadTreeV1(directory,inflight.stage)?.root);
  const rootOf=():ReadyKnownV1=>pinnedRoot;
   const toolsEligible=stage.name==="tools"&&stage.kind==="repo"&&JSON.stringify(stage.arguments)==='["tools","setup","all"]'&&
    readyPhysicalLaunchV1(inflight)!==undefined&&treePolicy.hardCapMultiplier===4&&!Object.hasOwn(treePolicy.stageHardCapMultipliers,"tools");
   const toolsPaths=toolsEligible?toolsProgressPaths(input.root,input.orphans?.registryRoot??readyRegistryRootV1(input.environment,process.platform,home)):undefined;
   let toolsGrace=prior?.toolsGrace??null;
   const observeToolsGrace=(table:readonly ReadyProcessV1[]):void=>{
    const now=Date.now();
    if(toolsPaths===undefined||now-inflight.startedMs<4*stageTimeout-treePolicy.stallMilliseconds)return;
    const value=readyIdentityV1(rootOf(),table)==="same"?sampleToolsProgress({...toolsPaths,owner,started:inflight.startedMs,now,
     tolerance:contain.edgeToleranceMilliseconds,members:known.filter(row=>row.pid!==inflight.pid)}):undefined;
    toolsGrace=recordToolsProgress(toolsGrace,value,now);dirty=true;
   };
  checkpoint();
  let overdueStep:Readonly<ReadyStepV1&{limit:number;quiet:number}>|undefined,aliveAt=Date.now(),confirming=false,killReason:ReadyStageStopV1=stop??"stall",inspecting=false,inspectedAt=0,silentHolder:Readonly<{pid:number;command:string;seconds:number}>|undefined;const yielded=new Set<number>();
  const recorder=(reason:ReadyKillReasonV1)=>readyKillRecorderV1({directory,stage:inflight.stage,owner:ownerOf(inflight.stage,inflight.startedMs),reason,seconds:Math.round(treePolicy.stallStopMilliseconds/1000),say,policy:contain});
  const progressEvery=Math.max(1,Math.round(P.progressMilliseconds/P.pollMilliseconds)),sampleEvery=Math.max(1,Math.round(P.samplerMilliseconds/P.pollMilliseconds)),mirrorEvery=Math.max(1,Math.round(P.mirrorMilliseconds/P.pollMilliseconds)),diskEvery=Math.max(1,Math.round(P.diskGuard.checkMilliseconds/P.pollMilliseconds)),imageEvery=Math.max(1,Math.round(READY_IMAGE_POLICY_V1.checkMilliseconds/P.pollMilliseconds));
  let settling:Promise<void>|undefined,diskHit:ReadyDiskGuardHitV1|undefined,collecting:Promise<void>|undefined,collectedAt=0,floorSince:number|null=null,floorCollectedAt=0;
  const settle=():Promise<void>=>settling??=(async()=>{
   if(!owns())return;
   const result=await readyStopV1({root:rootOf(),known,policy:contain,grace:killed?P.graceMilliseconds:contain.terminateGraceMilliseconds,spare,record:recorder(killed?killReason:"settle"),...windows?{}:{group:inflight.pid}});
   for(const row of result.survivors)left.set(row.pid,row);
   if(!owns())return;
   if(result.survivors.length===0){if(result.verified)readyForgetTreeV1(directory,inflight.stage);return;}
   readyWriteTreeV1(directory,inflight.stage,rootOf(),result.survivors,Date.now());
   for(const row of result.survivors)say(readyContainmentLineV1("survivor",{pid:row.pid,name:row.name??"?",passes:result.passes}));
  })();
  const kill=(reason:ReadyStageStopV1,evidence:ReadyStopEvidenceV1={}):boolean=>{
   if(killed||!watching)return false;
   checkpoint(true);
   const held=readInflightV1(inflightFile);
   if(held===undefined||!sameReadyFlightV1(held,inflight))return false;
   const stopBy={supervisor:process.pid,reason,atMs:Date.now()};
   if(held.launch!==undefined)try{writeReadyLocalRecordV1(inflightFile,{...held,stop:reason,...evidence,stopBy});}catch(error){failWatch(error);return false;}
   else try{writeReadyLocalRecordV1(inflightFile,{...held,stopBy});}catch{}
   killed=true;killReason=reason;evidenceAtStop=evidence;void settle().catch(failWatch);return true;
  };
  const readRecord=():ReadyReadRecordV1|undefined=>{try{const value=JSON.parse(readFileSync(machinePathV1(input.root,...treePolicy.readRecord),"utf8")) as Partial<ReadyReadRecordV1>;return Number.isSafeInteger(value.runner)&&Number.isFinite(value.startedMs)&&(value.reason==="request"||value.reason==="refresh")?{reason:value.reason,runner:value.runner!,read:value.read??null,startedMs:value.startedMs!,status:value.status??null,endedMs:value.endedMs??null}:undefined;}catch{return undefined;}};
  const inspect=async(named:ReadyHolderWaitV1,waiting:number):Promise<void>=>{
   const table=await readyReadTableV1(),entries=input.orphans===undefined?[]:readyRegistryEntriesV1(input.orphans.registryRoot,contain),entry=entries.filter(row=>row.pid===named.pid).sort((left,right)=>right.startedAtMs-left.startedAtMs)[0],verdict=readyHolderCustodyV1({wait:named,entry,table,nowMs:Date.now(),policy:treePolicy});
   if(killed||!watching||!owns())return;
   silentHolder=verdict.kind==="silent"?{pid:named.pid,command:named.command,seconds:Math.round(verdict.silentMs/1000)}:undefined;
   if(verdict.kind==="progressing"||verdict.kind==="gone"){
    const evidence=sha(JSON.stringify([named.pid,named.since,verdict.kind,verdict.kind==="gone"?null:entry?.updatedAtMs??named.seconds]));
    if(evidence!==holderEvidence){holderEvidence=evidence;moved();}
   }
   if(verdict.kind!=="progressing"&&verdict.kind!=="silent"||waiting<treePolicy.holderYieldMilliseconds||yielded.has(named.pid))return;
   const reader=readyBackgroundReaderV1({record:readRecord(),holder:named.pid,table,self:process.pid,policy:treePolicy});
   if(reader===undefined)return;
   yielded.add(named.pid);
   const seconds=Math.round(waiting/1000),record=readyKillRecorderV1({directory,stage:inflight.stage,owner:ownerOf(inflight.stage,inflight.startedMs),reason:"yield",seconds,say,policy:contain});
   say(readyContainmentLineV1("yielded-reader",{stage:inflight.stage,seconds,pid:named.pid,name:named.command.slice(0,80),reason:reader.reason}));
   await readyKillTreeV1({root:reader.root,known:[],signal:"SIGTERM",table,spare:new Set<number>(),record});
   setTimeout(()=>{void readyKillTreeV1({root:reader.root,known:[],signal:"SIGKILL",spare:new Set<number>(),record});},P.graceMilliseconds).unref();
  };
   const done=(result:ReadyRunResultV1):void=>{if(result.detached)checkpoint(true);else mirror();watching=false;clearInterval(timer);input.signal.removeEventListener("abort",detach);const full=result.detached?result:{...result,containment:{reaped:[...reaped],survivors:[...left.values()]},...silentHolder!==undefined&&result.stalled?{holderSilent:silentHolder}:{},...overdueStep!==undefined&&result.stalled?{stepOverdue:overdueStep}:{},...result.stalled?{progressed:stallAt>inflight.startedMs}:{}};finish(codeFile===undefined||result.detached?full:{...full,codeChanged:codeFile,...codeStale?{codeStale:true}:{},...codeStaleWaiting?{codeStaleWaiting:true}:{}});};
  const watchedMs=Date.now(),stamp=(milliseconds:number):string=>new Date(milliseconds).toISOString();
  const provenance=():string=>{const started=inflight.supervisor,same=started===process.pid;return same?`stage ${inflight.stage} (process ${inflight.pid}) started ${stamp(inflight.startedMs)} and watched by supervisor ${process.pid} / Stufe ${inflight.stage} (Prozess ${inflight.pid}) gestartet ${stamp(inflight.startedMs)} und von Aufseher ${process.pid} überwacht`:`stage ${inflight.stage} (process ${inflight.pid}) started ${stamp(inflight.startedMs)} by supervisor ${started??"?"}, adopted by supervisor ${process.pid} at ${stamp(watchedMs)} / Stufe ${inflight.stage} (Prozess ${inflight.pid}) gestartet ${stamp(inflight.startedMs)} von Aufseher ${started??"?"}, übernommen von Aufseher ${process.pid} um ${stamp(watchedMs)}`;};
  const ending=(signal:string|null,why?:string,extra:readonly string[]=[]):Readonly<{tail:string[];failure:string[]}>=>{const note=[...signal!==null?[exitSignalLineV1(signal)]:why===undefined?[]:[why],...extra],evidence=toolFailureEvidenceV1(inflight.stage,inflight.log),space=readyNoSpaceEvidenceV1(inflight.log,P.noSpacePatterns);return{tail:[...failureLinesV1(inflight.log,P.tailLines-note.length,P.tailCharacters,evidence),...failedTaskTailLinesV1(inflight.log,P.failedTaskSummaryLines),...space,...note],failure:[provenance(),...failureLinesV1(inflight.log,P.mirrorLines-1-note.length,P.mirrorLineCharacters,evidence,true),...failedTaskTailLinesV1(inflight.log),...space,...note]};};
  const snapshot=()=>detachedReadyStageV1(inflight.startedMs,logTailV1(inflight.log),peak>0?peak:null);
  watch.snapshot=snapshot;
  const detach=():void=>{if(!killed&&!finishing)done(snapshot());};
  const failWatch=(error:unknown):void=>{watching=false;clearInterval(timer);input.signal.removeEventListener("abort",detach);fail(error);};
  const complete=(result:ReadyRunResultV1,containTree:boolean,endedMs?:number):void=>{
   finishing=true;stageClaim?.clear();
   if(!killed&&owns())lastActivity=readyOwnedActivityV1(logSinceV1(inflight.log,phaseOffset).lines,lastActivity);
   result={...result,...diskHit===undefined?{}:{diskGuard:diskHit,tail:[...result.tail,readyDiskGuardLineV1(diskHit)]},...readyTimingWatchdogV1(lastActivity,evidenceAtStop),physicalEnd:readyPhysicalEndV1(inflight,Date.now(),endedMs)};
   const publish=()=>{
    try{
     if(owns()){
      if(input.completion!==undefined&&readyCleanExitV1(result)){
       const attribution=endedMs===undefined||endedMs<inflight.startedMs||readInflightV1(inflightFile)?.stop!==undefined?"unproven":input.completion.apply(inflight,endedMs,result);
       result={...result,attribution};
      }
      if(result.exit===0&&!containTree)readyForgetTreeV1(directory,inflight.stage);
      rmSync(inflightFile,{force:true});rmSync(inflight.exitFile,{force:true});
     }
     done(result);
    }catch(error){failWatch(error);}
   };
   if(!containTree){publish();return;}
   void settle().then(publish,failWatch);
  };
   const sampleTree=(maximum?:number):void=>{sampling=true;void readyReadTableV1(maximum,undefined,undefined,windows||maximum!==undefined?input.signal:undefined).then(table=>{
    sampling=false;if(maximum!==undefined)probeUntil=0;nextSample=Date.now()+readySampleDelayV1(Date.now()-inflight.startedMs,contain);
    if(killed||!watching||finishing||!owns())return;
    pinnedRoot=readyPinStageRootV1(inflight,table,pinnedRoot);
    const root=rootOf(),tolerance=treePolicy.toleranceMilliseconds,identity=readyIdentityV1(root,table,tolerance);
    if(maximum!==undefined&&identity!=="same"){toolsGrace=recordToolsProgress(toolsGrace,undefined,Date.now());dirty=true;}
    if(table.length===0)return;
    if(identity==="reused"&&!existsSync(inflight.exitFile)&&!confirming){
     confirming=true;const first=table.find(row=>row.pid===root.pid)!,confirmationBegan=Date.now();
     void readyReadTableV1(undefined,undefined,undefined,input.signal).then(second=>{
      confirming=false;if(!watching||finishing||killed||!owns()||existsSync(inflight.exitFile))return;
      const again=readyConfirmProcessReplacementV1(root,first,second,tolerance);if(again===undefined)return;
      const why=changedLineV1(root.pid,root.startMs,again.startMs,first.startMs,tolerance,Date.now()-confirmationBegan);
      complete({exit:null,signal:null,timedOut,spawnFailed:false,cancelled,stalled,detached:false,startedMs:inflight.startedMs,peakMegabytes:peak>0?peak:null,...ending(null,why)},true);
     });return;
    }
    known=readyTreeMembersV1(table,root,[...known,...readyHeirsV1({table,previous:[root,...known],sinceMs:sampledAt})],tolerance);sampledAt=Date.now();observeToolsGrace(table);readyWriteTreeV1(directory,inflight.stage,root,known,sampledAt);
     if(windows&&!existsSync(cancelFile)){
      const sample=readyWindowsTreeSampleV1({table,root:root.pid,members:known,previous:windowsBefore,maximum:P.watchdogCpuMaximum,minimumCpuMilliseconds:treePolicy.livenessCpuMilliseconds});
      if(sample!==undefined){
       windowsBefore=sample.checkpoint;
       if(sample.progress)movedByCpu();
       if(sample.megabytes!==null)stageClaim?.observe(sample.megabytes,holder.snapshot()!==null);
       const peakChanged=sample.megabytes!==null&&sample.megabytes>peak;
       if(peakChanged)peak=sample.megabytes!;
       dirty||=sample.changed||peakChanged;
       checkpoint(peakChanged);
      }
     }
    });};
   const probeNow=Date.now(),probeLimit=2*P.pollMilliseconds;
   let probeUntil=!fresh&&!killed&&toolsPaths!==undefined&&probeNow-progressAt<=treePolicy.stallMilliseconds&&
    toolsGraceActive(toolsGrace,{stage:stage.name,started:inflight.startedMs,now:probeNow,soft:stageTimeout,quiet:treePolicy.stallMilliseconds,sampleLimit:treePolicy.stallMilliseconds})?
    Math.min(probeNow+probeLimit,inflight.startedMs+5*stageTimeout):0;
   if(probeUntil>probeNow)sampleTree(probeUntil-probeNow);
  const stopSenderV1=(ended:string,endedAt:number,received:readonly {signal:string;atMs:number}[]|undefined,sent:readonly {signal:string;reason:string;atMs:number}[]|undefined)=>{
   let kills:ReturnType<typeof readyKillRowsV1>=[],warden:Parameters<typeof readyStopSenderV1>[0]["warden"];
   try{const file=join(directory,contain.killRecordName),size=statSync(file).size,fd=openSync(file,"r");try{const length=Math.min(size,treePolicy.senderRecordBytes),buffer=Buffer.alloc(length);readSync(fd,buffer,0,length,size-length);kills=readyKillRowsV1(buffer.toString("utf8"),inflight.startedMs);}finally{closeSync(fd);}}catch{}
   try{const row=readyWardenCancelOfStageV1(readHostWardenCancelsSinceV1(inflight.startedMs),inflight.pid);if(row!==undefined)warden={atMs:row.atMs,kind:row.kind,reason:row.reason,megabytes:row.megabytes};}catch{}
   const stopBy=readInflightV1(inflightFile)?.stopBy;
   return readyStopSenderV1({ended,stagePid:inflight.pid,endedMs:endedAt,observer:process.pid,kills,...stopBy===undefined?{}:{stopBy},...received===undefined?{}:{received},...sent===undefined?{}:{sent},...warden===undefined?{}:{warden}},treePolicy);
  };
  const wardenEnd=():Pick<ReadyRunResultV1,"wardenCancel">=>{try{const row=readyWardenCancelOfStageV1(readHostWardenCancelsSinceV1(inflight.startedMs),inflight.pid);return row===undefined?{}:{wardenCancel:{reason:row.reason,kind:row.kind,pid:row.pid,megabytes:row.megabytes}};}catch{return{};}};
  const timer=setInterval(()=>{
   tick++;
   if(finishing)return;
   if(!killed&&tick%diskEvery===0){const path=machinePathV1(input.root,".cache"),hit=readyDiskGuardV1(path,(input.facts??readyFilesystemFactsV1)(path));if(hit!==null&&kill("cancel")){diskHit=hit;say(readyDiskGuardLineV1(hit));}}
   if(!killed&&tick%imageEvery===0){
    const images=readyImageFactsV1(input.root,inflight.stage,input.facts??readyFilesystemFactsV1),verdict=readyImageVerdictV1(images,{belowSinceMs:floorSince,collecting:collecting!==undefined,collectedAtMs:floorCollectedAt},Date.now(),input.collectImage!==undefined);
    if(verdict.hit===null)floorSince=null;else floorSince??=Date.now();
    if(verdict.action==="stop"){const hit=verdict.hit!;if(kill("cancel")){diskHit=hit;say(readyDiskGuardLineV1(hit));say(`image ${hit.path} can still grow by ${verdict.growableMegabytes} MB: the next start grows it before the stage runs again / Image ${hit.path} kann noch um ${verdict.growableMegabytes} MB wachsen: der nächste Start vergrößert es, bevor die Stufe wieder läuft`);}}
    else if(verdict.action==="collect"&&collecting===undefined){
     const image=verdict.image!,floorRow=readyImageRowV1(image,readyImageNeedBytesV1(image.totalBytes));
     collectedAt=Date.now();say(`image ${floorRow.path} has ${floorRow.available} MB free of ${floorRow.total} MB, below its floor of ${verdict.hit!.floor} MB: collecting inside it before the stage is stopped (it can still grow by ${verdict.growableMegabytes} MB at the next start) / Image ${floorRow.path} hat ${floorRow.available} MB frei von ${floorRow.total} MB, unter der Grenze von ${verdict.hit!.floor} MB: darin wird gesammelt, bevor die Stufe gestoppt wird (es kann beim nächsten Start noch um ${verdict.growableMegabytes} MB wachsen)`);
     collecting=input.collectImage!(floorRow,input.signal).catch(()=>{}).finally(()=>{collecting=undefined;floorCollectedAt=Date.now();});
    }
    else if(verdict.hit===null&&input.collectImage!==undefined&&collecting===undefined&&Date.now()-collectedAt>=READY_IMAGE_POLICY_V1.reliefRetryMilliseconds){
     const low=readyImageReliefV1(images);
     if(low!==null){collectedAt=Date.now();say(`image ${low.path} has ${low.available} MB free of ${low.total} MB, below its low-water mark of ${low.needed} MB: collecting inside it while the stage runs / Image ${low.path} hat ${low.available} MB frei von ${low.total} MB, unter der Untergrenze von ${low.needed} MB: während die Stufe läuft wird darin gesammelt`);collecting=input.collectImage(low,input.signal).catch(()=>{}).finally(()=>{collecting=undefined;});}
    }
   }
   if(existsSync(inflight.exitFile)){
    let exit:number|null=null,spawnFailed=false,signal:string|null=null,endedMs:number|undefined,received:readonly {signal:string;atMs:number}[]|undefined,sent:readonly {signal:string;reason:string;atMs:number}[]|undefined;
     try{
      const row=JSON.parse(readFileSync(inflight.exitFile,"utf8")) as {exit:number|null;signal?:string|null;spawnFailed:boolean;endedMs?:number;received?:{signal:string;atMs:number}[];sent?:{signal:string;reason:string;atMs:number}[]};
      exit=row.exit;spawnFailed=row.spawnFailed;signal=row.signal??null;
      if(!validateJsonSchemaSubset(schema.definitions.Exit,row,schema).length){endedMs=row.endedMs;received=row.received;sent=row.sent;}
     }catch{}
    const ended=exit===0?null:signal??(exit!==null&&(P.externalStopExits as readonly number[]).includes(exit)?`exit status ${exit}`:null);
    const sender=ended===null?undefined:stopSenderV1(ended,endedMs??Date.now(),received,sent);
    const recordedStop=sender?.kind==="supervisor"?readInflightV1(inflightFile)?.stopBy:undefined,stoppedBy=!killed&&signal!==null?recordedStop:undefined;
    if(stoppedBy!==undefined){timedOut=stoppedBy.reason==="timeout";cancelled=stoppedBy.reason==="cancel";stalled=readyStoppedForStallV1(stoppedBy.reason);killed=true;}
    return complete({exit,signal,timedOut,spawnFailed,cancelled,stalled,detached:false,startedMs:inflight.startedMs,peakMegabytes:peak>0?peak:null,...exit===0||killed?{}:wardenEnd(),...sender===undefined||sender.kind==="none"?{}:{sender:sender.kind},...recordedStop===undefined?{}:{stopBy:recordedStop},...exit===0?{tail:logTailV1(inflight.log)}:ending(signal,undefined,sender===undefined||sender.kind==="none"?[]:[sender.line])},killed||signal!==null,endedMs);
   }
   if(!alive(inflight.pid)){
    if(existsSync(inflight.exitFile))return;
    const why=vanishedLineV1(inflight.pid,Math.round((Date.now()-aliveAt)/1000),killed);
    return complete({exit:null,signal:null,timedOut,spawnFailed:false,cancelled,stalled,detached:false,startedMs:inflight.startedMs,peakMegabytes:peak>0?peak:null,...ending(null,why)},true);
   }
   aliveAt=Date.now();
    if(tick%mirrorEvery===0)mirror();
    if(tick%mirrorEvery===0&&!killed){
     let held=false;try{held=readyWardenHoldsV1(currentHostWardenMirrorV1(undefined,Date.now()) as never,new Set([inflight.pid,...known.map(member=>member.pid)]));if(held)moved();}catch{}
     try{const slot=ownedSliceReportV1(input.root,ownerOf(inflight.stage,inflight.startedMs));if(slot!==undefined&&statSync(slot.file).size<=P.sliceReport.fileBytes){const report=JSON.parse(readFileSync(slot.file,"utf8"));if(!silentSampled&&!killed&&phase==="reading"&&readySliceSampleDueV1(report,Date.now(),inflight.startedMs)&&alive(report.pid)){silentSampled=true;void readySliceSampleV1({pid:report.pid,silentSeconds:Math.round((Date.now()-report.atMs)/1000)}).then(lines=>{if(lines.length>0)input.mirror?.(inflight.stage+"-sample",lines);});}const sliceSignature=readySliceMovedSignatureV1(report,inflight.startedMs);if(sliceSignature!==null&&sliceSignature!==sliceSeen){sliceSeen=sliceSignature;moved();}if(readyGrantWaitV1(report,Date.now(),treePolicy))moved();else if(!held&&phase==="reading"&&Date.now()>=armedAt&&Date.now()-linesAt>=treePolicy.sliceSilentMilliseconds&&readySliceSilentV1(report,Date.now(),inflight.startedMs,treePolicy)){stalled=true;kill("stall");}}}catch{}
    }
   const probeEnded=awaitingProbe&&Date.now()>=armedAt;if(probeEnded)awaitingProbe=false;
   if(tick%progressEvery===0||!fresh&&tick===1||probeEnded){const recent=analysisTailV1(inflight.log),line=formatLogLineV1(recent.at(-1)??"",P.tailCharacters),raw=readyProgressKeyV1(recent.slice(-P.tailLines)),key=raw===null?null:sha(raw);if(key!==null&&key!==progressKey){progressKey=key;progressAt=Date.now();dirty=true;}if(line!==""&&line!==last){last=line;input.onLine(line);}if([tracker.observe(recent),graphCounter.observe(recent)].some(Boolean)){linesAt=Date.now();moved();}if([workTracker.observe(recent.filter(text=>!readyWaitLineV1(text,treePolicy))),graphCounter.snapshot()>staleWaitingCounter].some(Boolean)){workedAt=Date.now();staleWaitingCounter=graphCounter.snapshot();}waitingNow=readyWaitLineV1(recent.at(-1)??"",treePolicy);if(readyCapacityWaitingV1(recent,treePolicy)){capacityWaitSince??=Date.now();}else capacityWaitSince=null;if(waitingNow&&!waitedSeeded){waitedSeeded=true;const waited=readyWaitedMillisecondsV1(recent.at(-1)??"");if(waited!==null)workedAt=Math.min(workedAt,Date.now()-waited);}const since=holder.snapshot(),waiting=holder.observe(recent,Date.now()),fed=logSinceV1(inflight.log,phaseOffset),seenPhase=readyStagePhaseV1(fed.lines,phase,treePolicy);if(phaseOffset!==fed.offset||since!==holder.snapshot())dirty=true;phaseOffset=fed.offset;if(!killed)lastActivity=readyOwnedActivityV1(fed.lines,lastActivity);if(!killed&&!inspecting&&holder.snapshot()!==null&&Date.now()-inspectedAt>=treePolicy.holderCheckMilliseconds){const named=readyHolderWaitOfV1(recent,treePolicy);if(named!==undefined){inspecting=true;inspectedAt=Date.now();void inspect(named,waiting).catch(()=>undefined).finally(()=>{inspecting=false;});}}if(seenPhase!==phase){if(phase!==null||seenPhase!=="reading")phaseAt=Date.now();phase=seenPhase;dirty=true;}const watched=stepWatch.observe(recent,Date.now());if(!killed&&watched.step!==undefined&&watched.step.seconds<=watched.step.bound&&watched.step.seconds>tickedStepSeconds){tickedStepSeconds=watched.step.seconds;moved();}else if(watched.step===undefined)tickedStepSeconds=-1;if(!killed&&Date.now()>=armedAt&&phase!=="executing"&&watched.overdue&&watched.step!==undefined){
     const {seconds,bound}=watched.step,limit=readyStepOverdueSecondsV1(bound,treePolicy),stepOverdue={seconds,bound,limit,quietMs:watched.quietMs};
     if(kill("overdue",readyValidStepEvidenceV1(stepOverdue)?{stepOverdue}:{})){overdueStep={...watched.step,limit,quiet:Math.round(watched.quietMs/1000)};stalled=true;}
    }if(!killed&&watching&&!reaping&&input.orphans!==undefined&&Date.now()-reapedAt>=contain.holderReapMilliseconds&&recent.some(line=>holderLines.some(expression=>expression.test(line)))&&owns()){reaping=true;reapedAt=Date.now();void reapRegistry({root:rootOf(),known},"holder",inflight.stage).then(swept=>{reaped.push(...swept.reaped);for(const row of swept.reaped)left.delete(row.pid);for(const row of swept.survivors)left.set(row.pid,row);}).finally(()=>{reaping=false;});}const codeVerdict=():ReturnType<typeof readyCodeStopV1>=>readyCodeStopV1({holderWaitMs:waiting,phase,phaseAgeMs:Date.now()-phaseAt,quietMs:Date.now()-progressAt,policy:treePolicy});if(!killed&&codeVerdict()!==null&&Date.now()-codeCheckedAt>=treePolicy.codeCheckMilliseconds){codeCheckedAt=Date.now();const why=codeVerdict(),file=readySemanticsChangedV1(input.root,semanticsAtStart,treePolicy,sha);if(file!==null&&why!==null){codeFile=file;codeStale=why==="stale";stalled=true;kill("restart");}}}
   if(Date.now()>=nextSample&&!sampling&&!killed)sampleTree();
   let peakChanged=false;
   if(!windows&&(tick%sampleEvery===0||!fresh&&tick===1)){const rows=spawnSync("ps",["-A","-o","pid=,ppid=,rss=,time="],{encoding:"utf8"});if(rows.status===0){const observedPeak=readyTreeMegabytesV1(rows.stdout,inflight.pid);if(observedPeak>peak){peak=observedPeak;peakChanged=dirty=true;}stageClaim?.observe(observedPeak,holder.snapshot()!==null);const cpu=readyTreeCpuV1(rows.stdout,inflight.pid,known.map(member=>member.pid));if(!killed&&readyLivenessV1(cpuBefore,cpu,treePolicy))movedByCpu();if(cpu.size!==cpuBefore.size||[...cpu].some(([pid,spent])=>cpuBefore.get(pid)!==spent))dirty=true;cpuBefore=cpu;}}
   checkpoint(peakChanged);
   if(!killed&&Date.now()>=armedAt&&readyStaleWaitingV1({older:olderCode,waiting:waitingNow,quietMs:Date.now()-workedAt,policy:treePolicy})){codeFile="code digest";codeStaleWaiting=true;stalled=true;kill("restart");}
   const capacityLive=readyCapacityWaitLiveV1({sinceMs:capacityWaitSince,logWrittenMs:(()=>{try{return statSync(inflight.log).mtimeMs;}catch{return null;}})(),nowMs:Date.now(),policy:treePolicy}),liveRead=capacityLive||readyReadingLiveV1({phase,record:phase==="reading"?readRecord():undefined,alive});
   if(!killed&&Date.now()>=armedAt&&(readyStalledV1(Date.now()-stallAt-unwatched,treePolicy,liveRead)||!liveRead&&readyCpuOnlyStalledV1(cpuSince,Date.now(),treePolicy))){stalled=true;kill("stall");}
   const elapsed=Date.now()-inflight.startedMs;
   if(elapsed>stageTimeout){
    const verdict=readyDeadlineV1({stage:stage.name,elapsedMs:elapsed,softMs:stageTimeout,sinceProgressMs:liveRead?0:Date.now()-progressAt,policy:treePolicy}),now=Date.now();
    const denial=verdict==="timeout"?toolsGraceDenial(toolsGrace,{stage:stage.name,eligible:toolsPaths!==undefined,progressing:now-progressAt<=treePolicy.stallMilliseconds,started:inflight.startedMs,now,
     soft:stageTimeout,quiet:treePolicy.stallMilliseconds,sampleLimit:2*contain.slowSampleMilliseconds+P.pollMilliseconds}):undefined;
    if(verdict==="timeout"&&denial!==undefined&&now>=probeUntil){
     timedOut=true;kill("timeout",{deadline:"hard-cap",...inflight.stage!=="tools"?{}:{toolsGraceDenial:denial}});
    }else if(verdict==="stalled"&&now>=armedAt){timedOut=true;kill("timeout",{deadline:"soft-cap-idle"});}
   }
   if(watching&&existsSync(cancelFile)&&owns()){cancelled=true;kill("cancel");if(killed)rmSync(cancelFile,{force:true});}
  },P.pollMilliseconds);
  input.signal.addEventListener("abort",detach,{once:true});
  if(input.signal.aborted)detach();
  if(killed)void settle().catch(failWatch);
 });
 if(record!==undefined)return wait(record);
 if(input.signal.aborted)return detachedReadyStageV1();
 const invocation=readyStageCommandV1(input.stage,input.root,home,input.environment);
 if(invocation===undefined)return failed("no repo binary below "+home+" / kein repo-Programm unter "+home,true);
 const launch=input.completion?.capture(input.stage,readyStageRecipeV1(input.stage,invocation));
 const {command,args,env}=invocation,startedMs=Date.now(),owner=ownerOf(input.stage.name,startedMs);
 env.CARGO_BUILD_JOBS=String(input.jobs);env.MAKEFLAGS="-j"+input.jobs;env[contain.ownerEnvironment]=owner;env[TERMINATION_POLICY_V1.ledgerEnvironment]=join(machinePathV1(input.root,...P.directory),contain.killRecordName);
 const log=join(directory,`${input.stage.name}.log`),exitFile=join(directory,`${input.stage.name}-${startedMs}-${process.pid}.exit`);
 rmSync(exitFile,{force:true});rmSync(cancelFile,{force:true});
 let child:ReturnType<typeof spawn>;
 try{renameSync(log,`${log.slice(0,-4)}${P.previousLogSuffix}`);}catch{}
 const handle=openSync(log,"w");
 if(lines.length>0)writeSync(handle,`${lines.join("\n")}\n`);
  const reportGrant=prepareOwnedSliceReportV1(input.root,owner);
  if(reportGrant!==undefined)env[READY_SLICE_REPORT_POLICY_V1.grantEnvironment]=reportGrant;
 try{child=spawn(process.execPath,[join(import.meta.dir,"🏃️wrapper","🟦️.ts"),...launch===undefined?[]:["--launch-record="+inflightFile],"--deadline="+readyAbsoluteDeadlineV1({stage:input.stage.name,startedMs,softMs:scaledMillisecondsV1(input.stage.timeoutMilliseconds,speed),policy:input.tree??readyScaledTreePolicyV1(speed)}),exitFile,command,...args],{cwd:input.root,env,stdio:["ignore",handle,handle],detached:true,windowsHide:true});}
 catch(error){closeSync(handle);return Promise.resolve(failed(String((error as Error).message).slice(0,P.tailCharacters),true));}
 closeSync(handle);
 child.on("error",()=>undefined);child.unref();
 if(child.pid===undefined)return Promise.resolve(failed(`${command} could not be started / ${command} konnte nicht gestartet werden`,true));
 const started:ReadyInflightV1={stage:input.stage.name,pid:child.pid,startedMs,exitFile,log,supervisor:process.pid,code:input.code??readyModuleCodeV1(),...launch===undefined?{}:{launch}};
 try{writeReadyLocalRecordV1(inflightFile,started);}catch(error){try{child.kill();}catch{}throw error;}
 return wait(started,true);
}

/** 🧮️ The free memory a stage needs to start on a host of `hostMegabytes`: the declared floor, never more than `memoryFloorShareBasisPoints` of the machine (a small worker must not wait for an absolute number it can hardly reach), never below `memoryFloorMinimumMegabytes`. A host of unknown size (0) keeps the declared floor. */
export function readyMemoryFloorMegabytesV1(hostMegabytes:number):number{
 if(!Number.isFinite(hostMegabytes)||hostMegabytes<=0)return P.memoryFloorMegabytes;
 return Math.min(P.memoryFloorMegabytes,Math.max(P.memoryFloorMinimumMegabytes,Math.floor(hostMegabytes*P.memoryFloorShareBasisPoints/10000)));
}

/** 🔌️ The real ports of a machine: the files below the worktree, the free memory by the memory gate of the graph (`fleetHostReadingV1` against `memoryFloorMegabytes`), the stage runner and `ready.json` in the store of the fleet. */
export function createReadyPortsV1(input:Readonly<{facts?(path:string):ReadyFilesystemFactsV1|null;root:string;environment:Readonly<Record<string,string|undefined>>;origin:string;group:string|undefined;home?:string;stages?:readonly ReadyStageV1[];work?:{busy:boolean;wake?:()=>void};say(line:string):void}>):ReadyPortsV1{
 const directory=machinePathV1(input.root,...P.directory),stateFile=join(directory,P.stateName),marker=machinePathV1(input.root,".cache","f",P.markerName),windows=process.platform==="win32";
 const status=createReadyStatusV1({store:input.environment.SEMIO_FLEET_STORE_ROOT,origin:input.origin});
 const inputHashes=new Map<string,{stamp:string;hash:string}>(),dependencyHashes=new Map<string,{stamp:string;hash:string}>();
 const stamp=(row:BigIntStats)=>[row.dev,row.ino,row.mode,row.size,row.mtimeNs,row.ctimeNs].join(":");
 const hashOf=(path:string,stage:ReadyStageV1):string|undefined=>{
   const absolute=join(input.root,...path.split("/")),dependencies=stage.dependencyInputs===true,hashes=dependencies?dependencyHashes:inputHashes;
   let fd:number|undefined;
  try{
    const row=statSync(absolute,{bigint:true}),before=stamp(row),cached=hashes.get(path);
    if(!row.isFile())throw Error("Regular readiness input required");
    if(cached?.stamp===before)return cached.hash;
    if(dependencies){
     const inside=relative(realpathSync(input.root),realpathSync(absolute));
     if(inside===".."||inside.startsWith(".."+sep)||isAbsolute(inside))throw Error("Readiness dependency input escapes the workspace");
    }
   fd=openSync(absolute,constants.O_RDONLY|(constants.O_NONBLOCK??0));
    if(before!==stamp(fstatSync(fd,{bigint:true})))throw Error("Readiness input changed before hashing");
    const hash=dependencies?path.endsWith("package.json")?readyManifestDigestV1(readFileSync(fd)):sha(readFileSync(fd)):readyInputHashV1(path,readFileSync(fd,"utf8"));
   if(before!==stamp(fstatSync(fd,{bigint:true}))||before!==stamp(statSync(absolute,{bigint:true})))throw Error("Readiness input changed during hashing");
    hashes.set(path,{stamp:before,hash});return hash;
   }catch{hashes.delete(path);return undefined;}
  finally{if(fd!==undefined)closeSync(fd);}
 };
 const recentRuns=():readonly ReadyTimingV1[]=>{try{return(JSON.parse(readFileSync(join(directory,P.timingsName),"utf8")) as {runs:ReadyTimingV1[]}).runs;}catch{return[];}};
 const owner=`${process.pid}:${Math.random().toString(16).slice(2)}`,jitter=parseInt(sha(input.origin).slice(0,8),16)/0x100000000;
  let lastSliceOwner:string|undefined,lastSliceReport:string|undefined;
 const plan=input.stages??readyStagesForRootV1(READY_STAGES_V1,input.root),platform=readyPlatformV1(),home=input.home??homedir(),lease=createReadyLeaseV1({directory,owner});
 const measure=(stages:readonly ReadyStageV1[])=>{
  const measured=stages.map(stage=>{
   if(!stage.dependencyInputs)return stage;
   const {manifest,lock}=schema.definitions.DependencyInputs.const;
   const paths=new Set([...stage.inputs,...javascriptPublishedLockInputsV1(input.root,manifest,lock)]);
   for(const key of dependencyHashes.keys())if(!paths.has(key))dependencyHashes.delete(key);
   return {...stage,inputs:[...paths].sort()};
  });
  return readyDigestsV1({platform,hash:hashOf,stages:measured});
 };
 const state:ReadyPortsV1["state"]={
  read:()=>{try{const value=JSON.parse(readFileSync(stateFile,"utf8")) as ReadyLocalStateV1;return value.version===1&&value.stages!==null&&typeof value.stages==="object"?value:emptyReadyStateV1();}catch{return emptyReadyStateV1();}},
  write:value=>writeReadyLocalRecordV1(stateFile,value),
 };
 const compiler=()=>{const seen=inspectTypescriptCompilerV1({root:input.root});return seen.ok||(seen.refusal.code==="typescript-compiler-declaration"&&seen.refusal.found===null&&!existsSync(seen.refusal.resolved))?null:{en:seen.refusal.en,de:seen.refusal.de};};
 const launchProof=(stage:string,recipe:string,current:ReadyLocalStateV1):ReadyStageLaunchV1|undefined=>{
  const index=plan.findIndex(row=>row.name===stage);
  if(index<0||index>64)return undefined;
   const digests=measure(plan.slice(0,index+1)).stages,prerequisites:ReadyStageLaunchV1["prerequisites"][number][]=[];
  for(const row of digests.slice(0,-1)){
   const completed=current.stages[row.name],proof=current.appliedCompletions?.[row.name];
   if(completed?.digest!==row.digest)return undefined;
   prerequisites.push({stage:row.name,digest:completed.digest,doneMs:completed.doneMs,...proof===undefined?{}:{completion:proof}});
  }
  const proof:ReadyStageLaunchV1={version:1,platform,digest:digests.at(-1)!.digest,recipe,prerequisites};
  return validateJsonSchemaSubset(schema.definitions.StageLaunch,proof,schema).length?undefined:proof;
 };
 const completion:ReadyStageCompletionV1={
  capture(stage,recipe){return lease.peek()==="own"?launchProof(stage.name,recipe,state.read()):undefined;},
  apply(flight,endedMs,result){
   const launch=flight.launch;
   if(launch===undefined||flight.stop!==undefined||!Number.isSafeInteger(endedMs)||endedMs>Date.now()||validateJsonSchemaSubset(schema.definitions.StageLaunch,launch,schema).length)return"unproven";
   if(lease.peek()!=="own")throw Error("The readiness lease is required to apply a stage completion");
   const stage=plan.find(row=>row.name===flight.stage),current=state.read(),id=watchdogOwnerV1(flight);
   if(stage===undefined)return"unproven";
   if(current.appliedCompletions?.[flight.stage]===id)return"known";
   const command=readyStageCommandV1(stage,input.root,home,input.environment),expected=command===undefined?undefined:launchProof(flight.stage,readyStageRecipeV1(stage,command),current);
   if(expected===undefined||!sameReadyLaunchV1(launch,expected)||stage.name==="native"&&readyRepoBinaryV1(home,input.environment)===undefined||stage.name==="dependencies"&&compiler()!==null)return"unproven";
   const done=current.stages[flight.stage];
   if(done?.digest===launch.digest&&done.doneMs>=endedMs)return"unproven";
   const ownFailure=current.failure?.stage===flight.stage,peakMegabytes=Math.max(current.peakMegabytes??0,result.peakMegabytes??0)||current.peakMegabytes;
   state.write({...current,stages:{...current.stages,[flight.stage]:{digest:launch.digest,doneMs:endedMs}},appliedCompletions:{...current.appliedCompletions,[flight.stage]:id},
    failure:ownFailure?null:current.failure,attempts:ownFailure||current.failure===null?0:current.attempts,peakMegabytes,
    readyMs:plan.at(-1)?.name===flight.stage?endedMs:current.readyMs});
   return"applied";
  },
 };

 return{
  server:isServer(input.origin,input.group),...input.stages===undefined&&plan===READY_STAGES_V1?{}:{stages:plan},now:Date.now,jitter,cores:availableParallelism(),
  sleep:(milliseconds,signal)=>new Promise<void>(done=>{
   if(signal.aborted)return done();
   const finish=()=>{clearTimeout(timer);signal.removeEventListener("abort",finish);done();};
   const timer=setTimeout(finish,milliseconds);
   signal.addEventListener("abort",finish,{once:true});
  }),
   digests:async()=>measure(plan),
   root:input.root,
   tree:()=>{const store=input.environment.SEMIO_FLEET_STORE_ROOT;if(store===undefined||store==="")return null;try{return readdirSync(join(store,"v1",P.treeStampDirectory)).filter(name=>!name.startsWith(".")).sort().join("|");}catch{return null;}},
   state,
  marker:{exists:()=>existsSync(marker),write:()=>{mkdirSync(machinePathV1(input.root,".cache","f"),{recursive:true});writeFileSync(marker,new Date().toISOString()+"\n");}},
  repoPresent:()=>readyRepoBinaryV1(input.home??homedir(),input.environment)!==undefined,
  human:()=>readyHumanActiveV1(directory),
  lease,
  inflight:()=>{const row=readInflightV1(join(directory,P.inflightName));return row!==undefined&&(stageProcessAliveV1(directory,row)||existsSync(row.exitFile))?row.stage:undefined;},
   disk:stage=>readyDiskShortV1({stage,paths:[machinePathV1(input.root,".cache"),tmpdir()],facts:input.facts??readyFilesystemFactsV1,runs:recentRuns(),derivedInodes:readyDerivedInodesV1(stage,input.root,readyScopesOfStageV1(plan,stage))})??readyImageShortV1(readyImageFactsV1(input.root,stage,input.facts??readyFilesystemFactsV1)),
   diskUnknown:()=>readyDiskUnknownV1({paths:[machinePathV1(input.root,".cache"),tmpdir()],facts:input.facts??readyFilesystemFactsV1}),
   low:stage=>readyImageReliefV1(readyImageFactsV1(input.root,stage,input.facts??readyFilesystemFactsV1)),
   freeInodes:()=>{const inodes=(input.facts??readyFilesystemFactsV1)(machinePathV1(input.root,".cache"))?.freeInodes;return typeof inodes==="number"?inodes:null;},
   units:()=>{try{return readBuildCountersV1(input.root).unit;}catch{return null;}},
   freeMegabytes:()=>{const free=[machinePathV1(input.root,".cache"),tmpdir()].map(path=>(input.facts??readyFilesystemFactsV1)(path)?.freeBytes).filter((bytes):bytes is number=>typeof bytes==="number");return free.length===0?null:Math.floor(Math.min(...free)/1048576);},
   recover:async request=>{
    const facts=input.facts??readyFilesystemFactsV1,reading=()=>{const row=facts(request.short.path);return row===null||row.freeBytes===null?undefined:{partition:request.short.path,totalBytes:row.freeBytes,freeBytes:row.freeBytes};};
    const{recoverNeedV1}=await import("../../../🌡️host-capacity/🧹️disk-recovery/🟦️.ts"),{realDiskWritersV1}=await import("../../../🌡️host-capacity/🧹️disk-recovery/🔌️real/🟦️.ts");
    const image=request.short.partition!==undefined,all=realDiskWritersV1({workspace:input.root,home:input.home??homedir(),tmp:tmpdir(),uid:process.getuid?.()??0,environment:input.environment,language:"both",report:request.report,free:()=>reading()?.freeBytes,...(!image&&request.short.resource==="space"?{pressure:{path:request.short.path,requiredFreeBytes:request.short.needed*1048576}}:{})}),writers=image&&(request.short.limitedByHost!==true||request.relief===true)?readyImageWritersV1(all,request.short.path,sep):all;
    if(image&&request.short.partition==="r")try{const{reclaimUnitStorageV1}=await import("../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts");await reclaimUnitStorageV1(readyImageStoreV1(request.short.path),Math.max(readyImageRetainBytesV1({freeBytes:request.short.available*1048576,totalBytes:(request.short.total??0)*1048576}),(request.short.needed-request.short.available)*1048576),{signal:request.signal});}catch(error){if(request.signal.aborted)throw error;request.report(`collecting inside the image failed (${String((error as Error)?.message??error).slice(0,200)}) / Sammeln im Image fehlgeschlagen`);}
    if(request.relief===true&&readyImageStoreV1(request.short.path)===request.short.path)return{fits:true,consumers:[]};
    const outcome=await recoverNeedV1({facts:reading,needBytes:request.short.needed*1048576,writers,language:"both",report:request.report,signal:request.signal});
    return{fits:outcome.verdict!=="refuse",consumers:outcome.consumers};
   },
   sweep:async request=>{
    const facts=input.facts??readyFilesystemFactsV1,path=machinePathV1(input.root,".cache"),reading=()=>{const row=facts(path);return row===null||row.freeBytes===null?undefined:{partition:path,totalBytes:row.totalBytes??row.freeBytes,freeBytes:row.freeBytes};};
    const{sweepDiskV1}=await import("../../../🌡️host-capacity/🧹️disk-recovery/🟦️.ts"),{realDiskWritersV1}=await import("../../../🌡️host-capacity/🧹️disk-recovery/🔌️real/🟦️.ts");
    const writers=realDiskWritersV1({workspace:input.root,home:input.home??homedir(),tmp:tmpdir(),uid:process.getuid?.()??0,environment:input.environment,language:"both",report:request.report,free:()=>reading()?.freeBytes,pressure:{path,requiredFreeBytes:request.neededMegabytes*1048576}});
    const outcome=await sweepDiskV1({facts:reading,needBytes:request.neededMegabytes*1048576,...request.scarce===undefined?{}:{scarce:request.scarce,census:{root:machinePathV1(input.root,".cache")}},writers,language:"both",report:request.report,signal:request.signal});
    return{fits:outcome.verdict!=="refuse",consumers:outcome.consumers};
   },
   native:()=>nativeShardConsumerV1(readNativeShardsV1(input.root))??null,
   relieve:async request=>{const outcome=await relieveNativeShardsV1({facts:readNativeShardsV1(input.root),hold:nativeShardHoldV1(input.root),language:"both",signal:request.signal});for(const line of outcome.lines)request.report(line);return outcome.freedBytes;},
   compiler,
   heal:async request=>{
    if(process.platform!=="darwin"||!READY_IMAGE_POLICY_V1.healStages.includes(request.stage))return{grown:0,unfit:null};
    const{growImagesV1}=await import("../../../🩺️doctor/🔎️probes/💾️storage/🟦️.ts"),{doctorPortsV1}=await import("../../../🩺️doctor/🔌️ports/🟦️.ts");
    const rows=await growImagesV1(doctorPortsV1(input.root,{signal:request.signal,environment:input.environment,progress:stage=>request.report(`growing the image / das Image wächst: ${stage}`),command:(program,args)=>new Promise<string>((done,fail)=>execFile(program,[...args],{windowsHide:true,encoding:"utf8",maxBuffer:8*1048576,timeout:P.healCommandMilliseconds,signal:request.signal},(error,stdout,stderr)=>error?fail(Error(`${program} ${args.join(" ")}: ${stderr||error.message}`)):done(stdout)))}),false);
    for(const row of rows)if(row.state!=="grown")request.report(row.note);
    const unfit=rows.find(row=>row.state==="skipped"&&row.kind==="unfit");
    return{grown:rows.filter(row=>row.state==="grown").length,unfit:request.stage==="dependencies"&&unfit?.state==="skipped"?{path:machinePathV1(input.root,...cachePolicy.partitions.dependencies.directory.split("/")),neededMegabytes:Number(unfit.values.needed),availableMegabytes:Number(unfit.values.free)}:null};
   },
   discardDependencies:()=>{const directory=machinePathV1(input.root,...cachePolicy.partitions.dependencies.directory.split("/"));return withNativeImageSharedReadV1({leaseDirectory:machinePathV1(input.root,".cache/b/l/m/data"),imageDirectory:directory,signal:AbortSignal.timeout(NATIVE_IMAGE_SHARED_READ_POLICY_V1.sampleWaitMilliseconds*2),timeoutMs:NATIVE_IMAGE_SHARED_READ_POLICY_V1.sampleWaitMilliseconds},()=>discardDependencyGenerationsV1(join(directory,"m")));},
 memory:()=>{const reading=fleetHostReadingV1({environment:input.environment}),free=reading.availableMegabytes,floor=readyMemoryFloorMegabytesV1(reading.hostMegabytes);return{availableMegabytes:free,ok:free===null||free>=floor,floorMegabytes:floor};},
  run:(stage,options)=>runReadyStageV1({completion,root:input.root,environment:input.environment,stage,...input.home===undefined?{}:{home:input.home},...input.facts===undefined?{}:{facts:input.facts},orphans:{registryRoot:readyRegistryRootV1(input.environment,process.platform,input.home??homedir()),windows:()=>{try{return readyStoppedWindowsV1(recentRuns().filter(run=>run.attempts!==null));}catch{return[];}},human:()=>{try{return readyHumanRecordV1(readFileSync(join(directory,P.humanName),"utf8"));}catch{return null;}}},...options}),
  status,say:input.say,
   mirror:(stage,lines,owner)=>{
   if(input.environment.SEMIO_FLEET_STORE_ROOT===undefined||input.environment.SEMIO_FLEET_STORE_ROOT==="")return;
   try{
    const facts=volumeFactsTextV1(input.root);let text=`${new Date().toISOString()} ${stage}\n${lines.join("\n")}\n${facts===""?"":facts+"\n"}`;
    if(text.length>P.mirrorBytes)text=text.slice(text.length-P.mirrorBytes);
    const dir=join(input.environment.SEMIO_FLEET_STORE_ROOT,"v1","origins",input.origin),file=join(dir,`${P.mirrorPrefix}${stage}.txt`);
    mkdirSync(dir,{recursive:true});readyWriteStoreFileV1(file,text);
   }catch{}
    if(owner===undefined)return;
    const slot=ownedSliceReportV1(input.root,owner);if(slot===undefined)return;
    let report:string|undefined;
    try{
     if(statSync(slot.file).size<=P.sliceReport.fileBytes){
      const bytes=readFileSync(slot.file);
      if(bytes.length<=P.sliceReport.fileBytes){const text=bytes.toString("utf8");if(JSON.parse(text)?.readyOwner===owner)report=text;}
     }
    }catch{}
    try{
     const dir=join(input.environment.SEMIO_FLEET_STORE_ROOT,"v1","origins",input.origin),target=join(dir,P.sliceReport.name);
     if(report!==undefined){
      if(report!==lastSliceReport)readyWriteStoreFileV1(target,report);
      lastSliceReport=report;
     }else if(lastSliceOwner!==owner){rmSync(target,{force:true});lastSliceReport=undefined;}
     lastSliceOwner=owner;
    }catch{}
  },
  timing:run=>{
   try{
    const local=join(directory,P.timingsName);
    let runs:ReadyTimingV1[]=[];
    try{runs=(JSON.parse(readFileSync(local,"utf8")) as {runs:ReadyTimingV1[]}).runs;}catch{}
    runs=[...runs,run].slice(-P.timingsKept);
    const doc={version:1,origin:input.origin,runs,updatedMs:Date.now()};
    mkdirSync(directory,{recursive:true});writeFileSync(local,JSON.stringify(doc));
    const store=input.environment.SEMIO_FLEET_STORE_ROOT;
    if(store!==undefined&&store!==""&&validateJsonSchemaSubset(schema.definitions.Timings,doc,schema).length===0){const dir=join(store,"v1","origins",input.origin),file=join(dir,P.timingsName);mkdirSync(dir,{recursive:true});readyWriteStoreFileV1(file,JSON.stringify(doc));}
   }catch{}
  },
  busy:busy=>{if(input.work===undefined)return;input.work.busy=busy;if(!busy)input.work.wake?.();},
 };
}

/** 🚦️ Runs the readiness loop of this machine until the signal aborts, at a low priority, with the fleet variables of the replica daemon; the group server writes nothing and only hosts the metrics collector until the signal aborts (a ref'd long timer keeps the idle event loop asleep: Bun 1.4.2 spins at 45 % of a core when only an awaited promise and unref'd timers are left). A restart for new code waits while a stage runs (`work.busy`). */
export async function runReady(context:RemoteExecutionSidecarContextV1):Promise<void>{
 const origin=remoteExecutionOriginV1(context.root,context.environment);
 let group=context.environment.SEMIO_FLEET_GROUP;
 if(group===undefined||group==="")try{group=(JSON.parse(readFileSync(machinePathV1(context.root,".cache","f","machine.json"),"utf8")) as {groupId?:string}).groupId;}catch{}
 const ports=createReadyPortsV1({root:context.root,environment:context.environment,origin,group,...context.environment.HOME===undefined?{}:{home:context.environment.HOME},...context.work===undefined?{}:{work:context.work},say:context.write});
 const stopMetrics=startFleetMetricsV1(context,origin),codes=createSidecarCodeMirrorV1({root:context.root,store:context.environment.SEMIO_FLEET_STORE_ROOT,origin,roles:P.codeRoles});
 if(ports.server){
  context.write("the group server is never set up by this service / der Gruppenserver wird von diesem Dienst nie eingerichtet");
  const hold=setInterval(()=>{codes.tick();},P.codeMirrorPollMilliseconds);
  try{await new Promise<void>(done=>{if(context.signal.aborted)done();else context.signal.addEventListener("abort",()=>done(),{once:true});});}finally{clearInterval(hold);stopMetrics();}
  return;
 }
 const mirror=createReadyMemoryMirrorV1({store:context.environment.SEMIO_FLEET_STORE_ROOT,origin}),census=createPartitionCensusMirrorV1({root:context.root,store:context.environment.SEMIO_FLEET_STORE_ROOT,origin,volume:()=>{const row=readyFilesystemFactsV1(machinePathV1(context.root,".cache")),whole=(value:number|null|undefined):number|null=>typeof value==="number"&&Number.isFinite(value)&&value>=0?Math.floor(value):null;return{freeBytes:whole(row.freeBytes),totalBytes:whole(row.totalBytes),freeInodes:whole(row.freeInodes),totalInodes:whole(row.totalInodes)};}}),sweep=createTicketGeneratedSweepV1({root:context.root,signal:context.signal,report:line=>context.write(line),volume:()=>{const row=readyFilesystemFactsV1(machinePathV1(context.root,".cache"));return{freeBytes:typeof row.freeBytes==="number"?row.freeBytes:null,totalBytes:typeof row.totalBytes==="number"?row.totalBytes:null};}}),timer=setInterval(()=>{mirror.tick();codes.tick();void census.tick();void sweep.tick();},P.memoryMirrorPollMilliseconds);
 timer.unref?.();mirror.tick();
 try{await runReadyV1(ports,context.signal);}finally{clearInterval(timer);stopMetrics();}
}
