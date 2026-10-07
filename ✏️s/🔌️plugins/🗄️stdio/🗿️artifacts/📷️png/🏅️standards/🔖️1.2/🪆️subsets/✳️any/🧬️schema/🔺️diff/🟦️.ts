/** 🔺️ Atomic owned Png image diff algebra. */
import {parsePngImage,parsePngSnapshot,type PngImage,type PngSnapshot} from "../📸️snapshot/🟦️.ts";
export interface PngDiff { image?:PngImage; }
const equal=(a:unknown,b:unknown):boolean=>{if(a===b)return true;if(a===null||b===null||typeof a!=="object"||typeof b!=="object")return false;const left=Object.keys(a),right=Object.keys(b);return left.length===right.length&&left.every(key=>Object.hasOwn(b,key)&&equal((a as Record<string,unknown>)[key],(b as Record<string,unknown>)[key]));};
export function parsePngDiff(value:unknown,at="$"):PngDiff{if(value===null||typeof value!=="object"||Array.isArray(value)||Object.keys(value).some(key=>key!=="image"))throw new TypeError(`${at}: invalid image diff`);const image=(value as {image?:unknown}).image;return image===undefined?{}:{image:parsePngImage(image,`${at}.image`)};}
export function applyPngDiff(base:PngSnapshot,diff:PngDiff):PngSnapshot{return parsePngSnapshot({schema:base.schema,image:diff.image??base.image});}
export function inversePngDiff(base:PngSnapshot,diff:PngDiff):PngDiff{return diff.image===undefined?{}:{image:parsePngImage(base.image)};}
export function betweenPngSnapshots(base:PngSnapshot,next:PngSnapshot):PngDiff{return equal(base.image,next.image)?{}:{image:parsePngImage(next.image)};}
export function absorbPngDiff(base:PngDiff,next:PngDiff):PngDiff{return parsePngDiff(next.image===undefined?base:next);}
