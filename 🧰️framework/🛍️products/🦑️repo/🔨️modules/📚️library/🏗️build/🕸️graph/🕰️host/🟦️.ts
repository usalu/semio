import type {BuildGraphPartStubV1,BuildGraphV1} from "../📋️model/🟦️.ts";
import * as observationV1 from "../../👁️observation/🟨️.mjs";
import {encodeGenerationPinV1,generationPinOwnerV1} from "../🕰️generations/🧷️pin/🟨️.mjs";
import {createGraphPhaseClockV1,writeGraphPhaseRowV1} from "../⏱️phases/🟨️.mjs";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {type BuildOwnerRegistryV1} from "../🚪️admission/🟦️.ts";
import {readdir,readFile,lstat} from "node:fs/promises";
import {join,dirname,relative,isAbsolute,resolve,sep} from "node:path";
import {randomBytes,createHash} from "node:crypto";
import {checkScriptInvocation,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {acquireQueuedResourceLease,acquireResourceLease} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {buildGraphWriterResourceV1} from "../../📥️inputs/💾️storage/🔒️writer/🟨️.mjs";
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {GENERATION_POLICY_V1,createFrontierV1,diffFrontiersV1,frontierHoldsV1,stableDirectoriesV1,canonicalJsonV1 as canonicalGenerationJsonV1} from "../🕰️generations/🟨️.mjs";
import {advanceFrontierV1} from "../🕰️generations/🌿️frontier-walk/🟨️.mjs";
import {type SliceClaimV1} from "../🕰️generations/🔐️custody/🟨️.mjs";
import {markBuildCapacityPhaseV1,upgradeBuildCapacityBeforeRebuildV1} from "../../🌡️host-capacity/🚏️marks/🟦️.ts";
import {renderGenerationMessageV1,survivingStubsV1} from "../🕰️generations/🔌️load/🟨️.mjs";
import {environmentRowHoldsV1} from "../🕰️generations/🌍️environment/🟨️.mjs";
import {createDirectoryStampsV1} from "../🕰️generations/🗂️directory-stamps/🟨️.mjs";
import {createEpochDoubtV1} from "../🕰️generations/🔎️doubt/🟨️.mjs";
import type {EnvironmentStoredPartsV1} from "../🕰️generations/🧵️edge-reads/🟨️.mjs";
import {graphStringInternerV1,createMemoryPortsV1} from "../../🧠️memory/🟨️.mjs";
import {journalGraphSeamsV1} from "../../🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs";
import {directJournalPortV1,installFingerprintV1,journalCursorStoreV1,renderJournalNoticeV1} from "../../🏡️resident/🕰️graph-hooks/📖️direct/🟦️.ts";
import {requireCurrentFormatStampsV1} from "../../🧷️format-stamps/🟨️.mjs";
import {resolveLanguageV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {CATALOG_LANGUAGES_V1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {pendingCrateManifestV1,buildGraphGenerationIdentityV1} from "../🏷️retention/🟦️.ts";
import {buildWalkDescendsV1,graphMemberPredicateV1,workspaceBuildFilesV1,buildGraphMembersV1,portableInputs} from "../📁️workspace/🟦️.ts";
import {inventoryDirectoriesV1} from "../🕊️release/🟦️.ts";
import {graphDeclarationV1,graphFormatStampsV1,loadedGraphImplementationRowsV1,GRAPH_LIMITS_V1} from "../🔐️authority/🟦️.ts";
import {setupSharesV1,type GenerationPrefetchRequestV1,builderIdleV1,previousEnvironmentOwnersV1,prefetchStaleGenerationV1} from "../🤝️prefetch/🟦️.ts";
import {withGenerationCustodyV1,generationFilesV1} from "../🛑️custody/🟦️.ts";
import {discoverBuildGraphPartV1} from "../🧭️discovery/🟦️.ts";
import {readBuildGraphLeanV1,readBuildGraphV1} from "../📜️contract/🟦️.ts";
import {recordSealedGenerationPinV1} from "../📍️pin/🟦️.ts";


/** 🕰️ Binds the repository to the graph generation load: shared reader custody, the builder custody, the bounded facts lease, fact checks that understand environment rows, the membership frontier and the partial inference. */
export function generationHostV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,registry:BuildOwnerRegistryV1,compact=false){
  const registryDigest=createHash("sha256").update(JSON.stringify(registry)).digest("hex"),state={files:[] as string[],stable:new Set<string>(),carry:false},leaseDirectory=join(root,".cache/b/l/m/data");
  const read=async(path:string)=>{try{return await readFile(join(root,path),"utf8");}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return undefined;throw error;}};
  const statFile=async(path:string)=>{try{const status=await lstat(join(root,path));return status.isFile()?{size:status.size,mtimeMs:status.mtimeMs}:undefined;}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return undefined;throw error;}};
  const scan={on:false,done:0,total:0,tick:undefined as undefined|((done:number)=>void)};
  const statEntry=async(path:string)=>{try{const status=await lstat(join(root,path));if(scan.on)scan.tick?.(++scan.done);return {directory:status.isDirectory(),file:status.isFile(),mtimeMs:status.mtimeMs,ino:status.ino,mode:status.mode};}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return undefined;throw error;}};
  const listEntry=async(directory:string)=>{try{return(await readdir(join(root,directory),{withFileTypes:true})).map(entry=>({name:entry.name,kind:(entry.isSymbolicLink()?"other":entry.isDirectory()?"directory":entry.isFile()?"file":"other") as "directory"|"file"|"other"}));}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return undefined;throw error;}};
  let analyses:Promise<void>|undefined;
  const analysesPrepared=():Promise<void>=>analyses??=import("../📥️inference/🟨️.mjs").then(async producer=>{await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();});
  let deriver:Promise<(row:{kind:string;path:string;fact:unknown},signal?:AbortSignal,pass?:object)=>Promise<{fact:unknown;rows:readonly {kind:string;path:string;fact:unknown}[]}>>|undefined;
  const deriveAgain=async(row:{kind:string;path:string;fact:unknown},signal?:AbortSignal,pass?:object)=>{deriver??=import("../📥️inference/🟨️.mjs").then(async producer=>{await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();return producer.openGenerationDeriverV1({workspaceRoot:root,registry});});return(await deriver)(row,signal,pass) as never;};
  const holdsByStat=async(rows:readonly {kind:string;path:string;fact:any}[],signal?:AbortSignal):Promise<boolean>=>{
    const plain:typeof rows[number][]=[],own:typeof rows[number][]=[];for(const row of rows){if(row.kind==="closure")continue;(row.kind==="digest"||row.kind==="field"&&row.fact?.extract!==undefined?own:plain).push(row);}
    for(let at=0;at<own.length;at+=64){signal?.throwIfAborted();if(!(await Promise.all(own.slice(at,at+64).map(row=>environmentRowHoldsV1(row as never,{read,stat:statFile})))).every(Boolean))return false;}
    if(plain.length===0)return true;
    await analysesPrepared();
    return observationV1.observationsHold(plain,64,state.stable,true,signal);
  };
  const lease=(control:ResourceLeaseControl)=>({async exclusive<T>(resource:string,body:()=>Promise<T>,options:{signal?:AbortSignal;timeoutMs?:number}={}):Promise<T>{
    const held=await acquireQueuedResourceLease({directory:leaseDirectory,resource,mode:"exclusive",owner:randomBytes(16).toString("base64url"),control,signal:options.signal??control.signal,...(options.timeoutMs!==undefined?{timeoutMs:options.timeoutMs}:{})});
    try{return await body();}finally{held.release();}
  }});
  const membershipByStat=async(previous:{manifest:{membership:string};frontier:any}|undefined,signal?:AbortSignal)=>{
    if(previous&&await frontierHoldsV1(previous.frontier,statEntry,{signal})){state.files=previous.frontier.members.map((row:[string,number])=>row[0]);return {digest:previous.manifest.membership,frontier:previous.frontier,changes:[]};}
    const config=(path:string)=>["📋️project.json","Cargo.toml"].includes(path.slice(path.lastIndexOf("/")+1));
    const classified=async(changes:ReturnType<typeof diffFrontiersV1>)=>Promise.all(changes.map(async change=>({...change,...(!change.directory&&config(change.path)?{config:true}:{}),...(!change.directory&&await pendingCrateManifestV1(registry,change.path,read)?{pending:true}:{})})));
    if(previous?.frontier){
      const capturedAt=Date.now(),descends=buildWalkDescendsV1(),advanced=await advanceFrontierV1({frontier:previous.frontier,stat:statEntry,list:listEntry,descend:(name,path,parent)=>descends(name,path,parent==="."?"":parent),member:graphMemberPredicateV1(),signal});
      if(advanced&&!("unchanged"in advanced)){
        const frontier=createFrontierV1({directories:advanced.directories,members:advanced.members,capturedAt}),digest=createHash("sha256").update(canonicalGenerationJsonV1(frontier.members)).digest("hex");
        state.files=frontier.members.map((row:[string,number])=>row[0]);
        return {digest,frontier,changes:await classified(diffFrontiersV1(previous.frontier,frontier))};
      }
    }
    const capturedAt=Date.now(),all=await workspaceBuildFilesV1(root,invocation),members=buildGraphMembersV1(all),directoryPaths=inventoryDirectoriesV1.get(all)??[];
    const stamp=async<T>(paths:readonly string[],row:(path:string,status:NonNullable<Awaited<ReturnType<typeof statEntry>>>)=>T):Promise<T[]>=>{
      const out:T[]=new Array(paths.length);let next=0;
      await Promise.all(Array.from({length:Math.min(64,paths.length)},async()=>{while(next<paths.length){const at=next++;checkScriptInvocation(invocation);const status=await statEntry(paths[at]!);if(!status)throw Object.assign(Error(`Build member removed during discovery: ${paths[at]}`),{code:"BUILD_MEMBERSHIP_CHANGED"});out[at]=row(paths[at]!,status);}}));
      return out;
    };
    const directories=await stamp(directoryPaths,(path,status)=>[path,status.mtimeMs,status.ino] as [string,number,number]),stamped=await stamp(members,(path,status)=>[path,status.mode] as [string,number]);
    const frontier=createFrontierV1({directories,members:stamped,capturedAt}),digest=createHash("sha256").update(canonicalGenerationJsonV1(frontier.members)).digest("hex");
    state.files=members;
    return {digest,frontier,changes:previous?await classified(diffFrontiersV1(previous.frontier,frontier)):[]};
  };
  let clockOf:undefined|{begin(phase:string,total:number):unknown;advance(phase:string,done:number):unknown;end(phase:string):unknown};
  const membershipScanned=async(previous:Parameters<typeof membershipByStat>[0],signal?:AbortSignal)=>{
    const clock=clockOf,total=previous?.frontier?previous.frontier.directories.length+previous.frontier.members.length:0;
    if(!clock||total===0)return membershipByStat(previous,signal);
    scan.done=0;scan.total=total;scan.on=true;scan.tick=done=>{void Promise.resolve(clock.advance("membership",Math.min(done,total))).catch(()=>undefined);};
    try{await clock.begin("membership",total);}catch{scan.on=false;scan.tick=undefined;return membershipByStat(previous,signal);}
    try{return await membershipByStat(previous,signal);}finally{scan.on=false;scan.tick=undefined;await clock.end("membership");}
  };
  const walkDescends=buildWalkDescendsV1(),notifyJournal=(notice:Parameters<typeof renderJournalNoticeV1>[0])=>{process.stderr.write(renderJournalNoticeV1(notice,resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language)+"\n");};
  const seams=journalGraphSeamsV1({root,journal:directJournalPortV1(root),store:journalCursorStoreV1(root),ports:{list:listEntry,stat:statEntry,descend:(name:string,path:string,parent:string)=>walkDescends(name,path,parent==="."?"":parent),member:graphMemberPredicateV1()},digest:members=>createHash("sha256").update(canonicalGenerationJsonV1(members)).digest("hex"),membership:membershipScanned,holds:holdsByStat,classify:changes=>Promise.all(changes.map(async change=>({...change,...(!change.directory&&await pendingCrateManifestV1(registry,change.path,read)?{pending:true}:{})}))),adopt:(files:string[])=>{state.files=files;},install:async()=>installFingerprintV1(root),notify:notifyJournal}),holds=seams.holds;
  /** 🔎️ Which fact rows need the precise check: the journal's dirty set decides when it vouches, else the directory stamps and the file stamps do (a stat per distinct directory and file, never a call per row); after the check the stamps of the directories whose rows all held are kept for the next pass. */
  const factDoubt=(store:RepositoryCacheStoreV1)=>{
    const epoch=createEpochDoubtV1({stamps:createDirectoryStampsV1({file:join(store.root,GENERATION_POLICY_V1.directoryStampsFile)})});let epochUsed=false;
    return {
      classify:async(rows:readonly unknown[],context:{signal?:AbortSignal;progress?:(event:unknown)=>Promise<void>})=>{const journal=seams.doubtful(rows as never);if(journal)return journal;epochUsed=true;return epoch.classify(rows as never,context);},
      settled:async(input:{rows:readonly unknown[];ids:readonly number[];stale:ReadonlySet<number>})=>{if(epochUsed)await epoch.settled(input as never);},
      published:async(input:{rows:readonly unknown[];observedAt:number})=>epoch.published(input as never)
    };
  };
  const membership=async(previous:Parameters<typeof seams.membership>[0],signal?:AbortSignal)=>{
    const live=await seams.membership(previous,signal);
    state.stable=previous?.frontier?new Set([...stableDirectoriesV1(previous.frontier,live.frontier)].map(path=>join(root,...path.split("/")))):new Set();
    observationV1.useStableDirectoriesV1(state.stable);
    return live;
  };
  return {
    workspaceRoot:root,separator:sep,language:resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,namedInputs:graphDeclarationV1("../🔣️policy.json").namedInputs,registryDigest,signal:invocation.control.signal,verbose:process.env.SEMIO_GRAPH_PHASES==="1"||process.env.SEMIO_BUILD_PROFILE_INPUTS==="1",
    async externalDependencies(){const{nxBuildEngineInputsV1}=await import("../../../🟨️.mjs");return["generic","native"].flatMap(family=>nxBuildEngineInputsV1(family as "generic"|"native").flatMap(row=>(row as {externalDependencies?:readonly string[]}).externalDependencies??[]));},
    async preflight(){const{canonicalTestAggregatePolicyV1:policy,requireCanonicalTestAggregateV1}=await import("../../../⚡️caching/🗂️suites/🧮️aggregate/🟦️.ts");let text:string;try{text=await readFile(join(root,policy.root,"📋️project.json"),"utf8");}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return;throw error;}requireCanonicalTestAggregateV1(JSON.parse(text),policy.root);},
    async identity(){requireCurrentFormatStampsV1(graphFormatStampsV1,"graph generation","refuse");return buildGraphGenerationIdentityV1(registry);},
    async implementationRows(){const producer=await import("../📥️inference/🟨️.mjs");await producer.libraryBootstrap;const rows:{kind:"digest";path:string;fact:{sha256:string;size:number;mtimeMs:number}}[]=[];for(const row of [...loadedGraphImplementationRowsV1,...producer.loadedImplementationRowsV1]){const inside=relative(root,row.path);if(inside===""||inside.startsWith("..")||isAbsolute(inside))continue;rows.push({...row,path:inside.split(sep).join("/")});}return producer.classifyImplementationRowsV1(rows);},
    release:async()=>{observationV1.releaseInferenceMemoryV1();(await import("../../📥️inputs/🟨️.mjs")).releaseInputMemoryV1();if(!state.carry){(await import("../📥️inference/🧭️script-inputs/🟨️.mjs")).resetClosureEntriesV1();setupSharesV1.delete(invocation);}graphStringInternerV1.clear();createMemoryPortsV1().collect();},
    releaseCutoff:async()=>{observationV1.releaseInferenceMemoryV1();graphStringInternerV1.clear();createMemoryPortsV1().collect();},
    carryClosures:(on:boolean)=>{state.carry=on;},
    builderIdle:(request:GenerationPrefetchRequestV1)=>builderIdleV1({root,invocation,phaseControl,request}),
    withShared:<T>(body:(store:RepositoryCacheStoreV1,control:ResourceLeaseControl)=>Promise<T>)=>withRepositoryCacheStoreV1(root,"graph",invocation,phaseControl,async(store,control)=>{await store.recheck();checkResourceLeaseControl(control);return body(store,control);}),
    withBuilder:async<T>(build:(store:RepositoryCacheStoreV1,control:ResourceLeaseControl,claim:SliceClaimV1)=>Promise<T>,request?:{readonly dirty:readonly string[];readonly structural:boolean})=>{
      const body=async(store:RepositoryCacheStoreV1,control:ResourceLeaseControl,claim:SliceClaimV1)=>{await markBuildCapacityPhaseV1("rebuild");return build(store,control,claim);};
      await upgradeBuildCapacityBeforeRebuildV1();
      let image:string|undefined;
      const result=await withGenerationCustodyV1(root,invocation,phaseControl,request,(store,control,claim)=>{image=store.root;return body(store,control,claim);});
      await markBuildCapacityPhaseV1("plan");
      if(image!==undefined)await sweepSupersededBlocksV1(root,image,invocation,phaseControl,(key,fields)=>process.stderr.write(renderGenerationMessageV1(key as never,fields,resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language)+"\n"));
      return result;
    },
    storeOptions:(store:RepositoryCacheStoreV1,control:ResourceLeaseControl)=>({root:join(store.root,"generations"),fs:generationFilesV1(store),lease:lease(control),holds,doubt:factDoubt(store),derive:deriveAgain,resources:{facts:"repository-graph-facts:"+resolve(root)}}),
    membership,
    prime:async(signal:AbortSignal|undefined)=>{const inference=await import("../📥️inference/🟨️.mjs");await inference.libraryBootstrap;await inference.primeSharedClosuresOfWorkspaceV1({workspaceRoot:root,signal});},
    discoverPart:(spec:{cold:boolean;owners:string[];discover:string[];removed:string[];changed?:string[];previousEnv:string|null;previousParts?:EnvironmentStoredPartsV1;stubs:Record<string,BuildGraphPartStubV1>},signal:AbortSignal|undefined,control:ResourceLeaseControl,ownerDone?:()=>void,ownerRunning?:(running:readonly {name:string;seconds:number;kind?:string;rows?:number}[],decision?:unknown)=>unknown)=>{signal?.throwIfAborted();requireCurrentFormatStampsV1(graphFormatStampsV1,"graph generation");return discoverBuildGraphPartV1(root,registry,invocation,control,state.files,{cold:spec.cold,ownerDone,ownerRunning,changed:spec.changed,roots:new Set([...spec.owners.map(name=>spec.stubs[name]!.root),...spec.discover.map(path=>dirname(path))]),stubs:survivingStubsV1(spec.stubs,spec.removed),previousEnv:spec.previousEnv,previousParts:spec.previousParts,previousOwners:previousEnvironmentOwnersV1(spec.stubs)});},
    expand:async(projects:Record<string,unknown>,control:ResourceLeaseControl)=>compact?projects:(await portableInputs(root,projects,invocation,control)) as Record<string,unknown>,
    expansionLimits:{maximumProjects:32,maximumBytes:Math.floor(graphDeclarationV1("../👁️observation/📥️reuse/🧬️schema/🔣️.json").definitions.Policy.const.maximumTextBytes/4)},
    readLean:(value:unknown)=>readBuildGraphLeanV1(value),
    readGraph:(value:unknown)=>readBuildGraphV1(value),
    sealed:(record:{generation:number;manifest:string;owners:readonly string[];covered:ReadonlySet<string>|undefined;atMs:number})=>{seams.commit(record);recordSealedGenerationPinV1(encodeGenerationPinV1({owner:generationPinOwnerV1(process.env),...record}));},
    clock:(control:ResourceLeaseControl)=>{const clock=createGraphPhaseClockV1({publish:resource=>control.publish({resource,mode:"shared",elapsedMs:0}),write:writeGraphPhaseRowV1});clockOf=clock;return Object.freeze({...clock,end:async(phase:string,details?:Record<string,unknown>)=>{const row=await clock.end(phase,details);if(phase==="facts")seams.factsVerified();return row;}});},
    scope:(full:boolean)=>{seams.scope(full);},
    say:(key:string,fields:Record<string,string|number>)=>{process.stderr.write(renderGenerationMessageV1(key as never,fields,resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language)+"\n");},
    async prefetch(request:GenerationPrefetchRequestV1){await upgradeBuildCapacityBeforeRebuildV1();return prefetchStaleGenerationV1({root,invocation,phaseControl,registry,files:()=>state.files,request,release:()=>this.release(),language:resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,book:()=>markBuildCapacityPhaseV1("rebuild")});},
  };
}


/** 🧹️ After a build, removes the observation blocks no record names any more once a record directory holds many more blocks than its records can name (each re-inference supersedes blocks, and a graph store whose inventory is exhausted refuses every command), outside the builder custody, group by group under the writer lease of the group, and never lets a failing sweep fail the build. */
async function sweepSupersededBlocksV1(root:string,image:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,say:(key:string,fields:Record<string,string|number>)=>void):Promise<void>{
  try{
    const scan=await import("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts"),directory=join(root,".cache/b/l/m/data");
    const timedOut=(error:unknown)=>/timed out/iu.test(String((error as Error).message)),resourceOf=(kind:"selection"|"dataset"|"canonical",identity:string)=>buildGraphWriterResourceV1(root,kind,identity);
    const report=await scan.supersedeBuildGraphObservationBlocksV1(image,{lease:scan.createSweepLeasePortV1({
      acquire:async(kind,identity)=>{try{return await acquireResourceLease({directory,resource:resourceOf(kind,identity),mode:"exclusive",signal:invocation.control.signal,control:phaseControl,timeoutMs:0});}catch(error){if(timedOut(error))return undefined;throw error;}},
      acquireQueued:async(kind,identity,timeoutMs)=>{try{return await acquireQueuedResourceLease({directory,resource:resourceOf(kind,identity),mode:"exclusive",owner:randomBytes(16).toString("base64url"),signal:invocation.control.signal,control:phaseControl,timeoutMs});}catch(error){if(timedOut(error))return undefined;throw error;}},
      waitMilliseconds:GENERATION_POLICY_V1.sweepWaitMilliseconds})},{cancelled:()=>invocation.control.signal.aborted,progress:()=>{}},{minimumBlocks:GRAPH_LIMITS_V1.sweepMinimumBlocks,blocksPerHeader:GRAPH_LIMITS_V1.sweepBlocksPerHeader});
    if(report.removed)say("swept",{removed:report.removed,megabytes:Math.round(report.bytes/1048576),headers:report.headers});
  }catch(error){if(invocation.control.signal.aborted)throw error;process.stderr.write("graph: sweep of superseded observation blocks failed: "+String((error as Error).message)+"\n");}
}


/** 🔍️ Names the first difference between a generation graph and the graph a full inference produced; the diagnostic oracle of `SEMIO_BUILD_GRAPH_VERIFY=1`. */
export function requireEqualBuildGraphsV1(actual:BuildGraphV1,expected:BuildGraphV1):void{
  const same=(left:unknown,right:unknown)=>canonicalGenerationJsonV1(left)===canonicalGenerationJsonV1(right);
  for(const name of new Set([...Object.keys(actual.projects),...Object.keys(expected.projects)]))if(!same(actual.projects[name],expected.projects[name]))throw Error(`Graph generation differs from full inference at project ${name}`);
  const edges=(graph:BuildGraphV1)=>graph.dependencies.map(edge=>canonicalGenerationJsonV1(edge)).sort();
  if(!same(edges(actual),edges(expected)))throw Error("Graph generation differs from full inference at dependencies");
  for(const key of["externalNodes","unavailable","namedInputs"] as const)if(!same(actual[key]??null,expected[key]??null))throw Error(`Graph generation differs from full inference at ${key}`);
}
