/** ↩️ Reconstruct the rule from the base layer. */
import type {SetLayerFillRule} from "../🦠️mutation/🟦️.ts";
export function inverse(payload:SetLayerFillRule,baseRule:SetLayerFillRule["fillRule"]|undefined):SetLayerFillRule[]{return baseRule===undefined?[]:[{layerId:payload.layerId,fillRule:baseRule}];}
