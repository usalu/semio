import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {createHash} from "node:crypto";
import {constants,readFileSync,readdirSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {open,type FileHandle} from "node:fs/promises";
import {totalmem} from "node:os";
import {join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {GENERATION_POLICY_V1 as P,readGenerationManifestV1,readGenerationPointerV1,type GenerationFilePortV1} from "../🟨️.mjs";
import {remoteCacheEffectiveUploadV1} from "../../../🌐️remote-cache/🟦️.ts";
import {remoteCacheFetchV1,resolveRemoteCacheV1} from "../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {httpRemoteExecutionContentV1,RemoteExecutionContentErrorV1,type RemoteExecutionContentV1} from "../../../🌐️remote-execution/🗄️content/🟦️.ts";
import {REAPI_V1,decodeProtobufV1,encodeProtobufV1} from "../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {documentNameV1,generationSignatureV1,neutralizeGenerationFromMemoryV1,neutralKeyV1,transformGenerationDocumentV1,transformGenerationManifestV1,transformNamedGenerationDocumentV1,type GenerationDocumentV1,type NeutralKnownV1,type NeutralRowV1} from "./🧳️portable/🟦️.ts";
import {profilePhaseV1} from "../../../⏱️profile/🟨️.mjs";
import {repositoryCacheBudgetV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import {CLOSURE_POLICY_V1,closureLeafPutterV1,obtainClosureLeafV1,type ClosureNamespaceV1} from "../../🩹️closure-repair/🟨️.mjs";
import {closureEntryV1,encodeClosureMemoV1,inventoryFragmentClosuresV1,packClosureShardsV1,readClosureMemoV1,readClosureShardsV1,type ClosureLeafV1,type ClosureShardV1,type PackedClosureShardV1} from "./🧲️closure-shards/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🏷️ The inline description of one uploaded generation: what a client compares before it downloads anything. `manifest` names the neutral manifest; `signature` is the root independent state; `stateMs` is the time of the tree state it describes and `stale` the owners its producer knew stale in it, which together rank the heads of different machines (`publishedMs` only dates the upload). */
export type GenerationNoteV1=Readonly<{version:1;generation:number;owners:number;identity:string;manifest:string;signature:string;platform:string;publishedMs:number;origin:string;stateMs:number;implementation:string;stale?:number;snapshot?:string;scope?:string}>;
/** 📍️ The state of the shared files a generation was verified against: a producer hands over the snapshot of its generation, a restoring machine installs it as the cursor of the restored generation. */
export type GenerationCursorsV1=Readonly<{snapshotFor(generation:number):Promise<{instance:string;bytes:Uint8Array}|undefined>;install(generation:number,snapshot:{instance:string;bytes:Uint8Array}):Promise<void>;distance?(snapshot:{instance:string;bytes:Uint8Array}):Promise<number|undefined>}>;
/** 🔗️ The producer's original neutral manifest, including when a receiver renumbers the generation. */
export type GenerationHeadEvidenceV1=Readonly<{manifest:string;origin:string;generation:number;scope:string}>;
/** 🎯️ Pins complete head evidence to the registry, platform and implementation that qualified it. */
export type GenerationHeadCandidateV1=GenerationHeadEvidenceV1&Readonly<{identity:string;platform:string;implementation:string}>;
/** 🧾️ What one exchange did; a refusal or a failure never fails the graph load that asked. */
export type GenerationExchangeOutcomeV1=Readonly<{status:"restored"|"uploaded"|"local"|"absent"|"foreign"|"empty"|"current"|"remote-better"|"incomplete"|"oversized"|"failed";generation?:number;localManifest?:string;owners?:number;documents?:number;sent?:number;bytes?:number;ageSeconds?:number;stale?:number;reason?:string;closures?:GenerationClosuresV1;head?:GenerationHeadEvidenceV1}>;
/** 🧲️ What an exchange did with the build input closure leaves a generation names: how many it names, how many were uploaded or fetched, and the leaves it could not obtain with the owners that name them (those owners are stale, the generation is not complete). */
export type GenerationClosuresV1=Readonly<{named:number;sent:number;fetched:number;missing:readonly Readonly<{key:string;namespace:ClosureNamespaceV1;projects:readonly string[]}>[]}>;
/** 🗄️ Where the closure leaves of this machine live and how they are read and written: the graph store of the workspace by default. */
export interface ClosurePortV1{
 directory(namespace:ClosureNamespaceV1):string;
 size(namespace:ClosureNamespaceV1,key:string):number|undefined;
 /** 📏️ Reads one leaf, bounding allocation to the supplied maximum when present. */
 read(namespace:ClosureNamespaceV1,key:string,maximumBytes?:number):Promise<Uint8Array|undefined>;
 verify?(namespace:ClosureNamespaceV1,key:string,signal?:AbortSignal):Promise<boolean|undefined>;
 put(namespace:ClosureNamespaceV1,key:string,bytes:Uint8Array):Promise<void>;
}
/** 🪟️ The verdict of one look at the heads of the group: `none` (no head of this registry, platform and scope), `other-code` (built by other code, a reader would find every owner stale), `partial` (its producer knew `stale` owners stale in it, a read is still going on), `complete` (its producer knew no stale owner); `ahead` tells whether a restore would take it and `held` how many owners the sealed generation of this machine has (0 without one). */
export type GenerationLookV1=Readonly<{state:"none"|"other-code"|"partial"|"complete";ahead:boolean;held:number;owners?:number;stale?:number;origin?:string;ageSeconds?:number;candidate?:GenerationHeadCandidateV1}>;
/** 🔌️ The two operations the generation load asks of the remote cache of the machine group. */
export interface GenerationExchangeV1{
 /** ⬆️ Whether this machine may write to the remote cache. */
 readonly upload:boolean;
 /** 📥️ A pinned restore accepts only the exact complete head and uses the caller’s original generation writer. */
 restore(options:{root:string;fs:GenerationFilePortV1;signal?:AbortSignal;implementation?:string;requireComplete?:true;expectedHead?:GenerationHeadCandidateV1}):Promise<GenerationExchangeOutcomeV1>;
 /** 🔭️ Whether a restore would take a newer state of a peer (one short request, nothing downloaded). */
 ahead(options:{root:string;fs:GenerationFilePortV1;signal?:AbortSignal;implementation?:string}):Promise<boolean>;
 /** 👀️ What the cache holds of this registry, platform and scope without downloading a document. A complete-only look prefers the complete compatible heads a strict restore can consider. */
 look(options:{root:string;fs:GenerationFilePortV1;signal?:AbortSignal;implementation?:string;requireComplete?:true}):Promise<GenerationLookV1>;
 /** ⬆️ Offers the newest sealed generation unless the cache holds a head that is at least as good; `stale` is the number of owners this producer knows stale in it (0 after a complete read, absent when unknown). */
 publish(options:{root:string;fs:GenerationFilePortV1;signal?:AbortSignal;ifAbsent?:boolean;stale?:number}):Promise<GenerationExchangeOutcomeV1>;
 /** 🧲️ One build input closure leaf from the fleet cache, verified against its name; nothing when the cache does not hold it or is unreachable. */
 leaf(options:{namespace:ClosureNamespaceV1;key:string;signal?:AbortSignal}):Promise<Uint8Array|undefined>;
}
type Files=Pick<GenerationFilePortV1,"read"|"write"|"list">;
type Location=Readonly<{content:RemoteExecutionContentV1;instance:string;key:string;root:string;workspace:string;platform:string;files:Files;signal?:AbortSignal;now?:()=>number;cursors?:GenerationCursorsV1;keys?:readonly string[];origin?:string;implementation?:string;stale?:number;serving?:boolean;closures?:ClosurePortV1;scope?:string}>;

const restoredReason="restored from the remote cache",policy=schema.definitions.Policy.const,document=new RegExp(policy.document,"u"),snapshotDocument=new RegExp(policy.snapshot,"u"),sha=(bytes:Uint8Array):string=>createHash("sha256").update(bytes).digest("hex");

/** 🗝️ The action key of the head of the generations one machine of the group published, for a registry on one platform: the generation format, the owner registry identity, the platform and the origin that writes it, never a path. The server keeps the first bytes of a key against every other origin (a key must determine its bytes), so each origin owns one head, overwrites only its own, and a reader takes the newest head of all origins. */
export function generationExchangeKeyV1(identity:string,platform:string,origin:string,scope:string=policy.allScope):string{return sha(Buffer.from(JSON.stringify(scope===policy.allScope?[policy.domain,identity,platform,origin]:[policy.domain,identity,platform,origin,scope])));}

/** 🧷️ Receipt fields of a head whose declared origin agrees with the addressed slot. */
function generationHeadEvidenceV1(note:GenerationNoteV1,key:string):GenerationHeadEvidenceV1|undefined{
 const head={manifest:note.manifest,origin:note.origin,generation:note.generation,scope:note.scope??policy.allScope};
 if(validateJsonSchemaSubset({$ref:"#/definitions/HeadEvidence"},head,schema).length)return undefined;
 return generationExchangeKeyV1(note.identity,note.platform,head.origin,head.scope)===key?head:undefined;
}

/** 🔭️ Whether a head describes the scope a reader wants: a head of one slice is never the generation of another slice or of the whole workspace, and the whole workspace is the scope of a head that names none. */
export const generationScopeMatchesV1=(note:Readonly<{scope?:string}>,scope:string|undefined):boolean=>(note.scope??policy.allScope)===(scope??policy.allScope);

type HeadV1=Readonly<{note:GenerationNoteV1|undefined;entries:readonly {path:string;hash:string;size:number}[];shards:readonly ClosureShardV1[];from:string}>;
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
 if(bytes.length>policy.maximumActionResultBytes)return undefined;
 const result=decodeProtobufV1(REAPI_V1.actionResult,bytes),files=result.output_files as {path:string;digest:{hash:string;size_bytes:number|string}}[],directories=result.output_directories as {path:string;tree_digest?:{hash:string;size_bytes:number|string}}[];
 return{note:noteOf(result.stdout_raw as Uint8Array|undefined),entries:files.map(file=>({path:file.path,hash:file.digest.hash,size:Number(file.digest.size_bytes)})),shards:directories.map(row=>({path:row.path,hash:row.tree_digest?.hash??"",size:Number(row.tree_digest?.size_bytes)})),from:location.key};
}

async function remoteNote(location:Location,reader:Readonly<{implementation?:string;nearest?:boolean;prefer?:(head:RankedV1)=>boolean}>={}):Promise<HeadV1|undefined>{
 const keys=location.keys??[location.key],settled=await Promise.allSettled(keys.map(key=>readHead({...location,key,keys:undefined})));
 if(settled.every(row=>row.status==="rejected"))throw(settled[0] as PromiseRejectedResult).reason;
 const found=settled.flatMap(row=>row.status==="fulfilled"&&row.value!==undefined?[row.value]:[]),valid=found.filter((head):head is RankedV1=>head.note!==undefined);
 if(valid.length===0)return found[0];
 const preferred=reader.prefer===undefined?valid:valid.filter(reader.prefer),now=(location.now??Date.now)();
 const ranked=rankGenerationHeadsV1(preferred.length?preferred:valid,now,{implementation:reader.implementation});
 if(!preferred.length||!reader.nearest||ranked.length<2||!location.cursors?.distance)return ranked[0];
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

function matchingHeadV1(location:Location&Readonly<{identity:string}>,note:GenerationNoteV1):boolean{
 return note.identity===location.identity&&note.platform===location.platform&&generationScopeMatchesV1(note,location.scope);
}

function completeAheadHeadV1(location:Location&Readonly<{identity:string;marginMilliseconds?:number}>,local:Awaited<ReturnType<typeof localState>>,head:RankedV1):boolean{
 const {note,from}=head;
 return note.stale===0&&note.implementation===location.implementation&&matchingHeadV1(location,note)
  &&generationHeadEvidenceV1(note,from)!==undefined&&!(local&&keepsLocalV1(local,note,from,location));
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

/** 👥️ Resolves owning projects only for closures that need a missing-leaf report. */
function closureOwnersOfV1(owners:ReturnType<typeof readGenerationManifestV1>["owners"]){
 const byFragment=new Map<string,string[]>();
 for(const [name,row] of Object.entries(owners)){const names=byFragment.get(row.fragment)??[];names.push(name);byFragment.set(row.fragment,names);}
 return (fragments:ReadonlySet<string>)=>[...fragments].flatMap(fragment=>byFragment.get(fragment)??[]).sort();
}

type UploadedClosuresV1={report:GenerationClosuresV1;staleOwners:number|undefined;leaves:readonly ClosureLeafV1[];shards:readonly PackedClosureShardV1[]};

/** 📒 Reads the closure keys previously checked or sent to this CAS instance. */
function readSentClosuresV1(file:string,instance:string):Set<string>{
 try{
  const value=JSON.parse(readFileSync(file,"utf8")) as {version:number;instance:string;keys:string[]};
  if(value.version===1&&value.instance===instance&&Array.isArray(value.keys))return new Set(value.keys);
 }catch{}
 return new Set();
}

/** 💾 Atomically remembers the closure keys qualified for the next publication. */
function writeSentClosuresV1(file:string,instance:string,keys:ReadonlySet<string>):void{
 const temporary=file+"."+process.pid+".tmp";
 try{writeFileSync(temporary,JSON.stringify({version:1,instance,keys:[...keys]}));renameSync(temporary,file);}catch{rmSync(temporary,{force:true});}
}

/** 📤 Sends a closure only when the current local bytes match its bounded digest. */
async function sendClosureV1({closures,signal,content,instance}:Location,row:ClosureLeafV1):Promise<boolean>{
 signal?.throwIfAborted();
 const data=await closures?.read(row.namespace,row.key,row.size);
 if(!data||data.length!==row.size||sha(data)!==row.key)return false;
 await content.write(instance,{hash:row.key,size:row.size},data,signal);
 return true;
}

/** 🛠️ Requalifies the addressed closure metadata and leaves after a cache eviction, then retries the same head once. */
async function writeGenerationHeadV1(location:Location,result:Uint8Array,uploaded:UploadedClosuresV1):Promise<number>{
 const{content,instance,key,signal,root}=location;
 const rejected=await content.writeActionResult(instance,key,result,signal).then(()=>undefined,error=>{
  const missing=error instanceof RemoteExecutionContentErrorV1&&error.code===3?error.missingOutput:undefined;
  if(missing===undefined||!uploaded.leaves.some(row=>row.key===missing)&&!uploaded.shards.some(row=>row.hash===missing))throw error;
  return missing;
 });
 if(rejected===undefined)return 0;
 const leaves=new Map(uploaded.leaves.map(row=>[row.key,row] as const)),shards=new Map(uploaded.shards.map(row=>[row.hash,row] as const));
 const checked=[...shards.values(),...[...leaves.values()].map(row=>({hash:row.key,size:row.size}))],unchecked=new Set([rejected]);let repaired=0;
 try{
  signal?.throwIfAborted();
  for(const row of await content.missing(instance,checked.filter(row=>row.hash!==rejected),signal))unchecked.add(row.hash);
  await each([...unchecked],async digest=>{
   const shard=shards.get(digest),leaf=leaves.get(digest);
   if(shard)await content.write(instance,shard,shard.bytes,signal);
   else if(leaf&&await sendClosureV1(location,leaf))repaired++;
   else return;
   unchecked.delete(digest);
  });
  signal?.throwIfAborted();
  if(unchecked.size)throw Error("Generation closure no longer available: "+unchecked.values().next().value);
  await content.writeActionResult(instance,key,result,signal);
  return repaired;
 }catch(error){
  if(error instanceof RemoteExecutionContentErrorV1&&error.code===3&&error.missingOutput!==undefined&&leaves.has(error.missingOutput))unchecked.add(error.missingOutput);
  if(unchecked.size){
   const file=root+".closures-sent.json",memo=readSentClosuresV1(file,instance);
   for(const hash of unchecked)memo.delete(hash);
   writeSentClosuresV1(file,instance,memo);
  }
  throw error;
 }
}

/** ⬆️ Uploads the closure leaves a generation names, deduplicated and bounded, before its head is written: every leaf this machine holds that the cache misses (the cache is asked once, by name and size) is read, verified against its name and written; a leaf this machine does not hold is reported with the owners that name it and never fails the upload. */
async function uploadClosuresV1(location:Location,manifest:ReturnType<typeof readGenerationManifestV1>,fragments:readonly string[]):Promise<UploadedClosuresV1>{
 const{closures,content,instance,signal,files,root}=location;
 const memoPath=root+".closures.json",memo=readClosureMemoV1(await readBoundedClosureLeafV1(memoPath,policy.maximumClosureMemoBytes).catch(()=>undefined));
 const unchanged=fragments.length===memo.size&&fragments.every(name=>memo.has(name));
 const references=await inventoryFragmentClosuresV1(fragments,name=>files.read(join(root,P.fragments,name+".json.br")),signal,memo);
 if(!unchanged){
  const text=encodeClosureMemoV1(fragments,memo),staged=memoPath+"."+process.pid+".tmp";
  if(text!==undefined)try{writeFileSync(staged,text);renameSync(staged,memoPath);}catch{rmSync(staged,{force:true});}
 }
 const named=[...references].sort((a,b)=>a[0]<b[0]?-1:1),held:ClosureLeafV1[]=[];
 for(const[,{key,namespace}]of named){
  if(held.length===policy.maximumClosures)break;
  signal?.throwIfAborted();
  const size=closures?.size(namespace,key);
  if(size!==undefined&&Number.isSafeInteger(size)&&size>0&&size<=CLOSURE_POLICY_V1.maximumLeafBytes)held.push({key,namespace,size});
 }
 const sentFile=root+".closures-sent.json",before=readSentClosuresV1(sentFile,instance);
 const asking=held.filter(row=>!before.has(row.key)),absent=new Set((await content.missing(instance,asking.map(row=>({hash:row.key,size:row.size})),signal)).map(digest=>digest.hash)),rejected=new Set<string>();let sent=0;
 await each(asking.filter(row=>absent.has(row.key)),async row=>{
  if(await sendClosureV1(location,row))sent++;
  else rejected.add(closureEntryV1(row.namespace,row.key));
 });
 const leaves=held.filter(row=>!rejected.has(closureEntryV1(row.namespace,row.key))),shards=packClosureShardsV1(leaves);
 const missingShards=new Set((await content.missing(instance,shards,signal)).map(row=>row.hash));
 await each(shards.filter(row=>missingShards.has(row.hash)),row=>content.write(instance,row,row.bytes,signal));
 writeSentClosuresV1(sentFile,instance,new Set(leaves.map(row=>row.key)));
 const listed=new Set(leaves.map(row=>closureEntryV1(row.namespace,row.key))),ownersOf=closureOwnersOfV1(manifest.owners);
 const stale=new Set<string>(),missing:GenerationClosuresV1["missing"][number][]=[];let globalMissing=false;
 for(const [entry,{key,namespace,fragments}] of named)if(!listed.has(entry)){
  if(fragments.has(manifest.global))globalMissing=true;
  const projects=ownersOf(fragments);
  for(const project of projects)stale.add(project);
  if(missing.length<policy.maximumClosureReports)missing.push({key,namespace,projects});
 }
 const staleOwners=globalMissing?(Object.keys(manifest.owners).length||undefined):stale.size;
 return{report:{named:named.length,sent,fetched:0,missing},staleOwners,leaves,shards};
}

/** 🌊️ Reads an unknown-length closure response into bounded storage, cancelling its stream on overflow. */
async function readRemoteClosureLeafV1({content,instance,signal}:Pick<Location,"content"|"instance"|"signal">,key:string,size?:number):Promise<Uint8Array|undefined>{
 signal?.throwIfAborted();
 const stream=await content.stream(instance,{hash:key,size:size??0},signal);
 if(!stream)return undefined;
 const maximum=size??CLOSURE_POLICY_V1.maximumLeafBytes;
 let bytes=Buffer.alloc(0),length=0;
 for await(const chunk of stream){
  signal?.throwIfAborted();
  if(chunk.length>maximum-length)return undefined;
  const next=length+chunk.length;
  if(next>bytes.length){
   const grown=Buffer.allocUnsafe(Math.min(maximum,Math.max(4096,next,bytes.length*2)));
   grown.set(bytes.subarray(0,length));bytes=grown;
  }
  bytes.set(chunk,length);length=next;
 }
 return size!==undefined&&length!==size?undefined:bytes.subarray(0,length);
}

/** ⬇️ Obtains the closure leaves the fragments of a generation about to be restored name and this machine lacks: each is read from the fleet cache by name (bounded and verified), and the leaves nobody has are reported with the owners that name them, whose records the load removes so they are derived again. */
async function restoreClosuresV1(location:Location,manifest:ReturnType<typeof readGenerationManifestV1>,inventory:ReadonlyMap<string,ClosureLeafV1>,requireComplete:boolean,completeInventory:boolean):Promise<GenerationClosuresV1|undefined>{
 const{closures,signal,files,root}=location;
 const names=[...new Set<string>([manifest.global,...Object.values(manifest.owners).map(owner=>owner.fragment)])];
 const named=await inventoryFragmentClosuresV1(names,name=>files.read(join(root,P.fragments,name+".json.br")),signal);
 if([...inventory.keys()].some(entry=>!named.has(entry))||completeInventory&&inventory.size!==named.size)throw new RestoreDocumentErrorV1("Generation closure inventory differs from its fragments");
 if(!closures)return named.size===0?{named:0,sent:0,fetched:0,missing:[]}:undefined;
 const verify=requireComplete?async(namespace:ClosureNamespaceV1,key:string):Promise<boolean>=>{
  signal?.throwIfAborted();
  const size=closures.size(namespace,key),maximum=CLOSURE_POLICY_V1.maximumLeafBytes;
  if(size===undefined||size<1||size>maximum||size!==inventory.get(closureEntryV1(namespace,key))?.size)return false;
  const verified=await closures.verify?.(namespace,key,signal);
  if(verified!==undefined)return verified;
  const bytes=await closures.read(namespace,key,maximum).catch(()=>undefined);
  return bytes!==undefined&&bytes.length===size&&sha(bytes)===key;
 }:undefined;
 const rows=[...named],absent:typeof rows=[];
 if(verify)await each(rows,async row=>{if(!await verify(row[1].namespace,row[1].key))absent.push(row);});
 else for(const row of rows)if(closures.size(row[1].namespace,row[1].key)===undefined)absent.push(row);
 const recovered=new Set<string>();let got=0;
 await each(absent.slice(0,policy.maximumClosures),async([entry,{namespace,key}])=>{
  const state=await obtainClosureLeafV1({directory:closures.directory,namespace,key,fetch:()=>readRemoteClosureLeafV1(location,key,inventory.get(entry)?.size),put:closures.put,signal,verify});
  if(state==="refetched")got++;
  if(state!=="missing")recovered.add(entry);
 });
 const ownersOf=closureOwnersOfV1(manifest.owners);
 const missing=absent.filter(([entry])=>!recovered.has(entry)).sort(([a],[b])=>a<b?-1:1).slice(0,policy.maximumClosureReports).map(([,row])=>({key:row.key,namespace:row.namespace,projects:ownersOf(row.fragments)}));
 return{named:named.size,sent:0,fetched:got,missing};
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

/** ⬆️ Uploads the newest sealed generation of one store in its neutral form (root replaced by the root token, every document renamed after its new bytes), unless the cache already holds this state (same signature, with a verified retry able to complete its own partial head) or a head of another machine that is at least as good (it leaves its readers the same number of stale owners or fewer; a machine under the serving memory size must beat it by the shadow margin). Only the documents the cache misses are sent, so an interrupted upload resumes from what arrived. Documents go first, the action result that names them last. */
export async function publishGenerationV1(location:Location&Readonly<{ifAbsent?:boolean}>):Promise<GenerationExchangeOutcomeV1>{
 const{content,instance,key,root,files,signal}=location;
 signal?.throwIfAborted();
 const local=await localState(root,files);if(!local)return{status:"empty"};
 const signature=generationSignatureV1(local.manifest),remote=await remoteNote(location,{implementation:local.manifest.implementation}).catch(()=>undefined),owners=Object.keys(local.manifest.owners).length;
 const stateMs=Math.floor(local.modifiedMs),current=remote?.note?.signature===signature&&(local.restored||remote.note.stateMs>=stateMs);
 const completing=current&&!local.restored&&location.stale===0&&remote?.from===key&&remote.note?.stateMs===stateMs&&remote.note.stale!==0;
 if(current&&!completing)return{status:"current",generation:local.generation,owners};
 const mine=Math.min(owners,location.stale??policy.unmeasuredStale)+(location.serving===false?policy.shadowMarginOwners:0);
 if(!completing&&remote&&(location.ifAbsent||remote.note!==undefined&&remote.from!==location.key&&generationHeadScoreV1(remote.note,(location.now??Date.now)(),{implementation:local.manifest.implementation})<=mine))return{status:"remote-better",generation:local.generation,owners};
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
 const fragmentNames=[...new Set<string>([local.manifest.global,...Object.values(local.manifest.owners).map(owner=>owner.fragment)])].sort(),uploaded=await profilePhaseV1("graph.upload.closures",()=>uploadClosuresV1(location,local.manifest,fragmentNames)).catch((error):UploadedClosuresV1=>({staleOwners:undefined,leaves:[],shards:[],report:{named:0,sent:0,fetched:0,missing:[{key:"0".repeat(64),namespace:"build-input-closures",projects:[String(error?.message??error).slice(0,120)]}]}}));
 signal?.throwIfAborted();
 const stale=location.stale===undefined||uploaded.staleOwners===undefined?undefined:Math.min(owners,Math.max(0,Math.floor(location.stale))+uploaded.staleOwners);
 if(completing&&stale!==0)return{status:"current",generation:local.generation,owners,closures:uploaded.report};
 const note:GenerationNoteV1={version:1,generation:local.generation,owners,identity:local.manifest.identity,manifest:manifestName,signature,platform:location.platform,publishedMs:(location.now??Date.now)(),origin:location.origin??"local",stateMs,implementation:local.manifest.implementation,...stale===undefined?{}:{stale},...shipped?{snapshot:shipped.instance}:{},...location.scope===undefined||location.scope===policy.allScope?{}:{scope:location.scope}};
 const result=encodeProtobufV1(REAPI_V1.actionResult,{output_files:rows.map(row=>({path:row.path,digest:{hash:row.hash,size_bytes:row.size}})),output_directories:uploaded.shards.map(row=>({path:row.path,tree_digest:{hash:row.hash,size_bytes:row.size}})),stdout_raw:Buffer.from(JSON.stringify(note))});
 if(result.length>policy.maximumActionResultBytes)return{status:"oversized",reason:String(result.length)+" ActionResult bytes",closures:uploaded.report};
 const repaired=await profilePhaseV1("graph.upload.head",()=>writeGenerationHeadV1(location,result,uploaded));
 const head=generationHeadEvidenceV1(note,key);
 return{status:"uploaded",generation:note.generation,owners:note.owners,documents:rows.length,sent,bytes:bytesSent,closures:{...uploaded.report,sent:uploaded.report.sent+repaired},...head?{head}:{}};
}

/** ⬇️ Makes a store serve the generation another machine of the group uploaded: when the store is empty, or when it holds another state (other signature) that is older than the upload by more than `marginMilliseconds`. Every document is verified against its name, moved to the root of this machine and renamed after its new bytes, the manifest is renumbered above the local generation, and the pointer is written last. Rows keep the stamps of the producer: the fact check compares content when a stamp differs, so the load verifies the restored generation by content and recomputes only what differs. */
/** 🔭️ Whether the remote cache holds a state of this registry and platform that this store would take: the same decision `restoreGenerationV1` makes before it downloads a single document, so a load can go to the restore before it spends a verification pass on a local state that the restore would replace. */
export async function remoteGenerationAheadV1(location:Location&Readonly<{identity:string;marginMilliseconds?:number}>):Promise<boolean>{
 const local=await localState(location.root,location.files),remote=await remoteNote(location,{implementation:location.implementation});if(!remote)return false;
 const{note}=remote;if(!note||!matchingHeadV1(location,note))return false;
 return!(local&&keepsLocalV1(local,note,remote.from,location));
}

/** 🔍️ Looks at the heads of the group like {@link remoteGenerationAheadV1} and says what they hold, so a machine without a generation can decide to fetch a complete head, wait for the machine that reads, or read. */
export async function remoteGenerationLookV1(location:Location&Readonly<{identity:string;marginMilliseconds?:number;requireComplete?:true}>):Promise<GenerationLookV1>{
 const local=await localState(location.root,location.files),held=local?Object.keys(local.manifest.owners).length:0;
 if(location.requireComplete&&location.implementation===undefined)return{ahead:false,held,state:"none"};
 const remote=await remoteNote(location,{implementation:location.implementation,...location.requireComplete?{prefer:head=>completeAheadHeadV1(location,local,head)}:{}}),note=remote?.note;
 if(!remote||!note||!matchingHeadV1(location,note)||location.requireComplete&&generationHeadEvidenceV1(note,remote.from)===undefined)return{ahead:false,held,state:"none"};
 const state=location.implementation!==undefined&&note.implementation!==location.implementation?"other-code":note.stale===0?"complete":"partial";
 const ahead=!(local&&keepsLocalV1(local,note,remote.from,location)),evidence=location.requireComplete&&ahead&&state==="complete"?generationHeadEvidenceV1(note,remote.from):undefined;
 const candidate=evidence?{...evidence,identity:note.identity,platform:note.platform,implementation:note.implementation}:undefined,facts={owners:note.owners,origin:note.origin,ageSeconds:Math.max(0,Math.round(((location.now??Date.now)()-note.stateMs)/1000)),...note.stale===undefined?{}:{stale:note.stale}};
 return{ahead,held,state,...facts,...candidate?{candidate}:{}};
}

type RemoteDocumentV1=HeadV1["entries"][number];
class RestoreDocumentErrorV1 extends Error{}

/** 📥️ Reads exactly the addressed compressed bytes and cancels the incoming body on a length mismatch. */
async function readRestoreDocumentV1({content,instance,signal}:Location,entry:RemoteDocumentV1):Promise<GenerationDocumentV1>{
 signal?.throwIfAborted();
 const stream=await content.stream(instance,{hash:entry.hash,size:entry.size},signal);
 if(!stream)throw new RestoreDocumentErrorV1("document missing in the remote cache: "+entry.path);
 const bytes=Buffer.allocUnsafe(entry.size),hash=createHash("sha256");
 let offset=0;
 for await(const chunk of stream){
  signal?.throwIfAborted();
  if(chunk.length>entry.size-offset)throw new RestoreDocumentErrorV1("document differs from its declared size: "+entry.path);
  bytes.set(chunk,offset);offset+=chunk.length;hash.update(chunk);
 }
 if(offset!==entry.size||hash.digest("hex")!==entry.hash)throw new RestoreDocumentErrorV1("document differs from its name: "+entry.path);
 return{path:entry.path,bytes};
}

/** 📮️ Stages one compressed read group at a time, with one expanded document active. A large document gets its own group. The current pointer is published separately. */
async function stageRestoreDocumentsV1(location:Location,documents:readonly RemoteDocumentV1[]):Promise<{renamed:Map<string,string>;bytes:number}>{
 const{files,root,workspace,signal}=location,renamed=new Map<string,string>();
 let bytes=0,index=0;
 while(index<documents.length){
  const group:RemoteDocumentV1[]=[];let size=0;
  while(index<documents.length&&group.length<policy.concurrency){
   const next=documents[index]!;
   if(group.length&&size+next.size>policy.restoreBatchBytes)break;
   group.push(next);size+=next.size;index++;
  }
  const fetched=await Promise.allSettled(group.map(entry=>readRestoreDocumentV1(location,entry)));
  for(const row of fetched)if(row.status==="rejected")throw row.reason;
  for(const row of fetched){
   if(row.status!=="fulfilled")continue;
   signal?.throwIfAborted();
   const moved=transformNamedGenerationDocumentV1(row.value,workspace,"expand"),path=join(root,moved.path),name=documentNameV1(moved.path);
   if(moved.bytes.length>policy.maximumDocumentBytes)throw new RestoreDocumentErrorV1("expanded generation document exceeds its finite limit: "+moved.path);
   if(await files.write(path,moved.bytes,{ifAbsent:true})===false){
    const held=await files.read(path);
    if(!held||sha(held)!==name)throw new RestoreDocumentErrorV1("stored generation document differs from its name: "+moved.path);
   }
   renamed.set(documentNameV1(row.value.path),name);bytes+=moved.bytes.length;
  }
 }
 return{renamed,bytes};
}

export async function restoreGenerationV1(location:Location&Readonly<{identity:string;marginMilliseconds?:number;requireComplete?:true;expectedHead?:GenerationHeadCandidateV1}>):Promise<GenerationExchangeOutcomeV1>{
 const{content,instance,root,files,signal,expectedHead}=location,requireComplete=location.requireComplete===true||expectedHead!==undefined;
 if(requireComplete&&location.implementation===undefined)return{status:"incomplete",reason:"complete compatible generation required"};
 const expectedKey=expectedHead&&generationExchangeKeyV1(location.identity,location.platform,expectedHead.origin,expectedHead.scope);
 if(expectedHead&&(expectedHead.identity!==location.identity||expectedHead.platform!==location.platform||expectedHead.implementation!==location.implementation||expectedHead.scope!==(location.scope??policy.allScope)||!(location.keys??[location.key]).includes(expectedKey!)))return{status:"foreign",reason:"expected generation head is outside this exchange"};
 const local=await localState(root,files);
 const remote=expectedKey?await readHead({...location,key:expectedKey,keys:undefined}):await remoteNote(location,{implementation:location.implementation,nearest:true,...requireComplete?{prefer:head=>completeAheadHeadV1(location,local,head)}:{}});
 if(!remote)return{status:"absent"};
 const{note,entries,shards}=remote;
 if(expectedHead){
  const evidence=note&&generationHeadEvidenceV1(note,remote.from);
  if(!note||!evidence||!matchingHeadV1(location,note)||note.stale!==0||note.implementation!==expectedHead.implementation||evidence.origin!==expectedHead.origin||evidence.scope!==expectedHead.scope||evidence.generation!==expectedHead.generation||evidence.manifest!==expectedHead.manifest)return{status:"incomplete",reason:"expected generation head changed"};
 }
 if(!note||!matchingHeadV1(location,note)||requireComplete&&generationHeadEvidenceV1(note,remote.from)===undefined)return{status:"foreign"};
 if(local&&keepsLocalV1(local,note,remote.from,location))return{status:"local"};
 if(requireComplete&&(note.stale!==0||note.implementation!==location.implementation))return{status:"incomplete",reason:"complete compatible generation required"};
 const manifestPath=`${P.manifests}/${note.manifest}.json`,snapshotPath=note.snapshot===undefined?undefined:`s/${note.snapshot}.snapshot.br`,documents=entries.filter(entry=>entry.path!==snapshotPath);
 const indexed=new Map(documents.map(entry=>[entry.path,entry])),sourceEntry=indexed.get(manifestPath);
 if(entries.length>policy.maximumDocuments+1||!sourceEntry||sourceEntry.size>P.maximumManifestBytes||indexed.size!==documents.length||documents.some(entry=>!document.test(entry.path)||entry.hash!==documentNameV1(entry.path)||!Number.isSafeInteger(entry.size)||entry.size<=0||entry.size>policy.maximumDocumentBytes))return{status:"incomplete",reason:"action result"};
 const uniqueManifest=documents.filter(entry=>entry.path.startsWith(P.manifests+"/")).length===1;
 if(requireComplete&&!uniqueManifest)return{status:"incomplete",reason:"one generation manifest required"};
 const source=await readRestoreDocumentV1(location,sourceEntry);
 let sourceManifest:ReturnType<typeof readGenerationManifestV1>;
 try{sourceManifest=readGenerationManifestV1(source.bytes);}catch{return{status:"incomplete",reason:"invalid generation manifest"};}
 if(requireComplete&&(sourceManifest.generation!==note.generation||sourceManifest.identity!==note.identity||sourceManifest.implementation!==note.implementation||Object.keys(sourceManifest.owners).length!==note.owners))return{status:"incomplete",reason:"generation manifest differs from its head"};
 const required=generationDocumentPathsV1(note.manifest,sourceManifest).slice(1);
 if(required.some(path=>!indexed.has(path))||requireComplete&&required.length+1!==documents.length)return{status:"incomplete",reason:"generation documents differ from their manifest"};
 const inventory=await readClosureShardsV1(shards,row=>content.stream(instance,row,signal),signal).catch(()=>undefined);
 signal?.throwIfAborted();
 if(!inventory)return{status:"incomplete",reason:"required generation closure inventory unavailable"};
 const {renamed,bytes:stagedBytes}=await profilePhaseV1("graph.restore.documents",()=>stageRestoreDocumentsV1(location,required.map(path=>indexed.get(path)!)));
 const generation=Math.max(local?.generation??-1,note.generation)+1,{document:moved}=transformGenerationManifestV1(source.bytes,renamed,location.workspace,"expand",generation),manifest=readGenerationManifestV1(moved.bytes);
 if(moved.bytes.length>P.maximumManifestBytes)return{status:"incomplete",reason:"expanded generation manifest exceeds its finite limit"};
 let closures;
 try{closures=await profilePhaseV1("graph.restore.closures",()=>restoreClosuresV1(location,manifest,inventory,requireComplete,note.stale===0));}
 catch(error){signal?.throwIfAborted();return{status:"incomplete",reason:String((error as Error).message)};}
 signal?.throwIfAborted();
 if(requireComplete&&(!closures||closures.missing.length>0))return{status:"incomplete",reason:"required generation closures unavailable",closures};
 const naming=new Set((closures?.missing??[]).flatMap(row=>row.projects)).size,bytes=stagedBytes+moved.bytes.length,manifestName=documentNameV1(moved.path);
 if(await files.write(join(root,moved.path),moved.bytes,{ifAbsent:true})===false){
  const held=await files.read(join(root,moved.path));
  if(!held||sha(held)!==manifestName)throw Error("stored generation manifest differs from its name: "+moved.path);
 }
 signal?.throwIfAborted();
 await files.write(join(root,P.pointer),Buffer.from(JSON.stringify({version:1,generation:manifest.generation,manifest:manifestName,parent:null,reason:restoredReason})));
 const snapshotEntry=entries.find(entry=>entry.path===snapshotPath);
 if(!expectedHead&&snapshotEntry&&snapshotDocument.test(snapshotEntry.path)&&snapshotEntry.path.includes(note.snapshot!)&&snapshotEntry.size<=policy.maximumSnapshotBytes&&location.cursors)try{
  const data=await content.read(instance,{hash:snapshotEntry.hash,size:snapshotEntry.size},signal);
  if(data&&sha(data)===snapshotEntry.hash)await location.cursors.install(manifest.generation,{instance:note.snapshot!,bytes:data});
 }catch{}
 const head=uniqueManifest&&sourceManifest.generation===note.generation?generationHeadEvidenceV1(note,remote.from):undefined;
 return{status:"restored",generation:manifest.generation,localManifest:manifestName,owners:note.owners,documents:required.length+1,bytes,ageSeconds:Math.max(0,Math.round(((location.now??Date.now)()-note.stateMs)/1000)),...note.stale===undefined&&naming===0?{}:{stale:Math.min(note.owners,Math.max(note.stale??0,naming))},...closures?{closures}:{},...head?{head}:{}};
}

/** 📏️ Reads a regular local closure within its caller's byte limit even if the directory entry changes after the size check. */
async function withBoundedClosureLeafV1<T>(path:string,maximumBytes:number,read:(file:FileHandle,size:number)=>Promise<T|undefined>):Promise<T|undefined>{
 const file=await open(path,constants.O_RDONLY|(constants.O_NONBLOCK??0));
 try{
  const status=await file.stat();
  if(!status.isFile()||status.size<1||status.size>maximumBytes)return undefined;
  const result=await read(file,status.size);
  if(result===undefined)return undefined;
  const{bytesRead}=await file.read(Buffer.allocUnsafe(1),0,1,null);
  return bytesRead===0?result:undefined;
 }finally{await file.close();}
}

async function readBoundedClosureLeafV1(path:string,maximumBytes:number):Promise<Uint8Array|undefined>{
 return withBoundedClosureLeafV1(path,maximumBytes,async(file,size)=>{
  const bytes=Buffer.allocUnsafe(size);
  let offset=0;
  while(offset<bytes.length){
   const{bytesRead}=await file.read(bytes,offset,bytes.length-offset,null);
   if(bytesRead===0)return undefined;
   offset+=bytesRead;
  }
  return bytes;
 });
}

async function verifyBoundedClosureLeafV1(path:string,key:string,signal?:AbortSignal):Promise<boolean>{
 const digest=await withBoundedClosureLeafV1(path,CLOSURE_POLICY_V1.maximumLeafBytes,async(file,size)=>{
  const hash=createHash("sha256"),chunk=Buffer.allocUnsafe(Math.min(size,64*1024));
  for(let remaining=size;remaining>0;){
   signal?.throwIfAborted();
   const{bytesRead}=await file.read(chunk,0,Math.min(remaining,chunk.length),null);
   if(bytesRead===0)return undefined;
   hash.update(chunk.subarray(0,bytesRead));
   remaining-=bytesRead;
  }
  return hash.digest("hex");
 });
 return digest===key;
}

/** 🗄️ The closure leaves of the graph store of a workspace (`<graph partition>/m/<namespace>`), read with the storage budget of the workspace; `undefined` stays out of the exchange when the store cannot be located. */
export function workspaceClosurePortV1(workspace:string):ClosurePortV1|undefined{
 try{
  const base=join(repositoryCacheBudgetV1(workspace).partitions.graph.directory,"m"),directory=(namespace:ClosureNamespaceV1)=>join(base,namespace);
  const path=(namespace:ClosureNamespaceV1,key:string)=>join(directory(namespace),key+".json");
  return{
   directory,
   size:(namespace,key)=>{try{const row=statSync(path(namespace,key));return row.isFile()?row.size:undefined;}catch{return undefined;}},
   read:async(namespace,key,maximumBytes)=>{try{return maximumBytes===undefined?readFileSync(path(namespace,key)):await readBoundedClosureLeafV1(path(namespace,key),maximumBytes);}catch{return undefined;}},
   verify:async(namespace,key,signal)=>{
    signal?.throwIfAborted();
    if((namespace!=="build-input-closures"&&namespace!=="build-lock-closures")||!/^[0-9a-f]{64}$/u.test(key))return undefined;
    try{
     return await verifyBoundedClosureLeafV1(path(namespace,key),key,signal);
    }catch(error){return (error as NodeJS.ErrnoException).code==="ENOENT"?false:undefined;}
    finally{signal?.throwIfAborted();}
   },
   put:closureLeafPutterV1(workspace)
  };
 }catch{return undefined;}
}

/** 🌐️ The exchange of this process: `undefined` while no remote cache endpoint is configured. A failed exchange pauses later requests for the configured retry interval, so a recovering cache remains usable in a long-lived process. */
export function remoteGenerationExchangeV1(input:Readonly<{environment:Readonly<Record<string,string|undefined>>;workspace:string;identity():Promise<string>;platform?:string;cursors?:GenerationCursorsV1;origins?:Readonly<{self:string;all:readonly string[]}>;serving?:boolean;closures?:ClosurePortV1;scope?:string;now?:()=>number}>):GenerationExchangeV1|undefined{
 let resolved;try{resolved=resolveRemoteCacheV1(input.environment,{root:input.workspace});}catch{return undefined;}
 if(!resolved)return undefined;
 const contract=resolved.contract,signedFetch=remoteCacheFetchV1(resolved.environment,{root:input.workspace});
 const platform=input.platform??`${process.platform}-${process.arch}`.toLowerCase(),token=resolved.token,make=(probe:boolean)=>httpRemoteExecutionContentV1({endpoint:contract.endpoint,token,fetch:signedFetch,timeoutSeconds:probe?policy.probeTimeoutSeconds:contract.timeoutSeconds,retries:probe?policy.probeRetries:contract.retries});
 const origins=input.origins??fleetOriginsV1(input.workspace),serving=input.serving??totalmem()/1048576>=policy.servingTotalMegabytes,scope=input.scope??policy.allScope,heads=(identity:string)=>({key:generationExchangeKeyV1(identity,platform,origins.self,scope),keys:origins.all.map(origin=>generationExchangeKeyV1(identity,platform,origin,scope)),origin:origins.self,scope});
 const closures=input.closures??workspaceClosurePortV1(input.workspace),now=input.now??Date.now;
 let retryAt=-Infinity;
 const waiting=()=>now()<retryAt;
 const guard=async(run:()=>Promise<GenerationExchangeOutcomeV1>,signal?:AbortSignal):Promise<GenerationExchangeOutcomeV1>=>{
  if(waiting())return{status:"failed",reason:"the remote cache retry is deferred"};
  try{return await run();}catch(error){if(!signal?.aborted&&!(error instanceof RestoreDocumentErrorV1))retryAt=now()+policy.failureRetryMilliseconds;return{status:"failed",reason:String((error as Error)?.message??error).slice(0,200)};}
 };
 const exchange:GenerationExchangeV1={
  upload:remoteCacheEffectiveUploadV1(contract),
  restore:({root,fs,signal,implementation,requireComplete,expectedHead})=>guard(async()=>{const identity=await input.identity();return restoreGenerationV1({content:make(false),instance:contract.instance,...heads(identity),identity,root,workspace:input.workspace,platform,files:fs,signal,implementation,cursors:input.cursors,closures,requireComplete,expectedHead});},signal),
  look:async({root,fs,signal,implementation,requireComplete})=>{if(waiting())return{ahead:false,held:0,state:"none"};try{const identity=await input.identity();return await remoteGenerationLookV1({content:make(true),instance:contract.instance,...heads(identity),identity,root,workspace:input.workspace,platform,files:fs,signal,implementation,cursors:input.cursors,requireComplete});}catch{return{ahead:false,held:0,state:"none"};}},
  ahead:async({root,fs,signal,implementation})=>{if(waiting())return false;try{const identity=await input.identity();return await remoteGenerationAheadV1({content:make(true),instance:contract.instance,...heads(identity),identity,root,workspace:input.workspace,platform,files:fs,signal,implementation,cursors:input.cursors});}catch{return false;}},
 publish:({root,fs,signal,ifAbsent,stale})=>guard(async()=>publishGenerationV1({content:make(ifAbsent===true),instance:contract.instance,...heads(await input.identity()),root,workspace:input.workspace,platform,files:fs,signal,ifAbsent,stale,serving,cursors:input.cursors,closures}),signal),
  leaf:async({namespace,key,signal})=>{
   if(waiting())return undefined;
   try{const data=await readRemoteClosureLeafV1({content:make(true),instance:contract.instance,signal},key);return data&&data.length>0&&sha(data)===key?data:undefined;}catch{return undefined;}
  },
 };
 return Object.freeze(exchange);
}
