import {declaredStoreReceiptV1} from "../../../🧾️store-receipt/🟨️.mjs";
import {fleetContentFromEnvironmentV1,publishStoreReceiptContentV1} from "../../../🧾️store-receipt/☁️fleet-content/🟦️.ts";
import {nxPlatformV1,nxTaskSlotV1} from "../../🪢️nx-slots/🟦️.ts";
import {remoteGuardV1} from "../../../🧮️task-model/🛡️remote-guard/🟦️.ts";
import {announceNxTaskFinishV1} from "../🚦️single-flight/🟦️.ts";

/** 🧩️ The parts of Nx's executor context the end of a task needs. */
export type NxTaskContextV1=Readonly<{root:string;projectName:string;targetName:string;configurationName?:string|undefined;projectsConfigurations?:{projects?:Record<string,{root?:string;namedInputs?:Record<string,readonly unknown[]>;targets?:Record<string,{inputs?:readonly unknown[];outputs?:readonly string[];metadata?:{semio?:{execution?:{kind?:string}}}}>}>}|undefined;nxJsonConfiguration?:{namedInputs?:Record<string,readonly unknown[]>}|undefined}>;

/** 🏁 Ends a task for the fleet: a successful task that declared a store receipt first uploads the content the receipt names (so the payload is in the fleet store before Nx uploads the receipt), then the single-flight front learns that the task ended, with the slot of its result, and releases its claim. Never throws and never waits beyond the bounded timeouts of its steps: a fleet that cannot be reached costs sharing, not the task. */
export async function finishNxTaskV1(context:NxTaskContextV1,succeeded:boolean,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<void>{
 try{
  const project=context.projectsConfigurations?.projects?.[context.projectName],target=project?.targets?.[context.targetName];
  const guarded=remoteGuardV1(context.targetName)!==undefined;
  if(succeeded&&!guarded){
   const receipt=declaredStoreReceiptV1({workspaceRoot:context.root,projectRoot:project?.root??".",target:context.targetName,outputs:target?.outputs}),store=fleetContentFromEnvironmentV1(environment);
   if(receipt!==undefined&&store!==undefined)await publishStoreReceiptContentV1({workspaceRoot:context.root,receipt,store});
  }
  const slot=nxTaskSlotV1({project:context.projectName,target:context.targetName,configuration:context.configurationName??null},nxPlatformV1());
  await announceNxTaskFinishV1(environment,{hash:environment.NX_TASK_HASH,project:context.projectName,target:context.targetName,configuration:context.configurationName,slot,guarded,succeeded});
 }catch{}
}
