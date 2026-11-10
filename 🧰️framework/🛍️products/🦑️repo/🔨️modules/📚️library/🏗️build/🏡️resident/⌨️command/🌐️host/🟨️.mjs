/** ⌨ `repo service host …`: start, status, stop and foreground run of the per-host graph service. */
import {HostUnavailableErrorV1,connectHostV1,requestHostStatusV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {spawnHostServiceV1} from "../../🏗️composition/🌐️host/🟨️.mjs";
import {renderTextV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const sleep=milliseconds=>new Promise(resolve=>setTimeout(resolve,milliseconds));

/** ⌨ Runs one host subcommand and returns the process exit code. `runForeground()` runs the service in this process; `command` is the argument vector that does so in a detached child. */
export async function runHostCommandV1({args,languages,implementation,runForeground,command,environment=process.env,platform=process.platform,cwd=process.cwd(),stdout=process.stdout,stderr=process.stderr,signal,policy}){
 const say=(key,fields)=>{const text=renderTextV1(key,fields,languages);return text?text[languages[0]]??Object.values(text)[0]:key;};
 const print=text=>stdout.write(text+"\n"),fail=text=>stderr.write(text+"\n"),sub=args[0];
 const status=()=>requestHostStatusV1({implementation,environment,platform});
 switch(sub){
  case"run":return runForeground();
  case"status":{
   const found=await status();
   if(!found){print(say("host-down",{}));return 1;}
   if(args.includes("--output=json")){print(JSON.stringify(found,null,2));return 0;}
   print(say("host-started",{pid:found.pid,workspaces:found.workspaces.length,megabytes:found.rssMb}));
   if(found.memory)print(say("host-memory",{rss:found.memory.rssMb,hwm:found.memory.hwmMb??"?",heap:found.memory.heapMb,objects:found.memory.objectCount,caches:found.memory.caches.length}));
   if(found.plans)print(say("host-plans",{total:found.plans.total,hits:found.plans.hits,computed:found.plans.computed,seconds:Math.round(found.plans.lastMilliseconds/1000),budget:found.plans.budgetRefusals,reinferred:found.plans.load?.ownersReinferred??"?",cutoff:found.plans.load?.ownersCutOff??"?",files:found.plans.captureFiles,probes:found.plans.captureProbes}));
   return 0;
  }
  case"start":{
   try{
    const{socket}=await connectHostV1({implementation,environment,platform,spawnService:spawnHostServiceV1({command,environment,cwd}),signal,languages,...policy?{policy}:{}});
    socket.destroy();
   }catch(error){fail(say("host-unavailable",{reason:error instanceof HostUnavailableErrorV1?error.reason:String(error?.message??error)}));return 1;}
   const found=await status();
   print(say("host-started",{pid:found?.pid??"?",workspaces:found?.workspaces.length??0,megabytes:found?.rssMb??0}));return 0;
  }
  case"stop":{
   const found=await status();
   if(!found){print(say("host-down",{}));return 0;}
   process.kill(found.pid,"SIGTERM");
   for(let at=0;at<200&&await status();at++)await sleep(50);
   print(say("host-stopped",{}));return 0;
  }
  default:fail(say("host-usage",{}));return 1;
 }
}
