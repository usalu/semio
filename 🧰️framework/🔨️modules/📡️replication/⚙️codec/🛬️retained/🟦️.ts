/** 🪪️ Source identity includes the original backing loan, exact framed origin and issuing operation. */
export interface OriginalWireSourceIdentity{address:unknown;extent:number;rangeStart:number;authority:readonly[bigint,bigint]|undefined}
/** 👓️ Outer descriptor movement preserves identity; equal bytes in another backing or frame do not. */
export function sameOriginalWireSource(left:OriginalWireSourceIdentity,right:OriginalWireSourceIdentity):boolean{return left.address===right.address&&left.extent===right.extent&&left.rangeStart===right.rangeStart&&(left.authority===undefined?right.authority===undefined:right.authority!==undefined&&left.authority[0]===right.authority[0]&&left.authority[1]===right.authority[1])}
