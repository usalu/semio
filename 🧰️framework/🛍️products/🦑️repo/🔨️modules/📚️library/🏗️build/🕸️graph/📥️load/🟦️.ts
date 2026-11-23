import{machinePathV1}from"../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import type {BuildGraphV1} from "../📋️model/🟦️.ts";
import {buildGraphRetryDiagnosticV1,sealVerdictV1,reportSealV1} from "../🔁retry/🟨️.mjs";
import {REVISION_POLICY_V1,captureBoundedSourceRevisionV1,clearPinnedAuthorityV1,physicalSourceOriginV1,withImplementationPinV1,type PhysicalSourceModuleMetaV1} from "../../📥️inputs/🔏️revision/🟨️.mjs";
import * as observationV1 from "../../👁️observation/🟨️.mjs";
import {decodeAuthorityObservationsV1,distinctObservationRowsV1,encodeAuthorityObservationsV1} from "../../👁️observation/🔢️row-table/🟨️.mjs";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {repositoryBuildGraphWriterDirectoryV1,withBuildGraphOptionalPublicationsV1,withRepositoryBuildGraphConsumerV1} from "../../📥️inputs/💾️storage/🟨️.mjs";
import {settleOwnerRegistryV1,withOwnerRegistryCustodyV1} from "./🧭️registry-follow/🟦️.ts";
import {ownerRegistryDigestV1,askOwnerRegistryV1} from "../🧾️registry-reads/🟨️.mjs";
import {readBuildOwnerRegistryV1,readBuildGraphSelectionV1,closeBuildOwnerAdmissionV1,buildGraphScopeKeyV1,type BuildGraphSelectionV1,type OwnerObservationRequestV1,type BuildOwnerRegistryV1,type BuildOwnerAdmissionV1} from "../🚪️admission/🟦️.ts";
import {selectLiteralBuildProjectsV1} from "../../🧭️selection/📥️options/🟦️.ts";
import {readdir,readFile,open,mkdir,writeFile,rename,lstat,rm} from "node:fs/promises";
import {constants} from "node:fs";
import {withArtifactDescriptorV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import {join,dirname,isAbsolute} from "node:path";
import {randomUUID,randomBytes,createHash} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {checkScriptInvocation,advanceScriptInvocation,requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {acquireQueuedResourceLease} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {decodeGraphRecordsV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📇️records/🟨️.mjs";
import {readGenerationExternalInputsV1,resolveGenerationExternalInputV1,canonicalJsonV1 as canonicalGenerationJsonV1} from "../🕰️generations/🟨️.mjs";
import {loadGenerationGraphV1,type GenerationLoadStatsV1} from "../🕰️generations/🔌️load/🟨️.mjs";
import {graphSnapshotPolicyV1,GRAPH_LIMITS_V1,graphDeclarationV1,graphPinSchemaV1,buildGraphProducerAuthorityV1,loadedGraphModuleAuthorityV1,loadedGraphAdmissionFormatV1,graphSelfUrlsV1} from "../🔐️authority/🟦️.ts";
import {openOwnerCheckpointV1} from "./💾️owner-checkpoint/🟦️.ts";
import {createPassReuseV1} from "./♻️pass-reuse/🟦️.ts";
import type {TakenMemoV1} from "../📥️inference/📦️project-nodes/♻️taken-memo/🟨️.mjs";
import {settleInventoryRoundsV1,type InventoryVolatileV1} from "./🔁️inventory-rounds/🟦️.ts";
import {graphInventoryTextV1,graphRetryTextV1,graphSettleTextV1,graphDirtyTextV1,graphVolatileTextV1,membershipChangePathsV1,followedPathsV1} from "./🧲️membership-follow/🟦️.ts";
import {awaitPlannerQuietV1} from "./🔕️settle-quiet/🟦️.ts";
import {observeOwnerWithRetryV1,ownerWithheldTextV1,stableOwnerFailureHoldsV1} from "./🛟️owner-isolation/🟦️.ts";
import {readAdmissionMemoV1,writeAdmissionMemoV1,packAdmissionMemoV1,ownersOfAdmissionMemoV1} from "./🧠️admission-memo/🟦️.ts";
import {describeCollectionV1,registerSliceCollectionsV1,sliceReadStepV1} from "../../🧠️memory/📋️slice-report/🟨️.mjs";
import sourceAuthoritySchema from "../../🧬️schema/📥️authority.json" with {type:"json"};
import sliceReportSchema from "../../🧠️memory/📋️slice-report/🧬️schema/🔣️.json" with {type:"json"};
import {loadSelectedThroughFleetV1} from "./🤝️selected-exchange/🔌️attempt/🟦️.ts";
import {portableInputs,buildGraphMembersV1,workspaceBuildFilesV1,buildGraphMembershipV1,buildGraphSourceFactsHoldV1} from "../📁️workspace/🟦️.ts";
import {readBuildGraphV1,buildGraphPathV1} from "../📜️contract/🟦️.ts";
import {inventoryDirectoriesV1} from "../🕊️release/🟦️.ts";
import {discoverBuildGraphV1} from "../🧭️discovery/🟦️.ts";
import {generationHostV1,requireEqualBuildGraphsV1} from "../🕰️host/🟦️.ts";
import {boundGenerationPinV1} from "../📍️pin/🟦️.ts";
import {withBuildGraphStateV1,type BuildGraphCriticalV1} from "../🛑️custody/🟦️.ts";


type OwnerRegistryInvalidationV1={readonly kind:"invalidated";readonly reason:"owner-registry"};

const ownerRegistryInvalidationV1:OwnerRegistryInvalidationV1=Object.freeze({kind:"invalidated",reason:"owner-registry"});

const buildGraphErrorCodeV1=(error:unknown):unknown=>error!==null&&typeof error==="object"&&"code" in error?error.code:undefined;

const isOwnerRegistryInvalidationV1=(value:unknown):value is OwnerRegistryInvalidationV1=>value===ownerRegistryInvalidationV1;

/** 🕯️ The admission facts of the owners this process observed, each with the rows that observation consumed: a later attempt of the same read asks for the same owner again, and a memo whose rows still hold answers without observing it, so one edit re-observes the owners whose rows it touched and no others.
 * @see ../../🧪️tests/🕸️graph/🩹️repair-rounds */
const ownerFactsMemoV1=new Map<string,{readonly facts:unknown;readonly rows:readonly unknown[]}>();

/** 📏️ What the owner memo holds, for the slice memory report: its entries, and its distinct rows beside the row references its entries hold (equal when every entry shares one instance per row).
 * @see ../../🧠️memory/📋️slice-report/🟨️.mjs */
registerSliceCollectionsV1("owner-facts",()=>{const distinct=new Set<unknown>();let held=0;for(const entry of ownerFactsMemoV1.values()){held+=entry.rows.length;for(const row of entry.rows)distinct.add(row);}return [describeCollectionV1("graph:ownerFactsMemo",ownerFactsMemoV1),{...describeCollectionV1("graph:ownerFactsMemoRows ("+held+" held, "+distinct.size+" distinct)",[...distinct].slice(0,64)),entries:distinct.size}];});

function rememberOwnerFactsV1(key:string,entry:{readonly facts:unknown;readonly rows:readonly unknown[]}):void{
  ownerFactsMemoV1.delete(key);ownerFactsMemoV1.set(key,entry);
  while(ownerFactsMemoV1.size>GRAPH_LIMITS_V1.ownerFactsMemoEntries)ownerFactsMemoV1.delete(ownerFactsMemoV1.keys().next().value as string);
}

/** 🧲️ The admission facts of one scope this process remembers and the way a generation of the fleet or the admission memo seeds them: the key text of a fact names the scope, the admission format of this process and the question, so a fact of another machine is stored under this format.
 * @see ./🤝️selected-exchange/🔌️attempt/🟦️.ts */
export const selectedAdmissionFactsV1={
  entries(scope:string):readonly {suffix:string;facts:unknown;rows:readonly unknown[]}[]{const prefix=scope+"\0"+loadedGraphAdmissionFormatV1+"\0";return [...ownerFactsMemoV1].filter(([key])=>key.startsWith(prefix)).map(([key,entry])=>({suffix:key.slice(prefix.length),facts:entry.facts,rows:entry.rows}));},
  seed(scope:string,suffix:string,facts:unknown,rows:readonly unknown[]):void{rememberOwnerFactsV1(scope+"\0"+loadedGraphAdmissionFormatV1+"\0"+suffix,{facts,rows:observationV1.shareObservationRowsV1((Object.isFrozen(rows)?[...rows]:rows) as unknown[])});}
};

/** 🪄️ Answers the admission facts of one owner from the memo while every row of its observation holds, and observes the owner (recording its rows apart) otherwise; the rows of a reused answer are retained by the capture around the caller, so the result stays held to what it rests on. An entry the caller already `trust`s (verified or observed earlier in the same follow-up of one read, whose final seal verifies every row again) answers without a second verification, so a restart of the inventory pays only for the owners that were forgotten.
 * @see ../../🧪️tests/🕸️graph/🩹️repair-rounds
 * @see ../../🧪️tests/🕸️graph/🩹️repair-rounds */
export async function observeOwnerFactsMemoizedV1(input:{readonly key:string;readonly hold:(rows:readonly unknown[])=>Promise<boolean>;readonly observation:Pick<typeof observationV1,"captureObservedSourceInputsV1"|"retainObservedRowsV1">&Partial<Pick<typeof observationV1,"shareObservationRowsV1">>;readonly observe:()=>Promise<unknown>;readonly boundary?:(error:unknown)=>unknown;readonly recover?:()=>Promise<{readonly facts:unknown;readonly rows:readonly unknown[]}|undefined>;readonly persist?:(entry:{readonly facts:unknown;readonly rows:readonly unknown[]})=>void;readonly outcome?:(kind:"memo"|"checkpoint"|"discovery")=>void;readonly owner?:string;readonly aborted?:()=>boolean;readonly foreign?:(error:unknown)=>boolean;readonly onRetry?:Parameters<typeof observeOwnerWithRetryV1>[0]["onRetry"];readonly trust?:{has(entry:object):boolean;add(entry:object):void}}):Promise<unknown>{
  const memoized=ownerFactsMemoV1.get(input.key);
  if(memoized!==undefined&&(input.trust?.has(memoized)===true||await input.hold(memoized.rows))){input.trust?.add(memoized);input.observation.retainObservedRowsV1(memoized.rows as never);input.outcome?.("memo");return memoized.facts;}
  const recovered=await input.recover?.();
  if(recovered!==undefined&&await input.hold(recovered.rows)){input.observation.shareObservationRowsV1?.(recovered.rows as never);rememberOwnerFactsV1(input.key,recovered);input.trust?.add(recovered);input.observation.retainObservedRowsV1(recovered.rows as never);input.outcome?.("checkpoint");return recovered.facts;}
  const observe=input.boundary===undefined?input.observe:async()=>{try{return await input.observe();}catch(error){let classified:unknown;try{classified=input.boundary!(error);}catch{classified=undefined;}throw classified??error;}},fresh=await observeOwnerWithRetryV1({owner:input.owner??input.key,attempt:onFailure=>input.observation.captureObservedSourceInputsV1(observe,onFailure as never,false),hold:input.hold,aborted:input.aborted,foreign:input.foreign,onRetry:input.onRetry}),entry={facts:fresh.value,rows:input.observation.shareObservationRowsV1?.(fresh.observations as never)??fresh.observations};
  input.persist?.(entry);rememberOwnerFactsV1(input.key,entry);input.trust?.add(entry);input.outcome?.("discovery");return fresh.value;
}


/** 🧹️ Forgets the memoized owner facts of `scope` whose rows name one of `paths` or the listing of its parent directory, so a member that vanished or changed type re-observes exactly the owners that read it; the answer is the number forgotten.
 * @see ./🧪️tests/🧲️membership-follow */
export function evictOwnerFactsReferencingV1(scope:string,paths:readonly string[]):number{
  return forgetOwnerFactsReferencingV1(scope,paths).length;
}

/** 🧹️ The dirty set of a moved path: forgets exactly the owners `evictOwnerFactsReferencingV1` forgets and names them, so a follow-up observes those owners and trusts every other.
 * @see ./🔁️inventory-rounds/🟦️.ts */
export function forgetOwnerFactsReferencingV1(scope:string,paths:readonly string[]):string[]{
  if(paths.length===0)return [];
  const named=new Set(paths),listings=new Set(paths.map(path=>path.slice(0,Math.max(path.lastIndexOf("/"),path.lastIndexOf("\\")))));
  const prefix=scope+"\0"+loadedGraphAdmissionFormatV1+"\0";const forgotten:string[]=[];
  for(const [key,entry] of [...ownerFactsMemoV1])if(key.startsWith(prefix)&&entry.rows.some(row=>{const {kind,path}=row as {kind:string;path:string};return named.has(path)||(kind==="readdir"||kind==="readdirNames")&&listings.has(path);})){ownerFactsMemoV1.delete(key);forgotten.push(key.slice(prefix.length).split(" ")[0]!);}
  return [...new Set(forgotten)];
}

/** 📌️ Reads immutable snapshot bytes while the existing original Graph store and exact content reader lease remain held. */
async function withRetainedGraphSnapshotV1<T>(workspace:string,store:RepositoryCacheStoreV1,path:string,control:ResourceLeaseControl,run:(source:string)=>Promise<T>):Promise<T>{
 const policy=graphSnapshotPolicyV1,lease=await acquireQueuedResourceLease({directory:machinePathV1(workspace,".cache/b/l/m/data"),resource:"build-graph:"+path,mode:"shared",owner:randomBytes(16).toString("base64url"),control,signal:control.signal});let failed=false;
 try{await store.recheck();checkResourceLeaseControl(control);const directory=dirname(path),parent=await lstat(directory,{bigint:true}),before=await lstat(path,{bigint:true});checkResourceLeaseControl(control);if(!parent.isDirectory()||parent.isSymbolicLink()||!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size<0n||before.size>BigInt(policy.maximumSnapshotBytes))throw Error("Original bounded Graph snapshot required");
 return await withArtifactDescriptorV1(()=>open(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0)),async file=>{const verify=async()=>{await store.recheck();checkResourceLeaseControl(control);requireNativeStoragePathV1(store.root,path);const linked=await lstat(path,{bigint:true}),opened=await file.stat({bigint:true}),actualParent=await lstat(directory,{bigint:true});checkResourceLeaseControl(control);if(!actualParent.isDirectory()||actualParent.isSymbolicLink()||actualParent.dev!==parent.dev||actualParent.ino!==parent.ino||[linked,opened].some(row=>!row.isFile()||row.isSymbolicLink()||["dev","ino","size","nlink","mtimeNs"].some(key=>row[key as keyof typeof row]!==before[key as keyof typeof before])))throw Error("Retained Graph snapshot descriptor changed");};await verify();const bytes=Buffer.alloc(Number(before.size));try{let offset=0;while(offset<bytes.length){await verify();const row=await file.read(bytes,offset,Math.min(GRAPH_LIMITS_V1.snapshotReadChunkBytes,bytes.length-offset),offset);checkResourceLeaseControl(control);if(!row.bytesRead)throw Error("Retained Graph snapshot shortened");offset+=row.bytesRead;await control.yieldContinuation();}if((await file.read(Buffer.alloc(1),0,1,bytes.length)).bytesRead)throw Error("Retained Graph snapshot grew");await verify();const value=await run(new TextDecoder("utf-8",{fatal:true}).decode(bytes));await verify();return value;}finally{bytes.fill(0);}});
 }catch(error){failed=true;throw error;}finally{try{lease.release();}catch(error){if(!failed)throw error;}}
}


/** 🏪️ Delivers the pinned reusable graph only through its existing private Store consumer and retained exact content descriptor. */
export function withRepositoryBuildGraphPublicationV1<T>(root:string,control:ResourceLeaseControl,body:(graph:BuildGraphV1,digest:string,actual:ResourceLeaseControl)=>Promise<T>):Promise<T>{
 const pin={path:process.env.SEMIO_BUILD_GRAPH_PATH,digest:process.env.SEMIO_BUILD_GRAPH_DIGEST};if(validateJsonSchemaSubset(graphPinSchemaV1,pin).length||!isAbsolute(pin.path!))throw Error("Original immutable build graph path and digest required");
 return withRepositoryBuildGraphConsumerV1(root,control,async(store,actual)=>{await store.recheck();checkResourceLeaseControl(actual);const expected=requireNativeStoragePathV1(store.root,join(store.root,"snapshots",pin.digest+".json"));if(pin.path!==expected)throw Error("Original retained Graph snapshot identity required");return withRetainedGraphSnapshotV1(root,store,expected,actual,async source=>{if(createHash("sha256").update(source).digest("hex")!==pin.digest)throw Error("Pinned build graph digest differs from original authority");let reads=0;const records=dirname(expected),graph=await decodeGraphRecordsV1(Buffer.from(source),async digest=>{if(++reads%64===0)await store.recheck();checkResourceLeaseControl(actual);const path=requireNativeStoragePathV1(store.root,join(records,digest+".gz")),status=await lstat(path);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>graphSnapshotPolicyV1.maximumSnapshotBytes)throw Error("Original bounded Graph record required");return readFile(path);},{policy:graphSnapshotPolicyV1.records as never}) as BuildGraphV1;checkResourceLeaseControl(actual);const value=await body(graph,pin.digest!,actual);await store.recheck();checkResourceLeaseControl(actual);return value;});});
}


type InventoryAwaitPhaseV1="registry-check"|"memo-read"|"memo-seed"|"memo-check"|"scoped-discovery"|"memo-publish"|"catalog"|"membership-hash";
type InventoryAwaitCountsV1={observations?:number;completedOwners?:number;admittedOwners?:number;verifiedMemoRows?:number;files?:number;directories?:number;reused?:boolean;published?:boolean};
type InventoryOwnerStageV1="lookup"|"memo-check"|"discovery";

/** 🔍️ Samples active and completed inventory work, keeping verified memo progress across retries of the same lookup. */
function createGraphInventoryAwaitReportV1(clock=()=>performance.now()){
  const completed=new Map<InventoryAwaitPhaseV1,{phase:InventoryAwaitPhaseV1;elapsedMilliseconds:number}&InventoryAwaitCountsV1>();
  let active:{phase:InventoryAwaitPhaseV1;started:number;counts:InventoryAwaitCountsV1}|undefined;
  let activeOwner:{owner:string;stage:InventoryOwnerStageV1;started:number;operation?:string}|undefined;
  const ownerLimit=sliceReportSchema.definitions.InventoryAwait.properties.activeOwner.properties.owner.maxLength;
  const memoHighWater=new Map<string,number>();
  const outcomes={memo:0,checkpoint:0,discovery:0};
  const elapsed=(started:number,at:number)=>Math.max(0,Math.round(at-started));
  const row=(value:NonNullable<typeof active>,at:number)=>({phase:value.phase,elapsedMilliseconds:elapsed(value.started,at),...value.counts});
  return{
    begin(phase:InventoryAwaitPhaseV1,counts:InventoryAwaitCountsV1={}){const now=clock();if(active)completed.set(active.phase,row(active,now));activeOwner=undefined;active={phase,started:now,counts};},
    ownerLookup(owner:string,key:string){
      if(owner.length>ownerLimit){let count=0,end=0;for(const point of owner){if(count++===ownerLimit)break;end+=point.length;}owner=owner.slice(0,end);}
      const lookup:NonNullable<typeof activeOwner>={owner,stage:"lookup",started:clock()};
      activeOwner=lookup;let digest:string|undefined;
      return{
        stage(stage:InventoryOwnerStageV1){lookup.stage=stage;lookup.started=clock();delete lookup.operation;},
        operation(value:string){if(lookup.stage==="discovery"&&sliceReportSchema.definitions.InventoryAwait.properties.activeOwner.properties.operation.enum.includes(value))lookup.operation=value;},
        verified(rows:number){
          if(activeOwner!==lookup||active?.phase!=="scoped-discovery"||lookup.stage!=="memo-check")return;
          digest??=createHash("sha256").update(key).digest("hex");
          const prior=memoHighWater.get(digest);
          if(rows<=(prior??0)||(prior===undefined&&memoHighWater.size>=GRAPH_LIMITS_V1.ownerFactsMemoEntries))return;
          memoHighWater.set(digest,rows);active.counts.verifiedMemoRows=(active.counts.verifiedMemoRows??0)+rows-(prior??0);
        },
        end(){if(activeOwner===lookup)activeOwner=undefined;},
      };
    },
    count(counts:InventoryAwaitCountsV1){if(active)Object.assign(active.counts,counts);},
    outcome(kind:keyof typeof outcomes){outcomes[kind]++;},
    outcomes(){return {...outcomes};},
    ownerCompleted(){if(active?.phase==="scoped-discovery")active.counts.completedOwners=(active.counts.completedOwners??0)+1;},
    read(){const now=clock();return {version:1,active:active?row(active,now):null,completed:[...completed.values()],...(activeOwner&&{activeOwner:{owner:activeOwner.owner,stage:activeOwner.stage,...activeOwner.operation&&{operation:activeOwner.operation},elapsedMilliseconds:elapsed(activeOwner.started,now)}})};},
  };
}

/** 🎬️ Resolves one candidate while keeping every candidate's memory within its own finite attempt. */
async function loadBuildGraphAttemptV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{selection:BuildGraphSelectionV1;reuse?:boolean;sealedOnly?:boolean;takenMemo?:TakenMemoV1},registry?:BuildOwnerRegistryV1,scope?:string,critical:BuildGraphCriticalV1=work=>work()):Promise<BuildGraphV1|OwnerRegistryInvalidationV1|undefined> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);
  const invalidated=(reason:string,details:Record<string,unknown>={})=>{if(process.env.SEMIO_BUILD_TRACE_AUTHORITY==="true"){let diagnostic;try{diagnostic=buildGraphRetryDiagnosticV1(root,reason,details,path=>captureBoundedSourceRevisionV1(path,REVISION_POLICY_V1.maximumSourceBytes).sha256);}catch{diagnostic=buildGraphRetryDiagnosticV1(root,reason);}process.stderr.write("[graph-retry] "+JSON.stringify(diagnostic)+"\n");}else process.stderr.write("[graph-retry] "+JSON.stringify({version:1,reason,stale:((details.stale as {path:string}[]|undefined)??[]).map(row=>row.path)})+"\n");return undefined;};
  const phaseRows:{phase:string;milliseconds:number}[]=[],timing=(phase:string,started:number,details:Record<string,unknown>={})=>{const row={phase,milliseconds:Math.round(performance.now()-started),...details};phaseRows.push(row);if(process.env.SEMIO_BUILD_PROFILE_INPUTS==="1"||process.env.SEMIO_GRAPH_PHASES==="1")process.stderr.write("[graph-phase] "+JSON.stringify(row)+"\n");};
  const image=async(source:string)=>{let started=performance.now();const decoded=JSON.parse(source);timing("graph-json-parse",started,{bytes:Buffer.byteLength(source)});started=performance.now();const expanded=await portableInputs(root,decoded,invocation,phaseControl);timing("graph-input-expand",started);started=performance.now();const graph=readBuildGraphV1(expanded);timing("graph-schema-validation",started,{projects:Object.keys(graph.projects).length,dependencies:graph.dependencies.length});return graph;};
  if(!registry)throw Error("Original canonical graph owner registry required");
  const registryDigest=ownerRegistryDigestV1(registry);
  const selected=options.selection.kind!=="all",path=scope?join(dirname(buildGraphPathV1(root)),"selected",scope+".json"):buildGraphPathV1(root),authorityPath=scope?join(dirname(path),scope+".authority.json"):join(dirname(path),graphDeclarationV1("../🧬️schema/📣️publication.json").properties.authority.const);
  if(options.sealedOnly){if(selected)return undefined;try{if(!buildGraphProducerAuthorityV1(JSON.parse(await readFile(authorityPath,"utf8")),loadedGraphModuleAuthorityV1))return undefined;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT"||error instanceof SyntaxError)return undefined;throw error;}}
  const check=()=>checkResourceLeaseControl(phaseControl);
  const observation=observationV1,[producer]=await Promise.all([import("../📥️inference/🟨️.mjs"),import("../../../../🧪️test/📦️packages/🟦️typescript/🟨️.mjs"),import("../📥️inference/🛡️policy/🟨️.mjs")]);checkResourceLeaseControl(phaseControl);
  const memoPath=scope?join(dirname(path),scope+".admission.json"):"";
  let recovery:Promise<Awaited<ReturnType<typeof openOwnerCheckpointV1>>|undefined>|undefined;
  const checkpointForScope=()=>recovery??=(async()=>{
    if(!scope)return undefined;
    try{return await openOwnerCheckpointV1(join(repositoryBuildGraphWriterDirectoryV1(root,"inference"),"oc"),scope,loadedGraphAdmissionFormatV1,{check,yieldContinuation:()=>phaseControl.yieldContinuation(),publish:(paths,plan,write)=>withBuildGraphOptionalPublicationsV1(paths,plan,write,undefined)});}catch{check();return undefined;}
  })();
  const inventory=async(report:ReturnType<typeof createGraphInventoryAwaitReportV1>,followUp=false,trust:{current:WeakSet<object>}={current:new WeakSet()},volatile:ReadonlySet<string>=new Set())=>{
    let started=performance.now();
    if(selected&&scope){
      report.begin("registry-check");
      if(ownerRegistryDigestV1(readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))))!==registryDigest)return ownerRegistryInvalidationV1;
      report.begin("memo-read");
    }
    let lent=selected&&scope&&!followUp?await readAdmissionMemoV1(memoPath,scope,value=>value.format===loadedGraphAdmissionFormatV1):undefined;
    const storedAdmission=lent?.admission;
    if(lent!==undefined&&scope!==undefined){
      report.count({observations:lent.observations.length});report.begin("memo-seed");
      const known=new Set(selectedAdmissionFactsV1.entries(scope).map(entry=>entry.suffix));for(const owner of ownersOfAdmissionMemoV1(lent,known))selectedAdmissionFactsV1.seed(scope,owner.suffix,owner.facts,owner.rows);
      report.begin("memo-check",{observations:lent.observations.length});
    }
    const remembered=lent!==undefined&&await buildGraphSourceFactsHoldV1(lent.observations,invocation)?lent:undefined;
    if(lent!==undefined)report.count({reused:remembered!==undefined});
    if(remembered===undefined)lent=undefined;
    if(selected)timing("admission-memo",started,{hit:remembered!==undefined});started=performance.now();
    if(remembered===undefined&&selected)report.begin("scoped-discovery",{completedOwners:0});
    const checkpoint=remembered===undefined&&selected&&scope?await checkpointForScope():undefined;
    const captured=remembered!==undefined?{value:remembered.admission as unknown,observations:remembered.observations}:selected?await observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>{
      const current=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));if(ownerRegistryDigestV1(current)!==registryDigest)return ownerRegistryInvalidationV1;askOwnerRegistryV1(root,registry!,["digest"]);
      const nativeCache=new Map(),nativeClosures=new Map();
      const epoch=randomUUID();let farm:ReturnType<typeof producer.ownerFactsFarmV1>,farmTold=0,farmAsked=false;
      const farmOf=()=>{if(!farmAsked){farmAsked=true;farm=producer.ownerFactsFarmV1?.(root);}return farm;},discoverHere=(owner:OwnerObservationRequestV1["owner"],targets:readonly string[],inputs:readonly string[],lookup:{operation:(value:string)=>void})=>producer.discoverBuildOwnerAdmissionFactsV1(owner,targets,registry,{workspaceRoot:root,invocation,phaseControl,selection:options.selection,nativeCache,nativeClosures,onOwnerDiscoveryPhase:lookup.operation},inputs);
      const discoverThrough=async(owner:OwnerObservationRequestV1["owner"],targets:readonly string[],inputs:readonly string[],lookup:{operation:(value:string)=>void})=>{
        const pool=farmOf();
        if(pool?.ready()===true)try{
          const answer=await pool.discover({owner:owner.name,targets,inputs,selection:options.selection,registryDigest,format:loadedGraphAdmissionFormatV1,epoch},invocation?.control.signal) as {facts:unknown;rows:readonly unknown[]};
          observation.retainObservedRowsV1(observation.shareObservationRowsV1(answer.rows as never[]) as never);return answer.facts;
        }catch(error){if(invocation?.control.signal.aborted===true)throw error;if(farmTold++<3)process.stderr.write("[graph-owner-farm] "+JSON.stringify({version:1,owner:owner.name,fallback:"parent",reason:String((error as Error)?.message??error).slice(0,200)})+"\n");}
        return discoverHere(owner,targets,inputs,lookup);
      };
      const run=async({owner,targets,inputs}:OwnerObservationRequestV1)=>{
        if(invocation)checkScriptInvocation(invocation);
        const suffix=owner.name+"\0"+JSON.stringify([targets,inputs]),key=scope+"\0"+loadedGraphAdmissionFormatV1+"\0"+suffix,lookup=report.ownerLookup(owner.name,key);
        try{
          const facts=await observeOwnerFactsMemoizedV1({
            key,owner:owner.name,aborted:()=>invocation?.control.signal.aborted===true,foreign:error=>followedPathsV1(root,error).length>0,onRetry:event=>{process.stderr.write("[graph-owner-retry] "+JSON.stringify({version:1,...event})+"\n");},
            hold:rows=>{lookup.stage("memo-check");return buildGraphSourceFactsHoldV1(rows,invocation,undefined,lookup.verified,true);},
            observation,
            recover:()=>checkpoint?.read(suffix)??Promise.resolve(undefined),
            persist:entry=>checkpoint?.save(suffix,entry),
            outcome:kind=>report.outcome(kind),
            trust:{has:entry=>trust.current.has(entry),add:entry=>trust.current.add(entry)},
            boundary:error=>producer.ownerBoundaryFailureV1(error,{workspaceRoot:root,fallback:join(root,owner.manifest)}),
            observe:()=>{lookup.stage("discovery");return discoverThrough(owner,targets,inputs,lookup);},
          }) as never;
          report.ownerCompleted();return facts;
        }finally{lookup.end();}
      };
      return closeBuildOwnerAdmissionV1(options.selection,registry!,(owner,targets,inputs)=>run({owner,targets,inputs}) as never,failure=>{process.stderr.write("[graph-owner] "+JSON.stringify({version:1,owners:failure.error.owners,needed:failure.needed,text:failure.needed?failure.error.text:ownerWithheldTextV1(failure.owner,failure.error)})+"\n");},(name,target)=>{const manifest=registry!.owners.find(owner=>owner.name===name)?.manifest;if(manifest===undefined||!manifest.endsWith(".json"))return undefined;try{const targets=(JSON.parse(observation.observedFs.readFileSync(join(root,manifest),"utf8")) as {targets?:Record<string,unknown>}).targets;return targets===undefined?undefined:Object.hasOwn(targets,target);}catch{return undefined;}},summary=>{report.count({admittedOwners:summary.owners});process.stderr.write("[graph-closure] "+JSON.stringify({version:1,...summary})+"\n");},()=>farm?.ready()===true?farm.size:0);
    },{signal:invocation?.control.signal,onRound:async event=>{
      const paths=event.paths??[],settled=event.settled===true,evicted=!settled&&scope?evictOwnerFactsReferencingV1(scope,paths):0,first=paths.slice(0,5).map(path=>path.startsWith(root)?path.slice(root.length+1):path);
      if(paths.length>0)process.stderr.write("[graph-settle] "+JSON.stringify({version:1,round:event.round,settled,paths:paths.length,first,evicted,text:graphSettleTextV1(event.round,paths.length,first.slice(0,3),evicted,settled)})+"\n");
      if(!settled&&paths.length===0)trust.current=new WeakSet();
      if(!settled&&paths.length>0)await awaitPlannerQuietV1({paths,signal:invocation?.control.signal});
    }})):undefined,candidate=captured?.value;
    if(isOwnerRegistryInvalidationV1(candidate))return candidate;
    const owners=report.outcomes(),unchanged=storedAdmission!==undefined&&owners.discovery===0&&owners.checkpoint===0&&JSON.stringify(captured?.value)===JSON.stringify(storedAdmission);
    if(remembered===undefined&&selected&&scope&&captured!==undefined&&!followUp&&!unchanged){
      checkpoint?.flush();
      report.count({observations:captured.observations.length});report.begin("memo-publish",{observations:captured.observations.length});
      if(invocation)checkScriptInvocation(invocation);let published:boolean|undefined;
      try{published=await writeAdmissionMemoV1(memoPath,packAdmissionMemoV1({format:loadedGraphAdmissionFormatV1,scope,admission:candidate as BuildOwnerAdmissionV1,observations:captured.observations,owners:selectedAdmissionFactsV1.entries(scope)}),graphSnapshotPolicyV1.maximumSnapshotBytes);}catch(error){if(invocation)checkScriptInvocation(invocation);}
      if(published!==undefined)report.count({published});timing("admission-memo-write",started);
    }
    report.begin("catalog");
    const admission=candidate as BuildOwnerAdmissionV1|undefined,files=admission?.files??buildGraphMembersV1(await workspaceBuildFilesV1(root,invocation,volatile)),counts={files:files.length,directories:inventoryDirectoriesV1.get(files)?.length??0};
    report.count(counts);timing("workspace-catalog-lookup",started,counts);started=performance.now();
    report.begin("membership-hash",counts);const digest=await buildGraphMembershipV1(root,files,invocation,volatile);timing("workspace-membership-hash",started);
    return {files,digest,admission,observations:captured?.observations??[]};
  };
  const settledInventory=async(orNothing:boolean):Promise<(Awaited<ReturnType<typeof inventory>>&{volatile?:InventoryVolatileV1})|undefined>=>{
    const started=performance.now(),trust={current:new WeakSet<object>()},relative=(path:string)=>(path.startsWith(root)?path.slice(root.length+1):path).split("\\").join("/");
    let attemptStarted=performance.now(),note:{en:string;de:string}|undefined,report=createGraphInventoryAwaitReportV1(),attempts=0;
    const settled=await settleInventoryRoundsV1({
      limits:{repairRounds:GRAPH_LIMITS_V1.repairRounds,volatileRounds:GRAPH_LIMITS_V1.inventoryVolatileRounds,inventoryRounds:GRAPH_LIMITS_V1.inventoryRounds},orNothing,
      attempt:async(round,volatile)=>{
        attempts=round;report=createGraphInventoryAwaitReportV1();attemptStarted=performance.now();
        const value=await sliceReadStepV1("inventory",()=>inventory(report,round>1,trust,new Set([...volatile].map(relative))),{},()=>({...report.read(),...(note&&{note})}));
        if(round>1)process.stderr.write("[graph-inventory] "+JSON.stringify({version:1,round,elapsedMilliseconds:Math.round(performance.now()-started),...report.outcomes(),text:graphInventoryTextV1(round,report.outcomes(),Math.round(performance.now()-started))})+"\n");
        return value;
      },
      follow:error=>{const membership=buildGraphErrorCodeV1(error)==="BUILD_MEMBERSHIP_CHANGED";return {membership,paths:membership?membershipChangePathsV1(root,error):followedPathsV1(root,error)};},
      forget:paths=>scope?forgetOwnerFactsReferencingV1(scope,paths):[],
      check:()=>{if(invocation)checkScriptInvocation(invocation);},
      pause:(round,membership)=>membership?Promise.resolve():new Promise<void>(accept=>setTimeout(accept,Math.min(round,8)*GRAPH_LIMITS_V1.editFollowPauseMilliseconds)),
      dirty:event=>{
        const first=event.paths.map(relative),text=graphDirtyTextV1(event.round,first,event.owners,event.streak),counts=report.outcomes(),elapsed=Math.round(performance.now()-attemptStarted);
        note={en:text.en.slice(0,480),de:text.de.slice(0,480)};
        process.stderr.write("[graph-retry] "+JSON.stringify({version:1,round:event.round,paths:event.paths.length,first:first.slice(0,5),dirtyOwners:event.owners.slice(0,16),dirtyCount:event.owners.length,streak:event.streak,...counts,elapsedMilliseconds:elapsed,text,retry:graphRetryTextV1(event.round,first[0]??"",event.paths.length,event.owners.length,counts,elapsed)})+"\n");
      },
    });
    if(settled.value===undefined)return undefined;
    if(settled.volatile.paths.length===0)return settled.value;
    const volatilePaths=settled.volatile.paths.map(relative),text=graphVolatileTextV1(volatilePaths,settled.volatile.owners);
    process.stderr.write("[graph-volatile] "+JSON.stringify({version:1,rounds:attempts,paths:volatilePaths.slice(0,16),pathCount:volatilePaths.length,owners:settled.volatile.owners.slice(0,16),ownerCount:settled.volatile.owners.length,text})+"\n");
    return {...settled.value,volatile:{paths:volatilePaths,owners:settled.volatile.owners}};
  };
  const initial=(await settledInventory(false))!;
  if(isOwnerRegistryInvalidationV1(initial)){invalidated("owner-registry");return initial;}
  const {files,digest:before}=initial;
  let phaseStarted=performance.now();
  await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();timing("source-parser-bootstrap",phaseStarted);
  let cachedReceipt:any;try{cachedReceipt=JSON.parse(await readFile(authorityPath,"utf8"));}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT"&&!(error instanceof SyntaxError))throw error;}
  if(cachedReceipt)try{
    phaseStarted=performance.now();timing("source-authority-read-parse",phaseStarted,{observations:cachedReceipt.observations?.length??0});phaseStarted=performance.now();
    let cachedObservations:readonly unknown[]|undefined;
    if(buildGraphProducerAuthorityV1(cachedReceipt,loadedGraphModuleAuthorityV1)&&cachedReceipt.membership===before){
      try{cachedObservations=decodeAuthorityObservationsV1(cachedReceipt.observations,sourceAuthoritySchema.definitions.ObservationKind.enum,check);}catch{check();}
    }
    timing("source-authority-schema",phaseStarted,{admitted:cachedObservations!==undefined});phaseStarted=performance.now();const factsHold=cachedObservations!==undefined&&await sliceReadStepV1("receipt-check",()=>buildGraphSourceFactsHoldV1(cachedObservations!,invocation));timing("source-authority-facts",phaseStarted,{holds:factsHold});
    if(factsHold){
      phaseStarted=performance.now();const source=await readFile(path,"utf8");timing("graph-file-read",phaseStarted,{bytes:Buffer.byteLength(source)});phaseStarted=performance.now();const digest=createHash("sha256").update(source).digest("hex");timing("graph-content-hash",phaseStarted);if(digest===cachedReceipt.graph){const graph=await image(source);if(selected&&JSON.stringify(graph.admission?.selection)!==JSON.stringify(options.selection))throw Error("Original selected publication scope differs");if(await buildGraphSourceFactsHoldV1([...initial.observations,...cachedObservations!],invocation))return graph;}
    }
  }catch(error){const code=buildGraphErrorCodeV1(error);if(code!=="ENOENT"&&code!=="closure-missing")throw error;check();}
  if(options.sealedOnly)return undefined;
  let captured!:{value:BuildGraphV1;observations:readonly unknown[]};
  const reuse=createPassReuseV1(options.takenMemo===undefined?{}:{memo:options.takenMemo});
  let basis=initial as Basis,failedFacts:unknown[]=[],failedAuthority:unknown[]=[];
  phaseStarted=performance.now();
  const discoverOnce=(on:Basis)=>observation.captureObservedSourceInputsV1(async()=>{
    for(const source of [...graphSelfUrlsV1(),new URL("../../🧬️schema/📣️publication.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../🧬️schema/📥️authority.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../🔣️policy.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../../⚡️caching/🔣️policy.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../../🗂️workspaces/🔣️policy.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../📤️arguments/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))])observation.observedFs.readFileSync(source);
    const admittedRegistry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));if(JSON.stringify(admittedRegistry)!==JSON.stringify(registry))return ownerRegistryInvalidationV1;
    askOwnerRegistryV1(root,registry!,["digest"]);
    for(const source of [new URL("../../🧬️schema/🚪️selection.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../🧬️schema/📋️owners.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../🧬️schema/🚪️admission.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🚪️admission/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))])observation.observedFs.readFileSync(source);
    return discoverBuildGraphV1(root,options.selection,registry!,invocation,phaseControl,on.files,on.admission,true,reuse.memo);
  },(rows:unknown[],authority:unknown[])=>{failedFacts=rows;failedAuthority=authority;});
  type Basis=Exclude<Awaited<ReturnType<typeof inventory>>,OwnerRegistryInvalidationV1>;
  const stale:{kind:string;path:string;fact:unknown}[]=[];let moved:string[]=[],staleCount=0,rounds=reuse.carried.round,previousStale="",membershipMoved=false,admissionMoved=false;
  const collect=(row:{kind:string;path:string;fact:unknown})=>{staleCount++;if(stale.length<GRAPH_LIMITS_V1.sealStalePaths)stale.push(row);},sameAdmission=(left:Basis,right:Basis)=>!selected||JSON.stringify(left.admission)===JSON.stringify(right.admission);
  const followed=(current:Awaited<ReturnType<typeof inventory>>|undefined)=>{rounds++;if(current!==undefined&&!isOwnerRegistryInvalidationV1(current))basis=current;};
  for(;;){
    stale.length=0;staleCount=0;reuse.boundary({round:rounds,moved});moved=[];
    let produced:Awaited<ReturnType<typeof discoverOnce>>;
    try{produced=await sliceReadStepV1("owners-round",()=>discoverOnce(basis),{round:rounds+1});}catch(error){
      if(invocation)checkScriptInvocation(invocation);if(buildGraphErrorCodeV1(error)==="BUILD_AUTHORITY_CHANGED")throw error;
      observation.requirePinnedImplementationFactsV1(failedFacts as readonly observationV1.SourceObservationV1[],invocation?.control.signal);
      const current=await settledInventory(true);if(current!==undefined&&isOwnerRegistryInvalidationV1(current))throw error;
      const holds=await sliceReadStepV1("facts-hold",()=>buildGraphSourceFactsHoldV1(failedFacts,invocation,collect),{round:rounds+1});
      if(stableOwnerFailureHoldsV1(error,holds)||holds&&current!==undefined&&current.digest===basis.digest&&sameAdmission(basis,current)||rounds>=GRAPH_LIMITS_V1.repairRounds)throw error;
      process.stderr.write("[graph-repair] "+JSON.stringify({round:rounds+1,failure:true,stale:stale.map(row=>row.path)})+"\n");moved=[...stale.map(row=>row.path),...followedPathsV1(root,error)];followed(current);continue;
    }
    if(isOwnerRegistryInvalidationV1(produced.value)){invalidated("owner-registry",{before:basis.digest});return produced.value;}
    if(rounds===0)timing("graph-original-inference",phaseStarted,{observations:produced.observations.length});
    observation.requirePinnedImplementationFactsV1(produced.observations,invocation?.control.signal);
    const current=await settledInventory(true);
    if(current!==undefined&&isOwnerRegistryInvalidationV1(current)){invalidated("owner-registry",{before:basis.digest});return current;}
    const observed=distinctObservationRowsV1((current?.observations??[]) as readonly observationV1.SourceObservationV1[],produced.observations),holds=await sliceReadStepV1("facts-hold",()=>buildGraphSourceFactsHoldV1(observed,invocation,collect),{round:rounds+1});
    membershipMoved=current===undefined||current.digest!==basis.digest;admissionMoved=current!==undefined&&!sameAdmission(basis,current);
    captured={value:produced.value as BuildGraphV1,observations:observed};
    if(holds&&!membershipMoved&&!admissionMoved)break;
    const signature=JSON.stringify(stale.map(row=>[row.path,row.fact]).sort());
    if(staleCount>0&&!membershipMoved&&!admissionMoved&&signature===previousStale){process.stderr.write("[graph-retry] "+JSON.stringify({version:1,reason:"unrefreshed-facts",stale:stale.map(row=>row.path)})+"\n");throw observation.staleSourceFactsErrorV1(stale);}
    if(rounds>=GRAPH_LIMITS_V1.repairRounds)break;
    previousStale=signature;const repairStarted=performance.now();process.stderr.write("[graph-repair] "+JSON.stringify({round:rounds+1,stale:stale.slice(0,8).map(row=>row.path),staleCount,membershipMoved,admissionMoved})+"\n");moved=stale.map(row=>row.path);followed(current);timing("graph-repair",repairStarted,{round:rounds,holds});
  }
  const verdict=sealVerdictV1({rounds,staleCount,stale,membershipMoved,admissionMoved,projects:captured.value.projects as never,rows:captured.observations as never});
  reportSealV1(verdict);process.stderr.write("[graph-seal] "+JSON.stringify({version:1,mode:verdict.mode,rounds,sealedWithLaterChanges:verdict.sealedWithLaterChanges,laterChangedOwners:verdict.laterChangedOwners,membershipMoved,admissionMoved,owners:Object.keys(verdict.owners).length})+"\n");
  const publishStarted=performance.now(),compact=captured.value,source=JSON.stringify(compact),digest=createHash("sha256").update(source).digest("hex"),authorityHead=JSON.stringify({version:sourceAuthoritySchema.properties.version.const,module:loadedGraphModuleAuthorityV1,membership:basis.digest,graph:digest,seal:verdict}).slice(0,-1)+',"observations":',authority=authorityHead+encodeAuthorityObservationsV1(captured.observations as readonly observationV1.SourceObservationV1[],graphSnapshotPolicyV1.maximumSnapshotBytes-Buffer.byteLength(authorityHead)-1,check)+"}";timing("graph-serialize",publishStarted,{graphBytes:Buffer.byteLength(source),authorityBytes:Buffer.byteLength(authority)});if(Buffer.byteLength(source)>graphSnapshotPolicyV1.maximumSnapshotBytes||Buffer.byteLength(authority)>graphSnapshotPolicyV1.maximumSnapshotBytes)throw Error("Build graph cache exceeds its finite publication limit: "+Buffer.byteLength(source)+" graph bytes, "+Buffer.byteLength(authority)+" authority bytes, "+graphSnapshotPolicyV1.maximumSnapshotBytes+" allowed");await mkdir(dirname(path),{recursive:true});const expandStarted=performance.now(),graph=readBuildGraphV1(await sliceReadStepV1("seal",()=>portableInputs(root,compact,invocation,phaseControl)));timing("graph-expansion",expandStarted);
  const temporary=path+"."+randomUUID(),receipt=authorityPath+"."+randomUUID();try{await sliceReadStepV1("seal",async()=>{await writeFile(temporary,source,{flag:"wx"});await writeFile(receipt,authority,{flag:"wx"});});if(invocation)checkScriptInvocation(invocation);timing("graph-publication",publishStarted);process.stderr.write("[graph-phase] "+JSON.stringify({summary:phaseRows.map(row=>[row.phase,row.milliseconds])})+"\n");await sliceReadStepV1("seal",()=>critical(async()=>{await rename(temporary,path);await rename(receipt,authorityPath);}));}finally{await Promise.all([rm(temporary,{force:true}),rm(receipt,{force:true})]);}
  return graph;
}

/** 🧲️ The attempt of a selected scope offered through the fleet: a sealed generation of another machine whose owners hold here is installed, this machine's own sealed result is published.
 * @see ./🤝️selected-exchange/🔌️attempt/🟦️.ts */
function selectedThroughFleetV1<G>(input:{root:string;invocation:ScriptInvocation;scope:string;staging:string;attempt:()=>Promise<G|undefined>}):Promise<G|undefined>{
  return loadSelectedThroughFleetV1({root:input.root,scope:input.scope,path:join(input.staging,input.scope+".json"),authorityPath:join(input.staging,input.scope+".authority.json"),attempt:input.attempt,sealed:value=>value!==undefined&&!isOwnerRegistryInvalidationV1(value),holds:rows=>buildGraphSourceFactsHoldV1(rows,input.invocation),facts:selectedAdmissionFactsV1,module:loadedGraphModuleAuthorityV1,verdict:({projects,rows})=>sealVerdictV1({rounds:0,staleCount:0,stale:[],membershipMoved:false,admissionMoved:false,projects:projects as never,rows:rows as never}),signal:input.invocation?.control.signal});
}


/** 🔓️ Serves a current sealed graph to any number of shared readers: only a cache-store reader is held, the graph writer and its exclusive lease are never requested, so readers never queue behind a builder that is still inferring. */
async function readSealedBuildGraphV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{selection:BuildGraphSelectionV1;reuse?:boolean},registry:BuildOwnerRegistryV1):Promise<BuildGraphV1|OwnerRegistryInvalidationV1|undefined> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  return withOwnerRegistryCustodyV1<BuildGraphV1|OwnerRegistryInvalidationV1|undefined>(phaseControl.signal,contain=>withRepositoryCacheStoreV1(root,"graph",invocation,phaseControl,async(store,actualControl)=>{await store.recheck();checkResourceLeaseControl(actualControl);return contain(()=>loadBuildGraphAttemptV1(root,invocation,actualControl,{...options,sealedOnly:true},registry,undefined));}));
}


/** 🔌️ Requires the pinned selected view to carry exactly the requested canonical external roots. */
export function requireBuildGraphExternalInputsV1(graph:BuildGraphV1,inputs:readonly string[]):readonly string[]{const requested=[...new Set(readGenerationExternalInputsV1(inputs).map(name=>{const key=resolveGenerationExternalInputV1(graph.externalNodes,name);if(key===undefined)throw Error("Pinned build graph lacks external input: "+name);return key;}))].sort();if(JSON.stringify(requested)!==JSON.stringify(graph.admission?.externalInputs??[]))throw Error("Original selected external input admission required");return requested;}


export {pendingOwnerRootsV1,vanishedOwnerRootsV1,adoptPendingBuildOwnersV1} from "./🧭️registry-follow/🟦️.ts";

/** 🕸️ Loads the graph and follows the owner registry when inference started before a peer added, moved or removed a native owner.
 * @see ./🧭️registry-follow/🟦️.ts */
export async function loadBuildGraphV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {
  const attempt=(carried?:{readonly takenMemo:TakenMemoV1})=>loadBuildGraphOnceV1(root,invocation,phaseControl,carried===undefined?options:{...options,takenMemo:carried.takenMemo});
  if(options.reuse)return attempt();
  return settleOwnerRegistryV1(root,attempt,{aborted:()=>invocation?.control.signal.aborted===true,say:line=>{process.stderr.write(line);}});
}


/** 🔪️ Whether a selection other than `all` is read over its owner closure alone, which is the behaviour of a selected read; `SEMIO_BUILD_GRAPH_SLICE_READ=generation` restores the cut from the generation of the whole workspace (staged rollout only). */
export function graphSliceReadSelectedV1(environment:Readonly<Record<string,string|undefined>>=process.env):boolean{return environment.SEMIO_BUILD_GRAPH_SLICE_READ!=="generation";}

async function loadBuildGraphOnceV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {
  if(options.selection?.kind&&!options.baseline&&!options.reuse&&invocation){
    clearPinnedAuthorityV1();
    if(options.selection.kind==="all")return withImplementationPinV1(()=>loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options));
    if(graphSliceReadSelectedV1())return withImplementationPinV1(()=>loadBuildGraphUnpinnedV1(root,invocation,phaseControl,{...options,baseline:true}));
    return withImplementationPinV1(async()=>{
      const sliced=await loadBuildGraphSliceV1(root,invocation,phaseControl,options);
      if(process.env.SEMIO_BUILD_GRAPH_VERIFY==="1")requireSelectedSubgraphV1(sliced,await loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options));
      return sliced;
    });
  }
  return loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options);
}


/** 🍰️ Derives the graph of a selected scope (tasks or literal owners) from the newest sealed generation: the closure of the selected projects is read, verified and, under a fresh policy, refreshed incrementally. The generation is the only source of a graph: when none is usable it is built first, and a selected project the generation does not know refuses. */
async function loadBuildGraphSliceV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {
  const producer=await import("../📥️inference/🟨️.mjs"),registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))),selection=readBuildGraphSelectionV1(options.selection);
  if(selection.kind==="all")throw Error("Selected build graph scope required");
  const roots=selection.kind==="tasks"?[...new Set(selection.tasks.map(task=>task.project))]:selectLiteralBuildProjectsV1(registry.owners,selection.projects,selection.exclude);
  const loaded=await loadGenerationGraphV1(generationHostV1(root,invocation,phaseControl,registry,options.graph?.compact===true),{policy:options.graph?.policy??"fresh",selection,slice:{selection,roots,providers:registry.catalogProviders.map(({consumerRoots,sourcePath})=>({consumerRoots,sourcePath}))},externalInputs:options.graph?.externalInputs,pin:boundGenerationPinV1()});options.onStats?.(loaded.stats);return loaded.graph as BuildGraphV1;
}


/** 🔭️ Names the first difference between a generation slice and the selected graph a full selected inference produced, in both directions: every selected project is equal, the dependencies among the selected projects are the same set, and every selected external node is equal; the slice may hold more projects. The oracle of `SEMIO_BUILD_GRAPH_VERIFY=1` for selected scopes. */
function requireSelectedSubgraphV1(slice:BuildGraphV1,selected:BuildGraphV1):void{
  const same=(left:unknown,right:unknown)=>canonicalGenerationJsonV1(left)===canonicalGenerationJsonV1(right),chosen=new Set(Object.keys(selected.projects));
  for(const[name,project]of Object.entries(selected.projects)){if(!slice.projects[name])throw Error(`Graph slice lacks the selected project ${name}`);if(!same(slice.projects[name],project))throw Error(`Graph slice differs from the selected graph at project ${name}`);}
  const inside=(edge:{source:string;target:string})=>chosen.has(edge.source)&&(chosen.has(edge.target)||selected.externalNodes?.[edge.target]!==undefined||slice.externalNodes?.[edge.target]!==undefined);
  const mine=new Set(slice.dependencies.filter(inside).map(edge=>canonicalGenerationJsonV1(edge))),theirs=new Set(selected.dependencies.map(edge=>canonicalGenerationJsonV1(edge)));
  for(const edge of theirs)if(!mine.has(edge))throw Error("Graph slice lacks a selected dependency: "+edge);
  for(const edge of mine)if(!theirs.has(edge))throw Error("Graph slice adds a dependency the selected graph lacks: "+edge);
  for(const[key,node]of Object.entries(selected.externalNodes??{}))if(!same(slice.externalNodes?.[key],node))throw Error(`Graph slice differs from the selected graph at external node ${key}`);
}


type BuildGraphLoadOptionsV1={selection:BuildGraphSelectionV1;reuse?:boolean;baseline?:boolean;takenMemo?:TakenMemoV1;onStats?:(stats:GenerationLoadStatsV1)=>void;graph?:{policy?:"fresh"|"current";closure?:readonly string[];sealedOnly?:boolean;lean?:boolean;compact?:boolean;externalInputs?:readonly string[]}};


/** 🕸️ Loads the graph with the strict self-checks of the loaded implementation; the generation path runs pinned and completes with what it loaded. A moved owner registry is adopted (the authority snapshot is dropped and the registry read once more, its owners are inferred or served by their own records) and never ends the read: under the pin the next registry read is fixed again, so the loop has no count to give up at and ends with the first candidate that holds. */
async function loadBuildGraphUnpinnedV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  options={...options,selection:readBuildGraphSelectionV1(options.selection)};
  if(options.reuse)return withRepositoryBuildGraphPublicationV1(root,phaseControl,async graph=>{checkScriptInvocation(invocation);if(!graph.admission||JSON.stringify(readBuildGraphSelectionV1(graph.admission.selection))!==JSON.stringify(readBuildGraphSelectionV1(options.selection)))throw Error("Original selected build graph admission required");if(options.graph?.externalInputs!==undefined)requireBuildGraphExternalInputsV1(graph,options.graph.externalInputs);return graph;});
  if(!invocation)throw Error("Original build graph producer invocation required");
  if(options.selection.kind==="all"&&!options.baseline){
    const producer=await import("../📥️inference/🟨️.mjs"),registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));
    const loadedAll=await loadGenerationGraphV1(generationHostV1(root,invocation,phaseControl,registry),{policy:options.graph?.policy??"fresh",closure:options.graph?.closure,sealedOnly:options.graph?.sealedOnly,lean:options.graph?.lean,externalInputs:options.graph?.externalInputs,pin:boundGenerationPinV1()});
    options.onStats?.(loadedAll.stats);const graph=loadedAll.graph as BuildGraphV1;
    if(process.env.SEMIO_BUILD_GRAPH_VERIFY==="1"&&!options.graph?.closure&&!options.graph?.lean)requireEqualBuildGraphsV1(graph,await loadBuildGraphV1(root,invocation,phaseControl,{selection:options.selection,baseline:true}));
    checkScriptInvocation(invocation);return graph;
  }
  for(;;){
    checkScriptInvocation(invocation);checkResourceLeaseControl(phaseControl);
    const producer=await import("../📥️inference/🟨️.mjs");checkResourceLeaseControl(phaseControl);const registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))),scope=options.selection.kind!=="all"?buildGraphScopeKeyV1(options.selection):undefined;
    if(scope===undefined){const sealed=await readSealedBuildGraphV1(root,invocation,phaseControl,options,registry);if(sealed&&!isOwnerRegistryInvalidationV1(sealed)){checkScriptInvocation(invocation);return sealed;}}
    const graph=await withOwnerRegistryCustodyV1<BuildGraphV1|OwnerRegistryInvalidationV1|undefined>(phaseControl.signal,contain=>withBuildGraphStateV1(root,invocation,phaseControl,async(store,actualControl,critical)=>{
      const staging=scope?join(store.root,"selected"):store.root;await store.recheck();checkResourceLeaseControl(actualControl);requireNativeStoragePathV1(store.root,staging);await mkdir(staging,{recursive:true});checkResourceLeaseControl(actualControl);const selectedStage=scope?new RegExp("^"+scope+"(?:\\.authority)?\\.json\\.[a-f0-9-]{36}$","u"):undefined;
      await critical(async()=>{const staged=await readdir(staging);checkResourceLeaseControl(actualControl);for(const name of staged){if(!(selectedStage?.test(name)??/^(?:canonical|authority)\.json\.[a-f0-9-]{36}$/u.test(name)))continue;await store.recheck();checkResourceLeaseControl(actualControl);const path=join(staging,name),status=await lstat(path);checkResourceLeaseControl(actualControl);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>graphSnapshotPolicyV1.maximumSnapshotBytes)throw Error("Owned bounded graph staging file required");await rm(path);checkResourceLeaseControl(actualControl);}});
      checkResourceLeaseControl(actualControl);await store.recheck();const attempt=()=>loadBuildGraphAttemptV1(root,invocation,actualControl,options,registry,scope,critical);
      return contain(()=>scope===undefined?attempt():selectedThroughFleetV1({root,invocation,scope,staging,attempt}));
    },scope));
    checkScriptInvocation(invocation);checkResourceLeaseControl(phaseControl);
    if(graph&&!isOwnerRegistryInvalidationV1(graph))return graph;
    clearPinnedAuthorityV1();process.stderr.write("[graph-repair] "+JSON.stringify({version:1,registry:"adopted"})+"\n");
    await advanceScriptInvocation(invocation,"build:refresh","running");
  }
}
