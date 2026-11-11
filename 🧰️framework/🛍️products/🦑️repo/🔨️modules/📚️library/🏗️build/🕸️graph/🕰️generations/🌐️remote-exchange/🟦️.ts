import {createHash} from "node:crypto";
import {join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {GENERATION_POLICY_V1 as P,readGenerationManifestV1,readGenerationPointerV1,type GenerationFilePortV1} from "../🟨️.mjs";
import {readRemoteCacheTokenV1,remoteCacheEffectiveUploadV1,resolveRemoteCacheContractV1} from "../../../🌐️remote-cache/🟦️.ts";
import {httpRemoteExecutionContentV1,type RemoteExecutionContentV1} from "../../../🌐️remote-execution/🗄️content/🟦️.ts";
import {REAPI_V1,decodeProtobufV1,encodeProtobufV1} from "../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🏷️ The inline description of one uploaded generation: what a client compares before it downloads anything. */
export type GenerationNoteV1=Readonly<{version:1;generation:number;owners:number;identity:string;manifest:string}>;
/** 🧾️ What one exchange did; a refusal or a failure never fails the graph load that asked. */
export type GenerationExchangeOutcomeV1=Readonly<{status:"restored"|"uploaded"|"local"|"absent"|"foreign"|"empty"|"current"|"remote-newer"|"incomplete"|"oversized"|"failed";generation?:number;owners?:number;documents?:number;bytes?:number;reason?:string}>;
/** 🔌️ The two operations the generation load asks of the remote cache of the machine group. */
export interface GenerationExchangeV1{
 /** ⬆️ Whether this machine may write to the remote cache. */
 readonly upload:boolean;
 restore(options:{root:string;fs:GenerationFilePortV1;signal?:AbortSignal}):Promise<GenerationExchangeOutcomeV1>;
 publish(options:{root:string;fs:GenerationFilePortV1;signal?:AbortSignal;ifAbsent?:boolean}):Promise<GenerationExchangeOutcomeV1>;
}
type Files=Pick<GenerationFilePortV1,"read"|"write">;
type Location=Readonly<{content:RemoteExecutionContentV1;instance:string;key:string;root:string;files:Files;signal?:AbortSignal}>;

const policy=schema.definitions.Policy.const,document=new RegExp(policy.document,"u"),sha=(bytes:Uint8Array):string=>createHash("sha256").update(bytes).digest("hex");

/** 🗝️ The action key of the generation of one checkout: the generation format, the owner registry identity and the absolute workspace root, because fact rows carry absolute paths. */
export function generationExchangeKeyV1(identity:string,workspace:string):string{return sha(Buffer.from(JSON.stringify([policy.domain,identity,workspace])));}

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

async function remoteNote(location:Location):Promise<{note:GenerationNoteV1|undefined;entries:readonly {path:string;hash:string;size:number}[]}|undefined>{
 const bytes=await location.content.readActionResult(location.instance,location.key,location.signal);if(!bytes)return undefined;
 const result=decodeProtobufV1(REAPI_V1.actionResult,bytes),files=(result.output_files??[]) as {path:string;digest:{hash:string;size_bytes:number|string}}[];
 return{note:noteOf(result.stdout_raw as Uint8Array|undefined),entries:files.map(file=>({path:file.path,hash:file.digest.hash,size:Number(file.digest.size_bytes)}))};
}

/** ⬆️ Uploads the newest sealed generation of one store (documents first, then the action result that names them), unless the remote cache already holds that generation or a larger one. */
export async function publishGenerationV1(location:Location&Readonly<{ifAbsent?:boolean}>):Promise<GenerationExchangeOutcomeV1>{
 const{content,instance,key,root,files,signal}=location,pointerBytes=await files.read(join(root,P.pointer));if(!pointerBytes)return{status:"empty"};
 const pointer=readGenerationPointerV1(pointerBytes);if(pointer.manifest===null)return{status:"empty"};
 const manifestBytes=await files.read(join(root,P.manifests,pointer.manifest+".json"));if(!manifestBytes||sha(manifestBytes)!==pointer.manifest)return{status:"incomplete",reason:"manifest"};
 const manifest=readGenerationManifestV1(manifestBytes),note:GenerationNoteV1={version:1,generation:manifest.generation,owners:Object.keys(manifest.owners).length,identity:manifest.identity,manifest:pointer.manifest};
 const remote=await remoteNote(location).catch(()=>undefined);
 if(remote?.note&&(remote.note.manifest===note.manifest))return{status:"current",generation:note.generation,owners:note.owners};
 if(remote&&(location.ifAbsent||remote.note&&(remote.note.owners>note.owners||remote.note.owners===note.owners&&remote.note.generation>=note.generation)))return{status:"remote-newer",generation:note.generation,owners:note.owners};
 const paths=generationDocumentPathsV1(pointer.manifest,manifest);if(paths.length>policy.maximumDocuments)return{status:"oversized",reason:String(paths.length)+" documents"};
 const rows:{path:string;hash:string;size:number}[]=[];
 await each(paths,async path=>{const bytes=await files.read(join(root,path));if(!bytes||bytes.length>policy.maximumDocumentBytes)throw Error("generation document unavailable: "+path);const hash=sha(bytes);if(!path.includes(hash))throw Error("generation document differs from its name: "+path);rows.push({path,hash,size:bytes.length});});
 rows.sort((a,b)=>a.path<b.path?-1:1);
 const missing=new Set((await content.missing(instance,rows.map(row=>({hash:row.hash,size:row.size})),signal)).map(digest=>digest.hash));let bytesSent=0;
 await each(rows.filter(row=>missing.has(row.hash)),async row=>{const bytes=await files.read(join(root,row.path));if(!bytes||sha(bytes)!==row.hash)throw Error("generation document changed while it was uploaded: "+row.path);await content.write(instance,{hash:row.hash,size:row.size},bytes,signal);bytesSent+=row.size;});
 const result=encodeProtobufV1(REAPI_V1.actionResult,{output_files:rows.map(row=>({path:row.path,digest:{hash:row.hash,size_bytes:row.size}})),stdout_raw:Buffer.from(JSON.stringify(note))});
 await content.writeActionResult(instance,key,result,signal);
 return{status:"uploaded",generation:note.generation,owners:note.owners,documents:rows.length,bytes:bytesSent};
}

/** ⬇️ Makes an empty store serve the generation another machine of the group uploaded: every document is verified against its name, the pointer is written last, and a store that already has a pointer is left alone. */
export async function restoreGenerationV1(location:Location&Readonly<{identity:string}>):Promise<GenerationExchangeOutcomeV1>{
 const{content,instance,root,files,signal}=location,pointerPath=join(root,P.pointer),present=await files.read(pointerPath);
 if(present)try{if(readGenerationPointerV1(present).manifest!==null)return{status:"local"};}catch{}
 const remote=await remoteNote(location);if(!remote)return{status:"absent"};
 const{note,entries}=remote;if(!note||note.identity!==location.identity)return{status:"foreign"};
 const manifestPath=`${P.manifests}/${note.manifest}.json`;
 if(entries.length>policy.maximumDocuments||!entries.some(entry=>entry.path===manifestPath)||entries.some(entry=>!document.test(entry.path)||!entry.path.includes(entry.hash)||!(entry.size>=0)))return{status:"incomplete",reason:"action result"};
 let bytes=0,documents=0;
 await each(entries,async entry=>{
  if(await files.read(join(root,entry.path)))return;
  const data=await content.read(instance,{hash:entry.hash,size:entry.size},signal);if(!data)throw Error("document missing in the remote cache: "+entry.path);
  if(data.length>policy.maximumDocumentBytes||sha(data)!==entry.hash)throw Error("document differs from its name: "+entry.path);
  await files.write(join(root,entry.path),data,{ifAbsent:true});bytes+=data.length;documents++;
 });
 const manifestBytes=await files.read(join(root,manifestPath));if(!manifestBytes||sha(manifestBytes)!==note.manifest)throw Error("restored manifest unreadable");
 const manifest=readGenerationManifestV1(manifestBytes);
 await files.write(pointerPath,Buffer.from(JSON.stringify({version:1,generation:manifest.generation,manifest:note.manifest,parent:null,reason:"restored from the remote cache"})));
 return{status:"restored",generation:manifest.generation,owners:note.owners,documents,bytes};
}

/** 🌐️ The exchange of this process: `undefined` while no remote cache endpoint is configured. A failure disables it for the rest of the process, so a dead cache server costs one short probe instead of a timeout per load. */
export function remoteGenerationExchangeV1(input:Readonly<{environment:Readonly<Record<string,string|undefined>>;workspace:string;identity():Promise<string>}>):GenerationExchangeV1|undefined{
 let contract;try{contract=resolveRemoteCacheContractV1(input.environment);}catch{return undefined;}
 if(!contract)return undefined;
 const token=readRemoteCacheTokenV1(contract,input.environment),make=(probe:boolean)=>httpRemoteExecutionContentV1({endpoint:contract.endpoint,token,timeoutSeconds:probe?policy.probeTimeoutSeconds:contract.timeoutSeconds,retries:probe?policy.probeRetries:contract.retries});
 let down=false;
 const guard=async(run:()=>Promise<GenerationExchangeOutcomeV1>):Promise<GenerationExchangeOutcomeV1>=>{
  if(down)return{status:"failed",reason:"the remote cache was unreachable earlier in this process"};
  try{return await run();}catch(error){down=true;return{status:"failed",reason:String((error as Error)?.message??error).slice(0,200)};}
 };
 return Object.freeze({
  upload:remoteCacheEffectiveUploadV1(contract),
  restore:({root,fs,signal})=>guard(async()=>restoreGenerationV1({content:make(false),instance:contract.instance,key:generationExchangeKeyV1(await input.identity(),input.workspace),identity:await input.identity(),root,files:fs,signal})),
  publish:({root,fs,signal,ifAbsent})=>guard(async()=>publishGenerationV1({content:make(ifAbsent===true),instance:contract.instance,key:generationExchangeKeyV1(await input.identity(),input.workspace),root,files:fs,signal,ifAbsent})),
 });
}
