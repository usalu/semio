import {createCipheriv,createDecipheriv,createHash,createPrivateKey,createPublicKey,diffieHellman,generateKeyPairSync,hkdfSync,randomBytes,type KeyObject} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🔐️ The route every sealed exchange is posted to; the path, the method, the credentials and the body of the real request travel inside it. */
export const SEALED_ROUTE_V1="/v1/fleet/sealed";

/** 🏷️ The header of a sealed request (`1.<epk>.<timestamp>.<nonce>`) and of its reply (`1.<epk>`). */
export const SEALED_HEADER_V1="x-semio-sealed",SEALED_REPLY_HEADER_V1="x-semio-sealed-reply";

/** 🕰️ The most the clock of the asker may differ from the clock of the answering server. */
export const SEALED_SKEW_MILLISECONDS_V1=60000;

/** 📜️ The real request or reply inside a sealed exchange, shaped by `🧬️schema/🔣️.json` (`RequestHead`, `ReplyHead`). */
export type SealedRequestV1=Readonly<{method:string;target:string;headers:Readonly<Record<string,string>>;body?:Uint8Array}>;
export type SealedReplyV1=Readonly<{status:number;headers:Readonly<Record<string,string>>;body:Uint8Array}>;

/** 🚫️ Why a sealed message was refused: `malformed` (not the wire format), `clock` (outside the skew), `replay` (seen before), `decrypt` (not from the holder of the key, or changed on the way), `large` (above the bound). */
export class SealedErrorV1 extends Error{readonly code:"malformed"|"clock"|"replay"|"decrypt"|"large";constructor(code:"malformed"|"clock"|"replay"|"decrypt"|"large"){super(`Sealed exchange refused: ${code}`);this.code=code;this.name="SealedErrorV1";}}

const REQUEST_INFO="semio-fleet-sealed/1 request",REPLY_INFO="semio-fleet-sealed/1 reply",AAD_LABEL="semio-fleet-sealed/1",P=(1n<<255n)-19n;
const ED_PUBLIC_PREFIX=Buffer.from("302a300506032b6570032100","hex"),ED_PRIVATE_PREFIX=Buffer.from("302e020100300506032b657004220420","hex"),X_PRIVATE_PREFIX=Buffer.from("302e020100300506032b656e04220420","hex"),X_PUBLIC_PREFIX=Buffer.from("302a300506032b656e032100","hex");
const sha=(value:Uint8Array):Buffer=>createHash("sha256").update(value).digest();
const channel=new WeakSet<Request>();

/** 🔐️ Whether a request reached the handler through a sealed exchange (marked by the sealed handler itself; no header can claim it). */
export const isSealedRequestV1=(request:Request):boolean=>channel.has(request);

/** 🛡️ Validates one value against a definition of the sealed schema and names the first problem. */
export function requireSealedShapeV1(definition:"RequestHead"|"ReplyHead"|"RequestHeader"|"ReplyHeader"|"TransportPolicy",value:unknown,message:string):void{
 const errors=validateJsonSchemaSubset({$ref:`#/definitions/${definition}`},value,schema);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

const power=(base:bigint,exponent:bigint):bigint=>{let result=1n,factor=base%P,rest=exponent;while(rest>0n){if((rest&1n)===1n)result=result*factor%P;factor=factor*factor%P;rest>>=1n;}return result;};
const fromLittle=(bytes:Uint8Array):bigint=>{let value=0n;for(let index=bytes.length-1;index>=0;index--)value=(value<<8n)|BigInt(bytes[index]!);return value;};
const toLittle=(value:bigint):Buffer=>{const out=Buffer.alloc(32);let rest=value;for(let index=0;index<32;index++){out[index]=Number(rest&255n);rest>>=8n;}return out;};

/** 🔑️ The X25519 public key (32 bytes) that belongs to an Ed25519 public key (SPKI, base64url): the same curve in Montgomery form, u = (1 + y) / (1 - y) mod 2^255 - 19. The fleet identity stays one key per origin; nothing new is published. */
export function exchangePublicKeyV1(originPublic:string):Buffer{
 const der=Buffer.from(originPublic,"base64url");
 if(der.length!==44||!der.subarray(0,12).equals(ED_PUBLIC_PREFIX))throw new SealedErrorV1("malformed");
 const raw=Buffer.from(der.subarray(12));raw[31]=raw[31]!&127;
 const y=fromLittle(raw),denominator=((1n-y)%P+P)%P;
 if(denominator===0n)throw new SealedErrorV1("malformed");
 return toLittle((1n+y)%P*power(denominator,P-2n)%P);
}

/** 🗝️ The X25519 private key that belongs to an Ed25519 private key (PKCS8, base64url): the clamped first half of the SHA-512 of the seed, the scalar the signing key already is. */
export function exchangePrivateKeyV1(originPrivate:string):KeyObject{
 const der=Buffer.from(originPrivate.trim(),"base64url");
 if(der.length!==48||!der.subarray(0,16).equals(ED_PRIVATE_PREFIX))throw new SealedErrorV1("malformed");
 const scalar=createHash("sha512").update(der.subarray(16)).digest().subarray(0,32),clamped=Buffer.from(scalar);
 clamped[0]=clamped[0]!&248;clamped[31]=(clamped[31]!&127)|64;
 return createPrivateKey({key:Buffer.concat([X_PRIVATE_PREFIX,clamped]),format:"der",type:"pkcs8"});
}

const privateOfSeed=(seed:Uint8Array):KeyObject=>createPrivateKey({key:Buffer.concat([X_PRIVATE_PREFIX,seed]),format:"der",type:"pkcs8"});
const publicOfX=(raw:Uint8Array):KeyObject=>createPublicKey({key:Buffer.concat([X_PUBLIC_PREFIX,raw]),format:"der",type:"spki"});
const rawPublic=(key:KeyObject):Buffer=>Buffer.from(createPublicKey(key).export({type:"spki",format:"der"})).subarray(12);

/** 🧬️ The 32-byte key and 12-byte IV of one direction, from the HKDF of the shared secrets. */
function derive(secret:Uint8Array,salt:Uint8Array,label:string,group:string):Readonly<{key:Buffer;iv:Buffer}>{
 const material=Buffer.from(hkdfSync("sha256",secret,salt,Buffer.concat([Buffer.from(label),Buffer.from([0]),Buffer.from(group)]),44));
 return {key:material.subarray(0,32),iv:material.subarray(32,44)};
}

const aadOf=(label:string,group:string,extra:Uint8Array):Buffer=>Buffer.concat([Buffer.from(label),Buffer.from([0]),Buffer.from(group),Buffer.from([0]),extra]);
const seal=(key:Buffer,iv:Buffer,aad:Buffer,plain:Uint8Array):Buffer=>{const cipher=createCipheriv("aes-256-gcm",key,iv,{authTagLength:16});cipher.setAAD(aad);return Buffer.concat([cipher.update(plain),cipher.final(),cipher.getAuthTag()]);};
const open=(key:Buffer,iv:Buffer,aad:Buffer,sealed:Uint8Array):Buffer=>{
 if(sealed.length<16)throw new SealedErrorV1("malformed");
 try{const decipher=createDecipheriv("aes-256-gcm",key,iv,{authTagLength:16});decipher.setAAD(aad);decipher.setAuthTag(Buffer.from(sealed.subarray(sealed.length-16)));return Buffer.concat([decipher.update(sealed.subarray(0,sealed.length-16)),decipher.final()]);}catch{throw new SealedErrorV1("decrypt");}
};

const frame=(head:unknown,body:Uint8Array):Buffer=>{const text=Buffer.from(JSON.stringify(head));const length=Buffer.alloc(4);length.writeUInt32BE(text.length);return Buffer.concat([length,text,body]);};
function unframe(plain:Uint8Array,definition:"RequestHead"|"ReplyHead"):Readonly<{head:Record<string,unknown>;body:Buffer}>{
 const buffer=Buffer.from(plain.buffer,plain.byteOffset,plain.byteLength);
 if(buffer.length<4)throw new SealedErrorV1("malformed");
 const length=buffer.readUInt32BE(0);
 if(length>65536||4+length>buffer.length)throw new SealedErrorV1("malformed");
 let head:Record<string,unknown>;
 try{head=JSON.parse(buffer.subarray(4,4+length).toString("utf8")) as Record<string,unknown>;requireSealedShapeV1(definition,head,"head");}catch{throw new SealedErrorV1("malformed");}
 return {head,body:buffer.subarray(4+length)};
}

/** 📦️ A request sealed for one server; `open` reads its reply. */
export type SealedOutgoingV1=Readonly<{headers:Readonly<Record<string,string>>;body:Uint8Array;open(header:string|null|undefined,body:Uint8Array):SealedReplyV1}>;

/** 🔏️ Seals a request for the server whose X25519 key is `serverPublic` (see `exchangePublicKeyV1`) of the group `group`: a one-time key pair makes the shared secret, so only the holder of the server's identity key can read the request, and only it can write a reply that opens. `seeds` (tests, vectors) replaces the random one-time key and nonce. */
export function sealRequestV1(input:Readonly<{serverPublic:Uint8Array;group:string;request:SealedRequestV1;now:number;seeds?:Readonly<{ephemeral:Uint8Array;nonce:Uint8Array}>}>):SealedOutgoingV1{
 requireSealedShapeV1("RequestHead",{method:input.request.method,target:input.request.target,headers:input.request.headers},"request");
 const ephemeral=input.seeds===undefined?generateKeyPairSync("x25519").privateKey:privateOfSeed(input.seeds.ephemeral),nonce=Buffer.from(input.seeds?.nonce??randomBytes(16)),epk=rawPublic(ephemeral);
 const first=Buffer.from(diffieHellman({privateKey:ephemeral,publicKey:publicOfX(input.serverPublic)})),request=derive(first,Buffer.concat([epk,input.serverPublic]),REQUEST_INFO,input.group);
 const stamp=Buffer.alloc(8);stamp.writeBigUInt64BE(BigInt(input.now));
 const ciphertext=seal(request.key,request.iv,aadOf(AAD_LABEL,input.group,Buffer.concat([stamp,nonce])),frame({method:input.request.method,target:input.request.target,headers:input.request.headers},input.request.body??new Uint8Array(0)));
 const header=`1.${epk.toString("base64url")}.${input.now}.${nonce.toString("base64url")}`;
 return {headers:{[SEALED_HEADER_V1]:header},body:ciphertext,open(replyHeader,body){
  if(typeof replyHeader!=="string"||validateJsonSchemaSubset({$ref:"#/definitions/ReplyHeader"},replyHeader,schema).length>0)throw new SealedErrorV1("malformed");
  const serverEpk=Buffer.from(replyHeader.slice(2),"base64url"),second=Buffer.from(diffieHellman({privateKey:ephemeral,publicKey:publicOfX(serverEpk)})),requestHash=sha(ciphertext);
  const reply=derive(Buffer.concat([first,second]),Buffer.concat([epk,serverEpk,requestHash]),REPLY_INFO,input.group);
  const {head,body:data}=unframe(open(reply.key,reply.iv,aadOf(AAD_LABEL+" reply",input.group,requestHash),body),"ReplyHead");
  return {status:head.status as number,headers:head.headers as Record<string,string>,body:data};
 }};
}

/** 📥️ A request opened by the server: the real request, a key for the replay table and `seal`, which makes the reply (with a one-time key of the server, so a later theft of the identity key does not open the replies of the past). */
export type SealedIncomingV1=Readonly<{request:SealedRequestV1;replay:string;seal(reply:SealedReplyV1,seed?:Uint8Array):Readonly<{header:string;body:Uint8Array}>}>;

/** 🔓️ Opens a sealed request with the private exchange key of the server of group `group`. Refuses a message outside the clock skew, one that is not the wire format, and one that does not authenticate (changed on the way or made without knowing the server's public key); it never says which part of a bad message was wrong beyond the code. */
export function openRequestV1(input:Readonly<{serverPrivate:KeyObject;group:string;header:string|null|undefined;body:Uint8Array;now:number;skewMilliseconds?:number}>):SealedIncomingV1{
 const header=input.header;
 if(typeof header!=="string"||validateJsonSchemaSubset({$ref:"#/definitions/RequestHeader"},header,schema).length>0)throw new SealedErrorV1("malformed");
 const [,encodedKey,stamped,encodedNonce]=header.split(".") as [string,string,string,string],epk=Buffer.from(encodedKey,"base64url"),nonce=Buffer.from(encodedNonce,"base64url"),timestamp=Number(stamped);
 if(!Number.isSafeInteger(timestamp))throw new SealedErrorV1("malformed");
 if(Math.abs(input.now-timestamp)>(input.skewMilliseconds??SEALED_SKEW_MILLISECONDS_V1))throw new SealedErrorV1("clock");
 const serverPublic=rawPublic(input.serverPrivate),first=Buffer.from(diffieHellman({privateKey:input.serverPrivate,publicKey:publicOfX(epk)})),request=derive(first,Buffer.concat([epk,serverPublic]),REQUEST_INFO,input.group);
 const stamp=Buffer.alloc(8);stamp.writeBigUInt64BE(BigInt(timestamp));
 const {head,body}=unframe(open(request.key,request.iv,aadOf(AAD_LABEL,input.group,Buffer.concat([stamp,nonce])),input.body),"RequestHead"),requestHash=sha(input.body);
 return {replay:`${encodedKey}.${encodedNonce}`,request:{method:head.method as string,target:head.target as string,headers:head.headers as Record<string,string>,body},seal(reply,seed){
  requireSealedShapeV1("ReplyHead",{status:reply.status,headers:reply.headers},"reply");
  const ephemeral=seed===undefined?generateKeyPairSync("x25519").privateKey:privateOfSeed(seed),serverEpk=rawPublic(ephemeral),second=Buffer.from(diffieHellman({privateKey:ephemeral,publicKey:publicOfX(epk)}));
  const answer=derive(Buffer.concat([first,second]),Buffer.concat([epk,serverEpk,requestHash]),REPLY_INFO,input.group);
  return {header:`1.${serverEpk.toString("base64url")}`,body:seal(answer.key,answer.iv,aadOf(AAD_LABEL+" reply",input.group,requestHash),frame({status:reply.status,headers:reply.headers},reply.body))};
 }};
}

const REQUEST_DROPPED=new Set(["host","connection","content-length","transfer-encoding","keep-alive","upgrade","accept-encoding","te","trailer","proxy-authorization",SEALED_HEADER_V1,SEALED_REPLY_HEADER_V1]),REPLY_KEPT=new Set(["content-type","x-semio-time","www-authenticate","retry-after","allow","etag"]);
const headersOf=(headers:Headers,kept?:ReadonlySet<string>):Record<string,string>=>{const out:Record<string,string>={};for(const [name,value] of headers){const key=name.toLowerCase();if(kept===undefined?REQUEST_DROPPED.has(key):!kept.has(key))continue;if(/^[a-z0-9][a-z0-9-]{0,63}$/u.test(key)&&value.length<=2048&&Object.keys(out).length<32)out[key]=value;}return out;};
const bodyOf=async(init:RequestInit|undefined,request:Request|undefined):Promise<Uint8Array|undefined>=>{
 const body=init?.body??undefined;
 if(body===undefined)return request===undefined||request.body===null?undefined:new Uint8Array(await request.clone().arrayBuffer());
 if(typeof body==="string")return new TextEncoder().encode(body);
 if(body instanceof Uint8Array)return body;
 if(body instanceof ArrayBuffer)return new Uint8Array(body);
 if(ArrayBuffer.isView(body))return new Uint8Array(body.buffer,body.byteOffset,body.byteLength);
 return new Uint8Array(await new Response(body as BodyInit).arrayBuffer());
};
const bodyless=(status:number):boolean=>status===101||status===204||status===205||status===304;

/** 🪝️ A `fetch` that sends every request as a sealed exchange to the server named by the URL: the server's key comes from `serverKey` (its origin public key, SPKI base64url: the registration in a store, or a pin of the topology); a server whose key is unknown answers 503 and nothing is sent. A refusal for the clock is retried once with the clock of the server. It never sends the real request in the clear, and an exchange that does not open is a 502, never a partial answer. */
export function createSealedFetchV1(options:Readonly<{group:string;serverKey:(url:URL)=>Promise<string|undefined>|string|undefined;fetch?:typeof fetch;now?:()=>number;maximumBytes?:number}>):typeof fetch{
 const base=options.fetch??fetch,clock=options.now??Date.now,limit=options.maximumBytes??2**28;
 let offset=0;
 const send=async(input:RequestInfo|URL,init:RequestInit|undefined,retry:boolean):Promise<Response>=>{
  const request=typeof input==="string"||input instanceof URL?undefined:input,url=new URL(request===undefined?String(input):request.url),key=await options.serverKey(url);
  if(key===undefined)return new Response("Sealed transport: the key of the server is unknown\n",{status:503,headers:{"x-semio-sealed-error":"server-key-unknown"}});
  const body=await bodyOf(init,request);
  const sealed=sealRequestV1({serverPublic:exchangePublicKeyV1(key),group:options.group,now:clock()+offset,request:{method:(init?.method??request?.method??"GET").toUpperCase(),target:url.pathname+url.search,headers:headersOf(new Headers(init?.headers??request?.headers)),...(body===undefined?{}:{body})}});
  const response=await base(`${url.origin}${SEALED_ROUTE_V1}`,{method:"POST",headers:{...sealed.headers,"content-type":"application/octet-stream"},body:sealed.body as BodyInit,...(init?.signal===undefined||init.signal===null?{}:{signal:init.signal})});
  if(response.status===401&&retry){
   const server=Number(response.headers.get("x-semio-time"));
   await response.body?.cancel();
   if(Number.isFinite(server)&&server>0&&Math.abs(server-(clock()+offset))>SEALED_SKEW_MILLISECONDS_V1/4){offset=server-clock();return send(input,init,false);}
   return new Response("Unauthorized\n",{status:401});
  }
  if(response.status!==200){await response.body?.cancel();return new Response("Sealed transport refused\n",{status:response.status===404||response.status===413?response.status:502,headers:{"x-semio-sealed-error":"refused"}});}
  const length=Number(response.headers.get("content-length"));
  if(Number.isFinite(length)&&length>limit+4096){await response.body?.cancel();return new Response("Sealed reply too large\n",{status:413});}
  try{
   const reply=sealed.open(response.headers.get(SEALED_REPLY_HEADER_V1),new Uint8Array(await response.arrayBuffer()));
   if(reply.body.length>limit)return new Response("Sealed reply too large\n",{status:413});
   return new Response(bodyless(reply.status)?null:reply.body as BodyInit,{status:reply.status,headers:reply.headers});
  }catch{return new Response("Sealed reply did not open\n",{status:502,headers:{"x-semio-sealed-error":"reply-refused"}});}
 };
 return ((input:RequestInfo|URL,init?:RequestInit)=>send(input,init,true)) as typeof fetch;
}

/** 🛡️ The server side of the route: answers `POST /v1/fleet/sealed` by opening the exchange, handing the real request to `handle` (which sees the channel `sealed`) and sealing what it answered; returns `undefined` for every other path so the caller serves it as usual. A message is refused when it is outside the skew, does not authenticate, is above the bound or was seen before (each one-time key and nonce is answered once); the refusal is a plain 400, 401 or 413 that says nothing about the inner request. */
export function createSealedHandlerV1(options:Readonly<{privateKey:KeyObject;group:string;maximumBytes:number;handle:(request:Request,peer:string|undefined)=>Promise<Response>;now?:()=>number;skewMilliseconds?:number;maximumSeen?:number}>):(request:Request,peer?:string)=>Promise<Response|undefined>{
 const clock=options.now??Date.now,skew=options.skewMilliseconds??SEALED_SKEW_MILLISECONDS_V1,seen=new Map<string,number>(),limit=options.maximumSeen??200000;
 const refuse=(status:number,message:string):Response=>new Response(message+"\n",{status,headers:{"content-type":"text/plain","x-semio-time":String(clock())}});
 return async(request,peer)=>{
  const url=new URL(request.url);
  if(url.pathname!==SEALED_ROUTE_V1)return undefined;
  if(request.method!=="POST")return new Response("Method not allowed\n",{status:405,headers:{allow:"POST"}});
  const length=Number(request.headers.get("content-length"));
  if(Number.isFinite(length)&&length>options.maximumBytes+65536)return refuse(413,"Too large");
  const bytes=new Uint8Array(await request.arrayBuffer());
  if(bytes.length>options.maximumBytes+65536)return refuse(413,"Too large");
  const now=clock();
  let incoming:SealedIncomingV1;
  try{incoming=openRequestV1({serverPrivate:options.privateKey,group:options.group,header:request.headers.get(SEALED_HEADER_V1),body:bytes,now,skewMilliseconds:skew});}
  catch(error){const code=error instanceof SealedErrorV1?error.code:"malformed";return code==="malformed"?refuse(400,"Malformed"):refuse(401,"Unauthorized");}
  for(const [key,expires] of seen){if(expires>=now)break;seen.delete(key);}
  if(seen.has(incoming.replay)||seen.size>=limit)return refuse(401,"Unauthorized");
  seen.set(incoming.replay,now+2*skew+1000);
  const inner=new Request(`${url.origin}${incoming.request.target}`,{method:incoming.request.method,headers:incoming.request.headers,...(incoming.request.body!==undefined&&incoming.request.body.length>0&&incoming.request.method!=="GET"&&incoming.request.method!=="HEAD"?{body:incoming.request.body as BodyInit}:{})});
  let response:Response;
  channel.add(inner);
  try{response=await options.handle(inner,peer);}catch{response=new Response("Internal error\n",{status:500,headers:{"content-type":"text/plain"}});}
  const payload=new Uint8Array(await response.arrayBuffer());
  if(payload.length>options.maximumBytes)return refuse(413,"Too large");
  const sealed=incoming.seal({status:response.status,headers:headersOf(response.headers,REPLY_KEPT),body:inner.method==="HEAD"?new Uint8Array(0):payload});
  return new Response(sealed.body as BodyInit,{status:200,headers:{[SEALED_REPLY_HEADER_V1]:sealed.header,"content-type":"application/octet-stream","cache-control":"no-store"}});
 };
}
