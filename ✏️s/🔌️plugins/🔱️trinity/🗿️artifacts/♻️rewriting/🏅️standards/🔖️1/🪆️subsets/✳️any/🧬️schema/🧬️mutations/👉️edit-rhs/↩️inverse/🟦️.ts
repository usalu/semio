/** ↩️ Restore the exact typed owner present before replacement. */
import type{EditRhs}from"../🟦️.ts";
import type{RewritingArtifact}from"../../../🟦️.ts";
export function inverse(base:RewritingArtifact):EditRhs{return{newRhs:base.rhs};}
