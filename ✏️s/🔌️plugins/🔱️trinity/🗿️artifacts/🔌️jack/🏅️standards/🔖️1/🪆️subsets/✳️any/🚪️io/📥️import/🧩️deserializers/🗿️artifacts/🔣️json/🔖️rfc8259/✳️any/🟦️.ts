/** 📥️ The declared Jack JSON word boundary constructs its actual canonical parent. */
import {parseJackArtifact,type JackArtifact} from "../../../../../../../🧬️schema/🟦️.ts";
import {parseJackDiff,type JackDiff} from "../../../../../../../🧬️schema/🔺️diff/🟦️.ts";
function scalar(value:unknown):{bits:bigint}{if(!value||typeof value!=="object"||Array.isArray(value)||Object.keys(value).length!==1||!("bits"in value)||typeof value.bits!=="string"||!/^[0-9a-f]{16}$/.test(value.bits))throw Error("Jack JSON camera requires a closed binary64 hex word");return{bits:BigInt("0x"+value.bits)};}
function camera(value:unknown):{x:{bits:bigint};y:{bits:bigint};zoom:{bits:bigint}}{if(!value||typeof value!=="object"||Array.isArray(value)||Object.keys(value).some(key=>key!=="x"&&key!=="y"&&key!=="zoom"))throw Error("Jack JSON camera fields differ");const fields=value as Record<string,unknown>;return{x:scalar(fields.x),y:scalar(fields.y),zoom:scalar(fields.zoom)};}
/** 📖️ Read the literal full manifest and independent identity without numeric conversion. */
export function parseJackJsonValue(value:unknown):JackArtifact{if(!value||typeof value!=="object"||Array.isArray(value))throw Error("Jack JSON document required");const input=value as Record<string,unknown>;return parseJackArtifact({...input,camera:camera(input.camera)});}
/** 🔺️ Read a sparse Jack diff whose only closed binary64 words are the replaced camera's. */
export function parseJackDiffJsonValue(value:unknown):JackDiff{if(!value||typeof value!=="object"||Array.isArray(value))throw Error("Jack JSON diff required");const input=value as Record<string,unknown>;return parseJackDiff({...input,...(input.camera==null?{}:{camera:camera(input.camera)})});}
