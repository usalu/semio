/** 🛰 Client of the per-host graph service: asks it for a plan over one local socket or pipe, starts it on demand, reports progress, and cancels the request when its signal aborts. */
import{explainServiceLossV1,readServiceExitRowsV1}from"../../🧾️exit-log/🟨️.mjs";
import{consentIdentityV1}from"../../🔐️consent/🟨️.mjs";
import {connect} from "node:net";
import {closeSync,existsSync,openSync,readFileSync,rmSync,statSync,writeSync} from "node:fs";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {createFrameDecoderV1,encodeFrameV1} from "../../📨️protocol/🟨️.mjs";
import {mintCapabilityV1} from "../../📨️protocol/🔑️capability/🟨️.mjs";
import {canonicalRootV1} from "../../📨️protocol/📍️endpoint/🟨️.mjs";
import {hostEndpointsV1,workspaceIdV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {HOST_POLICY_V1,renderBothV1,renderTextV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import waitSchema from "./🧬️schema/🔣️.json" with {type:"json"};
import {preparePrivateDirectoryV1} from "../../📨️protocol/📍️endpoint/🔐️directory/🟨️.mjs";

/** 🚨 The host service could not answer; `reason` is a refusal code or `down`, `timeout`, `protocol`. */
export class HostUnavailableErrorV1 extends Error{constructor(reason,message){super(message??"Host graph service unavailable ("+reason+")");this.name="HostUnavailableErrorV1";this.reason=reason;}}

/** 🚨 The service refused the request for a reason that retrying cannot fix. */
export class PlanRefusedErrorV1 extends Error{constructor(frame){super(frame.message.en);this.name="PlanRefusedErrorV1";this.code=frame.code;this.message_={...frame.message};this.retry=frame.retry;}}

const sleep=(milliseconds,signal)=>new Promise((resolve,reject)=>{
 if(signal?.aborted){reject(signal.reason);return;}
 const timer=setTimeout(()=>{signal?.removeEventListener("abort",abort);resolve();},milliseconds),abort=()=>{clearTimeout(timer);reject(signal.reason);};
 signal?.addEventListener("abort",abort,{once:true});
});

const logSize=path=>{try{return statSync(path).size;}catch{return -1;}};
const alive=pid=>{try{process.kill(pid,0);return true;}catch(error){return error?.code==="EPERM";}};

function tryConnect(endpoint){
 return new Promise(resolve=>{
  const socket=connect(endpoint);
  socket.once("connect",()=>{socket.removeAllListeners("error");resolve(socket);});
  socket.once("error",error=>{socket.destroy();resolve(error?.code==="ENOENT"||error?.code==="ECONNREFUSED"?undefined:Promise.reject(error));});
 });
}

/** 🪵 The last lines the service wrote to its log after `from` that are no progress heartbeats, as ` [log: …]`: the real cause of a start that died (`Internal error: …`), empty when it wrote none. */
function startOutputV1(path,from){
 try{const lines=readFileSync(path,"utf8").slice(from).split("\n").filter(line=>line.trim()!==""&&!line.startsWith("[script] {")&&!line.startsWith("[repo] still working")&&!line.startsWith("[repository:command] still running")&&!line.startsWith("[capacity]"));return lines.length?" [log: "+lines.slice(-3).join(" | ").slice(0,600)+"]":"";}catch{return"";}
}

function takeLock(path,staleMilliseconds,now=Date.now()){
 for(let attempt=0;attempt<2;attempt++){
  try{const descriptor=openSync(path,"wx",0o600);writeSync(descriptor,JSON.stringify({pid:process.pid,at:now}));closeSync(descriptor);return true;}
  catch(error){
   if(error?.code!=="EEXIST")throw error;
   try{const holder=JSON.parse(readFileSync(path,"utf8"));if(now-holder.at>staleMilliseconds||!alive(holder.pid)){rmSync(path,{force:true});continue;}}catch{rmSync(path,{force:true});continue;}
   return false;
  }
 }
 return false;
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
 preparePrivateDirectoryV1(runtime.directory);
 if(takeLock(lock,policy.lockStaleMilliseconds)){try{return await owned();}finally{rmSync(lock,{force:true});}}
 const began=performance.now();
 while(existsSync(lock)&&performance.now()-began<policy.startCeilingMilliseconds){await sleep(policy.startPollMilliseconds,signal);if(!takeLockProbeV1(lock,policy.lockStaleMilliseconds))break;}
 return joined();
}
const takeLockProbeV1=(path,staleMilliseconds)=>{try{const holder=JSON.parse(readFileSync(path,"utf8"));return Date.now()-holder.at<=staleMilliseconds&&alive(holder.pid);}catch{return false;}};

/** 🌐 A text of the start wait in the requested languages (both when a typed error carries it).
 * @see ./🧬️schema/🔣️.json */
export function waitMessagesV1(key,fields,languages){
 const fill=text=>text.replace(/\{(\w+)\}/gu,(_all,name)=>String(fields[name]));
 return Object.fromEntries(languages.map(language=>[language,fill(waitSchema.definitions.Texts.const[language==="de"?"de":"en"][key])]));
}

/** 🚀 Connects to the host service of `implementation`, starting it through `spawnService` when none answers. Resolves `{socket,runtime,started}`. `spawnService` may resolve a handle `{alive():boolean}` for the process it started: while that process lives the client keeps waiting (a cold start behind a loaded host is silent for minutes, up to `startCeilingMilliseconds`), when it died without a socket the start fails at once as `lost` with the reason from the exit log; without a handle, `startTimeoutMilliseconds` of log silence ends the wait. */
export async function connectHostV1({implementation,identify,environment=process.env,platform=process.platform,spawnService,signal,onProgress=()=>{},languages=["en"],policy:given=HOST_POLICY_V1,ignorePids=[]}){
 const policy={...HOST_POLICY_V1,...given},runtime=hostEndpointsV1(implementation,environment,platform),say=(key,fields)=>{const rendered=renderTextV1(key,fields,languages);return rendered??{en:key};};
 const first=await tryConnect(runtime.endpoint);
 if(first)return{socket:first,runtime,implementation,started:false};
 if(!spawnService)throw new HostUnavailableErrorV1("down");
 preparePrivateDirectoryV1(runtime.directory);
 const began=performance.now(),startedAt=Date.now()-1000;
 const candidates=[{implementation,runtime}],asked={at:-Infinity};
 let owner=false,announced=-2000,seen=logSize(runtime.log),firstSize=Math.max(0,logSize(runtime.log)),lastProgress=performance.now(),handle,checkedAt=-Infinity,living;
 const lives=()=>{if(typeof handle?.alive!=="function")return undefined;if(performance.now()-checkedAt>=policy.startLivenessMilliseconds){checkedAt=performance.now();try{living=handle.alive();}catch{living=undefined;}}return living;};
 try{
  owner=takeLock(runtime.lock,policy.lockStaleMilliseconds);
  if(owner){onProgress({phase:"start",done:0,total:1,message:say("host-start",{})});handle=await spawnService({runtime,implementation,environment});}
  for(;;){
   signal?.throwIfAborted();
   for(const candidate of[...candidates].reverse()){const socket=await tryConnect(candidate.runtime.endpoint);if(socket)return{socket,runtime:candidate.runtime,implementation:candidate.implementation,started:owner};}
   if(identify!==undefined&&performance.now()-asked.at>=waitSchema.definitions.Policy.const.identifyMilliseconds){
    asked.at=performance.now();
    try{const next=identify();if(typeof next==="string"&&!candidates.some(candidate=>candidate.implementation===next)){candidates.push({implementation:next,runtime:hostEndpointsV1(next,environment,platform)});onProgress({phase:"start",done:0,total:0,message:waitMessagesV1("moved",{seen:candidates.length},languages)});}}catch{}
   }
   const refused=consentRefusalOfExitV1(runtime,startedAt,ignorePids);
   if(refused)throw refused;
   const written=logSize(runtime.log);
   if(written!==seen){seen=written;lastProgress=performance.now();}
   const elapsed=Math.round(performance.now()-began);
   const alive=lives();
   if(alive===false)throw new HostUnavailableErrorV1("lost","Host graph service unavailable (lost): "+explainServiceLossV1(runtime.directory)+startOutputV1(runtime.log,firstSize));
   if(alive===true?elapsed>policy.startCeilingMilliseconds:performance.now()-lastProgress>policy.startTimeoutMilliseconds)throw alive===true?new HostUnavailableErrorV1("timeout",Object.values(waitMessagesV1("ceiling",{seconds:Math.round(policy.startCeilingMilliseconds/1000),seen:candidates.length},["en","de"])).join(" / ")):new HostUnavailableErrorV1("timeout");
   if(!owner&&takeLock(runtime.lock,policy.lockStaleMilliseconds)){owner=true;onProgress({phase:"start",done:0,total:1,message:say("host-start",{})});handle=await spawnService({runtime,implementation,environment});}
   if(elapsed-announced>=2000){announced=elapsed;onProgress({phase:"start",done:0,total:0,message:say("host-wait",{elapsed})});}
   await sleep(policy.startPollMilliseconds,signal);
  }
 }finally{if(owner)rmSync(runtime.lock,{force:true});}
}

/** 🔇 The failure of a request whose service sent no frame at all for `seconds`: a service that holds a request keeps it audible (progress, keepalive), so a silent one has lost it. */
const silentErrorV1=(seconds,progressed)=>Object.assign(new HostUnavailableErrorV1("silent","Host graph service unavailable (silent): "+Object.values(renderBothV1("host-silent",{seconds})).join(" / ")),{progressed});

function exchange({socket,runtime,implementation,frame,agent,workspace,languages,signal,onProgress,denialWaitMilliseconds=1000,ignorePids=[],silenceMilliseconds=HOST_POLICY_V1.planSilenceMilliseconds}){
 return new Promise((resolve,reject)=>{
  let finished=false,welcome,progressed=false,heardAt=Date.now(),watch;
  const end=(settle,value)=>{if(finished)return;finished=true;clearInterval(watch);signal?.removeEventListener("abort",abort);socket.destroy();settle(value);};
  if(silenceMilliseconds>0){watch=setInterval(()=>{if(Date.now()-heardAt>=silenceMilliseconds)end(reject,silentErrorV1(Math.round(silenceMilliseconds/1000),progressed));},Math.max(10,Math.min(5000,Math.floor(silenceMilliseconds/4))));watch.unref?.();}
  const abort=()=>{try{socket.write(encodeFrameV1({t:"cancel",id:frame.id}));}catch{}end(reject,signal.reason??new Error("aborted"));};
  let token;
  try{token=mintCapabilityV1(readFileSync(runtime.key),[{k:"scope",v:["query"]},{k:"notAfter",v:Date.now()+60000},{k:"worktree",v:workspace},{k:"agent",v:agent}]);}catch(error){end(reject,Object.assign(new HostUnavailableErrorV1("key"),{missing:error?.code==="ENOENT"}));return;}
  const decoder=createFrameDecoderV1({maximumFrameBytes:HOST_POLICY_V1.maximumFrameBytes,
   onFrame:message=>{
    heardAt=Date.now();
    if(message.t==="welcome"){welcome=message;if(!message.features?.includes("plan"))end(reject,new HostUnavailableErrorV1("protocol","service does not answer plan requests"));return;}
    if(message.t==="progress"){progressed=true;onProgress(message);return;}
    if(message.t==="plan-reply"&&message.id===frame.id){end(resolve,{reply:message,welcome});return;}
    if(message.t==="refused"){
     if(message.retry==="fallback")end(reject,new HostUnavailableErrorV1(message.code));
     else end(reject,Object.assign(new PlanRefusedErrorV1(message),{servicePid:welcome?.service?.pid}));
    }
   },
   onError:()=>end(reject,new HostUnavailableErrorV1("protocol")),
  });
  socket.on("data",chunk=>decoder.push(chunk));
  const lost=()=>Object.assign(new HostUnavailableErrorV1("lost","Host graph service unavailable (lost): "+explainServiceLossV1(runtime.directory)),{progressed});
  const began=Date.now()-2000,lostOrDenied=async()=>{for(let waited=0;waited<=denialWaitMilliseconds;waited+=50){const refused=consentRefusalOfExitV1(runtime,began,ignorePids);if(refused)return refused;if(waited<denialWaitMilliseconds)await sleep(50);}return lost();};
  socket.on("error",()=>{void lostOrDenied().then(error=>end(reject,error));});
  socket.on("close",()=>{void lostOrDenied().then(error=>end(reject,error));});
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

/** 🛰 Asks the host service for the plan of `targets` of the workspace at `root`; retries per lost connection (crash, restart); a connection lost after the service had begun to answer (progress frames seen) attaches to the replacement generation without using up an attempt, `reattachMaximum` times at most, and starts the service when none runs; a service that sends no frame at all for `silenceMilliseconds` (the declared `planSilenceMilliseconds`, 0 turns it off) has lost the request: it ends with the bilingual failure `silent` and is asked again like a lost connection.
 * @see ../../🧪️tests/💓️keepalive/🟦️.ts */
export async function requestPlanV1(options){
 const{root,implementation,targets,agent,priority=5,revision,parent,dirty,arguments:invocation,tool,graphPolicy,language,signal,onProgress=()=>{},environment=process.env,platform=process.platform,spawnService,attempts=3,deadlineMs,ignorePids,silenceMilliseconds,identify}=options;
 const canonical=canonicalRootV1(root),workspace=workspaceIdV1(canonical),languages=[language,...["en","de"].filter(other=>other!==language)];
 let last,reattached=0,keyWaits=0;
 for(let attempt=0;attempt<attempts;attempt++){
  signal?.throwIfAborted();
  const{socket,runtime,implementation:connected}=await connectHostV1({implementation,...identify===undefined?{}:{identify},environment,platform,spawnService,signal,onProgress,languages,...ignorePids?{ignorePids}:{}});
  const frame={t:"plan",id:1,workspace,root:canonical,targets:[...new Set(targets)],agent,priority,language,...(revision?{revision}:{}),...(parent?{parent}:{}),...(dirty?{dirty}:{}),...(invocation?{arguments:invocation}:{}),...(tool?{tool}:{}),...(graphPolicy?{graphPolicy}:{}),...(deadlineMs?{deadlineMs}:{})};
  try{
   const{reply,welcome}=await exchange({socket,runtime,implementation:connected??implementation,frame,agent,workspace,languages,signal,onProgress,...silenceMilliseconds===undefined?{}:{silenceMilliseconds},...ignorePids?{ignorePids}:{}});
   return{reply,welcome,runtime,plan:resolvePlanV1(reply,runtime)};
  }catch(error){
   if(!(error instanceof HostUnavailableErrorV1)||error.reason==="protocol"&&attempt===attempts-1)throw error;
   last=error;
   if(error.reason==="key"){if(!planKeyRetryV1(error,keyWaits))throw error;keyWaits++;attempt--;await new Promise(done=>setTimeout(done,PLAN_KEY_WAIT_V1.milliseconds));continue;}
   if((error.reason==="lost"||error.reason==="silent")&&error.progressed===true&&reattached<HOST_POLICY_V1.reattachMaximum){reattached++;attempt--;}
  }
 }
 throw last;
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

/** 🛑 Retires the service of `implementation` that denied access: asks it to end (`SIGTERM` to the pid its status names) and waits until it no longer answers, `timeoutMilliseconds` at most; answers `{pid,retired}` (`retired` is also true when no service runs). A client that gets a refusal from a service it cannot trust starts its own with this. With `only` (the pid that refused) a service of another pid is a newer generation that someone else already started and is left alone. */
export async function retireHostServiceV1({implementation,environment=process.env,platform=process.platform,timeoutMilliseconds=5000,only,status=requestHostStatusV1,kill=process.kill.bind(process),pause=milliseconds=>sleep(milliseconds)}){
 const found=await status({implementation,environment,platform});
 if(!found)return{pid:undefined,retired:true};
 if(only!==undefined&&found.pid!==only)return{pid:found.pid,retired:true};
 try{kill(found.pid,"SIGTERM");}catch(error){if(error?.code!=="ESRCH")return{pid:found.pid,retired:false};}
 for(let waited=0;waited<timeoutMilliseconds;waited+=50){
  if(!await status({implementation,environment,platform}))return{pid:found.pid,retired:true};
  await pause(50);
 }
 return{pid:found.pid,retired:!await status({implementation,environment,platform})};
}

/** 📊 Asks a running service for its status; resolves undefined when none runs. */
export async function requestHostStatusV1({implementation,environment=process.env,platform=process.platform}){
 const runtime=hostEndpointsV1(implementation,environment,platform),socket=await tryConnect(runtime.endpoint);
 if(!socket)return undefined;
 return new Promise(resolve=>{
  const done=value=>{socket.destroy();resolve(value);};
  const decoder=createFrameDecoderV1({maximumFrameBytes:HOST_POLICY_V1.maximumFrameBytes,onFrame:frame=>{if(frame.t==="service-status")done(frame);},onError:()=>done(undefined)});
  socket.on("data",chunk=>decoder.push(chunk));socket.on("error",()=>done(undefined));socket.on("close",()=>done(undefined));
  socket.write(encodeFrameV1({t:"hello",v:1,client:{name:"repo",pid:process.pid,lane:"status"},languages:["en"],implementation})+encodeFrameV1({t:"host-status",id:1}));
 });
}
