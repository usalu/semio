/** ↩️ Restore the exact typed owner present before replacement. */
import type{EditBeforeFixture}from"../🟦️.ts";
import type{RewritingArtifact}from"../../../🟦️.ts";
export function inverse(base:RewritingArtifact):EditBeforeFixture{return{newWorkingGraph:base.workingGraph};}
