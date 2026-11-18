import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{refuseBuildV1}from"../../../../../../🏗️build/📖️texts/🟦️.ts";
import schema from"../🧬️schema.json"with{type:"json"};
import policy from"../📋️policy.json"with{type:"json"};
import snapshotPolicy from"../../../🔏️authorizers/📦️snapshot/🔣️policy.json"with{type:"json"};
import derivativeSchema from"../../📥️source/🔗️derivative/🧬️schema/🔣️.json"with{type:"json"};
const clonePublicationBytes=2*snapshotPolicy.maximumSnapshotBytes+2*snapshotPolicy.maximumOutputBytes+policy.maximumInputBytes;
function derivativePublicationBytes(inputBytes:number):number{return policy.derivativeSnapshotCopies*snapshotPolicy.maximumSnapshotBytes+policy.derivativeProgramCopies*snapshotPolicy.maximumOutputBytes+inputBytes+derivativeSchema.definitions.Policy.const.maximumIndexBytes+policy.maximumDerivativePatchBytes+policy.maximumDerivativeReceiptBytes;}
function refusal(derivative:string,need:number,ceiling:number,free:number|undefined){return refuseBuildV1("refused","sdk.derivative-capacity",{sdk:"nx",derivative,need:String(need),ceiling:String(ceiling),free:free===undefined?"?":String(free)});}
function headroom(derivative:string,value:unknown,publicationBytes:number,free:number|undefined):number{if(validateJsonSchemaSubset(schema.definitions.Material,value,schema).length)throw Error("Closed actual SDK clone material required");const row=value as{entries:number;copyAllocatedBytes:number},bytes=row.copyAllocatedBytes+(row.entries+policy.publicationEntries)*policy.metadataBytesPerEntry+publicationBytes;if(!Number.isSafeInteger(bytes)||bytes>policy.maximumHeadroomBytes)throw refusal(derivative,bytes,policy.maximumHeadroomBytes,free);return bytes;}
/** 🧮️ Charges copied file extents, namespace metadata and the declared artefact sizes of the compiler publication; a need above the declared headroom is the typed refusal `sdk.derivative-capacity` with the free bytes of the tool store. */
export function nxSdkCloneHeadroomBytesV1(value:unknown,freeBytes?:number):number{
 return headroom("clone",value,clonePublicationBytes,freeBytes);
}
/** 🔗️ Funds the exact bounded derivative writers, including index sealing and patch replacement; the publication ceiling is derived from the same declared artefact sizes, so no artefact policy can outgrow it. */
export function nxSdkDerivativeHeadroomBytesV1(value:unknown,freeBytes?:number):number{if(validateJsonSchemaSubset(schema.definitions.DerivativeMaterial,value,schema).length)throw Error("Closed actual SDK derivative material required");const row=value as{version:1;entries:number;copyAllocatedBytes:number;inputBytes:number},publicationBytes=derivativePublicationBytes(row.inputBytes),ceiling=derivativePublicationBytes(policy.maximumInputBytes);if(!Number.isSafeInteger(publicationBytes)||publicationBytes>ceiling)throw refusal("derivative",publicationBytes,ceiling,freeBytes);return headroom("derivative",{version:row.version,entries:row.entries,copyAllocatedBytes:row.copyAllocatedBytes},publicationBytes,freeBytes);}
