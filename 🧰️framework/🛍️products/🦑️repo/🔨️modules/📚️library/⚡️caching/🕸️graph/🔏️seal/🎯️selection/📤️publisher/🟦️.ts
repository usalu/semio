import {createHash} from "node:crypto";
import {lstat,readFile,writeFile,rename,rm} from "node:fs/promises";
import {isAbsolute,join,relative} from "node:path";
import {createSelectedGraphSourceReceiptV1,selectedGraphSourceReceiptHoldsV1,isGlobalGraphIdentityPathV1,validateSelectedGraphValueV1,type SelectedGraphJournalWitnessV1,type SelectedGraphScopeV1,type SelectedGraphSourcePortsV1,type SelectedGraphSourceReceiptV1} from "../🟦️.ts";
import {graphEnvironmentIdentityV1} from "../../../../../🔌️nx-plugin/🌍️environment/🟦️.ts";
import {observationsHold} from "../../../../../🔌️nx-plugin/👁️observation/🟨️.mjs";

/** 🔏️ The graph producer binds its native file map and inference authorities to the live observer. */
export interface SelectedGraphPublisherReceiptV1 {readonly version:1;readonly root:string;readonly rootDirectory:Readonly<{device:string;inode:string}>;readonly pid:number;readonly startedAt:number;readonly toolingIdentity:string;readonly environmentIdentity:string;readonly journal:SelectedGraphJournalWitnessV1;readonly graph:Readonly<Record<string,readonly string[]>>;readonly receipts:Readonly<Record<string,string>>;}
/** 🧾️ A narrowed source handoff retains the actual publisher and immutable graph custody. */
export interface SelectedGraphReuseReceiptV1 {readonly version:1;readonly toolingIdentity:string;readonly environmentIdentity:string;readonly publisherSha256:string;readonly graph:Readonly<Record<string,readonly string[]>>;readonly source:SelectedGraphSourceReceiptV1;}
/** 🧭️ Exact task input ownership is resolved by the installed task graph through owned values. */
export interface SelectedGraphReusePortsV1 extends SelectedGraphSourcePortsV1 {readonly toolingIdentity:string;readonly environment:Readonly<Record<string,string|undefined>>;scope(graph:unknown):Promise<SelectedGraphScopeV1|null>;}
const destination="semio-selected-graph-publisher.json",graphs=["project-graph.json","file-map.json"],receipts=["emoji-project-json.publication.json","test-cases.publication.json"];
const hash=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex");
async function readPublication(file:string):Promise<Buffer>{const before=await lstat(file,{bigint:true});if(!before.isFile()||before.isSymbolicLink())throw Error("Original graph publication requires a real file");const bytes=await readFile(file),after=await lstat(file,{bigint:true});if(["dev","ino","size","mtimeNs","ctimeNs"].some(key=>(before as any)[key]!==(after as any)[key]))throw Error("Original graph publication changed during receipt");return bytes;}
async function graphStamps(directory:string):Promise<Record<string,readonly string[]>>{
 const result:Record<string,readonly string[]>={};for(const name of graphs){const status=await lstat(join(directory,name),{bigint:true});if(!status.isFile()||status.isSymbolicLink())throw Error("Original graph publisher requires real cache files");result[name]=[status.dev,status.ino,status.size,status.mtimeNs,status.ctimeNs].map(String);}return result;
}
function owned(path:string,scope:SelectedGraphScopeV1):boolean{return scope.files.includes(path)||scope.roots.some(root=>path===root||path.startsWith(root+"/")||root.startsWith(path+"/"))||scope.files.some(file=>file.startsWith(path+"/"));}
function observationPaths(root:string,value:{kind:string;path:string}):readonly string[]{
 const paths=value.kind==="resolve"?[JSON.parse(value.path)[0]]:[value.path];return paths.map(path=>relative(root,path).replaceAll("\\","/"));
}
function originalFiles(value:unknown):{file:string;hash:string|null}[]{
 const map=(value as {fileMap?:{projectFileMap?:Record<string,{file:string;hash:string|null}[]>;nonProjectFiles?:{file:string;hash:string|null}[]}})?.fileMap;
 if(!map?.projectFileMap||!Array.isArray(map.nonProjectFiles))throw Error("Original native source map required");return [...Object.values(map.projectFileMap).flat(),...map.nonProjectFiles];
}
async function publisherInputs(directory:string,receipt:SelectedGraphPublisherReceiptV1,scope?:SelectedGraphScopeV1,root?:string):Promise<unknown[]>{
 const facts:unknown[]=[],authority:unknown[]=[];
 for(const name of receipts){const bytes=await readPublication(join(directory,"inference",name));if(hash(bytes)!==receipt.receipts[name])throw Error("Original graph producer publication changed");const source=JSON.parse(bytes.toString());validateSelectedGraphValueV1("InferencePublicationV1",source);if(source.pid!==receipt.pid||source.startedAt<receipt.startedAt||source.completedAt<source.startedAt||source.completedAt>Number(receipt.graph["project-graph.json"]![3])/1e6)throw Error("Original graph publication producer refused");for(const fact of source.observations)facts.push(fact);for(const row of source.inference){if(!isAbsolute(row.path)||isAbsolute(relative(root!,row.path))||relative(root!,row.path).split(/[\\/]/u).includes(".."))throw Error("Original inference publication must remain in workspace custody");for(const fact of row.authorityObservations)authority.push(fact);for(const fact of row.observations)facts.push(fact);}}
 if(!scope)return [...authority,...facts];
 return [...authority,...facts.filter((value:any)=>observationPaths(root!,value).some(path=>isAbsolute(path)||path===".."||path.startsWith("../")||path.startsWith("node_modules/")||isGlobalGraphIdentityPathV1(path)||owned(path,scope)))];
}

/** ✍️ Only the actual cache publisher's original PID-bound receipts can create reuse authority. */
export async function writeSelectedGraphPublisherReceiptV1(root:string,directory:string,startedAt:number,ports:Pick<SelectedGraphReusePortsV1,"capture"|"toolingIdentity"|"environment">,signal?:AbortSignal):Promise<boolean>{
 const file=join(directory,destination);await rm(file,{force:true});
 try{
  signal?.throwIfAborted();if(!Number.isSafeInteger(startedAt)||startedAt<1||!/^[a-f0-9]{64}$/u.test(ports.toolingIdentity))throw Error("Original selected graph publisher identity required");
  const graph=await graphStamps(directory),journal=await ports.capture({roots:[],files:[]}),original=originalFiles(JSON.parse(await readFile(join(directory,"file-map.json"),"utf8")));
  validateSelectedGraphValueV1("JournalWitnessV1",journal);if(JSON.stringify([...new Set(original.map(row=>row.file).filter(isGlobalGraphIdentityPathV1))].sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right))))!==JSON.stringify(journal.globalIdentity.paths))throw Error("Original publisher global source membership disagrees with live observer");
  const evidence:Record<string,string>={};
  for(const name of receipts){const bytes=await readPublication(join(directory,"inference",name)),value=JSON.parse(bytes.toString());validateSelectedGraphValueV1("InferencePublicationV1",value);if(value.pid!==process.pid||value.startedAt<startedAt||value.completedAt<value.startedAt||value.completedAt>Number(graph["project-graph.json"]![3])/1e6)throw Error("Original selected graph publication belongs to another producer");evidence[name]=hash(bytes);}

  const rootStatus=await lstat(root,{bigint:true});if(!rootStatus.isDirectory()||rootStatus.isSymbolicLink())throw Error("Original graph publisher workspace requires real directory custody");
  const receipt:SelectedGraphPublisherReceiptV1={version:1,root,rootDirectory:{device:String(rootStatus.dev),inode:String(rootStatus.ino)},pid:process.pid,startedAt,toolingIdentity:ports.toolingIdentity,environmentIdentity:graphEnvironmentIdentityV1(ports.environment),journal,graph,receipts:evidence};
  validateSelectedGraphValueV1("PublisherReceiptV1",receipt);await publisherInputs(directory,receipt,undefined,root);
  const after=await ports.capture({roots:[],files:[]});if(JSON.stringify(after)!==JSON.stringify(journal)||JSON.stringify(await graphStamps(directory))!==JSON.stringify(graph))throw Error("Original selected graph publication changed during capture");
  const temporary=file+"."+process.pid+".tmp";await writeFile(temporary,JSON.stringify(receipt));await rename(temporary,file);return true;
 }catch(error){signal?.throwIfAborted();if(process.env.SEMIO_NX_PROFILE_INPUTS==="1"||process.env.NX_VERBOSE_LOGGING==="true")console.log("[DEBUG] Selected graph publisher refused: "+String(error));return false;}
}

/** ✅️ Current selected sources and observations may reuse only the original graph producer's immutable evidence. */
export async function captureSelectedGraphReuseReceiptV1(root:string,directory:string,ports:SelectedGraphReusePortsV1,signal?:AbortSignal):Promise<SelectedGraphReuseReceiptV1|null>{
 try{
  const publisherBytes=await readPublication(join(directory,destination)),receipt=JSON.parse(publisherBytes.toString()) as SelectedGraphPublisherReceiptV1;validateSelectedGraphValueV1("PublisherReceiptV1",receipt);
  const rootStatus=await lstat(root,{bigint:true});if(receipt.root!==root||!rootStatus.isDirectory()||rootStatus.isSymbolicLink()||String(rootStatus.dev)!==receipt.rootDirectory.device||String(rootStatus.ino)!==receipt.rootDirectory.inode||receipt.toolingIdentity!==ports.toolingIdentity||receipt.environmentIdentity!==graphEnvironmentIdentityV1(ports.environment)||JSON.stringify(await graphStamps(directory))!==JSON.stringify(receipt.graph))return null;
  const graph=JSON.parse(await readFile(join(directory,"project-graph.json"),"utf8")),scope=await ports.scope(graph);if(!scope)return null;
  validateSelectedGraphValueV1("ScopeV1",scope);if(scope.roots.includes("."))return null;
  const before=await ports.capture(scope);if(receipt.journal.epoch!==before.epoch||receipt.journal.globalIdentity.sha256!==before.globalIdentity.sha256)return null;
  const original=originalFiles(JSON.parse(await readFile(join(directory,"file-map.json"),"utf8"))),source=await createSelectedGraphSourceReceiptV1(root,scope,before,original,ports,signal),facts=await publisherInputs(directory,receipt,scope,root);
  if(!await observationsHold(facts,64,new Set(source.ancestry.map(row=>join(root,row.path))))||!await selectedGraphSourceReceiptHoldsV1(root,source,ports,signal))return null;
  if(JSON.stringify(await graphStamps(directory))!==JSON.stringify(receipt.graph))return null;const proof:SelectedGraphReuseReceiptV1={version:1,toolingIdentity:ports.toolingIdentity,environmentIdentity:receipt.environmentIdentity,publisherSha256:hash(publisherBytes),graph:receipt.graph,source};validateSelectedGraphValueV1("ReuseReceiptV1",proof);return proof;
 }catch(error){signal?.throwIfAborted();if(process.env.SEMIO_NX_PROFILE_INPUTS==="1"||process.env.NX_VERBOSE_LOGGING==="true")console.log("[DEBUG] Selected graph reuse refused: "+String(error));return null;}
}

/** ✅️ Public boolean proof delegates to the same exact narrowed custody capture. */
export async function selectedGraphPublisherReceiptHoldsV1(root:string,directory:string,ports:SelectedGraphReusePortsV1,signal?:AbortSignal):Promise<boolean>{return await captureSelectedGraphReuseReceiptV1(root,directory,ports,signal)!==null;}

/** 🔐️ Hashing and promotion recheck the exact handoff instead of re-reading unrelated graph sources. */
export async function selectedGraphReuseReceiptHoldsV1(root:string,directory:string,receipt:SelectedGraphReuseReceiptV1,ports:Pick<SelectedGraphReusePortsV1,"capture"|"toolingIdentity"|"hashFile"|"environment">,signal?:AbortSignal):Promise<boolean>{
 try{validateSelectedGraphValueV1("ReuseReceiptV1",receipt);if(receipt.toolingIdentity!==ports.toolingIdentity||receipt.environmentIdentity!==graphEnvironmentIdentityV1(ports.environment)||hash(await readPublication(join(directory,destination)))!==receipt.publisherSha256||JSON.stringify(await graphStamps(directory))!==JSON.stringify(receipt.graph))return false;return await selectedGraphSourceReceiptHoldsV1(root,receipt.source,ports,signal);}catch(error){signal?.throwIfAborted();return false;}
}
