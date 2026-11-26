import {randomUUID} from "node:crypto";
import {mkdir,readFile,readdir,rename,rm,stat,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {spawn as spawnProcess,spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {createConnection,createServer} from "node:net";
import {networkInterfaces} from "node:os";
import texts from "./📖️texts/🔣️.json" with {type:"json"};

/** 🧭️ The languages of the address messages; there is no default, the caller names one. */
export type FleetAddressLanguageV1="en"|"de";

/** 🌐️ One reachable address of one interface of a machine, shaped by `🧬️schema/🔣️.json`. */
export type FleetAddressV1=Readonly<{address:string;family:"IPv4"|"IPv6";interface:string;prefix:number;scope:"private"|"unique-local"|"global"}>;

/** 📇️ The addresses a machine publishes in its origin record: identified by the origin id, never by an address and never by the host name of the operating system; `name` is the fleet name `semio-<origin>` derived from the origin. */
export type FleetAddressSetV1=Readonly<{version:1;origin:string;name:string;hostLabel:string|null;advertise:FleetAdvertiseStateV1;addresses:readonly FleetAddressV1[];observedAtMs:number;ttlMs:number;generation:number}>;

/** 🚦️ Whether the machine advertises itself through DNS-SD and, when it stays silent, the origins that carry the same operating system host name; `hostLabel` in the set is that name's first label, kept only to detect such a duplicate, never as an identity. */
export type FleetAdvertiseStateV1=Readonly<{enabled:boolean;duplicates:readonly string[];namedAs:string|null}>;

/** ⏱️ How long a published set counts as fresh (a machine refreshes it at half of this and on every interface change). */
export const FLEET_ADDRESS_TTL_MS_V1=300_000;

const ORIGIN=/^[a-z][a-z0-9-]{0,63}$/u;

/** 🏷️ The DNS-SD service type under which every machine advertises itself (Bonjour on macOS and Windows, avahi on Linux): one instance per origin, named by the origin id. */
export const FLEET_SERVICE_TYPE_V1="_semio-fleet._tcp";

/** 🪪️ The fleet name of an origin: `semio-<origin>`, semantic and unique, derived from the origin id and never from the host name of the operating system. */
export function fleetNameV1(origin:string):string{
 if(!ORIGIN.test(origin))throw Error(`Declared fleet origin required: ${origin}`);
 return `semio-${origin}`;
}

/** 🗣️ A message of `📖️texts` in one language; a missing key or variable is an error. */
export function fleetAddressTextV1(language:FleetAddressLanguageV1,key:string,variables:Readonly<Record<string,string|number>>={}):string{
 const message=(texts.messages as Readonly<Record<string,Readonly<Record<FleetAddressLanguageV1,string>>>>)[key];
 if(message===undefined)throw Error(`Declared fleet address message required: ${key}`);
 return message[language].replace(/\{([a-z0-9]+)\}/gu,(_,name:string)=>{const value=variables[name];if(value===undefined)throw Error(`Fleet address message variable missing: ${name}`);return String(value);});
}

const quad=(address:string):readonly number[]|undefined=>{
 const parts=address.split(".");
 if(parts.length!==4)return undefined;
 const values=parts.map(part=>/^[0-9]{1,3}$/u.test(part)?Number(part):Number.NaN);
 return values.every(value=>value>=0&&value<=255)?values:undefined;
};

const groups=(address:string):readonly number[]|undefined=>{
 const bare=address.split("%")[0]!.toLowerCase();
 if(!/^[0-9a-f:.]+$/u.test(bare)||!bare.includes(":"))return undefined;
 const [head,tail,...rest]=bare.split("::");
 if(rest.length>0)return undefined;
 const expand=(text:string):number[]|undefined=>{
  if(text==="")return [];
  const out:number[]=[];
  for(const part of text.split(":")){
   if(part.includes(".")){const v4=quad(part);if(v4===undefined)return undefined;out.push(v4[0]!*256+v4[1]!,v4[2]!*256+v4[3]!);}
   else if(/^[0-9a-f]{1,4}$/u.test(part))out.push(parseInt(part,16));
   else return undefined;
  }
  return out;
 };
 const left=expand(head!),right=tail===undefined?[]:expand(tail);
 if(left===undefined||right===undefined)return undefined;
 if(tail===undefined)return left.length===8?left:undefined;
 const fill=8-left.length-right.length;
 return fill<1?undefined:[...left,...Array<number>(fill).fill(0),...right];
};

/** 🚫 Whether an address is link-local (169.254/16, fe80::/10) and therefore never published. */
export function fleetLinkLocalV1(address:string):boolean{
 const v4=quad(address);
 if(v4!==undefined)return v4[0]===169&&v4[1]===254;
 const v6=groups(address);
 return v6!==undefined&&(v6[0]!&0xffc0)===0xfe80;
}

/** 🔁️ Whether an address is a loopback address (the whole 127/8 and ::1). */
export function fleetLoopbackV1(address:string):boolean{
 const v4=quad(address);
 if(v4!==undefined)return v4[0]===127;
 const v6=groups(address);
 return v6!==undefined&&v6.slice(0,7).every(group=>group===0)&&v6[7]===1;
}

/** 🔭️ How far an address reaches: private (10/8, 172.16/12, 192.168/16, 100.64/10), unique-local (fc00::/7), else global. */
export function fleetAddressScopeV1(address:string):FleetAddressV1["scope"]{
 const v4=quad(address);
 if(v4!==undefined)return v4[0]===10||(v4[0]===172&&v4[1]!>=16&&v4[1]!<=31)||(v4[0]===192&&v4[1]===168)||(v4[0]===100&&v4[1]!>=64&&v4[1]!<=127)?"private":"global";
 const v6=groups(address);
 return v6!==undefined&&(v6[0]!&0xfe00)===0xfc00?"unique-local":"global";
}

const prefixOf=(row:Readonly<{address:string;netmask?:string;cidr?:string|null}>):number=>{
 const cidr=row.cidr?.split("/")[1];
 if(cidr!==undefined&&/^[0-9]{1,3}$/u.test(cidr))return Number(cidr);
 const mask=row.netmask===undefined?undefined:quad(row.netmask);
 if(mask!==undefined)return mask.reduce((sum,octet)=>sum+octet.toString(2).replaceAll("0","").length,0);
 return quad(row.address)===undefined?64:24;
};

/** 🏷️ The first label of an operating system host name (lower case), used only to notice that two machines share a name; `null` when it has no usable label. */
export function fleetHostLabelV1(hostname:string):string|null{
 const label=hostname.trim().toLowerCase().split(".")[0]!.replace(/[^a-z0-9-]+/gu,"-").replace(/^-+|-+$/gu,"");
 return /^[a-z0-9][a-z0-9-]{0,62}$/u.test(label)?label:null;
}

const labelBase=(label:string):string=>label.replace(/-[0-9]+$/u,"");

/** 🚦️ The conflict gate of the DNS-SD advertisement: a machine advertises only when its host name label is usable and no other origin published the same label or the same label with the numeric suffix that macOS appends when it renames a conflicting name (`name-2`). A host name `semio-<other origin>` that belongs to another origin (`namedAs`) keeps it silent too. Otherwise it stays silent and the origins it clashes with are named. Origins that are alive (their `ready.json` is younger than ten minutes) but have not published a set yet are `unverified`: their host name is not known, so the machine stays silent until they have. */
export function fleetAdvertiseGateV1(input:Readonly<{origin:string;hostLabel:string|null;others:readonly Readonly<{origin:string;hostLabel:string|null}>[];unverified?:readonly string[]}>):FleetAdvertiseStateV1{
 if(input.hostLabel===null)return {enabled:false,duplicates:[],namedAs:null};
 const claimed=/^semio-([a-z][a-z0-9-]{0,63})$/u.exec(input.hostLabel)?.[1],namedAs=claimed!==undefined&&claimed!==input.origin?claimed:null,base=labelBase(input.hostLabel),duplicates=input.others.filter(other=>other.origin!==input.origin&&other.hostLabel!==null&&labelBase(other.hostLabel)===base).map(other=>other.origin).sort();
 return {enabled:duplicates.length===0&&namedAs===null&&(input.unverified??[]).length===0,duplicates,namedAs};
}

/** 🔎️ Every address a machine can be reached by: all interfaces, IPv4 and IPv6, with loopback, link-local and internal rows left out, ordered by interface name and address so the same interfaces always give the same set. */
export function collectFleetAddressesV1(interfaces:ReturnType<typeof networkInterfaces>):readonly FleetAddressV1[]{
 const rows:FleetAddressV1[]=[];
 for(const [name,list] of Object.entries(interfaces))for(const row of list??[]){
  const address=row.address.split("%")[0]!;
  if(row.internal||fleetLoopbackV1(address)||fleetLinkLocalV1(address))continue;
  const family=quad(address)!==undefined?"IPv4":groups(address)!==undefined?"IPv6":undefined;
  if(family===undefined)continue;
  rows.push({address,family,interface:name,prefix:prefixOf({...row,address}),scope:fleetAddressScopeV1(address)});
 }
 return rows.sort((left,right)=>left.interface.localeCompare(right.interface)||left.address.localeCompare(right.address)).slice(0,32);
}

/** 🧾️ The identity of what a set says (the addresses, never the time): equal signatures mean nothing to republish. */
export function fleetAddressSignatureV1(set:Pick<FleetAddressSetV1,"addresses"|"hostLabel"|"advertise">):string{
 return JSON.stringify([set.hostLabel,set.advertise,set.addresses.map(row=>[row.interface,row.address,row.prefix])]);
}

/** 🏗️ Builds the set of a machine from its interfaces. */
export function createFleetAddressSetV1(input:Readonly<{origin:string;interfaces:ReturnType<typeof networkInterfaces>;now:number;generation?:number;ttlMs?:number;hostLabel?:string|null;advertise?:FleetAdvertiseStateV1}>):FleetAddressSetV1{
 return {version:1,origin:input.origin,name:fleetNameV1(input.origin),hostLabel:input.hostLabel??null,advertise:input.advertise??{enabled:false,duplicates:[],namedAs:null},addresses:collectFleetAddressesV1(input.interfaces),observedAtMs:input.now,ttlMs:input.ttlMs??FLEET_ADDRESS_TTL_MS_V1,generation:input.generation??1};
}

/** 🛡️ Checks the closed shape of a set read from a store and returns it; anything else throws. */
export function requireFleetAddressSetV1(value:unknown):FleetAddressSetV1{
 const fail=(reason:string):never=>{throw Error(`Closed fleet address set required: ${reason}`);};
 if(typeof value!=="object"||value===null||Array.isArray(value))return fail("an object");
 const set=value as Record<string,unknown>;
 const keys=Object.keys(set).sort().join(),expected=["addresses","advertise","generation","hostLabel","name","observedAtMs","origin","ttlMs","version"].join();
 if(keys!==expected)return fail(`keys ${keys}`);
 if(set.version!==1)return fail("version 1");
 if(typeof set.origin!=="string"||!/^[a-z][a-z0-9-]{0,63}$/u.test(set.origin))return fail("origin");
 if(set.name!==`semio-${set.origin}`)return fail("name semio-<origin>");
 if(set.hostLabel!==null&&(typeof set.hostLabel!=="string"||!/^[a-z0-9][a-z0-9-]{0,62}$/u.test(set.hostLabel)))return fail("hostLabel");
 const advertise=set.advertise as Record<string,unknown>|null;
 if(typeof advertise!=="object"||advertise===null||Object.keys(advertise).sort().join()!=="duplicates,enabled,namedAs"||typeof advertise.enabled!=="boolean"||(advertise.namedAs!==null&&(typeof advertise.namedAs!=="string"||!ORIGIN.test(advertise.namedAs)))||!Array.isArray(advertise.duplicates)||advertise.duplicates.some(item=>typeof item!=="string"||!ORIGIN.test(item)))return fail("advertise");
 for(const name of ["observedAtMs","ttlMs","generation"])if(!Number.isInteger(set[name])||(set[name] as number)<0)return fail(name);
 if((set.ttlMs as number)<1000||(set.generation as number)<1)return fail("ttlMs and generation");
 if(!Array.isArray(set.addresses)||set.addresses.length>32)return fail("addresses");
 for(const row of set.addresses as unknown[]){
  if(typeof row!=="object"||row===null)return fail("address row");
  const address=row as Record<string,unknown>;
  if(Object.keys(address).sort().join()!==["address","family","interface","prefix","scope"].join())return fail("address keys");
  if(typeof address.address!=="string"||fleetLinkLocalV1(address.address)||fleetLoopbackV1(address.address))return fail("address");
  if(address.family!==(quad(address.address)!==undefined?"IPv4":"IPv6"))return fail("family");
  if(typeof address.interface!=="string"||address.interface==="")return fail("interface");
  if(!Number.isInteger(address.prefix)||(address.prefix as number)<0||(address.prefix as number)>128)return fail("prefix");
  if(address.scope!==fleetAddressScopeV1(address.address))return fail("scope");
 }
 return set as unknown as FleetAddressSetV1;
}

/** 📂️ Where the set of an origin lives: `<store>/v1/origins/<origin>/addresses.json`. */
export function fleetAddressPathV1(storeRoot:string,origin:string):string{
 return join(storeRoot,"v1","origins",origin,"addresses.json");
}

const writeAtomic=async(path:string,text:string):Promise<void>=>{
 const temporary=`${path}.${process.pid}-${randomUUID()}.tmp`;
 await mkdir(dirname(path),{recursive:true});
 try{await writeFile(temporary,text,{mode:0o644});await rename(temporary,path);}catch(error){await rm(temporary,{force:true}).catch(()=>{});throw error;}
};

/** 📖️ Reads the published set of an origin; `undefined` when absent or not of the closed shape; `fresh` says whether its time to live has not passed. */
export async function readFleetAddressSetV1(storeRoot:string,origin:string,now:number=Date.now()):Promise<Readonly<{set:FleetAddressSetV1;fresh:boolean}>|undefined>{
 try{
  const set=requireFleetAddressSetV1(JSON.parse(await readFile(fleetAddressPathV1(storeRoot,origin),"utf8")));
  return set.origin===origin?{set,fresh:now<set.observedAtMs+set.ttlMs}:undefined;
 }catch{return undefined;}
}

/** 📚️ The published sets of every origin of a store that has one (at most 512), for the checks that compare machines with each other. */
export async function readFleetAddressSetsV1(storeRoot:string,now:number=Date.now()):Promise<readonly FleetAddressSetV1[]>{
 try{
  const names=(await readdir(join(storeRoot,"v1","origins"))).filter(name=>ORIGIN.test(name)).slice(0,512);
  return (await Promise.all(names.map(name=>readFleetAddressSetV1(storeRoot,name,now)))).flatMap(row=>row===undefined?[]:[row.set]);
 }catch{return [];}
}

/** 🕵️ The origins of a store that show a sign of life (`ready.json` younger than `withinMs`, default ten minutes) but have no published set among `known`: their host names cannot be compared yet. */
export async function readFleetUnverifiedOriginsV1(storeRoot:string,known:readonly FleetAddressSetV1[],now:number=Date.now(),withinMs:number=600_000):Promise<readonly string[]>{
 try{
  const have=new Set(known.map(set=>set.origin)),names=(await readdir(join(storeRoot,"v1","origins"))).filter(name=>ORIGIN.test(name)&&!have.has(name)).slice(0,512),found:string[]=[];
  for(const name of names)try{if(now-(await stat(join(storeRoot,"v1","origins",name,"ready.json"))).mtimeMs<withinMs)found.push(name);}catch{}
  return found;
 }catch{return [];}
}

/** 📣️ Publishes the set of this machine atomically (temporary file, then rename). It writes only when the addresses changed or the stamp is older than half of the time to live, and raises the generation exactly when the content changed. */
export async function publishFleetAddressSetV1(input:Readonly<{storeRoot:string;origin:string;interfaces:ReturnType<typeof networkInterfaces>;now?:number;ttlMs?:number;hostLabel?:string|null;advertise?:FleetAdvertiseStateV1}>):Promise<Readonly<{changed:boolean;refreshed:boolean;set:FleetAddressSetV1}>>{
 const now=input.now??Date.now(),previous=await readFleetAddressSetV1(input.storeRoot,input.origin,now),draft=createFleetAddressSetV1({origin:input.origin,interfaces:input.interfaces,now,...input.ttlMs===undefined?{}:{ttlMs:input.ttlMs},...input.hostLabel===undefined?{}:{hostLabel:input.hostLabel},...input.advertise===undefined?{}:{advertise:input.advertise}});
 const same=previous!==undefined&&fleetAddressSignatureV1(previous.set)===fleetAddressSignatureV1(draft)&&previous.set.ttlMs===draft.ttlMs;
 if(same&&now-previous!.set.observedAtMs<previous!.set.ttlMs/2)return {changed:false,refreshed:false,set:previous!.set};
 const set:FleetAddressSetV1={...draft,generation:same?previous!.set.generation:(previous?.set.generation??0)+1};
 await writeAtomic(fleetAddressPathV1(input.storeRoot,input.origin),JSON.stringify(set,null,1)+"\n");
 return {changed:!same,refreshed:same,set};
}

/** 👀️ Keeps the set of this machine current: checks the interfaces every `intervalMs` (default 15 s), republishes on a change or at half of the time to live, and says so through `report` in the given language. It ends with the signal and never throws: a store that cannot be written is retried on the next turn. */
export async function watchFleetAddressSetV1(input:Readonly<{storeRoot:string;origin:string;hostname?:string;interfaces:()=>ReturnType<typeof networkInterfaces>;signal:AbortSignal;language:FleetAddressLanguageV1;report:(line:string)=>void;intervalMs?:number;now?:()=>number;wait?:(milliseconds:number,signal:AbortSignal)=>Promise<void>}>):Promise<void>{
 const wait=input.wait??((milliseconds:number,signal:AbortSignal)=>new Promise<void>(resolve=>{const timer=setTimeout(resolve,milliseconds);signal.addEventListener("abort",()=>{clearTimeout(timer);resolve();},{once:true});}));
 let first=true;
 while(!input.signal.aborted){
  try{
   const hostLabel=input.hostname===undefined?null:fleetHostLabelV1(input.hostname),others=await readFleetAddressSetsV1(input.storeRoot),advertise=fleetAdvertiseGateV1({origin:input.origin,hostLabel,others,unverified:(await readFleetUnverifiedOriginsV1(input.storeRoot,others,input.now?.())).filter(name=>name!==input.origin)}),result=await publishFleetAddressSetV1({storeRoot:input.storeRoot,origin:input.origin,interfaces:input.interfaces(),hostLabel,advertise,...input.now===undefined?{}:{now:input.now()}});
   if(result.changed){
    input.report(fleetAddressTextV1(input.language,first?"addresses.published":"addresses.changed",{count:result.set.addresses.length,origin:input.origin,generation:result.set.generation}));
    if(advertise.namedAs!==null)input.report(fleetAddressTextV1(input.language,"mdns.misnamed",{label:hostLabel??"",named:advertise.namedAs,origin:input.origin}));
    if(!advertise.enabled&&advertise.duplicates.length>0)input.report(fleetAddressTextV1(input.language,"mdns.silent",{label:hostLabel??"",duplicates:advertise.duplicates.join(", "),origin:input.origin,type:FLEET_SERVICE_TYPE_V1}));
    if(result.set.addresses.length===0)input.report(fleetAddressTextV1(input.language,"addresses.none",{origin:input.origin,service:FLEET_SERVICE_TYPE_V1}));
   }
   first=false;
  }catch{}
  await wait(input.intervalMs??15_000,input.signal);
 }
}

/** 🔊️ The wildcard host a server binds so that it answers on every interface, present now or added later (an IPv6 wildcard also takes IPv4 where the system allows both): `::` when this machine can listen on it, else the IPv4 wildcard. */
export async function fleetBindHostV1():Promise<"::"|"0.0.0.0">{
 return await new Promise<"::"|"0.0.0.0">(resolve=>{
  const probe=createServer();
  probe.once("error",()=>resolve("0.0.0.0"));
  probe.listen({port:0,host:"::"},()=>{probe.close(()=>resolve("::"));});
 });
}

const bits=(address:string):string|undefined=>{
 const v4=quad(address);
 if(v4!==undefined)return v4.map(octet=>octet.toString(2).padStart(8,"0")).join("");
 const v6=groups(address);
 return v6===undefined?undefined:v6.map(group=>group.toString(2).padStart(16,"0")).join("");
};

/** 🧮️ Whether two addresses lie in the same network of `prefix` bits (same family only). */
export function fleetSameNetworkV1(left:string,right:string,prefix:number):boolean{
 const a=bits(left),b=bits(right);
 return a!==undefined&&b!==undefined&&a.length===b.length&&prefix<=a.length&&a.slice(0,prefix)===b.slice(0,prefix);
}

const SCOPE_RANK:Readonly<Record<FleetAddressV1["scope"],number>>={private:0,"unique-local":1,global:2};

/** 📶️ The addresses of a peer in the order a client tries them: those in a network the client is also in first (a shared subnet answers fastest and survives a gateway change), then by scope (private, unique-local, global), IPv4 before IPv6, and every tie by the published order. */
export function orderFleetAddressesV1(peer:readonly FleetAddressV1[],local:readonly FleetAddressV1[]):readonly FleetAddressV1[]{
 const shared=(row:FleetAddressV1):boolean=>local.some(mine=>mine.family===row.family&&fleetSameNetworkV1(mine.address,row.address,Math.min(mine.prefix,row.prefix)));
 return peer.map((row,index)=>({row,index,rank:[shared(row)?0:1,SCOPE_RANK[row.scope],row.family==="IPv4"?0:1]})).sort((left,right)=>{
  for(let step=0;step<left.rank.length;step++)if(left.rank[step]!==right.rank[step])return left.rank[step]!-right.rank[step]!;
  return left.index-right.index;
 }).map(item=>item.row);
}

/** 🌍️ The host part of a URL for an address: IPv6 goes in square brackets. */
export function fleetUrlHostV1(address:string):string{
 return address.includes(":")?`[${address}]`:address;
}

/** 🧭️ The endpoints to try for one service of a peer from what is known without asking the network, most promising first: the last good endpoint, then the published addresses ordered by `orderFleetAddressesV1`. */
export function fleetEndpointCandidatesV1(input:Readonly<{set:FleetAddressSetV1|undefined;local:readonly FleetAddressV1[];port:number;scheme?:"http"|"https";lastGood?:string}>):readonly string[]{
 const scheme=input.scheme??"http",url=(host:string):string=>`${scheme}://${host}:${input.port}`;
 const all=[...new Set(input.set===undefined?[]:orderFleetAddressesV1(input.set.addresses,input.local).map(row=>url(fleetUrlHostV1(row.address))))];
 return input.lastGood!==undefined&&all.includes(input.lastGood)?[input.lastGood,...all.filter(endpoint=>endpoint!==input.lastGood)]:all;
}

/** 🛤️ By which path an endpoint was found: `last-good` (remembered from the previous run), `published` (addresses in the shared store), `mirror` (the local copy of those addresses while the store is unreadable) or `dns-sd` (service discovery on the local network). */
export type FleetResolutionPathV1="last-good"|"published"|"mirror"|"dns-sd";

/** 📌️ What `connectFleetEndpointV1` found: the first endpoint that answered and every attempt made on the way. */
export type FleetConnectionV1=Readonly<{endpoint:string;attempts:readonly Readonly<{endpoint:string;ok:boolean;milliseconds:number}>[]}>;

/** 🔌️ Tries the candidates in order, each with its own time limit (default 1500 ms), and returns the first for which `probe` is true; a probe that throws or times out counts as unreachable and the next candidate is tried. `undefined` when none answers. It never waits longer than the number of candidates times the limit. */
export async function connectFleetEndpointV1(input:Readonly<{candidates:readonly string[];probe:(endpoint:string,signal:AbortSignal)=>Promise<boolean>;timeoutMs?:number;signal?:AbortSignal;report?:(line:string)=>void;language?:FleetAddressLanguageV1}>):Promise<FleetConnectionV1|undefined>{
 const limit=input.timeoutMs??1500,attempts:{endpoint:string;ok:boolean;milliseconds:number}[]=[];
 for(const endpoint of input.candidates){
  if(input.signal?.aborted===true)return undefined;
  const started=Date.now(),own=new AbortController(),timer=setTimeout(()=>own.abort(),limit),relay=()=>own.abort();
  input.signal?.addEventListener("abort",relay,{once:true});
  let ok=false;
  try{ok=await Promise.race([input.probe(endpoint,own.signal),new Promise<boolean>(resolve=>own.signal.addEventListener("abort",()=>resolve(false),{once:true}))]);}catch{ok=false;}
  finally{clearTimeout(timer);input.signal?.removeEventListener("abort",relay);}
  attempts.push({endpoint,ok,milliseconds:Date.now()-started});
  if(ok)return {endpoint,attempts};
  if(input.report!==undefined&&input.language!==undefined)input.report(fleetAddressTextV1(input.language,"addresses.failover",{failed:endpoint,milliseconds:limit}));
 }
 return undefined;
}

/** 📍️ Where the last good endpoint per service is remembered on this machine: `<root>/.cache/f/last-good.json`. */
export function fleetLastGoodPathV1(root:string):string{
 return join(root,".cache","f","last-good.json");
}

/** 📖️ The remembered last good endpoint of a service key (`<origin>/<service>`), or `undefined`. */
export async function readFleetLastGoodV1(root:string,key:string):Promise<string|undefined>{
 try{const value=(JSON.parse(await readFile(fleetLastGoodPathV1(root),"utf8")) as Record<string,unknown>)[key];return typeof value==="string"?value:undefined;}catch{return undefined;}
}

/** 💾️ Remembers the last good endpoint of a service key atomically; a failure to write is not an error of the caller. */
export async function writeFleetLastGoodV1(root:string,key:string,endpoint:string):Promise<void>{
 try{
  let current:Record<string,unknown>={};
  try{current=JSON.parse(await readFile(fleetLastGoodPathV1(root),"utf8")) as Record<string,unknown>;}catch{}
  if(current[key]===endpoint)return;
  await writeAtomic(fleetLastGoodPathV1(root),JSON.stringify({...current,[key]:endpoint},null,1)+"\n");
 }catch{}
}

/** 🪞️ Where this machine keeps the last published address set of an origin that it read from the store: `<root>/.cache/f/addresses/<origin>.json`, so that a store that is unmounted or stalled does not take the way to the server with it. */
export function fleetAddressMirrorPathV1(root:string,origin:string):string{
 return join(root,".cache","f","addresses",`${origin}.json`);
}

const mirrorAddresses=async(root:string,set:FleetAddressSetV1):Promise<void>=>{
 try{
  const path=fleetAddressMirrorPathV1(root,set.origin),text=JSON.stringify(set,null,1)+"\n";
  if(await readFile(path,"utf8").catch(()=>undefined)===text)return;
  await writeAtomic(path,text);
 }catch{}
};

/** 🪞️ Reads the local copy of the published set of an origin; `undefined` when there is none. */
export const readFleetAddressMirrorV1=async(root:string,origin:string):Promise<FleetAddressSetV1|undefined>=>{
 try{const set=requireFleetAddressSetV1(JSON.parse(await readFile(fleetAddressMirrorPathV1(root,origin),"utf8")));return set.origin===origin?set:undefined;}catch{return undefined;}
};

/** 📛️ The roles of a machine and the port of each (0 for a role that listens on none), which its DNS-SD advertisement carries as TXT records. */
export type FleetServiceRolesV1=Readonly<Record<string,number>>;

/** 📡️ One DNS-SD advertisement: the instance (the origin id), the service type, the port and the TXT records (`name`, `group`, `roles` and one `<role>=<port>` per role). */
export type FleetServiceRecordV1=Readonly<{instance:string;type:string;port:number;txt:readonly string[]}>;

/** 🏗️ Builds the DNS-SD advertisement of a machine from its origin, its group and its roles; the service port is the first non-zero role port in role order. */
export function fleetServiceRecordV1(input:Readonly<{origin:string;group:string;roles:FleetServiceRolesV1}>):FleetServiceRecordV1{
 const entries=Object.entries(input.roles).sort(([left],[right])=>left.localeCompare(right));
 if(entries.length===0||entries.some(([role,port])=>!/^[a-z][a-z0-9-]{0,31}$/u.test(role)||!Number.isInteger(port)||port<0||port>65535))throw Error("Declared fleet roles with ports required");
 return {instance:input.origin,type:FLEET_SERVICE_TYPE_V1,port:entries.find(([,port])=>port>0)?.[1]??0,txt:[`origin=${input.origin}`,`name=${fleetNameV1(input.origin)}`,`group=${input.group}`,`roles=${entries.map(([role])=>role).join(",")}`,...entries.map(([role,port])=>`${role}=${port}`)]};
}

/** 🛠️ The platform tool and arguments that advertise a record without elevated rights: `dns-sd -R` (Bonjour on macOS, `dns-sd.exe` on Windows) or `avahi-publish -s` (Linux). */
export function fleetAdvertiseCommandV1(record:FleetServiceRecordV1,platform:NodeJS.Platform=process.platform):Readonly<{command:string;args:readonly string[]}>{
 return platform==="linux"?{command:"avahi-publish",args:["-s",record.instance,record.type,String(record.port),...record.txt]}:{command:platform==="win32"?"dns-sd.exe":"dns-sd",args:["-R",record.instance,record.type,"local",String(record.port),...record.txt]};
}

const discoveryOff=():boolean=>process.env.SEMIO_FLEET_DISCOVERY==="off"||process.env.NODE_ENV==="test";

const runTool=(command:string,args:readonly string[],signal:AbortSignal,onStart:()=>void):Promise<number>=>new Promise<number>(resolve=>{
 if(discoveryOff()){resolve(-1);return;}
 try{
  const child=spawnProcess(command,[...args],{stdio:"ignore",windowsHide:true,signal});
  child.once("spawn",onStart);
  child.once("error",error=>resolve((error as NodeJS.ErrnoException).code==="ENOENT"?-1:-2));
  child.once("exit",code=>resolve(code??0));
 }catch{resolve(-1);}
});

/** 📣️ Keeps the DNS-SD advertisement of this machine in step with the conflict gate: while `enabled()` says yes (checked every 15 s) the platform tool runs, when it says no the tool is stopped and the machine is silent. A missing tool (no avahi, no Bonjour) is reported once, in the chosen language, and the machine relies on the addresses published in the store. It never advertises in a test run or with `SEMIO_FLEET_DISCOVERY=off`, ends with the signal and never throws. */
export async function advertiseFleetServiceV1(input:Readonly<{record:FleetServiceRecordV1;signal:AbortSignal;language:FleetAddressLanguageV1;report:(line:string)=>void;enabled:()=>Promise<boolean>;platform?:NodeJS.Platform;run?:(command:string,args:readonly string[],signal:AbortSignal,onStart:()=>void)=>Promise<number>;wait?:(milliseconds:number,signal:AbortSignal)=>Promise<void>}>):Promise<void>{
 const tool=fleetAdvertiseCommandV1(input.record,input.platform),run=input.run??runTool,wait=input.wait??((milliseconds:number,signal:AbortSignal)=>new Promise<void>(resolve=>{const timer=setTimeout(resolve,milliseconds);signal.addEventListener("abort",()=>{clearTimeout(timer);resolve();},{once:true});}));
 let child:{stop:AbortController;done:Promise<number>}|undefined,warned=false;
 const stop=async():Promise<void>=>{const old=child;child=undefined;if(old!==undefined){old.stop.abort();await old.done.catch(()=>0);}};
 try{
  while(!input.signal.aborted){
   const enabled=await input.enabled().catch(()=>false);
   if(enabled&&child===undefined){
    const own=new AbortController(),done=run(tool.command,tool.args,AbortSignal.any([input.signal,own.signal]),()=>input.report(fleetAddressTextV1(input.language,"mdns.advertising",{instance:input.record.instance,type:input.record.type,port:input.record.port,tool:tool.command}))).then(code=>{
     if(child?.stop===own)child=undefined;
     if(own.signal.aborted||input.signal.aborted)return code;
     if(code===-1){if(!warned)input.report(fleetAddressTextV1(input.language,"mdns.unavailable",{tool:tool.command}));warned=true;}
     else input.report(fleetAddressTextV1(input.language,"mdns.ended",{tool:tool.command,code,seconds:15}));
     return code;
    });
    child={stop:own,done};
   }else if(!enabled&&child!==undefined){await stop();input.report(fleetAddressTextV1(input.language,"mdns.stopped",{instance:input.record.instance}));}
   await wait(15_000,input.signal);
  }
 }finally{await stop();}
}

/** 🪪️ The origin id of this machine from `.cache/f/machine.json` of its worktree; a missing or unreadable file is refused with the way out, in both languages. */
export function readFleetOwnOriginV1(root:string,language:FleetAddressLanguageV1):string{
 const path=join(root,".cache","f","machine.json");
 try{
  const value=(JSON.parse(readFileSync(path,"utf8")) as {originId?:unknown}).originId;
  if(typeof value==="string"&&ORIGIN.test(value))return value;
 }catch{}
 throw Error(fleetAddressTextV1(language,"hostname.missing",{path}));
}

/** 🏷️ The command that sets the host name of a machine to `semio-<origin>` on its platform (macOS `scutil --set LocalHostName`, Linux `hostnamectl set-hostname`), or `undefined` where it is not automated. */
export function fleetHostnameCommandV1(name:string,platform:NodeJS.Platform=process.platform):Readonly<{command:string;args:readonly string[]}>|undefined{
 return platform==="darwin"?{command:"sudo",args:["scutil","--set","LocalHostName",name]}:platform==="linux"?{command:"sudo",args:["hostnamectl","set-hostname",name]}:undefined;
}

const currentHostname=(platform:NodeJS.Platform):string|undefined=>{
 const run=platform==="darwin"?spawnSync("scutil",["--get","LocalHostName"],{encoding:"utf8"}):spawnSync("hostname",[],{encoding:"utf8"});
 return run.status===0?run.stdout.trim():undefined;
};

/** 🏷️ Gives this machine the host name `semio-<origin>` derived from its own origin id (never typed by hand): it refuses without `.cache/f/machine.json`, says nothing to do when the name is already right, prints the planned command and runs it only after `confirm` says yes. The single way the fleet ever changes a host name; it never renames silently. Returns what happened. */
export async function applyFleetHostnameV1(input:Readonly<{root:string;language:FleetAddressLanguageV1;report:(line:string)=>void;confirm:(question:string)=>Promise<boolean>;run:(command:string,args:readonly string[])=>Promise<number>;platform?:NodeJS.Platform;current?:()=>string|undefined}>):Promise<"already"|"applied"|"declined"|"unsupported">{
 const platform=input.platform??process.platform,origin=readFleetOwnOriginV1(input.root,input.language),name=fleetNameV1(origin),plan=fleetHostnameCommandV1(name,platform);
 if(plan===undefined){input.report(fleetAddressTextV1(input.language,"hostname.unsupported",{platform,name}));return "unsupported";}
 const current=(input.current??(()=>currentHostname(platform)))();
 if(current!==undefined&&current.toLowerCase()===name)return input.report(fleetAddressTextV1(input.language,"hostname.already",{name,origin})),"already";
 const line=[plan.command,...plan.args].join(" ");
 input.report(fleetAddressTextV1(input.language,"hostname.plan",{origin,name,command:line}));
 if(!await input.confirm(fleetAddressTextV1(input.language,"hostname.confirm",{name}))){input.report(fleetAddressTextV1(input.language,"hostname.declined"));return "declined";}
 const code=await input.run(plan.command,plan.args);
 if(code!==0)throw Error(fleetAddressTextV1(input.language,"hostname.failed",{command:line,code}));
 input.report(fleetAddressTextV1(input.language,"hostname.done",{name}));
 return "applied";
}

/** 🔎️ What service discovery found for one instance: its addresses (loopback and link-local left out), its service port and its TXT records. */
export type FleetDiscoveryV1=Readonly<{addresses:readonly string[];port:number|null;txt:Readonly<Record<string,string>>}>;

const textRecords=(line:string):Record<string,string>=>Object.fromEntries(line.trim().split(/\s+/u).filter(token=>/^[A-Za-z0-9_-]+=/u.test(token)).map(token=>{const at=token.indexOf("=");return [token.slice(0,at),token.slice(at+1)];}));

/** 🧩️ Reads the answer of `dns-sd -L <instance> <type> local`: the target host, the port and the TXT records. */
export function parseDnsSdLookupV1(text:string):Readonly<{host:string;port:number;txt:Readonly<Record<string,string>>}>|undefined{
 const lines=text.split("\n");
 for(let index=0;index<lines.length;index++){
  const match=/can be reached at (\S+?)\.?:(\d+)/u.exec(lines[index]!);
  if(match!==null)return {host:match[1]!,port:Number(match[2]),txt:textRecords(lines[index+1]??"")};
 }
 return undefined;
}

const usable=(address:string):boolean=>(quad(address)!==undefined||groups(address)!==undefined)&&!fleetLoopbackV1(address)&&!fleetLinkLocalV1(address);

/** 🧩️ Reads the answer of `dns-sd -G v4v6 <host>`: every added address, once. */
export function parseDnsSdAddressesV1(text:string):readonly string[]{
 const found:string[]=[];
 for(const line of text.split("\n")){
  const tokens=line.trim().split(/\s+/u);
  if(!tokens.includes("Add"))continue;
  const address=tokens.map(token=>token.split("%")[0]!).find(token=>usable(token));
  if(address!==undefined&&!found.includes(address))found.push(address);
 }
 return found;
}

/** 🧩️ Reads the answer of `avahi-browse -rtp <type>` for one instance. */
export function parseAvahiBrowseV1(text:string,instance:string):FleetDiscoveryV1|undefined{
 const addresses:string[]=[];
 let port:number|null=null,txt:Record<string,string>={};
 for(const line of text.split("\n")){
  const fields=line.split(";");
  if(fields[0]!=="="||fields[3]!==instance)continue;
  const address=(fields[7]??"").split("%")[0]!;
  if(usable(address)&&!addresses.includes(address))addresses.push(address);
  port=Number(fields[8]);
  txt={...txt,...textRecords((fields[9]??"").replaceAll('"',""))};
 }
 return addresses.length===0?undefined:{addresses,port,txt};
}

const collectTool=(command:string,args:readonly string[],milliseconds:number):Promise<string>=>new Promise<string>(resolve=>{
 let out="";
 if(discoveryOff()){resolve(out);return;}
 try{
  const child=spawnProcess(command,[...args],{stdio:["ignore","pipe","ignore"],windowsHide:true});
  const timer=setTimeout(()=>{child.kill();},milliseconds);
  child.stdout!.on("data",chunk=>{out+=String(chunk);});
  child.once("error",()=>{clearTimeout(timer);resolve(out);});
  child.once("close",()=>{clearTimeout(timer);resolve(out);});
 }catch{resolve(out);}
});

/** 🔭️ Finds one machine by its instance name (the origin id) through the platform's service discovery without elevated rights: `avahi-browse` on Linux, `dns-sd -L` and `dns-sd -G` on macOS and Windows. `undefined` when the tool is missing, mDNS is blocked or the machine does not advertise; the caller then relies on the addresses published in the store. */
export async function discoverFleetServiceV1(input:Readonly<{instance:string;platform?:NodeJS.Platform;timeoutMs?:number;run?:(command:string,args:readonly string[],milliseconds:number)=>Promise<string>}>):Promise<FleetDiscoveryV1|undefined>{
 const platform=input.platform??process.platform,run=input.run??collectTool,limit=input.timeoutMs??1500;
 if(platform==="linux")return parseAvahiBrowseV1(await run("avahi-browse",["-rtp",FLEET_SERVICE_TYPE_V1],limit),input.instance);
 const command=platform==="win32"?"dns-sd.exe":"dns-sd",found=parseDnsSdLookupV1(await run(command,["-L",input.instance,FLEET_SERVICE_TYPE_V1,"local"],limit));
 if(found===undefined)return undefined;
 const addresses=parseDnsSdAddressesV1(await run(command,["-G","v4v6",found.host],limit));
 return addresses.length===0?undefined:{addresses,port:found.port,txt:found.txt};
}

/** 🔌️ A probe that counts an endpoint as answering when a TCP connection to its host and port opens (for services that do not speak HTTP, such as the NFS port of the store). */
export function fleetTcpProbeV1(endpoint:string,signal:AbortSignal):Promise<boolean>{
 return new Promise<boolean>(resolve=>{
  let url:URL;
  try{url=new URL(endpoint);}catch{resolve(false);return;}
  const socket=createConnection({host:url.hostname.replace(/^\[|\]$/gu,""),port:Number(url.port)});
  const done=(ok:boolean):void=>{signal.removeEventListener("abort",abort);socket.destroy();resolve(ok);},abort=():void=>done(false);
  signal.addEventListener("abort",abort,{once:true});
  socket.once("connect",()=>done(true));
  socket.once("error",()=>done(false));
 });
}

/** 🧭️ THE resolver of every client of a fleet service: finds one service of one machine, named by its origin id, in one call and with one failover policy. Order: the endpoint that answered last time, then the addresses the machine published in the store (a copy of them kept on this machine serves while the store is unreadable), same subnet first; then DNS-SD discovery of the instance. A published set that is stale puts discovery before its addresses, because the machine may have moved to another network. Each attempt has its own time limit. The answer is remembered; the path that found it (`via`) is returned (a remembered endpoint on another port than the published one is forgotten) and, when it is not the steady state of the same endpoint found again, reported in the chosen language; `SEMIO_FLEET_DISCOVERY=off` switches service discovery off (tests, hosts without multicast); `storeRoot` may be left out, then only the local copy and discovery are used (a stalled store is never read); `undefined` when nothing answers, so the caller falls back to local work. */
export async function resolveFleetEndpointV1(input:Readonly<{root:string;storeRoot?:string;origin:string;service:string;port:number;local?:readonly FleetAddressV1[];probe:(endpoint:string,signal:AbortSignal)=>Promise<boolean>;discover?:(instance:string)=>Promise<readonly string[]>;timeoutMs?:number;signal?:AbortSignal;now?:number;report?:(line:string)=>void;language?:FleetAddressLanguageV1}>):Promise<Readonly<FleetConnectionV1&{via:FleetResolutionPathV1}>|undefined>{
 const key=`${input.origin}/${input.service}`,local=input.local??collectFleetAddressesV1(networkInterfaces()),read=input.storeRoot===undefined?undefined:await readFleetAddressSetV1(input.storeRoot,input.origin,input.now);
 if(read!==undefined)await mirrorAddresses(input.root,read.set);
 const mirrored=read===undefined?await readFleetAddressMirrorV1(input.root,input.origin):undefined,set=read?.set??mirrored,fresh=read?.fresh??(mirrored!==undefined&&(input.now??Date.now())<mirrored.observedAtMs+mirrored.ttlMs),remembered=await readFleetLastGoodV1(input.root,key),lastGood=remembered!==undefined&&(()=>{try{return Number(new URL(remembered).port||80)===input.port;}catch{return false;}})()?remembered:undefined;
 const known=fleetEndpointCandidatesV1({set,local,port:input.port}),scheme=lastGood===undefined||!lastGood.startsWith("https")?"http":"https";
 const discover=input.discover??(async(instance:string):Promise<readonly string[]>=>discoveryOff()?[]:(await discoverFleetServiceV1({instance}))?.addresses??[]);
 const discovered=async():Promise<readonly string[]>=>{try{return (await discover(input.origin)).map(address=>`${scheme}://${fleetUrlHostV1(address)}:${input.port}`);}catch{return [];}};
 const attempts:{endpoint:string;ok:boolean;milliseconds:number}[]=[],tried=new Set<string>();
 const stage=async(candidates:readonly string[],via:FleetResolutionPathV1):Promise<Readonly<FleetConnectionV1&{via:FleetResolutionPathV1}>|undefined>=>{
  const fresh_=candidates.filter(endpoint=>!tried.has(endpoint));
  for(const endpoint of fresh_)tried.add(endpoint);
  if(fresh_.length===0)return undefined;
  const found=await connectFleetEndpointV1({candidates:fresh_,probe:input.probe,...input.timeoutMs===undefined?{}:{timeoutMs:input.timeoutMs},...input.signal===undefined?{}:{signal:input.signal},...input.report===undefined?{}:{report:input.report},...input.language===undefined?{}:{language:input.language}});
  if(found===undefined){for(const endpoint of fresh_)attempts.push({endpoint,ok:false,milliseconds:input.timeoutMs??1500});return undefined;}
  attempts.push(...found.attempts);
  return {endpoint:found.endpoint,attempts:[...attempts],via};
 };
 const stored=mirrored!==undefined?"mirror":"published";
 let found=lastGood===undefined||(set!==undefined&&fresh&&!known.includes(lastGood))?undefined:await stage([lastGood],"last-good");
 if(found===undefined&&fresh)found=await stage(known,stored);
 if(found===undefined)found=await stage(await discovered(),"dns-sd");
 if(found===undefined&&!fresh)found=await stage(known,stored);
 if(found===undefined){
  if(input.report!==undefined&&input.language!==undefined)input.report(fleetAddressTextV1(input.language,"addresses.unreachable",{origin:input.origin,count:tried.size}));
  return undefined;
 }
 await writeFleetLastGoodV1(input.root,key,found.endpoint);
 if(input.report!==undefined&&input.language!==undefined&&(found.via!=="last-good"||found.endpoint!==lastGood))input.report(fleetAddressTextV1(input.language,"addresses.resolved",{origin:input.origin,endpoint:found.endpoint,via:found.via}));
 return found;
}
