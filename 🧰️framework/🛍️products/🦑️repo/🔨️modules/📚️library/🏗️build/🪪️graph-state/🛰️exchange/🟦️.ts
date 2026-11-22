import {createHash} from "node:crypto";
import {brotliCompressSync,brotliDecompressSync,constants} from "node:zlib";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {REAPI_V1,decodeProtobufV1,encodeProtobufV1} from "../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {createGraphFleetClientV1,type GraphFleetClientV1,type GraphFleetSettingsV1,type GraphLookupFaultV1,type GraphPutV1,type GraphPutTimeoutV1} from "./📡️client/🟦️.ts";
import {graphRefusalOfV1,type GraphRefusalV1} from "./🧯️refusal/🟦️.ts";
export type {GraphFleetClientV1,GraphFleetSettingsV1,GraphLookupFaultV1,GraphPutV1,GraphPutTimeoutV1,GraphRefusalV1};
export {graphRefusalOfV1};
import {slotIdV1} from "../../🪢️slots/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {GRAPH_STATE_POLICY_V1 as policy,graphStateSlotParts,type GraphStateV1} from "../🟦️.ts";


const sha256=(bytes:Uint8Array|string):string=>createHash("sha256").update(bytes).digest("hex");

function sleep(milliseconds:number,signal?:AbortSignal):Promise<void>{
 if(signal?.aborted)return Promise.resolve();
 return new Promise(done=>{
  const finish=()=>{clearTimeout(timer);signal?.removeEventListener("abort",finish);done();};
  const timer=setTimeout(finish,milliseconds);
  signal?.addEventListener("abort",finish,{once:true});
 });
}

/** 📄️ One document of an entry, in its neutral form: the bytes with the machine root replaced by the root token. */
export type GraphDocumentV1=Readonly<{name:string;bytes:Uint8Array}>;
/** 🌊️ A document of an entry that was not downloaded with it: `read` downloads its blobs (verified against their names by the client) and decompresses them when asked, `keep` leaves the compressed blobs in memory (within a small budget per entry) so that the next `read` of the same document does not download them again. `hash` is the digest of the stored blob of a document that is one blob (the name the local stores know it by), `size` the compressed bytes. */
export type GraphDeferredV1=Readonly<{hash:string|undefined;size:number;read(options?:Readonly<{keep?:boolean}>):Promise<GraphDocumentV1|undefined>}>;
/** 🏷️ The inline description of an entry: see the schema definition `Note`. */
export type GraphEntryNoteV1=Readonly<{version:1;kind:GraphStateV1["frame"]["kind"];digest:string;platform:string;producer:string;createdMs:number;documents:readonly Readonly<{name:string;hash:string;size:number}>[];leaves?:readonly GraphLeafV1[]}>;
/** 🍃️ A content addressed piece an entry refers to by name, stored in the blob store of the cache under its own digest before the entry is: see the schema definition `Note`. */
export type GraphLeafV1=Readonly<{namespace:"build-input-closures"|"build-lock-closures";hash:string;size:number}>;
/** 🍃️ A leaf with the way to read its bytes on the producing machine. */
export type GraphLeafSourceV1=GraphLeafV1&Readonly<{read():Promise<Uint8Array|undefined>}>;
const LEAF_FOLDER_V1={"build-input-closures":"i","build-lock-closures":"l"} as const,storedLeaves=new Map<string,Set<string>>();
/** 🍃️ The path under which the action result of an entry references a leaf, so the retention of the cache keeps the blob as long as the entry. */
export const graphLeafPathV1=(leaf:GraphLeafV1):string=>`leaf/${LEAF_FOLDER_V1[leaf.namespace]}/${leaf.hash}.json`;
/** 🔔️ A lifecycle message the caller may print (keys of the text catalogue). */
export type GraphSayV1=(key:keyof typeof schema.definitions.TextCatalogue.const,fields:Readonly<Record<string,string|number>>)=>void;

export {expandRootV1,neutralizeRootV1} from "../../🕸️graph/🕰️generations/🌐️remote-exchange/🧳️portable/🟦️.ts";

/** 🗝️ The action key an entry is stored under: the state digest in the keyspace of the kind. */
export function graphEntryKeyV1(state:GraphStateV1):string{return sha256(JSON.stringify([policy.domain,"entry",state.frame.kind,state.digest]));}

/** 🪢️ The retention slot of an entry: a newer state of the same kind, platform and tooling supersedes older ones. */
export function graphEntrySlotV1(state:GraphStateV1):string{return slotIdV1(policy.slotDomain as never,graphStateSlotParts(state.frame));}

/** 🗜️ Compresses a neutral document into the blobs the cache stores (one per 32 MiB of compressed output), named `<name>.br` or `<name>.br.<n>`. */
export function packGraphDocumentV1(document:GraphDocumentV1):readonly GraphDocumentV1[]{
 const packed=brotliCompressSync(document.bytes,{params:{[constants.BROTLI_PARAM_QUALITY]:5,[constants.BROTLI_PARAM_LGWIN]:22,[constants.BROTLI_PARAM_SIZE_HINT]:document.bytes.length}}),part=policy.blobMaximumBytes;
 if(packed.length<=part)return [{name:document.name+".br",bytes:packed}];
 const parts:GraphDocumentV1[]=[];
 for(let from=0,index=0;from<packed.length;from+=part,index++)parts.push({name:`${document.name}.br.${index}`,bytes:packed.subarray(from,Math.min(packed.length,from+part))});
 return parts;
}

/** 📂️ The reverse of {@link packGraphDocumentV1} for the blobs of one entry: joins the parts of every document and decompresses it. */
export function unpackGraphDocumentsV1(blobs:readonly GraphDocumentV1[]):readonly GraphDocumentV1[]{
 const groups=new Map<string,{index:number;bytes:Uint8Array}[]>();
 for(const blob of blobs){
  const match=/^(.*)\.br(?:\.(\d+))?$/u.exec(blob.name);
  if(!match)throw Error("Compressed graph document expected: "+blob.name);
  const rows=groups.get(match[1]!)??[];rows.push({index:Number(match[2]??0),bytes:blob.bytes});groups.set(match[1]!,rows);
 }
 return [...groups].map(([name,rows])=>({name,bytes:new Uint8Array(brotliDecompressSync(Buffer.concat(rows.sort((a,b)=>a.index-b.index).map(row=>row.bytes)),{maxOutputLength:policy.documentMaximumBytes}))}));
}

/** 📡️ The client of the graph exchange; it keeps the unreachable-or-denied breaker next to the cache so a dead server costs one short probe. */
export function graphFleetClientV1(settings:GraphFleetSettingsV1,world?:Readonly<{fetch?:typeof fetch}>):GraphFleetClientV1{return createGraphFleetClientV1(settings,world);}

/** 🎫️ A claim this machine holds: renewed on a timer until released. */
export type GraphHoldV1=Readonly<{release():Promise<void>;stop():void;lost():boolean}>;

/** 🧭️ What the fleet decided for one state: an entry to install, or the permission (hold) or the need (no hold) to compute it here. */
export type GraphDecisionV1=Readonly<{outcome:"hit";via:"cache"|"wait";documents:readonly GraphDocumentV1[];deferred:ReadonlyMap<string,GraphDeferredV1>;note:GraphEntryNoteV1;waitedMs:number}>|Readonly<{outcome:"compute";hold:GraphHoldV1|undefined;reason:string;waitedMs:number}>;

/** ⏭️ The answer of a decision that leaves an owner to a live peer: this reader has other owners to infer, so it goes on with them and comes back to this one later. */
export type GraphSkipV1=Readonly<{outcome:"skip";by:string;reason:"busy";waitedMs:number}>;

const firstHeld=new Map<string,number>(),waiting=new Map<string,Readonly<{kind:string;digest:string;by:string;sinceMs:number}>>();

/** ⏳️ The owners this process waits for right now: the kind and digest of the entry, the machine that holds its claim and since when it waits; what a reader that makes no progress names as its reason. */
export function graphFleetWaitingV1():readonly Readonly<{kind:string;digest:string;by:string;sinceMs:number}>[]{return[...waiting.values()];}

/** ⚖️ What a reader does about an owner whose claim another machine holds: `lapsed` (the lease ended: claim it again, nobody is working on it), `compute` (a large machine never waits for a smaller one, and an owner held for the lease time by a holder that keeps renewing is taken over: the oldest-held first), `skip` (a reader with other owners to infer goes on with them) or `wait` (nothing else to infer, the lease is live and younger than the lease time).
 * @see ../🧪️tests/⏳️lease-wait/🟦️.ts */
export type GraphHeldVerdictV1="lapsed"|"compute"|"skip"|"wait";
export function graphHeldVerdictV1(input:Readonly<{untilMilliseconds:number;now:number;sinceMilliseconds:number;busy:boolean;strong:boolean}>):GraphHeldVerdictV1{
 if(!(input.untilMilliseconds>input.now))return "lapsed";
 if(input.strong||input.now-input.sinceMilliseconds>=policy.leaseTakeOverMilliseconds)return "compute";
 return input.busy?"skip":"wait";
}

function holdOf(client:GraphFleetClientV1,key:string,owner:string,ttl:number=policy.claimTtlMilliseconds):GraphHoldV1{
 let stopped=false,gone=false;
 const timer=setInterval(()=>{void client.renew(key,owner,ttl).then(state=>{if(state==="lost"){gone=true;stopped=true;clearInterval(timer);}});},policy.claimHeartbeatMilliseconds);timer.unref?.();
 const stop=()=>{if(!stopped){stopped=true;clearInterval(timer);}};
 return Object.freeze({stop,lost:()=>gone,async release(){stop();if(!gone)await Promise.race([client.release(key,owner),sleep(600)]);}});
}

function noteOf(bytes:Uint8Array):{note:GraphEntryNoteV1;entries:readonly {path:string;hash:string;size:number}[]}|undefined{
 try{
  const result=decodeProtobufV1(REAPI_V1.actionResult,bytes),files=(result.output_files??[]) as {path:string;digest:{hash:string;size_bytes:number|string}}[],text=Buffer.from(result.stdout_raw as Uint8Array).toString("utf8"),note=JSON.parse(text) as GraphEntryNoteV1;
  if(validateJsonSchemaSubset({$ref:"#/definitions/Note"},note,schema).length)return undefined;
  const entries=files.filter(file=>!file.path.startsWith("leaf/")).map(file=>({path:file.path,hash:file.digest.hash,size:Number(file.digest.size_bytes)}));
  return {note,entries};
 }catch{return undefined;}
}

async function each<T>(items:readonly T[],limit:number,visit:(item:T)=>Promise<void>):Promise<void>{
 let next=0;await Promise.all(Array.from({length:Math.min(limit,items.length)},async()=>{while(next<items.length)await visit(items[next++]!);}));
}

/** 🧷️ Whether the caller already holds the blob of an entry document and does not want it downloaded: it is asked once per blob with the stored name (`<document>.br`), the digest and the size, and a blob it holds is not fetched and its document is not returned. */
export type GraphHeldV1=(entry:Readonly<{path:string;hash:string;size:number}>)=>boolean|Promise<boolean>;

/** ⚖️ Which limit the documents of an entry break, or undefined when they are within the declared limits: no document above `documentMaximumBytes` and no entry above `entryMaximumBytes`, counted before compression. */
export function graphDocumentLimitV1(documents:readonly GraphDocumentV1[]):Readonly<{code:"limit-document"|"limit-entry";text:string}>|undefined{
 const mebibytes=(bytes:number)=>Math.ceil(bytes/1048576),largest=documents.reduce((most,document)=>document.bytes.length>most.bytes.length?document:most,{name:"",bytes:new Uint8Array(0)}),total=documents.reduce((sum,document)=>sum+document.bytes.length,0);
 if(largest.bytes.length>policy.documentMaximumBytes)return{code:"limit-document",text:`the document ${largest.name} is ${mebibytes(largest.bytes.length)} MiB, above the limit of ${mebibytes(policy.documentMaximumBytes)} MiB / das Dokument ${largest.name} hat ${mebibytes(largest.bytes.length)} MiB, mehr als die Grenze von ${mebibytes(policy.documentMaximumBytes)} MiB`};
 if(total>policy.entryMaximumBytes)return{code:"limit-entry",text:`the entry is ${mebibytes(total)} MiB in ${documents.length} documents, above the limit of ${mebibytes(policy.entryMaximumBytes)} MiB / der Eintrag hat ${mebibytes(total)} MiB in ${documents.length} Dokumenten, mehr als die Grenze von ${mebibytes(policy.entryMaximumBytes)} MiB`};
 return undefined;
}

/** ⚖️ Why the documents of an entry cannot be stored, in both languages, or undefined when they are within the declared limits (see {@link graphDocumentLimitV1}). */
export function graphDocumentProblemV1(documents:readonly GraphDocumentV1[]):string|undefined{return graphDocumentLimitV1(documents)?.text;}

const DEFERRED_KEEP_BYTES_V1=32<<20;
/** ⬇️ Looks the entry of a state up and downloads its documents, except the blobs `held` says the caller has and the documents `defer` names: those are not downloaded and come back in `deferred`, to be read one at a time, so a caller that unpacks many large documents never holds more than it reads. `undefined` is a miss; `"fault"` is an unreachable, denied or damaged entry (the caller computes). Every blob is verified against its name before it counts. */
export async function fetchGraphEntryV1(client:GraphFleetClientV1,state:GraphStateV1,held?:GraphHeldV1,defer?:(name:string)=>boolean,signal?:AbortSignal,onFault?:(reason:GraphLookupFaultV1)=>void,onResultDigest?:(digest:string)=>void):Promise<{note:GraphEntryNoteV1;documents:readonly GraphDocumentV1[];deferred:ReadonlyMap<string,GraphDeferredV1>}|undefined|"fault"> {
 signal?.throwIfAborted();
 const fault=(reason:GraphLookupFaultV1)=>{onFault?.(reason);return "fault" as const;};
 if(!client.usable())return fault("unavailable");
 const answer=await client.getResult(graphEntryKeyV1(state),signal);
 signal?.throwIfAborted();
 if(answer==="miss")return undefined;
 if(typeof answer==="string")return fault(answer==="denied"?"denied":answer==="down"?"transport":"unavailable");
 const described=noteOf(answer);
 if(!described||described.note.digest!==state.digest||described.note.kind!==state.frame.kind||described.entries.length>policy.entryMaximumDocuments)return fault("record");
 const blobs:GraphDocumentV1[]=[],lazy=new Map<string,{path:string;hash:string;size:number;index:number}[]>(),eager:{path:string;hash:string;size:number}[]=[];let broken:GraphLookupFaultV1|undefined;
 for(const entry of described.entries){
  signal?.throwIfAborted();
  const named=defer===undefined?null:/^(.*)\.br(?:\.(\d+))?$/u.exec(entry.path);
  if(named===null||!defer!(named[1]!)){eager.push(entry);continue;}
  if(held!==undefined&&await held({path:entry.path,hash:entry.hash,size:entry.size}))continue;
  const rows=lazy.get(named[1]!)??[];rows.push({...entry,index:Number(named[2]??0)});lazy.set(named[1]!,rows);
 }
 await each(eager,4,async entry=>{
  if(broken||signal?.aborted)return;
  try{
   if(held!==undefined&&await held({path:entry.path,hash:entry.hash,size:entry.size}))return;
   if(signal?.aborted)return;
   let reason:GraphLookupFaultV1|undefined;
   const bytes=await client.getBlob(entry.hash,entry.size,signal,onFault===undefined?undefined:kind=>{reason=kind;});
   if(bytes===undefined){broken??=reason??"unknown";return;}
   blobs.push({name:entry.path,bytes});
  }catch{broken??="unknown";}
 });
 signal?.throwIfAborted();
 if(broken!==undefined)return fault(broken);
 const deferred=new Map<string,GraphDeferredV1>();let keptBytes=0;
 for(const [name,rows] of lazy){
  rows.sort((left,right)=>left.index-right.index);
  const size=rows.reduce((sum,row)=>sum+row.size,0);let kept:Uint8Array[]|undefined;
  deferred.set(name,Object.freeze({hash:rows.length===1?rows[0]!.hash:undefined,size,async read(options){
   signal?.throwIfAborted();
   let parts=kept;
   if(parts!==undefined){kept=undefined;keptBytes-=size;}
   else{parts=[];for(const row of rows){const bytes=await client.getBlob(row.hash,row.size,signal);signal?.throwIfAborted();if(bytes===undefined)return undefined;parts.push(bytes);}}
   if(options?.keep===true&&keptBytes+size<=DEFERRED_KEEP_BYTES_V1){kept=parts;keptBytes+=size;}
   try{return unpackGraphDocumentsV1(parts.map((bytes,index)=>({name:rows.length===1?name+".br":`${name}.br.${index}`,bytes})))[0];}catch{return undefined;}
  }}));
 }
 try{
  const documents=unpackGraphDocumentsV1(blobs);
  if(onResultDigest!==undefined)onResultDigest(sha256(answer));
  return {note:described.note,documents,deferred};
 }catch{return fault("payload");}
}

/** 🧭️ What {@link decideGraphEntryV1} is given: `busy` says whether the caller has other owners to infer (then an owner held by a live peer is skipped, never waited for), `strong` that the caller is a large machine (it never waits for a smaller one). */
export type GraphDecideInputV1=Readonly<{client:GraphFleetClientV1;state:GraphStateV1;owner:string;signal?:AbortSignal;say?:GraphSayV1;expectedMilliseconds?:number;waitLimitMilliseconds?:number;skip?:GraphHeldV1;defer?:(name:string)=>boolean;now?:()=>number;busy?:()=>boolean;strong?:boolean;onLookupFault?:(reason:GraphLookupFaultV1)=>void}>;

/** 🧭️ The protocol for one state: look it up; on a miss claim it; when another machine holds a live claim and this reader has nothing else to infer wait for its upload (at most the lease time since the owner was first seen held) and fetch it; when the holder lapses, or the lease time has passed, take over; a busy reader skips the owner and a large machine computes it at once. Anything unreachable, denied or too slow ends in `compute` without a hold, because correctness never depends on the fleet.
 * @see ../🧪️tests/⏳️lease-wait/🟦️.ts */
export async function decideGraphEntryV1(input:GraphDecideInputV1&Readonly<{busy:()=>boolean}>):Promise<GraphDecisionV1|GraphSkipV1>;
export async function decideGraphEntryV1(input:GraphDecideInputV1):Promise<GraphDecisionV1>;
export async function decideGraphEntryV1(input:GraphDecideInputV1):Promise<GraphDecisionV1|GraphSkipV1>{
 const {client,state,owner,say}=input,now=input.now??Date.now,started=now(),key=graphEntryKeyV1(state),compute=(reason:string,hold?:GraphHoldV1):GraphDecisionV1=>{firstHeld.delete(key);return{outcome:"compute",hold,reason,waitedMs:now()-started};},label=state.digest.slice(0,12);
 const hit=(found:{note:GraphEntryNoteV1;documents:readonly GraphDocumentV1[];deferred:ReadonlyMap<string,GraphDeferredV1>},via:"cache"|"wait"):GraphDecisionV1=>{firstHeld.delete(key);say?.("hit",{kind:state.frame.kind,digest:label,files:found.documents.length+found.deferred.size,megabytes:Math.round((found.documents.reduce((sum,document)=>sum+document.bytes.length,0)+[...found.deferred.values()].reduce((sum,each)=>sum+each.size,0))/1048576),seconds:Math.round((now()-started)/1000)});return {outcome:"hit",via,documents:found.documents,deferred:found.deferred,note:found.note,waitedMs:now()-started};};
 const read=()=>fetchGraphEntryV1(client,state,input.skip,input.defer,input.signal,input.onLookupFault);
 try{
  const first=await read();
  if(first==="fault"){say?.("skipped",{kind:state.frame.kind,reason:"fault"});return compute("fault");}
  if(first)return hit(first,"cache");
  if(!client.claimsUsable())return compute("no-claims");
  let deadline=Infinity;
  for(let round=0;round<policy.claimRoundsMaximum;round++){
   input.signal?.throwIfAborted();
   const claim=await client.claim(key,owner,policy.claimTtlMilliseconds,input.expectedMilliseconds);
   if(claim.state==="granted"){
    const hold=holdOf(client,key,owner);
    if(input.signal?.aborted){await hold.release().catch(()=>undefined);input.signal.throwIfAborted();}
    return compute("granted",hold);
   }
   input.signal?.throwIfAborted();
   if(claim.state==="stored"){const again=await read();if(again&&again!=="fault")return hit(again,"wait");return compute("stored-unreadable");}
   if(claim.state!=="held")return compute(claim.state==="full"?"claims-full":"claims-"+claim.fault);
   const at=now(),since=firstHeld.get(key)??at;firstHeld.set(key,since);
   const verdict=graphHeldVerdictV1({untilMilliseconds:claim.untilMilliseconds,now:at,sinceMilliseconds:since,busy:input.busy?.()===true,strong:input.strong===true});
   if(verdict==="skip"){await sleep(100,input.signal);return{outcome:"skip",by:claim.by,reason:"busy",waitedMs:now()-started};}
   if(verdict==="compute")return compute(input.strong===true?"strong":"holder-slow");
   if(verdict==="lapsed"){await sleep(250,input.signal);continue;}
   waiting.set(key,{kind:state.frame.kind,digest:label,by:claim.by,sinceMs:since});
   if(deadline===Infinity){deadline=since+Math.min(policy.claimWaitMaximumMilliseconds,input.waitLimitMilliseconds??policy.claimWaitMaximumMilliseconds,policy.leaseTakeOverMilliseconds);say?.("waited",{kind:state.frame.kind,digest:label,seconds:0});}
   const remaining=deadline-now();
   if(remaining<=0)return compute("holder-slow");
   const waited=await client.await(key,remaining,input.signal);
   if(waited==="fault")return compute("await-fault");
   if(waited==="stored"){const again=await read();if(again&&again!=="fault")return hit(again,"wait");if(again==="fault")return compute("fault");}
  }
  return compute("rounds");
 }catch{return compute("error");}finally{waiting.delete(key);}
}

/** 🎯️ The answer of one look at a state that never waits: the entry is stored (nothing to compute), this machine holds the claim now, another machine holds it (look again later) or the fleet cannot say (the caller leaves the state alone). */
export type GraphTryV1=Readonly<{outcome:"stored"}>|Readonly<{outcome:"granted";hold:GraphHoldV1}>|Readonly<{outcome:"held";by:string;untilMilliseconds:number}>|Readonly<{outcome:"fault";reason:string}>;

/** 🎯️ Claims the computation of a state for a machine that pulls work: one request, no waiting. A claim answers `stored` for an entry that exists, so a pulled state is never claimed twice and never computed after it was published; a claim whose holder lapsed is granted to the next machine that asks. Anything unreachable or denied is a `fault`, never a grant. */
export async function tryGraphEntryV1(input:Readonly<{client:GraphFleetClientV1;state:GraphStateV1;owner:string;expectedMilliseconds?:number;ttlMilliseconds?:number}>):Promise<GraphTryV1>{
 const {client,state,owner}=input;
 try{
  if(!client.usable())return{outcome:"fault",reason:"down"};
  if(!client.claimsUsable())return{outcome:"fault",reason:"no-claims"};
  const ttl=input.ttlMilliseconds??policy.claimTtlMilliseconds,claim=await client.claim(graphEntryKeyV1(state),owner,ttl,input.expectedMilliseconds);
  if(claim.state==="granted")return{outcome:"granted",hold:holdOf(client,graphEntryKeyV1(state),owner,ttl)};
  if(claim.state==="stored")return{outcome:"stored"};
  if(claim.state==="held")return{outcome:"held",by:claim.by,untilMilliseconds:claim.untilMilliseconds};
  return{outcome:"fault",reason:claim.state==="full"?"claims-full":"claims-"+claim.fault};
 }catch{return{outcome:"fault",reason:"error"};}
}

/** 🔁️ Replaces a mutable head by compare-and-swap: the head is read, the new result is written with the digest of what was read (or `none`), the cache accepts it only while the held bytes still have that digest (whoever wrote them), and a lost race is read again and retried up to four times: the newest writer wins and nobody is refused as divergent. */
async function swapHeadV1(client:GraphFleetClientV1,key:string,result:Uint8Array,slot:string):Promise<GraphPutV1>{
 let last:GraphPutV1={stored:false,refusal:graphRefusalOfV1(409,"the head moved")};
 for(let attempt=0;attempt<4;attempt++){
  const current=await client.getResult(key),expect=current==="miss"?"none":typeof current==="string"?undefined:sha256(current);
  last=await client.putResult(key,result,slot,expect);
  if(last.stored||expect===undefined)return last;
 }
 return last;
}

/** 🧾️ How a publication ended: the entry is stored, or the refusal with its true reason. */
export type GraphPublishV1=GraphPutV1;

const refused=(answer:Parameters<typeof graphRefusalOfV1>[0],detail?:string):GraphPublishV1=>{const found=graphRefusalOfV1(answer,detail);return{stored:false,refusal:detail===undefined?found:{...found,detail}};};

/** ⬆️ Uploads the documents of a computed state and the action result that names them (blobs first, so a reader never sees a result without its blobs), then drops the claim unless `keepHold` says the caller will try again with the same claim. Returns whether the entry is stored or why it is not: the answer of the cache (status and first words), no answer in time, an open breaker, a lost claim, or a limit of the client. A machine without upload rights, an unreachable server or an oversized entry never throws. */
export async function publishGraphEntryOutcomeV1(input:Readonly<{client:GraphFleetClientV1;state:GraphStateV1;owner:string;documents:readonly GraphDocumentV1[];hold?:GraphHoldV1;keepHold?:boolean;head?:boolean;expectedHead?:string;say?:GraphSayV1;producer?:string;now?:()=>number;leaves?:readonly GraphLeafSourceV1[];leafWidth?:number}>):Promise<GraphPublishV1>{
 const {client,state,hold,say}=input;
 try{
  if(!client.settings.upload)return refused("no-upload");
  if(!client.usable())return refused("breaker");
  if(hold?.lost()===true)return refused("claim-lost");
  const limit=graphDocumentLimitV1(input.documents);
  if(limit!==undefined)return refused(limit.code,limit.text);
  const blobs=input.documents.flatMap(document=>packGraphDocumentV1(document)).map(blob=>({name:blob.name,bytes:blob.bytes,hash:sha256(blob.bytes)}));
  if(blobs.length>policy.entryMaximumDocuments)return refused("limit-documents",`${blobs.length} blobs, above ${policy.entryMaximumDocuments} / ${blobs.length} Blobs, mehr als ${policy.entryMaximumDocuments}`);
  let failure:GraphPutV1|undefined;
  await each(blobs,3,async blob=>{if(failure!==undefined)return;if(await client.hasBlob(blob.hash)===true)return;const put=await client.putBlob(blob.hash,blob.bytes);if(!put.stored)failure=put;});
  if(failure!==undefined)return failure;
  const leaves=[...new Map((input.leaves??[]).map(leaf=>[leaf.hash,leaf] as const)).values()].sort((left,right)=>left.hash<right.hash?-1:1),known=storedLeaves.get(client.settings.endpoint+"\0"+client.settings.instance)??storedLeaves.set(client.settings.endpoint+"\0"+client.settings.instance,new Set<string>()).get(client.settings.endpoint+"\0"+client.settings.instance)!;
  let lost:GraphLeafSourceV1|undefined;
  await each(leaves,input.leafWidth??4,async leaf=>{
   if(failure!==undefined||lost!==undefined||input.head!==true&&known.has(leaf.hash))return;
   if(await client.hasBlob(leaf.hash)===true){known.add(leaf.hash);return;}
   const bytes=await leaf.read();
   if(bytes===undefined||bytes.length!==leaf.size||sha256(bytes)!==leaf.hash){lost=leaf;return;}
   const put=await client.putBlob(leaf.hash,bytes);
   if(!put.stored){failure=put;return;}
   known.add(leaf.hash);
  });
  if(failure!==undefined)return failure;
  if(lost!==undefined)return refused("leaf-missing",`${lost.namespace}/${lost.hash}`);
  const note:GraphEntryNoteV1={version:1,kind:state.frame.kind,digest:state.digest,platform:state.frame.platform,producer:(input.producer??input.owner).slice(0,160),createdMs:(input.now??Date.now)(),documents:blobs.map(blob=>({name:blob.name,hash:blob.hash,size:blob.bytes.length})),...leaves.length===0?{}:{leaves:leaves.map(leaf=>({namespace:leaf.namespace,hash:leaf.hash,size:leaf.size}))}};
  const result=encodeProtobufV1(REAPI_V1.actionResult,{output_files:[...blobs.map(blob=>({path:blob.name,digest:{hash:blob.hash,size_bytes:blob.bytes.length}})),...leaves.map(leaf=>({path:graphLeafPathV1(leaf),digest:{hash:leaf.hash,size_bytes:leaf.size}}))],stdout_raw:Buffer.from(JSON.stringify(note))});
  const key=graphEntryKeyV1(state),slot=graphEntrySlotV1(state);
  const stored=input.head===true&&input.expectedHead===undefined?await swapHeadV1(client,key,result,slot):await client.putResult(key,result,slot,input.expectedHead);
  if(stored.stored)say?.("published",{kind:state.frame.kind,digest:state.digest.slice(0,12),files:blobs.length,megabytes:Math.round(blobs.reduce((sum,blob)=>sum+blob.bytes.length,0)/1048576)});
  return stored;
 }catch(error){return refused("network",String((error as Error)?.message??error).slice(0,160));}finally{if(input.keepHold!==true)await hold?.release().catch(()=>undefined);}
}

/** ⬆️ {@link publishGraphEntryOutcomeV1} for a caller that only needs to know whether the entry is stored. */
export async function publishGraphEntryV1(input:Parameters<typeof publishGraphEntryOutcomeV1>[0]):Promise<boolean>{return(await publishGraphEntryOutcomeV1(input)).stored;}
