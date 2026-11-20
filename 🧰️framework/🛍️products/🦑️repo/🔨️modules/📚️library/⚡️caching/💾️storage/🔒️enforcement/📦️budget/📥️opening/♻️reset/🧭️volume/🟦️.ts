import {validateJsonSchemaSubset} from "../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 📏️ Names the measured device and requested free bytes for one automatic pressure recovery. */
export type ImageRecoveryVolumeV1=Readonly<{version:1;device:number;requiredFreeBytes:number}>;
export const IMAGE_RECOVERY_VOLUME_ENV_V1=schema.$defs.ImageRecoveryEnvironment.const;
/** 🧾️ Reads an automatic image request without carrying a path or capability from its launcher. */
export function readImageRecoveryVolumeV1(text:string|undefined):ImageRecoveryVolumeV1|undefined{
 if(text===undefined)return undefined;
 if(text.length>160)throw Error("Bounded image recovery volume required");
 const value:unknown=JSON.parse(text);
 if(validateJsonSchemaSubset({$defs:schema.$defs,$ref:"#/$defs/ImageRecoveryVolume"},value).length)throw Error("Closed image recovery volume required");
 return Object.freeze(value as ImageRecoveryVolumeV1);
}
/** 💽️ Reads only the exact pressured device and bytes; a marker or claimed pressure grants no authority. */
export function meetsImageRecoveryPressureV1(observed:Readonly<{device:number;availableBytes:number;totalBytes:number}>,request:ImageRecoveryVolumeV1):boolean{
 return observed.device===request.device&&observed.availableBytes<request.requiredFreeBytes&&request.requiredFreeBytes<=observed.totalBytes;
}
