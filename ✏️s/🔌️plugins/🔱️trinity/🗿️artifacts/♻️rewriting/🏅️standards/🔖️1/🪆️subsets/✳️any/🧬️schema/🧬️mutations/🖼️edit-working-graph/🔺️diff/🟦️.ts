/** 🔺️ Exact typed replacement preserves all owner fields. */
import type{EditWorkingGraph}from"../🟦️.ts";
export function diff(payload:EditWorkingGraph){return{workingGraph:payload.newWorkingGraph};}
