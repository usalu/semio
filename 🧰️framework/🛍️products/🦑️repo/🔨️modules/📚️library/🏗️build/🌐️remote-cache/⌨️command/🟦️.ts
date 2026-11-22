import {noticeLanguagesV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {readFile,stat} from "node:fs/promises";
import {isAbsolute,join,relative,resolve} from "node:path";
import {readRemoteCachePolicyV1,remoteCacheBaseUrlV1,remoteCacheEffectiveUploadV1,requireRemoteCacheShapeV1,type RemoteCacheContractV1,type RemoteCacheRenderingV1} from "../🟦️.ts";
import {fleetCacheFactsPathV1,remoteCacheFetchV1,resolveAuthorisedRemoteCacheV1} from "../🛰️fleet/🟦️.ts";
import {fleetGroupViewV1,fleetNetworkV1} from "../🛰️fleet/📣️publication/🟦️.ts";
import {fleetInterfaceAddressV1,readFleetMachineV1,rotateFleetIdentityV1,syncFleetIdentityV1} from "../🛰️fleet/🪪️identity/🟦️.ts";
import type {RemoteCacheValidationV1} from "../🔬️action-result/🟦️.ts";
import {collectRemoteCacheDirectoryV1,scanRemoteCacheDirectoryV1,type RemoteCacheCollectionReportV1} from "../🧹️collector/🟦️.ts";
import {RemoteCacheDirectoryBusyV1,startRemoteCacheServerV1,type RemoteCacheServerStatusV1} from "../🖥️server/🟦️.ts";
import {diffEvidenceV1,type EvidenceDiffV1} from "../🧾️diff/🟦️.ts";
import texts from "../📖️texts/🔣️.json" with {type:"json"};

/** 🔌️ The build tool's side of `remote-cache status`: which HTTP protocol it speaks, which read-only route proves reachability and credentials, and how it renders the contract. */
export interface RemoteCacheToolV1{
 readonly tool:string;
 readonly protocol:string;
 readonly probePath:string;
 render(contract:RemoteCacheContractV1,environment:Readonly<Record<string,string|undefined>>):RemoteCacheRenderingV1;
}

/** 🎛️ Carries the issued phase into the command: where the workspace is, the environment, cancellation, progress, output and the two build tools. */
export type RemoteCacheCommandContextV1=Readonly<{root:string;environment:Readonly<Record<string,string|undefined>>;signal:AbortSignal;progress:(stage:string)=>Promise<void>;write:(line:string)=>void;tools:()=>Promise<readonly RemoteCacheToolV1[]>}>;

/** 🏁️ How a command ended: the process exit code and an optional machine-readable document to print. */
export type RemoteCacheCommandResultV1=Readonly<{exitCode:number;result?:unknown}>;

/** 🗣️ The languages of all messages; there is no default among them. */
export type RemoteCacheLanguageV1="en"|"de";

type Operation="serve"|"status"|"gc"|"rotate"|"evidence";

const LANGUAGES=texts.languages as readonly RemoteCacheLanguageV1[],
 OPTIONS:Readonly<Record<Operation,Readonly<{valued:readonly string[];plain:readonly string[]}>>>={
  serve:{valued:["--port","--host","--directory","--max-bytes","--grace-seconds","--tls-certificate","--tls-key","--lang","--mirror-directory","--staleness"],plain:["--read-only","--no-mirror"]},
  status:{valued:["--directory","--lang"],plain:[]},
  gc:{valued:["--directory","--max-bytes","--grace-seconds","--current-keys","--lang"],plain:["--dry-run","--json"]},
  rotate:{valued:["--lang"],plain:[]},
  evidence:{valued:["--lang"],plain:[]}
 };

/** 💬️ Renders one catalogue message in one language. */
export function remoteCacheTextV1(language:RemoteCacheLanguageV1,key:string,variables:Readonly<Record<string,string|number>>={}):string{
 const message=(texts.messages as Readonly<Record<string,Readonly<Record<RemoteCacheLanguageV1,string>>>>)[key];
 if(!message)throw Error(`Declared remote cache message required: ${key}`);
 return message[language].replace(/\{([a-z]+)\}/gu,(_,name:string)=>{const value=variables[name];if(value===undefined)throw Error(`Remote cache message variable missing: ${name}`);return String(value);});
}

/** ☁️ Picks the output languages: the explicit request, else the one the language protocol chooses, else all of them, because no language is the default. */
export function remoteCacheLanguagesV1(requested:string|undefined,environment:Readonly<Record<string,string|undefined>>):readonly RemoteCacheLanguageV1[]{
 if(requested!==undefined){
  if(!LANGUAGES.includes(requested as RemoteCacheLanguageV1))throw Error(both("error.language",{language:requested}));
  return [requested as RemoteCacheLanguageV1];
 }
 return noticeLanguagesV1(LANGUAGES,environment);
}

function both(key:string,variables:Readonly<Record<string,string|number>>={}):string{return LANGUAGES.map(language=>remoteCacheTextV1(language,key,variables)).join("\n");}

/** 🔠️ Spells a byte count in binary units, deterministically. */
export function formatRemoteCacheBytesV1(bytes:number):string{
 if(bytes<1024)return `${bytes} B`;
 const units=["KiB","MiB","GiB","TiB"];let value=bytes,index=-1;
 do{value/=1024;index++;}while(value>=1024&&index<units.length-1);
 return `${value.toFixed(2)} ${units[index]}`;
}

/** 📝️ The closed arguments of one `remote-cache` operation: valued options by name and the plain flags that were given. */
export type RemoteCacheArgumentsV1=Readonly<{operation:Operation;values:Readonly<Record<string,string>>;flags:ReadonlySet<string>;words:readonly string[]}>;

/** 🧭️ Reads `serve|status|gc|rotate|evidence` with its options (`evidence` also takes the words `list` or `show <id>`) (`--name value` or `--name=value`); every unknown, misplaced or repeated option is refused in both languages. */
export function parseRemoteCacheArgumentsV1(args:readonly string[]):RemoteCacheArgumentsV1{
 const operation=args[0];
 if(operation!=="serve"&&operation!=="status"&&operation!=="gc"&&operation!=="rotate"&&operation!=="evidence")throw Error(both("error.syntax"));
 const allowed=OPTIONS[operation],seen=new Set<string>(),values:Record<string,string>={},flags=new Set<string>(),words:string[]=[];
 for(let index=1;index<args.length;index++){
  const argument=args[index]!,cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);
  if(operation==="evidence"&&!argument.startsWith("--")){words.push(argument);continue;}
  if(seen.has(name))throw Error(both("error.repeated",{option:name}));
  if(allowed.valued.includes(name)){
   const value=cut<0?args[++index]:argument.slice(cut+1);
   if(value===undefined||value===""||value.startsWith("--"))throw Error(both("error.value",{option:name}));
   values[name]=value;
  }else if(allowed.plain.includes(name)&&cut<0)flags.add(name);
  else throw Error(both("error.option",{operation,option:argument}));
  seen.add(name);
 }
 return Object.freeze({operation,values:Object.freeze(values),flags,words:Object.freeze(words)});
}

const whole=(name:string,value:string,minimum:number,maximum:number):number=>{
 const number=Number(value);
 if(!/^[0-9]+$/u.test(value)||!Number.isSafeInteger(number)||number<minimum||number>maximum)throw Error(both("error.number",{option:name,minimum,maximum}));
 return number;
};

const where=(root:string,path:string):string=>isAbsolute(path)?path:resolve(root,path);
const inside=(root:string,path:string):boolean=>{const relation=relative(root,path);return relation===""||!relation.startsWith("..")&&!isAbsolute(relation);};

function reporter(context:RemoteCacheCommandContextV1,languages:readonly RemoteCacheLanguageV1[]):(key:string,variables?:Readonly<Record<string,string|number>>)=>void{
 return (key,variables)=>{
  for(const language of languages){
   const resolved:Record<string,string|number>={};
   for(const [name,value] of Object.entries(variables??{}))resolved[name]=typeof value=="string"&&value.startsWith("@")?remoteCacheTextV1(language,value.slice(1)):value;
   context.write(remoteCacheTextV1(language,key,resolved));
  }
 };
}

/** 🚀️ Runs the repository's own cache server until the phase is cancelled; progress covers the store scan, listening and every eviction. */
export async function serveRemoteCacheV1(args:readonly string[],context:RemoteCacheCommandContextV1):Promise<RemoteCacheCommandResultV1>{
 const parsed=parseRemoteCacheArgumentsV1(["serve",...args]),flags=parsed.values,languages=remoteCacheLanguagesV1(flags["--lang"],context.environment),say=reporter(context,languages);
 const all=readRemoteCachePolicyV1(),policy=all.server,names=policy.environment,read=context.environment[policy.tokenEnvironment.read],write=context.environment[policy.tokenEnvironment.write];
 for(const token of[read,write])if(token!==undefined&&token!=="")requireRemoteCacheShapeV1("Token",token,"Closed remote cache server token required");
 const variable=(name:string):string|undefined=>{const value=context.environment[name];return value===undefined||value===""?undefined:value;};
 const tls=policy.tls as Readonly<{certificate:string;key:string}>|null,switched=variable(names.readOnly),readOnly=parsed.flags.has("--read-only")||(switched===undefined?policy.readOnly:switched==="true");
 if(switched!==undefined&&switched!=="true"&&switched!=="false")throw Error(both("error.option",{operation:"serve",option:`${names.readOnly}=${switched}`}));
 const certificate=flags["--tls-certificate"]??variable(names.tlsCertificate)??tls?.certificate,key=flags["--tls-key"]??variable(names.tlsKey)??tls?.key;
 if((certificate===undefined)!==(key===undefined))throw Error(both("error.tls"));
 const asked=flags["--host"]??policy.host,bare={read:read===""?undefined:read,write:write===""?undefined:write};
 const directory=where(context.root,flags["--directory"]??policy.directory),maximumBytes=flags["--max-bytes"]===undefined?policy.maximumBytes:whole("--max-bytes",flags["--max-bytes"],1048576,Number.MAX_SAFE_INTEGER);
 const machine=readFleetMachineV1(context.root),grouped=machine!==undefined&&machine.storeRoot!==undefined&&machine.originId===`${machine.group}-server`,network=grouped?await fleetNetworkV1(context.root,context.environment):undefined;
 if(grouped)await syncFleetIdentityV1({root:context.root});
 const fleetAddress=network===undefined?undefined:fleetInterfaceAddressV1(network);
 const host=asked==="fleet"?fleetAddress??(()=>{throw Error(both("error.fleet-interface",{network:network??"?"}));})():["0.0.0.0","::","[::]"].includes(asked)&&fleetAddress!==undefined?fleetAddress:asked,loopback=["127.0.0.1","localhost","::1","[::1]"].includes(host);
 const view=grouped?await fleetGroupViewV1(context.root,context.environment):undefined,fleet=grouped&&network!==undefined?{storeRoot:machine!.storeRoot!,group:machine!.group,...(view?.enforceNetwork===true?{networks:[network]}:{}),peerStores:view?.peerStores??{},root:context.root,inGroup:view?.inGroup??"sealed"}:undefined;
 const tokens=bare;
 if(!loopback&&tokens.read===undefined&&tokens.write===undefined&&fleet===undefined)throw Error(both("error.token"));
 const modeAsked=flags["--staleness"]??policy.staleness.mode;
 if(modeAsked!=="off"&&modeAsked!=="dry-run"&&modeAsked!=="enforce")throw Error(both("error.option",{operation:"serve",option:`--staleness=${modeAsked}`}));
 const chosen=variable(names.toolchains)?.split(",").map(row=>row.trim()).filter(row=>/^[0-9a-f]{64}$/u.test(row)),toolchains=chosen===undefined||chosen.length===0?undefined:new Set(chosen);
 await context.progress("remote-cache:open");say("serve.open",{directory});if(modeAsked!=="off")say("serve.staleness",{mode:modeAsked,workspace:context.root});
 const{createCasStoreV1,casRootV1}=await import("../../🗄️cas/🗃️store/🟦️.ts"),hostStore=createCasStoreV1({root:casRootV1(context.environment)});
 const server=await startRemoteCacheServerV1({
  hostStore,directory,host,...(fleet===undefined?{}:{fleet}),
  port:flags["--port"]===undefined?policy.port:whole("--port",flags["--port"],0,65535),
  maximumBytes,
  maximumEntryBytes:Math.min(policy.maximumEntryBytes,Math.floor(maximumBytes/2)),
  maximumAgeDays:policy.maximumAgeDays,
  graceMilliseconds:(flags["--grace-seconds"]===undefined?policy.graceSeconds:whole("--grace-seconds",flags["--grace-seconds"],0,604800))*1000,
  defaultInstance:all.defaults.instance,
  tokens,readOnly,
  ...(parsed.flags.has("--no-mirror")||(flags["--mirror-directory"]===undefined)?{}:{mirror:{directory:where(context.root,flags["--mirror-directory"]??join(directory,"mirror"))}}),
  tls:certificate===undefined||key===undefined?undefined:{certificate:where(context.root,certificate),key:where(context.root,key)},
  validation:policy.actionResults as RemoteCacheValidationV1,
  maximumActionResultBytes:policy.maximumActionResultBytes,
  maintenanceSeconds:policy.maintenanceSeconds,
  staleness:{mode:modeAsked,workspace:context.root,recheckSeconds:policy.staleness.recheckSeconds,manifestNames:policy.staleness.manifestNames,examples:policy.staleness.examples,...toolchains===undefined?{}:{toolchains:()=>toolchains}},
  signal:context.signal,
  onEvent:event=>{
   if(event.stage==="listening"){
    context.write(`[remote-cache] listening ${event.url}`);void context.progress("remote-cache:serving");
    say("serve.listening",{url:event.url,access:readOnly?"@serve.access.read-only":"@serve.access.read-write",transport:event.url.startsWith("https")?"@serve.transport.https":"@serve.transport.http",validation:"@serve.validation."+policy.actionResults});
    if(tokens.read===undefined&&tokens.write===undefined&&fleet===undefined)say("serve.open-access");
    say("serve.client",{url:event.url});
   }else if(event.stage==="scan")void context.progress("remote-cache:scan");
   else if(event.stage==="ready"){void context.progress("remote-cache:ready");say("serve.ready",{entries:event.entries,bytes:formatRemoteCacheBytesV1(event.bytes)});}
   else if(event.stage==="evict")say("serve.evicted",{entries:event.entries,bytes:formatRemoteCacheBytesV1(event.bytes)});
   else if(event.stage==="reclaim")say("serve.reclaimed",{entries:event.entries,bytes:formatRemoteCacheBytesV1(event.bytes)});
   else if(event.stage==="stopped"){context.write("[remote-cache] stopped");say("serve.stopped");}
  }
 }).catch(error=>{throw error instanceof RemoteCacheDirectoryBusyV1?Error(both("error.busy",{directory:error.directory})):error;});
 await server.stopped;
 return {exitCode:0};
}

type Probe={protocol:RemoteCacheToolV1["protocol"];reachable:boolean;authorized:boolean;status:number|null;milliseconds:number};

async function probeRemoteCacheV1(contract:RemoteCacheContractV1,tool:RemoteCacheToolV1,call:typeof fetch,signal:AbortSignal):Promise<Probe>{
 const started=performance.now(),elapsed=()=>Math.round(performance.now()-started),combined=AbortSignal.any([signal,AbortSignal.timeout(contract.timeoutSeconds*1000)]);
 try{
  const response=await call(remoteCacheBaseUrlV1(contract)+tool.probePath,{method:"GET",signal:combined});
  await response.body?.cancel();
  return {protocol:tool.protocol,reachable:true,authorized:response.status===200||response.status===404,status:response.status,milliseconds:elapsed()};
 }catch(error){
  if(signal.aborted)throw error;
  return {protocol:tool.protocol,reachable:false,authorized:false,status:null,milliseconds:elapsed()};
 }
}

type ServerFault=Readonly<{reason:"unreachable"|"denied"|"unsupported"|"failed"|"malformed";status:number|null;message:string}>;
type ServerAnswer=Readonly<{server:RemoteCacheServerStatusV1;fault:null}>|Readonly<{server:null;fault:ServerFault}>;

/** 🩺️ Asks the running server for its own numbers through the signed fetch (the same one every cache client uses, so the request carries this machine's fleet signature or token) and names why there is no answer: unreachable, denied (401, 403), unsupported (404, 405), failed (other statuses) or malformed (not JSON or not a server status); the message comes in both languages. Cancelling the phase throws. */
async function askServerV1(contract:RemoteCacheContractV1,call:typeof fetch,signal:AbortSignal):Promise<ServerAnswer>{
 const faulted=(reason:ServerFault["reason"],status:number|null):ServerAnswer=>({server:null,fault:{reason,status,message:both(`status.fault.${reason}`,{status:status??0})}});
 let response:Response;
 try{response=await call(`${contract.endpoint}/status`,{signal:AbortSignal.any([signal,AbortSignal.timeout(contract.timeoutSeconds*1000)])});}
 catch(error){if(signal.aborted)throw error;return faulted("unreachable",null);}
 try{
  if(response.status!==200){await response.body?.cancel();return faulted(response.status===401||response.status===403?"denied":response.status===404||response.status===405?"unsupported":"failed",response.status);}
  const document=await response.json();
  requireRemoteCacheShapeV1("ServerStatus",document,"Closed remote cache server status required");
  return {server:document as RemoteCacheServerStatusV1,fault:null};
 }catch(error){
  if(signal.aborted)throw error;
  return faulted("malformed",response.status);
 }
}

/** 📋️ Prints where the endpoint and the token came from (environment or fleet facts), the resolved contract, how both build tools render it, whether the endpoint answers and accepts the token on both protocols, what the running server itself reports (entries and bytes as the server counts them, never a guessed local directory) and, when asked for a directory, what that local store holds; secrets never appear, only whether a token is present and where it came from. */
export async function remoteCacheStatusV1(args:readonly string[],context:RemoteCacheCommandContextV1):Promise<unknown>{
 const parsed=parseRemoteCacheArgumentsV1(["status",...args]),all=readRemoteCachePolicyV1(),tools=await context.tools(),directory=where(context.root,parsed.values["--directory"]??all.server.directory);
 await context.progress("remote-cache:scan");
 const root=join(directory,"v1"),present=await stat(root).then(info=>info.isDirectory(),()=>false);
 let store=null;
 if(present){
  const index=await scanRemoteCacheDirectoryV1(root,{signal:context.signal,onProgress:()=>{void context.progress("remote-cache:scan");}});
  let bytes=0;for(const row of index.values())bytes+=row.size;
  store={directory,entries:index.size,bytes,maximumBytes:all.server.maximumBytes};
 }
 const found=await resolveAuthorisedRemoteCacheV1({...context.environment,SEMIO_WORKSPACE_ROOT:context.environment.SEMIO_WORKSPACE_ROOT??context.root},{root:context.root}),resolved=found.resolution;
 if(resolved===undefined)return {version:1,configured:false,contract:null,effectiveUpload:false,token:null,renderings:null,probes:null,server:null,serverFault:null,store,resolution:null,remote:null};
 await context.progress("remote-cache:probe");
 const contract=resolved.contract,token=resolved.token,probes:Probe[]=[],renderings:Record<string,RemoteCacheRenderingV1>={},call=remoteCacheFetchV1(resolved.environment,{root:context.root});
 for(const tool of tools){renderings[tool.tool]=tool.render(contract,resolved.environment);probes.push(await probeRemoteCacheV1(contract,tool,call,context.signal));}
 const {server,fault:serverFault}=await askServerV1(contract,call,context.signal),sum=(field:"hits"|"misses"):number|null=>server===null?null:Object.values(server.counters).reduce((total,row)=>total+row[field],0);
 const status={version:1,configured:true,contract,effectiveUpload:remoteCacheEffectiveUploadV1(contract),token:{environment:contract.auth.tokenEnvironment,present:token!==undefined},renderings,probes,server,serverFault,store,
  resolution:{endpointFrom:resolved.endpointFrom,tokenFrom:resolved.tokenFrom,factsGeneration:resolved.generation,factsState:found.refresh?.state??"none"},
  remote:{endpoint:contract.endpoint,reachable:probes.some(probe=>probe.reachable),authorised:probes.length>0&&probes.every(probe=>probe.authorized),entries:server?.entries??null,bytes:server?.bytes??null,maximumBytes:server?.maximumBytes??null,hits:sum("hits"),misses:sum("misses"),fleetGeneration:resolved.generation}};
 requireRemoteCacheShapeV1("Status",status,"Closed remote cache status required");return status;
}

/** 🔄️ Rotates the signing keys of this machine: new keys for every role (the origin key stays, so servers keep recognising the machine), published to the machine's own directory of the group store at once. Servers pick the new registration up within seconds; requests signed with an old key are refused from then on. Run it on any machine whose keys may have leaked; to cut a machine off for good, retire its origin instead. */
export async function rotateRemoteCacheV1(args:readonly string[],context:RemoteCacheCommandContextV1):Promise<RemoteCacheCommandResultV1>{
 const parsed=parseRemoteCacheArgumentsV1(["rotate",...args]),languages=remoteCacheLanguagesV1(parsed.values["--lang"],context.environment),say=reporter(context,languages),machine=readFleetMachineV1(context.root);
 if(machine===undefined){say("rotate.not-joined");return {exitCode:1};}
 const registration=await rotateFleetIdentityV1({root:context.root,originId:machine.originId,group:machine.group,roles:["cache-read","cache-write","submit","worker","admin"]});
 const synced=await syncFleetIdentityV1({root:context.root});
 say("rotate.done",{sequence:registration.sequence,origin:machine.originId,published:synced.published?"@rotate.published":"@rotate.local"});
 return {exitCode:0};
}

/** 🪦️ The reclaim port of a collection from outside: the blobs no remaining entry references go to the host store, which applies its own protections. */
async function hostReclaimV1(environment:Readonly<Record<string,string|undefined>>):Promise<(hashes:readonly string[],dryRun:boolean)=>Promise<{entries:number;bytes:number}>>{
 const{createCasStoreV1,casRootV1}=await import("../../🗄️cas/🗃️store/🟦️.ts"),{reclaimCasObjectsV1}=await import("../../🗄️cas/🪦️reclaim/🟦️.ts"),host=createCasStoreV1({root:casRootV1(environment)});
 return async(hashes,dryRun)=>{const freed=await reclaimCasObjectsV1(host,hashes,{dryRun});return {entries:freed.entries,bytes:freed.bytes};};
}

/** 🧪️ The blob presence port of a collection from outside: the blobs of action results live in the host store, so a result is dangling only when the host store lacks one of them. */
async function hostCollectionBlobsV1(environment:Readonly<Record<string,string|undefined>>):Promise<Pick<import("../🧹️collector/🟦️.ts").RemoteCacheCollectionRequestV1,"hasBlobs"|"readTree">>{
 const{createCasStoreV1,casRootV1}=await import("../../🗄️cas/🗃️store/🟦️.ts"),{readRemoteCacheTreeBytesV1}=await import("../🏷️retention/🟦️.ts"),host=createCasStoreV1({root:casRootV1(environment)});
 return{hasBlobs:hashes=>host.has(hashes),readTree:async(_namespace,tree)=>{const leaf=await host.get(tree.hash);return leaf?.size===tree.size?readRemoteCacheTreeBytesV1(leaf.path,tree.size):undefined;}};
}

/** 🧹️ Collects the store directory like a running server would, from outside: removes unusable action results, entries idle for too long and the least recently used entries beyond the bound, under the collector lock; `--dry-run` only plans. Progress is published and cancelling stops at the next checkpoint. */
export async function collectRemoteCacheV1(args:readonly string[],context:RemoteCacheCommandContextV1):Promise<RemoteCacheCommandResultV1>{
 const parsed=parseRemoteCacheArgumentsV1(["gc",...args]),languages=remoteCacheLanguagesV1(parsed.values["--lang"],context.environment),say=reporter(context,languages),policy=readRemoteCachePolicyV1().server;
 const directory=where(context.root,parsed.values["--directory"]??policy.directory),maximumBytes=parsed.values["--max-bytes"]===undefined?policy.maximumBytes:whole("--max-bytes",parsed.values["--max-bytes"],1048576,Number.MAX_SAFE_INTEGER),dryRun=parsed.flags.has("--dry-run"),json=parsed.flags.has("--json");
 const talk=json?()=>{}:say;
 if(parsed.values["--current-keys"]!==undefined){
  const{auditRemoteCacheDirectoryV1}=await import("../🏷️retention/🟦️.ts"),{readFile}=await import("node:fs/promises"),{requireSlotShapeV1,slotTextV1}=await import("../../🪢️slots/🟦️.ts");
  const keys=new Set((await readFile(where(context.root,parsed.values["--current-keys"]),"utf8")).split("\n").map(line=>line.trim()).filter(line=>line!==""&&!line.startsWith("#")));
  const audit=await auditRemoteCacheDirectoryV1(join(directory,"v1"),keys,{graceMilliseconds:(parsed.values["--grace-seconds"]===undefined?policy.graceSeconds:whole("--grace-seconds",parsed.values["--grace-seconds"],0,604800))*1000,signal:context.signal});
  requireSlotShapeV1("AuditReport",audit,"Closed cache audit report required");
  if(!json)for(const language of languages){context.write(slotTextV1("audit.title",language,{cache:directory,current:audit.current,grace:audit.grace,stray:audit.stray.length,unslotted:audit.unslotted}));for(const key of audit.stray)context.write(slotTextV1("audit.stray",language,{key}));}
  return {exitCode:audit.stray.length>0?1:0,...(json?{result:audit}:{})};
 }
 if(!await stat(join(directory,"v1")).then(info=>info.isDirectory(),()=>false)){
  talk("gc.absent",{directory});
  const absent={version:1,dryRun,directory,present:false};if(json)requireRemoteCacheShapeV1("GcResult",absent,"Closed remote cache gc result required");
  return {exitCode:0,...(json?{result:absent}:{})};
 }
 talk("gc.start",{directory,bytes:formatRemoteCacheBytesV1(maximumBytes),days:policy.maximumAgeDays,mode:dryRun?"@gc.mode.plan":"@gc.mode.apply"});
 let report;
 try{
  report=await collectRemoteCacheDirectoryV1({root:join(directory,"v1"),maximumBytes,maximumAgeMilliseconds:policy.maximumAgeDays*86400000,dryRun,verifyResults:policy.actionResults==="complete",graceMilliseconds:(parsed.values["--grace-seconds"]===undefined?policy.graceSeconds:whole("--grace-seconds",parsed.values["--grace-seconds"],0,604800))*1000,reclaim:await hostReclaimV1(context.environment),...await hostCollectionBlobsV1(context.environment),signal:context.signal,onProgress:stage=>{void context.progress(`remote-cache:${stage}`);}});
 }catch(error){
  if(context.signal.aborted){talk("gc.cancelled");return {exitCode:130};}
  throw error;
 }
 if(report.locked){talk("gc.locked",{lock:report.lock});return {exitCode:1};}
 const value=report.value;
 talk(dryRun?"gc.summary.plan":"gc.summary.apply",{entries:value.removed.entries,total:value.scanned.entries,bytes:formatRemoteCacheBytesV1(value.removed.bytes),stale:value.removed.stale,age:value.removed.age,budget:value.removed.budget,dangling:value.removed.dangling,remaining:value.remaining.entries,left:formatRemoteCacheBytesV1(value.remaining.bytes),blobs:value.reclaimed.entries,freed:formatRemoteCacheBytesV1(value.reclaimed.bytes)});
 if(value.deferred)talk("gc.deferred");
 const document={directory,...value};if(json)requireRemoteCacheShapeV1("GcResult",document,"Closed remote cache gc result required");
 return {exitCode:0,...(json?{result:document}:{})};
}

/** 🧾️ Collects the workspace's store for `repo cache gc` and answers one language-neutral line for its report; the ledger delegates its remote cache area here, so both commands apply the same bounds. */
export async function collectRemoteCacheForLedgerV1(workspace:string,dryRun:boolean,signal:AbortSignal):Promise<string>{
 const policy=readRemoteCachePolicyV1().server,root=join(where(workspace,policy.directory),"v1");
 if(!await stat(root).then(info=>info.isDirectory(),()=>false))return "absent";
 const outcome=await collectRemoteCacheDirectoryV1({root,maximumBytes:policy.maximumBytes,maximumAgeMilliseconds:policy.maximumAgeDays*86400000,dryRun,verifyResults:policy.actionResults==="complete",graceMilliseconds:policy.graceSeconds*1000,reclaim:await hostReclaimV1(process.env),...await hostCollectionBlobsV1(process.env),signal});
 if(outcome.locked)return `busy: ${outcome.lock}`;
 const report:RemoteCacheCollectionReportV1=outcome.value;
 return `${dryRun?"would remove":"removed"} ${report.removed.entries} of ${report.scanned.entries} entries (${formatRemoteCacheBytesV1(report.removed.bytes)})`;
}

/** 🗂️ Reads the evidence the running cache server keeps of the writes it refused as divergences (`repo remote-cache evidence list`: the entries, newest first; `show <id>`: one entry with, for every output that differs, both digests and sizes, the first differing byte, the number of differing bytes and the lines only one side has, Brotli and JSON unpacked). The entries are fetched with the same signed client as `status`; a server that does not answer is a fault, never a hang. */
export async function remoteCacheEvidenceV1(args:readonly string[],context:RemoteCacheCommandContextV1):Promise<unknown>{
 const parsed=parseRemoteCacheArgumentsV1(["evidence",...args]),words=parsed.words,action=words[0]??"list";
 if(action!=="list"&&action!=="show"||action==="list"&&words.length>1||action==="show"&&words.length!==2)throw Error(both("evidence.syntax"));
 const found=await resolveAuthorisedRemoteCacheV1({...context.environment,SEMIO_WORKSPACE_ROOT:context.environment.SEMIO_WORKSPACE_ROOT??context.root},{root:context.root}),resolved=found.resolution;
 if(resolved===undefined)throw Error(both("evidence.off"));
 const call=remoteCacheFetchV1(resolved.environment,{root:context.root}),endpoint=resolved.contract.endpoint,limit=resolved.contract.timeoutSeconds*1000;
 const get=async(path:string):Promise<Response>=>{
  let response:Response;
  try{response=await call(endpoint+path,{signal:AbortSignal.any([context.signal,AbortSignal.timeout(limit)])});}catch(error){if(context.signal.aborted)throw error;throw Error(both("evidence.fault",{status:0}));}
  if(response.status!==200){await response.body?.cancel();throw Error(both("evidence.fault",{status:response.status}));}
  return response;
 };
 await context.progress("remote-cache:evidence");
 const listed=(await (await get("/evidence")).json()) as {evidence:readonly EvidenceRowV1[]};
 if(action==="list"){const result={version:1,action:"list",evidence:listed.evidence};requireRemoteCacheShapeV1("EvidenceList",result,"Closed remote cache evidence list required");return result;}
 const matches=listed.evidence.filter(row=>row.id===words[1]||row.id.startsWith(words[1]!)),row=matches.find(candidate=>candidate.id===words[1])??(matches.length===1?matches[0]:undefined);
 if(row===undefined)throw Error(both("evidence.unknown",{id:words[1]!}));
 const diffs:EvidenceDiffV1[]=[];
 for(const path of row.differing){
  const sides=row.files.filter(file=>file.path===path),pick=async(side:"kept"|"refused")=>{
   const file=sides.find(candidate=>candidate.side===side);
   if(file===undefined)return undefined;
   const bytes=file.stored?new Uint8Array(await (await get(`/evidence/${row.id}/${file.name}`)).arrayBuffer()):undefined;
   return{sha256:file.sha256,bytes,size:file.bytes};
  };
  diffs.push(diffEvidenceV1(path,await pick("kept"),await pick("refused")));
 }
 const result={version:1,action:"show",evidence:row,diffs};requireRemoteCacheShapeV1("EvidenceShow",result,"Closed remote cache evidence required");return result;
}

type EvidenceRowV1=Readonly<{id:string;atMs:number;sort:string;namespace:string;key:string;keptOrigin:string;refusedOrigin:string;differing:readonly string[];files:readonly Readonly<{name:string;side:"kept"|"refused";path:string;sha256:string;bytes:number;stored:boolean}>[]}>;

/** 🎬️ Routes `remote-cache serve|status|gc|rotate|evidence`; `args` are the words after `remote-cache`. */
export async function runRemoteCacheCommandV1(args:readonly string[],context:RemoteCacheCommandContextV1):Promise<RemoteCacheCommandResultV1>{
 const parsed=parseRemoteCacheArgumentsV1(args);
 if(parsed.operation==="serve")return serveRemoteCacheV1(args.slice(1),context);
 if(parsed.operation==="gc")return collectRemoteCacheV1(args.slice(1),context);
 if(parsed.operation==="rotate")return rotateRemoteCacheV1(args.slice(1),context);
 if(parsed.operation==="evidence")return {exitCode:0,result:await remoteCacheEvidenceV1(args.slice(1),context)};
 return {exitCode:0,result:await remoteCacheStatusV1(args.slice(1),context)};
}
