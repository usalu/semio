/** ↩️ Restore the exact typed owner present before replacement. */
import type{EditWorkingGraph}from"../🟦️.ts";
import type{RewritingArtifact}from"../../../🟦️.ts";
export function inverse(base:RewritingArtifact):EditWorkingGraph{return{newWorkingGraph:base.workingGraph};}
