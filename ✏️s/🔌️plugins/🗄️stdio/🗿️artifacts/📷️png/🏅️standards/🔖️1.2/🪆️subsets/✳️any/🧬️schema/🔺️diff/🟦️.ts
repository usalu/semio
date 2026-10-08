/** 🔺️ Sparse Png diff algebra: a whole-image replacement, a gamma setter and ordered sample rectangles. */
import {parsePngImage,parsePngSnapshot,pngSamplesPerPixel,type PngImage,type PngSnapshot} from "../📸️snapshot/🟦️.ts";
export interface PngRegion { x:number; y:number; width:number; height:number; }
export interface PngSampleRect { region:PngRegion; samples:number[]; }
export interface PngGammaValue { gama:number|null; }
export interface PngDiff { image?:PngImage; gamma?:PngGammaValue; rects?:PngSampleRect[]; }
const equal=(a:unknown,b:unknown):boolean=>{if(a===b)return true;if(a===null||b===null||typeof a!=="object"||typeof b!=="object")return false;const left=Object.keys(a),right=Object.keys(b);return left.length===right.length&&left.every(key=>Object.hasOwn(b,key)&&equal((a as Record<string,unknown>)[key],(b as Record<string,unknown>)[key]));};
const fail=(at:string,why:string):never=>{throw new TypeError(`${at}: ${why}`);};
const isRecord=(value:unknown):value is Record<string,unknown>=>value!==null&&typeof value==="object"&&!Array.isArray(value);
const integer=(value:unknown,at:string,maximum:number):number=>Number.isSafeInteger(value)&&Number(value)>=0&&Number(value)<=maximum?Number(value):fail(at,"integer outside owned range");
function parseRect(value:unknown,at:string):PngSampleRect{
  if(!isRecord(value)||Object.keys(value).some(key=>key!=="region"&&key!=="samples")||!isRecord(value.region)||!Array.isArray(value.samples))return fail(at,"invalid sample rectangle");
  const region=value.region;if(Object.keys(region).some(key=>!["x","y","width","height"].includes(key)))return fail(at,"invalid region");
  return {region:{x:integer(region.x,`${at}.region.x`,4294967295),y:integer(region.y,`${at}.region.y`,4294967295),width:integer(region.width,`${at}.region.width`,4294967295),height:integer(region.height,`${at}.region.height`,4294967295)},samples:value.samples.map((sample,index)=>integer(sample,`${at}.samples[${index}]`,65535))};
}
export function parsePngDiff(value:unknown,at="$"):PngDiff{
  if(!isRecord(value)||Object.keys(value).some(key=>!["image","gamma","rects"].includes(key)))return fail(at,"invalid image diff");
  const diff:PngDiff={};
  if(value.image!==undefined)diff.image=parsePngImage(value.image,`${at}.image`);
  if(value.gamma!==undefined){if(!isRecord(value.gamma)||Object.keys(value.gamma).some(key=>key!=="gama"))return fail(at,"invalid gamma");diff.gamma={gama:value.gamma.gama===null||value.gamma.gama===undefined?null:integer(value.gamma.gama,`${at}.gamma.gama`,4294967295)};}
  if(value.rects!==undefined){if(!Array.isArray(value.rects))return fail(at,"invalid sample rectangles");if(value.rects.length>0)diff.rects=value.rects.map((rect,index)=>parseRect(rect,`${at}.rects[${index}]`));}
  return diff;
}
const rowStart=(image:PngImage,region:PngRegion):number|undefined=>region.width>0&&region.height>0&&region.x+region.width<=image.width&&region.y+region.height<=image.height?(region.y*image.width+region.x)*pngSamplesPerPixel(image.colorType):undefined;
export function pngRegionSamples(image:PngImage,region:PngRegion):number[]{
  const start=rowStart(image,region);if(start===undefined)return [];
  const spp=pngSamplesPerPixel(image.colorType),row=region.width*spp,stride=image.width*spp,out:number[]=[];
  for(let line=0;line<region.height;line+=1)out.push(...image.samples.slice(start+line*stride,start+line*stride+row));
  return out;
}
function writeRect(image:PngImage,rect:PngSampleRect):void{
  const start=rowStart(image,rect.region);if(start===undefined)return fail("$.rects","sample rectangle exceeds the owned image or is empty");
  const spp=pngSamplesPerPixel(image.colorType),row=rect.region.width*spp,stride=image.width*spp;
  if(rect.samples.length!==row*rect.region.height)return fail("$.rects","sample rectangle cardinality differs from its region");
  for(let line=0;line<rect.region.height;line+=1)image.samples.splice(start+line*stride,row,...rect.samples.slice(line*row,(line+1)*row));
}
export function applyPngDiff(base:PngSnapshot,diff:PngDiff):PngSnapshot{
  const image:PngImage=structuredClone(diff.image??base.image);
  if(diff.gamma!==undefined)image.gamma=diff.gamma.gama;
  (diff.rects??[]).forEach(rect=>writeRect(image,rect));
  return parsePngSnapshot({schema:base.schema,image});
}
export function inversePngDiff(base:PngSnapshot,diff:PngDiff):PngDiff{
  if(diff.image!==undefined)return {image:parsePngImage(base.image)};
  const out:PngDiff={};
  if(diff.gamma!==undefined&&diff.gamma.gama!==base.image.gamma)out.gamma={gama:base.image.gamma};
  const rects=diff.rects??[];
  if(rects.length>0){
    const running:PngImage=structuredClone(base.image);
    out.rects=rects.map(rect=>{const restore={region:{...rect.region},samples:pngRegionSamples(running,rect.region)};writeRect(running,rect);return restore;}).reverse();
  }
  return out;
}
export function betweenPngSnapshots(base:PngSnapshot,next:PngSnapshot):PngDiff{return equal(base.image,next.image)?{}:{image:parsePngImage(next.image)};}
export function absorbPngDiff(base:PngDiff,next:PngDiff):PngDiff{
  if(next.image!==undefined)return parsePngDiff(next);
  const out:PngDiff={...base};
  if(next.gamma!==undefined)out.gamma=next.gamma;
  if(next.rects!==undefined&&next.rects.length>0)out.rects=[...(base.rects??[]),...next.rects];
  return parsePngDiff(out);
}
