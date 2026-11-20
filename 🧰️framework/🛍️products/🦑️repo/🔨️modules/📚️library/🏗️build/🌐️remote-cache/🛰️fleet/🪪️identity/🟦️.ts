import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {createHash,createPrivateKey,createPublicKey,generateKeyPairSync,randomBytes,sign,verify,type KeyObject} from "node:crypto";
import {readFileSync,statSync} from "node:fs";
import {mkdir,readdir,readFile,rename,rm,writeFile} from "node:fs/promises";
import {networkInterfaces} from "node:os";
import {dirname,join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🎭️ The five roles of the fleet, each with its own key so one leaked key is one role: `cache-read` and `cache-write` for the cache, `submit` for handing actions to the scheduler, `worker` for executing them, `admin` for the group server only. */
export const FLEET_ROLES_V1=Object.freeze(["cache-read","cache-write","submit","worker","admin"] as const);
export type FleetRoleV1=typeof FLEET_ROLES_V1[number];

/** 🎯️ The service a signature is made for; a signature for one audience is worthless at the other. */
export type FleetAudienceV1="cache"|"execution";

/** 📜️ One registration, shaped by `🧬️schema/🔣️.json` (`Registration`). */
export type FleetRegistrationV1=Readonly<{version:1;originId:string;group:string;sequence:number;createdAtMs:number;origin:string;keys:Readonly<Partial<Record<FleetRoleV1,string>>>;rotatedFrom?:Readonly<{publicKey:string;signature:string}>;signature:string}>;

/** 🧾️ What the registry says about one origin: usable (`ok`), `retired`, `conflict` (another origin key than the pinned one, without proof), `invalid` (broken file) or `pending` (nothing published yet). */
export type FleetOriginStatusV1=Readonly<{status:"ok"|"retired"|"conflict"|"invalid"|"pending";reason?:string;sequence?:number}>;

const AUDIENCES:Readonly<Record<FleetRoleV1,readonly FleetAudienceV1[]>>=Object.freeze({"cache-read":["cache"],"cache-write":["cache"],submit:["execution"],worker:["execution"],admin:["cache","execution"]});
const SCHEME="Semio-Fleet",CONTENT_HEADER="x-semio-content-sha256",TIME_HEADER="x-semio-time",PREFIX="semio-fleet-v1",ORIGIN=/^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$/u,SKEW=60000,KEYS=4096;

/** 🏷️ The header that carries the hash of the request body (`-` when the body is not bound), signed together with the request. */
export const FLEET_CONTENT_HEADER_V1=CONTENT_HEADER;

/** 🕰️ The response header with the server's clock, so a client with a skewed clock can correct itself once. */
export const FLEET_TIME_HEADER_V1=TIME_HEADER;

/** 🛡️ Validates one value against a named definition of the identity schema and names the first problem. */
export function requireFleetIdentityShapeV1(definition:"Registration"|"Pins"|"Divergences",value:unknown,message:string):void{
 const errors=validateJsonSchemaSubset({$ref:`#/definitions/${definition}`},value,schema);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

/** 🔢️ Whether a role may sign for an audience. */
export const fleetRoleAudienceV1=(role:FleetRoleV1,audience:FleetAudienceV1):boolean=>AUDIENCES[role].includes(audience);

/** 🧮️ The SHA-256 of bytes or text as hex, the value of the content header. */
export const fleetContentHashV1=(data:Uint8Array|string):string=>createHash("sha256").update(data).digest("hex");

const canonical=(value:unknown):string=>value===null||typeof value!=="object"?JSON.stringify(value):Array.isArray(value)?`[${value.map(canonical).join(",")}]`:`{${Object.keys(value).sort().filter(key=>(value as Record<string,unknown>)[key]!==undefined).map(key=>`${JSON.stringify(key)}:${canonical((value as Record<string,unknown>)[key])}`).join(",")}}`;
const text=(registration:FleetRegistrationV1):Buffer=>Buffer.from(canonical({...registration,signature:undefined,rotatedFrom:registration.rotatedFrom===undefined?undefined:{publicKey:registration.rotatedFrom.publicKey}}));
const publicOf=(encoded:string):KeyObject=>createPublicKey({key:Buffer.from(encoded,"base64url"),format:"der",type:"spki"});
const privateOf=(encoded:string):KeyObject=>createPrivateKey({key:Buffer.from(encoded.trim(),"base64url"),format:"der",type:"pkcs8"});
const exportPublic=(key:KeyObject):string=>key.export({type:"spki",format:"der"}).toString("base64url");
const exportPrivate=(key:KeyObject):string=>key.export({type:"pkcs8",format:"der"}).toString("base64url");
const valid=(key:KeyObject,data:Uint8Array,signature:string):boolean=>{try{return verify(null,data,key,Buffer.from(signature,"base64url"));}catch{return false;}};

/** 🗂️ Where a machine keeps its identity: `<worktree>/.cache/f/identity/`, never replicated. */
export const fleetIdentityDirectoryV1=(root:string):string=>machinePathV1(root,".cache","f","identity");

const keyPath=(root:string,name:string):string=>join(fleetIdentityDirectoryV1(root),`${name}.key`);
const registrationPath=(root:string):string=>join(fleetIdentityDirectoryV1(root),"registration.json");

/** 🏠️ The machine facts the identity needs from `.cache/f/machine.json`: its origin, its group and the store root. */
export function readFleetMachineV1(root:string):Readonly<{originId:string;group:string;storeRoot:string|undefined}>|undefined{
 try{
  const value=JSON.parse(readFileSync(machinePathV1(root,".cache","f","machine.json"),"utf8")) as {originId?:unknown;groupId?:unknown;storeRoot?:unknown};
  return typeof value.originId==="string"&&ORIGIN.test(value.originId)&&typeof value.groupId==="string"&&ORIGIN.test(value.groupId)?{originId:value.originId,group:value.groupId,storeRoot:typeof value.storeRoot==="string"&&value.storeRoot!==""?value.storeRoot:undefined}:undefined;
 }catch{return undefined;}
}

async function write(path:string,content:string,mode:number):Promise<void>{
 const temporary=`${path}.${process.pid}-${randomBytes(6).toString("hex")}.tmp`;
 await mkdir(dirname(path),{recursive:true,mode:0o700});
 try{await writeFile(temporary,content,{mode});await rename(temporary,path);}catch(error){await rm(temporary,{force:true}).catch(()=>{});throw error;}
}

async function readRegistration(root:string):Promise<FleetRegistrationV1|undefined>{
 try{const value=JSON.parse(await readFile(registrationPath(root),"utf8")) as unknown;requireFleetIdentityShapeV1("Registration",value,"Closed fleet registration required");return value as FleetRegistrationV1;}catch{return undefined;}
}

const roleNames=(admin:boolean):readonly FleetRoleV1[]=>admin?FLEET_ROLES_V1:FLEET_ROLES_V1.filter(role=>role!=="admin");

/** 🆕️ Makes sure the machine has its identity: one Ed25519 key per role and one for the origin, each private key in its own file (mode 0600, directory 0700), and a registration signed by the origin key. Creates everything on the first call and changes nothing afterwards. `admin` exists only for the origin `<group>-server`. */
export async function ensureFleetIdentityV1(options:Readonly<{root:string;originId:string;group:string;now?:number}>):Promise<Readonly<{registration:FleetRegistrationV1;created:boolean}>>{
 const found=await readRegistration(options.root),admin=options.originId===`${options.group}-server`;
 if(found!==undefined&&found.originId===options.originId&&found.group===options.group&&roleNames(admin).every(role=>found.keys[role]!==undefined)){
  try{for(const role of["origin",...roleNames(admin)])privateOf(readFileSync(keyPath(options.root,role),"utf8"));return {registration:found,created:false};}catch{}
 }
 return {registration:await renew({...options,all:true,sequence:(found?.sequence??0)+1,previous:found}),created:true};
}

async function renew(options:Readonly<{root:string;originId:string;group:string;now?:number;all:boolean;roles?:readonly FleetRoleV1[];sequence:number;previous:FleetRegistrationV1|undefined}>):Promise<FleetRegistrationV1>{
 const admin=options.originId===`${options.group}-server`,keys:Partial<Record<FleetRoleV1,string>>={},fresh=new Map<string,KeyObject>();
 for(const role of roleNames(admin)){
  const keep=!options.all&&!(options.roles??[]).includes(role)&&options.previous?.keys[role]!==undefined;
  if(keep){keys[role]=options.previous!.keys[role]!;continue;}
  const pair=generateKeyPairSync("ed25519");fresh.set(role,pair.privateKey);keys[role]=exportPublic(pair.publicKey);
 }
 let originKey:KeyObject,previousOrigin:KeyObject|undefined;
 try{previousOrigin=privateOf(readFileSync(keyPath(options.root,"origin"),"utf8"));}catch{previousOrigin=undefined;}
 const replaceOrigin=options.all||previousOrigin===undefined;
 if(replaceOrigin){const pair=generateKeyPairSync("ed25519");originKey=pair.privateKey;fresh.set("origin",pair.privateKey);}else originKey=previousOrigin!;
 const origin=exportPublic(createPublicKey(originKey)),draft:FleetRegistrationV1={version:1,originId:options.originId,group:options.group,sequence:options.sequence,createdAtMs:options.now??Date.now(),origin,keys,signature:""};
 const rotated=replaceOrigin&&previousOrigin!==undefined?{publicKey:exportPublic(createPublicKey(previousOrigin)),signature:sign(null,text({...draft,rotatedFrom:{publicKey:exportPublic(createPublicKey(previousOrigin)),signature:""}}),previousOrigin).toString("base64url")}:undefined;
 const unsigned:FleetRegistrationV1={...draft,...(rotated===undefined?{}:{rotatedFrom:rotated})},registration:FleetRegistrationV1={...unsigned,signature:sign(null,text(unsigned),originKey).toString("base64url")};
 for(const [name,key] of fresh)await write(keyPath(options.root,name),exportPrivate(key)+"\n",0o600);
 await write(registrationPath(options.root),JSON.stringify(registration)+"\n",0o600);
 return registration;
}

/** 🔄️ Replaces the keys of the given roles (or, with `origin`, the origin key too, signed by the previous one) and bumps the sequence; the other roles keep working. This is how one leaked role key is retired without touching the rest. */
export async function rotateFleetIdentityV1(options:Readonly<{root:string;originId:string;group:string;roles:readonly FleetRoleV1[];origin?:boolean;now?:number}>):Promise<FleetRegistrationV1>{
 const found=await ensureFleetIdentityV1(options);
 if(options.origin===true)return renew({...options,all:true,sequence:found.registration.sequence+1,previous:found.registration});
 return renew({...options,all:false,sequence:found.registration.sequence+1,previous:found.registration});
}

/** 🔑️ The origin public key (SPKI, base64url) that a store registers for an origin: the registration must be well formed and signed by that very key; `undefined` when there is none or it does not verify. This is the key a peer group's server is sealed to when the topology does not pin it. */
export async function readFleetOriginKeyV1(storeRoots:readonly string[],originId:string,signal?:AbortSignal):Promise<string|undefined>{
 for(const root of storeRoots){
  signal?.throwIfAborted();
  try{
   const value=JSON.parse(await readFile(fleetRegistrationPathV1(root,originId),{encoding:"utf8",signal})) as unknown;
   requireFleetIdentityShapeV1("Registration",value,"Closed fleet registration required");
   const registration=value as FleetRegistrationV1;
   if(registration.originId===originId&&valid(publicOf(registration.origin),text(registration),registration.signature))return registration.origin;
  }catch{signal?.throwIfAborted();}
 }
 return undefined;
}

/** 🗝️ The private origin key (PKCS8, base64url) of this machine, or `undefined` without an identity; only a group server uses it, to open what peers and members seal to it. */
export function readFleetOriginPrivateV1(root:string):string|undefined{
 try{const value=readFileSync(keyPath(root,"origin"),"utf8").trim();privateOf(value);return value;}catch{return undefined;}
}

/** 📍️ Where an origin publishes its registration in the group store. */
export const fleetRegistrationPathV1=(storeRoot:string,originId:string):string=>join(storeRoot,"v1","origins",originId,"fleet-auth.json");

async function bounded<T>(work:Promise<T>,milliseconds:number):Promise<T>{
 let timer:ReturnType<typeof setTimeout>|undefined;
 try{return await Promise.race([work,new Promise<never>((_,reject)=>{timer=setTimeout(()=>reject(Error(`store took longer than ${milliseconds} ms`)),milliseconds);})]);}
 finally{if(timer)clearTimeout(timer);work.catch(()=>{});}
}

/** 📣️ Makes the identity of this machine exist and visible: ensures the keys, then publishes the registration to its own origin directory of the group store when that differs. Never throws; a stalled or missing store leaves the machine registered locally and `error` set. */
export async function syncFleetIdentityV1(options:Readonly<{root:string;timeoutMilliseconds?:number;now?:number}>):Promise<Readonly<{registration:FleetRegistrationV1|undefined;published:boolean;error?:string}>>{
 const machine=readFleetMachineV1(options.root);
 if(machine===undefined)return {registration:undefined,published:false,error:"no origin in .cache/f/machine.json"};
 try{
  const {registration}=await ensureFleetIdentityV1({root:options.root,originId:machine.originId,group:machine.group,...(options.now===undefined?{}:{now:options.now})});
  if(machine.storeRoot===undefined)return {registration,published:false,error:"no store root in .cache/f/machine.json"};
  const path=fleetRegistrationPathV1(machine.storeRoot,machine.originId),wanted=JSON.stringify(registration)+"\n";
  try{
   const current=await bounded(readFile(path,"utf8"),options.timeoutMilliseconds??1500).catch(()=>"");
   if(current===wanted)return {registration,published:false};
   await bounded(write(path,wanted,0o644),options.timeoutMilliseconds??1500);return {registration,published:true};
  }catch(error){return {registration,published:false,error:String((error as Error)?.message??error).slice(0,200)};}
 }catch(error){return {registration:undefined,published:false,error:String((error as Error)?.message??error).slice(0,200)};}
}

/** ✍️ What a signer produces for one request: the `authorization` header and the content header. */
export type FleetSignatureInputV1=Readonly<{audience:FleetAudienceV1;method:string;target:string;content?:string|undefined;now?:number;nonce?:string}>;

/** 🔏️ One role of one origin, ready to sign; `offset` is the learned difference to the server clock. */
export interface FleetSignerV1{readonly originId:string;readonly role:FleetRoleV1;offset:number;sign(input:FleetSignatureInputV1):Record<string,string>}

const stringToSign=(audience:string,origin:string,role:string,timestamp:number,nonce:string,method:string,target:string,content:string):string=>[PREFIX,audience,origin,role,timestamp,nonce,method.toUpperCase(),target,content].join("\n");
const signers=new Map<string,{stamp:number;signer:FleetSignerV1}>();

/** 🔑️ Loads the signer of one role from this machine's identity directory; only that role's key file is read. `undefined` when the machine has no identity (yet). A rotated key file is picked up by the next call. */
export function loadFleetSignerV1(root:string,role:FleetRoleV1):FleetSignerV1|undefined{
 const path=keyPath(root,role);
 try{
  const stamp=statSync(path).mtimeMs,cached=signers.get(path);
  if(cached!==undefined&&cached.stamp===stamp)return cached.signer;
  const registration=JSON.parse(readFileSync(registrationPath(root),"utf8")) as FleetRegistrationV1,key=privateOf(readFileSync(path,"utf8"));
  if(typeof registration.originId!=="string"||!ORIGIN.test(registration.originId))return undefined;
  const signer:FleetSignerV1={originId:registration.originId,role,offset:cached?.signer.offset??0,sign(input){
   const timestamp=(input.now??Date.now())+signer.offset,nonce=input.nonce??randomBytes(16).toString("base64url"),content=input.content??"-";
   const signature=sign(null,Buffer.from(stringToSign(input.audience,signer.originId,role,timestamp,nonce,input.method,input.target,content)),key).toString("base64url");
   return {authorization:`${SCHEME} o=${signer.originId} r=${role} a=${input.audience} t=${timestamp} n=${nonce} s=${signature}`,[CONTENT_HEADER]:content};
  }};
  signers.set(path,{stamp,signer});return signer;
 }catch{return undefined;}
}

/** 🖋️ Signs a document (not a request) with a role key of this machine: the origin and the signature, or `undefined` without that key. The label keeps a document signature from ever being a request signature. */
export function signFleetDocumentV1(root:string,role:FleetRoleV1,label:string,text:string):Readonly<{origin:string;signature:string}>|undefined{
 try{
  const registration=JSON.parse(readFileSync(registrationPath(root),"utf8")) as FleetRegistrationV1,key=privateOf(readFileSync(keyPath(root,role),"utf8"));
  return {origin:registration.originId,signature:sign(null,Buffer.from(`${PREFIX}-document\n${label}\n${text}`),key).toString("base64url")};
 }catch{return undefined;}
}

/** 🔍️ Checks a document signature against the key that the group store registers for the origin and role (pinned, not retired). */
export async function verifyFleetDocumentV1(registry:FleetRegistryV1,origin:string,role:FleetRoleV1,label:string,text:string,signature:string):Promise<boolean>{
 if(!ORIGIN.test(origin))return false;
 await registry.refresh();
 let key=registry.key(origin,role);
 if(key===undefined&&registry.status(origin).status==="pending"){await registry.learn(origin);key=registry.key(origin,role);}
 return key!==undefined&&valid(key,Buffer.from(`${PREFIX}-document\n${label}\n${text}`),signature);
}

/** 🌐️ The path and query of a URL, the part of the request that is signed. */
export const fleetTargetV1=(url:string|URL):string=>{const parsed=new URL(url);return parsed.pathname+parsed.search;};

/** 🧰️ The signed headers of one request for a role of this machine, or an empty object when it has no such key. */
export function fleetHeadersV1(root:string,role:FleetRoleV1,audience:FleetAudienceV1,method:string,url:string|URL,content?:string):Record<string,string>{
 return loadFleetSignerV1(root,role)?.sign({audience,method,target:fleetTargetV1(url),content})??{};
}

/** 📞️ What `fleetSignedFetchV1` is told. */
export type FleetSignedFetchOptionsV1=Readonly<{root:string;audience:FleetAudienceV1;roleOf?:(method:string,pathname:string)=>FleetRoleV1;fetch?:typeof fetch}>;

const defaultRole=(method:string):FleetRoleV1=>method==="GET"||method==="HEAD"?"cache-read":"cache-write";
const bytesOf=(value:unknown):Uint8Array|string|undefined=>typeof value==="string"?value:value instanceof Uint8Array?value:value instanceof ArrayBuffer?new Uint8Array(value):undefined;

/** 🪝️ A `fetch` that signs every request with the machine's role key: the role follows the method (reads with `cache-read`, writes with `cache-write`) unless `roleOf` says otherwise, the body hash is bound when the body is text or bytes, and a 401 that carries the server's clock is retried once with the corrected clock. No secret is sent; without an identity the request goes out unsigned and the server refuses it, which callers treat as "stay local". */
export function fleetSignedFetchV1(options:FleetSignedFetchOptionsV1):typeof fetch{
 const base=options.fetch??fetch,roleOf=options.roleOf??((method:string)=>defaultRole(method));
 const send=async(input:RequestInfo|URL,init:RequestInit|undefined,retry:boolean):Promise<Response>=>{
  const request=typeof input==="string"||input instanceof URL?undefined:input,url=request===undefined?String(input):request.url,method=(init?.method??request?.method??"GET").toUpperCase(),parsed=new URL(url);
  const payload=bytesOf(init?.body),signer=loadFleetSignerV1(options.root,roleOf(method,parsed.pathname));
  const headers=new Headers(init?.headers??request?.headers);
  if(signer!==undefined)for(const [name,value] of Object.entries(signer.sign({audience:options.audience,method,target:fleetTargetV1(parsed),content:payload!==undefined?fleetContentHashV1(payload):method==="PUT"?(/\/cas\/([0-9a-f]{64})$/u.exec(parsed.pathname)?.[1]??"-"):"-"})))headers.set(name,value);
  const response=await base(input,{...init,headers});
  if(response.status===401&&retry&&signer!==undefined&&(init?.body===undefined||init?.body===null||payload!==undefined)){
   const server=Number(response.headers.get(TIME_HEADER));
   if(Number.isFinite(server)&&server>0&&Math.abs(server-(Date.now()+signer.offset))>SKEW/4){await response.body?.cancel();signer.offset=server-Date.now();return send(input,init,false);}
  }
  return response;
 };
 return ((input:RequestInfo|URL,init?:RequestInit)=>send(input,init,true)) as typeof fetch;
}

function inNetwork(value:number,network:string):boolean{
 const [base,bits]=network.split("/"),start=ipv4(base!),length=Number(bits);
 if(start===undefined||!Number.isInteger(length)||length<0||length>32)return false;
 const mask=length===0?0:(0xFFFFFFFF<<(32-length))>>>0;
 return ((value&mask)>>>0)===((start&mask)>>>0);
}
function ipv4(address:string):number|undefined{
 const parts=address.split(".");
 if(parts.length!==4||parts.some(part=>!/^\d{1,3}$/u.test(part)||Number(part)>255))return undefined;
 return parts.reduce((total,part)=>total*256+Number(part),0);
}

/** 🌍️ Whether an address belongs to one of the CIDR networks; loopback always does. IPv4 and IPv4-mapped IPv6 addresses are understood, an unknown or missing peer never belongs. */
export function fleetPeerAllowedV1(peer:string|undefined,networks:readonly string[]):boolean{
 if(peer===undefined)return false;
 const address=peer.replace(/^::ffff:/iu,"");
 if(address==="::1"||/^127\./u.test(address))return true;
 const value=ipv4(address);
 return value!==undefined&&networks.some(network=>inNetwork(value,network));
}

/** 🧭️ The IPv4 address of this machine inside a CIDR network (one of `prefer` first, such as the address the facts publish): the fleet interface a server binds to (a loopback network yields 127.0.0.1, which tests use). */
export function fleetInterfaceAddressV1(network:string,interfaces:ReturnType<typeof networkInterfaces>=networkInterfaces(),prefer:readonly string[]=[]):string|undefined{
 const found:string[]=[];
 for(const rows of Object.values(interfaces))for(const row of rows??[]){const value=row.family==="IPv4"&&!row.internal?ipv4(row.address):undefined;if(value!==undefined&&inNetwork(value,network))found.push(row.address);}
 if(found.length>0)return prefer.find(address=>found.includes(address))??found[0];
 return inNetwork(0x7F000001,network)?"127.0.0.1":undefined;
}

/** 📚️ The origins a server knows, read from the group store. */
export interface FleetRegistryV1{
 refresh(force?:boolean):Promise<void>;
 learn(originId:string):Promise<void>;
 key(originId:string,role:FleetRoleV1):KeyObject|undefined;
 groupOf(originId:string):string|undefined;
 status(originId:string):FleetOriginStatusV1;
 origins():Readonly<Record<string,FleetOriginStatusV1>>;
}

/** 📚️ What `createFleetRegistryV1` needs. */
export type FleetRegistryOptionsV1=Readonly<{storeRoot:string;group:string;groups?:readonly string[];peerStores?:Readonly<Record<string,string>>;pinned?:Readonly<Record<string,string>>;pinsPath?:string;ttlMilliseconds?:number;timeoutMilliseconds?:number;now?:()=>number}>;

/** 📚️ Reads `origins/<origin>/fleet-auth.json` and `retired.json` of the group store, at most once per TTL (5 s) and bounded by a timeout; an origin it does not know yet is looked up on its own, at most once per second per origin, keeping the last good view when the store stalls. The fleet may be several groups: `groups` names the peer groups whose origins are admitted when they appear in this store (copied by the relay), `peerStores` maps a peer group to the folder where its store is mounted, and its `origins/` are read from there; `pinned` maps an origin to the public key that is trusted for it before anything is seen (a topology pin, stronger than first sight). Pins the origin key of the first valid registration per origin in `pinsPath`; a later registration with another origin key is accepted only with a rotation signature of the pinned key, or after the origin was retired and re-admitted; a retired origin has no keys. */
export function createFleetRegistryV1(options:FleetRegistryOptionsV1):FleetRegistryV1{
 const clock=options.now??Date.now,ttl=options.ttlMilliseconds??5000,timeout=options.timeoutMilliseconds??1500;
 let checked=-Infinity,view=new Map<string,{status:FleetOriginStatusV1;keys:Map<FleetRoleV1,KeyObject>;group?:string}>(),pins:Record<string,{publicKey:string;pinnedAtMs:number}>|undefined,running:Promise<void>|undefined;
 const loadPins=async():Promise<Record<string,{publicKey:string;pinnedAtMs:number}>>=>{
  if(pins!==undefined)return pins;
  if(options.pinsPath===undefined){pins={};for(const [id,publicKey] of Object.entries(options.pinned??{}))pins[id]={publicKey,pinnedAtMs:0};return pins;}
  try{const value=JSON.parse(await readFile(options.pinsPath,"utf8")) as unknown;requireFleetIdentityShapeV1("Pins",value,"Closed pins required");pins=(value as {pins:Record<string,{publicKey:string;pinnedAtMs:number}>}).pins;}catch{pins={};}
  for(const [id,publicKey] of Object.entries(options.pinned??{}))pins[id]={publicKey,pinnedAtMs:0};
  return pins;
 };
 const savePins=async():Promise<void>=>{if(options.pinsPath!==undefined&&pins!==undefined)await write(options.pinsPath,JSON.stringify({version:1,pins})+"\n",0o600).catch(()=>{});};
 type Entry={status:FleetOriginStatusV1;keys:Map<FleetRoleV1,KeyObject>;group?:string};
 const admitted=new Set([options.group,...(options.groups??[]),...Object.keys(options.peerStores??{})]);
 const readOrigin=async(id:string,held:Record<string,{publicKey:string;pinnedAtMs:number}>,storeRoot:string=options.storeRoot,expected?:string):Promise<Readonly<{entry:Entry;changed:boolean}>> =>{
  const root=join(storeRoot,"v1","origins"),entry:Entry={status:{status:"pending"},keys:new Map<FleetRoleV1,KeyObject>()};
  let retiredAt:number|undefined,retired=false;
  try{const value=JSON.parse(await readFile(join(root,id,"retired.json"),"utf8")) as {retired?:unknown;wallMs?:unknown};retired=value.retired===true;retiredAt=typeof value.wallMs==="number"?value.wallMs:undefined;}catch{}
  if(retired){entry.status={status:"retired"};return {entry,changed:false};}
  let registration:FleetRegistrationV1;
  try{const value=JSON.parse(await readFile(fleetRegistrationPathV1(storeRoot,id),"utf8")) as unknown;requireFleetIdentityShapeV1("Registration",value,"Closed fleet registration required");registration=value as FleetRegistrationV1;}
  catch(error){entry.status=(error as NodeJS.ErrnoException).code==="ENOENT"?{status:"pending"}:{status:"invalid",reason:"unreadable registration"};return {entry,changed:false};}
  if(registration.originId!==id||!admitted.has(registration.group)||(expected!==undefined&&registration.group!==expected)){entry.status={status:"invalid",reason:"origin or group differs"};return {entry,changed:false};}
  entry.group=registration.group;
  let origin:KeyObject;
  try{origin=publicOf(registration.origin);if(!valid(origin,text(registration),registration.signature))throw Error("signature");}catch{entry.status={status:"invalid",reason:"bad signature"};return {entry,changed:false};}
  const pin=held[id],readmitted=pin!==undefined&&retiredAt!==undefined&&!retired&&retiredAt>pin.pinnedAtMs;
  if(pin!==undefined&&pin.publicKey!==registration.origin&&!readmitted){
   const proof=registration.rotatedFrom;
   let proven=false;
   if(proof!==undefined&&proof.publicKey===pin.publicKey){try{proven=valid(publicOf(pin.publicKey),text(registration),proof.signature);}catch{proven=false;}}
   if(!proven){entry.status={status:"conflict",reason:"origin key differs from the pinned one",sequence:registration.sequence};return {entry,changed:false};}
  }
  let changed=false;
  if(pin===undefined||pin.publicKey!==registration.origin){held[id]={publicKey:registration.origin,pinnedAtMs:clock()};changed=true;}
  try{for(const role of FLEET_ROLES_V1){const encoded=registration.keys[role];if(encoded!==undefined)entry.keys.set(role,publicOf(encoded));}}catch{entry.status={status:"invalid",reason:"bad role key"};entry.keys.clear();return {entry,changed};}
  if(registration.keys.admin!==undefined&&id!==`${registration.group}-server`)entry.keys.delete("admin");
  entry.status={status:"ok",sequence:registration.sequence};
  return {entry,changed};
 };
 const read=async():Promise<void>=>{
  const root=join(options.storeRoot,"v1","origins"),next=new Map<string,Entry>(),held=await loadPins();let changed=false;
  for(const id of(await readdir(root).catch(()=>[] as string[])).filter(name=>ORIGIN.test(name)).sort().slice(0,KEYS)){const row=await readOrigin(id,held);next.set(id,row.entry);changed=changed||row.changed;}
  for(const [peer,peerRoot] of Object.entries(options.peerStores??{})){
   let ids:string[]=[];
   try{ids=(await readdir(join(peerRoot,"v1","origins"))).filter(name=>ORIGIN.test(name)&&!next.has(name)).sort().slice(0,KEYS);}catch{continue;}
   for(const id of ids){const row=await readOrigin(id,held,peerRoot,peer);next.set(id,row.entry);changed=changed||row.changed;}
  }
  view=next;if(changed)await savePins();
 };
 const learned=new Map<string,number>();
 const learn=async(id:string):Promise<void>=>{
  const now=clock();
  if(now-(learned.get(id)??-Infinity)<1000)return;
  if(learned.size>=4096){for(const [name,at] of learned)if(now-at>=1000)learned.delete(name);if(learned.size>=4096)return;}
  learned.set(id,now);
  try{
   const held=await loadPins();
   let row=await bounded(readOrigin(id,held),timeout);
   for(const [peer,peerRoot] of Object.entries(options.peerStores??{}))if(row.entry.status.status==="pending")row=await bounded(readOrigin(id,held,peerRoot,peer),timeout);
   const next=new Map(view);next.set(id,row.entry);view=next;if(row.changed)await savePins();
  }catch{}
 };
 const refresh=async(force=false):Promise<void>=>{
  if(!force&&clock()-checked<ttl)return;
  running??=bounded(read(),timeout).then(()=>{checked=clock();},()=>{checked=clock();}).finally(()=>{running=undefined;});
  await running;
 };
 return Object.freeze<FleetRegistryV1>({refresh,learn,key:(id,role)=>view.get(id)?.keys.get(role),groupOf:id=>view.get(id)?.group,status:id=>view.get(id)?.status??{status:"pending",reason:"unknown origin"},origins:()=>Object.fromEntries([...view].map(([id,row])=>[id,row.status]))});
}

/** 🏁️ The verdict on one request: the origin and role that signed it and the content hash it declared, or a refusal with the status to answer and a reason for the counters only (the answer never says why). */
export type FleetVerdictV1=Readonly<{ok:true;origin:string;group:string;role:FleetRoleV1;content:string}>|Readonly<{ok:false;status:401|503;reason:string;serverTime:number}>;

/** 🛂️ What `createFleetVerifierV1` needs. */
export type FleetVerifierOptionsV1=Readonly<{registry:FleetRegistryV1;audience:FleetAudienceV1;adminOrigin?:string;skewMilliseconds?:number;maximumNonces?:number;now?:()=>number;startedAtMs?:number}>;

/** 🛂️ Checks signed requests: the audience matches, the clock is within the skew (60 s) and not before the server started, the origin is registered, not retired and holds a key for the role, the signature covers method, target and content hash, and the nonce has not been seen. `admin` is accepted only from the admin origin. Counts refusals by reason. */
export interface FleetVerifierV1{
 verify(input:Readonly<{method:string;target:string;authorization:string|null|undefined;content:string|null|undefined}>):Promise<FleetVerdictV1>;
 denied():Readonly<Record<string,number>>;
}

/** 🛂️ Creates a verifier over a registry. */
export function createFleetVerifierV1(options:FleetVerifierOptionsV1):FleetVerifierV1{
 const clock=options.now??Date.now,skew=options.skewMilliseconds??SKEW,limit=options.maximumNonces??200000,started=options.startedAtMs??clock(),seen=new Map<string,number>(),counts:Record<string,number>={};
 const refuse=(reason:string,status:401|503=401):FleetVerdictV1=>{counts[reason]=(counts[reason]??0)+1;return {ok:false,status,reason,serverTime:clock()};};
 const sweep=(now:number):void=>{for(const [key,expires] of seen)if(expires<now)seen.delete(key);};
 return Object.freeze<FleetVerifierV1>({
  denied:()=>({...counts}),
  async verify(input){
   const header=input.authorization;
   if(typeof header!=="string"||!header.startsWith(SCHEME+" "))return refuse("no-signature");
   const fields=new Map<string,string>();
   for(const part of header.slice(SCHEME.length+1).split(" ")){const equals=part.indexOf("=");if(equals>0)fields.set(part.slice(0,equals),part.slice(equals+1));}
   const origin=fields.get("o"),role=fields.get("r") as FleetRoleV1|undefined,audience=fields.get("a"),timestamp=Number(fields.get("t")),nonce=fields.get("n"),signature=fields.get("s");
   if(origin===undefined||!ORIGIN.test(origin)||role===undefined||!FLEET_ROLES_V1.includes(role)||audience!==options.audience||!Number.isSafeInteger(timestamp)||nonce===undefined||!/^[A-Za-z0-9_-]{16,64}$/u.test(nonce)||signature===undefined||!/^[A-Za-z0-9_-]{86}$/u.test(signature))return refuse("malformed");
   if(!fleetRoleAudienceV1(role,options.audience))return refuse("role-audience");
   const content=input.content??"-";
   if(content!=="-"&&!/^[0-9a-f]{64}$/u.test(content))return refuse("malformed");
   const now=clock();
   if(Math.abs(now-timestamp)>skew)return refuse("clock");
   if(timestamp<started-1000)return refuse("before-start");
   await options.registry.refresh();
   let key=options.registry.key(origin,role);
   if(key===undefined&&options.registry.status(origin).status==="pending"){await options.registry.learn(origin);key=options.registry.key(origin,role);}
   if(key===undefined)return refuse(options.registry.status(origin).status==="ok"?"no-role-key":options.registry.status(origin).status);
   if(role==="admin"&&origin!==options.adminOrigin)return refuse("admin-origin");
   if(!valid(key,Buffer.from(stringToSign(audience,origin,role,timestamp,nonce,input.method,input.target,content)),signature))return refuse("bad-signature");
   const identifier=`${origin}|${nonce}`;
   if(seen.has(identifier))return refuse("replay");
   if(seen.size>=limit){sweep(now);if(seen.size>=limit)return refuse("busy",503);}
   seen.set(identifier,now+2*skew+1000);
   if(seen.size%1024===0)sweep(now);
   return {ok:true,origin,group:options.registry.groupOf(origin)??"",role,content};
  },
 });
}
