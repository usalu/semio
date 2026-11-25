/** ⌨ `repo service host …`: start, status, stop and foreground run of the per-host graph service. */
import {HostUnavailableErrorV1,connectHostV1,requestHostStatusV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {spawnSync} from "node:child_process";
import {spawnHostServiceV1} from "../../🏗️composition/🌐️host/🟨️.mjs";
import {stopServiceJobsV1} from "../../🏗️composition/🌐️host/🍎️launchd/🟨️.mjs";
import {consentIdentityV1,probeConsentV1} from "../../🔐️consent/🟨️.mjs";
import {HOST_POLICY_V1,renderTextV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const sleep=milliseconds=>new Promise(resolve=>setTimeout(resolve,milliseconds));

/** ⌨ Runs one host subcommand and returns the process exit code. `runForeground()` runs the service in this process; `command` is the argument vector that does so in a detached child. */
export async function runHostCommandV1({args,languages,implementation,runForeground,command,environment=process.env,platform=process.platform,cwd=process.cwd(),stdout=process.stdout,stderr=process.stderr,signal,policy,probe=probeConsentV1,launchctl=(program,list)=>{const result=spawnSync(program,list,{windowsHide:true,encoding:"utf8",timeout:10000});return{status:result.status??1,stdout:result.stdout??""};}}){
 const say=(key,fields)=>{const text=renderTextV1(key,fields,languages);return text?text[languages[0]]??Object.values(text)[0]:key;};
 const print=text=>stdout.write(text+"\n"),fail=text=>stderr.write(text+"\n"),sub=args[0];
 const statusLimit=policy?.statusReplyMilliseconds??HOST_POLICY_V1.statusReplyMilliseconds;
 const status=(timeoutMilliseconds=statusLimit)=>requestHostStatusV1({implementation,environment,platform,signal,timeoutMilliseconds});
 try{switch(sub){
  case"run":return runForeground();
  case"status":{
   const found=await status();
   if(!found){print(say("host-down",{}));return 1;}
   if(args.includes("--output=json")){print(JSON.stringify(found,null,2));return 0;}
   print(say("host-started",{pid:found.pid,workspaces:found.workspaces.length,megabytes:found.rssMb}));
   if(found.identity)print(say("host-identity",{code:found.identity.implementation.slice(0,12),root:found.identity.root,state:found.identity.state||"(checkout)",started:new Date(found.startedAt).toISOString(),inFlight:found.inFlight,queued:found.queue,endpoint:found.identity.endpoint,log:found.identity.log}));
   if(found.owner)print(say("host-owner-"+found.owner.route,{pid:found.owner.pid,seconds:found.owner.idleSeconds}));
   if(found.memory)print(say("host-memory",{rss:found.memory.rssMb,hwm:found.memory.hwmMb??"?",heap:found.memory.heapMb,objects:found.memory.objectCount,caches:found.memory.caches.length}));
   if(found.plans)print(say("host-plans",{total:found.plans.total,hits:found.plans.hits,computed:found.plans.computed,seconds:Math.round(found.plans.lastMilliseconds/1000),budget:found.plans.budgetRefusals,reinferred:found.plans.load?.ownersReinferred??"?",cutoff:found.plans.load?.ownersCutOff??"?",files:found.plans.captureFiles,probes:found.plans.captureProbes}));
   if(found.reload&&found.reload.state!=="current")print(say("host-reload-"+found.reload.state,{next:String(found.reload.next??"").slice(0,12),seconds:Math.round((found.reload.state==="draining"?HOST_POLICY_V1.reload.drainStallMilliseconds:HOST_POLICY_V1.reload.maximumDeferralMilliseconds)/1000),reason:found.reload.reason??""}));
   return 0;
  }
  case"start":{
   const access=await probe(cwd,{platform});
   if(access.state==="denied"||access.state==="waiting"){fail(say("consent-"+access.state,{folder:access.folder,root:cwd,place:access.place??cwd,identity:consentIdentityV1({platform,environment})?.path??"repo"}));return 4;}
   try{
    const{socket}=await connectHostV1({implementation,environment,platform,spawnService:spawnHostServiceV1({command,environment,cwd}),signal,languages,...policy?{policy}:{}});
    socket.destroy();
   }catch(error){fail(say("host-unavailable",{reason:error instanceof HostUnavailableErrorV1?error.notice(languages[0]??"en"):String(error?.message??error)}));return 1;}
   const found=await status();
   print(say("host-started",{pid:found?.pid??"?",workspaces:found?.workspaces.length??0,megabytes:found?.rssMb??0}));return 0;
  }
  case"stop":{
   const found=await status(),unload=()=>stopServiceJobsV1({platform,home:environment.HOME,run:launchctl});
   if(!found){unload();print(say("host-down",{}));return 0;}
   process.kill(found.pid,"SIGTERM");
   const deadline=performance.now()+10_000;
   let remaining=deadline-performance.now();
   while(remaining>0){
    if(!await status(Math.min(statusLimit,Math.ceil(remaining))))break;
    remaining=deadline-performance.now();
    if(remaining>0)await sleep(Math.min(50,remaining));
    remaining=deadline-performance.now();
   }
   if(remaining<=0)throw new HostUnavailableErrorV1("timeout");
   unload();
   print(say("host-stopped",{}));return 0;
  }
  default:fail(say("host-usage",{}));return 1;
 }}catch(error){
  if(!(error instanceof HostUnavailableErrorV1)||error.reason!=="timeout")throw error;
  fail(say("host-status-timeout",{}));return 1;
 }
}
