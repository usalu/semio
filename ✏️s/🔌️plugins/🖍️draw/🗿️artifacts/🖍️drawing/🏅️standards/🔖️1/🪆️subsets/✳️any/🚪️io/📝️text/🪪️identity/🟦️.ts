import type {DrawingIdentityCommitment} from "../../../🧬️schema/🪪️identity/🟦️.ts";
import {kinds,type IdentityControl} from "../../💾️binary/🪪️identity/🟦️.ts";
export function encodeIdentityText(commitment:DrawingIdentityCommitment,control:IdentityControl):string {
 const {kind,digest}=commitment;const prefix=kind==="imageAsset"?"image-asset":kind;const size=prefix.length+65;if(!kinds.includes(kind)||digest.length!==32||digest.some(byte=>!Number.isInteger(byte)||byte<0||byte>255))throw Error("Drawing identity commitment invalid");if(control.onProgress?.(0,32,0)===false)throw Error("Drawing identity canceled");if(!Number.isSafeInteger(control.maximumBytes)||control.maximumBytes<size)throw Error("Drawing identity spelling ownership limit");let output=prefix+"-";for(let i=0;i<32;i++){output+=digest[i]!.toString(16).padStart(2,"0");if(control.onProgress?.(i+1,32,size)===false)throw Error("Drawing identity canceled");}return output;
}
