/** 🧳️ Charges stable candidate capacity, pointer handoff and one initialized field placement separately. */
export function stableRecordProjectionDemand(row:Readonly<{recordBytes:number;cursorBytes:number;pointerBytes:number;metadataBytes:number;fieldBytes:number}>){return{birthCopyBytes:2*row.cursorBytes,birthCapacityBytes:row.recordBytes,takeCopyBytes:2*row.pointerBytes+row.metadataBytes,placementCopyBytes:2*row.fieldBytes+2*row.pointerBytes,placementReleaseBytes:row.fieldBytes};}
/** 📐️ Stable field placement requires natural alignment for every declared field. */
export function recordProjectionLayoutAccepted(representations:readonly string[]):boolean{return !representations.includes("packed");}
