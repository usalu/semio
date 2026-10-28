import {createHash} from "node:crypto";
import {closeSync,fstatSync,lstatSync,openSync,readSync,readlinkSync,type BigIntStats} from "node:fs";
import {isAbsolute,join,relative,resolve,sep} from "node:path";
import {compileJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {javascriptGenerationSchemaV1 as schema} from "../💾️storage/🧱️schemas/🟦️.ts";

const protocol=schema.$defs.Protocol.const,receiptSchema=compileJsonSchemaSubset(schema.$defs.Receipt,schema);

/** 🧾️ Names the closed neutral contract (the schema) every generation receipt, evidence file and verifier follows. */
export const generationProtocolV1=protocol;
/** 🚦️ Closes every reason an ordinary command must fall back to the complete ownership and provider walk. */
export type GenerationMismatchClassV1="receipt-invalid"|"evidence-invalid"|"protocol-changed"|"runtime-changed"|"configuration-changed"|"conditions-open"|"binding-changed"|"ready-changed"|"input-changed"|"manifest-changed"|"membership-changed"|"membership-churn"|"provider-changed"|"unreadable"|"sha-changed";
/** 🧰️ Carries cancellation and bounded progress for every long verification and capture loop. */
export interface GenerationControlV1{readonly signal:AbortSignal;check():void;tick(work:number):Promise<void>}
/** 🏷️ Pins the tool identity that defines which generation a Source can select. */
export interface GenerationRuntimeV1{readonly manager:string;readonly revision:string;readonly node:string;readonly platform:string;readonly arch:string;readonly sha256:string}
/** 🔖️ Points at one immutable content-addressed evidence file next to the receipt. */
export interface GenerationEvidenceV1{readonly name:string;readonly sha256:string;readonly bytes:number;readonly rows:number}
/** 🗝️ Names one installer configuration file as every installer level could read it; a missing file is part of the proof. */
export interface GenerationProducerFileV1{readonly path:string;readonly kind:"missing"|"file"|"other";readonly bytes:number;readonly sha256:string|null}
/** 🌍️ Names one ambient variable that selects registries, caches or transport, by the digest of its value so credentials never enter a receipt. */
export interface GenerationProducerVariableV1{readonly name:string;readonly sha256:string}
/** 🧪️ Collects every producer-side input outside the Source bytes that could change what an installer produces. */
export interface GenerationProducerV1{readonly files:readonly GenerationProducerFileV1[];readonly environment:readonly GenerationProducerVariableV1[]}
/** 📑️ Is the sealed, schema-defined proof that a complete walk accepted exactly these observations. */
export interface GenerationReceiptV1{
 readonly version:1;readonly purpose:"dependency-generation-receipt";readonly scope:string;readonly walkStamp:string;readonly runtime:GenerationRuntimeV1;readonly producer:GenerationProducerV1;readonly recipeSha256:string;readonly ownershipSha256:string;readonly required:readonly string[];
 readonly inputs:readonly Readonly<{scope:string;sources:readonly Readonly<{path:string;kind:"file"|"directory"|"missing";dev:string;ino:string;mode:number;sha256:string|null}>[]}>[];
 readonly membership:Readonly<{frontier:GenerationEvidenceV1;texts:readonly Readonly<{path:string;sha256:string}>[]}>;
 readonly bindings:readonly Readonly<{scope:string;pairs:readonly Readonly<{source:string;target:string}>[];observationSha256:string}>[];
 readonly ready:readonly Readonly<{scope:string;generation:string;location:string;fingerprint:string}>[];
 readonly execution:Readonly<{evidence:GenerationEvidenceV1;scopes:readonly Readonly<{scope:string;sha256:string}>[];runtimeSha256:string}>;
 readonly scopeReadSha256:string;readonly sha256:string;
}
/** 🌳️ Reports exactly what the ownership walk retained: directory listings, path states and manifest texts. */
export interface GenerationMembershipV1{readonly root:string;readonly paths:readonly Readonly<{path:string;state:readonly (string|number)[];list?:readonly Readonly<{name:string;kind:string}>[]}>[];readonly texts:readonly Readonly<{path:string;sha256:string}>[]}
/** 🔭️ Reports one physical observation of the provider walk exactly as it replays them; stamp and anchor are the colon-joined decimal fields named by the protocol. */
export interface GenerationObservationV1{readonly path:string;readonly stamp:string|null;readonly anchor:string|null;readonly kind:string;readonly target:string|null;readonly full:boolean}
/** 📥️ Collects everything a completed walk produced; nothing else may enter a receipt. */
export interface GenerationCaptureInputV1{
 readonly workspace:string;readonly storeRoot:string;readonly scope:string;readonly startedAtMs:number;readonly runtime:GenerationRuntimeV1;readonly producer:GenerationProducerV1;readonly recipeSha256:string;readonly ownershipSha256:string;readonly required:readonly string[];
 readonly inputs:readonly Readonly<{scope:string;sources:readonly Readonly<{path:string;kind:"file"|"directory"|"missing";dev:string;ino:string;mode:number;bytes:Uint8Array}>[]}>[];
 readonly membership:GenerationMembershipV1;
 readonly bindings:readonly Readonly<{scope:string;pairs:readonly Readonly<{sourceDirectory:string;targetDirectory:string}>[];observation:string}>[];
 readonly ready:readonly Readonly<{scope:string;generation:string;path:string;fingerprint:string}>[];
 readonly execution:Readonly<{scopes:readonly Readonly<{scope:string;sha256:string}>[];runtimeSha256:string;observations:readonly GenerationObservationV1[]}>;
 readonly scopeReadSha256:string;readonly control:GenerationControlV1;
}
/** 📤️ Is either a sealed receipt with its evidence bytes or a deferral that never publishes unsound facts. */
export type GenerationCaptureResultV1=Readonly<{status:"captured";receipt:GenerationReceiptV1;files:readonly Readonly<{name:string;bytes:Buffer}>[]}>|Readonly<{status:"deferred";reason:string;path?:string}>;
/** 🔎️ Is the complete input to one verification; every store specific operation arrives as a port, and `recheck` leaves out the provider observations, the last and by far the largest tier, for a second look seconds after a complete verification of the same receipt. */
export interface GenerationVerifyInputV1{
 readonly workspace:string;readonly storeRoot:string;readonly receipt:GenerationReceiptV1;readonly readEvidence:(name:string,maximumBytes:number)=>Uint8Array;readonly runtime:()=>GenerationRuntimeV1;readonly conditionsClosed:()=>boolean;
 readonly producer:()=>GenerationProducerV1;readonly recheck?:boolean;
 /** 🤝️ Answers which of the given workspace-relative paths a change journal proves untouched since the receipt was last verified; the verifier then skips their rows. Without it, or with an empty answer, every row is looked at. */
 readonly vouched?:(paths:readonly string[])=>Promise<ReadonlySet<string>>|ReadonlySet<string>;
 readonly project:(path:string)=>readonly Readonly<{name:string;kind:string}>[];
 readonly observeBindings:(scope:string,pairs:readonly Readonly<{sourceDirectory:string;targetDirectory:string}>[])=>string;
 readonly readReady:(path:string)=>Promise<string>|string;readonly control:GenerationControlV1;
}
/** 🟢️ Reproduces the exact result of a complete walk without performing it. */
export type GenerationVerifyResultV1=Readonly<{status:"match";version:1;required:readonly string[];receipts:readonly Readonly<{scope:string;generation:string;fingerprint:string}>[];sha256:string}>|Readonly<{status:"mismatch";class:GenerationMismatchClassV1;path?:string;detail?:string}>;

/** #️⃣ Digests bytes or text with the one hash every neutral implementation shares. */
export const generationSha256V1=(data:string|Uint8Array):string=>createHash("sha256").update(data).digest("hex");
const sortKeys=(value:unknown):unknown=>Array.isArray(value)?value.map(sortKeys):value!==null&&typeof value==="object"?Object.fromEntries(Object.keys(value).sort().map(key=>[key,sortKeys((value as Record<string,unknown>)[key])])):value;
/** 📐️ Serializes with sorted keys and no whitespace so any language reproduces the same seal. */
export const generationCanonicalJsonV1=(value:unknown):string=>JSON.stringify(sortKeys(value));
/** 🔏️ Seals a receipt body with the digest of its canonical form. */
export function sealGenerationReceiptV1(body:Omit<GenerationReceiptV1,"sha256">):GenerationReceiptV1{return Object.freeze({...body,sha256:generationSha256V1(generationCanonicalJsonV1(body))});}
/** 🔓️ Admits only a schema-valid, correctly sealed receipt. */
export function parseGenerationReceiptV1(bytes:Uint8Array):GenerationReceiptV1{
 if(bytes.length>protocol.maximumReceiptBytes)throw Error("Bounded dependency generation receipt required");
 const value=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes))as GenerationReceiptV1&Record<string,unknown>,errors=receiptSchema.validate(value);
 if(errors.length)throw Error("Closed dependency generation receipt required",{cause:errors.slice(0,4)});
 const{sha256,...body}=value;if(generationSha256V1(generationCanonicalJsonV1(body))!==sha256)throw Error("Sealed dependency generation receipt required");
 return Object.freeze(value);
}
/** 📍️ Names a path as workspace-, store- or absolute-relative so receipts survive no relocation yet stay compact. */
export function generationLocationV1(path:string,workspace:string,storeRoot:string):string{
 const within=(root:string)=>{const rel=relative(root,path);return rel===""||rel!==".."&&!rel.startsWith(".."+sep)&&!isAbsolute(rel)?(rel||".").split(sep).join("/"):undefined;},store=within(storeRoot),local=within(workspace);
 return store!==undefined?"s:"+store:local!==undefined?"w:"+local:"a:"+path;
}
/** 🧭️ Resolves a receipt location back to the current physical path. */
export function generationResolveV1(location:string,workspace:string,storeRoot:string):string{
 const rest=location.slice(2);if(location[1]!==":"||!rest)throw Error("Closed generation location required");
 if(location[0]==="s")return resolve(storeRoot,rest);if(location[0]==="w")return resolve(workspace,rest);if(location[0]==="a")return rest;
 throw Error("Closed generation location required");
}
/** 🧮️ Digests a directory listing projection into a short fixed width key. */
export const generationProjectionDigestV1=(rows:readonly Readonly<{name:string;kind:string}>[]):string=>generationSha256V1(JSON.stringify(rows.map(row=>({name:row.name,kind:row.kind})))).slice(0,protocol.projectionDigestUnits);
const fields=(facts:BigIntStats,names:readonly string[]):string=>names.map(name=>String(facts[name as keyof BigIntStats])).join(":");
/** 🪪️ Renders the full physical stamp exactly as the provider walk compares it. */
export const generationStampV1=(facts:BigIntStats):string=>fields(facts,protocol.stampFields);
/** ⚓️ Renders the stable identity anchor exactly as the provider walk compares it. */
export const generationAnchorV1=(facts:BigIntStats):string=>fields(facts,protocol.anchorFields);
const lstatBig=(path:string):BigIntStats|undefined=>lstatSync(path,{bigint:true,throwIfNoEntry:false});
/** 📖️ Reads one repository DATA file through a descriptor, so a manifest that lives in a schema directory is data and never an implementation read of the loader's captured file path. */
function readGenerationData(path:string,maximumBytes:number):Buffer{
 const descriptor=openSync(path,"r");
 try{
  const size=fstatSync(descriptor).size;if(size>maximumBytes)throw Error("Bounded generation data required");
  const bytes=Buffer.allocUnsafe(size);let offset=0;
  while(offset<size){const count=readSync(descriptor,bytes,offset,size-offset,offset);if(!count)break;offset+=count;}
  return offset===size?bytes:bytes.subarray(0,offset);
 }finally{closeSync(descriptor);}
}
const kindLetter=(kind:string):string=>kind==="link"?"l":kind==="directory"?"d":kind==="file"?"f":kind==="missing"?"m":"x";
const stateKind=(facts:BigIntStats):string=>facts.isSymbolicLink()?"symlink":facts.isDirectory()?"directory":facts.isFile()?"file":"other";
const stateTuple=(facts:BigIntStats|undefined):string=>JSON.stringify(facts?[stateKind(facts),String(facts.dev),String(facts.ino),Number(facts.mode)]:["missing"]);
const posixRelative=(root:string,path:string):string=>(relative(root,path)||".").split(sep).join("/");
const encodeRows=(rows:readonly (readonly unknown[])[]):Buffer=>Buffer.from(rows.map(row=>JSON.stringify(row)).join("\n")+(rows.length?"\n":""));
let decodedLast:{sha256:string;rows:(readonly unknown[])[];timer:ReturnType<typeof setTimeout>}|undefined;
const decodeRows=(bytes:Uint8Array,expected:GenerationEvidenceV1):(readonly unknown[])[]=>{
 if(bytes.length!==expected.bytes||generationSha256V1(bytes)!==expected.sha256)throw Error("Sealed generation evidence required");
 if(decodedLast?.sha256===expected.sha256)return decodedLast.rows;
 const text=new TextDecoder("utf-8",{fatal:true}).decode(bytes),rows=text.length?text.slice(0,-1).split("\n").map(line=>JSON.parse(line)as unknown[]):[];
 if(rows.length!==expected.rows||rows.some(row=>!Array.isArray(row)))throw Error("Complete generation evidence rows required");
 if(decodedLast)clearTimeout(decodedLast.timer);
 const timer=setTimeout(()=>{decodedLast=undefined;},protocol.decodedRowsRetainMilliseconds);timer.unref?.();
 decodedLast={sha256:expected.sha256,rows,timer};
 return rows;
};
const evidenceOf=(rows:readonly (readonly unknown[])[],suffix:string):Readonly<{evidence:GenerationEvidenceV1;file:Readonly<{name:string;bytes:Buffer}>}>=>{
 const bytes=encodeRows(rows),sha256=generationSha256V1(bytes);
 if(rows.length>protocol.maximumRows||bytes.length>protocol.maximumEvidenceBytes)throw Error("Generation evidence capacity exceeded");
 return Object.freeze({evidence:Object.freeze({name:sha256+suffix,sha256,bytes:bytes.length,rows:rows.length}),file:Object.freeze({name:sha256+suffix,bytes})});
};
const deferred=(reason:string,path?:string):GenerationCaptureResultV1=>Object.freeze({status:"deferred",reason,...(path===undefined?{}:{path})});

/** 📸️ Freezes a completed walk into a sealed receipt, or defers whenever a timestamp is too young to prove stability. */
export async function captureGenerationReceiptV1(input:GenerationCaptureInputV1):Promise<GenerationCaptureResultV1>{
 const{workspace,storeRoot,control,membership}=input,threshold=BigInt(Math.trunc(input.startedAtMs)-protocol.settleMilliseconds)*1000000n;let work=0;
 const advance=async()=>{if(++work%protocol.checkWork===0)control.check();if(work%protocol.progressWork===0)await control.tick(work);};
 const frontier:(readonly unknown[])[]=[],listed=new Map<string,readonly Readonly<{name:string;kind:string}>[]>(),memo=new Map<string,boolean>();
 for(const row of membership.paths)if(row.list)listed.set(row.path,row.list);
 const relevant=(path:string):boolean=>{
  const known=memo.get(path);if(known!==undefined)return known;
  const list=listed.get(path);memo.set(path,true);
  const value=list===undefined||list.some(entry=>entry.name==="package.json"||entry.kind==="symlink"||entry.kind==="directory"&&relevant(join(path,entry.name)));
  memo.set(path,value);return value;
 };
 for(const row of membership.paths){
  await advance();const path=posixRelative(workspace,row.path);
  if(!row.list){frontier.push(["s",path,...row.state]);continue;}
  const facts=lstatBig(row.path);
  if(!facts||!facts.isDirectory()||stateTuple(facts)!==JSON.stringify(row.state))return deferred("membership-directory-moved",path);
  frontier.push(["d",path,String(facts.dev),String(facts.ino),Number(facts.mode),String(facts.mtimeNs),String(facts.ctimeNs),facts.mtimeNs<threshold&&facts.ctimeNs<threshold?1:0,generationProjectionDigestV1(row.list),generationProjectionDigestV1(row.list.filter(entry=>entry.kind!=="directory"||relevant(join(row.path,entry.name)))),relevant(row.path)?1:0]);
 }
 const observations:(readonly unknown[])[]=[];
 for(const row of input.execution.observations){
  await advance();
  if(row.full&&row.stamp!==null){const parts=row.stamp.split(":");if(BigInt(parts[5]!)>=threshold||BigInt(parts[6]!)>=threshold)return deferred("provider-observation-unsettled",generationLocationV1(row.path,workspace,storeRoot));}
  observations.push(["o",generationLocationV1(row.path,workspace,storeRoot),kindLetter(row.kind),row.anchor,row.stamp,row.target,row.full?1:0]);
 }
 const inputs=input.inputs.map(group=>Object.freeze({scope:group.scope,sources:Object.freeze(group.sources.map(source=>Object.freeze({path:generationLocationV1(source.path,workspace,storeRoot),kind:source.kind,dev:source.dev,ino:source.ino,mode:source.mode,sha256:source.kind==="file"?generationSha256V1(source.bytes):null})))}));
 const frontierFile=evidenceOf(frontier,protocol.frontierSuffix),executionFile=evidenceOf(observations,protocol.executionSuffix),sha=(text:string)=>generationSha256V1(text);
 const receipt=sealGenerationReceiptV1({
  version:1,purpose:"dependency-generation-receipt",scope:input.scope,walkStamp:protocol.walkStamp,runtime:input.runtime,producer:input.producer,recipeSha256:input.recipeSha256,ownershipSha256:input.ownershipSha256,required:[...input.required],inputs,
  membership:{frontier:frontierFile.evidence,texts:membership.texts.map(row=>({path:row.path,sha256:row.sha256}))},
  bindings:input.bindings.map(row=>({scope:row.scope,pairs:row.pairs.map(pair=>({source:generationLocationV1(pair.sourceDirectory,workspace,storeRoot),target:generationLocationV1(pair.targetDirectory,workspace,storeRoot)})),observationSha256:sha(row.observation)})),
  ready:input.ready.map(row=>({scope:row.scope,generation:row.generation,location:generationLocationV1(row.path,workspace,storeRoot),fingerprint:row.fingerprint})),
  execution:{evidence:executionFile.evidence,scopes:input.execution.scopes.map(row=>({scope:row.scope,sha256:row.sha256})),runtimeSha256:input.execution.runtimeSha256},scopeReadSha256:input.scopeReadSha256,
 });
 const errors=receiptSchema.validate(receipt);if(errors.length)throw Error("Closed dependency generation receipt required",{cause:errors.slice(0,4)});
 return Object.freeze({status:"captured",receipt,files:Object.freeze([frontierFile.file,executionFile.file])});
}

const mismatch=(cls:GenerationMismatchClassV1,path?:string,detail?:string):GenerationVerifyResultV1=>Object.freeze({status:"mismatch",class:cls,...(path===undefined?{}:{path}),...(detail===undefined?{}:{detail})});
const runtimeFields=["manager","revision","node","platform","arch","sha256"]as const,movedRuntime=(left:GenerationRuntimeV1,right:GenerationRuntimeV1):string|undefined=>runtimeFields.find(field=>left[field]!==right[field]);
/** 🩺️ Reduces any verification answer to the language-neutral cause a report names. */
export function generationCauseV1(result:GenerationVerifyResultV1|Readonly<{status:"absent"}>,scope:string):Readonly<{class:string;path?:string;detail?:string}>{
 if(result.status==="mismatch")return Object.freeze({class:result.class,...(result.path===undefined?{}:{path:result.path}),...(result.detail===undefined?{}:{detail:result.detail})});
 return Object.freeze(result.status==="absent"?{class:"absent",path:scope}:{class:"sha-changed"});
}
/** 🗣️ Renders the exact input a mismatch names in one declared language; there is no hidden default language. */
export function describeGenerationMismatchV1(language:string,result:Readonly<{class:string;path?:string;detail?:string}>,values:Readonly<Record<string,string|number>>={}):string{
 const table=(protocol.texts as Readonly<Record<string,Readonly<Record<string,string>>>>)[language],template=table?.[result.class+"."+(result.detail??"")]??table?.[result.class];
 if(!table||typeof template!=="string")throw Error("Declared language and message required");
 return Object.entries({path:result.path??"",detail:result.detail??"",expected:protocol.walkStamp,limit:protocol.maximumRelist,...values}).reduce((text,[key,value])=>text.replaceAll("{"+key+"}",String(value)),template);
}

/** 🔬️ Replays a receipt against the live tree in cheapest-first tiers; never throws except on cancellation and never accepts a doubt. */
export async function verifyGenerationReceiptV1(input:GenerationVerifyInputV1):Promise<GenerationVerifyResultV1>{
 const{workspace,storeRoot,receipt,control}=input;let work=0;
 const advance=async()=>{if(++work%protocol.checkWork===0)control.check();if(work%protocol.progressWork===0)await control.tick(work);};
 try{
  control.check();
  if(receipt.walkStamp!==protocol.walkStamp)return mismatch("protocol-changed",undefined,receipt.walkStamp);
  const movedField=movedRuntime(input.runtime(),receipt.runtime);if(movedField!==undefined)return mismatch("runtime-changed",undefined,movedField);
  const producer=input.producer(),sealed=generationCanonicalJsonV1(receipt.producer);
  if(generationCanonicalJsonV1(producer)!==sealed){
   const sealedRows=[...receipt.producer.files,...receipt.producer.environment],liveRows=[...producer.files,...producer.environment],known=new Set(sealedRows.map(row=>generationCanonicalJsonV1(row))),present=new Set(liveRows.map(row=>generationCanonicalJsonV1(row))),moved=liveRows.find(row=>!known.has(generationCanonicalJsonV1(row)))??sealedRows.find(row=>!present.has(generationCanonicalJsonV1(row)))??sealedRows[0];
   return mismatch("configuration-changed",moved?("path"in moved?moved.path:"env:"+moved.name):undefined);
  }
  if(!input.conditionsClosed())return mismatch("conditions-open");
  const observed=new Map<string,string>();
  for(const row of receipt.bindings){
   await advance();let text:string;
   try{text=input.observeBindings(row.scope,row.pairs.map(pair=>({sourceDirectory:generationResolveV1(pair.source,workspace,storeRoot),targetDirectory:generationResolveV1(pair.target,workspace,storeRoot)})));}catch(error){if(control.signal.aborted)throw error;return mismatch("binding-changed",row.scope,String((error as Error)?.message??error).slice(0,200));}
   if(generationSha256V1(text)!==row.observationSha256)return mismatch("binding-changed",row.scope);observed.set(row.scope,text);
  }
  const fingerprints=new Map<string,string>();
  for(const row of receipt.ready){await advance();let fingerprint:string;try{fingerprint=await input.readReady(generationResolveV1(row.location,workspace,storeRoot));}catch(error){if(control.signal.aborted)throw error;return mismatch("ready-changed",row.scope);}if(fingerprint!==row.fingerprint)return mismatch("ready-changed",row.scope);fingerprints.set(row.scope,fingerprint);}
  const asRelative=(absolute:string):string|undefined=>{const from=relative(workspace,absolute).split(sep).join("/");return from===""||from===".."||from.startsWith("../")||isAbsolute(from)?undefined:from;};
  const known=input.vouched?await input.vouched([...receipt.inputs.flatMap(group=>group.sources.map(source=>asRelative(generationResolveV1(source.path,workspace,storeRoot)))),...receipt.membership.texts.map(row=>asRelative(resolve(workspace,row.path)))].filter((path):path is string=>path!==undefined)):new Set<string>();
  for(const group of receipt.inputs)for(const source of group.sources){
   await advance();const path=generationResolveV1(source.path,workspace,storeRoot);if(known.has(asRelative(path)??""))continue;const facts=lstatBig(path);
   if(source.kind==="missing"){if(facts)return mismatch("input-changed",source.path,"appeared");continue;}
   if(!facts||facts.isSymbolicLink()||(source.kind==="file"?!facts.isFile():!facts.isDirectory())||String(facts.dev)!==source.dev||String(facts.ino)!==source.ino||Number(facts.mode)!==source.mode)return mismatch("input-changed",source.path,"identity");
   if(source.kind==="file"){if(facts.size>BigInt(protocol.maximumSourceFileBytes)||generationSha256V1(readGenerationData(path,protocol.maximumSourceFileBytes))!==source.sha256)return mismatch("input-changed",source.path,"bytes");}
  }
  for(const row of receipt.membership.texts){
   await advance();if(known.has(asRelative(resolve(workspace,row.path))??""))continue;let text:string;
   try{text=new TextDecoder("utf-8",{fatal:true}).decode(readGenerationData(resolve(workspace,row.path),protocol.maximumSourceFileBytes));}catch(error){if(control.signal.aborted)throw error;return mismatch("manifest-changed",row.path,"unreadable");}
   if(generationSha256V1(text)!==row.sha256)return mismatch("manifest-changed",row.path,"bytes");
  }
  let relisted=0,scanned=0;
  const rows=decodeRows(input.readEvidence(receipt.membership.frontier.name,protocol.maximumEvidenceBytes),receipt.membership.frontier),flags=new Map<string,number>(),child=(parent:string,name:string)=>parent==="."?name:parent+"/"+name;
  for(const row of rows)if(row[0]==="d")flags.set(row[1]as string,row[10]as number);
  const settled=input.vouched?await input.vouched(rows.map(row=>row[1]as string)):new Set<string>();
  const scan=(path:string):boolean=>{
   if(++scanned>protocol.maximumScan)return true;
   let list:readonly Readonly<{name:string;kind:string}>[];try{list=input.project(path);}catch{return true;}
   return list.some(entry=>entry.name==="package.json"||entry.kind==="symlink"||entry.kind==="directory"&&scan(join(path,entry.name)));
  };
  for(const row of rows){
   if(settled.has(row[1]as string)){if(++work%protocol.checkWork===0)control.check();if(work%protocol.progressWork===0)await control.tick(work);continue;}
   await advance();const path=resolve(workspace,row[1]as string),facts=lstatBig(path);
   if(row[0]==="s"){if(stateTuple(facts)!==JSON.stringify(row.slice(2)))return mismatch("membership-changed",row[1]as string,"state");continue;}
   if(!facts||!facts.isDirectory()){if(row[10]===0&&!facts?.isSymbolicLink())continue;return mismatch("membership-changed",row[1]as string,"directory");}
   if(row[7]===1&&String(facts.dev)===row[2]&&String(facts.ino)===row[3]&&Number(facts.mode)===row[4]&&String(facts.mtimeNs)===row[5]&&String(facts.ctimeNs)===row[6])continue;
   if(++relisted>protocol.maximumRelist)return mismatch("membership-churn",row[1]as string);
   const live=input.project(path);if(generationProjectionDigestV1(live)===row[8])continue;
   const pruned=live.filter(entry=>entry.kind!=="directory"||(flags.get(child(row[1]as string,entry.name))??(scan(join(path,entry.name))?1:0))===1);
   if(generationProjectionDigestV1(pruned)!==row[9])return mismatch("membership-changed",row[1]as string,"listing");
  }
  if(!input.recheck)for(const row of decodeRows(input.readEvidence(receipt.execution.evidence.name,protocol.maximumEvidenceBytes),receipt.execution.evidence)){
   await advance();const path=generationResolveV1(row[1]as string,workspace,storeRoot),facts=lstatBig(path),full=row[6]===1,expected=full?row[4]:row[3];
   if((facts?(full?generationStampV1(facts):generationAnchorV1(facts)):null)!==expected)return mismatch("provider-changed",row[1]as string,"identity");
   if(row[2]==="l"&&readlinkSync(path)!==row[5])return mismatch("provider-changed",row[1]as string,"target");
  }
  const receipts=receipt.required.map(scope=>{const ready=receipt.ready.find(row=>row.scope===scope);if(!ready)throw Error("Complete ready rows required");return Object.freeze({scope,generation:ready.generation,fingerprint:fingerprints.get(scope)!});}),
   sha256=generationSha256V1(JSON.stringify([receipts,receipt.required.map(scope=>[scope,observed.get(scope)]),receipt.execution.scopes]));
  if(sha256!==receipt.scopeReadSha256)return mismatch("sha-changed");
  control.check();
  return Object.freeze({status:"match",version:1,required:Object.freeze([...receipt.required]),receipts:Object.freeze(receipts),sha256});
 }catch(error){
  if(control.signal.aborted)throw error;
  return mismatch(/evidence/iu.test(String((error as Error)?.message))?"evidence-invalid":"unreadable",undefined,String((error as Error)?.message??error).slice(0,200));
 }
}

/** 🧫️ Is the answer of an in-process replay: the walk's own recorded observations compared with the live tree, a deferral that proves nothing, or a typed mismatch. */
export type GenerationReplayResultV1=Readonly<{status:"match";sha256:string}>|Readonly<{status:"deferred";reason:string;path?:string}>|Readonly<{status:"mismatch";class:GenerationMismatchClassV1|"seal";path?:string}>;
/** 🔂️ Is how a completed walk was proven against the live tree before the command trusted it. */
export type GenerationConfirmationV1<W>=Readonly<{via:"replay";sha256:string}>|Readonly<{via:"walk";sha256:string;walk:W}>;
/** 🔁️ Confirms a completed walk by replaying its recorded observations and falls back to one more complete walk only when the replay cannot decide. */
export async function confirmGenerationWalkV1<W extends Readonly<{sha256:string}>>(ports:Readonly<{replay:()=>Promise<GenerationReplayResultV1>;walk:()=>Promise<W>}>):Promise<GenerationConfirmationV1<W>>{
 const replayed=await ports.replay();
 if(replayed.status==="match")return Object.freeze({via:"replay",sha256:replayed.sha256});
 const walk=await ports.walk();
 return Object.freeze({via:"walk",sha256:walk.sha256,walk});
}
/** 🕊️ Closes every way one receipt publication ends; only a cancelled run throws. */
export type GenerationPublishOutcomeV1="published"|"present"|"deferred"|"stale"|"failed"|"busy";
/** 🤲️ Runs one publication attempt that must never decide readiness: every failure except cancellation is reported and swallowed. */
export async function publishGenerationBestEffortV1(ports:Readonly<{attempt:()=>Promise<GenerationPublishOutcomeV1>;aborted:()=>boolean;report:(outcome:GenerationPublishOutcomeV1|"error",detail?:string)=>void}>):Promise<GenerationPublishOutcomeV1|"error">{
 try{const outcome=await ports.attempt();ports.report(outcome);return outcome;}
 catch(error){if(ports.aborted())throw error;ports.report("error",String((error as Error)?.message??error).slice(0,300));return "error";}
}
/** 🧷️ Serialises receipt writers among themselves without queueing: a held writer lease is reported as busy, a cancelled run throws. */
export async function runGenerationWriterV1(ports:Readonly<{acquire:()=>Promise<Readonly<{release:()=>void}>>;aborted:()=>boolean;body:()=>Promise<GenerationPublishOutcomeV1>}>):Promise<GenerationPublishOutcomeV1>{
 let lease:Readonly<{release:()=>void}>;
 try{lease=await ports.acquire();}
 catch(error){if(ports.aborted())throw error;if(/^Resource lease timed out/u.test(String((error as Error)?.message)))return "busy";throw error;}
 try{return await ports.body();}
 finally{lease.release();}
}

/** 🛡️ Contains a replay that cannot even capture: only cancellation throws, any other error is a deferral so the complete walk decides. */
export async function guardGenerationReplayV1(ports:Readonly<{run:()=>Promise<GenerationReplayResultV1>;aborted:()=>boolean}>):Promise<GenerationReplayResultV1>{
 try{return await ports.run();}
 catch(error){if(ports.aborted())throw error;return Object.freeze({status:"deferred",reason:"capture-failed",path:String((error as Error)?.message??error).slice(0,200)});}
}
/** ⏮️ Is the start time that bounds the settle rule of a walk: the earliest of the shared walk's own start and the current pass. */
export const generationWalkOriginV1=(sharedStartedAtMs:number|undefined,passStartedAtMs:number):number=>sharedStartedAtMs===undefined?passStartedAtMs:Math.min(sharedStartedAtMs,passStartedAtMs);
