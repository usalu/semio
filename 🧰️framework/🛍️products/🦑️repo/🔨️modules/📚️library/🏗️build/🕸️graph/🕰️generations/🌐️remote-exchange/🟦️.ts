import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {createHash} from "node:crypto";
import {readFileSync,readdirSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {totalmem} from "node:os";
import {join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {GENERATION_POLICY_V1 as P,readGenerationManifestV1,readGenerationPointerV1,type GenerationFilePortV1} from "../🟨️.mjs";
import {remoteCacheEffectiveUploadV1} from "../../../🌐️remote-cache/🟦️.ts";
import {remoteCacheFetchV1,resolveRemoteCacheV1} from "../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {httpRemoteExecutionContentV1,type RemoteExecutionContentV1} from "../../../🌐️remote-execution/🗄️content/🟦️.ts";
import {REAPI_V1,decodeProtobufV1,encodeProtobufV1} from "../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {documentNameV1,generationSignatureV1,neutralizeGenerationFromMemoryV1,neutralKeyV1,transformGenerationDocumentV1,transformGenerationV1,type GenerationDocumentV1,type NeutralKnownV1,type NeutralRowV1} from "./🧳️portable/🟦️.ts";
import {profilePhaseV1} from "../../../⏱️profile/🟨️.mjs";
import {repositoryCacheBudgetV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import {CLOSURE_POLICY_V1,closureLeafPutterV1,closureOwnersV1,closuresOfFragmentsV1,obtainClosureLeafV1,type ClosureNamespaceV1} from "../../🩹️closure-repair/🟨️.mjs";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🏷️ The inline description of one uploaded generation: what a client compares before it downloads anything. `manifest` names the neutral manifest; `signature` is the root independent state; `stateMs` is the time of the tree state it describes and `stale` the owners its producer knew stale in it, which together rank the heads of different machines (`publishedMs` only dates the upload). */
export type GenerationNoteV1=Readonly<{version:1;generation:number;owners:number;identity:string;manifest:string;signature:string;platform:string;publishedMs:number;origin:string;stateMs:number;implementation:string;stale?:number;snapshot?:string;scope?:string}>;
/** 📍️ The state of the shared files a generation was verified against: a producer hands over the snapshot of its generation, a restoring machine installs it as the cursor of the restored generation. */
export type GenerationCursorsV1=Readonly<{snapshotFor(generation:number):Promise<{instance:string;bytes:Uint8Array}|undefined>;install(generation:number,snapshot:{instance:string;bytes:Uint8Array}):Promise<void>;distance?(snapshot:{instance:string;bytes:Uint8Array}):Promise<number|undefined>}>;
/** 🧾️ What one exchange did; a refusal or a failure never fails the graph load that asked. */
export type GenerationExchangeOutcomeV1=Readonly<{status:"restored"|"uploaded"|"local"|"absent"|"foreign"|"empty"|"current"|"remote-better"|"incomplete"|"oversized"|"failed";generation?:number;owners?:number;documents?:number;sent?:number;bytes?:number;ageSeconds?:number;stale?:number;reason?:string;closures?:GenerationClosuresV1}>;
/** 🧲️ What an exchange did with the build input closure leaves a generation names: how many it names, how many were uploaded or fetched, and the leaves it could not obtain with the owners that name them (those owners are stale, the generation is not complete). */
export type GenerationClosuresV1=Readonly<{named:number;sent:number;fetched:number;missing:readonly Readonly<{key:string;namespace:ClosureNamespaceV1;projects:readonly string[]}>[]}>;
/** 🗄️ Where the closure leaves of this machine live and how they are read and written: the graph store of the workspace by default. */
export interface ClosurePortV1{
 directory(namespace:ClosureNamespaceV1):string;
 size(namespace:ClosureNamespaceV1,key:string):number|undefined;
 read(namespace:ClosureNamespaceV1,key:string):Promise<Uint8Array|undefined>;
 put(namespace:ClosureNamespaceV1,key:string,bytes:Uint8Array):Promise<void>;
}
/** 🔌️ The two operations the generation load asks of the remote cache of the machine group. */
export interface GenerationExchangeV1{
 /** ⬆️ Whether this machine may write to the remote cache. */
 readonly upload:boolean;
 restore(options:{root:string;fs:GenerationFilePortV1;signal?:AbortSignal;implementation?:string}):Promise<GenerationExchangeOutcomeV1>;
 /** 🔭️ Whether a restore would take a newer state of a peer (one short request, nothing downloaded). */
 ahead(options:{root:string;fs:GenerationFilePortV1;signal?:AbortSignal;implementation?:string}):Promise<boolean>;
 /** ⬆️ Offers the newest sealed generation unless the cache holds a head that is at least as good; `stale` is the number of owners this producer knows stale in it (0 after a complete read, absent when unknown). */
 publish(options:{root:string;fs:GenerationFilePortV1;signal?:AbortSignal;ifAbsent?:boolean;stale?:number}):Promise<GenerationExchangeOutcomeV1>;
 /** 🧲️ One build input closure leaf from the fleet cache, verified against its name; nothing when the cache does not hold it or is unreachable. */
 leaf(options:{namespace:ClosureNamespaceV1;key:string;signal?:AbortSignal}):Promise<Uint8Array|undefined>;
}
type Files=Pick<GenerationFilePortV1,"read"|"write"|"list">;
type Location=Readonly<{content:RemoteExecutionContentV1;instance:string;key:string;root:string;workspace:string;platform:string;files:Files;signal?:AbortSignal;now?:()=>number;cursors?:GenerationCursorsV1;keys?:readonly string[];origin?:string;implementation?:string;stale?:number;serving?:boolean;closures?:ClosurePortV1;scope?:string}>;

const restoredReason="restored from the remote cache",policy=schema.definitions.Policy.const,document=new RegExp(policy.document,"u"),closureDocument=new RegExp(policy.closure,"u"),snapshotDocument=new RegExp(policy.snapshot,"u"),sha=(bytes:Uint8Array):string=>createHash("sha256").update(bytes).digest("hex");

/** 🗝️ The action key of the head of the generations one machine of the group published, for a registry on one platform: the generation format, the owner registry identity, the platform and the origin that writes it, never a path. The server keeps the first bytes of a key against every other origin (a key must determine its bytes), so each origin owns one head, overwrites only its own, and a reader takes the newest head of all origins. */
export function generationExchangeKeyV1(identity:string,platform:string,origin:string,scope:string=policy.allScope):string{return sha(Buffer.from(JSON.stringify(scope===policy.allScope?[policy.domain,identity,platform,origin]:[policy.domain,identity,platform,origin,scope])));}

/** 🔭️ Whether a head describes the scope a reader wants: a head of one slice is never the generation of another slice or of the whole workspace, and the whole workspace is the scope of a head that names none. */
export const generationScopeMatchesV1=(note:Readonly<{scope?:string}>,scope:string|undefined):boolean=>(note.scope??policy.allScope)===(scope??policy.allScope);

type HeadV1=Readonly<{note:GenerationNoteV1|undefined;entries:readonly {path:string;hash:string;size:number}[];from:string}>;
type RankedV1=HeadV1&Readonly<{note:GenerationNoteV1}>;

/** 🏁️ How many owners a reader would find stale in a head at `nowMs`: the owners its producer knew stale (all of them when the reader's code differs from the code that built it, the policy's guess when the producer did not know), plus the owners the tree moved under since: counted from the paths that changed between the head's snapshot and the reader's tree when the reader measured them, else estimated from the age of the state. Lower is better. */
export function generationHeadScoreV1(note:GenerationNoteV1,nowMs:number,reader:Readonly<{implementation?:string;changedPaths?:number}>={}):number{
 const known=reader.implementation!==undefined&&reader.implementation!==note.implementation?note.owners:Math.min(note.owners,note.stale??policy.unmeasuredStale);
 return known+(reader.changedPaths!==undefined?reader.changedPaths*policy.ownersPerChangedPath:Math.max(0,nowMs-note.stateMs)/60000*policy.staleOwnersPerMinute);
}

/** 🥇️ Orders heads best first: the fewest owners a reader would find stale, then the newest tree state, then the origin name; the time of an upload never counts, so an old state that reached the cache later does not shadow a newer one. */
export function rankGenerationHeadsV1<T extends Readonly<{note:GenerationNoteV1}>>(heads:readonly T[],nowMs:number,reader:Readonly<{implementation?:string;changed?:ReadonlyMap<T,number>}>={}):T[]{
 const score=new Map(heads.map(head=>[head,generationHeadScoreV1(head.note,nowMs,{implementation:reader.implementation,changedPaths:reader.changed?.get(head)})] as const));
 return[...heads].sort((a,b)=>score.get(a)!-score.get(b)!||b.note.stateMs-a.note.stateMs||(a.note.origin<b.note.origin?-1:a.note.origin>b.note.origin?1:0));
}

/** 🪪️ The origin of this machine in the replica group and every origin of the group (the folders below `<store>/v1/origins`), read from the replica state of the worktree; a worktree without a replica is its own group of one. */
export function fleetOriginsV1(workspace:string):Readonly<{self:string;all:readonly string[]}>{
 try{
  const machine=JSON.parse(readFileSync(machinePathV1(workspace,".cache","f","machine.json"),"utf8")) as {originId:string;storeRoot:string},valid=/^[a-z0-9][a-z0-9-]{0,39}$/u;
  if(!valid.test(machine.originId))throw Error("origin");
  let listed:string[]=[];try{listed=readdirSync(join(machine.storeRoot,"v1","origins")).filter(name=>valid.test(name));}catch{}
  return{self:machine.originId,all:[...new Set([machine.originId,...listed])].sort()};
 }catch{return{self:"local",all:["local"]};}
}

async function each<T>(items:readonly T[],visit:(item:T)=>Promise<void>):Promise<void>{
 let next=0;await Promise.all(Array.from({length:Math.min(policy.concurrency,items.length)},async()=>{while(next<items.length)await visit(items[next++]!);}));
}

/** 📚️ The relative paths of every document one manifest needs: itself, its fact index, its frontier, its global fragment and the fragment of each owner. */
export function generationDocumentPathsV1(digest:string,manifest:ReturnType<typeof readGenerationManifestV1>):readonly string[]{
 const fragments=new Set<string>([manifest.global,...Object.values(manifest.owners).map(owner=>owner.fragment)]);
 return[`${P.manifests}/${digest}.json`,`${P.indexes}/${manifest.factIndex}.facts.br`,...manifest.frontier===null?[]:[`${P.frontiers}/${manifest.frontier}.json.br`],...[...fragments].sort().map(name=>`${P.fragments}/${name}.json.br`)];
}

const noteOf=(bytes:Uint8Array|undefined):GenerationNoteV1|undefined=>{
 if(!bytes?.length)return undefined;
 try{const value=JSON.parse(Buffer.from(bytes).toString("utf8"));return validateJsonSchemaSubset({$ref:"#/definitions/Note"},value,schema).length?undefined:value;}catch{return undefined;}
};

async function readHead(location:Location):Promise<HeadV1|undefined>{
 const bytes=await location.content.readActionResult(location.instance,location.key,location.signal);if(!bytes)return undefined;
 const result=decodeProtobufV1(REAPI_V1.actionResult,bytes),files=(result.output_files??[]) as {path:string;digest:{hash:string;size_bytes:number|string}}[];
 return{note:noteOf(result.stdout_raw as Uint8Array|undefined),entries:files.map(file=>({path:file.path,hash:file.digest.hash,size:Number(file.digest.size_bytes)})),from:location.key};
}

async function remoteNote(location:Location,reader:Readonly<{implementation?:string;nearest?:boolean}>={}):Promise<HeadV1|undefined>{
 const keys=location.keys??[location.key],settled=await Promise.allSettled(keys.map(key=>readHead({...location,key,keys:undefined})));
 if(settled.every(row=>row.status==="rejected"))throw(settled[0] as PromiseRejectedResult).reason;
 const found=settled.flatMap(row=>row.status==="fulfilled"&&row.value!==undefined?[row.value]:[]),valid=found.filter((head):head is RankedV1=>head.note!==undefined);
 if(valid.length===0)return found[0];
 const now=(location.now??Date.now)(),ranked=rankGenerationHeadsV1(valid,now,{implementation:reader.implementation});
 if(!reader.nearest||ranked.length<2||!location.cursors?.distance)return ranked[0];
 const candidates=ranked.slice(0,policy.rankCandidates),changed=new Map<RankedV1,number>();
 for(const head of candidates){
  const path=head.note.snapshot===undefined?undefined:`s/${head.note.snapshot}.snapshot.br`,entry=head.entries.find(row=>row.path===path);
  if(!entry||!snapshotDocument.test(entry.path)||!(entry.size<=policy.maximumSnapshotBytes))return ranked[0];
  const data=await location.content.read(location.instance,{hash:entry.hash,size:entry.size},location.signal).catch(()=>undefined);
  if(!data||sha(data)!==entry.hash)return ranked[0];
  const distance=await location.cursors.distance({instance:head.note.snapshot!,bytes:data}).catch(()=>undefined);
  if(distance===undefined)return ranked[0];
  changed.set(head,distance);
 }
 return rankGenerationHeadsV1(candidates,now,{implementation:reader.implementation,changed})[0];
}

/** 🛡️ Whether a store keeps its own generation instead of taking the head `note` of `from`: it already holds that state (same signature from its own head, or a head that is not newer than its pointer by the margin), or it holds a generation of the reader's own code and the head was built by other code, which would leave every owner stale. */
function keepsLocalV1(local:NonNullable<Awaited<ReturnType<typeof localState>>>,note:GenerationNoteV1,from:string,location:Location&Readonly<{marginMilliseconds?:number}>):boolean{
 if(generationSignatureV1(local.manifest)===note.signature&&from===location.key||note.stateMs<=local.modifiedMs+(location.marginMilliseconds??policy.restoreMarginMilliseconds))return true;
 return location.implementation!==undefined&&note.implementation!==location.implementation&&local.manifest.implementation===location.implementation;
}

/** 🆔️ The registry identity of the generation a store holds, for a process that has no registry of its own (the agent that uploads the waves of a read it does not run). */
export async function localGenerationIdentityV1(root:string,files:Files):Promise<string>{
 const local=await localState(root,files);if(!local)throw Error("no sealed generation in "+root);
 return local.manifest.identity;
}

async function localState(root:string,files:Files):Promise<{manifest:ReturnType<typeof readGenerationManifestV1>;digest:string;generation:number;modifiedMs:number;restored:boolean}|undefined>{
 const pointerBytes=await files.read(join(root,P.pointer));if(!pointerBytes)return undefined;
 let pointer;try{pointer=readGenerationPointerV1(pointerBytes);}catch{return undefined;}
 if(pointer.manifest===null)return undefined;
 const manifestBytes=await files.read(join(root,P.manifests,pointer.manifest+".json"));if(!manifestBytes||sha(manifestBytes)!==pointer.manifest)return undefined;
 const listed=(await files.list(root).catch(()=>[])).find(row=>row.name===P.pointer);
 return{manifest:readGenerationManifestV1(manifestBytes),digest:pointer.manifest,generation:pointer.generation,modifiedMs:listed?.modifiedMs??0,restored:pointer.reason===restoredReason};
}

const closureFolderV1:Readonly<Record<ClosureNamespaceV1,string>>={"build-input-closures":"i","build-lock-closures":"l"};
const closurePathV1=(namespace:ClosureNamespaceV1,key:string):string=>`c/${closureFolderV1[namespace]}/${key}.json`;

/** 🧾️ The leaves of the fragments (`names`, their brotli documents read by `read`) with the memory file `memoFile` of fragments seen before; a fragment is content addressed so its leaves never change. */
async function namedClosuresV1(names:readonly string[],read:(name:string)=>Promise<Uint8Array|undefined>,memoFile:string|undefined){
 const memo=new Map<string,[string,ClosureNamespaceV1][]>();
 if(memoFile!==undefined)try{const value=JSON.parse(readFileSync(memoFile,"utf8")) as {version:number;entries:[string,[string,ClosureNamespaceV1][]][]};if(value.version===1&&Array.isArray(value.entries))for(const[name,leaves]of value.entries)if(typeof name==="string"&&Array.isArray(leaves))memo.set(name,leaves);}catch{}
 const named=await closuresOfFragmentsV1({names,read,memo,concurrency:policy.concurrency});
 if(memoFile!==undefined){const keep=new Set(names),temporary=memoFile+"."+process.pid+".tmp";try{writeFileSync(temporary,JSON.stringify({version:1,entries:[...memo].filter(([name])=>keep.has(name))}));renameSync(temporary,memoFile);}catch{rmSync(temporary,{force:true});}}
 return named;
}

/** ⬆️ Uploads the closure leaves a generation names, deduplicated and bounded, before its head is written: every leaf this machine holds that the cache misses (the cache is asked once, by name and size) is read, verified against its name and written; a leaf this machine does not hold is reported with the owners that name it and never fails the upload. */
async function uploadClosuresV1(location:Location,manifest:ReturnType<typeof readGenerationManifestV1>,fragments:readonly string[],entries:{path:string;hash:string;size:number}[]):Promise<GenerationClosuresV1>{
 const{closures,content,instance,signal,files,root}=location;
 if(!closures)return{named:0,sent:0,fetched:0,missing:[]};
 const named=closureOwnersV1(await namedClosuresV1(fragments,async name=>files.read(join(root,P.fragments,name+".json.br")),root+".closures.json"),manifest.owners),held:{key:string;namespace:ClosureNamespaceV1;size:number}[]=[],missing:{key:string;namespace:ClosureNamespaceV1;projects:readonly string[]}[]=[];
 for(const[key,row]of[...named].sort((a,b)=>a[0]<b[0]?-1:1)){
  const size=closures.size(row.namespace,key);
  if(size===undefined||size<1||size>CLOSURE_POLICY_V1.maximumLeafBytes)missing.push({key,namespace:row.namespace,projects:row.projects});
  else if(held.length<policy.maximumClosures)held.push({key,namespace:row.namespace,size});
 }
 const sentFile=root+".closures-sent.json",before=new Set<string>();
 try{const value=JSON.parse(readFileSync(sentFile,"utf8")) as {version:number;instance:string;keys:string[]};if(value.version===1&&value.instance===instance&&Array.isArray(value.keys))for(const key of value.keys)before.add(key);}catch{}
 const asking=held.filter(row=>!before.has(row.key)),absent=new Set((await content.missing(instance,asking.map(row=>({hash:row.key,size:row.size})),signal)).map(digest=>digest.hash));let sent=0;
 await each(held.filter(row=>absent.has(row.key)),async row=>{const data=await closures.read(row.namespace,row.key);if(!data||sha(data)!==row.key){missing.push({key:row.key,namespace:row.namespace,projects:named.get(row.key)!.projects});return;}await content.write(instance,{hash:row.key,size:data.length},data,signal);sent++;});
 const lost=new Set(missing.map(row=>row.key));
 for(const row of held)if(!lost.has(row.key))entries.push({path:closurePathV1(row.namespace,row.key),hash:row.key,size:row.size});
 const temporary=sentFile+"."+process.pid+".tmp";try{writeFileSync(temporary,JSON.stringify({version:1,instance,keys:held.filter(row=>!lost.has(row.key)).map(row=>row.key)}));renameSync(temporary,sentFile);}catch{rmSync(temporary,{force:true});}
 return{named:named.size,sent,fetched:0,missing:missing.slice(0,policy.maximumClosureReports)};
}

/** ⬇️ Obtains the closure leaves the fragments of a generation about to be restored name and this machine lacks: each is read from the fleet cache by name (bounded and verified), and the leaves nobody has are reported with the owners that name them, whose records the load removes so they are derived again. */
async function restoreClosuresV1(location:Location,fetched:readonly GenerationDocumentV1[],owners:Readonly<Record<string,{fragment:string}>>):Promise<GenerationClosuresV1|undefined>{
 const{closures,content,instance,signal}=location;
 if(!closures)return undefined;
 const prefix=P.fragments+"/",suffix=".json.br",byName=new Map(fetched.filter(item=>item.path.startsWith(prefix)).map(item=>[item.path.slice(prefix.length,-suffix.length),item.bytes] as const));
 const named=closureOwnersV1(await closuresOfFragmentsV1({names:[...byName.keys()],read:async name=>byName.get(name),concurrency:policy.concurrency}),owners),absent=[...named].filter(([key,row])=>!(closures.size(row.namespace,key)!==undefined)).slice(0,policy.maximumClosures);
 const missing:{key:string;namespace:ClosureNamespaceV1;projects:readonly string[]}[]=[];let got=0;
 await each(absent,async([key,row])=>{
  const state=await obtainClosureLeafV1({directory:closures.directory,namespace:row.namespace,key,fetch:async()=>content.read(instance,{hash:key,size:0},signal),put:closures.put,signal});
  if(state==="refetched")got++;else if(state==="missing")missing.push({key,namespace:row.namespace,projects:row.projects});
 });
 return{named:named.size,sent:0,fetched:got,missing:missing.sort((a,b)=>a.key<b.key?-1:1).slice(0,policy.maximumClosureReports)};
}

/** 🧠️ The neutral documents an earlier upload of this store made (original name to neutral name and size), valid only under the key of the root and the code that made them; empty where the file is missing, damaged or of another key. */
function readNeutralMemoV1(file:string,key:string|undefined):NeutralKnownV1{
 if(key===undefined)return new Map();
 try{const value=JSON.parse(readFileSync(file,"utf8")) as {version:number;key:string;entries:[string,string,number][]};if(value.version!==1||value.key!==key||!Array.isArray(value.entries))return new Map();return new Map(value.entries.filter(row=>Array.isArray(row)&&typeof row[0]==="string"&&/^[0-9a-f]{64}$/u.test(row[1])&&Number.isSafeInteger(row[2])).map(row=>[row[0],{name:row[1],size:row[2]}] as const));}catch{return new Map();}
}
/** 💾️ Remembers the neutral documents of the generation just uploaded, replacing the older memory atomically; losing the write only costs the next upload a full rewrite. */
function writeNeutralMemoV1(file:string,key:string|undefined,known:NeutralKnownV1):void{
 if(key===undefined)return;
 const temporary=file+"."+process.pid+".tmp";
 try{writeFileSync(temporary,JSON.stringify({version:1,key,entries:[...known].map(([original,row])=>[original,row.name,row.size])}));renameSync(temporary,file);}catch{rmSync(temporary,{force:true});}
}
class NeutralStaleV1 extends Error{}

/** ⬆️ Uploads the newest sealed generation of one store in its neutral form (root replaced by the root token, every document renamed after its new bytes), unless the cache already holds this state (same signature) or a head of another machine that is at least as good (it leaves its readers the same number of stale owners or fewer; a machine under the serving memory size must beat it by the shadow margin). Only the documents the cache misses are sent, so an interrupted upload resumes from what arrived. Documents go first, the action result that names them last. */
export async function publishGenerationV1(location:Location&Readonly<{ifAbsent?:boolean}>):Promise<GenerationExchangeOutcomeV1>{
 const{content,instance,key,root,files,signal}=location,local=await localState(root,files);if(!local)return{status:"empty"};
 const signature=generationSignatureV1(local.manifest),remote=await remoteNote(location,{implementation:local.manifest.implementation}).catch(()=>undefined),owners=Object.keys(local.manifest.owners).length;
 if(remote?.note?.signature===signature&&(local.restored||remote.note.stateMs>=Math.floor(local.modifiedMs)))return{status:"current",generation:local.generation,owners};
 const mine=Math.min(owners,location.stale??policy.unmeasuredStale)+(location.serving===false?policy.shadowMarginOwners:0);
 if(remote&&(location.ifAbsent||remote.note!==undefined&&remote.from!==location.key&&generationHeadScoreV1(remote.note,(location.now??Date.now)(),{implementation:local.manifest.implementation})<=mine))return{status:"remote-better",generation:local.generation,owners};
 const paths=generationDocumentPathsV1(local.digest,local.manifest);if(paths.length>policy.maximumDocuments)return{status:"oversized",reason:String(paths.length)+" documents"};
 const snapshot=await location.cursors?.snapshotFor(local.generation).catch(()=>undefined),shipped=snapshot!==undefined&&snapshot.bytes.length<=policy.maximumSnapshotBytes?snapshot:undefined;
 const memoFile=root+".neutral.json",memoKey=neutralKeyV1(location.workspace);
 const neutral=async(known:NeutralKnownV1)=>{
  const wanted=paths.filter(path=>path.startsWith(P.manifests+"/")||!known.has(documentNameV1(path))),bytes=new Map<string,Uint8Array>();
  await profilePhaseV1("graph.upload.read",()=>each(wanted,async path=>{const data=await files.read(join(root,path));if(!data||data.length>policy.maximumDocumentBytes)throw Error("generation document unavailable: "+path);if(!path.includes(sha(data)))throw Error("generation document differs from its name: "+path);bytes.set(path,data);}));
  const moved=profilePhaseV1("graph.upload.transform",()=>neutralizeGenerationFromMemoryV1({paths,bytes,root:location.workspace,known})),rows:NeutralRowV1[]=[...moved.rows].sort((a,b)=>a.path<b.path?-1:1);
  if(shipped)rows.push({path:`s/${shipped.instance}.snapshot.br`,hash:sha(shipped.bytes),size:shipped.bytes.length,bytes:shipped.bytes});
  const missing=new Set((await profilePhaseV1("graph.upload.missing",()=>content.missing(instance,rows.map(row=>({hash:row.hash,size:row.size})),signal))).map(digest=>digest.hash)),sending=new Map<string,Uint8Array>();
  await each(rows.filter(row=>missing.has(row.hash)&&row.bytes===undefined),async row=>{
   const data=await files.read(join(root,row.source!));if(!data||!row.source!.includes(sha(data)))throw new NeutralStaleV1("remembered document unavailable");
   const again=transformGenerationDocumentV1(row.source!,data,location.workspace,"neutralize");if(sha(again)!==row.hash)throw new NeutralStaleV1("remembered document differs");sending.set(row.hash,again);
  });
  return{moved,rows,missing,sending};
 };
 let staged;try{staged=await neutral(readNeutralMemoV1(memoFile,memoKey));}catch(error){if(!(error instanceof NeutralStaleV1))throw error;rmSync(memoFile,{force:true});staged=await neutral(new Map());}
 const{moved,rows,missing,sending}=staged;
 const manifestName=moved.manifestPath.slice(P.manifests.length+1,-5);let bytesSent=0,sent=0;
 await profilePhaseV1("graph.upload.write",()=>each(rows.filter(row=>missing.has(row.hash)),async row=>{const data=row.bytes??sending.get(row.hash)!;await content.write(instance,{hash:row.hash,size:row.size},data,signal);bytesSent+=row.size;sent++;}));
 writeNeutralMemoV1(memoFile,memoKey,moved.known);
 const entries=rows.map(row=>({path:row.path,hash:row.hash,size:row.size})),fragmentNames=[...new Set<string>([local.manifest.global,...Object.values(local.manifest.owners).map(owner=>owner.fragment)])].sort(),leaves=await profilePhaseV1("graph.upload.closures",()=>uploadClosuresV1(location,local.manifest,fragmentNames,entries)).catch((error):GenerationClosuresV1=>({named:0,sent:0,fetched:0,missing:[{key:"0".repeat(64),namespace:"build-input-closures",projects:[String(error?.message??error).slice(0,120)]}]})),naming=new Set(leaves.missing.flatMap(row=>row.projects)).size;
 const note:GenerationNoteV1={version:1,generation:local.generation,owners:Object.keys(local.manifest.owners).length,identity:local.manifest.identity,manifest:manifestName,signature,platform:location.platform,publishedMs:(location.now??Date.now)(),origin:location.origin??"local",stateMs:Math.floor(local.modifiedMs),implementation:local.manifest.implementation,...location.stale===undefined?{}:{stale:Math.min(owners,Math.max(0,Math.floor(location.stale))+naming)},...shipped?{snapshot:shipped.instance}:{},...location.scope===undefined||location.scope===policy.allScope?{}:{scope:location.scope}};
 const result=encodeProtobufV1(REAPI_V1.actionResult,{output_files:entries.map(row=>({path:row.path,digest:{hash:row.hash,size_bytes:row.size}})),stdout_raw:Buffer.from(JSON.stringify(note))});
 try{await content.writeActionResult(instance,key,result,signal);}catch(error){if((error as {code?:number})?.code===3)rmSync(root+".closures-sent.json",{force:true});throw error;}
 return{status:"uploaded",generation:note.generation,owners:note.owners,documents:rows.length,sent,bytes:bytesSent,closures:leaves};
}

/** ⬇️ Makes a store serve the generation another machine of the group uploaded: when the store is empty, or when it holds another state (other signature) that is older than the upload by more than `marginMilliseconds`. Every document is verified against its name, moved to the root of this machine and renamed after its new bytes, the manifest is renumbered above the local generation, and the pointer is written last. Rows keep the stamps of the producer: the fact check compares content when a stamp differs, so the load verifies the restored generation by content and recomputes only what differs. */
/** 🔭️ Whether the remote cache holds a state of this registry and platform that this store would take: the same decision `restoreGenerationV1` makes before it downloads a single document, so a load can go to the restore before it spends a verification pass on a local state that the restore would replace. */
export async function remoteGenerationAheadV1(location:Location&Readonly<{identity:string;marginMilliseconds?:number}>):Promise<boolean>{
 const local=await localState(location.root,location.files),remote=await remoteNote(location,{implementation:location.implementation});if(!remote)return false;
 const{note}=remote;if(!note||note.identity!==location.identity||note.platform!==location.platform||!generationScopeMatchesV1(note,location.scope))return false;
 return!(local&&keepsLocalV1(local,note,remote.from,location));
}

export async function restoreGenerationV1(location:Location&Readonly<{identity:string;marginMilliseconds?:number}>):Promise<GenerationExchangeOutcomeV1>{
 const{content,instance,root,files,signal}=location,local=await localState(root,files),remote=await remoteNote(location,{implementation:location.implementation,nearest:true});if(!remote)return{status:"absent"};
 const{note,entries}=remote;if(!note||note.identity!==location.identity||note.platform!==location.platform||!generationScopeMatchesV1(note,location.scope))return{status:"foreign"};
 if(local&&keepsLocalV1(local,note,remote.from,location))return{status:"local"};
 const manifestPath=`${P.manifests}/${note.manifest}.json`,snapshotPath=note.snapshot===undefined?undefined:`s/${note.snapshot}.snapshot.br`,documents=entries.filter(entry=>entry.path!==snapshotPath&&!closureDocument.test(entry.path));
 if(documents.length>policy.maximumDocuments+1||entries.length>policy.maximumDocuments+policy.maximumClosures+1||!documents.some(entry=>entry.path===manifestPath)||documents.some(entry=>!document.test(entry.path)||!entry.path.includes(entry.hash)||!(entry.size>=0)))return{status:"incomplete",reason:"action result"};
 const fetched:GenerationDocumentV1[]=[];
 await each(documents,async entry=>{
  const data=await content.read(instance,{hash:entry.hash,size:entry.size},signal);if(!data)throw Error("document missing in the remote cache: "+entry.path);
  if(data.length>policy.maximumDocumentBytes||sha(data)!==entry.hash)throw Error("document differs from its name: "+entry.path);
  fetched.push({path:entry.path,bytes:data});
 });
 const generation=Math.max(local?.generation??-1,note.generation)+1,moved=transformGenerationV1(fetched,location.workspace,"expand",generation);
 let bytes=0;
 await each(moved.documents,async item=>{await files.write(join(root,item.path),item.bytes,{ifAbsent:true});bytes+=item.bytes.length;});
 const manifestBytes=moved.documents.find(item=>item.path===moved.manifestPath)!.bytes,manifest=readGenerationManifestV1(manifestBytes);
 const closures=await profilePhaseV1("graph.restore.closures",()=>restoreClosuresV1(location,moved.documents,manifest.owners)).catch(()=>undefined),naming=new Set((closures?.missing??[]).flatMap(row=>row.projects)).size;
 await files.write(join(root,P.pointer),Buffer.from(JSON.stringify({version:1,generation:manifest.generation,manifest:moved.manifestPath.slice(P.manifests.length+1,-5),parent:null,reason:restoredReason})));
 const snapshotEntry=entries.find(entry=>entry.path===snapshotPath);
 if(snapshotEntry&&snapshotDocument.test(snapshotEntry.path)&&snapshotEntry.path.includes(note.snapshot!)&&snapshotEntry.size<=policy.maximumSnapshotBytes&&location.cursors)try{
  const data=await content.read(instance,{hash:snapshotEntry.hash,size:snapshotEntry.size},signal);
  if(data&&sha(data)===snapshotEntry.hash)await location.cursors.install(manifest.generation,{instance:note.snapshot!,bytes:data});
 }catch{}
 return{status:"restored",generation:manifest.generation,owners:note.owners,documents:moved.documents.length,bytes,ageSeconds:Math.max(0,Math.round(((location.now??Date.now)()-note.stateMs)/1000)),...note.stale===undefined&&naming===0?{}:{stale:Math.min(note.owners,Math.max(note.stale??0,naming))},...closures?{closures}:{}};
}

/** 🗄️ The closure leaves of the graph store of a workspace (`<graph partition>/m/<namespace>`), read with the storage budget of the workspace; `undefined` stays out of the exchange when the store cannot be located. */
export function workspaceClosurePortV1(workspace:string):ClosurePortV1|undefined{
 try{
  const base=join(repositoryCacheBudgetV1(workspace).partitions.graph.directory,"m"),directory=(namespace:ClosureNamespaceV1)=>join(base,namespace);
  return{directory,size:(namespace,key)=>{try{const row=statSync(join(directory(namespace),key+".json"));return row.isFile()?row.size:undefined;}catch{return undefined;}},read:async(namespace,key)=>{try{return readFileSync(join(directory(namespace),key+".json"));}catch{return undefined;}},put:closureLeafPutterV1(workspace)};
 }catch{return undefined;}
}

/** 🌐️ The exchange of this process: `undefined` while no remote cache endpoint is configured. A failure disables it for the rest of the process, so a dead cache server costs one short probe instead of a timeout per load. */
export function remoteGenerationExchangeV1(input:Readonly<{environment:Readonly<Record<string,string|undefined>>;workspace:string;identity():Promise<string>;platform?:string;cursors?:GenerationCursorsV1;origins?:Readonly<{self:string;all:readonly string[]}>;serving?:boolean;closures?:ClosurePortV1;scope?:string}>):GenerationExchangeV1|undefined{
 let resolved;try{resolved=resolveRemoteCacheV1(input.environment,{root:input.workspace});}catch{return undefined;}
 if(!resolved)return undefined;
 const contract=resolved.contract,signedFetch=remoteCacheFetchV1(resolved.environment,{root:input.workspace});
 const platform=input.platform??`${process.platform}-${process.arch}`.toLowerCase(),token=resolved.token,make=(probe:boolean)=>httpRemoteExecutionContentV1({endpoint:contract.endpoint,token,fetch:signedFetch,timeoutSeconds:probe?policy.probeTimeoutSeconds:contract.timeoutSeconds,retries:probe?policy.probeRetries:contract.retries});
 const origins=input.origins??fleetOriginsV1(input.workspace),serving=input.serving??totalmem()/1048576>=policy.servingTotalMegabytes,scope=input.scope??policy.allScope,heads=(identity:string)=>({key:generationExchangeKeyV1(identity,platform,origins.self,scope),keys:origins.all.map(origin=>generationExchangeKeyV1(identity,platform,origin,scope)),origin:origins.self,scope});
 const closures=input.closures??workspaceClosurePortV1(input.workspace);let down=false;
 const guard=async(run:()=>Promise<GenerationExchangeOutcomeV1>):Promise<GenerationExchangeOutcomeV1>=>{
  if(down)return{status:"failed",reason:"the remote cache was unreachable earlier in this process"};
  try{return await run();}catch(error){down=true;return{status:"failed",reason:String((error as Error)?.message??error).slice(0,200)};}
 };
 const exchange:GenerationExchangeV1={
  upload:remoteCacheEffectiveUploadV1(contract),
  restore:({root,fs,signal,implementation})=>guard(async()=>{const identity=await input.identity();return restoreGenerationV1({content:make(false),instance:contract.instance,...heads(identity),identity,root,workspace:input.workspace,platform,files:fs,signal,implementation,cursors:input.cursors,closures});}),
  ahead:async({root,fs,signal,implementation})=>{if(down)return false;try{const identity=await input.identity();return await remoteGenerationAheadV1({content:make(true),instance:contract.instance,...heads(identity),identity,root,workspace:input.workspace,platform,files:fs,signal,implementation,cursors:input.cursors});}catch{return false;}},
 publish:({root,fs,signal,ifAbsent,stale})=>guard(async()=>publishGenerationV1({content:make(ifAbsent===true),instance:contract.instance,...heads(await input.identity()),root,workspace:input.workspace,platform,files:fs,signal,ifAbsent,stale,serving,cursors:input.cursors,closures})),
  leaf:async({namespace,key,signal})=>{
   if(down)return undefined;
   try{const data=await make(true).read(contract.instance,{hash:key,size:0},signal);return data&&data.length>0&&data.length<=CLOSURE_POLICY_V1.maximumLeafBytes&&sha(data)===key?data:undefined;}catch{return undefined;}
  },
 };
 return Object.freeze(exchange);
}
