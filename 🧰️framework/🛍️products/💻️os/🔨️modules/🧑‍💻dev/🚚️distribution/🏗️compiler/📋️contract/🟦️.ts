import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
/** 🧾️ Carries a bounded expected publication identity without issuing reader custody. */
export interface DistributionCompileReceiptV1{readonly version:1;readonly mode:"prototype"|"release";readonly owner:"os:distribution-witness";readonly manifestSha256:string}
/** 📥️ Refuses unknown or oversized handback DATA before selecting any published product. */
export function readDistributionCompileReceiptV1(text:string,mode:"prototype"|"release"):DistributionCompileReceiptV1{
 if(typeof text!=="string"||Buffer.byteLength(text)>schema.definitions.Protocol.const.maximumReceiptBytes)throw Error("Finite distribution compile receipt required");const value:unknown=JSON.parse(text);if(validateJsonSchemaSubset(schema,value).length||(value as DistributionCompileReceiptV1).mode!==mode)throw Error("Exact distribution compile receipt required");return Object.freeze(value as DistributionCompileReceiptV1);
}
