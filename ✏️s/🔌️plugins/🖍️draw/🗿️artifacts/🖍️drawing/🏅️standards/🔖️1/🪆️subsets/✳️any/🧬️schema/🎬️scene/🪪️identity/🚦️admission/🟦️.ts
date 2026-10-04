/** 🚦️ A captured query observes one exact live geometry publication. */
import {sceneIdentityMatches,type SceneIdentity} from "../🟦️.ts";
export type SceneAdmissionStatus="ready"|"pending"|"stale"|"failed"|"unavailable";
export interface SceneAdmission{readonly captured:SceneIdentity;readonly live:SceneIdentity|null;readonly visual:SceneIdentity|null;readonly preparing:boolean;readonly failed:boolean;}
/** 🔐️ An old picture can remain visible without granting authority to query it. */
export function sceneAdmission(input:SceneAdmission):SceneAdmissionStatus{
 if(typeof input.preparing!=="boolean"||typeof input.failed!=="boolean"||!sceneIdentityMatches(input.captured,input.captured)||input.live!==null&&!sceneIdentityMatches(input.live,input.live)||input.visual!==null&&!sceneIdentityMatches(input.visual,input.visual))return"unavailable";
 if(input.live===null||!sceneIdentityMatches(input.captured,input.live))return"stale";
 if(input.visual!==null&&sceneIdentityMatches(input.captured,input.visual))return"ready";
 return input.failed?"failed":input.preparing?"pending":"unavailable";
}
