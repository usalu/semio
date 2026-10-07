/** 🔺️ Atomic owned Bmp image diff algebra. */
import {parseBmpImage,parseBmpSnapshot,type BmpImage,type BmpSnapshot} from "../📸️snapshot/🟦️.ts";
export interface BmpDiff { image?:BmpImage; }
const equal=(a:unknown,b:unknown):boolean=>{if(a===b)return true;if(a===null||b===null||typeof a!=="object"||typeof b!=="object")return false;const left=Object.keys(a),right=Object.keys(b);return left.length===right.length&&left.every(key=>Object.hasOwn(b,key)&&equal((a as Record<string,unknown>)[key],(b as Record<string,unknown>)[key]));};
export function parseBmpDiff(value:unknown,at="$"):BmpDiff{if(value===null||typeof value!=="object"||Array.isArray(value)||Object.keys(value).some(key=>key!=="image"))throw new TypeError(`${at}: invalid image diff`);const image=(value as {image?:unknown}).image;return image===undefined?{}:{image:parseBmpImage(image,`${at}.image`)};}
export function applyBmpDiff(base:BmpSnapshot,diff:BmpDiff):BmpSnapshot{return parseBmpSnapshot({schema:base.schema,image:diff.image??base.image});}
export function inverseBmpDiff(base:BmpSnapshot,diff:BmpDiff):BmpDiff{return diff.image===undefined?{}:{image:parseBmpImage(base.image)};}
export function betweenBmpSnapshots(base:BmpSnapshot,next:BmpSnapshot):BmpDiff{return equal(base.image,next.image)?{}:{image:parseBmpImage(next.image)};}
export function absorbBmpDiff(base:BmpDiff,next:BmpDiff):BmpDiff{return parseBmpDiff(next.image===undefined?base:next);}
