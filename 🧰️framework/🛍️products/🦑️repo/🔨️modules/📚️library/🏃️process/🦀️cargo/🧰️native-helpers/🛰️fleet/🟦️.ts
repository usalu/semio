import {createHash} from "node:crypto";
import {chmodSync,existsSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {fleetFetchContextV1} from "../../../../⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🟦️.ts";
import {fleetSetupOutputV1,type SetupOutcomeV1} from "../../../../⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🧰️setup/🟦️.ts";
import {captureSchedulerReceiptV1,schedulerIdentityV1,schedulerInputsV1,schedulerReceiptFileV1,writeSchedulerReceiptV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🔖️scheduler/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The authored task name and executables of the shared scheduler record.
 * @see ./🧬️schema/🔣️.json */
export const SCHEDULER_SHARE_POLICY_V1=schema.definitions.Policy.const;

/** 🛰️ Obtains the scheduler executables of one machine through the fleet: when a machine of the same platform class already built them from the same sources, compiler identity and profile they are restored (digest and signature verified) and a local receipt is written; otherwise exactly one machine builds while the others wait on the fleet claim, and the finished executables are published. A broken or absent fleet ends in the plain local `build`.
 * @see ../🟦️.ts
 * @see ../../../../⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🧰️setup/🟦️.ts */
export async function shareSchedulerBuildV1(request:Readonly<{workspace:string;targetDirectory:string;toolchain:string;environment:Readonly<Record<string,string|undefined>>;mode:string;manifest:string;build:()=>Promise<ReadonlyMap<string,string>>;context?:ReturnType<typeof fleetFetchContextV1>;say?:(line:string)=>void}>):Promise<ReadonlyMap<string,string>>{
 const policy=SCHEDULER_SHARE_POLICY_V1,suffix=process.platform==="win32"?".exe":"",profile=request.mode==="release"?"release":request.mode,identity=schedulerIdentityV1(request.toolchain,request.environment);
 const roots=policy.binaries.map(name=>`${profile}/${name}${suffix}`),paths=new Map(policy.binaries.map(name=>[name,join(request.targetDirectory,profile,name+suffix)] as const));
 const inputs:Record<string,string>={identity,profile,...Object.fromEntries(schedulerInputsV1(request.manifest).map((path,index)=>["source"+index,createHash("sha256").update(readFileSync(path)).digest("hex")]))};
 let built:ReadonlyMap<string,string>|undefined;
 const outcome:SetupOutcomeV1=await fleetSetupOutputV1(request.context??fleetFetchContextV1(request.environment,request.workspace),{task:policy.task,inputs,kind:"host",base:request.targetDirectory,roots},async()=>{built=await request.build();},{
  environment:request.environment,...request.say===undefined?{}:{say:request.say},
  verify:async()=>{
   for(const path of paths.values())if(!existsSync(path))return false;
   if(process.platform!=="win32")for(const path of paths.values())chmodSync(path,0o755);
   const receipt=captureSchedulerReceiptV1(identity,request.manifest,paths);
   if(receipt===undefined)return false;
   writeSchedulerReceiptV1(join(request.targetDirectory,schedulerReceiptFileV1()),receipt);
   return true;
  },
 });
 return built??(outcome.source==="restored"?paths:await request.build());
}
