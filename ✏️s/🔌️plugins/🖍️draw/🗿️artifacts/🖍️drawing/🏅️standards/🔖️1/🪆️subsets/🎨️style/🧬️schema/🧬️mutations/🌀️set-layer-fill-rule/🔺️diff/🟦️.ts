/** 🔺️ One sparse fill-rule field change. */
import type {SetLayerFillRule} from "../🦠️mutation/🟦️.ts";
export function diff(payload:SetLayerFillRule){return {layers:{patched:[{id:payload.layerId,patch:{fillRule:payload.fillRule}}]}};}
