import{spawn}from"node:child_process";
import{createSidecarRestartDebounceV1}from"../../🌐️remote-execution/🛰️sidecar/🧮️closure/🟦️.ts";

/** 🔄️ The reload of a running host service onto a newer implementation. One `tick` per `policy.checkMilliseconds` asks `identify` which implementation the tree wants now (`{implementation}` when it is sealable, `{implementation,blocked}` when it is not): the same digest keeps the service, another one waits for the quiet period (`createSidecarRestartDebounceV1`, so an edit storm never replaces a service per edit), then `replace` starts the replacement and only a replacement that answers lets the service `drain`. A blocked identity, a failed identification or a replacement that did not start keep this service running and are reported in `state()` (the status frame); a failed replacement is not tried again before `retryMilliseconds`. Single-flight, one reload per life.
 * @see ../🏛️service/🌐️host/🟨️.mjs the service that ticks it, ../../🧭️invocation/🔄️reload/🟦️.ts the identity of the tree, ../../🌐️remote-execution/🛰️sidecar/🟦️.ts the same rule for the sidecars */
export function createHostReloadV1({implementation,identify,replace,drain,policy,now=()=>Date.now(),log=()=>{}}){
 const debounce=createSidecarRestartDebounceV1(policy,now);
 let running=false,finished=false,retryAt=0,state=Object.freeze({state:"current",sinceMs:now()});
 const set=(next,reason,detail)=>{
  if(state.state===next&&state.next===detail?.next&&state.reason===reason)return;
  state=Object.freeze({state:next,sinceMs:state.state===next?state.sinceMs:now(),...detail?.next?{next:detail.next}:{},...reason?{reason}:{}});
  log("reload-"+next,JSON.stringify({next:detail?.next,reason}));
 };
 return Object.freeze({
  state:()=>state,
  async tick(signal){
   if(running||finished||now()<retryAt)return;
   running=true;let next;
   try{
    const found=await identify(signal);next=found.implementation;
    if(found.blocked!==undefined){set("blocked",String(found.blocked).slice(0,512),{next});return;}
    const verdict=debounce.observe(found.implementation,implementation);
    if(verdict.state==="same"){set("current");return;}
    if(verdict.state==="deferred"){set("pending",undefined,{next});return;}
    const outcome=await replace({implementation:found.implementation,signal});
    if(signal?.aborted)return;
    if(!outcome.started){retryAt=now()+policy.retryMilliseconds;set("failed",String(outcome.reason).slice(0,512),{next});return;}
    finished=true;set("draining",undefined,{next});
    void drain();
   }catch(error){
    if(signal?.aborted)return;
    retryAt=now()+policy.retryMilliseconds;set("failed",String(error?.message??error).slice(0,512),{next});
   }finally{running=false;}
  },
 });
}

/** 🚀️ Starts the replacement of a host service as the process `command` (`repo service host start`, which goes through the front door and so through the seal) and resolves `{started}` only when that process ended with status 0 within `timeoutMilliseconds` and a service answers on the wanted implementation (`status`); anything else is `{started:false,reason}` with the tail of its output. Nothing is left running on a refusal: the process is killed when the time or the signal ends first. `spawnProcess` and `status` are the seams of the tests.
 * @see ../🚪️client/🛰️plan/🟨️.mjs `requestHostStatusV1`, ../⌨️command/🌐️host/🟨️.mjs `start` */
export function replaceHostServiceV1({command,environment,cwd,timeoutMilliseconds,status,spawnProcess=spawn}){
 return({implementation,signal})=>new Promise(resolve=>{
  let tail="",done=false,child;
  const end=async(started,reason)=>{
   if(done)return;done=true;clearTimeout(timer);signal?.removeEventListener("abort",abort);
   if(!started){try{child?.kill("SIGTERM");}catch{}resolve({started:false,reason});return;}
   const found=await Promise.resolve(status({implementation})).catch(()=>undefined);
   resolve(found&&found.implementation===implementation?{started:true}:{started:false,reason:"no service answers on "+implementation.slice(0,12)});
  };
  const abort=()=>{void end(false,"cancelled");},timer=setTimeout(()=>{void end(false,"timeout after "+Math.round(timeoutMilliseconds/1000)+" s: "+tail.trim().slice(-200));},timeoutMilliseconds);
  signal?.addEventListener("abort",abort,{once:true});
  try{
   child=spawnProcess(command[0],command.slice(1),{cwd,env:environment,stdio:["ignore","pipe","pipe"],windowsHide:true});
  }catch(error){void end(false,String(error?.message??error));return;}
  const collect=chunk=>{tail=(tail+chunk).slice(-1000);};
  child.stdout?.on("data",collect);child.stderr?.on("data",collect);
  child.once("error",error=>{void end(false,String(error?.message??error));});
  child.once("close",code=>{void end(code===0,code===0?undefined:"exit "+code+": "+tail.trim().slice(-200));});
 });
}
