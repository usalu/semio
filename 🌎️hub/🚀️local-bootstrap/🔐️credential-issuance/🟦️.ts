import {localHubHostPolicyV1,captureLocalHubStartV1,LocalHubHostLedgerV1} from "../🎛️host/🟦️.ts";
import {checkResourceLeaseControl} from "@semio-tech/process/leases";
import { LOCAL_SESSION_BROKER_REQUEST_MAX_BYTES, isLocalSessionTokenV1, LOCAL_SESSION_BROKER_FILE, LOCAL_SESSION_BROKER_SCHEMA, LOCAL_SESSION_SCHEMA, parseLocalSessionBrokerRecordV1, parseLocalSessionRequestV1, parseLocalSessionV1, type LocalSessionBrokerRecordV1, type LocalSessionV1 } from "@semio-tech/framework-os/modules/directory/local-session/broker/schema";

import { randomBytes, timingSafeEqual } from "node:crypto";
import { existsSync, lstatSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { protectOwnerOnly } from "@semio-tech/repo-lib/process";
import { LOCAL_BOOTSTRAP_DEADLINE_MS, writeLocalFrame } from "../📡️framing/🟦️.ts";
import { authenticatedFrame, LOCAL_BOOTSTRAP_SCHEMA, type LocalClientClass, type LocalProfile, verifyAuthenticatedFrame } from "../🛂authentication/🟦️.ts";
import {requireLocalHubRunV1,withLocalHubRunTaskV1,type LocalHubRun } from "../🏃️execution/🟦️.ts";

/** 🔐️ Issues one authenticated client-class credential bound to the active local Hub run. */
export function issueLocalCredential(run:LocalHubRun,profileId:string,clientClass:LocalClientClass,sequence=2,exchangeId=randomBytes(16).toString("hex")):Promise<Record<string,any>>{return withLocalHubRunTaskV1(run,()=>issueLocalCredentialLiveV1(run,profileId,clientClass,sequence,exchangeId));}

async function issueLocalCredentialLiveV1(run:LocalHubRun,profileId:string,clientClass:LocalClientClass,sequence:number,exchangeId:string):Promise<Record<string,any>>{
  const now = Date.now();
  const issue = authenticatedFrame(run.channelKey, {
    schema: LOCAL_BOOTSTRAP_SCHEMA,
    kind: "issue",
    runId: run.runId,
    sequence,
    exchangeId,
    issuedAt: now,
    expiresAt: now + LOCAL_BOOTSTRAP_DEADLINE_MS,
    profileId,
    deviceInstanceId: `${clientClass}-launcher`,
    clientClass,
  });
  await writeLocalFrame(run.pipe, issue);
  const envelope = await run.reader.read();
  if (envelope.kind === "reject") throw localBootstrapRefusal(run, envelope, exchangeId);
  verifyCredentialEnvelope(run, envelope, profileId, clientClass, exchangeId);
  return envelope;
}

/** 🚫️ The hub's signed answer that it refused one exchange (`resource-limit` inside a busy window,
 * `expired`, `denied`, …). The run stays usable: the next issue takes the next sequence. */
export class LocalBootstrapRefusedError extends Error {
  constructor(readonly code: string, readonly exchangeId: string) {
    super(`local bootstrap refused exchange ${exchangeId}: ${code}`);
  }
}

function localBootstrapRefusal(run: LocalHubRun, reject: Record<string, any>, exchangeId: string): LocalBootstrapRefusedError {
  verifyAuthenticatedFrame(run.channelKey, reject);
  if (reject.schema !== LOCAL_BOOTSTRAP_SCHEMA || reject.runId !== run.runId || reject.exchangeId !== exchangeId || typeof reject.code !== "string") throw new Error("local bootstrap refusal binding mismatch");
  return new LocalBootstrapRefusedError(reject.code, exchangeId);
}

function verifyCredentialEnvelope(run: LocalHubRun, envelope: Record<string, any>, profileId: string, clientClass: LocalClientClass, exchangeId: string): void {
  verifyAuthenticatedFrame(run.channelKey, envelope);
  if (
    envelope.schema !== "semio.hub.local-credential-envelope/v1" ||
    envelope.runId !== run.runId ||
    envelope.exchangeId !== exchangeId ||
    envelope.profileId !== profileId ||
    envelope.clientClass !== clientClass ||
    envelope.sessionKind !== "development-local" ||
    !Number.isSafeInteger(envelope.authorizationGeneration) ||
    envelope.authorizationGeneration < 1
  )
    throw new Error("local credential envelope binding mismatch");
}

//#region 🎫️SessionBroker
/** 🎫️ Local development session broker: the ONE process that owns a loopback hub's local-bootstrap pipe hands fresh
 * react-relay sessions to local development UIs (every `s` serve, both two-user rows), so a UI that finds a hub it did
 * not start still signs in with no manual step, and a UI whose 15-minute local session expired gets a new one.
 *
 * Trust boundary: the broker record lives in the hub's own `0700` data root (`0600` file) and carries a random bearer
 * secret; reading it is the same privilege as operating the hub. The broker listens on loopback only and mints only
 * the run's declared local profiles, through the authenticated pipe — never a password, never a network credential path.
 * @see ../🧬️schema/🔣️.json */
/** 🛡️ The admin-relay capability a broker started with an administrator profile keeps `0600` in the same data root — what
 * the hub's operator surfaces and the acceptance gates that read the hub's own state (head sequences, connection census)
 * present — and the sibling file a caller creates to ask for a fresh one (consumed; the capability lives 15 minutes). */
export const LOCAL_ADMIN_CAPABILITY_SCHEMA = "semio.hub.local-admin-capability/v1";
export const LOCAL_ADMIN_CAPABILITY_FILE = "admin-capability.json";
export const LOCAL_ADMIN_REQUEST_FILE = "admin-request";
const LOCAL_ADMIN_REQUEST_POLL_MS = 5_000;

export type LocalAdminCapabilityV1 = Readonly<{ schema: typeof LOCAL_ADMIN_CAPABILITY_SCHEMA; origin: string; capability: string; sessionId: string; expiresAt: number }>;
export type LocalSessionBrokerV1 = Readonly<{ record: LocalSessionBrokerRecordV1 }>;

/** 🧾️ Parses one admin capability file exactly. */
export function parseLocalAdminCapabilityV1(value: unknown): LocalAdminCapabilityV1 {
  const file = value as Record<string, unknown> | null;
  if (
    file === null ||
    typeof file !== "object" ||
    Object.keys(file).sort().join(",") !== "capability,expiresAt,origin,schema,sessionId" ||
    file.schema !== LOCAL_ADMIN_CAPABILITY_SCHEMA ||
    typeof file.origin !== "string" ||
    !/^http:\/\/127\.0\.0\.1:\d{1,5}$/u.test(file.origin) ||
    typeof file.capability !== "string" ||
    !isLocalSessionTokenV1(file.capability) ||
    typeof file.sessionId !== "string" ||
    file.sessionId.length === 0 ||
    file.sessionId.length > 256 ||
    !Number.isSafeInteger(file.expiresAt) ||
    (file.expiresAt as number) < 1
  )
    throw new Error("local admin capability invalid");
  return Object.freeze({ ...(file as LocalAdminCapabilityV1) });
}

/** 🔐️ Holds the exact run data and joins accepted broker requests before its real server closes. */
export function withLocalSessionBrokerV1<T>(run:LocalHubRun,profiles:readonly LocalProfile[],firstSequence:number,adminProfileId:string|undefined,body:(broker:LocalSessionBrokerV1)=>Promise<T>):Promise<T>{
 requireLocalHubRunV1(run);profiles=captureLocalHubStartV1(profiles,{}).profiles;
 return withLocalHubRunTaskV1(run,async(control,context)=>{
  const dataDir=context.dataRoot,hubOrigin=`http://127.0.0.1:${run.port}`,secret=randomBytes(32).toString("hex"),secretBytes=Buffer.from(secret,"hex"),profileIds=profiles.filter(profile=>profile.allowedClientClasses.includes("react-relay")).map(profile=>profile.profileId),ledger=new LocalHubHostLedgerV1(),adminPath=join(dataDir,LOCAL_ADMIN_CAPABILITY_FILE),adminRequestPath=join(dataDir,LOCAL_ADMIN_REQUEST_FILE),path=join(dataDir,LOCAL_SESSION_BROKER_FILE),created=new Map<string,NonNullable<ReturnType<typeof lstatSync>>>();let sequence=firstSequence,tail:Promise<unknown>=Promise.resolve(),adminRequests:ReturnType<typeof setInterval>|undefined,server:ReturnType<typeof Bun.serve>|undefined,result:T|undefined;const failures:unknown[]=[];
  const check=():void=>{checkResourceLeaseControl(control);},write=(target:string,value:unknown):void=>{check();const old=created.get(target);if(old){const current=lstatSync(target);if(!current.isFile()||current.isSymbolicLink()||current.nlink!==1||current.dev!==old.dev||current.ino!==old.ino)throw Error("Exact created broker record required");}writeFileSync(target,JSON.stringify(value)+"\n",{mode:0o600,flag:old?"w":"wx"});created.set(target,lstatSync(target));protectOwnerOnly(target,"file");};
  const enqueue=<U>(operation:()=>Promise<U>):Promise<U>=>{const previous=tail,work=ledger.run(async()=>{await previous;check();return operation();});tail=work.then(()=>{},()=>{});return work;};
  const next=():number=>{if(!Number.isSafeInteger(sequence)||sequence<2||sequence===Number.MAX_SAFE_INTEGER)throw Error("Finite broker sequence required");return sequence++;};
  const issueAdmin=(profileId:string):Promise<void>=>enqueue(async()=>{const envelope=await issueLocalCredential(run,profileId,"admin-relay",next()),file=parseLocalAdminCapabilityV1({schema:LOCAL_ADMIN_CAPABILITY_SCHEMA,origin:hubOrigin,capability:String(envelope.capability??""),sessionId:String(envelope.sessionId??""),expiresAt:Number(envelope.expiresAt)});write(adminPath,file);});
  const issue=(profileId:string):Promise<LocalSessionV1>=>enqueue(async()=>{const envelope=await issueLocalCredential(run,profileId,"react-relay",next()),token=String(envelope.capability??""),response=await fetch(`${hubOrigin}/auth/sessions/me`,{headers:{authorization:`Bearer ${token}`},signal:AbortSignal.any([control.signal,AbortSignal.timeout(10000)])}),reader=response.body?.getReader();if(!reader)throw Error("Broker session response body required");const chunks:Uint8Array[]=[];let bytes=0;try{for(;;){check();const row=await reader.read();if(row.done)break;bytes+=row.value.byteLength;if(bytes>LOCAL_SESSION_BROKER_REQUEST_MAX_BYTES||chunks.length>=localHubHostPolicyV1().maximumReadinessChunks)throw Error("Broker session response bound exceeded");chunks.push(row.value);}const value=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(Buffer.concat(chunks)));return parseLocalSessionV1({schema:LOCAL_SESSION_SCHEMA,profileId,token,userId:value.userId??value.user_id??""});}finally{reader.releaseLock();}});
  try{
   if(typeof body!=="function"||!Number.isSafeInteger(firstSequence)||firstSequence<2||adminProfileId!==undefined&&!profiles.some(profile=>profile.profileId===adminProfileId&&profile.allowedClientClasses.includes("admin-relay")))throw Error("Exact broker profiles and finite first sequence required");
   server=Bun.serve({hostname:"127.0.0.1",port:0,fetch(request){return ledger.run(async()=>{
    check();if(new URL(request.url).pathname!=="/session"||request.method!=="POST")return new Response(null,{status:404});const bearer=Buffer.from((request.headers.get("authorization")??"").replace(/^Bearer /u,""),"hex");if(bearer.length!==secretBytes.length||!timingSafeEqual(bearer,secretBytes))return new Response(null,{status:401});const reader=request.body?.getReader();if(!reader)return new Response(null,{status:400});const chunks:Uint8Array[]=[];let bytes=0;try{for(;;){check();const row=await reader.read();if(row.done)break;bytes+=row.value.byteLength;if(bytes>LOCAL_SESSION_BROKER_REQUEST_MAX_BYTES||chunks.length>=localHubHostPolicyV1().maximumReadinessChunks){await reader.cancel();return new Response(null,{status:413});}chunks.push(row.value);}}finally{reader.releaseLock();}let profileId:string;try{profileId=parseLocalSessionRequestV1(JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(Buffer.concat(chunks))));}catch{return new Response(null,{status:400});}if(!profileIds.includes(profileId))return new Response(null,{status:404});return Response.json(await issue(profileId),{headers:{"cache-control":"no-store"}});
   }).catch(()=>new Response(null,{status:503}));}});
   const record=parseLocalSessionBrokerRecordV1({schema:LOCAL_SESSION_BROKER_SCHEMA,hubOrigin,runId:run.runId,port:server.port,secret,profiles:profileIds});write(path,record);
   if(adminProfileId!==undefined){await issueAdmin(adminProfileId);adminRequests=setInterval(()=>{if(!existsSync(adminRequestPath))return;try{check();const request=lstatSync(adminRequestPath);if(!request.isFile()||request.isSymbolicLink()||request.nlink!==1||request.size!==0)throw Error("Exact empty administrator request required");rmSync(adminRequestPath);void issueAdmin(adminProfileId).catch(()=>{});}catch(error){void ledger.run(async()=>{throw error;});}},LOCAL_ADMIN_REQUEST_POLL_MS);}
   result=await body(Object.freeze({record}));
  }catch(error){failures.push(error);}
  if(adminRequests!==undefined)clearInterval(adminRequests);ledger.stop();try{await server?.stop(true);}catch(error){failures.push(error);}try{await ledger.drain();}catch(error){if(!failures.includes(error))failures.push(error);}ledger.close();secretBytes.fill(0);
  for(const[target,identity]of created)try{const current=lstatSync(target);if(!current.isFile()||current.isSymbolicLink()||current.dev!==identity.dev||current.ino!==identity.ino)throw Error("Created broker record identity changed");rmSync(target);}catch(error){failures.push(error);}
  if(failures.length===1)throw failures[0];if(failures.length)throw new AggregateError(failures,"Session broker body and accepted cleanup failed");return result as T;
 });
}

//#endregion 🎫️SessionBroker
