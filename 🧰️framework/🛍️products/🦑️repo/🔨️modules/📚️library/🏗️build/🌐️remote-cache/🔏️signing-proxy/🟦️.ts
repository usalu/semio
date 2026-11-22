import {runtimeServeV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🌍️serve/🟦️.ts";

/** 🔏️ How one loopback signing proxy forwards: the origin of the fleet cache, the `fetch` that signs (the one every cache client uses), the time one upstream call may take before the proxy answers a miss, the size it buffers for one upload and how long it stays away from an upstream that failed. */
export type RemoteCacheSigningProxyOptionsV1=Readonly<{upstream:string;fetch:typeof fetch;timeoutMilliseconds?:number;maximumBodyBytes?:number;downMilliseconds?:number;port?:number}>;

/** 📊️ What one proxy did since it started: requests, answers from the fleet, misses it answered itself (upstream down, slow or refusing) and requests it refused (path, method, size). */
export type RemoteCacheSigningProxyStatsV1=Readonly<{requests:number;forwarded:number;answeredLocally:number;refused:number}>;

/** 🔏️ A running proxy: the loopback base URL a client puts in front of the cache routes, its counters and an orderly stop. */
export interface RemoteCacheSigningProxyV1{
 readonly url:string;
 stats():RemoteCacheSigningProxyStatsV1;
 stop():Promise<void>;
}

const ROUTE=/^\/(?:[a-z0-9][a-z0-9._-]{0,63}\/)?(?:ac|cas)\/[0-9a-f]{64}$/u,METHODS=new Set(["GET","HEAD","PUT"]),LOOPBACK_HOSTS=/^(?:127\.0\.0\.1|localhost|\[::1\])(?::\d+)?$/u;
const plain=(status:number,message:string,headers:Record<string,string>={})=>new Response(message+"\n",{status,headers:{"content-type":"text/plain",...headers}});

/** 🔏️ Starts a loopback proxy for clients that cannot sign: it accepts the unsigned HTTP cache routes `/[instance/]ac/<sha256>` and `/[instance/]cas/<sha256>` (GET, HEAD, PUT; nothing else, so it is no open relay) on 127.0.0.1 and forwards each one to the fleet cache through the signing `fetch`, with the method and body intact. An upstream that does not answer in time, refuses the signature or fails stays away for `downMilliseconds`: reads are answered at once as a plain miss (404) and uploads as unavailable (507, which clients do not retry), so a broken fleet cache never stalls or fails the client's build; the first request after the window tries the upstream again.
 * @see ../🛰️fleet/🪪️identity/🟦️.ts */
export async function startRemoteCacheSigningProxyV1(options:RemoteCacheSigningProxyOptionsV1):Promise<RemoteCacheSigningProxyV1>{
 const upstream=options.upstream.replace(/\/+$/u,""),limit=options.timeoutMilliseconds??30000,maximum=options.maximumBodyBytes??536870912,away=options.downMilliseconds??15000,counters={requests:0,forwarded:0,answeredLocally:0,refused:0};
 let downUntil=0;
 const unavailable=(method:string)=>{counters.answeredLocally++;return method==="PUT"?plain(507,"Fleet cache unavailable"):plain(404,"Not found");};
 const handle=async(request:Request):Promise<Response>=>{
  const url=new URL(request.url),method=request.method;
  if(url.pathname==="/healthz"&&(method==="GET"||method==="HEAD"))return new Response(method==="GET"?'{"status":"ok"}\n':null,{headers:{"content-type":"application/json"}});
  counters.requests++;
  if(!LOOPBACK_HOSTS.test(request.headers.get("host")??"")||!METHODS.has(method)||!ROUTE.test(url.pathname)){counters.refused++;return plain(METHODS.has(method)?404:405,"Refused",METHODS.has(method)?{}:{allow:"GET, HEAD, PUT"});}
  if(Date.now()<downUntil)return unavailable(method);
  let body:Uint8Array|undefined;
  if(method==="PUT"){
   const declared=Number(request.headers.get("content-length")??0);
   if(declared>maximum){counters.refused++;return plain(413,"Too large");}
   body=new Uint8Array(await request.arrayBuffer());
   if(body.length>maximum){counters.refused++;return plain(413,"Too large");}
  }
  const controller=new AbortController(),timer=setTimeout(()=>controller.abort(),limit+(body===undefined?0:Math.ceil(body.length/2097152*1000)));
  try{
   const response=await options.fetch(upstream+url.pathname,{method,signal:controller.signal,...(body===undefined?{}:{body:body as BodyInit,headers:{"content-type":request.headers.get("content-type")??"application/octet-stream"}})});
   clearTimeout(timer);
   if(response.status===401||response.status===403||response.status>=500&&response.status!==507){await response.body?.cancel();downUntil=Date.now()+away;return unavailable(method);}
   counters.forwarded++;
   const headers=new Headers();for(const name of["content-type","content-length"]){const value=response.headers.get(name);if(value!==null&&method!=="PUT")headers.set(name,value);}
   return new Response(method==="HEAD"?null:response.body,{status:response.status,headers});
  }catch{
   clearTimeout(timer);downUntil=Date.now()+away;return unavailable(method);
  }
 };
 const server=await runtimeServeV1({hostname:"127.0.0.1",port:options.port??0,idleTimeoutSeconds:60,maximumRequestBodyBytes:maximum,fetch:request=>handle(request)});
 return Object.freeze({url:`http://127.0.0.1:${server.port}`,stats:()=>({...counters}),stop:()=>server.stop(true)});
}

type Shared={proxy:Promise<RemoteCacheSigningProxyV1>;holders:number};
const shared=new Map<string,Shared>();

/** 🔏️ A handle on a shared proxy: the base URL and the release of this holder. */
export type RemoteCacheSigningProxyLeaseV1=Readonly<{url:string;release():Promise<void>}>;

const healthy=async(proxy:RemoteCacheSigningProxyV1):Promise<boolean>=>{try{return (await fetch(proxy.url+"/healthz",{signal:AbortSignal.timeout(1000)})).status===200;}catch{return false;}};

/** 🔏️ Takes a proxy for one invocation: a healthy proxy of this process for the same upstream is reused (every holder counts), otherwise a new one is started; the proxy stops when its last holder releases it. A proxy that cannot start answers `undefined`, so the caller goes on without a cache instead of failing. */
export async function acquireRemoteCacheSigningProxyV1(options:RemoteCacheSigningProxyOptionsV1):Promise<RemoteCacheSigningProxyLeaseV1|undefined>{
 const key=options.upstream.replace(/\/+$/u,"");
 try{
  let entry=shared.get(key);
  if(entry!==undefined&&!await entry.proxy.then(healthy,()=>false)){shared.delete(key);void entry.proxy.then(proxy=>proxy.stop(),()=>{});entry=undefined;}
  if(entry===undefined){entry={proxy:startRemoteCacheSigningProxyV1(options),holders:0};shared.set(key,entry);}
  entry.holders++;
  const held=entry,proxy=await held.proxy;
  let released=false;
  return Object.freeze({url:proxy.url,async release(){if(released)return;released=true;if(--held.holders>0)return;if(shared.get(key)===held)shared.delete(key);await proxy.stop().catch(()=>{});}});
 }catch{
  shared.delete(key);return undefined;
 }
}
