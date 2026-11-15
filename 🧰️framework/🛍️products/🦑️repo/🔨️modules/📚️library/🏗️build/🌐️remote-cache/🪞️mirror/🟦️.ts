import {createHash,randomBytes} from "node:crypto";
import {createReadStream,createWriteStream} from "node:fs";
import {mkdir,readdir,readFile,rename,rm,stat,utimes,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {Readable} from "node:stream";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The authored mirror policy.
 * @see ./🧬️schema/🔣️.json */
export const MIRROR_POLICY_V1=schema.definitions.Policy.const;
/** 🗺️ The ecosystems the mirror forwards: id, upstream origin, which paths are immutable, and the tool setting that points at it. */
export const MIRROR_ECOSYSTEMS_V1=schema.definitions.Ecosystems.const;
const policy=MIRROR_POLICY_V1;
type Head=Readonly<{kind:"stream";stream:ReadableStream<Uint8Array>;meta:MirrorEntryMetaV1}>|Readonly<{kind:"revalidated";meta:MirrorEntryMetaV1}>|Readonly<{kind:"refused";status:number}>|Readonly<{kind:"failed";reason:string}>|undefined;
type Flight=Readonly<{head:Promise<Head>;done:Promise<MirrorEntryMetaV1|undefined>;joined:boolean}>;

/** 🧾️ The record kept next to a stored body. */
export type MirrorEntryMetaV1=Readonly<{version:1;path:string;status:200;contentType:string;etag?:string;bytes:number;fetchedAtMs:number;immutable:boolean}>;

/** 🔌️ What the mirror needs: where to keep entries, the budget, the upstream transport and the clock. */
export type MirrorOptionsV1=Readonly<{directory:string;maximumBytes?:number;fetch?:typeof fetch;now?:()=>number;ecosystems?:readonly (typeof MIRROR_ECOSYSTEMS_V1)[number][];slotWaitMilliseconds?:number;upstreamTimeoutMilliseconds?:number;onEvent?:(event:Readonly<{ecosystem:string;path:string;outcome:"hit"|"miss"|"revalidated"|"stale"|"error"|"joined"|"fallback";bytes:number;reason?:string}>)=>void}>;

const text=(status:number,message:string)=>new Response(message+"\n",{status,headers:{"content-type":"text/plain"}});
const TEXTS=schema.definitions.Texts.const;
const say=(key:keyof typeof TEXTS.en,fields:Readonly<Record<string,string>>)=>(["en","de"] as const).map(language=>TEXTS[language][key].replace(/\{(\w+)\}/gu,(_all,name:string)=>fields[name]??"")).join("\n");
const digest=(value:string)=>createHash("sha256").update(value).digest("hex");

/** 🪞️ Builds the mirror as a web-standard function. A request under `/mirror/<ecosystem>/…` is answered from the kept copy, or fetched once from the ecosystem's upstream: concurrent requests for one path share a single upstream fetch, an immutable path (a release archive, a package tarball, a module zip) is kept forever, any other path is kept and revalidated after `revalidateSeconds` with its etag, and a failing upstream is answered from the stale copy so an outage of the internet never breaks a machine that has seen the path before. Bodies are kept verbatim and stream through to the first requester while they are written; the clients verify the content themselves (rustup manifests, npm integrity, the Go checksum database), so the mirror needs no trust. `undefined` for every path outside the prefix. */
export function createMirrorHandlerV1(options:MirrorOptionsV1):(request:Request)=>Promise<Response|undefined>{
 const upstream=options.fetch??fetch,slotWait=options.slotWaitMilliseconds??policy.slotWaitMilliseconds,headerTimeout=options.upstreamTimeoutMilliseconds??policy.upstreamTimeoutMilliseconds,waiters:(()=>void)[]=[],now=options.now??Date.now,ecosystems=options.ecosystems??MIRROR_ECOSYSTEMS_V1,maximumBytes=options.maximumBytes??policy.maximumBytes,flights=new Map<string,Flight>();
 const patterns=new Map(ecosystems.map(row=>[row.id,row.immutable.map(source=>new RegExp(source,"u"))]));
 let lastPrune=0,pruning=false;
 const files=(key:string)=>{const base=join(options.directory,key.slice(0,2),key);return {body:base+".body",meta:base+".json"};};
 const readMeta=async(key:string):Promise<MirrorEntryMetaV1|undefined>=>{try{const value=JSON.parse(await readFile(files(key).meta,"utf8")) as MirrorEntryMetaV1;const facts=await stat(files(key).body);return value.version===1&&facts.size===value.bytes?value:undefined;}catch{return undefined;}};
 const prune=async():Promise<void>=>{
  if(pruning||now()-lastPrune<policy.pruneIntervalSeconds*1000)return;pruning=true;lastPrune=now();
  try{
   const rows:{path:string;size:number;at:number}[]=[];let total=0;
   for(const shard of await readdir(options.directory).catch(()=>[])){
    const folder=join(options.directory,shard);
    for(const name of await readdir(folder).catch(()=>[])){if(!name.endsWith(".body"))continue;const facts=await stat(join(folder,name)).catch(()=>undefined);if(facts){rows.push({path:join(folder,name),size:facts.size,at:facts.mtimeMs});total+=facts.size;}}
   }
   if(total<=maximumBytes)return;
   rows.sort((left,right)=>left.at-right.at);
   for(const row of rows){if(total<=maximumBytes*policy.lowWaterPercent/100)break;await rm(row.path,{force:true});await rm(row.path.replace(/\.body$/u,".json"),{force:true});total-=row.size;}
  }catch{}finally{pruning=false;}
 };
 const hostOf=(value:string):string=>new URL(value).hostname,registrable=(host:string):string=>host.split(".").slice(-2).join(".");
 const bounded=async(first:string,headers:Record<string,string>):Promise<Response>=>{
  const origin=hostOf(first);let url=first;
  for(let hop=0;;hop++){
   const controller=new AbortController(),timer=setTimeout(()=>controller.abort(),headerTimeout);let response:Response;
   try{response=await upstream(url,{headers,signal:controller.signal,redirect:"manual"});}catch(error){if(controller.signal.aborted)throw Error("upstream timeout");throw error;}finally{clearTimeout(timer);}
   if(response.status<300||response.status>=400||response.status===304)return response;
   await response.body?.cancel().catch(()=>{});
   const next=new URL(response.headers.get("location")??"",url),host=next.hostname;
   if(hop>=policy.maximumRedirects||next.protocol!=="https:"||!(registrable(host)===registrable(origin)||policy.redirectSuffixes.some(suffix=>host===suffix||host.endsWith("."+suffix))))throw Error("redirect refused");
   url=next.toString();
  }
 };
 const start=(key:string,target:string,immutable:boolean,url:string,previous:MirrorEntryMetaV1|undefined):Flight=>{
  const running=flights.get(key);if(running)return {...running,joined:true};
  if(flights.size>=policy.maximumConcurrentUpstream)return {head:Promise.resolve({kind:"failed",reason:"busy"} as const),done:Promise.resolve(undefined),joined:false};
  let settleHead!:(head:Head)=>void,settleDone!:(meta:MirrorEntryMetaV1|undefined)=>void;
  const head=new Promise<Head>(resolve=>{settleHead=resolve;}),done=new Promise<MirrorEntryMetaV1|undefined>(resolve=>{settleDone=resolve;});
  const flight={head,done,joined:false};flights.set(key,flight);
  void done.finally(()=>{flights.delete(key);waiters.shift()?.();});
  void(async()=>{
   const headers:Record<string,string>={"accept-encoding":"identity","user-agent":"semio-fleet-mirror"};
   if(previous?.etag&&!immutable)headers["if-none-match"]=previous.etag;
   let response:Response;
   try{response=await bounded(url,headers);}catch(error){const message=error instanceof Error?error.message:"";settleHead({kind:"failed",reason:message==="redirect refused"?"redirect-refused":message==="upstream timeout"?"upstream-timeout":"upstream-unreachable"});settleDone(undefined);return;}
   if(response.status===304&&previous){const refreshed={...previous,fetchedAtMs:now()};await writeFile(files(key).meta,JSON.stringify(refreshed)).catch(()=>{});await response.body?.cancel().catch(()=>{});settleHead({kind:"revalidated",meta:refreshed});settleDone(refreshed);return;}
   if(response.status===404||response.status===410){await response.body?.cancel().catch(()=>{});settleHead({kind:"refused",status:response.status});settleDone(undefined);return;}
   if(response.status!==200||!response.body){await response.body?.cancel().catch(()=>{});settleHead({kind:"failed",reason:response.status===200?"upstream-empty":"upstream-"+response.status});settleDone(undefined);return;}
   await mkdir(dirname(files(key).body),{recursive:true});
   const temporary=files(key).body+"."+randomBytes(6).toString("hex")+".tmp",sink=createWriteStream(temporary,{flags:"wx"}),reader=response.body.getReader();
   const contentType=response.headers.get("content-type")??"application/octet-stream",etag=response.headers.get("etag")??undefined,length=response.headers.get("content-length");
   let bytes=0;
   const abandon=async(error?:unknown)=>{sink.destroy();await rm(temporary,{force:true}).catch(()=>{});settleDone(undefined);void error;};
   const stream=new ReadableStream<Uint8Array>({
    pull:async controller=>{
     let timer:ReturnType<typeof setTimeout>|undefined;
     try{
      const next=await Promise.race([reader.read(),new Promise<never>((_,reject)=>{timer=setTimeout(()=>reject(Error("upstream idle")),policy.upstreamIdleMilliseconds);})]);
      if(next.done){
       if(length!==null&&Number(length)!==bytes)throw Error("upstream body shortened");
       await new Promise<void>((finish,reject)=>sink.end((error?:Error|null)=>error?reject(error):finish()));
       const meta:MirrorEntryMetaV1={version:1,path:target,status:200,contentType,...(etag?{etag}:{}),bytes,fetchedAtMs:now(),immutable};
       await rename(temporary,files(key).body);await writeFile(files(key).meta,JSON.stringify(meta));
       controller.close();settleDone(meta);void prune();return;
      }
      bytes+=next.value.byteLength;
      if(!sink.write(next.value))await new Promise<void>(release=>sink.once("drain",()=>release()));
      controller.enqueue(next.value);
     }catch(error){await abandon(error);controller.error(error);}finally{if(timer)clearTimeout(timer);}
    },
    cancel:async()=>{await reader.cancel().catch(()=>{});await abandon();}
   });
   settleHead({kind:"stream",stream,meta:{version:1,path:target,status:200,contentType,...(etag?{etag}:{}),bytes:length===null?-1:Number(length),fetchedAtMs:now(),immutable}});
  })().catch(()=>{settleHead({kind:"failed",reason:"upstream-error"});settleDone(undefined);});
  return flight;
 };
 const slice=async(meta:MirrorEntryMetaV1,key:string,request:Request,origin:string,ecosystem:string):Promise<Response>=>{
  const range=request.headers.get("range")?.match(/^bytes=(\d*)-(\d*)$/u),headers:Record<string,string>={"content-type":meta.contentType,"accept-ranges":"bytes","x-semio-mirror":"hit"};
  void utimes(files(key).body,new Date(),new Date()).catch(()=>{});
  const rewrite=ecosystems.find(row=>row.id===ecosystem)?.rewrite;
  if(rewrite==="npm-tarball"&&!meta.immutable){const upstreamBase=ecosystems.find(row=>row.id===ecosystem)!.upstream,raw=meta.bytes>policy.maximumMetadataBytes?"":(await readFile(files(key).body,"utf8")).split(upstreamBase+"/").join(`${origin}${policy.prefix}${ecosystem}/`);return new Response(request.method==="HEAD"?null:raw,{headers:{...headers,"content-length":String(Buffer.byteLength(raw))}});}
  if(range&&(range[1]!==""||range[2]!=="")){
   const first=range[1]===""?Math.max(0,meta.bytes-Number(range[2])):Number(range[1]),last=range[1]===""?meta.bytes-1:range[2]===""?meta.bytes-1:Math.min(Number(range[2]),meta.bytes-1);
   if(first>last||first>=meta.bytes)return new Response(null,{status:416,headers:{"content-range":`bytes */${meta.bytes}`}});
   return new Response(request.method==="HEAD"?null:Readable.toWeb(createReadStream(files(key).body,{start:first,end:last})) as unknown as ReadableStream<Uint8Array>,{status:206,headers:{...headers,"content-length":String(last-first+1),"content-range":`bytes ${first}-${last}/${meta.bytes}`}});
  }
  return new Response(request.method==="HEAD"||meta.bytes===0?null:Readable.toWeb(createReadStream(files(key).body)) as unknown as ReadableStream<Uint8Array>,{headers:{...headers,"content-length":String(meta.bytes)}});
 };
 const fallback=(ecosystem:string,path:string,reason:string,direct:string,stale:MirrorEntryMetaV1|undefined):Response=>{
  if(stale!==undefined)return text(502,"Upstream unavailable");
  if(reason==="redirect-refused"){options.onEvent?.({ecosystem,path,outcome:"error",bytes:0,reason});return text(502,"Upstream unavailable");}
  options.onEvent?.({ecosystem,path,outcome:"fallback",bytes:0,reason});
  return new Response(say("fallback",{reason,url:direct})+"\n",{status:policy.fallbackStatus,headers:{location:direct,"content-type":"text/plain","x-semio-mirror":"fallback","x-semio-mirror-reason":reason}});
 };
 return async request=>{
  const url=new URL(request.url);if(!url.pathname.startsWith(policy.prefix))return undefined;
  if(url.pathname===policy.probePath)return Response.json({status:"ok",ecosystems:ecosystems.map(row=>row.id)});
  if(request.method!=="GET"&&request.method!=="HEAD")return new Response("Method not allowed\n",{status:405,headers:{allow:"GET, HEAD"}});
  const rest=url.pathname.slice(policy.prefix.length),slash=rest.indexOf("/"),id=slash<0?rest:rest.slice(0,slash),path=slash<0?"":rest.slice(slash+1),row=ecosystems.find(item=>item.id===id);
  if(!row||path===""||path.includes("..")||path.includes("\\")||path.length>2048)return text(404,"Not found");
  const target=path,immutable=(patterns.get(id)??[]).some(pattern=>pattern.test(path)),key=digest(id+"\n"+target),origin=url.origin,upstreamUrl=`${row.upstream}/${target}`;
  const cached=await readMeta(key);
  if(cached&&(immutable||now()-cached.fetchedAtMs<policy.revalidateSeconds*1000)){options.onEvent?.({ecosystem:id,path,outcome:"hit",bytes:cached.bytes});return slice(cached,key,request,origin,id);}
  const free=()=>flights.has(key)||flights.size<policy.maximumConcurrentUpstream;
  if(!free()){
   const deadline=Date.now()+slotWait;
   while(!free()){const left=deadline-Date.now();if(left<=0||!await new Promise<boolean>(resolve=>{const waiter=()=>{clearTimeout(timer);resolve(true);},timer=setTimeout(()=>{const at=waiters.indexOf(waiter);if(at>=0)waiters.splice(at,1);resolve(false);},left);waiters.push(waiter);}))break;}
   if(!free())return fallback(id,path,"busy",upstreamUrl,cached);
   const again=await readMeta(key);
   if(again&&(immutable||now()-again.fetchedAtMs<policy.revalidateSeconds*1000)){options.onEvent?.({ecosystem:id,path,outcome:"hit",bytes:again.bytes});return slice(again,key,request,origin,id);}
  }
  const flight=start(key,target,immutable,upstreamUrl,cached),head=await flight.head,plain=request.method==="GET"&&!request.headers.has("range")&&!(row.rewrite&&!immutable);
  if(head?.kind==="stream"){
   if(!flight.joined&&plain){options.onEvent?.({ecosystem:id,path,outcome:"miss",bytes:head.meta.bytes});return new Response(head.stream,{headers:{"content-type":head.meta.contentType,...(head.meta.bytes>=0?{"content-length":String(head.meta.bytes)}:{}),"x-semio-mirror":"miss"}});}
   if(!flight.joined)void head.stream.pipeTo(new WritableStream()).catch(()=>{});
  }
  if(head?.kind==="refused")return text(head.status,"Not found upstream");
  const settled=await flight.done,present=settled&&await readMeta(key);
  if(present){options.onEvent?.({ecosystem:id,path,outcome:flight.joined?"joined":head?.kind==="revalidated"?"revalidated":"miss",bytes:present.bytes});return slice(present,key,request,origin,id);}
  if(cached){options.onEvent?.({ecosystem:id,path,outcome:"stale",bytes:cached.bytes});return slice(cached,key,request,origin,id);}
  return fallback(id,path,head?.kind==="failed"?head.reason:"upstream-unreachable",upstreamUrl,undefined);
 };
}
