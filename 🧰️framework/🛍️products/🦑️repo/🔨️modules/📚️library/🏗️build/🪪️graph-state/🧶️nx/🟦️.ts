import {createHash,randomBytes} from "node:crypto";
import {hostname} from "node:os";
import {mkdir,readFile,rename,rm,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {gunzipSync} from "node:zlib";
import {remoteCacheEffectiveUploadV1} from "../../🌐️remote-cache/🟦️.ts";
import {resolveRemoteCacheV1} from "../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {nxGenerationHoldsV1,writeNxGenerationReceiptV1,type NxGenerationClaimV1} from "../../../⚡️caching/🕸️graph/🔏️seal/📌️generation/🟦️.ts";
import {resolveLanguageV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {CATALOG_LANGUAGES_V1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {GRAPH_STATE_POLICY_V1 as policy,GraphStateFoldV1,graphPlatformV1,graphStateTextV1,sealGraphStateV1,type GraphStateV1} from "../🟦️.ts";
import {decideGraphEntryV1,expandRootV1,graphFleetClientV1,neutralizeRootV1,publishGraphEntryV1,type GraphDocumentV1,type GraphFleetSettingsV1,type GraphHoldV1,type GraphSayV1} from "../🛰️exchange/🟦️.ts";

type Environment=Readonly<Record<string,string|undefined>>;
const sha256=(bytes:Uint8Array|string):string=>createHash("sha256").update(bytes).digest("hex"),FILES=["project-graph.json","file-map.json","source-maps.json"] as const,PREFIX="nx/";

/** 💬️ Prints the lifecycle messages of the exchange to the error stream in the language of the environment. */
export function graphSayV1(environment:Environment):GraphSayV1{
 const language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],environment as NodeJS.ProcessEnv).language as "en"|"de";
 return (key,fields)=>{try{process.stderr.write(`[fleet] ${graphStateTextV1(key,language,fields)}\n`);}catch{}};
}

/** 🔌️ The fleet cache of this machine for graph entries: the explicit environment first, then the facts the replica daemon installed (the resolver of the remote cache module). `undefined` when no cache is configured, when the switch `SEMIO_FLEET_GRAPH=off` is set, or when the resolver refuses the environment. */
export function graphFleetSettingsV1(root:string,environment:Environment):GraphFleetSettingsV1|undefined{
 if(environment.SEMIO_FLEET_GRAPH==="off")return undefined;
 try{
  const resolved=resolveRemoteCacheV1(environment,{root});
  if(resolved===undefined)return undefined;
  return Object.freeze({endpoint:resolved.contract.endpoint.replace(/\/+$/u,""),instance:resolved.contract.instance,token:resolved.token,upload:remoteCacheEffectiveUploadV1(resolved.contract),stateDirectory:join(root,".cache","f","graph-state")});
 }catch{return undefined;}
}

/** 🧼️ The digest of a pinned portable graph with the root of this machine replaced by the root token in every record: records are content addressed by their compressed bytes, so the digest of each is computed once and kept in `memoFile` (per root), and a graph that differs in few projects costs only those. `undefined` when the snapshot is unreadable. */
export async function rootlessGraphDigestV1(manifestFile:string,root:string,memoFile?:string):Promise<string|undefined>{
 try{
  const manifest=JSON.parse(await readFile(manifestFile,"utf8")) as {version:number;graph:unknown;projects:[string,string][];dependencies:string[];externalNodes?:string[]},directory=dirname(manifestFile),scope=sha256(root).slice(0,16);
  let memo:Record<string,string>={};
  if(memoFile!==undefined)try{const value=JSON.parse(await readFile(memoFile,"utf8")) as {version:number;scope:string;rows:Record<string,string>};if(value.version===1&&value.scope===scope)memo=value.rows;}catch{}
  const wanted=new Set<string>([...manifest.projects.map(row=>row[1]),...manifest.dependencies,...(manifest.externalNodes??[])]),next:Record<string,string>={};
  let dirty=false;
  for(const digest of wanted){
   if(!/^[a-f0-9]{64}$/u.test(digest))return undefined;
   const known=memo[digest];if(known!==undefined){next[digest]=known;continue;}
   next[digest]=sha256(neutralizeRootV1(gunzipSync(await readFile(join(directory,digest+".gz"))),root));dirty=true;
  }
  if(memoFile!==undefined&&(dirty||Object.keys(memo).length!==wanted.size)){
   const temporary=`${memoFile}.${process.pid}.tmp`;
   try{await mkdir(dirname(memoFile),{recursive:true});await writeFile(temporary,JSON.stringify({version:1,scope,rows:next}));await rename(temporary,memoFile);}catch{await rm(temporary,{force:true}).catch(()=>undefined);}
  }
  const graph=sha256(neutralizeRootV1(Buffer.from(JSON.stringify(manifest.graph)),root));
  return sha256(JSON.stringify([manifest.version,graph,manifest.projects.map(([name,digest])=>[name,next[digest]]),manifest.dependencies.map(digest=>next[digest]),(manifest.externalNodes??[]).map(digest=>next[digest])]));
 }catch{return undefined;}
}

/** ⏱️ How long the last local computation of the Nx graph took on this machine (`undefined` before the first), kept in an ignored cache file so a waiting machine can judge how long the holder of a claim may need. */
export async function readNxGraphCostV1(root:string):Promise<number|undefined>{
 try{const value=JSON.parse(await readFile(join(root,".cache","f","graph-state-cost.json"),"utf8")) as {nx?:number};return Number.isFinite(value.nx)&&value.nx!>0?value.nx:undefined;}catch{return undefined;}
}

/** 📝️ Remembers the duration of a local computation of the Nx graph. */
export async function rememberNxGraphCostV1(root:string,milliseconds:number):Promise<void>{
 const file=join(root,".cache","f","graph-state-cost.json"),temporary=`${file}.${process.pid}.tmp`;
 try{await mkdir(dirname(file),{recursive:true});await writeFile(temporary,JSON.stringify({nx:Math.round(milliseconds)}));await rename(temporary,file);}catch{await rm(temporary,{force:true}).catch(()=>undefined);}
}

/** 🪪️ The state of an Nx project graph: no file folds (the graph is a function of the portable graph, the Nx tooling and the plugin implementation), so the identity is the frame alone: kind, the three digests the build already computes, the platform and the root-neutral digest of the portable graph. */
export function nxGraphStateV1(claim:NxGenerationClaimV1,rootless:string,platform:string=graphPlatformV1()):GraphStateV1{
 return sealGraphStateV1({kind:"nx-graph",tooling:claim.tooling,implementation:claim.implementation,platform,extra:{graph:rootless,identity:claim.identity}},new GraphStateFoldV1(),new GraphStateFoldV1());
}

/** 📤️ The documents of the Nx graph a finished run left in `directory`, neutralized: `undefined` while one of the three graph files is missing. */
export async function collectNxGraphV1(directory:string,root:string):Promise<readonly GraphDocumentV1[]|undefined>{
 const documents:GraphDocumentV1[]=[];
 for(const name of FILES){
  let bytes:Buffer;
  try{bytes=await readFile(join(directory,name));}catch{return undefined;}
  documents.push({name:PREFIX+name,bytes:neutralizeRootV1(bytes,root)});
 }
 return documents;
}

/** 📥️ Writes the documents of an entry into the Nx workspace data `directory` for the root of this machine: only the three graph files are accepted, each is written beside its final name and renamed, so a reader sees the old file or the new one. Returns the number of bytes written; `undefined` when the entry is not exactly the three graph files. */
export async function installNxGraphV1(directory:string,documents:readonly GraphDocumentV1[],root:string):Promise<number|undefined>{
 const names=documents.map(document=>document.name).sort();
 if(names.length!==FILES.length||FILES.some(name=>!names.includes(PREFIX+name)))return undefined;
 await mkdir(directory,{recursive:true});
 let bytes=0;
 for(const document of documents){
  const target=join(directory,document.name.slice(PREFIX.length)),temporary=`${target}.${process.pid}.fleet`,content=expandRootV1(document.bytes,root);
  try{await writeFile(temporary,content,{flag:"wx",mode:policy.installFileMode});await rename(temporary,target);bytes+=content.length;}finally{await rm(temporary,{force:true}).catch(()=>undefined);}
 }
 return bytes;
}

/** 🚦️ What the fleet did before an Nx run: `installed` when the graph of the state came from the cache (the receipt of the local generation is written), and `finish` for after the run. */
export type NxFleetV1=Readonly<{installed:boolean;state:GraphStateV1|undefined;finish(succeeded:boolean):Promise<void>}>;
const idle:NxFleetV1=Object.freeze({installed:false,state:undefined,async finish(){}});

/** 🛰️ The fleet step in front of an Nx run that has no valid graph of its own: look the graph of this state up and install it, or claim it so that the other machines wait for this one, or compute it locally as always. `finish(true)` after a successful run uploads the graph files that run left behind and drops the claim. Never throws and never waits longer than `waitLimitMilliseconds` for a holder. */
export async function beginNxFleetV1(input:Readonly<{root:string;environment:Environment;claim:NxGenerationClaimV1;directory:string;manifestFile:string|undefined;say?:GraphSayV1;signal?:AbortSignal;settings?:GraphFleetSettingsV1;waitLimitMilliseconds?:number;owner?:string;fund?:<T>(bytes:number,body:()=>Promise<T>)=>Promise<T>}>):Promise<NxFleetV1>{
 try{
  const settings=input.settings??graphFleetSettingsV1(input.root,input.environment);
  if(settings===undefined||input.manifestFile===undefined)return idle;
  const rootless=await rootlessGraphDigestV1(input.manifestFile,input.root,join(input.root,".cache","f","graph-state-records.json"));
  if(rootless===undefined)return idle;
  const state=nxGraphStateV1(input.claim,rootless),client=graphFleetClientV1(settings,{fetch:(await import("../../🌐️remote-cache/🛰️fleet/🟦️.ts")).remoteCacheFetchV1(input.environment,{root:input.root})}),owner=input.owner??`${hostname()}:${process.pid}:${randomBytes(3).toString("hex")}`.replace(/[^A-Za-z0-9._:@/+-]/gu,"-").slice(0,160);
  const cost=await readNxGraphCostV1(input.root),limit=Math.min(input.waitLimitMilliseconds??policy.claimWaitMaximumMilliseconds,Math.max(60000,2*(cost??240000))),began=Date.now();
  const decision=await decideGraphEntryV1({client,state,owner,expectedMilliseconds:cost??240000,waitLimitMilliseconds:limit,...(input.signal===undefined?{}:{signal:input.signal}),...(input.say===undefined?{}:{say:input.say})});
  if(decision.outcome==="hit"){
   const install=async():Promise<boolean>=>{const bytes=await installNxGraphV1(input.directory,decision.documents,input.root);return bytes!==undefined&&await writeNxGenerationReceiptV1(input.root,input.directory,input.claim,input.signal)&&await nxGenerationHoldsV1(input.root,input.directory,input.claim,input.signal);};
   const funded=Math.ceil(decision.documents.reduce((sum,document)=>sum+document.bytes.length,0)*1.5/4096)*4096+8192;
   if(await(input.fund?input.fund(funded,install):install()).catch(()=>false))return Object.freeze({installed:true,state,async finish(){}});
   input.say?.("skipped",{kind:"nx-graph",reason:"the entry could not be installed"});
   return idle;
  }
  if(decision.reason!=="granted")input.say?.("skipped",{kind:"nx-graph",reason:decision.reason});
  const hold:GraphHoldV1|undefined=decision.hold;
  let done=false;
  return Object.freeze({installed:false,state,async finish(succeeded:boolean){
   if(done)return;done=true;
   try{
    if(succeeded)await rememberNxGraphCostV1(input.root,Date.now()-began);
    if(!succeeded||hold===undefined){await hold?.release();return;}
    const documents=await collectNxGraphV1(input.directory,input.root);
    if(documents===undefined){await hold.release();return;}
    await publishGraphEntryV1({client,state,owner,documents,hold,...(input.say===undefined?{}:{say:input.say}),producer:owner});
   }catch{await hold?.release().catch(()=>undefined);}
  }});
 }catch{return idle;}
}
