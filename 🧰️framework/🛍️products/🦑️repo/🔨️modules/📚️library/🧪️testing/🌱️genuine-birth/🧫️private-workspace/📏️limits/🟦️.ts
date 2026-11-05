import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
/** 💰️ Bounded data for the existing native child grant and complete separated capture. */
export interface PrivateNativeFundingV1{readonly maximumMilliseconds:number;readonly maximumBytes:number;readonly maximumLines:number}
/** 📐️ Refines declared child funding and residual time without creating process or resource authority. */
export function readPrivateNativeFundingV1(value:unknown):PrivateNativeFundingV1{
 if(validateJsonSchemaSubset(schema.definitions.Funding,value).length)throw Error("Closed private Native funding data required");const row=value as {remainingMilliseconds:number|null;maximumChildMilliseconds:number;maximumBytes:number;maximumLines:number},maximumMilliseconds=Math.floor(Math.min(row.remainingMilliseconds??row.maximumChildMilliseconds,row.maximumChildMilliseconds));if(!Number.isSafeInteger(maximumMilliseconds)||maximumMilliseconds<1)throw Error("Positive finite private Native child funding required");
 return Object.freeze({maximumMilliseconds,maximumBytes:Math.min(row.maximumBytes,schema.definitions.Protocol.const.maximumCaptureBytes),maximumLines:Math.min(row.maximumLines,schema.definitions.Protocol.const.maximumCaptureLines)});
}
