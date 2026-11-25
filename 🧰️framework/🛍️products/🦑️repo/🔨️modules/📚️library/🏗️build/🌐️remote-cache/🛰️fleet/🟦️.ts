import{machinePathV1}from"../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {readFileSync} from "node:fs";
import {mkdir,readFile,rename,rm,writeFile} from "node:fs/promises";
import {createConnection} from "node:net";
import {randomUUID} from "node:crypto";
import {dirname,isAbsolute,join,resolve} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {readRemoteCachePolicyV1,readRemoteCacheTokenV1,resolveRemoteCacheContractV1,type RemoteCacheContractV1,type RemoteCachePolicyV1} from "../🟦️.ts";
import {fleetSignedFetchV1,loadFleetSignerV1,readFleetOriginKeyV1,syncFleetIdentityV1} from "./🪪️identity/🟦️.ts";
import {createSealedFetchV1} from "../🔐️sealed/🟦️.ts";

/** 🛰️ The facts of one fleet group, shaped by `🧬️schema/🔣️.json` (`Facts`): where its cache answers, under which namespace, with which fleet-scope credential, and (reserved) where its execution scheduler listens. */
export type FleetCacheFactsV1=Readonly<{version:1;group:string;generation:number;publishedAtMs:number;publisher:Readonly<{directory:string;port:number;host:string}>;cache:Readonly<{endpoints:readonly string[];instance:string;access:"read-write";protocols:readonly ["bazel-http","nx-self-hosted"];protocolVersion:1;transport?:"sealed"|"trusted-lan"}>;auth:Readonly<{scheme:"semio-fleet-v1";roles:readonly ["cache-read","cache-write","submit","worker","admin"]}>;execution?:Readonly<{endpoint:string;protocolVersion:number}>}>;

/** 🗃️ The last known copy of the facts on one machine, shaped by `LocalCopy`: when it was fetched and last checked, and which endpoint answered. */
export type FleetCacheLocalCopyV1=Readonly<{version:1;fetchedAtMs:number;checkedAtMs:number;selectedEndpoint:string|null;facts:FleetCacheFactsV1}>;

/** 🧾️ How one refresh ended: the copy is `fresh` (checked within the TTL, store not read), `checked` (store read, nothing new), `updated` (new facts stored), `stale` (store unreadable or slow, last known copy kept) or `absent` (nothing known). */
export type FleetCacheRefreshV1=Readonly<{state:"fresh"|"checked"|"updated"|"stale"|"absent";copy:FleetCacheLocalCopyV1|undefined;error?:string}>;

/** 🧮️ The resolved remote cache of one process: the contract, the token (never printed), the input environment completed with the resolved values, and where endpoint and token came from. */
export type RemoteCacheResolutionV1=Readonly<{contract:RemoteCacheContractV1;token:string|undefined;environment:Readonly<Record<string,string|undefined>>;endpointFrom:"environment"|"fleet-facts";tokenFrom:"environment"|"fleet-identity"|"none";generation:number|null;instanceFrom:"environment"|"fleet-facts"|"policy";instanceRefused:string|null}>;

/** ⚙️ What `resolveRemoteCacheV1` may be told: the worktree root whose `.cache/f` holds the local copy, a ready-made copy (tests), and the policy. */
export type RemoteCacheResolutionOptionsV1=Readonly<{root?:string;copy?:FleetCacheLocalCopyV1;policy?:RemoteCachePolicyV1}>;

/** ⏱️ What `refreshFleetCacheFactsV1` may be told. */
export type FleetCacheRefreshOptionsV1=Readonly<{root:string;force?:boolean;ttlMilliseconds?:number;timeoutMilliseconds?:number;probeMilliseconds?:number;now?:number;fetch?:typeof fetch}>;

const TTL=30000,TIMEOUT=1500,PROBE=700,KEPT=7*86400000,pending=new Map<string,Promise<FleetCacheRefreshV1>>();

/** 🛡️ Validates one value against a named definition of the fleet schema and names the first problem. */
export function requireFleetCacheShapeV1(definition:"Facts"|"LocalCopy",value:unknown,message:string):void{
 const errors=validateJsonSchemaSubset({$ref:`#/definitions/${definition}`},value,schema);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

/** 📍️ Where a group store keeps the facts: `<store>/v1/fleet/cache.json`. */
export function fleetCacheFactsPathV1(storeRoot:string):string{return join(storeRoot,"v1","fleet","cache.json");}

/** 📍️ Where a machine keeps its local copy: `<worktree>/.cache/f/cache-facts.json`. */
export function fleetCacheLocalPathV1(root:string):string{return machinePathV1(root,".cache","f","cache-facts.json");}

/** 📖️ Reads and validates the text of a facts file. */
export function parseFleetCacheFactsV1(text:string):FleetCacheFactsV1{
 const value=JSON.parse(text) as unknown;requireFleetCacheShapeV1("Facts",value,"Closed fleet cache facts required");return value as FleetCacheFactsV1;
}

/** 🏠️ The worktree root of this process, `SEMIO_WORKSPACE_ROOT` (every `repo` build sets it); `undefined` without it, so a test or a tool outside a build never reads the facts of the machine it happens to run on. */
export function fleetCacheRootV1(environment:Readonly<Record<string,string|undefined>>=process.env):string|undefined{
 const named=environment.SEMIO_WORKSPACE_ROOT;
 return named!==undefined&&named!==""&&isAbsolute(named)?named:undefined;
}

/** 🗂️ The store root that `.cache/f/machine.json` of a worktree names, or `undefined`. */
export function fleetStoreRootV1(root:string):string|undefined{
 try{const value=(JSON.parse(readFileSync(machinePathV1(root,".cache","f","machine.json"),"utf8")) as {storeRoot?:unknown}).storeRoot;return typeof value==="string"&&value!==""?value:undefined;}catch{return undefined;}
}

/** 🗃️ Reads the local copy synchronously and never touches the store; `undefined` when it is absent, invalid or older than seven days. */
export function readLocalFleetCacheFactsV1(root:string,now:number=Date.now()):FleetCacheLocalCopyV1|undefined{
 try{
  const value=JSON.parse(readFileSync(fleetCacheLocalPathV1(root),"utf8")) as FleetCacheLocalCopyV1;
  requireFleetCacheShapeV1("LocalCopy",value,"Closed fleet cache copy required");
  return now-value.fetchedAtMs>KEPT?undefined:value;
 }catch{return undefined;}
}

const filled=(environment:Readonly<Record<string,string|undefined>>,name:string):string|undefined=>{const value=environment[name];return value===undefined||value===""?undefined:value;};
const origin=(endpoint:string):string=>{try{return new URL(endpoint).origin;}catch{return endpoint;}};

/** 🧷️ The name of the variable that makes an endpoint pointer of the environment win over healthy fleet facts: set to any value, `SEMIO_REMOTE_CACHE_ENDPOINT` is the operator's explicit choice. */
export const FLEET_POINTER_OVERRIDE_ENVIRONMENT_V1="SEMIO_REMOTE_CACHE_ENDPOINT_OVERRIDE";

/** 🪢️ Whether the endpoint pointer of the environment is a leftover that the fleet facts supersede: facts are present and healthy (they name at least one endpoint), the pointer names none of their endpoints, and the operator did not set the override variable. Machines provisioned before the facts existed carry such a pointer in their profile files; the facts are the source of truth. */
export function pointerSupersededV1(environment:Readonly<Record<string,string|undefined>>,endpointName:string,facts:Readonly<{cache:Readonly<{endpoints:readonly string[]}>}>|undefined):boolean{
 const pointer=filled(environment,endpointName);
 return pointer!==undefined&&facts!==undefined&&facts.cache.endpoints.length>0&&filled(environment,FLEET_POINTER_OVERRIDE_ENVIRONMENT_V1)===undefined&&!facts.cache.endpoints.some(endpoint=>origin(endpoint)===origin(pointer));
}

/** 🪢️ The instance (namespace) of the environment that the healthy fleet facts contradict: it is a leftover of an older provisioning and the facts, the single source of truth that the cache server publishes, win, unless the operator set the override variable; `undefined` when nothing contradicts. A leftover wrote entries into a namespace that no reader of the fleet looks into. */
export function instanceRefusedV1(environment:Readonly<Record<string,string|undefined>>,instanceName:string,facts:Readonly<{cache:Readonly<{instance:string}>}>|undefined):string|undefined{
 const value=filled(environment,instanceName);
 return value!==undefined&&facts!==undefined&&value!==facts.cache.instance&&filled(environment,FLEET_POINTER_OVERRIDE_ENVIRONMENT_V1)===undefined?value:undefined;
}

/** 🧭️ THE resolver of every client of the remote cache: the environment first, field by field, then the fleet facts of the local copy. The credential of a joined machine is its own signing identity (`tokenFrom: fleet-identity`, no secret in any variable or file that others read) and is only ever used for an endpoint the facts name, so an explicitly foreign endpoint never sees even a signature; an operator token from the environment (`tokenFrom: environment`) is sent as before. Synchronous and local: a stalled store cannot stall a caller; keep the copy fresh with `refreshFleetCacheFactsV1`. `undefined` means the remote cache is off. */
export function resolveRemoteCacheV1(environment:Readonly<Record<string,string|undefined>>,options:RemoteCacheResolutionOptionsV1={}):RemoteCacheResolutionV1|undefined{
 const policy=options.policy??readRemoteCachePolicyV1(),names=policy.environment,root=options.root??fleetCacheRootV1(environment),copy=options.copy??(root===undefined?undefined:readLocalFleetCacheFactsV1(root)),facts=copy?.facts;
 const overlay:Record<string,string|undefined>={...environment},pointer=filled(environment,names.endpoint),superseded=pointerSupersededV1(environment,names.endpoint,facts),explicit=superseded?undefined:pointer,tokenName=filled(environment,names.tokenEnvironment)??policy.defaults.tokenEnvironment,own=filled(environment,tokenName);
 let endpointFrom:RemoteCacheResolutionV1["endpointFrom"]="environment",named=false;
 const instanceRefused=instanceRefusedV1(environment,names.instance,facts),instanceFrom:RemoteCacheResolutionV1["instanceFrom"]=facts!==undefined&&copy!==undefined&&(filled(environment,names.instance)===undefined||instanceRefused!==undefined)?"fleet-facts":filled(environment,names.instance)===undefined?"policy":"environment";
 if(facts!==undefined&&copy!==undefined){
  if(explicit===undefined){overlay[names.endpoint]=copy.selectedEndpoint??facts.cache.endpoints[0];endpointFrom="fleet-facts";}
  if(filled(environment,names.instance)===undefined||instanceRefused!==undefined)overlay[names.instance]=facts.cache.instance;
  if(filled(environment,names.access)===undefined)overlay[names.access]=facts.cache.access;
  named=facts.cache.endpoints.some(endpoint=>origin(endpoint)===origin(overlay[names.endpoint]??""));
 }
 const contract=resolveRemoteCacheContractV1(overlay,policy);
 if(contract===undefined)return undefined;
 const token=readRemoteCacheTokenV1(contract,overlay),tokenFrom:RemoteCacheResolutionV1["tokenFrom"]=token!==undefined?"environment":named&&root!==undefined&&loadFleetSignerV1(root,"cache-read")!==undefined?"fleet-identity":"none";
 if(tokenFrom==="fleet-identity")overlay.SEMIO_REMOTE_CACHE_SIGNED="1";
 return Object.freeze({contract,token,environment:Object.freeze(overlay),endpointFrom,tokenFrom,generation:facts?.generation??null,instanceFrom,instanceRefused:instanceRefused??null});
}

const copies=new Map<string,{at:number;copy:FleetCacheLocalCopyV1|undefined}>();
const facted=(root:string):FleetCacheLocalCopyV1|undefined=>{const known=copies.get(root),now=Date.now();if(known!==undefined&&now-known.at<5000)return known.copy;const copy=readLocalFleetCacheFactsV1(root);copies.set(root,{at:now,copy});return copy;};

/** 🪝️ The `fetch` every client of the remote cache uses: it adds what the resolved credential needs. An operator token of the environment becomes a bearer header. Otherwise, for an origin that the fleet facts of this machine name (the cache endpoints and the scheduler), every request is signed with the machine's role key (`cache-read` for reads, `cache-write` for writes, `submit` or `worker` as `roleOf` says), so no secret is ever sent; any other URL is requested as it is. */
export function remoteCacheFetchV1(environment:Readonly<Record<string,string|undefined>>,options:Readonly<{root?:string;base?:typeof fetch;audience?:"cache"|"execution";roleOf?:(method:string,pathname:string)=>import("./🪪️identity/🟦️.ts").FleetRoleV1}>={}):typeof fetch{
 const policy=readRemoteCachePolicyV1(),root=options.root??fleetCacheRootV1(environment),base=options.base??fetch,tokenName=filled(environment,policy.environment.tokenEnvironment)??policy.defaults.tokenEnvironment,token=filled(environment,tokenName),audience=options.audience??"cache";
 const signed=root===undefined?undefined:fleetSignedFetchV1({root,audience,...(options.roleOf===undefined?{}:{roleOf:options.roleOf}),fetch:base});
 const named=(url:string):boolean=>{if(root===undefined)return false;const facts=facted(root)?.facts;if(facts===undefined)return false;const where=origin(url);return facts.cache.endpoints.some(endpoint=>origin(endpoint)===where)||(facts.execution!==undefined&&origin(facts.execution.endpoint)===where)||(facts.execution!==undefined&&sameHost(facts.execution.endpoint,url));};
 const pointer=filled(environment,policy.environment.endpoint),explicitNow=():string|undefined=>pointer===undefined||pointerSupersededV1(environment,policy.environment.endpoint,root===undefined?undefined:facted(root)?.facts)?undefined:pointer;
 return ((input:RequestInfo|URL,init?:RequestInit)=>{
  const url=typeof input==="string"||input instanceof URL?String(input):input.url,isNamed=named(url);
  const facts=root===undefined||!isNamed?undefined:facted(root)?.facts;
  if(facts!==undefined&&facts.cache.transport==="sealed"&&facts.cache.endpoints.some(endpoint=>origin(endpoint)===origin(url))&&root!==undefined&&loadFleetSignerV1(root,"cache-read")!==undefined)return sealedSignedV1(root,facts.group,audience,base,options.roleOf)(input,init);
  if(isNamed&&signed!==undefined&&root!==undefined&&loadFleetSignerV1(root,"cache-read")!==undefined)return signed(input,init);
  if(token!==undefined&&(isNamed||(explicit=>explicit!==undefined&&origin(url)===origin(explicit))(explicitNow()))){const headers=new Headers(init?.headers);headers.set("authorization",`Bearer ${token}`);return base(input,{...init,headers});}
  return base(input,init);
 }) as typeof fetch;
}
const sealedFetchers=new Map<string,typeof fetch>();
const sealedSignedV1=(root:string,group:string,audience:"cache"|"execution",base:typeof fetch,roleOf:Parameters<typeof fleetSignedFetchV1>[0]["roleOf"]):typeof fetch=>{
 const key=`${root}\0${group}\0${audience}`;
 let found=sealedFetchers.get(key);
 if(found===undefined){
  let held:{key:string;at:number}|undefined;
   const serverKey=async(_url:URL,signal?:AbortSignal):Promise<string|undefined>=>{
    signal?.throwIfAborted();
   if(held!==undefined&&Date.now()-held.at<60000)return held.key;
    const store=fleetStoreRootV1(root),read=store===undefined?undefined:await readFleetOriginKeyV1([store],`${group}-server`,signal);
   if(read!==undefined)held={key:read,at:Date.now()};
   return read;
  };
  found=fleetSignedFetchV1({root,audience,...(roleOf===undefined?{}:{roleOf}),fetch:createSealedFetchV1({group,serverKey,fetch:base})});
  if(sealedFetchers.size>64)sealedFetchers.clear();
  sealedFetchers.set(key,found);
 }
 return found;
};
const sameHost=(left:string,right:string):boolean=>{try{return new URL(left).hostname===new URL(right).hostname;}catch{return false;}};

/** 🔭️ The execution scheduler of the group from the local copy, or `undefined` while none is published. */
export function readFleetExecutionV1(environment:Readonly<Record<string,string|undefined>>=process.env,options:Readonly<{root?:string;copy?:FleetCacheLocalCopyV1}>={}):Readonly<{endpoint:string;protocolVersion:number}>|undefined{
 const root=options.root??fleetCacheRootV1(environment),copy=options.copy??(root===undefined?undefined:readLocalFleetCacheFactsV1(root));
 return copy?.facts.execution;
}

async function bounded<T>(work:Promise<T>,milliseconds:number,what:string):Promise<T>{
 let timer:ReturnType<typeof setTimeout>|undefined;
 try{return await Promise.race([work,new Promise<never>((_,reject)=>{timer=setTimeout(()=>reject(Error(`${what} took longer than ${milliseconds} ms`)),milliseconds);})]);}
 finally{if(timer)clearTimeout(timer);work.catch(()=>{});}
}

async function answers(endpoint:string,milliseconds:number,ask:typeof fetch):Promise<boolean>{
 try{const response=await ask(`${endpoint}/healthz`,{signal:AbortSignal.timeout(milliseconds)});await response.body?.cancel();return response.status===200;}catch{return false;}
}

async function select(endpoints:readonly string[],preferred:string|null,milliseconds:number,ask:typeof fetch):Promise<string|null>{
 for(const endpoint of preferred!==null&&endpoints.includes(preferred)?[preferred,...endpoints.filter(candidate=>candidate!==preferred)]:endpoints)if(await answers(endpoint,milliseconds,ask))return endpoint;
 return null;
}

/** 🪟️ Renames a staged file over its target; on Windows a reader that holds the target open (every compiler wrapper reads this copy) makes the rename fail with `EPERM`, `EBUSY` or `EACCES` for a moment, so it is retried a few times with a short pause before the error counts. */
export async function renameOverReadersV1(from:string,to:string,platform:NodeJS.Platform=process.platform,pause:(milliseconds:number)=>Promise<void>=milliseconds=>new Promise(done=>setTimeout(done,milliseconds)),rename_:typeof rename=rename):Promise<number>{
 for(let attempt=0;;attempt++){
  try{await rename_(from,to);return attempt;}
  catch(error){const code=(error as NodeJS.ErrnoException).code;if(platform!=="win32"||attempt>=5||(code!=="EPERM"&&code!=="EBUSY"&&code!=="EACCES"))throw error;await pause(20*(attempt+1));}
 }
}

async function store(root:string,copy:FleetCacheLocalCopyV1):Promise<void>{
 const path=fleetCacheLocalPathV1(root),temporary=`${path}.${process.pid}-${randomUUID()}.tmp`;
 await mkdir(dirname(path),{recursive:true});
 try{await writeFile(temporary,JSON.stringify(copy)+"\n",{mode:0o600});await renameOverReadersV1(temporary,path);}catch(error){await rm(temporary,{force:true}).catch(()=>{});throw error;}
}

const FLEET_STORE_MOUNTS=new Set(["/Volumes/semio-store","/mnt/semio-store","S:\\","S:/"]);

function probeTcpHost(host:string,port:number,milliseconds:number):Promise<boolean>{
 return new Promise(done=>{
  let settled=false;
  const finish=(ok:boolean)=>{if(settled)return;settled=true;clearTimeout(timer);socket.destroy();done(ok);};
  const socket=createConnection({host,port},()=>finish(true));
  socket.once("error",()=>finish(false));
  const timer=setTimeout(()=>finish(false),milliseconds);
  timer.unref?.();
 });
}

async function ensureMountedStoreReachable(storeRoot:string,milliseconds:number):Promise<void>{
 const host=filled(process.env,"SEMIO_FLEET_NFS_HOST");
 if(host===undefined||!FLEET_STORE_MOUNTS.has(storeRoot))return;
 if(!await probeTcpHost(host,2049,Math.min(milliseconds,500)))throw Error(`fleet NFS host ${host}:2049 is unreachable`);
}

async function refresh(options:FleetCacheRefreshOptionsV1):Promise<FleetCacheRefreshV1>{
 const now=options.now??Date.now(),local=readLocalFleetCacheFactsV1(options.root,now);
 if(options.force!==true&&local!==undefined&&now-local.checkedAtMs<(options.ttlMilliseconds??TTL))return {state:"fresh",copy:local};
 try{
  const storeRoot=fleetStoreRootV1(options.root);
  if(storeRoot===undefined)throw Error("no store root in .cache/f/machine.json");
  await ensureMountedStoreReachable(storeRoot,options.timeoutMilliseconds??TIMEOUT);
  await syncFleetIdentityV1({root:options.root,...(options.timeoutMilliseconds===undefined?{}:{timeoutMilliseconds:options.timeoutMilliseconds})});
  const text=await bounded(readFile(fleetCacheFactsPathV1(storeRoot),"utf8"),options.timeoutMilliseconds??TIMEOUT,"reading the fleet cache facts"),facts=parseFleetCacheFactsV1(text),unchanged=local!==undefined&&JSON.stringify(local.facts)===JSON.stringify(facts);
  const selectedEndpoint=await select(facts.cache.endpoints,unchanged?local.selectedEndpoint:null,options.probeMilliseconds??PROBE,options.fetch??fetch);
  if(selectedEndpoint===null&&local!==undefined&&!unchanged&&local.selectedEndpoint!==null)return {state:"stale",copy:local,error:"no endpoint of the published facts answers; the previous good copy is kept"};
  const copy:FleetCacheLocalCopyV1={version:1,fetchedAtMs:unchanged?local.fetchedAtMs:now,checkedAtMs:now,selectedEndpoint,facts};
  await store(options.root,copy);
  return {state:unchanged?"checked":"updated",copy};
 }catch(error){
  return {state:local===undefined?"absent":"stale",copy:local,error:String((error as Error)?.message??error).slice(0,200)};
 }
}

/** 🔄️ Brings the local copy up to date from the group store: skipped within the TTL (30 s), otherwise `<store>/v1/fleet/cache.json` is read through the store root of `.cache/f/machine.json` within the timeout (1.5 s), the endpoints are probed on `/healthz` (700 ms each, the last answering one first) and the copy is replaced atomically with mode 0600. A stalled or missing store keeps the last known copy (`stale`), and with none the caller stays local (`absent`); this never throws. `force` ignores the TTL, which is what a caller does after a 401, because the credential may have rotated. Concurrent calls of one process for one root share one run. */
export function refreshFleetCacheFactsV1(options:FleetCacheRefreshOptionsV1):Promise<FleetCacheRefreshV1>{
 const key=`${resolve(options.root)}\0${options.force===true}`,running=pending.get(key);
 if(running!==undefined)return running;
 const started=refresh(options).finally(()=>{pending.delete(key);});pending.set(key,started);return started;
}

/** 🩺️ What `resolveAuthorisedRemoteCacheV1` found: the resolution (`undefined` = off), how the facts refresh ended, the status the probe route answered (`null` = unreachable) and whether it counts as authorised (200 or 404). */
export type AuthorisedRemoteCacheV1=Readonly<{resolution:RemoteCacheResolutionV1|undefined;refresh:FleetCacheRefreshV1|undefined;status:number|null;authorised:boolean}>;

/** 🔐️ Resolves the remote cache the way a build needs it: refresh the facts (TTL, timeout; this also registers the identity of the machine in the group store), resolve, probe the Nx archive route once. A 401 or an unreachable endpoint with fleet facts at hand forces a refresh (a new identity may not be known to the server yet, another endpoint may answer) and tries once more. Never throws and never waits longer than a few probes, so a dead cache degrades to local. */
export async function resolveAuthorisedRemoteCacheV1(environment:Readonly<Record<string,string|undefined>>,options:Readonly<{root?:string;probeMilliseconds?:number;timeoutMilliseconds?:number;fetch?:typeof fetch;now?:number}>={}):Promise<AuthorisedRemoteCacheV1>{
 const root=options.root??fleetCacheRootV1(environment),ask=options.fetch??fetch,refreshOptions=(force:boolean)=>({root:root!,force,...(options.timeoutMilliseconds===undefined?{}:{timeoutMilliseconds:options.timeoutMilliseconds}),...(options.probeMilliseconds===undefined?{}:{probeMilliseconds:options.probeMilliseconds}),...(options.fetch===undefined?{}:{fetch:options.fetch}),...(options.now===undefined?{}:{now:options.now})});
 const probe=async(resolution:RemoteCacheResolutionV1):Promise<number|null>=>{
  try{
   const response=await remoteCacheFetchV1({...environment,...resolution.environment},{...(root===undefined?{}:{root}),base:ask})(`${resolution.contract.endpoint}/${resolution.contract.instance}/v1/cache/semio-probe`,{signal:AbortSignal.timeout(options.probeMilliseconds??1500)});
   await response.body?.cancel();return response.status;
  }catch{return null;}
 };
 const accepted=(status:number|null):boolean=>status===200||status===404;
 let refresh=root===undefined?undefined:await refreshFleetCacheFactsV1(refreshOptions(false)),resolution=resolveRemoteCacheV1(environment,root===undefined?{}:{root});
 if(resolution===undefined)return {resolution,refresh,status:null,authorised:false};
 let status=await probe(resolution);
 if(!accepted(status)&&root!==undefined&&refresh?.copy!==undefined&&(status===401||status===403||status===null)){
  refresh=await refreshFleetCacheFactsV1(refreshOptions(true));
  const again=resolveRemoteCacheV1(environment,{root});
  if(again!==undefined&&(again.contract.endpoint!==resolution.contract.endpoint||again.token!==resolution.token||again.tokenFrom!==resolution.tokenFrom)){resolution=again;status=await probe(again);}
 }
 return {resolution,refresh,status,authorised:accepted(status)};
}
