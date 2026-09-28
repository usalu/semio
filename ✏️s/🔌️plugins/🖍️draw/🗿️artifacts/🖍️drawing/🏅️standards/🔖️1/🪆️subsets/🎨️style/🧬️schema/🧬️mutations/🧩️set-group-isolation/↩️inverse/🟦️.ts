/** ↩️ Restore a known group's authored isolation value. */
import type {SetGroupIsolation} from "../🦠️mutation/🟦️.ts";
export function inverse(payload:SetGroupIsolation,isolation:boolean|undefined):SetGroupIsolation[]{return isolation===undefined?[]:[{layerId:payload.layerId,isolation}];}
