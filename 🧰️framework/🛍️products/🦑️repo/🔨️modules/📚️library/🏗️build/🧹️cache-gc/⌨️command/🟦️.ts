import {existsSync} from "node:fs";
import {readFile,stat} from "node:fs/promises";
import {repositoryCacheBudgetV1,type RepositoryCachePartitionIdV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import budgetPolicy from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {join} from "node:path";
import {CACHE_GC_LIMITS_V1} from "../🧬️schema/🟦️.ts";
import {cacheSizeChargeV1,CACHE_RUNTIME_V1,cacheAreaDirectoryV1,cacheLedgerV1,cacheRootDirectoryV1,cacheTextV1,formatCacheBytesV1,formatCacheReportV1,type CacheLanguageV1,type CacheLedgerV1,type CachePartitionBudgetsV1,type CacheSizeRecordV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import type {MemberScanControlV1,MemberVerifyReportV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts";
import {formatMachinePlanV1,machinePolicyV1,parseMachineBytesV1,planMachineBudgetV1,resolveMachineTotalV1,type MachinePolicyV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts";
import {dedupeTreeV1} from "../👯️dedupe/🟦️.ts";
import {collectMachineBoundsV1} from "../../🌡️host-capacity/🧹️disk-recovery/🔌️real/🟦️.ts";
import {cacheRunnerPortsV1,collectCacheV1,loadCacheSizesV1,machineHostFreeBytesV1,machineHostTotalBytesV1,machineUsesV1,removeCacheTreeV1,reportCacheLedgerV1,type CacheRunnerPortsV1} from "../🏃️runner/🟦️.ts";
import type {CargoGcControlV1} from "../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/📏️tree/🟦️.ts";
import type {MachinePlanV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts";
import {resolveLanguageV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {CATALOG_LANGUAGES_V1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {writeResultLineV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";
import {nativeFreeSpaceSyncV1,nativeStorageBackendOfDirectoryV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

/** 📝️ The closed arguments of `repo cache status|gc|verify`. */
export type CacheArgumentsV1=Readonly<{operation:"status"|"gc"|"verify"|"dedupe";json:boolean;deep:boolean;dryRun:boolean;only:ReadonlySet<string>|undefined;language:string|undefined;budget:string|undefined;reclaimBytes?:number|undefined}>;
const both=(key:string,variables:Readonly<Record<string,string|number>>={}):string=>cacheTextV1("en",key,variables)+"\n"+cacheTextV1("de",key,variables);
const valued=Object.freeze(["--area","--lang","--budget","--reclaim"]),plain=Object.freeze(["--json","--dry-run","--deep"]);

/** 🧷️ Reads `status|gc|verify` with its options; every unknown or repeated option is refused in both languages. */
export function cacheArgumentsV1(args:readonly string[],ledger:CacheLedgerV1=cacheLedgerV1(),declared:MachinePolicyV1=machinePolicyV1()):CacheArgumentsV1{
 const operation=args[0];if(operation!=="status"&&operation!=="gc"&&operation!=="verify"&&operation!=="dedupe")throw Error(both("error.syntax"));
 const seen=new Set<string>(),values:Record<string,string>={};let json=false,dryRun=false,deep=false;
 for(let index=1;index<args.length;index++){
  const argument=args[index]!,cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);
  if(seen.has(name))throw Error(both("error.repeated",{option:name}));
  seen.add(name);
  if(valued.includes(name)&&!(operation==="verify"&&(name==="--area"||name==="--budget"))&&!(operation==="dedupe"&&name==="--budget")&&!(name==="--reclaim"&&operation!=="gc")){const value=cut<0?args[++index]:argument.slice(cut+1);if(!value||value.startsWith("--"))throw Error(both("error.value",{option:name}));values[name]=value;}
  else if(plain.includes(name)&&cut<0&&(name!=="--dry-run"||operation==="gc"||operation==="dedupe")&&(name!=="--deep"||operation!=="gc"&&operation!=="dedupe")){if(name==="--json")json=true;else if(name==="--deep")deep=true;else dryRun=true;}
  else throw Error(both("error.option",{option:argument}));
 }
 if(operation==="verify"&&!deep)throw Error(both("error.verify"));
 const ids=new Set(operation==="dedupe"?declared.dedupe.map(target=>target.id):[...ledger.areas.map(area=>area.id),"retired","undeclared"]),only=values["--area"]?new Set(values["--area"].split(",")):undefined;
 for(const id of only??[]){if(!id)throw Error(both("error.areaEmpty"));if(!ids.has(id))throw Error(both("error.area",{id}));}
 return Object.freeze({operation,json,deep,dryRun,only,language:values["--lang"],budget:values["--budget"],reclaimBytes:values["--reclaim"]===undefined?undefined:parseMachineBytesV1(values["--reclaim"])});
}

/** 🎛️ What the command needs besides the runner ports: the workspace, the environment, the output and the ledger. */
export type CacheCommandContextV1=Readonly<{workspace:string;environment:Readonly<Record<string,string|undefined>>;write(line:string):void;ledger?:CacheLedgerV1;partitions?:CachePartitionBudgetsV1;verify?(control:MemberScanControlV1):Promise<MemberVerifyReportV1>;machinePolicy?:MachinePolicyV1;freeBytes?(path:string):number|undefined;clone?(source:string,destination:string):Promise<boolean>}>;

/** 💰️ Resolves the disk budget of this machine from `--budget`, the environment and the machine file inside the repository cache root, in that order, falling back to the declared default. */
async function machineTotalV1(context:CacheCommandContextV1,parsed:CacheArgumentsV1,ledger:CacheLedgerV1){
 const fileText=await readFile(join(cacheRootDirectoryV1(context.workspace,ledger,"repository"),machinePolicyV1().file),"utf8").catch(()=>undefined);
 return resolveMachineTotalV1({...parsed.budget!==undefined?{argument:parsed.budget}:{},environment:context.environment,...fileText!==undefined?{fileText}:{}});
}

/** 👯️ Looks for identical large files in every declared place of the machine budget and, unless only planning, replaces each duplicate by a copy-on-write clone of one canonical file; a place whose image is not mounted is skipped. */
async function runDedupeV1(context:CacheCommandContextV1,parsed:CacheArgumentsV1,language:CacheLanguageV1,ports:CacheRunnerPortsV1):Promise<number>{
 const declared=context.machinePolicy??machinePolicyV1(),budgets:CachePartitionBudgetsV1=context.partitions??budgetPolicy.partitions,rows:Record<string,unknown>[]=[];let replaced=0,freed=0,skipped=0,failed=0,gained=0,unmeasured=0;
 const free=context.freeBytes??((path:string):number|undefined=>nativeFreeSpaceSyncV1(path)?.availableBytes);
 for(const target of declared.dedupe){
  if(parsed.only&&!parsed.only.has(target.id))continue;
  const partition=budgets[target.partition],base=partition?join(context.workspace,partition.directory,"m",...(target.directory?.split("/")??[])):undefined,present=base!==undefined&&await stat(base).then(row=>row.isDirectory(),()=>false);
  if(!base||!present){if(!parsed.json)context.write(cacheTextV1(language,"dedupe.absent",{id:target.id,directory:target.directory??target.partition}));continue;}
  if(!parsed.json)context.write(cacheTextV1(language,"dedupe.target",{id:target.id,directory:base}));
  await ports.progress("dedupe:"+target.id);const before=parsed.dryRun?undefined:free(base);
  const outcome=await dedupeTreeV1({base,minimumBytes:target.minimumBytes,quietMs:target.quietMs,maximumFiles:target.maximumFiles,dryRun:parsed.dryRun,language,write:parsed.json?()=>{}:context.write},ports,context.clone);
  if(outcome.partial&&!parsed.json)context.write(cacheTextV1(language,"dedupe.partial",{id:target.id,count:target.maximumFiles}));
  const after=parsed.dryRun?undefined:free(base),gain=before!==undefined&&after!==undefined?Math.max(0,after-before):null;
  if(gain!==null)gained+=gain;else if(!parsed.dryRun&&outcome.clones){unmeasured++;if(!parsed.json)context.write(cacheTextV1(language,"dedupe.freeUnknown",{directory:base}));}
  replaced+=outcome.replaced;freed+=outcome.freedBytes;skipped+=outcome.skipped;failed+=outcome.failed.length;
  for(const message of outcome.failed)if(!parsed.json)context.write(target.id+": "+message);
  rows.push({id:target.id,directory:base,groups:outcome.plan.groups,files:outcome.plan.files,clones:outcome.clones,plannedReplacements:outcome.plan.replacements.length,reclaimableBytes:outcome.plan.reclaimableBytes,replaced:outcome.replaced,freedBytes:outcome.freedBytes,volumeGainBytes:parsed.dryRun?0:gain,skipped:outcome.skipped,failed:outcome.failed,partial:outcome.partial});
 }
 const planned=rows.reduce((sum,row)=>sum+(row.plannedReplacements as number),0),reclaimable=rows.reduce((sum,row)=>sum+(row.reclaimableBytes as number),0);
 if(parsed.json)context.write(JSON.stringify({version:1,dryRun:parsed.dryRun,targets:rows},null,2));
 else context.write(parsed.dryRun?cacheTextV1(language,"dedupe.summary.plan",{replaced:planned,freed:formatCacheBytesV1(reclaimable)}):cacheTextV1(language,unmeasured>0?"dedupe.summary.unmeasured":"dedupe.summary",{replaced,gained:formatCacheBytesV1(gained),freed:formatCacheBytesV1(freed),skipped,failed}));
 return failed?1:0;
}

/** 🚀️ Runs `repo cache status|gc|verify|dedupe`: status reads the ledger without any lease and without walking a tree (statfs of the partitions plus the size ledger; `--deep` walks everything with progress and cancellation and records the sizes); gc collects retired roots, undeclared entries and over-budget areas under the lease-aware runner; `verify --deep` reads every indexed optional graph leaf and every content-addressed closure inventory again and reports what disagrees with its recorded hash or name, with progress and cancellation. Returns the process exit code. */
export async function runCacheLedgerCommandV1(args:readonly string[],context:CacheCommandContextV1,ports:CacheRunnerPortsV1):Promise<number>{
 if(args[0]==="cas"){const{runCasCommandV1}=await import("../../🗄️cas/⌨️command/🟦️.ts");return runCasCommandV1(args.slice(1),context,ports);}
 const ledger=context.ledger??cacheLedgerV1(),parsed=cacheArgumentsV1(args,ledger,context.machinePolicy),language:CacheLanguageV1=resolveLanguageV1(CATALOG_LANGUAGES_V1,args,context.environment).language;
 try{
  if(parsed.operation==="status"){
   const report=await reportCacheLedgerV1(context.workspace,ledger,ports,context.partitions,parsed.only,{deep:parsed.deep}),total=await machineTotalV1(context,parsed,ledger),machine=planMachineBudgetV1({totalBytes:total.bytes,hostFreeBytes:machineHostFreeBytesV1(context.workspace),hostTotalBytes:machineHostTotalBytesV1(context.workspace),uses:machineUsesV1(report)});
   if(parsed.json)context.write(JSON.stringify({...report,machine:{...machine,source:total.source}},null,2));else{for(const line of formatCacheReportV1(language,report,ledger))context.write(line);context.write("");for(const line of formatMachinePlanV1(language,machine,total.source))context.write(line);}
   return 0;
  }
  if(parsed.operation==="verify"){
   if(!context.verify){context.write(cacheTextV1(language,"verify.unavailable"));return 1;}
   let announced=false;
   const report=await context.verify({cancelled:()=>ports.signal.aborted,progress:(label,done,total)=>{if(!announced&&!parsed.json){announced=true;context.write(cacheTextV1(language,"verify.start",{groups:total}));}return ports.progress(cacheTextV1(language,"verify.group",{id:label,done,total}));}});
   if(report.cancelled){context.write(cacheTextV1(language,"error.cancelled"));return 130;}
   if(parsed.json)context.write(JSON.stringify({version:1,...report},null,2));
   else{for(const path of report.mismatched)context.write(cacheTextV1(language,"verify.mismatch",{path}));for(const path of report.closureMismatched)context.write(cacheTextV1(language,"verify.closureMismatch",{path}));context.write(cacheTextV1(language,"verify.summary",{verified:report.verified,bytes:formatCacheBytesV1(report.bytes),groups:report.groups,closures:report.closures,mismatches:report.mismatches,inventorymismatches:report.closureMismatches,stale:report.stale,unindexed:report.unindexed,vanished:report.vanished}));}
   return report.mismatches||report.closureMismatches?1:0;
  }
  if(parsed.operation==="dedupe")return runDedupeV1(context,parsed,language,ports);
  const total=await machineTotalV1(context,parsed,ledger),result=await collectCacheV1(context.workspace,ledger,{dryRun:parsed.dryRun,only:parsed.only,...parsed.reclaimBytes===undefined?{}:{reclaimBytes:parsed.reclaimBytes},machine:{totalBytes:total.bytes,source:total.source}},ports,{language,write:parsed.json?()=>{}:context.write},context.partitions);
  const host=parsed.only===undefined?await collectHostStoreV1(context,ledger,language,parsed,ports,result.sizeUpdates):undefined,bounds=parsed.only===undefined?await collectMachineBoundsOfCommandV1(context,language,parsed,ports):undefined;
  if(parsed.json)context.write(JSON.stringify({version:1,dryRun:result.dryRun,freedBytes:result.freedBytes,kept:result.kept,removed:result.removed,failed:result.failed,delegated:result.delegated,machine:result.machine,...result.fleetStatePartial?{fleetStatePartial:true}:{},plans:result.plans,hostStore:host,machineBounds:bounds?.freed},null,2));
  else{for(const failure of result.failed)context.write(failure.target+"/"+failure.name+": "+failure.message);if(host)context.write(host.line);for(const line of bounds?.lines??[])context.write(line);}
  return result.failed.length?1:0;
 }catch(error){
  if(ports.signal.aborted){context.write(cacheTextV1(language,"error.cancelled"));return 130;}
  throw error;
 }
}

/** 🗄️ Applies host-wide collection after workspace owners finish. The shared store and graph memos age out and are cut oldest-first; workspace source inventories and fixed images count toward the host cap. */
async function collectHostStoreV1(context:CacheCommandContextV1,ledger:CacheLedgerV1,language:CacheLanguageV1,parsed:CacheArgumentsV1,ports:CacheRunnerPortsV1,sizeUpdates:Readonly<Record<string,CacheSizeRecordV1|null>>):Promise<{line:string;result:unknown}|undefined>{
 const{collectCasV1,formatCasCollectionV1}=await import("../../🗄️cas/🧹️collector/🟦️.ts"),{adoptCasV1,casPlacesOfWorkspaceV1}=await import("../../🗄️cas/🧲️adopt/🟦️.ts"),{casHomeOfV1,formatCasAdoptionV1}=await import("../../🗄️cas/⌨️command/🟦️.ts"),{casAreasOfWorkspaceV1}=await import("../../🗄️cas/📂️areas/🟦️.ts"),{measureCasPartitionV1}=await import("../../🗄️cas/🌡️partition/🟦️.ts"),{CAS_POLICY_V1}=await import("../../🗄️cas/🧬️schema/🟦️.ts"),{casRootV1}=await import("../../🗄️cas/🗃️store/🟦️.ts"),home=casHomeOfV1(context.environment);
 if(home===undefined)return undefined;
 const sizes=await loadCacheSizesV1(context.workspace,ledger),records={...sizes.records};
 for(const [id,record] of Object.entries(sizeUpdates)){if(record===null)delete records[id];else records[id]=record;}
 const external=cacheSizeChargeV1(ledger,{...sizes,records});
 const root=casRootV1(context.environment,process.platform,home),adoption=await adoptCasV1({root,places:casPlacesOfWorkspaceV1(context.workspace),signal:ports.signal,dryRun:parsed.dryRun}),result=await collectCasV1({root,policy:CAS_POLICY_V1,areas:await casAreasOfWorkspaceV1(context.workspace,home),externalBytes:external,partition:measureCasPartitionV1(root),signal:ports.signal,dryRun:parsed.dryRun});
 return{line:formatCasAdoptionV1(adoption,language)+"\n"+formatCasCollectionV1(result,CAS_POLICY_V1,language),result:{adoption,collection:result}};
}

/** 🧺️ Applies the in-process bounds of the machine after the host store: the owner-pool waves, the footprint logs, the read verdicts, the staging and temporary directories of gone owners and the files the replica overwrote, each by its declared age and count; a dry run only reports what would go. A bound that fails is skipped, never the command. @see ../../🌡️host-capacity/🧹️disk-recovery/🧺️bounds/🟦️.ts */
async function collectMachineBoundsOfCommandV1(context:CacheCommandContextV1,language:CacheLanguageV1,parsed:CacheArgumentsV1,ports:CacheRunnerPortsV1):Promise<{lines:readonly string[];freed:Readonly<Record<string,number>>}|undefined>{
 const{homedir,tmpdir}=await import("node:os");
 try{return await collectMachineBoundsV1({workspace:context.workspace,home:context.environment.HOME??(context.environment===process.env?homedir():context.workspace),tmp:context.environment.TMPDIR??(context.environment===process.env?tmpdir():join(context.workspace,".cache","scratch")),uid:process.getuid?.()??0,environment:context.environment,dryRun:parsed.dryRun},language==="de"?"de":"en",ports.signal);}
 catch(error){if(ports.signal.aborted)throw error;return undefined;}
}

/** 🚰️ Binds the graph reclaim to the real retained graph partition, its free space and the two shared scoped recoveries. */
async function graphReclaim(root:string,dryRun:boolean,language:CacheLanguageV1,phaseControl:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts").ResourceLeaseControl,leases:typeof import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts"),opening:typeof import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts"),partitions:typeof import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts"),headroom:typeof import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🧹️headroom/🟦️.ts")):Promise<string>{
 const{reclaimGraphOptionalV1,graphProducerPublicationsV1,graphStaleBuildJournalsV1}=await import("../🚰️reclaim/🟦️.ts"),{nativeImageSpaceV1}=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts"),directory=opening.repositoryLeaseDirectoryV1(root);
 type Image=Awaited<ReturnType<typeof partitions.openRepositoryCachePartitionV1>>;
 const exclusive=async(resource:string)=>{try{return await leases.acquireResourceLease({directory,resource,mode:"exclusive",signal:phaseControl.signal,control:phaseControl,timeoutMs:0});}catch(error){if(/timed out/iu.test(String((error as Error).message)))return undefined;throw error;}};
 return reclaimGraphOptionalV1(root,cacheLedgerV1(),dryRun,language,{
  exclusive,
  exists:()=>{const graph=repositoryCacheBudgetV1(root).partitions.graph.directory;return existsSync(join(graph,nativeStorageBackendOfDirectoryV1(graph).backingFilename));},open:()=>partitions.openRepositoryCachePartitionV1(root,"graph",phaseControl),
  supersede:async(image,dryRun)=>{const scan=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts"),{buildGraphWriterResourceV1}=await import("../../📥️inputs/💾️storage/🔒️writer/🟨️.mjs");return scan.supersedeBuildGraphObservationBlocksV1((image as Image).root,{lease:(kind,identity)=>exclusive(buildGraphWriterResourceV1(root,kind,identity))},{cancelled:()=>phaseControl.signal.aborted,progress:()=>{}},{dryRun});},
  index:async image=>{const scan=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts"),{buildGraphWriterResourceV1}=await import("../../📥️inputs/💾️storage/🔒️writer/🟨️.mjs"),{resolve}=await import("node:path"),imageRoot=(image as Image).root;return scan.indexBuildGraphMembersV1(imageRoot,{lease:(kind,identity)=>exclusive(kind==="canonical"?"repository-graph-facts:"+resolve(root):buildGraphWriterResourceV1(root,kind,identity)),read:scan.closureReadChildPortV1(imageRoot,phaseControl)},{cancelled:()=>phaseControl.signal.aborted,progress:()=>{}});},
  closures:async(image,dryRun)=>{const scan=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts"),{buildGraphWriterResourceV1}=await import("../../📥️inputs/💾️storage/🔒️writer/🟨️.mjs"),{resolve}=await import("node:path"),imageRoot=(image as Image).root;const resourceOf=(kind:"selection"|"dataset"|"canonical",identity:string)=>kind==="canonical"?"repository-graph-facts:"+resolve(root):buildGraphWriterResourceV1(root,kind,identity),{randomBytes}=await import("node:crypto");return scan.supersedeBuildGraphClosuresV1(imageRoot,{lease:(kind,identity)=>exclusive(resourceOf(kind,identity)),queue:async(kind,identity)=>{try{return await leases.acquireQueuedResourceLease({directory,resource:resourceOf(kind,identity),mode:"exclusive",owner:randomBytes(8).toString("hex"),signal:phaseControl.signal,control:phaseControl,timeoutMs:CACHE_GC_LIMITS_V1.graphLeaseWaitMilliseconds});}catch(error){if(/timed out/iu.test(String((error as Error).message)))return undefined;throw error;}},read:scan.closureReadChildPortV1(imageRoot,phaseControl)},{cancelled:()=>phaseControl.signal.aborted,progress:()=>{}},{dryRun});},
  closureStores:async image=>{const{readdir}=await import("node:fs/promises"),{join}=await import("node:path"),{collectClosureStoreV1}=await import("../../🕸️graph/📥️inference/🧭️script-inputs/🧹️retention/🟨️.mjs"),{INFERENCE_LIMITS_V1}=await import("../../🕸️graph/📥️inference/🧬️schema/🟨️.mjs"),base=join((image as Image).root,"inference","selections"),total={stores:0,entries:0,layers:0,values:0,kept:0,bytes:0,graceMilliseconds:INFERENCE_LIMITS_V1.closureStoreGraceMilliseconds,cancelled:false};for(const id of await readdir(base).catch(()=>[] as string[])){const report=await collectClosureStoreV1({directory:join(base,id,"ce"),graceMilliseconds:INFERENCE_LIMITS_V1.closureStoreGraceMilliseconds,dryRun,cancelled:()=>phaseControl.signal.aborted});if(report.cancelled){total.cancelled=true;break;}total.stores++;total.entries+=report.removed!.entries;total.layers+=report.removed!.layers;total.values+=report.removed!.values;total.kept+=report.kept!.entries;total.bytes+=report.bytes!;}return total;},
  publications:async(image,dryRun)=>graphProducerPublicationsV1((image as Image).root,dryRun,phaseControl.signal),
  journals:async(image,dryRun)=>graphStaleBuildJournalsV1((image as Image).root,dryRun,phaseControl.signal),
  lru:async(image,dryRun)=>{const scan=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts"),{buildGraphWriterResourceV1}=await import("../../📥️inputs/💾️storage/🔒️writer/🟨️.mjs"),{resolve}=await import("node:path");return scan.retireBuildGraphOptionalGroupsLruV1((image as Image).root,{lease:(kind,identity)=>exclusive(kind==="canonical"?"repository-graph-facts:"+resolve(root):buildGraphWriterResourceV1(root,kind,identity))},{cancelled:()=>phaseControl.signal.aborted,progress:()=>{}},{dryRun});},
  free:async image=>{const space=nativeImageSpaceV1((image as Image).root,(image as Image).filesystemBytes);return space.bavail*space.bsize;},
  selected:(image,request)=>headroom.recoverRepositoryCachePartitionSelectedHeadroomV1(root,image as Image,phaseControl,request),
  optional:(image,request)=>headroom.recoverRepositoryCachePartitionOptionalHeadroomV1(root,image as Image,phaseControl,request)
 });
}

/** 🌱️ Collects (or, in a dry run, only reports) what the mounted rust image can give back, least recently used first, with the same single-flight reclaim the automatic gate of the Cargo admission runs (`collectRustImageV1`, under the claim it shares); lock-aware (compiler locks, build leases, running builds). It frees what the image needs to reach its free-space target and what its machine quota exceeds (`machine`), whichever is more, the pressure rule measured on the honest available space of the image. */
async function cargoIncrementalReclaim(root:string,dryRun:boolean,language:CacheLanguageV1,ports:CacheRunnerPortsV1,machine:MachinePlanV1|null):Promise<string>{
 const{existsSync}=await import("node:fs"),{join}=await import("node:path"),ledger=cacheLedgerV1(),area=ledger.areas.find(row=>row.id==="rust")!,directory=cacheAreaDirectoryV1(root,ledger,area),store=directory===null?null:join(directory,"m");
 if(store===null||!existsSync(store))return cacheTextV1(language,"gc.imageAbsent",{id:"cargo-store"});
 const{collectRustImageV1,autoReclaimTextV1,RUST_IMAGE_RECLAIM_POLICY_V1}=await import("../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/♻️auto-reclaim/🟦️.ts"),{unitBudgetBytesV1}=await import("../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts"),{createHonestMeasureV1}=await import("../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🏦️headroom/🟦️.ts"),{tryClaimWithBallastV1}=await import("../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🛟️ballast/🟦️.ts"),{UNIT_CACHE_POLICY_V1}=await import("../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts");
 const honest=createHonestMeasureV1(),measure=()=>honest(store,1),quotaBytes=machine?.stores.find(row=>row.id==="rust")?.evictBytes??0,lines:string[]=[];
 const control=(collector:string):CargoGcControlV1=>{
  let sequence=0;
  const last=new Map<string,number>();
  const progress=async(stage:string)=>{
   ports.signal.throwIfAborted();
   const now=performance.now();
   if(now-(last.get(stage)??-Infinity)<CACHE_RUNTIME_V1.progressIntervalMs)return;
   last.set(stage,now);
   await ports.progress(`cargo:${collector}:${stage}:${++sequence}`);
  };
  return {signal:ports.signal,progress:(_done,_total,stage)=>progress(stage),remove:path=>removeCacheTreeV1(path,{...ports,progress},"deleting")};
 };
 const run=()=>collectRustImageV1(store,{dryRun,language,demandBytes:0,quotaBytes,unitBudgetBytes:unitBudgetBytesV1(repositoryCacheBudgetV1(root).partitions.rust.capacityBytes),measure,control,write:line=>{lines.push(line);}});
 if(dryRun){await run();return lines.join("\n");}
 const claim=tryClaimWithBallastV1(join(store,"u",UNIT_CACHE_POLICY_V1.layout.locks),RUST_IMAGE_RECLAIM_POLICY_V1.claim);
 if(!claim)return autoReclaimTextV1("joined",{free:(measure().freeBytes/1024**3).toFixed(1)},language);
 try{await run();}finally{claim.release();}
 return lines.join("\n");
}

/** 🧪️ Retires the per-run test folders whose run has ended (no live task or run reader lease), oldest first, once the diagnostics image is above its high-water mark; a dry run only reports. */
async function testRunsReclaim(root:string,dryRun:boolean,language:CacheLanguageV1,phaseControl:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts").ResourceLeaseControl,invocation:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation):Promise<string>{
 const storage=await import("../../../../🧪️test/📦️packages/🟦️typescript/💾️storage/🟦️.ts"),policy=(await import("../../../../🧪️test/📦️packages/🟦️typescript/💾️storage/🔣️policy/🔣️json/🟦️.ts")).default;
 const outcome=await storage.retainRepositoryTestArtifactsV1(root,invocation,phaseControl,{...policy,version:1},undefined,{dryRun});
 return storage.repositoryTestRetentionReportV1(language,outcome,dryRun);
}

/** 🧶️ Bounds the Nx toolkit generations of the tools image: the newest few stay, the current activation and every generation a live reader holds always stay, the rest is retired oldest first (a limited number per run) inside the whole-image exclusive scope, which never waits for running commands (a busy image is a plain verdict, the next run asks again); a dry run only plans. */
async function nxToolingReclaim(root:string,dryRun:boolean,language:CacheLanguageV1,phaseControl:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts").ResourceLeaseControl,invocation:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,machine:MachinePlanV1|null,selected:boolean):Promise<string>{
 const{runNxToolingStoreBoundV1,censusNxToolingStoreV1,formatNxToolingStoreBoundV1}=await import("../../../⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🏃️run/🟦️.ts"),{probeNxToolingHeldV1,probeNxToolingImageIdleV1}=await import("../../../⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🏃️run/🔌️host/🟦️.ts"),{resolveNxToolingStoreBoundPolicyV1}=await import("../../../⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🟦️.ts"),opening=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts"),routing=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
 const area=cacheLedgerV1().areas.find(row=>row.id==="nx-tooling")!,policy=selected?resolveNxToolingStoreBoundPolicyV1(process.env):{...resolveNxToolingStoreBoundPolicyV1({}),idleWaitMs:0},mount=join(root,budgetPolicy.partitions.tools.directory,"m"),leaseDirectory=opening.repositoryLeaseDirectoryV1(root),signal=phaseControl.signal;
 const progress=(stage:string)=>routing.advanceScriptInvocation(invocation,"cache:"+stage,"running"),free=async():Promise<number|null>=>nativeFreeSpaceSyncV1(mount)?.availableBytes??null;
 let pressure:boolean|undefined;
 const leases=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts");
 let single:Awaited<ReturnType<typeof leases.acquireResourceLease>>;
 try{single=await leases.acquireResourceLease({directory:leaseDirectory,resource:"repository-nx-tooling-collector:"+root,mode:"exclusive",signal,control:phaseControl,timeoutMs:0});}catch(error){if(/timed out/iu.test(String((error as Error).message)))return cacheTextV1(language,"nxTooling.running");throw error;}
 const pressed=async():Promise<boolean>=>pressure??=((await free())??Infinity)<area.reclaim!.softAvailableBytes||(machine?.stores.find(store=>store.id==="tools")?.excessBytes??0)>0;
 const retire=async(identities:readonly string[],limit:number,onIdentity:(identity:string,index:number,total:number)=>Promise<void>)=>{
  const{withRepositoryCacheStoreMaintenanceV1}=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts"),{retireQuiescentManagedToolingGenerationsV1}=await import("../../../⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🟦️.ts"),request={version:1 as const,mode:"count-bound" as const,identities,limit};
  const scope=(store:Parameters<typeof retireQuiescentManagedToolingGenerationsV1>[1],actual:typeof phaseControl)=>retireQuiescentManagedToolingGenerationsV1(root,store,actual,request,onIdentity);
  try{const result=await withRepositoryCacheStoreMaintenanceV1(root,"tools",invocation,phaseControl,scope);return{retired:result.retired,held:result.held,kept:result.kept};}
  catch(error){if(String((error as Error).message)==="Quiescent pre-pin cache storage admission required")return "busy" as const;throw error;}
 };
 const ports={signal,now:()=>Date.now(),progress,census:()=>censusNxToolingStoreV1(root,mount,{signal,progress}),held:(identity:string)=>probeNxToolingHeldV1(leaseDirectory,identity,signal),pressure:pressed,freeBytes:free,idle:()=>probeNxToolingImageIdleV1(leaseDirectory,join(root,budgetPolicy.partitions.tools.directory),signal),sleep:(milliseconds:number)=>new Promise<void>((done,fail)=>{const timer=setTimeout(done,milliseconds);signal.addEventListener("abort",()=>{clearTimeout(timer);fail(signal.reason);},{once:true});}),...dryRun?{}:{retire}};
 try{return formatNxToolingStoreBoundV1(language,await runNxToolingStoreBoundV1(dryRun,ports,policy)).join("\n");}finally{single.release();}
}

/** 🏗️ Binds the command to the running build CLI: its invocation for progress and cancellation, and the lease directory of the leases partition for the area leases. */
export async function runCacheLedgerV1(root:string,args:readonly string[],invocation:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts").ResourceLeaseControl):Promise<void>{
 const routing=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),leases=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts"),opening=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts"),partitions=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts"),headroom=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🧹️headroom/🟦️.ts"),directory=opening.repositoryLeaseDirectoryV1(root);
 const ports:CacheRunnerPortsV1=cacheRunnerPortsV1({
  signal:invocation.control.signal,
  progress:stage=>routing.advanceScriptInvocation(invocation,"cache:"+stage,"running"),
  exclusive:async resource=>{
   try{return await leases.acquireResourceLease({directory,resource,mode:"exclusive",signal:phaseControl.signal,control:phaseControl,timeoutMs:0});}
   catch(error){if(/timed out/iu.test(String((error as Error).message)))return undefined;throw error;}
  },
  delegate:(owner,id,dryRun,language,context)=>{
   if(owner==="nx-tooling"&&id==="nx-tooling")return nxToolingReclaim(root,dryRun,language,phaseControl,invocation,context?.machine??null,context?.selected??false).catch(error=>{if(invocation.control.signal.aborted)throw error;return "failed: "+String((error as Error).message);});
   if(owner==="graph"&&id==="graph-inference")return routing.withRequestedIssuedResourceControlV1(phaseControl,CACHE_GC_LIMITS_V1.phaseBudgetMilliseconds,control=>graphReclaim(root,dryRun,language,control,leases,opening,partitions,headroom));
   if(owner==="cargo"&&id==="cargo-store")return cargoIncrementalReclaim(root,dryRun,language,{...ports,signal:phaseControl.signal},context?.machine??null).catch(error=>{if(phaseControl.signal.aborted)throw error;return "failed: "+String((error as Error).message);});
   if(owner==="test-evidence"&&id==="test-runs")return testRunsReclaim(root,dryRun,language,phaseControl,invocation).catch(error=>"failed: "+String((error as Error).message));
   if(owner==="remote-execution")return import("../../🌐️remote-execution/👷️worker/🟦️.ts").then(module=>module.collectRemoteExecutionForLedgerV1(root,dryRun,phaseControl.signal,id)).catch(error=>"failed: "+String((error as Error).message));
   if(owner==="remote-cache")return import("../../🌐️remote-cache/⌨️command/🟦️.ts").then(module=>module.collectRemoteCacheForLedgerV1(root,dryRun,phaseControl.signal)).catch(error=>"failed: "+String((error as Error).message));
   return import("../../🔌️backends/🟦️.ts").then(async registry=>registry.buildBackendIdsV1().includes(owner)?routing.withRequestedIssuedResourceControlV1(phaseControl,CACHE_GC_LIMITS_V1.phaseBudgetMilliseconds,async control=>(await(await import("../../🔌️backends/🚚️loading/🟦️.ts")).loadBuildBackendV1(owner)).collectCache?.({root,invocation,phase:control,dryRun,signal:control.signal,area:id,language})):undefined);
  }
 });
 const verify=async(control:import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts").MemberScanControlV1)=>{const scan=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts"),image=await partitions.openRepositoryCachePartitionV1(root,"graph",phaseControl);try{return await scan.verifyBuildGraphMembersDeepV1(image.root,control);}finally{await image.close();}};
 const wholeGc=args[0]==="gc"&&!args.some(arg=>arg==="--area"||arg.startsWith("--area="));
 const dependencyArea=args.some(arg=>arg.startsWith("--area=")&&arg.slice(7).split(",").includes("dependencies")),collectDependencies=async(dryRun:boolean,bounded:boolean):Promise<void>=>{
  const{collectUnpinnedDependencyGenerationsV1}=await import("../../../⚡️caching/🚀️bootstrap/📦️dependencies/♻️collection/🟦️.ts"),{concurrentPinResourceV1}=await import("../../../⚡️caching/🚀️bootstrap/📦️dependencies/🚦️concurrent/🟦️.ts"),{resolve:resolvePath}=await import("node:path"),store=join(root,...budgetPolicy.partitions.dependencies.directory.split("/"),"m");
  if(!existsSync(store))return;
  try{
   const result=await collectUnpinnedDependencyGenerationsV1(store,{dryRun,signal:invocation.control.signal,...bounded?{maximumMilliseconds:CACHE_GC_LIMITS_V1.idleCompactionMaximumMilliseconds}:{},progress:row=>routing.advanceScriptInvocation(invocation,"cache:dependencies-"+row.phase+"-"+row.done+"-"+row.total+"-"+row.entries,"running"),acquire:async key=>{try{return await leases.acquireResourceLease({directory,resource:concurrentPinResourceV1(resolvePath(root),key),mode:"exclusive",signal:invocation.control.signal,control:phaseControl,timeoutMs:0});}catch(error){if(/timed out/iu.test(String((error as Error).message)))return null;throw error;}}});
   if(result.refused!==undefined)writeResultLineV1("Dependency generations kept: "+result.refused+" / Abhängigkeits-Generationen behalten: "+result.refused);
   else if(result.retired.length>0||result.finished.length>0){const finished=result.finished.length>0?" and "+(dryRun?"would finish ":"finished ")+result.finished.length+" interrupted removal(s)":"",fertig=result.finished.length>0?" und "+result.finished.length+" unterbrochene Entfernung(en) "+(dryRun?"würden beendet":"beendet"):"";writeResultLineV1((dryRun?"Would retire ":"Retired ")+result.retired.length+" dependency generation(s)"+finished+(result.pressure?" (image under pressure)":"")+" / "+result.retired.length+" Abhängigkeits-Generation(en) "+(dryRun?"würden entfernt":"entfernt")+fertig+(result.pressure?" (Image unter Druck)":""));}
   if(result.more)writeResultLineV1("More dependency generations follow in the next pass / Weitere Abhängigkeits-Generationen folgen im nächsten Durchgang");
  }catch(error){if(invocation.control.signal.aborted)throw error;writeResultLineV1("Dependency generations not collected: "+String((error as Error).message)+" / Abhängigkeits-Generationen nicht gesammelt: "+String((error as Error).message));}
 };
 const idleOnly=args.includes("--idle"),reclaimImages=async(order:readonly RepositoryCachePartitionIdV1[])=>{
  const reclaimDue=(await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts")).repositoryPartitionReclaimDueV1,{createHash}=await import("node:crypto"),{tmpdir}=await import("node:os"),{mkdirSync,readFileSync,writeFileSync}=await import("node:fs"),{compactOnIdleV1}=await import("../🛌️idle/🟦️.ts");
  const stamps=join(tmpdir(),"semio-idle-compaction-"+createHash("sha256").update(root).digest("hex").slice(0,16));
  if(args.includes("--dry-run")){for(const partition of order)if(reclaimDue(root,partition))writeResultLineV1("Would reclaim the host blocks of image "+partition);return;}
  const{refreshStaleImageIfQuiescentV1}=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🔄️refresh/🟦️.ts"),{repositoryCacheBudgetV1}=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts");
  for(const partition of order){
   const declared=repositoryCacheBudgetV1(root).partitions[partition];if(declared===undefined)continue;
   const refreshed=await refreshStaleImageIfQuiescentV1({directory:declared.directory,leaseDirectory:directory,control:phaseControl});
   if(refreshed.outcome==="refreshed")writeResultLineV1("Refreshed the available space of image "+partition+": "+(refreshed.beforeAvailableBytes===undefined?"?":formatCacheBytesV1(refreshed.beforeAvailableBytes))+" -> "+(refreshed.afterAvailableBytes===undefined?"?":formatCacheBytesV1(refreshed.afterAvailableBytes))+(refreshed.cured?"":" (still stale)")+" / Verfügbarer Platz von Image "+partition+" aufgefrischt"+(refreshed.cured?"":" (weiterhin veraltet)"));
  }
  const results=await compactOnIdleV1({partitions:order,minimumIntervalMilliseconds:idleOnly?CACHE_GC_LIMITS_V1.idleCompactionIntervalMilliseconds:0,maximumMilliseconds:CACHE_GC_LIMITS_V1.idleCompactionMaximumMilliseconds,signal:invocation.control.signal,ports:{
   now:Date.now,due:partition=>reclaimDue(root,partition as RepositoryCachePartitionIdV1),
   lastAt:partition=>{try{return (JSON.parse(readFileSync(join(stamps,partition+".json"),"utf8")) as {at:number}).at;}catch{return undefined;}},
   stamp:(partition,at)=>{try{mkdirSync(stamps,{recursive:true});writeFileSync(join(stamps,partition+".json"),JSON.stringify({at}));}catch{}},
   reclaim:async partition=>{
    const outcome=await partitions.reclaimImageIfQuiescentV1(invocation,phaseControl,partition as RepositoryCachePartitionIdV1,(done,total)=>routing.advanceScriptInvocation(invocation,"cache:reclaim-"+partition+"-"+done+"-"+total,"running"));
    return outcome.outcome==="skipped"?{outcome:"skipped",reason:outcome.reason}:{outcome:"reclaimed",beforeBytes:outcome.beforeBytes,afterBytes:outcome.afterBytes};
   }}});
  for(const row of results){
   if(row.result==="compacted")writeResultLineV1("Reclaimed image "+row.partition+": "+formatCacheBytesV1(row.beforeBytes!)+" -> "+formatCacheBytesV1(row.afterBytes!)+" on the host");
   else if(row.result==="busy"||row.result==="failed")writeResultLineV1("Image "+row.partition+" not compacted: "+(row.message??row.result)+" / Image "+row.partition+" nicht verdichtet: "+(row.message??row.result));
  }
 };
 const retireIdle=async():Promise<void>=>{
  const signal=invocation.control.signal;
  await import("../../🌐️remote-execution/👷️worker/🟦️.ts").then(async module=>writeResultLineV1("Remote execution worker home: "+await module.collectRemoteExecutionForLedgerV1(root,false,signal,"remote-execution-worker")),()=>undefined).catch(()=>undefined);
  const registry=await import("../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts"),order=await import("../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts");
  const pruned=await registry.pruneLeaseRegistryIfDueV1(registry.leaseRegistryRootV1(),order.leaseOrderPolicyV1(process.env),Date.now(),signal).catch(()=>undefined);
  if(pruned!==undefined)writeResultLineV1("Lease registry: removed "+pruned.removedFolders+" folders and "+pruned.removedEntries+" rows"+(pruned.more?" (more follow)":"")+" / Lease-Register: "+pruned.removedFolders+" Ordner und "+pruned.removedEntries+" Zeilen entfernt"+(pruned.more?" (weitere folgen)":""));
 };
 if(idleOnly){await collectDependencies(false,true);await retireIdle();await reclaimImages(["rust","dependencies","graph","products","tools","diagnostics","nx"]);return;}
 if(wholeGc&&!args.includes("--dry-run"))await reclaimImages(["rust"]);
 const code=await runCacheLedgerCommandV1(args,{workspace:root,environment:process.env,write:writeResultLineV1,verify},ports);
 if(code!==0)process.exitCode=code;
 else if(wholeGc||dependencyArea){await collectDependencies(args.includes("--dry-run"),false);if(wholeGc)await reclaimImages(["rust","dependencies","graph","products","tools","diagnostics","nx"]);}
}
