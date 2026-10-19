/** ❄️ Ordered wire-value reference arithmetic preserves every intermediate refusal. */
export function applyOrderedDiffColdReference<P,D>(base:P,edits:readonly D[],apply:(diff:D,base:P)=>P):P{
 return edits.reduce((post,diff)=>apply(diff,post),base);
}
