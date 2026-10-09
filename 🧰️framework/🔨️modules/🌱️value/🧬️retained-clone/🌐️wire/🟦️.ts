/** 🎟️ The schema-first original physical authority has all five independent axes. */
export type RetainedCloneGrant = { readonly maximumItems:number; readonly maximumCopyBytes:number; readonly maximumCapacityBytes:number; readonly maximumReleaseBytes:number; readonly maximumDepth:number };
/** 🪪️ Admits a complete canonical authority without supplying missing fields. */
export function parseRetainedCloneGrant(original:unknown):RetainedCloneGrant{
 if(typeof original!=="object"||original===null||Array.isArray(original))throw Error("canonical retained grant requires an object");
 const fields=["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"] as const;
 if(Object.keys(original).length!==fields.length)throw Error("canonical retained grant requires exactly five axes");
 const value=original as Record<string,unknown>;
 for(const field of fields)if(!Number.isSafeInteger(value[field])||(value[field] as number)<0)throw Error("canonical retained grant requires nonnegative integer axes");
 return original as RetainedCloneGrant;
}
