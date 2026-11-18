import {createHash} from "node:crypto";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {REAPI_V1,decodeProtobufV1,encodeProtobufV1} from "../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {neutralizeRootV1,expandRootV1,packGraphDocumentV1,unpackGraphDocumentsV1,graphDocumentLimitV1,type GraphDocumentV1,type GraphFleetClientV1} from "../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import {neutralRowsV1,localizeRowsV1,portableRowsPolicyV1,type HostV1,type ObservationRowV1} from "../../../🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts";

/** 🏛️ The declared policy of the exchange of sealed selected generations: the semantics revision (part of every head key), the claim lease, the bounded wait and the limits.
 * @see ./🧬️schema/🔣️.json */
export const SELECTED_EXCHANGE_POLICY_V1=schema.definitions.Policy.const;
/** 🗣️ The English and German texts of the exchange, keyed by message.
 * @see ./🧬️schema/🔣️.json */
export const SELECTED_EXCHANGE_TEXTS_V1=schema.definitions.TextCatalogue.const;
const policy=SELECTED_EXCHANGE_POLICY_V1,token=portableRowsPolicyV1.rootToken,workspaceOwner="(workspace)",sha=(value:Uint8Array|string):string=>createHash("sha256").update(value).digest("hex");

type Say=(key:keyof typeof SELECTED_EXCHANGE_TEXTS_V1,fields:Readonly<Record<string,string|number>>)=>void;
/** 🧱️ One admission fact of one owner as the read remembers it: the owner, the key text of its question and the rows the answer consumed. */
export type SelectedAdmissionEntryV1=Readonly<{owner:string;suffix:string;facts:unknown;rows:readonly ObservationRowV1[]}>;
/** 🧾️ What a sealed read hands to the exchange: the graph text, the rows its owners were sealed for, the membership digest, the admission facts it consumed. */
export type SelectedSealedV1=Readonly<{graph:string;rows:readonly ObservationRowV1[];membership:string;admission:readonly SelectedAdmissionEntryV1[]}>;
/** 🏷️ The head a machine publishes: see the schema definition `Head`. */
export type SelectedHeadV1=Readonly<{version:1;revision:number;scope:string;platform:string;createdMs:number;membership:string;owners:Readonly<Record<string,string>>;documents:readonly Readonly<{name:string;hash:string;size:number}>[]}>;
/** 📥️ The generation of a head as the reader's machine installs it: the graph text at its own root, the rows of every owner in its own spelling. */
export type SelectedFetchedV1=Readonly<{head:SelectedHeadV1;graph:string;rowsByOwner:ReadonlyMap<string,readonly ObservationRowV1[]>;admission:readonly SelectedAdmissionEntryV1[]}>;

/** 🗝️ The key of the head of one scope: the domain, the semantics revision, the platform and the digest of the scope (selection and owner registry), never a path, a state root, a home directory or a host name. */
export function selectedHeadKeyV1(input:Readonly<{scope:string;platform:string;revision?:number}>):string{return sha(JSON.stringify([policy.domain,"head",input.revision??policy.revision,input.platform,input.scope]));}

/** 🪢️ The retention slot of a head: a head of a newer revision supersedes the older ones of the same scope and platform. */
export function selectedHeadSlotV1(input:Readonly<{scope:string;platform:string}>):string{return sha(JSON.stringify([policy.domain,"slot",input.platform,input.scope]));}

/** 🧭️ The owner of a neutral row: the path below the root token goes through `ownerOf`, every other row (a listing, a registry fact, a tool) belongs to the workspace. */
export function neutralRowOwnerV1(row:ObservationRowV1,ownerOf:(relative:string)=>string|undefined):string{
 const path=typeof row.path==="string"?row.path:"";
 return path.startsWith(token+"/")?ownerOf(path.slice(token.length+1))??workspaceOwner:workspaceOwner;
}

/** 🧮️ The rows of a read in the form every machine can replay (root token, no stamps, no location rows), grouped by the owner of the file they observed and sorted. */
export function selectedRowsByOwnerV1(rows:readonly ObservationRowV1[],host:HostV1,ownerOf:(relative:string)=>string|undefined):ReadonlyMap<string,readonly ObservationRowV1[]>{
 const groups=new Map<string,ObservationRowV1[]>();
 for(const row of neutralRowsV1(rows,host)){const owner=neutralRowOwnerV1(row,ownerOf),list=groups.get(owner);if(list)list.push(row);else groups.set(owner,[row]);}
 return new Map([...groups].sort(([left],[right])=>left<right?-1:left>right?1:0).map(([owner,list])=>[owner,list.sort((a,b)=>canonicalRow(a)<canonicalRow(b)?-1:canonicalRow(a)>canonicalRow(b)?1:0)]));
}
const canonicalRow=(row:ObservationRowV1):string=>JSON.stringify([row.kind,row.path,row.fact??null]);

/** 🔏️ The input digest of one owner: the SHA-256 of its sorted neutral rows. Two machines whose trees hold the same content for the files the owner was sealed on compute the same digest, whatever their roots, homes and file stamps. */
export function selectedOwnerDigestV1(rows:readonly ObservationRowV1[]):string{return sha(rows.map(canonicalRow).sort().join("\n"));}

/** 🔏️ The digests of every owner of a grouped read, by owner name. */
export function selectedOwnerDigestsV1(groups:ReadonlyMap<string,readonly ObservationRowV1[]>):Record<string,string>{return Object.fromEntries([...groups].map(([owner,rows])=>[owner,selectedOwnerDigestV1(rows)]));}

const encode=(value:unknown):Uint8Array=>Buffer.from(JSON.stringify(value));
const swapped=async(client:GraphFleetClientV1,key:string,result:Uint8Array,slot:string)=>{
 let last:Awaited<ReturnType<GraphFleetClientV1["putResult"]>>={stored:false} as never;
 for(let attempt=0;attempt<policy.headSwapAttempts;attempt++){
  const current=await client.getResult(key),expect=current==="miss"?"none":typeof current==="string"?undefined:sha(current);
  last=await client.putResult(key,result,slot,expect);
  if(last.stored||expect===undefined)return last;
 }
 return last;
};

/** 🏗️ The head and the documents of one sealed read for one machine: the root of the machine is replaced by the token everywhere, the rows are neutral, the owner digests are computed from them. Machines of different roots and homes produce the same bytes for the same content. */
export function buildSelectedGenerationV1(input:Readonly<{scope:string;platform:string;host:HostV1;sealed:SelectedSealedV1;ownerOf:(relative:string)=>string|undefined;now:number}>):Readonly<{head:SelectedHeadV1;documents:readonly GraphDocumentV1[]}>{
 const groups=selectedRowsByOwnerV1(input.sealed.rows,input.host,input.ownerOf),owners=selectedOwnerDigestsV1(groups);
 const move=(value:unknown)=>neutralizeRootV1(encode(value),input.host.root);
 const documents:GraphDocumentV1[]=[
  {name:"graph",bytes:neutralizeRootV1(Buffer.from(input.sealed.graph),input.host.root)},
  {name:"owners",bytes:encode({version:1,rows:Object.fromEntries(groups)})},
  {name:"admission",bytes:move({version:1,entries:input.sealed.admission.map(entry=>({owner:entry.owner,suffix:entry.suffix,facts:entry.facts,rows:neutralRowsV1(entry.rows,input.host)}))})}
 ];
 const blobs=documents.flatMap(document=>packGraphDocumentV1(document)).map(blob=>({name:blob.name,hash:sha(blob.bytes),size:blob.bytes.length}));
 return{head:{version:1,revision:policy.revision,scope:input.scope,platform:input.platform,createdMs:input.now,membership:input.sealed.membership,owners,documents:blobs},documents};
}

/** ⬆️ Offers a sealed generation to the fleet: the blobs first (those the cache lacks), the head last by compare-and-swap, so a reader never finds a head without its blobs and the newest writer wins. A head whose owner digests equal the held head is not written again. A machine without upload rights, an unreachable cache or an oversized generation never throws. */
export async function publishSelectedGenerationV1(input:Readonly<{client:GraphFleetClientV1;scope:string;platform:string;host:HostV1;sealed:SelectedSealedV1;ownerOf:(relative:string)=>string|undefined;now?:()=>number;held?:SelectedHeadV1}>):Promise<Readonly<{stored:boolean;head?:SelectedHeadV1;reason?:string}>>{
 const {client}=input;
 try{
  if(!client.settings.upload)return{stored:false,reason:"no-upload"};
  if(!client.usable())return{stored:false,reason:"breaker"};
  const built=buildSelectedGenerationV1({scope:input.scope,platform:input.platform,host:input.host,sealed:input.sealed,ownerOf:input.ownerOf,now:(input.now??Date.now)()});
  const same=input.held!==undefined&&JSON.stringify(input.held.owners)===JSON.stringify(built.head.owners)&&input.held.membership===built.head.membership;
  if(same)return{stored:false,head:input.held,reason:"current"};
  if(Object.keys(built.head.owners).length>policy.maximumOwners)return{stored:false,reason:"limit-owners"};
  const limit=graphDocumentLimitV1(built.documents);if(limit!==undefined)return{stored:false,reason:limit.code};
  for(const blob of built.documents.flatMap(document=>packGraphDocumentV1(document))){
   if(await client.hasBlob(sha(blob.bytes))===true)continue;
   const put=await client.putBlob(sha(blob.bytes),blob.bytes);if(!put.stored)return{stored:false,reason:"blob"};
  }
  const result=encodeProtobufV1(REAPI_V1.actionResult,{output_files:built.head.documents.map(row=>({path:row.name,digest:{hash:row.hash,size_bytes:row.size}})),stdout_raw:Buffer.from(JSON.stringify(built.head))});
  const stored=await swapped(client,selectedHeadKeyV1(input),result,selectedHeadSlotV1(input));
  return stored.stored?{stored:true,head:built.head}:{stored:false,reason:"head"};
 }catch(error){return{stored:false,reason:"network:"+String((error as Error)?.message??error).slice(0,120)};}
}

const headOf=(bytes:Uint8Array,scope:string,platform:string):{head:SelectedHeadV1;entries:readonly {path:string;hash:string;size:number}[]}|undefined=>{
 try{
  const result=decodeProtobufV1(REAPI_V1.actionResult,bytes),head=JSON.parse(Buffer.from(result.stdout_raw as Uint8Array).toString("utf8")) as SelectedHeadV1;
  if(validateJsonSchemaSubset({$ref:"#/definitions/Head"},head,schema).length)return undefined;
  if(head.revision!==policy.revision||head.scope!==scope||head.platform!==platform||Object.keys(head.owners).length>policy.maximumOwners)return undefined;
  const entries=((result.output_files??[]) as {path:string;digest:{hash:string;size_bytes:number|string}}[]).map(file=>({path:file.path,hash:file.digest.hash,size:Number(file.digest.size_bytes)}));
  if(JSON.stringify(entries.map(row=>[row.path,row.hash,row.size]))!==JSON.stringify(head.documents.map(row=>[row.name,row.hash,row.size])))return undefined;
  return{head,entries};
 }catch{return undefined;}
};

/** 🔭️ Looks the head of a scope up and fetches its documents, every blob verified against its name and every owner digest against its rows, and puts the generation in the spelling of this machine. `"miss"` is no head or a head of another revision, scope or platform; `"fault"` is an unreachable, denied or damaged head (the caller reads itself). */
export async function lookSelectedGenerationV1(input:Readonly<{client:GraphFleetClientV1;scope:string;platform:string;host:HostV1}>):Promise<SelectedFetchedV1|"miss"|"fault">{
 const {client}=input;
 try{
  if(!client.usable())return"fault";
  const answer=await client.getResult(selectedHeadKeyV1(input));
  if(answer==="miss")return"miss";
  if(typeof answer==="string")return"fault";
  const found=headOf(answer,input.scope,input.platform);if(found===undefined)return"miss";
  const blobs:GraphDocumentV1[]=[];
  for(const entry of found.entries){const bytes=await client.getBlob(entry.hash,entry.size);if(bytes===undefined||sha(bytes)!==entry.hash)return"fault";blobs.push({name:entry.path,bytes});}
  const documents=new Map(unpackGraphDocumentsV1(blobs).map(document=>[document.name,document.bytes])),own=(name:string)=>{const bytes=documents.get(name);if(bytes===undefined)throw Error("document missing: "+name);return expandRootV1(bytes,input.host.root);};
  const owners=JSON.parse(Buffer.from(documents.get("owners")!).toString("utf8")) as {rows:Record<string,ObservationRowV1[]>};
  for(const [owner,digest] of Object.entries(found.head.owners))if(selectedOwnerDigestV1(owners.rows[owner]??[])!==digest)return"fault";
  if(Object.keys(owners.rows).length!==Object.keys(found.head.owners).length)return"fault";
  const rowsByOwner=new Map(Object.entries(owners.rows).map(([owner,rows])=>[owner,localizeRowsV1(rows,input.host)]));
  const admission=(JSON.parse(Buffer.from(own("admission")).toString("utf8")) as {entries:{owner:string;suffix:string;facts:unknown;rows:ObservationRowV1[]}[]}).entries.map(entry=>({...entry,rows:localizeRowsV1(entry.rows,input.host)}));
  return{head:found.head,graph:Buffer.from(own("graph")).toString("utf8"),rowsByOwner,admission};
 }catch{return"fault";}
}

/** ⚖️ What a reader keeps of a head, owner by owner: `matching` owners hold in its tree (the digest equals the one of its own admission memo, or every row of the owner replays), `differing` owners must be read again, `added` owners are in the reader's closure but not in the head, `removed` owners are in the head only. Never rejects the whole generation for one differing owner. */
export async function acceptSelectedOwnersV1(input:Readonly<{fetched:SelectedFetchedV1;holds:(rows:readonly ObservationRowV1[])=>Promise<boolean>;own?:ReadonlyMap<string,string>;owners?:readonly string[]}>):Promise<Readonly<{matching:readonly string[];differing:readonly string[];added:readonly string[];removed:readonly string[]}>>{
 const matching:string[]=[],differing:string[]=[],published=input.fetched.head.owners;
 for(const owner of Object.keys(published).sort()){
  const known=input.own?.get(owner);
  const holds=known!==undefined?known===published[owner]:await input.holds(input.fetched.rowsByOwner.get(owner)??[]);
  (holds?matching:differing).push(owner);
 }
 const local=input.owners===undefined?undefined:new Set(input.owners);
 return{matching,differing,added:local===undefined?[]:[...local].filter(owner=>published[owner]===undefined).sort(),removed:local===undefined?[]:Object.keys(published).filter(owner=>!local.has(owner)&&owner!==workspaceOwner).sort()};
}

/** 🎫️ A claim this machine holds: renewed on a timer until released. */
function holdV1(client:GraphFleetClientV1,key:string,owner:string,ttl:number,beat:number,alive:()=>boolean=()=>true){
 let stopped=false,gone=false;
 const timer=setInterval(()=>{if(!alive())return;void client.renew(key,owner,ttl).then(state=>{if(state==="lost"){gone=true;stopped=true;clearInterval(timer);}},()=>undefined);},beat);timer.unref?.();
 return Object.freeze({lost:()=>gone,async release(){if(!stopped){stopped=true;clearInterval(timer);}if(!gone)await Promise.race([client.release(key,owner).catch(()=>undefined),new Promise<void>(done=>setTimeout(done,600))]);}});
}

/** 🪜️ Runs one await of the exchange as a named step of the read (the progress line names it with its bound), `detail` fills its text. */
export type SelectedStepV1=<T>(id:"exchange-look"|"exchange-wait"|"exchange-install"|"exchange-publish",run:()=>Promise<T>,detail?:Readonly<Record<string,string|number>>)=>Promise<T>;

/** 🏁️ How one selected read ended: `installed` (every owner held, nothing was read), `repaired` (some owners were read again), `read` (no head, this machine read and published), `waited` (another machine read, this one took its head), `taken-over` (the reader died, this machine read with the owner records it had published), `unreachable` (the fleet could not say, this machine read). */
export type SelectedOutcomeV1=Readonly<{outcome:"installed"|"repaired"|"read"|"waited"|"taken-over"|"unreachable";held:number;differing:readonly string[];waitedMilliseconds:number;published:boolean}>;

/** 🤝️ One selected read as the fleet shares it: look the head up; every owner whose rows hold is installed, the others (and a miss) are read by `read`, and the sealed result is published as the new head. Simultaneous starts: one machine claims the head and reads, the others wait with a named bound and progress, then take the head; when the claimant stops renewing its lease the next machine takes over. Every fleet fault ends in `read`: a broken cache never fails or hangs a read.
 * @see ./🧪️tests/🟦️.ts */
export async function selectedGenerationV1(input:Readonly<{
 client:GraphFleetClientV1;scope:string;platform:string;host:HostV1;owner:string;
 ownerOf:(relative:string)=>string|undefined;
 holds:(rows:readonly ObservationRowV1[])=>Promise<boolean>;
 own?:()=>ReadonlyMap<string,string>|undefined;owners?:()=>readonly string[]|undefined;
 install:(plan:Readonly<{fetched:SelectedFetchedV1;matching:readonly string[];differing:readonly string[];complete:boolean}>)=>Promise<void>;
 read:(differing:readonly string[]|undefined)=>Promise<SelectedSealedV1|undefined>;
 say?:Say;now?:()=>number;sleep?:(milliseconds:number)=>Promise<void>;signal?:AbortSignal;
 alive?:()=>boolean;step?:SelectedStepV1;
 claimTtlMilliseconds?:number;claimHeartbeatMilliseconds?:number;waitMaximumMilliseconds?:number;waitSliceMilliseconds?:number}>):Promise<SelectedOutcomeV1>{
 const {client}=input,now=input.now??Date.now,started=now(),key=selectedHeadKeyV1(input),ttl=input.claimTtlMilliseconds??policy.claimTtlMilliseconds,beat=input.claimHeartbeatMilliseconds??policy.claimHeartbeatMilliseconds,limit=input.waitMaximumMilliseconds??policy.waitMaximumMilliseconds,slice=input.waitSliceMilliseconds??policy.waitSliceMilliseconds;
 const say=input.say??(()=>undefined),step:SelectedStepV1=input.step??((_id,run)=>run()),alive=input.alive??(()=>true),seconds=()=>Math.round((now()-started)/1000);
 const finish=(outcome:SelectedOutcomeV1["outcome"],held:number,differing:readonly string[],published:boolean):SelectedOutcomeV1=>({outcome,held,differing,waitedMilliseconds:now()-started,published});
 const publish=async(sealed:SelectedSealedV1|undefined,held:SelectedHeadV1|undefined)=>{
  if(sealed===undefined)return false;
  const stored=await step("exchange-publish",()=>publishSelectedGenerationV1({client,scope:input.scope,platform:input.platform,host:input.host,sealed,ownerOf:input.ownerOf,now,held}));
  if(stored.stored)say("published",{owners:Object.keys(stored.head!.owners).length});
  return stored.stored;
 };
 const take=async(fetched:SelectedFetchedV1,through:"installed"|"waited"|"taken-over"):Promise<SelectedOutcomeV1>=>{
  const verdict=await acceptSelectedOwnersV1({fetched,holds:input.holds,own:input.own?.(),owners:input.owners?.()}),differing=[...verdict.differing,...verdict.added].sort(),complete=differing.length===0&&verdict.removed.length===0;
  await step("exchange-install",()=>input.install({fetched,matching:verdict.matching,differing,complete}));
  if(complete){say("installed",{owners:verdict.matching.length,seconds:seconds()});return finish(through==="installed"?"installed":through,verdict.matching.length,[],false);}
  say("repaired",{held:verdict.matching.length,differing:differing.length});
  const sealed=await input.read(differing),published=await publish(sealed,fetched.head);
  return finish("repaired",verdict.matching.length,differing,published);
 };
 const readHere=async(outcome:SelectedOutcomeV1["outcome"],held:SelectedHeadV1|undefined):Promise<SelectedOutcomeV1>=>{const sealed=await input.read(undefined),published=await publish(sealed,held);return finish(outcome,0,[],published);};
 const first=await step("exchange-look",()=>lookSelectedGenerationV1(input));
 if(first!=="miss"&&first!=="fault")return take(first,"installed");
 if(first==="fault"||!client.claimsUsable()||!client.settings.upload)return readHere("unreachable",undefined);
 let since:number|undefined,lapsed=false;
 for(let round=0;round<policy.maximumDocuments*8;round++){
  input.signal?.throwIfAborted();
  const claim=await client.claim(key,input.owner,ttl).catch(()=>({state:"fault",fault:"error"}) as never) as {state:string;by?:string;untilMilliseconds?:number};
  if(claim.state==="granted"){
   const hold=holdV1(client,key,input.owner,ttl,beat,alive);
   try{
    const again=await lookSelectedGenerationV1(input);
    if(again!=="miss"&&again!=="fault")return take(again,lapsed?"taken-over":"waited");
    if(lapsed)say("taken-over",{});
    return await readHere(lapsed?"taken-over":"read",undefined);
   }finally{await hold.release();}
  }
  if(claim.state==="stored"){const again=await lookSelectedGenerationV1(input);if(again!=="miss"&&again!=="fault")return take(again,"waited");return readHere("unreachable",undefined);}
  if(claim.state!=="held")return readHere("unreachable",undefined);
  since??=now();
  const remaining=since+limit-now();
  if(remaining<=0)return readHere("unreachable",undefined);
  say("waiting",{seconds:Math.round((now()-since)/1000),limit:Math.round(limit/1000),by:String(claim.by??"?")});
  const waited=await step("exchange-wait",()=>client.await(key,Math.min(slice,remaining)),{who:String(claim.by??"?"),boundMilliseconds:limit}).catch(()=>"fault" as const);
  if(waited==="fault")return readHere("unreachable",undefined);
  if(waited==="lapsed"){lapsed=true;if(input.sleep)await input.sleep(50);continue;}
  if(waited==="stored"){const again=await lookSelectedGenerationV1(input);if(again!=="miss"&&again!=="fault")return take(again,"waited");}
 }
 return readHere("unreachable",undefined);
}
