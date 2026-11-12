/** 🛰 Client of the per-host graph service: asks it for a plan over one local socket or pipe, starts it on demand, reports progress, and cancels the request when its signal aborts. */
import{explainServiceLossV1}from"../../🧾️exit-log/🟨️.mjs";
import {connect} from "node:net";
import {closeSync,openSync,readFileSync,rmSync,statSync,writeSync} from "node:fs";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {createFrameDecoderV1,encodeFrameV1} from "../../📨️protocol/🟨️.mjs";
import {mintCapabilityV1} from "../../📨️protocol/🔑️capability/🟨️.mjs";
import {canonicalRootV1} from "../../📨️protocol/📍️endpoint/🟨️.mjs";
import {hostEndpointsV1,workspaceIdV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {HOST_POLICY_V1,renderTextV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
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

/** 🚀 Connects to the host service of `implementation`, starting it through `spawnService` when none answers. Resolves `{socket,runtime,started}`. */
export async function connectHostV1({implementation,environment=process.env,platform=process.platform,spawnService,signal,onProgress=()=>{},languages=["en"],policy=HOST_POLICY_V1}){
 const runtime=hostEndpointsV1(implementation,environment,platform),say=(key,fields)=>{const rendered=renderTextV1(key,fields,languages);return rendered??{en:key};};
 const first=await tryConnect(runtime.endpoint);
 if(first)return{socket:first,runtime,started:false};
 if(!spawnService)throw new HostUnavailableErrorV1("down");
 preparePrivateDirectoryV1(runtime.directory);
 const began=performance.now();
 let owner=false,announced=-2000,seen=logSize(runtime.log),lastProgress=performance.now();
 try{
  owner=takeLock(runtime.lock,policy.lockStaleMilliseconds);
  if(owner){onProgress({phase:"start",done:0,total:1,message:say("host-start",{})});await spawnService({runtime,implementation,environment});}
  for(;;){
   signal?.throwIfAborted();
   const socket=await tryConnect(runtime.endpoint);
   if(socket)return{socket,runtime,started:owner};
   const written=logSize(runtime.log);
   if(written!==seen){seen=written;lastProgress=performance.now();}
   const elapsed=Math.round(performance.now()-began);
   if(performance.now()-lastProgress>policy.startTimeoutMilliseconds)throw new HostUnavailableErrorV1("timeout");
   if(!owner&&takeLock(runtime.lock,policy.lockStaleMilliseconds)){owner=true;onProgress({phase:"start",done:0,total:1,message:say("host-start",{})});await spawnService({runtime,implementation,environment});}
   if(elapsed-announced>=2000){announced=elapsed;onProgress({phase:"start",done:0,total:0,message:say("host-wait",{elapsed})});}
   await sleep(policy.startPollMilliseconds,signal);
  }
 }finally{if(owner)rmSync(runtime.lock,{force:true});}
}

function exchange({socket,runtime,implementation,frame,agent,workspace,languages,signal,onProgress}){
 return new Promise((resolve,reject)=>{
  let finished=false,welcome;
  const end=(settle,value)=>{if(finished)return;finished=true;signal?.removeEventListener("abort",abort);socket.destroy();settle(value);};
  const abort=()=>{try{socket.write(encodeFrameV1({t:"cancel",id:frame.id}));}catch{}end(reject,signal.reason??new Error("aborted"));};
  let token;
  try{token=mintCapabilityV1(readFileSync(runtime.key),[{k:"scope",v:["query"]},{k:"notAfter",v:Date.now()+60000},{k:"worktree",v:workspace},{k:"agent",v:agent}]);}catch{end(reject,new HostUnavailableErrorV1("key"));return;}
  const decoder=createFrameDecoderV1({maximumFrameBytes:HOST_POLICY_V1.maximumFrameBytes,
   onFrame:message=>{
    if(message.t==="welcome"){welcome=message;if(!message.features?.includes("plan"))end(reject,new HostUnavailableErrorV1("protocol","service does not answer plan requests"));return;}
    if(message.t==="progress"){onProgress(message);return;}
    if(message.t==="plan-reply"&&message.id===frame.id){end(resolve,{reply:message,welcome});return;}
    if(message.t==="refused"){
     if(message.retry==="fallback")end(reject,new HostUnavailableErrorV1(message.code));
     else end(reject,new PlanRefusedErrorV1(message));
    }
   },
   onError:()=>end(reject,new HostUnavailableErrorV1("protocol")),
  });
  socket.on("data",chunk=>decoder.push(chunk));
  const lost=()=>new HostUnavailableErrorV1("lost","Host graph service unavailable (lost): "+explainServiceLossV1(runtime.directory));
  socket.on("error",()=>end(reject,lost()));
  socket.on("close",()=>end(reject,lost()));
  if(signal?.aborted){abort();return;}
  signal?.addEventListener("abort",abort,{once:true});
  socket.write(encodeFrameV1({t:"hello",v:1,client:{name:"repo",pid:process.pid,lane:agent},languages,cap:token,implementation})+encodeFrameV1({...frame,cap:token}));
 });
}

/** 🛰 Asks the host service for the plan of `targets` of the workspace at `root`; retries once per lost connection (crash, restart) and starts the service when none runs. */
export async function requestPlanV1(options){
 const{root,implementation,targets,agent,priority=5,revision,parent,dirty,arguments:invocation,tool,graphPolicy,language,signal,onProgress=()=>{},environment=process.env,platform=process.platform,spawnService,attempts=3,deadlineMs}=options;
 const canonical=canonicalRootV1(root),workspace=workspaceIdV1(canonical),languages=[language,...["en","de"].filter(other=>other!==language)];
 let last;
 for(let attempt=0;attempt<attempts;attempt++){
  signal?.throwIfAborted();
  const{socket,runtime}=await connectHostV1({implementation,environment,platform,spawnService,signal,onProgress,languages});
  const frame={t:"plan",id:1,workspace,root:canonical,targets:[...new Set(targets)],agent,priority,language,...(revision?{revision}:{}),...(parent?{parent}:{}),...(dirty?{dirty}:{}),...(invocation?{arguments:invocation}:{}),...(tool?{tool}:{}),...(graphPolicy?{graphPolicy}:{}),...(deadlineMs?{deadlineMs}:{})};
  try{
   const{reply,welcome}=await exchange({socket,runtime,implementation,frame,agent,workspace,languages,signal,onProgress});
   return{reply,welcome,runtime,plan:resolvePlanV1(reply,runtime)};
  }catch(error){
   if(!(error instanceof HostUnavailableErrorV1)||error.reason==="protocol"&&attempt===attempts-1)throw error;
   last=error;
   if(error.reason==="key")throw error;
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
