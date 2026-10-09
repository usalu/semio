import {sha256} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🔏️hash/🟦️.ts";
import type {DrawingIdentityKind,DrawingIdentityCommitment} from "../../../🧬️schema/🪪️identity/🟦️.ts";
export interface IdentityControl {maximumBytes:number;onProgress?:(completed:number,total:number,ownedBytes:number)=>boolean}
export const kinds:DrawingIdentityKind[]=["layer","path","group","boolean","trace","shape","text","image","svg","imageAsset"];
function progress(control:IdentityControl,completed:number,total:number,owned:number){if(control.onProgress?.(completed,total,owned)===false)throw Error("Drawing identity canceled");}
export function encodeIdentityPreimage(kind:DrawingIdentityKind,parts:Uint8Array[],control:IdentityControl):Uint8Array {
 const tag=kinds.indexOf(kind);const total=13+parts.reduce((n,part)=>n+8+part.length,0);progress(control,0,total,0);if(tag<0||parts.length>64||total>65536||!Number.isSafeInteger(control.maximumBytes)||control.maximumBytes<total)throw Error("Drawing identity input or ownership limit");
 const output=new Uint8Array(total);output.set([68,82,65,87,73,68,48,49,tag]);const view=new DataView(output.buffer);view.setUint32(9,parts.length,true);let at=13;progress(control,at,total,total);
 for(const part of parts){view.setBigUint64(at,BigInt(part.length),true);at+=8;progress(control,at,total,total);for(let offset=0;offset<part.length;offset+=256){const span=part.subarray(offset,offset+256);output.set(span,at);at+=span.length;progress(control,at,total,total);}}return output;
}
export function commitIdentity(kind:DrawingIdentityKind,parts:Uint8Array[],control:IdentityControl):DrawingIdentityCommitment {
 const preimage=encodeIdentityPreimage(kind,parts,control);if(preimage.length+32>control.maximumBytes)throw Error("Drawing identity digest ownership limit");progress(control,preimage.length,preimage.length,preimage.length+32);return {kind,digest:Array.from(sha256(preimage))};
}
