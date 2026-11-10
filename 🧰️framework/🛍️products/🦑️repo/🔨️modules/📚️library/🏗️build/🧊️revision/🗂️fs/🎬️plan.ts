import type {CasStoreV1} from "../../🗄️cas/🗃️store/🟦️.ts";
import {withObservedWorldV1} from "../../👁️observation/🟨️.mjs";
import type {PinnedRevisionV1} from "../📌️pin/🟦️.ts";
import type {InputManifestV1,RevisionV1} from "../🧬️schema/🟦️.ts";
import {createCapturingFsV1,type CapturedPlanV1} from "./📸️capture.ts";
import type {ReadFsV1} from "./🟦️.ts";

export type CapturedPlanOptionsV1=Readonly<{cas:CasStoreV1;root:string;workspace:string;holder:string;journalCursor:string;parent?:Readonly<{revision:RevisionV1;manifest:InputManifestV1}>;ttlMs?:number;signal?:AbortSignal;fallback?:ReadFsV1;passThrough?(relativePath:string):boolean}>;

/** 🎬️ Runs one plan with every observed read of workspace sources captured (see `createCapturingFsV1`) and answers the plan's result together with the revision it read as the plan's pin. */
export async function withCapturedPlanV1<T>(options:CapturedPlanOptionsV1,body:()=>Promise<T>):Promise<CapturedPlanV1<T>>{
 const capture=createCapturingFsV1({cas:options.cas,root:options.root,...options.fallback===undefined?{}:{fallback:options.fallback},...options.passThrough===undefined?{}:{passThrough:options.passThrough}});
 const result=await withObservedWorldV1(capture.fs,body);
 const pinned=await capture.seal({workspace:options.workspace,holder:options.holder,journalCursor:options.journalCursor,...options.parent===undefined?{}:{parent:options.parent},...options.ttlMs===undefined?{}:{ttlMs:options.ttlMs},...options.signal===undefined?{}:{signal:options.signal}});
 const stats=capture.stats();
 return Object.freeze({result,pinned,probes:stats.probes,unstaged:stats.unstaged});
}
