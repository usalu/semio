/** 📥️ Private encoded PNG admission completes before a semantic image import can publish. */
import {DrawingImageAdmissionJob} from "../../../🚪️io/🖼️image/🟦️.ts";
import type {DrawingImageAsset} from "../../../🧬️schema/🟦️.ts";
export interface ImportImage {payload:string;name?:string|null;parentId?:string|null;index?:number|null}
export const IMAGE_IMPORT_SOURCE_BYTES=89478512;
export function validateImageImport(input:ImportImage):void {if(!input.payload.startsWith("data:image/png;base64,")||input.payload.length<=22||input.payload.length>IMAGE_IMPORT_SOURCE_BYTES||input.name!==null&&input.name!==undefined&&new TextEncoder().encode(input.name).length>1024)throw RangeError("Choose a PNG image within the image import limits");if(input.parentId!==undefined&&input.parentId!==null&&(input.parentId.length===0||new TextEncoder().encode(input.parentId).length>1024)||input.index!==undefined&&input.index!==null&&(!Number.isSafeInteger(input.index)||input.index<0))throw RangeError("Invalid image import destination");}
export class DrawingImageImportJob {
 private job:DrawingImageAdmissionJob;private complete=false;private cancelled=false;
 constructor(input:ImportImage) {validateImageImport(input);this.job=new DrawingImageAdmissionJob({mime:"image/png",data:input.payload,maxSourceBytes:IMAGE_IMPORT_SOURCE_BYTES,maxBytes:67108864,maxPixels:16777216,maxChunks:65536});}
 advance(grant:number):ReturnType<DrawingImageAdmissionJob["advance"]> {if(this.cancelled)throw new DOMException("Image import cancelled","AbortError");const state=this.job.advance(grant);this.complete=state.done;return state;}
 result():DrawingImageAsset {if(this.cancelled||!this.complete)throw Error("Image import is incomplete");return this.job.result();}
 cancel():void {this.cancelled=true;this.job.cancel();}
}
