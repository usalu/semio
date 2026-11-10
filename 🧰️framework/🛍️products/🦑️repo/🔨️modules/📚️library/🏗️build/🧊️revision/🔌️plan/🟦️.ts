import type {CasStoreV1} from "../../🗄️cas/🗃️store/🟦️.ts";
import {loadManifestV1,pinRevisionV1} from "../📌️pin/🟦️.ts";
import type {DigestV1,PinV1} from "../🧬️schema/🟦️.ts";
import {withCapturedPlanV1} from "../🗂️fs/🎬️plan.ts";
import type {CapturedPlanV1} from "../🗂️fs/📸️capture.ts";

/** 🔌️ The pin hook of the resident planner port: pins a revision of the plan's input closure, with the previous revision as parent and the journal dirty set, and answers what the plan reply carries. A parent that left the store is ignored (full restage). */
export type RevisionPinRequestV1=Readonly<{workspace?:string;root?:string;agent?:string;targets:readonly string[];order:readonly string[];parent?:string;dirty?:{paths:readonly string[];overflow?:boolean;cursor?:string};cursor?:string;signal?:AbortSignal}>;
export type RevisionPinAnswerV1=Readonly<{revision:DigestV1;manifest:DigestV1;pin:PinV1}>;

/** 🔌️ Creates the hook. `inputsOf` lists the workspace-relative input paths of the planned tasks (from the scoped plan's input closure). */
export function createRevisionPinHookV1(options:{cas:CasStoreV1;inputsOf(request:RevisionPinRequestV1):Promise<readonly string[]>;progress?:Parameters<typeof pinRevisionV1>[0]["progress"]}):(request:RevisionPinRequestV1)=>Promise<RevisionPinAnswerV1>{
 return async request=>{
  const parent=request.parent===undefined?undefined:await loadManifestV1(options.cas,request.parent,request.signal).then(loaded=>({revision:loaded.revision,manifest:loaded.manifest}),()=>undefined);
  const pinned=await pinRevisionV1({
   cas:options.cas,root:request.root!,workspace:request.workspace!,paths:await options.inputsOf(request),holder:request.agent??"agent",journalCursor:request.cursor??request.dirty?.cursor??"0",
   ...parent===undefined?{}:{parent,...request.dirty===undefined?{dirty:{paths:[],overflow:true}}:{dirty:request.dirty}},
   ...request.signal===undefined?{}:{signal:request.signal},...options.progress===undefined?{}:{progress:options.progress},
  });
  return{revision:pinned.revision.id,manifest:pinned.manifest.digest,pin:pinned.pin};
 };
}

/** 🔌️ Creates the capture hook of the resident planner port: runs one plan with its workspace reads captured and answers the result with the revision the plan read as its pin (`parent` is the previous revision id; one that left the store is ignored). */
export function createRevisionCaptureHookV1(options:{cas:CasStoreV1;progress?:never}):<T>(request:RevisionPinRequestV1,body:()=>Promise<T>)=>Promise<CapturedPlanV1<T>>{
 return async(request,body)=>{
  const parent=request.parent===undefined?undefined:await loadManifestV1(options.cas,request.parent,request.signal).then(loaded=>({revision:loaded.revision,manifest:loaded.manifest}),()=>undefined);
  return withCapturedPlanV1({cas:options.cas,root:request.root!,workspace:request.workspace!,holder:request.agent??"agent",journalCursor:request.cursor??request.dirty?.cursor??"0",...parent===undefined?{}:{parent},...request.signal===undefined?{}:{signal:request.signal}},body);
 };
}
