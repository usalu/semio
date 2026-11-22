import {createHash,randomBytes} from "node:crypto";
import {mkdir,readdir,readFile,rename,rm,stat,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {platformClassV1} from "../../🪪️graph-state/🌍️platform-class/🟦️.ts";
import {fleetCacheFactsPathV1,parseFleetCacheFactsV1} from "../🛰️fleet/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🧭️ The header of a read that a group server asks of a peer group: the answering server never reads through to its own peers, so two groups cannot ask each other in a circle. */
export const FLEET_HOP_HEADER_V1="x-semio-hop";

/** 📨️ What a peer group answered: a status (200 with the bytes, 404 for a miss, anything else for a refusal). */
export type FleetPeerAnswerV1=Readonly<{status:number;bytes?:Uint8Array}>;

/** 🔌️ How one peer group is reached. Everything that travels is content-addressed (cache blobs, action results of graph generations and wanted requests, Nx archives), so a read can go by two routes that give the same bytes: `direct` (a routable address and an authenticated HTTP request) and `relayed` (a request file in the store of the asking group, answered through the stores that the group servers mount of each other). */
export interface FleetPeerTransportV1{
 readonly group:string;
 readonly kind:"direct"|"relayed";
 routed():Promise<boolean>;
 get(path:string,signal:AbortSignal):Promise<FleetPeerAnswerV1>;
}

/** 📊️ What the read-through did since it started, for `GET /status`; `absent` is the number of configured peer groups that have no route right now (no attested endpoint, no readable store): they are never asked, so they are neither a miss nor an error. */
export type FleetPeerCountersV1=Readonly<{hits:number;misses:number;errors:number;skipped:number;absent:number;bytes:number}>;

/** 🔁️ Asks the peer groups for a read that the own cache could not answer. */
export interface FleetPeerReadThroughV1{
 fetch(path:string):Promise<Readonly<{group:string;bytes:Uint8Array}>|undefined>;
 counters():FleetPeerCountersV1;
}

const sha=(bytes:Uint8Array|string):string=>createHash("sha256").update(bytes).digest("hex");
const wait=(milliseconds:number,signal:AbortSignal):Promise<void>=>new Promise(done=>{if(signal.aborted)return done();const timer=setTimeout(done,milliseconds);signal.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});});
const readable=(path:string):boolean=>validateJsonSchemaSubset({$ref:"#/definitions/Path"},path,schema).length===0;

/** 🛡️ Validates one value against a definition of the exchange schema and names the first problem. */
export function requireFleetPeerShapeV1(definition:"Request"|"Answer"|"Path",value:unknown,message:string):void{
 const errors=validateJsonSchemaSubset({$ref:`#/definitions/${definition}`},value,schema);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

/** 🌐️ The endpoints of a peer group by its attested facts (`<peer store>/v1/fleet/cache.json`, as published by its group server and copied by the relay) followed by the endpoints that the topology lists for the link; none when neither exists. */
export async function fleetPeerEndpointsV1(peerStore:string|undefined,listed:readonly string[]=[]):Promise<readonly string[]>{
 let attested:readonly string[]=[];
 if(peerStore!==undefined)try{attested=parseFleetCacheFactsV1(await readFile(fleetCacheFactsPathV1(peerStore),"utf8")).cache.endpoints;}catch{attested=[];}
 return [...new Set([...attested,...listed])];
}

/** 🛰️ The direct route: a read request to the first endpoint of the peer that answers, signed by the caller's `fetch` (the group server signs with its own `cache-read` key); the answer is bounded in size and time. */
export function directPeerTransportV1(options:Readonly<{group:string;endpoints:()=>Promise<readonly string[]>;fetch:typeof fetch;maximumBytes:number}>):FleetPeerTransportV1{
 return Object.freeze<FleetPeerTransportV1>({group:options.group,kind:"direct" as const,async routed(){return(await options.endpoints()).length>0;},async get(path,signal){
  requireFleetPeerShapeV1("Path",path,"Readable cache path required");
  for(const endpoint of await options.endpoints()){
   let response:Response;
   try{response=await options.fetch(endpoint+path,{signal,headers:{[FLEET_HOP_HEADER_V1]:"1"}});}catch{continue;}
   if(response.status===404){await response.body?.cancel();return {status:404};}
   if(response.status!==200){await response.body?.cancel();return {status:response.status};}
   const length=Number(response.headers.get("content-length"));
   if(Number.isFinite(length)&&length>options.maximumBytes){await response.body?.cancel();return {status:413};}
   const bytes=new Uint8Array(await response.arrayBuffer());
   return bytes.byteLength>options.maximumBytes?{status:413}:{status:200,bytes};
  }
  return {status:503};
 }});
}

const exchangeDirectory=(store:string,peer:string,part:"requests"|"answers"):string=>join(store,"v1","fleet","exchange",peer,part);

async function write(path:string,bytes:Uint8Array|string):Promise<void>{
 const temporary=`${path}.${process.pid}-${randomBytes(4).toString("hex")}.tmp`;
 await mkdir(dirname(path),{recursive:true});
 try{await writeFile(temporary,bytes);await rename(temporary,path);}catch(error){await rm(temporary,{force:true}).catch(()=>{});throw error;}
}

/** 📮️ The relayed route: the request is a file in the own store (`v1/fleet/exchange/<peer>/requests/<id>.json`), the answer a file in the peer's store (`v1/fleet/exchange/<own>/answers/<id>.json` with its body `<id>.bin`) that this server reads through its read-only mount `peerStore`. Nothing is ever written into a store of another group. It needs no route between the two networks, only the stores the relay already mounts; the answer is checked against its declared hash and size. */
export function relayedPeerTransportV1(options:Readonly<{group:string;ownGroup:string;ownStore:string;peerStore:string;pollMilliseconds?:number;maximumBytes:number;now?:()=>number}>):FleetPeerTransportV1{
 const clock=options.now??Date.now,poll=options.pollMilliseconds??100;
 return Object.freeze<FleetPeerTransportV1>({group:options.group,kind:"relayed" as const,async routed(){return readdir(join(options.peerStore,"v1")).then(()=>true,()=>false);},async get(path,signal){
  requireFleetPeerShapeV1("Path",path,"Readable cache path required");
  const id=randomBytes(12).toString("hex"),request={version:1,id,from:options.ownGroup,to:options.group,path,createdMs:clock()},file=join(exchangeDirectory(options.ownStore,options.group,"requests"),`${id}.json`),answers=exchangeDirectory(options.peerStore,options.ownGroup,"answers");
  await write(file,JSON.stringify(request));
  try{
   while(!signal.aborted){
    let head:string|undefined;
    try{head=await readFile(join(answers,`${id}.json`),"utf8");}catch{head=undefined;}
    if(head!==undefined){
     const answer=JSON.parse(head) as {status:number;size?:number;sha256?:string};
     requireFleetPeerShapeV1("Answer",answer,"Closed exchange answer required");
     if(answer.status!==200)return {status:answer.status};
     const bytes=new Uint8Array(await readFile(join(answers,`${id}.bin`)));
     if(bytes.byteLength>options.maximumBytes)return {status:413};
     return bytes.byteLength===answer.size&&sha(bytes)===answer.sha256?{status:200,bytes}:{status:502};
    }
    await wait(poll,signal);
   }
   return {status:504};
  }finally{await rm(file,{force:true}).catch(()=>{});}
 }});
}

/** 📬️ The answering side of the relayed route, run by the group server beside its cache: reads the requests that peer groups wrote for it (through the read-only mounts `peers`), lets `answer` read the own cache, and writes each answer into the own store; answers and unclaimed requests older than `maximumAgeMilliseconds` are dropped. Returns when `signal` aborts; one broken request never stops the loop. */
export async function servePeerExchangeV1(options:Readonly<{ownGroup:string;ownStore:string;peers:Readonly<Record<string,string>>;answer:(path:string)=>Promise<FleetPeerAnswerV1>;signal:AbortSignal;pollMilliseconds?:number;maximumAgeMilliseconds?:number;now?:()=>number}>):Promise<void>{
 const clock=options.now??Date.now,poll=options.pollMilliseconds??100,limit=options.maximumAgeMilliseconds??300000,done=new Set<string>();
 while(!options.signal.aborted){
  for(const [peer,store] of Object.entries(options.peers)){
   let names:string[]=[];
   try{names=(await readdir(exchangeDirectory(store,options.ownGroup,"requests"))).filter(name=>name.endsWith(".json"));}catch{continue;}
   const answers=exchangeDirectory(options.ownStore,peer,"answers");
   for(const name of names.slice(0,64)){
    const id=name.slice(0,-5),key=`${peer}/${id}`;
    if(done.has(key))continue;
    try{
     const request=JSON.parse(await readFile(join(exchangeDirectory(store,options.ownGroup,"requests"),name),"utf8")) as {id:string;from:string;to:string;path:string;createdMs:number};
     requireFleetPeerShapeV1("Request",request,"Closed exchange request required");
     if(request.id!==id||request.from!==peer||request.to!==options.ownGroup||clock()-request.createdMs>limit){done.add(key);continue;}
     const reply=await options.answer(request.path).catch((): FleetPeerAnswerV1=>({status:502}));
     if(reply.status===200&&reply.bytes!==undefined){
      await write(join(answers,`${id}.bin`),reply.bytes);
      await write(join(answers,`${id}.json`),JSON.stringify({version:1,id,status:200,size:reply.bytes.byteLength,sha256:sha(reply.bytes),atMs:clock()}));
     }else await write(join(answers,`${id}.json`),JSON.stringify({version:1,id,status:reply.status,atMs:clock()}));
     done.add(key);
    }catch{done.add(key);}
   }
   try{
    for(const name of await readdir(answers)){const path=join(answers,name);if(clock()-(await stat(path)).mtimeMs>limit){await rm(path,{force:true});done.delete(`${peer}/${name.replace(/\.(json|bin)$/u,"")}`);}}
   }catch{}
  }
  if(done.size>4096)done.clear();
  await wait(poll,options.signal);
 }
}

/** 🧮️ The read-through over several peers: all transports are asked at once (at most `parallel`, each bounded by `timeoutMilliseconds`), the first hit wins and the others are cancelled. A peer that failed `failures` times in a row is left out for `pauseMilliseconds`; a path that every peer answered with a miss is not asked again for `missMilliseconds`. A broken or silent link therefore only costs the local group one bounded wait and never an error. */
export function createFleetPeerReadThroughV1(options:Readonly<{transports:readonly FleetPeerTransportV1[];timeoutMilliseconds?:number;missMilliseconds?:number;failures?:number;pauseMilliseconds?:number;parallel?:number;now?:()=>number}>):FleetPeerReadThroughV1{
 const clock=options.now??Date.now,timeout=options.timeoutMilliseconds??4000,remember=options.missMilliseconds??3000,failures=options.failures??3,pause=options.pauseMilliseconds??30000,parallel=options.parallel??4;
 const broken=new Map<string,{count:number;until:number}>(),misses=new Map<string,number>(),pending=new Map<string,Promise<Readonly<{group:string;bytes:Uint8Array}>|undefined>>();
 const counts={hits:0,misses:0,errors:0,skipped:0,absent:0,bytes:0};
 const ask=async(path:string):Promise<Readonly<{group:string;bytes:Uint8Array}>|undefined>=>{
  const now=clock(),routes=await Promise.all(options.transports.map(transport=>transport.routed().catch(()=>false)));
  counts.absent=routes.filter(routed=>!routed).length;
  const present=options.transports.filter((transport,index)=>routes[index]),usable=present.filter(transport=>(broken.get(transport.group)?.until??0)<=now).slice(0,parallel);
  counts.skipped+=present.length-usable.length;
  if(usable.length===0)return undefined;
  const controller=new AbortController(),timer=setTimeout(()=>controller.abort(),timeout);
  try{
   return await new Promise<Readonly<{group:string;bytes:Uint8Array}>|undefined>(resolve=>{
    let open=usable.length;
    for(const transport of usable)void transport.get(path,controller.signal).then(answer=>{
     if(answer.status===200&&answer.bytes!==undefined){broken.delete(transport.group);resolve({group:transport.group,bytes:answer.bytes});}
     else if(answer.status===404){broken.delete(transport.group);}
     else{counts.errors++;const row=broken.get(transport.group)??{count:0,until:0};row.count++;if(row.count>=failures)row.until=clock()+pause;broken.set(transport.group,row);}
    },()=>{counts.errors++;const row=broken.get(transport.group)??{count:0,until:0};row.count++;if(row.count>=failures)row.until=clock()+pause;broken.set(transport.group,row);}).finally(()=>{if(--open===0)resolve(undefined);});
   });
  }finally{clearTimeout(timer);controller.abort();}
 };
 return Object.freeze<FleetPeerReadThroughV1>({
  async fetch(path){
   if(!readable(path))return undefined;
   const known=misses.get(path);
   if(known!==undefined&&known>clock()){counts.skipped++;return undefined;}
   let running=pending.get(path);
   if(running===undefined){running=ask(path).finally(()=>{pending.delete(path);});pending.set(path,running);}
   const found=await running;
   if(found===undefined){if(counts.absent<options.transports.length)counts.misses++;misses.set(path,clock()+remember);if(misses.size>4096)for(const [name,until] of misses)if(until<=clock())misses.delete(name);return undefined;}
   counts.hits++;counts.bytes+=found.bytes.byteLength;misses.delete(path);return found;
  },
  counters:()=>({...counts}),
 });
}

/** 🏷️ The platform class a machine of an operating system (`macos`, `linux`, `windows`, as attested) works in. */
export const fleetOperatingSystemClassV1=(system:string):string=>platformClassV1(system==="macos"?"darwin-arm64":system==="linux"?"linux-x64":system==="windows"?"windows-x64":system);

/** 🙋️ The machines of the peer groups whose wanted requests this group may serve: an origin of a peer group is forwardable when no other machine of its own group, attested within `windowMilliseconds`, works in its platform class, so nobody there can answer it. The attestations are read from the stores (`v1/origins/<origin>/attest.json`): the peer's mounted store when there is one, else the copy of the relay in the own store. */
export async function forwardableRequestersV1(options:Readonly<{ownStore:string;peers:Readonly<Record<string,string>>;groups?:readonly string[];now?:()=>number;windowMilliseconds?:number}>):Promise<readonly string[]>{
 const clock=options.now??Date.now,window=options.windowMilliseconds??600000,found:string[]=[];
 for(const peer of new Set([...Object.keys(options.peers),...(options.groups??[])])){
  const rows:{id:string;system:string;live:boolean}[]=[],roots=[options.peers[peer],options.ownStore].filter((root):root is string=>root!==undefined);
  for(const root of roots){
   let names:string[]=[];
   try{names=(await readdir(join(root,"v1","origins"))).filter(name=>name.startsWith(`${peer}-`)&&name!==`${peer}-server`);}catch{continue;}
   for(const id of names){
    if(rows.some(row=>row.id===id))continue;
    try{const attest=JSON.parse(await readFile(join(root,"v1","origins",id,"attest.json"),"utf8")) as {platform?:string;wallMs?:number};rows.push({id,system:attest.platform??"",live:typeof attest.wallMs==="number"&&clock()-attest.wallMs<=window});}catch{continue;}
   }
  }
  for(const row of rows){
   if(!row.live)continue;
   const sameClass=rows.filter(other=>other.live&&other.id!==row.id&&fleetOperatingSystemClassV1(other.system)===fleetOperatingSystemClassV1(row.system));
   if(sameClass.length===0)found.push(row.id);
  }
 }
 return found.sort();
}
