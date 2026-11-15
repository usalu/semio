import {createHash,randomBytes} from "node:crypto";
import {hostname} from "node:os";
import {mkdir,readdir,readFile,rename,rm,stat,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {fileSha256V1,stageKeyDigestV1,type StageCacheStatsV1,type StageCacheV1,type StageEntryV1,type StageFileV1,type StageKeyV1,type StageResultV1} from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {classifySlotsV1,slotIdV1} from "../../🏗️build/🪢️slots/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1,type FleetClientV1} from "../🦀️cargo/🗃️unit-cache/🛰️fleet/📡️client/🟦️.ts";
import {fleetRemoteEnvironmentV1} from "../🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";
import {decodeActionResultV1,encodeActionResultV1,wireDigestOfV1,type WireFileV1} from "../🦀️cargo/🗃️unit-cache/🛰️fleet/📦️wire/🟦️.ts";
import {obtainFleetEntryV1,type FlightHoldV1} from "./🎟️flight/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** ☁️ The authored limits, timeouts and names of the stage cache's fleet tier.
 * @see ./🧬️schema/🔣️.json */
export const STAGE_FLEET_POLICY_V1=schema.definitions.Policy.const;
const policy=STAGE_FLEET_POLICY_V1,SAFE=/^[^/\\\u0000]+(?:\/[^/\\\u0000]+)*$/u,HEX=/^[a-f0-9]{64}$/u;

/** 🔑️ The key a stage entry has at the fleet cache: SHA-256 of the namespace, the machine's platform and the stage key digest. Equal on every machine of one platform, whatever the checkout path. */
export function stageRemoteKeyV1(digest:string,platform:string=`${process.platform}-${process.arch}`):string{
 return createHash("sha256").update(JSON.stringify([policy.namespace,"stage",platform,digest])).digest("hex");
}

/** 🪢️ The slot of a stage request: the stage, its schema version and the module it builds, without any content input. */
export function stageSlotIdV1(stage:string,schemaVersion:number,slot:string):string{
 return slotIdV1("stage-output",[stage,String(schemaVersion),slot]);
}

/** 🗣️ Fills one text of the catalogue in one language. */
export function stageFleetTextV1(language:"en"|"de",key:keyof typeof schema.definitions.Messages.const,variables:Readonly<Record<string,string|number>>):string{
 return schema.definitions.Messages.const[key][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(variables[name]??""));
}

/** 🗣️ One event as a sentence in one language. */
export function stageFleetEventTextV1(language:"en"|"de",event:StageFleetEventV1):string{
 return stageFleetTextV1(language,event.kind,{stage:event.stage,slot:event.slot,files:event.files??0,bytes:event.bytes??0,seconds:event.seconds??0,reason:event.reason??""});
}

/** 📣️ What the sharing cache reports: a stage served by the fleet, computed after waiting for another machine, shared with the fleet, or computed here without it. */
export type StageFleetEventV1=Readonly<{kind:"served"|"waited"|"shared"|"skipped";stage:string;slot:string;files?:number;bytes?:number;seconds?:number;reason?:string}>;

/** 📊️ What the fleet tier did beside the local counters. */
export type StageFleetStatsV1=Readonly<{served:number;shared:number;waited:number;skipped:number}>;

/** ☁️ A stage cache with the fleet tier and slot supersession. */
export interface SharedStageCacheV1 extends StageCacheV1{fleet():StageFleetStatsV1;}

/** ⚙️ How the tier is wired: the client (none = local only), the platform in the keys, the claim owner, the clock, the reporter and policy overrides for tests. */
export type SharedStageOptionsV1=Readonly<{client?:FleetClientV1;platform?:string;owner?:string;now?:()=>number;report?:(event:StageFleetEventV1)=>void;claim?:Partial<typeof policy.claim>;graceMilliseconds?:number}>;

type Manifest=StageEntryV1;

/** 🏷️ The owner name a process claims under: machine, process and a random suffix, so a restarted process never inherits a dead one's claim. */
export const stageFleetOwnerV1=():string=>`${hostname().replace(/[^A-Za-z0-9._-]/gu,"-").slice(0,60)}:${process.pid}:${randomBytes(3).toString("hex")}`;

const order=(left:string,right:string)=>Buffer.compare(Buffer.from(left),Buffer.from(right));

async function filesOf(directory:string):Promise<StageFileV1[]>{
 const rows:StageFileV1[]=[];
 const walk=async(prefix:string):Promise<void>=>{
  for(const entry of await readdir(join(directory,prefix),{withFileTypes:true})){
   const path=prefix===""?entry.name:`${prefix}/${entry.name}`;
   if(entry.isDirectory())await walk(path);
   else if(entry.isFile())rows.push({path,sha256:await fileSha256V1(join(directory,path)),bytes:(await stat(join(directory,path))).size});
   else throw Error("Stage output must hold regular files only: "+path);
  }
 };
 await walk("");
 return rows.sort((left,right)=>order(left.path,right.path));
}

async function pool<T>(items:readonly T[],limit:number,work:(item:T)=>Promise<void>):Promise<void>{
 let next=0,failure:unknown;
 await Promise.all(Array.from({length:Math.min(limit,items.length)},async()=>{for(;;){if(failure!==undefined)return;const at=next++;if(at>=items.length)return;try{await work(items[at]!);}catch(error){failure??=error;return;}}}));
 if(failure!==undefined)throw failure;
}

function readManifest(bytes:Uint8Array,digest:string):Manifest{
 if(bytes.length>policy.limits.maximumManifestBytes)throw Error("manifest too large");
 const value=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes)) as Manifest;
 if(value.version!==1||value.digest!==digest||!Array.isArray(value.files)||value.files.length>policy.limits.maximumFiles)throw Error("manifest header invalid");
 if(stageKeyDigestV1(value.key)!==digest)throw Error("manifest names another key");
 const seen=new Set<string>();
 for(const file of value.files){
  if(typeof file.path!=="string"||!SAFE.test(file.path)||file.path.split("/").some((part:string)=>part==="."||part===".."||part===""))throw Error("file path unsafe");
  if(!HEX.test(file.sha256)||!Number.isSafeInteger(file.bytes)||file.bytes<0||file.bytes>policy.limits.maximumFileBytes||seen.has(file.path))throw Error("file row invalid");
  seen.add(file.path);
 }
 if(value.files.reduce((sum,file)=>sum+file.bytes,0)>policy.limits.maximumEntryBytes)throw Error("entry too large");
 return value;
}

async function fetchStage(client:FleetClientV1,remote:string,digest:string,directory:string,signal:AbortSignal):Promise<{state:"hit";files:readonly StageFileV1[]}|{state:"miss"|"fault"}>{
 const answer=await client.getResult(remote);
 if(answer==="miss")return {state:"miss"};
 if(typeof answer==="string")return {state:"fault"};
 try{
  const result=decodeActionResultV1(answer),head=result.files.find(file=>file.path===policy.manifestName);
  if(head===undefined)return {state:"miss"};
  const text=await client.getBlob(head.digest.hash,head.digest.size);
  if(text===undefined)return {state:"miss"};
  const manifest=readManifest(text,digest),listed=new Map(result.files.filter(file=>file!==head).map(file=>[file.path,file]));
  if(listed.size!==manifest.files.length||manifest.files.some(file=>{const row=listed.get(policy.outputPrefix+file.path);return row===undefined||row.digest.hash!==file.sha256||row.digest.size!==file.bytes;}))return {state:"miss"};
  await mkdir(directory,{recursive:true});
  await pool(manifest.files,policy.limits.downloadConcurrency,async file=>{
   signal.throwIfAborted();
   const bytes=await client.getBlob(file.sha256,file.bytes);
   if(bytes===undefined)throw Error("blob absent");
   const target=join(directory,...file.path.split("/"));await mkdir(dirname(target),{recursive:true});await writeFile(target,bytes);
  });
  return {state:"hit",files:manifest.files};
 }catch{
  await rm(directory,{recursive:true,force:true}).catch(()=>{});await mkdir(directory,{recursive:true}).catch(()=>{});
  return {state:"miss"};
 }
}

async function publishStage(client:FleetClientV1,remote:string,key:StageKeyV1,digest:string,slotId:string,directory:string,signal:AbortSignal):Promise<{files:number;bytes:number}|undefined>{
 const files=await filesOf(directory);
 if(files.length===0||files.length>policy.limits.maximumFiles||files.some(file=>file.bytes>policy.limits.maximumFileBytes))return undefined;
 const manifest:Manifest={version:1,key,digest,files},text=new TextEncoder().encode(`${JSON.stringify(manifest)}\n`),head=wireDigestOfV1(text);
 const wire:WireFileV1[]=[{path:policy.manifestName,digest:head,executable:false},...files.map(file=>({path:policy.outputPrefix+file.path,digest:{hash:file.sha256,size:file.bytes},executable:false}))];
 const blobs=new Map<string,string|Uint8Array>([[head.hash,text],...files.map(file=>[file.sha256,join(directory,...file.path.split("/"))] as const)]);
 await pool([...blobs],policy.limits.uploadConcurrency,async([hash,source])=>{
  signal.throwIfAborted();
  const present=await client.hasBlob(hash);
  if(present===undefined)throw Error("fleet cache unreachable");
  if(present)return;
  if(!await client.putBlob(hash,typeof source==="string"?new Uint8Array(await readFile(source)):source))throw Error("blob refused");
 });
 signal.throwIfAborted();
 const stored=await client.putResult(remote,encodeActionResultV1({files:wire,slot:slotId}),slotId);
 return stored===true?{files:files.length,bytes:files.reduce((sum,file)=>sum+file.bytes,0)}:undefined;
}

async function readIndex(path:string):Promise<{digest:string;storedAt:number}[]>{
 try{
  const value=JSON.parse(await readFile(path,"utf8")) as unknown;
  return validateJsonSchemaSubset(schema.definitions.SlotIndex,value,schema).length===0?(value as {rows:{digest:string;storedAt:number}[]}).rows:[];
 }catch{return [];}
}

/** 🧹️ Records that `digest` is what the synchronized worktree asks of `slot` now and reclaims the entries the slot superseded longer ago than the grace (machines a few minutes behind and builds in flight read them meanwhile). Entries without a slot, of other slots and of other stages are never touched.
 * @see ../../🏗️build/🪢️slots/🟦️.ts */
export async function settleStageSlotV1(root:string,input:Readonly<{stage:string;slotId:string;digest:string;now:number;graceMilliseconds:number}>):Promise<readonly string[]>{
 const path=join(root,".slots",input.stage,`${input.slotId}.json`),rows:{digest:string;storedAt:number}[]=[],known=await readIndex(path),newest=known.reduce<{digest:string;storedAt:number}|undefined>((best,row)=>best===undefined||row.storedAt>best.storedAt?row:best,undefined);
 for(const row of known)if(row.digest!==input.digest&&await stat(join(root,input.stage,row.digest)).then(()=>true,()=>false))rows.push(row);
 rows.push(newest?.digest===input.digest?newest:{digest:input.digest,storedAt:input.now});
 const verdicts=classifySlotsV1(rows.map(row=>({key:row.digest,slot:input.slotId,storedAt:row.storedAt,accessed:row.storedAt,size:0})),{now:input.now,graceMilliseconds:input.graceMilliseconds}),expired=rows.filter(row=>verdicts.get(row.digest)?.state==="expired").map(row=>row.digest);
 for(const digest of expired)await rm(join(root,input.stage,digest),{recursive:true,force:true});
 const kept=rows.filter(row=>!expired.includes(row.digest)).sort((left,right)=>right.storedAt-left.storedAt),temporary=`${path}.${randomBytes(4).toString("hex")}`;
 await mkdir(dirname(path),{recursive:true});await writeFile(temporary,JSON.stringify({version:1,rows:kept}));await rename(temporary,path);
 return expired;
}

/** ☁️ Wraps a local stage cache with slot supersession and, when a client is given, the fleet tier. A request without a slot is served by the local cache alone. A local miss looks the stage up at the fleet cache, waits for a machine that is computing it, or claims it, computes, and shares the files and the manifest (stored once by their digests) before releasing the claim. A local hit of an entry the fleet never saw is offered once. Every remote step is bounded and falls back to computing here, so a down, denying, stalling or corrupt cache never fails or hangs a stage.
 * @see ./🧬️schema/🔣️.json
 * @see ./🎟️flight/🟦️.ts */
export function sharedStageCacheV1(inner:StageCacheV1,options:SharedStageOptionsV1={}):SharedStageCacheV1{
 const client=options.client,now=options.now??Date.now,owner=options.owner??stageFleetOwnerV1(),grace=options.graceMilliseconds??policy.localGraceMilliseconds,counters={served:0,shared:0,waited:0,skipped:0};
 const report=(event:StageFleetEventV1)=>{try{options.report?.(event);}catch{}};
 async function run(key:StageKeyV1,output:string,produce:(directory:string)=>Promise<void>,signal?:AbortSignal,slot?:string):Promise<StageResultV1>{
  if(slot===undefined)return inner.run(key,output,produce,signal);
  const digest=stageKeyDigestV1(key),slotId=stageSlotIdV1(key.stage,key.schemaVersion,slot),remote=stageRemoteKeyV1(digest,options.platform),stopping=signal??new AbortController().signal;
  let served:readonly StageFileV1[]|undefined,shared:{files:number;bytes:number}|undefined,waitedMs=0,why:string|undefined;
  const wrapped=client===undefined?produce:async(directory:string):Promise<void>=>{
   if(!client.usable()){why=`breaker ${client.state()}`;return produce(directory);}
   const decision=await obtainFleetEntryV1({client,key:remote,owner,signal:stopping,...(options.claim===undefined?{}:{policy:options.claim}),expectedMilliseconds:policy.claim.expectedMilliseconds[key.stage as keyof typeof policy.claim.expectedMilliseconds]??policy.claim.defaultExpectedMilliseconds,fetch:async()=>{const found=await fetchStage(client,remote,digest,directory,AbortSignal.any([stopping,AbortSignal.timeout(policy.timeouts.fetchMilliseconds)]));if(found.state==="hit")served=found.files;return found.state;}});
   if(decision.outcome==="hit"){waitedMs=decision.via==="wait"?decision.waitedMs:0;return;}
   why=decision.reason;
   try{
    await produce(directory);
    if(client.settings.upload)try{shared=await publishStage(client,remote,key,digest,slotId,directory,AbortSignal.any([stopping,AbortSignal.timeout(policy.timeouts.publishMilliseconds)]));}catch{shared=undefined;}
   }finally{await (decision.hold as FlightHoldV1|undefined)?.release();}
  };
  const result=await inner.run(key,output,wrapped,signal);
  if(served!==undefined){
   counters.served++;
   if(waitedMs>0){counters.waited++;report({kind:"waited",stage:key.stage,slot,seconds:Math.round(waitedMs/1000)});}
   report({kind:"served",stage:key.stage,slot,files:served.length,bytes:served.reduce((sum,file)=>sum+file.bytes,0)});
   if(client!==undefined)await Promise.race([client.claimSlot(remote,slotId),new Promise<void>(done=>{const wait=setTimeout(done,policy.timeouts.releaseMilliseconds);wait.unref?.();})]).catch(()=>{});
  }else if(shared!==undefined){counters.shared++;report({kind:"shared",stage:key.stage,slot,files:shared.files,bytes:shared.bytes});}
  else if(client!==undefined&&why!==undefined&&!result.hit){counters.skipped++;report({kind:"skipped",stage:key.stage,slot,reason:why==="granted"?"not shared":why});}
  else if(client!==undefined&&result.hit&&client.settings.upload&&client.usable())await offer(client,remote,key,digest,slotId,output,stopping,inner.root);
  await settleStageSlotV1(inner.root,{stage:key.stage,slotId,digest,now:now(),graceMilliseconds:grace}).catch(()=>[]);
  return served===undefined?result:Object.freeze({...result,hit:true});
 }
 return {
  root:inner.root,
  run,
  stats():StageCacheStatsV1{const local=inner.stats();return Object.freeze({...local,hits:local.hits+counters.served,misses:Math.max(0,local.misses-counters.served)});},
  fleet:()=>Object.freeze({...counters}),
 };
 async function offer(shareClient:FleetClientV1,remote:string,key:StageKeyV1,digest:string,slotId:string,directory:string,stopping:AbortSignal,root:string):Promise<void>{
  const marker=join(root,".offered",remote);
  if(await stat(marker).then(()=>true,()=>false))return;
  try{
   const present=await shareClient.hasResult(remote);
   if(present===undefined)return;
   if(!present&&await publishStage(shareClient,remote,key,digest,slotId,directory,AbortSignal.any([stopping,AbortSignal.timeout(policy.timeouts.publishMilliseconds)]))===undefined)return;
   await mkdir(dirname(marker),{recursive:true});await writeFile(marker,"");
  }catch{}
 }
}

/** 📡️ The fleet cache client of a stage cache, read from the one resolver of the remote cache (the environment first, then the fleet facts of this machine's local copy, which is read and never refreshed, so a stage waits for no network to find out): `undefined` when no cache is named, when the unit cache's switch `SEMIO_UNIT_CACHE_REMOTE=off` or the stage cache's own `SEMIO_STAGE_CACHE_REMOTE=off` is set. The breaker state lives in `<stateRoot>/fl`. */
export function stageFleetClientV1(input:Readonly<{stateRoot:string;workspaceRoot?:string;environment:Readonly<Record<string,string|undefined>>}>):FleetClientV1|undefined{
 try{
  if(input.environment.SEMIO_STAGE_CACHE_REMOTE==="off")return undefined;
  const settings=readFleetSettingsV1(input.stateRoot,{...input.environment,...fleetRemoteEnvironmentV1(input.environment,input.workspaceRoot)});
  return settings===undefined?undefined:createFleetClientV1(settings);
 }catch{return undefined;}
}

/** 🎁️ The decoration a build driver applies to its generation's stage cache: slot supersession always, the fleet tier when a cache is named. */
export function stageFleetDecoratorV1(input:Readonly<{workspaceRoot?:string;environment:Readonly<Record<string,string|undefined>>;report?:(event:StageFleetEventV1)=>void}>):(cache:StageCacheV1)=>StageCacheV1{
 return cache=>{
  const client=stageFleetClientV1({stateRoot:join(cache.root,".fleet"),...(input.workspaceRoot===undefined?{}:{workspaceRoot:input.workspaceRoot}),environment:input.environment});
  return sharedStageCacheV1(cache,{...(client===undefined?{}:{client}),...(input.report===undefined?{}:{report:input.report})});
 };
}
