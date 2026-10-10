/** 📦️ Original initialized operation octets. */
export type ArtifactPreparedOperationOutput = number[];

/** 🛂️ Admits the original dense octet array without copying its backing. */
export function parseArtifactPreparedOperationOutput(value: unknown): ArtifactPreparedOperationOutput {
  if(!Array.isArray(value))throw new TypeError("Operation output must be an octet array");
  for(let index=0;index<value.length;index++)if(!Object.hasOwn(value,index)||!Number.isInteger(value[index])||value[index]<0||value[index]>255)throw new TypeError("Operation output contains an invalid octet");
  return value;
}

/** 🧺️ Original protocol operation octets retain their source order. */
export type ArtifactPreparedOperations = ArtifactPreparedOperationOutput[];

/** 🛂️ Admits original dense operation rows without cloning any row or array. */
export function parseArtifactPreparedOperations(value: unknown): ArtifactPreparedOperations {
  if(!Array.isArray(value))throw new TypeError("Prepared operations must be an array");
  for(let index=0;index<value.length;index++)if(!Object.hasOwn(value,index))throw new TypeError("Prepared operations contain a missing row");else parseArtifactPreparedOperationOutput(value[index]);
  return value;
}
