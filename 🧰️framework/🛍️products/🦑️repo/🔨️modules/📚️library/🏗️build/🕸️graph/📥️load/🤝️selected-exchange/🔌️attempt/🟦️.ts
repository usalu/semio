import {createHash,randomUUID} from "node:crypto";
import {mkdir,readFile,rename,rm,writeFile} from "node:fs/promises";
import {dirname} from "node:path";
import {hostname,totalmem} from "node:os";
import {hostOfRootV1,type ObservationRowV1} from "../../../../🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts";
import {graphFleetClientV1,type GraphFleetClientV1} from "../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import {graphFleetSettingsV1} from "../../../../🪪️graph-state/🧶️nx/🟦️.ts";
import {remoteCacheFetchV1} from "../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {decodeAuthorityObservationsV1,encodeAuthorityObservationsV1} from "../../../../👁️observation/🔢️row-table/🟨️.mjs";
import authoritySchema from "../../../../🧬️schema/📥️authority.json" with {type:"json"};
import snapshotPolicy from "../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🔣️policy.json" with {type:"json"};
import {createProjectFileIndexV1} from "../../../🧭️file-owner/🟨️.mjs";
import {registerSliceReportSourceV1,sliceReadStepV1,sliceReadActiveV1} from "../../../../🧠️memory/📋️slice-report/🟨️.mjs";
import {SELECTED_EXCHANGE_TEXTS_V1,selectedGenerationV1,type SelectedAdmissionEntryV1,type SelectedSealedV1} from "../🟦️.ts";

const sha=(text:string):string=>createHash("sha256").update(text).digest("hex");

/** 🔌️ What the attempt of the load module lends to the exchange: its own read, the admission facts the process remembers per owner and the receipt of this implementation. */
export type SelectedAttemptPortV1<G>=Readonly<{
 root:string;scope:string;path:string;authorityPath:string;
 attempt():Promise<G|undefined>;
 sealed(value:G|undefined):boolean;
 holds(rows:readonly unknown[]):Promise<boolean>;
 facts:Readonly<{entries(scope:string):readonly Readonly<{suffix:string;facts:unknown;rows:readonly unknown[]}>[];seed(scope:string,suffix:string,facts:unknown,rows:readonly unknown[]):void}>;
 module:unknown;
 verdict(input:Readonly<{projects:unknown;rows:readonly unknown[]}>):unknown;
 client?:GraphFleetClientV1|undefined;totalMegabytes?:number;environment?:Readonly<Record<string,string|undefined>>;platform?:string;owner?:string;
 say?:(line:string)=>void;signal?:AbortSignal;
}>;

/** 🔌️ The client of the fleet exchange of this machine, or nothing when no remote cache is configured or the fleet graph exchange is off. */
export function selectedFleetClientV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env):GraphFleetClientV1|undefined{
 const settings=graphFleetSettingsV1(root,environment);
 return settings===undefined?undefined:graphFleetClientV1(settings,{fetch:remoteCacheFetchV1(environment,{root})});
}

const textOf=(language:"en"|"de")=>(key:keyof typeof SELECTED_EXCHANGE_TEXTS_V1,fields:Readonly<Record<string,string|number>>):string=>SELECTED_EXCHANGE_TEXTS_V1[key][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(fields[name]??""));

/** 🧲️ Reads a selection through the fleet: the head of the scope is looked up, a generation whose owners hold in this tree is installed into the graph store (their admission facts seed the memo, the sealed graph and its receipt are written when every owner holds), and the attempt then runs as ever, so a complete generation is served by the receipt without one inference and a partial one costs only the owners that differ. The sealed result of an attempt is offered to the fleet as the new head. Without a fleet, or on any fleet fault, the attempt runs alone.
 * @see ../🟦️.ts */
export async function loadSelectedThroughFleetV1<G>(port:SelectedAttemptPortV1<G>):Promise<G|undefined>{
 let environment=port.environment??process.env,client:GraphFleetClientV1|undefined;
 try{client=port.client??selectedFleetClientV1(port.root,environment);}catch{client=undefined;}
 if(client===undefined)return port.attempt();
 const language=environment.SEMIO_LANGUAGE==="de"?"de":"en",say=port.say??((line:string)=>{try{process.stderr.write(line+"\n");}catch{}}),host=hostOfRootV1(port.root),platform=port.platform??`${process.platform}-${process.arch}`.toLowerCase();
 let result:G|undefined,ran=false,failed=false,failure:unknown,ownerOf:(relative:string)=>string|undefined=()=>undefined;
 const sealedOf=async():Promise<SelectedSealedV1|undefined>=>{
  const source=await readFile(port.path,"utf8"),authority=JSON.parse(await readFile(port.authorityPath,"utf8")) as {version:number;membership:string;graph:string;observations:unknown};
  if(authority.version!==authoritySchema.properties.version.const||authority.graph!==sha(source))return undefined;
  const projects=(JSON.parse(source) as {projects?:Record<string,{root:string}>}).projects??{};ownerOf=createProjectFileIndexV1(projects);
  const admission:SelectedAdmissionEntryV1[]=port.facts.entries(port.scope).map(entry=>({owner:entry.suffix.slice(0,entry.suffix.indexOf("\0")),suffix:entry.suffix,facts:entry.facts,rows:entry.rows as ObservationRowV1[]}));
  return{graph:source,rows:decodeAuthorityObservationsV1(authority.observations,authoritySchema.definitions.ObservationKind.enum,()=>port.signal?.throwIfAborted()) as ObservationRowV1[],membership:authority.membership,admission};
 };
 const outcome=await selectedGenerationV1({
  client,scope:port.scope,platform,host,owner:port.owner??`${hostname()}:${process.pid}:${randomUUID().slice(0,6)}`,
  ownerOf:relative=>ownerOf(relative),holds:rows=>port.holds(rows),
  say:(key,fields)=>say(textOf(language)(key,fields)),
  install:async plan=>{
   const matching=new Set(plan.matching);
   for(const entry of plan.fetched.admission)if(matching.has(entry.owner))port.facts.seed(port.scope,entry.suffix,entry.facts,entry.rows);
   if(!plan.complete)return;
   const rows=[...plan.fetched.rowsByOwner.values()].flat(),projects=(JSON.parse(plan.fetched.graph) as {projects?:unknown}).projects??{};
   const header=JSON.stringify({version:authoritySchema.properties.version.const,module:port.module,membership:plan.fetched.head.membership,graph:sha(plan.fetched.graph),seal:port.verdict({projects,rows})}).slice(0,-1)+',"observations":';
   const authority=header+encodeAuthorityObservationsV1(rows as readonly (ObservationRowV1 & {fact:unknown})[],snapshotPolicy.maximumSnapshotBytes-Buffer.byteLength(header)-1,()=>port.signal?.throwIfAborted())+"}";
   await mkdir(dirname(port.path),{recursive:true});
   const graphTemp=port.path+"."+randomUUID(),receiptTemp=port.authorityPath+"."+randomUUID();
   try{await writeFile(graphTemp,plan.fetched.graph,{flag:"wx"});await writeFile(receiptTemp,authority,{flag:"wx"});await rename(graphTemp,port.path);await rename(receiptTemp,port.authorityPath);}finally{await Promise.all([rm(graphTemp,{force:true}),rm(receiptTemp,{force:true})]);}
  },
  readBesideMegabytes:port.totalMegabytes??Math.floor(totalmem()/1048576),
  signal:port.signal,alive:()=>sliceReadActiveV1(),step:sliceReadStepV1 as never,
  read:async()=>{ran=true;try{result=await port.attempt();}catch(error){failed=true;failure=error;throw error;}if(!port.sealed(result))return undefined;try{return await sealedOf();}catch{return undefined;}}
 }).catch(error=>{if(failed)throw failure;if(port.signal?.aborted)throw error;return undefined;});
 if(outcome!==undefined){
  const said={outcome:outcome.outcome,held:outcome.held,differing:outcome.differing.length,published:outcome.published,waitedMilliseconds:outcome.waitedMilliseconds};
  try{registerSliceReportSourceV1("exchange",()=>said);process.stderr.write("[selected-exchange] "+JSON.stringify({version:1,...said})+"\n");}catch{}
 }
 if(outcome===undefined||!ran)return ran?result:port.attempt();
 return result;
}
