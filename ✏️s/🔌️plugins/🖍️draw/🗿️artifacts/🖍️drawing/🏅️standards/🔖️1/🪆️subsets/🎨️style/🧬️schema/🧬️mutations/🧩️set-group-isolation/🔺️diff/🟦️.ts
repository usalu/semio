/** 🔺️ One sparse group isolation field change. */
import type {SetGroupIsolation} from "../🦠️mutation/🟦️.ts";
export function diff(payload:SetGroupIsolation){return {layers:{modified:[{id:payload.layerId,patch:{isolation:payload.isolation}}]}};}
