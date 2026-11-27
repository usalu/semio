/** 🛰 Client of the per-host graph service: asks it for a plan over one local socket or pipe, starts it on demand, reports progress, and cancels the request when its signal aborts. */
import{explainServiceLossV1,readServiceExitRowsV1}from"../../🧾️exit-log/🟨️.mjs";
import{consentIdentityV1}from"../../🔐️consent/🟨️.mjs";
import {connect} from "node:net";
import {closeSync,fstatSync,openSync,readFileSync,readSync,statSync} from "node:fs";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {createFrameDecoderV1,encodeFrameV1} from "../../📨️protocol/🟨️.mjs";
import {mintCapabilityV1} from "../../📨️protocol/🔑️capability/🟨️.mjs";
import {canonicalRootV1} from "../../📨️protocol/📍️endpoint/🟨️.mjs";
import {hostEndpointsV1,workspaceIdV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {HOST_POLICY_V1,renderBothV1,renderTextV1,validatorV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import waitSchema from "./🧬️schema/🔣️.json" with {type:"json"};
import {preparePrivateDirectoryV1} from "../../📨️protocol/📍️endpoint/🔐️directory/🟨️.mjs";
import {ownedSliceReportSinkV1} from "../../../🧠️memory/📋️slice-report/🏷️owner/🟦️.ts";

/** 🚨 The host service could not answer; `reason` is a refusal code or `down`, `timeout`, `protocol`. */
export class HostUnavailableErrorV1 extends Error{
 constructor(reason,message,startup){super(message??(startup?["en","de"].map(language=>startupNoticeV1(startup,language)).join(" / "):"Host graph service unavailable ("+reason+")"));this.name="HostUnavailableErrorV1";this.reason=reason;if(startup)this.startup=startup;}
 notice(language){
  if(this.startup)return this.reason+": "+startupNoticeV1(this.startup,language);
  if(this.planSilence)return this.reason+": "+planSilenceNoticeV1(this.planSilence,language);
  return this.reason+(this.reason==="lost"?this.message.replace(/^[^:]*\):/u,""):"");
 }
}

/** 🚨 The service refused the request for a reason that retrying cannot fix. */
export class PlanRefusedErrorV1 extends Error{constructor(frame){super(frame.message.en);this.name="PlanRefusedErrorV1";this.code=frame.code;this.message_={...frame.message};this.retry=frame.retry;}}

const sleep=(milliseconds,signal)=>new Promise((resolve,reject)=>{
 if(signal?.aborted){reject(signal.reason);return;}
 const timer=setTimeout(()=>{signal?.removeEventListener("abort",abort);resolve();},milliseconds),abort=()=>{clearTimeout(timer);reject(signal.reason);};
 signal?.addEventListener("abort",abort,{once:true});
});

const logSize=path=>{try{return statSync(path).size;}catch{return -1;}};

function tryConnect(endpoint){
 return new Promise(resolve=>{
  const socket=connect(endpoint);
  socket.once("connect",()=>{socket.removeAllListeners("error");resolve(socket);});
  socket.once("error",error=>{socket.destroy();resolve(HOST_POLICY_V1.unavailableConnectErrors.includes(error?.code)?undefined:Promise.reject(error));});
 });
}

const startupDiagnosticV1=waitSchema.definitions.Diagnostic.const;

/** 🪵 Reads only recent log bytes for one observed startup coordinate. */
/** 🌱️ The phase the starting service reports in its log (`[repo] still working after N s; last phase started: <phase>`), with the files it walked, or nothing while it has said none.
 * @see ./🧬️schema/🔣️.json */
function bootPhaseV1(path,from){
 let file;
 try{
  file=openSync(path,"r");const facts=fstatSync(file),start=Math.max(from<=facts.size?from:0,facts.size-8192),buffer=Buffer.alloc(facts.size-start);
  if(buffer.length===0)return undefined;
  const text=buffer.subarray(0,readSync(file,buffer,0,buffer.length,start)).toString("utf8"),found=[...text.matchAll(/^\[repo\] still working after \d+ s; last phase started: (\S+)(?: \(walked (\d+) files)?/gmu)].at(-1);
  return found===undefined?undefined:found[1]+(found[2]===undefined?"":" ("+found[2]+" files)");
 }catch{return undefined;}finally{if(file!==undefined)try{closeSync(file);}catch{}}
}

function startOutputV1(path,from,since,lost=false){
 let file;
 try{
  file=openSync(path,"r");const facts=fstatSync(file);
  if(!facts.isFile()||facts.mtimeMs<since)return "";
  const first=from<=facts.size?from:0,offset=Math.max(first,facts.size-startupDiagnosticV1.tailBytes),buffer=Buffer.alloc(facts.size-offset);
  const lines=buffer.subarray(0,readSync(file,buffer,0,buffer.length,offset)).toString("utf8").split("\n");
  if(offset>first)lines.shift();
  const text=lines.filter(line=>line.trim()!==""&&!line.startsWith("[script] {")&&!line.startsWith("[repo] still working")&&!line.startsWith("[repository:command] still running")&&!line.startsWith("[capacity]")).slice(-3).join(" | ");
  return lost?text.slice(0,startupDiagnosticV1.lostTailCharacters):text.slice(-startupDiagnosticV1.tailCharacters);
 }catch{return "";}finally{if(file!==undefined)try{closeSync(file);}catch{}}
}

function startupNoticeV1(startup,language){
 const fields={seconds:startup.limitMilliseconds/1000,seen:startup.seen,pid:startup.pid??"?",candidates:startup.candidates.map(row=>row.tag).join(", ")},render=key=>waitMessagesV1(key,fields,[language])[language];
 const logs=startup.candidates.filter(row=>row.log).map(row=>"["+row.tag+" "+row.log+"]");
 return [render(startup.mode==="live-ceiling"?"ceiling":"silence"),...(startup.mode==="live-ceiling"?[render("live")]:[]),render("candidates"),...logs].join(" ");
}

/** 🔐 Retains one stable private startup file through the host's process-owned kernel lock. */
async function tryStartupOwnerV1(path,signal){
 const{physicalLockBackendV1}=await import("../../../../../../../../🔨️modules/🏃️process/🔒️leases/📁️physical/🟦️.ts");
 signal?.throwIfAborted();
 const lease=physicalLockBackendV1();let owned=false;
 try{
  try{closeSync(openSync(path,"wx",0o600));}catch(error){if(error?.code!=="EEXIST")throw error;}
  lease.retain(path);
  if(!lease.tryLock())return;
  lease.verify();owned=true;return lease;
 }finally{if(!owned)lease.close();}
}

/** 🔐️ The refusal a service left when it ended because macOS keeps it from reading the checkout (its exit row `access-denied` of this implementation, written after `since`): the typed refusal `consent` with both languages, `undefined` when there is none; the rows of the pids in `ignore` (a service this client retired, whose row is written after its replacement started) are another generation's and never refuse the next start. */
function consentRefusalOfExitV1(runtime,since,ignore=[]){
 const row=lastAccessDenialRowV1(runtime,since,ignore);
 if(row===undefined)return undefined;
 const key=row.changed===true?"consent-launcher-changed":row.state==="waiting"?"consent-waiting":"consent-denied";
 return Object.assign(new PlanRefusedErrorV1({t:"refused",code:"consent",retry:"later",message:renderBothV1(key,{folder:String(row.folder??"?"),root:String(row.root??"?"),place:String(row.place??row.root??"?"),identity:consentIdentityV1()?.path??"repo"})}),Number.isInteger(row.pid)?{servicePid:row.pid}:{});
}

const lastAccessDenialRowV1=(runtime,since,ignore=[])=>readServiceExitRowsV1(runtime.directory).findLast(entry=>entry.event==="exit"&&entry.reason==="access-denied"&&entry.tag===runtime.tag&&entry.at>=since&&!ignore.includes(entry.pid));

/** 🔍️ The last exit row `access-denied` of the service of `implementation` (written at or after `since`): its route (`launchd`, `direct`, `foreground`), pid, root and place, or `undefined`; what a client reads to learn which start produced a service without access. */
export function lastAccessDenialV1({implementation,environment=process.env,platform=process.platform,since=0}){
 return lastAccessDenialRowV1(hostEndpointsV1(implementation,environment,platform),since);
}

/** 🔒 The single host-wide owner of the restart after a consent denial: the first client takes the lock `restart-<tag>.lock` and runs `owned()` (retire the denied generation, start the next); every other client waits until the owner is done, up to `startCeilingMilliseconds`, and then runs `joined()`, which attaches to the generation the owner started instead of retiring it. */
export async function withRestartOwnerV1({implementation,environment=process.env,platform=process.platform,signal,policy=HOST_POLICY_V1},owned,joined){
 const runtime=hostEndpointsV1(implementation,environment,platform),lock=join(runtime.directory,"restart-"+runtime.tag+".lock");
 signal?.throwIfAborted();
 preparePrivateDirectoryV1(runtime.directory);
 const lease=await tryStartupOwnerV1(lock,signal);
 if(lease){try{signal?.throwIfAborted();return await owned();}finally{lease.close();}}
 const began=performance.now();
 for(let remaining=policy.startCeilingMilliseconds;remaining>0;remaining=policy.startCeilingMilliseconds-(performance.now()-began)){
  await sleep(Math.min(remaining,policy.startPollMilliseconds),signal);
  const released=await tryStartupOwnerV1(lock,signal);if(released){released.close();break;}
 }
 signal?.throwIfAborted();
 return joined();
}

/** 🌐 A text of the start wait in the requested languages (both when a typed error carries it).
 * @see ./🧬️schema/🔣️.json */
export function waitMessagesV1(key,fields,languages){
 const fill=text=>text.replace(/\{(\w+)\}/gu,(_all,name)=>String(fields[name]));
 return Object.fromEntries(languages.map(language=>[language,fill(waitSchema.definitions.Texts.const[language==="de"?"de":"en"][key])]));
}

/** 🚀 Connects to the host service of `implementation`, starting it through `spawnService` when none answers. Resolves `{socket,runtime,started}`. `spawnService` may resolve a handle `{alive():boolean}` for the process it started: while that process lives the client keeps waiting (a cold start behind a loaded host is silent for minutes, up to `startCeilingMilliseconds`), when it died without a socket the start fails at once as `lost` with the reason from the exit log; without a handle, `startTimeoutMilliseconds` of log silence ends the wait. */
export async function connectHostV1({implementation,identify,environment=process.env,platform=process.platform,spawnService,signal,onProgress=()=>{},languages=["en"],policy:given=HOST_POLICY_V1,ignorePids=[]}){
 const policy={...HOST_POLICY_V1,...given},runtime=hostEndpointsV1(implementation,environment,platform),say=(key,fields)=>{const rendered=renderTextV1(key,fields,languages);return rendered??{en:key};};
 const initial={implementation,runtime},attach=async(candidate,started)=>{
  signal?.throwIfAborted();const socket=await tryConnect(candidate.runtime.endpoint);
  if(signal?.aborted){socket?.destroy();signal.throwIfAborted();}
  return socket?{socket,implementation:candidate.implementation,runtime:candidate.runtime,started}:undefined;
 };
 const first=await attach(initial,false);
 if(first)return first;
 if(!spawnService)throw new HostUnavailableErrorV1("down");
 preparePrivateDirectoryV1(runtime.directory);
 const began=performance.now(),logSince=Date.now(),startedAt=logSince-1000;
 const initialSize=logSize(runtime.log),initialLogOffset=Math.max(0,initialSize),candidates=[{...initial,logOffset:initialLogOffset}],asked={at:-Infinity};
 let owner,started=false,announced=-2000,seen=initialSize,lastProgress=performance.now(),handle,checkedAt=-Infinity,living;
 const lives=()=>{if(typeof handle?.alive!=="function")return undefined;if(performance.now()-checkedAt>=policy.startLivenessMilliseconds){checkedAt=performance.now();try{living=handle.alive();}catch{living=undefined;}}return living;};
 const start=async()=>{
  owner=await tryStartupOwnerV1(runtime.lock,signal);
  if(!owner)return;
  const existing=await attach(initial,false);if(existing)return existing;
  owner.verify();onProgress({phase:"start",done:0,total:1,message:say("host-start",{})});signal?.throwIfAborted();owner.verify();
  started=true;handle=await spawnService({runtime,implementation,environment});
 };
 try{
  const existing=await start();if(existing)return existing;
  for(;;){
   signal?.throwIfAborted();
   for(const candidate of[...candidates].reverse()){const found=await attach(candidate,started);if(found)return found;}
   if(identify!==undefined&&performance.now()-asked.at>=waitSchema.definitions.Policy.const.identifyMilliseconds){
    asked.at=performance.now();
    try{const next=identify();if(typeof next==="string"&&!candidates.some(candidate=>candidate.implementation===next)){candidates.push({implementation:next,runtime:hostEndpointsV1(next,environment,platform),logOffset:0});onProgress({phase:"start",done:0,total:0,message:waitMessagesV1("moved",{seen:candidates.length},languages)});}}catch{}
   }
   const refused=consentRefusalOfExitV1(runtime,startedAt,ignorePids);
   if(refused)throw refused;
   const written=logSize(runtime.log);
   if(written!==seen){seen=written;lastProgress=performance.now();}
   const elapsed=Math.round(performance.now()-began);
   const alive=lives();
   if(alive===false){const log=startOutputV1(runtime.log,initialLogOffset,logSince,true);throw new HostUnavailableErrorV1("lost","Host graph service unavailable (lost): "+explainServiceLossV1(runtime.directory)+(log?" [log: "+log+"]":""));}
   if(alive===true?elapsed>policy.startCeilingMilliseconds:performance.now()-lastProgress>policy.startTimeoutMilliseconds){
    const keepFrom=Math.max(1,candidates.length-startupDiagnosticV1.maximumCandidates+1),observations=[candidates[0],...candidates.slice(keepFrom)].map(candidate=>Object.freeze({tag:candidate.runtime.tag,log:startOutputV1(candidate.runtime.log,candidate.logOffset,logSince)}));
    const startup=Object.freeze({mode:alive===true?"live-ceiling":"log-silence",elapsedMilliseconds:elapsed,limitMilliseconds:alive===true?policy.startCeilingMilliseconds:policy.startTimeoutMilliseconds,pid:Number.isSafeInteger(handle?.pid)&&handle.pid>0?handle.pid:null,seen:candidates.length,candidates:Object.freeze(observations)});
    throw new HostUnavailableErrorV1("timeout",undefined,startup);
   }
   if(!owner){const existing=await start();if(existing)return existing;}
   if(elapsed-announced>=2000){announced=elapsed;const booting=bootPhaseV1(runtime.log,initialLogOffset);onProgress({phase:"start",done:0,total:0,message:booting===undefined?say("host-wait",{elapsed}):say("host-wait-phase",{elapsed,phase:booting})});}
   await sleep(policy.startPollMilliseconds,signal);
  }
 }finally{owner?.close();}
}

const validatePlanAcceptance=validatorV1("Accepted"),validatePlanProgress=validatorV1("Progress");

function planSilenceNoticeV1(diagnostic,language){
 const selected=language==="de"?"de":"en",text=(key,fields)=>renderTextV1(key,fields,[selected])[selected];
 return [text("host-silent",{seconds:diagnostic.limitMilliseconds/1000}),text("host-silent-"+diagnostic.stage),...(diagnostic.lastProgress===undefined?[]:[text("host-silent-progress",{phase:diagnostic.lastProgress})])].join(" ");
}

/** 🔇 Reports the last confirmed evidence for a silent plan on this connection. */
const silentErrorV1=(diagnostic,progressed)=>Object.assign(new HostUnavailableErrorV1("silent","Host graph service unavailable (silent): "+["en","de"].map(language=>planSilenceNoticeV1(diagnostic,language)).join(" / ")),{progressed,planSilence:Object.freeze(diagnostic)});

function exchange({socket,runtime,implementation,frame,agent,workspace,languages,signal,onProgress,sliceReport,denialWaitMilliseconds=1000,ignorePids=[],silenceMilliseconds=HOST_POLICY_V1.planSilenceMilliseconds,retainForRetry,onJobProgress}){
 return new Promise((resolve,reject)=>{
  let finished=false,welcome,accepted=false,jobProgressed=false,lastProgress,heardAt=Date.now(),watch;
  const end=(settle,value,retain=false)=>{if(finished)return;finished=true;clearInterval(watch);signal?.removeEventListener("abort",abort);if(retain)retainForRetry(socket,runtime.endpoint,silenceMilliseconds);else socket.destroy();settle(value);};
  if(silenceMilliseconds>0){watch=setInterval(()=>{if(Date.now()-heardAt>=silenceMilliseconds)end(reject,silentErrorV1({stage:welcome===undefined?"before-welcome":accepted?"after-acceptance":"before-acceptance",limitMilliseconds:silenceMilliseconds,lastProgress},lastProgress!==undefined),jobProgressed&&retainForRetry!==undefined);},Math.max(10,Math.min(5000,Math.floor(silenceMilliseconds/4))));watch.unref?.();}
  const abort=()=>{try{socket.write(encodeFrameV1({t:"cancel",id:frame.id}));}catch{}end(reject,signal.reason??new Error("aborted"));};
  let token;
  try{token=mintCapabilityV1(readFileSync(runtime.key),[{k:"scope",v:["query"]},{k:"notAfter",v:Date.now()+60000},{k:"worktree",v:workspace},{k:"agent",v:agent}]);}catch(error){end(reject,Object.assign(new HostUnavailableErrorV1("key"),{missing:error?.code==="ENOENT"}));return;}
  const decoder=createFrameDecoderV1({maximumFrameBytes:HOST_POLICY_V1.maximumFrameBytes,
   onFrame:message=>{
    if(finished||message===null||typeof message!=="object")return;
    if(message.t==="welcome"){
     if(welcome!==undefined)return;
     if(message.v!==1||!Array.isArray(message.features)||!message.features.includes("plan")){end(reject,new HostUnavailableErrorV1("protocol","service does not answer plan requests"));return;}
     welcome=message;heardAt=Date.now();return;
    }
    if(message.t==="refused"&&(message.id===undefined||message.id===frame.id)){
     if(message.retry==="fallback")end(reject,new HostUnavailableErrorV1(message.code));
     else end(reject,Object.assign(new PlanRefusedErrorV1(message),{servicePid:welcome?.service?.pid}));
     return;
    }
    if(welcome===undefined||message.id!==frame.id)return;
    if(message.t==="accepted"&&validatePlanAcceptance(message).length===0){if(!accepted){accepted=true;heardAt=Date.now();}return;}
    if(message.t==="slice-report"&&sliceReport!==undefined&&typeof message.report==="string"){accepted=true;heardAt=Date.now();sliceReport(message.report);return;}
    if(message.t==="progress"&&validatePlanProgress(message).length===0){
     accepted=true;heardAt=Date.now();lastProgress=message.phase;
     if(["queued","load","plan","slice"].includes(message.phase)){jobProgressed=true;onJobProgress?.();}
     onProgress(message);return;
    }
    if(message.t==="plan-reply")end(resolve,{reply:message,welcome});
   },
   onError:()=>end(reject,new HostUnavailableErrorV1("protocol")),
  });
  socket.on("data",chunk=>decoder.push(chunk));
  const lost=()=>Object.assign(new HostUnavailableErrorV1("lost","Host graph service unavailable (lost): "+explainServiceLossV1(runtime.directory)),{progressed:lastProgress!==undefined});
  const began=Date.now()-2000,lostOrDenied=async()=>{for(let waited=0;waited<=denialWaitMilliseconds;waited+=50){const refused=consentRefusalOfExitV1(runtime,began,ignorePids);if(refused)return refused;if(waited<denialWaitMilliseconds)await sleep(50);}return lost();};
  const disconnected=()=>{if(!finished)void lostOrDenied().then(error=>end(reject,error));};
  socket.on("error",disconnected);
  socket.on("close",disconnected);
  if(signal?.aborted){abort();return;}
  signal?.addEventListener("abort",abort,{once:true});
  socket.write(encodeFrameV1({t:"hello",v:1,client:{name:"repo",pid:process.pid,lane:agent},languages,cap:token,implementation})+encodeFrameV1({...frame,cap:token}));
 });
}

/** 🗝️ How long a plan request waits for the key file of a service that is being replaced: the old generation removed it and the new one writes it once it listens. */
export const PLAN_KEY_WAIT_V1=Object.freeze({maximum:30,milliseconds:1000});

/** 🗝️ Whether a request retries after an unreadable key: only a key file that is missing (ENOENT, the service is between two generations) and not more than `maximum` times; any other failure (permission, damage) stays a refusal.
 * @see ../../🧪️tests/🚪️client/🗝️key/🟦️.ts */
export function planKeyRetryV1(error,waited){return error?.reason==="key"&&error.missing===true&&waited<PLAN_KEY_WAIT_V1.maximum;}

/** 🛟 What a request does about an unreadable key: `wait` while the service may write it back (its next check does, see `keepRootKeyV1`), `restart` once when it did not (a service of an older generation that never heals the key: it is retired and the next start writes a new one), `fail` for every other failure.
 * @see ../../🧪️tests/🛟️key-keeper/🟦️.ts */
export function planKeyActionV1(error,waited,restarted){
 if(planKeyRetryV1(error,waited))return"wait";
 return error?.reason==="key"&&error.missing===true&&!restarted?"restart":"fail";
}

/** 🛰 Requests a plan and retries lost connections. Exact progress preserves an attempt up to `reattachMaximum`. After `silenceMilliseconds` without activity for this connection's plan, the request reports its last confirmed stage and retries like a lost connection. When a silent request received job progress, its connection stays open for at most one silence interval while the retry joins the existing job. Only the current connection may answer the request. Zero disables the silence timeout.
 * @see ../../🧪️tests/💓️keepalive/🟦️.ts */
export async function requestPlanV1(options){
 const{root,implementation,targets,agent,priority=5,revision,parent,dirty,arguments:invocation,tool,graphPolicy,language,signal,onProgress=()=>{},environment=process.env,platform=process.platform,spawnService,attempts=3,deadlineMs,ignorePids,silenceMilliseconds,identify}=options;
 const canonical=canonicalRootV1(root),workspace=workspaceIdV1(canonical),languages=[language,...["en","de"].filter(other=>other!==language)],sliceReport=ownedSliceReportSinkV1(canonical,environment);
 const frame={t:"plan",id:1,workspace,root:canonical,targets:[...new Set(targets)],agent,priority,language,...(revision?{revision}:{}),...(parent?{parent}:{}),...(dirty?{dirty:{...dirty,paths:[...dirty.paths]}}:{}),...(invocation?{arguments:[...invocation]}:{}),...(tool?{tool}:{}),...(graphPolicy?{graphPolicy}:{}),...(deadlineMs?{deadlineMs}:{}),...(/^[A-Za-z0-9_-]{8,32}$/u.test(environment.SEMIO_HOST_CAPACITY_GRANT??"")?{grant:environment.SEMIO_HOST_CAPACITY_GRANT}:{}),...sliceReport===undefined?{}:{sliceReport:true}};
 let last,reattached=0,keyWaits=0,keyRestarted=false,previous;
 const releasePrevious=()=>previous?.release();
 const retainForRetry=(socket,endpoint,milliseconds)=>{
  releasePrevious();
  const release=()=>{
   clearTimeout(expiry);signal?.removeEventListener("abort",release);socket.off("close",release);socket.destroy();
   if(previous?.release===release)previous=undefined;
  };
  const expiry=setTimeout(release,milliseconds);expiry.unref?.();
  previous={endpoint,release};
  socket.once("close",release);signal?.addEventListener("abort",release,{once:true});
  if(signal?.aborted)release();
 };
 try{
  for(let attempt=0;attempt<attempts;attempt++){
   signal?.throwIfAborted();
   const{socket,runtime,implementation:connected}=await connectHostV1({implementation,...identify===undefined?{}:{identify},environment,platform,spawnService,signal,onProgress,languages,...ignorePids?{ignorePids}:{}});
   if(previous&&previous.endpoint!==runtime.endpoint)releasePrevious();
   try{
    const canRetry=attempt+1<attempts||reattached<HOST_POLICY_V1.reattachMaximum;
    const{reply,welcome}=await exchange({socket,runtime,implementation:connected??implementation,frame,agent,workspace,languages,signal,onProgress,sliceReport,retainForRetry:canRetry?retainForRetry:undefined,onJobProgress:releasePrevious,...silenceMilliseconds===undefined?{}:{silenceMilliseconds},...ignorePids?{ignorePids}:{}});
    return{reply,welcome,runtime,plan:resolvePlanV1(reply,runtime)};
   }catch(error){
    if(!(error instanceof HostUnavailableErrorV1)||error.reason==="protocol"&&attempt===attempts-1)throw error;
    last=error;
    if(error.reason==="key"){
     const action=planKeyActionV1(error,keyWaits,keyRestarted);
     if(action==="fail"||action==="restart"&&spawnService===undefined)throw error;
     attempt--;
     if(action==="wait"){keyWaits++;await new Promise(done=>setTimeout(done,PLAN_KEY_WAIT_V1.milliseconds));}
     else{keyRestarted=true;await retireHostServiceV1({implementation,environment,platform,signal}).catch(()=>undefined);}
     continue;
    }
    if((error.reason==="lost"||error.reason==="silent")&&error.progressed===true&&reattached<HOST_POLICY_V1.reattachMaximum){reattached++;attempt--;}
   }
  }
  throw last;
 }finally{releasePrevious();}
}

/** 📦 The plan of a reply: inline, or read and verified from the content-addressed spill the reference names. */
export function resolvePlanV1(reply,runtime){
 if(reply.plan.tasks)return reply.plan;
 const{hash:digest,size:bytes}=reply.plan.ref,path=join(runtime.directory,"plans-"+runtime.tag,digest+".json");
 const body=readFileSync(path,"utf8");
 if(Buffer.byteLength(body)!==bytes||createHash("sha256").update(body).digest("hex")!==digest)throw new HostUnavailableErrorV1("protocol","plan reference does not match its content");
 return JSON.parse(body);
}

/** 🔗 Holds one open, request-free connection to a running host service for as long as the caller works on something that lives off the service (the remote cache endpoint it hosts): an open connection keeps the idle rule from retiring the service. Resolves a release function, or a no-op when no service runs; losing the connection is no error here, the service ending for another reason ends the work's cache access on its own. */
export async function holdHostV1({implementation,environment=process.env,platform=process.platform,agent="hold"}){
 const runtime=hostEndpointsV1(implementation,environment,platform),socket=await tryConnect(runtime.endpoint);
 if(!socket)return async()=>{};
 socket.on("error",()=>{});
 socket.write(encodeFrameV1({t:"hello",v:1,client:{name:"repo",pid:process.pid,lane:agent},languages:["en"],implementation}));
 return async()=>{socket.destroy();};
}

/** 🛑 Retires the exact service named by its first status reply and bounds the post-signal wait. A silent endpoint raises the status timeout without proving retirement. */
export async function retireHostServiceV1({implementation,environment=process.env,platform=process.platform,signal,timeoutMilliseconds=HOST_POLICY_V1.statusReplyMilliseconds,only,status=requestHostStatusV1,kill=process.kill.bind(process),pause=sleep}){
 const inspect=milliseconds=>status({implementation,environment,platform,signal,timeoutMilliseconds:Math.min(HOST_POLICY_V1.statusReplyMilliseconds,Math.max(1,Math.ceil(milliseconds)))});
 const found=await inspect(timeoutMilliseconds);
 if(!found)return{pid:undefined,retired:true};
 if(only!==undefined&&found.pid!==only)return{pid:found.pid,retired:true};
 signal?.throwIfAborted();
 try{kill(found.pid,"SIGTERM");}catch(error){if(error?.code!=="ESRCH")return{pid:found.pid,retired:false};}
 const deadline=performance.now()+timeoutMilliseconds;
 for(let remaining=deadline-performance.now();remaining>0;remaining=deadline-performance.now()){
  const current=await inspect(remaining);
  if(!current||current.pid!==found.pid)return{pid:found.pid,retired:true};
  remaining=deadline-performance.now();
  if(remaining>0)await pause(Math.min(50,remaining),signal);
 }
 return{pid:found.pid,retired:false};
}

/** 📊 Asks the exact implementation's local endpoint for status under one cancellable deadline. */
export async function requestHostStatusV1({implementation,environment=process.env,platform=process.platform,signal,timeoutMilliseconds=HOST_POLICY_V1.statusReplyMilliseconds}){
 signal?.throwIfAborted();
 const {endpoint}=hostEndpointsV1(implementation,environment,platform);
 return new Promise((resolve,reject)=>{
  const socket=connect(endpoint);
  let connected=false,finished=false;
  const finish=(complete,value)=>{
   if(finished)return;
   finished=true;clearTimeout(timer);signal?.removeEventListener("abort",abort);socket.destroy();complete(value);
  };
  const abort=()=>finish(reject,signal.reason),timer=setTimeout(()=>finish(reject,new HostUnavailableErrorV1("timeout")),timeoutMilliseconds);
  signal?.addEventListener("abort",abort,{once:true});
  const decoder=createFrameDecoderV1({maximumFrameBytes:HOST_POLICY_V1.maximumFrameBytes,onFrame:frame=>{if(frame.t==="service-status")finish(resolve,frame);},onError:()=>finish(resolve,undefined)});
  socket.on("data",chunk=>decoder.push(chunk));socket.on("close",()=>finish(resolve,undefined));
  socket.on("error",error=>{if(connected||HOST_POLICY_V1.unavailableConnectErrors.includes(error?.code))finish(resolve,undefined);else finish(reject,error);});
  socket.once("connect",()=>{
   if(finished)return;
   connected=true;
   socket.write(encodeFrameV1({t:"hello",v:1,client:{name:"repo",pid:process.pid,lane:"status"},languages:["en"],implementation})+encodeFrameV1({t:"host-status",id:1}));
  });
 });
}
