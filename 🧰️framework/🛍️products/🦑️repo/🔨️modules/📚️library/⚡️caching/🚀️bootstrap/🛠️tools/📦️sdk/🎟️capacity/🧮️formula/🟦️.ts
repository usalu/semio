import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema.json"with{type:"json"};
import policy from"../📋️policy.json"with{type:"json"};
import snapshotPolicy from"../../../🔏️authorizers/📦️snapshot/🔣️policy.json"with{type:"json"};
import derivativeSchema from"../../📥️source/🔗️derivative/🧬️schema/🔣️.json"with{type:"json"};
function headroom(value:unknown,publicationBytes:number):number{if(validateJsonSchemaSubset(schema.definitions.Material,value,schema).length)throw Error("Closed actual SDK clone material required");const row=value as{entries:number;copyAllocatedBytes:number},bytes=row.copyAllocatedBytes+(row.entries+policy.publicationEntries)*policy.metadataBytesPerEntry+publicationBytes;if(!Number.isSafeInteger(bytes)||bytes>policy.maximumHeadroomBytes)throw Error("SDK clone headroom capacity exceeded");return bytes;}
/** 🧮️ Charges copied file extents, namespace metadata and finite compiler publication material. */
export function nxSdkCloneHeadroomBytesV1(value:unknown):number{
 return headroom(value,policy.maximumPublicationBytes);
}
/** 🔗️ Funds the exact bounded derivative writers, including index sealing and patch replacement. */
export function nxSdkDerivativeHeadroomBytesV1(value:unknown):number{if(validateJsonSchemaSubset(schema.definitions.DerivativeMaterial,value,schema).length)throw Error("Closed actual SDK derivative material required");const row=value as{version:1;entries:number;copyAllocatedBytes:number;inputBytes:number};const publicationBytes=policy.derivativeSnapshotCopies*snapshotPolicy.maximumSnapshotBytes+policy.derivativeProgramCopies*snapshotPolicy.maximumOutputBytes+row.inputBytes+derivativeSchema.definitions.Policy.const.maximumIndexBytes+policy.maximumDerivativePatchBytes+policy.maximumDerivativeReceiptBytes;if(!Number.isSafeInteger(publicationBytes)||publicationBytes>policy.maximumPublicationBytes)throw Error("SDK derivative publication byte ceiling exceeded");return headroom({version:row.version,entries:row.entries,copyAllocatedBytes:row.copyAllocatedBytes},publicationBytes);}
